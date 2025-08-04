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

use rhema_git::automation::{GitAutomationManager, default_automation_config};
use git2::Repository;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Automation Integration Example ===\n");

    // Initialize repository (assuming we're in a git repository)
    let repo = Repository::open(".")?;
    let mut config = default_automation_config();
    
    // Enable workflow automation
    config.git_workflow_integration.workflow_automation = true;
    
    // Enable specific automation features
    config.git_workflow_integration.rules.feature_rules.auto_setup_context = true;
    config.git_workflow_integration.rules.feature_rules.auto_validate = true;
    config.git_workflow_integration.rules.feature_rules.auto_merge = true;
    config.git_workflow_integration.rules.feature_rules.auto_cleanup = true;
    
    config.git_workflow_integration.rules.release_rules.auto_prepare_context = true;
    config.git_workflow_integration.rules.release_rules.auto_validate = true;
    config.git_workflow_integration.rules.release_rules.auto_merge_to_main = true;
    config.git_workflow_integration.rules.release_rules.auto_merge_to_develop = true;
    config.git_workflow_integration.rules.release_rules.auto_cleanup = true;
    
    config.git_workflow_integration.rules.hotfix_rules.auto_setup_context = true;
    config.git_workflow_integration.rules.hotfix_rules.auto_validate = true;
    config.git_workflow_integration.rules.hotfix_rules.auto_merge_to_main = true;
    config.git_workflow_integration.rules.hotfix_rules.auto_merge_to_develop = true;
    config.git_workflow_integration.rules.hotfix_rules.auto_cleanup = true;

    let automation_manager = GitAutomationManager::new(repo, config);

    // 1. Start automation
    println!("1. Starting automation...");
    automation_manager.start_automation()?;
    println!("✓ Automation started");

    // 2. Trigger workflow automation based on events
    println!("\n2. Triggering workflow automation events...");

    // Simulate branch creation event
    let mut branch_data = HashMap::new();
    branch_data.insert("branch_name".to_string(), "feature/new-feature".to_string());
    automation_manager.trigger_workflow_automation("branch_creation", Some(branch_data))?;
    println!("✓ Triggered branch creation automation");

    // Simulate commit push event
    let mut commit_data = HashMap::new();
    commit_data.insert("branch_name".to_string(), "feature/new-feature".to_string());
    automation_manager.trigger_workflow_automation("commit_push", Some(commit_data))?;
    println!("✓ Triggered commit push automation");

    // Simulate pull request event
    let mut pr_data = HashMap::new();
    pr_data.insert("action".to_string(), "opened".to_string());
    pr_data.insert("branch_name".to_string(), "feature/new-feature".to_string());
    automation_manager.trigger_workflow_automation("pull_request", Some(pr_data))?;
    println!("✓ Triggered pull request automation");

    // 3. Trigger specific feature automation
    println!("\n3. Triggering specific feature automation...");
    automation_manager.trigger_feature_automation("new-feature", "setup_context")?;
    println!("✓ Triggered feature context setup");
    
    automation_manager.trigger_feature_automation("new-feature", "validate")?;
    println!("✓ Triggered feature validation");

    // 4. Trigger release automation
    println!("\n4. Triggering release automation...");
    automation_manager.trigger_release_automation("1.2.0", "prepare_context")?;
    println!("✓ Triggered release context preparation");
    
    automation_manager.trigger_release_automation("1.2.0", "validate")?;
    println!("✓ Triggered release validation");

    // 5. Trigger hotfix automation
    println!("\n5. Triggering hotfix automation...");
    automation_manager.trigger_hotfix_automation("1.2.1", "setup_context")?;
    println!("✓ Triggered hotfix context setup");
    
    automation_manager.trigger_hotfix_automation("1.2.1", "validate")?;
    println!("✓ Triggered hotfix validation");

    // 6. Schedule workflow automation
    println!("\n6. Scheduling workflow automation...");
    automation_manager.schedule_workflow_automation()?;
    println!("✓ Scheduled workflow automation");

    // 7. Manual workflow trigger
    println!("\n7. Manual workflow trigger...");
    let mut manual_data = HashMap::new();
    manual_data.insert("workflow_type".to_string(), "feature".to_string());
    manual_data.insert("feature_name".to_string(), "manual-feature".to_string());
    manual_data.insert("action".to_string(), "setup_context".to_string());
    automation_manager.trigger_workflow_automation("manual", Some(manual_data))?;
    println!("✓ Triggered manual workflow");

    // 8. Check automation status
    println!("\n8. Checking automation status...");
    let status = automation_manager.get_status()?;
    println!("✓ Automation running: {}", status.running);
    println!("✓ Total tasks: {}", status.total_tasks);
    println!("✓ Completed tasks: {}", status.completed_tasks);
    println!("✓ Failed tasks: {}", status.failed_tasks);
    println!("✓ Running tasks: {}", status.running_tasks);
    println!("✓ Pending tasks: {}", status.pending_tasks);

    // 9. Get task history
    println!("\n9. Getting task history...");
    let task_history = automation_manager.get_task_history(Some(10));
    println!("✓ Retrieved {} tasks from history", task_history.len());
    
    for task in task_history.iter().take(5) {
        println!("  - Task {}: {:?} ({:?})", task.id, task.task_type, task.status);
    }

    // 10. Stop automation
    println!("\n10. Stopping automation...");
    automation_manager.stop_automation()?;
    println!("✓ Automation stopped");

    println!("\n=== Automation Integration Example Complete ===");
    Ok(())
} 