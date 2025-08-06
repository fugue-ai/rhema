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

use rhema::lock::resolver::{
    ConflictResolutionMethod, ConflictType, DependencyResolver,
    DependencySpec, ResolutionConfig, VersionConstraint,
};
use rhema::schema::{DependencyType, ResolutionStrategy};
use semver::{Version, VersionReq};
use std::collections::HashMap;

#[test]
fn test_comprehensive_version_constraint_parsing() {
    // Test exact versions
    assert!(matches!(
        DependencyResolver::parse_version_constraint("=1.2.3").unwrap(),
        VersionConstraint::Exact(_)
    ));
    assert!(matches!(
        DependencyResolver::parse_version_constraint("1.2.3").unwrap(),
        VersionConstraint::Exact(_)
    ));

    // Test ranges
    assert!(matches!(
        DependencyResolver::parse_version_constraint(">=1.2.0,<2.0.0").unwrap(),
        VersionConstraint::Range(_)
    ));
    assert!(matches!(
        DependencyResolver::parse_version_constraint("^1.2.0").unwrap(),
        VersionConstraint::Range(_)
    ));
    assert!(matches!(
        DependencyResolver::parse_version_constraint("~1.2.0").unwrap(),
        VersionConstraint::Range(_)
    ));

    // Test special keywords
    assert!(matches!(
        DependencyResolver::parse_version_constraint("latest").unwrap(),
        VersionConstraint::Latest
    ));
    assert!(matches!(
        DependencyResolver::parse_version_constraint("earliest").unwrap(),
        VersionConstraint::Earliest
    ));

    // Test invalid constraints
    assert!(DependencyResolver::parse_version_constraint("invalid").is_err());
    assert!(DependencyResolver::parse_version_constraint("=invalid").is_err());
    assert!(DependencyResolver::parse_version_constraint(">=invalid").is_err());
}

#[test]
fn test_all_resolution_strategies() {
    let versions = vec![
        Version::parse("1.0.0").unwrap(),
        Version::parse("1.1.0").unwrap(),
        Version::parse("1.2.0").unwrap(),
        Version::parse("2.0.0").unwrap(),
        Version::parse("2.1.0").unwrap(),
    ];

    let constraint = VersionConstraint::Range(
        VersionReq::parse(">=1.0.0,<3.0.0").unwrap()
    );

    // Test Latest strategy
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Latest);
    let resolved = resolver.resolve_version(&constraint, &versions).unwrap();
    assert_eq!(resolved, Version::parse("2.1.0").unwrap());

    // Test Earliest strategy
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Earliest);
    let resolved = resolver.resolve_version(&constraint, &versions).unwrap();
    assert_eq!(resolved, Version::parse("1.0.0").unwrap());

    // Test Range strategy (should behave like Latest)
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Range);
    let resolved = resolver.resolve_version(&constraint, &versions).unwrap();
    assert_eq!(resolved, Version::parse("2.1.0").unwrap());

    // Test Compatible strategy
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Compatible);
    let resolved = resolver.resolve_version(&constraint, &versions).unwrap();
    assert_eq!(resolved, Version::parse("2.1.0").unwrap());

    // Test Pinned strategy
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Pinned);
    let pinned_constraint = VersionConstraint::Pinned(Version::parse("1.2.0").unwrap());
    let resolved = resolver.resolve_version(&pinned_constraint, &versions).unwrap();
    assert_eq!(resolved, Version::parse("1.2.0").unwrap());
}

#[test]
fn test_version_filtering_edge_cases() {
    let versions = vec![
        Version::parse("1.0.0").unwrap(),
        Version::parse("1.0.0-alpha").unwrap(),
        Version::parse("1.0.0-beta").unwrap(),
        Version::parse("1.0.0-rc.1").unwrap(),
        Version::parse("1.0.0-dev").unwrap(),
        Version::parse("1.0.0+build").unwrap(),
    ];

    // Test with prereleases disabled
    let mut resolver = DependencyResolver::with_config(ResolutionConfig {
        strategy: ResolutionStrategy::Latest,
        fallback_strategy: None,
        allow_prereleases: false,
        allow_dev_versions: false,
        max_attempts: 3,
        enable_caching: true,
        conflict_resolution: ConflictResolutionMethod::Automatic,
    });

    let filtered = resolver.filter_versions(&versions).unwrap();
    assert_eq!(filtered.len(), 2); // Only 1.0.0 and 1.0.0+build
    assert!(filtered.contains(&Version::parse("1.0.0").unwrap()));

    // Test with prereleases enabled
    let mut resolver = DependencyResolver::with_config(ResolutionConfig {
        strategy: ResolutionStrategy::Latest,
        fallback_strategy: None,
        allow_prereleases: true,
        allow_dev_versions: true,
        max_attempts: 3,
        enable_caching: true,
        conflict_resolution: ConflictResolutionMethod::Automatic,
    });

    let filtered = resolver.filter_versions(&versions).unwrap();
    assert_eq!(filtered.len(), 6); // All versions included
}

