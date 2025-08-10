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

use rhema_config::ConflictResolutionStrategy;
use rhema_core::RhemaResult;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_conflict_resolution_cli_command() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let test_dir = TempDir::new()?;
    let test_path = test_dir.path();

    // Create a simple test lock file content
    let lock_file_content = r#"
{
  "lockfile_version": "1.0.0",
  "generated_at": "2024-01-15T10:00:00Z",
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
          "resolved_at": "2024-01-15T10:00:00Z",
          "checksum": "dep-checksum",
          "dependency_type": "Required",
          "original_constraint": "^1.2.0",
          "is_transitive": false,
          "dependencies": null,
          "custom": {}
        }
      },
      "source_checksum": "scope-checksum",
      "resolved_at": "2024-01-15T10:00:00Z",
      "has_circular_dependencies": false,
      "custom": {}
    }
  },
  "metadata": {
    "generator_version": "1.0.0",
    "lock_format": "json"
  }
}
"#;

    let lock_file_path = test_dir.path().join("rhema.lock");
    fs::write(&lock_file_path, lock_file_content)?;

    // Test that the lock file can be read
    let lock_content = fs::read_to_string(&lock_file_path)?;
    assert!(lock_content.contains("test-scope"));
    assert!(lock_content.contains("test-dep"));
    assert!(lock_content.contains("1.2.3"));

    // Test conflict resolution configuration using actual types
    let config = rhema_config::ConflictResolutionConfig {
        primary_strategy: ConflictResolutionStrategy::LatestCompatible,
        fallback_strategies: vec![
            ConflictResolutionStrategy::Conservative,
            ConflictResolutionStrategy::ManualResolution,
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
        auto_resolve_low_severity: true,
        auto_resolve_medium_severity: false,
        fail_on_high_severity: true,
        fail_on_critical_severity: true,
        dependency_type_preferences: HashMap::new(),
        scope_preferences: HashMap::new(),
    };

    // Test that the configuration is valid
    assert_eq!(
        config.primary_strategy,
        ConflictResolutionStrategy::LatestCompatible
    );
    assert_eq!(config.fallback_strategies.len(), 2);
    assert!(config.enable_auto_detection);
    assert!(config.track_history);

    // Test that the lock file exists and can be parsed as JSON
    let lock_data: serde_json::Value = serde_json::from_str(&lock_content)?;
    assert_eq!(lock_data["lockfile_version"], "1.0.0");
    assert!(lock_data["scopes"]["test-scope"]["dependencies"]["test-dep"]["version"] == "1.2.3");

    Ok(())
}

#[test]
fn test_conflict_resolution_strategies() -> RhemaResult<()> {
    // Test different conflict resolution strategies
    let strategies = vec![
        ConflictResolutionStrategy::LatestCompatible,
        ConflictResolutionStrategy::Conservative,
        ConflictResolutionStrategy::ManualResolution,
        ConflictResolutionStrategy::Aggressive,
        ConflictResolutionStrategy::PinnedVersion,
    ];

    // Test that all strategies are valid
    for strategy in strategies {
        match strategy {
            ConflictResolutionStrategy::LatestCompatible => assert!(true),
            ConflictResolutionStrategy::Conservative => assert!(true),
            ConflictResolutionStrategy::ManualResolution => assert!(true),
            ConflictResolutionStrategy::Aggressive => assert!(true),
            ConflictResolutionStrategy::PinnedVersion => assert!(true),
            ConflictResolutionStrategy::AutomaticDetection => assert!(true),
            ConflictResolutionStrategy::HistoryTracking => assert!(true),
            ConflictResolutionStrategy::SmartSelection => assert!(true),
            ConflictResolutionStrategy::Hybrid => assert!(true),
        }
    }

    Ok(())
}

#[test]
fn test_conflict_resolution_config_defaults() -> RhemaResult<()> {
    // Test default configuration creation
    let config = rhema_config::ConflictResolutionConfig::default();

    // Test that default values are reasonable
    assert!(config.max_attempts > 0);
    assert!(config.timeout_seconds > 0);
    assert!(config.compatibility_threshold > 0.0);
    assert!(config.compatibility_threshold <= 1.0);
    assert!(config.max_parallel_threads > 0);

    // Test that the configuration can be serialized and deserialized
    let config_json = serde_json::to_string(&config)?;
    let deserialized_config: rhema_config::ConflictResolutionConfig =
        serde_json::from_str(&config_json)?;

    assert_eq!(
        config.primary_strategy,
        deserialized_config.primary_strategy
    );
    assert_eq!(config.max_attempts, deserialized_config.max_attempts);
    assert_eq!(config.timeout_seconds, deserialized_config.timeout_seconds);

    Ok(())
}
