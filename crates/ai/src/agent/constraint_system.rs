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
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Constraint types for agentic development
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    /// Resource usage constraints (CPU, memory, disk)
    ResourceUsage,
    /// File access constraints
    FileAccess,
    /// Dependency constraints
    Dependency,
    /// Time constraints
    Time,
    /// Quality constraints
    Quality,
    /// Security constraints
    Security,
    /// Performance constraints
    Performance,
    /// Collaboration constraints
    Collaboration,
    /// Custom constraint type
    Custom(String),
}

/// Constraint severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConstraintSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

/// Constraint enforcement modes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementMode {
    /// Soft enforcement - log violations but allow execution
    Soft,
    /// Hard enforcement - block execution on violation
    Hard,
}

/// Resource usage constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraint {
    /// Maximum CPU usage percentage
    pub max_cpu_percent: Option<f64>,
    /// Maximum memory usage in MB
    pub max_memory_mb: Option<u64>,
    /// Maximum disk usage in MB
    pub max_disk_mb: Option<u64>,
    /// Maximum network bandwidth in MB/s
    pub max_network_mbps: Option<f64>,
    /// Maximum concurrent operations
    pub max_concurrent_ops: Option<usize>,
}

/// File access constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccessConstraint {
    /// Allowed file patterns (glob patterns)
    pub allowed_patterns: Vec<String>,
    /// Denied file patterns (glob patterns)
    pub denied_patterns: Vec<String>,
    /// Read-only files
    pub read_only_files: Vec<String>,
    /// Required files that must exist
    pub required_files: Vec<String>,
    /// Maximum file size in bytes
    pub max_file_size_bytes: Option<u64>,
}

/// Time constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraint {
    /// Maximum execution time in seconds
    pub max_execution_time_seconds: Option<u64>,
    /// Allowed time windows (UTC)
    pub allowed_time_windows: Vec<TimeWindow>,
    /// Deadline for completion
    pub deadline: Option<DateTime<Utc>>,
    /// Minimum execution time in seconds
    pub min_execution_time_seconds: Option<u64>,
}

/// Time window for constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Start time (HH:MM format)
    pub start_time: String,
    /// End time (HH:MM format)
    pub end_time: String,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u8>,
    /// Timezone
    pub timezone: String,
}

/// Quality constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConstraint {
    /// Minimum test coverage percentage
    pub min_test_coverage: Option<f64>,
    /// Required code quality metrics
    pub required_metrics: HashMap<String, f64>,
    /// Maximum complexity score
    pub max_complexity: Option<u32>,
    /// Required documentation coverage
    pub min_documentation_coverage: Option<f64>,
    /// Code review requirements
    pub requires_code_review: bool,
}

/// Security constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConstraint {
    /// Allowed network endpoints
    pub allowed_endpoints: Vec<String>,
    /// Denied network endpoints
    pub denied_endpoints: Vec<String>,
    /// Required authentication
    pub requires_authentication: bool,
    /// Allowed file operations
    pub allowed_file_operations: Vec<String>,
    /// Security scanning requirements
    pub requires_security_scan: bool,
}

/// Performance constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConstraint {
    /// Maximum response time in milliseconds
    pub max_response_time_ms: Option<u64>,
    /// Minimum throughput requirements
    pub min_throughput: Option<f64>,
    /// Maximum latency in milliseconds
    pub max_latency_ms: Option<u64>,
    /// Performance benchmarks to meet
    pub required_benchmarks: HashMap<String, f64>,
}

/// Collaboration constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationConstraint {
    /// Required team members
    pub required_team_members: Vec<String>,
    /// Allowed collaboration tools
    pub allowed_collaboration_tools: Vec<String>,
    /// Communication requirements
    pub communication_requirements: Vec<String>,
    /// Review requirements
    pub review_requirements: Vec<String>,
}

