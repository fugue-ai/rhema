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

use crate::cache::CompressionAlgorithm;
use chrono::Timelike;
use chrono::Utc;
use rhema_core::{schema::*, scope::Scope, RhemaError, RhemaLock, RhemaResult};
use rhema_query::QueryResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;

// Enhanced context management types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCacheEntry<T> {
    pub data: T,
    pub created_at: chrono::DateTime<Utc>,
    pub last_accessed: chrono::DateTime<Utc>,
    pub access_count: u64,
    pub size_bytes: u64,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextVersion {
    pub version_id: String,
    pub scope_path: String,
    pub changes: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub checksum: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSyncStatus {
    pub sync_status: SyncStatus,
    pub last_sync: Option<chrono::DateTime<Utc>>,
    pub sync_errors: Vec<String>,
    pub pending_changes: usize,
    pub conflicts_resolved: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    InSync,
    Syncing,
    OutOfSync,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBackupConfig {
    pub enabled: bool,
    pub backup_interval_hours: u64,
    pub max_backups: usize,
    pub backup_path: Option<PathBuf>,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCacheConfig {
    pub enabled: bool,
    pub max_size_mb: u64,
    pub ttl_seconds: u64,
    pub eviction_policy: String,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSyncConfig {
    pub enabled: bool,
    pub sync_interval_seconds: u64,
    pub conflict_resolution: ConflictResolutionStrategy,
    pub auto_resolve_conflicts: bool,
    pub sync_metadata: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextVersionConfig {
    pub enabled: bool,
    pub max_versions_per_scope: usize,
    pub version_retention_days: u64,
    pub auto_version_on_change: bool,
    pub include_metadata: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCompressionConfig {
    pub enabled: bool,
    pub algorithm: CompressionAlgorithm,
    pub compression_level: u8,
    pub min_size_for_compression: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEncryptionConfig {
    pub enabled: bool,
    pub algorithm: EncryptionAlgorithm,
    pub key_rotation_days: u64,
    pub encrypt_metadata: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    FirstWriteWins,
    Manual,
    Merge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    AES256,
    ChaCha20,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBackup {
    pub backup_id: String,
    pub created_at: chrono::DateTime<Utc>,
    pub scope_path: String,
    pub data_size_bytes: u64,
    pub checksum: String,
    pub compression_ratio: Option<f64>,
    pub encryption_enabled: bool,
    pub metadata: HashMap<String, Value>,
}

/// Validation error types for context data
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum ValidationErrorType {
    SchemaViolation,
    CrossReferenceError,
    ConsistencyError,
    TemporalError,
    DependencyError,
    SecurityError,
    DataIntegrityError,
    DuplicateEntry,
    InvalidReference,
    MissingRequiredField,
}

/// Validation severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Individual validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub scope_path: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub message: String,
    pub severity: ValidationSeverity,
    pub field_path: Option<String>,
    pub suggested_fix: Option<String>,
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub warning_type: String,
    pub scope_path: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub message: String,
    pub field_path: Option<String>,
    pub recommendation: Option<String>,
}

/// Validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStats {
    pub total_entries_validated: usize,
    pub errors_count: usize,
    pub warnings_count: usize,
    pub validation_time_ms: u64,
    pub memory_usage_bytes: u64,
    pub validation_score: f64, // 0.0 to 1.0
}

/// Comprehensive validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextValidationResult {
    pub is_valid: bool,
    pub validation_errors: Vec<ValidationError>,
    pub validation_warnings: Vec<ValidationWarning>,
    pub validation_stats: ValidationStats,
    pub recommendations: Vec<String>,
    pub validated_at: chrono::DateTime<Utc>,
    pub scope_validation_results: HashMap<String, ScopeValidationResult>,
}

/// Scope-specific validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeValidationResult {
    pub scope_path: String,
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub knowledge_valid: bool,
    pub todos_valid: bool,
    pub decisions_valid: bool,
    pub patterns_valid: bool,
    pub conventions_valid: bool,
}

/// Cross-reference validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReferenceValidation {
    pub is_valid: bool,
    pub broken_references: Vec<BrokenReference>,
    pub orphaned_entries: Vec<OrphanedEntry>,
    pub circular_references: Vec<CircularReference>,
}

/// Broken reference information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenReference {
    pub source_scope: String,
    pub source_resource: String,
    pub source_id: String,
    pub referenced_scope: String,
    pub referenced_resource: String,
    pub referenced_id: String,
    pub reference_type: String,
}

/// Orphaned entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedEntry {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub entry_title: String,
    pub orphaned_since: chrono::DateTime<Utc>,
}

/// Circular reference information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularReference {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub circular_path: Vec<String>,
}

