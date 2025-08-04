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
use rhema_git::git::automation::{
    AutomationConfig, GitAutomationManager, AIServiceConfig, SmartAutomationRules,
    BranchNamingPatterns, CommitMessagePatterns, WorkflowOptimizationRules, ContextInjectionRules,
};
use std::collections::HashMap;
use std::path::Path;

/// Example demonstrating Context-Aware Automation features
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Context-Aware Automation Example");
    println!("====================================\n");

    // Initialize repository (replace with your actual repository path)
    let repo_path = std::env::current_dir()?;
    let repo = Repository::open(&repo_path)?;

    // Create enhanced automation configuration
    let config = create_enhanced_automation_config();
    
    // Create automation manager
    let automation_manager = GitAutomationManager::new(repo, config);

    // Example 1: AI-Driven Workflow Suggestions
    println!("📋 Example 1: AI-Driven Workflow Suggestions");
    println!("--------------------------------------------");
    let suggestions = automation_manager.get_workflow_suggestions("Current workflow analysis").await?;
    for suggestion in suggestions {
        println!("💡 Suggestion: {} (confidence: {:.2})", suggestion.title, suggestion.confidence);
        println!("   Description: {}", suggestion.description);
        println!("   Implementation steps:");
        for step in &suggestion.implementation_steps {
            println!("     - {}", step);
        }
        println!();
    }

    // Example 2: Smart Branch Naming
    println!("🌿 Example 2: Smart Branch Naming");
    println!("--------------------------------");
    let branch_name = automation_manager.generate_branch_name("feature", "user authentication system").await?;
    println!("Generated branch name: {}", branch_name);
    
    let bugfix_branch = automation_manager.generate_branch_name("bugfix", "login validation error").await?;
    println!("Generated bugfix branch name: {}", bugfix_branch);
    println!();

    // Example 3: Automated Commit Messages
    println!("💬 Example 3: Automated Commit Messages");
    println!("--------------------------------------");
    let changes = vec![
        "Add user authentication module".to_string(),
        "Implement JWT token validation".to_string(),
        "Add password hashing utilities".to_string(),
    ];
    let commit_message = automation_manager.generate_commit_message(&changes, "feature").await?;
    println!("Generated commit message: {}", commit_message);
    
    let bugfix_changes = vec![
        "Fix login validation logic".to_string(),
        "Update error handling".to_string(),
    ];
    let bugfix_commit = automation_manager.generate_commit_message(&bugfix_changes, "bugfix").await?;
    println!("Generated bugfix commit message: {}", bugfix_commit);
    println!();

    // Example 4: Context Injection
    println!("🔗 Example 4: Context Injection");
    println!("-------------------------------");
    let context = automation_manager.inject_workflow_context("feature_development").await?;
    println!("Injected context for feature development:");
    println!("{}", context);
    println!();

    // Example 5: AI Workflow Automation Triggers
    println!("🤖 Example 5: AI Workflow Automation Triggers");
    println!("--------------------------------------------");
    
    // Smart branch creation trigger
    let mut branch_data = HashMap::new();
    branch_data.insert("task_type".to_string(), "feature".to_string());
    branch_data.insert("description".to_string(), "payment processing integration".to_string());
    automation_manager.trigger_ai_workflow("smart_branch_creation", Some(branch_data)).await?;
    
    // Smart commit message trigger
    let mut commit_data = HashMap::new();
    commit_data.insert("changes".to_string(), "Add payment gateway, Update billing logic, Fix currency conversion".to_string());
    commit_data.insert("task_type".to_string(), "feature".to_string());
    automation_manager.trigger_ai_workflow("smart_commit_message", Some(commit_data)).await?;
    
    // Workflow optimization trigger
    automation_manager.trigger_ai_workflow("workflow_optimization", None).await?;
    
    // Context injection trigger
    let mut context_data = HashMap::new();
    context_data.insert("operation".to_string(), "code_review".to_string());
    automation_manager.trigger_ai_workflow("context_injection", Some(context_data)).await?;

    println!("✅ All Context-Aware Automation examples completed successfully!");
    Ok(())
}

