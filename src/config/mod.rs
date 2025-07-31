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

pub mod global;
pub mod repository;
pub mod scope;
pub mod security;
pub mod tools;
pub mod validation;
pub mod migration;
pub mod backup;

#[cfg(test)]
mod test_config;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use validator::Validate;

/// Configuration version for compatibility tracking
pub const CURRENT_CONFIG_VERSION: &str = "1.0.0";

/// Main configuration manager for Rhema CLI
pub struct ConfigManager {
    global_config: global::GlobalConfig,
    repository_configs: HashMap<PathBuf, repository::RepositoryConfig>,
    scope_configs: HashMap<PathBuf, scope::ScopeConfig>,
    security_manager: security::SecurityManager,
    tools_manager: tools::ToolsManager,
    validation_manager: validation::ValidationManager,
    migration_manager: migration::MigrationManager,
    backup_manager: backup::BackupManager,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> crate::RhemaResult<Self> {
        let global_config = global::GlobalConfig::load()?;
        let security_manager = security::SecurityManager::new(&global_config)?;
        let tools_manager = tools::ToolsManager::new(&global_config)?;
        let validation_manager = validation::ValidationManager::new(&global_config)?;
        let migration_manager = migration::MigrationManager::new(&global_config)?;
        let backup_manager = backup::BackupManager::new(&global_config)?;

        Ok(Self {
            global_config,
            repository_configs: HashMap::new(),
            scope_configs: HashMap::new(),
            security_manager,
            tools_manager,
            validation_manager,
            migration_manager,
            backup_manager,
        })
    }

    /// Get global configuration
    pub fn global_config(&self) -> &global::GlobalConfig {
        &self.global_config
    }

    /// Get mutable global configuration
    pub fn global_config_mut(&mut self) -> &mut global::GlobalConfig {
        &mut self.global_config
    }

    /// Load repository configuration
    pub fn load_repository_config(&mut self, repo_path: &Path) -> crate::RhemaResult<&repository::RepositoryConfig> {
        if !self.repository_configs.contains_key(repo_path) {
            let config = repository::RepositoryConfig::load(repo_path)?;
            self.repository_configs.insert(repo_path.to_path_buf(), config);
        }
        Ok(self.repository_configs.get(repo_path).unwrap())
    }

    /// Load scope configuration
    pub fn load_scope_config(&mut self, scope_path: &Path) -> crate::RhemaResult<&scope::ScopeConfig> {
        if !self.scope_configs.contains_key(scope_path) {
            let config = scope::ScopeConfig::load(scope_path)?;
            self.scope_configs.insert(scope_path.to_path_buf(), config);
        }
        Ok(self.scope_configs.get(scope_path).unwrap())
    }

    /// Get security manager
    pub fn security(&self) -> &security::SecurityManager {
        &self.security_manager
    }

    /// Get tools manager
    pub fn tools(&self) -> &tools::ToolsManager {
        &self.tools_manager
    }

    /// Get validation manager
    pub fn validation(&self) -> &validation::ValidationManager {
        &self.validation_manager
    }

    /// Get mutable validation manager
    pub fn validation_mut(&mut self) -> &mut validation::ValidationManager {
        &mut self.validation_manager
    }

    /// Get migration manager
    pub fn migration(&self) -> &migration::MigrationManager {
        &self.migration_manager
    }

    /// Get mutable migration manager
    pub fn migration_mut(&mut self) -> &mut migration::MigrationManager {
        &mut self.migration_manager
    }

    /// Get backup manager
    pub fn backup(&self) -> &backup::BackupManager {
        &self.backup_manager
    }

    /// Get mutable backup manager
    pub fn backup_mut(&mut self) -> &mut backup::BackupManager {
        &mut self.backup_manager
    }

    /// Validate all configurations
    pub fn validate_all(&self) -> crate::RhemaResult<validation::ValidationReport> {
        self.validation_manager.validate_all(&self.global_config, &self.repository_configs, &self.scope_configs)
    }

    /// Backup all configurations
    pub fn backup_all(&mut self) -> crate::RhemaResult<backup::BackupReport> {
        self.backup_manager.backup_all(&self.global_config, &self.repository_configs, &self.scope_configs)
    }

    /// Migrate configurations to latest version
    pub fn migrate_all(&self) -> crate::RhemaResult<migration::MigrationReport> {
        self.migration_manager.migrate_all(&self.global_config, &self.repository_configs, &self.scope_configs)
    }
}

/// Configuration trait for all configuration types
pub trait Config: Serialize + DeserializeOwned + Validate {
    /// Configuration version
    fn version(&self) -> &str;
    
    /// Validate configuration
    fn validate_config(&self) -> crate::RhemaResult<()>;
    
    /// Load configuration from file
    fn load_from_file(path: &Path) -> crate::RhemaResult<Self>;
    
