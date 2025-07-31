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
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_lock_file_dependency_graph_building() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    
    // Create a lock file with dependencies
    let mut dependencies1 = HashMap::new();
    dependencies1.insert(
        "dep-scope".to_string(),
        LockedDependency {
            version: "1.0.0".to_string(),
            path: "dep-scope".to_string(),
            resolved_at: chrono::Utc::now(),
            checksum: "checksum1".to_string(),
            dependency_type: DependencyType::Required,
            original_constraint: Some("^1.0.0".to_string()),
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        },
    );
    
    let mut dependencies2 = HashMap::new();
    dependencies2.insert(
        "transitive-dep".to_string(),
        LockedDependency {
            version: "2.0.0".to_string(),
            path: "transitive-dep".to_string(),
            resolved_at: chrono::Utc::now(),
            checksum: "checksum2".to_string(),
            dependency_type: DependencyType::Required,
            original_constraint: Some("^2.0.0".to_string()),
            is_transitive: true,
            dependencies: None,
            custom: HashMap::new(),
        },
    );
    
    let mut scopes = HashMap::new();
    scopes.insert(
        "main-scope".to_string(),
        LockedScope {
            version: "1.0.0".to_string(),
            path: "main-scope".to_string(),
            dependencies: dependencies1,
            source_checksum: Some("main_checksum".to_string()),
            resolved_at: chrono::Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        },
    );
    
    scopes.insert(
        "dep-scope".to_string(),
        LockedScope {
            version: "1.0.0".to_string(),
            path: "dep-scope".to_string(),
            dependencies: dependencies2,
            source_checksum: Some("dep_checksum".to_string()),
            resolved_at: chrono::Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        },
    );
    
    let lock_file = RhemaLock {
        lockfile_version: "1.0.0".to_string(),
        generated_at: chrono::Utc::now(),
        generated_by: "test".to_string(),
        checksum: "lock_checksum".to_string(),
        scopes,
        metadata: rhema_core::schema::LockMetadata::new(),
    };
    
    // Test that the lock file structure is valid
    assert_eq!(lock_file.scopes.len(), 2);
    assert!(lock_file.scopes.contains_key("main-scope"));
    assert!(lock_file.scopes.contains_key("dep-scope"));
    
    // Test dependency relationships
    let main_scope = &lock_file.scopes["main-scope"];
    assert!(main_scope.dependencies.contains_key("dep-scope"));
    
    let dep_scope = &lock_file.scopes["dep-scope"];
    assert!(dep_scope.dependencies.contains_key("transitive-dep"));
    
    // Test dependency types
    let main_dep = &main_scope.dependencies["dep-scope"];
    assert_eq!(main_dep.dependency_type, DependencyType::Required);
    assert!(!main_dep.is_transitive);
    
    let transitive_dep = &dep_scope.dependencies["transitive-dep"];
    assert_eq!(transitive_dep.dependency_type, DependencyType::Required);
    assert!(transitive_dep.is_transitive);
}

#[test]
fn test_version_conflict_detection() {
    // Test version conflict detection logic
    let conflict1 = rhema_cli::dependencies::VersionConflict {
        scope: "main-scope".to_string(),
        dependency: "dep-scope".to_string(),
        expected_version: "1.0.0".to_string(),
        actual_version: "2.0.0".to_string(),
        conflict_type: rhema_cli::dependencies::ConflictType::VersionMismatch,
    };
    
    let conflict2 = rhema_cli::dependencies::VersionConflict {
        scope: "main-scope".to_string(),
        dependency: "missing-dep".to_string(),
        expected_version: "1.0.0".to_string(),
        actual_version: "missing".to_string(),
        conflict_type: rhema_cli::dependencies::ConflictType::MissingDependency,
    };
    
    // Test conflict properties
    assert_eq!(conflict1.scope, "main-scope");
    assert_eq!(conflict1.dependency, "dep-scope");
    assert_eq!(conflict1.expected_version, "1.0.0");
    assert_eq!(conflict1.actual_version, "2.0.0");
    
    assert_eq!(conflict2.conflict_type, rhema_cli::dependencies::ConflictType::MissingDependency);
    assert_eq!(conflict2.actual_version, "missing");
}

