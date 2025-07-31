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

use std::path::PathBuf;
use std::collections::HashMap;

// Re-export types from core crate
pub use rhema_core::{RhemaError, RhemaResult, Scope, schema::*, scope};

// Re-export types from other crates
pub use rhema_query::{QueryProvenance, QueryResult};
pub use rhema_git::git_basic;
pub use rhema_ai::ai_service;
pub use rhema_mcp::mcp;
pub use rhema_config::config;
pub use rhema_monitoring::monitoring;
pub use rhema_integrations::integrations;

/// Main Rhema context manager
#[derive(Debug, Clone)]
pub struct Rhema {
    repo_root: PathBuf,
}

impl Rhema {
    /// Create a new Rhema instance for the current repository
    pub fn new() -> RhemaResult<Self> {
        let repo_root = git_basic::find_repo_root()?;
        Ok(Self { repo_root })
    }

    /// Create a new Rhema instance for a specific repository path
    pub fn new_from_path(repo_root: PathBuf) -> RhemaResult<Self> {
        // Verify that the path contains a git repository
        if !repo_root.join(".git").exists() {
            return Err(RhemaError::GitRepoNotFound(
                format!("No Git repository found at {}", repo_root.display())
            ));
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
    pub fn discover_scopes(&self) -> RhemaResult<Vec<Scope>> {
        Ok(scope::discover_scopes(&self.repo_root)?)
    }

    /// Get a specific scope by path
    pub fn get_scope(&self, path: &str) -> RhemaResult<Scope> {
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
    pub fn query_with_stats(&self, query: &str) -> RhemaResult<(serde_yaml::Value, HashMap<String, serde_yaml::Value>)> {
        let result = rhema_query::execute_query(&self.repo_root, query)?;
        let stats = rhema_query::get_query_stats(&self.repo_root, query)?;
        Ok((result, stats))
    }

    /// Execute a CQL query with full provenance tracking
    pub fn query_with_provenance(&self, query: &str) -> RhemaResult<(serde_yaml::Value, QueryProvenance)> {
        Ok(rhema_query::execute_query_with_provenance(&self.repo_root, query)?)
    }

    /// Search context with regex support
    pub fn search_regex(&self, pattern: &str, file_filter: Option<&str>) -> RhemaResult<Vec<QueryResult>> {
        Ok(rhema_query::search_context_regex(&self.repo_root, pattern, file_filter)?)
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
    pub fn load_scope(&self, name: &str) -> RhemaResult<Scope> {
        self.get_scope(name)
    }

    /// List all scopes
    pub fn list_scopes(&self) -> RhemaResult<Vec<Scope>> {
        self.discover_scopes()
    }
} 