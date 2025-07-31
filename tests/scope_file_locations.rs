use rhema::scope::Scope;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_scope_file_locations() {
    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Test 1: rhema.yaml in .rhema directory
    let rhema_dir = temp_path.join(".rhema");
    fs::create_dir(&rhema_dir).unwrap();

    let rhema_content = r#"
name: test-scope
scope_type: application
description: Test scope
version: 1.0.0
schema_version: 1.0.0
dependencies: null
protocol_info:
  version: 1.0.0
  description: Test protocol
  concepts: []
  cql_examples: []
  patterns: []
  integrations: []
  troubleshooting: []
build_files: []
tech_stack:
  languages: []
  frameworks: []
  databases: []
  infrastructure: []
  build_tools: []
directories: []
file_extensions: {}
"#;

    fs::write(rhema_dir.join("rhema.yaml"), rhema_content).unwrap();

    // Test that the scope can be loaded
    let scope = Scope::new(rhema_dir.clone());
    assert!(
        scope.is_ok(),
        "Should be able to load scope from .rhema/rhema.yaml"
    );

    // Test 2: scope.yaml in .rhema directory
    fs::remove_file(rhema_dir.join("rhema.yaml")).unwrap();
    fs::write(rhema_dir.join("scope.yaml"), rhema_content).unwrap();

    let scope = Scope::new(rhema_dir.clone());
    assert!(
        scope.is_ok(),
        "Should be able to load scope from .rhema/scope.yaml"
    );

    // Test 3: rhema.yaml in parent directory
    fs::remove_file(rhema_dir.join("scope.yaml")).unwrap();
    fs::write(temp_path.join("rhema.yaml"), rhema_content).unwrap();

    let scope = Scope::new(rhema_dir.clone());
    assert!(
        scope.is_ok(),
        "Should be able to load scope from parent/rhema.yaml"
    );

    // Test 4: scope.yaml in parent directory
    fs::remove_file(temp_path.join("rhema.yaml")).unwrap();
    fs::write(temp_path.join("scope.yaml"), rhema_content).unwrap();

    let scope = Scope::new(rhema_dir.clone());
    assert!(
        scope.is_ok(),
        "Should be able to load scope from parent/scope.yaml"
    );

    // Test 5: No file should fail
    fs::remove_file(temp_path.join("scope.yaml")).unwrap();

    let scope = Scope::new(rhema_dir.clone());
    assert!(scope.is_err(), "Should fail when no scope file exists");

    // Test 6: Priority order - rhema.yaml should be preferred over scope.yaml
    // Create both files in the .rhema directory to test priority within same directory
    fs::write(rhema_dir.join("scope.yaml"), rhema_content).unwrap();
    fs::write(rhema_dir.join("rhema.yaml"), rhema_content).unwrap();

    let rhema_file = Scope::find_scope_file(&rhema_dir).unwrap();
    // Should prefer rhema.yaml over scope.yaml in the same directory
    assert_eq!(
        rhema_file.file_name().unwrap(),
        "rhema.yaml",
        "Should prefer rhema.yaml over scope.yaml in same directory"
    );

    // Test 7: Priority order with files in different locations
    fs::remove_file(rhema_dir.join("scope.yaml")).unwrap();
    fs::write(temp_path.join("scope.yaml"), rhema_content).unwrap();

    let rhema_file = Scope::find_scope_file(&rhema_dir).unwrap();
    // Should prefer rhema.yaml in .rhema directory over scope.yaml in parent directory
    assert_eq!(
        rhema_file.file_name().unwrap(),
        "rhema.yaml",
        "Should prefer rhema.yaml in .rhema over scope.yaml in parent"
    );
}
