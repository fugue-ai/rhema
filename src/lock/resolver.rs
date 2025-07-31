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

use crate::{
    schema::{DependencyType, LockedDependency, ResolutionStrategy},
    RhemaError, RhemaResult,
};
use chrono::Utc;
use log::{error, info, warn};
use semver::{Version, VersionReq};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::path::Path;
use std::time::Instant;

/// Enhanced version constraint specification with advanced features
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    /// Exact version (e.g., "1.2.3")
    Exact(Version),
    /// Version range (e.g., ">=1.2.0,<2.0.0")
    Range(VersionReq),
    /// Latest version
    Latest,
    /// Earliest compatible version
    Earliest,
    /// Pinned version (from lock file)
    Pinned(Version),
    /// Caret range (e.g., "^1.2.3" for >=1.2.3,<2.0.0)
    Caret(Version),
    /// Tilde range (e.g., "~1.2.3" for >=1.2.3,<1.3.0)
    Tilde(Version),
    /// Wildcard version (e.g., "1.2.*" for >=1.2.0,<1.3.0)
    Wildcard(Version),
    /// Pre-release version constraint
    Prerelease(Version),
    /// Development version constraint
    Development(Version),
}

/// Enhanced dependency specification with comprehensive metadata
#[derive(Debug, Clone)]
pub struct DependencySpec {
    /// Path to the dependency
    pub path: String,
    /// Version constraint
    pub version_constraint: VersionConstraint,
    /// Type of dependency
    pub dependency_type: DependencyType,
    /// Whether this is a transitive dependency
    pub is_transitive: bool,
    /// Original constraint string for reference
    pub original_constraint: String,
    /// Scope that requires this dependency
    pub scope_path: String,
    /// Priority level for resolution (higher = more important)
    pub priority: u8,
    /// Whether this dependency is optional
    pub optional: bool,
    /// Alternative dependencies if this one fails
    pub alternatives: Vec<String>,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Enhanced dependency conflict information with detailed analysis
#[derive(Debug, Clone)]
pub struct DependencyConflict {
    /// Name of the conflicting dependency
    pub dependency_name: String,
    /// Conflicting version requirements
    pub requirements: Vec<DependencyRequirement>,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Suggested resolution
    pub suggested_resolution: Option<Version>,
    /// Conflict severity
    pub severity: ConflictSeverity,
    /// Detailed conflict description
    pub description: String,
    /// Affected scopes
    pub affected_scopes: Vec<String>,
    /// Resolution recommendations
    pub recommendations: Vec<String>,
}

/// Individual dependency requirement with enhanced metadata
#[derive(Debug, Clone)]
pub struct DependencyRequirement {
    /// Scope that requires this dependency
    pub scope_path: String,
    /// Version constraint
    pub constraint: VersionConstraint,
    /// Dependency type
    pub dependency_type: DependencyType,
    /// Priority level
    pub priority: u8,
    /// Whether this is optional
    pub optional: bool,
    /// Original constraint string
    pub original_constraint: String,
}

/// Enhanced conflict types with detailed categorization
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    /// Version range incompatibility
    VersionIncompatibility,
    /// Circular dependency
    CircularDependency,
    /// Missing dependency
    MissingDependency,
    /// Ambiguous resolution
    AmbiguousResolution,
    /// Security vulnerability
    SecurityVulnerability,
    /// License incompatibility
    LicenseIncompatibility,
    /// Architecture incompatibility
    ArchitectureIncompatibility,
    /// Platform incompatibility
    PlatformIncompatibility,
    /// Dependency depth exceeded
    DepthExceeded,
    /// Version constraint parsing error
    ConstraintParseError,
}

/// Conflict severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ConflictSeverity {
    /// Low severity - can be auto-resolved
    Low,
    /// Medium severity - requires user attention
    Medium,
    /// High severity - may break functionality
    High,
    /// Critical severity - must be resolved manually
    Critical,
}

/// Enhanced resolution strategy configuration
#[derive(Debug, Clone)]
pub struct ResolutionConfig {
    /// Primary resolution strategy
    pub strategy: ResolutionStrategy,
    /// Fallback strategy if primary fails
    pub fallback_strategy: Option<ResolutionStrategy>,
    /// Whether to allow pre-release versions
    pub allow_prereleases: bool,
    /// Whether to allow development versions
    pub allow_dev_versions: bool,
    /// Maximum resolution attempts
    pub max_attempts: usize,
    /// Whether to use caching
    pub enable_caching: bool,
    /// Conflict resolution method
    pub conflict_resolution: ConflictResolutionMethod,
    /// Maximum dependency depth
    pub max_depth: usize,
    /// Whether to resolve transitive dependencies
    pub resolve_transitive: bool,
    /// Whether to prefer stable versions
    pub prefer_stable: bool,
    /// Whether to allow version downgrades
    pub allow_downgrades: bool,
    /// Whether to use semantic versioning strictly
    pub strict_semver: bool,
    /// Timeout for resolution operations (in seconds)
    pub timeout_seconds: u64,
    /// Whether to enable parallel resolution
    pub parallel_resolution: bool,
    /// Maximum parallel resolution threads
    pub max_parallel_threads: usize,
}

/// Enhanced conflict resolution methods
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictResolutionMethod {
    /// Automatically resolve conflicts
    Automatic,
    /// Prompt user for resolution
    Prompt,
    /// Skip conflicting dependencies
    Skip,
    /// Fail on conflicts
    Fail,
    /// Use highest compatible version
    HighestCompatible,
    /// Use lowest compatible version
    LowestCompatible,
    /// Use most recent version
    MostRecent,
    /// Use most stable version
    MostStable,
    /// Use version with highest priority
    HighestPriority,
    /// Use version with best compatibility score
    BestCompatibility,
}

/// Enhanced dependency graph node with comprehensive metadata
#[derive(Debug, Clone)]
pub struct DependencyNode {
    /// Scope path
    pub path: String,
    /// Dependencies of this scope
    pub dependencies: Vec<String>,
    /// Version information
    pub version: Option<Version>,
    /// Whether this node has been resolved
    pub resolved: bool,
    /// Resolution depth
    pub depth: usize,
    /// Whether this is a root dependency
    pub is_root: bool,
    /// Resolution priority
    pub priority: u8,
    /// Resolution timestamp
    pub resolved_at: Option<chrono::DateTime<Utc>>,
    /// Resolution metadata
    pub metadata: HashMap<String, String>,
}

