pub mod advanced;
pub mod automation;
pub mod branch;
pub mod feature_automation;
pub mod history;
pub mod hooks;
pub mod monitoring;
pub mod security;
pub mod workflow;
pub mod version_management;

// Export specific types to avoid conflicts
pub use feature_automation::{
    FeatureAutomationManager, FeatureAutomationConfig, FeatureContext,
    ValidationResult, MergeResult, CleanupResult,
    default_feature_automation_config
};

// Export version management types
pub use version_management::{
    VersionManager, VersionManagementConfig, VersionManagementResult,
    BumpType, CommitType, CommitInfo, default_version_management_config,
}; 