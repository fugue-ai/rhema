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

use chrono::{DateTime, Utc};
use rhema_core::{RhemaLock, RhemaResult, LockedScope, LockedDependency};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Lock file context provider for AI agents
pub struct LockFileContextProvider {
    lock_file: Option<RhemaLock>,
    lock_file_path: PathBuf,
    last_updated: Option<DateTime<Utc>>,
}

impl LockFileContextProvider {
    /// Create a new lock file context provider
    pub fn new(lock_file_path: PathBuf) -> Self {
        Self {
            lock_file: None,
            lock_file_path,
            last_updated: None,
        }
    }

    /// Load or reload the lock file
    pub fn load_lock_file(&mut self) -> RhemaResult<()> {
        if self.lock_file_path.exists() {
            match rhema_core::lock::LockFileOps::read_lock_file(&self.lock_file_path) {
                Ok(lock_file) => {
                    self.lock_file = Some(lock_file);
                    self.last_updated = Some(Utc::now());
                }
                Err(e) => {
                    return Err(rhema_core::RhemaError::InvalidInput(
                        format!("Failed to load lock file: {}", e)
                    ));
                }
            }
        }
        Ok(())
    }

    /// Get comprehensive lock file context for AI agents
    pub fn get_ai_context(&self) -> RhemaResult<LockFileAIContext> {
        let lock_file = self.lock_file.as_ref()
            .ok_or_else(|| rhema_core::RhemaError::InvalidInput("No lock file loaded".to_string()))?;

        let mut context = LockFileAIContext {
            summary: self.generate_summary(lock_file),
            dependency_analysis: self.analyze_dependencies(lock_file),
            conflict_analysis: self.analyze_conflicts(lock_file),
            health_assessment: self.assess_health(lock_file),
            recommendations: self.generate_recommendations(lock_file),
            scope_details: HashMap::new(),
            last_updated: self.last_updated,
        };

        // Add detailed scope information
        for (scope_path, locked_scope) in &lock_file.scopes {
            context.scope_details.insert(scope_path.clone(), self.analyze_scope(locked_scope));
        }

        Ok(context)
    }

    /// Get scope-specific lock file context
    pub fn get_scope_context(&self, scope_path: &str) -> RhemaResult<ScopeLockContext> {
        let lock_file = self.lock_file.as_ref()
            .ok_or_else(|| rhema_core::RhemaError::InvalidInput("No lock file loaded".to_string()))?;

        if let Some(locked_scope) = lock_file.scopes.get(scope_path) {
            Ok(ScopeLockContext {
                scope_path: scope_path.to_string(),
                version: locked_scope.version.clone(),
                dependencies: self.analyze_scope_dependencies(locked_scope),
                health: self.assess_scope_health(locked_scope),
                recommendations: self.generate_scope_recommendations(locked_scope),
                last_resolved: locked_scope.resolved_at,
            })
        } else {
            Err(rhema_core::RhemaError::InvalidInput(
                format!("Scope not found in lock file: {}", scope_path)
            ))
        }
    }

    /// Generate a summary of the lock file for AI context
    fn generate_summary(&self, lock_file: &RhemaLock) -> LockFileSummary {
        LockFileSummary {
            total_scopes: lock_file.metadata.total_scopes as usize,
            total_dependencies: lock_file.metadata.total_dependencies as usize,
            circular_dependencies: lock_file.metadata.circular_dependencies as usize,
            validation_status: format!("{:?}", lock_file.metadata.validation_status),
            resolution_strategy: format!("{:?}", lock_file.metadata.resolution_strategy),
            conflict_resolution: format!("{:?}", lock_file.metadata.conflict_resolution),
            generated_at: lock_file.generated_at,
            generated_by: lock_file.generated_by.clone(),
        }
    }