/// Enhanced dependency resolution result with comprehensive analysis
#[derive(Debug, Clone)]
pub struct ResolutionResult {
    /// Resolved dependencies
    pub dependencies: HashMap<String, LockedDependency>,
    /// Detected conflicts
    pub conflicts: Vec<DependencyConflict>,
    /// Resolution statistics
    pub stats: ResolutionStats,
    /// Whether resolution was successful
    pub successful: bool,
    /// Dependency graph
    pub dependency_graph: HashMap<String, DependencyNode>,
    /// Resolution warnings
    pub warnings: Vec<String>,
    /// Resolution recommendations
    pub recommendations: Vec<String>,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Enhanced resolution statistics with detailed metrics
#[derive(Debug, Clone)]
pub struct ResolutionStats {
    /// Total dependencies processed
    pub total_dependencies: usize,
    /// Successfully resolved dependencies
    pub resolved_dependencies: usize,
    /// Failed resolutions
    pub failed_resolutions: usize,
    /// Cache hits
    pub cache_hits: usize,
    /// Cache misses
    pub cache_misses: usize,
    /// Resolution time in milliseconds
    pub resolution_time_ms: u64,
    /// Number of conflicts detected
    pub conflicts_detected: usize,
    /// Number of circular dependencies
    pub circular_dependencies: usize,
    /// Number of transitive dependencies
    pub transitive_dependencies: usize,
    /// Number of version conflicts
    pub version_conflicts: usize,
    /// Number of security vulnerabilities
    pub security_vulnerabilities: usize,
    /// Average resolution depth
    pub average_depth: f64,
    /// Maximum resolution depth
    pub max_depth: usize,
}

/// Performance metrics for resolution operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Total resolution time
    pub total_time_ms: u64,
    /// Time spent on version resolution
    pub version_resolution_time_ms: u64,
    /// Time spent on conflict detection
    pub conflict_detection_time_ms: u64,
    /// Time spent on graph building
    pub graph_building_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Number of cache operations
    pub cache_operations: usize,
    /// Number of parallel operations
    pub parallel_operations: usize,
}

impl Default for ResolutionStats {
    fn default() -> Self {
        Self {
            total_dependencies: 0,
            resolved_dependencies: 0,
            failed_resolutions: 0,
            cache_hits: 0,
            cache_misses: 0,
            resolution_time_ms: 0,
            conflicts_detected: 0,
            circular_dependencies: 0,
            transitive_dependencies: 0,
            version_conflicts: 0,
            security_vulnerabilities: 0,
            average_depth: 0.0,
            max_depth: 0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_time_ms: 0,
            version_resolution_time_ms: 0,
            conflict_detection_time_ms: 0,
            graph_building_time_ms: 0,
            memory_usage_bytes: 0,
            cache_operations: 0,
            parallel_operations: 0,
        }
    }
}

/// Enhanced dependency resolver with comprehensive resolution capabilities
pub struct DependencyResolver {
    config: ResolutionConfig,
    cache: HashMap<String, LockedDependency>,
    available_versions: HashMap<String, Vec<Version>>,
    resolution_history: Vec<String>,
    version_cache: HashMap<String, Version>,
    conflict_cache: HashMap<String, Vec<DependencyConflict>>,
    dependency_graph: HashMap<String, DependencyNode>,
    resolution_metrics: PerformanceMetrics,
}

impl DependencyResolver {
    /// Create a new dependency resolver with default configuration
    pub fn new(strategy: ResolutionStrategy) -> Self {
        Self::with_config(ResolutionConfig {
            strategy,
            fallback_strategy: None,
            allow_prereleases: false,
            allow_dev_versions: false,
            max_attempts: 3,
            enable_caching: true,
            conflict_resolution: ConflictResolutionMethod::Automatic,
            max_depth: 10,
            resolve_transitive: true,
            prefer_stable: true,
            allow_downgrades: false,
            strict_semver: true,
            timeout_seconds: 30,
            parallel_resolution: false,
            max_parallel_threads: 4,
        })
    }

    /// Create a new dependency resolver with custom configuration
    pub fn with_config(config: ResolutionConfig) -> Self {
        Self {
            cache: HashMap::new(),
            available_versions: HashMap::new(),
            resolution_history: Vec::new(),
            version_cache: HashMap::new(),
            conflict_cache: HashMap::new(),
            dependency_graph: HashMap::new(),
            resolution_metrics: PerformanceMetrics::default(),
            config,
        }
    }

    /// Enhanced dependency resolution with comprehensive analysis
    pub fn resolve_dependencies(
        &mut self,
        dependencies: &[DependencySpec],
        repo_path: &Path,
    ) -> RhemaResult<ResolutionResult> {
        let start_time = Instant::now();
        info!("Starting enhanced dependency resolution for {} dependencies", dependencies.len());

        let mut result = ResolutionResult {
            dependencies: HashMap::new(),
            conflicts: Vec::new(),
            stats: ResolutionStats::default(),
            successful: true,
            dependency_graph: HashMap::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
        };

        result.stats.total_dependencies = dependencies.len();

        // Build enhanced dependency graph
        let graph_start = Instant::now();
        self.build_enhanced_dependency_graph(dependencies, &mut result)?;
        result.performance_metrics.graph_building_time_ms = graph_start.elapsed().as_millis() as u64;
        
        // Detect circular dependencies with enhanced algorithm
        let conflict_start = Instant::now();
        let circular_deps = self.detect_circular_dependencies_enhanced(&result.dependency_graph)?;
        if !circular_deps.is_empty() {
            warn!("Enhanced circular dependencies detected: {:?}", circular_deps);
            result.stats.circular_dependencies = circular_deps.len();
            for cycle in circular_deps {
                result.conflicts.push(DependencyConflict {
                    dependency_name: cycle.join(" -> "),
                    requirements: Vec::new(),
                    conflict_type: ConflictType::CircularDependency,
                    suggested_resolution: None,
                    severity: ConflictSeverity::High,
                    description: format!("Circular dependency detected: {}", cycle.join(" -> ")),
                    affected_scopes: cycle.clone(),
                    recommendations: vec![
                        "Remove one of the dependencies in the cycle".to_string(),
                        "Use dependency injection to break the cycle".to_string(),
                        "Consider using interfaces or abstractions".to_string(),
                    ],
                });
            }
        }
        result.performance_metrics.conflict_detection_time_ms = conflict_start.elapsed().as_millis() as u64;

        // Detect version conflicts
        let version_conflicts = self.detect_version_conflicts_enhanced(dependencies);
        result.stats.version_conflicts = version_conflicts.len();
        result.conflicts.extend(version_conflicts);

        // Resolve each dependency with enhanced strategies
        let version_resolution_start = Instant::now();
        for dep_spec in dependencies {
            match self.resolve_single_dependency_enhanced(dep_spec, repo_path, &mut result) {
                Ok(resolved_dep) => {
                    result.dependencies.insert(dep_spec.path.clone(), resolved_dep);
                    result.stats.resolved_dependencies += 1;
                }
                Err(e) => {
                    error!("Failed to resolve dependency {}: {}", dep_spec.path, e);
                    result.stats.failed_resolutions += 1;
                    result.successful = false;
                    
                    // Add to conflicts if it's a resolution error
                    if let RhemaError::LockError(_) = e {
                        result.conflicts.push(DependencyConflict {
                            dependency_name: dep_spec.path.clone(),
                            requirements: vec![DependencyRequirement {
                                scope_path: dep_spec.scope_path.clone(),
                                constraint: dep_spec.version_constraint.clone(),
                                dependency_type: dep_spec.dependency_type.clone(),
                                priority: dep_spec.priority,
                                optional: dep_spec.optional,
                                original_constraint: dep_spec.original_constraint.clone(),
                            }],
                            conflict_type: ConflictType::MissingDependency,
                            suggested_resolution: None,
                            severity: ConflictSeverity::Medium,
                            description: format!("Failed to resolve dependency: {}", e),
                            affected_scopes: vec![dep_spec.scope_path.clone()],
                            recommendations: vec![
                                "Check if the dependency exists".to_string(),
                                "Verify the version constraint".to_string(),
                                "Consider using an alternative dependency".to_string(),
                            ],
                        });
                    }
                }
            }
        }
        result.performance_metrics.version_resolution_time_ms = version_resolution_start.elapsed().as_millis() as u64;

        // Calculate final statistics
        result.stats.resolution_time_ms = start_time.elapsed().as_millis() as u64;
        result.stats.conflicts_detected = result.conflicts.len();
        result.performance_metrics.total_time_ms = result.stats.resolution_time_ms;
        result.dependency_graph = self.dependency_graph.clone();

        // Generate recommendations
        self.generate_resolution_recommendations(&mut result);

        info!(
            "Enhanced dependency resolution completed: {}/{} resolved, {} conflicts, {}ms",
            result.stats.resolved_dependencies,
            result.stats.total_dependencies,
            result.conflicts.len(),
            result.stats.resolution_time_ms
        );

        Ok(result)
    }

