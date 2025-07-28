use gacp_cli::{Gacp, GacpResult};
use std::fs;

use tempfile::TempDir;

#[test]
fn test_gacp_initialization() -> GacpResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a GACP instance for the temp directory
    let gacp = Gacp::new_from_path(temp_path.to_path_buf())?;
    
    // Verify we're in the right directory
    // Use canonical paths to handle symlinks properly
    let temp_canonical = temp_path.canonicalize()?;
    let repo_canonical = gacp.repo_root().canonicalize()?;
    assert_eq!(repo_canonical, temp_canonical);
    
    // Discover scopes (should be empty initially)
    let scopes = gacp.discover_scopes()?;
    assert_eq!(scopes.len(), 0);
    
    Ok(())
}

#[test]
fn test_scope_creation() -> GacpResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a .gacp directory manually
    let gacp_dir = temp_path.join(".gacp");
    fs::create_dir(&gacp_dir)?;
    
    // Create a basic gacp.yaml
    let gacp_yaml = r#"
name: test-scope
scope_type: service
description: Test scope for unit testing
version: "1.0.0"
dependencies: null
"#;
    fs::write(gacp_dir.join("gacp.yaml"), gacp_yaml)?;
    
    // Create a todos.yaml with some data
    let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Test todo"
    status: pending
    priority: medium
    created_at: "2024-01-15T10:00:00Z"
"#;
    fs::write(gacp_dir.join("todos.yaml"), todos_yaml)?;
    
    // Create a GACP instance for the temp directory
    let gacp = Gacp::new_from_path(temp_path.to_path_buf())?;
    
    // Discover scopes
    let scopes = gacp.discover_scopes()?;
    assert_eq!(scopes.len(), 1);
    
    let scope = &scopes[0];
    assert_eq!(scope.definition.name, "test-scope");
    assert_eq!(scope.definition.scope_type, "service");
    
    Ok(())
}

#[test]
fn test_query_execution() -> GacpResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a .gacp directory manually
    let gacp_dir = temp_path.join(".gacp");
    fs::create_dir(&gacp_dir)?;
    
    // Create a basic gacp.yaml
    let gacp_yaml = r#"
name: test-scope
scope_type: service
description: Test scope for unit testing
version: "1.0.0"
dependencies: null
"#;
    fs::write(gacp_dir.join("gacp.yaml"), gacp_yaml)?;
    
    // Create a todos.yaml with some data
    let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Test todo"
    status: pending
    priority: medium
    created_at: "2024-01-15T10:00:00Z"
"#;
    fs::write(gacp_dir.join("todos.yaml"), todos_yaml)?;
    
    // Create a GACP instance for the temp directory
    let gacp = Gacp::new_from_path(temp_path.to_path_buf())?;
    
    // Execute a query
    let result = gacp.query("todos")?;
    
    // Verify the result contains the expected data
    let result_str = serde_yaml::to_string(&result)?;
    assert!(result_str.contains("todo-001"));
    assert!(result_str.contains("Test todo"));
    
    Ok(())
} 