    /// Save configuration to file
    fn save_to_file(&self, path: &Path) -> crate::RhemaResult<()>;
    
    /// Get configuration schema
    fn schema() -> serde_json::Value;
    
    /// Get configuration documentation
    fn documentation() -> &'static str;
}

/// Configuration environment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConfigEnvironment {
    Development,
    Testing,
    Staging,
    Production,
    Custom(String),
}

/// Configuration priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ConfigPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Configuration change tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigChange {
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub change_type: ConfigChangeType,
    pub path: PathBuf,
    pub description: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
}

/// Configuration change types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigChangeType {
    Created,
    Updated,
    Deleted,
    Migrated,
    Validated,
    BackedUp,
    Restored,
}

impl std::fmt::Display for ConfigChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigChangeType::Created => write!(f, "Created"),
            ConfigChangeType::Updated => write!(f, "Updated"),
            ConfigChangeType::Deleted => write!(f, "Deleted"),
            ConfigChangeType::Migrated => write!(f, "Migrated"),
            ConfigChangeType::Validated => write!(f, "Validated"),
            ConfigChangeType::BackedUp => write!(f, "BackedUp"),
            ConfigChangeType::Restored => write!(f, "Restored"),
        }
    }
}

/// Configuration audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAuditLog {
    pub changes: Vec<ConfigChange>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ConfigAuditLog {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            changes: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_change(&mut self, change: ConfigChange) {
        self.changes.push(change);
        self.updated_at = Utc::now();
    }

    pub fn get_changes_for_path(&self, path: &Path) -> Vec<&ConfigChange> {
        self.changes
            .iter()
            .filter(|change| change.path == path)
            .collect()
    }

    pub fn get_changes_since(&self, timestamp: DateTime<Utc>) -> Vec<&ConfigChange> {
        self.changes
            .iter()
            .filter(|change| change.timestamp >= timestamp)
            .collect()
    }
}

/// Configuration health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHealth {
    pub status: ConfigHealthStatus,
    pub issues: Vec<ConfigIssue>,
    pub recommendations: Vec<String>,
    pub last_check: DateTime<Utc>,
}

/// Configuration health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigHealthStatus {
    Healthy,
    Warning,
    Error,
    Unknown,
}

/// Configuration issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigIssue {
    pub severity: ConfigIssueSeverity,
    pub message: String,
    pub path: Option<PathBuf>,
    pub field: Option<String>,
    pub suggestion: Option<String>,
}

/// Configuration issue severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ConfigIssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for ConfigIssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigIssueSeverity::Info => write!(f, "Info"),
            ConfigIssueSeverity::Warning => write!(f, "Warning"),
            ConfigIssueSeverity::Error => write!(f, "Error"),
            ConfigIssueSeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Configuration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStats {
    pub total_configs: usize,
    pub global_configs: usize,
    pub repository_configs: usize,
    pub scope_configs: usize,
    pub encrypted_configs: usize,
    pub backup_count: usize,
    pub last_backup: Option<DateTime<Utc>>,
    pub validation_errors: usize,
    pub migration_pending: usize,
}

impl ConfigStats {
    pub fn new() -> Self {
        Self {
            total_configs: 0,
            global_configs: 0,
            repository_configs: 0,
            scope_configs: 0,
            encrypted_configs: 0,
            backup_count: 0,
            last_backup: None,
            validation_errors: 0,
            migration_pending: 0,
        }
    }
}

/// Configuration error types
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("Invalid configuration format: {0}")]
    InvalidFormat(String),
    
    #[error("Configuration validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Configuration migration failed: {0}")]
    MigrationFailed(String),
    
    #[error("Configuration backup failed: {0}")]
    BackupFailed(String),
    
    #[error("Configuration restore failed: {0}")]
    RestoreFailed(String),
    
    #[error("Configuration encryption failed: {0}")]
    EncryptionFailed(String),
    
    #[error("Configuration decryption failed: {0}")]
    DecryptionFailed(String),
    
    #[error("Configuration access denied: {0}")]
    AccessDenied(String),
    
    #[error("Configuration version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },
    
    #[error("Configuration circular dependency detected: {0}")]
    CircularDependency(String),
    
    #[error("Configuration inheritance error: {0}")]
    InheritanceError(String),
    
    #[error("Configuration override error: {0}")]
    OverrideError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Bincode error: {0}")]
    BincodeError(#[from] Box<bincode::ErrorKind>),
}

/// Configuration result type
pub type ConfigResult<T> = Result<T, ConfigError>;

// Re-export submodules
pub use global::GlobalConfig;
pub use repository::RepositoryConfig;
pub use scope::ScopeConfig;
pub use security::SecurityManager;
pub use tools::ToolsManager;
pub use validation::ValidationManager;
pub use migration::MigrationManager;
pub use backup::BackupManager; 