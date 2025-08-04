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
    let has_where_filter = provenance
        .applied_filters
        .iter()
        .any(|filter| matches!(filter.filter_type, query::FilterType::WhereCondition));
    assert!(has_where_filter);

    Ok(())
}

#[test]
fn test_core_data_loading() -> RhemaResult<()> {
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

    // Create a knowledge.yaml with some data
    let knowledge_yaml = r#"
entries:
  - id: "knowledge-001"
    title: "Test knowledge"
    content: "This is test knowledge content"
    confidence: 8
    created_at: "2024-01-15T10:00:00Z"
categories:
  test: "Test Category"
"#;
    fs::write(rhema_dir.join("knowledge.yaml"), knowledge_yaml)?;

    // Create a decisions.yaml with some data
    let decisions_yaml = r#"
decisions:
  - id: "decision-001"
    title: "Test decision"
    description: "This is a test decision"
    status: approved
    decided_at: "2024-01-15T10:00:00Z"
"#;
    fs::write(rhema_dir.join("decisions.yaml"), decisions_yaml)?;

    // Create a patterns.yaml with some data
    let patterns_yaml = r#"
patterns:
  - id: "pattern-001"
    name: "Test Pattern"
    description: "This is a test pattern"
    pattern_type: "design"
    usage: required
    created_at: "2024-01-15T10:00:00Z"
"#;
    fs::write(rhema_dir.join("patterns.yaml"), patterns_yaml)?;

    // Create a conventions.yaml with some data
    let conventions_yaml = r#"
conventions:
  - id: "convention-001"
    name: "Test Convention"
    description: "This is a test convention"
    convention_type: "naming"
    enforcement: required
    created_at: "2024-01-15T10:00:00Z"
"#;
    fs::write(rhema_dir.join("conventions.yaml"), conventions_yaml)?;

    // Create a Rhema instance for the temp directory
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Test loading todos
    let todos = rhema.load_todos("test-scope")?;
    assert_eq!(todos.todos.len(), 1);
    assert_eq!(todos.todos[0].id, "todo-001");
    assert_eq!(todos.todos[0].title, "Test todo");

    // Test loading knowledge
    let knowledge = rhema.load_knowledge("test-scope")?;
    assert_eq!(knowledge.entries.len(), 1);
    assert_eq!(knowledge.entries[0].id, "knowledge-001");
    assert_eq!(knowledge.entries[0].title, "Test knowledge");
    assert_eq!(knowledge.categories.as_ref().unwrap().get("test"), Some(&"Test Category".to_string()));

    // Test loading decisions
    let decisions = rhema.load_decisions("test-scope")?;
    assert_eq!(decisions.decisions.len(), 1);
    assert_eq!(decisions.decisions[0].id, "decision-001");
    assert_eq!(decisions.decisions[0].title, "Test decision");

    // Test loading patterns
    let patterns = rhema.load_patterns("test-scope")?;
    assert_eq!(patterns.patterns.len(), 1);
    assert_eq!(patterns.patterns[0].id, "pattern-001");
    assert_eq!(patterns.patterns[0].name, "Test Pattern");

    // Test loading conventions
    let conventions = rhema.load_conventions("test-scope")?;
    assert_eq!(conventions.conventions.len(), 1);
    assert_eq!(conventions.conventions[0].id, "convention-001");
    assert_eq!(conventions.conventions[0].name, "Test Convention");

    Ok(())
}

#[test]
fn test_search_functionality() -> RhemaResult<()> {
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

    // Create a todos.yaml with searchable content
    let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Important todo"
    status: pending
    priority: high
    created_at: "2024-01-15T10:00:00Z"
  - id: "todo-002"
    title: "Another todo"
    status: completed
    priority: medium
    created_at: "2024-01-15T11:00:00Z"
"#;
    fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;

    // Create a Rhema instance for the temp directory
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Test regex search
    let search_results = rhema.search_regex("Important", None)?;
    assert!(!search_results.is_empty(), "Search should find 'Important' in todos");

    // Test search with file filter
    let filtered_results = rhema.search_regex("todo", Some("todos.yaml"))?;
    assert!(!filtered_results.is_empty(), "Filtered search should find 'todo' in todos.yaml");

    Ok(())
}

#[test]
fn test_query_with_stats() -> RhemaResult<()> {
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

    // Test query with stats
    let (result, stats) = rhema.query_with_stats("SELECT * FROM todos")?;
    
    // Verify result is not empty
    assert!(result.as_mapping().is_some() || result.as_sequence().is_some(), "Query should return data");
    
    // Verify stats contain expected information
    assert!(!stats.is_empty(), "Stats should not be empty");
    
    // Check for common stats fields
    let has_execution_time = stats.contains_key("execution_time_ms") || 
                           stats.contains_key("total_time_ms") ||
                           stats.contains_key("duration_ms");
    assert!(has_execution_time, "Stats should contain execution time information");

    Ok(())
}

#[test]
fn test_current_scope_detection() -> RhemaResult<()> {
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

    // Create a Rhema instance for the temp directory
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Test current scope detection
    let current_scope_path = rhema.get_current_scope_path()?;
    
    // The current scope should be the .rhema directory since we're in the repo root
    let expected_path = temp_path.join(".rhema");
    assert_eq!(current_scope_path, expected_path, "Current scope should be the .rhema directory");

    Ok(())
}
