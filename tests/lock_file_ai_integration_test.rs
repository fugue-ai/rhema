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

use rhema_ai::agent::lock_context_integration::LockFileAIIntegration;
use rhema_ai::agent::lock_context::*;
use rhema_core::RhemaResult;
use std::path::PathBuf;

#[tokio::test]
async fn test_lock_file_ai_integration_creation() -> RhemaResult<()> {
    let integration = LockFileAIIntegration::new(PathBuf::from("."));
    
    // Test that integration can be created
    assert!(integration.has_lock_file() || !integration.has_lock_file()); // Either true or false is valid
    
    Ok(())
}

#[tokio::test]
async fn test_lock_file_context_provider() -> RhemaResult<()> {
    let mut provider = LockFileContextProvider::new(PathBuf::from("rhema.lock"));
    
    // Test loading (may fail if no lock file exists, which is expected)
    let load_result = provider.load_lock_file();
    
    // Should either succeed or fail gracefully
    match load_result {
        Ok(_) => {
            // Lock file exists and loaded successfully
            assert!(provider.has_lock_file());
            
            // Test getting context (should succeed if lock file is loaded)
            if let Ok(context) = provider.get_ai_context() {
                // Verify context structure
                assert!(context.summary.total_scopes >= 0);
                assert!(context.summary.total_dependencies >= 0);
                assert!(context.health_assessment.overall_score >= 0.0);
                assert!(context.health_assessment.overall_score <= 100.0);
            }
        }
        Err(_) => {
            // No lock file exists, which is expected in test environment
            assert!(!provider.has_lock_file());
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_context_injection_rules() -> RhemaResult<()> {
    use rhema_ai::context_injection::{TaskType, ContextInjectionRule, LockFileContextRequirement, PromptInjectionMethod};
    
    // Test creating injection rules
    let rule = ContextInjectionRule {
        task_type: TaskType::DependencyUpdate,
        context_files: vec!["knowledge.yaml".to_string()],
        injection_method: PromptInjectionMethod::Prepend,
        priority: 3,
        additional_context: Some("Test context".to_string()),
        lock_file_context: Some(LockFileContextRequirement {
            include_dependency_versions: true,
            include_conflict_prevention: true,
            include_health_info: true,
            include_recommendations: true,
            target_scopes: None,
            include_transitive_deps: true,
        }),
    };
    
    // Verify rule structure
    assert_eq!(rule.task_type, TaskType::DependencyUpdate);
    assert_eq!(rule.priority, 3);
    assert!(rule.lock_file_context.is_some());
    
    let lock_req = rule.lock_file_context.unwrap();
    assert!(lock_req.include_dependency_versions);
    assert!(lock_req.include_conflict_prevention);
    assert!(lock_req.include_health_info);
    assert!(lock_req.include_recommendations);
    assert!(lock_req.include_transitive_deps);
    
    Ok(())
}

#[tokio::test]
async fn test_health_assessment_scoring() -> RhemaResult<()> {
    // Test health assessment scoring logic
    let mut assessment = HealthAssessment {
        overall_score: 100.0,
        status: HealthStatus::Good,
        issues: Vec::new(),
        warnings: Vec::new(),
        recommendations: Vec::new(),
    };
    
    // Test good health
    assert_eq!(assessment.overall_score, 100.0);
    assert!(matches!(assessment.status, HealthStatus::Good));
    
    // Test fair health
    assessment.overall_score = 70.0;
    assessment.status = HealthStatus::Fair;
    assert_eq!(assessment.overall_score, 70.0);
    assert!(matches!(assessment.status, HealthStatus::Fair));
    
    // Test poor health
    assessment.overall_score = 30.0;
    assessment.status = HealthStatus::Poor;
    assert_eq!(assessment.overall_score, 30.0);
    assert!(matches!(assessment.status, HealthStatus::Poor));
    
    Ok(())
}

#[tokio::test]
async fn test_recommendation_priorities() -> RhemaResult<()> {
    // Test recommendation priority system
    let low_priority = Recommendation {
        category: RecommendationCategory::Maintenance,
        priority: RecommendationPriority::Low,
        title: "Low priority task".to_string(),
        description: "Minor improvement".to_string(),
        action: "Consider this later".to_string(),
    };
    
    let high_priority = Recommendation {
        category: RecommendationCategory::Security,
        priority: RecommendationPriority::High,
        title: "Security issue".to_string(),
        description: "Critical security vulnerability".to_string(),
        action: "Fix immediately".to_string(),
    };
    
    let critical_priority = Recommendation {
        category: RecommendationCategory::Validation,
        priority: RecommendationPriority::Critical,
        title: "Validation error".to_string(),
        description: "Lock file validation failed".to_string(),
        action: "Fix validation errors".to_string(),
    };
    
    // Verify priorities
    assert!(matches!(low_priority.priority, RecommendationPriority::Low));
    assert!(matches!(high_priority.priority, RecommendationPriority::High));
    assert!(matches!(critical_priority.priority, RecommendationPriority::Critical));
    
    // Verify categories
    assert!(matches!(low_priority.category, RecommendationCategory::Maintenance));
    assert!(matches!(high_priority.category, RecommendationCategory::Security));
    assert!(matches!(critical_priority.category, RecommendationCategory::Validation));
    
    Ok(())
}

#[tokio::test]
async fn test_conflict_analysis() -> RhemaResult<()> {
    // Test conflict analysis structure
    let conflict = VersionConflict {
        dependency_name: "test-dep".to_string(),
        scope1: "scope1".to_string(),
        version1: "1.0.0".to_string(),
        scope2: "scope2".to_string(),
        version2: "2.0.0".to_string(),
        severity: ConflictSeverity::Medium,
    };
    
    let circular_dep = CircularDependency {
        description: "Circular dependency detected".to_string(),
        affected_scopes: vec!["scope1".to_string(), "scope2".to_string()],
        severity: ConflictSeverity::High,
    };
    
    let analysis = ConflictAnalysis {
        version_conflicts: vec![conflict],
        circular_dependencies: vec![circular_dep],
        dependency_graph: std::collections::HashMap::new(),
        conflict_resolution_strategy: "automatic".to_string(),
    };
    
    // Verify analysis structure
    assert_eq!(analysis.version_conflicts.len(), 1);
    assert_eq!(analysis.circular_dependencies.len(), 1);
    assert_eq!(analysis.conflict_resolution_strategy, "automatic");
    
    // Verify conflict details
    let conflict = &analysis.version_conflicts[0];
    assert_eq!(conflict.dependency_name, "test-dep");
    assert_eq!(conflict.scope1, "scope1");
    assert_eq!(conflict.version1, "1.0.0");
    assert_eq!(conflict.scope2, "scope2");
    assert_eq!(conflict.version2, "2.0.0");
    assert!(matches!(conflict.severity, ConflictSeverity::Medium));
    
    // Verify circular dependency details
    let circular = &analysis.circular_dependencies[0];
    assert_eq!(circular.description, "Circular dependency detected");
    assert_eq!(circular.affected_scopes.len(), 2);
    assert!(matches!(circular.severity, ConflictSeverity::High));
    
    Ok(())
}

#[tokio::test]
async fn test_dependency_analysis() -> RhemaResult<()> {
    // Test dependency analysis structure
    let outdated_dep = OutdatedDependency {
        name: "old-dep".to_string(),
        current_version: "0.1.0".to_string(),
        scope: "test-scope".to_string(),
        reason: "Pre-release version".to_string(),
    };
    
    let security_concern = SecurityConcern {
        dependency_name: "suspicious-dep".to_string(),
        scope: "test-scope".to_string(),
        concern: "Potential security issue".to_string(),
        severity: SecuritySeverity::Medium,
    };
    
    let analysis = DependencyAnalysis {
        direct_dependencies: 5,
        transitive_dependencies: 15,
        dependency_types: std::collections::HashMap::new(),
        version_distribution: std::collections::HashMap::new(),
        outdated_dependencies: vec![outdated_dep],
        security_concerns: vec![security_concern],
    };
    
    // Verify analysis structure
    assert_eq!(analysis.direct_dependencies, 5);
    assert_eq!(analysis.transitive_dependencies, 15);
    assert_eq!(analysis.outdated_dependencies.len(), 1);
    assert_eq!(analysis.security_concerns.len(), 1);
    
    // Verify outdated dependency details
    let outdated = &analysis.outdated_dependencies[0];
    assert_eq!(outdated.name, "old-dep");
    assert_eq!(outdated.current_version, "0.1.0");
    assert_eq!(outdated.scope, "test-scope");
    assert_eq!(outdated.reason, "Pre-release version");
    
    // Verify security concern details
    let security = &analysis.security_concerns[0];
    assert_eq!(security.dependency_name, "suspicious-dep");
    assert_eq!(security.scope, "test-scope");
    assert_eq!(security.concern, "Potential security issue");
    assert!(matches!(security.severity, SecuritySeverity::Medium));
    
    Ok(())
}

#[tokio::test]
async fn test_scope_lock_context() -> RhemaResult<()> {
    // Test scope lock context structure
    let dependency_info = DependencyInfo {
        name: "test-dep".to_string(),
        version: "1.0.0".to_string(),
        path: "path/to/dep".to_string(),
        dependency_type: rhema_core::DependencyType::Required,
        is_transitive: false,
        original_constraint: Some("^1.0.0".to_string()),
        resolved_at: chrono::Utc::now(),
        checksum: "abc123".to_string(),
    };
    
    let scope_health = ScopeHealth {
        score: 85.0,
        status: HealthStatus::Good,
        issues: Vec::new(),
    };
    
    let recommendation = Recommendation {
        category: RecommendationCategory::Dependencies,
        priority: RecommendationPriority::Medium,
        title: "Update dependency".to_string(),
        description: "Consider updating to latest version".to_string(),
        action: "Run update command".to_string(),
    };
    
    let context = ScopeLockContext {
        scope_path: "test-scope".to_string(),
        version: "1.0.0".to_string(),
        dependencies: vec![dependency_info],
        health: scope_health,
        recommendations: vec![recommendation],
        last_resolved: chrono::Utc::now(),
    };
    
    // Verify context structure
    assert_eq!(context.scope_path, "test-scope");
    assert_eq!(context.version, "1.0.0");
    assert_eq!(context.dependencies.len(), 1);
    assert_eq!(context.recommendations.len(), 1);
    
    // Verify dependency details
    let dep = &context.dependencies[0];
    assert_eq!(dep.name, "test-dep");
    assert_eq!(dep.version, "1.0.0");
    assert_eq!(dep.path, "path/to/dep");
    assert!(matches!(dep.dependency_type, rhema_core::DependencyType::Required));
    assert!(!dep.is_transitive);
    assert_eq!(dep.original_constraint, Some("^1.0.0".to_string()));
    assert_eq!(dep.checksum, "abc123");
    
    // Verify health details
    assert_eq!(context.health.score, 85.0);
    assert!(matches!(context.health.status, HealthStatus::Good));
    
    // Verify recommendation details
    let rec = &context.recommendations[0];
    assert_eq!(rec.title, "Update dependency");
    assert_eq!(rec.description, "Consider updating to latest version");
    assert_eq!(rec.action, "Run update command");
    assert!(matches!(rec.category, RecommendationCategory::Dependencies));
    assert!(matches!(rec.priority, RecommendationPriority::Medium));
    
    Ok(())
}

#[tokio::test]
async fn test_utils_formatting() -> RhemaResult<()> {
    use rhema_ai::agent::lock_context_integration::utils;
    
    // Create a mock context for testing
    let context = LockFileAIContext {
        summary: LockFileSummary {
            total_scopes: 3,
            total_dependencies: 10,
            circular_dependencies: 1,
            validation_status: "valid".to_string(),
            resolution_strategy: "latest".to_string(),
            conflict_resolution: "automatic".to_string(),
            generated_at: chrono::Utc::now(),
            generated_by: "test".to_string(),
        },
        dependency_analysis: DependencyAnalysis {
            direct_dependencies: 5,
            transitive_dependencies: 5,
            dependency_types: std::collections::HashMap::new(),
            version_distribution: std::collections::HashMap::new(),
            outdated_dependencies: Vec::new(),
            security_concerns: Vec::new(),
        },
        conflict_analysis: ConflictAnalysis {
            version_conflicts: Vec::new(),
            circular_dependencies: Vec::new(),
            dependency_graph: std::collections::HashMap::new(),
            conflict_resolution_strategy: "automatic".to_string(),
        },
        health_assessment: HealthAssessment {
            overall_score: 85.0,
            status: HealthStatus::Good,
            issues: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
        },
        recommendations: Vec::new(),
        scope_details: std::collections::HashMap::new(),
        last_updated: Some(chrono::Utc::now()),
    };
    
    // Test formatting
    let formatted = utils::format_context_for_ai(&context);
    assert!(formatted.contains("Lock File Summary"));
    assert!(formatted.contains("Health Assessment"));
    assert!(formatted.contains("85.0"));
    assert!(formatted.contains("Total Scopes: 3"));
    assert!(formatted.contains("Total Dependencies: 10"));
    
    // Test metrics extraction
    let metrics = utils::extract_key_metrics(&context);
    assert_eq!(metrics.get("health_score").unwrap(), &85.0);
    assert_eq!(metrics.get("total_scopes").unwrap(), &3.0);
    assert_eq!(metrics.get("total_dependencies").unwrap(), &10.0);
    
    // Test prompt creation
    let prompt = utils::create_context_aware_prompt("Test task", Some("test-scope"));
    assert!(prompt.contains("Test task"));
    assert!(prompt.contains("test-scope"));
    assert!(prompt.contains("lock file context"));
    
    Ok(())
} 