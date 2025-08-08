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

use rhema_action_cargo::CargoTool;
use rhema_action_tool::{
    ActionIntent, ActionType, SafetyLevel, TransformationTool, ValidationTool,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cargo_tool = CargoTool;

    // Example 1: Workspace root only validation
    println!("=== Example 1: Workspace Root Only ===");
    let root_only_intent = ActionIntent::new(
        "workspace-root-only",
        ActionType::Test,
        "Validate workspace root only",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Low,
    );

    match cargo_tool.validate(&root_only_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: All workspace members validation
    println!("\n=== Example 2: All Workspace Members ===");
    let mut all_members_intent = ActionIntent::new(
        "workspace-all-members",
        ActionType::Test,
        "Validate all workspace members",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Medium,
    );
    all_members_intent.metadata = json!({
        "commands": ["check", "clippy"],
        "workspace_mode": "all_members",
        "json_output": true,
        "verbose": false
    });

    match cargo_tool.validate(&all_members_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 3: Root and members validation
    println!("\n=== Example 3: Root and Members ===");
    let mut root_and_members_intent = ActionIntent::new(
        "workspace-root-and-members",
        ActionType::Test,
        "Validate workspace root and all members",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Medium,
    );
    root_and_members_intent.metadata = json!({
        "commands": ["check", "test"],
        "workspace_mode": "root_and_members",
        "json_output": true,
        "verbose": true
    });

    match cargo_tool.validate(&root_and_members_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 4: Selected members validation
    println!("\n=== Example 4: Selected Members ===");
    let mut selected_members_intent = ActionIntent::new(
        "workspace-selected-members",
        ActionType::Test,
        "Validate selected workspace members",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Low,
    );
    selected_members_intent.metadata = json!({
        "commands": ["check", "audit"],
        "workspace_mode": "selected_members",
        "member_filter": ["core", "api"],
        "exclude_members": ["tests"],
        "json_output": true,
        "verbose": false
    });

    match cargo_tool.validate(&selected_members_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 5: Workspace transformation
    println!("\n=== Example 5: Workspace Transformation ===");
    let mut workspace_transform_intent = ActionIntent::new(
        "workspace-transformation",
        ActionType::Refactor,
        "Transform workspace code",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Medium,
    );
    workspace_transform_intent.metadata = json!({
        "commands": ["fmt", "clippy"],
        "workspace_mode": "all_members",
        "json_output": true,
        "verbose": false
    });

    match cargo_tool.execute(&workspace_transform_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Changes: {:?}", result.changes);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 6: Workspace dependency analysis
    println!("\n=== Example 6: Workspace Dependency Analysis ===");
    let mut dependency_analysis_intent = ActionIntent::new(
        "workspace-dependency-analysis",
        ActionType::Dependency,
        "Analyze workspace dependencies",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Low,
    );
    dependency_analysis_intent.metadata = json!({
        "commands": ["outdated", "audit"],
        "workspace_mode": "root_and_members",
        "json_output": true,
        "verbose": false
    });

    match cargo_tool.validate(&dependency_analysis_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
