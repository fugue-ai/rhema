use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::cmp::Ordering;
use semver::{Version, VersionReq};
use chrono::{DateTime, Utc};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, DependencyType, HealthStatus};

/// Version constraint for dependencies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionConstraint {
    /// Version requirement (e.g., ">=1.0.0, <2.0.0")
    pub requirement: String,
    /// Parsed version requirement
    pub parsed_requirement: VersionReq,
    /// Preferred version if available
    pub preferred_version: Option<Version>,
    /// Whether this is a hard requirement
    pub is_hard_requirement: bool,
}

impl VersionConstraint {
    /// Create a new version constraint
    pub fn new(requirement: String, preferred_version: Option<Version>, is_hard_requirement: bool) -> Result<Self> {
        let parsed_requirement = VersionReq::parse(&requirement)
            .map_err(|e| Error::InvalidVersionConstraint(format!("Invalid version requirement '{}': {}", requirement, e)))?;
        
        Ok(Self {
            requirement,
            parsed_requirement,
            preferred_version,
            is_hard_requirement,
        })
    }

    /// Check if a version satisfies this constraint
    pub fn satisfies(&self, version: &Version) -> bool {
        self.parsed_requirement.matches(version)
    }

    /// Get the best matching version from a list
    pub fn best_match(&self, versions: &[Version]) -> Option<Version> {
        let mut matching_versions: Vec<Version> = versions
            .iter()
            .filter(|v| self.satisfies(v))
            .cloned()
            .collect();

        if matching_versions.is_empty() {
            return None;
        }

        // Sort by version (newest first)
        matching_versions.sort_by(|a, b| b.cmp(a));

        // If we have a preferred version and it matches, use it
        if let Some(preferred) = &self.preferred_version {
            if self.satisfies(preferred) {
                return Some(preferred.clone());
            }
        }

        // Return the highest matching version
        matching_versions.first().cloned()
    }
}

/// Dependency resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResolutionStrategy {
    /// Use the latest compatible version
    Latest,
    /// Use the minimum compatible version
    Minimum,
    /// Use a specific version if available
    Specific(Version),
    /// Use the most stable version (lowest patch number)
    MostStable,
    /// Use the version with the best health score
    BestHealth,
}

/// Dependency conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConflict {
    /// Dependency ID
    pub dependency_id: String,
    /// Conflicting dependency ID
    pub conflicting_dependency_id: String,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Conflict description
    pub description: String,
    /// Severity level
    pub severity: ConflictSeverity,
    /// Suggested resolution
    pub suggested_resolution: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of dependency conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictType {
    /// Version conflict - incompatible versions
    VersionConflict,
    /// Circular dependency
    CircularDependency,
    /// Resource conflict - competing for same resource
    ResourceConflict,
    /// Security conflict - security policy violation
    SecurityConflict,
    /// Performance conflict - performance degradation
    PerformanceConflict,
}

/// Conflict severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConflictSeverity {
    /// Low severity - minor issue
    Low,
    /// Medium severity - moderate issue
    Medium,
    /// High severity - significant issue
    High,
    /// Critical severity - blocking issue
    Critical,
}

/// Dependency resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    /// Resolved dependencies
    pub resolved_dependencies: HashMap<String, ResolvedDependency>,
    /// Unresolved dependencies
    pub unresolved_dependencies: Vec<String>,
    /// Conflicts found
    pub conflicts: Vec<DependencyConflict>,
    /// Resolution strategy used
    pub strategy: ResolutionStrategy,
    /// Resolution time
    pub resolution_time: std::time::Duration,
    /// Whether resolution was successful
    pub successful: bool,
    /// Resolution metadata
    pub metadata: HashMap<String, String>,
}

/// Resolved dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    /// Dependency ID
    pub dependency_id: String,
    /// Resolved version
    pub resolved_version: Version,
    /// Alternative versions considered
    pub alternatives: Vec<Version>,
    /// Resolution confidence (0.0 to 1.0)
    pub confidence: f64,
    /// Health score of resolved version
    pub health_score: f64,
    /// Security score of resolved version
    pub security_score: f64,
    /// Performance score of resolved version
    pub performance_score: f64,
    /// Resolution timestamp
    pub timestamp: DateTime<Utc>,
}

