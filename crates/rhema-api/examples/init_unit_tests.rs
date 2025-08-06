//! Unit tests for Rhema init module functionality
//! 
//! This example demonstrates comprehensive unit testing for the Rhema init module,
//! including scope initialization, template file creation, repository analysis,
//! error handling, and protocol information generation.

use rhema_api::{Rhema, RhemaResult, RhemaError, init_run};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;

use serde_yaml;

/// Test fixture for creating temporary repositories
struct InitTestFixture {
    temp_dir: TempDir,
    repo_path: PathBuf,
    rhema: Rhema,
}

impl InitTestFixture {
    fn new() -> RhemaResult<Self> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path().join("test_repo");
        fs::create_dir_all(&repo_path)?;
        
        // Initialize git repository
        std::process::Command::new("git")
            .args(&["init"])
            .current_dir(&repo_path)
            .output()?;

        // Create a basic .gitignore to make it look more like a real repo
        fs::write(repo_path.join(".gitignore"), "target/\n*.log\n")?;

        // Create Rhema instance
        let rhema = Rhema::new_from_path(repo_path.clone())?;
        
        Ok(Self {
            temp_dir,
            repo_path,
            rhema,
        })
    }

    fn create_existing_rhema_files(&self, scope_path: &PathBuf) -> RhemaResult<()> {
        let rhema_files = [
            "rhema.yaml",
            "scope.yaml", 
            "knowledge.yaml",
            "todos.yaml",
            "decisions.yaml",
            "patterns.yaml",
            "conventions.yaml",
        ];

        for file in &rhema_files {
            fs::write(scope_path.join(file), format!("# Existing {}", file))?;
        }
        
        Ok(())
    }

    fn create_repo_structure(&self) -> RhemaResult<()> {
        // Create some typical repository structure for testing auto-config
        fs::create_dir_all(self.repo_path.join("src"))?;
        fs::create_dir_all(self.repo_path.join("tests"))?;
        fs::create_dir_all(self.repo_path.join("docs"))?;
        
        // Create Cargo.toml for Rust project detection
        fs::write(self.repo_path.join("Cargo.toml"), r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#)?;

        // Create package.json for Node.js project detection
        fs::write(self.repo_path.join("package.json"), r#"
{
  "name": "test-project",
  "version": "1.0.0",
  "description": "Test project"
}
"#)?;

        // Create docker-compose.yml for infrastructure detection
        fs::write(self.repo_path.join("docker-compose.yml"), r#"
version: '3.8'
services:
  app:
    image: test-app
"#)?;

        Ok(())
    }

    fn verify_rhema_files_exist(&self, scope_path: &PathBuf) -> RhemaResult<()> {
        let expected_files = [
            "rhema.yaml",
            "knowledge.yaml",
            "todos.yaml", 
            "decisions.yaml",
            "patterns.yaml",
            "conventions.yaml",
        ];

        for file in &expected_files {
            let file_path = scope_path.join(file);
            if !file_path.exists() {
                return Err(RhemaError::ValidationError(format!("Expected file {} does not exist", file)));
            }
        }
        
        Ok(())
    }

    fn verify_rhema_yaml_content(&self, scope_path: &PathBuf) -> RhemaResult<()> {
        let rhema_yaml_path = scope_path.join("rhema.yaml");
        let content = fs::read_to_string(rhema_yaml_path)?;
        let rhema_scope: serde_yaml::Value = serde_yaml::from_str(&content)?;
        
        // Verify required fields exist
        let name = rhema_scope.get("name")
            .ok_or_else(|| RhemaError::ValidationError("Missing 'name' field in rhema.yaml".to_string()))?;
        let scope_type = rhema_scope.get("scope_type")
            .ok_or_else(|| RhemaError::ValidationError("Missing 'scope_type' field in rhema.yaml".to_string()))?;
        let version = rhema_scope.get("version")
            .ok_or_else(|| RhemaError::ValidationError("Missing 'version' field in rhema.yaml".to_string()))?;
        
        // Verify values are not empty
        if name.as_str().unwrap_or("").is_empty() {
            return Err(RhemaError::ValidationError("Name field is empty".to_string()));
        }
        if scope_type.as_str().unwrap_or("").is_empty() {
            return Err(RhemaError::ValidationError("Scope type field is empty".to_string()));
        }
        if version.as_str().unwrap_or("").is_empty() {
            return Err(RhemaError::ValidationError("Version field is empty".to_string()));
        }
        
        Ok(())
    }

    fn verify_template_content(&self, scope_path: &PathBuf) -> RhemaResult<()> {
        // Verify knowledge.yaml template
        let knowledge_content = fs::read_to_string(scope_path.join("knowledge.yaml"))?;
        if !knowledge_content.contains("Knowledge Base") {
            return Err(RhemaError::ValidationError("Knowledge template missing expected content".to_string()));
        }
        if !knowledge_content.contains("entries: []") {
            return Err(RhemaError::ValidationError("Knowledge template missing entries array".to_string()));
        }

        // Verify todos.yaml template
        let todos_content = fs::read_to_string(scope_path.join("todos.yaml"))?;
        if !todos_content.contains("Todo Items") {
            return Err(RhemaError::ValidationError("Todos template missing expected content".to_string()));
        }
        if !todos_content.contains("todos: []") {
            return Err(RhemaError::ValidationError("Todos template missing todos array".to_string()));
        }

        // Verify decisions.yaml template
        let decisions_content = fs::read_to_string(scope_path.join("decisions.yaml"))?;
        if !decisions_content.contains("Decisions") {
            return Err(RhemaError::ValidationError("Decisions template missing expected content".to_string()));
        }
        if !decisions_content.contains("decisions: []") {
            return Err(RhemaError::ValidationError("Decisions template missing decisions array".to_string()));
        }

        // Verify patterns.yaml template
        let patterns_content = fs::read_to_string(scope_path.join("patterns.yaml"))?;
        if !patterns_content.contains("Patterns") {
            return Err(RhemaError::ValidationError("Patterns template missing expected content".to_string()));
        }
        if !patterns_content.contains("patterns: []") {
            return Err(RhemaError::ValidationError("Patterns template missing patterns array".to_string()));
        }

        // Verify conventions.yaml template
        let conventions_content = fs::read_to_string(scope_path.join("conventions.yaml"))?;
        if !conventions_content.contains("Conventions") {
            return Err(RhemaError::ValidationError("Conventions template missing expected content".to_string()));
        }
        if !conventions_content.contains("conventions: []") {
            return Err(RhemaError::ValidationError("Conventions template missing conventions array".to_string()));
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("🧪 Running Rhema Init Unit Tests...\n");

    // Test 1: Basic initialization at repository root
    test_basic_init_at_repo_root().await?;
    println!("✅ Basic initialization at repo root tests passed");

    // Test 2: Basic initialization in subdirectory
    test_basic_init_in_subdirectory().await?;
    println!("✅ Basic initialization in subdirectory tests passed");

    // Test 3: Auto-configuration with repository analysis
    test_auto_configuration().await?;
    println!("✅ Auto-configuration tests passed");

    // Test 4: Error handling for existing files
    test_error_handling_existing_files().await?;
    println!("✅ Error handling for existing files tests passed");

    // Test 5: Custom scope type and name
    test_custom_scope_type_and_name().await?;
    println!("✅ Custom scope type and name tests passed");

    // Test 6: Template file creation
    test_template_file_creation().await?;
    println!("✅ Template file creation tests passed");

    // Test 7: Protocol info generation
    test_protocol_info_generation().await?;
    println!("✅ Protocol info generation tests passed");

    // Test 8: Edge cases and error conditions
    test_edge_cases_and_errors().await?;
    println!("✅ Edge cases and error conditions tests passed");

    // Test 9: Integration with Rhema instance
    test_rhema_integration().await?;
    println!("✅ Rhema integration tests passed");

    // Test 10: Performance and concurrent initialization
    test_performance_and_concurrent_init().await?;
    println!("✅ Performance and concurrent initialization tests passed");

    println!("\n🎉 All init unit tests passed successfully!");
    Ok(())
}

async fn test_basic_init_at_repo_root() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Run initialization
    init_run(&fixture.rhema, None, None, false)?;
    
    // Verify .rhema directory was created at repo root
    let scope_path = fixture.repo_path.join(".rhema");
    if !scope_path.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created at repo root".to_string()));
    }
    
    // Verify all required files were created
    fixture.verify_rhema_files_exist(&scope_path)?;
    
    // Verify rhema.yaml content
    fixture.verify_rhema_yaml_content(&scope_path)?;
    
    // Verify template content
    fixture.verify_template_content(&scope_path)?;
    
    Ok(())
}

async fn test_basic_init_in_subdirectory() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Create a subdirectory
    let subdir = fixture.repo_path.join("src").join("service");
    fs::create_dir_all(&subdir)?;
    
    // Set current directory to subdirectory
    std::env::set_current_dir(&subdir)?;
    
    // Run initialization
    init_run(&fixture.rhema, None, None, false)?;
    
    // Verify .rhema directory was created in subdirectory
    let scope_path = subdir.join(".rhema");
    if !scope_path.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created in subdirectory".to_string()));
    }
    
    // Verify all required files were created
    fixture.verify_rhema_files_exist(&scope_path)?;
    
    // Verify rhema.yaml content
    fixture.verify_rhema_yaml_content(&scope_path)?;
    
    Ok(())
}

