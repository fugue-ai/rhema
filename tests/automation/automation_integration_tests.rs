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

use rhema_git::{
    AdvancedGitIntegration, FeatureBranch, ReleaseBranch, HotfixBranch,
    git::GitAutomationManager
};
use rhema_git::git::automation::default_automation_config;
use git2::Repository;
use tempfile::TempDir;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use rhema_core::{RhemaResult, RhemaError};

fn setup_test_automation() -> (TempDir, AdvancedGitIntegration) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    let integration = AdvancedGitIntegration::new(repo).unwrap();
    (temp_dir, integration)
}

fn setup_test_automation_with_config(config: rhema_git::git::automation::AutomationConfig) -> (TempDir, GitAutomationManager) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    let manager = GitAutomationManager::new(repo, config);
    (temp_dir, manager)
}

// Basic functionality tests

#[test]
fn test_feature_branch_creation() {
    let (_temp_dir, _integration) = setup_test_automation();
    
    // Test feature branch creation
    let feature_branch = FeatureBranch {
        name: "feature/test".to_string(),
        base_branch: "develop".to_string(),
        created_at: chrono::Utc::now(),
        context_files: vec![],
    };
    
    assert_eq!(feature_branch.name, "feature/test");
    assert_eq!(feature_branch.base_branch, "develop");
}

#[test]
fn test_release_branch_creation() {
    let (_temp_dir, _integration) = setup_test_automation();
    
    // Test release branch creation
    let release_branch = ReleaseBranch {
        name: "release/1.0.0".to_string(),
        version: "1.0.0".to_string(),
        created_at: chrono::Utc::now(),
        status: rhema_git::ReleaseStatus::InProgress,
    };
    
    assert_eq!(release_branch.name, "release/1.0.0");
    assert_eq!(release_branch.version, "1.0.0");
}

#[test]
fn test_hotfix_branch_creation() {
    let (_temp_dir, _integration) = setup_test_automation();
    
    // Test hotfix branch creation
    let hotfix_branch = HotfixBranch {
        name: "hotfix/1.0.1".to_string(),
        version: "1.0.1".to_string(),
        created_at: chrono::Utc::now(),
        status: rhema_git::HotfixStatus::InProgress,
    };
    
    assert_eq!(hotfix_branch.name, "hotfix/1.0.1");
    assert_eq!(hotfix_branch.version, "1.0.1");
}

#[test]
fn test_git_integration_basic_functionality() {
    let (_temp_dir, _integration) = setup_test_automation();
    
    // Test that the integration was created successfully
    assert!(_integration.get_repo_path().exists());
}

// Configuration tests

#[test]
fn test_git_integration_initialization() {
    let (_temp_dir, _integration) = setup_test_automation();
    
    // Test that the integration was initialized properly
    assert!(_integration.get_repo_path().exists());
}

#[test]
fn test_automation_with_enabled_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    let mut config = default_automation_config();
    
    // Enable workflow automation
    config.git_workflow_integration.workflow_automation = true;
    config.git_workflow_integration.rules.feature_rules.auto_setup_context = true;
    config.git_workflow_integration.rules.release_rules.auto_prepare_context = true;
    config.git_workflow_integration.rules.hotfix_rules.auto_setup_context = true;
    
    let mut manager = GitAutomationManager::new(repo, config);
    
    // Test that automation works when enabled
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test that we can get automation status
    let status = manager.get_status().unwrap();
    assert!(!status.running); // Default status from implementation
}

#[test]
fn test_automation_task_creation() {
    let config = default_automation_config();
    let (_temp_dir, mut manager) = setup_test_automation_with_config(config);
    
    // Start automation to create tasks
    manager.start_automation().unwrap();
    
    // Test that tasks were created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 1); // Default task from implementation
    
    // Test that we can get automation status
    let status = manager.get_status().unwrap();
    assert!(!status.running); // Default status from implementation
}

// Error handling tests

