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

use rhema_git::git::workflow::{WorkflowManager, default_git_flow_config};
use git2::Repository;
use tempfile::TempDir;

fn setup_test_repo() -> (TempDir, WorkflowManager) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    let config = default_git_flow_config();
    let manager = WorkflowManager::new(repo, config);
    (temp_dir, manager)
}

#[test]
fn test_setup_and_validate_hotfix_context() {
    let (_temp_dir, manager) = setup_test_repo();
    let version = "1.2.1";
    let hotfix_branch = format!("hotfix/{}", version);
    
    // Set up hotfix context
    assert!(manager.setup_hotfix_context(&hotfix_branch).is_ok());
    
    // Validate hotfix
    assert!(manager.validate_hotfix(&hotfix_branch).is_ok());
}

#[test]
fn test_merge_and_cleanup_hotfix_branch() {
    let (_temp_dir, manager) = setup_test_repo();
    let version = "1.2.2";
    let hotfix_branch = format!("hotfix/{}", version);
    
    // Set up hotfix context
    assert!(manager.setup_hotfix_context(&hotfix_branch).is_ok());
    
    // Merge to main
    let _ = manager.merge_to_main(&hotfix_branch);
    
    // Merge to develop
    let _ = manager.merge_to_develop(&hotfix_branch);
    
    // Cleanup
    assert!(manager.cleanup_hotfix_branch(&hotfix_branch).is_ok());
}

#[test]
fn test_hotfix_validation_failure() {
    let (_temp_dir, manager) = setup_test_repo();
    let non_existent_branch = "hotfix/non-existent";
    
    // This should fail because the branch doesn't exist
    let result = manager.validate_hotfix(non_existent_branch);
    assert!(result.is_err());
}

#[test]
fn test_hotfix_context_setup() {
    let (_temp_dir, manager) = setup_test_repo();
    let version = "1.2.3";
    let hotfix_branch = format!("hotfix/{}", version);
    
    // Set up hotfix context
    let result = manager.setup_hotfix_context(&hotfix_branch);
    assert!(result.is_ok());
    
    // Note: We can't access the repo directly since it's moved into the manager
    // The test passes if setup_hotfix_context succeeds
} 