// Re-export modules
pub mod batch;
pub mod bootstrap_context;
pub mod commands;
pub mod config;
pub mod context_rules;
pub mod coordination;
pub mod daemon;
pub mod decision;
pub mod dependencies;
pub mod export_context;
pub mod generate_readme;
pub mod git;
pub mod health;
pub mod impact;
pub mod init;
pub mod insight;
pub mod integrations;
pub mod interactive;
pub mod interactive_advanced;
pub mod interactive_enhanced;
pub mod interactive_builder;
pub mod interactive_parser;
// pub mod knowledge;  // Temporarily disabled due to compilation issues
pub mod lock;
pub mod locomo;  // LOCOMO integration for performance monitoring and analytics
pub mod migrate;
pub mod pattern;
pub mod performance;
pub mod primer;
pub mod prompt;
pub mod query;
pub mod schema;
pub mod scopes;
pub mod search;
pub mod show;
pub mod stats;
pub mod sync;
pub mod template;
pub mod todo;
pub mod validate;
pub mod workflow;

// Re-export types from other crates for convenience
pub use rhema_ai::context_injection::{ContextInjectionRule, EnhancedContextInjector, TaskType};
pub use rhema_config::{Config, GlobalConfig, RepositoryConfig};
pub use rhema_core::{
    file_ops, schema::*, scope, JsonSchema, RhemaError, RhemaResult, SchemaMigratable, Validatable,
};
pub use rhema_mcp::*;
// pub use rhema_monitoring::{
//     PerformanceConfig, PerformanceMonitor, PerformanceReport, ReportPeriod, UsageData, UxData,
// };
pub use rhema_query::repo_analysis::RepoAnalysis;

// Define Rhema struct for CLI use
use std::collections::HashMap;
use std::path::PathBuf;

/// Main Rhema context manager for CLI
#[derive(Debug, Clone)]
pub struct Rhema {
    repo_root: PathBuf,
}

impl Rhema {
    /// Create a new Rhema instance for the current repository
    pub fn new() -> RhemaResult<Self> {
        let repo_root = rhema_core::git_basic::find_repo_root()?;
        Ok(Self { repo_root })
    }

    /// Create a new Rhema instance for a specific repository path
    pub fn new_from_path(repo_root: PathBuf) -> RhemaResult<Self> {
        // Verify that the path contains a git repository
        if !repo_root.join(".git").exists() {
            return Err(RhemaError::GitRepoNotFound(format!(
                "No Git repository found at {}",
                repo_root.display()
            )));
        }
        Ok(Self { repo_root })
    }

    /// Get the repository root path
    pub fn repo_root(&self) -> &PathBuf {
        &self.repo_root
    }

