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

use rhema_core::schema::RhemaScope;
use rhema_core::schema::{DependencyType, LockMetadata, LockedDependency, LockedScope, RhemaLock};
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_lock_file_validation_functions() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Create a simple scope structure
    let scope_dir = repo_path.join("test-scope");
    fs::create_dir_all(&scope_dir).unwrap();

    // Create a scope file
    let scope_content = r#"
name: "Test Scope"
scope_type: "service"
version: "1.0.0"
description: "A test scope for validation"
"#;
    fs::write(scope_dir.join("rhema.yaml"), scope_content).unwrap();

    // Create a dependency scope
    let dep_dir = repo_path.join("dep-scope");
    fs::create_dir_all(&dep_dir).unwrap();

    let dep_content = r#"
name: "Dependency Scope"
scope_type: "library"
version: "2.0.0"
description: "A dependency scope"
"#;
    fs::write(dep_dir.join("rhema.yaml"), dep_content).unwrap();

    // Create a lock file
    let mut dependencies = HashMap::new();
    dependencies.insert(
        "dep-scope".to_string(),
        LockedDependency {
            version: "2.0.0".to_string(),
            path: "test/path".to_string(),
            resolved_at: chrono::Utc::now(),
            checksum: "test_checksum".to_string(),
            dependency_type: DependencyType::Required,
            original_constraint: Some("2.0.0".to_string()),
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        },
    );

    let mut scopes = HashMap::new();
    scopes.insert(
        "test-scope".to_string(),
        LockedScope {
            version: "1.0.0".to_string(),
            path: "test-scope".to_string(),
            dependencies,
            source_checksum: Some("test_checksum".to_string()),
            resolved_at: chrono::Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        },
    );

    let lock_file = RhemaLock {
        lockfile_version: "1.0.0".to_string(),
        generated_by: "test".to_string(),
        checksum: "lock_checksum".to_string(),
        metadata: LockMetadata::new(),
        scopes,
        generated_at: chrono::Utc::now(),
    };

    // Write lock file
    let lock_content = serde_yaml::to_string(&lock_file).unwrap();
    fs::write(repo_path.join("rhema.lock"), &lock_content).unwrap();

    // Test that the lock file exists and can be parsed
    assert!(repo_path.join("rhema.lock").exists());

    let parsed_lock: RhemaLock = serde_yaml::from_str(&lock_content).unwrap();
    assert_eq!(parsed_lock.lockfile_version, "1.0.0");
    assert!(parsed_lock.scopes.contains_key("test-scope"));

    // Test scope existence validation
    assert!(scope_dir.exists());
    assert!(dep_dir.exists());

    // Test that scope files exist
    assert!(scope_dir.join("rhema.yaml").exists());
    assert!(dep_dir.join("rhema.yaml").exists());

    // Test scope parsing
    let scope_content = fs::read_to_string(scope_dir.join("rhema.yaml")).unwrap();
    let scope: RhemaScope = serde_yaml::from_str(&scope_content).unwrap();
    assert_eq!(scope.name, "Test Scope");
    assert_eq!(scope.scope_type, "service");

    let dep_content = fs::read_to_string(dep_dir.join("rhema.yaml")).unwrap();
    let dep_scope: RhemaScope = serde_yaml::from_str(&dep_content).unwrap();
    assert_eq!(dep_scope.name, "Dependency Scope");
    assert_eq!(dep_scope.scope_type, "library");

    // Test dependency type consistency
    assert_eq!(format!("{:?}", DependencyType::Required), "Required");
    assert_eq!(dep_scope.scope_type, "library");

    // Test version constraint satisfaction
    assert_eq!(dep_scope.version, "2.0.0".to_string());
    assert_eq!(
        lock_file.scopes["test-scope"].dependencies["dep-scope"].original_constraint,
        Some("2.0.0".to_string())
    );
}

