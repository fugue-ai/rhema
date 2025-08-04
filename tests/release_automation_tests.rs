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

use rhema_git::workflow::{WorkflowManager, default_git_flow_config};
use git2::Repository;
use tempfile::TempDir;

fn setup_test_repo() -> (TempDir, Repository, WorkflowManager) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    let config = default_git_flow_config();
    let manager = WorkflowManager::new(repo.clone(), config);
    (temp_dir, repo, manager)
}

#[test]
fn test_prepare_and_validate_release_context() {
    let (_temp_dir, _repo, manager) = setup_test_repo();
    let version = "0.1.0-test";
    let release_branch = format!("release/{}", version);
    // Prepare context
    assert!(manager.prepare_release_context(&release_branch, version).is_ok());
    // Validate
    assert!(manager.validate_release(&release_branch).is_ok());
}

#[test]
fn test_merge_and_cleanup_release_branch() {
    let (_temp_dir, _repo, manager) = setup_test_repo();
    let version = "0.2.0-test";
    let release_branch = format!("release/{}", version);
    // Prepare context
    assert!(manager.prepare_release_context(&release_branch, version).is_ok());
    // Merge to main
    let _ = manager.merge_to_main(&release_branch);
    // Merge to develop
    let _ = manager.merge_to_develop(&release_branch);
    // Cleanup
    assert!(manager.cleanup_release_branch(&release_branch).is_ok());
} 