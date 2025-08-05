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

use super::conflict_resolver::*;
use crate::schema::DependencyType;
use semver::Version;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn test_conflict_resolver_basic_functionality() {
    // Test basic creation and configuration
    let resolver = ConflictResolver::new();
    assert_eq!(resolver.config.primary_strategy, ConflictResolutionStrategy::LatestCompatible);
    assert!(resolver.config.enable_auto_detection);
    assert!(resolver.config.track_history);
}

#[test]
fn test_conflict_resolver_with_custom_config() {
    let config = ConflictResolutionConfig {
        primary_strategy: ConflictResolutionStrategy::PinnedVersion,
        fallback_strategies: vec![ConflictResolutionStrategy::ManualResolution],
        enable_auto_detection: false,
        track_history: false,
        max_attempts: 5,
        allow_user_prompts: false,
        prefer_stable: false,
        strict_pinning: true,
        compatibility_threshold: 0.9,
        parallel_resolution: false,
        max_parallel_threads: 1,
        timeout_seconds: 60,
    };

    let resolver = ConflictResolver::with_config(config);
    assert_eq!(resolver.config.primary_strategy, ConflictResolutionStrategy::PinnedVersion);
    assert!(!resolver.config.enable_auto_detection);
    assert!(!resolver.config.track_history);
    assert_eq!(resolver.config.max_attempts, 5);
}

#[test]
fn test_version_constraint_compatibility() {
    let resolver = ConflictResolver::new();
    
    // Test exact version compatibility
    let exact1 = VersionConstraint::Exact(Version::parse("1.2.3").unwrap());
    let exact2 = VersionConstraint::Exact(Version::parse("1.2.3").unwrap());
    let exact3 = VersionConstraint::Exact(Version::parse("1.2.4").unwrap());
    
    assert!(resolver.constraints_are_compatible(&exact1, &exact2));
    assert!(!resolver.constraints_are_compatible(&exact1, &exact3));
}

#[test]
fn test_version_satisfies_constraint() {
    let resolver = ConflictResolver::new();
    let version = Version::parse("1.2.3").unwrap();
    
    // Test exact constraint
    let exact = VersionConstraint::Exact(Version::parse("1.2.3").unwrap());
    assert!(resolver.version_satisfies_constraint(&version, &exact));
    
    let exact_different = VersionConstraint::Exact(Version::parse("1.2.4").unwrap());
    assert!(!resolver.version_satisfies_constraint(&version, &exact_different));
    
    // Test pinned constraint
    let pinned = VersionConstraint::Pinned(Version::parse("1.2.3").unwrap());
    assert!(resolver.version_satisfies_constraint(&version, &pinned));
    
    let pinned_different = VersionConstraint::Pinned(Version::parse("1.2.4").unwrap());
    assert!(!resolver.version_satisfies_constraint(&version, &pinned_different));
}

#[test]
fn test_conflict_detection() {
    let mut resolver = ConflictResolver::new();
    let repo_path = PathBuf::from("/tmp/test");
    
    // Create conflicting dependencies
    let dep1 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: "1.2.3".to_string(),
        scope_path: "scope1".to_string(),
        priority: 8,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dep2 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Exact(Version::parse("1.2.4").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: "1.2.4".to_string(),
        scope_path: "scope2".to_string(),
        priority: 7,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dependencies = vec![dep1, dep2];
    let conflicts = resolver.detect_conflicts(&dependencies, &repo_path).unwrap();
    
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].dependency_name, "test-dep");
    assert_eq!(conflicts[0].conflict_type, ConflictType::VersionIncompatibility);
    assert_eq!(conflicts[0].severity, ConflictSeverity::High);
}

