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

use rhema_git::automation::{
    GitAutomationManager, default_automation_config,
    GitWorkflowIntegration, WorkflowAutomationIntervals, WorkflowAutomationTriggers,
    WorkflowAutomationRules, FeatureAutomationRules, ReleaseAutomationRules, HotfixAutomationRules
};
use git2::Repository;
use tempfile::TempDir;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use rhema_git::RhemaError;

fn setup_test_automation() -> (TempDir, Repository, GitAutomationManager) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    let config = default_automation_config();
    let manager = GitAutomationManager::new(repo.clone(), config);
    (temp_dir, repo, manager)
}

fn setup_test_automation_with_config(config: rhema_git::automation::AutomationConfig) -> (TempDir, Repository, GitAutomationManager) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    let manager = GitAutomationManager::new(repo.clone(), config);
    (temp_dir, repo, manager)
}

// Basic functionality tests

#[test]
fn test_workflow_automation_trigger() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test workflow automation trigger
    let mut data = HashMap::new();
    data.insert("branch_name".to_string(), "feature/test".to_string());
    
    let result = manager.trigger_workflow_automation("branch_creation", Some(data));
    assert!(result.is_ok());
}

#[test]
fn test_feature_automation_trigger() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test feature automation trigger
    let result = manager.trigger_feature_automation("test-feature", "setup_context");
    assert!(result.is_ok());
}

#[test]
fn test_release_automation_trigger() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test release automation trigger
    let result = manager.trigger_release_automation("1.0.0", "prepare_context");
    assert!(result.is_ok());
}

#[test]
fn test_hotfix_automation_trigger() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test hotfix automation trigger
    let result = manager.trigger_hotfix_automation("1.0.1", "setup_context");
    assert!(result.is_ok());
}

// Configuration tests

#[test]
fn test_automation_configuration() {
    let (_temp_dir, _repo, _manager) = setup_test_automation();
    
    // Test that automation configuration is properly structured
    let config = default_automation_config();
    
    // Check workflow integration settings
    assert_eq!(config.git_workflow_integration.workflow_automation, false);
    assert_eq!(config.git_workflow_integration.auto_create_feature_branches, false);
    assert_eq!(config.git_workflow_integration.auto_merge_feature_branches, false);
    
    // Check intervals
    assert_eq!(config.git_workflow_integration.intervals.feature_automation_interval, 300);
    assert_eq!(config.git_workflow_integration.intervals.release_automation_interval, 3600);
    assert_eq!(config.git_workflow_integration.intervals.hotfix_automation_interval, 600);
    
    // Check triggers
    assert_eq!(config.git_workflow_integration.triggers.on_branch_creation, false);
    assert_eq!(config.git_workflow_integration.triggers.on_commit_push, false);
    assert_eq!(config.git_workflow_integration.triggers.on_schedule, false);
    
    // Check rules
    assert_eq!(config.git_workflow_integration.rules.feature_rules.auto_setup_context, false);
    assert_eq!(config.git_workflow_integration.rules.release_rules.auto_prepare_context, false);
    assert_eq!(config.git_workflow_integration.rules.hotfix_rules.auto_setup_context, false);
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
    
    let manager = GitAutomationManager::new(repo, config);
    
    // Test that automation works when enabled
    let result = manager.trigger_feature_automation("test-feature", "setup_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_release_automation("1.0.0", "prepare_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_hotfix_automation("1.0.1", "setup_context");
    assert!(result.is_ok());
}

#[test]
fn test_automation_task_creation() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Trigger multiple automation tasks
    manager.trigger_feature_automation("feature1", "setup_context").unwrap();
    manager.trigger_release_automation("1.0.0", "prepare_context").unwrap();
    manager.trigger_hotfix_automation("1.0.1", "setup_context").unwrap();
    
    // Check that tasks were created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 3);
    
    // Check task types
    let task_types: Vec<_> = history.iter().map(|t| &t.task_type).collect();
    assert!(task_types.contains(&&rhema_git::automation::TaskType::FeatureAutomation));
    assert!(task_types.contains(&&rhema_git::automation::TaskType::ReleaseAutomation));
    assert!(task_types.contains(&&rhema_git::automation::TaskType::HotfixAutomation));
}

// Error handling tests

#[test]
fn test_automation_disabled_behavior() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // When automation is disabled, triggers should return Ok(()) without doing anything
    let result = manager.trigger_workflow_automation("branch_creation", None);
    assert!(result.is_ok());
    
    let result = manager.trigger_feature_automation("test", "setup_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_release_automation("1.0.0", "prepare_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_hotfix_automation("1.0.1", "setup_context");
    assert!(result.is_ok());
    
    // No tasks should be created when automation is disabled
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 0);
}

