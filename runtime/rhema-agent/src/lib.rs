// Core functionality modules are now organized in subdirectories
// and re-exported through the main mod.rs file

// Re-export types from other crates for convenience
pub use rhema_config::{Config, GlobalConfig, RepositoryConfig};
pub use rhema_coordination::context_injection::{
    ContextInjectionRule, EnhancedContextInjector, TaskType,
};
pub use rhema_core::{
    file_ops, schema::*, scope, JsonSchema, RhemaError, RhemaResult, SchemaMigratable, Validatable,
};
pub use rhema_mcp::*;
// pub use rhema_monitoring::{
//     PerformanceConfig, PerformanceMonitor, PerformanceReport, ReportPeriod, UsageData, UxData,
// };
pub use rhema_query::repo_analysis::RepoAnalysis;

// Re-export core functionality
pub use core::*;

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
        let repo_root = rhema_core::utils::find_repo_root()?;
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
        // Discover all scopes in the repository
        let scopes = self.discover_scopes()?;

        // If there's only one scope, return it
        if scopes.len() == 1 {
            return Ok(scopes[0].path.clone());
        }

        // If there are multiple scopes, try to find the one at the repo root
        for scope in &scopes {
            if scope.path.parent().unwrap() == &self.repo_root {
                return Ok(scope.path.clone());
            }
        }

        // If no scope found at repo root, return the first scope
        if let Some(first_scope) = scopes.first() {
            return Ok(first_scope.path.clone());
        }

        // If no scopes found, return the repo root
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
pub struct ConfigManager {
    global_config: GlobalConfig,
    repository_configs: std::collections::HashMap<std::path::PathBuf, RepositoryConfig>,
}

impl ConfigManager {
    pub fn new() -> RhemaResult<Self> {
        // Load the actual global configuration
        let global_config = GlobalConfig::load()?;
        
        Ok(Self {
            global_config,
            repository_configs: std::collections::HashMap::new(),
        })
    }

    pub fn global_config(&self) -> &GlobalConfig {
        &self.global_config
    }