/// Create an enhanced automation configuration with AI features enabled
fn create_enhanced_automation_config() -> AutomationConfig {
    // AI service configuration
    let ai_config = AIServiceConfig {
        endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: 1000,
        enable_caching: true,
        cache_ttl: 3600,
    };

    // Enhanced branch naming patterns
    let branch_patterns = BranchNamingPatterns {
        feature_pattern: "feature/{description}-{timestamp}".to_string(),
        release_pattern: "release/v{version}-{date}".to_string(),
        hotfix_pattern: "hotfix/v{version}-{issue}".to_string(),
        bugfix_pattern: "bugfix/{description}-{priority}".to_string(),
        documentation_pattern: "docs/{description}-{type}".to_string(),
        refactor_pattern: "refactor/{description}-{scope}".to_string(),
        test_pattern: "test/{description}-{framework}".to_string(),
    };

    // Enhanced commit message patterns
    let commit_patterns = CommitMessagePatterns {
        feature_pattern: "feat: {changes}\n\n- Implements new functionality\n- Includes tests\n- Updates documentation".to_string(),
        bugfix_pattern: "fix: {changes}\n\n- Resolves reported issue\n- Includes regression tests\n- Updates affected documentation".to_string(),
        documentation_pattern: "docs: {changes}\n\n- Updates documentation\n- Improves clarity\n- Adds examples".to_string(),
        refactor_pattern: "refactor: {changes}\n\n- Improves code structure\n- Maintains functionality\n- Updates tests".to_string(),
        test_pattern: "test: {changes}\n\n- Adds comprehensive tests\n- Improves test coverage\n- Validates functionality".to_string(),
        release_pattern: "release: {changes}\n\n- Prepares for release\n- Updates version\n- Includes changelog".to_string(),
        hotfix_pattern: "hotfix: {changes}\n\n- Critical security fix\n- Emergency deployment\n- Minimal changes".to_string(),
    };

    // Workflow optimization rules
    let workflow_optimization = WorkflowOptimizationRules {
        enable_suggestions: true,
        enable_performance_optimization: true,
        enable_conflict_prediction: true,
        enable_dependency_analysis: true,
        enable_security_scanning: true,
    };

    // Context injection rules
    let context_injection = ContextInjectionRules {
        include_git_history: true,
        include_file_changes: true,
        include_dependency_context: true,
        include_project_structure: true,
        include_team_context: true,
        include_documentation_context: true,
    };

    // Smart automation rules
    let smart_rules = SmartAutomationRules {
        branch_naming_patterns: branch_patterns,
        commit_message_patterns: commit_patterns,
        workflow_optimization,
        context_injection_rules: context_injection,
    };

    // Create the enhanced configuration
    let mut config = AutomationConfig {
        auto_context_updates: true,
        auto_synchronization: true,
        auto_notifications: false,
        auto_backups: true,
        intervals: rhema_git::git::automation::AutomationIntervals {
            context_update_interval: 300,
            sync_interval: 1800,
            backup_interval: 86400,
            health_check_interval: 3600,
        },
        notifications: rhema_git::git::automation::NotificationSettings {
            email: None,
            slack: None,
            webhook: None,
            events: rhema_git::git::automation::NotificationEvents {
                context_updated: true,
                sync_completed: true,
                backup_created: true,
                health_check_failed: true,
                conflict_detected: true,
                validation_failed: true,
            },
        },
        backup_settings: rhema_git::git::automation::BackupSettings {
            backup_directory: std::path::PathBuf::from(".rhema/backups"),
            max_backups: 100,
            compression: true,
            encryption: false,
            retention_policy: rhema_git::git::automation::RetentionPolicy {
                daily_retention_days: 7,
                weekly_retention_weeks: 4,
                monthly_retention_months: 12,
            },
        },
        sync_settings: rhema_git::git::automation::SyncSettings {
            strategy: rhema_git::git::automation::SyncStrategy::Incremental,
            conflict_resolution: rhema_git::git::automation::ConflictResolution::Auto,
            filters: rhema_git::git::automation::SyncFilters {
                include_patterns: vec!["*.yaml".to_string()],
                exclude_patterns: vec!["*.tmp".to_string(), "*.bak".to_string()],
                max_file_size: Some(1024 * 1024),
                allowed_extensions: vec!["yaml".to_string(), "yml".to_string()],
            },
            validation: rhema_git::git::automation::SyncValidation {
                validate_before: true,
                validate_after: true,
                health_checks: true,
                check_dependencies: true,
            },
        },
        advanced_features: rhema_git::git::automation::AdvancedAutomationFeatures {
            ml_automation: true,
            predictive_automation: true,
            adaptive_automation: true,
            intelligent_scheduling: true,
        },
        git_workflow_integration: rhema_git::git::automation::GitWorkflowIntegration {
            workflow_automation: true,
            auto_create_feature_branches: true,
            auto_merge_feature_branches: false,
            auto_create_release_branches: true,
            auto_merge_release_branches: false,
            auto_create_hotfix_branches: true,
            auto_merge_hotfix_branches: false,
            intervals: rhema_git::git::automation::WorkflowAutomationIntervals {
                feature_automation_interval: 300,
                release_automation_interval: 3600,
                hotfix_automation_interval: 600,
                workflow_validation_interval: 300,
            },
            triggers: rhema_git::git::automation::WorkflowAutomationTriggers {
                on_branch_creation: true,
                on_branch_merge: true,
                on_commit_push: true,
                on_pull_request: true,
                on_schedule: true,
                on_manual_request: true,
            },
            rules: rhema_git::git::automation::WorkflowAutomationRules {
                feature_rules: rhema_git::git::automation::FeatureAutomationRules {
                    auto_setup_context: true,
                    auto_validate: true,
                    auto_merge: false,
                    auto_cleanup: true,
                    required_checks: vec!["tests".to_string(), "linting".to_string()],
                    merge_strategy: "squash".to_string(),
                },
                release_rules: rhema_git::git::automation::ReleaseAutomationRules {
                    auto_prepare_context: true,
                    auto_validate: true,
                    auto_merge_to_main: false,
                    auto_merge_to_develop: false,
                    auto_cleanup: true,
                    required_checks: vec!["integration_tests".to_string(), "security_scan".to_string()],
                    merge_strategy: "merge".to_string(),
                },
                hotfix_rules: rhema_git::git::automation::HotfixAutomationRules {
                    auto_setup_context: true,
                    auto_validate: true,
                    auto_merge_to_main: false,
                    auto_merge_to_develop: false,
                    auto_cleanup: true,
                    required_checks: vec!["critical_tests".to_string(), "security_scan".to_string()],
                    merge_strategy: "merge".to_string(),
                },
            },
        },
        context_aware_automation: rhema_git::git::automation::ContextAwareAutomation {
            context_aware_updates: true,
            context_aware_sync: true,
            context_aware_backups: true,
            ai_driven_workflows: true,
            smart_branch_naming: true,
            automated_commit_messages: true,
            context_injection: true,
            ai_service_config: Some(ai_config),
            smart_automation_rules: smart_rules,
        },
        security_automation: rhema_git::git::automation::SecurityAutomation {
            security_scanning: true,
            vulnerability_checks: true,
            access_control_automation: true,
        },
    };

    config
}

