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
pub use rhema_coordination::{ai_service, context_injection};

// Re-export monitoring and performance from the monitoring crate
pub use rhema_monitoring::{monitoring, performance};

// Re-export Git integration from the git crate
pub use rhema_git::git_basic;

// Re-export MCP daemon from the mcp crate
pub use rhema_mcp::mcp;

// Re-export configuration from the config crate
pub use rhema_config::{config, GlobalConfig, RepositoryConfig};

// Re-export CLI commands from the cli crate
pub use rhema_api::{
    interactive, interactive_advanced, interactive_parser, Rhema as CliRhema,
    // Re-export all the modules that main.rs needs
    init, scopes, show, query, search, validate, migrate, schema, health, stats,
    todo, insight, pattern, coordination, decision, dependencies, impact, sync, git,
    export_context, primer, generate_readme, bootstrap_context, daemon, performance,
    config, prompt, context_rules, workflow, template,
};

// Re-export integrations from the integrations crate
pub use rhema_integrations::{integrations, IntegrationManager};

// Re-export lock file system
pub mod lock;
pub use lock::{LockGenerator, LockSystem, DependencyResolver, LockValidator};

// Re-export commands from CLI crate
pub use rhema_api::commands;

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
        let repo_root = rhema_core::utils::find_repo_root()?;
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
        let current_dir = std::env::current_dir().map_err(|e| RhemaError::IoError(e))?;
        let scopes = self.discover_scopes()?;
        
        // Find the nearest scope to the current directory
        if let Some(nearest_scope) = rhema_core::scope::find_nearest_scope(&current_dir, &scopes) {
            Ok(nearest_scope.path.clone())
        } else {
            // If no scope found, return the repo root
            Ok(self.repo_root.clone())
        }
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
        let result = rhema_query::query::execute_query(&self.repo_root, query)?;
        let stats = rhema_query::query::get_query_stats(&self.repo_root, query)?;
        Ok((result, stats))
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
        pattern: &str,
        file_filter: Option<&str>,
    ) -> Result<Vec<rhema_query::query::QueryResult>> {
        Ok(rhema_query::query::search_context_regex(&self.repo_root, pattern, file_filter)?)
    }

    /// Load knowledge for a scope
    pub fn load_knowledge(&self, scope_name: &str) -> Result<rhema_core::schema::Knowledge> {
        let scope_path = self.scope_path(scope_name)?;
        let knowledge_file = rhema_core::file_ops::get_or_create_knowledge_file(&scope_path)?;
        let knowledge: rhema_core::schema::Knowledge = rhema_core::file_ops::read_yaml_file(&knowledge_file)?;
        Ok(knowledge)
    }

    /// Load todos for a scope
    pub fn load_todos(&self, scope_name: &str) -> Result<rhema_core::schema::Todos> {
        let scope_path = self.scope_path(scope_name)?;
        let todos_file = rhema_core::file_ops::get_or_create_todos_file(&scope_path)?;
        let todos: rhema_core::schema::Todos = rhema_core::file_ops::read_yaml_file(&todos_file)?;
        Ok(todos)
    }

    /// Load decisions for a scope
    pub fn load_decisions(&self, scope_name: &str) -> Result<rhema_core::schema::Decisions> {
        let scope_path = self.scope_path(scope_name)?;
        let decisions_file = rhema_core::file_ops::get_or_create_decisions_file(&scope_path)?;
        let decisions: rhema_core::schema::Decisions = rhema_core::file_ops::read_yaml_file(&decisions_file)?;
        Ok(decisions)
    }

    /// Load patterns for a scope
    pub fn load_patterns(&self, scope_name: &str) -> Result<rhema_core::schema::Patterns> {
        let scope_path = self.scope_path(scope_name)?;
        let patterns_file = rhema_core::file_ops::get_or_create_patterns_file(&scope_path)?;
        let patterns: rhema_core::schema::Patterns = rhema_core::file_ops::read_yaml_file(&patterns_file)?;
        Ok(patterns)
    }

    /// Load conventions for a scope
    pub fn load_conventions(&self, scope_name: &str) -> Result<rhema_core::schema::Conventions> {
        let scope_path = self.scope_path(scope_name)?;
        let conventions_file = rhema_core::file_ops::get_or_create_conventions_file(&scope_path)?;
        let conventions: rhema_core::schema::Conventions = rhema_core::file_ops::read_yaml_file(&conventions_file)?;
        Ok(conventions)
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
