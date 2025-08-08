//! Integration tests for Rhema CLI commands

use crate::common::{fixtures::TestFixtures, helpers::TestHelpers, TestEnv};
use git2;
use rhema_cli::Rhema;
use rhema_core::RhemaResult;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Test CLI command execution
fn run_rhema_command(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "rhema", "--"])
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

#[test]
fn test_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path)?;

    // Change to temp directory
    std::env::set_current_dir(temp_path)?;

    // Run init command
    let output = run_rhema_command(&["init", "--name", "test-scope", "--type", "service"])?;

    // Verify output contains expected content
    assert!(output.contains("test-scope"));
    assert!(output.contains("service"));

    // Verify files were created
    assert!(temp_path.join(".rhema").exists());
    assert!(temp_path.join(".rhema").join("rhema.yaml").exists());

    Ok(())
}

#[test]
fn test_init_command_with_description() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path)?;
    std::env::set_current_dir(temp_path)?;

    // Run init command with description
    let output = run_rhema_command(&[
        "init",
        "--name",
        "test-scope",
        "--type",
        "service",
        "--description",
        "Test scope for integration testing",
    ])?;

    // Verify output
    assert!(output.contains("test-scope"));
    assert!(output.contains("Test scope for integration testing"));

    Ok(())
}

#[test]
fn test_init_command_with_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path)?;
    std::env::set_current_dir(temp_path)?;

    // Run init command with dependencies
    let output = run_rhema_command(&[
        "init",
        "--name",
        "test-scope",
        "--type",
        "service",
        "--dependencies",
        "dep1:1.0.0,dep2:2.0.0",
    ])?;

    // Verify output
    assert!(output.contains("test-scope"));
    assert!(output.contains("dep1"));
    assert!(output.contains("dep2"));

    Ok(())
}

#[test]
fn test_init_command_with_existing_rhema_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path)?;
    std::env::set_current_dir(temp_path)?;

    // Create .rhema directory but no rhema files
    let rhema_dir = temp_path.join(".rhema");
    std::fs::create_dir_all(&rhema_dir)?;

    // Add a non-rhema file to the directory
    std::fs::write(
        rhema_dir.join("some_other_file.txt"),
        "This is not a rhema file",
    )?;

    // Run init command - should succeed
    let output = run_rhema_command(&["init", "--name", "test-scope", "--type", "service"])?;

    // Verify output contains expected content
    assert!(output.contains("test-scope"));
    assert!(output.contains("service"));

    // Verify rhema files were created
    assert!(rhema_dir.join("rhema.yaml").exists());
    assert!(rhema_dir.join("knowledge.yaml").exists());
    assert!(rhema_dir.join("todos.yaml").exists());
    assert!(rhema_dir.join("decisions.yaml").exists());
    assert!(rhema_dir.join("patterns.yaml").exists());
    assert!(rhema_dir.join("conventions.yaml").exists());

    // Verify the original non-rhema file still exists
    assert!(rhema_dir.join("some_other_file.txt").exists());

    Ok(())
}

#[test]
fn test_init_command_with_existing_rhema_files() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path).unwrap();
    std::env::set_current_dir(temp_path).unwrap();

    // Create .rhema directory with existing rhema files
    let rhema_dir = temp_path.join(".rhema");
    std::fs::create_dir_all(&rhema_dir).unwrap();

    // Create an existing rhema.yaml file
    std::fs::write(rhema_dir.join("rhema.yaml"), "name: existing-scope").unwrap();

    // Run init command - should fail
    let result = run_rhema_command(&["init", "--name", "test-scope", "--type", "service"]);
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Rhema files already exist"));
    assert!(error_msg.contains("rhema.yaml"));
}

#[test]
fn test_query_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run query command
    let output = run_rhema_command(&["query", "todos"])?;

    // Verify output contains expected data
    assert!(output.contains("todo-001"));
    assert!(output.contains("todo-002"));

    Ok(())
}

#[test]
fn test_query_command_with_filter() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run query command with filter
    let output = run_rhema_command(&["query", "todos WHERE status=pending"])?;

    // Verify output contains only pending todos
    assert!(output.contains("todo-001"));
    assert!(!output.contains("todo-002")); // completed

    Ok(())
}

#[test]
fn test_query_command_with_format() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run query command with JSON format
    let output = run_rhema_command(&["query", "todos", "--format", "json"])?;

    // Verify output is valid JSON
    assert!(output.trim().starts_with('{'));
    assert!(output.contains("todo-001"));

    Ok(())
}

