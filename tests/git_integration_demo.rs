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

use rhema::git;
use rhema::RhemaResult;
use std::path::PathBuf;

/// Demonstration of advanced Git integration features
pub fn demonstrate_git_integration() -> RhemaResult<()> {
    println!("=== Advanced Git Integration Demo ===\n");
    
    // Create a temporary repository path for demonstration
    let repo_path = PathBuf::from(".");
    
    // 1. Initialize advanced Git integration
    println!("1. Initializing Advanced Git Integration...");
    let mut git_integration = git::create_advanced_git_integration(&repo_path)?;
    git_integration.initialize()?;
    println!("   ✓ Advanced Git integration initialized\n");
    
    // 2. Demonstrate hook management
    println!("2. Git Hook Management...");
    let hook_result = git_integration.execute_hook(git::HookType::PreCommit)?;
    if hook_result.success {
        println!("   ✓ Pre-commit hook executed successfully");
        println!("   Messages: {:?}", hook_result.messages);
    } else {
        println!("   ⚠ Pre-commit hook executed with warnings");
        println!("   Warnings: {:?}", hook_result.warnings);
    }
    println!();
    
    // 3. Demonstrate workflow management
    println!("3. Git Workflow Management...");
    let workflow_status = git_integration.get_workflow_status()?;
    println!("   Current branch: {}", workflow_status.current_branch);
    println!("   Branch type: {:?}", workflow_status.branch_type);
    println!("   Workflow type: {:?}", workflow_status.workflow_type);
    println!("   Status: {}", workflow_status.status);
    println!();
    
    // 4. Demonstrate context history tracking
    println!("4. Context History Tracking...");
    let evolution = git_integration.track_context_evolution(".", Some(5))?;
    println!("   Context evolution entries: {}", evolution.len());
    for (i, entry) in evolution.iter().take(3).enumerate() {
        println!("   {}. {} - {}: {}", 
            i + 1, 
            entry.timestamp.format("%Y-%m-%d %H:%M"),
            entry.author.name,
            entry.commit_message
        );
    }
    println!();
    
    // 5. Demonstrate automation management
    println!("5. Automation Management...");
    let automation_status = git_integration.get_automation_status()?;
    println!("   Automation running: {}", automation_status.running);
    println!("   Total tasks: {}", automation_status.total_tasks);
    println!("   Completed tasks: {}", automation_status.completed_tasks);
    println!("   Failed tasks: {}", automation_status.failed_tasks);
    println!("   Pending tasks: {}", automation_status.pending_tasks);
    println!();
    
    // 6. Demonstrate security features
    println!("6. Security Features...");
    let scan_result = git_integration.run_security_scan(&repo_path)?;
    println!("   Security scan completed");
    println!("   Issues found: {}", scan_result.issues.len());
    println!("   Risk level: {}", scan_result.risk_level);
    println!("   Scan duration: {:?}", scan_result.scan_duration);
    println!();
    
    // 7. Demonstrate monitoring features
    println!("7. Monitoring Features...");
    let monitoring_status = git_integration.get_monitoring_status()?;
    println!("   Monitoring active: {}", monitoring_status.is_active);
    println!("   Metrics collected: {}", monitoring_status.metrics_count);
    println!("   Events recorded: {}", monitoring_status.events_count);
    println!("   Last update: {}", monitoring_status.last_update.format("%Y-%m-%d %H:%M:%S"));
    println!();
    
    // 8. Demonstrate branch context management
    println!("8. Branch Context Management...");
    let validation_status = git_integration.validate_branch_context()?;
    match validation_status {
        git::ValidationStatus::Valid => println!("   ✓ Branch context validation passed"),
        git::ValidationStatus::Invalid(errors) => println!("   ✗ Branch context validation failed: {:?}", errors),
        git::ValidationStatus::Pending => println!("   ⏳ Branch context validation pending"),
        git::ValidationStatus::Skipped => println!("   ⏭ Branch context validation skipped"),
    }
    println!();
    
    // 9. Demonstrate integration status
    println!("9. Integration Status...");
    let integration_status = git_integration.get_integration_status()?;
    println!("   Integration enabled: {}", integration_status.enabled);
    println!("   Hooks installed: {}", integration_status.hooks_installed);
    println!("   Hook status: {:?}", integration_status.hook_status);
    println!();
    
    // 10. Demonstrate shutdown
    println!("10. Shutting Down Integration...");
    git_integration.shutdown()?;
    println!("   ✓ Advanced Git integration shut down successfully");
    println!();
    
    println!("=== Demo Completed Successfully ===");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_git_integration_demo() {
        // This test demonstrates the advanced Git integration features
        // It's more of a demonstration than a traditional test
        let result = demonstrate_git_integration();
        assert!(result.is_ok(), "Git integration demo should complete successfully");
    }
} 