async fn test_auto_configuration() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Create repository structure for auto-detection
    fixture.create_repo_structure()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Run initialization with auto-config
    init_run(&fixture.rhema, None, None, true)?;
    
    // Verify .rhema directory was created
    let scope_path = fixture.repo_path.join(".rhema");
    if !scope_path.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created".to_string()));
    }
    
    // Verify rhema.yaml was created and contains auto-detected values
    let rhema_yaml_path = scope_path.join("rhema.yaml");
    let content = fs::read_to_string(rhema_yaml_path)?;
    let rhema_scope: serde_yaml::Value = serde_yaml::from_str(&content)?;
    
    // Verify auto-detected fields are present
    let name = rhema_scope.get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| RhemaError::ValidationError("Missing or invalid name field".to_string()))?;
    let scope_type = rhema_scope.get("scope_type")
        .and_then(|s| s.as_str())
        .ok_or_else(|| RhemaError::ValidationError("Missing or invalid scope_type field".to_string()))?;
    
    // Verify the values are not empty
    if name.is_empty() {
        return Err(RhemaError::ValidationError("Auto-detected name is empty".to_string()));
    }
    if scope_type.is_empty() {
        return Err(RhemaError::ValidationError("Auto-detected scope_type is empty".to_string()));
    }
    
    // Verify protocol info was created
    let protocol_info = rhema_scope.get("protocol_info")
        .ok_or_else(|| RhemaError::ValidationError("Missing protocol_info field".to_string()))?;
    
    if protocol_info.get("version").is_none() {
        return Err(RhemaError::ValidationError("Protocol info missing version".to_string()));
    }
    
    Ok(())
}