    /// Analyze dependencies across all scopes
    fn analyze_dependencies(&self, lock_file: &RhemaLock) -> DependencyAnalysis {
        let mut analysis = DependencyAnalysis {
            direct_dependencies: 0,
            transitive_dependencies: 0,
            dependency_types: Vec::new(),
            version_distribution: HashMap::new(),
            outdated_dependencies: Vec::new(),
            security_concerns: Vec::new(),
        };

        for locked_scope in lock_file.scopes.values() {
            for (dep_name, dep) in &locked_scope.dependencies {
                // Count dependency types
                if let Some(pos) = analysis.dependency_types.iter().position(|(dt, _)| dt == &dep.dependency_type) {
                    analysis.dependency_types[pos].1 += 1;
                } else {
                    analysis.dependency_types.push((dep.dependency_type.clone(), 1));
                }

                // Count direct vs transitive
                if dep.is_transitive {
                    analysis.transitive_dependencies += 1;
                } else {
                    analysis.direct_dependencies += 1;
                }

                // Analyze version distribution
                *analysis.version_distribution.entry(dep.version.clone()).or_insert(0) += 1;

                // Check for outdated dependencies (simplified check)
                if dep.version.starts_with("0.") {
                    analysis.outdated_dependencies.push(OutdatedDependency {
                        name: dep_name.clone(),
                        current_version: dep.version.clone(),
                        scope: locked_scope.path.clone(),
                        reason: "Pre-release version".to_string(),
                    });
                }

                // Check for potential security concerns (simplified)
                if dep.version.contains("alpha") || dep.version.contains("beta") {
                    analysis.security_concerns.push(SecurityConcern {
                        dependency_name: dep_name.clone(),
                        scope: locked_scope.path.clone(),
                        concern: "Pre-release version may have security issues".to_string(),
                        severity: SecuritySeverity::Medium,
                    });
                }
            }
        }

        analysis
    }