#[test]
fn test_circular_dependency_detection_complex() {
    let mut graph = HashMap::new();
    
    // Simple cycle: A -> B -> C -> A
    graph.insert("A".to_string(), vec!["B".to_string()]);
    graph.insert("B".to_string(), vec!["C".to_string()]);
    graph.insert("C".to_string(), vec!["A".to_string()]);
    graph.insert("D".to_string(), vec!["A".to_string()]); // D depends on A but not part of cycle

    let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
    let cycles = resolver.detect_circular_dependencies(&graph).unwrap();
    
    assert!(!cycles.is_empty());
    assert!(cycles.len() >= 1);

    // Test with no cycles
    let mut graph = HashMap::new();
    graph.insert("A".to_string(), vec!["B".to_string()]);
    graph.insert("B".to_string(), vec!["C".to_string()]);
    graph.insert("C".to_string(), vec![]);

    let cycles = resolver.detect_circular_dependencies(&graph).unwrap();
    assert!(cycles.is_empty());

    // Test self-cycle
    let mut graph = HashMap::new();
    graph.insert("A".to_string(), vec!["A".to_string()]);

    let cycles = resolver.detect_circular_dependencies(&graph).unwrap();
    assert!(!cycles.is_empty());
}

#[test]
fn test_conflict_detection_comprehensive() {
    let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
    
    // Test no conflicts
    let deps = vec![
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Exact(Version::parse("1.0.0").unwrap()),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "=1.0.0".to_string(),
        },
    ];

    let conflicts = resolver.detect_conflicts(&deps);
    assert_eq!(conflicts.len(), 0);

    // Test version incompatibility
    let deps = vec![
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Exact(Version::parse("1.0.0").unwrap()),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "=1.0.0".to_string(),
        },
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Exact(Version::parse("2.0.0").unwrap()),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "=2.0.0".to_string(),
        },
    ];

    let conflicts = resolver.detect_conflicts(&deps);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].dependency_name, "crates/rhema-core");
    assert_eq!(conflicts[0].conflict_type, ConflictType::VersionIncompatibility);

    // Test range conflicts
    let deps = vec![
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Range(
                VersionReq::parse(">=1.0.0,<2.0.0").unwrap()
            ),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: ">=1.0.0,<2.0.0".to_string(),
        },
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Range(
                VersionReq::parse(">=2.0.0,<3.0.0").unwrap()
            ),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: ">=2.0.0,<3.0.0".to_string(),
        },
    ];

    let conflicts = resolver.detect_conflicts(&deps);
    // This should detect a conflict since the ranges don't overlap
    assert_eq!(conflicts.len(), 1);
}

#[test]
fn test_fallback_strategy() {
    let versions = vec![
        Version::parse("1.0.0").unwrap(),
        Version::parse("2.0.0").unwrap(),
    ];

    let constraint = VersionConstraint::Range(
        VersionReq::parse(">=3.0.0").unwrap() // No matching versions
    );

    // Test with fallback strategy
    let mut resolver = DependencyResolver::with_config(ResolutionConfig {
        strategy: ResolutionStrategy::Latest,
        fallback_strategy: Some(ResolutionStrategy::Earliest),
        allow_prereleases: false,
        allow_dev_versions: false,
        max_attempts: 3,
        enable_caching: true,
        conflict_resolution: ConflictResolutionMethod::Automatic,
    });

    // Should fail even with fallback since no versions match the constraint
    let result = resolver.resolve_version(&constraint, &versions);
    assert!(result.is_err());
}

#[test]
fn test_caching_behavior() {
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Latest);
    
    // Initially cache should be empty
    assert_eq!(resolver.cache_size(), 0);
    
    // Clear cache
    resolver.clear_cache();
    assert_eq!(resolver.cache_size(), 0);
}