async fn test_error_handling_existing_files() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Create .rhema directory with existing files
    let scope_path = fixture.repo_path.join(".rhema");
    fs::create_dir_all(&scope_path)?;
    fixture.create_existing_rhema_files(&scope_path)?;
    
    // Attempt to run initialization - should fail
    let result = init_run(&fixture.rhema, None, None, false);
    
    match result {
        Ok(_) => return Err(RhemaError::ValidationError("Expected initialization to fail with existing files".to_string())),
        Err(e) => {
            let error_msg = e.to_string();
            if !error_msg.contains("already exist") {
                return Err(RhemaError::ValidationError(format!("Expected error about existing files, got: {}", error_msg)));
            }
        }
    }
    
    Ok(())
}

async fn test_custom_scope_type_and_name() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Run initialization with custom scope type and name
    let custom_scope_type = "microservice";
    let custom_scope_name = "user-service";
    
    init_run(&fixture.rhema, Some(custom_scope_type), Some(custom_scope_name), false)?;
    
    // Verify .rhema directory was created
    let scope_path = fixture.repo_path.join(".rhema");
    if !scope_path.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created".to_string()));
    }
    
    // Verify rhema.yaml contains custom values
    let rhema_yaml_path = scope_path.join("rhema.yaml");
    let content = fs::read_to_string(rhema_yaml_path)?;
    let rhema_scope: serde_yaml::Value = serde_yaml::from_str(&content)?;
    
    let name = rhema_scope.get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| RhemaError::ValidationError("Missing name field".to_string()))?;
    let scope_type = rhema_scope.get("scope_type")
        .and_then(|s| s.as_str())
        .ok_or_else(|| RhemaError::ValidationError("Missing scope_type field".to_string()))?;
    
    if name != custom_scope_name {
        return Err(RhemaError::ValidationError(format!("Expected name '{}', got '{}'", custom_scope_name, name)));
    }
    if scope_type != custom_scope_type {
        return Err(RhemaError::ValidationError(format!("Expected scope_type '{}', got '{}'", custom_scope_type, scope_type)));
    }
    
    Ok(())
}