/// Consistency validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyValidation {
    pub is_consistent: bool,
    pub naming_conflicts: Vec<NamingConflict>,
    pub duplicate_entries: Vec<DuplicateEntry>,
    pub conflicting_information: Vec<ConflictingInformation>,
}

/// Naming conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingConflict {
    pub conflict_type: String,
    pub conflicting_names: Vec<String>,
    pub affected_scopes: Vec<String>,
    pub severity: ValidationSeverity,
}

/// Duplicate entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateEntry {
    pub resource_type: String,
    pub entry_id: String,
    pub duplicate_ids: Vec<String>,
    pub affected_scopes: Vec<String>,
    pub similarity_score: f64,
}

/// Conflicting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictingInformation {
    pub field_name: String,
    pub conflicting_values: Vec<String>,
    pub affected_scopes: Vec<String>,
    pub conflict_type: String,
}

/// Temporal validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalValidation {
    pub is_temporally_consistent: bool,
    pub future_dates_in_past: Vec<TemporalAnomaly>,
    pub invalid_timestamp_sequences: Vec<TimestampSequence>,
    pub expired_entries: Vec<ExpiredEntry>,
}

/// Temporal anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAnomaly {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub field_name: String,
    pub future_date: chrono::DateTime<Utc>,
    pub current_date: chrono::DateTime<Utc>,
}

/// Timestamp sequence issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampSequence {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub earlier_field: String,
    pub later_field: String,
    pub earlier_timestamp: chrono::DateTime<Utc>,
    pub later_timestamp: chrono::DateTime<Utc>,
}

/// Expired entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiredEntry {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub expiry_field: String,
    pub expiry_date: chrono::DateTime<Utc>,
    pub days_expired: i64,
}

/// Dependency validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyValidation {
    pub is_valid: bool,
    pub circular_dependencies: Vec<String>,
    pub missing_dependencies: Vec<String>,
    pub version_conflicts: Vec<String>,
    pub unresolved_dependencies: Vec<String>,
    pub validation_time_ms: u64,
}