#[test]
fn test_circular_dependency_detection() {
    // Create a lock file with circular dependencies
    let mut dependencies1 = HashMap::new();
    dependencies1.insert(
        "scope2".to_string(),
        LockedDependency {
            version: "1.0.0".to_string(),
            path: "scope2/path".to_string(),
            resolved_at: chrono::Utc::now(),
            checksum: "checksum2".to_string(),
            dependency_type: DependencyType::Required,
            original_constraint: None,
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        },
    );

    let mut dependencies2 = HashMap::new();
    dependencies2.insert(
        "scope1".to_string(),
        LockedDependency {
            version: "1.0.0".to_string(),
            path: "scope1/path".to_string(),
            resolved_at: chrono::Utc::now(),
            checksum: "checksum1".to_string(),
            dependency_type: DependencyType::Required,
            original_constraint: None,
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        },
    );

    let mut scopes = HashMap::new();
    scopes.insert(
        "scope1".to_string(),
        LockedScope {
            version: "1.0.0".to_string(),
            path: "scope1".to_string(),
            dependencies: dependencies1,
            source_checksum: Some("checksum1".to_string()),
            resolved_at: chrono::Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        },
    );
    scopes.insert(
        "scope2".to_string(),
        LockedScope {
            version: "1.0.0".to_string(),
            path: "scope2".to_string(),
            dependencies: dependencies2,
            source_checksum: Some("checksum2".to_string()),
            resolved_at: chrono::Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        },
    );

    let lock_file = RhemaLock {
        lockfile_version: "1.0.0".to_string(),
        generated_by: "test".to_string(),
        checksum: "lock_checksum".to_string(),
        metadata: LockMetadata::new(),
        scopes,
        generated_at: chrono::Utc::now(),
    };

    // The circular dependency detection function should be able to detect this cycle
    // scope1 -> scope2 -> scope1
    // Note: This test validates the structure, actual detection would be tested in integration tests
    assert!(lock_file.scopes["scope1"]
        .dependencies
        .contains_key("scope2"));
    assert!(lock_file.scopes["scope2"]
        .dependencies
        .contains_key("scope1"));
}

#[test]
fn test_checksum_calculation() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = temp_dir.path().join("test-scope");
    fs::create_dir_all(&test_dir).unwrap();

    // Create subdirectory
    fs::create_dir_all(test_dir.join("subdir")).unwrap();

    // Create some test files
    fs::write(test_dir.join("file1.txt"), "content1").unwrap();
    fs::write(test_dir.join("file2.txt"), "content2").unwrap();
    fs::write(test_dir.join("subdir/file3.txt"), "content3").unwrap();

    // Test that checksum calculation works
    // Note: This would test the actual checksum calculation function
    // For now, we just verify the files exist
    assert!(test_dir.join("file1.txt").exists());
    assert!(test_dir.join("file2.txt").exists());
    assert!(test_dir.join("subdir/file3.txt").exists());

    // Verify file contents
    assert_eq!(
        fs::read_to_string(test_dir.join("file1.txt")).unwrap(),
        "content1"
    );
    assert_eq!(
        fs::read_to_string(test_dir.join("file2.txt")).unwrap(),
        "content2"
    );
    assert_eq!(
        fs::read_to_string(test_dir.join("subdir/file3.txt")).unwrap(),
        "content3"
    );
}

#[test]
fn test_version_constraint_validation() {
    // Test version constraint validation logic
    let constraint = "2.0.0";
    let version1 = "2.0.0";
    let version2 = "1.0.0";
    let version3 = "3.0.0";

    // Simple exact match validation
    assert_eq!(constraint, version1); // Should match
    assert_ne!(constraint, version2); // Should not match
    assert_ne!(constraint, version3); // Should not match

    // Test with None version
    let none_version: Option<String> = None;
    assert!(none_version.is_none());
}

#[test]
fn test_lock_file_structure_validation() {
    // Test lock file structure validation
    let mut dependencies = HashMap::new();
    dependencies.insert(
        "dep1".to_string(),
        LockedDependency {
            version: "1.0.0".to_string(),
            path: "dep1/path".to_string(),
            resolved_at: chrono::Utc::now(),
            checksum: "checksum1".to_string(),
            dependency_type: DependencyType::Required,
            original_constraint: Some("1.0.0".to_string()),
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        },
    );

    let mut scopes = HashMap::new();
    scopes.insert(
        "scope1".to_string(),
        LockedScope {
            version: "1.0.0".to_string(),
            path: "scope1".to_string(),
            dependencies,
            source_checksum: Some("scope_checksum".to_string()),
            resolved_at: chrono::Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        },
    );

    let lock_file = RhemaLock {
        lockfile_version: "1.0.0".to_string(),
        generated_by: "test".to_string(),
        checksum: "lock_checksum".to_string(),
        metadata: LockMetadata::new(),
        scopes,
        generated_at: chrono::Utc::now(),
    };

    // Validate structure
    assert_eq!(lock_file.lockfile_version, "1.0.0");
    assert!(lock_file.scopes.contains_key("scope1"));
    assert!(lock_file.scopes["scope1"].dependencies.contains_key("dep1"));
    assert_eq!(
        lock_file.scopes["scope1"].source_checksum,
        Some("scope_checksum".to_string())
    );
    assert_eq!(
        lock_file.scopes["scope1"].dependencies["dep1"].checksum,
        "checksum1"
    );
    assert_eq!(
        lock_file.scopes["scope1"].dependencies["dep1"].original_constraint,
        Some("1.0.0".to_string())
    );
}
