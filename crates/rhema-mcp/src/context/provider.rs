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

use chrono::Utc;
use rhema_core::{scope::Scope, RhemaError, RhemaLock, RhemaResult, Knowledge, Todos, Decisions, Patterns, Conventions, TodoStatus, Validatable};
use rhema_query::QueryResult;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::*;
use super::validation::{ValidationSeverity, ValidationError, ValidationWarning, ValidationStats, ValidationErrorType, ContextValidationResult, ScopeValidationResult, CrossReferenceValidation, BrokenReference, ConsistencyValidation, NamingConflict, DuplicateEntry, TemporalValidation, TemporalAnomaly, TimestampSequence, ExpiredEntry, DependencyValidation};

/// Context provider for Rhema data
pub struct ContextProvider {
    repo_root: PathBuf,
    scopes: Arc<RwLock<Vec<Scope>>>,
    knowledge_cache: Arc<RwLock<HashMap<String, Knowledge>>>,
    todos_cache: Arc<RwLock<HashMap<String, Todos>>>,
    decisions_cache: Arc<RwLock<HashMap<String, Decisions>>>,
    patterns_cache: Arc<RwLock<HashMap<String, Patterns>>>,
    conventions_cache: Arc<RwLock<HashMap<String, Conventions>>>,
    // Lock file cache for AI agent context
    lock_file_cache: Arc<RwLock<Option<RhemaLock>>>,

    // Enhanced context management features
    context_cache: Arc<RwLock<HashMap<String, ContextCacheEntry<Value>>>>,
    context_versions: Arc<RwLock<HashMap<String, Vec<ContextVersion>>>>,
    sync_status: Arc<RwLock<ContextSyncStatus>>,
    backup_config: ContextBackupConfig,
    cache_config: ContextCacheConfig,
    sync_config: ContextSyncConfig,
    version_config: ContextVersionConfig,
    compression_config: ContextCompressionConfig,
    encryption_config: ContextEncryptionConfig,

    // Background tasks
    sync_task: Option<tokio::task::JoinHandle<()>>,
    backup_task: Option<tokio::task::JoinHandle<()>>,
    cleanup_task: Option<tokio::task::JoinHandle<()>>,
}

impl Clone for ContextProvider {
    fn clone(&self) -> Self {
        Self {
            repo_root: self.repo_root.clone(),
            scopes: self.scopes.clone(),
            knowledge_cache: self.knowledge_cache.clone(),
            todos_cache: self.todos_cache.clone(),
            decisions_cache: self.decisions_cache.clone(),
            patterns_cache: self.patterns_cache.clone(),
            conventions_cache: self.conventions_cache.clone(),
            lock_file_cache: self.lock_file_cache.clone(),
            context_cache: self.context_cache.clone(),
            context_versions: self.context_versions.clone(),
            sync_status: self.sync_status.clone(),
            backup_config: self.backup_config.clone(),
            cache_config: self.cache_config.clone(),
            sync_config: self.sync_config.clone(),
            version_config: self.version_config.clone(),
            compression_config: self.compression_config.clone(),
            encryption_config: self.encryption_config.clone(),
            sync_task: None,    // JoinHandle cannot be cloned
            backup_task: None,  // JoinHandle cannot be cloned
            cleanup_task: None, // JoinHandle cannot be cloned
        }
    }
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
            lock_file_cache: Arc::new(RwLock::new(None)),

            // Enhanced features
            context_cache: Arc::new(RwLock::new(HashMap::new())),
            context_versions: Arc::new(RwLock::new(HashMap::new())),
            sync_status: Arc::new(RwLock::new(ContextSyncStatus {
                last_sync: Some(Utc::now()),
                sync_status: SyncStatus::InSync,
                sync_errors: Vec::new(),
                pending_changes: 0,
                conflicts_resolved: 0,
            })),
            backup_config: ContextBackupConfig::default(),
            cache_config: ContextCacheConfig::default(),
            sync_config: ContextSyncConfig::default(),
            version_config: ContextVersionConfig::default(),
            compression_config: ContextCompressionConfig::default(),
            encryption_config: ContextEncryptionConfig::default(),

