pub mod backup;
pub mod config;
pub mod global;
pub mod invariants;
pub mod migration;
pub mod repository;
pub mod scope;
pub mod security;
pub mod test_config;
pub mod tools;
pub mod types;
pub mod validation;
pub mod validator;

// Re-export core types
pub use rhema_core::{RhemaError, RhemaResult};

// Re-export main config types
pub use config::{SafetyValidator, SafetyViolation, ValidationStatistics};

// Re-export types from the types module
pub use types::{
    Config, ConfigAuditEntry, ConfigAuditLog, ConfigChange, ConfigChangeType, ConfigEnvironment,
    ConfigError, ConfigHealth, ConfigHealthStatus, ConfigIssue, ConfigIssueSeverity, ConfigStats,
    CURRENT_CONFIG_VERSION,
};

// Re-export specific types from modules
pub use backup::RestoredConfig;
pub use global::GlobalConfig;
pub use repository::RepositoryConfig;
pub use scope::ScopeConfig;
pub use security::SecurityConfig;
pub use tools::{
    ConfigBackupTool, ConfigDocumentationTool, ConfigEditor, ConfigMigrator, ConfigValidator,
    ToolsConfig,
};
pub use validation::{ValidationReport, ValidationResult, ValidationRule};

// Error type conversions
impl From<ConfigError> for RhemaError {
    fn from(err: ConfigError) -> Self {
        RhemaError::ConfigError(err.to_string())
    }
}