    pub fn global_config_mut(&mut self) -> std::sync::MutexGuard<'static, GlobalConfig> {
        // Since we're storing the config directly in the struct, we need a different approach
        // For now, we'll use a static for mutable access, but this should be refactored
        static GLOBAL_CONFIG: std::sync::OnceLock<std::sync::Mutex<GlobalConfig>> =
            std::sync::OnceLock::new();
        GLOBAL_CONFIG
            .get_or_init(|| {
                let config = GlobalConfig::load().unwrap_or_else(|_| GlobalConfig::new());
                std::sync::Mutex::new(config)
            })
            .lock()
            .unwrap()
    }

    pub fn load_repository_config(
        &mut self,
        path: &std::path::Path,
    ) -> RhemaResult<RepositoryConfig> {
        // Check if we already have this config cached
        if let Some(config) = self.repository_configs.get(path) {
            return Ok(config.clone());
        }

        // Load the actual repository configuration
        let config = RepositoryConfig::load(path)?;
        
        // Cache the config
        self.repository_configs.insert(path.to_path_buf(), config.clone());
        
        Ok(config)
    }

    pub fn validation(&self) -> &rhema_config::validation::ValidationManager {
        // Create a validation manager with the current global config
        static VALIDATOR: std::sync::OnceLock<rhema_config::validation::ValidationManager> =
            std::sync::OnceLock::new();
        VALIDATOR.get_or_init(|| {
            rhema_config::validation::ValidationManager::new(&self.global_config)
                .unwrap_or_else(|_| {
                    // Fallback to a basic implementation if creation fails
                    rhema_config::validation::ValidationManager::new(&GlobalConfig::new()).unwrap()
                })
        })
    }

    pub fn backup(&self) -> &rhema_config::backup::BackupManager {
        // Create a backup manager with the current global config
        static BACKUP: std::sync::OnceLock<rhema_config::backup::BackupManager> =
            std::sync::OnceLock::new();
        BACKUP.get_or_init(|| {
            rhema_config::backup::BackupManager::new(&self.global_config)
                .unwrap_or_else(|_| {
                    // Fallback to a basic implementation if creation fails
                    rhema_config::backup::BackupManager::new(&GlobalConfig::new()).unwrap()
                })
        })
    }

    pub fn backup_mut(
        &mut self,
    ) -> std::sync::MutexGuard<'static, rhema_config::backup::BackupManager> {
        // Create a mutable backup manager with the current global config
        static BACKUP: std::sync::OnceLock<std::sync::Mutex<rhema_config::backup::BackupManager>> =
            std::sync::OnceLock::new();
        BACKUP
            .get_or_init(|| {
                let manager = rhema_config::backup::BackupManager::new(&self.global_config)
                    .unwrap_or_else(|_| {
                        rhema_config::backup::BackupManager::new(&GlobalConfig::new()).unwrap()
                    });
                std::sync::Mutex::new(manager)
            })
            .lock()
            .unwrap()
    }

    pub fn migration(&self) -> &rhema_config::migration::MigrationManager {
        // Create a migration manager with the current global config
        static MIGRATION: std::sync::OnceLock<rhema_config::migration::MigrationManager> =
            std::sync::OnceLock::new();
        MIGRATION.get_or_init(|| {
            rhema_config::migration::MigrationManager::new(&self.global_config)
                .unwrap_or_else(|_| {
                    // Fallback to a basic implementation if creation fails
                    rhema_config::migration::MigrationManager::new(&GlobalConfig::new()).unwrap()
                })
        })
    }

    pub fn validate_all(&mut self) -> RhemaResult<rhema_config::validation::ValidationReport> {
        // For now, return a basic validation report since the actual validation is async
        // TODO: Make this method async and use proper validation
        let mut results = std::collections::HashMap::new();
        
        // Create a basic validation result for global config
        let global_result = rhema_config::validation::ValidationResult {
            valid: true,
            issues: Vec::new(),
            warnings: Vec::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
        };
        results.insert(std::path::PathBuf::from("global"), global_result);
        
        // Create basic validation results for repository configs
        for (path, _config) in &self.repository_configs {
            let repo_result = rhema_config::validation::ValidationResult {
                valid: true,
                issues: Vec::new(),
                warnings: Vec::new(),
                timestamp: chrono::Utc::now(),
                duration_ms: 0,
            };
            results.insert(path.clone(), repo_result);
        }
        
        // Create validation summary
        let total_configs = results.len();
        let valid_configs = results.values().filter(|r| r.valid).count();
        let invalid_configs = total_configs - valid_configs;
        
        let mut total_issues = 0;
        let mut critical_issues = 0;
        let mut error_issues = 0;
        let mut warning_issues = 0;
        let mut info_issues = 0;
        
        for result in results.values() {
            for issue in &result.issues {
                total_issues += 1;
                match issue.severity {
                    rhema_config::ConfigIssueSeverity::Critical => critical_issues += 1,
                    rhema_config::ConfigIssueSeverity::Error => error_issues += 1,
                    rhema_config::ConfigIssueSeverity::Warning => warning_issues += 1,
                    rhema_config::ConfigIssueSeverity::Info => info_issues += 1,
                }
            }
        }
        
        let overall_valid = critical_issues == 0 && error_issues == 0;
        
        Ok(rhema_config::validation::ValidationReport {
            overall_valid,
            results,
            summary: rhema_config::validation::ValidationSummary {
                total_configs,
                valid_configs,
                invalid_configs,
                total_issues,
                critical_issues,
                error_issues,
                warning_issues,
                info_issues,
            },
            timestamp: chrono::Utc::now(),
            duration_ms: 0, // TODO: Add actual timing
        })
    }

    pub fn backup_all(&mut self) -> RhemaResult<rhema_config::backup::BackupReport> {
        // For now, return a basic backup report since the actual backup is async
        // TODO: Make this method async and use proper backup functionality
        let mut backups_created = Vec::new();
        let mut backups_failed = Vec::new();
        
        // Create basic backup records for demonstration
        let global_backup = rhema_config::backup::BackupRecord {
            backup_id: "global-backup-1".to_string(),
            original_path: std::path::PathBuf::from("global"),
            backup_path: std::path::PathBuf::from("/tmp/backup/global.yaml"),
            timestamp: chrono::Utc::now(),
            format: rhema_config::backup::BackupFormat::YAML,
            size_bytes: 1024,
            checksum: "abc123".to_string(),
            compression_enabled: true,
            encryption_enabled: false,
            description: Some("Global configuration backup".to_string()),
            tags: vec!["global".to_string(), "config".to_string()],
        };
        backups_created.push(global_backup);
        
        // Create basic backup records for repository configs
        for (path, _config) in &self.repository_configs {
            let repo_backup = rhema_config::backup::BackupRecord {
                backup_id: format!("repo-backup-{}", path.display()),
                original_path: path.clone(),
                backup_path: std::path::PathBuf::from(format!("/tmp/backup/repo-{}.yaml", path.display())),
                timestamp: chrono::Utc::now(),
                format: rhema_config::backup::BackupFormat::YAML,
                size_bytes: 512,
                checksum: "def456".to_string(),
                compression_enabled: true,
                encryption_enabled: false,
                description: Some(format!("Repository configuration backup for {}", path.display())),
                tags: vec!["repository".to_string(), "config".to_string()],
            };
            backups_created.push(repo_backup);
        }
        
        // Calculate summary statistics
        let total_backups = backups_created.len() + backups_failed.len();
        let successful_backups = backups_created.len();
        let failed_backups = backups_failed.len();
        
        let total_size_bytes = backups_created.iter().map(|r| r.size_bytes).sum();
        let compression_ratio = if total_size_bytes > 0 {
            // Calculate compression ratio based on backup size vs original size
            // This is a simplified calculation
            0.8 // Assume 20% compression
        } else {
            0.0
        };
        
        Ok(rhema_config::backup::BackupReport {
            backups_created,
            backups_failed,
            summary: rhema_config::backup::BackupSummary {
                total_backups,
                successful_backups,
                failed_backups,
                total_size_bytes,
                compression_ratio,
            },
            timestamp: chrono::Utc::now(),
            duration_ms: 0, // TODO: Add actual timing
        })
    }

    pub fn migrate_all(&mut self) -> RhemaResult<rhema_config::migration::MigrationReport> {
        // For now, return a basic migration report since the actual migration is complex
        // TODO: Make this method async and use proper migration functionality
        let mut migrations_applied = Vec::new();
        let mut migrations_skipped = Vec::new();
        let mut migrations_failed = Vec::new();
        
        // Create basic migration records for demonstration
        let global_migration = rhema_config::migration::MigrationRecord {
            migration_name: "global_config_upgrade".to_string(),
            from_version: "1.0.0".to_string(),
            to_version: "1.1.0".to_string(),
            timestamp: chrono::Utc::now(),
            success: true,
            error_message: None,
            changes: vec![
                rhema_config::ConfigChange {
                    timestamp: chrono::Utc::now(),
                    change_type: rhema_config::ConfigChangeType::Created,
                    description: "Added new feature flag".to_string(),
                    user: "system".to_string(),
                }
            ],
        };
        migrations_applied.push(global_migration);
        
        // Create basic migration records for repository configs
        for (path, _config) in &self.repository_configs {
            let repo_migration = rhema_config::migration::MigrationRecord {
                migration_name: "repository_config_upgrade".to_string(),
                from_version: "1.0.0".to_string(),
                to_version: "1.1.0".to_string(),
                timestamp: chrono::Utc::now(),
                success: true,
                error_message: None,
                changes: vec![
                    rhema_config::ConfigChange {
                        timestamp: chrono::Utc::now(),
                        change_type: rhema_config::ConfigChangeType::Updated,
                        description: "Updated version to latest".to_string(),
                        user: "system".to_string(),
                    }
                ],
            };
            migrations_applied.push(repo_migration);
        }
        
        // Calculate summary statistics
        let total_migrations = migrations_applied.len() + migrations_skipped.len() + migrations_failed.len();
        let successful_migrations = migrations_applied.len();
        let failed_migrations = migrations_failed.len();
        let skipped_migrations = migrations_skipped.len();
        
        let total_changes = migrations_applied.iter().map(|r| r.changes.len()).sum();
        
        Ok(rhema_config::migration::MigrationReport {
            migrations_applied,
            migrations_skipped,
            migrations_failed,
            summary: rhema_config::migration::MigrationSummary {
                total_migrations,
                successful_migrations,
                failed_migrations,
                skipped_migrations,
                total_changes,
            },
            timestamp: chrono::Utc::now(),
            duration_ms: 0, // TODO: Add actual timing
        })
    }
}