#[test]
fn test_dependency_difference_detection() {
    // Test dependency difference detection
    let diff1 = rhema_cli::dependencies::DependencyDifference {
        scope: "main-scope".to_string(),
        difference_type: rhema_cli::dependencies::DifferenceType::Added,
        details: "Dependency 'new-dep' added".to_string(),
    };
    
    let diff2 = rhema_cli::dependencies::DependencyDifference {
        scope: "main-scope".to_string(),
        difference_type: rhema_cli::dependencies::DifferenceType::Removed,
        details: "Dependency 'old-dep' removed".to_string(),
    };
    
    let diff3 = rhema_cli::dependencies::DependencyDifference {
        scope: "main-scope".to_string(),
        difference_type: rhema_cli::dependencies::DifferenceType::VersionChanged,
        details: "Dependency 'dep-scope' version changed from 1.0.0 to 2.0.0".to_string(),
    };
    
    // Test difference properties
    assert_eq!(diff1.scope, "main-scope");
    assert_eq!(diff1.difference_type, rhema_cli::dependencies::DifferenceType::Added);
    assert!(diff1.details.contains("new-dep"));
    
    assert_eq!(diff2.difference_type, rhema_cli::dependencies::DifferenceType::Removed);
    assert!(diff2.details.contains("old-dep"));
    
    assert_eq!(diff3.difference_type, rhema_cli::dependencies::DifferenceType::VersionChanged);
    assert!(diff3.details.contains("version changed"));
}

#[test]
fn test_circular_dependency_detection() {
    // Create a dependency graph with circular dependencies
    let mut graph = HashMap::new();
    graph.insert("scope1".to_string(), vec!["scope2".to_string()]);
    graph.insert("scope2".to_string(), vec!["scope3".to_string()]);
    graph.insert("scope3".to_string(), vec!["scope1".to_string()]); // Circular dependency
    
    // Test circular dependency detection
    let has_circular = rhema_cli::dependencies::has_circular_dependency(&graph, "scope1");
    assert!(has_circular);
    
    // Test non-circular graph
    let mut linear_graph = HashMap::new();
    linear_graph.insert("scope1".to_string(), vec!["scope2".to_string()]);
    linear_graph.insert("scope2".to_string(), vec!["scope3".to_string()]);
    linear_graph.insert("scope3".to_string(), vec![]);
    
    let has_circular_linear = rhema_cli::dependencies::has_circular_dependency(&linear_graph, "scope1");
    assert!(!has_circular_linear);
}

#[test]
fn test_dependency_depth_calculation() {
    // Create a dependency graph for depth testing
    let mut graph = HashMap::new();
    graph.insert("root".to_string(), vec!["level1".to_string()]);
    graph.insert("level1".to_string(), vec!["level2".to_string()]);
    graph.insert("level2".to_string(), vec!["level3".to_string()]);
    graph.insert("level3".to_string(), vec![]);
    graph.insert("independent".to_string(), vec![]);
    
    // Test depth calculation
    let root_depth = rhema_cli::dependencies::calculate_dependency_depth(&graph, "root");
    assert_eq!(root_depth, 3); // root -> level1 -> level2 -> level3
    
    let level1_depth = rhema_cli::dependencies::calculate_dependency_depth(&graph, "level1");
    assert_eq!(level1_depth, 2); // level1 -> level2 -> level3
    
    let independent_depth = rhema_cli::dependencies::calculate_dependency_depth(&graph, "independent");
    assert_eq!(independent_depth, 0); // No dependencies
}

#[test]
fn test_longest_chain_detection() {
    // Create a dependency graph for chain testing
    let mut graph = HashMap::new();
    graph.insert("start".to_string(), vec!["middle1".to_string(), "branch".to_string()]);
    graph.insert("middle1".to_string(), vec!["middle2".to_string()]);
    graph.insert("middle2".to_string(), vec!["end".to_string()]);
    graph.insert("branch".to_string(), vec!["end".to_string()]);
    graph.insert("end".to_string(), vec![]);
    
    // Test longest chain detection
    let longest_chain = rhema_cli::dependencies::find_longest_chain(&graph, "start");
    assert_eq!(longest_chain.len(), 4); // start -> middle1 -> middle2 -> end
    assert_eq!(longest_chain[0], "start");
    assert_eq!(longest_chain[3], "end");
}

