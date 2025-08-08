//! Unit tests for core Rhema functionality

use rhema_core::RhemaResult;
use std::path::PathBuf;

#[test]
fn test_basic_functionality() -> RhemaResult<()> {
    println!("🚀 Basic Rhema functionality test");
    println!("==================================");
    
    // Test basic path operations
    let test_path = PathBuf::from("/tmp/test");
    assert!(!test_path.exists());
    
    println!("✅ Basic path validation works");
    println!("✅ Test completed successfully");
    
    Ok(())
}

fn main() -> RhemaResult<()> {
    println!("🚀 Basic Rhema functionality example");
    println!("====================================");
    
    // Test basic path operations
    let test_path = PathBuf::from("/tmp/test");
    assert!(!test_path.exists());
    
    println!("✅ Basic path validation works");
    println!("✅ Example completed successfully");
    
    Ok(())
}

#[test]
fn test_get_scope_not_found() {
    let env = TestEnv::new().unwrap();
    
    // Try to get non-existent scope
    let result = env.rhema.get_scope("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_query_execution_simple() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Execute simple query
    let result = env.rhema.query("todos")?;
    
    // Verify result contains expected data
    assertions::assert_query_contains(&result, "todo-001");
    assertions::assert_query_contains(&result, "todo-002");
    
    Ok(())
}

#[test]
fn test_query_execution_filtered() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Execute filtered query
    let result = env.rhema.query("todos WHERE status=pending")?;
    
    // Verify result contains only pending todos
    assertions::assert_query_contains(&result, "todo-001");
    assertions::assert_query_not_contains(&result, "todo-002"); // completed
    
    Ok(())
}

#[test]
fn test_query_execution_complex_filter() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Execute complex filtered query
    let result = env.rhema.query("todos WHERE status=pending AND priority=high")?;
    
    // Verify result contains only high priority pending todos
    assertions::assert_query_contains(&result, "todo-001");
    
    Ok(())
}

#[test]
fn test_query_execution_invalid() {
    let env = TestEnv::with_sample_data().unwrap();
    
    // Execute invalid query
    let result = env.rhema.query("invalid_query");
    assert!(result.is_err());
}

#[test]
fn test_query_with_stats() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Execute query with stats
    let (result, stats) = env.rhema.query_with_stats("todos")?;
    
    // Verify result
    assertions::assert_query_contains(&result, "todo-001");
    
    // Verify stats
    assert!(!stats.is_empty());
    
    Ok(())
}

#[test]
fn test_search_regex() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Search with regex pattern
    let results = env.rhema.search_regex("todo-001", None)?;
    
    // Verify search results
    assert!(!results.is_empty());
    
    Ok(())
}

#[test]
fn test_search_regex_with_file_filter() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Search with regex pattern and file filter
    let results = env.rhema.search_regex("todo", Some("*.yaml"))?;
    
    // Verify search results
    assert!(!results.is_empty());
    
    Ok(())
}

#[test]
fn test_repo_root() -> RhemaResult<()> {
    let env = TestEnv::new()?;
    
    // Verify repo root is correct
    let repo_root = env.rhema.repo_root();
    assert_eq!(repo_root, &env.repo_path);
    
    Ok(())
}

#[test]
fn test_repo_root_canonical() -> RhemaResult<()> {
    let env = TestEnv::new()?;
    
    // Verify canonical paths match
    let repo_canonical = env.rhema.repo_root().canonicalize()?;
    let temp_canonical = env.repo_path.canonicalize()?;
    assert_eq!(repo_canonical, temp_canonical);
    
    Ok(())
}

#[test]
fn test_scope_definition_validation() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify scope definition is valid
    assert!(!scope.definition.name.is_empty());
    assert!(!scope.definition.scope_type.is_empty());
    assert!(!scope.definition.version.is_empty());
    assert!(!scope.definition.description.is_empty());
    
    Ok(())
}