#[test]
fn test_input_validation_errors() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
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
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test with invalid trigger type
    let result = manager.trigger_workflow_automation("invalid_trigger", None);
    // Should still return Ok(()) when automation is disabled
    assert!(result.is_ok());
    
    // Test with enabled automation
    let mut config = default_automation_config();
    config.git_workflow_integration.workflow_automation = true;
    let (_temp_dir2, _repo2, manager2) = setup_test_automation_with_config(config);
    
    let result = manager2.trigger_workflow_automation("invalid_trigger", None);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
}

#[test]
fn test_invalid_actions() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
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
    let (_temp_dir2, _repo2, manager2) = setup_test_automation_with_config(config);
    
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
    let (_temp_dir, _repo, manager) = setup_test_automation_with_config(config);
    
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
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
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
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
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
    let (_temp_dir, _repo, manager) = setup_test_automation_with_config(config);
    
    // Test with zero interval
    let result = manager.schedule_workflow_automation();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RhemaError::ValidationError(_)));
    
    // Test with valid interval
    config.git_workflow_integration.intervals.workflow_validation_interval = 300;
    let (_temp_dir2, _repo2, manager2) = setup_test_automation_with_config(config);
    
    let result = manager2.schedule_workflow_automation();
    assert!(result.is_ok());
}

// Task management tests

#[test]
fn test_task_status_tracking() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Get initial status
    let initial_status = manager.get_status().unwrap();
    assert_eq!(initial_status.running, false);
    assert_eq!(initial_status.total_tasks, 0);
    assert_eq!(initial_status.completed_tasks, 0);
    assert_eq!(initial_status.failed_tasks, 0);
    assert_eq!(initial_status.running_tasks, 0);
    assert_eq!(initial_status.pending_tasks, 0);
}

#[test]
fn test_task_history_limits() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Trigger multiple tasks
    for i in 0..10 {
        manager.trigger_feature_automation(&format!("feature{}", i), "setup_context").unwrap();
    }
    
    // Test with limit
    let limited_history = manager.get_task_history(Some(5));
    assert_eq!(limited_history.len(), 5);
    
    // Test without limit
    let full_history = manager.get_task_history(None);
    assert_eq!(full_history.len(), 10);
}

#[test]
fn test_task_cancellation() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Trigger a task
    manager.trigger_feature_automation("test-feature", "setup_context").unwrap();
    
    // Get the task ID
    let history = manager.get_task_history(Some(1));
    let task_id = &history[0].id;
    
    // Cancel the task
    let result = manager.cancel_task(task_id);
    assert!(result.is_ok());
}

#[test]
fn test_task_history_clear() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Trigger some tasks
    manager.trigger_feature_automation("feature1", "setup_context").unwrap();
    manager.trigger_release_automation("1.0.0", "prepare_context").unwrap();
    
    // Verify tasks exist
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 2);
    
    // Clear history
    let result = manager.clear_task_history();
    assert!(result.is_ok());
    
    // Verify history is cleared
    let history = manager.get_task_history(None);
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
    let (_temp_dir, _repo, manager) = setup_test_automation_with_config(config);
    
    // Test that custom configuration is applied
    let result = manager.trigger_feature_automation("test-feature", "setup_context");
    assert!(result.is_ok());
    
    // Verify task was created (since automation is enabled)
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 1);
}

// Edge case tests

#[test]
fn test_concurrent_task_creation() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Trigger multiple tasks concurrently
    let mut handles = vec![];
    
    for i in 0..5 {
        let manager_clone = &manager;
        let handle = std::thread::spawn(move || {
            manager_clone.trigger_feature_automation(&format!("feature{}", i), "setup_context")
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result.is_ok());
    }
    
    // Verify all tasks were created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 5);
}

#[test]
fn test_large_data_payload() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Create a large data payload
    let mut large_data = HashMap::new();
    for i in 0..100 {
        large_data.insert(format!("key{}", i), format!("value{}", i));
    }
    
    let result = manager.trigger_workflow_automation("branch_creation", Some(large_data));
    assert!(result.is_ok());
}

#[test]
fn test_special_characters_in_names() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test with special characters in names
    let result = manager.trigger_feature_automation("feature-with-dashes", "setup_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_feature_automation("feature_with_underscores", "setup_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_release_automation("1.0.0-beta", "prepare_context");
    assert!(result.is_ok());
    
    let result = manager.trigger_hotfix_automation("1.0.1-rc1", "setup_context");
    assert!(result.is_ok());
}

#[test]
fn test_very_long_names() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test with very long names
    let long_feature_name = "a".repeat(1000);
    let result = manager.trigger_feature_automation(&long_feature_name, "setup_context");
    assert!(result.is_ok());
    
    let long_version = "1.0.0".repeat(100);
    let result = manager.trigger_release_automation(&long_version, "prepare_context");
    assert!(result.is_ok());
}

// Integration tests

