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

use crate::{RhemaError, RhemaResult};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_conflict_resolution_cli_command() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let test_dir = temp_dir.path();
    
    // Create a simple lock file for testing
    let lock_file_content = r#"{
        "lockfile_version": "1.0.0",
        "generated_at": "2025-01-01T00:00:00Z",
        "generated_by": "test",
        "checksum": "test-checksum",
        "scopes": {
            "test-scope": {
                "version": "1.0.0",
                "path": "test-scope",
                "dependencies": {
                    "test-dep": {
                        "version": "1.2.3",
                        "path": "test-dep",
                        "resolved_at": "2025-01-01T00:00:00Z",
                        "checksum": "test-checksum",
                        "dependency_type": "Required",
                        "original_constraint": "1.2.3",
                        "is_transitive": false,
                        "dependencies": null,
                        "custom": {}
                    }
                },
                "source_checksum": null,
                "resolved_at": "2025-01-01T00:00:00Z",
                "has_circular_dependencies": false,
                "custom": {}
            }
        },
        "metadata": {
            "total_scopes": 1,
            "total_dependencies": 1,
            "circular_dependencies": 0,
            "validation_status": "Valid",
            "resolution_strategy": "Latest",
            "conflict_resolution": "Automatic",
            "last_validated": null,
            "validation_messages": null,
            "performance_metrics": null,
            "custom": {}
        }
    }"#;
    
    let lock_file_path = test_dir.join("rhema.lock");
    fs::write(&lock_file_path, lock_file_content)?;
    
    // Test that the lock file can be read
    let lock_content = fs::read_to_string(&lock_file_path)?;
    let lock_file: crate::schema::RhemaLock = serde_json::from_str(&lock_content)?;
    
    // Verify the lock file structure
    assert_eq!(lock_file.lockfile_version, "1.0.0");
    assert_eq!(lock_file.scopes.len(), 1);
    assert!(lock_file.scopes.contains_key("test-scope"));
    
    let test_scope = &lock_file.scopes["test-scope"];
    assert_eq!(test_scope.version, "1.0.0");
    assert_eq!(test_scope.path, "test-scope");
    assert_eq!(test_scope.dependencies.len(), 1);
    assert!(test_scope.dependencies.contains_key("test-dep"));
    
    let test_dep = &test_scope.dependencies["test-dep"];
    assert_eq!(test_dep.version, "1.2.3");
    assert_eq!(test_dep.path, "test-dep");
    
    // Test conflict resolution configuration
    let config = crate::lock::ConflictResolutionConfig {
        primary_strategy: crate::lock::ConflictResolutionStrategy::LatestCompatible,
        fallback_strategies: vec![
            crate::lock::ConflictResolutionStrategy::Conservative,
            crate::lock::ConflictResolutionStrategy::ManualResolution,
        ],
        enable_auto_detection: true,
        track_history: true,
        max_attempts: 10,
        allow_user_prompts: false,
        prefer_stable: true,
        strict_pinning: false,
        compatibility_threshold: 0.8,
        parallel_resolution: false,
        max_parallel_threads: 4,
        timeout_seconds: 300,
    };
    
    // Test that the configuration is valid
    assert_eq!(config.primary_strategy, crate::lock::ConflictResolutionStrategy::LatestCompatible);
    assert_eq!(config.fallback_strategies.len(), 2);
    assert!(config.enable_auto_detection);
    assert!(config.track_history);
    
    // Test dependency extraction (simulating what the CLI command would do)
    let dependencies: Vec<crate::lock::DependencySpec> = lock_file
        .scopes
        .iter()
        .flat_map(|(scope_path, scope)| {
            scope.dependencies.iter().map(|(dep_name, dep)| {
                // Convert version string to VersionConstraint
                let version_constraint = if let Ok(ver) = semver::Version::parse(&dep.version) {
                    crate::lock::conflict_resolver::VersionConstraint::Exact(ver)
                } else {
                    crate::lock::conflict_resolver::VersionConstraint::Latest
                };

                crate::lock::DependencySpec {
                    path: dep_name.clone(),
                    version_constraint,
                    dependency_type: dep.dependency_type.clone(),
                    is_transitive: dep.is_transitive,
                    original_constraint: dep.original_constraint.clone(),
                    scope_path: scope_path.clone(),
                    priority: 5, // Default priority
                    optional: matches!(dep.dependency_type, crate::schema::DependencyType::Optional),
                    alternatives: Vec::new(),
                    metadata: std::collections::HashMap::new(),
                }
            })
        })
        .collect();
    
    // Verify dependency extraction
    assert_eq!(dependencies.len(), 1);
    let dep = &dependencies[0];
    assert_eq!(dep.path, "test-dep");
    assert_eq!(dep.scope_path, "test-scope");
    assert!(!dep.optional);
    
    // Test that the conflict resolver can be created
    let mut resolver = crate::lock::ConflictResolver::with_config(config);
    assert_eq!(resolver.config.primary_strategy, crate::lock::ConflictResolutionStrategy::LatestCompatible);
    
    // Test that the resolver can process the dependencies (should find no conflicts)
    let result = resolver.resolve_conflicts(&dependencies, test_dir)?;
    
    // Verify the result
    assert!(result.successful);
    assert_eq!(result.stats.total_conflicts, 0);
    assert_eq!(result.stats.auto_resolved, 0);
    assert_eq!(result.stats.manual_resolution_required, 0);
    assert_eq!(result.stats.unresolved_conflicts, 0);
    assert!(result.detected_conflicts.is_empty());
    
    Ok(())
}

