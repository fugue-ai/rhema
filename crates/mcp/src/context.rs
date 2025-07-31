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

use rhema_core::{RhemaResult, RhemaError, scope::Scope, schema::*};
use rhema_query::{execute_query, QueryResult};
use serde_json::Value;
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use chrono::Utc;

/// Context provider for Rhema data
pub struct ContextProvider {
    repo_root: PathBuf,
    scopes: Arc<RwLock<Vec<Scope>>>,
    knowledge_cache: Arc<RwLock<HashMap<String, Knowledge>>>,
    todos_cache: Arc<RwLock<HashMap<String, Todos>>>,
    decisions_cache: Arc<RwLock<HashMap<String, Decisions>>>,
    patterns_cache: Arc<RwLock<HashMap<String, Patterns>>>,
    conventions_cache: Arc<RwLock<HashMap<String, Conventions>>>,
}

impl ContextProvider {
    /// Create a new context provider
    pub fn new(repo_root: PathBuf) -> RhemaResult<Self> {
        Ok(Self {
            repo_root,
            scopes: Arc::new(RwLock::new(Vec::new())),
            knowledge_cache: Arc::new(RwLock::new(HashMap::new())),
            todos_cache: Arc::new(RwLock::new(HashMap::new())),
            decisions_cache: Arc::new(RwLock::new(HashMap::new())),
            patterns_cache: Arc::new(RwLock::new(HashMap::new())),
            conventions_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Initialize the context provider by loading all data
    pub async fn initialize(&self) -> RhemaResult<()> {
        tracing::info!("Initializing context provider for {:?}", self.repo_root);
        
        // Load scopes
        self.load_scopes().await?;
        
        // Load all context data
        self.load_all_context().await?;
        
        tracing::info!("Context provider initialized successfully");
        Ok(())
    }

    /// Reload all context data
    pub async fn reload(&self) -> RhemaResult<()> {
        tracing::info!("Reloading context data");
        self.initialize().await
    }

    /// Get all scopes
    pub async fn get_scopes(&self) -> RhemaResult<Vec<Scope>> {
        let scopes = self.scopes.read().await;
        Ok(scopes.clone())
    }

    /// Get scope by path
    pub async fn get_scope(&self, path: &str) -> RhemaResult<Option<Scope>> {
        let scopes = self.get_scopes().await?;
        Ok(scopes.iter().find(|s| s.path.to_string_lossy() == path).cloned())
    }

    /// Get knowledge for a scope
    pub async fn get_knowledge(&self, scope_path: &str) -> RhemaResult<Option<Knowledge>> {
        let knowledge = self.knowledge_cache.read().await;
        Ok(knowledge.get(scope_path).cloned())
    }

    /// Get todos for a scope
    pub async fn get_todos(&self, scope_path: &str) -> RhemaResult<Option<Todos>> {
        let todos = self.todos_cache.read().await;
        Ok(todos.get(scope_path).cloned())
    }

    /// Get decisions for a scope
    pub async fn get_decisions(&self, scope_path: &str) -> RhemaResult<Option<Decisions>> {
        let decisions = self.decisions_cache.read().await;
        Ok(decisions.get(scope_path).cloned())
    }

    /// Get patterns for a scope
    pub async fn get_patterns(&self, scope_path: &str) -> RhemaResult<Option<Patterns>> {
        let patterns = self.patterns_cache.read().await;
        Ok(patterns.get(scope_path).cloned())
    }

    /// Get conventions for a scope
    pub async fn get_conventions(&self, scope_path: &str) -> RhemaResult<Option<Conventions>> {
        let conventions = self.conventions_cache.read().await;
        Ok(conventions.get(scope_path).cloned())
    }

    /// Execute a CQL query
    pub async fn execute_query(&self, query: &str) -> RhemaResult<serde_json::Value> {
        let result = execute_query(&self.repo_root, query)?;
        // Convert serde_yaml::Value to serde_json::Value
        let json_string = serde_json::to_string(&result)?;
        let json_value = serde_json::from_str(&json_string)?;
        Ok(json_value)
    }

    /// Execute a CQL query with statistics
    pub async fn execute_query_with_stats(&self, query: &str) -> RhemaResult<(Value, HashMap<String, Value>)> {
        // TODO: Implement query with stats
        let result = self.execute_query(query).await?;
        let stats = HashMap::new();
        Ok((result, stats))
    }

    /// Search context with regex
    pub async fn search_regex(&self, pattern: &str, file_filter: Option<&str>) -> RhemaResult<Vec<QueryResult>> {
        rhema_query::search_context_regex(&self.repo_root, pattern, file_filter)
    }

    /// Get context statistics
    pub async fn get_stats(&self) -> RhemaResult<ContextStats> {
        let scopes = self.scopes.read().await;
        let knowledge = self.knowledge_cache.read().await;
        let todos = self.todos_cache.read().await;
        let decisions = self.decisions_cache.read().await;
        let patterns = self.patterns_cache.read().await;
        let conventions = self.conventions_cache.read().await;

        let total_knowledge_entries: usize = knowledge.values().map(|k| k.entries.len()).sum();
        let total_todos: usize = todos.values().map(|t| t.todos.len()).sum();
        let total_decisions: usize = decisions.values().map(|d| d.decisions.len()).sum();
        let total_patterns: usize = patterns.values().map(|p| p.patterns.len()).sum();
        let total_conventions: usize = conventions.values().map(|c| c.conventions.len()).sum();

        Ok(ContextStats {
            scopes_count: scopes.len(),
            knowledge_entries_count: total_knowledge_entries,
            todos_count: total_todos,
            decisions_count: total_decisions,
            patterns_count: total_patterns,
            conventions_count: total_conventions,
            last_updated: Utc::now(),
        })
    }

    /// Get context changes since a specific time
    pub async fn get_changes_since(&self, _since: chrono::DateTime<Utc>) -> RhemaResult<Vec<ContextChange>> {
        // TODO: Implement change tracking
        Ok(Vec::new())
    }

    async fn load_scopes(&self) -> RhemaResult<()> {
        // TODO: Fix this when Rhema is properly available
        // let rhema = crate::Rhema::new_from_path(self.repo_root.clone())?;
        // let scopes = rhema.discover_scopes()?;
        // *scopes_lock = scopes;
        // tracing::debug!("Loaded {} scopes", scopes_lock.len());
        return Err(rhema_core::RhemaError::InvalidInput("Rhema::new_from_path not implemented yet".to_string()));
        Ok(())
    }

    /// Load all context data
    async fn load_all_context(&self) -> RhemaResult<()> {
        let scopes = self.get_scopes().await?;
        
        // Load knowledge
        for scope in &scopes {
            if let Ok(knowledge) = self.load_knowledge_for_scope(scope).await {
                let mut knowledge_cache = self.knowledge_cache.write().await;
                knowledge_cache.insert(scope.path.to_string_lossy().to_string(), knowledge);
            }
        }
        
        // Load todos
        for scope in &scopes {
            if let Ok(todos) = self.load_todos_for_scope(scope).await {
                let mut todos_cache = self.todos_cache.write().await;
                todos_cache.insert(scope.path.to_string_lossy().to_string(), todos);
            }
        }
        
        // Load decisions
        for scope in &scopes {
            if let Ok(decisions) = self.load_decisions_for_scope(scope).await {
                let mut decisions_cache = self.decisions_cache.write().await;
                decisions_cache.insert(scope.path.to_string_lossy().to_string(), decisions);
            }
        }
        
        // Load patterns
        for scope in &scopes {
            if let Ok(patterns) = self.load_patterns_for_scope(scope).await {
                let mut patterns_cache = self.patterns_cache.write().await;
                patterns_cache.insert(scope.path.to_string_lossy().to_string(), patterns);
            }
        }
        
        // Load conventions
        for scope in &scopes {
            if let Ok(conventions) = self.load_conventions_for_scope(scope).await {
                let mut conventions_cache = self.conventions_cache.write().await;
                conventions_cache.insert(scope.path.to_string_lossy().to_string(), conventions);
            }
        }
        
        Ok(())
    }

    async fn load_knowledge_for_scope(&self, scope: &Scope) -> RhemaResult<Knowledge> {
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

    async fn load_todos_for_scope(&self, scope: &Scope) -> RhemaResult<Todos> {
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

    async fn load_decisions_for_scope(&self, scope: &Scope) -> RhemaResult<Decisions> {
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

    async fn load_patterns_for_scope(&self, scope: &Scope) -> RhemaResult<Patterns> {
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

    async fn load_conventions_for_scope(&self, scope: &Scope) -> RhemaResult<Conventions> {
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
}

/// Context statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextStats {
    pub scopes_count: usize,
    pub knowledge_entries_count: usize,
    pub todos_count: usize,
    pub decisions_count: usize,
    pub patterns_count: usize,
    pub conventions_count: usize,
    pub last_updated: chrono::DateTime<Utc>,
}

/// Context change information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextChange {
    pub scope_path: String,
    pub change_type: ChangeType,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub timestamp: chrono::DateTime<Utc>,
    pub details: Option<Value>,
}

/// Change type enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
}

/// Resource type enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ResourceType {
    Scope,
    Knowledge,
    Todo,
    Decision,
    Pattern,
    Convention,
} 
