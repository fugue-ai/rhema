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

use git2::{BranchType, Repository, Signature};
use rhema_cli::Rhema;
use rhema_core::RhemaResult;
use rhema_core::ValidationResult;
use rhema_git::{
    git::security::Operation,
    git_basic::{
        AdvancedGitIntegration, AutomationTask, BranchContext, BranchManager, ContextBlame,
        ContextEvolution, MergeResult, PullRequestAnalysis, WorkflowType,
    },
    git_hooks::HookType,
};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test fixture for Git integration tests
struct GitIntegrationTestFixture {
    temp_dir: TempDir,
    rhema: Rhema,
    git_integration: AdvancedGitIntegration,
}

impl GitIntegrationTestFixture {
    fn new() -> RhemaResult<Self> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path();

        // Initialize Git repository
        let repo = Repository::init(repo_path)?;

        // Create initial commit
        let signature = Signature::new("Test User", "test@example.com", &git2::Time::new(0, 0))?;
        let tree_id = repo.index()?.write_tree()?;
        {
            let tree = repo.find_tree(tree_id)?;
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Initial commit",
                &tree,
                &[],
            )?;
        }

        // Initialize Rhema
        let rhema = Rhema::new()?;

        // Create Git integration
        let git_integration = AdvancedGitIntegration::new(repo)?;

        Ok(Self {
            temp_dir,
            rhema,
            git_integration,
        })
    }

    fn create_test_file(&self, path: &str, content: &str) -> RhemaResult<()> {
        let file_path = self.temp_dir.path().join(path);
        fs::create_dir_all(file_path.parent().unwrap())?;
        fs::write(file_path, content)?;
        Ok(())
    }

    fn create_context_file(&self, name: &str, content: &str) -> RhemaResult<()> {
        let context_content = format!(
            r#"
scopes:
  test:
    name: "Test Scope"
    description: "Test scope for integration testing"
    
knowledge:
  test-knowledge:
    title: "Test Knowledge"
    content: "{}"
    tags: ["test", "integration"]
    
todos:
  test-todo:
    title: "Test Todo"
    description: "{}"
    priority: "medium"
    status: "pending"
"#,
            content, content
        );
        self.create_test_file(&format!("context/{}.yaml", name), &context_content)
    }
}

#[test]
fn test_advanced_git_integration_initialization() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test initialization
    fixture.git_integration.initialize()?;

    // Verify .rhema directory was created
    let rhema_dir = fixture.temp_dir.path().join(".rhema");
    assert!(rhema_dir.exists());

    // Verify configuration file was created
    let config_file = rhema_dir.join("git-integration.yaml");
    assert!(config_file.exists());

    Ok(())
}

#[test]
fn test_advanced_hooks_installation() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test hook installation
    let hook_result = fixture.git_integration.execute_hook(HookType::PreCommit)?;
    assert!(hook_result.success);

    // Test hook status
    let integration_status = fixture.git_integration.get_integration_status()?;
    assert!(integration_status.hooks_installed);

    Ok(())
}

#[test]
fn test_branch_context_management() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create test context files
    fixture.create_context_file("main", "Main branch context")?;
    fixture.create_context_file("feature", "Feature branch context")?;

    // Test branch context initialization
    let mut branch_manager = fixture.git_integration.branches();
    let context = branch_manager?.initialize_branch_context(Some("main".to_string()))?;
    assert_eq!(context.name, "main");

    // Test branch context validation
    let validation_status = fixture.git_integration.validate_branch_context()?;
    assert!(validation_status.is_valid);

    // Test branch context backup
    let backup_path = fixture.git_integration.backup_branch_context("main")?;
    assert!(backup_path.exists());

    Ok(())
}

#[test]
fn test_workflow_integration() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test feature branch creation
    let feature_branch = fixture
        .git_integration
        .create_feature_branch("test-feature", "develop")?;
    assert_eq!(feature_branch.name, "feature/test-feature");

    // Test feature branch finishing
    let result = fixture
        .git_integration
        .finish_feature_branch("test-feature")?;
    assert!(result.success);

    // Test release branch creation
    let release_branch = fixture.git_integration.start_release_branch("1.0.0")?;
    assert_eq!(release_branch.name, "release/1.0.0");

    // Test release branch finishing
    let result = fixture.git_integration.finish_release_branch("1.0.0")?;
    assert!(result.success);

    Ok(())
}