#[test]
fn test_no_conflict_detection() {
    let mut resolver = ConflictResolver::new();
    let repo_path = PathBuf::from("/tmp/test");
    
    // Create non-conflicting dependencies
    let dep1 = DependencySpec {
        path: "test-dep1".to_string(),
        version_constraint: VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: "1.2.3".to_string(),
        scope_path: "scope1".to_string(),
        priority: 8,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dep2 = DependencySpec {
        path: "test-dep2".to_string(),
        version_constraint: VersionConstraint::Exact(Version::parse("1.2.4").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: "1.2.4".to_string(),
        scope_path: "scope2".to_string(),
        priority: 7,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dependencies = vec![dep1, dep2];
    let conflicts = resolver.detect_conflicts(&dependencies, &repo_path).unwrap();
    
    assert_eq!(conflicts.len(), 0);
}

#[test]
fn test_latest_compatible_resolution() {
    let mut resolver = ConflictResolver::new();
    let repo_path = PathBuf::from("/tmp/test");
    
    // Create conflicting dependencies
    let dep1 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Range(semver::VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: ">=1.0.0,<2.0.0".to_string(),
        scope_path: "scope1".to_string(),
        priority: 8,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dep2 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Range(semver::VersionReq::parse(">=1.5.0,<3.0.0").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: ">=1.5.0,<3.0.0".to_string(),
        scope_path: "scope2".to_string(),
        priority: 7,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dependencies = vec![dep1, dep2];
    let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
    
    assert!(result.successful);
    assert_eq!(result.stats.auto_resolved, 1);
    assert_eq!(result.stats.version_upgrades, 1);
    assert!(!result.resolved_dependencies.is_empty());
}

#[test]
fn test_pinned_version_resolution() {
    let mut resolver = ConflictResolver::with_config(ConflictResolutionConfig {
        primary_strategy: ConflictResolutionStrategy::PinnedVersion,
        ..Default::default()
    });
    
    let repo_path = PathBuf::from("/tmp/test");
    
    // Create dependencies with pinned version
    let dep1 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Pinned(Version::parse("1.2.3").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: "1.2.3".to_string(),
        scope_path: "scope1".to_string(),
        priority: 8,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dep2 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Range(semver::VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: ">=1.0.0,<2.0.0".to_string(),
        scope_path: "scope2".to_string(),
        priority: 7,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dependencies = vec![dep1, dep2];
    let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
    
    assert!(result.successful);
    assert_eq!(result.stats.pinned_versions_enforced, 1);
}

#[test]
fn test_manual_resolution_workflow() {
    let mut resolver = ConflictResolver::with_config(ConflictResolutionConfig {
        primary_strategy: ConflictResolutionStrategy::ManualResolution,
        ..Default::default()
    });
    
    let repo_path = PathBuf::from("/tmp/test");
    
    // Create conflicting dependencies
    let dep1 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: "1.2.3".to_string(),
        scope_path: "scope1".to_string(),
        priority: 8,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dep2 = DependencySpec {
        path: "test-dep".to_string(),
        version_constraint: VersionConstraint::Exact(Version::parse("1.2.4").unwrap()),
        dependency_type: DependencyType::Required,
        is_transitive: false,
        original_constraint: "1.2.4".to_string(),
        scope_path: "scope2".to_string(),
        priority: 7,
        optional: false,
        alternatives: Vec::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let dependencies = vec![dep1, dep2];
    let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
    
    assert!(!result.successful);
    assert_eq!(result.stats.manual_resolution_required, 1);
    assert!(!result.warnings.is_empty());
}

#[test]
fn test_history_tracking() {
    let mut resolver = ConflictResolver::new();
    
    // Add a historical resolution
    let history_entry = ResolutionHistoryEntry {
        timestamp: chrono::Utc::now(),
        dependency_name: "test-dep".to_string(),
        conflict_type: ConflictType::VersionIncompatibility,
        strategy: ConflictResolutionStrategy::LatestCompatible,
        previous_version: Some(Version::parse("1.2.3").unwrap()),
        new_version: Version::parse("1.2.4").unwrap(),
        successful: true,
        notes: Some("Previous successful resolution".to_string()),
        resolved_by: Some("test-user".to_string()),
    };
    
    resolver.add_resolution_to_history(history_entry);
    
    assert_eq!(resolver.get_resolution_history().len(), 1);
    
    // Test finding historical resolution
    let historical = resolver.find_historical_resolution("test-dep");
    assert!(historical.is_some());
    assert_eq!(historical.unwrap().new_version, Version::parse("1.2.4").unwrap());
}

#[test]
fn test_conflict_severity_calculation() {
    let resolver = ConflictResolver::new();
    
    // Test high priority required dependency
    let high_priority_req = ConflictRequirement {
        scope_path: "scope1".to_string(),
        constraint: VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
        dependency_type: DependencyType::Required,
        priority: 9,
        optional: false,
        original_constraint: "1.2.3".to_string(),
    };
    
    let requirements = vec![high_priority_req];
    let severity = resolver.calculate_conflict_severity(&requirements);
    assert_eq!(severity, ConflictSeverity::High);
    
    // Test critical (required + high priority)
    let critical_req = ConflictRequirement {
        scope_path: "scope1".to_string(),
        constraint: VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
        dependency_type: DependencyType::Required,
        priority: 10,
        optional: false,
        original_constraint: "1.2.3".to_string(),
    };
    
    let requirements = vec![critical_req];
    let severity = resolver.calculate_conflict_severity(&requirements);
    assert_eq!(severity, ConflictSeverity::Critical);
}

#[test]
fn test_cache_operations() {
    let mut resolver = ConflictResolver::new();
    
    // Test cache clearing
    resolver.clear_caches();
    
    // Test history operations
    resolver.clear_resolution_history();
    assert_eq!(resolver.get_resolution_history().len(), 0);
    
    // Add some history
    let history_entry = ResolutionHistoryEntry {
        timestamp: chrono::Utc::now(),
        dependency_name: "test-dep".to_string(),
        conflict_type: ConflictType::VersionIncompatibility,
        strategy: ConflictResolutionStrategy::LatestCompatible,
        previous_version: None,
        new_version: Version::parse("1.2.3").unwrap(),
        successful: true,
        notes: None,
        resolved_by: None,
    };
    
    resolver.add_resolution_to_history(history_entry);
    assert_eq!(resolver.get_resolution_history().len(), 1);
}

#[test]
fn test_conflict_report_generation() {
    let resolver = ConflictResolver::new();
    let repo_path = PathBuf::from("/tmp/test");
    
    let conflict = DependencyConflict {
        dependency_name: "test-dep".to_string(),
        requirements: vec![
            ConflictRequirement {
                scope_path: "scope1".to_string(),
                constraint: VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
                dependency_type: DependencyType::Required,
                priority: 8,
                optional: false,
                original_constraint: "1.2.3".to_string(),
            }
        ],
        conflict_type: ConflictType::VersionIncompatibility,
        suggested_resolution: Some(Version::parse("1.2.4").unwrap()),
        severity: ConflictSeverity::High,
        description: "Test conflict".to_string(),
        affected_scopes: vec!["scope1".to_string()],
        recommendations: vec!["Test recommendation".to_string()],
        compatibility_scores: std::collections::HashMap::new(),
        auto_resolved: false,
        resolved_at: None,
        resolution_method: None,
    };
    
    let dependencies = vec![];
    let report = resolver.generate_conflict_report(&conflict, &dependencies, &repo_path).unwrap();
    
    assert!(report.contains("test-dep"));
    assert!(report.contains("VersionIncompatibility"));
    assert!(report.contains("High"));
    assert!(report.contains("Test conflict"));
} 

#[test]
fn test_cli_command_configuration() {
    // Test CLI command configuration parsing
    let config = ConflictResolutionConfig {
        primary_strategy: ConflictResolutionStrategy::SmartSelection,
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
        parallel_resolution: true,
        max_parallel_threads: 8,
        timeout_seconds: 300,
    };

    assert_eq!(config.primary_strategy, ConflictResolutionStrategy::SmartSelection);
    assert_eq!(config.fallback_strategies.len(), 2);
    assert!(config.enable_auto_detection);
    assert!(config.track_history);
    assert!(config.prefer_stable);
    assert!(!config.strict_pinning);
    assert_eq!(config.compatibility_threshold, 0.8);
    assert!(config.parallel_resolution);
    assert_eq!(config.max_parallel_threads, 8);
    assert_eq!(config.timeout_seconds, 300);
}

#[test]
fn test_strategy_parsing() {
    // Test strategy string parsing (simulating CLI argument parsing)
    let strategy_strings = vec![
        "latest_compatible",
        "pinned_version", 
        "manual_resolution",
        "automatic_detection",
        "history_tracking",
        "smart_selection",
        "conservative",
        "aggressive",
        "hybrid",
    ];

    let expected_strategies = vec![
        ConflictResolutionStrategy::LatestCompatible,
        ConflictResolutionStrategy::PinnedVersion,
        ConflictResolutionStrategy::ManualResolution,
        ConflictResolutionStrategy::AutomaticDetection,
        ConflictResolutionStrategy::HistoryTracking,
        ConflictResolutionStrategy::SmartSelection,
        ConflictResolutionStrategy::Conservative,
        ConflictResolutionStrategy::Aggressive,
        ConflictResolutionStrategy::Hybrid,
    ];

    for (i, strategy_str) in strategy_strings.iter().enumerate() {
        let parsed_strategy = match *strategy_str {
            "latest_compatible" => ConflictResolutionStrategy::LatestCompatible,
            "pinned_version" => ConflictResolutionStrategy::PinnedVersion,
            "manual_resolution" => ConflictResolutionStrategy::ManualResolution,
            "automatic_detection" => ConflictResolutionStrategy::AutomaticDetection,
            "history_tracking" => ConflictResolutionStrategy::HistoryTracking,
            "smart_selection" => ConflictResolutionStrategy::SmartSelection,
            "conservative" => ConflictResolutionStrategy::Conservative,
            "aggressive" => ConflictResolutionStrategy::Aggressive,
            "hybrid" => ConflictResolutionStrategy::Hybrid,
            _ => panic!("Unknown strategy: {}", strategy_str),
        };

        assert_eq!(parsed_strategy, expected_strategies[i]);
    }
}

#[test]
fn test_fallback_strategies_parsing() {
    // Test fallback strategies parsing (simulating CLI comma-separated argument)
    let fallback_str = "conservative,manual_resolution,smart_selection";
    
    let parsed_strategies: Vec<ConflictResolutionStrategy> = fallback_str
        .split(',')
        .filter_map(|s| {
            match s.trim() {
                "latest_compatible" => Some(ConflictResolutionStrategy::LatestCompatible),
                "pinned_version" => Some(ConflictResolutionStrategy::PinnedVersion),
                "manual_resolution" => Some(ConflictResolutionStrategy::ManualResolution),
                "automatic_detection" => Some(ConflictResolutionStrategy::AutomaticDetection),
                "history_tracking" => Some(ConflictResolutionStrategy::HistoryTracking),
                "smart_selection" => Some(ConflictResolutionStrategy::SmartSelection),
                "conservative" => Some(ConflictResolutionStrategy::Conservative),
                "aggressive" => Some(ConflictResolutionStrategy::Aggressive),
                "hybrid" => Some(ConflictResolutionStrategy::Hybrid),
                _ => None,
            }
        })
        .collect();

    assert_eq!(parsed_strategies.len(), 3);
    assert_eq!(parsed_strategies[0], ConflictResolutionStrategy::Conservative);
    assert_eq!(parsed_strategies[1], ConflictResolutionStrategy::ManualResolution);
    assert_eq!(parsed_strategies[2], ConflictResolutionStrategy::SmartSelection);
}

#[test]
fn test_cli_output_formats() {
    // Test CLI output format generation
    let result = ConflictResolutionResult {
        resolved_dependencies: HashMap::new(),
        detected_conflicts: vec![],
        resolution_actions: vec![],
        stats: ConflictResolutionStats {
            total_conflicts: 0,
            auto_resolved: 0,
            manual_resolution_required: 0,
            unresolved_conflicts: 0,
            resolution_attempts: 1,
            average_resolution_time_ms: 10.0,
            total_resolution_time_ms: 10,
            version_upgrades: 0,
            version_downgrades: 0,
            pinned_versions_enforced: 0,
            compatibility_checks: 0,
            cache_hit_rate: 1.0,
        },
        successful: true,
        warnings: vec![],
        recommendations: vec!["All conflicts resolved successfully".to_string()],
        performance_metrics: ConflictPerformanceMetrics {
            total_time_ms: 10,
            detection_time_ms: 5,
            strategy_execution_time_ms: 3,
            compatibility_scoring_time_ms: 2,
            user_interaction_time_ms: 0,
            memory_usage_bytes: 1024,
            parallel_operations: 0,
            cache_operations: 1,
        },
    };

    // Test text format generation
    let text_report = format!(
        "Conflict Resolution Report\n========================\n\nStatus: {}\nTotal conflicts: {}\nAuto-resolved: {}\nManual resolution required: {}\nUnresolved: {}\nTotal time: {}ms\n",
        if result.successful { "✅ Success" } else { "❌ Failed" },
        result.stats.total_conflicts,
        result.stats.auto_resolved,
        result.stats.manual_resolution_required,
        result.stats.unresolved_conflicts,
        result.performance_metrics.total_time_ms
    );

    assert!(text_report.contains("✅ Success"));
    assert!(text_report.contains("Total conflicts: 0"));
    assert!(text_report.contains("Total time: 10ms"));

    // Test JSON format generation
    let json_report = serde_json::to_string_pretty(&serde_json::json!({
        "status": if result.successful { "success" } else { "failed" },
        "stats": {
            "total_conflicts": result.stats.total_conflicts,
            "auto_resolved": result.stats.auto_resolved,
            "manual_resolution_required": result.stats.manual_resolution_required,
            "unresolved_conflicts": result.stats.unresolved_conflicts,
            "total_time_ms": result.performance_metrics.total_time_ms
        },
        "recommendations": result.recommendations,
        "warnings": result.warnings
    })).unwrap();

    assert!(json_report.contains("\"status\": \"success\""));
    assert!(json_report.contains("\"total_conflicts\": 0"));
    assert!(json_report.contains("\"recommendations\""));
} 