    /// Enhanced single dependency resolution with multiple strategies
    fn resolve_single_dependency_enhanced(
        &mut self,
        dep_spec: &DependencySpec,
        repo_path: &Path,
        result: &mut ResolutionResult,
    ) -> RhemaResult<LockedDependency> {
        let cache_key = format!("{}:{}:{}", dep_spec.path, dep_spec.original_constraint, dep_spec.scope_path);
        
        // Check cache first
        if self.config.enable_caching {
            if let Some(cached) = self.cache.get(&cache_key) {
                result.stats.cache_hits += 1;
                return Ok(cached.clone());
            }
        }
        result.stats.cache_misses += 1;

        // Get available versions with enhanced discovery
        let available_versions = self.get_available_versions_enhanced(&dep_spec.path, repo_path)?;
        
        // Resolve version with enhanced strategies
        let resolved_version = self.resolve_version_enhanced(&dep_spec.version_constraint, &available_versions, dep_spec)?;
        
        // Create enhanced locked dependency
        let mut locked_dep = LockedDependency::new(
            &resolved_version.to_string(),
            &dep_spec.path,
            dep_spec.dependency_type.clone(),
        );
        
        locked_dep.set_original_constraint(&dep_spec.original_constraint);
        if dep_spec.is_transitive {
            locked_dep.mark_transitive();
        }
        
        // Add enhanced metadata
        locked_dep.custom.insert("priority".to_string(), dep_spec.priority.to_string().into());
        locked_dep.custom.insert("scope_path".to_string(), dep_spec.scope_path.clone().into());
        locked_dep.custom.insert("optional".to_string(), dep_spec.optional.to_string().into());
        
        // Calculate enhanced checksum
        locked_dep.update_checksum();
        
        // Cache the result
        if self.config.enable_caching {
            self.cache.insert(cache_key, locked_dep.clone());
        }
        
        Ok(locked_dep)
    }

    /// Enhanced version resolution with multiple strategies and fallbacks
    pub fn resolve_version_enhanced(
        &self,
        constraint: &VersionConstraint,
        available_versions: &[Version],
        dep_spec: &DependencySpec,
    ) -> RhemaResult<Version> {
        let mut attempts = 0;
        let mut strategies = vec![&self.config.strategy];
        
        if let Some(fallback) = &self.config.fallback_strategy {
            strategies.push(fallback);
        }

        // Add additional fallback strategies based on configuration
        if self.config.prefer_stable {
            strategies.push(&ResolutionStrategy::Compatible);
        }

        for strategy in strategies {
            while attempts < self.config.max_attempts {
                match self.resolve_version_with_enhanced_strategy(constraint, available_versions, strategy, dep_spec) {
                    Ok(version) => return Ok(version),
                    Err(e) => {
                        attempts += 1;
                        warn!(
                            "Enhanced version resolution attempt {} failed with strategy {:?}: {}",
                            attempts, strategy, e
                        );
                    }
                }
            }
        }

        Err(RhemaError::LockError(format!(
            "Failed to resolve version for constraint {:?} after {} attempts",
            constraint, self.config.max_attempts
        )))
    }

    /// Enhanced version resolution using specific strategy with advanced features
    fn resolve_version_with_enhanced_strategy(
        &self,
        constraint: &VersionConstraint,
        available_versions: &[Version],
        strategy: &ResolutionStrategy,
        dep_spec: &DependencySpec,
    ) -> RhemaResult<Version> {
        let mut filtered_versions = self.filter_versions_enhanced(available_versions, dep_spec)?;
        
        if filtered_versions.is_empty() {
            return Err(RhemaError::LockError(format!(
                "No compatible versions found for constraint {:?}",
                constraint
            )));
        }

        match strategy {
            ResolutionStrategy::Latest => {
                filtered_versions.sort();
                Ok(filtered_versions.pop().unwrap())
            }
            ResolutionStrategy::Earliest => {
                filtered_versions.sort();
                Ok(filtered_versions.remove(0))
            }
            ResolutionStrategy::Pinned => {
                match constraint {
                    VersionConstraint::Pinned(version) => Ok(version.clone()),
                    _ => Err(RhemaError::LockError(
                        "Pinned strategy requires pinned version constraint".to_string(),
                    )),
                }
            }
            ResolutionStrategy::Range => {
                // For range strategy, use the highest compatible version
                filtered_versions.sort();
                Ok(filtered_versions.pop().unwrap())
            }
            ResolutionStrategy::Compatible => {
                // Find the most compatible version with enhanced scoring
                self.find_most_compatible_version_enhanced(&filtered_versions, dep_spec)
            }
        }
    }

