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

use rhema_git::feature_automation::{
    FeatureAutomationManager, default_feature_automation_config,
    FeatureAutomationConfig, ContextSetupConfig, ValidationConfig,
    MergeConfig, CleanupConfig, AdvancedFeatureFeatures,
    MergeStrategy, ConflictResolution
};
use git2::{Repository, Signature};
use tempfile::TempDir;
use std::fs;
use std::path::Path;

/// Test fixture for feature automation tests
struct FeatureAutomationTestFixture {
    temp_dir: TempDir,
    repo: Repository,
    automation_manager: FeatureAutomationManager,
}

impl FeatureAutomationTestFixture {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let repo = Repository::init(temp_dir.path())?;
        
        // Create initial commit
        let signature = Signature::now("Test User", "test@example.com")?;
        let tree_id = repo.index()?.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        repo.commit(Some("refs/heads/main"), &signature, &signature, "Initial commit", &tree, &[])?;
        
        // Create develop branch
        let main_commit = repo.find_branch("main", git2::BranchType::Local)?.get().peel_to_commit()?;
        repo.branch("develop", &main_commit, false)?;
        
        let config = default_feature_automation_config();
        let automation_manager = FeatureAutomationManager::new(repo, config);
        
        Ok(Self {
            temp_dir,
            repo: automation_manager.repo.clone(),
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
        let file_path = self.temp_dir.path().join(path);
        let mut index = self.repo.index()?;
        index.add_path(Path::new(path))?;
        index.write()?;
        
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let signature = Signature::now("Test User", "test@example.com")?;
        
        let head = self.repo.head()?;
        let parent = head.peel_to_commit()?;
        
        self.repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent])?;
        Ok(())
    }
}

#[test]
fn test_setup_feature_context() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Test setting up feature context
    let context = fixture.automation_manager.setup_feature_context("test-feature", "develop")?;
    
    assert_eq!(context.branch_name, "test-feature");
    assert_eq!(context.base_branch, "develop");
    assert!(context.context_directory.exists());
    assert_eq!(context.validation_status, rhema_git::feature_automation::ValidationStatus::Pending);
    assert_eq!(context.merge_status, rhema_git::feature_automation::MergeStatus::NotStarted);
    
    // Check that context files were created
    let config_file = context.context_directory.join("config.json");
    assert!(config_file.exists());
    
    let config_content = fs::read_to_string(&config_file)?;
    let config: serde_json::Value = serde_json::from_str(&config_content)?;
    assert_eq!(config["context_type"], "feature");
    
    Ok(())
}

#[test]
fn test_validate_feature_branch() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Create a feature branch first
    let context = fixture.automation_manager.setup_feature_context("test-feature", "develop")?;
    
    // Test validation
    let validation_result = fixture.automation_manager.validate_feature_branch(&context.branch_name)?;
    
    // Should pass basic validation
    assert!(validation_result.success);
    assert_eq!(validation_result.status, rhema_git::feature_automation::ValidationStatus::Passed);
    
    Ok(())
}

#[test]
fn test_validate_feature_branch_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Test validation of non-existent branch
    let validation_result = fixture.automation_manager.validate_feature_branch("nonexistent-branch")?;
    
    // Should fail validation
    assert!(!validation_result.success);
    assert!(matches!(validation_result.status, rhema_git::feature_automation::ValidationStatus::Failed(_)));
    assert!(!validation_result.errors.is_empty());
    
    Ok(())
}

#[test]
fn test_merge_feature_branch() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Create a feature branch
    let context = fixture.automation_manager.setup_feature_context("test-feature", "develop")?;
    
    // Create some content in the feature branch
    fixture.create_test_file("feature-file.txt", "Feature content")?;
    fixture.commit_file("feature-file.txt", "Add feature file")?;
    
    // Test merging
    let merge_result = fixture.automation_manager.merge_feature_branch(&context.branch_name, "develop")?;
    
    // Should merge successfully
    assert!(merge_result.success);
    assert_eq!(merge_result.source_branch, context.branch_name);
    assert_eq!(merge_result.target_branch, "develop");
    
    Ok(())
}

#[test]
fn test_cleanup_feature_branch() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Create a feature branch
    let context = fixture.automation_manager.setup_feature_context("test-feature", "develop")?;
    
    // Test cleanup
    let cleanup_result = fixture.automation_manager.cleanup_feature_branch(&context.branch_name)?;
    
    // Should cleanup successfully
    assert!(cleanup_result.success);
    assert_eq!(cleanup_result.branch_name, context.branch_name);
    assert!(!cleanup_result.messages.is_empty());
    
    Ok(())
}