async fn test_template_file_creation() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Run initialization
    init_run(&fixture.rhema, None, None, false)?;
    
    // Verify .rhema directory was created
    let scope_path = fixture.repo_path.join(".rhema");
    if !scope_path.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created".to_string()));
    }
    
    // Verify all template files were created with correct content
    fixture.verify_template_content(&scope_path)?;
    
    // Verify specific template content details
    let knowledge_content = fs::read_to_string(scope_path.join("knowledge.yaml"))?;
    if !knowledge_content.contains("categories:") {
        return Err(RhemaError::ValidationError("Knowledge template missing categories section".to_string()));
    }
    
    let todos_content = fs::read_to_string(scope_path.join("todos.yaml"))?;
    if !todos_content.contains("Todo Items") {
        return Err(RhemaError::ValidationError("Todos template missing expected header".to_string()));
    }
    
    Ok(())
}

async fn test_protocol_info_generation() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Run initialization
    init_run(&fixture.rhema, None, None, false)?;
    
    // Verify .rhema directory was created
    let scope_path = fixture.repo_path.join(".rhema");
    if !scope_path.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created".to_string()));
    }
    
    // Verify rhema.yaml contains protocol info
    let rhema_yaml_path = scope_path.join("rhema.yaml");
    let content = fs::read_to_string(rhema_yaml_path)?;
    let rhema_scope: serde_yaml::Value = serde_yaml::from_str(&content)?;
    
    let protocol_info = rhema_scope.get("protocol_info")
        .ok_or_else(|| RhemaError::ValidationError("Missing protocol_info field".to_string()))?;
    
    // Verify protocol info structure
    let version = protocol_info.get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RhemaError::ValidationError("Protocol info missing version".to_string()))?;
    
    if version != "1.0.0" {
        return Err(RhemaError::ValidationError(format!("Expected protocol version '1.0.0', got '{}'", version)));
    }
    
    // Verify concepts are present
    let concepts = protocol_info.get("concepts")
        .ok_or_else(|| RhemaError::ValidationError("Protocol info missing concepts".to_string()))?;
    
    // Verify concepts are present and not null
    if matches!(concepts, serde_yaml::Value::Null) {
        return Err(RhemaError::ValidationError("Concepts should not be null".to_string()));
    }
    
    // Verify CQL examples are present
    let cql_examples = protocol_info.get("cql_examples")
        .ok_or_else(|| RhemaError::ValidationError("Protocol info missing cql_examples".to_string()))?;
    
    // Verify CQL examples are present and not null
    if matches!(cql_examples, serde_yaml::Value::Null) {
        return Err(RhemaError::ValidationError("CQL examples should not be null".to_string()));
    }
    
    // Verify patterns are present
    let patterns = protocol_info.get("patterns")
        .ok_or_else(|| RhemaError::ValidationError("Protocol info missing patterns".to_string()))?;
    
    // Verify patterns are present and not null
    if matches!(patterns, serde_yaml::Value::Null) {
        return Err(RhemaError::ValidationError("Patterns should not be null".to_string()));
    }
    
    Ok(())
}