    /// Enhanced version filtering with advanced criteria
    pub fn filter_versions_enhanced(&self, versions: &[Version], dep_spec: &DependencySpec) -> RhemaResult<Vec<Version>> {
        let filtered: Vec<Version> = versions
            .iter()
            .filter(|v| {
                // Filter out pre-releases unless allowed
                if !self.config.allow_prereleases && !v.pre.is_empty() {
                    return false;
                }
                
                // Filter out dev versions unless allowed
                if !self.config.allow_dev_versions && v.build.contains("dev") {
                    return false;
                }

                // Apply constraint-specific filtering
                match &dep_spec.version_constraint {
                    VersionConstraint::Exact(target) => *v == target,
                    VersionConstraint::Range(req) => req.matches(v),
                    VersionConstraint::Caret(base) => {
                        let major = base.major;
                        let min_version = base.clone();
                        let max_version = Version::new(major + 1, 0, 0);
                        **v >= min_version && **v < max_version
                    }
                    VersionConstraint::Tilde(base) => {
                        let min_version = base.clone();
                        let max_version = Version::new(base.major, base.minor + 1, 0);
                        **v >= min_version && **v < max_version
                    }
                    VersionConstraint::Wildcard(base) => {
                        let min_version = base.clone();
                        let max_version = Version::new(base.major, base.minor + 1, 0);
                        **v >= min_version && **v < max_version
                    }
                    VersionConstraint::Prerelease(base) => {
                        **v >= *base && !v.pre.is_empty()
                    }
                    VersionConstraint::Development(base) => {
                        **v >= *base && v.build.contains("dev")
                    }
                    _ => true, // Latest, Earliest, Pinned don't need filtering here
                }
            })
            .cloned()
            .collect();

        if filtered.is_empty() {
            return Err(RhemaError::LockError(
                "No versions available after enhanced filtering".to_string(),
            ));
        }

        Ok(filtered)
    }

    /// Enhanced compatibility scoring for version selection
    fn find_most_compatible_version_enhanced(&self, versions: &[Version], dep_spec: &DependencySpec) -> RhemaResult<Version> {
        if versions.is_empty() {
            return Err(RhemaError::LockError("No versions available for compatibility analysis".to_string()));
        }

        // Score each version based on multiple criteria
        let mut scored_versions: Vec<(Version, f64)> = versions
            .iter()
            .map(|v| {
                let mut score = 0.0;
                
                // Prefer stable versions
                if v.pre.is_empty() && !v.build.contains("dev") {
                    score += 10.0;
                }
                
                // Prefer recent versions
                score += v.major as f64 * 100.0 + v.minor as f64 * 10.0 + v.patch as f64;
                
                // Prefer versions that match the original constraint more closely
                if let VersionConstraint::Exact(target) = &dep_spec.version_constraint {
                    let diff = (v.major as i64 - target.major as i64).abs() +
                              (v.minor as i64 - target.minor as i64).abs() +
                              (v.patch as i64 - target.patch as i64).abs();
                    score -= diff as f64;
                }
                
                (v.clone(), score)
            })
            .collect();

        // Sort by score (highest first)
        scored_versions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(scored_versions[0].0.clone())
    }

    /// Enhanced available version discovery
    fn get_available_versions_enhanced(&mut self, dep_path: &str, repo_path: &Path) -> RhemaResult<Vec<Version>> {
        // Check cache first
        if let Some(versions) = self.available_versions.get(dep_path) {
            return Ok(versions.clone());
        }

        // Enhanced version discovery logic
        let dep_path_buf = repo_path.join(dep_path);
        if !dep_path_buf.exists() {
            return Err(RhemaError::LockError(format!(
                "Dependency path does not exist: {}",
                dep_path
            )));
        }

        // In a real implementation, this would:
        // 1. Parse the dependency's scope file
        // 2. Extract version information from git tags
        // 3. Check for version metadata in the dependency
        // 4. Consider external version sources
        
        // For now, we'll simulate available versions with more realistic data
        let versions = vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.0.1").unwrap(),
            Version::parse("1.1.0").unwrap(),
            Version::parse("1.1.1").unwrap(),
            Version::parse("1.2.0").unwrap(),
            Version::parse("1.2.1").unwrap(),
            Version::parse("2.0.0").unwrap(),
            Version::parse("2.0.1").unwrap(),
            Version::parse("2.1.0").unwrap(),
            Version::parse("2.1.1").unwrap(),
        ];

        // Cache the versions
        self.available_versions.insert(dep_path.to_string(), versions.clone());
        
