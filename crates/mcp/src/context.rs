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
use rhema_core::{schema::*, scope::Scope, RhemaError, RhemaResult, RhemaLock};
use rhema_query::{execute_query, QueryResult};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

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
        })
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
    pub async fn get_scope_lock_context(&self, scope_path: &str) -> RhemaResult<Option<LockScopeContext>> {
        let lock_file = self.get_lock_file().await?;
        
        if let Some(lock) = lock_file {
            if let Some(locked_scope) = lock.scopes.get(scope_path) {
                return Ok(Some(LockScopeContext {
                    scope_path: scope_path.to_string(),
                    version: locked_scope.version.clone(),
                    dependencies: locked_scope.dependencies.clone(),
                    has_circular_dependencies: locked_scope.has_circular_dependencies,
                    resolved_at: locked_scope.resolved_at.clone(),
                    source_checksum: locked_scope.source_checksum.clone(),
                }));
            }
        }
        
        Ok(None)
    }

    /// Get dependency version information for AI agents
    pub async fn get_dependency_versions(&self, scope_path: &str) -> RhemaResult<Vec<DependencyVersionInfo>> {
        let lock_context = self.get_scope_lock_context(scope_path).await?;
        
        if let Some(context) = lock_context {
            let mut dependency_info = Vec::new();
            
            for (dep_name, dep) in &context.dependencies {
                dependency_info.push(DependencyVersionInfo {
                    name: dep_name.clone(),
                    version: dep.version.clone(),
                    path: dep.path.clone(),
                    dependency_type: dep.dependency_type.clone(),
                    is_transitive: dep.is_transitive,
                    original_constraint: dep.original_constraint.clone(),
                    resolved_at: dep.resolved_at.clone(),
                    checksum: dep.checksum.clone(),
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
                conflict_info.dependency_graph.insert(scope_path.clone(), deps);
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
                is_valid: lock.metadata.validation_status == rhema_core::schema::ValidationStatus::Valid,
                validation_status: format!("{:?}", lock.metadata.validation_status),
                validation_messages: lock.metadata.validation_messages.clone().unwrap_or_default(),
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
                health_info.issues.push("Lock file validation failed".to_string());
            }
            
            // Deduct points for circular dependencies
            if lock.metadata.circular_dependencies > 0 {
                score -= 20.0 * lock.metadata.circular_dependencies as f64;
                health_info.issues.push(format!("{} circular dependencies detected", lock.metadata.circular_dependencies));
            }
            
            // Deduct points for performance issues
            if let Some(metrics) = &lock.metadata.performance_metrics {
                if metrics.generation_time_ms > 5000 {
                    score -= 10.0;
                    health_info.issues.push("Lock file generation took too long".to_string());
                }
                
                // Calculate cache hit rate manually
                let total_cache_operations = metrics.cache_hits + metrics.cache_misses;
                if total_cache_operations > 0 {
                    let hit_rate = metrics.cache_hits as f64 / total_cache_operations as f64;
                    if hit_rate < 0.5 {
                        score -= 5.0;
                        health_info.issues.push("Low cache hit rate detected".to_string());
                    }
                }
            }
            
            health_info.health_score = score.max(0.0);

            // Generate recommendations
            if lock.metadata.circular_dependencies > 0 {
                health_info.recommendations.push("Review and resolve circular dependencies".to_string());
            }
            
            if lock.metadata.validation_status != rhema_core::schema::ValidationStatus::Valid {
                health_info.recommendations.push("Fix validation issues in lock file".to_string());
            }
            
            if health_info.health_score < 70.0 {
                health_info.recommendations.push("Consider regenerating lock file".to_string());
            }

            return Ok(health_info);
        }
        
        Ok(LockFileHealthInfo::default())
    }

    /// Get context-aware dependency recommendations for AI agents
    pub async fn get_dependency_recommendations(&self, scope_path: &str) -> RhemaResult<Vec<DependencyRecommendation>> {
        let lock_context = self.get_scope_lock_context(scope_path).await?;
        let health_info = self.get_lock_file_health().await?;
        let conflict_info = self.get_conflict_prevention_info().await?;
        
        let mut recommendations = Vec::new();
        
        if let Some(context) = lock_context {
            // Check for outdated dependencies
            for (dep_name, dep) in &context.dependencies {
                // This is a simplified check - in a real implementation, you'd check against latest versions
                if dep.version.starts_with("0.") {
                    recommendations.push(DependencyRecommendation {
                        dependency_name: dep_name.clone(),
                        current_version: dep.version.clone(),
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
            stats.insert("lock_file_scopes".to_string(), serde_json::to_value(lock_file.metadata.total_scopes)?);
            stats.insert("lock_file_dependencies".to_string(), serde_json::to_value(lock_file.metadata.total_dependencies)?);
            stats.insert("lock_file_validation_status".to_string(), serde_json::to_value(format!("{:?}", lock_file.metadata.validation_status))?);
        }
        
        Ok((serde_json::to_value(result)?, stats))
    }

    /// Search with regex pattern
    pub async fn search_regex(
        &self,
        pattern: &str,
        file_filter: Option<&str>,
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

    /// Get changes since a specific time
    pub async fn get_changes_since(
        &self,
        _since: chrono::DateTime<Utc>,
    ) -> RhemaResult<Vec<ContextChange>> {
        // Implementation would go here
        Ok(Vec::new())
    }

    async fn load_scopes(&self) -> RhemaResult<()> {
        // Implementation would go here
        Ok(())
    }

    async fn load_all_context(&self) -> RhemaResult<()> {
        // Implementation would go here
        Ok(())
    }

    async fn load_lock_file(&self) -> RhemaResult<()> {
        let lock_file_path = self.repo_root.join("rhema.lock");
        
        if lock_file_path.exists() {
            match rhema_core::lock::LockFileOps::read_lock_file(&lock_file_path) {
                Ok(lock_file) => {
                    let mut cache = self.lock_file_cache.write().await;
                    *cache = Some(lock_file);
                    tracing::info!("Lock file loaded successfully");
                }
                Err(e) => {
                    tracing::warn!("Failed to load lock file: {}", e);
                }
            }
        } else {
            tracing::info!("No lock file found at {:?}", lock_file_path);
        }
        
        Ok(())
    }

    async fn load_knowledge_for_scope(&self, scope: &Scope) -> RhemaResult<Knowledge> {
        // Implementation would go here
        Ok(Knowledge {
            entries: Vec::new(),
            categories: None,
            custom: HashMap::new(),
        })
    }

    async fn load_todos_for_scope(&self, scope: &Scope) -> RhemaResult<Todos> {
        // Implementation would go here
        Ok(Todos {
            todos: Vec::new(),
            custom: HashMap::new(),
        })
    }

    async fn load_decisions_for_scope(&self, scope: &Scope) -> RhemaResult<Decisions> {
        // Implementation would go here
        Ok(Decisions {
            decisions: Vec::new(),
            custom: HashMap::new(),
        })
    }

    async fn load_patterns_for_scope(&self, scope: &Scope) -> RhemaResult<Patterns> {
        // Implementation would go here
        Ok(Patterns {
            patterns: Vec::new(),
            custom: HashMap::new(),
        })
    }

    async fn load_conventions_for_scope(&self, scope: &Scope) -> RhemaResult<Conventions> {
        // Implementation would go here
        Ok(Conventions {
            conventions: Vec::new(),
            custom: HashMap::new(),
        })
    }
}

/// Lock file context for a specific scope
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LockScopeContext {
    pub scope_path: String,
    pub version: String,
    pub dependencies: HashMap<String, rhema_core::LockedDependency>,
    pub has_circular_dependencies: bool,
    pub resolved_at: chrono::DateTime<Utc>,
    pub source_checksum: Option<String>,
}

/// Dependency version information for AI agents
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyVersionInfo {
    pub name: String,
    pub version: String,
    pub path: String,
    pub dependency_type: rhema_core::DependencyType,
    pub is_transitive: bool,
    pub original_constraint: Option<String>,
    pub resolved_at: chrono::DateTime<Utc>,
    pub checksum: String,
}

/// Conflict prevention information for AI agents
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConflictPreventionInfo {
    pub total_scopes: usize,
    pub total_dependencies: usize,
    pub circular_dependencies: usize,
    pub validation_status: String,
    pub resolution_strategy: String,
    pub conflict_resolution: String,
    pub potential_conflicts: Vec<VersionConflict>,
    pub dependency_graph: HashMap<String, Vec<String>>,
}

impl Default for ConflictPreventionInfo {
    fn default() -> Self {
        Self {
            total_scopes: 0,
            total_dependencies: 0,
            circular_dependencies: 0,
            validation_status: "unknown".to_string(),
            resolution_strategy: "unknown".to_string(),
            conflict_resolution: "unknown".to_string(),
            potential_conflicts: Vec::new(),
            dependency_graph: HashMap::new(),
        }
    }
}

/// Version conflict information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionConflict {
    pub dependency_name: String,
    pub scope1: String,
    pub version1: String,
    pub scope2: String,
    pub version2: String,
}

/// Lock file health information for AI agents
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LockFileHealthInfo {
    pub is_valid: bool,
    pub validation_status: String,
    pub validation_messages: Vec<String>,
    pub last_validated: Option<chrono::DateTime<Utc>>,
    pub performance_metrics: Option<rhema_core::LockPerformanceMetrics>,
    pub health_score: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

impl Default for LockFileHealthInfo {
    fn default() -> Self {
        Self {
            is_valid: false,
            validation_status: "unknown".to_string(),
            validation_messages: Vec::new(),
            last_validated: None,
            performance_metrics: None,
            health_score: 0.0,
            issues: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

/// Dependency recommendation for AI agents
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyRecommendation {
    pub dependency_name: String,
    pub current_version: String,
    pub recommended_version: String,
    pub reason: String,
    pub priority: RecommendationPriority,
    pub impact: String,
}

/// Recommendation priority levels
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
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
    pub lock_file_stats: Option<LockFileContextStats>,
}

/// Lock file context statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LockFileContextStats {
    pub total_scopes: usize,
    pub total_dependencies: usize,
    pub circular_dependencies: usize,
    pub validation_status: String,
    pub health_score: f64,
}

/// Context change information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextChange {
    pub scope_path: String,
    pub change_type: ChangeType,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub timestamp: chrono::DateTime<Utc>,
    pub details: Option<serde_json::Value>,
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
    LockFile,
}
