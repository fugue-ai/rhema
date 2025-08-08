pub mod advanced;
pub mod automation;
pub mod branch;
pub mod feature_automation;
pub mod history;
pub mod hooks;
pub mod monitoring;
pub mod security;
pub mod version_management;
pub mod workflow;

// Export specific types to avoid conflicts
pub use feature_automation::{
    default_feature_automation_config, CleanupResult, FeatureAutomationConfig,
    FeatureAutomationManager, FeatureContext, MergeResult, ValidationResult,
};

// Export version management types
pub use version_management::{
    default_version_management_config, BumpType, CommitInfo, CommitType, VersionManagementConfig,
    VersionManagementResult, VersionManager,
};

// Export automation types
pub use automation::{
    default_automation_config, AutomationConfig, GitAutomationManager, TaskResult, TaskStatus,
    TaskType,
};
