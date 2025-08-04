use rhema_git::git::workflow::{
    WorkflowManager, WorkflowConfig, WorkflowType, BranchConventions, 
    ContextAwareWorkflowSettings, ContextAwareFeatureBranching,
    ContextAwareReleaseManagement, ContextAwareHotfixManagement,
    ContextAwareMergeStrategies, ContextMergeStrategy, ContextMergeStrategyType,
    ContextConflictResolution, ConflictResolutionType
};
use rhema_git::git::security::SecurityManager;
use rhema_core::RhemaResult;
use std::path::PathBuf;
use tempfile::TempDir;
use git2::Repository;

#[tokio::test]
async fn test_git_workflow_integration() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();
    
    // Initialize a Git repository
    let repo = Repository::init(repo_path)?;
    
    // Create initial commit
    let signature = git2::Signature::now("Test User", "test@example.com")?;
    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    repo.commit(
        Some("refs/heads/main"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[]
    )?;
    
    // Create develop branch
    let main_ref = repo.find_branch("main", git2::BranchType::Local)?;
    let main_commit = main_ref.get().peel_to_commit()?;
    repo.branch("develop", &main_commit, false)?;
    
    // Create workflow configuration
    let config = create_test_workflow_config();
    
    // Create workflow manager
    let workflow_manager = WorkflowManager::new(repo, config);
    
    // Test feature branch workflow
    test_feature_workflow(&workflow_manager).await?;
    
    // Test release workflow
    test_release_workflow(&workflow_manager).await?;
    
    // Test hotfix workflow
    test_hotfix_workflow(&workflow_manager).await?;
    
    Ok(())
}

fn create_test_workflow_config() -> WorkflowConfig {
    WorkflowConfig {
        workflow_type: WorkflowType::GitFlow,
        branch_conventions: BranchConventions {
            main_branch: "main".to_string(),
            develop_branch: Some("develop".to_string()),
            feature_prefix: "feature/".to_string(),
            release_prefix: "release/".to_string(),
            hotfix_prefix: "hotfix/".to_string(),
            support_prefix: "support/".to_string(),
        },
        context_rules: Default::default(),
        release_management: Default::default(),
        pull_request_settings: Default::default(),
        automation: Default::default(),
        advanced_features: Default::default(),
        context_aware: ContextAwareWorkflowSettings {
            context_aware_feature_branching: ContextAwareFeatureBranching {
                auto_isolate_context: true,
                auto_sync_parent: true,
                auto_validate_before_merge: true,
                auto_resolve_conflicts: false,
                inheritance_rules: vec![],
                boundary_rules: vec![],
                validation_rules: vec![],
            },
            context_aware_release_management: ContextAwareReleaseManagement {
                auto_prepare_context: true,
                auto_validate_release_context: true,
                auto_generate_release_notes: true,
                auto_update_version: true,
                validation_rules: vec![],
                preparation_steps: vec![],
                cleanup_steps: vec![],
            },
            context_aware_hotfix_management: ContextAwareHotfixManagement {
                auto_isolate_context: true,
                auto_validate_context: true,
                auto_merge_context: false,
                validation_rules: vec![],
                merge_strategies: vec![],
            },
            context_aware_pr_analysis: Default::default(),
            context_aware_merge_strategies: ContextAwareMergeStrategies {
                feature_merge_strategy: ContextMergeStrategy {
                    name: "Feature Merge".to_string(),
                    description: "Merge feature branches".to_string(),
                    strategy_type: ContextMergeStrategyType::Auto,
                    conflict_resolution: ContextConflictResolution {
                        resolution_type: ConflictResolutionType::Merge,
                        auto_resolve_simple: true,
                        manual_resolution_required: false,
                        resolution_rules: vec![],
                    },
                    validation_rules: vec![],
                },
                release_merge_strategy: ContextMergeStrategy {
                    name: "Release Merge".to_string(),
                    description: "Merge release branches".to_string(),
                    strategy_type: ContextMergeStrategyType::Auto,
                    conflict_resolution: ContextConflictResolution {
                        resolution_type: ConflictResolutionType::Merge,
                        auto_resolve_simple: true,
                        manual_resolution_required: false,
                        resolution_rules: vec![],
                    },
                    validation_rules: vec![],
                },
                hotfix_merge_strategy: ContextMergeStrategy {
                    name: "Hotfix Merge".to_string(),
                    description: "Merge hotfix branches".to_string(),
                    strategy_type: ContextMergeStrategyType::Auto,
                    conflict_resolution: ContextConflictResolution {
                        resolution_type: ConflictResolutionType::Merge,
                        auto_resolve_simple: true,
                        manual_resolution_required: false,
                        resolution_rules: vec![],
                    },
                    validation_rules: vec![],
                },
                custom_strategies: vec![],
            },
        },
        integrations: Default::default(),
    }
}