/// Example of using Context-Aware Automation in a real workflow
pub async fn demonstrate_real_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Real Workflow Example: Feature Development");
    println!("============================================\n");

    // Initialize automation manager
    let repo_path = std::env::current_dir()?;
    let repo = Repository::open(&repo_path)?;
    let config = create_enhanced_automation_config();
    let automation_manager = GitAutomationManager::new(repo, config);

    // Step 1: Generate smart branch name for new feature
    println!("Step 1: Creating feature branch");
    let branch_name = automation_manager.generate_branch_name("feature", "real-time notifications").await?;
    println!("   Generated branch: {}", branch_name);
    println!("   (In real implementation, this would create the branch)\n");

    // Step 2: Get workflow suggestions
    println!("Step 2: Getting workflow suggestions");
    let suggestions = automation_manager.get_workflow_suggestions("Starting feature development").await?;
    println!("   Found {} suggestions:", suggestions.len());
    for suggestion in suggestions {
        println!("   - {} (confidence: {:.2})", suggestion.title, suggestion.confidence);
    }
    println!();

    // Step 3: Inject context for development
    println!("Step 3: Injecting development context");
    let context = automation_manager.inject_workflow_context("feature_development").await?;
    println!("   Context injected for development workflow\n");

    // Step 4: Generate commit message for changes
    println!("Step 4: Committing changes");
    let changes = vec![
        "Add WebSocket connection manager".to_string(),
        "Implement notification service".to_string(),
        "Add real-time event handling".to_string(),
    ];
    let commit_message = automation_manager.generate_commit_message(&changes, "feature").await?;
    println!("   Generated commit message: {}", commit_message);
    println!("   (In real implementation, this would commit the changes)\n");

    // Step 5: Trigger AI workflow optimization
    println!("Step 5: AI workflow optimization");
    automation_manager.trigger_ai_workflow("workflow_optimization", None).await?;
    println!("   AI workflow optimization triggered\n");

    println!("✅ Real workflow demonstration completed!");
    Ok(())
} 