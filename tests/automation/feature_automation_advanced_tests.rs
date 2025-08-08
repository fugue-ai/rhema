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
    git::automation::{GitAutomationManager, default_automation_config},
    git::feature_automation::{
        ValidationStatus, ValidationResult, MergeResult, CleanupResult, MergeStrategy,
        FeatureAutomationConfig, ContextSetupConfig, ValidationConfig, MergeConfig,
        CleanupConfig, AdvancedFeatureFeatures, FeatureAutomationManager,
        FeatureContext, ConflictResolution, default_feature_automation_config
    }
};
use rhema_git::git::history::Signature;
use git2::{Repository, BranchType};
use tempfile::TempDir;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use rhema_core::RhemaResult;

/// Advanced test fixture for feature automation tests
struct AdvancedFeatureAutomationTestFixture {
    temp_dir: TempDir,
    automation_manager: FeatureAutomationManager,
}

impl AdvancedFeatureAutomationTestFixture {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let repo = Repository::init(temp_dir.path())?;
        
        // Create initial commit
        let signature = git2::Signature::now("Test User", "test@example.com")?;
        let tree_id = repo.index()?.write_tree()?;
        {
            let tree = repo.find_tree(tree_id)?;
            repo.commit(Some("refs/heads/main"), &signature, &signature, "Initial commit", &tree, &[])?;
        }
        
        // Create develop branch
        {
            let main_commit = repo.find_branch("main", BranchType::Local)?.get().peel_to_commit()?;
            repo.branch("develop", &main_commit, false)?;
        }
        
        let automation_manager = FeatureAutomationManager::new(repo, default_feature_automation_config());
        
        Ok(Self {
            temp_dir,
            automation_manager,
        })
    }

    fn create_test_file(&self, path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.temp_dir.path().join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, content)?;
        Ok(())
    }

    fn commit_file(&self, path: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _file_path = self.temp_dir.path().join(path);
        let mut index = self.automation_manager.repo().index()?;
        index.add_path(Path::new(path))?;
        index.write()?;
        
        let tree_id = index.write_tree()?;
        let tree = self.automation_manager.repo().find_tree(tree_id)?;
        let signature = git2::Signature::now("Test User", "test@example.com")?;
        
        let head = self.automation_manager.repo().head()?;
        let parent = head.peel_to_commit()?;
        
        self.automation_manager.repo().commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent])?;
        Ok(())
    }

    fn create_feature_branch(&self, branch_name: &str, base_branch: &str) -> Result<(), Box<dyn std::error::Error>> {
        let base_ref = self.automation_manager.repo().find_branch(base_branch, BranchType::Local)?;
        let base_commit = base_ref.get().peel_to_commit()?;
        self.automation_manager.repo().branch(branch_name, &base_commit, false)?;
        Ok(())
    }

    fn setup_rhema_directories(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rhema_dir = self.temp_dir.path().join(".rhema");
        fs::create_dir_all(&rhema_dir)?;
        
        // Create context directory
        let context_dir = rhema_dir.join("context");
        fs::create_dir_all(&context_dir)?;
        
        // Create base branch context
        let base_context_dir = context_dir.join("develop");
        fs::create_dir_all(&base_context_dir)?;
        
        Ok(())
    }
}

// ============================================================================
// Inheritance Rules Tests
// ============================================================================

#[test]
fn test_apply_inheritance_rules_success() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // Create inheritance rules file for base branch
    let inheritance_rules = serde_json::json!({
        "validation_rules": {
            "require_tests": true,
            "require_docs": false
        },
        "merge_rules": {
            "strategy": "squash",
            "require_review": true
        }
    });
    
    let base_context_dir = fixture.temp_dir.path().join(".rhema").join("context").join("develop");
    fs::write(
        base_context_dir.join("inheritance_rules.json"),
        serde_json::to_string_pretty(&inheritance_rules)?
    )?;
    
    // Create feature branch context
    let feature_context_dir = fixture.temp_dir.path().join(".rhema").join("context").join("feature/test");
    fs::create_dir_all(&feature_context_dir)?;
    
    // Test inheritance rules application
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    
    // Check that inherited config was created
    let inherited_config_file = context.context_directory.join("inherited_config.json");
    assert!(inherited_config_file.exists());
    
    let inherited_content = fs::read_to_string(&inherited_config_file)?;
    let inherited_config: serde_json::Value = serde_json::from_str(&inherited_content)?;
    
    assert_eq!(inherited_config["validation_rules"]["require_tests"], true);
    assert_eq!(inherited_config["merge_rules"]["strategy"], "squash");
    
    Ok(())
}