/// Advanced dependency resolver
pub struct DependencyResolver {
    /// Available versions for each dependency
    available_versions: HashMap<String, Vec<Version>>,
    /// Version constraints
    version_constraints: HashMap<String, VersionConstraint>,
    /// Health scores for versions
    health_scores: HashMap<String, HashMap<Version, f64>>,
    /// Security scores for versions
    security_scores: HashMap<String, HashMap<Version, f64>>,
    /// Performance scores for versions
    performance_scores: HashMap<String, HashMap<Version, f64>>,
    /// Resolution cache
    resolution_cache: HashMap<String, ResolutionResult>,
    /// Cache TTL
    cache_ttl: std::time::Duration,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        Self {
            available_versions: HashMap::new(),
            version_constraints: HashMap::new(),
            health_scores: HashMap::new(),
            security_scores: HashMap::new(),
            performance_scores: HashMap::new(),
            resolution_cache: HashMap::new(),
            cache_ttl: std::time::Duration::from_secs(3600), // 1 hour
        }
    }

    /// Add available versions for a dependency
    pub fn add_available_versions(&mut self, dependency_id: String, versions: Vec<Version>) {
        self.available_versions.insert(dependency_id, versions);
    }

    /// Set version constraint for a dependency
    pub fn set_version_constraint(&mut self, dependency_id: String, constraint: VersionConstraint) {
        self.version_constraints.insert(dependency_id, constraint);
    }

    /// Set health scores for dependency versions
    pub fn set_health_scores(&mut self, dependency_id: String, scores: HashMap<Version, f64>) {
        self.health_scores.insert(dependency_id, scores);
    }

    /// Set security scores for dependency versions
    pub fn set_security_scores(&mut self, dependency_id: String, scores: HashMap<Version, f64>) {
        self.security_scores.insert(dependency_id, scores);
    }

    /// Set performance scores for dependency versions
    pub fn set_performance_scores(&mut self, dependency_id: String, scores: HashMap<Version, f64>) {
        self.performance_scores.insert(dependency_id, scores);
    }

    /// Resolve dependencies using the specified strategy
    pub fn resolve_dependencies(
        &mut self,
        dependencies: &[DependencyConfig],
        strategy: ResolutionStrategy,
    ) -> Result<ResolutionResult> {
        let start_time = std::time::Instant::now();
        
        let mut resolved_dependencies = HashMap::new();
        let mut unresolved_dependencies = Vec::new();
        let mut conflicts = Vec::new();

        for dependency in dependencies {
            match self.resolve_single_dependency(dependency, &strategy) {
                Ok(resolved) => {
                    resolved_dependencies.insert(dependency.id.clone(), resolved);
                }
                Err(e) => {
                    unresolved_dependencies.push(dependency.id.clone());
                    // Create conflict for unresolved dependency
                    conflicts.push(DependencyConflict {
                        dependency_id: dependency.id.clone(),
                        conflicting_dependency_id: "".to_string(),
                        conflict_type: ConflictType::VersionConflict,
                        description: format!("Failed to resolve dependency: {}", e),
                        severity: ConflictSeverity::High,
                        suggested_resolution: None,
                        timestamp: Utc::now(),
                    });
                }
            }
        }

        // Check for circular dependencies
        let circular_conflicts = self.detect_circular_dependencies(dependencies);
        conflicts.extend(circular_conflicts);

        // Check for resource conflicts
        let resource_conflicts = self.detect_resource_conflicts(&resolved_dependencies);
        conflicts.extend(resource_conflicts);

        let resolution_time = start_time.elapsed();
        let successful = unresolved_dependencies.is_empty() && conflicts.is_empty();

        let result = ResolutionResult {
            resolved_dependencies,
            unresolved_dependencies,
            conflicts,
            strategy: strategy.clone(),
            resolution_time,
            successful,
            metadata: HashMap::new(),
        };

        // Cache the result
        let cache_key = self.generate_cache_key(dependencies, &strategy);
        self.resolution_cache.insert(cache_key, result.clone());

        Ok(result)
    }

    /// Resolve a single dependency
    fn resolve_single_dependency(
        &self,
        dependency: &DependencyConfig,
        strategy: &ResolutionStrategy,
    ) -> Result<ResolvedDependency> {
        let available_versions = self.available_versions
            .get(&dependency.id)
            .ok_or_else(|| Error::DependencyNotFound(dependency.id.clone()))?;

        let constraint = self.version_constraints.get(&dependency.id);
        let compatible_versions = if let Some(constraint) = constraint {
            available_versions
                .iter()
                .filter(|v| constraint.satisfies(v))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            available_versions.clone()
        };

        if compatible_versions.is_empty() {
            return Err(Error::NoCompatibleVersion(dependency.id.clone()));
        }

        let resolved_version = match strategy {
            ResolutionStrategy::Latest => {
                compatible_versions.iter().max().cloned().unwrap()
            }
            ResolutionStrategy::Minimum => {
                compatible_versions.iter().min().cloned().unwrap()
            }
            ResolutionStrategy::Specific(version) => {
                if compatible_versions.contains(version) {
                    version.clone()
                } else {
                    return Err(Error::SpecificVersionNotFound(dependency.id.clone(), version.clone()));
                }
            }
            ResolutionStrategy::MostStable => {
                self.select_most_stable_version(&compatible_versions)
            }
            ResolutionStrategy::BestHealth => {
                self.select_best_health_version(&dependency.id, &compatible_versions)
            }
        };

        let health_score = self.get_health_score(&dependency.id, &resolved_version);
        let security_score = self.get_security_score(&dependency.id, &resolved_version);
        let performance_score = self.get_performance_score(&dependency.id, &resolved_version);

        let confidence = self.calculate_resolution_confidence(
            &dependency.id,
            &resolved_version,
            &compatible_versions,
        );

        Ok(ResolvedDependency {
            dependency_id: dependency.id.clone(),
            resolved_version,
            alternatives: compatible_versions,
            confidence,
            health_score,
            security_score,
            performance_score,
            timestamp: Utc::now(),
        })
    }

    /// Select the most stable version (lowest patch number)
    fn select_most_stable_version(&self, versions: &[Version]) -> Version {
        versions
            .iter()
            .min_by(|a, b| {
                // Compare by major.minor, then by patch (prefer lower patch)
                match (a.major, a.minor).cmp(&(b.major, b.minor)) {
                    Ordering::Equal => a.patch.cmp(&b.patch),
                    other => other,
                }
            })
            .cloned()
            .unwrap()
    }

    /// Select the version with the best health score
    fn select_best_health_version(&self, dependency_id: &str, versions: &[Version]) -> Version {
        versions
            .iter()
            .max_by(|a, b| {
                let score_a = self.get_health_score(dependency_id, a);
                let score_b = self.get_health_score(dependency_id, b);
                score_a.partial_cmp(&score_b).unwrap_or(Ordering::Equal)
            })
            .cloned()
            .unwrap()
    }

    /// Get health score for a version
    fn get_health_score(&self, dependency_id: &str, version: &Version) -> f64 {
        self.health_scores
            .get(dependency_id)
            .and_then(|scores| scores.get(version))
            .copied()
            .unwrap_or(0.5) // Default score
    }

    /// Get security score for a version
    fn get_security_score(&self, dependency_id: &str, version: &Version) -> f64 {
        self.security_scores
            .get(dependency_id)
            .and_then(|scores| scores.get(version))
            .copied()
            .unwrap_or(0.5) // Default score
    }

    /// Get performance score for a version
    fn get_performance_score(&self, dependency_id: &str, version: &Version) -> f64 {
        self.performance_scores
            .get(dependency_id)
            .and_then(|scores| scores.get(version))
            .copied()
            .unwrap_or(0.5) // Default score
    }

    /// Calculate resolution confidence
    fn calculate_resolution_confidence(
        &self,
        dependency_id: &str,
        resolved_version: &Version,
        alternatives: &[Version],
    ) -> f64 {
        let health_score = self.get_health_score(dependency_id, resolved_version);
        let security_score = self.get_security_score(dependency_id, resolved_version);
        let performance_score = self.get_performance_score(dependency_id, resolved_version);

        // Weighted average of scores
        (health_score * 0.4 + security_score * 0.3 + performance_score * 0.3)
            .min(1.0)
            .max(0.0)
    }

    /// Detect circular dependencies
    fn detect_circular_dependencies(&self, dependencies: &[DependencyConfig]) -> Vec<DependencyConflict> {
        // This is a simplified implementation
        // In a real implementation, you would use a proper graph algorithm
        Vec::new()
    }

    /// Detect resource conflicts
    fn detect_resource_conflicts(
        &self,
        resolved_dependencies: &HashMap<String, ResolvedDependency>,
    ) -> Vec<DependencyConflict> {
        // This is a simplified implementation
        // In a real implementation, you would check for resource conflicts
        Vec::new()
    }

    /// Generate cache key for resolution result
    fn generate_cache_key(&self, dependencies: &[DependencyConfig], strategy: &ResolutionStrategy) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        
        // Hash dependency IDs and versions
        for dep in dependencies {
            dep.id.hash(&mut hasher);
        }
        
        // Hash strategy
        format!("{:?}", strategy).hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }

    /// Clear expired cache entries
    pub fn clear_expired_cache(&mut self) {
        let now = std::time::Instant::now();
        self.resolution_cache.retain(|_, result| {
            result.resolution_time < self.cache_ttl
        });
    }

    /// Get cached resolution result
    pub fn get_cached_result(&self, cache_key: &str) -> Option<&ResolutionResult> {
        self.resolution_cache.get(cache_key)
    }

    /// Get resolution statistics
    pub fn get_statistics(&self) -> ResolutionStatistics {
        ResolutionStatistics {
            total_resolutions: self.resolution_cache.len(),
            cache_hit_rate: 0.0, // Would need to track hits/misses
            average_resolution_time: std::time::Duration::from_millis(0), // Would need to track
            total_conflicts: 0, // Would need to track
        }
    }
}