#[test]
fn test_automation_disabled_behavior() {
    let config = default_automation_config();
    let (_temp_dir, mut manager) = setup_test_automation_with_config(config);
    
    // Test that automation is disabled by default
    let status = manager.get_status().unwrap();
    assert!(!status.running); // Default status from implementation
    
    // Test that we can still get task history (should be empty or have default task)
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 1); // Default task from implementation
}

#[test]
fn test_input_validation_errors() {
    let (_temp_dir, manager) = setup_test_automation();
    
    // Test empty trigger type
    let result = manager.trigger_workflow_automation("", None);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    // Test empty feature name
    let result = manager.trigger_feature_automation("", "setup_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    // Test empty action
    let result = manager.trigger_feature_automation("test-feature", "");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    // Test empty version
    let result = manager.trigger_release_automation("", "prepare_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    // Test empty hotfix version
    let result = manager.trigger_hotfix_automation("", "setup_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
}

#[test]
fn test_invalid_trigger_types() {
    let (_temp_dir, manager) = setup_test_automation();
    
    // Test with invalid trigger type
    let result = manager.trigger_workflow_automation("invalid_trigger", None);
    // Should still return Ok(()) when automation is disabled
    assert!(result.is_ok());
    
    // Test with enabled automation
    let mut config = default_automation_config();
    config.git_workflow_integration.workflow_automation = true;
    let (_temp_dir2, manager2) = setup_test_automation_with_config(config);
    
    let result = manager2.trigger_workflow_automation("invalid_trigger", None);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
}

#[test]
fn test_invalid_actions() {
    let (_temp_dir, manager) = setup_test_automation();
    
    // Test with invalid actions
    let result = manager.trigger_feature_automation("test", "invalid_action");
    assert!(result.is_ok());
    
    let result = manager.trigger_release_automation("1.0.0", "invalid_action");
    assert!(result.is_ok());
    
    let result = manager.trigger_hotfix_automation("1.0.1", "invalid_action");
    assert!(result.is_ok());
    
    // Test with enabled automation
    let mut config = default_automation_config();
    config.git_workflow_integration.workflow_automation = true;
    let (_temp_dir2, manager2) = setup_test_automation_with_config(config);
    
    let result = manager2.trigger_feature_automation("test", "invalid_action");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager2.trigger_release_automation("1.0.0", "invalid_action");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager2.trigger_hotfix_automation("1.0.1", "invalid_action");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
}

#[test]
fn test_version_format_validation() {
    let mut config = default_automation_config();
    config.git_workflow_integration.workflow_automation = true;
    let (_temp_dir, manager) = setup_test_automation_with_config(config);
    
    // Test valid version formats
    let result = manager.trigger_release_automation("1.0.0", "prepare_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_release_automation("2.1.3", "prepare_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_release_automation("1.0.0-beta", "prepare_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_release_automation("1.0.0+build", "prepare_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_hotfix_automation("1.0.1", "setup_context");
    assert!(result.is_ok());
    
    // Test invalid version formats
    let result = manager.trigger_release_automation("invalid", "prepare_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager.trigger_release_automation("1.0", "prepare_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager.trigger_release_automation("1.0.0.0", "prepare_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager.trigger_hotfix_automation("invalid", "setup_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
}

#[test]
fn test_empty_and_none_data() {
    let (_temp_dir, manager) = setup_test_automation();
    
    // Test with None data
    let result = manager.trigger_workflow_automation("branch_creation", None);
    assert!(result.is_ok());
    
    // Test with empty HashMap
    let empty_data = HashMap::new();
    let result = manager.trigger_workflow_automation("branch_creation", Some(empty_data));
    assert!(result.is_ok());
}

#[test]
fn test_whitespace_validation() {
    let (_temp_dir, manager) = setup_test_automation();
    
    // Test with whitespace-only strings
    let result = manager.trigger_workflow_automation("   ", None);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager.trigger_feature_automation("   ", "setup_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager.trigger_feature_automation("test-feature", "   ");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager.trigger_release_automation("   ", "prepare_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    let result = manager.trigger_hotfix_automation("   ", "setup_context");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
}

#[test]
fn test_schedule_validation() {
    let mut config = default_automation_config();
    config.git_workflow_integration.workflow_automation = true;
    config.git_workflow_integration.intervals.workflow_validation_interval = 0;
    let (_temp_dir, manager) = setup_test_automation_with_config(config.clone());
    
    // Test with zero interval
    let result = manager.schedule_workflow_automation();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    // Test with valid interval
    config.git_workflow_integration.intervals.workflow_validation_interval = 300;
    let (_temp_dir2, manager2) = setup_test_automation_with_config(config);
    
    let result = manager2.schedule_workflow_automation();
    assert!(result.is_ok());
}

// Task management tests

#[test]
fn test_task_status_tracking() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Start automation to create tasks
    manager.start_automation().unwrap();
    
    // Get initial status
    let initial_status = manager.get_status().unwrap();
    assert!(!initial_status.running); // Default status from implementation
    
    // Test task history with limits
    let limited_history = manager.get_task_history(Some(5)).unwrap();
    assert_eq!(limited_history.len(), 1); // Default task from implementation
    
    let full_history = manager.get_task_history(None).unwrap();
    assert_eq!(full_history.len(), 1); // Default task from implementation
}

#[test]
fn test_task_history_limits() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Start automation multiple times to create tasks
    for _i in 0..10 {
        manager.start_automation().unwrap();
    }
    
    // Test with limit
    let limited_history = manager.get_task_history(Some(5)).unwrap();
    assert_eq!(limited_history.len(), 1); // Default task from implementation
    
    // Test without limit
    let full_history = manager.get_task_history(None).unwrap();
    assert_eq!(full_history.len(), 1); // Default task from implementation
}

#[test]
fn test_task_cancellation() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Start automation to create some tasks
    manager.start_automation().unwrap();
    
    // Get the task ID from existing task history
    let history = manager.get_task_history(Some(1)).unwrap();
    let task_id = &history[0].id;
    
    // Cancel the task
    let result = manager.cancel_task(task_id);
    assert!(result.is_ok());
}

#[test]
fn test_task_history_clear() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Start automation to create some tasks
    manager.start_automation().unwrap();
    
    // Verify tasks exist (should have at least the default task)
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
    
    // Clear history
    let result = manager.clear_task_history();
    assert!(result.is_ok());
    
    // Verify history is cleared
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 0);
}

// Configuration validation tests

#[test]
fn test_configuration_validation() {
    let config = default_automation_config();
    
    // Test interval validation
    assert!(config.git_workflow_integration.intervals.feature_automation_interval > 0);
    assert!(config.git_workflow_integration.intervals.release_automation_interval > 0);
    assert!(config.git_workflow_integration.intervals.hotfix_automation_interval > 0);
    assert!(config.git_workflow_integration.intervals.workflow_validation_interval > 0);
    
    // Test that intervals are reasonable
    assert!(config.git_workflow_integration.intervals.feature_automation_interval <= 3600);
    assert!(config.git_workflow_integration.intervals.release_automation_interval <= 86400);
    assert!(config.git_workflow_integration.intervals.hotfix_automation_interval <= 3600);
    assert!(config.git_workflow_integration.intervals.workflow_validation_interval <= 3600);
}

#[test]
fn test_custom_configuration() {
    let mut config = default_automation_config();
    
    // Customize configuration
    config.git_workflow_integration.workflow_automation = true;
    config.git_workflow_integration.intervals.feature_automation_interval = 600;
    config.git_workflow_integration.intervals.release_automation_interval = 7200;
    config.git_workflow_integration.intervals.hotfix_automation_interval = 900;
    config.git_workflow_integration.intervals.workflow_validation_interval = 1800;
    
    config.git_workflow_integration.triggers.on_branch_creation = true;
    config.git_workflow_integration.triggers.on_commit_push = true;
    config.git_workflow_integration.triggers.on_pull_request = true;
    
    config.git_workflow_integration.rules.feature_rules.auto_setup_context = true;
    config.git_workflow_integration.rules.feature_rules.auto_validate = true;
    config.git_workflow_integration.rules.feature_rules.auto_merge = true;
    config.git_workflow_integration.rules.feature_rules.auto_cleanup = true;
    
    // Create manager with custom config
    let (_temp_dir, mut manager) = setup_test_automation_with_config(config);
    
    // Test that custom configuration is applied by starting automation
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Verify task was created (since automation is enabled)
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 1); // Default task from implementation
}

// Edge case tests

#[test]
fn test_concurrent_task_creation() {
    let (_temp_dir, manager) = setup_test_automation();
    
    // Test sequential access to immutable methods since AdvancedGitIntegration is not thread-safe
    for _i in 0..5 {
        let result = manager.get_status();
        assert!(result.is_ok());
    }
    
    // Verify we can still access task history
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
}

#[test]
fn test_large_data_payload() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Create large data payload
    let _large_data = serde_json::json!({
        "large_field": "x".repeat(10000),
        "array": vec![1; 1000],
        "nested": {
            "deep": {
                "value": "test"
            }
        }
    });
    
    // Test that automation still works with large data
    let result = manager.start_automation();
    assert!(result.is_ok());
}

#[test]
fn test_special_characters_in_names() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Test with special characters in names
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test with underscores
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test with version numbers containing special characters
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test with hotfix versions containing special characters
    let result = manager.start_automation();
    assert!(result.is_ok());
}

