pub mod git;
pub mod git_hooks;
pub mod utils;

// Re-export version management types
pub use git::version_management::{
    default_version_management_config, BumpType, CommitInfo, CommitType, VersionManagementConfig,
    VersionManagementResult, VersionManager,
};

use rhema_core::RhemaResult;
use std::path::Path;

// Re-export the basic types that the CLI needs
pub use utils::*;

/// Create an advanced Git integration instance
pub fn create_advanced_git_integration(repo_path: &Path) -> RhemaResult<AdvancedGitIntegration> {
    let repo = get_repo(repo_path)?;
    AdvancedGitIntegration::new(repo)
}

/// Create an advanced Git integration instance with custom configuration
pub fn create_advanced_git_integration_with_config(
    repo_path: &Path,
    config: serde_json::Value, // Implement config handling
) -> RhemaResult<AdvancedGitIntegration> {
    // Implement config handling
    let mut integration = create_advanced_git_integration(repo_path)?;

    // Parse and apply configuration
    if let Some(config_obj) = config.as_object() {
        // Apply hook configuration
        if let Some(hooks_config) = config_obj.get("hooks") {
            integration.apply_hooks_config(hooks_config)?;
        }

        // Apply workflow configuration
        if let Some(workflow_config) = config_obj.get("workflow") {
            integration.apply_workflow_config(workflow_config)?;
        }

        // Apply automation configuration
        if let Some(automation_config) = config_obj.get("automation") {
            integration.apply_automation_config(automation_config)?;
        }

        // Apply security configuration
        if let Some(security_config) = config_obj.get("security") {
            integration.apply_security_config(security_config)?;
        }

        // Apply monitoring configuration
        if let Some(monitoring_config) = config_obj.get("monitoring") {
            integration.apply_monitoring_config(monitoring_config)?;
        }

        // Apply context configuration
        if let Some(context_config) = config_obj.get("context") {
            integration.apply_context_config(context_config)?;
        }

        // Apply performance configuration
        if let Some(performance_config) = config_obj.get("performance") {
            integration.apply_performance_config(performance_config)?;
        }

        // Apply integration configuration
        if let Some(integration_config) = config_obj.get("integrations") {
            integration.apply_integration_config(integration_config)?;
        }
    }

    Ok(integration)
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
        let _create_fn: fn(&Path) -> RhemaResult<AdvancedGitIntegration> =
            create_advanced_git_integration;
        let _create_with_config_fn: fn(
            &Path,
            serde_json::Value,
        ) -> RhemaResult<AdvancedGitIntegration> = create_advanced_git_integration_with_config;
    }
}