        Ok(versions)
    }

    /// Enhanced dependency graph building
    fn build_enhanced_dependency_graph(
        &mut self,
        dependencies: &[DependencySpec],
        result: &mut ResolutionResult,
    ) -> RhemaResult<()> {
        for (index, dep) in dependencies.iter().enumerate() {
            let node = DependencyNode {
                path: dep.path.clone(),
                dependencies: Vec::new(), // Would be populated with actual dependencies
                version: None,
                resolved: false,
                depth: 0,
                is_root: index < dependencies.len() / 2, // Simple heuristic
                priority: dep.priority,
                resolved_at: None,
                metadata: dep.metadata.clone(),
            };
            
            self.dependency_graph.insert(dep.path.clone(), node);
        }
        
        result.dependency_graph = self.dependency_graph.clone();
        Ok(())
    }

    /// Enhanced circular dependency detection with detailed analysis
    pub fn detect_circular_dependencies_enhanced(&self, graph: &HashMap<String, DependencyNode>) -> RhemaResult<Vec<Vec<String>>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut cycles = Vec::new();

        for node in graph.keys() {
            if !visited.contains(node) {
                let mut path = Vec::new();
                if self.has_cycle_dfs_enhanced(graph, node, &mut visited, &mut rec_stack, &mut path) {
                    cycles.push(path);
                }
            }
        }

        Ok(cycles)
    }

    /// Enhanced DFS helper for cycle detection
    fn has_cycle_dfs_enhanced(
        &self,
        graph: &HashMap<String, DependencyNode>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(node_data) = graph.get(node) {
            for neighbor in &node_data.dependencies {
                if !visited.contains(neighbor) {
                    if self.has_cycle_dfs_enhanced(graph, neighbor, visited, rec_stack, path) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    // Found a back edge, indicating a cycle
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        false
    }

    /// Enhanced version constraint parsing with support for advanced formats
    pub fn parse_version_constraint_enhanced(constraint: &str) -> RhemaResult<VersionConstraint> {
        match constraint {
            "latest" => Ok(VersionConstraint::Latest),
            "earliest" => Ok(VersionConstraint::Earliest),
            _ => {
                if constraint.starts_with('=') {
                    // Exact version
                    let version_str = &constraint[1..];
                    let version = Version::parse(version_str)
                        .map_err(|e| RhemaError::LockError(format!("Invalid version: {}", e)))?;
                    Ok(VersionConstraint::Exact(version))
                } else if constraint.starts_with('^') {
                    // Caret range
                    let version_str = &constraint[1..];
                    let version = Version::parse(version_str)
                        .map_err(|e| RhemaError::LockError(format!("Invalid version: {}", e)))?;
                    Ok(VersionConstraint::Caret(version))
                } else if constraint.starts_with('~') {
                    // Tilde range
                    let version_str = &constraint[1..];
                    let version = Version::parse(version_str)
                        .map_err(|e| RhemaError::LockError(format!("Invalid version: {}", e)))?;
                    Ok(VersionConstraint::Tilde(version))
                } else if constraint.contains(',') || constraint.contains('<') || constraint.contains('>') {
                    // Version range
                    let req = VersionReq::parse(constraint)
                        .map_err(|e| RhemaError::LockError(format!("Invalid version requirement: {}", e)))?;
                    Ok(VersionConstraint::Range(req))
                } else if constraint.contains('*') {
                    // Wildcard version
                    let version_str = constraint.replace('*', "0");
                    let version = Version::parse(&version_str)
                        .map_err(|e| RhemaError::LockError(format!("Invalid version: {}", e)))?;
                    Ok(VersionConstraint::Wildcard(version))
                } else {
                    // Try to parse as exact version
                    match Version::parse(constraint) {
                        Ok(version) => Ok(VersionConstraint::Exact(version)),
                        Err(_) => Err(RhemaError::LockError(format!(
                            "Invalid version constraint: {}",
                            constraint
                        ))),
                    }
                }
            }
        }
    }

    /// Enhanced conflict detection with detailed analysis
    pub fn detect_version_conflicts_enhanced(&self, dependencies: &[DependencySpec]) -> Vec<DependencyConflict> {
        let mut conflicts = Vec::new();
        let mut requirements: HashMap<String, Vec<DependencyRequirement>> = HashMap::new();

        // Group requirements by dependency name
        for dep in dependencies {
            let reqs = requirements.entry(dep.path.clone()).or_insert_with(Vec::new);
            reqs.push(DependencyRequirement {
                scope_path: dep.scope_path.clone(),
                constraint: dep.version_constraint.clone(),
                dependency_type: dep.dependency_type.clone(),
                priority: dep.priority,
                optional: dep.optional,
                original_constraint: dep.original_constraint.clone(),
            });
        }

        // Check for conflicts with enhanced analysis
        for (dep_name, reqs) in requirements {
            if reqs.len() > 1 {
                if let Some(conflict) = self.check_version_conflicts_enhanced(&dep_name, &reqs) {
                    conflicts.push(conflict);
                }
            }
        }

        conflicts
    }

    /// Enhanced version conflict checking with detailed analysis
    fn check_version_conflicts_enhanced(
        &self,
        dep_name: &str,
        requirements: &[DependencyRequirement],
    ) -> Option<DependencyConflict> {
        if requirements.len() <= 1 {
            return None;
        }

        let mut exact_versions = HashSet::new();
        let mut range_requirements = Vec::new();
        let mut incompatible_requirements = Vec::new();

        for req in requirements {
            match &req.constraint {
                VersionConstraint::Exact(version) => {
                    exact_versions.insert(version.clone());
                }
                VersionConstraint::Range(range_req) => {
                    range_requirements.push((req.clone(), range_req.clone()));
                }
                _ => {
                    // For other constraint types, we'll need more sophisticated analysis
                    incompatible_requirements.push(req.clone());
                }
            }
        }

        // Check for exact version conflicts
        if exact_versions.len() > 1 {
            return Some(DependencyConflict {
                dependency_name: dep_name.to_string(),
                requirements: requirements.to_vec(),
                conflict_type: ConflictType::VersionIncompatibility,
                suggested_resolution: exact_versions.iter().next().cloned(),
                severity: ConflictSeverity::High,
                description: format!("Multiple exact version requirements: {:?}", exact_versions),
                affected_scopes: requirements.iter().map(|r| r.scope_path.clone()).collect(),
                recommendations: vec![
                    "Choose one of the exact versions".to_string(),
                    "Use a version range instead of exact versions".to_string(),
                    "Consider using a compatible version".to_string(),
                ],
            });
        }

        // Check for range incompatibilities
        if !range_requirements.is_empty() {
            // This would require more sophisticated range intersection analysis
            // For now, we'll just check if there are multiple range requirements
            if range_requirements.len() > 1 {
                return Some(DependencyConflict {
                    dependency_name: dep_name.to_string(),
                    requirements: requirements.to_vec(),
                    conflict_type: ConflictType::VersionIncompatibility,
                    suggested_resolution: None,
                    severity: ConflictSeverity::Medium,
                    description: "Multiple range requirements that may be incompatible".to_string(),
                    affected_scopes: requirements.iter().map(|r| r.scope_path.clone()).collect(),
                    recommendations: vec![
                        "Analyze range intersections".to_string(),
                        "Choose a compatible version range".to_string(),
                        "Consider using a more specific version constraint".to_string(),
                    ],
                });
            }
        }

        None
    }

    /// Generate resolution recommendations based on analysis
    fn generate_resolution_recommendations(&self, result: &mut ResolutionResult) {
        if result.stats.conflicts_detected > 0 {
            result.recommendations.push(
                "Review and resolve dependency conflicts before proceeding".to_string()
            );
        }

        if result.stats.circular_dependencies > 0 {
            result.recommendations.push(
                "Break circular dependencies to improve build stability".to_string()
            );
        }

        if result.stats.failed_resolutions > 0 {
            result.recommendations.push(
                "Check failed dependency resolutions and update constraints".to_string()
            );
        }

        if result.stats.average_depth > 5.0 {
            result.recommendations.push(
                "Consider flattening dependency tree to reduce complexity".to_string()
            );
        }

        if result.stats.cache_hits < result.stats.cache_misses {
            result.recommendations.push(
                "Enable caching to improve resolution performance".to_string()
            );
        }
    }

    /// Clear all caches
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.available_versions.clear();
        self.resolution_history.clear();
        self.version_cache.clear();
        self.conflict_cache.clear();
        self.dependency_graph.clear();
    }

    /// Get comprehensive resolution statistics
    pub fn get_stats(&self) -> ResolutionStats {
        ResolutionStats {
            cache_hits: 0, // Would be tracked in real implementation
            cache_misses: 0, // Would be tracked in real implementation
            ..Default::default()
        }
    }

    /// Get cache size for testing
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Get dependency graph for analysis
    pub fn get_dependency_graph(&self) -> &HashMap<String, DependencyNode> {
        &self.dependency_graph
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.resolution_metrics
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new(ResolutionStrategy::Latest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::DependencyType;

    #[test]
    fn test_enhanced_version_constraint_parsing() {
        // Test exact version
        let constraint = DependencyResolver::parse_version_constraint_enhanced("=1.2.3").unwrap();
        assert!(matches!(constraint, VersionConstraint::Exact(_)));

        // Test range
        let constraint = DependencyResolver::parse_version_constraint_enhanced(">=1.2.0,<2.0.0").unwrap();
        assert!(matches!(constraint, VersionConstraint::Range(_)));

        // Test latest
        let constraint = DependencyResolver::parse_version_constraint_enhanced("latest").unwrap();
        assert!(matches!(constraint, VersionConstraint::Latest));

        // Test earliest
        let constraint = DependencyResolver::parse_version_constraint_enhanced("earliest").unwrap();
        assert!(matches!(constraint, VersionConstraint::Earliest));

        // Test caret range
        let constraint = DependencyResolver::parse_version_constraint_enhanced("^1.2.3").unwrap();
        assert!(matches!(constraint, VersionConstraint::Caret(_)));

        // Test tilde range
        let constraint = DependencyResolver::parse_version_constraint_enhanced("~1.2.3").unwrap();
        assert!(matches!(constraint, VersionConstraint::Tilde(_)));

        // Test wildcard
        let constraint = DependencyResolver::parse_version_constraint_enhanced("1.2.*").unwrap();
        assert!(matches!(constraint, VersionConstraint::Wildcard(_)));

        // Test invalid constraint
        let result = DependencyResolver::parse_version_constraint_enhanced("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_enhanced_version_resolution() {
        let mut resolver = DependencyResolver::new(ResolutionStrategy::Latest);
        let versions = vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.1.0").unwrap(),
            Version::parse("2.0.0").unwrap(),
        ];

        let constraint = VersionConstraint::Range(
            VersionReq::parse(">=1.0.0,<3.0.0").unwrap()
        );

        let dep_spec = DependencySpec {
            path: "test/dep".to_string(),
            version_constraint: constraint.clone(),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: ">=1.0.0,<3.0.0".to_string(),
            scope_path: "test/scope".to_string(),
            priority: 1,
            optional: false,
            alternatives: Vec::new(),
            metadata: HashMap::new(),
        };

        let resolved = resolver.resolve_version_enhanced(&constraint, &versions, &dep_spec).unwrap();
        assert_eq!(resolved, Version::parse("2.0.0").unwrap());
    }

    #[test]
    fn test_enhanced_circular_dependency_detection() {
        let mut graph = HashMap::new();
        
        // Create a circular dependency: A -> B -> C -> A
        graph.insert("A".to_string(), DependencyNode {
            path: "A".to_string(),
            dependencies: vec!["B".to_string()],
            version: None,
            resolved: false,
            depth: 0,
            is_root: true,
            priority: 1,
            resolved_at: None,
            metadata: HashMap::new(),
        });
        
        graph.insert("B".to_string(), DependencyNode {
            path: "B".to_string(),
            dependencies: vec!["C".to_string()],
            version: None,
            resolved: false,
            depth: 1,
            is_root: false,
            priority: 1,
            resolved_at: None,
            metadata: HashMap::new(),
        });
        
        graph.insert("C".to_string(), DependencyNode {
            path: "C".to_string(),
            dependencies: vec!["A".to_string()],
            version: None,
            resolved: false,
            depth: 2,
            is_root: false,
            priority: 1,
            resolved_at: None,
            metadata: HashMap::new(),
        });

        let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
        let cycles = resolver.detect_circular_dependencies_enhanced(&graph).unwrap();
        
        assert!(!cycles.is_empty());
        assert!(cycles.iter().any(|cycle| cycle.len() >= 3));
    }

    #[test]
    fn test_enhanced_dependency_spec_creation() {
        let spec = DependencySpec {
            path: "crates/core".to_string(),
            version_constraint: VersionConstraint::Range(
                VersionReq::parse(">=1.0.0,<2.0.0").unwrap()
            ),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: ">=1.0.0,<2.0.0".to_string(),
            scope_path: "crates/app".to_string(),
            priority: 5,
            optional: false,
            alternatives: vec!["crates/core-alt".to_string()],
            metadata: {
                let mut map = HashMap::new();
                map.insert("category".to_string(), "core".to_string());
                map
            },
        };

        assert_eq!(spec.path, "crates/core");
        assert!(!spec.is_transitive);
        assert_eq!(spec.priority, 5);
        assert!(!spec.optional);
        assert_eq!(spec.alternatives.len(), 1);
        assert_eq!(spec.metadata.get("category").unwrap(), "core");
    }

    #[test]
    fn test_enhanced_conflict_detection() {
        let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
        
        let deps = vec![
            DependencySpec {
                path: "crates/core".to_string(),
                version_constraint: VersionConstraint::Exact(Version::parse("1.0.0").unwrap()),
                dependency_type: DependencyType::Required,
                is_transitive: false,
                original_constraint: "=1.0.0".to_string(),
                scope_path: "crates/app1".to_string(),
                priority: 1,
                optional: false,
                alternatives: Vec::new(),
                metadata: HashMap::new(),
            },
            DependencySpec {
                path: "crates/core".to_string(),
                version_constraint: VersionConstraint::Exact(Version::parse("2.0.0").unwrap()),
                dependency_type: DependencyType::Required,
                is_transitive: false,
                original_constraint: "=2.0.0".to_string(),
                scope_path: "crates/app2".to_string(),
                priority: 1,
                optional: false,
                alternatives: Vec::new(),
                metadata: HashMap::new(),
            },
        ];

        let conflicts = resolver.detect_version_conflicts_enhanced(&deps);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].dependency_name, "crates/core");
        assert_eq!(conflicts[0].conflict_type, ConflictType::VersionIncompatibility);
        assert_eq!(conflicts[0].severity, ConflictSeverity::High);
    }

    #[test]
    fn test_enhanced_resolution_strategies() {
        let versions = vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.1.0").unwrap(),
            Version::parse("2.0.0").unwrap(),
        ];

        let dep_spec = DependencySpec {
            path: "test/dep".to_string(),
            version_constraint: VersionConstraint::Range(
                VersionReq::parse(">=1.0.0").unwrap()
            ),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: ">=1.0.0".to_string(),
            scope_path: "test/scope".to_string(),
            priority: 1,
            optional: false,
            alternatives: Vec::new(),
            metadata: HashMap::new(),
        };

        // Test Latest strategy
        let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
        let constraint = VersionConstraint::Range(
            VersionReq::parse(">=1.0.0").unwrap()
        );
        let resolved = resolver.resolve_version_enhanced(&constraint, &versions, &dep_spec).unwrap();
        assert_eq!(resolved, Version::parse("2.0.0").unwrap());

        // Test Earliest strategy
        let resolver = DependencyResolver::new(ResolutionStrategy::Earliest);
        let resolved = resolver.resolve_version_enhanced(&constraint, &versions, &dep_spec).unwrap();
        assert_eq!(resolved, Version::parse("1.0.0").unwrap());

        // Test Pinned strategy
        let resolver = DependencyResolver::new(ResolutionStrategy::Pinned);
        let pinned_constraint = VersionConstraint::Pinned(Version::parse("1.1.0").unwrap());
        let resolved = resolver.resolve_version_enhanced(&pinned_constraint, &versions, &dep_spec).unwrap();
        assert_eq!(resolved, Version::parse("1.1.0").unwrap());
    }

    #[test]
    fn test_enhanced_version_filtering() {
        let versions = vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.0.0-alpha").unwrap(),
            Version::parse("1.0.0-dev").unwrap(),
            Version::parse("1.0.0-beta").unwrap(),
        ];

        let dep_spec = DependencySpec {
            path: "test/dep".to_string(),
            version_constraint: VersionConstraint::Latest,
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "latest".to_string(),
            scope_path: "test/scope".to_string(),
            priority: 1,
            optional: false,
            alternatives: Vec::new(),
            metadata: HashMap::new(),
        };

        let resolver = DependencyResolver::with_config(ResolutionConfig {
            strategy: ResolutionStrategy::Latest,
            fallback_strategy: None,
            allow_prereleases: false,
            allow_dev_versions: false,
            max_attempts: 3,
            enable_caching: true,
            conflict_resolution: ConflictResolutionMethod::Automatic,
            max_depth: 10,
            resolve_transitive: true,
            prefer_stable: true,
            allow_downgrades: false,
            strict_semver: true,
            timeout_seconds: 30,
            parallel_resolution: false,
            max_parallel_threads: 4,
        });

        let filtered = resolver.filter_versions_enhanced(&versions, &dep_spec).unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0], Version::parse("1.0.0").unwrap());
    }

    #[test]
    fn test_enhanced_cache_operations() {
        let mut resolver = DependencyResolver::new(ResolutionStrategy::Latest);
        
        // Initially cache should be empty
        assert_eq!(resolver.cache.len(), 0);
        
        // Clear cache
        resolver.clear_cache();
        assert_eq!(resolver.cache.len(), 0);
        assert_eq!(resolver.available_versions.len(), 0);
        assert_eq!(resolver.version_cache.len(), 0);
        assert_eq!(resolver.conflict_cache.len(), 0);
        assert_eq!(resolver.dependency_graph.len(), 0);
    }

    #[test]
    fn test_enhanced_error_handling() {
        let resolver = DependencyResolver::new(ResolutionStrategy::Latest);
        
        // Test invalid version constraint
        let result = DependencyResolver::parse_version_constraint_enhanced("invalid");
        assert!(result.is_err());
        
        // Test empty versions list
        let constraint = VersionConstraint::Range(
            VersionReq::parse(">=1.0.0").unwrap()
        );
        let dep_spec = DependencySpec {
            path: "test/dep".to_string(),
            version_constraint: constraint.clone(),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: ">=1.0.0".to_string(),
            scope_path: "test/scope".to_string(),
            priority: 1,
            optional: false,
            alternatives: Vec::new(),
            metadata: HashMap::new(),
        };
        let result = resolver.resolve_version_enhanced(&constraint, &[], &dep_spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_enhanced_compatibility_scoring() {
        let versions = vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.1.0").unwrap(),
            Version::parse("2.0.0").unwrap(),
            Version::parse("2.0.0-alpha").unwrap(),
        ];

        let dep_spec = DependencySpec {
            path: "test/dep".to_string(),
            version_constraint: VersionConstraint::Exact(Version::parse("1.1.0").unwrap()),
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "=1.1.0".to_string(),
            scope_path: "test/scope".to_string(),
            priority: 1,
            optional: false,
            alternatives: Vec::new(),
            metadata: HashMap::new(),
        };

        let resolver = DependencyResolver::new(ResolutionStrategy::Compatible);
        let constraint = VersionConstraint::Range(
            VersionReq::parse(">=1.0.0").unwrap()
        );
        let resolved = resolver.resolve_version_enhanced(&constraint, &versions, &dep_spec).unwrap();
        
        // Should prefer stable version closest to the exact constraint
        assert_eq!(resolved, Version::parse("1.1.0").unwrap());
    }

    #[test]
    fn test_enhanced_caret_range_parsing() {
        let constraint = DependencyResolver::parse_version_constraint_enhanced("^1.2.3").unwrap();
        if let VersionConstraint::Caret(version) = constraint {
            assert_eq!(version, Version::parse("1.2.3").unwrap());
        } else {
            panic!("Expected Caret constraint");
        }
    }

    #[test]
    fn test_enhanced_tilde_range_parsing() {
        let constraint = DependencyResolver::parse_version_constraint_enhanced("~1.2.3").unwrap();
        if let VersionConstraint::Tilde(version) = constraint {
            assert_eq!(version, Version::parse("1.2.3").unwrap());
        } else {
            panic!("Expected Tilde constraint");
        }
    }

    #[test]
    fn test_enhanced_wildcard_parsing() {
        let constraint = DependencyResolver::parse_version_constraint_enhanced("1.2.*").unwrap();
        if let VersionConstraint::Wildcard(version) = constraint {
            assert_eq!(version, Version::parse("1.2.0").unwrap());
        } else {
            panic!("Expected Wildcard constraint");
        }
    }

    #[test]
    fn test_enhanced_conflict_severity() {
        let conflict = DependencyConflict {
            dependency_name: "test/dep".to_string(),
            requirements: Vec::new(),
            conflict_type: ConflictType::CircularDependency,
            suggested_resolution: None,
            severity: ConflictSeverity::Critical,
            description: "Critical circular dependency".to_string(),
            affected_scopes: vec!["scope1".to_string(), "scope2".to_string()],
            recommendations: vec!["Fix immediately".to_string()],
        };

        assert_eq!(conflict.severity, ConflictSeverity::Critical);
        assert_eq!(conflict.conflict_type, ConflictType::CircularDependency);
        assert_eq!(conflict.affected_scopes.len(), 2);
    }

    #[test]
    fn test_enhanced_resolution_config() {
        let config = ResolutionConfig {
            strategy: ResolutionStrategy::Latest,
            fallback_strategy: Some(ResolutionStrategy::Compatible),
            allow_prereleases: false,
            allow_dev_versions: false,
            max_attempts: 5,
            enable_caching: true,
            conflict_resolution: ConflictResolutionMethod::Automatic,
            max_depth: 15,
            resolve_transitive: true,
            prefer_stable: true,
            allow_downgrades: false,
            strict_semver: true,
            timeout_seconds: 60,
            parallel_resolution: true,
            max_parallel_threads: 8,
        };

        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.max_depth, 15);
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.max_parallel_threads, 8);
        assert!(config.parallel_resolution);
        assert!(config.prefer_stable);
        assert!(!config.allow_downgrades);
    }

    #[test]
    fn test_enhanced_dependency_node() {
        let node = DependencyNode {
            path: "test/dep".to_string(),
            dependencies: vec!["dep1".to_string(), "dep2".to_string()],
            version: Some(Version::parse("1.0.0").unwrap()),
            resolved: true,
            depth: 2,
            is_root: false,
            priority: 5,
            resolved_at: Some(Utc::now()),
            metadata: {
                let mut map = HashMap::new();
                map.insert("category".to_string(), "test".to_string());
                map
            },
        };

        assert_eq!(node.path, "test/dep");
        assert_eq!(node.dependencies.len(), 2);
        assert!(node.resolved);
        assert_eq!(node.depth, 2);
        assert!(!node.is_root);
        assert_eq!(node.priority, 5);
        assert!(node.resolved_at.is_some());
        assert_eq!(node.metadata.get("category").unwrap(), "test");
    }

    #[test]
    fn test_enhanced_performance_metrics() {
        let metrics = PerformanceMetrics {
            total_time_ms: 1000,
            version_resolution_time_ms: 500,
            conflict_detection_time_ms: 200,
            graph_building_time_ms: 100,
            memory_usage_bytes: 1024 * 1024,
            cache_operations: 50,
            parallel_operations: 10,
        };

        assert_eq!(metrics.total_time_ms, 1000);
        assert_eq!(metrics.version_resolution_time_ms, 500);
        assert_eq!(metrics.conflict_detection_time_ms, 200);
        assert_eq!(metrics.graph_building_time_ms, 100);
        assert_eq!(metrics.memory_usage_bytes, 1024 * 1024);
        assert_eq!(metrics.cache_operations, 50);
        assert_eq!(metrics.parallel_operations, 10);
    }

    #[test]
    fn test_enhanced_resolution_stats() {
        let stats = ResolutionStats {
            total_dependencies: 100,
            resolved_dependencies: 95,
            failed_resolutions: 5,
            cache_hits: 80,
            cache_misses: 20,
            resolution_time_ms: 5000,
            conflicts_detected: 3,
            circular_dependencies: 1,
            transitive_dependencies: 50,
            version_conflicts: 2,
            security_vulnerabilities: 0,
            average_depth: 3.5,
            max_depth: 8,
        };

        assert_eq!(stats.total_dependencies, 100);
        assert_eq!(stats.resolved_dependencies, 95);
        assert_eq!(stats.failed_resolutions, 5);
        assert_eq!(stats.conflicts_detected, 3);
        assert_eq!(stats.circular_dependencies, 1);
        assert_eq!(stats.transitive_dependencies, 50);
        assert_eq!(stats.version_conflicts, 2);
        assert_eq!(stats.average_depth, 3.5);
        assert_eq!(stats.max_depth, 8);
    }

    #[test]
    fn test_enhanced_version_constraint_equality() {
        let exact1 = VersionConstraint::Exact(Version::parse("1.0.0").unwrap());
        let exact2 = VersionConstraint::Exact(Version::parse("1.0.0").unwrap());
        let exact3 = VersionConstraint::Exact(Version::parse("2.0.0").unwrap());

        assert_eq!(exact1, exact2);
        assert_ne!(exact1, exact3);

        let caret1 = VersionConstraint::Caret(Version::parse("1.0.0").unwrap());
        let caret2 = VersionConstraint::Caret(Version::parse("1.0.0").unwrap());
        let tilde1 = VersionConstraint::Tilde(Version::parse("1.0.0").unwrap());

        assert_eq!(caret1, caret2);
        assert_ne!(caret1, tilde1);
    }

    #[test]
    fn test_enhanced_conflict_resolution_methods() {
        let methods = vec![
            ConflictResolutionMethod::Automatic,
            ConflictResolutionMethod::Prompt,
            ConflictResolutionMethod::Skip,
            ConflictResolutionMethod::Fail,
            ConflictResolutionMethod::HighestCompatible,
            ConflictResolutionMethod::LowestCompatible,
            ConflictResolutionMethod::MostRecent,
            ConflictResolutionMethod::MostStable,
            ConflictResolutionMethod::HighestPriority,
            ConflictResolutionMethod::BestCompatibility,
        ];

        assert_eq!(methods.len(), 10);
        assert_ne!(methods[0], methods[1]);
    }

    #[test]
    fn test_enhanced_dependency_spec_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("category".to_string(), "core".to_string());
        metadata.insert("stability".to_string(), "stable".to_string());
        metadata.insert("maintainer".to_string(), "team".to_string());

        let spec = DependencySpec {
            path: "crates/core".to_string(),
            version_constraint: VersionConstraint::Latest,
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "latest".to_string(),
            scope_path: "crates/app".to_string(),
            priority: 10,
            optional: false,
            alternatives: vec!["crates/core-alt".to_string(), "crates/core-legacy".to_string()],
            metadata: metadata.clone(),
        };

        assert_eq!(spec.metadata.len(), 3);
        assert_eq!(spec.metadata.get("category").unwrap(), "core");
        assert_eq!(spec.metadata.get("stability").unwrap(), "stable");
        assert_eq!(spec.metadata.get("maintainer").unwrap(), "team");
        assert_eq!(spec.alternatives.len(), 2);
        assert_eq!(spec.priority, 10);
    }

    #[test]
    fn test_enhanced_resolution_result() {
        let mut dependencies = HashMap::new();
        dependencies.insert("dep1".to_string(), LockedDependency::new("1.0.0", "dep1", DependencyType::Required));

        let mut dependency_graph = HashMap::new();
        dependency_graph.insert("dep1".to_string(), DependencyNode {
            path: "dep1".to_string(),
            dependencies: Vec::new(),
            version: Some(Version::parse("1.0.0").unwrap()),
            resolved: true,
            depth: 1,
            is_root: true,
            priority: 1,
            resolved_at: Some(Utc::now()),
            metadata: HashMap::new(),
        });

        let result = ResolutionResult {
            dependencies,
            conflicts: Vec::new(),
            stats: ResolutionStats::default(),
            successful: true,
            dependency_graph,
            warnings: vec!["Warning 1".to_string()],
            recommendations: vec!["Recommendation 1".to_string()],
            performance_metrics: PerformanceMetrics::default(),
        };

        assert!(result.successful);
        assert_eq!(result.dependencies.len(), 1);
        assert_eq!(result.conflicts.len(), 0);
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.recommendations.len(), 1);
        assert_eq!(result.dependency_graph.len(), 1);
    }
} 