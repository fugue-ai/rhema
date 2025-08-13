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

use git2::Repository;
use rhema_git::git::workflow::{default_git_flow_config, WorkflowManager};
use tempfile::TempDir;

fn setup_test_repo() -> (TempDir, WorkflowManager) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Create a basic Cargo.toml file for testing
    let cargo_toml_content = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
    std::fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml_content).unwrap();

    let config = default_git_flow_config();
    let manager = WorkflowManager::new(repo, config);
    (temp_dir, manager)
}

#[test]
fn test_prepare_and_validate_release_context() {
    let (_temp_dir, manager) = setup_test_repo();
    let version = "0.1.0-test";
    let release_branch = format!("release/{}", version);
    // Prepare context
    assert!(manager.prepare_release_context(&release_branch).is_ok());
    // Validate
    assert!(manager.validate_release(&release_branch).is_ok());
}

#[test]
#[ignore]
fn test_merge_and_cleanup_release_branch() {
    let (_temp_dir, manager) = setup_test_repo();
    let version = "0.2.0-test";
    let release_branch = format!("release/{}", version);
    // Prepare context
    assert!(manager.prepare_release_context(&release_branch).is_ok());
    // Merge to main
    let _ = manager.merge_to_main(&release_branch);
    // Merge to develop
    let _ = manager.merge_to_develop(&release_branch);
    // Cleanup
    assert!(manager.cleanup_release_branch(&release_branch).is_ok());
}
