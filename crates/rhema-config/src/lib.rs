pub mod backup;
pub mod comprehensive_validator;
pub mod config;
pub mod global;
pub mod invariants;
pub mod lock;
pub mod migration;
pub mod repository;
pub mod schema_validator;
pub mod scope;
pub mod security;
#[cfg(test)]
pub mod test_config;
pub mod tools;
pub mod types;
pub mod validation;
pub mod validation_rules;
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
pub use backup::{
    BackupFormat, BackupFrequency, BackupManager, BackupRecord, BackupReport, BackupSchedule,
    BackupSummary, DetailedBackupStats, RestoreReport, RestoreSummary, RestoredConfig,
};
pub use comprehensive_validator::{
    ComprehensiveValidationIssue, ComprehensiveValidationReport, ComprehensiveValidationResult,
    ComprehensiveValidationStatistics, ComprehensiveValidationSummary, ComprehensiveValidator,
    ValidationCategory,
};
pub use global::GlobalConfig;
pub use invariants::{
    AgentValidator, ContextValidator, DependencyValidator, LockValidator, SyncValidator,
};
pub use lock::{
    AlertThresholds, CacheConfig, CacheEvictionPolicy, CacheType, ConflictResolutionConfig,
    ConflictResolutionStrategy, ConstraintType, EnvironmentLockConfig, LockConfig, MemoryConfig,
    MetricsFormat, MonitoringConfig, NetworkConfig, NotificationChannel, OptimizationConfig,
    OptimizationLevel, ParallelConfig, PerformanceConfig, ResolutionConfig, ResolutionStrategy,
    UpdateFrequency, UpdateNotificationConfig, UpdatePoliciesConfig, UpdateRollbackConfig,
    UpdateSchedulingConfig, ValidationConfig, ValidationSeverity, VersionConstraintConfig,
};
pub use migration::{
    Migration, MigrationCondition, MigrationConditionOperator, MigrationManager, MigrationRecord,
    MigrationReport, MigrationStep, MigrationStepType, MigrationSummary,
};
pub use repository::RepositoryConfig;
pub use schema_validator::{
    SchemaType, SchemaValidationIssue, SchemaValidationResult, SchemaValidationStatistics,
    SchemaValidator,
};
pub use scope::{DependencyType, ScopeConfig, ScopeDependency};
pub use security::{
    AccessControlSettings, AccessDecision, AuditSettings, ComplianceReport, ComplianceSettings,
    ComplianceStatus, EncryptionSettings, SecurityConfig, SecurityManager,
};
pub use tools::{
    BackupRetention, BackupSettings, BackupStatus, CICDIntegration, ConfigBackupTool,
    ConfigDocumentationTool, ConfigEditor, ConfigMigrator, ConfigValidator, DocumentationFormat,
    DocumentationReport, DocumentationSettings, DocumentationStatus, DocumentationStyle,
    EditorSettings, EditorType, ExternalTool, GitIntegration, IDEIntegration, MigrationSettings,
    MigrationStatus, MigrationStrategy, RetentionPolicy, ToolIntegrations, ToolsConfig,
    ToolsManager, ValidationCache, ValidationSettings, ValidationStatus,
};
pub use validation::{ValidationManager, ValidationReport, ValidationResult};
pub use validation_rules::{
    ActionType, ConditionOperator, CustomValidatorConfig, GlobalValidationSettings, RuleAction,
    RuleCondition, RuleEvaluationResult, RuleSet, RuleType, SchemaOverride, ValidationRule,
    ValidationRulesConfig, ValidationRulesManager, ValidationRulesStatistics,
};

// Error type conversions
impl From<ConfigError> for RhemaError {
    fn from(err: ConfigError) -> Self {
        RhemaError::ConfigError(err.to_string())
    }
}
