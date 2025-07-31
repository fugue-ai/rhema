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

use crate::{Rhema, RhemaResult, BatchSubcommands};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_batch_validation_operations() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Test batch validation
    let subcommand = BatchSubcommands::Validate {
        validation_type: "validate".to_string(),
        scope_filter: None,
        output_file: None,
        detailed: false,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    Ok(())
}

#[test]
fn test_batch_health_check_operations() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Test batch health check
    let subcommand = BatchSubcommands::Validate {
        validation_type: "health-check".to_string(),
        scope_filter: None,
        output_file: None,
        detailed: true,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    Ok(())
}

#[test]
fn test_batch_schema_check_operations() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Test batch schema check
    let subcommand = BatchSubcommands::Validate {
        validation_type: "schema-check".to_string(),
        scope_filter: None,
        output_file: None,
        detailed: true,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    Ok(())
}

#[test]
fn test_batch_dependency_check_operations() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Test batch dependency check
    let subcommand = BatchSubcommands::Validate {
        validation_type: "dependency-check".to_string(),
        scope_filter: None,
        output_file: None,
        detailed: true,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    Ok(())
}

#[test]
fn test_batch_reporting_operations() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Test batch reporting
    let subcommand = BatchSubcommands::Report {
        report_type: "summary".to_string(),
        scope_filter: None,
        output_file: "test_report.md".to_string(),
        format: "markdown".to_string(),
        include_details: true,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    // Verify report was created
    assert!(Path::new("test_report.md").exists());
    
    // Clean up
    fs::remove_file("test_report.md")?;
    
    Ok(())
}

#[test]
fn test_batch_data_export_operations() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Test batch data export
    let subcommand = BatchSubcommands::Data {
        operation: "export".to_string(),
        input_path: "".to_string(), // Not used for export
        output_path: "test_export.json".to_string(),
        format: "json".to_string(),
        scope_filter: None,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    // Verify export was created
    assert!(Path::new("test_export.json").exists());
    
    // Clean up
    fs::remove_file("test_export.json")?;
    
    Ok(())
}

#[test]
fn test_batch_command_execution() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Create test command file
    let command_file = "test_commands.yaml";
    fs::write(command_file, r#"
- command: "validate"
  description: "Validate all YAML files in each scope"
  args:
    recursive: true
    json_schema: false
    migrate: false

- command: "health"
  description: "Check health status of each scope"
  args: {}
"#)?;
    
    // Test batch command execution
    let subcommand = BatchSubcommands::Commands {
        command_file: command_file.to_string(),
        scope_filter: None,
        parallel: false,
        max_workers: 2,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    // Clean up
    fs::remove_file(command_file)?;
    
    Ok(())
}

#[test]
fn test_batch_context_operations() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = Rhema::new(temp_dir.path())?;
    
    // Create test scopes
    create_test_scopes(&rhema, temp_dir.path())?;
    
    // Create test input file
    let input_file = "test_input.yaml";
    fs::write(input_file, r#"
validation:
  recursive: true
  json_schema: false
  migrate: false
  strict_mode: true
  ignore_warnings: false

health_check:
  check_dependencies: true
  check_file_permissions: true
  check_schema_compliance: true
  check_data_integrity: true
  generate_report: true
  report_format: "json"
"#)?;
    
    // Test batch context operations
    let subcommand = BatchSubcommands::Context {
        operation: "validate".to_string(),
        input_file: input_file.to_string(),
        scope_filter: None,
        dry_run: true,
    };
    
    crate::commands::batch::run(&rhema, &subcommand)?;
    
    // Clean up
    fs::remove_file(input_file)?;
    
    Ok(())
}

fn create_test_scopes(rhema: &Rhema, repo_root: &Path) -> RhemaResult<()> {
    // Create test scope 1
    let scope1_path = repo_root.join("test_scope_1");
    fs::create_dir_all(&scope1_path)?;
    
    // Create rhema.yaml for scope 1
    fs::write(scope1_path.join("rhema.yaml"), r#"
name: "Test Scope 1"
version: "1.0.0"
description: "A test scope for batch operations"
scope_type: "service"
dependencies: []
"#)?;
    
    // Create knowledge.yaml for scope 1
    fs::write(scope1_path.join("knowledge.yaml"), r#"
entries:
  - id: "test-knowledge-1"
    title: "Test Knowledge"
    content: "This is test knowledge content"
    category: "testing"
    tags: ["test", "batch"]
    confidence: 85
    created_at: "2024-01-01T00:00:00Z"
    updated_at: null
    source: null
    custom: {}
categories: {}
custom: {}
"#)?;
    
    // Create todos.yaml for scope 1
    fs::write(scope1_path.join("todos.yaml"), r#"
todos:
  - id: "test-todo-1"
    title: "Test Todo"
    description: "A test todo item"
    status: "pending"
    priority: "medium"
    assigned_to: "test-user"
    due_date: null
    created_at: "2024-01-01T00:00:00Z"
    completed_at: null
    outcome: null
    related_knowledge: null
    custom: {}
custom: {}
"#)?;
    
    // Create decisions.yaml for scope 1
    fs::write(scope1_path.join("decisions.yaml"), r#"
decisions:
  - id: "test-decision-1"
    title: "Test Decision"
    description: "A test decision"
    status: "pending"
    context: "Testing context"
    decision_makers: ["test-user"]
    alternatives: ["option1", "option2"]
    rationale: "Test rationale"
    consequences: ["consequence1"]
    created_at: "2024-01-01T00:00:00Z"
    decided_at: null
    custom: {}
custom: {}
"#)?;
    
    // Create patterns.yaml for scope 1
    fs::write(scope1_path.join("patterns.yaml"), r#"
patterns:
  - id: "test-pattern-1"
    name: "Test Pattern"
    description: "A test pattern"
    pattern_type: "testing"
    usage: "recommended"
    effectiveness: 80
    examples: ["example1"]
    anti_patterns: ["anti-pattern1"]
    created_at: "2024-01-01T00:00:00Z"
    updated_at: null
    custom: {}
custom: {}
"#)?;
    
    // Create test scope 2
    let scope2_path = repo_root.join("test_scope_2");
    fs::create_dir_all(&scope2_path)?;
    
    // Create rhema.yaml for scope 2
    fs::write(scope2_path.join("rhema.yaml"), r#"
name: "Test Scope 2"
version: "1.0.0"
description: "Another test scope for batch operations"
scope_type: "application"
dependencies:
  - path: "test_scope_1"
"#)?;
    
    // Create minimal files for scope 2
    fs::write(scope2_path.join("knowledge.yaml"), r#"
entries: []
categories: {}
custom: {}
"#)?;
    
    fs::write(scope2_path.join("todos.yaml"), r#"
todos: []
custom: {}
"#)?;
    
    fs::write(scope2_path.join("decisions.yaml"), r#"
decisions: []
custom: {}
"#)?;
    
    fs::write(scope2_path.join("patterns.yaml"), r#"
patterns: []
custom: {}
"#)?;
    
    Ok(())
} 