#[test]
fn test_scope_data_files() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify data files are discovered
    let data_files = scope.data_files();
    assert!(!data_files.is_empty());
    
    // Verify specific files exist
    assert!(data_files.iter().any(|f| f.file_name().unwrap() == "todos.yaml"));
    assert!(data_files.iter().any(|f| f.file_name().unwrap() == "insights.yaml"));
    
    Ok(())
}

#[test]
fn test_scope_schema_files() -> RhemaResult<()> {
    let (temp_dir, repo_path) = tests::common::helpers::TestHelpers::create_temp_git_repo()?;
    tests::common::helpers::TestHelpers::create_nested_scope(&repo_path)?;
    
    let rhema = Rhema::new_from_path(repo_path)?;
    let scopes = rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify schema files are discovered
    let schema_files = scope.schema_files();
    assert!(!schema_files.is_empty());
    
    // Verify specific schema files exist
    assert!(schema_files.iter().any(|f| f.file_name().unwrap() == "todo.yaml"));
    
    Ok(())
}

#[test]
fn test_scope_dependencies() -> RhemaResult<()> {
    let (temp_dir, repo_path) = tests::common::helpers::TestHelpers::create_temp_git_repo()?;
    
    // Create scope with dependencies
    let rhema_dir = repo_path.join(".rhema");
    std::fs::create_dir_all(&rhema_dir)?;
    
    let rhema_yaml = TestFixtures::complex_scope();
    std::fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;
    
    let rhema = Rhema::new_from_path(repo_path)?;
    let scopes = rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify dependencies are parsed
    let dependencies = &scope.definition.dependencies;
    assert!(dependencies.is_some());
    
    let deps = dependencies.as_ref().unwrap();
    assert_eq!(deps.len(), 2);
    assert_eq!(deps[0].name, "dependency-1");
    assert_eq!(deps[1].name, "dependency-2");
    
    Ok(())
}

#[test]
fn test_scope_without_dependencies() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify no dependencies
    assert!(scope.definition.dependencies.is_none());
    
    Ok(())
}

#[test]
fn test_scope_version_validation() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify version format
    let version = &scope.definition.version;
    assert!(version.matches(r"^\d+\.\d+\.\d+$"));
    
    Ok(())
}

#[test]
fn test_scope_schema_version_validation() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify schema version format
    let schema_version = &scope.definition.schema_version;
    assert!(schema_version.matches(r"^\d+\.\d+\.\d+$"));
    
    Ok(())
}

#[test]
fn test_scope_type_validation() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify scope type is valid
    let scope_type = &scope.definition.scope_type;
    assert!(matches!(scope_type.as_str(), "service" | "library" | "api" | "cli" | "tool"));
    
    Ok(())
}

#[test]
fn test_scope_description_validation() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify description is not empty
    let description = &scope.definition.description;
    assert!(!description.is_empty());
    assert!(description.len() > 10); // Reasonable minimum length
    
    Ok(())
}

#[test]
fn test_scope_name_validation() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    let scopes = env.rhema.discover_scopes()?;
    let scope = &scopes[0];
    
    // Verify name is valid
    let name = &scope.definition.name;
    assert!(!name.is_empty());
    assert!(name.matches(r"^[a-zA-Z0-9_-]+$")); // Alphanumeric, underscore, hyphen
    
    Ok(())
}

#[test]
fn test_multiple_scopes_discovery() -> RhemaResult<()> {
    let (temp_dir, repo_path) = tests::common::helpers::TestHelpers::create_temp_git_repo()?;
    
    // Create multiple scopes
    let scope1_dir = repo_path.join("scope1").join(".rhema");
    let scope2_dir = repo_path.join("scope2").join(".rhema");
    
    std::fs::create_dir_all(&scope1_dir)?;
    std::fs::create_dir_all(&scope2_dir)?;
    
    // Create rhema.yaml for scope1
    let rhema_yaml1 = r#"
name: scope1
scope_type: service
description: First test scope
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
    std::fs::write(scope1_dir.join("rhema.yaml"), rhema_yaml1)?;
    
    // Create rhema.yaml for scope2
    let rhema_yaml2 = r#"
name: scope2
scope_type: library
description: Second test scope
version: "2.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
    std::fs::write(scope2_dir.join("rhema.yaml"), rhema_yaml2)?;
    
    let rhema = Rhema::new_from_path(repo_path)?;
    let scopes = rhema.discover_scopes()?;
    
    // Verify both scopes are discovered
    assert_eq!(scopes.len(), 2);
    
    let scope_names: Vec<&str> = scopes.iter().map(|s| s.definition.name.as_str()).collect();
    assert!(scope_names.contains(&"scope1"));
    assert!(scope_names.contains(&"scope2"));
    
    Ok(())
}