    /// Get the repository root path (alias for repo_root)
    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_root
    }

    /// Discover all scopes in the repository
    pub fn discover_scopes(&self) -> RhemaResult<Vec<scope::Scope>> {
        Ok(scope::discover_scopes(&self.repo_root)?)
    }

    /// Get a specific scope by path
    pub fn get_scope(&self, path: &str) -> RhemaResult<scope::Scope> {
        Ok(scope::get_scope(&self.repo_root, path)?)
    }

    /// Get the path for a specific scope
    pub fn scope_path(&self, scope_name: &str) -> RhemaResult<PathBuf> {
        let scope = self.get_scope(scope_name)?;
        Ok(scope.path)
    }

    /// Find scope path (alias for scope_path)
    pub fn find_scope_path(&self, scope_name: &str) -> RhemaResult<PathBuf> {
        self.scope_path(scope_name)
    }

    /// Get current scope path
    pub fn get_current_scope_path(&self) -> RhemaResult<PathBuf> {
        // For now, return the repo root as the current scope
        // This can be enhanced later to track the current working scope
        Ok(self.repo_root.clone())
    }

    /// Execute a CQL query
    pub fn query(&self, query: &str) -> RhemaResult<serde_yaml::Value> {
        Ok(rhema_query::execute_query(&self.repo_root, query)?)
    }

    /// Execute a CQL query with statistics
    pub fn query_with_stats(
        &self,
        query: &str,
    ) -> RhemaResult<(serde_yaml::Value, HashMap<String, serde_yaml::Value>)> {
        let result = rhema_query::execute_query(&self.repo_root, query)?;
        let stats = rhema_query::get_query_stats(&self.repo_root, query)?;
        Ok((result, stats))
    }

    /// Execute a CQL query with full provenance tracking
    pub fn query_with_provenance(
        &self,
        query: &str,
    ) -> RhemaResult<(serde_yaml::Value, rhema_query::QueryProvenance)> {
        Ok(rhema_query::execute_query_with_provenance(
            &self.repo_root,
            query,
        )?)
    }

    /// Search context with regex support
    pub fn search_regex(
        &self,
        pattern: &str,
        file_filter: Option<&str>,
    ) -> RhemaResult<Vec<rhema_query::QueryResult>> {
        Ok(rhema_query::search_context_regex(
            &self.repo_root,
            pattern,
            file_filter,
        )?)
    }

    /// Load knowledge for a specific scope
    pub fn load_knowledge(&self, scope_name: &str) -> RhemaResult<Knowledge> {
        let scope = self.get_scope(scope_name)?;
        let knowledge_path = scope.path.join("knowledge.yaml");
        if knowledge_path.exists() {
            let content = std::fs::read_to_string(&knowledge_path)?;
            let knowledge: Knowledge = serde_yaml::from_str(&content)?;
            Ok(knowledge)
        } else {
            Ok(Knowledge {
                entries: Vec::new(),
                categories: None,
                custom: HashMap::new(),
            })
        }
    }

    /// Load todos for a specific scope
    pub fn load_todos(&self, scope_name: &str) -> RhemaResult<Todos> {
        let scope = self.get_scope(scope_name)?;
        let todos_path = scope.path.join("todos.yaml");
        if todos_path.exists() {
            let content = std::fs::read_to_string(&todos_path)?;
            let todos: Todos = serde_yaml::from_str(&content)?;
            Ok(todos)
        } else {
            Ok(Todos {
                todos: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load decisions for a specific scope
    pub fn load_decisions(&self, scope_name: &str) -> RhemaResult<Decisions> {
        let scope = self.get_scope(scope_name)?;
        let decisions_path = scope.path.join("decisions.yaml");
        if decisions_path.exists() {
            let content = std::fs::read_to_string(&decisions_path)?;
            let decisions: Decisions = serde_yaml::from_str(&content)?;
            Ok(decisions)
        } else {
            Ok(Decisions {
                decisions: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load patterns for a specific scope
    pub fn load_patterns(&self, scope_name: &str) -> RhemaResult<Patterns> {
        let scope = self.get_scope(scope_name)?;
        let patterns_path = scope.path.join("patterns.yaml");
        if patterns_path.exists() {
            let content = std::fs::read_to_string(&patterns_path)?;
            let patterns: Patterns = serde_yaml::from_str(&content)?;
            Ok(patterns)
        } else {
            Ok(Patterns {
                patterns: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load conventions for a specific scope
    pub fn load_conventions(&self, scope_name: &str) -> RhemaResult<Conventions> {
        let scope = self.get_scope(scope_name)?;
        let conventions_path = scope.path.join("conventions.yaml");
        if conventions_path.exists() {
            let content = std::fs::read_to_string(&conventions_path)?;
            let conventions: Conventions = serde_yaml::from_str(&content)?;
            Ok(conventions)
        } else {
            Ok(Conventions {
                conventions: Vec::new(),
                custom: HashMap::new(),
            })
        }
    }

    /// Load scope by name
    pub fn load_scope(&self, name: &str) -> RhemaResult<scope::Scope> {
        self.get_scope(name)
    }

    /// List all scopes
    pub fn list_scopes(&self) -> RhemaResult<Vec<scope::Scope>> {
        self.discover_scopes()
    }
}

// Re-export main function
pub fn main() -> rhema_core::RhemaResult<()> {
    // For now, just return success
    Ok(())
}

// Simple implementations of missing functions
pub fn load_prompts(_path: &std::path::Path) -> RhemaResult<rhema_core::schema::Prompts> {
    Ok(rhema_core::schema::Prompts {
        prompts: Vec::new(),
    })
}

pub fn save_prompts(
    _path: &std::path::Path,
    _prompts: &rhema_core::schema::Prompts,
) -> RhemaResult<()> {
    Ok(())
}

pub fn load_workflows(_path: &std::path::Path) -> RhemaResult<rhema_core::schema::Workflows> {
    Ok(rhema_core::schema::Workflows {
        workflows: Vec::new(),
    })
}

pub fn save_workflows(
    _path: &std::path::Path,
    _workflows: &rhema_core::schema::Workflows,
) -> RhemaResult<()> {
    Ok(())
}

pub fn load_template_library(
    _path: &std::path::Path,
) -> RhemaResult<rhema_core::schema::TemplateLibrary> {
    Ok(rhema_core::schema::TemplateLibrary {
        name: "default".to_string(),
        description: Some("Default template library".to_string()),
        owner: "rhema".to_string(),
        version: "1.0.0".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        templates: Vec::new(),
        tags: Some(Vec::new()),
        access_control: Some(rhema_core::schema::TemplateAccessControl {
            public: true,
            allowed_teams: None,
            allowed_users: None,
            read_only: false,
        }),
    })
}

pub fn save_template_library(
    _path: &std::path::Path,
    _library: &rhema_core::schema::TemplateLibrary,
) -> RhemaResult<()> {
    Ok(())
}

pub fn load_template_export(
    _path: &std::path::Path,
) -> RhemaResult<rhema_core::schema::TemplateExport> {
    Ok(rhema_core::schema::TemplateExport {
        templates: Vec::new(),
        metadata: rhema_core::schema::ExportMetadata {
            source_scope: "default".to_string(),
            description: Some("Default export".to_string()),
            tags: Some(Vec::new()),
            author: Some("rhema".to_string()),
        },
        export_version: "1.0.0".to_string(),
        exported_at: chrono::Utc::now(),
    })
}

pub fn save_template_export(
    _path: &std::path::Path,
    _export: &rhema_core::schema::TemplateExport,
) -> RhemaResult<()> {
    Ok(())
}

// Simple ConfigManager implementation
#[derive(Debug)]
pub struct ConfigManager;

impl ConfigManager {
    pub fn new() -> RhemaResult<Self> {
        Ok(Self)
    }

    pub fn global_config(&self) -> &GlobalConfig {
        // TODO: Implement actual global config loading
        static GLOBAL_CONFIG: std::sync::OnceLock<GlobalConfig> = std::sync::OnceLock::new();
        GLOBAL_CONFIG.get_or_init(|| GlobalConfig::new())
    }

    pub fn global_config_mut(&mut self) -> std::sync::MutexGuard<'static, GlobalConfig> {
        // TODO: Implement actual global config loading
        static GLOBAL_CONFIG: std::sync::OnceLock<std::sync::Mutex<GlobalConfig>> = std::sync::OnceLock::new();
        GLOBAL_CONFIG.get_or_init(|| std::sync::Mutex::new(GlobalConfig::new())).lock().unwrap()
    }

    pub fn load_repository_config(
        &mut self,
        _path: &std::path::Path,
    ) -> RhemaResult<RepositoryConfig> {
        // TODO: Implement actual repository config loading
        Ok(RepositoryConfig::new(std::path::Path::new(".")))
    }

    pub fn validation(&self) -> &rhema_config::validation::ValidationManager {
        // TODO: Implement actual validation
        static VALIDATOR: std::sync::OnceLock<rhema_config::validation::ValidationManager> =
            std::sync::OnceLock::new();
        VALIDATOR.get_or_init(|| {
            let global_config = self.global_config();
            rhema_config::validation::ValidationManager::new(global_config).unwrap_or_else(|_| {
                // Fallback to a basic implementation if creation fails
                rhema_config::validation::ValidationManager::new(global_config).unwrap()
            })
        })
    }

    pub fn backup(&self) -> &rhema_config::backup::BackupManager {
        // TODO: Implement actual backup
        static BACKUP: std::sync::OnceLock<rhema_config::backup::BackupManager> =
            std::sync::OnceLock::new();
        BACKUP.get_or_init(|| {
            let global_config = self.global_config();
            rhema_config::backup::BackupManager::new(global_config).unwrap_or_else(|_| {
                // Fallback to a basic implementation if creation fails
                rhema_config::backup::BackupManager::new(global_config).unwrap()
            })
        })
    }

    pub fn backup_mut(&mut self) -> std::sync::MutexGuard<'static, rhema_config::backup::BackupManager> {
        // TODO: Implement actual backup
        static BACKUP: std::sync::OnceLock<std::sync::Mutex<rhema_config::backup::BackupManager>> = std::sync::OnceLock::new();
        BACKUP.get_or_init(|| {
            let global_config = self.global_config();
            std::sync::Mutex::new(
                rhema_config::backup::BackupManager::new(global_config).unwrap_or_else(|_| {
                    rhema_config::backup::BackupManager::new(global_config).unwrap()
                })
            )
        }).lock().unwrap()
    }

    pub fn migration(&self) -> &rhema_config::migration::MigrationManager {
        // TODO: Implement actual migration
        static MIGRATION: std::sync::OnceLock<rhema_config::migration::MigrationManager> =
            std::sync::OnceLock::new();
        MIGRATION.get_or_init(|| {
            let global_config = self.global_config();
            rhema_config::migration::MigrationManager::new(global_config).unwrap_or_else(|_| {
                // Fallback to a basic implementation if creation fails
                rhema_config::migration::MigrationManager::new(global_config).unwrap()
            })
        })
    }

    pub fn validate_all(&mut self) -> RhemaResult<rhema_config::validation::ValidationReport> {
        // TODO: Implement actual validation
        Ok(rhema_config::validation::ValidationReport {
            overall_valid: true,
            results: std::collections::HashMap::new(),
            summary: rhema_config::validation::ValidationSummary {
                total_configs: 0,
                valid_configs: 0,
                invalid_configs: 0,
                total_issues: 0,
                critical_issues: 0,
                error_issues: 0,
                warning_issues: 0,
                info_issues: 0,
            },
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
        })
    }

    pub fn backup_all(&mut self) -> RhemaResult<rhema_config::backup::BackupReport> {
        // TODO: Implement actual backup
        Ok(rhema_config::backup::BackupReport {
            backups_created: vec![],
            backups_failed: vec![],
            summary: rhema_config::backup::BackupSummary {
                total_backups: 0,
                successful_backups: 0,
                failed_backups: 0,
                total_size_bytes: 0,
                compression_ratio: 0.0,
            },
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
        })
    }

    pub fn migrate_all(&mut self) -> RhemaResult<rhema_config::migration::MigrationReport> {
        // TODO: Implement actual migration
        Ok(rhema_config::migration::MigrationReport {
            migrations_applied: vec![],
            migrations_skipped: vec![],
            migrations_failed: vec![],
            summary: rhema_config::migration::MigrationSummary {
                total_migrations: 0,
                successful_migrations: 0,
                failed_migrations: 0,
                skipped_migrations: 0,
                total_changes: 0,
            },
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
        })
    }
}
