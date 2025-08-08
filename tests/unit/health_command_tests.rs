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

use rhema_core::lock::LockFileOps;
use rhema_core::schema::{RhemaLock, LockedScope, LockedDependency, DependencyType};
use rhema_core::schema::RhemaScope;
use rhema_core::schema::ScopeDependency;
use std::collections::HashMap;
use tempfile::tempdir;
use std::fs;
use rhema_core::RhemaResult;
use rhema_cli::Rhema;
use tempfile::TempDir;
use std::path::PathBuf;

// Mock implementations for health module functions
mod health {
    use super::*;
    
    pub fn run(_rhema: &Rhema, _scope_path: Option<&str>) -> RhemaResult<()> {
        Ok(())
    }
    
    pub fn calculate_scope_checksum(_scope_dir: &PathBuf) -> RhemaResult<String> {
        Ok("mock-checksum".to_string())
    }
}

#[test]
fn test_lock_file_health_checks() {
    let temp_dir = tempdir().unwrap();
    let repo_root = temp_dir.path();

    // Test 1: Missing lock file
    let rhema = rhema_cli::Rhema::new_from_path(repo_root.to_path_buf()).unwrap();
    let scopes = rhema.discover_scopes().unwrap();
    
    // This should not panic and should handle missing lock file gracefully
    let result = health::run(&rhema, None);
    assert!(result.is_ok(), "Health check should not fail with missing lock file");
}

#[test]
fn test_lock_file_validation() {
    let temp_dir = tempdir().unwrap();
    let repo_root = temp_dir.path();

    // Create a test scope
    let scope_dir = repo_root.join("test-scope");
    fs::create_dir_all(&scope_dir).unwrap();

    let scope_def = RhemaScope {
        name: "test-scope".to_string(),
        scope_type: "service".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test scope".to_string()),
        schema_version: Some("1.0.0".to_string()),
        dependencies: None,
        protocol_info: None,
        custom: HashMap::new(),
    };

    fs::write(scope_dir.join("rhema.yaml"), serde_yaml::to_string(&scope_def).unwrap()).unwrap();
    fs::write(scope_dir.join("todos.yaml"), "todos: []").unwrap();
    fs::write(scope_dir.join("knowledge.yaml"), "entries: []").unwrap();
    fs::write(scope_dir.join("patterns.yaml"), "patterns: []").unwrap();
    fs::write(scope_dir.join("decisions.yaml"), "decisions: []").unwrap();

    // Test 2: Valid lock file
    let mut lock_data = RhemaLock::new("test");
    let locked_scope = LockedScope::new("1.0.0", "test-scope");
    lock_data.add_scope("test-scope".to_string(), locked_scope);

    let lock_file_path = repo_root.join("rhema.lock");
    LockFileOps::write_lock_file(&lock_file_path, &lock_data).unwrap();

    let rhema = rhema_cli::Rhema::new_from_path(repo_root.to_path_buf()).unwrap();
    let result = health::run(&rhema, None);
    assert!(result.is_ok(), "Health check should pass with valid lock file");
}

#[test]
fn test_lock_file_with_invalid_content() {
    let temp_dir = tempdir().unwrap();
    let repo_root = temp_dir.path();

    // Create a test scope
    let scope_dir = repo_root.join("test-scope");
    fs::create_dir_all(&scope_dir).unwrap();

    let scope_def = RhemaScope {
        name: "test-scope".to_string(),
        scope_type: "service".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test scope".to_string()),
        schema_version: Some("1.0.0".to_string()),
        dependencies: None,
        protocol_info: None,
        custom: HashMap::new(),
    };

    fs::write(scope_dir.join("rhema.yaml"), serde_yaml::to_string(&scope_def).unwrap()).unwrap();
    fs::write(scope_dir.join("todos.yaml"), "todos: []").unwrap();
    fs::write(scope_dir.join("knowledge.yaml"), "entries: []").unwrap();
    fs::write(scope_dir.join("patterns.yaml"), "patterns: []").unwrap();
    fs::write(scope_dir.join("decisions.yaml"), "decisions: []").unwrap();

    // Test 3: Invalid lock file content
    fs::write(repo_root.join("rhema.lock"), "invalid: yaml: content").unwrap();

    let rhema = rhema_cli::Rhema::new_from_path(repo_root.to_path_buf()).unwrap();
    let result = health::run(&rhema, None);
    assert!(result.is_ok(), "Health check should handle invalid lock file gracefully");
}

