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
    println!("=== Release Branch Automation Example ===\n");

    // Initialize repository (assuming we're in a git repository)
    let repo = Repository::open(".")?;
    let workflow_config = default_git_flow_config();
    let workflow_manager = WorkflowManager::new(repo, workflow_config);

    let version = "1.2.3";
    let release_branch = format!("release/{}", version);

    // 1. Prepare release context
    println!("1. Preparing release context...");
    match workflow_manager.prepare_release_context(&release_branch, version) {
        Ok(_) => println!("✓ Release context prepared: {}", release_branch),
        Err(e) => {
            println!("✗ Failed to prepare release context: {}", e);
            return Err(e.into());
        }
    }

    // 2. Validate release branch
    println!("\n2. Validating release branch...");
    match workflow_manager.validate_release(&release_branch) {
        Ok(_) => println!("✓ Release branch validated: {}", release_branch),
        Err(e) => {
            println!("✗ Release branch validation failed: {}", e);
            return Err(e.into());
        }
    }

    // 3. Merge to main
    println!("\n3. Merging release branch to main...");
    match workflow_manager.merge_to_main(&release_branch) {
        Ok(true) => println!("✓ Release branch merged to main"),
        Ok(false) => println!("✗ Merge to main failed (see logs)"),
        Err(e) => {
            println!("✗ Merge to main error: {}", e);
            return Err(e.into());
        }
    }

    // 4. Merge to develop
    println!("\n4. Merging release branch to develop...");
    match workflow_manager.merge_to_develop(&release_branch) {
        Ok(true) => println!("✓ Release branch merged to develop"),
        Ok(false) => println!("✗ Merge to develop failed (see logs)"),
        Err(e) => {
            println!("✗ Merge to develop error: {}", e);
            return Err(e.into());
        }
    }

    // 5. Cleanup release branch
    println!("\n5. Cleaning up release branch...");
    match workflow_manager.cleanup_release_branch(&release_branch) {
        Ok(_) => println!("✓ Release branch cleaned up: {}", release_branch),
        Err(e) => {
            println!("✗ Cleanup failed: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Release Branch Automation Example Complete ===");
    Ok(())
} 