#[test]
fn test_context_history_tracking() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create test context files
    fixture.create_context_file("test", "Initial context")?;

    // Commit changes
    let signature = Signature::new("Test User", "test@example.com", &git2::Time::new(0, 0))?;
    let mut index = fixture.git_integration.repository().index()?;
    index.add_path(std::path::Path::new("context/test.yaml"))?;
    let tree_id = index.write_tree()?;
    let tree = fixture.git_integration.repository().find_tree(tree_id)?;
    fixture.git_integration.repository().commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Add test context",
        &tree,
        &[],
    )?;

    // Test context evolution tracking
    let evolution = fixture
        .git_integration
        .track_context_evolution(".", Some(10))?;
    assert!(!evolution.entries.is_empty());

    // Test context blame
    let blame = fixture
        .git_integration
        .get_context_blame("context/test.yaml")?;
    assert!(!blame.entries.is_empty());

    // Test context version creation
    let context_version =
        fixture
            .git_integration
            .create_context_version("1.0.0", "patch", "Test version")?;
    assert_eq!(context_version.version, "1.0.0");

    Ok(())
}

#[test]
fn test_automation_features() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test automation start
    fixture.git_integration.start_automation()?;

    // Test automation status
    let automation_status = fixture.git_integration.get_automation_status()?;
    assert!(automation_status.running);

    // Test task history
    let task_history = fixture.git_integration.get_task_history(Some(10));
    assert!(!task_history?.is_empty());

    // Test automation stop
    fixture.git_integration.stop_automation()?;

    let automation_status = fixture.git_integration.get_automation_status()?;
    assert!(!automation_status.running);

    Ok(())
}

#[test]
fn test_security_features() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test security scan
    let scan_result = fixture
        .git_integration
        .run_security_scan(fixture.temp_dir.path().to_str().unwrap())?;
    assert!(scan_result.issues.is_empty());

    // Test access validation
    let has_access = fixture.git_integration.validate_access(
        "test-user",
        &Operation::Read,
        "context/test.yaml",
    )?;
    assert!(has_access);

    // Test commit security validation
    let head = fixture.git_integration.repository().head()?;
    let commit = head.peel_to_commit()?;
    let validation_result = fixture
        .git_integration
        .validate_commit_security(&commit.id().to_string())?;
    assert!(validation_result.is_valid);

    Ok(())
}

#[test]
fn test_monitoring_features() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test monitoring start
    fixture.git_integration.start_monitoring()?;

    // Test monitoring status
    let monitoring_status = fixture.git_integration.get_monitoring_status()?;
    assert!(monitoring_status.is_active);

    // Test Git operation recording
    fixture
        .git_integration
        .record_git_operation("test-operation", chrono::Duration::seconds(1))?;

    // Test context operation recording
    fixture.git_integration.record_context_operation(
        "test-context-operation",
        chrono::Duration::milliseconds(500),
    )?;

    // Test monitoring stop
    fixture.git_integration.stop_monitoring()?;

    let monitoring_status = fixture.git_integration.get_monitoring_status()?;
    assert!(!monitoring_status.is_active);

    Ok(())
}

#[test]
fn test_context_conflict_detection() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create conflicting context files
    fixture.create_context_file("conflict", "Original content")?;

    // Create a branch and modify the file
    let signature = Signature::new("Test User", "test@example.com", &git2::Time::new(0, 0))?;
    let mut index = fixture.git_integration.repository().index()?;
    index.add_path(std::path::Path::new("context/conflict.yaml"))?;
    let tree_id = index.write_tree()?;
    {
        let tree = fixture.git_integration.repository().find_tree(tree_id)?;
        fixture.git_integration.repository().commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Add conflict file",
            &tree,
            &[],
        )?;
    }

    // Create feature branch
    let feature_branch = fixture
        .git_integration
        .create_feature_branch("conflict-test", "main")?;

    // Modify file in feature branch
    fixture.create_context_file("conflict", "Modified content in feature branch")?;

    // Test conflict detection
    let conflicts = fixture.git_integration.detect_conflicts()?;
    assert!(!conflicts.is_empty());

    Ok(())
}

#[test]
fn test_context_merge_strategies() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create source and target context files
    fixture.create_context_file("source", "Source branch context")?;
    fixture.create_context_file("target", "Target branch context")?;

    // Test merge strategies
    let merge_result = fixture
        .git_integration
        .merge_branch_context("source", "target")?;
    assert!(merge_result.success);
    assert!(merge_result.conflicts.is_empty());

    Ok(())
}

#[test]
fn test_pull_request_analysis() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create test context for PR analysis
    fixture.create_context_file("pr-test", "PR test context")?;

    // Test PR analysis (mock)
    let analysis = fixture.git_integration.analyze_pull_request(1)?;
    assert!(!analysis.context_changes.is_empty());

    Ok(())
}

#[test]
fn test_context_versioning() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create test context
    fixture.create_context_file("version-test", "Version test context")?;

    // Create multiple versions
    let version1 =
        fixture
            .git_integration
            .create_context_version("1.0.0", "major", "Major version")?;

    let version2 =
        fixture
            .git_integration
            .create_context_version("1.1.0", "minor", "Minor version")?;

    assert_eq!(version1.version, "1.0.0");
    assert_eq!(version2.version, "1.1.0");

    // Test rollback
    fixture.git_integration.rollback_to_version("1.0.0")?;

    Ok(())
}