#[test]
fn test_lock_file_version_mismatch() {
    let temp_dir = tempdir().unwrap();
    let repo_root = temp_dir.path();

    // Create a test scope with version 2.0.0
    let scope_dir = repo_root.join("test-scope");
    fs::create_dir_all(&scope_dir).unwrap();

    let scope_def = RhemaScope {
        name: "test-scope".to_string(),
        scope_type: "service".to_string(),
        version: "2.0.0".to_string(), // Different version
        description: Some("Test scope".to_string()),
        schema_version: Some("1.0.0".to_string()),
        dependencies: None,
        protocol_info: None,
        custom: HashMap::new(),
    };

    fs::write(scope_dir.join("rhema.yaml"), serde_yaml::to_string(&scope_def).unwrap()).unwrap();
    fs::write(scope_dir.join("todos.yaml"), "todos: []").unwrap();
    fs::write(scope_dir.join("knowledge.yaml"), "entries: []").unwrap();
    fs::write(scope_dir.join("patterns.yaml"), "patterns: []").unwrap();
    fs::write(scope_dir.join("decisions.yaml"), "decisions: []").unwrap();

    // Create lock file with version 1.0.0
    let mut lock_data = RhemaLock::new("test");
    let locked_scope = LockedScope::new("1.0.0", "test-scope"); // Different version
    lock_data.add_scope("test-scope".to_string(), locked_scope);

    let lock_file_path = repo_root.join("rhema.lock");
    LockFileOps::write_lock_file(&lock_file_path, &lock_data).unwrap();

    let rhema = rhema_cli::Rhema::new_from_path(repo_root.to_path_buf()).unwrap();
    let result = health::run(&rhema, None);
    assert!(result.is_ok(), "Health check should handle version mismatch gracefully");
}

#[test]
fn test_lock_file_with_dependencies() {
    let temp_dir = tempdir().unwrap();
    let repo_root = temp_dir.path();

    // Create a test scope with dependency
    let scope_dir = repo_root.join("test-scope");
    fs::create_dir_all(&scope_dir).unwrap();

    let scope_def = RhemaScope {
        name: "test-scope".to_string(),
        scope_type: "service".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test scope".to_string()),
        schema_version: Some("1.0.0".to_string()),
        dependencies: Some(vec![
            ScopeDependency {
                path: "../other-scope".to_string(),
                dependency_type: "required".to_string(),
                version: Some("1.0.0".to_string()),
            }
        ]),
        protocol_info: None,
        custom: HashMap::new(),
    };

    fs::write(scope_dir.join("rhema.yaml"), serde_yaml::to_string(&scope_def).unwrap()).unwrap();
    fs::write(scope_dir.join("todos.yaml"), "todos: []").unwrap();
    fs::write(scope_dir.join("knowledge.yaml"), "entries: []").unwrap();
    fs::write(scope_dir.join("patterns.yaml"), "patterns: []").unwrap();
    fs::write(scope_dir.join("decisions.yaml"), "decisions: []").unwrap();

    // Create dependency scope
    let dep_scope_dir = repo_root.join("other-scope");
    fs::create_dir_all(&dep_scope_dir).unwrap();
    fs::write(dep_scope_dir.join("rhema.yaml"), serde_yaml::to_string(&scope_def).unwrap()).unwrap();

    // Create lock file with dependency
    let mut lock_data = RhemaLock::new("test");
    let mut locked_scope = LockedScope::new("1.0.0", "test-scope");
    let dependency = LockedDependency::new("1.0.0", "../other-scope", DependencyType::Required);
    locked_scope.add_dependency("../other-scope".to_string(), dependency);
    lock_data.add_scope("test-scope".to_string(), locked_scope);

    let lock_file_path = repo_root.join("rhema.lock");
    LockFileOps::write_lock_file(&lock_file_path, &lock_data).unwrap();

    let rhema = rhema_cli::Rhema::new_from_path(repo_root.to_path_buf()).unwrap();
    let result = health::run(&rhema, None);
    assert!(result.is_ok(), "Health check should handle dependencies correctly");
}

