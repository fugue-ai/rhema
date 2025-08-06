pub mod git;
pub mod git_basic;
pub mod workflow_templates;
pub mod git_hooks;

// Re-export version management types
pub use git::version_management::{
    VersionManager, VersionManagementConfig, VersionManagementResult,
    BumpType, CommitType, CommitInfo, default_version_management_config,
};

use rhema_core::RhemaResult;
use std::path::Path;

// Re-export the basic types that the CLI needs
pub use git_basic::*;

/// Create an advanced Git integration instance
pub fn create_advanced_git_integration(repo_path: &Path) -> RhemaResult<AdvancedGitIntegration> {
    let repo = get_repo(repo_path)?;
    AdvancedGitIntegration::new(repo)
}

/// Create an advanced Git integration instance with custom configuration
pub fn create_advanced_git_integration_with_config(
    repo_path: &Path,
    _config: serde_json::Value, // TODO: Implement config handling
) -> RhemaResult<AdvancedGitIntegration> {
    create_advanced_git_integration(repo_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_git_workflow_api() {
        // Test that we can create the basic types
        let _feature_branch = FeatureBranch {
            name: "feature/test".to_string(),
            base_branch: "develop".to_string(),
            created_at: chrono::Utc::now(),
            context_files: vec![],
        };

        let _release_branch = ReleaseBranch {
            name: "release/1.0.0".to_string(),
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now(),
            status: ReleaseStatus::InProgress,
        };

        let _hotfix_branch = HotfixBranch {
            name: "hotfix/1.0.1".to_string(),
            version: "1.0.1".to_string(),
            created_at: chrono::Utc::now(),
            status: HotfixStatus::InProgress,
        };

        // Test that the API functions exist and have the right signatures
        let _create_fn: fn(&Path) -> RhemaResult<AdvancedGitIntegration> = create_advanced_git_integration;
        let _create_with_config_fn: fn(&Path, serde_json::Value) -> RhemaResult<AdvancedGitIntegration> = create_advanced_git_integration_with_config;
    }
}