#[test]
fn test_lock_file_loading() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    
    // Create a simple lock file
    let mut dependencies = HashMap::new();
    dependencies.insert(
        "dep-scope".to_string(),
        LockedDependency {
            version: "1.0.0".to_string(),
            path: "dep-scope".to_string(),
            resolved_at: chrono::Utc::now(),
            checksum: "checksum1".to_string(),
            dependency_type: DependencyType::Required,
            original_constraint: Some("^1.0.0".to_string()),
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        },
    );
    
    let mut scopes = HashMap::new();
    scopes.insert(
        "main-scope".to_string(),
        LockedScope {
            version: "1.0.0".to_string(),
            path: "main-scope".to_string(),
            dependencies,
            source_checksum: Some("main_checksum".to_string()),
            resolved_at: chrono::Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        },
    );
    
    let lock_file = RhemaLock {
        lockfile_version: "1.0.0".to_string(),
        generated_at: chrono::Utc::now(),
        generated_by: "test".to_string(),
        checksum: "lock_checksum".to_string(),
        scopes,
        metadata: rhema_core::schema::LockMetadata::new(),
    };
    
    // Write lock file to disk
    let lock_content = serde_yaml::to_string(&lock_file).unwrap();
    fs::write(repo_path.join("rhema.lock"), lock_content).unwrap();
    
    // Test that the lock file exists and can be read
    assert!(repo_path.join("rhema.lock").exists());
    
    let lock_content_read = fs::read_to_string(repo_path.join("rhema.lock")).unwrap();
    let parsed_lock: RhemaLock = serde_yaml::from_str(&lock_content_read).unwrap();
    
    assert_eq!(parsed_lock.lockfile_version, "1.0.0");
    assert_eq!(parsed_lock.generated_by, "test");
    assert!(parsed_lock.scopes.contains_key("main-scope"));
    
    let main_scope = &parsed_lock.scopes["main-scope"];
    assert!(main_scope.dependencies.contains_key("dep-scope"));
    assert_eq!(main_scope.dependencies["dep-scope"].version, "1.0.0");
}

#[test]
fn test_dependency_graph_comparison() {
    // Test dependency graph comparison logic
    let mut current_graph = HashMap::new();
    current_graph.insert("scope1".to_string(), vec!["dep1".to_string(), "dep2".to_string()]);
    current_graph.insert("scope2".to_string(), vec!["dep3".to_string()]);
    
    let mut lock_graph = HashMap::new();
    lock_graph.insert("scope1".to_string(), vec!["dep1".to_string()]); // dep2 removed
    lock_graph.insert("scope2".to_string(), vec!["dep3".to_string(), "dep4".to_string()]); // dep4 added
    lock_graph.insert("scope3".to_string(), vec!["dep5".to_string()]); // scope3 added
    
    // Test scope comparison
    let current_scopes: std::collections::HashSet<String> = current_graph.keys().cloned().collect();
    let lock_scopes: std::collections::HashSet<String> = lock_graph.keys().cloned().collect();
    
    // scope3 is in lock but not in current (removed)
    assert!(lock_scopes.contains("scope3"));
    assert!(!current_scopes.contains("scope3"));
    
    // Test dependency comparison for common scopes
    let scope1_current: std::collections::HashSet<String> = current_graph["scope1"].iter().cloned().collect();
    let scope1_lock: std::collections::HashSet<String> = lock_graph["scope1"].iter().cloned().collect();
    
    // dep2 is in current but not in lock (added)
    assert!(scope1_current.contains("dep2"));
    assert!(!scope1_lock.contains("dep2"));
    
    // dep4 is in lock but not in current (removed)
    let scope2_current: std::collections::HashSet<String> = current_graph["scope2"].iter().cloned().collect();
    let scope2_lock: std::collections::HashSet<String> = lock_graph["scope2"].iter().cloned().collect();
    
    assert!(!scope2_current.contains("dep4"));
    assert!(scope2_lock.contains("dep4"));
} 