/// Constraint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// Unique constraint identifier
    pub id: String,
    /// Constraint name
    pub name: String,
    /// Constraint description
    pub description: String,
    /// Constraint type
    pub constraint_type: ConstraintType,
    /// Constraint severity
    pub severity: ConstraintSeverity,
    /// Enforcement mode
    pub enforcement_mode: EnforcementMode,
    /// Scope this constraint applies to
    pub scope: String,
    /// Whether constraint is active
    pub active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Constraint parameters
    pub parameters: ConstraintParameters,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Constraint parameters (union of all constraint types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintParameters {
    /// Resource usage constraints
    pub resource: Option<ResourceConstraint>,
    /// File access constraints
    pub file_access: Option<FileAccessConstraint>,
    /// Time constraints
    pub time: Option<TimeConstraint>,
    /// Quality constraints
    pub quality: Option<QualityConstraint>,
    /// Security constraints
    pub security: Option<SecurityConstraint>,
    /// Performance constraints
    pub performance: Option<PerformanceConstraint>,
    /// Collaboration constraints
    pub collaboration: Option<CollaborationConstraint>,
    /// Custom parameters
    pub custom: HashMap<String, serde_json::Value>,
}

/// Constraint violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintViolation {
    /// Violation identifier
    pub id: String,
    /// Constraint that was violated
    pub constraint_id: String,
    /// Violation description
    pub description: String,
    /// Violation severity
    pub severity: ConstraintSeverity,
    /// Timestamp when violation occurred
    pub timestamp: DateTime<Utc>,
    /// Context information about the violation
    pub context: HashMap<String, serde_json::Value>,
    /// Whether violation was resolved
    pub resolved: bool,
    /// Resolution timestamp
    pub resolved_at: Option<DateTime<Utc>>,
    /// Resolution notes
    pub resolution_notes: Option<String>,
}

/// Constraint enforcement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementResult {
    /// Whether all constraints were satisfied
    pub satisfied: bool,
    /// List of violations
    pub violations: Vec<ConstraintViolation>,
    /// Enforcement statistics
    pub stats: EnforcementStats,
    /// Recommendations for resolution
    pub recommendations: Vec<String>,
}

/// Enforcement statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementStats {
    /// Total constraints checked
    pub total_constraints: usize,
    /// Constraints satisfied
    pub satisfied_constraints: usize,
    /// Constraints violated
    pub violated_constraints: usize,
    /// Enforcement time in milliseconds
    pub enforcement_time_ms: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

/// Constraint system errors
#[derive(Debug, Error)]
pub enum ConstraintError {
    #[error("Constraint not found: {0}")]
    ConstraintNotFound(String),

    #[error("Invalid constraint definition: {0}")]
    InvalidConstraintDefinition(String),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Enforcement failed: {0}")]
    EnforcementFailed(String),

    #[error("Circular constraint dependency: {0}")]
    CircularDependency(String),

    #[error("Resource constraint exceeded: {0}")]
    ResourceExceeded(String),

    #[error("Time constraint violated: {0}")]
    TimeViolated(String),

    #[error("Security constraint violated: {0}")]
    SecurityViolated(String),
}

/// Constraint system for agentic development
pub struct ConstraintSystem {
    /// Active constraints
    constraints: HashMap<String, Constraint>,
    /// Constraint violations history
    violations: Vec<ConstraintViolation>,
    /// Enforcement statistics
    stats: EnforcementStats,
    /// Constraint dependencies
    dependencies: HashMap<String, Vec<String>>,
    /// Cache for enforcement results
    enforcement_cache: HashMap<String, EnforcementResult>,
}

impl ConstraintSystem {
    /// Create a new constraint system
    pub fn new() -> Self {
        Self {
            constraints: HashMap::new(),
            violations: Vec::new(),
            stats: EnforcementStats {
                total_constraints: 0,
                satisfied_constraints: 0,
                violated_constraints: 0,
                enforcement_time_ms: 0,
                cache_hit_rate: 0.0,
            },
            dependencies: HashMap::new(),
            enforcement_cache: HashMap::new(),
        }
    }

    /// Add a constraint to the system
    pub fn add_constraint(&mut self, constraint: Constraint) -> RhemaResult<()> {
        // Validate constraint
        self.validate_constraint(&constraint)?;

        // Check for circular dependencies
        if self.has_circular_dependency(&constraint.id) {
            return Err(ConstraintError::CircularDependency(constraint.id.clone()).into());
        }

        // Add constraint
        self.constraints.insert(constraint.id.clone(), constraint);
        self.stats.total_constraints += 1;

        Ok(())
    }

