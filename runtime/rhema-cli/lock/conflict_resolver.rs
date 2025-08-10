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
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use semver::{Version, VersionReq};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::path::Path;
use std::time::Instant;

/// Advanced conflict resolution strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictResolutionStrategy {
    /// Latest compatible version resolution
    LatestCompatible,
    /// Pinned version enforcement
    PinnedVersion,
    /// Manual conflict resolution workflows
    ManualResolution,
    /// Automatic conflict detection and reporting
    AutomaticDetection,
    /// Conflict resolution history tracking
    HistoryTracking,
    /// Smart version selection based on compatibility scores
    SmartSelection,
    /// Conservative version selection (prefer stable, tested versions)
    Conservative,
    /// Aggressive version selection (prefer latest features)
    Aggressive,
    /// Hybrid approach combining multiple strategies
    Hybrid,
}

/// Conflict resolution configuration
#[derive(Debug, Clone)]
pub struct ConflictResolutionConfig {
    /// Primary resolution strategy
    pub primary_strategy: ConflictResolutionStrategy,
    /// Fallback strategies in order of preference
    pub fallback_strategies: Vec<ConflictResolutionStrategy>,
    /// Whether to enable automatic conflict detection
    pub enable_auto_detection: bool,
    /// Whether to track resolution history
    pub track_history: bool,
    /// Maximum resolution attempts
    pub max_attempts: usize,
    /// Whether to allow user prompts for manual resolution
    pub allow_user_prompts: bool,
    /// Whether to prefer stable versions
    pub prefer_stable: bool,
    /// Whether to enforce pinned versions strictly
    pub strict_pinning: bool,
    /// Compatibility threshold for smart selection (0.0-1.0)
    pub compatibility_threshold: f64,
    /// Whether to enable parallel resolution
    pub parallel_resolution: bool,
    /// Maximum parallel resolution threads
    pub max_parallel_threads: usize,
    /// Timeout for resolution operations (in seconds)
    pub timeout_seconds: u64,
}

/// Conflict resolution result with detailed information
#[derive(Debug, Clone)]
pub struct ConflictResolutionResult {
    /// Resolved dependencies
    pub resolved_dependencies: HashMap<String, LockedDependency>,
    /// Detected conflicts
    pub detected_conflicts: Vec<DependencyConflict>,
    /// Resolution actions taken
    pub resolution_actions: Vec<ResolutionAction>,
    /// Resolution statistics
    pub stats: ConflictResolutionStats,
    /// Whether resolution was successful
    pub successful: bool,
    /// Resolution warnings
    pub warnings: Vec<String>,
    /// Resolution recommendations
    pub recommendations: Vec<String>,
    /// Performance metrics
    pub performance_metrics: ConflictPerformanceMetrics,
}

/// Individual dependency conflict with enhanced analysis
#[derive(Debug, Clone)]
pub struct DependencyConflict {
    /// Name of the conflicting dependency
    pub dependency_name: String,
    /// Conflicting version requirements
    pub requirements: Vec<ConflictRequirement>,
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
    /// Compatibility scores for potential resolutions
    pub compatibility_scores: HashMap<Version, f64>,
    /// Whether this conflict was auto-resolved
    pub auto_resolved: bool,
    /// Resolution timestamp
    pub resolved_at: Option<DateTime<Utc>>,
    /// Resolution method used
    pub resolution_method: Option<ConflictResolutionStrategy>,
}

/// Individual conflict requirement
#[derive(Debug, Clone)]
pub struct ConflictRequirement {
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

/// Enhanced version constraint specification
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

/// Conflict types with enhanced categorization
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Pinned version conflict
    PinnedVersionConflict,
    /// Compatibility score below threshold
    LowCompatibilityScore,
    /// Breaking change detected
    BreakingChange,
    /// Deprecated version
    DeprecatedVersion,
}

/// Conflict severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

/// Resolution action taken during conflict resolution
#[derive(Debug, Clone)]
pub struct ResolutionAction {
    /// Action type
    pub action_type: ResolutionActionType,
    /// Dependency name
    pub dependency_name: String,
    /// Previous version (if applicable)
    pub previous_version: Option<Version>,
    /// New version
    pub new_version: Version,
    /// Reason for the action
    pub reason: String,
    /// Timestamp when action was taken
    pub timestamp: DateTime<Utc>,
    /// Strategy used for this action
    pub strategy: ConflictResolutionStrategy,
    /// Whether this was an automatic action
    pub automatic: bool,
}

/// Types of resolution actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionActionType {
    /// Version upgrade
    Upgrade,
    /// Version downgrade
    Downgrade,
    /// Version pinning
    Pin,
    /// Version unpinning
    Unpin,
    /// Dependency removal
    Remove,
    /// Dependency addition
    Add,
    /// Constraint modification
    ModifyConstraint,
    /// Alternative dependency selection
    SelectAlternative,
}

/// Conflict resolution statistics
#[derive(Debug, Clone)]
pub struct ConflictResolutionStats {
    /// Total conflicts detected
    pub total_conflicts: usize,
    /// Conflicts auto-resolved
    pub auto_resolved: usize,
    /// Conflicts requiring manual resolution
    pub manual_resolution_required: usize,
    /// Conflicts that could not be resolved
    pub unresolved_conflicts: usize,
    /// Resolution attempts made
    pub resolution_attempts: usize,
    /// Average resolution time per conflict (ms)
    pub average_resolution_time_ms: f64,
    /// Total resolution time (ms)
    pub total_resolution_time_ms: u64,
    /// Number of version upgrades
    pub version_upgrades: usize,
    /// Number of version downgrades
    pub version_downgrades: usize,
    /// Number of pinned versions enforced
    pub pinned_versions_enforced: usize,
    /// Number of compatibility checks performed
    pub compatibility_checks: usize,
    /// Cache hit rate for resolution decisions
    pub cache_hit_rate: f64,
}