            // Background tasks
            sync_task: None,
            backup_task: None,
            cleanup_task: None,
        })
    }

    /// Get the repository root path
    pub fn repo_root(&self) -> &std::path::Path {
        &self.repo_root
    }

    /// List all available resources
    pub async fn list_resources(&self) -> RhemaResult<Vec<serde_json::Value>> {
        let scopes = self.get_scopes().await?;
        let mut resources = Vec::new();

        for scope in scopes {
            let scope_path_str = scope.path.to_string_lossy();
            let scope_resource = serde_json::json!({
                "uri": format!("scope://{}", scope_path_str),
                "name": scope_path_str,
                "description": "Scope resource",
                "mime_type": "application/json",
                "content": scope,
                "metadata": {
                    "type": "scope",
                    "path": scope_path_str
                }
            });
            resources.push(scope_resource);

            // Add knowledge resource
            if let Some(knowledge) = self.get_knowledge(&scope_path_str).await? {
                let knowledge_resource = serde_json::json!({
                    "uri": format!("knowledge://{}", scope_path_str),
                    "name": format!("{}_knowledge", scope_path_str),
                    "description": "Knowledge resource",
                    "mime_type": "application/json",
                    "content": knowledge,
                    "metadata": {
                        "type": "knowledge",
                        "scope": scope_path_str
                    }
                });
                resources.push(knowledge_resource);
            }

            // Add todos resource
            if let Some(todos) = self.get_todos(&scope_path_str).await? {
                let todos_resource = serde_json::json!({
                    "uri": format!("todos://{}", scope_path_str),
                    "name": format!("{}_todos", scope_path_str),
                    "description": "Todos resource",
                    "mime_type": "application/json",
                    "content": todos,
                    "metadata": {
                        "type": "todos",
                        "scope": scope_path_str
                    }
                });
                resources.push(todos_resource);
            }
        }

        Ok(resources)
    }

    /// Get a specific resource by URI
    pub async fn get_resource(&self, uri: &str) -> RhemaResult<serde_json::Value> {
        if uri.starts_with("scope://") {
            let scope_path = uri.strip_prefix("scope://").unwrap();
            if let Some(scope) = self.get_scope(scope_path).await? {
                return Ok(serde_json::json!({
                    "uri": uri,
                    "name": scope.path,
                    "description": "Scope resource",
                    "mime_type": "application/json",
                    "content": scope,
                    "metadata": {
                        "type": "scope",
                        "path": scope.path
                    }
                }));
            }
        } else if uri.starts_with("knowledge://") {
            let scope_path = uri.strip_prefix("knowledge://").unwrap();
            if let Some(knowledge) = self.get_knowledge(scope_path).await? {
                return Ok(serde_json::json!({
                    "uri": uri,
                    "name": format!("{}_knowledge", scope_path),
                    "description": "Knowledge resource",
                    "mime_type": "application/json",
                    "content": knowledge,
                    "metadata": {
                        "type": "knowledge",
                        "scope": scope_path
                    }
                }));
            }
        } else if uri.starts_with("todos://") {
            let scope_path = uri.strip_prefix("todos://").unwrap();
            if let Some(todos) = self.get_todos(scope_path).await? {
                return Ok(serde_json::json!({
                    "uri": uri,
                    "name": format!("{}_todos", scope_path),
                    "description": "Todos resource",
                    "mime_type": "application/json",
                    "content": todos,
                    "metadata": {
                        "type": "todos",
                        "scope": scope_path
                    }
                }));
            }
        }

        Err(RhemaError::InvalidInput(format!(
            "Resource not found: {}",
            uri
        )))
    }

    /// Initialize the context provider by loading all data
    pub async fn initialize(&self) -> RhemaResult<()> {
        tracing::info!("Initializing context provider for {:?}", self.repo_root);

        // Load scopes
        self.load_scopes().await?;

        // Load all context data
        self.load_all_context().await?;

        // Load lock file data
        self.load_lock_file().await?;

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
        Ok(scopes
            .iter()
            .find(|s| s.path.to_string_lossy() == path)
            .cloned())
    }

    /// Get knowledge for a scope
    pub async fn get_knowledge(&self, scope_path: &str) -> RhemaResult<Option<Knowledge>> {
        let knowledge = self.knowledge_cache.read().await;
        Ok(knowledge.get(scope_path).cloned())
    }

    /// Get knowledge for a scope (for MCP compatibility)
    pub async fn get_knowledge_for_mcp(&self, scope_path: &str) -> RhemaResult<serde_json::Value> {
        let knowledge = self.get_knowledge(scope_path).await?;
        match knowledge {
            Some(k) => Ok(serde_json::to_value(k)?),
            None => Ok(serde_json::json!({ "error": "Scope not found" })),
        }
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

    /// Get lock file information for AI agent context
    pub async fn get_lock_file(&self) -> RhemaResult<Option<RhemaLock>> {
        let lock_file = self.lock_file_cache.read().await;
        Ok(lock_file.clone())
    }

    /// Get lock file context for a specific scope
    pub async fn get_scope_lock_context(
        &self,
        scope_path: &str,
    ) -> RhemaResult<Option<LockScopeContext>> {
        let lock_file = self.get_lock_file().await?;

        if let Some(lock) = lock_file {
            if let Some(locked_scope) = lock.scopes.get(scope_path) {
                return Ok(Some(LockScopeContext {
                    scope_path: scope_path.to_string(),
                    version: locked_scope.version.clone(),
                    dependencies: HashMap::new(), // TODO: Convert LockedDependency to ScopeDependency
                    has_circular_dependencies: locked_scope.has_circular_dependencies,
                    resolved_at: Some(locked_scope.resolved_at.clone()),
                    source_checksum: locked_scope.source_checksum.clone(),
                }));
            }
        }

        Ok(None)
    }

    /// Get dependency version information for AI agents
    pub async fn get_dependency_versions(
        &self,
        scope_path: &str,
    ) -> RhemaResult<Vec<DependencyVersionInfo>> {
        let lock_context = self.get_scope_lock_context(scope_path).await?;

        if let Some(context) = lock_context {
            let mut dependency_info = Vec::new();

            for (dep_name, dep) in &context.dependencies {
                dependency_info.push(DependencyVersionInfo {
                    name: dep_name.clone(),
                    version: dep.version.clone().expect("REASON"),
                    path: dep.path.clone(),
                    dependency_type: dep.dependency_type.clone(),
                    is_transitive: false, // Default value since field doesn't exist
                    original_constraint: None, // Default value since field doesn't exist
                    resolved_at: None, // Default value since field doesn't exist
                    checksum: None, // Default value since field doesn't exist
                });
            }

            return Ok(dependency_info);
        }

        Ok(Vec::new())
    }

    /// Get conflict prevention information for AI agents
    pub async fn get_conflict_prevention_info(&self) -> RhemaResult<ConflictPreventionInfo> {
        let lock_file = self.get_lock_file().await?;

        if let Some(lock) = lock_file {
            let mut conflict_info = ConflictPreventionInfo {
                total_scopes: lock.metadata.total_scopes as usize,
                total_dependencies: lock.metadata.total_dependencies as usize,
                circular_dependencies: lock.metadata.circular_dependencies as usize,
                validation_status: format!("{:?}", lock.metadata.validation_status),
                resolution_strategy: format!("{:?}", lock.metadata.resolution_strategy),
                conflict_resolution: format!("{:?}", lock.metadata.conflict_resolution),
                potential_conflicts: Vec::new(),
                dependency_graph: HashMap::new(),
            };

            // Analyze potential conflicts
            for (scope_path, locked_scope) in &lock.scopes {
                for (dep_name, dep) in &locked_scope.dependencies {
                    // Check for version conflicts
                    for (other_scope_path, other_scope) in &lock.scopes {
                        if scope_path != other_scope_path {
                            if let Some(other_dep) = other_scope.dependencies.get(dep_name) {
                                if dep.version != other_dep.version {
                                    conflict_info.potential_conflicts.push(VersionConflict {
                                        dependency_name: dep_name.clone(),
                                        scope1: scope_path.clone(),
                                        version1: dep.version.clone(),
                                        scope2: other_scope_path.clone(),
                                        version2: other_dep.version.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // Build dependency graph
            for (scope_path, locked_scope) in &lock.scopes {
                let mut deps = Vec::new();
                for dep_name in locked_scope.dependencies.keys() {
                    deps.push(dep_name.clone());
                }
                conflict_info
                    .dependency_graph
                    .insert(scope_path.clone(), deps);
            }

            return Ok(conflict_info);
        }

        Ok(ConflictPreventionInfo::default())
    }

    /// Get lock file health information for AI agents
    pub async fn get_lock_file_health(&self) -> RhemaResult<LockFileHealthInfo> {
        let lock_file = self.get_lock_file().await?;

        if let Some(lock) = lock_file {
            let mut health_info = LockFileHealthInfo {
                is_valid: lock.metadata.validation_status
                    == rhema_core::schema::ValidationStatus::Valid,
                validation_status: format!("{:?}", lock.metadata.validation_status),
                validation_messages: lock
                    .metadata
                    .validation_messages
                    .clone()
                    .unwrap_or_default(),
                last_validated: lock.metadata.last_validated.clone(),
                performance_metrics: lock.metadata.performance_metrics.clone(),
                health_score: 0.0,
                issues: Vec::new(),
                recommendations: Vec::new(),
            };

            // Calculate health score
            let mut score = 100.0;

            // Deduct points for validation issues
            if lock.metadata.validation_status != rhema_core::schema::ValidationStatus::Valid {
                score -= 30.0;
                health_info
                    .issues
                    .push("Lock file validation failed".to_string());
            }

            // Deduct points for circular dependencies
            if lock.metadata.circular_dependencies > 0 {
                score -= 20.0 * lock.metadata.circular_dependencies as f64;
                health_info.issues.push(format!(
                    "{} circular dependencies detected",
                    lock.metadata.circular_dependencies
                ));
            }

            // Deduct points for performance issues
            if let Some(metrics) = &lock.metadata.performance_metrics {
                if metrics.generation_time_ms > 5000 {
                    score -= 10.0;
                    health_info
                        .issues
                        .push("Lock file generation took too long".to_string());
                }

                // Calculate cache hit rate manually
                let total_cache_operations = metrics.cache_hits + metrics.cache_misses;
                if total_cache_operations > 0 {
                    let hit_rate = metrics.cache_hits as f64 / total_cache_operations as f64;
                    if hit_rate < 0.5 {
                        score -= 5.0;
                        health_info
                            .issues
                            .push("Low cache hit rate detected".to_string());
                    }
                }
            }

            health_info.health_score = score.max(0.0);

            // Generate recommendations
            if lock.metadata.circular_dependencies > 0 {
                health_info
                    .recommendations
                    .push("Review and resolve circular dependencies".to_string());
            }

            if lock.metadata.validation_status != rhema_core::schema::ValidationStatus::Valid {
                health_info
                    .recommendations
                    .push("Fix validation issues in lock file".to_string());
            }

            if health_info.health_score < 70.0 {
                health_info
                    .recommendations
                    .push("Consider regenerating lock file".to_string());
            }

            return Ok(health_info);
        }

        Ok(LockFileHealthInfo::default())
    }

    /// Get context-aware dependency recommendations for AI agents
    pub async fn get_dependency_recommendations(
        &self,
        scope_path: &str,
    ) -> RhemaResult<Vec<DependencyRecommendation>> {
        let lock_context = self.get_scope_lock_context(scope_path).await?;
        let health_info = self.get_lock_file_health().await?;
        let conflict_info = self.get_conflict_prevention_info().await?;

        let mut recommendations = Vec::new();

        if let Some(context) = lock_context {
            // Check for outdated dependencies
            for (dep_name, dep) in &context.dependencies {
                // This is a simplified check - in a real implementation, you'd check against latest versions
                if dep.version.clone().expect("REASON").starts_with("0.") {
                    recommendations.push(DependencyRecommendation {
                        dependency_name: dep_name.clone(),
                        current_version: dep.version.clone().expect("REASON"),
                        recommended_version: "1.0.0".to_string(), // Placeholder
                        reason: "Dependency is in pre-release version".to_string(),
                        priority: RecommendationPriority::Medium,
                        impact: "Consider upgrading to stable version".to_string(),
                    });
                }
            }
        }

        // Add recommendations based on health issues
        for issue in &health_info.issues {
            if issue.contains("circular dependencies") {
                recommendations.push(DependencyRecommendation {
                    dependency_name: "circular_deps".to_string(),
                    current_version: "".to_string(),
                    recommended_version: "".to_string(),
                    reason: "Circular dependencies detected".to_string(),
                    priority: RecommendationPriority::High,
                    impact: "May cause build issues and maintenance problems".to_string(),
                });
            }
        }

        // Add recommendations based on conflicts
        for conflict in &conflict_info.potential_conflicts {
            recommendations.push(DependencyRecommendation {
                dependency_name: conflict.dependency_name.clone(),
                current_version: conflict.version1.clone(),
                recommended_version: conflict.version2.clone(),
                reason: "Version conflict detected".to_string(),
                priority: RecommendationPriority::High,
                impact: "May cause runtime issues".to_string(),
            });
        }

        Ok(recommendations)
    }

    /// Execute a query with lock file context
    pub async fn execute_query(&self, query: &str) -> RhemaResult<serde_json::Value> {
        let result = rhema_query::query::execute_query(&self.repo_root, query)?;
        Ok(serde_json::to_value(result)?)
    }

    /// Execute a query with statistics including lock file info
    pub async fn execute_query_with_stats(
        &self,
        query: &str,
    ) -> RhemaResult<(Value, HashMap<String, Value>)> {
        let result = rhema_query::query::execute_query(&self.repo_root, query)?;
        let mut stats = HashMap::new();

        // Add lock file statistics
        if let Some(lock_file) = self.get_lock_file().await? {
            stats.insert(
                "lock_file_scopes".to_string(),
                serde_json::to_value(lock_file.metadata.total_scopes)?,
            );
            stats.insert(
                "lock_file_dependencies".to_string(),
                serde_json::to_value(lock_file.metadata.total_dependencies)?,
            );
            stats.insert(
                "lock_file_validation_status".to_string(),
                serde_json::to_value(format!("{:?}", lock_file.metadata.validation_status))?,
            );
        }

        Ok((serde_json::to_value(result)?, stats))
    }

    /// Search with regex pattern
    pub async fn search_regex(
        &self,
        _pattern: &str,
        _file_filter: Option<&str>,
    ) -> RhemaResult<Vec<QueryResult>> {
        // Implementation would go here
        Ok(Vec::new())
    }

    /// Get comprehensive context statistics including lock file info
    pub async fn get_stats(&self) -> RhemaResult<ContextStats> {
        let scopes = self.scopes.read().await;
        let knowledge = self.knowledge_cache.read().await;
        let todos = self.todos_cache.read().await;
        let decisions = self.decisions_cache.read().await;
        let patterns = self.patterns_cache.read().await;
        let conventions = self.conventions_cache.read().await;
        let lock_file = self.lock_file_cache.read().await;

        let mut stats = ContextStats {
            scopes_count: scopes.len(),
            knowledge_entries_count: knowledge.values().map(|k| k.entries.len()).sum(),
            todos_count: todos.values().map(|t| t.todos.len()).sum(),
            decisions_count: decisions.values().map(|d| d.decisions.len()).sum(),
            patterns_count: patterns.values().map(|p| p.patterns.len()).sum(),
            conventions_count: conventions.values().map(|c| c.conventions.len()).sum(),
            last_updated: Utc::now(),
            lock_file_stats: None,
        };

        // Add lock file statistics if available
        if let Some(lock) = lock_file.as_ref() {
            stats.lock_file_stats = Some(LockFileContextStats {
                total_scopes: lock.metadata.total_scopes as usize,
                total_dependencies: lock.metadata.total_dependencies as usize,
                circular_dependencies: lock.metadata.circular_dependencies as usize,
                validation_status: format!("{:?}", lock.metadata.validation_status),
                health_score: self.get_lock_file_health().await?.health_score,
            });
        }

        Ok(stats)
    }

    /// Get changes since a specific timestamp
    pub async fn get_changes_since(
        &self,
        since: chrono::DateTime<Utc>,
    ) -> RhemaResult<Vec<ContextChange>> {
        let mut changes = Vec::new();
        
        // Track changes in knowledge entries
        let knowledge_cache = self.knowledge_cache.read().await;
        for (scope_path, knowledge) in knowledge_cache.iter() {
            for entry in &knowledge.entries {
                // Check if entry was created after the since timestamp
                if entry.created_at > since {
                    changes.push(ContextChange {
                        scope_path: scope_path.clone(),
                        change_type: ChangeType::Created,
                        resource_type: ResourceType::Knowledge,
                        resource_id: Some(entry.id.clone()),
                        timestamp: entry.created_at,
                        details: Some(serde_json::json!({
                            "title": entry.title,
                            "content_length": entry.content.len(),
                            "tags": entry.tags
                        })),
                    });
                }
                
                // Check if entry was updated after the since timestamp
                if let Some(updated_at) = entry.updated_at {
                    if updated_at > since {
                        changes.push(ContextChange {
                            scope_path: scope_path.clone(),
                            change_type: ChangeType::Updated,
                            resource_type: ResourceType::Knowledge,
                            resource_id: Some(entry.id.clone()),
                            timestamp: updated_at,
                            details: Some(serde_json::json!({
                                "title": entry.title,
                                "content_length": entry.content.len(),
                                "tags": entry.tags
                            })),
                        });
                    }
                }
            }
        }
        drop(knowledge_cache);
        
        // Track changes in todos
        let todos_cache = self.todos_cache.read().await;
        for (scope_path, todos) in todos_cache.iter() {
            for todo in &todos.todos {
                // Check if todo was created after the since timestamp
                if todo.created_at > since {
                    changes.push(ContextChange {
                        scope_path: scope_path.clone(),
                        change_type: ChangeType::Created,
                        resource_type: ResourceType::Todo,
                        resource_id: Some(todo.id.clone()),
                        timestamp: todo.created_at,
                        details: Some(serde_json::json!({
                            "title": todo.title,
                            "status": todo.status,
                            "priority": todo.priority
                        })),
                    });
                }
                
                // Note: TodoEntry doesn't have updated_at field, only created_at and completed_at
                
                // Check if todo was completed after the since timestamp
                if let Some(completed_at) = todo.completed_at {
                    if completed_at > since {
                        changes.push(ContextChange {
                            scope_path: scope_path.clone(),
                            change_type: ChangeType::Updated,
                            resource_type: ResourceType::Todo,
                            resource_id: Some(todo.id.clone()),
                            timestamp: completed_at,
                            details: Some(serde_json::json!({
                                "title": todo.title,
                                "status": "Completed",
                                "completion_type": "completed_at"
                            })),
                        });
                    }
                }
            }
        }
        drop(todos_cache);
        
        // Track changes in decisions
        let decisions_cache = self.decisions_cache.read().await;
        for (scope_path, decisions) in decisions_cache.iter() {
            for decision in &decisions.decisions {
                // Check if decision was decided after the since timestamp
                if decision.decided_at > since {
                    changes.push(ContextChange {
                        scope_path: scope_path.clone(),
                        change_type: ChangeType::Created,
                        resource_type: ResourceType::Decision,
                        resource_id: Some(decision.id.clone()),
                        timestamp: decision.decided_at,
                        details: Some(serde_json::json!({
                            "title": decision.title,
                            "status": decision.status,
                            "context": decision.context
                        })),
                    });
                }
                
                // Check if decision was reviewed after the since timestamp
                if let Some(review_date) = decision.review_date {
                    if review_date > since {
                        changes.push(ContextChange {
                            scope_path: scope_path.clone(),
                            change_type: ChangeType::Updated,
                            resource_type: ResourceType::Decision,
                            resource_id: Some(decision.id.clone()),
                            timestamp: review_date,
                            details: Some(serde_json::json!({
                                "title": decision.title,
                                "status": decision.status,
                                "review_type": "review_date"
                            })),
                        });
                    }
                }
            }
        }
        drop(decisions_cache);
        
        // Track changes in patterns
        let patterns_cache = self.patterns_cache.read().await;
        for (scope_path, patterns) in patterns_cache.iter() {
            for pattern in &patterns.patterns {
                // Check if pattern was created after the since timestamp
                if pattern.created_at > since {
                    changes.push(ContextChange {
                        scope_path: scope_path.clone(),
                        change_type: ChangeType::Created,
                        resource_type: ResourceType::Pattern,
                        resource_id: Some(pattern.id.clone()),
                        timestamp: pattern.created_at,
                        details: Some(serde_json::json!({
                            "name": pattern.name,
                            "pattern_type": pattern.pattern_type,
                            "usage": format!("{:?}", pattern.usage)
                        })),
                    });
                }
                
                // Check if pattern was updated after the since timestamp
                if let Some(updated_at) = pattern.updated_at {
                    if updated_at > since {
                        changes.push(ContextChange {
                            scope_path: scope_path.clone(),
                            change_type: ChangeType::Updated,
                            resource_type: ResourceType::Pattern,
                            resource_id: Some(pattern.id.clone()),
                            timestamp: updated_at,
                            details: Some(serde_json::json!({
                                "name": pattern.name,
                                "pattern_type": pattern.pattern_type,
                                "usage": format!("{:?}", pattern.usage)
                            })),
                        });
                    }
                }
            }
        }
        drop(patterns_cache);
        
        // Track changes in conventions
        let conventions_cache = self.conventions_cache.read().await;
        for (scope_path, conventions) in conventions_cache.iter() {
            for convention in &conventions.conventions {
                // Check if convention was created after the since timestamp
                if convention.created_at > since {
                    changes.push(ContextChange {
                        scope_path: scope_path.clone(),
                        change_type: ChangeType::Created,
                        resource_type: ResourceType::Convention,
                        resource_id: Some(convention.id.clone()),
                        timestamp: convention.created_at,
                        details: Some(serde_json::json!({
                            "name": convention.name,
                            "convention_type": convention.convention_type,
                            "enforcement": format!("{:?}", convention.enforcement)
                        })),
                    });
                }
                
                // Check if convention was updated after the since timestamp
                if let Some(updated_at) = convention.updated_at {
                    if updated_at > since {
                        changes.push(ContextChange {
                            scope_path: scope_path.clone(),
                            change_type: ChangeType::Updated,
                            resource_type: ResourceType::Convention,
                            resource_id: Some(convention.id.clone()),
                            timestamp: updated_at,
                            details: Some(serde_json::json!({
                                "name": convention.name,
                                "convention_type": convention.convention_type,
                                "enforcement": format!("{:?}", convention.enforcement)
                            })),
                        });
                    }
                }
            }
        }
        drop(conventions_cache);
        
        // Track changes in lock file
        let lock_file_cache = self.lock_file_cache.read().await;
        if let Some(lock_file) = lock_file_cache.as_ref() {
            if lock_file.generated_at > since {
                changes.push(ContextChange {
                    scope_path: "lock_file".to_string(),
                    change_type: ChangeType::Created,
                    resource_type: ResourceType::LockFile,
                    resource_id: None,
                    timestamp: lock_file.generated_at,
                    details: Some(serde_json::json!({
                        "total_scopes": lock_file.metadata.total_scopes,
                        "total_dependencies": lock_file.metadata.total_dependencies,
                        "validation_status": format!("{:?}", lock_file.metadata.validation_status),
                        "generated_by": lock_file.generated_by
                    })),
                });
            }
            
            // Check if lock file was last validated after the since timestamp
            if let Some(last_validated) = lock_file.metadata.last_validated {
                if last_validated > since {
                    changes.push(ContextChange {
                        scope_path: "lock_file".to_string(),
                        change_type: ChangeType::Updated,
                        resource_type: ResourceType::LockFile,
                        resource_id: None,
                        timestamp: last_validated,
                        details: Some(serde_json::json!({
                            "total_scopes": lock_file.metadata.total_scopes,
                            "total_dependencies": lock_file.metadata.total_dependencies,
                            "validation_status": format!("{:?}", lock_file.metadata.validation_status),
                            "update_type": "validation"
                        })),
                    });
                }
            }
        }
        drop(lock_file_cache);
        
        // Sort changes by timestamp (oldest first)
        changes.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        Ok(changes)
    }

    /// Get changes in the last specified duration
    pub async fn get_recent_changes(
        &self,
        duration: chrono::Duration,
    ) -> RhemaResult<Vec<ContextChange>> {
        let since = Utc::now() - duration;
        self.get_changes_since(since).await
    }

    /// Get changes in the last hour
    pub async fn get_changes_last_hour(&self) -> RhemaResult<Vec<ContextChange>> {
        self.get_recent_changes(chrono::Duration::hours(1)).await
    }

    /// Get changes in the last day
    pub async fn get_changes_last_day(&self) -> RhemaResult<Vec<ContextChange>> {
        self.get_recent_changes(chrono::Duration::days(1)).await
    }

    /// Get changes in the last week
    pub async fn get_changes_last_week(&self) -> RhemaResult<Vec<ContextChange>> {
        self.get_recent_changes(chrono::Duration::weeks(1)).await
    }

    // ============================================================================
    // CONTEXT DATA INTEGRITY VALIDATION METHODS
    // ============================================================================

    /// Validate all loaded context data comprehensively
    pub async fn validate_context_data(&self) -> RhemaResult<ContextValidationResult> {
        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut scope_results = HashMap::new();
        let mut total_entries = 0;

        // Get all scopes
        let scopes = self.get_scopes().await?;

        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();
            let scope_result = self.validate_scope_context(&scope_path).await?;

            total_entries += scope_result.errors.len() + scope_result.warnings.len();
            errors.extend(scope_result.errors.clone());
            warnings.extend(scope_result.warnings.clone());
            scope_results.insert(scope_path.to_string(), scope_result);
        }

        // Validate cross-references
        let cross_ref_validation = self.validate_cross_references().await?;
        if !cross_ref_validation.is_valid {
            for broken_ref in &cross_ref_validation.broken_references {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::CrossReferenceError,
                    scope_path: Some(broken_ref.source_scope.clone()),
                    resource_type: Some(broken_ref.source_resource.clone()),
                    resource_id: Some(broken_ref.source_id.clone()),
                    message: format!(
                        "Broken reference to {} in {}",
                        broken_ref.referenced_id, broken_ref.referenced_resource
                    ),
                    severity: ValidationSeverity::Error,
                    field_path: Some("references".to_string()),
                    suggested_fix: Some("Update or remove the broken reference".to_string()),
                });
            }
        }

        // Validate consistency
        let consistency_validation = self.validate_consistency().await?;
        if !consistency_validation.is_consistent {
            for conflict in &consistency_validation.naming_conflicts {
                warnings.push(ValidationWarning {
                    warning_type: "naming_conflict".to_string(),
                    scope_path: None,
                    resource_type: None,
                    resource_id: None,
                    message: format!("Naming conflict: {}", conflict.conflict_type),
                    field_path: None,
                    recommendation: Some("Consider standardizing naming conventions".to_string()),
                });
            }
        }

        // Validate temporal consistency
        let temporal_validation = self.validate_temporal_consistency().await?;
        if !temporal_validation.is_temporally_consistent {
            for anomaly in &temporal_validation.future_dates_in_past {
                warnings.push(ValidationWarning {
                    warning_type: "temporal_anomaly".to_string(),
                    scope_path: Some(anomaly.scope_path.clone()),
                    resource_type: Some(anomaly.resource_type.clone()),
                    resource_id: Some(anomaly.entry_id.clone()),
                    message: format!("Future date in past entry: {}", anomaly.field_name),
                    field_path: Some(anomaly.field_name.clone()),
                    recommendation: Some("Check and correct the timestamp".to_string()),
                });
            }
        }

        let validation_time = start_time.elapsed().as_millis() as u64;
        let is_valid = errors.is_empty();
        let validation_score = if total_entries > 0 {
            1.0 - (errors.len() as f64 / total_entries as f64)
        } else {
            1.0
        };

        let stats = ValidationStats {
            total_entries_validated: total_entries,
            errors_count: errors.len(),
            warnings_count: warnings.len(),
            validation_time_ms: validation_time,
            memory_usage_bytes: self.estimate_memory_usage().await,
            validation_score,
        };

        let recommendations = self
            .generate_validation_recommendations(&errors, &warnings)
            .await;

        Ok(ContextValidationResult {
            is_valid,
            validation_errors: errors,
            validation_warnings: warnings,
            validation_stats: stats,
            recommendations,
            validated_at: Utc::now(),
            scope_validation_results: scope_results,
        })
    }

    /// Validate context for a specific scope
    pub async fn validate_scope_context(
        &self,
        scope_path: &str,
    ) -> RhemaResult<ScopeValidationResult> {
        let mut errors = Vec::new();
        let warnings = Vec::new();
        let mut knowledge_valid = true;
        let mut todos_valid = true;
        let mut decisions_valid = true;
        let mut patterns_valid = true;
        let mut conventions_valid = true;

        // Validate knowledge
        if let Some(_knowledge) = self.get_knowledge(scope_path).await? {
            match self.validate_knowledge(scope_path).await {
                Ok(_) => {}
                Err(e) => {
                    knowledge_valid = false;
                    errors.push(ValidationError {
                        error_type: ValidationErrorType::SchemaViolation,
                        scope_path: Some(scope_path.to_string()),
                        resource_type: Some("knowledge".to_string()),
                        resource_id: None,
                        message: format!("Knowledge validation failed: {}", e),
                        severity: ValidationSeverity::Error,
                        field_path: None,
                        suggested_fix: Some("Check knowledge schema compliance".to_string()),
                    });
                }
            }
        }

        // Validate todos
        if let Some(_todos) = self.get_todos(scope_path).await? {
            match self.validate_todos(scope_path).await {
                Ok(_) => {}
                Err(e) => {
                    todos_valid = false;
                    errors.push(ValidationError {
                        error_type: ValidationErrorType::SchemaViolation,
                        scope_path: Some(scope_path.to_string()),
                        resource_type: Some("todos".to_string()),
                        resource_id: None,
                        message: format!("Todos validation failed: {}", e),
                        severity: ValidationSeverity::Error,
                        field_path: None,
                        suggested_fix: Some("Check todos schema compliance".to_string()),
                    });
                }
            }
        }

        // Validate decisions
        if let Some(_decisions) = self.get_decisions(scope_path).await? {
            match self.validate_decisions(scope_path).await {
                Ok(_) => {}
                Err(e) => {
                    decisions_valid = false;
                    errors.push(ValidationError {
                        error_type: ValidationErrorType::SchemaViolation,
                        scope_path: Some(scope_path.to_string()),
                        resource_type: Some("decisions".to_string()),
                        resource_id: None,
                        message: format!("Decisions validation failed: {}", e),
                        severity: ValidationSeverity::Error,
                        field_path: None,
                        suggested_fix: Some("Check decisions schema compliance".to_string()),
                    });
                }
            }
        }

        // Validate patterns
        if let Some(_patterns) = self.get_patterns(scope_path).await? {
            match self.validate_patterns(scope_path).await {
                Ok(_) => {}
                Err(e) => {
                    patterns_valid = false;
                    errors.push(ValidationError {
                        error_type: ValidationErrorType::SchemaViolation,
                        scope_path: Some(scope_path.to_string()),
                        resource_type: Some("patterns".to_string()),
                        resource_id: None,
                        message: format!("Patterns validation failed: {}", e),
                        severity: ValidationSeverity::Error,
                        field_path: None,
                        suggested_fix: Some("Check patterns schema compliance".to_string()),
                    });
                }
            }
        }

        // Validate conventions
        if let Some(_conventions) = self.get_conventions(scope_path).await? {
            match self.validate_conventions(scope_path).await {
                Ok(_) => {}
                Err(e) => {
                    conventions_valid = false;
                    errors.push(ValidationError {
                        error_type: ValidationErrorType::SchemaViolation,
                        scope_path: Some(scope_path.to_string()),
                        resource_type: Some("conventions".to_string()),
                        resource_id: None,
                        message: format!("Conventions validation failed: {}", e),
                        severity: ValidationSeverity::Error,
                        field_path: None,
                        suggested_fix: Some("Check conventions schema compliance".to_string()),
                    });
                }
            }
        }

        let is_valid = errors.is_empty();

        Ok(ScopeValidationResult {
            scope_path: scope_path.to_string(),
            is_valid,
            errors,
            warnings,
            knowledge_valid,
            todos_valid,
            decisions_valid,
            patterns_valid,
            conventions_valid,
        })
    }

    /// Validate knowledge entries for a scope
    pub async fn validate_knowledge(&self, scope_path: &str) -> RhemaResult<()> {
        if let Some(knowledge) = self.get_knowledge(scope_path).await? {
            // Use the core schema validation
            knowledge.validate()?;
            knowledge.validate_schema_version()?;
            knowledge.validate_cross_fields()?;

            // Additional knowledge-specific validation
            for entry in &knowledge.entries {
                if entry.id.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Knowledge entry ID cannot be empty".to_string(),
                    ));
                }

                if entry.title.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Knowledge entry title cannot be empty".to_string(),
                    ));
                }

                if entry.content.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Knowledge entry content cannot be empty".to_string(),
                    ));
                }

                // Validate confidence level if present
                if let Some(confidence) = entry.confidence {
                    if confidence > 10 {
                        return Err(RhemaError::ValidationError(
                            "Knowledge confidence level must be between 1-10".to_string(),
                        ));
                    }
                }

                // Validate timestamps
                if let Some(updated_at) = entry.updated_at {
                    if updated_at < entry.created_at {
                        return Err(RhemaError::ValidationError(
                            "Knowledge entry updated_at cannot be before created_at".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate todos for a scope
    pub async fn validate_todos(&self, scope_path: &str) -> RhemaResult<()> {
        if let Some(todos) = self.get_todos(scope_path).await? {
            // Use the core schema validation
            todos.validate()?;
            todos.validate_schema_version()?;
            todos.validate_cross_fields()?;

            // Additional todo-specific validation
            for todo in &todos.todos {
                if todo.id.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Todo ID cannot be empty".to_string(),
                    ));
                }

                if todo.title.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Todo title cannot be empty".to_string(),
                    ));
                }

                // Validate status transitions
                if let Some(completed_at) = todo.completed_at {
                    if todo.status != TodoStatus::Completed && todo.status != TodoStatus::Cancelled
                    {
                        return Err(RhemaError::ValidationError(
                            "Todo with completion timestamp must have Completed or Cancelled status".to_string(),
                        ));
                    }

                    if completed_at < todo.created_at {
                        return Err(RhemaError::ValidationError(
                            "Todo completion time cannot be before creation time".to_string(),
                        ));
                    }
                }

                // Validate due date
                if let Some(due_date) = todo.due_date {
                    if due_date < todo.created_at {
                        return Err(RhemaError::ValidationError(
                            "Todo due date cannot be before creation date".to_string(),
                        ));
                    }
                }

                // Validate related knowledge references
                if let Some(related_knowledge) = &todo.related_knowledge {
                    for knowledge_id in related_knowledge {
                        if knowledge_id.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Related knowledge ID cannot be empty".to_string(),
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate decisions for a scope
    pub async fn validate_decisions(&self, scope_path: &str) -> RhemaResult<()> {
        if let Some(decisions) = self.get_decisions(scope_path).await? {
            // Use the core schema validation
            decisions.validate()?;
            decisions.validate_schema_version()?;
            decisions.validate_cross_fields()?;

            // Additional decision-specific validation
            for decision in &decisions.decisions {
                if decision.id.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Decision ID cannot be empty".to_string(),
                    ));
                }

                if decision.title.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Decision title cannot be empty".to_string(),
                    ));
                }

                if decision.description.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Decision description cannot be empty".to_string(),
                    ));
                }

                // Validate review date
                if let Some(review_date) = decision.review_date {
                    if review_date < decision.decided_at {
                        return Err(RhemaError::ValidationError(
                            "Decision review date cannot be before decision date".to_string(),
                        ));
                    }
                }

                // Validate alternatives
                if let Some(alternatives) = &decision.alternatives {
                    for alternative in alternatives {
                        if alternative.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Decision alternative cannot be empty".to_string(),
                            ));
                        }
                    }
                }

                // Validate consequences
                if let Some(consequences) = &decision.consequences {
                    for consequence in consequences {
                        if consequence.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Decision consequence cannot be empty".to_string(),
                            ));
                        }
                    }
                }

                // Validate decision makers
                if let Some(decision_makers) = &decision.decision_makers {
                    for maker in decision_makers {
                        if maker.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Decision maker cannot be empty".to_string(),
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate patterns for a scope
    pub async fn validate_patterns(&self, scope_path: &str) -> RhemaResult<()> {
        if let Some(patterns) = self.get_patterns(scope_path).await? {
            // Use the core schema validation
            patterns.validate()?;
            patterns.validate_schema_version()?;
            patterns.validate_cross_fields()?;

            // Additional pattern-specific validation
            for pattern in &patterns.patterns {
                if pattern.id.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Pattern ID cannot be empty".to_string(),
                    ));
                }

                if pattern.name.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Pattern name cannot be empty".to_string(),
                    ));
                }

                if pattern.description.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Pattern description cannot be empty".to_string(),
                    ));
                }

                if pattern.pattern_type.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Pattern type cannot be empty".to_string(),
                    ));
                }

                // Validate effectiveness rating
                if let Some(effectiveness) = pattern.effectiveness {
                    if effectiveness > 10 {
                        return Err(RhemaError::ValidationError(
                            "Pattern effectiveness must be between 1-10".to_string(),
                        ));
                    }
                }

                // Validate examples
                if let Some(examples) = &pattern.examples {
                    for example in examples {
                        if example.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Pattern example cannot be empty".to_string(),
                            ));
                        }
                    }
                }

                // Validate anti-patterns
                if let Some(anti_patterns) = &pattern.anti_patterns {
                    for anti_pattern in anti_patterns {
                        if anti_pattern.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Anti-pattern cannot be empty".to_string(),
                            ));
                        }
                    }
                }

                // Validate related patterns
                if let Some(related_patterns) = &pattern.related_patterns {
                    for related_pattern in related_patterns {
                        if related_pattern.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Related pattern cannot be empty".to_string(),
                            ));
                        }
                    }
                }

                // Validate timestamps
                if let Some(updated_at) = pattern.updated_at {
                    if updated_at < pattern.created_at {
                        return Err(RhemaError::ValidationError(
                            "Pattern updated_at cannot be before created_at".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate conventions for a scope
    pub async fn validate_conventions(&self, scope_path: &str) -> RhemaResult<()> {
        if let Some(conventions) = self.get_conventions(scope_path).await? {
            // Use the core schema validation
            conventions.validate()?;
            conventions.validate_schema_version()?;
            conventions.validate_cross_fields()?;

            // Additional convention-specific validation
            for convention in &conventions.conventions {
                if convention.id.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Convention ID cannot be empty".to_string(),
                    ));
                }

                if convention.name.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Convention name cannot be empty".to_string(),
                    ));
                }

                if convention.description.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Convention description cannot be empty".to_string(),
                    ));
                }

                if convention.convention_type.trim().is_empty() {
                    return Err(RhemaError::ValidationError(
                        "Convention type cannot be empty".to_string(),
                    ));
                }

                // Validate examples
                if let Some(examples) = &convention.examples {
                    for example in examples {
                        if example.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Convention example cannot be empty".to_string(),
                            ));
                        }
                    }
                }

                // Validate tools
                if let Some(tools) = &convention.tools {
                    for tool in tools {
                        if tool.trim().is_empty() {
                            return Err(RhemaError::ValidationError(
                                "Convention tool cannot be empty".to_string(),
                            ));
                        }
                    }
                }

                // Validate timestamps
                if let Some(updated_at) = convention.updated_at {
                    if updated_at < convention.created_at {
                        return Err(RhemaError::ValidationError(
                            "Convention updated_at cannot be before created_at".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate cross-references between context types
    pub async fn validate_cross_references(&self) -> RhemaResult<CrossReferenceValidation> {
        let mut broken_references = Vec::new();
        let orphaned_entries = Vec::new();
        let circular_references = Vec::new();

        let scopes = self.get_scopes().await?;

        // Build a map of all valid IDs across all scopes
        let mut valid_ids = HashMap::new();

        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();

            // Collect knowledge IDs
            if let Some(knowledge) = self.get_knowledge(&scope_path).await? {
                for entry in &knowledge.entries {
                    valid_ids.insert(entry.id.clone(), ("knowledge", scope_path.to_string()));
                }
            }

            // Collect todo IDs
            if let Some(todos) = self.get_todos(&scope_path).await? {
                for todo in &todos.todos {
                    valid_ids.insert(todo.id.clone(), ("todos", scope_path.to_string()));
                }
            }

            // Collect decision IDs
            if let Some(decisions) = self.get_decisions(&scope_path).await? {
                for decision in &decisions.decisions {
                    valid_ids.insert(decision.id.clone(), ("decisions", scope_path.to_string()));
                }
            }

            // Collect pattern IDs
            if let Some(patterns) = self.get_patterns(&scope_path).await? {
                for pattern in &patterns.patterns {
                    valid_ids.insert(pattern.id.clone(), ("patterns", scope_path.to_string()));
                }
            }

            // Collect convention IDs
            if let Some(conventions) = self.get_conventions(&scope_path).await? {
                for convention in &conventions.conventions {
                    valid_ids.insert(
                        convention.id.clone(),
                        ("conventions", scope_path.to_string()),
                    );
                }
            }
        }

        // Check for broken references
        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();

            // Check todo references to knowledge
            if let Some(todos) = self.get_todos(&scope_path).await? {
                for todo in &todos.todos {
                    if let Some(related_knowledge) = &todo.related_knowledge {
                        for knowledge_id in related_knowledge {
                            if !valid_ids.contains_key(knowledge_id) {
                                broken_references.push(BrokenReference {
                                    source_scope: scope_path.to_string(),
                                    source_resource: "todos".to_string(),
                                    source_id: todo.id.clone(),
                                    referenced_scope: "unknown".to_string(),
                                    referenced_resource: "knowledge".to_string(),
                                    referenced_id: knowledge_id.clone(),
                                    reference_type: "related_knowledge".to_string(),
                                });
                            }
                        }
                    }
                }
            }

            // Check pattern references to other patterns
            if let Some(patterns) = self.get_patterns(&scope_path).await? {
                for pattern in &patterns.patterns {
                    if let Some(related_patterns) = &pattern.related_patterns {
                        for related_pattern_id in related_patterns {
                            if !valid_ids.contains_key(related_pattern_id) {
                                broken_references.push(BrokenReference {
                                    source_scope: scope_path.to_string(),
                                    source_resource: "patterns".to_string(),
                                    source_id: pattern.id.clone(),
                                    referenced_scope: "unknown".to_string(),
                                    referenced_resource: "patterns".to_string(),
                                    referenced_id: related_pattern_id.clone(),
                                    reference_type: "related_patterns".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        let is_valid = broken_references.is_empty()
            && orphaned_entries.is_empty()
            && circular_references.is_empty();

        Ok(CrossReferenceValidation {
            is_valid,
            broken_references,
            orphaned_entries,
            circular_references,
        })
    }

    /// Validate consistency across all scopes
    pub async fn validate_consistency(&self) -> RhemaResult<ConsistencyValidation> {
        let mut naming_conflicts = Vec::new();
        let mut duplicate_entries = Vec::new();
        let conflicting_information = Vec::new();

        let scopes = self.get_scopes().await?;

        // Check for naming conflicts across scopes
        let mut scope_names: HashMap<String, String> = HashMap::new();
        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();
            if let Some(existing_scope) = scope_names.get(&scope.definition.name) {
                naming_conflicts.push(NamingConflict {
                    conflict_type: "duplicate_scope_name".to_string(),
                    conflicting_names: vec![scope.definition.name.clone()],
                    affected_scopes: vec![existing_scope.clone(), scope_path.to_string()],
                    severity: ValidationSeverity::Error,
                });
            } else {
                scope_names.insert(scope.definition.name.clone(), scope_path.to_string());
            }
        }

        // Check for duplicate entries within the same resource type
        let mut knowledge_titles: HashMap<String, String> = HashMap::new();
        let mut todo_titles: HashMap<String, String> = HashMap::new();
        let mut decision_titles: HashMap<String, String> = HashMap::new();
        let mut pattern_names: HashMap<String, String> = HashMap::new();
        let mut convention_names: HashMap<String, String> = HashMap::new();

        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();

            // Check knowledge titles
            if let Some(knowledge) = self.get_knowledge(&scope_path).await? {
                for entry in &knowledge.entries {
                    if let Some(existing) = knowledge_titles.get(&entry.title) {
                        duplicate_entries.push(DuplicateEntry {
                            resource_type: "knowledge".to_string(),
                            entry_id: entry.id.clone(),
                            duplicate_ids: vec![existing.clone()],
                            affected_scopes: vec![scope_path.to_string()],
                            similarity_score: 1.0,
                        });
                    } else {
                        knowledge_titles.insert(entry.title.clone(), entry.id.clone());
                    }
                }
            }

            // Check todo titles
            if let Some(todos) = self.get_todos(&scope_path).await? {
                for todo in &todos.todos {
                    if let Some(existing) = todo_titles.get(&todo.title) {
                        duplicate_entries.push(DuplicateEntry {
                            resource_type: "todos".to_string(),
                            entry_id: todo.id.clone(),
                            duplicate_ids: vec![existing.clone()],
                            affected_scopes: vec![scope_path.to_string()],
                            similarity_score: 1.0,
                        });
                    } else {
                        todo_titles.insert(todo.title.clone(), todo.id.clone());
                    }
                }
            }

            // Check decision titles
            if let Some(decisions) = self.get_decisions(&scope_path).await? {
                for decision in &decisions.decisions {
                    if let Some(existing) = decision_titles.get(&decision.title) {
                        duplicate_entries.push(DuplicateEntry {
                            resource_type: "decisions".to_string(),
                            entry_id: decision.id.clone(),
                            duplicate_ids: vec![existing.clone()],
                            affected_scopes: vec![scope_path.to_string()],
                            similarity_score: 1.0,
                        });
                    } else {
                        decision_titles.insert(decision.title.clone(), decision.id.clone());
                    }
                }
            }

            // Check pattern names
            if let Some(patterns) = self.get_patterns(&scope_path).await? {
                for pattern in &patterns.patterns {
                    if let Some(existing) = pattern_names.get(&pattern.name) {
                        duplicate_entries.push(DuplicateEntry {
                            resource_type: "patterns".to_string(),
                            entry_id: pattern.id.clone(),
                            duplicate_ids: vec![existing.clone()],
                            affected_scopes: vec![scope_path.to_string()],
                            similarity_score: 1.0,
                        });
                    } else {
                        pattern_names.insert(pattern.name.clone(), pattern.id.clone());
                    }
                }
            }

            // Check convention names
            if let Some(conventions) = self.get_conventions(&scope_path).await? {
                for convention in &conventions.conventions {
                    if let Some(existing) = convention_names.get(&convention.name) {
                        duplicate_entries.push(DuplicateEntry {
                            resource_type: "conventions".to_string(),
                            entry_id: convention.id.clone(),
                            duplicate_ids: vec![existing.clone()],
                            affected_scopes: vec![scope_path.to_string()],
                            similarity_score: 1.0,
                        });
                    } else {
                        convention_names.insert(convention.name.clone(), convention.id.clone());
                    }
                }
            }
        }

        let is_consistent = naming_conflicts.is_empty()
            && duplicate_entries.is_empty()
            && conflicting_information.is_empty();

        Ok(ConsistencyValidation {
            is_consistent,
            naming_conflicts,
            duplicate_entries,
            conflicting_information,
        })
    }

    /// Validate temporal consistency
    pub async fn validate_temporal_consistency(&self) -> RhemaResult<TemporalValidation> {
        let mut future_dates_in_past = Vec::new();
        let mut invalid_timestamp_sequences = Vec::new();
        let mut expired_entries = Vec::new();
        let current_time = Utc::now();

        let scopes = self.get_scopes().await?;

        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();

            // Check knowledge entries
            if let Some(knowledge) = self.get_knowledge(&scope_path).await? {
                for entry in &knowledge.entries {
                    // Check for future creation dates
                    if entry.created_at > current_time {
                        future_dates_in_past.push(TemporalAnomaly {
                            scope_path: scope_path.to_string(),
                            resource_type: "knowledge".to_string(),
                            entry_id: entry.id.clone(),
                            field_name: "created_at".to_string(),
                            future_date: entry.created_at,
                            current_date: current_time,
                        });
                    }

                    // Check for future update dates
                    if let Some(updated_at) = entry.updated_at {
                        if updated_at > current_time {
                            future_dates_in_past.push(TemporalAnomaly {
                                scope_path: scope_path.to_string(),
                                resource_type: "knowledge".to_string(),
                                entry_id: entry.id.clone(),
                                field_name: "updated_at".to_string(),
                                future_date: updated_at,
                                current_date: current_time,
                            });
                        }

                        // Check sequence
                        if updated_at < entry.created_at {
                            invalid_timestamp_sequences.push(TimestampSequence {
                                scope_path: scope_path.to_string(),
                                resource_type: "knowledge".to_string(),
                                entry_id: entry.id.clone(),
                                earlier_field: "updated_at".to_string(),
                                later_field: "created_at".to_string(),
                                earlier_timestamp: updated_at,
                                later_timestamp: entry.created_at,
                            });
                        }
                    }
                }
            }

            // Check todo entries
            if let Some(todos) = self.get_todos(&scope_path).await? {
                for todo in &todos.todos {
                    // Check for future creation dates
                    if todo.created_at > current_time {
                        future_dates_in_past.push(TemporalAnomaly {
                            scope_path: scope_path.to_string(),
                            resource_type: "todos".to_string(),
                            entry_id: todo.id.clone(),
                            field_name: "created_at".to_string(),
                            future_date: todo.created_at,
                            current_date: current_time,
                        });
                    }

                    // Check due dates
                    if let Some(due_date) = todo.due_date {
                        if due_date < current_time {
                            let days_expired = (current_time - due_date).num_days();
                            expired_entries.push(ExpiredEntry {
                                scope_path: scope_path.to_string(),
                                resource_type: "todos".to_string(),
                                entry_id: todo.id.clone(),
                                expiry_field: "due_date".to_string(),
                                expiry_date: due_date,
                                days_expired,
                            });
                        }
                    }

                    // Check completion dates
                    if let Some(completed_at) = todo.completed_at {
                        if completed_at > current_time {
                            future_dates_in_past.push(TemporalAnomaly {
                                scope_path: scope_path.to_string(),
                                resource_type: "todos".to_string(),
                                entry_id: todo.id.clone(),
                                field_name: "completed_at".to_string(),
                                future_date: completed_at,
                                current_date: current_time,
                            });
                        }

                        // Check sequence
                        if completed_at < todo.created_at {
                            invalid_timestamp_sequences.push(TimestampSequence {
                                scope_path: scope_path.to_string(),
                                resource_type: "todos".to_string(),
                                entry_id: todo.id.clone(),
                                earlier_field: "completed_at".to_string(),
                                later_field: "created_at".to_string(),
                                earlier_timestamp: completed_at,
                                later_timestamp: todo.created_at,
                            });
                        }
                    }
                }
            }

            // Check decision entries
            if let Some(decisions) = self.get_decisions(&scope_path).await? {
                for decision in &decisions.decisions {
                    // Check for future decision dates
                    if decision.decided_at > current_time {
                        future_dates_in_past.push(TemporalAnomaly {
                            scope_path: scope_path.to_string(),
                            resource_type: "decisions".to_string(),
                            entry_id: decision.id.clone(),
                            field_name: "decided_at".to_string(),
                            future_date: decision.decided_at,
                            current_date: current_time,
                        });
                    }

                    // Check review dates
                    if let Some(review_date) = decision.review_date {
                        if review_date > current_time {
                            future_dates_in_past.push(TemporalAnomaly {
                                scope_path: scope_path.to_string(),
                                resource_type: "decisions".to_string(),
                                entry_id: decision.id.clone(),
                                field_name: "review_date".to_string(),
                                future_date: review_date,
                                current_date: current_time,
                            });
                        }

                        // Check sequence
                        if review_date < decision.decided_at {
                            invalid_timestamp_sequences.push(TimestampSequence {
                                scope_path: scope_path.to_string(),
                                resource_type: "decisions".to_string(),
                                entry_id: decision.id.clone(),
                                earlier_field: "review_date".to_string(),
                                later_field: "decided_at".to_string(),
                                earlier_timestamp: review_date,
                                later_timestamp: decision.decided_at,
                            });
                        }
                    }
                }
            }
        }

        let is_temporally_consistent = future_dates_in_past.is_empty()
            && invalid_timestamp_sequences.is_empty()
            && expired_entries.is_empty();

        Ok(TemporalValidation {
            is_temporally_consistent,
            future_dates_in_past,
            invalid_timestamp_sequences,
            expired_entries,
        })
    }

    /// Validate scope dependencies
    pub async fn validate_scope_dependencies(&self) -> RhemaResult<DependencyValidation> {
        let mut circular_dependencies = Vec::new();
        let mut missing_dependencies = Vec::new();
        let mut version_conflicts = Vec::new();
        let unresolved_dependencies = Vec::new();
        let start_time = std::time::Instant::now();

        let scopes = self.get_scopes().await?;

        // Build dependency graph
        let mut dependency_graph = HashMap::new();
        let mut scope_versions = HashMap::new();

        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();
            scope_versions.insert(scope_path.to_string(), scope.definition.version.clone());

            if let Some(deps) = &scope.definition.dependencies {
                let mut dependencies = Vec::new();
                for dep in deps {
                    dependencies.push(dep.path.clone());

                    // Check if dependency exists
                    let dep_exists = scopes.iter().any(|s| s.path.to_string_lossy() == dep.path);
                    if !dep_exists {
                        missing_dependencies.push(format!("{} -> {}", scope_path, dep.path));
                    }

                    // Check version conflicts
                    if let Some(dep_version) = &dep.version {
                        if let Some(existing_version) = scope_versions.get(&dep.path) {
                            if dep_version != existing_version {
                                version_conflicts.push(format!(
                                    "{} requires {} but found {}",
                                    dep.path, dep_version, existing_version
                                ));
                            }
                        }
                    }
                }
                dependency_graph.insert(scope_path.to_string(), dependencies);
            }
        }

        // Check for circular dependencies using DFS
        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();
            if let Some(circular) =
                self.detect_circular_dependencies(&dependency_graph, &scope_path)
            {
                circular_dependencies.push(circular);
            }
        }

        let validation_time = start_time.elapsed().as_millis() as u64;
        let is_valid = circular_dependencies.is_empty()
            && missing_dependencies.is_empty()
            && version_conflicts.is_empty()
            && unresolved_dependencies.is_empty();

        Ok(DependencyValidation {
            is_valid,
            circular_dependencies,
            missing_dependencies,
            version_conflicts,
            unresolved_dependencies,
            validation_time_ms: validation_time,
        })
    }

    // ============================================================================
    // PRIVATE HELPER METHODS
    // ============================================================================

    /// Load scopes from the repository
    async fn load_scopes(&self) -> RhemaResult<()> {
        // TODO: Implement scope loading
        // For now, just create an empty vector
        let mut scopes_guard = self.scopes.write().await;
        *scopes_guard = Vec::new();
        Ok(())
    }

    /// Load all context data
    async fn load_all_context(&self) -> RhemaResult<()> {
        let scopes = self.get_scopes().await?;

        for scope in &scopes {
            let scope_path = scope.path.to_string_lossy();
            self.load_scope_context(&scope_path).await?;
        }

        Ok(())
    }

                    /// Load context data for a specific scope
                async fn load_scope_context(&self, scope_path: &str) -> RhemaResult<()> {
                    // TODO: Implement context loading
                    // For now, just create empty data structures
                    let mut knowledge_cache = self.knowledge_cache.write().await;
                    knowledge_cache.insert(scope_path.to_string(), Knowledge {
                        entries: Vec::new(),
                        categories: None,
                        custom: HashMap::new(),
                    });

                    let mut todos_cache = self.todos_cache.write().await;
                    todos_cache.insert(scope_path.to_string(), Todos {
                        todos: Vec::new(),
                        custom: HashMap::new(),
                    });

                    let mut decisions_cache = self.decisions_cache.write().await;
                    decisions_cache.insert(scope_path.to_string(), Decisions {
                        decisions: Vec::new(),
                        custom: HashMap::new(),
                    });

                    let mut patterns_cache = self.patterns_cache.write().await;
                    patterns_cache.insert(scope_path.to_string(), Patterns {
                        patterns: Vec::new(),
                        custom: HashMap::new(),
                    });

                    let mut conventions_cache = self.conventions_cache.write().await;
                    conventions_cache.insert(scope_path.to_string(), Conventions {
                        conventions: Vec::new(),
                        custom: HashMap::new(),
                    });

                    Ok(())
                }

    /// Load lock file data
    async fn load_lock_file(&self) -> RhemaResult<()> {
        // TODO: Implement lock file loading
        // For now, just create an empty lock file
        let mut lock_cache = self.lock_file_cache.write().await;
        *lock_cache = Some(rhema_core::schema::RhemaLock::new("context_provider"));
        Ok(())
    }

    /// Estimate memory usage of the context provider
    async fn estimate_memory_usage(&self) -> u64 {
        let mut total_bytes = 0u64;

        // Estimate scopes memory
        let scopes = self.scopes.read().await;
        total_bytes += (scopes.len() * 1024) as u64; // Rough estimate per scope

        // Estimate knowledge cache memory
        let knowledge = self.knowledge_cache.read().await;
        for (_, knowledge_data) in knowledge.iter() {
            total_bytes += (knowledge_data.entries.len() * 2048) as u64; // Rough estimate per entry
        }

        // Estimate todos cache memory
        let todos = self.todos_cache.read().await;
        for (_, todos_data) in todos.iter() {
            total_bytes += (todos_data.todos.len() * 1024) as u64; // Rough estimate per todo
        }

        // Estimate decisions cache memory
        let decisions = self.decisions_cache.read().await;
        for (_, decisions_data) in decisions.iter() {
            total_bytes += (decisions_data.decisions.len() * 1536) as u64; // Rough estimate per decision
        }

        // Estimate patterns cache memory
        let patterns = self.patterns_cache.read().await;
        for (_, patterns_data) in patterns.iter() {
            total_bytes += (patterns_data.patterns.len() * 1792) as u64; // Rough estimate per pattern
        }

        // Estimate conventions cache memory
        let conventions = self.conventions_cache.read().await;
        for (_, conventions_data) in conventions.iter() {
            total_bytes += (conventions_data.conventions.len() * 1280) as u64; // Rough estimate per convention
        }

        total_bytes
    }

    /// Generate validation recommendations based on errors and warnings
    async fn generate_validation_recommendations(
        &self,
        errors: &[ValidationError],
        warnings: &[ValidationWarning],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Count error types
        let mut error_counts = HashMap::new();
        for error in errors {
            *error_counts.entry(&error.error_type).or_insert(0) += 1;
        }

        // Generate recommendations based on error patterns
        if let Some(&count) = error_counts.get(&ValidationErrorType::SchemaViolation) {
            if count > 0 {
                recommendations.push(format!(
                    "Fix {} schema validation errors by ensuring all data conforms to the expected schema format",
                    count
                ));
            }
        }

        if let Some(&count) = error_counts.get(&ValidationErrorType::CrossReferenceError) {
            if count > 0 {
                recommendations.push(format!(
                    "Fix {} broken cross-references by updating or removing invalid references",
                    count
                ));
            }
        }

        if let Some(&count) = error_counts.get(&ValidationErrorType::TemporalError) {
            if count > 0 {
                recommendations.push(format!(
                    "Fix {} temporal inconsistencies by correcting timestamp sequences",
                    count
                ));
            }
        }

        if let Some(&count) = error_counts.get(&ValidationErrorType::DuplicateEntry) {
            if count > 0 {
                recommendations.push(format!(
                    "Resolve {} duplicate entries by merging or removing duplicates",
                    count
                ));
            }
        }

        // Add general recommendations
        if !errors.is_empty() {
            recommendations.push("Run validation regularly to catch issues early".to_string());
            recommendations.push(
                "Consider implementing automated validation in your CI/CD pipeline".to_string(),
            );
        }

        if !warnings.is_empty() {
            recommendations.push("Review warnings to improve data quality".to_string());
        }

        recommendations
    }

    /// Detect circular dependencies using DFS
    fn detect_circular_dependencies(
        &self,
        graph: &HashMap<String, Vec<String>>,
        start: &str,
    ) -> Option<String> {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        let mut path = Vec::new();

        if self.dfs_has_cycle(graph, start, &mut visited, &mut rec_stack, &mut path) {
            Some(path.join(" -> "))
        } else {
            None
        }
    }

    /// DFS helper for cycle detection
    fn dfs_has_cycle(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if self.dfs_has_cycle(graph, neighbor, visited, rec_stack, path) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        false
    }
}