async fn test_feature_workflow(workflow_manager: &WorkflowManager) -> RhemaResult<()> {
    println!("Testing feature workflow...");
    
    let feature_branch = "feature/test-feature";
    
    // Test setup feature context
    workflow_manager.setup_feature_context(feature_branch).await?;
    println!("✓ Feature context setup completed");
    
    // Test validate feature branch
    workflow_manager.validate_feature_branch(feature_branch).await?;
    println!("✓ Feature branch validation completed");
    
    // Test merge feature branch
    workflow_manager.merge_feature_branch(feature_branch).await?;
    println!("✓ Feature branch merge completed");
    
    // Test cleanup feature branch
    workflow_manager.cleanup_feature_branch(feature_branch).await?;
    println!("✓ Feature branch cleanup completed");
    
    Ok(())
}

async fn test_release_workflow(workflow_manager: &WorkflowManager) -> RhemaResult<()> {
    println!("Testing release workflow...");
    
    let version = "1.0.0";
    
    // Test prepare release context
    workflow_manager.prepare_release_context(version).await?;
    println!("✓ Release context preparation completed");
    
    // Test validate release
    workflow_manager.validate_release(version).await?;
    println!("✓ Release validation completed");
    
    // Test merge to main
    workflow_manager.merge_to_main(version).await?;
    println!("✓ Release merge to main completed");
    
    // Test merge to develop
    workflow_manager.merge_to_develop(version).await?;
    println!("✓ Release merge to develop completed");
    
    // Test cleanup release branch
    workflow_manager.cleanup_release_branch(version).await?;
    println!("✓ Release branch cleanup completed");
    
    Ok(())
}

async fn test_hotfix_workflow(workflow_manager: &WorkflowManager) -> RhemaResult<()> {
    println!("Testing hotfix workflow...");
    
    let version = "1.0.1";
    
    // Test setup hotfix context
    workflow_manager.setup_hotfix_context(version).await?;
    println!("✓ Hotfix context setup completed");
    
    // Test validate hotfix
    workflow_manager.validate_hotfix(version).await?;
    println!("✓ Hotfix validation completed");
    
    // Test cleanup hotfix branch
    workflow_manager.cleanup_hotfix_branch(version).await?;
    println!("✓ Hotfix branch cleanup completed");
    
    Ok(())
}

#[tokio::test]
async fn test_workflow_status() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();
    
    // Initialize a Git repository
    let repo = Repository::init(repo_path)?;
    
    // Create initial commit
    let signature = git2::Signature::now("Test User", "test@example.com")?;
    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    repo.commit(
        Some("refs/heads/main"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[]
    )?;
    
    // Create workflow configuration
    let config = create_test_workflow_config();
    
    // Create workflow manager
    let workflow_manager = WorkflowManager::new(repo, config);
    
    // Test get workflow status
    let status = workflow_manager.get_workflow_status().await?;
    println!("Current workflow status: {:?}", status);
    
    // Test get current branch workflow
    let branch_workflow = workflow_manager.get_current_branch_workflow().await?;
    println!("Current branch workflow: {:?}", branch_workflow);
    
    Ok(())
}

#[tokio::test]
async fn test_context_aware_features() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();
    
    // Initialize a Git repository
    let repo = Repository::init(repo_path)?;
    
    // Create initial commit
    let signature = git2::Signature::now("Test User", "test@example.com")?;
    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    repo.commit(
        Some("refs/heads/main"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[]
    )?;
    
    // Create workflow configuration with context-aware features enabled
    let mut config = create_test_workflow_config();
    config.context_aware.context_aware_feature_branching.auto_isolate_context = true;
    config.context_aware.context_aware_feature_branching.auto_sync_parent = true;
    config.context_aware.context_aware_feature_branching.auto_validate_before_merge = true;
    
    // Create workflow manager
    let workflow_manager = WorkflowManager::new(repo, config);
    
    let feature_branch = "feature/context-aware-test";
    
    // Test context-aware feature workflow
    workflow_manager.setup_feature_context(feature_branch).await?;
    println!("✓ Context-aware feature setup completed");
    
    workflow_manager.validate_feature_branch(feature_branch).await?;
    println!("✓ Context-aware feature validation completed");
    
    workflow_manager.merge_feature_branch(feature_branch).await?;
    println!("✓ Context-aware feature merge completed");
    
    Ok(())
} 