/// Performance metrics for conflict resolution
#[derive(Debug, Clone)]
pub struct ConflictPerformanceMetrics {
    /// Total resolution time
    pub total_time_ms: u64,
    /// Time spent on conflict detection
    pub detection_time_ms: u64,
    /// Time spent on resolution strategy execution
    pub strategy_execution_time_ms: u64,
    /// Time spent on compatibility scoring
    pub compatibility_scoring_time_ms: u64,
    /// Time spent on user interaction (if any)
    pub user_interaction_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Number of parallel operations
    pub parallel_operations: usize,
    /// Number of cache operations
    pub cache_operations: usize,
}

/// Resolution history entry
#[derive(Debug, Clone)]
pub struct ResolutionHistoryEntry {
    /// Timestamp of the resolution
    pub timestamp: DateTime<Utc>,
    /// Dependency name
    pub dependency_name: String,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Resolution strategy used
    pub strategy: ConflictResolutionStrategy,
    /// Previous version
    pub previous_version: Option<Version>,
    /// New version
    pub new_version: Version,
    /// Whether resolution was successful
    pub successful: bool,
    /// Resolution notes
    pub notes: Option<String>,
    /// User who performed the resolution (if manual)
    pub resolved_by: Option<String>,
}

impl Default for ConflictResolutionConfig {
    fn default() -> Self {
        Self {
            primary_strategy: ConflictResolutionStrategy::LatestCompatible,
            fallback_strategies: vec![
                ConflictResolutionStrategy::Conservative,
                ConflictResolutionStrategy::ManualResolution,
            ],
            enable_auto_detection: true,
            track_history: true,
            max_attempts: 3,
            allow_user_prompts: true,
            prefer_stable: true,
            strict_pinning: false,
            compatibility_threshold: 0.8,
            parallel_resolution: true,
            max_parallel_threads: 4,
            timeout_seconds: 30,
        }
    }
}

impl Default for ConflictResolutionStats {
    fn default() -> Self {
        Self {
            total_conflicts: 0,
            auto_resolved: 0,
            manual_resolution_required: 0,
            unresolved_conflicts: 0,
            resolution_attempts: 0,
            average_resolution_time_ms: 0.0,
            total_resolution_time_ms: 0,
            version_upgrades: 0,
            version_downgrades: 0,
            pinned_versions_enforced: 0,
            compatibility_checks: 0,
            cache_hit_rate: 0.0,
        }
    }
}

impl Default for ConflictPerformanceMetrics {
    fn default() -> Self {
        Self {
            total_time_ms: 0,
            detection_time_ms: 0,
            strategy_execution_time_ms: 0,
            compatibility_scoring_time_ms: 0,
            user_interaction_time_ms: 0,
            memory_usage_bytes: 0,
            parallel_operations: 0,
            cache_operations: 0,
        }
    }
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

/// Main conflict resolver for advanced dependency conflict resolution
pub struct ConflictResolver {
    config: ConflictResolutionConfig,
    resolution_history: Vec<ResolutionHistoryEntry>,
    compatibility_cache: HashMap<String, HashMap<Version, f64>>,
    conflict_cache: HashMap<String, Vec<DependencyConflict>>,
    version_cache: HashMap<String, Vec<Version>>,
    performance_metrics: ConflictPerformanceMetrics,
    resolution_stats: ConflictResolutionStats,
}

impl ConflictResolver {
    /// Create a new conflict resolver with default configuration
    pub fn new() -> Self {
        Self {
            config: ConflictResolutionConfig::default(),
            resolution_history: Vec::new(),
            compatibility_cache: HashMap::new(),
            conflict_cache: HashMap::new(),
            version_cache: HashMap::new(),
            performance_metrics: ConflictPerformanceMetrics::default(),
            resolution_stats: ConflictResolutionStats::default(),
        }
    }

    /// Create a new conflict resolver with custom configuration
    pub fn with_config(config: ConflictResolutionConfig) -> Self {
        Self {
            config,
            resolution_history: Vec::new(),
            compatibility_cache: HashMap::new(),
            conflict_cache: HashMap::new(),
            version_cache: HashMap::new(),
            performance_metrics: ConflictPerformanceMetrics::default(),
            resolution_stats: ConflictResolutionStats::default(),
        }
    }

    /// Resolve conflicts in a set of dependencies
    pub fn resolve_conflicts(
        &mut self,
        dependencies: &[DependencySpec],
        repo_path: &Path,
    ) -> RhemaResult<ConflictResolutionResult> {
        let start_time = Instant::now();
        let mut result = ConflictResolutionResult {
            resolved_dependencies: HashMap::new(),
            detected_conflicts: Vec::new(),
            resolution_actions: Vec::new(),
            stats: ConflictResolutionStats::default(),
            successful: false,
            warnings: Vec::new(),
            recommendations: Vec::new(),
            performance_metrics: ConflictPerformanceMetrics::default(),
        };

        info!("Starting conflict resolution for {} dependencies", dependencies.len());

        // Step 1: Detect conflicts
        let detection_start = Instant::now();
        let conflicts = self.detect_conflicts(dependencies, repo_path)?;
        result.detected_conflicts = conflicts;
        result.stats.total_conflicts = result.detected_conflicts.len();
        self.performance_metrics.detection_time_ms = detection_start.elapsed().as_millis() as u64;

        if result.detected_conflicts.is_empty() {
            info!("No conflicts detected, resolution complete");
            result.successful = true;
            return Ok(result);
        }

        // Step 2: Resolve conflicts using primary strategy
        let strategy_start = Instant::now();
        let primary_strategy = self.config.primary_strategy.clone();
        self.resolve_conflicts_with_strategy(
            &mut result,
            &primary_strategy,
            dependencies,
            repo_path,
        )?;
        self.performance_metrics.strategy_execution_time_ms = strategy_start.elapsed().as_millis() as u64;

        // Step 3: Apply fallback strategies if needed
        if !result.successful && !self.config.fallback_strategies.is_empty() {
            let fallback_strategies = self.config.fallback_strategies.clone();
            for fallback_strategy in &fallback_strategies {
                if result.successful {
                    break;
                }
                let fallback_start = Instant::now();
                self.resolve_conflicts_with_strategy(
                    &mut result,
                    fallback_strategy,
                    dependencies,
                    repo_path,
                )?;
                self.performance_metrics.strategy_execution_time_ms += fallback_start.elapsed().as_millis() as u64;
            }
        }

        // Step 4: Update performance metrics
        self.performance_metrics.total_time_ms = start_time.elapsed().as_millis() as u64;
        result.performance_metrics = self.performance_metrics.clone();
        result.stats = self.resolution_stats.clone();

        // Step 5: Generate recommendations
        self.generate_recommendations(&mut result);

        info!(
            "Conflict resolution completed: {} conflicts resolved, {} remaining",
            result.stats.auto_resolved,
            result.stats.unresolved_conflicts
        );

        Ok(result)
    }