    /// Analyze potential conflicts in the lock file
    fn analyze_conflicts(&self, lock_file: &RhemaLock) -> ConflictAnalysis {
        let mut analysis = ConflictAnalysis {
            version_conflicts: Vec::new(),
            circular_dependencies: Vec::new(),
            dependency_graph: HashMap::new(),
            conflict_resolution_strategy: format!("{:?}", lock_file.metadata.conflict_resolution),
        };

        // Build dependency graph
        for (scope_path, locked_scope) in &lock_file.scopes {
            let mut deps = Vec::new();
            for dep_name in locked_scope.dependencies.keys() {
                deps.push(dep_name.clone());
            }
            analysis.dependency_graph.insert(scope_path.clone(), deps);
        }

        // Detect version conflicts
        for (scope1_path, scope1) in &lock_file.scopes {
            for (dep_name, dep1) in &scope1.dependencies {
                for (scope2_path, scope2) in &lock_file.scopes {
                    if scope1_path != scope2_path {
                        if let Some(dep2) = scope2.dependencies.get(dep_name) {
                            if dep1.version != dep2.version {
                                analysis.version_conflicts.push(VersionConflict {
                                    dependency_name: dep_name.clone(),
                                    scope1: scope1_path.clone(),
                                    version1: dep1.version.clone(),
                                    scope2: scope2_path.clone(),
                                    version2: dep2.version.clone(),
                                    severity: ConflictSeverity::Medium,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Detect circular dependencies (simplified)
        if lock_file.metadata.circular_dependencies > 0 {
            analysis.circular_dependencies.push(CircularDependency {
                description: format!("{} circular dependencies detected", lock_file.metadata.circular_dependencies),
                affected_scopes: lock_file.scopes.keys().cloned().collect(),
                severity: ConflictSeverity::High,
            });
        }

        analysis
    }

    /// Assess the overall health of the lock file
    fn assess_health(&self, lock_file: &RhemaLock) -> HealthAssessment {
        let mut score = 100.0;
        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        // Check validation status
        if lock_file.metadata.validation_status != rhema_core::schema::ValidationStatus::Valid {
            score -= 30.0;
            issues.push("Lock file validation failed".to_string());
        }

        // Check circular dependencies
        if lock_file.metadata.circular_dependencies > 0 {
            score -= 20.0 * lock_file.metadata.circular_dependencies as f64;
            issues.push(format!("{} circular dependencies detected", lock_file.metadata.circular_dependencies));
        }

        // Check performance metrics
        if let Some(metrics) = &lock_file.metadata.performance_metrics {
            if metrics.generation_time_ms > 5000 {
                score -= 10.0;
                warnings.push("Lock file generation took too long".to_string());
            }

            // Calculate cache hit rate manually
            let total_cache_operations = metrics.cache_hits + metrics.cache_misses;
            if total_cache_operations > 0 {
                let hit_rate = metrics.cache_hits as f64 / total_cache_operations as f64;
                if hit_rate < 0.5 {
                    score -= 5.0;
                    warnings.push("Low cache hit rate detected".to_string());
                }
            }
        }

        // Check for outdated dependencies
        let outdated_count = lock_file.scopes.values()
            .flat_map(|scope| scope.dependencies.values())
            .filter(|dep| dep.version.starts_with("0."))
            .count();

        if outdated_count > 0 {
            score -= 5.0 * outdated_count as f64;
            warnings.push(format!("{} outdated dependencies found", outdated_count));
        }

        HealthAssessment {
            overall_score: score.max(0.0),
            status: if score >= 80.0 { HealthStatus::Good } else if score >= 60.0 { HealthStatus::Fair } else { HealthStatus::Poor },
            issues: issues.clone(),
            warnings: warnings.clone(),
            recommendations: self.generate_health_recommendations(score, &issues, &warnings),
        }
    }

    /// Generate recommendations based on lock file analysis
    fn generate_recommendations(&self, lock_file: &RhemaLock) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        // Recommendations based on validation status
        if lock_file.metadata.validation_status != rhema_core::schema::ValidationStatus::Valid {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Validation,
                priority: RecommendationPriority::High,
                title: "Fix validation issues".to_string(),
                description: "The lock file has validation errors that should be resolved".to_string(),
                action: "Review and fix validation errors in the lock file".to_string(),
            });
        }

        // Recommendations based on circular dependencies
        if lock_file.metadata.circular_dependencies > 0 {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Dependencies,
                priority: RecommendationPriority::High,
                title: "Resolve circular dependencies".to_string(),
                description: format!("{} circular dependencies detected", lock_file.metadata.circular_dependencies),
                action: "Review dependency relationships and break circular dependencies".to_string(),
            });
        }

        // Recommendations based on outdated dependencies
        let outdated_deps: Vec<_> = lock_file.scopes.values()
            .flat_map(|scope| scope.dependencies.iter())
            .filter(|(_, dep)| dep.version.starts_with("0."))
            .collect();

        if !outdated_deps.is_empty() {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Dependencies,
                priority: RecommendationPriority::Medium,
                title: "Update outdated dependencies".to_string(),
                description: format!("{} dependencies are using pre-release versions", outdated_deps.len()),
                action: "Consider upgrading to stable versions where possible".to_string(),
            });
        }

        // Performance recommendations
        if let Some(metrics) = &lock_file.metadata.performance_metrics {
            if metrics.generation_time_ms > 5000 {
                recommendations.push(Recommendation {
                    category: RecommendationCategory::Performance,
                    priority: RecommendationPriority::Medium,
                    title: "Optimize lock file generation".to_string(),
                    description: "Lock file generation is taking too long".to_string(),
                    action: "Consider optimizing dependency resolution or caching".to_string(),
                });
            }
        }

        recommendations
    }

    /// Analyze a specific scope
    fn analyze_scope(&self, locked_scope: &LockedScope) -> ScopeAnalysis {
        ScopeAnalysis {
            version: locked_scope.version.clone(),
            dependency_count: locked_scope.dependencies.len(),
            has_circular_dependencies: locked_scope.has_circular_dependencies,
            last_resolved: locked_scope.resolved_at,
            dependencies: self.analyze_scope_dependencies(locked_scope),
        }
    }

    /// Analyze dependencies for a specific scope
    fn analyze_scope_dependencies(&self, locked_scope: &LockedScope) -> Vec<DependencyInfo> {
        locked_scope.dependencies.iter().map(|(name, dep)| {
            DependencyInfo {
                name: name.clone(),
                version: dep.version.clone(),
                path: dep.path.clone(),
                dependency_type: dep.dependency_type.clone(),
                is_transitive: dep.is_transitive,
                original_constraint: dep.original_constraint.clone(),
                resolved_at: dep.resolved_at,
                checksum: dep.checksum.clone(),
            }
        }).collect()
    }

    /// Assess health for a specific scope
    fn assess_scope_health(&self, locked_scope: &LockedScope) -> ScopeHealth {
        let mut score = 100.0;
        let mut issues = Vec::new();

        if locked_scope.has_circular_dependencies {
            score -= 30.0;
            issues.push("Scope has circular dependencies".to_string());
        }

        let outdated_count = locked_scope.dependencies.values()
            .filter(|dep| dep.version.starts_with("0."))
            .count();

        if outdated_count > 0 {
            score -= 10.0 * outdated_count as f64;
            issues.push(format!("{} outdated dependencies", outdated_count));
        }

        ScopeHealth {
            score: score.max(0.0),
            status: if score >= 80.0 { HealthStatus::Good } else if score >= 60.0 { HealthStatus::Fair } else { HealthStatus::Poor },
            issues,
        }
    }

    /// Generate recommendations for a specific scope
    fn generate_scope_recommendations(&self, locked_scope: &LockedScope) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        if locked_scope.has_circular_dependencies {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Dependencies,
                priority: RecommendationPriority::High,
                title: "Resolve circular dependencies".to_string(),
                description: "This scope has circular dependencies".to_string(),
                action: "Review and break circular dependency chains".to_string(),
            });
        }

        let outdated_deps: Vec<_> = locked_scope.dependencies.iter()
            .filter(|(_, dep)| dep.version.starts_with("0."))
            .collect();

        if !outdated_deps.is_empty() {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Dependencies,
                priority: RecommendationPriority::Medium,
                title: "Update outdated dependencies".to_string(),
                description: format!("{} dependencies are using pre-release versions", outdated_deps.len()),
                action: "Consider upgrading to stable versions".to_string(),
            });
        }

        recommendations
    }

    /// Generate health recommendations
    fn generate_health_recommendations(&self, score: f64, issues: &[String], warnings: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if score < 70.0 {
            recommendations.push("Consider regenerating the lock file".to_string());
        }

        if issues.iter().any(|issue| issue.contains("circular dependencies")) {
            recommendations.push("Review and resolve circular dependencies".to_string());
        }

        if warnings.iter().any(|warning| warning.contains("outdated")) {
            recommendations.push("Update outdated dependencies to stable versions".to_string());
        }

        if score < 80.0 {
            recommendations.push("Monitor lock file health regularly".to_string());
        }

        recommendations
    }

    /// Check if lock file is available
    pub fn has_lock_file(&self) -> bool {
        self.lock_file.is_some()
    }

    /// Get the lock file path
    pub fn get_lock_file_path(&self) -> &PathBuf {
        &self.lock_file_path
    }

    /// Get last updated timestamp
    pub fn get_last_updated(&self) -> Option<DateTime<Utc>> {
        self.last_updated
    }
}