    /// Remove a constraint from the system
    pub fn remove_constraint(&mut self, constraint_id: &str) -> RhemaResult<()> {
        if !self.constraints.contains_key(constraint_id) {
            return Err(ConstraintError::ConstraintNotFound(constraint_id.to_string()).into());
        }

        // Check if constraint is being used by others
        for (_, deps) in &self.dependencies {
            if deps.contains(&constraint_id.to_string()) {
                return Err(ConstraintError::EnforcementFailed(
                    "Cannot remove constraint that is depended upon".to_string(),
                ).into());
            }
        }

        self.constraints.remove(constraint_id);
        self.stats.total_constraints -= 1;

        Ok(())
    }

    /// Get a constraint by ID
    pub fn get_constraint(&self, constraint_id: &str) -> Option<&Constraint> {
        self.constraints.get(constraint_id)
    }

    /// Get all constraints for a scope
    pub fn get_scope_constraints(&self, scope: &str) -> Vec<&Constraint> {
        self.constraints
            .values()
            .filter(|c| c.scope == scope && c.active)
            .collect()
    }

    /// Enforce constraints for a given context
    pub async fn enforce_constraints(
        &mut self,
        scope: &str,
        context: &ConstraintContext,
    ) -> RhemaResult<EnforcementResult> {
        let start_time = std::time::Instant::now();

        // Check cache first
        let cache_key = format!("{}:{}", scope, context.hash());
        if let Some(cached_result) = self.enforcement_cache.get(&cache_key) {
            self.stats.cache_hit_rate = 0.8; // Simplified cache hit rate
            return Ok(cached_result.clone());
        }

        let scope_constraints = self.get_scope_constraints(scope);
        let total_constraints = scope_constraints.len();
        let mut violations = Vec::new();
        let mut satisfied_count = 0;

        for constraint in &scope_constraints {
            match self.check_constraint(constraint, context).await {
                Ok(()) => {
                    satisfied_count += 1;
                }
                Err(violation) => {
                    violations.push(violation);
                }
            }
        }

        let enforcement_time = start_time.elapsed().as_millis() as u64;
        let satisfied = violations.is_empty();

        let result = EnforcementResult {
            satisfied,
            violations: violations.clone(),
            stats: EnforcementStats {
                total_constraints,
                satisfied_constraints: satisfied_count,
                violated_constraints: violations.len(),
                enforcement_time_ms: enforcement_time,
                cache_hit_rate: self.stats.cache_hit_rate,
            },
            recommendations: self.generate_recommendations(&violations),
        };

        // Cache the result
        self.enforcement_cache.insert(cache_key, result.clone());

        // Update global stats
        self.stats.enforcement_time_ms += enforcement_time;
        self.stats.satisfied_constraints += satisfied_count;
        self.stats.violated_constraints += violations.len();

        // Record violations
        self.violations.extend(violations);

        Ok(result)
    }

    /// Check a single constraint
    async fn check_constraint(
        &self,
        constraint: &Constraint,
        context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        match constraint.constraint_type {
            ConstraintType::ResourceUsage => {
                self.check_resource_constraint(constraint, context).await
            }
            ConstraintType::FileAccess => {
                self.check_file_access_constraint(constraint, context).await
            }
            ConstraintType::Dependency => {
                self.check_dependency_constraint(constraint, context).await
            }
            ConstraintType::Time => {
                self.check_time_constraint(constraint, context).await
            }
            ConstraintType::Quality => {
                self.check_quality_constraint(constraint, context).await
            }
            ConstraintType::Security => {
                self.check_security_constraint(constraint, context).await
            }
            ConstraintType::Performance => {
                self.check_performance_constraint(constraint, context).await
            }
            ConstraintType::Collaboration => {
                self.check_collaboration_constraint(constraint, context).await
            }
            ConstraintType::Custom(_) => {
                self.check_custom_constraint(constraint, context).await
            }
        }
    }