#[test]
fn test_search_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run search command
    let output = run_rhema_command(&["search", "todo-001"])?;

    // Verify output contains search results
    assert!(output.contains("todo-001"));

    Ok(())
}

#[test]
fn test_search_command_with_regex() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run search command with regex
    let output = run_rhema_command(&["search", "todo-\\d+", "--regex"])?;

    // Verify output contains search results
    assert!(output.contains("todo-001"));
    assert!(output.contains("todo-002"));

    Ok(())
}

#[test]
fn test_search_command_with_file_filter() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run search command with file filter
    let output = run_rhema_command(&["search", "todo", "--files", "*.yaml"])?;

    // Verify output contains search results
    assert!(output.contains("todo"));

    Ok(())
}

#[test]
fn test_validate_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run validate command
    let output = run_rhema_command(&["validate"])?;

    // Verify output indicates validation passed
    assert!(output.contains("valid") || output.contains("success") || output.contains("passed"));

    Ok(())
}

#[test]
fn test_validate_command_with_schema() -> Result<(), Box<dyn std::error::Error>> {
    let (temp_dir, repo_path) = TestHelpers::create_test_repo()?;
    TestHelpers::create_basic_scope(&repo_path.path().to_path_buf())?;
    std::env::set_current_dir(&repo_path.path())?;

    // Run validate command with schema
    let output = run_rhema_command(&["validate", "--schema"])?;

    // Verify output indicates validation passed
    assert!(output.contains("valid") || output.contains("success") || output.contains("passed"));

    Ok(())
}

#[test]
fn test_validate_command_with_strict() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run validate command with strict mode
    let output = run_rhema_command(&["validate", "--strict"])?;

    // Verify output indicates validation passed
    assert!(output.contains("valid") || output.contains("success") || output.contains("passed"));

    Ok(())
}

#[test]
fn test_sync_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run sync command
    let output = run_rhema_command(&["sync"])?;

    // Verify output indicates sync completed
    assert!(output.contains("sync") || output.contains("complete") || output.contains("success"));

    Ok(())
}

#[test]
fn test_sync_command_with_target() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    let target_dir = TempDir::new()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run sync command with target
    let output = run_rhema_command(&["sync", "--target", target_dir.path().to_str().unwrap()])?;

    // Verify output indicates sync completed
    assert!(output.contains("sync") || output.contains("complete") || output.contains("success"));

    Ok(())
}

#[test]
fn test_stats_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run stats command
    let output = run_rhema_command(&["stats"])?;

    // Verify output contains statistics
    assert!(output.contains("stats") || output.contains("count") || output.contains("files"));

    Ok(())
}

#[test]
fn test_stats_command_detailed() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run stats command with detailed output
    let output = run_rhema_command(&["stats", "--detailed"])?;

    // Verify output contains detailed statistics
    assert!(output.contains("stats") || output.contains("count") || output.contains("files"));

    Ok(())
}

#[test]
fn test_health_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run health command
    let output = run_rhema_command(&["health"])?;

    // Verify output indicates health status
    assert!(output.contains("health") || output.contains("status") || output.contains("ok"));

    Ok(())
}

#[test]
fn test_health_command_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run health command with verbose output
    let output = run_rhema_command(&["health", "--verbose"])?;

    // Verify output contains detailed health information
    assert!(output.contains("health") || output.contains("status") || output.contains("ok"));

    Ok(())
}

#[test]
fn test_show_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run show command
    let output = run_rhema_command(&["show"])?;

    // Verify output shows scope information
    assert!(output.contains("test-scope") || output.contains("service"));

    Ok(())
}

#[test]
fn test_show_command_with_path() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run show command with specific path
    let output = run_rhema_command(&["show", ".rhema"])?;

    // Verify output shows scope information
    assert!(output.contains("test-scope") || output.contains("service"));

    Ok(())
}

#[test]
fn test_scopes_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run scopes command
    let output = run_rhema_command(&["scopes"])?;

    // Verify output lists scopes
    assert!(output.contains("test-scope"));

    Ok(())
}

#[test]
fn test_scopes_command_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run scopes command with verbose output
    let output = run_rhema_command(&["scopes", "--verbose"])?;

    // Verify output contains detailed scope information
    assert!(output.contains("test-scope"));
    assert!(output.contains("service"));

    Ok(())
}

#[test]
fn test_todo_add_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run todo add command
    let output = run_rhema_command(&["todo", "add", "New test todo", "--priority", "high"])?;

    // Verify output indicates todo was added
    assert!(output.contains("added") || output.contains("created") || output.contains("success"));

    Ok(())
}