/// Comprehensive lock file context for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileAIContext {
    pub summary: LockFileSummary,
    pub dependency_analysis: DependencyAnalysis,
    pub conflict_analysis: ConflictAnalysis,
    pub health_assessment: HealthAssessment,
    pub recommendations: Vec<Recommendation>,
    pub scope_details: HashMap<String, ScopeAnalysis>,
    pub last_updated: Option<DateTime<Utc>>,
}

/// Lock file summary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileSummary {
    pub total_scopes: usize,
    pub total_dependencies: usize,
    pub circular_dependencies: usize,
    pub validation_status: String,
    pub resolution_strategy: String,
    pub conflict_resolution: String,
    pub generated_at: DateTime<Utc>,
    pub generated_by: String,
}

/// Dependency analysis information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    pub direct_dependencies: usize,
    pub transitive_dependencies: usize,
    pub dependency_types: Vec<(rhema_core::DependencyType, usize)>,
    pub version_distribution: HashMap<String, usize>,
    pub outdated_dependencies: Vec<OutdatedDependency>,
    pub security_concerns: Vec<SecurityConcern>,
}

/// Outdated dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutdatedDependency {
    pub name: String,
    pub current_version: String,
    pub scope: String,
    pub reason: String,
}

/// Security concern information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConcern {
    pub dependency_name: String,
    pub scope: String,
    pub concern: String,
    pub severity: SecuritySeverity,
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Conflict analysis information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictAnalysis {
    pub version_conflicts: Vec<VersionConflict>,
    pub circular_dependencies: Vec<CircularDependency>,
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub conflict_resolution_strategy: String,
}

/// Version conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConflict {
    pub dependency_name: String,
    pub scope1: String,
    pub version1: String,
    pub scope2: String,
    pub version2: String,
    pub severity: ConflictSeverity,
}

/// Circular dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularDependency {
    pub description: String,
    pub affected_scopes: Vec<String>,
    pub severity: ConflictSeverity,
}

/// Conflict severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Health assessment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAssessment {
    pub overall_score: f64,
    pub status: HealthStatus,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Health status levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Good,
    Fair,
    Poor,
}

/// Recommendation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub action: String,
}

/// Recommendation categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Validation,
    Dependencies,
    Performance,
    Security,
    Maintenance,
}

/// Recommendation priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Scope-specific lock file context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeLockContext {
    pub scope_path: String,
    pub version: String,
    pub dependencies: Vec<DependencyInfo>,
    pub health: ScopeHealth,
    pub recommendations: Vec<Recommendation>,
    pub last_resolved: DateTime<Utc>,
}

/// Scope analysis information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeAnalysis {
    pub version: String,
    pub dependency_count: usize,
    pub has_circular_dependencies: bool,
    pub last_resolved: DateTime<Utc>,
    pub dependencies: Vec<DependencyInfo>,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub path: String,
    pub dependency_type: rhema_core::DependencyType,
    pub is_transitive: bool,
    pub original_constraint: Option<String>,
    pub resolved_at: DateTime<Utc>,
    pub checksum: String,
}

/// Scope health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeHealth {
    pub score: f64,
    pub status: HealthStatus,
    pub issues: Vec<String>,
} 