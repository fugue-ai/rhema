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
    feature_automation::{FeatureAutomationManager, default_feature_automation_config},
    workflow::{WorkflowManager, default_git_flow_config},
};
use git2::Repository;
use std::path::Path;

/// Example demonstrating feature branch automation
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Feature Branch Automation Example ===\n");

    // Initialize repository (assuming we're in a git repository)
    let repo = Repository::open(".")?;
    
    // Create workflow manager
    let workflow_config = default_git_flow_config();
    let workflow_manager = WorkflowManager::new(repo.clone(), workflow_config);

    // Create feature automation manager
    let automation_config = default_feature_automation_config();
    let automation_manager = FeatureAutomationManager::new(repo, automation_config);

    // Example 1: Set up feature context
    println!("1. Setting up feature context...");
    let feature_name = "user-authentication";
    let base_branch = "develop";
    
    match automation_manager.setup_feature_context(feature_name, base_branch) {
        Ok(context) => {
            println!("✓ Feature context set up successfully");
            println!("  Branch: {}", context.branch_name);
            println!("  Base branch: {}", context.base_branch);
            println!("  Context directory: {:?}", context.context_directory);
            println!("  Context files: {}", context.context_files.len());
        }
        Err(e) => {
            println!("✗ Failed to set up feature context: {}", e);
            return Err(e.into());
        }
    }

    // Example 2: Validate feature branch
    println!("\n2. Validating feature branch...");
    let branch_name = format!("feature/{}", feature_name);
    
    match automation_manager.validate_feature_branch(&branch_name) {
        Ok(validation_result) => {
            if validation_result.success {
                println!("✓ Feature branch validation passed");
                if !validation_result.warnings.is_empty() {
                    println!("  Warnings:");
                    for warning in &validation_result.warnings {
                        println!("    - {}", warning);
                    }
                }
            } else {
                println!("✗ Feature branch validation failed");
                println!("  Errors:");
                for error in &validation_result.errors {
                    println!("    - {}", error);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to validate feature branch: {}", e);
            return Err(e.into());
        }
    }

    // Example 3: Merge feature branch
    println!("\n3. Merging feature branch...");
    let target_branch = "develop";
    
    match automation_manager.merge_feature_branch(&branch_name, target_branch) {
        Ok(merge_result) => {
            if merge_result.success {
                println!("✓ Feature branch merged successfully");
                println!("  Source branch: {}", merge_result.source_branch);
                println!("  Target branch: {}", merge_result.target_branch);
                if !merge_result.messages.is_empty() {
                    println!("  Messages:");
                    for message in &merge_result.messages {
                        println!("    - {}", message);
                    }
                }
            } else {
                println!("✗ Feature branch merge failed");
                if !merge_result.conflicts.is_empty() {
                    println!("  Conflicts:");
                    for conflict in &merge_result.conflicts {
                        println!("    - {}", conflict);
                    }
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to merge feature branch: {}", e);
            return Err(e.into());
        }
    }

    // Example 4: Clean up feature branch
    println!("\n4. Cleaning up feature branch...");
    
    match automation_manager.cleanup_feature_branch(&branch_name) {
        Ok(cleanup_result) => {
            if cleanup_result.success {
                println!("✓ Feature branch cleanup completed successfully");
                println!("  Branch: {}", cleanup_result.branch_name);
                if !cleanup_result.messages.is_empty() {
                    println!("  Messages:");
                    for message in &cleanup_result.messages {
                        println!("    - {}", message);
                    }
                }
            } else {
                println!("✗ Feature branch cleanup failed");
                println!("  Errors:");
                for error in &cleanup_result.errors {
                    println!("    - {}", error);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to cleanup feature branch: {}", e);
            return Err(e.into());
        }
    }

    // Example 5: Complete workflow using WorkflowManager
    println!("\n5. Complete workflow using WorkflowManager...");
    
    match workflow_manager.start_feature("complete-workflow-example") {
        Ok(feature_branch) => {
            println!("✓ Started feature branch: {}", feature_branch.name);
            println!("  Base branch: {}", feature_branch.base_branch);
            println!("  Created at: {}", feature_branch.created_at);
            
            // Finish the feature branch
            match workflow_manager.finish_feature("complete-workflow-example") {
                Ok(feature_result) => {
                    if feature_result.success {
                        println!("✓ Finished feature branch successfully");
                        println!("  Merged to: {}", feature_result.target_branch);
                        if !feature_result.messages.is_empty() {
                            println!("  Messages:");
                            for message in &feature_result.messages {
                                println!("    - {}", message);
                            }
                        }
                    } else {
                        println!("✗ Failed to finish feature branch");
                        if !feature_result.conflicts.is_empty() {
                            println!("  Conflicts:");
                            for conflict in &feature_result.conflicts {
                                println!("    - {}", conflict);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to finish feature branch: {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to start feature branch: {}", e);
        }
    }

    println!("\n=== Feature Branch Automation Example Complete ===");
    Ok(())
}

/// Helper function to check if we're in a git repository
fn is_git_repository(path: &Path) -> bool {
    path.join(".git").exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use git2::Repository;

    #[test]
    fn test_feature_automation_config() {
        let config = default_feature_automation_config();
        assert!(config.auto_context_setup);
        assert!(config.auto_validation);
        assert!(config.auto_merging);
        assert!(config.auto_cleanup);
    }

    #[test]
    fn test_feature_automation_manager_creation() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        
        let config = default_feature_automation_config();
        let manager = FeatureAutomationManager::new(repo, config);
        
        // The manager should be created successfully
        assert_eq!(manager.config.auto_context_setup, true);
    }
} 