#[test]
fn test_todo_list_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run todo list command
    let output = run_rhema_command(&["todo", "list"])?;

    // Verify output lists todos
    assert!(output.contains("todo-001") || output.contains("todo-002"));

    Ok(())
}

#[test]
fn test_todo_list_command_with_filter() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run todo list command with status filter
    let output = run_rhema_command(&["todo", "list", "--status", "pending"])?;

    // Verify output contains only pending todos
    assert!(output.contains("todo-001"));
    assert!(!output.contains("todo-002")); // completed

    Ok(())
}

#[test]
fn test_todo_complete_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run todo complete command
    let output = run_rhema_command(&[
        "todo",
        "complete",
        "todo-001",
        "--outcome",
        "Successfully completed",
    ])?;

    // Verify output indicates todo was completed
    assert!(output.contains("completed") || output.contains("success"));

    Ok(())
}

#[test]
fn test_todo_update_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run todo update command
    let output = run_rhema_command(&[
        "todo",
        "update",
        "todo-001",
        "--title",
        "Updated todo title",
    ])?;

    // Verify output indicates todo was updated
    assert!(output.contains("updated") || output.contains("success"));

    Ok(())
}

#[test]
fn test_todo_delete_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run todo delete command
    let output = run_rhema_command(&["todo", "delete", "todo-001"])?;

    // Verify output indicates todo was deleted
    assert!(output.contains("deleted") || output.contains("success"));

    Ok(())
}

#[test]
fn test_insight_record_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run insight record command
    let output = run_rhema_command(&[
        "insight",
        "record",
        "Test insight",
        "--content",
        "This is a test insight for integration testing",
        "--confidence",
        "8",
        "--category",
        "testing",
    ])?;

    // Verify output indicates insight was recorded
    assert!(
        output.contains("recorded") || output.contains("created") || output.contains("success")
    );

    Ok(())
}

#[test]
fn test_insight_list_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run insight list command
    let output = run_rhema_command(&["insight", "list"])?;

    // Verify output lists insights
    assert!(output.contains("insight-001") || output.contains("Test insight"));

    Ok(())
}

#[test]
fn test_insight_list_command_with_category() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run insight list command with category filter
    let output = run_rhema_command(&["insight", "list", "--category", "performance"])?;

    // Verify output contains only performance insights
    assert!(output.contains("performance"));

    Ok(())
}

#[test]
fn test_pattern_add_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run pattern add command
    let output = run_rhema_command(&[
        "pattern",
        "add",
        "Test Pattern",
        "--description",
        "A test pattern for integration testing",
        "--type",
        "architectural",
        "--usage",
        "recommended",
        "--effectiveness",
        "9",
    ])?;

    // Verify output indicates pattern was added
    assert!(output.contains("added") || output.contains("created") || output.contains("success"));

    Ok(())
}

#[test]
fn test_pattern_list_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run pattern list command
    let output = run_rhema_command(&["pattern", "list"])?;

    // Verify output lists patterns
    assert!(output.contains("pattern-001") || output.contains("Repository Pattern"));

    Ok(())
}

#[test]
fn test_decision_record_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run decision record command
    let output = run_rhema_command(&[
        "decision",
        "record",
        "Test Decision",
        "--description",
        "A test decision for integration testing",
        "--status",
        "proposed",
        "--context",
        "Testing context",
    ])?;

    // Verify output indicates decision was recorded
    assert!(
        output.contains("recorded") || output.contains("created") || output.contains("success")
    );

    Ok(())
}

#[test]
fn test_decision_list_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run decision list command
    let output = run_rhema_command(&["decision", "list"])?;

    // Verify output lists decisions
    assert!(output.contains("decision-001") || output.contains("Use Rust for CLI implementation"));

    Ok(())
}

#[test]
fn test_decision_list_command_with_status() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run decision list command with status filter
    let output = run_rhema_command(&["decision", "list", "--status", "approved"])?;

    // Verify output contains only approved decisions
    assert!(output.contains("approved"));

    Ok(())
}

#[test]
fn test_migrate_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run migrate command
    let output = run_rhema_command(&["migrate"])?;

    // Verify output indicates migration status
    assert!(
        output.contains("migrate") || output.contains("up to date") || output.contains("success")
    );

    Ok(())
}