    /// Detect conflicts in dependencies
    fn detect_conflicts(
        &mut self,
        dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<Vec<DependencyConflict>> {
        let mut conflicts = Vec::new();
        let mut dependency_groups: HashMap<String, Vec<&DependencySpec>> = HashMap::new();

        // Group dependencies by name
        for dep in dependencies {
            dependency_groups
                .entry(dep.path.clone())
                .or_insert_with(Vec::new)
                .push(dep);
        }

        // Check for conflicts in each dependency group
        for (dep_name, deps) in dependency_groups {
            if deps.len() > 1 {
                // Multiple requirements for the same dependency - check for conflicts
                if let Some(conflict) = self.check_dependency_conflicts(&dep_name, &deps) {
                    conflicts.push(conflict);
                }
            }
        }

        // Check for circular dependencies
        let circular_deps = self.detect_circular_dependencies(dependencies)?;
        for cycle in circular_deps {
            let conflict = DependencyConflict {
                dependency_name: cycle.join(" -> "),
                requirements: Vec::new(),
                conflict_type: ConflictType::CircularDependency,
                suggested_resolution: None,
                severity: ConflictSeverity::Critical,
                description: format!("Circular dependency detected: {}", cycle.join(" -> ")),
                affected_scopes: cycle,
                recommendations: vec![
                    "Remove one of the circular dependencies".to_string(),
                    "Restructure dependencies to break the cycle".to_string(),
                ],
                compatibility_scores: HashMap::new(),
                auto_resolved: false,
                resolved_at: None,
                resolution_method: None,
            };
            conflicts.push(conflict);
        }

        Ok(conflicts)
    }

    /// Check for conflicts in a specific dependency
    fn check_dependency_conflicts(
        &self,
        dep_name: &str,
        deps: &[&DependencySpec],
    ) -> Option<DependencyConflict> {
        let mut requirements = Vec::new();
        let mut affected_scopes = Vec::new();

        for dep in deps {
            let requirement = ConflictRequirement {
                scope_path: dep.scope_path.clone(),
                constraint: dep.version_constraint.clone(),
                dependency_type: dep.dependency_type.clone(),
                priority: dep.priority,
                optional: dep.optional,
                original_constraint: dep.original_constraint.clone(),
            };
            requirements.push(requirement);
            affected_scopes.push(dep.scope_path.clone());
        }

        // Check for version incompatibilities
        if self.has_version_conflicts(&requirements) {
            let compatibility_scores = self.calculate_compatibility_scores(dep_name, &requirements);
            let suggested_resolution = self.suggest_resolution(&requirements, &compatibility_scores);

            return Some(DependencyConflict {
                dependency_name: dep_name.to_string(),
                requirements: requirements.clone(),
                conflict_type: ConflictType::VersionIncompatibility,
                suggested_resolution,
                severity: self.calculate_conflict_severity(&requirements),
                description: format!("Version conflict detected for dependency '{}'", dep_name),
                affected_scopes,
                recommendations: self.generate_conflict_recommendations(&requirements),
                compatibility_scores,
                auto_resolved: false,
                resolved_at: None,
                resolution_method: None,
            });
        }

        None
    }

    /// Check if there are version conflicts in requirements
    fn has_version_conflicts(&self, requirements: &[ConflictRequirement]) -> bool {
        if requirements.len() <= 1 {
            return false;
        }

        // Check if any two requirements have incompatible version constraints
        for i in 0..requirements.len() {
            for j in (i + 1)..requirements.len() {
                if !self.constraints_are_compatible(&requirements[i].constraint, &requirements[j].constraint) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if two version constraints are compatible
    fn constraints_are_compatible(&self, constraint1: &VersionConstraint, constraint2: &VersionConstraint) -> bool {
        match (constraint1, constraint2) {
            (VersionConstraint::Exact(v1), VersionConstraint::Exact(v2)) => v1 == v2,
            (VersionConstraint::Exact(v), VersionConstraint::Range(req)) | 
            (VersionConstraint::Range(req), VersionConstraint::Exact(v)) => req.matches(v),
            (VersionConstraint::Range(_req1), VersionConstraint::Range(_req2)) => {
                // Check if there's any overlap between the ranges
                // This is a simplified check - in practice, you'd want more sophisticated range intersection logic
                true // Simplified for now
            }
            _ => true, // Other constraint types are considered compatible for now
        }
    }

    /// Calculate compatibility scores for potential resolutions
    fn calculate_compatibility_scores(
        &self,
        dep_name: &str,
        requirements: &[ConflictRequirement],
    ) -> HashMap<Version, f64> {
        let mut scores = HashMap::new();
        
        // Get available versions (this would typically come from a version registry)
        let available_versions = self.get_available_versions(dep_name);
        
        for version in available_versions {
            let mut total_score = 0.0;
            let mut valid_requirements = 0;
            
            for requirement in requirements {
                if self.version_satisfies_constraint(&version, &requirement.constraint) {
                    let score = self.calculate_requirement_score(&version, requirement);
                    total_score += score;
                    valid_requirements += 1;
                }
            }
            
            if valid_requirements > 0 {
                scores.insert(version, total_score / valid_requirements as f64);
            }
        }
        
        scores
    }

    /// Check if a version satisfies a constraint
    fn version_satisfies_constraint(&self, version: &Version, constraint: &VersionConstraint) -> bool {
        match constraint {
            VersionConstraint::Exact(req_version) => version == req_version,
            VersionConstraint::Range(req) => req.matches(version),
            VersionConstraint::Latest => true, // Latest always satisfies
            VersionConstraint::Earliest => true, // Earliest always satisfies
            VersionConstraint::Pinned(pinned_version) => version == pinned_version,
            VersionConstraint::Caret(base_version) => {
                let req = VersionReq::parse(&format!("^{}", base_version)).unwrap_or_default();
                req.matches(version)
            }
            VersionConstraint::Tilde(base_version) => {
                let req = VersionReq::parse(&format!("~{}", base_version)).unwrap_or_default();
                req.matches(version)
            }
            VersionConstraint::Wildcard(base_version) => {
                let req = VersionReq::parse(&format!("{}.*", base_version)).unwrap_or_default();
                req.matches(version)
            }
            VersionConstraint::Prerelease(_) => version.pre.is_empty(), // Prefer stable versions
            VersionConstraint::Development(_) => version.pre.is_empty(), // Prefer stable versions
        }
    }

    /// Calculate score for a specific requirement
    fn calculate_requirement_score(&self, version: &Version, requirement: &ConflictRequirement) -> f64 {
        let mut score = 1.0;
        
        // Adjust score based on priority
        score *= requirement.priority as f64 / 10.0;
        
        // Adjust score based on dependency type
        match requirement.dependency_type {
            DependencyType::Required => score *= 1.0,
            DependencyType::Optional => score *= 0.8,
            DependencyType::Peer => score *= 0.9,
            DependencyType::Development => score *= 0.7,
            DependencyType::Build => score *= 0.6,
        }
        
        // Prefer stable versions if configured
        if self.config.prefer_stable && !version.pre.is_empty() {
            score *= 0.5;
        }
        
        score
    }

    /// Suggest a resolution based on compatibility scores
    fn suggest_resolution(
        &self,
        requirements: &[ConflictRequirement],
        compatibility_scores: &HashMap<Version, f64>,
    ) -> Option<Version> {
        if compatibility_scores.is_empty() {
            return None;
        }

        // Find the version with the highest compatibility score
        compatibility_scores
            .iter()
            .max_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(version, _)| version.clone())
    }

    /// Calculate conflict severity based on requirements
    fn calculate_conflict_severity(&self, requirements: &[ConflictRequirement]) -> ConflictSeverity {
        let has_required = requirements.iter().any(|r| r.dependency_type == DependencyType::Required);
        let has_high_priority = requirements.iter().any(|r| r.priority >= 8);
        
        if has_required && has_high_priority {
            ConflictSeverity::Critical
        } else if has_required {
            ConflictSeverity::High
        } else if has_high_priority {
            ConflictSeverity::Medium
        } else {
            ConflictSeverity::Low
        }
    }

    /// Generate recommendations for conflict resolution
    fn generate_conflict_recommendations(&self, requirements: &[ConflictRequirement]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Check for pinned version conflicts
        let pinned_requirements: Vec<_> = requirements
            .iter()
            .filter(|r| matches!(r.constraint, VersionConstraint::Pinned(_)))
            .collect();
        
        if pinned_requirements.len() > 1 {
            recommendations.push("Multiple pinned versions detected. Consider using a single pinned version.".to_string());
        }
        
        // Check for version range conflicts
        let range_requirements: Vec<_> = requirements
            .iter()
            .filter(|r| matches!(r.constraint, VersionConstraint::Range(_)))
            .collect();
        
        if range_requirements.len() > 1 {
            recommendations.push("Multiple version ranges detected. Consider consolidating to a single range.".to_string());
        }
        
        // General recommendations
        recommendations.push("Review dependency requirements and consider standardizing version constraints.".to_string());
        recommendations.push("Use semantic versioning to ensure compatibility.".to_string());
        
        recommendations
    }

    /// Get available versions for a dependency
    fn get_available_versions(&self, _dep_name: &str) -> Vec<Version> {
        // This would typically query a version registry or package manager
        // For now, return some example versions
        vec![
            Version::parse("1.0.0").unwrap(),
            Version::parse("1.1.0").unwrap(),
            Version::parse("1.2.0").unwrap(),
            Version::parse("2.0.0").unwrap(),
            Version::parse("2.1.0").unwrap(),
        ]
    }

    /// Detect circular dependencies
    fn detect_circular_dependencies(&self, dependencies: &[DependencySpec]) -> RhemaResult<Vec<Vec<String>>> {
        // Build dependency graph
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        
        for dep in dependencies {
            graph
                .entry(dep.scope_path.clone())
                .or_insert_with(Vec::new)
                .push(dep.path.clone());
        }
        
        // Detect cycles using DFS
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node in graph.keys() {
            if !visited.contains(node) {
                let mut path = Vec::new();
                if self.has_cycle_dfs(&graph, node, &mut visited, &mut rec_stack, &mut path) {
                    cycles.push(path);
                }
            }
        }
        
        Ok(cycles)
    }

    /// DFS helper for cycle detection
    fn has_cycle_dfs(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if self.has_cycle_dfs(graph, neighbor, visited, rec_stack, path) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    // Found a back edge - cycle detected
                    return true;
                }
            }
        }
        
        rec_stack.remove(node);
        path.pop();
        false
    }

