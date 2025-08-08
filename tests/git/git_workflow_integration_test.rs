use rhema_git::git::workflow::{
    GitWorkflow, WorkflowConfig, default_git_flow_config,
    ContextRules, IsolationRules, ReleaseManagement, BranchPreparation,
    ReleaseValidation, ReleaseAutomation, PullRequestSettings,
    AutomationSettings, AdvancedWorkflowFeatures, ContextAwarePrAnalysis,
    WorkflowIntegrationSettings, VersioningStrategy,
    WorkflowManager, WorkflowType, BranchConventions,
    ContextAwareWorkflowSettings, ContextAwareFeatureBranching,
    ContextAwareReleaseManagement, ContextAwareHotfixManagement,
    ContextAwareMergeStrategies, ContextMergeStrategy,
    ContextMergeStrategyType, ContextConflictResolution,
    ConflictResolutionType
};
use rhema_git::git::history::Signature;
use git2::{Repository, BranchType};
use std::path::Path;
use std::collections::HashMap;
use tempfile::TempDir;
use std::path::PathBuf;
use rhema_core::RhemaResult;

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
    {
        let tree = repo.find_tree(tree_id)?;
        repo.commit(
            Some("refs/heads/main"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[]
        )?;
    }
    
    // Create develop branch
    {
        let main_ref = repo.find_branch("main", BranchType::Local)?;
        let main_commit = main_ref.get().peel_to_commit()?;
        repo.branch("develop", &main_commit, false)?;
    }
    
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
        context_rules: ContextRules {
            require_feature_validation: true,
            require_release_validation: true,
            require_hotfix_validation: true,
            merge_strategies: HashMap::new(),
            isolation_rules: IsolationRules {
                isolate_feature: true,
                isolate_release: true,
                isolate_hotfix: true,
                shared_files: vec![],
            },
        },
        release_management: ReleaseManagement {
            versioning: VersioningStrategy::Semantic,
            branch_preparation: BranchPreparation {
                prepare_context: true,
                update_version: true,
                generate_notes: true,
                validate_readiness: true,
            },
            validation: ReleaseValidation {
                validate_context: true,
                validate_dependencies: true,
                validate_breaking_changes: true,
                run_tests: true,
            },
            automation: ReleaseAutomation {
                auto_create_branch: true,
                auto_version_bump: true,
                auto_release_notes: true,
                auto_deploy: false,
            },
        },
        pull_request_settings: PullRequestSettings {
            require_context_analysis: true,
            require_impact_analysis: true,
            require_dependency_review: true,
            require_health_checks: true,
            automated_checks: vec![],
        },
        automation: AutomationSettings {
            auto_context_updates: true,
            auto_synchronization: true,
            auto_notifications: true,
            auto_backups: true,
        },
        advanced_features: AdvancedWorkflowFeatures {
            context_aware_branching: true,
            auto_context_sync: true,
            context_conflict_resolution: true,
            context_validation_workflows: true,
            context_evolution_tracking: true,
            context_analytics: true,
            context_optimization: true,
            context_backup_workflows: true,
        },
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
            context_aware_pr_analysis: ContextAwarePrAnalysis {
                auto_analyze_context_changes: true,
                auto_detect_conflicts: true,
                auto_generate_impact_report: true,
                auto_suggest_improvements: true,
                analysis_rules: vec![],
                validation_rules: vec![],
                automation_rules: vec![],
            },
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
        integrations: WorkflowIntegrationSettings {
            ci_cd: None,
            issue_tracker: None,
            chat: None,
            monitoring: None,
        },
    }
}

async fn test_feature_workflow(workflow_manager: &WorkflowManager) -> RhemaResult<()> {
    println!("Testing feature workflow...");
    
    let feature_branch = "feature/test-feature";
    
    // Test setup feature context
    workflow_manager.setup_feature_context(feature_branch)?;
    println!("✓ Feature context setup completed");
    
    // Test validate feature branch
    workflow_manager.validate_feature_branch(feature_branch)?;
    println!("✓ Feature branch validation completed");
    
    // Test merge feature branch
    workflow_manager.merge_feature_branch(feature_branch)?;
    println!("✓ Feature branch merge completed");
    
    // Test cleanup feature branch
    workflow_manager.cleanup_feature_branch(feature_branch)?;
    println!("✓ Feature branch cleanup completed");
    
    Ok(())
}

async fn test_release_workflow(workflow_manager: &WorkflowManager) -> RhemaResult<()> {
    println!("Testing release workflow...");
    
    let version = "1.0.0";
    
    // Test prepare release context
    workflow_manager.prepare_release_context(version)?;
    println!("✓ Release context preparation completed");
    
    // Test validate release
    workflow_manager.validate_release(version)?;
    println!("✓ Release validation completed");
    
    // Test merge to main
    workflow_manager.merge_to_main(version)?;
    println!("✓ Release merge to main completed");
    
    // Test merge to develop
    workflow_manager.merge_to_develop(version)?;
    println!("✓ Release merge to develop completed");
    
    // Test cleanup release branch
    workflow_manager.cleanup_release_branch(version)?;
    println!("✓ Release branch cleanup completed");
    
    Ok(())
}

async fn test_hotfix_workflow(workflow_manager: &WorkflowManager) -> RhemaResult<()> {
    println!("Testing hotfix workflow...");
    
    let version = "1.0.1";
    
    // Test setup hotfix context
    workflow_manager.setup_hotfix_context(version)?;
    println!("✓ Hotfix context setup completed");
    
    // Test validate hotfix
    workflow_manager.validate_hotfix(version)?;
    println!("✓ Hotfix validation completed");
    
    // Test cleanup hotfix branch
    workflow_manager.cleanup_hotfix_branch(version)?;
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
    {
        let tree = repo.find_tree(tree_id)?;
        repo.commit(
            Some("refs/heads/main"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[]
        )?;
    }
    
    // Create workflow configuration
    let config = create_test_workflow_config();
    
    // Create workflow manager
    let workflow_manager = WorkflowManager::new(repo, config);
    
        // Test get workflow status
    let status = workflow_manager.get_workflow_status()?;
    println!("Current workflow status: {:?}", status);

    // Test get current branch workflow
    let branch_workflow = workflow_manager.get_current_branch_workflow()?;
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
    {
        let tree = repo.find_tree(tree_id)?;
        repo.commit(
            Some("refs/heads/main"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[]
        )?;
    }
    
    // Create workflow configuration with context-aware features enabled
    let mut config = create_test_workflow_config();
    config.context_aware.context_aware_feature_branching.auto_isolate_context = true;
    config.context_aware.context_aware_feature_branching.auto_sync_parent = true;
    config.context_aware.context_aware_feature_branching.auto_validate_before_merge = true;
    
    // Create workflow manager
    let workflow_manager = WorkflowManager::new(repo, config);
    
    let feature_branch = "feature/context-aware-test";
    
        // Test context-aware feature workflow
    workflow_manager.setup_feature_context(feature_branch)?;
    println!("✓ Context-aware feature setup completed");

    workflow_manager.validate_feature_branch(feature_branch)?;
    println!("✓ Context-aware feature validation completed");

    workflow_manager.merge_feature_branch(feature_branch)?;
    println!("✓ Context-aware feature merge completed");
    
    Ok(())
} 