async fn test_edge_cases_and_errors() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Test 1: Empty scope type (should use default)
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Remove any existing .rhema directory
    let scope_path = fixture.repo_path.join(".rhema");
    if scope_path.exists() {
        fs::remove_dir_all(&scope_path)?;
    }
    
    // Run initialization with empty scope type
    init_run(&fixture.rhema, Some(""), None, false)?;
    
    // Verify it still works (should use default "service")
    if !scope_path.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created with empty scope type".to_string()));
    }
    
    // Test 2: Very long scope name
    let long_name = "a".repeat(1000);
    let scope_path2 = fixture.repo_path.join("long_name_test");
    fs::create_dir_all(&scope_path2)?;
    std::env::set_current_dir(&scope_path2)?;
    
    // This should work (no length validation in the current implementation)
    init_run(&fixture.rhema, None, Some(&long_name), false)?;
    
    let scope_path_long = scope_path2.join(".rhema");
    if !scope_path_long.exists() {
        return Err(RhemaError::ValidationError("Expected .rhema directory not created with long name".to_string()));
    }
    
    Ok(())
}

async fn test_rhema_integration() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Run initialization
    init_run(&fixture.rhema, None, None, false)?;
    
    // Verify the Rhema instance can discover the created scope
    let scopes = fixture.rhema.discover_scopes()?;
    if scopes.is_empty() {
        return Err(RhemaError::ValidationError("Rhema instance should discover the created scope".to_string()));
    }
    
    // Verify the scope can be loaded
    let scope = fixture.rhema.get_scope(".")?;
    // The scope path should be the repo path or a subdirectory of it
    if !scope.path.starts_with(&fixture.repo_path) {
        return Err(RhemaError::ValidationError(format!(
            "Discovered scope path {:?} should be within repo path {:?}",
            scope.path, fixture.repo_path
        )));
    }
    
    // Verify scope files are accessible
    let knowledge = fixture.rhema.load_knowledge(".")?;
    if knowledge.entries.len() != 0 {
        return Err(RhemaError::ValidationError("New scope should have empty knowledge entries".to_string()));
    }
    
    let todos = fixture.rhema.load_todos(".")?;
    if todos.todos.len() != 0 {
        return Err(RhemaError::ValidationError("New scope should have empty todos".to_string()));
    }
    
    Ok(())
}