#[test]
fn test_error_handling_edge_cases() {
    let resolver = DependencyResolver::new(ResolutionStrategy::Latest);

    // Test empty versions list
    let constraint = VersionConstraint::Range(
        VersionReq::parse(">=1.0.0").unwrap()
    );
    let result = resolver.resolve_version(&constraint, &[]);
    assert!(result.is_err());

    // Test invalid version constraint parsing
    assert!(DependencyResolver::parse_version_constraint("").is_err());
    assert!(DependencyResolver::parse_version_constraint("=invalid").is_err());
    assert!(DependencyResolver::parse_version_constraint(">=invalid").is_err());

    // Test pinned strategy with non-pinned constraint
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Pinned);
    let constraint = VersionConstraint::Range(
        VersionReq::parse(">=1.0.0").unwrap()
    );
    let versions = vec![Version::parse("1.0.0").unwrap()];
    let result = resolver.resolve_version(&constraint, &versions);
    assert!(result.is_err());
}

#[test]
fn test_resolution_statistics() {
    let mut resolver = DependencyResolver::new(ResolutionStrategy::Latest);
    
    let stats = resolver.get_stats();
    assert_eq!(stats.total_dependencies, 0);
    assert_eq!(stats.resolved_dependencies, 0);
    assert_eq!(stats.failed_resolutions, 0);
    assert_eq!(stats.conflicts_detected, 0);
}

#[test]
fn test_complex_dependency_scenarios() {
    let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
    
    // Test multiple dependencies with different types
    let deps = vec![
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Range(
                VersionReq::parse(">=1.0.0,<2.0.0").unwrap()
            ),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: ">=1.0.0,<2.0.0".to_string(),
        },
        DependencySpec {
            path: "crates/rhema-ai".to_string(),
            version_constraint: VersionConstraint::Exact(Version::parse("2.0.0").unwrap()),
            dependency_type: DependencyType::Optional,
            is_transitive: false,
            original_constraint: "=2.0.0".to_string(),
        },
        DependencySpec {
            path: "crates/rhema-config".to_string(),
            version_constraint: VersionConstraint::Latest,
            dependency_type: DependencyType::Development,
            is_transitive: true,
            original_constraint: "latest".to_string(),
        },
    ];

    // Test conflict detection
    let conflicts = resolver.detect_conflicts(&deps);
    assert_eq!(conflicts.len(), 0); // No conflicts in this scenario
}

#[test]
fn test_conflict_resolution_methods() {
    let resolver = DependencyResolver::with_config(ResolutionConfig {
        strategy: ResolutionStrategy::Latest,
        fallback_strategy: None,
        allow_prereleases: false,
        allow_dev_versions: false,
        max_attempts: 3,
        enable_caching: true,
        conflict_resolution: ConflictResolutionMethod::Fail,
    });

    // Test that different conflict resolution methods are properly configured
    let deps = vec![
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Exact(Version::parse("1.0.0").unwrap()),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "=1.0.0".to_string(),
        },
        DependencySpec {
            path: "crates/rhema-core".to_string(),
            version_constraint: VersionConstraint::Exact(Version::parse("2.0.0").unwrap()),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "=2.0.0".to_string(),
        },
    ];

    let conflicts = resolver.detect_conflicts(&deps);
    assert_eq!(conflicts.len(), 1);
}

#[test]
fn test_version_compatibility_logic() {
    let versions = vec![
        Version::parse("1.0.0").unwrap(),
        Version::parse("1.1.0").unwrap(),
        Version::parse("1.2.0").unwrap(),
        Version::parse("2.0.0").unwrap(),
        Version::parse("2.1.0").unwrap(),
    ];

    let mut resolver = DependencyResolver::new(ResolutionStrategy::Compatible);

    // Test compatible version selection
    let constraint = VersionConstraint::Range(
        VersionReq::parse(">=1.0.0,<2.0.0").unwrap()
    );
    let resolved = resolver.resolve_version(&constraint, &versions).unwrap();
    assert_eq!(resolved, Version::parse("1.2.0").unwrap());

    // Test with broader range
    let constraint = VersionConstraint::Range(
        VersionReq::parse(">=1.0.0").unwrap()
    );
    let resolved = resolver.resolve_version(&constraint, &versions).unwrap();
    assert_eq!(resolved, Version::parse("2.1.0").unwrap());
}

#[test]
fn test_performance_under_load() {
    let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
    
    // Create many dependencies to test performance
    let mut deps = Vec::new();
    for i in 0..100 {
        deps.push(DependencySpec {
            path: format!("crates/dep_{}", i),
            version_constraint: VersionConstraint::Exact(Version::parse("1.0.0").unwrap()),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "=1.0.0".to_string(),
        });
    }

    let start = std::time::Instant::now();
    let conflicts = resolver.detect_conflicts(&deps);
    let duration = start.elapsed();

    assert_eq!(conflicts.len(), 0); // No conflicts in this scenario
    assert!(duration.as_millis() < 1000); // Should complete within 1 second
} 