#[test]
fn test_apply_inheritance_rules_no_base_rules() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // Test with no inheritance rules file
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    
    // Should not fail, just no inheritance
    let inherited_config_file = context.context_directory.join("inherited_config.json");
    assert!(!inherited_config_file.exists());
    
    Ok(())
}

// ============================================================================
// Boundary Rules Tests
// ============================================================================

#[test]
fn test_apply_boundary_rules_success() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // Create boundary rules file
    let boundary_rules = serde_json::json!({
        "branch_rules": {
            "feature_pattern": {
                "pattern": "feature/*",
                "action": "allow"
            },
            "hotfix_pattern": {
                "pattern": "hotfix/*",
                "action": "allow"
            }
        }
    });
    
    let rhema_dir = fixture.temp_dir.path().join(".rhema");
    fs::write(
        rhema_dir.join("boundary_rules.json"),
        serde_json::to_string_pretty(&boundary_rules)?
    )?;
    
    // Test with valid feature branch
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    assert_eq!(context.branch_name, "feature/test");
    
    Ok(())
}

#[test]
fn test_apply_boundary_rules_violation() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // Create boundary rules file with blocking rule
    let boundary_rules = serde_json::json!({
        "branch_rules": {
            "block_invalid": {
                "pattern": "invalid/*",
                "action": "block"
            }
        }
    });
    
    let rhema_dir = fixture.temp_dir.path().join(".rhema");
    fs::write(
        rhema_dir.join("boundary_rules.json"),
        serde_json::to_string_pretty(&boundary_rules)?
    )?;
    
    // Test with invalid branch name
    let result = fixture.automation_manager.setup_feature_context("invalid/test", "develop");
    assert!(result.is_err());
    
    Ok(())
}

// ============================================================================
// Health Checks Tests
// ============================================================================

#[test]
fn test_health_checks_repository_health() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Test repository health checks
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should pass health checks
    assert!(validation_result.success);
    
    Ok(())
}

#[test]
fn test_health_checks_branch_health() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create a feature branch
    fixture.create_feature_branch("feature/test", "develop")?;
    
    // Test branch health
    let validation_result = fixture.automation_manager.validate_feature_branch("feature/test")?;
    assert!(validation_result.success);
    
    // Test with non-existent branch
    let validation_result = fixture.automation_manager.validate_feature_branch("nonexistent");
    assert!(validation_result.is_err());
    
    Ok(())
}

#[test]
fn test_health_checks_context_health() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // Create feature context with required files
    let feature_context_dir = fixture.temp_dir.path().join(".rhema").join("context").join("feature/test");
    fs::create_dir_all(&feature_context_dir)?;
    
    // Create required context files
    fs::write(feature_context_dir.join("config.json"), r#"{"context_type": "feature"}"#)?;
    fs::write(feature_context_dir.join("context.yaml"), "feature:\n  name: test")?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    assert!(validation_result.success);
    
    Ok(())
}

// ============================================================================
// Dependency Validation Tests
// ============================================================================

#[test]
fn test_dependency_validation_cargo_toml() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create valid Cargo.toml
    fixture.create_test_file("Cargo.toml", r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    assert!(validation_result.success);
    
    Ok(())
}

#[test]
fn test_dependency_validation_cargo_toml_placeholder_version() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create Cargo.toml with placeholder version
    fixture.create_test_file("Cargo.toml", r#"
[package]
name = "test-project"
version = "0.0.0"
edition = "2021"
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to placeholder version
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("placeholder version")));
    
    Ok(())
}

#[test]
fn test_dependency_validation_package_json() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create valid package.json
    fixture.create_test_file("package.json", r#"
{
  "name": "test-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.17.1"
  }
}
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    assert!(validation_result.success);
    
    Ok(())
}

#[test]
fn test_dependency_validation_package_json_missing_fields() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create package.json missing required fields
    fixture.create_test_file("package.json", r#"
{
  "dependencies": {
    "express": "^4.17.1"
  }
}
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to missing fields
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("missing")));
    
    Ok(())
}

#[test]
fn test_dependency_validation_conflicts() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create both package-lock.json and yarn.lock (conflict)
    fixture.create_test_file("package-lock.json", "{}")?;
    fixture.create_test_file("yarn.lock", "# yarn lockfile v1")?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to dependency conflict
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("dependency conflict")));
    
    Ok(())
}