/// Resolution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionStatistics {
    /// Total number of resolutions performed
    pub total_resolutions: usize,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Average resolution time
    pub average_resolution_time: std::time::Duration,
    /// Total number of conflicts detected
    pub total_conflicts: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyConfig;

    fn create_test_dependency(id: &str, name: &str, dependency_type: DependencyType) -> DependencyConfig {
        DependencyConfig::new(
            id.to_string(),
            name.to_string(),
            dependency_type,
            "test-target".to_string(),
            vec!["test".to_string()],
        ).unwrap()
    }

    #[test]
    fn test_version_constraint_new() {
        let constraint = VersionConstraint::new(">=1.0.0".to_string(), None, true).unwrap();
        assert_eq!(constraint.requirement, ">=1.0.0");
        assert!(constraint.is_hard_requirement);
    }

    #[test]
    fn test_version_constraint_satisfies() {
        let constraint = VersionConstraint::new(">=1.0.0, <2.0.0".to_string(), None, true).unwrap();
        let version = Version::parse("1.5.0").unwrap();
        assert!(constraint.satisfies(&version));
    }

    #[test]
    fn test_version_constraint_best_match() {
        let constraint = VersionConstraint::new(">=1.0.0, <2.0.0".to_string(), None, true).unwrap();
        let versions = vec![
            Version::parse("0.9.0").unwrap(),
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.5.0").unwrap(),
            Version::parse("2.0.0").unwrap(),
        ];
        
        let best_match = constraint.best_match(&versions);
        assert_eq!(best_match, Some(Version::parse("1.5.0").unwrap()));
    }

    #[test]
    fn test_dependency_resolver_new() {
        let resolver = DependencyResolver::new();
        assert!(resolver.available_versions.is_empty());
        assert!(resolver.version_constraints.is_empty());
    }

    #[test]
    fn test_dependency_resolver_add_versions() {
        let mut resolver = DependencyResolver::new();
        let versions = vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.1.0").unwrap(),
            Version::parse("2.0.0").unwrap(),
        ];
        
        resolver.add_available_versions("test-dep".to_string(), versions);
        assert_eq!(resolver.available_versions.get("test-dep").unwrap().len(), 3);
    }
} 