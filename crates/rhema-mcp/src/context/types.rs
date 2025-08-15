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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

// Enhanced context management types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCacheEntry<T> {
    pub data: T,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
    pub size_bytes: u64,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextVersion {
    pub version_id: String,
    pub scope_path: String,
    pub changes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub checksum: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSyncStatus {
    pub sync_status: SyncStatus,
    pub last_sync: Option<DateTime<Utc>>,
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

impl Default for ContextBackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backup_interval_hours: 24,
            max_backups: 10,
            backup_path: None,
            compression_enabled: true,
            encryption_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCacheConfig {
    pub enabled: bool,
    pub max_size_mb: u64,
    pub ttl_seconds: u64,
    pub eviction_policy: String,
    pub compression_enabled: bool,
}

impl Default for ContextCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 100,
            ttl_seconds: 3600,
            eviction_policy: "lru".to_string(),
            compression_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSyncConfig {
    pub enabled: bool,
    pub sync_interval_seconds: u64,
    pub conflict_resolution: ConflictResolutionStrategy,
    pub auto_resolve_conflicts: bool,
    pub sync_metadata: bool,
}

impl Default for ContextSyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sync_interval_seconds: 300,
            conflict_resolution: ConflictResolutionStrategy::LastWriteWins,
            auto_resolve_conflicts: true,
            sync_metadata: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextVersionConfig {
    pub enabled: bool,
    pub max_versions_per_scope: usize,
    pub version_retention_days: u64,
    pub auto_version_on_change: bool,
    pub include_metadata: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCompressionConfig {
    pub enabled: bool,
    pub algorithm: crate::cache::CompressionAlgorithm,
    pub compression_level: u8,
    pub min_size_for_compression: u64,
}

impl Default for ContextCompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: crate::cache::CompressionAlgorithm::Gzip,
            compression_level: 6,
            min_size_for_compression: 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEncryptionConfig {
    pub enabled: bool,
    pub algorithm: EncryptionAlgorithm,
    pub key_rotation_days: u64,
    pub encrypt_metadata: bool,
}

impl Default for ContextEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: EncryptionAlgorithm::None,
            key_rotation_days: 90,
            encrypt_metadata: false,
        }
    }
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
    pub created_at: DateTime<Utc>,
    pub scope_path: String,
    pub data_size_bytes: u64,
    pub checksum: String,
    pub compression_ratio: Option<f64>,
    pub encryption_enabled: bool,
    pub metadata: HashMap<String, serde_json::Value>,
}

// Context statistics types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextStats {
    pub scopes_count: usize,
    pub knowledge_entries_count: usize,
    pub todos_count: usize,
    pub decisions_count: usize,
    pub patterns_count: usize,
    pub conventions_count: usize,
    pub last_updated: DateTime<Utc>,
    pub lock_file_stats: Option<LockFileContextStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileContextStats {
    pub total_scopes: usize,
    pub total_dependencies: usize,
    pub circular_dependencies: usize,
    pub validation_status: String,
    pub health_score: f64,
}

// Context change tracking types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextChange {
    pub scope_path: String,
    pub change_type: ChangeType,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Knowledge,
    Todo,
    Decision,
    Pattern,
    Convention,
    LockFile,
}

// AI agent context types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockScopeContext {
    pub scope_path: String,
    pub version: String,
    pub dependencies: HashMap<String, rhema_core::schema::ScopeDependency>,
    pub has_circular_dependencies: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub source_checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyVersionInfo {
    pub name: String,
    pub version: String,
    pub path: String,
    pub dependency_type: String,
    pub is_transitive: bool,
    pub original_constraint: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            validation_status: "Unknown".to_string(),
            resolution_strategy: "Unknown".to_string(),
            conflict_resolution: "Unknown".to_string(),
            potential_conflicts: Vec::new(),
            dependency_graph: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConflict {
    pub dependency_name: String,
    pub scope1: String,
    pub version1: String,
    pub scope2: String,
    pub version2: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileHealthInfo {
    pub is_valid: bool,
    pub validation_status: String,
    pub validation_messages: Vec<String>,
    pub last_validated: Option<DateTime<Utc>>,
    pub performance_metrics: Option<rhema_core::schema::LockPerformanceMetrics>,
    pub health_score: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

impl Default for LockFileHealthInfo {
    fn default() -> Self {
        Self {
            is_valid: false,
            validation_status: "Unknown".to_string(),
            validation_messages: Vec::new(),
            last_validated: None,
            performance_metrics: None,
            health_score: 0.0,
            issues: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRecommendation {
    pub dependency_name: String,
    pub current_version: String,
    pub recommended_version: String,
    pub reason: String,
    pub priority: RecommendationPriority,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}