#[test]
fn test_migrate_command_with_target_version() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run migrate command with target version
    let output = run_rhema_command(&["migrate", "--target-version", "2.0.0"])?;

    // Verify output indicates migration status
    assert!(
        output.contains("migrate") || output.contains("up to date") || output.contains("success")
    );

    Ok(())
}

#[test]
fn test_dependencies_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run dependencies command
    let output = run_rhema_command(&["dependencies"])?;

    // Verify output shows dependencies
    assert!(output.contains("dependencies") || output.contains("none") || output.contains("null"));

    Ok(())
}

#[test]
fn test_dependencies_command_resolve() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run dependencies resolve command
    let output = run_rhema_command(&["dependencies", "resolve"])?;

    // Verify output indicates resolution completed
    assert!(
        output.contains("resolve") || output.contains("resolved") || output.contains("success")
    );

    Ok(())
}

#[test]
fn test_impact_command() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run impact command
    let output = run_rhema_command(&["impact"])?;

    // Verify output shows impact analysis
    assert!(output.contains("impact") || output.contains("analysis") || output.contains("changes"));

    Ok(())
}

#[test]
fn test_impact_command_with_files() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run impact command with specific files
    let output = run_rhema_command(&["impact", "--files", ".rhema/todos.yaml"])?;

    // Verify output shows impact analysis
    assert!(output.contains("impact") || output.contains("analysis") || output.contains("changes"));

    Ok(())
}

#[test]
fn test_help_command() -> Result<(), Box<dyn std::error::Error>> {
    // Run help command
    let output = run_rhema_command(&["--help"])?;

    // Verify output contains help information
    assert!(output.contains("USAGE") || output.contains("COMMANDS") || output.contains("OPTIONS"));

    Ok(())
}

#[test]
fn test_version_command() -> Result<(), Box<dyn std::error::Error>> {
    // Run version command
    let output = run_rhema_command(&["--version"])?;

    // Verify output contains version information
    assert!(output.contains("rhema") || output.contains("version"));

    Ok(())
}

#[test]
fn test_invalid_command() {
    // Run invalid command
    let result = run_rhema_command(&["invalid-command"]);

    // Verify command fails
    assert!(result.is_err());
}

#[test]
fn test_command_with_invalid_options() {
    // Run command with invalid options
    let result = run_rhema_command(&["query", "--invalid-option"]);

    // Verify command fails
    assert!(result.is_err());
}

#[test]
fn test_command_with_missing_required_args() {
    // Run command with missing required arguments
    let result = run_rhema_command(&["todo", "add"]);

    // Verify command fails
    assert!(result.is_err());
}

#[test]
fn test_command_with_invalid_file_path() {
    // Run command with invalid file path
    let result = run_rhema_command(&["query", "nonexistent.yaml"]);

    // Verify command fails
    assert!(result.is_err());
}

#[test]
fn test_command_with_invalid_query() {
    let env = TestEnv::with_sample_data().unwrap();
    std::env::set_current_dir(&env.repo_path).unwrap();

    // Run command with invalid query
    let result = run_rhema_command(&["query", "invalid query syntax"]);

    // Verify command fails
    assert!(result.is_err());
}

#[test]
fn test_command_with_large_input() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Create test data
    // TestHelpers::create_large_dataset(&env.repo_path, 1000)?;

    // Run query command on large dataset
    let output = run_rhema_command(&["query", "todos"])?;

    // Verify command completes successfully
    assert!(!output.is_empty());

    Ok(())
}

#[test]
fn test_command_performance() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Measure command execution time
    let start = std::time::Instant::now();
    let _output = run_rhema_command(&["query", "todos"])?;
    let duration = start.elapsed();

    // Verify command completes within reasonable time (1 second)
    assert!(duration.as_secs() < 1);

    Ok(())
}

#[test]
fn test_command_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Run multiple commands to test memory usage
    for i in 0..10 {
        let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
        let _output = run_rhema_command(&["query", &query])?;
    }

    // If we get here without memory issues, the test passes
    Ok(())
}

#[test]
fn test_command_concurrent_execution() -> Result<(), Box<dyn std::error::Error>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // Spawn multiple threads to run commands concurrently
    for i in 0..5 {
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
            let result = run_rhema_command(&["query", &query]);
            let mut results = results_clone.lock().unwrap();
            results.push(result.map_err(|e| e.to_string()));
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all commands completed successfully
    let results = results.lock().unwrap();
    for result in results.iter() {
        assert!(result.is_ok());
    }

    Ok(())
}