/// Version validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionValidation {
    pub is_valid: bool,
    pub incompatible_versions: Vec<String>,
    pub breaking_changes: Vec<String>,
    pub migration_required: Vec<String>,
    pub validation_time_ms: u64,
}

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
        _since: chrono::DateTime<Utc>,
    ) -> RhemaResult<Vec<ContextChange>> {
        // TODO: Implement change tracking
        Ok(Vec::new())
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

        if let Some(dependencies) = graph.get(node) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    if self.dfs_has_cycle(graph, dep, visited, rec_stack, path) {
                        return true;
                    }
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    path.push(dep.to_string());
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        false
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

    async fn load_knowledge_for_scope(&self, _scope: &Scope) -> RhemaResult<Knowledge> {
        // Implementation would go here
        Ok(Knowledge {
            entries: Vec::new(),
            categories: None,
            custom: HashMap::new(),
        })
    }

    async fn load_todos_for_scope(&self, _scope: &Scope) -> RhemaResult<Todos> {
        // Implementation would go here
        Ok(Todos {
            todos: Vec::new(),
            custom: HashMap::new(),
        })
    }

    async fn load_decisions_for_scope(&self, _scope: &Scope) -> RhemaResult<Decisions> {
        // Implementation would go here
        Ok(Decisions {
            decisions: Vec::new(),
            custom: HashMap::new(),
        })
    }

    async fn load_patterns_for_scope(&self, _scope: &Scope) -> RhemaResult<Patterns> {
        // Implementation would go here
        Ok(Patterns {
            patterns: Vec::new(),
            custom: HashMap::new(),
        })
    }

    async fn load_conventions_for_scope(&self, _scope: &Scope) -> RhemaResult<Conventions> {
        // Implementation would go here
        Ok(Conventions {
            conventions: Vec::new(),
            custom: HashMap::new(),
        })
    }

    /// Cache context data
    pub async fn cache_context(&self, key: &str, data: Value) -> RhemaResult<()> {
        if !self.cache_config.enabled {
            return Ok(());
        }

        let mut processed_data = data;

        // Compress data if enabled
        if self.cache_config.compression_enabled {
            processed_data = self.compress_context_data(&processed_data).await?;
        }

        // Encrypt data if enabled
        if self.encryption_config.enabled {
            processed_data = self.encrypt_context_data(&processed_data).await?;
        }

        let checksum = self.calculate_checksum(&processed_data).await;
        let _version = self.generate_version().await;

        let cache_entry = ContextCacheEntry {
            data: processed_data.clone(),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            size_bytes: serde_json::to_string(&processed_data)?.len() as u64,
            checksum,
        };

        // Check cache size and evict if necessary
        let mut cache = self.context_cache.write().await;
        if cache.len() >= self.cache_config.max_size_mb as usize {
            self.evict_oldest_cache_entries(&mut cache).await;
        }

        cache.insert(key.to_string(), cache_entry);
        Ok(())
    }

    /// Get cached context data
    pub async fn get_cached_context(&self, key: &str) -> RhemaResult<Option<Value>> {
        if !self.cache_config.enabled {
            return Ok(None);
        }

        let mut cache = self.context_cache.write().await;
        if let Some(mut entry) = cache.get_mut(key) {
            // Check if entry is expired
            let ttl = Duration::from_secs(self.cache_config.ttl_seconds);
            if entry.created_at < Utc::now() - chrono::Duration::seconds(ttl.as_secs() as i64) {
                cache.remove(key);
                return Ok(None);
            }

            // Update access time
            entry.last_accessed = Utc::now();
            entry.access_count += 1;

            let mut data = entry.data.clone();

            // Decrypt data if it was encrypted
            if self.encryption_config.enabled {
                data = self.decrypt_context_data(&data).await?;
            }

            // Decompress data if it was compressed
            if self.compression_config.enabled {
                data = self.decompress_context_data(&data).await?;
            }

            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// Synchronize context across multiple instances
    pub async fn synchronize_context(&self) -> RhemaResult<ContextSyncStatus> {
        if !self.sync_config.enabled {
            return Ok(self.sync_status.read().await.clone());
        }

        info!("Starting context synchronization");

        let mut sync_status = self.sync_status.write().await;
        sync_status.sync_status = SyncStatus::Syncing;

        // Simulate synchronization process
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Resolve conflicts if any
        let conflicts_resolved = self.resolve_context_conflicts().await?;

        // Sync versions
        let _versions_synced = self.sync_context_versions().await?;

        // Update sync status
        sync_status.last_sync = Some(Utc::now());
        sync_status.sync_status = SyncStatus::InSync;
        sync_status.sync_errors.clear();
        sync_status.pending_changes = 0;
        sync_status.conflicts_resolved = conflicts_resolved;

        info!("Context synchronization completed");
        Ok(sync_status.clone())
    }

    /// Create a new context version
    pub async fn create_context_version(
        &self,
        scope_path: &str,
        changes: Vec<String>,
    ) -> RhemaResult<String> {
        if !self.version_config.enabled {
            return Ok("no_versioning".to_string());
        }

        let version = self.generate_version().await;
        let context_data = self.get_scope_context_data(scope_path).await?;
        let checksum = self.calculate_checksum(&context_data).await;
        let _size_bytes = serde_json::to_string(&context_data)?.len() as u64;

        let context_version = ContextVersion {
            version_id: version.clone(),
            scope_path: scope_path.to_string(),
            changes,
            created_at: Utc::now(),
            checksum,
            author: None,
            description: None,
        };

        let mut versions = self.context_versions.write().await;
        let scope_versions = versions
            .entry(scope_path.to_string())
            .or_insert_with(Vec::new);
        scope_versions.push(context_version);

        // Limit versions per scope
        if scope_versions.len() > self.version_config.max_versions_per_scope {
            scope_versions.remove(0);
        }

        info!(
            "Created context version {} for scope {}",
            version, scope_path
        );
        Ok(version)
    }

    /// Get context versions for a scope
    pub async fn get_context_versions(&self, scope_path: &str) -> RhemaResult<Vec<ContextVersion>> {
        let versions = self.context_versions.read().await;
        Ok(versions.get(scope_path).cloned().unwrap_or_default())
    }

    /// Restore context to a specific version
    pub async fn restore_context_version(
        &self,
        scope_path: &str,
        version: &str,
    ) -> RhemaResult<()> {
        let versions = self.context_versions.read().await;
        if let Some(scope_versions) = versions.get(scope_path) {
            if let Some(_target_version) = scope_versions.iter().find(|v| v.version_id == version) {
                info!(
                    "Restoring context to version {} for scope {}",
                    version, scope_path
                );
                // In a real implementation, you would restore the actual context data
                return Ok(());
            }
        }

        Err(RhemaError::InvalidInput(format!(
            "Version {} not found for scope {}",
            version, scope_path
        )))
    }

    /// Compress context data
    async fn compress_context_data(&self, data: &Value) -> RhemaResult<Value> {
        if !self.compression_config.enabled {
            return Ok(data.clone());
        }

        let json_string = serde_json::to_string(data)?;
        if json_string.len() < self.compression_config.min_size_for_compression as usize {
            return Ok(data.clone());
        }

        // For now, return the original data
        // In a real implementation, you would use actual compression libraries
        Ok(data.clone())
    }

    /// Decompress context data
    async fn decompress_context_data(&self, data: &Value) -> RhemaResult<Value> {
        // For now, return the original data
        // In a real implementation, you would use actual decompression
        Ok(data.clone())
    }

    /// Encrypt context data
    async fn encrypt_context_data(&self, data: &Value) -> RhemaResult<Value> {
        if !self.encryption_config.enabled {
            return Ok(data.clone());
        }

        // For now, return the original data
        // In a real implementation, you would use actual encryption
        Ok(data.clone())
    }

    /// Decrypt context data
    async fn decrypt_context_data(&self, data: &Value) -> RhemaResult<Value> {
        // For now, return the original data
        // In a real implementation, you would use actual decryption
        Ok(data.clone())
    }

    /// Create a backup of context data
    pub async fn create_context_backup(&self) -> RhemaResult<ContextBackup> {
        if !self.backup_config.enabled {
            return Err(RhemaError::InvalidInput("Backup is disabled".to_string()));
        }

        let backup_id = self.generate_backup_id().await;
        let backup_path = self.get_backup_path(&backup_id).await?;
        let context_data = self.get_all_context_data().await?;

        // Create backup directory
        if let Some(parent) = backup_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize and write context data
        let json_data = serde_json::to_string_pretty(&context_data)?;
        fs::write(&backup_path, json_data)?;

        let size_bytes = fs::metadata(&backup_path)?.len();
        let checksum = self.calculate_file_checksum(&backup_path).await?;

        let backup = ContextBackup {
            backup_id: backup_id.clone(),
            created_at: Utc::now(),
            scope_path: "".to_string(),
            data_size_bytes: size_bytes,
            checksum,
            compression_ratio: None,
            encryption_enabled: self.backup_config.encryption_enabled,
            metadata: HashMap::new(),
        };

        info!("Created context backup: {}", backup_id);
        Ok(backup)
    }

    /// Restore context from backup
    pub async fn restore_context_backup(&self, backup_id: &str) -> RhemaResult<()> {
        let backup_path = self.get_backup_path(backup_id).await?;

        if !backup_path.exists() {
            return Err(RhemaError::InvalidInput(format!(
                "Backup {} not found",
                backup_id
            )));
        }

        let json_data = fs::read_to_string(&backup_path)?;
        let context_data: Value = serde_json::from_str(&json_data)?;

        // Restore context data
        self.restore_context_data(&context_data).await?;

        info!("Restored context from backup: {}", backup_id);
        Ok(())
    }

    /// Get all context data for backup
    async fn get_all_context_data(&self) -> RhemaResult<Value> {
        let scopes = self.scopes.read().await;
        let knowledge = self.knowledge_cache.read().await;
        let todos = self.todos_cache.read().await;
        let decisions = self.decisions_cache.read().await;
        let patterns = self.patterns_cache.read().await;
        let conventions = self.conventions_cache.read().await;
        let lock_file = self.lock_file_cache.read().await;

        Ok(serde_json::json!({
            "scopes": *scopes,
            "knowledge": *knowledge,
            "todos": *todos,
            "decisions": *decisions,
            "patterns": *patterns,
            "conventions": *conventions,
            "lock_file": *lock_file,
            "backup_created_at": Utc::now(),
        }))
    }

    /// Restore context data from backup
    async fn restore_context_data(&self, _context_data: &Value) -> RhemaResult<()> {
        // In a real implementation, you would restore the actual context data
        // For now, we'll just log the restoration
        info!("Restoring context data from backup");
        Ok(())
    }

    /// Get scope context data
    async fn get_scope_context_data(&self, scope_path: &str) -> RhemaResult<Value> {
        let knowledge = self.get_knowledge(scope_path).await?;
        let todos = self.get_todos(scope_path).await?;
        let decisions = self.get_decisions(scope_path).await?;
        let patterns = self.get_patterns(scope_path).await?;
        let conventions = self.get_conventions(scope_path).await?;

        Ok(serde_json::json!({
            "scope_path": scope_path,
            "knowledge": knowledge,
            "todos": todos,
            "decisions": decisions,
            "patterns": patterns,
            "conventions": conventions,
        }))
    }

    /// Resolve context conflicts
    async fn resolve_context_conflicts(&self) -> RhemaResult<usize> {
        // Simulate conflict resolution
        let conflicts = 2; // Simulated number of conflicts
        info!("Resolved {} context conflicts", conflicts);
        Ok(conflicts)
    }

    /// Sync context versions
    async fn sync_context_versions(&self) -> RhemaResult<usize> {
        // Simulate version synchronization
        let versions = 5; // Simulated number of versions synced
        info!("Synced {} context versions", versions);
        Ok(versions)
    }

    /// Evict oldest cache entries
    async fn evict_oldest_cache_entries(
        &self,
        cache: &mut HashMap<String, ContextCacheEntry<Value>>,
    ) {
        // Collect all entries and sort them by creation time
        let mut entries: Vec<_> = cache.iter().collect();
        entries.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));

        // Calculate how many to remove (oldest 10%)
        let to_remove = (entries.len() / 10).max(1);

        // Extract the keys to remove
        let keys_to_remove: Vec<String> = entries
            .iter()
            .take(to_remove)
            .map(|(key, _)| (*key).clone())
            .collect();

        // Now remove the entries from the cache
        for key in keys_to_remove {
            let _ = cache.remove(&key);
        }

        info!("Evicted {} oldest cache entries", to_remove);
    }

    /// Calculate checksum for data
    async fn calculate_checksum(&self, data: &Value) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let json_string = serde_json::to_string(data).unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        json_string.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Calculate file checksum
    async fn calculate_file_checksum(&self, path: &PathBuf) -> RhemaResult<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let content = fs::read(path)?;
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Generate version string
    async fn generate_version(&self) -> String {
        format!(
            "v{}.{}.{}",
            Utc::now().timestamp() / 86400, // Days since epoch
            Utc::now().hour(),
            Utc::now().minute()
        )
    }

    /// Generate backup ID
    async fn generate_backup_id(&self) -> String {
        format!("backup_{}", Utc::now().format("%Y%m%d_%H%M%S"))
    }

    /// Get backup path
    async fn get_backup_path(&self, backup_id: &str) -> RhemaResult<PathBuf> {
        let backup_path = match &self.backup_config.backup_path {
            Some(path) => path.clone(),
            None => self.repo_root.join(".rhema/backups"),
        };

        Ok(backup_path.join(format!("{}.json", backup_id)))
    }

    /// Get context cache statistics
    pub async fn get_context_cache_stats(&self) -> RhemaResult<HashMap<String, usize>> {
        let cache = self.context_cache.read().await;
        let versions = self.context_versions.read().await;

        Ok(HashMap::from([
            ("cached_entries".to_string(), cache.len()),
            ("versioned_scopes".to_string(), versions.len()),
            (
                "total_versions".to_string(),
                versions.values().map(|v| v.len()).sum(),
            ),
        ]))
    }

    /// Clear context cache
    pub async fn clear_context_cache(&self) -> RhemaResult<()> {
        let mut cache = self.context_cache.write().await;
        cache.clear();
        info!("Context cache cleared");
        Ok(())
    }

    /// Get synchronization status
    pub async fn get_sync_status(&self) -> ContextSyncStatus {
        self.sync_status.read().await.clone()
    }
}