async fn test_performance_and_concurrent_init() -> RhemaResult<()> {
    let fixture = InitTestFixture::new()?;
    
    // Set current directory to repo root
    std::env::set_current_dir(&fixture.repo_path)?;
    
    // Test performance by running initialization multiple times
    let start = std::time::Instant::now();
    
    // Run initialization
    init_run(&fixture.rhema, None, None, false)?;
    
    let duration = start.elapsed();
    println!("  ⏱️  Initialization took: {:?}", duration);
    
    // Verify it completed in reasonable time (less than 5 seconds)
    if duration > std::time::Duration::from_secs(5) {
        return Err(RhemaError::ValidationError("Initialization took too long".to_string()));
    }
    
    // Test concurrent initialization attempts (should fail due to existing files)
    let mut handles = vec![];
    
    for i in 0..3 {
        let rhema_clone = Rhema::new_from_path(fixture.repo_path.clone())?;
        let handle = tokio::spawn(async move {
            init_run(&rhema_clone, Some(&format!("service-{}", i)), None, false)
        });
        handles.push(handle);
    }
    
    // Wait for all concurrent attempts
    for handle in handles {
        let result = handle.await.map_err(|e| RhemaError::ValidationError(format!("Join error: {}", e)))?;
        // All should fail due to existing files
        if result.is_ok() {
            return Err(RhemaError::ValidationError("Concurrent initialization should fail due to existing files".to_string()));
        }
    }
    
    Ok(())
}

// Additional helper tests for specific functionality

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[tokio::test]
    async fn test_init_with_special_characters() -> RhemaResult<()> {
        let fixture = InitTestFixture::new()?;
        
        // Test with scope name containing special characters
        let special_name = "test-service-v1.0.0";
        
        std::env::set_current_dir(&fixture.repo_path)?;
        init_run(&fixture.rhema, None, Some(special_name), false)?;
        
        let scope_path = fixture.repo_path.join(".rhema");
        let rhema_yaml_path = scope_path.join("rhema.yaml");
        let content = fs::read_to_string(rhema_yaml_path)?;
        let rhema_scope: serde_yaml::Value = serde_yaml::from_str(&content)?;
        
        let name = rhema_scope.get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| RhemaError::ValidationError("Missing name field".to_string()))?;
        
        if name != special_name {
            return Err(RhemaError::ValidationError(format!("Expected name '{}', got '{}'", special_name, name)));
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_init_with_unicode_characters() -> RhemaResult<()> {
        let fixture = InitTestFixture::new()?;
        
        // Test with scope name containing unicode characters
        let unicode_name = "service-测试-🚀";
        
        std::env::set_current_dir(&fixture.repo_path)?;
        init_run(&fixture.rhema, None, Some(unicode_name), false)?;
        
        let scope_path = fixture.repo_path.join(".rhema");
        let rhema_yaml_path = scope_path.join("rhema.yaml");
        let content = fs::read_to_string(rhema_yaml_path)?;
        let rhema_scope: serde_yaml::Value = serde_yaml::from_str(&content)?;
        
        let name = rhema_scope.get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| RhemaError::ValidationError("Missing name field".to_string()))?;
        
        if name != unicode_name {
            return Err(RhemaError::ValidationError(format!("Expected name '{}', got '{}'", unicode_name, name)));
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_init_with_different_scope_types() -> RhemaResult<()> {
        let fixture = InitTestFixture::new()?;
        
        let scope_types = ["service", "library", "application", "microservice", "monorepo"];
        
        for scope_type in &scope_types {
            // Create a new subdirectory for each test
            let test_dir = fixture.repo_path.join(format!("test-{}", scope_type));
            fs::create_dir_all(&test_dir)?;
            std::env::set_current_dir(&test_dir)?;
            
            init_run(&fixture.rhema, Some(scope_type), None, false)?;
            
            let scope_path = test_dir.join(".rhema");
            let rhema_yaml_path = scope_path.join("rhema.yaml");
            let content = fs::read_to_string(rhema_yaml_path)?;
            let rhema_scope: serde_yaml::Value = serde_yaml::from_str(&content)?;
            
            let detected_scope_type = rhema_scope.get("scope_type")
                .and_then(|s| s.as_str())
                .ok_or_else(|| RhemaError::ValidationError("Missing scope_type field".to_string()))?;
            
            if detected_scope_type != *scope_type {
                return Err(RhemaError::ValidationError(format!("Expected scope_type '{}', got '{}'", scope_type, detected_scope_type)));
            }
        }
        
        Ok(())
    }
} 