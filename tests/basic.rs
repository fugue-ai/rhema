use rhema::{Rhema, RhemaResult};
use std::fs;

use tempfile::TempDir;

#[test]
fn test_rhema_initialization() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a Rhema instance for the temp directory
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    
    // Verify we're in the right directory
    // Use canonical paths to handle symlinks properly
    let temp_canonical = temp_path.canonicalize()?;
    let repo_canonical = rhema.repo_root().canonicalize()?;
    assert_eq!(repo_canonical, temp_canonical);
    
    // Discover scopes (should be empty initially)
    let scopes = rhema.discover_scopes()?;
    assert_eq!(scopes.len(), 0);
    
    Ok(())
}

#[test]
fn test_scope_creation() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a .rhema directory manually
    let rhema_dir = temp_path.join(".rhema");
    fs::create_dir(&rhema_dir)?;
    
    // Create a basic rhema.yaml
    let rhema_yaml = r#"
name: test-scope
scope_type: service
description: Test scope for unit testing
version: "1.0.0"
dependencies: null
"#;
    fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;
    
    // Create a todos.yaml with some data
    let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Test todo"
    status: pending
    priority: medium
    created_at: "2024-01-15T10:00:00Z"
"#;
    fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;
    
    // Create a Rhema instance for the temp directory
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    
    // Discover scopes
    let scopes = rhema.discover_scopes()?;
    assert_eq!(scopes.len(), 1);
    
    let scope = &scopes[0];
    assert_eq!(scope.definition.name, "test-scope");
    assert_eq!(scope.definition.scope_type, "service");
    
    Ok(())
}

#[test]
fn test_query_execution() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a .rhema directory manually
    let rhema_dir = temp_path.join(".rhema");
    fs::create_dir(&rhema_dir)?;
    
    // Create a basic rhema.yaml
    let rhema_yaml = r#"
name: test-scope
scope_type: service
description: Test scope for unit testing
version: "1.0.0"
dependencies: null
"#;
    fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;
    
    // Create a todos.yaml with some data
    let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Test todo"
    status: pending
    priority: medium
    created_at: "2024-01-15T10:00:00Z"
"#;
    fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;
    
    // Create a Rhema instance for the temp directory
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    
    // Execute a query
    let result = rhema.query("todos")?;
    
    // Verify the result contains the expected data
    let result_str = serde_yaml::to_string(&result)?;
    assert!(result_str.contains("todo-001"));
    assert!(result_str.contains("Test todo"));
    
    Ok(())
} 

#[test]
fn test_query_provenance() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a .rhema directory manually
    let rhema_dir = temp_path.join(".rhema");
    fs::create_dir(&rhema_dir)?;
    
    // Create a basic rhema.yaml
    let rhema_yaml = r#"
name: test-scope
scope_type: service
description: Test scope for provenance testing
version: "1.0.0"
dependencies: null
"#;
    fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;
    
    // Create a todos.yaml with some data
    let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Test todo"
    status: pending
    priority: medium
    created_at: "2024-01-15T10:00:00Z"
  - id: "todo-002"
    title: "Another todo"
    status: completed
    priority: high
    created_at: "2024-01-16T10:00:00Z"
"#;
    fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;
    
    // Create a Rhema instance for the temp directory
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    
    // Execute a query with provenance
    let (result, provenance) = rhema.query_with_provenance("todos WHERE status = 'pending'")?;
    
    // Verify the result contains the expected data
    let result_str = serde_yaml::to_string(&result)?;
    assert!(result_str.contains("todo-001"));
    assert!(result_str.contains("Test todo"));
    assert!(!result_str.contains("todo-002")); // Should be filtered out
    
    // Verify provenance information
    assert_eq!(provenance.original_query, "todos WHERE status = 'pending'");
    assert!(provenance.execution_time_ms > 0);
    assert!(!provenance.scopes_searched.is_empty());
    assert!(!provenance.files_accessed.is_empty());
    assert!(!provenance.applied_filters.is_empty());
    
    // Verify that we have a WHERE condition filter
    let has_where_filter = provenance.applied_filters.iter()
        .any(|filter| matches!(filter.filter_type, query::FilterType::WhereCondition));
    assert!(has_where_filter);
    
    Ok(())
} 