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

use rhema_git::workflow::{WorkflowManager, default_git_flow_config};
use git2::Repository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hotfix Branch Automation Example ===\n");

    // Initialize repository (assuming we're in a git repository)
    let repo = Repository::open(".")?;
    let workflow_config = default_git_flow_config();
    let workflow_manager = WorkflowManager::new(repo, workflow_config);

    let version = "1.2.1";
    let hotfix_branch = format!("hotfix/{}", version);

    // 1. Set up hotfix context
    println!("1. Setting up hotfix context...");
    match workflow_manager.setup_hotfix_context(&hotfix_branch) {
        Ok(_) => println!("✓ Hotfix context set up: {}", hotfix_branch),
        Err(e) => {
            println!("✗ Failed to set up hotfix context: {}", e);
            return Err(e.into());
        }
    }

    // 2. Validate hotfix branch
    println!("\n2. Validating hotfix branch...");
    match workflow_manager.validate_hotfix(&hotfix_branch) {
        Ok(_) => println!("✓ Hotfix branch validated: {}", hotfix_branch),
        Err(e) => {
            println!("✗ Hotfix branch validation failed: {}", e);
            return Err(e.into());
        }
    }

    // 3. Merge to main (using existing release automation)
    println!("\n3. Merging hotfix branch to main...");
    match workflow_manager.merge_to_main(&hotfix_branch) {
        Ok(true) => println!("✓ Hotfix branch merged to main"),
        Ok(false) => println!("✗ Merge to main failed (see logs)"),
        Err(e) => {
            println!("✗ Merge to main error: {}", e);
            return Err(e.into());
        }
    }

    // 4. Merge to develop (using existing release automation)
    println!("\n4. Merging hotfix branch to develop...");
    match workflow_manager.merge_to_develop(&hotfix_branch) {
        Ok(true) => println!("✓ Hotfix branch merged to develop"),
        Ok(false) => println!("✗ Merge to develop failed (see logs)"),
        Err(e) => {
            println!("✗ Merge to develop error: {}", e);
            return Err(e.into());
        }
    }

    // 5. Clean up hotfix branch
    println!("\n5. Cleaning up hotfix branch...");
    match workflow_manager.cleanup_hotfix_branch(&hotfix_branch) {
        Ok(_) => println!("✓ Hotfix branch cleaned up: {}", hotfix_branch),
        Err(e) => {
            println!("✗ Cleanup failed: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Hotfix Branch Automation Example Complete ===");
    Ok(())
} 