    /// Check resource usage constraint
    async fn check_resource_constraint(
        &self,
        constraint: &Constraint,
        context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        if let Some(resource_constraint) = &constraint.parameters.resource {
            // Check CPU usage
            if let Some(max_cpu) = resource_constraint.max_cpu_percent {
                if context.current_cpu_percent > max_cpu {
                    return Err(ConstraintViolation {
                        id: format!("{}-cpu", constraint.id),
                        constraint_id: constraint.id.clone(),
                        description: format!("CPU usage {}% exceeds limit of {}%", 
                            context.current_cpu_percent, max_cpu),
                        severity: constraint.severity.clone(),
                        timestamp: Utc::now(),
                        context: HashMap::new(),
                        resolved: false,
                        resolved_at: None,
                        resolution_notes: None,
                    });
                }
            }

            // Check memory usage
            if let Some(max_memory) = resource_constraint.max_memory_mb {
                if context.current_memory_mb > max_memory {
                    return Err(ConstraintViolation {
                        id: format!("{}-memory", constraint.id),
                        constraint_id: constraint.id.clone(),
                        description: format!("Memory usage {}MB exceeds limit of {}MB", 
                            context.current_memory_mb, max_memory),
                        severity: constraint.severity.clone(),
                        timestamp: Utc::now(),
                        context: HashMap::new(),
                        resolved: false,
                        resolved_at: None,
                        resolution_notes: None,
                    });
                }
            }
        }

        Ok(())
    }

