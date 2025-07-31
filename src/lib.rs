/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Re-export core types from the core crate
pub use rhema_core::{error, file_ops, schema, scope, RhemaError, RhemaResult};

// Re-export query functionality from the query crate
pub use rhema_query::{query, repo_analysis};

// Re-export AI service and context injection from the ai crate
pub use rhema_ai::{ai_service, context_injection};

// Re-export monitoring and performance from the monitoring crate
pub use rhema_monitoring::{monitoring, performance};

// Re-export Git integration from the git crate
pub use rhema_git::git_basic;

// Re-export MCP daemon from the mcp crate
pub use rhema_mcp::mcp;

// Re-export configuration from the config crate
pub use rhema_config::{config, GlobalConfig, RepositoryConfig};

// Re-export CLI commands from the cli crate
pub use rhema_cli::{
    interactive, interactive_advanced, interactive_parser, Rhema as CliRhema,
};

// Re-export integrations from the integrations crate
pub use rhema_integrations::{integrations, IntegrationManager};

// Re-export lock file system
pub mod lock;
pub use lock::{LockGenerator, LockSystem, DependencyResolver, LockValidator};

// Re-export commands from CLI crate
pub use rhema_cli::commands;

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

/// Main Rhema context manager
#[derive(Debug, Clone)]
pub struct Rhema {
    repo_root: PathBuf,
}

impl Rhema {
    /// Create a new Rhema instance for the current repository
    pub fn new() -> Result<Self> {
        let repo_root = rhema_core::git_basic::find_repo_root()?;
        Ok(Self { repo_root })
    }

    /// Create a new Rhema instance for a specific repository path
    pub fn new_from_path(repo_root: PathBuf) -> Result<Self> {
        // Verify that the path contains a git repository
        if !repo_root.join(".git").exists() {
            return Err(rhema_core::RhemaError::GitRepoNotFound(format!(
                "No Git repository found at {}",
                repo_root.display()
            ))
            .into());
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
    pub fn discover_scopes(&self) -> Result<Vec<rhema_core::Scope>> {
        Ok(rhema_core::scope::discover_scopes(&self.repo_root)?)
    }

    /// Get a specific scope by path
    pub fn scope_path(&self, scope_name: &str) -> Result<PathBuf> {
        Ok(self.repo_root.join(scope_name))
    }

    /// Find scope path by name
    pub fn find_scope_path(&self, scope_name: &str) -> Result<PathBuf> {
        Ok(self.repo_root.join(scope_name))
    }

    /// Get current scope path
    pub fn get_current_scope_path(&self) -> Result<PathBuf> {
        // TODO: Implement current scope detection
        Ok(self.repo_root.clone())
    }

    /// Execute a CQL query
    pub fn query(&self, query: &str) -> Result<serde_yaml::Value> {
        Ok(rhema_query::query::execute_query(&self.repo_root, query)?)
    }

    /// Execute a CQL query with statistics
    pub fn query_with_stats(
        &self,
        query: &str,
    ) -> Result<(serde_yaml::Value, HashMap<String, serde_yaml::Value>)> {
        // TODO: Implement query with stats
        let result = rhema_query::query::execute_query(&self.repo_root, query)?;
        Ok((result, HashMap::new()))
    }

    /// Execute a CQL query with provenance tracking
    pub fn query_with_provenance(
        &self,
        query: &str,
    ) -> Result<(serde_yaml::Value, rhema_query::query::QueryProvenance)> {
        Ok(rhema_query::query::execute_query_with_provenance(
            &self.repo_root,
            query,
        )?)
    }

    /// Search using regex pattern
    pub fn search_regex(
        &self,
        _pattern: &str,
        _file_filter: Option<&str>,
    ) -> Result<Vec<rhema_query::query::QueryResult>> {
        // TODO: Implement regex search
        Ok(vec![])
    }

    /// Load knowledge for a scope
    pub fn load_knowledge(&self, _scope_name: &str) -> Result<rhema_core::schema::Knowledge> {
        // TODO: Implement knowledge loading
        Ok(rhema_core::schema::Knowledge {
            entries: vec![],
            categories: None,
            custom: std::collections::HashMap::new(),
        })
    }

    /// Load todos for a scope
    pub fn load_todos(&self, _scope_name: &str) -> Result<rhema_core::schema::Todos> {
        // TODO: Implement todos loading
        Ok(rhema_core::schema::Todos {
            todos: vec![],
            custom: std::collections::HashMap::new(),
        })
    }

    /// Load decisions for a scope
    pub fn load_decisions(&self, _scope_name: &str) -> Result<rhema_core::schema::Decisions> {
        // TODO: Implement decisions loading
        Ok(rhema_core::schema::Decisions {
            decisions: vec![],
            custom: std::collections::HashMap::new(),
        })
    }

    /// Load patterns for a scope
    pub fn load_patterns(&self, _scope_name: &str) -> Result<rhema_core::schema::Patterns> {
        // TODO: Implement patterns loading
        Ok(rhema_core::schema::Patterns {
            patterns: vec![],
            custom: std::collections::HashMap::new(),
        })
    }

    /// Load conventions for a scope
    pub fn load_conventions(&self, _scope_name: &str) -> Result<rhema_core::schema::Conventions> {
        // TODO: Implement conventions loading
        Ok(rhema_core::schema::Conventions {
            conventions: vec![],
            custom: std::collections::HashMap::new(),
        })
    }

    /// Load a specific scope
    pub fn load_scope(&self, name: &str) -> Result<rhema_core::Scope> {
        Ok(rhema_core::scope::get_scope(&self.repo_root, name)?)
    }

    /// List all scopes
    pub fn list_scopes(&self) -> Result<Vec<rhema_core::Scope>> {
        Ok(rhema_core::scope::discover_scopes(&self.repo_root)?)
    }
}