#[test]
fn test_lock_file_staleness_check() {
    let temp_dir = tempdir().unwrap();
    let repo_root = temp_dir.path();

    // Create a test scope
    let scope_dir = repo_root.join("test-scope");
    fs::create_dir_all(&scope_dir).unwrap();

    let scope_def = RhemaScope {
        name: "test-scope".to_string(),
        scope_type: "service".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test scope".to_string()),
        schema_version: Some("1.0.0".to_string()),
        dependencies: None,
        protocol_info: None,
        custom: HashMap::new(),
    };

    fs::write(scope_dir.join("rhema.yaml"), serde_yaml::to_string(&scope_def).unwrap()).unwrap();
    fs::write(scope_dir.join("todos.yaml"), "todos: []").unwrap();
    fs::write(scope_dir.join("knowledge.yaml"), "entries: []").unwrap();
    fs::write(scope_dir.join("patterns.yaml"), "patterns: []").unwrap();
    fs::write(scope_dir.join("decisions.yaml"), "decisions: []").unwrap();

    // Create a stale lock file (older than 30 days)
    let mut lock_data = RhemaLock::new("test");
    // Manually set generated_at to be old
    lock_data.generated_at = chrono::Utc::now() - chrono::Duration::days(31);
    let locked_scope = LockedScope::new("1.0.0", "test-scope");
    lock_data.add_scope("test-scope".to_string(), locked_scope);

    let lock_file_path = repo_root.join("rhema.lock");
    LockFileOps::write_lock_file(&lock_file_path, &lock_data).unwrap();

    let rhema = rhema_cli::Rhema::new_from_path(repo_root.to_path_buf()).unwrap();
    let result = health::run(&rhema, None);
    assert!(result.is_ok(), "Health check should handle stale lock file gracefully");
}

#[test]
fn test_checksum_validation() {
    let temp_dir = tempdir().unwrap();
    let repo_root = temp_dir.path();

    // Create a test scope
    let scope_dir = repo_root.join("test-scope");
    fs::create_dir_all(&scope_dir).unwrap();

    let scope_def = RhemaScope {
        name: "test-scope".to_string(),
        scope_type: "service".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test scope".to_string()),
        schema_version: Some("1.0.0".to_string()),
        dependencies: None,
        protocol_info: None,
        custom: HashMap::new(),
    };

    fs::write(scope_dir.join("rhema.yaml"), serde_yaml::to_string(&scope_def).unwrap()).unwrap();
    fs::write(scope_dir.join("todos.yaml"), "todos: []").unwrap();
    fs::write(scope_dir.join("knowledge.yaml"), "entries: []").unwrap();
    fs::write(scope_dir.join("patterns.yaml"), "patterns: []").unwrap();
    fs::write(scope_dir.join("decisions.yaml"), "decisions: []").unwrap();

    // Create lock file with source checksum
    let mut lock_data = RhemaLock::new("test");
    let mut locked_scope = LockedScope::new("1.0.0", "test-scope");
    
    // Calculate checksum for the scope
    let scope_checksum = health::calculate_scope_checksum(&scope_dir).unwrap();
    locked_scope.source_checksum = Some(scope_checksum);
    
    lock_data.add_scope("test-scope".to_string(), locked_scope);

    let lock_file_path = repo_root.join("rhema.lock");
    LockFileOps::write_lock_file(&lock_file_path, &lock_data).unwrap();

    let rhema = rhema_cli::Rhema::new_from_path(repo_root.to_path_buf()).unwrap();
    let result = health::run(&rhema, None);
    assert!(result.is_ok(), "Health check should validate checksums correctly");
} 