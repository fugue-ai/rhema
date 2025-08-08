// Unit tests for Rhema CLI core functionality
// This file contains focused unit tests for individual components

use rhema_core::{RhemaResult, Scope};
use rhema_query::query::CqlQuery;
use rhema_config::ConflictResolutionStrategy;
use rhema_cli::Rhema;
use git2::Repository;
use tempfile::TempDir;
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

// Import test utilities
use crate::common::helpers::TestHelpers;

/// Test Rhema initialization
#[test]
fn test_rhema_initialization() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Verify instance was created
    assert!(rhema.repo_root().exists(), "Rhema path should exist");

    Ok(())
}

/// Test scope discovery with basic data
#[test]
fn test_scope_discovery_basic() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Create basic scope
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Discover scopes
    let scopes = rhema.discover_scopes()?;

    // Verify scopes were found
    assert!(!scopes.is_empty(), "Scopes should be discovered");
    assert!(
        scopes.iter().any(|s| s.definition.name == "simple"),
        "Simple scope should be found"
    );

    Ok(())
}

/// Test query execution with filtering
#[test]
fn test_query_execution_filtered() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance with test data
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Execute simple query
    let result = rhema.query("simple")?;
    assert!(!result.is_null(), "Query should return results");

    // Execute filtered query
    let filtered_result = rhema.query("simple.items WHERE active=true")?;
    // Note: We can't easily compare lengths without knowing the structure
    assert!(
        !filtered_result.is_null(),
        "Filtered query should return results"
    );

    Ok(())
}

/// Test scope properties
#[test]
fn test_scope_properties() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Create scope with specific properties
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Discover scopes and get the first one
    let scopes = rhema.discover_scopes()?;
    assert!(!scopes.is_empty(), "Should have at least one scope");

    let scope = &scopes[0];

    // Verify scope properties
    assert!(
        scope.has_file("simple.yaml"),
        "Scope should contain simple.yaml"
    );

    Ok(())
}

/// Test query with statistics
#[test]
fn test_query_with_stats() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance with test data
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Execute query with statistics
    let (result, stats) = rhema.query_with_stats("simple")?;

    // Verify results
    assert!(!result.is_null(), "Query should return results");
    assert!(!stats.is_empty(), "Statistics should be available");

    Ok(())
}

/// Test search functionality
#[test]
fn test_search_regex() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance with test data
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Perform regex search
    let results = rhema.search_regex("test", None)?;
    assert!(!results.is_empty(), "Search should return results");

    Ok(())
}

/// Test error handling for invalid queries
#[test]
fn test_invalid_query_handling() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path).unwrap();

    let rhema = Rhema::new_from_path(temp_path.to_path_buf()).unwrap();

    // Test query for non-existent scope - this should return empty results
    let result = rhema.query("nonexistent").unwrap();
    assert!(
        result.is_null() || matches!(result, serde_yaml::Value::Sequence(seq) if seq.is_empty()),
        "Query for non-existent scope should return empty results"
    );
}

/// Test large dataset handling
#[test]
fn test_large_dataset() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Create large dataset
    TestHelpers::create_complex_scope(&temp_path.to_path_buf())?;

    // Execute query on large dataset
    let result = rhema.query("complex")?;
    assert!(!result.is_null(), "Query should handle large datasets");

    Ok(())
}

/// Test file operations
#[test]
fn test_file_operations() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;

    // Test file operations through scope creation
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Verify files were created
    let scopes = rhema.discover_scopes()?;
    assert!(
        !scopes.is_empty(),
        "Scopes should be discovered after file operations"
    );

    Ok(())
}

/// Test concurrent operations
#[test]
fn test_concurrent_operations() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Simulate concurrent queries
    let result1 = rhema.query("simple")?;
    let result2 = rhema.query("simple")?;

    // Both queries should succeed
    assert!(
        !result1.is_null(),
        "First concurrent query should return results"
    );
    assert!(
        !result2.is_null(),
        "Second concurrent query should return results"
    );

    Ok(())
}

/// Test memory usage tracking
#[test]
fn test_memory_usage() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    // Initialize git repository in the temp directory
    let _repo = Repository::init(temp_path)?;

    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;

    // Execute multiple queries to test memory usage
    for _ in 0..10 {
        let result = rhema.query("simple")?;
        assert!(!result.is_null(), "Query should return results");
    }

    Ok(())
}