    /// Resolve conflicts using a specific strategy
    fn resolve_conflicts_with_strategy(
        &mut self,
        result: &mut ConflictResolutionResult,
        strategy: &ConflictResolutionStrategy,
        dependencies: &[DependencySpec],
        repo_path: &Path,
    ) -> RhemaResult<()> {
        match strategy {
            ConflictResolutionStrategy::LatestCompatible => {
                self.resolve_latest_compatible(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::PinnedVersion => {
                self.resolve_pinned_version(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::ManualResolution => {
                self.resolve_manual(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::AutomaticDetection => {
                self.resolve_automatic(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::HistoryTracking => {
                self.resolve_with_history(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::SmartSelection => {
                self.resolve_smart_selection(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::Conservative => {
                self.resolve_conservative(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::Aggressive => {
                self.resolve_aggressive(result, dependencies, repo_path)?;
            }
            ConflictResolutionStrategy::Hybrid => {
                self.resolve_hybrid(result, dependencies, repo_path)?;
            }
        }
        
        Ok(())
    }

    /// Generate recommendations for the resolution result
    fn generate_recommendations(&self, result: &mut ConflictResolutionResult) {
        if result.stats.unresolved_conflicts > 0 {
            result.recommendations.push(
                "Some conflicts could not be automatically resolved. Consider manual intervention.".to_string()
            );
        }
        
        if result.stats.version_upgrades > 0 {
            result.recommendations.push(
                "Multiple version upgrades were performed. Test thoroughly to ensure compatibility.".to_string()
            );
        }
        
        if result.stats.pinned_versions_enforced > 0 {
            result.recommendations.push(
                "Pinned versions were enforced. Consider reviewing version constraints.".to_string()
            );
        }
        
        result.recommendations.push(
            "Regular dependency updates can help prevent future conflicts.".to_string()
        );
    }

    /// Resolve conflicts using latest compatible version strategy
    fn resolve_latest_compatible(
        &mut self,
        result: &mut ConflictResolutionResult,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying latest compatible version resolution strategy");
        
        for conflict in &mut result.detected_conflicts {
            if let Some(_suggested_version) = &conflict.suggested_resolution {
                // Find the latest version that satisfies all requirements
                let available_versions = self.get_available_versions(&conflict.dependency_name);
                let compatible_versions: Vec<_> = available_versions
                    .into_iter()
                    .filter(|v| {
                        conflict.requirements.iter().all(|req| {
                            self.version_satisfies_constraint(v, &req.constraint)
                        })
                    })
                    .collect();
                
                if let Some(latest_compatible) = compatible_versions.iter().max() {
                    let action = ResolutionAction {
                        action_type: ResolutionActionType::Upgrade,
                        dependency_name: conflict.dependency_name.clone(),
                        previous_version: None, // We don't have previous version info here
                        new_version: latest_compatible.clone(),
                        reason: "Latest compatible version selected".to_string(),
                        timestamp: Utc::now(),
                        strategy: ConflictResolutionStrategy::LatestCompatible,
                        automatic: true,
                    };
                    
                    result.resolution_actions.push(action);
                    conflict.auto_resolved = true;
                    conflict.resolved_at = Some(Utc::now());
                    conflict.resolution_method = Some(ConflictResolutionStrategy::LatestCompatible);
                    
                    // Create resolved dependency
                    let resolved_dep = LockedDependency::new(
                        &latest_compatible.to_string(),
                        &conflict.dependency_name,
                        DependencyType::Required, // Default to required
                    );
                    result.resolved_dependencies.insert(conflict.dependency_name.clone(), resolved_dep);
                    
                    self.resolution_stats.auto_resolved += 1;
                    self.resolution_stats.version_upgrades += 1;
                }
            }
        }
        
        result.successful = result.detected_conflicts.iter().all(|c| c.auto_resolved);
        Ok(())
    }

    /// Resolve conflicts using pinned version enforcement
    fn resolve_pinned_version(
        &mut self,
        result: &mut ConflictResolutionResult,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying pinned version enforcement strategy");
        
        for conflict in &mut result.detected_conflicts {
            // Find pinned version requirements
            let pinned_requirements: Vec<_> = conflict.requirements
                .iter()
                .filter(|r| matches!(r.constraint, VersionConstraint::Pinned(_)))
                .collect();
            
            if let Some(pinned_req) = pinned_requirements.first() {
                if let VersionConstraint::Pinned(version) = &pinned_req.constraint {
                    let action = ResolutionAction {
                        action_type: ResolutionActionType::Pin,
                        dependency_name: conflict.dependency_name.clone(),
                        previous_version: None,
                        new_version: version.clone(),
                        reason: "Pinned version enforced".to_string(),
                        timestamp: Utc::now(),
                        strategy: ConflictResolutionStrategy::PinnedVersion,
                        automatic: true,
                    };
                    
                    result.resolution_actions.push(action);
                    conflict.auto_resolved = true;
                    conflict.resolved_at = Some(Utc::now());
                    conflict.resolution_method = Some(ConflictResolutionStrategy::PinnedVersion);
                    
                    // Create resolved dependency
                    let resolved_dep = LockedDependency::new(
                        &version.to_string(),
                        &conflict.dependency_name,
                        pinned_req.dependency_type.clone(),
                    );
                    result.resolved_dependencies.insert(conflict.dependency_name.clone(), resolved_dep);
                    
                    self.resolution_stats.auto_resolved += 1;
                    self.resolution_stats.pinned_versions_enforced += 1;
                }
            }
        }
        
        result.successful = result.detected_conflicts.iter().all(|c| c.auto_resolved);
        Ok(())
    }

    /// Resolve conflicts using manual resolution workflow
    fn resolve_manual(
        &mut self,
        result: &mut ConflictResolutionResult,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying manual resolution workflow");
        
        // Mark conflicts as requiring manual resolution
        for conflict in &mut result.detected_conflicts {
            if !conflict.auto_resolved {
                self.resolution_stats.manual_resolution_required += 1;
                result.warnings.push(format!(
                    "Manual resolution required for dependency '{}': {}",
                    conflict.dependency_name, conflict.description
                ));
            }
        }
        
        // In a real implementation, this would prompt the user for input
        // For now, we'll just mark that manual resolution is needed
        result.successful = false;
        Ok(())
    }

    /// Resolve conflicts using automatic detection and reporting
    fn resolve_automatic(
        &mut self,
        result: &mut ConflictResolutionResult,
        dependencies: &[DependencySpec],
        repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying automatic detection and reporting strategy");
        
        // This strategy focuses on detecting and reporting conflicts
        // rather than resolving them automatically
        for conflict in &mut result.detected_conflicts {
            // Generate detailed conflict report
            let report = self.generate_conflict_report(conflict, dependencies, repo_path)?;
            result.warnings.push(report);
            
            // Mark as requiring attention
            self.resolution_stats.unresolved_conflicts += 1;
        }
        
        result.successful = false; // Automatic detection doesn't resolve conflicts
        Ok(())
    }

    /// Resolve conflicts using history tracking
    fn resolve_with_history(
        &mut self,
        result: &mut ConflictResolutionResult,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying history tracking resolution strategy");
        
        for conflict in &mut result.detected_conflicts {
            // Look for similar conflicts in history
            if let Some(historical_resolution) = self.find_historical_resolution(&conflict.dependency_name) {
                let action = ResolutionAction {
                    action_type: ResolutionActionType::Upgrade,
                    dependency_name: conflict.dependency_name.clone(),
                    previous_version: None,
                    new_version: historical_resolution.new_version.clone(),
                    reason: format!("Historical resolution applied: {}", historical_resolution.notes.as_ref().unwrap_or(&"".to_string())),
                    timestamp: Utc::now(),
                    strategy: ConflictResolutionStrategy::HistoryTracking,
                    automatic: true,
                };
                
                result.resolution_actions.push(action);
                conflict.auto_resolved = true;
                conflict.resolved_at = Some(Utc::now());
                conflict.resolution_method = Some(ConflictResolutionStrategy::HistoryTracking);
                
                // Create resolved dependency
                let resolved_dep = LockedDependency::new(
                    &historical_resolution.new_version.to_string(),
                    &conflict.dependency_name,
                    DependencyType::Required,
                );
                result.resolved_dependencies.insert(conflict.dependency_name.clone(), resolved_dep);
                
                self.resolution_stats.auto_resolved += 1;
            } else {
                self.resolution_stats.unresolved_conflicts += 1;
            }
        }
        
        result.successful = result.detected_conflicts.iter().all(|c| c.auto_resolved);
        Ok(())
    }

    /// Resolve conflicts using smart selection based on compatibility scores
    fn resolve_smart_selection(
        &mut self,
        result: &mut ConflictResolutionResult,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying smart selection resolution strategy");
        
        for conflict in &mut result.detected_conflicts {
            if let Some(best_version) = self.select_best_version_by_compatibility(&conflict) {
                let action = ResolutionAction {
                    action_type: ResolutionActionType::Upgrade,
                    dependency_name: conflict.dependency_name.clone(),
                    previous_version: None,
                    new_version: best_version.clone(),
                    reason: "Smart selection based on compatibility scores".to_string(),
                    timestamp: Utc::now(),
                    strategy: ConflictResolutionStrategy::SmartSelection,
                    automatic: true,
                };
                
                result.resolution_actions.push(action);
                conflict.auto_resolved = true;
                conflict.resolved_at = Some(Utc::now());
                conflict.resolution_method = Some(ConflictResolutionStrategy::SmartSelection);
                
                // Create resolved dependency
                let resolved_dep = LockedDependency::new(
                    &best_version.to_string(),
                    &conflict.dependency_name,
                    DependencyType::Required,
                );
                result.resolved_dependencies.insert(conflict.dependency_name.clone(), resolved_dep);
                
                self.resolution_stats.auto_resolved += 1;
            } else {
                self.resolution_stats.unresolved_conflicts += 1;
            }
        }
        
        result.successful = result.detected_conflicts.iter().all(|c| c.auto_resolved);
        Ok(())
    }

    /// Resolve conflicts using conservative approach
    fn resolve_conservative(
        &mut self,
        result: &mut ConflictResolutionResult,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying conservative resolution strategy");
        
        for conflict in &mut result.detected_conflicts {
            // Prefer stable, well-tested versions
            let available_versions = self.get_available_versions(&conflict.dependency_name);
            let stable_versions: Vec<_> = available_versions
                .into_iter()
                .filter(|v| v.pre.is_empty() && v.build.is_empty()) // Stable versions only
                .collect();
            
            if let Some(conservative_version) = stable_versions.iter().max() {
                let action = ResolutionAction {
                    action_type: ResolutionActionType::Upgrade,
                    dependency_name: conflict.dependency_name.clone(),
                    previous_version: None,
                    new_version: conservative_version.clone(),
                    reason: "Conservative version selection (stable, tested)".to_string(),
                    timestamp: Utc::now(),
                    strategy: ConflictResolutionStrategy::Conservative,
                    automatic: true,
                };
                
                result.resolution_actions.push(action);
                conflict.auto_resolved = true;
                conflict.resolved_at = Some(Utc::now());
                conflict.resolution_method = Some(ConflictResolutionStrategy::Conservative);
                
                // Create resolved dependency
                let resolved_dep = LockedDependency::new(
                    &conservative_version.to_string(),
                    &conflict.dependency_name,
                    DependencyType::Required,
                );
                result.resolved_dependencies.insert(conflict.dependency_name.clone(), resolved_dep);
                
                self.resolution_stats.auto_resolved += 1;
            } else {
                self.resolution_stats.unresolved_conflicts += 1;
            }
        }
        
        result.successful = result.detected_conflicts.iter().all(|c| c.auto_resolved);
        Ok(())
    }

    /// Resolve conflicts using aggressive approach
    fn resolve_aggressive(
        &mut self,
        result: &mut ConflictResolutionResult,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying aggressive resolution strategy");
        
        for conflict in &mut result.detected_conflicts {
            // Prefer latest versions with latest features
            let available_versions = self.get_available_versions(&conflict.dependency_name);
            
            if let Some(latest_version) = available_versions.iter().max() {
                let action = ResolutionAction {
                    action_type: ResolutionActionType::Upgrade,
                    dependency_name: conflict.dependency_name.clone(),
                    previous_version: None,
                    new_version: latest_version.clone(),
                    reason: "Aggressive version selection (latest features)".to_string(),
                    timestamp: Utc::now(),
                    strategy: ConflictResolutionStrategy::Aggressive,
                    automatic: true,
                };
                
                result.resolution_actions.push(action);
                conflict.auto_resolved = true;
                conflict.resolved_at = Some(Utc::now());
                conflict.resolution_method = Some(ConflictResolutionStrategy::Aggressive);
                
                // Create resolved dependency
                let resolved_dep = LockedDependency::new(
                    &latest_version.to_string(),
                    &conflict.dependency_name,
                    DependencyType::Required,
                );
                result.resolved_dependencies.insert(conflict.dependency_name.clone(), resolved_dep);
                
                self.resolution_stats.auto_resolved += 1;
                self.resolution_stats.version_upgrades += 1;
            } else {
                self.resolution_stats.unresolved_conflicts += 1;
            }
        }
        
        result.successful = result.detected_conflicts.iter().all(|c| c.auto_resolved);
        Ok(())
    }

    /// Resolve conflicts using hybrid approach
    fn resolve_hybrid(
        &mut self,
        result: &mut ConflictResolutionResult,
        dependencies: &[DependencySpec],
        repo_path: &Path,
    ) -> RhemaResult<()> {
        info!("Applying hybrid resolution strategy");
        
        // Try multiple strategies in sequence
        let strategies = vec![
            ConflictResolutionStrategy::LatestCompatible,
            ConflictResolutionStrategy::Conservative,
            ConflictResolutionStrategy::SmartSelection,
        ];
        
        for strategy in strategies {
            if result.successful {
                break;
            }
            
            self.resolve_conflicts_with_strategy(result, &strategy, dependencies, repo_path)?;
        }
        
        Ok(())
    }

    /// Generate detailed conflict report
    fn generate_conflict_report(
        &self,
        conflict: &DependencyConflict,
        _dependencies: &[DependencySpec],
        _repo_path: &Path,
    ) -> RhemaResult<String> {
        let mut report = format!(
            "CONFLICT REPORT for '{}':\n",
            conflict.dependency_name
        );
        
        report.push_str(&format!("Type: {:?}\n", conflict.conflict_type));
        report.push_str(&format!("Severity: {:?}\n", conflict.severity));
        report.push_str(&format!("Description: {}\n", conflict.description));
        
        report.push_str("Requirements:\n");
        for req in &conflict.requirements {
            report.push_str(&format!("  - {}: {:?} (priority: {})\n", 
                req.scope_path, req.constraint, req.priority));
        }
        
        if let Some(suggested) = &conflict.suggested_resolution {
            report.push_str(&format!("Suggested resolution: {}\n", suggested));
        }
        
        report.push_str("Recommendations:\n");
        for rec in &conflict.recommendations {
            report.push_str(&format!("  - {}\n", rec));
        }
        
        Ok(report)
    }

    /// Find historical resolution for a dependency
    fn find_historical_resolution(&self, dependency_name: &str) -> Option<&ResolutionHistoryEntry> {
        self.resolution_history
            .iter()
            .filter(|entry| entry.dependency_name == dependency_name && entry.successful)
            .max_by_key(|entry| entry.timestamp)
    }

    /// Select best version based on compatibility scores
    fn select_best_version_by_compatibility(&self, conflict: &DependencyConflict) -> Option<Version> {
        if conflict.compatibility_scores.is_empty() {
            return None;
        }
        
        // Find version with highest compatibility score above threshold
        conflict.compatibility_scores
            .iter()
            .filter(|(_, score)| **score >= self.config.compatibility_threshold)
            .max_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(version, _)| version.clone())
    }

    /// Add resolution to history
    pub fn add_resolution_to_history(&mut self, entry: ResolutionHistoryEntry) {
        self.resolution_history.push(entry);
        
        // Keep only recent history (last 1000 entries)
        if self.resolution_history.len() > 1000 {
            self.resolution_history.remove(0);
        }
    }

    /// Get resolution history
    pub fn get_resolution_history(&self) -> &[ResolutionHistoryEntry] {
        &self.resolution_history
    }

    /// Clear resolution history
    pub fn clear_resolution_history(&mut self) {
        self.resolution_history.clear();
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &ConflictPerformanceMetrics {
        &self.performance_metrics
    }

    /// Get resolution statistics
    pub fn get_resolution_stats(&self) -> &ConflictResolutionStats {
        &self.resolution_stats
    }

    /// Clear caches
    pub fn clear_caches(&mut self) {
        self.compatibility_cache.clear();
        self.conflict_cache.clear();
        self.version_cache.clear();
    }
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new()
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_dependency_spec(
        path: &str,
        constraint: VersionConstraint,
        scope_path: &str,
        priority: u8,
    ) -> DependencySpec {
        DependencySpec {
            path: path.to_string(),
            version_constraint: constraint,
            dependency_type: DependencyType::Required,
            is_transitive: false,
            original_constraint: "1.0.0".to_string(),
            scope_path: scope_path.to_string(),
            priority,
            optional: false,
            alternatives: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_conflict_resolver_creation() {
        let resolver = ConflictResolver::new();
        assert_eq!(resolver.config.primary_strategy, ConflictResolutionStrategy::LatestCompatible);
        assert!(resolver.config.enable_auto_detection);
        assert!(resolver.config.track_history);
    }

    #[test]
    fn test_conflict_resolver_with_config() {
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
        
        // Test range compatibility
        let range1 = VersionConstraint::Range(VersionReq::parse(">=1.2.0,<2.0.0").unwrap());
        let range2 = VersionConstraint::Range(VersionReq::parse(">=1.3.0,<1.5.0").unwrap());
        let exact_in_range = VersionConstraint::Exact(Version::parse("1.3.0").unwrap());
        
        assert!(resolver.constraints_are_compatible(&range1, &exact_in_range));
        assert!(resolver.constraints_are_compatible(&range1, &range2));
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
        
        // Test range constraint
        let range = VersionConstraint::Range(VersionReq::parse(">=1.2.0,<2.0.0").unwrap());
        assert!(resolver.version_satisfies_constraint(&version, &range));
        
        let range_outside = VersionConstraint::Range(VersionReq::parse(">=2.0.0,<3.0.0").unwrap());
        assert!(!resolver.version_satisfies_constraint(&version, &range_outside));
        
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
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Exact(Version::parse("1.2.4").unwrap()),
            "scope2",
            7,
        );
        
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
        let dep1 = create_test_dependency_spec(
            "test-dep1",
            VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep2",
            VersionConstraint::Exact(Version::parse("1.2.4").unwrap()),
            "scope2",
            7,
        );
        
        let dependencies = vec![dep1, dep2];
        let conflicts = resolver.detect_conflicts(&dependencies, &repo_path).unwrap();
        
        assert_eq!(conflicts.len(), 0);
    }

    #[test]
    fn test_latest_compatible_resolution() {
        let mut resolver = ConflictResolver::new();
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create conflicting dependencies
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.5.0,<3.0.0").unwrap()),
            "scope2",
            7,
        );
        
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
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Pinned(Version::parse("1.2.3").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            "scope2",
            7,
        );
        
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
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Exact(Version::parse("1.2.4").unwrap()),
            "scope2",
            7,
        );
        
        let dependencies = vec![dep1, dep2];
        let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
        
        assert!(!result.successful);
        assert_eq!(result.stats.manual_resolution_required, 1);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_automatic_detection_strategy() {
        let mut resolver = ConflictResolver::with_config(ConflictResolutionConfig {
            primary_strategy: ConflictResolutionStrategy::AutomaticDetection,
            ..Default::default()
        });
        
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create conflicting dependencies
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Exact(Version::parse("1.2.3").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Exact(Version::parse("1.2.4").unwrap()),
            "scope2",
            7,
        );
        
        let dependencies = vec![dep1, dep2];
        let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
        
        assert!(!result.successful);
        assert_eq!(result.stats.unresolved_conflicts, 1);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_history_tracking() {
        let mut resolver = ConflictResolver::new();
        
        // Add a historical resolution
        let history_entry = ResolutionHistoryEntry {
            timestamp: Utc::now(),
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
    fn test_smart_selection_strategy() {
        let mut resolver = ConflictResolver::with_config(ConflictResolutionConfig {
            primary_strategy: ConflictResolutionStrategy::SmartSelection,
            compatibility_threshold: 0.8,
            ..Default::default()
        });
        
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create dependencies with different priorities
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            "scope1",
            9, // High priority
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.5.0,<3.0.0").unwrap()),
            "scope2",
            6, // Lower priority
        );
        
        let dependencies = vec![dep1, dep2];
        let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
        
        // Should resolve based on compatibility scores
        assert!(result.successful);
        assert_eq!(result.stats.auto_resolved, 1);
    }

    #[test]
    fn test_conservative_strategy() {
        let mut resolver = ConflictResolver::with_config(ConflictResolutionConfig {
            primary_strategy: ConflictResolutionStrategy::Conservative,
            prefer_stable: true,
            ..Default::default()
        });
        
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create dependencies
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.5.0,<3.0.0").unwrap()),
            "scope2",
            7,
        );
        
        let dependencies = vec![dep1, dep2];
        let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
        
        assert!(result.successful);
        assert_eq!(result.stats.auto_resolved, 1);
    }

    #[test]
    fn test_aggressive_strategy() {
        let mut resolver = ConflictResolver::with_config(ConflictResolutionConfig {
            primary_strategy: ConflictResolutionStrategy::Aggressive,
            prefer_stable: false,
            ..Default::default()
        });
        
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create dependencies
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.5.0,<3.0.0").unwrap()),
            "scope2",
            7,
        );
        
        let dependencies = vec![dep1, dep2];
        let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
        
        assert!(result.successful);
        assert_eq!(result.stats.auto_resolved, 1);
        assert_eq!(result.stats.version_upgrades, 1);
    }

    #[test]
    fn test_hybrid_strategy() {
        let mut resolver = ConflictResolver::with_config(ConflictResolutionConfig {
            primary_strategy: ConflictResolutionStrategy::Hybrid,
            fallback_strategies: vec![
                ConflictResolutionStrategy::Conservative,
                ConflictResolutionStrategy::ManualResolution,
            ],
            ..Default::default()
        });
        
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create dependencies
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.5.0,<3.0.0").unwrap()),
            "scope2",
            7,
        );
        
        let dependencies = vec![dep1, dep2];
        let result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
        
        // Hybrid should succeed by trying multiple strategies
        assert!(result.successful);
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
    fn test_compatibility_score_calculation() {
        let resolver = ConflictResolver::new();
        
        let req1 = ConflictRequirement {
            scope_path: "scope1".to_string(),
            constraint: VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            dependency_type: DependencyType::Required,
            priority: 8,
            optional: false,
            original_constraint: ">=1.0.0,<2.0.0".to_string(),
        };
        
        let req2 = ConflictRequirement {
            scope_path: "scope2".to_string(),
            constraint: VersionConstraint::Range(VersionReq::parse(">=1.5.0,<3.0.0").unwrap()),
            dependency_type: DependencyType::Optional,
            priority: 6,
            optional: true,
            original_constraint: ">=1.5.0,<3.0.0".to_string(),
        };
        
        let requirements = vec![req1, req2];
        let scores = resolver.calculate_compatibility_scores("test-dep", &requirements);
        
        assert!(!scores.is_empty());
        
        // Check that scores are reasonable
        for (_, score) in &scores {
            assert!(*score >= 0.0 && *score <= 1.0);
        }
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut resolver = ConflictResolver::new();
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create circular dependencies
        let dep1 = create_test_dependency_spec(
            "scope2",
            VersionConstraint::Exact(Version::parse("1.0.0").unwrap()),
            "scope1",
            8,
        );
        
        let dep2 = create_test_dependency_spec(
            "scope1",
            VersionConstraint::Exact(Version::parse("1.0.0").unwrap()),
            "scope2",
            7,
        );
        
        let dependencies = vec![dep1, dep2];
        let conflicts = resolver.detect_conflicts(&dependencies, &repo_path).unwrap();
        
        // Should detect circular dependency
        assert!(!conflicts.is_empty());
        assert!(conflicts.iter().any(|c| c.conflict_type == ConflictType::CircularDependency));
    }

    #[test]
    fn test_performance_metrics() {
        let mut resolver = ConflictResolver::new();
        let repo_path = PathBuf::from("/tmp/test");
        
        // Create some dependencies
        let dep1 = create_test_dependency_spec(
            "test-dep",
            VersionConstraint::Range(VersionReq::parse(">=1.0.0,<2.0.0").unwrap()),
            "scope1",
            8,
        );
        
        let dependencies = vec![dep1];
        let _result = resolver.resolve_conflicts(&dependencies, &repo_path).unwrap();
        
        let metrics = resolver.get_performance_metrics();
        assert!(metrics.total_time_ms > 0);
        assert!(metrics.detection_time_ms > 0);
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
            timestamp: Utc::now(),
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
            compatibility_scores: HashMap::new(),
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
} 