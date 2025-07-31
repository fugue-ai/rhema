pub mod config;
pub mod invariants;
pub mod validator;
pub mod test_config;
pub mod migration;
pub mod repository;
pub mod scope;
pub mod validation;
pub mod backup;
pub mod global;
pub mod security;
pub mod tools;
pub mod types;

// Re-export core types
pub use rhema_core::{RhemaError, RhemaResult};

// Re-export main config types
pub use config::{SafetyValidator, SafetyViolation, ValidationStatistics};

// Re-export types from the types module
pub use types::{
    ConfigEnvironment, ConfigAuditLog, ConfigAuditEntry, ConfigHealth, ConfigHealthStatus,
    ConfigStats, ConfigError, ConfigChangeType, ConfigChange, ConfigIssueSeverity, ConfigIssue,
    Config, CURRENT_CONFIG_VERSION
};

// Re-export specific types from modules
pub use repository::{RepositoryConfig};
pub use scope::{ScopeConfig};
pub use validation::{ValidationRule, ValidationResult, ValidationReport};
pub use backup::{RestoredConfig};
pub use global::{GlobalConfig};
pub use security::{SecurityConfig};
pub use tools::{ToolsConfig, ConfigEditor, ConfigValidator, ConfigMigrator, ConfigBackupTool, ConfigDocumentationTool};

// Error type conversions
impl From<ConfigError> for RhemaError {
    fn from(err: ConfigError) -> Self {
        RhemaError::ConfigError(err.to_string())
    }
}