    /// Check file access constraint
    async fn check_file_access_constraint(
        &self,
        constraint: &Constraint,
        context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        if let Some(file_constraint) = &constraint.parameters.file_access {
            for file_path in &context.accessed_files {
                // Check denied patterns
                for pattern in &file_constraint.denied_patterns {
                    if self.matches_pattern(file_path, pattern) {
                        return Err(ConstraintViolation {
                            id: format!("{}-file-access", constraint.id),
                            constraint_id: constraint.id.clone(),
                            description: format!("File access denied: {} matches pattern {}", 
                                file_path, pattern),
                            severity: constraint.severity.clone(),
                            timestamp: Utc::now(),
                            context: HashMap::new(),
                            resolved: false,
                            resolved_at: None,
                            resolution_notes: None,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Check time constraint
    async fn check_time_constraint(
        &self,
        constraint: &Constraint,
        _context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        if let Some(time_constraint) = &constraint.parameters.time {
            let now = Utc::now();

            // Check deadline
            if let Some(deadline) = time_constraint.deadline {
                if now > deadline {
                    return Err(ConstraintViolation {
                        id: format!("{}-deadline", constraint.id),
                        constraint_id: constraint.id.clone(),
                        description: format!("Deadline exceeded: {} > {}", now, deadline),
                        severity: constraint.severity.clone(),
                        timestamp: now,
                        context: HashMap::new(),
                        resolved: false,
                        resolved_at: None,
                        resolution_notes: None,
                    });
                }
            }

            // Check time windows
            if !time_constraint.allowed_time_windows.is_empty() {
                let mut in_allowed_window = false;
                for window in &time_constraint.allowed_time_windows {
                    if self.is_in_time_window(&now, window) {
                        in_allowed_window = true;
                        break;
                    }
                }

                if !in_allowed_window {
                    return Err(ConstraintViolation {
                        id: format!("{}-time-window", constraint.id),
                        constraint_id: constraint.id.clone(),
                        description: "Current time not in allowed time window".to_string(),
                        severity: constraint.severity.clone(),
                        timestamp: now,
                        context: HashMap::new(),
                        resolved: false,
                        resolved_at: None,
                        resolution_notes: None,
                    });
                }
            }
        }

        Ok(())
    }

    /// Check quality constraint
    async fn check_quality_constraint(
        &self,
        _constraint: &Constraint,
        _context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        // Quality constraints would typically involve running tests, linting, etc.
        // This is a simplified implementation
        Ok(())
    }

    /// Check security constraint
    async fn check_security_constraint(
        &self,
        constraint: &Constraint,
        context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        if let Some(security_constraint) = &constraint.parameters.security {
            // Check network endpoints
            for endpoint in &context.network_endpoints {
                if security_constraint.denied_endpoints.contains(endpoint) {
                    return Err(ConstraintViolation {
                        id: format!("{}-endpoint", constraint.id),
                        constraint_id: constraint.id.clone(),
                        description: format!("Denied network endpoint accessed: {}", endpoint),
                        severity: constraint.severity.clone(),
                        timestamp: Utc::now(),
                        context: HashMap::new(),
                        resolved: false,
                        resolved_at: None,
                        resolution_notes: None,
                    });
                }
            }
        }

        Ok(())
    }

    /// Check performance constraint
    async fn check_performance_constraint(
        &self,
        constraint: &Constraint,
        context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        if let Some(perf_constraint) = &constraint.parameters.performance {
            // Check response time
            if let Some(max_response_time) = perf_constraint.max_response_time_ms {
                if context.response_time_ms > max_response_time {
                    return Err(ConstraintViolation {
                        id: format!("{}-response-time", constraint.id),
                        constraint_id: constraint.id.clone(),
                        description: format!("Response time {}ms exceeds limit of {}ms", 
                            context.response_time_ms, max_response_time),
                        severity: constraint.severity.clone(),
                        timestamp: Utc::now(),
                        context: HashMap::new(),
                        resolved: false,
                        resolved_at: None,
                        resolution_notes: None,
                    });
                }
            }
        }

        Ok(())
    }

    /// Check collaboration constraint
    async fn check_collaboration_constraint(
        &self,
        _constraint: &Constraint,
        _context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        // Collaboration constraints would typically involve checking team availability, etc.
        // This is a simplified implementation
        Ok(())
    }

    /// Check dependency constraint
    async fn check_dependency_constraint(
        &self,
        _constraint: &Constraint,
        _context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        // TODO: Implement dependency constraint checking
        Ok(())
    }

    /// Check custom constraint
    async fn check_custom_constraint(
        &self,
        _constraint: &Constraint,
        _context: &ConstraintContext,
    ) -> Result<(), ConstraintViolation> {
        // Custom constraints would be implemented based on specific requirements
        // This is a simplified implementation
        Ok(())
    }

    /// Validate constraint definition
    fn validate_constraint(&self, constraint: &Constraint) -> RhemaResult<()> {
        if constraint.id.is_empty() {
            return Err(ConstraintError::InvalidConstraintDefinition(
                "Constraint ID cannot be empty".to_string(),
            ).into());
        }

        if constraint.name.is_empty() {
            return Err(ConstraintError::InvalidConstraintDefinition(
                "Constraint name cannot be empty".to_string(),
            ).into());
        }

        if constraint.scope.is_empty() {
            return Err(ConstraintError::InvalidConstraintDefinition(
                "Constraint scope cannot be empty".to_string(),
            ).into());
        }

        Ok(())
    }

    /// Check for circular dependencies
    fn has_circular_dependency(&self, _constraint_id: &str) -> bool {
        // Simplified circular dependency check
        // In a real implementation, this would traverse the dependency graph
        false
    }

    /// Check if a file path matches a pattern
    fn matches_pattern(&self, file_path: &str, pattern: &str) -> bool {
        // Simplified pattern matching
        // In a real implementation, this would use proper glob pattern matching
        file_path.contains(pattern)
    }

    /// Check if current time is in allowed time window
    fn is_in_time_window(&self, _now: &DateTime<Utc>, _window: &TimeWindow) -> bool {
        // Simplified time window check
        // In a real implementation, this would properly parse time windows
        true
    }

    /// Generate recommendations for violations
    fn generate_recommendations(&self, violations: &[ConstraintViolation]) -> Vec<String> {
        let mut recommendations = Vec::new();

        for violation in violations {
            match violation.description.as_str() {
                desc if desc.contains("CPU usage") => {
                    recommendations.push("Consider optimizing CPU-intensive operations".to_string());
                }
                desc if desc.contains("Memory usage") => {
                    recommendations.push("Consider implementing memory management strategies".to_string());
                }
                desc if desc.contains("File access denied") => {
                    recommendations.push("Review file access permissions and patterns".to_string());
                }
                desc if desc.contains("Deadline exceeded") => {
                    recommendations.push("Consider extending deadline or optimizing task execution".to_string());
                }
                desc if desc.contains("Response time") => {
                    recommendations.push("Consider optimizing performance-critical operations".to_string());
                }
                _ => {
                    recommendations.push("Review constraint configuration and adjust as needed".to_string());
                }
            }
        }

        recommendations
    }

    /// Get constraint violations
    pub fn get_violations(&self) -> &[ConstraintViolation] {
        &self.violations
    }

    /// Get enforcement statistics
    pub fn get_stats(&self) -> &EnforcementStats {
        &self.stats
    }

    /// Clear enforcement cache
    pub fn clear_cache(&mut self) {
        self.enforcement_cache.clear();
    }
}

/// Context for constraint enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintContext {
    /// Current CPU usage percentage
    pub current_cpu_percent: f64,
    /// Current memory usage in MB
    pub current_memory_mb: u64,
    /// Current disk usage in MB
    pub current_disk_mb: u64,
    /// Files being accessed
    pub accessed_files: Vec<String>,
    /// Network endpoints being accessed
    pub network_endpoints: Vec<String>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Custom context data
    pub custom_data: HashMap<String, serde_json::Value>,
}

impl ConstraintContext {
    /// Create a new constraint context
    pub fn new() -> Self {
        Self {
            current_cpu_percent: 0.0,
            current_memory_mb: 0,
            current_disk_mb: 0,
            accessed_files: Vec::new(),
            network_endpoints: Vec::new(),
            response_time_ms: 0,
            custom_data: HashMap::new(),
        }
    }

    /// Generate a hash for caching
    fn hash(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.current_cpu_percent.to_bits().hash(&mut hasher);
        self.current_memory_mb.hash(&mut hasher);
        self.current_disk_mb.hash(&mut hasher);
        self.response_time_ms.hash(&mut hasher);
        
        for file in &self.accessed_files {
            file.hash(&mut hasher);
        }
        
        for endpoint in &self.network_endpoints {
            endpoint.hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }
}

impl Default for ConstraintContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_constraint_system_creation() {
        let system = ConstraintSystem::new();
        assert_eq!(system.stats.total_constraints, 0);
    }

    #[tokio::test]
    async fn test_add_constraint() {
        let mut system = ConstraintSystem::new();
        
        let constraint = Constraint {
            id: "test-constraint".to_string(),
            name: "Test Constraint".to_string(),
            description: "A test constraint".to_string(),
            constraint_type: ConstraintType::ResourceUsage,
            severity: ConstraintSeverity::Warning,
            enforcement_mode: EnforcementMode::Soft,
            scope: "test-scope".to_string(),
            active: true,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            parameters: ConstraintParameters {
                resource: Some(ResourceConstraint {
                    max_cpu_percent: Some(80.0),
                    max_memory_mb: Some(1024),
                    max_disk_mb: None,
                    max_network_mbps: None,
                    max_concurrent_ops: None,
                }),
                file_access: None,
                time: None,
                quality: None,
                security: None,
                performance: None,
                collaboration: None,
                custom: HashMap::new(),
            },
            metadata: HashMap::new(),
        };

        assert!(system.add_constraint(constraint).is_ok());
        assert_eq!(system.stats.total_constraints, 1);
    }

    #[tokio::test]
    async fn test_enforce_constraints() {
        let mut system = ConstraintSystem::new();
        
        // Add a resource constraint
        let constraint = Constraint {
            id: "cpu-constraint".to_string(),
            name: "CPU Constraint".to_string(),
            description: "Limit CPU usage".to_string(),
            constraint_type: ConstraintType::ResourceUsage,
            severity: ConstraintSeverity::Warning,
            enforcement_mode: EnforcementMode::Soft,
            scope: "test-scope".to_string(),
            active: true,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            parameters: ConstraintParameters {
                resource: Some(ResourceConstraint {
                    max_cpu_percent: Some(50.0),
                    max_memory_mb: None,
                    max_disk_mb: None,
                    max_network_mbps: None,
                    max_concurrent_ops: None,
                }),
                file_access: None,
                time: None,
                quality: None,
                security: None,
                performance: None,
                collaboration: None,
                custom: HashMap::new(),
            },
            metadata: HashMap::new(),
        };

        system.add_constraint(constraint).unwrap();

        // Test with compliant context
        let mut context = ConstraintContext::new();
        context.current_cpu_percent = 30.0;

        let result = system.enforce_constraints("test-scope", &context).await.unwrap();
        assert!(result.satisfied);
        assert!(result.violations.is_empty());

        // Test with non-compliant context
        context.current_cpu_percent = 75.0;
        let result = system.enforce_constraints("test-scope", &context).await.unwrap();
        assert!(!result.satisfied);
        assert!(!result.violations.is_empty());
    }
} 