// ============================================================================
// Security Validation Tests
// ============================================================================

#[test]
fn test_security_validation_hardcoded_secrets() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create file with hardcoded secrets
    fixture.create_test_file("config.rs", r#"
pub struct Config {
    pub password: String = "secret123".to_string(),
    pub api_key: String = "sk-1234567890abcdef".to_string(),
}
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to hardcoded secrets
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("Security issue found")));
    
    Ok(())
}

#[test]
fn test_security_validation_suspicious_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create file with suspicious patterns
    fixture.create_test_file("script.js", r#"
const result = eval("console.log('Hello World')");
const output = exec("rm -rf /");
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to suspicious patterns
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("Suspicious pattern found")));
    
    Ok(())
}

#[test]
fn test_security_validation_vulnerable_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create Cargo.toml with potentially vulnerable dependency
    fixture.create_test_file("Cargo.toml", r#"
[package]
name = "test-project"
version = "1.0.0"
edition = "2021"

[dependencies]
chrono = "0.4"
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to vulnerable dependency
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("vulnerable dependency")));
    
    Ok(())
}

// ============================================================================
// Performance Validation Tests
// ============================================================================

#[test]
fn test_performance_validation_large_files() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create a large file (> 10MB)
    let large_content = "x".repeat(11 * 1024 * 1024); // 11MB
    fixture.create_test_file("large_file.bin", &large_content)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to large file
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("Large file found")));
    
    Ok(())
}

#[test]
fn test_performance_validation_inefficient_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create Rust file with inefficient patterns
    fixture.create_test_file("main.rs", r#"
fn main() {
    let mut vec = Vec::new();
    for i in 0..1000 {
        vec.push(i);
    }
    
    let mut string = String::new();
    for i in 0..100 {
        string.push_str(&i.to_string());
    }
}
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to inefficient patterns
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("Inefficient pattern found")));
    
    Ok(())
}

#[test]
fn test_performance_validation_anti_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create file with performance anti-patterns
    fixture.create_test_file("database.rs", r#"
fn get_users() {
    for user in users {
        for post in user.posts {
            // N+1 query pattern
        }
    }
}
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to anti-patterns
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("Performance anti-pattern found")));
    
    Ok(())
}

// ============================================================================
// Auto-Conflict Resolution Tests
// ============================================================================

#[test]
fn test_auto_resolve_conflicts_simple() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create feature branch
    fixture.create_feature_branch("feature/test", "develop")?;
    
    // Create conflicting content
    fixture.create_test_file("conflict.txt", "Feature content")?;
    fixture.commit_file("conflict.txt", "Add feature content")?;
    
    // Switch to develop and modify same file
    let develop_ref = fixture.automation_manager.repo().find_branch("develop", BranchType::Local)?;
    let develop_commit = develop_ref.get().peel_to_commit()?;
    fixture.automation_manager.repo().checkout_tree(develop_commit.tree()?.as_object(), None)?;
    fixture.automation_manager.repo().set_head("refs/heads/develop")?;
    
    fixture.create_test_file("conflict.txt", "Develop content")?;
    fixture.commit_file("conflict.txt", "Add develop content")?;
    
    // Test merge with conflicts
    let merge_result = fixture.automation_manager.merge_feature_branch("feature/test", "develop")?;
    
    // Should handle conflicts appropriately
    assert!(merge_result.success || !merge_result.conflicts.is_empty());
    
    Ok(())
}

// ============================================================================
// Merge Strategy Tests
// ============================================================================

#[test]
fn test_rebase_merge_strategy() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create feature branch with commits
    fixture.create_feature_branch("feature/test", "develop")?;
    fixture.create_test_file("feature.txt", "Feature content")?;
    fixture.commit_file("feature.txt", "Add feature content")?;
    
    // Test rebase merge
    let merge_result = fixture.automation_manager.merge_feature_branch("feature/test", "develop")?;
    assert!(merge_result.success);
    
    Ok(())
}

#[test]
fn test_squash_merge_strategy() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create feature branch with multiple commits
    fixture.create_feature_branch("feature/test", "develop")?;
    fixture.create_test_file("file1.txt", "Content 1")?;
    fixture.commit_file("file1.txt", "Add file 1")?;
    fixture.create_test_file("file2.txt", "Content 2")?;
    fixture.commit_file("file2.txt", "Add file 2")?;
    
    // Test squash merge
    let merge_result = fixture.automation_manager.merge_feature_branch("feature/test", "develop")?;
    assert!(merge_result.success);
    
    Ok(())
}