#[test]
fn test_advanced_hook_configuration() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test advanced hook execution
    let hook_result = fixture
        .git_integration
        .execute_hook(rhema_git::git_hooks::HookType::PreCommit)?;
    assert!(hook_result.success);

    Ok(())
}

#[test]
fn test_workflow_integration_configuration() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test workflow initialization
    fixture.git_integration.initialize()?;

    // Test workflow status
    let workflow_status = fixture.git_integration.get_workflow_status()?;
    assert_eq!(format!("{:?}", workflow_status.workflow_type), "GitFlow");

    Ok(())
}

#[test]
fn test_automation_integration_configuration() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Test automation start
    fixture.git_integration.start_automation()?;

    // Test automation status
    let automation_status = fixture.git_integration.get_automation_status()?;
    assert!(automation_status.running);

    // Test automation stop
    fixture.git_integration.stop_automation()?;

    Ok(())
}

#[test]
fn test_integration_error_handling() -> RhemaResult<()> {
    // Test with non-existent repository
    let temp_dir = tempfile::tempdir()?;
    let result = Repository::init(temp_dir.path());
    assert!(result.is_err());

    // Test with valid repository
    let mut fixture = GitIntegrationTestFixture::new()?;
    let mut integration = fixture.git_integration;

    // Test that integration doesn't fail
    integration.initialize()?;

    Ok(())
}

#[test]
fn test_performance_monitoring() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Start monitoring
    fixture.git_integration.start_monitoring()?;

    // Record various operations
    fixture
        .git_integration
        .record_git_operation("commit", chrono::Duration::milliseconds(150))?;
    fixture
        .git_integration
        .record_git_operation("push", chrono::Duration::milliseconds(300))?;
    fixture
        .git_integration
        .record_context_operation("validate", chrono::Duration::milliseconds(75))?;
    fixture
        .git_integration
        .record_context_operation("update", chrono::Duration::milliseconds(100))?;

    // Get monitoring status
    let monitoring_status = fixture.git_integration.get_monitoring_status()?;
    assert!(monitoring_status.is_active);
    assert!(monitoring_status.metrics_count > 0);

    // Stop monitoring
    fixture.git_integration.stop_monitoring()?;

    Ok(())
}

#[test]
fn test_context_evolution_analytics() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create multiple context changes
    for i in 1..=5 {
        fixture.create_context_file(
            &format!("evolution-{}", i),
            &format!("Evolution content {}", i),
        )?;

        // Commit changes
        let signature = Signature::new("Test User", "test@example.com", &git2::Time::new(0, 0))?;
        let mut index = fixture.git_integration.repository().index()?;
        index.add_path(std::path::Path::new(&format!(
            "context/evolution-{}.yaml",
            i
        )))?;
        let tree_id = index.write_tree()?;
        let tree = fixture.git_integration.repository().find_tree(tree_id)?;
        fixture.git_integration.repository().commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!("Add evolution {}", i),
            &tree,
            &[],
        )?;
    }

    // Test evolution tracking
    let evolution = fixture
        .git_integration
        .track_context_evolution(".", Some(10))?;
    assert_eq!(evolution.entries.len(), 5);

    // Test evolution report
    let report = fixture
        .git_integration
        .generate_evolution_report(".", None)?;
    assert_eq!(report.total_commits, 5);
    assert!(!report.changes_by_type.is_empty());
    assert!(!report.top_contributors.is_empty());

    Ok(())
}

#[test]
fn test_security_validation() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Create test files with different security characteristics
    fixture.create_test_file("secure.yaml", "secure: true\nencrypted: true")?;
    fixture.create_test_file("insecure.yaml", "password: secret123\napi_key: abc123")?;

    // Test security scan
    let scan_result = fixture
        .git_integration
        .run_security_scan(fixture.temp_dir.path().to_str().unwrap())?;
    assert!(!scan_result.issues.is_empty());

    // Test access validation
    let read_access =
        fixture
            .git_integration
            .validate_access("user1", &Operation::Read, "secure.yaml")?;
    assert!(read_access);

    let write_access =
        fixture
            .git_integration
            .validate_access("user2", &Operation::Write, "insecure.yaml")?;
    assert!(write_access);

    Ok(())
}

#[test]
fn test_integration_shutdown() -> RhemaResult<()> {
    let mut fixture = GitIntegrationTestFixture::new()?;

    // Initialize integration
    fixture.git_integration.initialize()?;

    // Start automation and monitoring
    fixture.git_integration.start_automation()?;
    fixture.git_integration.start_monitoring()?;

    // Test shutdown
    fixture.git_integration.shutdown()?;

    // Verify shutdown state
    let automation_status = fixture.git_integration.get_automation_status()?;
    assert!(!automation_status.running);

    let monitoring_status = fixture.git_integration.get_monitoring_status()?;
    assert!(!monitoring_status.is_active);

    Ok(())
}