#[test]
fn test_full_workflow_automation_cycle() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Simulate a complete workflow automation cycle
    let mut branch_data = HashMap::new();
    branch_data.insert("branch_name".to_string(), "feature/user-auth".to_string());
    
    // 1. Branch creation
    let result = manager.trigger_workflow_automation("branch_creation", Some(branch_data.clone()));
    assert!(result.is_ok());
    
    // 2. Commit push
    let result = manager.trigger_workflow_automation("commit_push", Some(branch_data.clone()));
    assert!(result.is_ok());
    
    // 3. Pull request opened
    let mut pr_data = branch_data.clone();
    pr_data.insert("action".to_string(), "opened".to_string());
    let result = manager.trigger_workflow_automation("pull_request", Some(pr_data));
    assert!(result.is_ok());
    
    // 4. Pull request closed
    let mut pr_data = branch_data.clone();
    pr_data.insert("action".to_string(), "closed".to_string());
    let result = manager.trigger_workflow_automation("pull_request", Some(pr_data));
    assert!(result.is_ok());
    
    // 5. Branch merge
    let result = manager.trigger_workflow_automation("branch_merge", Some(branch_data));
    assert!(result.is_ok());
    
    // Verify tasks were created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 5);
}

#[test]
fn test_multiple_workflow_types() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test feature workflow
    manager.trigger_feature_automation("feature1", "setup_context").unwrap();
    manager.trigger_feature_automation("feature1", "validate").unwrap();
    manager.trigger_feature_automation("feature1", "merge").unwrap();
    manager.trigger_feature_automation("feature1", "cleanup").unwrap();
    
    // Test release workflow
    manager.trigger_release_automation("1.0.0", "prepare_context").unwrap();
    manager.trigger_release_automation("1.0.0", "validate").unwrap();
    manager.trigger_release_automation("1.0.0", "merge_to_main").unwrap();
    manager.trigger_release_automation("1.0.0", "merge_to_develop").unwrap();
    manager.trigger_release_automation("1.0.0", "cleanup").unwrap();
    
    // Test hotfix workflow
    manager.trigger_hotfix_automation("1.0.1", "setup_context").unwrap();
    manager.trigger_hotfix_automation("1.0.1", "validate").unwrap();
    manager.trigger_hotfix_automation("1.0.1", "merge_to_main").unwrap();
    manager.trigger_hotfix_automation("1.0.1", "merge_to_develop").unwrap();
    manager.trigger_hotfix_automation("1.0.1", "cleanup").unwrap();
    
    // Verify all tasks were created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 13);
    
    // Verify task types
    let task_types: Vec<_> = history.iter().map(|t| &t.task_type).collect();
    assert_eq!(task_types.iter().filter(|&&t| matches!(t, rhema_git::automation::TaskType::FeatureAutomation)).count(), 4);
    assert_eq!(task_types.iter().filter(|&&t| matches!(t, rhema_git::automation::TaskType::ReleaseAutomation)).count(), 5);
    assert_eq!(task_types.iter().filter(|&&t| matches!(t, rhema_git::automation::TaskType::HotfixAutomation)).count(), 4);
}

// Performance tests

#[test]
fn test_rapid_task_creation() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Create many tasks rapidly
    for i in 0..100 {
        manager.trigger_feature_automation(&format!("feature{}", i), "setup_context").unwrap();
    }
    
    // Verify all tasks were created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 100);
}

#[test]
fn test_task_id_uniqueness() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Create multiple tasks
    for i in 0..10 {
        manager.trigger_feature_automation(&format!("feature{}", i), "setup_context").unwrap();
    }
    
    // Verify task IDs are unique
    let history = manager.get_task_history(None);
    let task_ids: Vec<_> = history.iter().map(|t| &t.id).collect();
    let unique_ids: std::collections::HashSet<_> = task_ids.iter().collect();
    assert_eq!(task_ids.len(), unique_ids.len());
}

// Async tests (if tokio runtime is available)

#[tokio::test]
async fn test_async_automation_operations() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Test async operations
    let result = manager.trigger_feature_automation("async-feature", "setup_context");
    assert!(result.is_ok());
    
    // Wait a bit for async operations to complete
    sleep(Duration::from_millis(100)).await;
    
    // Check that task was created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 1);
}

// Error recovery tests

#[test]
fn test_error_recovery_after_failed_task() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Trigger a task that might fail
    let result = manager.trigger_feature_automation("test-feature", "invalid_action");
    assert!(result.is_ok());
    
    // Trigger another task - should still work
    let result = manager.trigger_feature_automation("test-feature2", "setup_context");
    assert!(result.is_ok());
    
    // Verify both tasks were created
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 2);
}

#[test]
fn test_automation_manager_persistence() {
    let (_temp_dir, _repo, manager) = setup_test_automation();
    
    // Create some tasks
    manager.trigger_feature_automation("feature1", "setup_context").unwrap();
    manager.trigger_release_automation("1.0.0", "prepare_context").unwrap();
    
    // Verify tasks exist
    let history = manager.get_task_history(None);
    assert_eq!(history.len(), 2);
    
    // The manager should persist tasks across operations
    let status = manager.get_status().unwrap();
    assert_eq!(status.total_tasks, 2);
} 