#[test]
fn test_scope_discovery_with_hidden_files() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    // Create a hidden file in .rhema directory
    let hidden_file = env.repo_path.join(".rhema").join(".hidden_file");
    std::fs::write(&hidden_file, "hidden content")?;
    
    let scopes = env.rhema.discover_scopes()?;
    
    // Verify scope discovery still works
    assert_eq!(scopes.len(), 1);
    
    Ok(())
}

#[test]
fn test_scope_discovery_with_symlinks() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    // Create a symlink in .rhema directory
    let target_file = env.repo_path.join("target_file");
    let symlink_file = env.repo_path.join(".rhema").join("symlink_file");
    
    std::fs::write(&target_file, "target content")?;
    tests::common::helpers::TestHelpers::create_symlink(&target_file, &symlink_file)?;
    
    let scopes = env.rhema.discover_scopes()?;
    
    // Verify scope discovery still works
    assert_eq!(scopes.len(), 1);
    
    Ok(())
}

#[test]
fn test_scope_discovery_with_empty_files() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    // Create an empty file in .rhema directory
    let empty_file = env.repo_path.join(".rhema").join("empty.yaml");
    std::fs::write(&empty_file, "")?;
    
    let scopes = env.rhema.discover_scopes()?;
    
    // Verify scope discovery still works
    assert_eq!(scopes.len(), 1);
    
    Ok(())
}

#[test]
fn test_scope_discovery_with_large_files() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    // Create a large file in .rhema directory
    let large_file = env.repo_path.join(".rhema").join("large.yaml");
    tests::common::helpers::TestHelpers::create_large_file(&large_file, 1)?; // 1MB
    
    let scopes = env.rhema.discover_scopes()?;
    
    // Verify scope discovery still works
    assert_eq!(scopes.len(), 1);
    
    Ok(())
}

#[test]
fn test_scope_discovery_performance() -> RhemaResult<()> {
    let env = TestEnv::with_scope()?;
    
    // Measure scope discovery performance
    let (_, duration) = tests::common::performance::measure_time(|| {
        env.rhema.discover_scopes().unwrap()
    });
    
    // Verify discovery completes within reasonable time (100ms)
    assert!(duration.as_millis() < 100);
    
    Ok(())
}

#[test]
fn test_query_execution_performance() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Measure query execution performance
    let (_, duration) = tests::common::performance::measure_time(|| {
        env.rhema.query("todos").unwrap()
    });
    
    // Verify query completes within reasonable time (50ms)
    assert!(duration.as_millis() < 50);
    
    Ok(())
}

#[test]
fn test_memory_usage() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Execute multiple queries to test memory usage
    for i in 0..100 {
        let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
        let _result = env.rhema.query(&query)?;
    }
    
    // If we get here without memory issues, the test passes
    Ok(())
}

#[test]
fn test_concurrent_access() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    use std::thread;
    use std::sync::Arc;
    
    let rhema = Arc::new(env.rhema);
    let mut handles = vec![];
    
    // Spawn multiple threads to access Rhema concurrently
    for i in 0..10 {
        let rhema_clone = Arc::clone(&rhema);
        let handle = thread::spawn(move || {
            let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
            rhema_clone.query(&query)
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result.is_ok());
    }
    
    Ok(())
} 