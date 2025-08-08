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


/// Configuration environment types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfigEnvironment {
    Development,
    Testing,
    Staging,
    Production,
    Custom(String),
}

/// Configuration audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAuditLog {
    pub entries: Vec<ConfigAuditEntry>,
}

impl ConfigAuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

/// Configuration audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAuditEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub action: String,
    pub user: String,
    pub details: String,
}

/// Configuration health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHealth {
    pub status: ConfigHealthStatus,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub issues: Vec<String>,
}

/// Configuration health status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigHealthStatus {
    Healthy,
    Warning,
    Error,
    Unknown,
}

/// Configuration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStats {
    pub total_configs: usize,
    pub valid_configs: usize,
    pub invalid_configs: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl ConfigStats {
    pub fn new() -> Self {
        Self {
            total_configs: 0,
            valid_configs: 0,
            invalid_configs: 0,
            last_updated: chrono::Utc::now(),
        }
    }
}

impl Default for ConfigStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration error types
#[derive(Debug, thiserror::Error, Clone)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("YAML error: {0}")]
    YamlError(String),

    #[error("TOML error: {0}")]
    TomlError(String),

    #[error("Bincode error: {0}")]
    BincodeError(String),

    #[error("Version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: String, found: String },

    #[error("Migration failed: {0}")]
    MigrationFailed(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Backup failed: {0}")]
    BackupFailed(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// Configuration change types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigChangeType {
    Created,
    Updated,
    Deleted,
    Migrated,
}

/// Configuration change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigChange {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub change_type: ConfigChangeType,
    pub description: String,
    pub user: String,
}

/// Configuration issue severity
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub enum ConfigIssueSeverity {
    Critical,
    Error,
    Warning,
    Info,
}

/// Configuration issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigIssue {
    pub severity: ConfigIssueSeverity,
    pub message: String,
    pub location: Option<String>,
    pub suggestion: Option<String>,
}

/// Configuration trait
pub trait Config: serde::Serialize + for<'de> serde::Deserialize<'de> {
    fn version(&self) -> &str;
    fn validate_config(&self) -> rhema_core::RhemaResult<()>;
    fn load_from_file(path: &std::path::Path) -> rhema_core::RhemaResult<Self>
    where
        Self: Sized;
    fn save_to_file(&self, path: &std::path::Path) -> rhema_core::RhemaResult<()>;
    fn schema() -> serde_json::Value;
    fn documentation() -> &'static str;
}

/// Current configuration version
pub const CURRENT_CONFIG_VERSION: &str = "0.1.0";
