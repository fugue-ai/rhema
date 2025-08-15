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

use rhema_action_type_checking::TypeCheckingTool;
use rhema_action_tool::{ActionIntent, ActionType, SafetyLevel, SafetyTool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the type checking tool
    let tool = TypeCheckingTool;
    
    println!("Type Checking Tool Example");
    println!("==========================");
    
    // Check if the tool is available
    let is_available = tool.is_available().await;
    println!("Tool available: {}", is_available);
    
    if !is_available {
        println!("Warning: No type checking tools are available on this system");
        println!("Install the required tools for the languages you want to check:");
        println!("  - TypeScript: npm install -g typescript");
        println!("  - Python: pip install mypy");
        println!("  - Rust: rustup install stable");
        println!("  - Go: Download from https://golang.org/dl/");
        println!("  - Java: Install OpenJDK or Oracle JDK");
        println!("  - Kotlin: Download from https://kotlinlang.org/");
        println!("  - Swift: Install Xcode Command Line Tools (macOS)");
        return Ok(());
    }
    
    // Example 1: TypeScript files
    println!("\nExample 1: TypeScript files");
    let ts_intent = ActionIntent::new(
        "ts-example-001",
        ActionType::Refactor,
        "TypeScript refactoring example",
        vec![
            "examples/test.ts".to_string(),
            "examples/test.tsx".to_string(),
        ],
        SafetyLevel::Medium,
    );
    
    match tool.check(&ts_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Duration: {:?}", result.duration);
            if !result.errors.is_empty() {
                println!("Errors:");
                for error in &result.errors {
                    println!("  - {}", error);
                }
            }
            if !result.warnings.is_empty() {
                println!("Warnings:");
                for warning in &result.warnings {
                    println!("  - {}", warning);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Example 2: Mixed language files
    println!("\nExample 2: Mixed language files");
    let mixed_intent = ActionIntent::new(
        "mixed-example-002",
        ActionType::Feature,
        "Mixed language feature example",
        vec![
            "examples/test.ts".to_string(),
            "examples/test.py".to_string(),
            "examples/test.rs".to_string(),
            "examples/test.go".to_string(),
            "examples/test.java".to_string(),
            "examples/test.kt".to_string(),
            "examples/test.swift".to_string(),
        ],
        SafetyLevel::High,
    );
    
    match tool.check(&mixed_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            println!("Duration: {:?}", result.duration);
            println!("Changes:");
            for change in &result.changes {
                println!("  - {}", change);
            }
            if !result.errors.is_empty() {
                println!("Errors:");
                for error in &result.errors {
                    println!("  - {}", error);
                }
            }
            if !result.warnings.is_empty() {
                println!("Warnings:");
                for warning in &result.warnings {
                    println!("  - {}", warning);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Example 3: Empty scope
    println!("\nExample 3: Empty scope");
    let empty_intent = ActionIntent::new(
        "empty-example-003",
        ActionType::Test,
        "Empty scope example",
        vec![],
        SafetyLevel::Low,
    );
    
    match tool.check(&empty_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
        }
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Example 4: Unknown file types
    println!("\nExample 4: Unknown file types");
    let unknown_intent = ActionIntent::new(
        "unknown-example-004",
        ActionType::Documentation,
        "Unknown file types example",
        vec![
            "examples/test.txt".to_string(),
            "examples/test.md".to_string(),
            "examples/test.json".to_string(),
        ],
        SafetyLevel::Low,
    );
    
    match tool.check(&unknown_intent).await {
        Ok(result) => {
            println!("Success: {}", result.success);
            println!("Output: {}", result.output);
            if !result.warnings.is_empty() {
                println!("Warnings:");
                for warning in &result.warnings {
                    println!("  - {}", warning);
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
    
    println!("\nExample completed successfully!");
    Ok(())
}
