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

    // Example 1: Basic validation with cargo check
    println!("=== Example 1: Basic Cargo Check ===");
    let basic_intent = ActionIntent::new(
        "basic-check",
        ActionType::Test,
        "Basic cargo check",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Low,
    );

    match cargo_tool.validate(&basic_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Comprehensive validation with multiple commands
    println!("\n=== Example 2: Comprehensive Validation ===");
    let mut comprehensive_intent = ActionIntent::new(
        "comprehensive-validation",
        ActionType::Test,
        "Comprehensive validation",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Medium,
    );
    comprehensive_intent.metadata = json!({
        "commands": ["check", "clippy", "test", "audit"],
        "json_output": true,
        "verbose": true
    });

    match cargo_tool.validate(&comprehensive_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 3: Code transformation with formatting and auto-fix
    println!("\n=== Example 3: Code Transformation ===");
    let mut transformation_intent = ActionIntent::new(
        "code-transformation",
        ActionType::Refactor,
        "Code transformation",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Medium,
    );
    transformation_intent.metadata = json!({
        "commands": ["fmt", "clippy"],
        "json_output": true,
        "verbose": false
    });

    match cargo_tool.execute(&transformation_intent).await {
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

    // Example 4: Dependency analysis
    println!("\n=== Example 4: Dependency Analysis ===");
    let mut dependency_intent = ActionIntent::new(
        "dependency-analysis",
        ActionType::Dependency,
        "Dependency analysis",
        vec!["Cargo.toml".to_string()],
        SafetyLevel::Low,
    );
    dependency_intent.metadata = json!({
        "commands": ["outdated", "audit"],
        "json_output": true,
        "verbose": false
    });

    match cargo_tool.validate(&dependency_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Errors: {:?}", result.errors);
            println!("Warnings: {:?}", result.warnings);
            println!("Duration: {:?}", result.duration);
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 5: Check tool availability
    println!("\n=== Example 5: Tool Availability ===");
    let is_available = ValidationTool::is_available(&cargo_tool).await;
    println!("Cargo tool available: {}", is_available);

    Ok(())
}