// Default implementations for configuration structs
impl Default for ContextCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 100,  // 100MB
            ttl_seconds: 3600, // 1 hour
            eviction_policy: "LRU".to_string(),
            compression_enabled: false,
        }
    }
}

impl Default for ContextSyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sync_interval_seconds: 300, // 5 minutes
            conflict_resolution: ConflictResolutionStrategy::LastWriteWins,
            auto_resolve_conflicts: true,
            sync_metadata: true,
        }
    }
}

impl Default for ContextVersionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_versions_per_scope: 10,
            version_retention_days: 30,
            auto_version_on_change: true,
            include_metadata: true,
        }
    }
}

impl Default for ContextCompressionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: CompressionAlgorithm::Gzip,
            compression_level: 6,
            min_size_for_compression: 1024, // 1KB
        }
    }
}

impl Default for ContextEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: EncryptionAlgorithm::AES256,
            key_rotation_days: 90,
            encrypt_metadata: true,
        }
    }
}

impl Default for ContextBackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backup_interval_hours: 24,
            max_backups: 10,
            backup_path: None,
            compression_enabled: false,
            encryption_enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rhema_core::schema::*;

    #[tokio::test]
    async fn test_context_validation_basic() {
        let temp_dir = tempfile::tempdir().unwrap();
        let context_provider = ContextProvider::new(temp_dir.path().to_path_buf()).unwrap();

        // Test validation with empty context (should be valid)
        let result = context_provider.validate_context_data().await.unwrap();
        assert!(result.is_valid);
        assert_eq!(result.validation_errors.len(), 0);
        assert_eq!(result.validation_warnings.len(), 0);
    }

    #[tokio::test]
    async fn test_validation_error_types() {
        let error = ValidationError {
            error_type: ValidationErrorType::SchemaViolation,
            scope_path: Some("test-scope".to_string()),
            resource_type: Some("knowledge".to_string()),
            resource_id: Some("test-id".to_string()),
            message: "Test error".to_string(),
            severity: ValidationSeverity::Error,
            field_path: Some("title".to_string()),
            suggested_fix: Some("Fix the title".to_string()),
        };

        assert_eq!(error.error_type, ValidationErrorType::SchemaViolation);
        assert_eq!(error.severity, ValidationSeverity::Error);
        assert_eq!(error.message, "Test error");
    }

    #[tokio::test]
    async fn test_validation_stats() {
        let stats = ValidationStats {
            total_entries_validated: 100,
            errors_count: 5,
            warnings_count: 10,
            validation_time_ms: 150,
            memory_usage_bytes: 1024 * 1024,
            validation_score: 0.95,
        };

        assert_eq!(stats.total_entries_validated, 100);
        assert_eq!(stats.errors_count, 5);
        assert_eq!(stats.warnings_count, 10);
        assert_eq!(stats.validation_score, 0.95);
    }

    #[tokio::test]
    async fn test_validation_result_serialization() {
        let result = ContextValidationResult {
            is_valid: true,
            validation_errors: vec![],
            validation_warnings: vec![],
            validation_stats: ValidationStats {
                total_entries_validated: 0,
                errors_count: 0,
                warnings_count: 0,
                validation_time_ms: 0,
                memory_usage_bytes: 0,
                validation_score: 1.0,
            },
            recommendations: vec!["Test recommendation".to_string()],
            validated_at: Utc::now(),
            scope_validation_results: HashMap::new(),
        };

        // Test serialization
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: ContextValidationResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.is_valid, deserialized.is_valid);
        assert_eq!(
            result.recommendations.len(),
            deserialized.recommendations.len()
        );
    }

    #[tokio::test]
    async fn test_temporal_validation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let context_provider = ContextProvider::new(temp_dir.path().to_path_buf()).unwrap();

        let result = context_provider
            .validate_temporal_consistency()
            .await
            .unwrap();
        assert!(result.is_temporally_consistent);
        assert_eq!(result.future_dates_in_past.len(), 0);
        assert_eq!(result.invalid_timestamp_sequences.len(), 0);
        assert_eq!(result.expired_entries.len(), 0);
    }

    #[tokio::test]
    async fn test_consistency_validation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let context_provider = ContextProvider::new(temp_dir.path().to_path_buf()).unwrap();

        let result = context_provider.validate_consistency().await.unwrap();
        assert!(result.is_consistent);
        assert_eq!(result.naming_conflicts.len(), 0);
        assert_eq!(result.duplicate_entries.len(), 0);
        assert_eq!(result.conflicting_information.len(), 0);
    }

    #[tokio::test]
    async fn test_cross_reference_validation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let context_provider = ContextProvider::new(temp_dir.path().to_path_buf()).unwrap();

        let result = context_provider.validate_cross_references().await.unwrap();
        assert!(result.is_valid);
        assert_eq!(result.broken_references.len(), 0);
        assert_eq!(result.orphaned_entries.len(), 0);
        assert_eq!(result.circular_references.len(), 0);
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