#[test]
fn test_custom_automation_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let repo = Repository::init(temp_dir.path())?;
    
    // Create custom configuration
    let config = FeatureAutomationConfig {
        auto_context_setup: true,
        auto_validation: false,
        auto_merging: true,
        auto_cleanup: false,
        context_setup: ContextSetupConfig {
            create_context_directory: true,
            initialize_config: true,
            apply_inheritance_rules: false,
            apply_boundary_rules: false,
            setup_isolation: true,
            create_templates: false,
            initialize_tracking: false,
        },
        validation: ValidationConfig {
            validate_branch_existence: true,
            validate_context_integrity: false,
            validate_uncommitted_changes: false,
            run_health_checks: false,
            run_tests: false,
            validate_dependencies: false,
            validate_security: false,
            validate_performance: false,
            custom_validation_commands: vec!["echo 'test'".to_string()],
        },
        merge: MergeConfig {
            strategy: MergeStrategy::FastForward,
            conflict_resolution: ConflictResolution::Auto,
            pre_merge_validation: false,
            post_merge_validation: false,
            auto_resolve_simple: true,
            require_manual_resolution: false,
            create_merge_commit: false,
            squash_commits: false,
            delete_source_branch: true,
        },
        cleanup: CleanupConfig {
            delete_branch: true,
            cleanup_context_files: false,
            cleanup_temp_files: false,
            cleanup_backups: false,
            archive_context: false,
            update_references: false,
            notify_stakeholders: false,
        },
        advanced_features: AdvancedFeatureFeatures {
            context_evolution_tracking: false,
            context_analytics: false,
            context_optimization: false,
            predictive_merging: false,
            intelligent_conflict_resolution: false,
            automated_testing: false,
            performance_monitoring: false,
        },
    };
    
    let automation_manager = FeatureAutomationManager::new(repo, config);
    
    // Test that custom config is applied
    assert!(automation_manager.config.auto_context_setup);
    assert!(!automation_manager.config.auto_validation);
    assert!(automation_manager.config.auto_merging);
    assert!(!automation_manager.config.auto_cleanup);
    
    Ok(())
}

#[test]
fn test_feature_context_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Create feature context
    let context = fixture.automation_manager.setup_feature_context("test-feature", "develop")?;
    
    // Test serialization
    let json = serde_json::to_string(&context)?;
    let deserialized_context: rhema_git::feature_automation::FeatureContext = serde_json::from_str(&json)?;
    
    assert_eq!(context.branch_name, deserialized_context.branch_name);
    assert_eq!(context.base_branch, deserialized_context.base_branch);
    assert_eq!(context.config.context_type, deserialized_context.config.context_type);
    
    Ok(())
}

#[test]
fn test_validation_result_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Create validation result
    let validation_result = rhema_git::feature_automation::ValidationResult {
        success: true,
        status: rhema_git::feature_automation::ValidationStatus::Passed,
        errors: vec!["Error 1".to_string()],
        warnings: vec!["Warning 1".to_string()],
    };
    
    // Test serialization
    let json = serde_json::to_string(&validation_result)?;
    let deserialized_result: rhema_git::feature_automation::ValidationResult = serde_json::from_str(&json)?;
    
    assert_eq!(validation_result.success, deserialized_result.success);
    assert_eq!(validation_result.errors.len(), deserialized_result.errors.len());
    assert_eq!(validation_result.warnings.len(), deserialized_result.warnings.len());
    
    Ok(())
}

#[test]
fn test_merge_result_serialization() -> Result<(), Box<dyn std::error::Error>> {
    // Create merge result
    let merge_result = rhema_git::feature_automation::MergeResult {
        success: true,
        target_branch: "develop".to_string(),
        source_branch: "feature/test".to_string(),
        conflicts: vec!["conflict1".to_string()],
        messages: vec!["Merge successful".to_string()],
    };
    
    // Test serialization
    let json = serde_json::to_string(&merge_result)?;
    let deserialized_result: rhema_git::feature_automation::MergeResult = serde_json::from_str(&json)?;
    
    assert_eq!(merge_result.success, deserialized_result.success);
    assert_eq!(merge_result.target_branch, deserialized_result.target_branch);
    assert_eq!(merge_result.source_branch, deserialized_result.source_branch);
    assert_eq!(merge_result.conflicts.len(), deserialized_result.conflicts.len());
    assert_eq!(merge_result.messages.len(), deserialized_result.messages.len());
    
    Ok(())
}

#[test]
fn test_cleanup_result_serialization() -> Result<(), Box<dyn std::error::Error>> {
    // Create cleanup result
    let cleanup_result = rhema_git::feature_automation::CleanupResult {
        success: true,
        branch_name: "feature/test".to_string(),
        messages: vec!["Cleanup successful".to_string()],
        errors: vec!["Error during cleanup".to_string()],
    };
    
    // Test serialization
    let json = serde_json::to_string(&cleanup_result)?;
    let deserialized_result: rhema_git::feature_automation::CleanupResult = serde_json::from_str(&json)?;
    
    assert_eq!(cleanup_result.success, deserialized_result.success);
    assert_eq!(cleanup_result.branch_name, deserialized_result.branch_name);
    assert_eq!(cleanup_result.messages.len(), deserialized_result.messages.len());
    assert_eq!(cleanup_result.errors.len(), deserialized_result.errors.len());
    
    Ok(())
}

#[test]
fn test_default_config() {
    let config = default_feature_automation_config();
    
    // Test default values
    assert!(config.auto_context_setup);
    assert!(config.auto_validation);
    assert!(config.auto_merging);
    assert!(config.auto_cleanup);
    
    // Test nested configs
    assert!(config.context_setup.create_context_directory);
    assert!(config.validation.validate_branch_existence);
    assert!(matches!(config.merge.strategy, MergeStrategy::Merge));
    assert!(config.cleanup.delete_branch);
    assert!(config.advanced_features.context_evolution_tracking);
}

#[test]
fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = FeatureAutomationTestFixture::new()?;
    
    // Test with invalid branch name
    let result = fixture.automation_manager.validate_feature_branch("");
    assert!(result.is_err());
    
    // Test with invalid base branch
    let result = fixture.automation_manager.setup_feature_context("test", "invalid-base");
    assert!(result.is_err());
    
    Ok(())
} 