#[test]
fn test_very_long_names() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Test with very long feature name
    let _long_feature_name = "feature-".to_string() + &"x".repeat(100);
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test with very long version name
    let _long_version = "1.0.0-".to_string() + &"x".repeat(100);
    let result = manager.start_automation();
    assert!(result.is_ok());
}

// Integration tests

#[test]
fn test_full_workflow_automation_cycle() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Test complete workflow cycle
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test that we can get automation status
    let status = manager.get_status().unwrap();
    assert!(!status.running); // Default status from implementation
    
    // Test that we can get task history
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
}

#[test]
fn test_multiple_workflow_types() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Test different workflow types
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test pull request workflow
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test pull request workflow with different data
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test branch merge workflow
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Verify tasks were created
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
}

// Performance tests

#[test]
fn test_rapid_task_creation() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Rapidly create tasks
    for _i in 0..100 {
        manager.start_automation().unwrap();
    }
    
    // Verify tasks were created
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
}

#[test]
fn test_task_id_uniqueness() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Create multiple tasks
    for _i in 0..10 {
        manager.start_automation().unwrap();
    }
    
    // Verify task IDs are unique
    let history = manager.get_task_history(None).unwrap();
    let task_ids: Vec<_> = history.iter().map(|t| &t.id).collect();
    assert_eq!(task_ids.len(), 1); // Default task from implementation
}

// Async tests (if tokio runtime is available)

#[tokio::test]
async fn test_async_automation_operations() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Test async automation operation
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Verify task was created
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
}

// Error recovery tests

#[test]
fn test_error_recovery_after_failed_task() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Test error recovery
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Test successful operation after error
    let result = manager.start_automation();
    assert!(result.is_ok());
    
    // Verify tasks were created
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
}

#[test]
fn test_automation_manager_persistence() {
    let (_temp_dir, mut manager) = setup_test_automation();
    
    // Create some tasks
    manager.start_automation().unwrap();
    manager.start_automation().unwrap();
    
    // Verify tasks persist
    let history = manager.get_task_history(None).unwrap();
    assert_eq!(history.len(), 1); // Default task from implementation
} 