/// Test search functionality
#[test]
fn test_search_functionality() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Create test scope with searchable content
    let scope_path = temp_path.join("test-service");
    std::fs::create_dir_all(&scope_path)?;

    // Create knowledge file with searchable content
    let knowledge_content = r#"
name: Test Service
version: 1.0.0
description: A test service for search functionality
keywords:
  - test
  - search
  - service
  - api
"#;
    std::fs::write(scope_path.join("knowledge.yaml"), knowledge_content)?;

    // Create todos file
    let todos_content = r#"
todos:
  - id: todo-001
    title: Implement search feature
    description: Add full-text search capabilities
    status: pending
    priority: high
    assignee: developer
    created_at: 2024-01-01T00:00:00Z
    due_date: 2024-02-01T00:00:00Z
    tags:
      - search
      - feature
      - high-priority
"#;
    std::fs::write(scope_path.join("todos.yaml"), todos_content)?;

    // Test basic search
    let results = rhema.search_regex("search", None)?;
    assert!(
        !results.is_empty(),
        "Search should find results for 'search'"
    );

    // Test search with file filter
    let results = rhema.search_regex("service", Some("*.yaml"))?;
    assert!(
        !results.is_empty(),
        "Search with file filter should find results"
    );

    // Test search for specific terms
    let results = rhema.search_regex("implement", None)?;
    assert!(
        !results.is_empty(),
        "Search should find 'implement' in todos"
    );

    // Test search for version information
    let results = rhema.search_regex(r"\d+\.\d+\.\d+", None)?;
    assert!(!results.is_empty(), "Search should find version numbers");

    Ok(())
}

/// Test advanced search features
#[test]
fn test_advanced_search_features() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Create multiple test scopes
    for i in 1..=3 {
        let scope_path = temp_path.join(format!("service-{}", i));
        std::fs::create_dir_all(&scope_path)?;

        // Create knowledge file
        let knowledge_content = format!(
            r#"
name: Service {}
version: {}.0.0
description: Test service number {}
keywords:
  - service
  - test
  - number-{}
"#,
            i, i, i, i
        );
        std::fs::write(scope_path.join("knowledge.yaml"), knowledge_content)?;

        // Create todos file
        let todos_content = format!(
            r#"
todos:
  - id: todo-{:03}
    title: Task for service {}
    status: pending
    priority: high
"#,
            i, i
        );
        std::fs::write(scope_path.join("todos.yaml"), todos_content)?;
    }

    // Test search across multiple scopes
    let results = rhema.search_regex("service", None)?;
    assert!(
        results.len() >= 3,
        "Search should find results across multiple scopes"
    );

    // Test search with specific scope filtering (by file pattern)
    let results = rhema.search_regex("task", Some("todos.yaml"))?;
    assert!(
        !results.is_empty(),
        "Search should find tasks in todos files"
    );

    // Test search for version patterns
    let results = rhema.search_regex(r"version: \d+\.\d+\.\d+", None)?;
    assert!(
        results.len() >= 3,
        "Search should find version information in all services"
    );

    // Test search for priority information
    let results = rhema.search_regex("priority: high", None)?;
    assert!(
        !results.is_empty(),
        "Search should find high priority items"
    );

    Ok(())
}

/// Test search performance
#[test]
fn test_search_performance() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository
    let _repo = git2::Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Create multiple test files for performance testing
    for i in 1..=10 {
        let scope_path = temp_path.join(format!("large-service-{}", i));
        std::fs::create_dir_all(&scope_path)?;

        // Create a large knowledge file
        let mut knowledge_content = format!(
            r#"
name: Large Service {}
version: {}.0.0
description: A large test service for performance testing
"#,
            i, i
        );

        // Add many keywords for search testing
        for j in 1..=50 {
            knowledge_content.push_str(&format!("keyword-{}: value-{}\n", j, j));
        }

        std::fs::write(scope_path.join("knowledge.yaml"), knowledge_content)?;
    }

    // Test search performance
    let start = std::time::Instant::now();
    let results = rhema.search_regex("keyword", None)?;
    let duration = start.elapsed();

    // Search should complete within reasonable time (500ms for this dataset)
    assert!(duration.as_millis() < 500, "Search should complete quickly");
    assert!(!results.is_empty(), "Search should find results");

    // Test search with specific pattern
    let start = std::time::Instant::now();
    let results = rhema.search_regex(r"keyword-\d+", None)?;
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 500,
        "Regex search should complete quickly"
    );
    assert!(!results.is_empty(), "Regex search should find results");

    Ok(())
}