#[test]
fn test_conflict_resolution_strategies() -> RhemaResult<()> {
    // Test all available conflict resolution strategies
    let strategies = vec![
        crate::lock::ConflictResolutionStrategy::LatestCompatible,
        crate::lock::ConflictResolutionStrategy::PinnedVersion,
        crate::lock::ConflictResolutionStrategy::ManualResolution,
        crate::lock::ConflictResolutionStrategy::AutomaticDetection,
        crate::lock::ConflictResolutionStrategy::HistoryTracking,
        crate::lock::ConflictResolutionStrategy::SmartSelection,
        crate::lock::ConflictResolutionStrategy::Conservative,
        crate::lock::ConflictResolutionStrategy::Aggressive,
        crate::lock::ConflictResolutionStrategy::Hybrid,
    ];
    
    for strategy in strategies {
        let config = crate::lock::ConflictResolutionConfig {
            primary_strategy: strategy.clone(),
            fallback_strategies: vec![],
            enable_auto_detection: true,
            track_history: true,
            max_attempts: 10,
            allow_user_prompts: false,
            prefer_stable: true,
            strict_pinning: false,
            compatibility_threshold: 0.8,
            parallel_resolution: false,
            max_parallel_threads: 4,
            timeout_seconds: 300,
        };
        
        let resolver = crate::lock::ConflictResolver::with_config(config);
        assert_eq!(resolver.config.primary_strategy, strategy);
    }
    
    Ok(())
}

#[test]
fn test_conflict_resolution_config_defaults() -> RhemaResult<()> {
    // Test default configuration
    let config = crate::lock::ConflictResolutionConfig::default();
    
    assert_eq!(config.primary_strategy, crate::lock::ConflictResolutionStrategy::LatestCompatible);
    assert_eq!(config.fallback_strategies.len(), 2);
    assert!(config.enable_auto_detection);
    assert!(config.track_history);
    assert_eq!(config.max_attempts, 10);
    assert!(!config.allow_user_prompts);
    assert!(!config.prefer_stable);
    assert!(!config.strict_pinning);
    assert_eq!(config.compatibility_threshold, 0.8);
    assert!(!config.parallel_resolution);
    assert_eq!(config.max_parallel_threads, 4);
    assert_eq!(config.timeout_seconds, 300);
    
    Ok(())
} 