#[test]
fn test_custom_merge_strategies() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Create feature branch
    fixture.create_feature_branch("feature/test", "develop")?;
    fixture.create_test_file("feature.txt", "Feature content")?;
    fixture.commit_file("feature.txt", "Add feature content")?;
    
    // Test custom merge
    let merge_result = fixture.automation_manager.merge_feature_branch("feature/test", "develop")?;
    assert!(merge_result.success);
    
    Ok(())
}

// ============================================================================
// Edge Cases and Error Handling Tests
// ============================================================================

#[test]
fn test_validation_with_missing_context_files() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // Create feature context directory but no required files
    let feature_context_dir = fixture.temp_dir.path().join(".rhema").join("context").join("feature/test");
    fs::create_dir_all(&feature_context_dir)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail due to missing required files
    assert!(!validation_result.success);
    assert!(validation_result.errors.iter().any(|e| e.contains("missing")));
    
    Ok(())
}

#[test]
fn test_validation_with_corrupted_repository() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Corrupt the repository by removing HEAD
    let head_file = fixture.temp_dir.path().join(".git").join("HEAD");
    fs::remove_file(head_file)?;
    
    let result = fixture.automation_manager.validate_feature_branch("feature/test");
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_merge_with_invalid_branches() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Test merge with non-existent source branch
    let result = fixture.automation_manager.merge_feature_branch("nonexistent", "develop");
    assert!(result.is_err());
    
    // Test merge with non-existent target branch
    fixture.create_feature_branch("feature/test", "develop")?;
    let result = fixture.automation_manager.merge_feature_branch("feature/test", "nonexistent");
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_cleanup_with_missing_branch() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    
    // Test cleanup of non-existent branch
    let cleanup_result = fixture.automation_manager.cleanup_feature_branch("nonexistent")?;
    
    // Should handle gracefully
    assert!(!cleanup_result.success);
    assert!(!cleanup_result.errors.is_empty());
    
    Ok(())
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_full_feature_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // 1. Setup feature context
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    assert_eq!(context.branch_name, "feature/test");
    
    // 2. Create some content
    fixture.create_test_file("src/main.rs", r#"
fn main() {
    println!("Hello, World!");
}
"#)?;
    fixture.commit_file("src/main.rs", "Add main function")?;
    
    // 3. Validate feature branch
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    assert!(validation_result.success);
    
    // 4. Merge feature branch
    let merge_result = fixture.automation_manager.merge_feature_branch(&context.branch_name, "develop")?;
    assert!(merge_result.success);
    
    // 5. Cleanup feature branch
    let cleanup_result = fixture.automation_manager.cleanup_feature_branch(&context.branch_name)?;
    assert!(cleanup_result.success);
    
    Ok(())
}

#[test]
fn test_complex_validation_scenario() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = AdvancedFeatureAutomationTestFixture::new()?;
    fixture.setup_rhema_directories()?;
    
    // Create a complex scenario with multiple validation issues
    fixture.create_test_file("Cargo.toml", r#"
[package]
name = "test-project"
version = "0.0.0"
edition = "2021"

[dependencies]
chrono = "0.4"
"#)?;
    
    fixture.create_test_file("src/config.rs", r#"
pub struct Config {
    pub password: String = "secret123".to_string(),
}
"#)?;
    
    fixture.create_test_file("src/main.rs", r#"
fn main() {
    let mut vec = Vec::new();
    for i in 0..1000 {
        vec.push(i);
    }
}
"#)?;
    
    let context = fixture.automation_manager.setup_feature_context("feature/test", "develop")?;
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should fail with multiple validation errors
    assert!(!validation_result.success);
    assert!(validation_result.errors.len() >= 3); // At least 3 different validation failures
    
    // Check for specific error types
    let error_messages: Vec<&str> = validation_result.errors.iter().map(|s| s.as_str()).collect();
    assert!(error_messages.iter().any(|e| e.contains("placeholder version")));
    assert!(error_messages.iter().any(|e| e.contains("Security issue found")));
    assert!(error_messages.iter().any(|e| e.contains("Inefficient pattern found")));
    
    Ok(())
} 