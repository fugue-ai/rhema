// Integration tests for Rhema CLI
// This file contains end-to-end tests that verify the complete functionality

use rhema::{Rhema, RhemaResult};

// Import test utilities
mod common {
    pub mod fixtures;
    pub mod helpers;
    pub mod assertions;
}

use common::helpers::TestHelpers;

/// Test basic Rhema initialization
#[test]
fn test_rhema_initialization() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    
    // Verify instance was created
    assert!(rhema.repo_root().exists(), "Rhema path should exist");
    
    Ok(())
}

/// Test scope discovery
#[test]
fn test_scope_discovery_integration() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a Rhema instance
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    
    // Create some test scopes
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;
    
    // Discover scopes
    let scopes = rhema.discover_scopes()?;
    
    // Verify scopes were found
    assert!(!scopes.is_empty(), "No scopes were discovered");
    
    Ok(())
}

/// Test query execution with filtering
#[test]
fn test_query_execution_integration() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a Rhema instance with test data
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;
    
    // Execute a simple query
    let result = rhema.query("simple")?;
    
    // Verify query result
    assert!(!result.is_null(), "Query should return results");
    
    Ok(())
}

/// Test search functionality
#[test]
fn test_search_integration() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();
    
    // Initialize git repository in the temp directory
    let _repo = git2::Repository::init(temp_path)?;
    
    // Create a Rhema instance with test data
    let rhema = Rhema::new_from_path(temp_path.to_path_buf())?;
    TestHelpers::create_basic_scope(&temp_path.to_path_buf())?;
    
    // Perform a search
    let search_results = rhema.search_regex("test", None)?;
    
    // Verify search results
    assert!(!search_results.is_empty(), "Search should return results");
    
    Ok(())
} 