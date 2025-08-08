pub mod backup;
pub mod config;
pub mod global;
pub mod invariants;
pub mod lock;
pub mod migration;
pub mod repository;
pub mod scope;
pub mod security;
#[cfg(test)]
pub mod test_config;
pub mod tools;
pub mod types;
pub mod validation;
pub mod validator;
pub mod schema_validator;
pub mod comprehensive_validator;
pub mod validation_rules;

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
pub use backup::{
    BackupManager, BackupFormat, BackupSchedule, BackupFrequency, RestoredConfig,
    DetailedBackupStats, BackupReport, BackupRecord, BackupSummary, RestoreReport, RestoreSummary
};
pub use global::GlobalConfig;
pub use lock::{
    AlertThresholds, CacheConfig, CacheEvictionPolicy, CacheType, ConflictResolutionConfig,
    ConflictResolutionStrategy, ConstraintType, EnvironmentLockConfig, LockConfig, MemoryConfig,
    MetricsFormat, MonitoringConfig, NetworkConfig, NotificationChannel, OptimizationConfig,
    OptimizationLevel, ParallelConfig, PerformanceConfig, ResolutionConfig, ResolutionStrategy,
    UpdateFrequency, UpdateNotificationConfig, UpdatePoliciesConfig, UpdateRollbackConfig,
    UpdateSchedulingConfig, ValidationConfig, ValidationSeverity, VersionConstraintConfig,
};
pub use repository::RepositoryConfig;
pub use scope::{ScopeConfig, ScopeDependency, DependencyType};
pub use security::{SecurityManager, SecurityConfig, EncryptionSettings, AccessControlSettings, AuditSettings, ComplianceSettings, AccessDecision, ComplianceReport, ComplianceStatus};
pub use migration::{
    MigrationManager, Migration, MigrationStep, MigrationStepType, MigrationCondition, MigrationConditionOperator,
    MigrationReport, MigrationRecord, MigrationSummary
};
pub use tools::{
    ConfigBackupTool, ConfigDocumentationTool, ConfigEditor, ConfigMigrator, ConfigValidator,
    ToolsConfig, ToolsManager, EditorSettings, ValidationSettings, MigrationSettings, BackupSettings,
    DocumentationSettings, ToolIntegrations, EditorType, ValidationCache,
    MigrationStrategy, BackupRetention, RetentionPolicy, DocumentationFormat, DocumentationStyle,
    GitIntegration, IDEIntegration, CICDIntegration, ExternalTool, DocumentationReport, ValidationStatus, MigrationStatus, BackupStatus, DocumentationStatus
};
pub use validation::{ValidationManager, ValidationReport, ValidationResult};
pub use schema_validator::{SchemaValidator, SchemaType, SchemaValidationResult, SchemaValidationIssue, SchemaValidationStatistics};
pub use comprehensive_validator::{
    ComprehensiveValidator, ComprehensiveValidationResult, ComprehensiveValidationIssue,
    ComprehensiveValidationReport, ComprehensiveValidationSummary, ComprehensiveValidationStatistics,
    ValidationCategory,
};
pub use validation_rules::{
    ValidationRulesConfig, ValidationRule, RuleType, RuleCondition, ConditionOperator,
    RuleAction, ActionType, RuleSet, GlobalValidationSettings, SchemaOverride,
    CustomValidatorConfig, ValidationRulesManager, RuleEvaluationResult, ValidationRulesStatistics,
};
pub use invariants::{ContextValidator, DependencyValidator, AgentValidator, LockValidator, SyncValidator};

// Error type conversions
impl From<ConfigError> for RhemaError {
    fn from(err: ConfigError) -> Self {
        RhemaError::ConfigError(err.to_string())
    }
}
