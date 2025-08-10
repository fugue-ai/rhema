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

use crate::schema::{
    DependencyType, LockedDependency, LockedScope, RhemaLock,
};
use crate::{RhemaError, RhemaResult};
use chrono::{Duration, Utc};
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;

/// Validation mode for lock file validation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationMode {
    /// Strict validation - all issues are treated as errors
    Strict,
    /// Lenient validation - some issues are treated as warnings
    Lenient,
}

/// Validation issue severity
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ValidationSeverity {
    /// Information message
    Info,
    /// Warning - issue that should be addressed but doesn't prevent operation
    Warning,
    /// Error - issue that prevents proper operation
    Error,
}

/// Validation issue with detailed information
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Severity of the issue
    pub severity: ValidationSeverity,
    /// Category of the issue
    pub category: String,
    /// Detailed message describing the issue
    pub message: String,
    /// Path or location where the issue occurred
    pub location: Option<String>,
    /// Suggested fix or recommendation
    pub suggestion: Option<String>,
}

/// Validation result containing all issues found
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// All validation issues found
    pub issues: Vec<ValidationIssue>,
    /// Whether validation passed (no errors in strict mode, no errors in lenient mode)
    pub is_valid: bool,
    /// Validation duration
    pub duration: std::time::Duration,
    /// Summary statistics
    pub summary: ValidationSummary,
}

/// Summary statistics for validation
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    /// Number of info messages
    pub info_count: usize,
    /// Number of warnings
    pub warning_count: usize,
    /// Number of errors
    pub error_count: usize,
    /// Total number of scopes validated
    pub scopes_validated: usize,
    /// Total number of dependencies validated
    pub dependencies_validated: usize,
    /// Number of circular dependencies detected
    pub circular_dependencies: usize,
    /// Number of checksum mismatches
    pub checksum_mismatches: usize,
    /// Number of version constraint violations
    pub version_violations: usize,
}

/// Comprehensive lock file validator
pub struct LockValidator {
    /// Validation mode (strict or lenient)
    mode: ValidationMode,
    /// Whether to validate checksums
    validate_checksums: bool,
    /// Whether to check for circular dependencies
    check_circular_dependencies: bool,
    /// Whether to validate scope existence
    validate_scope_existence: bool,
    /// Maximum age for lock file freshness (in hours)
    max_lock_age_hours: Option<u64>,
    /// Repository path for scope existence validation
    repo_path: Option<String>,
}

impl LockValidator {
    /// Create a new lock validator with default settings
    pub fn new() -> Self {
        Self {
            mode: ValidationMode::Strict,
            validate_checksums: true,
            check_circular_dependencies: true,
            validate_scope_existence: true,
            max_lock_age_hours: Some(24), // 24 hours default
            repo_path: None,
        }
    }

    /// Set the validation mode
    pub fn with_mode(mut self, mode: ValidationMode) -> Self {
        self.mode = mode;
        self
    }

    /// Enable or disable checksum validation
    pub fn with_checksum_validation(mut self, enabled: bool) -> Self {
        self.validate_checksums = enabled;
        self
    }

    /// Enable or disable circular dependency checking
    pub fn with_circular_dependency_check(mut self, enabled: bool) -> Self {
        self.check_circular_dependencies = enabled;
        self
    }

    /// Enable or disable scope existence validation
    pub fn with_scope_existence_validation(mut self, enabled: bool) -> Self {
        self.validate_scope_existence = enabled;
        self
    }

    /// Set maximum lock file age for freshness validation
    pub fn with_max_lock_age(mut self, hours: Option<u64>) -> Self {
        self.max_lock_age_hours = hours;
        self
    }

    /// Set repository path for scope existence validation
    pub fn with_repo_path(mut self, path: Option<String>) -> Self {
        self.repo_path = path;
        self
    }

    /// Validate a lock file with comprehensive checks
    pub fn validate(&self, lock_file: &RhemaLock) -> RhemaResult<()> {
        let mut issues = Vec::new();

        // 1. Validate lock file schema compliance
        issues.extend(self.validate_schema_compliance(lock_file));

        // 2. Verify checksums for all dependencies
        if self.validate_checksums {
            issues.extend(self.validate_checksums(lock_file));
        }

        // 3. Check for circular dependencies
        if self.check_circular_dependencies {
            issues.extend(self.validate_circular_dependencies(lock_file));
        }

        // 4. Validate version constraints
        issues.extend(self.validate_version_constraints(lock_file));

        // 5. Ensure all referenced scopes exist
        if self.validate_scope_existence {
            issues.extend(self.validate_scope_existence(lock_file));
        }

        // 6. Verify lock file freshness and consistency
        issues.extend(self.validate_lock_file_freshness(lock_file));

        // 7. Validate metadata consistency
        issues.extend(self.validate_metadata_consistency(lock_file));

        let is_valid = self.determine_validity(&issues);
        
        if !is_valid {
            let error_messages: Vec<String> = issues
                .iter()
                .filter(|issue| issue.severity == ValidationSeverity::Error)
                .map(|issue| format!("{}: {}", issue.category, issue.message))
                .collect();
            
            return Err(crate::RhemaError::ValidationError(format!(
                "Lock file validation failed: {}",
                error_messages.join("; ")
            )));
        }

        Ok(())
    }

    /// Validate lock file schema compliance
    fn validate_schema_compliance(&self, lock_file: &RhemaLock) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Validate lockfile version format
        if !self.is_valid_version_format(&lock_file.lockfile_version) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "Schema".to_string(),
                message: format!(
                    "Invalid lockfile version format: {}",
                    lock_file.lockfile_version
                ),
                location: Some("lockfile_version".to_string()),
                suggestion: Some("Version should follow semantic versioning format (e.g., 1.0.0)".to_string()),
            });
        }

        // Validate checksum format
        if !self.is_valid_checksum_format(&lock_file.checksum) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "Schema".to_string(),
                message: format!("Invalid checksum format: {}", lock_file.checksum),
                location: Some("checksum".to_string()),
                suggestion: Some("Checksum should be a 64-character hexadecimal string".to_string()),
            });
        }

        // Validate generated_at timestamp
        if lock_file.generated_at > Utc::now() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "Schema".to_string(),
                message: "Lock file generated_at timestamp is in the future".to_string(),
                location: Some("generated_at".to_string()),
                suggestion: Some("Check system clock and regenerate lock file if necessary".to_string()),
            });
        }

        // Validate generated_by field
        if lock_file.generated_by.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: "Schema".to_string(),
                message: "Generated_by field is empty".to_string(),
                location: Some("generated_by".to_string()),
                suggestion: Some("Include information about what generated this lock file".to_string()),
            });
        }

        issues
    }

    /// Validate checksums for all dependencies
    fn validate_checksums(&self, lock_file: &RhemaLock) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Validate main lock file checksum
        let calculated_checksum = self.calculate_lock_file_checksum(lock_file);
        if calculated_checksum != lock_file.checksum {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "Checksum".to_string(),
                message: format!(
                    "Lock file checksum mismatch. Expected: {}, Got: {}",
                    calculated_checksum, lock_file.checksum
                ),
                location: Some("checksum".to_string()),
                suggestion: Some("Regenerate lock file to fix checksum".to_string()),
            });
        }

        // Validate scope checksums
        for (scope_path, scope) in &lock_file.scopes {
            if let Some(source_checksum) = &scope.source_checksum {
                let calculated_scope_checksum = self.calculate_scope_checksum(scope_path, scope);
                if calculated_scope_checksum != *source_checksum {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        category: "Checksum".to_string(),
                        message: format!(
                            "Scope checksum mismatch for '{}'. Expected: {}, Got: {}",
                            scope_path, calculated_scope_checksum, source_checksum
                        ),
                        location: Some(format!("scopes.{}.source_checksum", scope_path)),
                        suggestion: Some("Scope source has changed, regenerate lock file".to_string()),
                    });
                }
            }

            // Validate dependency checksums
            for (dep_name, dep) in &scope.dependencies {
                let calculated_dep_checksum = self.calculate_dependency_checksum(dep_name, dep);
                if calculated_dep_checksum != dep.checksum {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        category: "Checksum".to_string(),
                        message: format!(
                            "Dependency checksum mismatch for '{}' in scope '{}'. Expected: {}, Got: {}",
                            dep_name, scope_path, calculated_dep_checksum, dep.checksum
                        ),
                        location: Some(format!("scopes.{}.dependencies.{}.checksum", scope_path, dep_name)),
                        suggestion: Some("Dependency has changed, regenerate lock file".to_string()),
                    });
                }
            }
        }

        issues
    }

    /// Check for circular dependencies
    fn validate_circular_dependencies(&self, lock_file: &RhemaLock) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();

        for scope_path in lock_file.scopes.keys() {
            if !visited.contains(scope_path) {
                if let Some(cycle) = self.detect_cycle(lock_file, scope_path, &mut visited, &mut recursion_stack) {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        category: "CircularDependency".to_string(),
                        message: format!("Circular dependency detected: {}", cycle.join(" -> ")),
                        location: Some("scopes".to_string()),
                        suggestion: Some("Review and resolve circular dependencies in scope definitions".to_string()),
                    });
                }
            }
        }

        issues
    }

    /// Validate version constraints
    fn validate_version_constraints(&self, lock_file: &RhemaLock) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for (scope_path, scope) in &lock_file.scopes {
            // Validate scope version format
            if !self.is_valid_version_format(&scope.version) {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    category: "Version".to_string(),
                    message: format!(
                        "Invalid scope version format in '{}': {}",
                        scope_path, scope.version
                    ),
                    location: Some(format!("scopes.{}.version", scope_path)),
                    suggestion: Some("Version should follow semantic versioning format".to_string()),
                });
            }

            // Validate dependency versions
            for (dep_name, dep) in &scope.dependencies {
                if !self.is_valid_version_format(&dep.version) {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        category: "Version".to_string(),
                        message: format!(
                            "Invalid dependency version format for '{}' in scope '{}': {}",
                            dep_name, scope_path, dep.version
                        ),
                        location: Some(format!("scopes.{}.dependencies.{}.version", scope_path, dep_name)),
                        suggestion: Some("Version should follow semantic versioning format".to_string()),
                    });
                }

                // Validate original constraint if present
                if let Some(constraint) = &dep.original_constraint {
                    if !self.is_valid_version_constraint(constraint) {
                        issues.push(ValidationIssue {
                            severity: ValidationSeverity::Warning,
                            category: "Version".to_string(),
                            message: format!(
                                "Invalid version constraint for '{}' in scope '{}': {}",
                                dep_name, scope_path, constraint
                            ),
                            location: Some(format!("scopes.{}.dependencies.{}.original_constraint", scope_path, dep_name)),
                            suggestion: Some("Review version constraint format".to_string()),
                        });
                    }
                }
            }
        }

        issues
    }

    /// Ensure all referenced scopes exist
    fn validate_scope_existence(&self, lock_file: &RhemaLock) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        if let Some(repo_path) = &self.repo_path {
            for (scope_path, scope) in &lock_file.scopes {
                let full_path = Path::new(repo_path).join(&scope.path);
                if !full_path.exists() {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        category: "ScopeExistence".to_string(),
                        message: format!(
                            "Scope path does not exist: {}",
                            full_path.display()
                        ),
                        location: Some(format!("scopes.{}.path", scope_path)),
                        suggestion: Some("Check if scope has been moved or deleted".to_string()),
                    });
                } else if !full_path.is_dir() {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        category: "ScopeExistence".to_string(),
                        message: format!(
                            "Scope path is not a directory: {}",
                            full_path.display()
                        ),
                        location: Some(format!("scopes.{}.path", scope_path)),
                        suggestion: Some("Scope path should point to a directory".to_string()),
                    });
                }
            }
        }

        issues
    }

    /// Verify lock file freshness and consistency
    fn validate_lock_file_freshness(&self, lock_file: &RhemaLock) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check lock file age
        if let Some(max_age_hours) = self.max_lock_age_hours {
            let age = Utc::now() - lock_file.generated_at;
            let max_age = Duration::hours(max_age_hours as i64);
            
            if age > max_age {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: "Freshness".to_string(),
                    message: format!(
                        "Lock file is {} hours old (max: {} hours)",
                        age.num_hours(),
                        max_age_hours
                    ),
                    location: Some("generated_at".to_string()),
                    suggestion: Some("Consider regenerating lock file for latest dependency versions".to_string()),
                });
            }
        }

        // Check for stale resolution timestamps
        for (scope_path, scope) in &lock_file.scopes {
            let scope_age = Utc::now() - scope.resolved_at;
            if scope_age > Duration::hours(168) { // 1 week
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Info,
                    category: "Freshness".to_string(),
                    message: format!(
                        "Scope '{}' was resolved {} days ago",
                        scope_path,
                        scope_age.num_days()
                    ),
                    location: Some(format!("scopes.{}.resolved_at", scope_path)),
                    suggestion: Some("Consider re-resolving scope dependencies".to_string()),
                });
            }

            for (dep_name, dep) in &scope.dependencies {
                let dep_age = Utc::now() - dep.resolved_at;
                if dep_age > Duration::hours(168) { // 1 week
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Info,
                        category: "Freshness".to_string(),
                        message: format!(
                            "Dependency '{}' in scope '{}' was resolved {} days ago",
                            dep_name,
                            scope_path,
                            dep_age.num_days()
                        ),
                        location: Some(format!("scopes.{}.dependencies.{}.resolved_at", scope_path, dep_name)),
                        suggestion: Some("Consider checking for newer dependency versions".to_string()),
                    });
                }
            }
        }

        issues
    }

    /// Validate metadata consistency
    fn validate_metadata_consistency(&self, lock_file: &RhemaLock) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        let metadata = &lock_file.metadata;

        // Validate total counts
        let actual_scopes = lock_file.scopes.len() as u32;
        if metadata.total_scopes != actual_scopes {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "Metadata".to_string(),
                message: format!(
                    "Total scopes count mismatch. Expected: {}, Got: {}",
                    metadata.total_scopes, actual_scopes
                ),
                location: Some("metadata.total_scopes".to_string()),
                suggestion: Some("Regenerate lock file to fix metadata counts".to_string()),
            });
        }

        let actual_dependencies: u32 = lock_file
            .scopes
            .values()
            .map(|scope| scope.dependencies.len() as u32)
            .sum();
        if metadata.total_dependencies != actual_dependencies {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: "Metadata".to_string(),
                message: format!(
                    "Total dependencies count mismatch. Expected: {}, Got: {}",
                    metadata.total_dependencies, actual_dependencies
                ),
                location: Some("metadata.total_dependencies".to_string()),
                suggestion: Some("Regenerate lock file to fix metadata counts".to_string()),
            });
        }

        // Validate performance metrics if present
        if let Some(metrics) = &metadata.performance_metrics {
            if metrics.generation_time_ms == 0 {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: "Metadata".to_string(),
                    message: "Generation time is zero, which seems unlikely".to_string(),
                    location: Some("metadata.performance_metrics.generation_time_ms".to_string()),
                    suggestion: Some("Check if performance metrics are being recorded correctly".to_string()),
                });
            }

            if metrics.cache_hits + metrics.cache_misses > 0 {
                let calculated_hit_rate = metrics.cache_hits as f64 / (metrics.cache_hits + metrics.cache_misses) as f64;
                // Validate cache hit rate is reasonable (between 0 and 1)
                if calculated_hit_rate < 0.0 || calculated_hit_rate > 1.0 {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Warning,
                        category: "Performance".to_string(),
                        message: format!("Cache hit rate {} is outside expected range [0.0, 1.0]", calculated_hit_rate),
                        location: Some("metadata.performance_metrics".to_string()),
                        suggestion: Some("Check cache configuration and usage patterns".to_string()),
                    });
                }
            }
        }

        issues
    }

    /// Calculate validation summary
    fn calculate_summary(&self, issues: &[ValidationIssue], lock_file: &RhemaLock) -> ValidationSummary {
        let mut summary = ValidationSummary {
            info_count: 0,
            warning_count: 0,
            error_count: 0,
            scopes_validated: lock_file.scopes.len(),
            dependencies_validated: lock_file
                .scopes
                .values()
                .map(|scope| scope.dependencies.len())
                .sum(),
            circular_dependencies: 0,
            checksum_mismatches: 0,
            version_violations: 0,
        };

        for issue in issues {
            match issue.severity {
                ValidationSeverity::Info => summary.info_count += 1,
                ValidationSeverity::Warning => summary.warning_count += 1,
                ValidationSeverity::Error => summary.error_count += 1,
            }

            match issue.category.as_str() {
                "CircularDependency" => summary.circular_dependencies += 1,
                "Checksum" => summary.checksum_mismatches += 1,
                "Version" => summary.version_violations += 1,
                _ => {}
            }
        }

        summary
    }

    /// Determine if validation passed based on mode and issues
    fn determine_validity(&self, issues: &[ValidationIssue]) -> bool {
        let has_errors = issues.iter().any(|issue| issue.severity == ValidationSeverity::Error);
        
        match self.mode {
            ValidationMode::Strict => !has_errors,
            ValidationMode::Lenient => !has_errors, // In lenient mode, warnings don't fail validation
        }
    }

    /// Helper methods

    fn is_valid_version_format(&self, version: &str) -> bool {
        let version_regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
        version_regex.is_match(version)
    }

    fn is_valid_checksum_format(&self, checksum: &str) -> bool {
        let checksum_regex = Regex::new(r"^[a-fA-F0-9]{64}$").unwrap();
        checksum_regex.is_match(checksum)
    }

    fn is_valid_version_constraint(&self, constraint: &str) -> bool {
        // Basic version constraint validation - can be enhanced for more complex constraints
        !constraint.trim().is_empty()
    }

    fn calculate_lock_file_checksum(&self, lock_file: &RhemaLock) -> String {
        // Create a copy without the checksum field for calculation
        let mut lock_copy = lock_file.clone();
        lock_copy.checksum = String::new();
        
        let serialized = serde_json::to_string(&lock_copy).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn calculate_scope_checksum(&self, scope_path: &str, scope: &LockedScope) -> String {
        let mut hasher = Sha256::new();
        hasher.update(scope_path.as_bytes());
        hasher.update(scope.version.as_bytes());
        hasher.update(scope.path.as_bytes());
        hasher.update(scope.resolved_at.timestamp().to_string().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn calculate_dependency_checksum(&self, dep_name: &str, dep: &LockedDependency) -> String {
        let mut hasher = Sha256::new();
        hasher.update(dep_name.as_bytes());
        hasher.update(dep.version.as_bytes());
        hasher.update(dep.path.as_bytes());
        hasher.update(dep.resolved_at.timestamp().to_string().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn detect_cycle(
        &self,
        lock_file: &RhemaLock,
        current: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
    ) -> Option<Vec<String>> {
        if recursion_stack.contains(current) {
            return Some(vec![current.to_string()]);
        }

        if visited.contains(current) {
            return None;
        }

        visited.insert(current.to_string());
        recursion_stack.insert(current.to_string());

        if let Some(scope) = lock_file.scopes.get(current) {
            for dep_name in scope.dependencies.keys() {
                if let Some(mut cycle) = self.detect_cycle(lock_file, dep_name, visited, recursion_stack) {
                    if cycle.len() == 1 && cycle[0] == current {
                        cycle.push(current.to_string());
                    } else {
                        cycle.push(current.to_string());
                    }
                    recursion_stack.remove(current);
                    return Some(cycle);
                }
            }
        }

        recursion_stack.remove(current);
        None
    }
}

impl Default for LockValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for quick validation
pub fn validate_lock_file(lock_file: &RhemaLock) -> RhemaResult<()> {
    LockValidator::new().validate(lock_file)
}

/// Convenience function for strict validation
pub fn validate_lock_file_strict(lock_file: &RhemaLock) -> RhemaResult<()> {
    LockValidator::new()
        .with_mode(ValidationMode::Strict)
        .validate(lock_file)
}

/// Convenience function for lenient validation
pub fn validate_lock_file_lenient(lock_file: &RhemaLock) -> RhemaResult<()> {
    LockValidator::new()
        .with_mode(ValidationMode::Lenient)
        .validate(lock_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::RhemaLock;

    fn create_test_lock_file() -> RhemaLock {
        let mut lock = RhemaLock::new("test-validator");
        lock.lockfile_version = "1.0.0".to_string();
        
        let mut scope = LockedScope::new("1.0.0", "crates/rhema-core");
        let dep = LockedDependency::new("1.0.0", "crates/rhema-ai", DependencyType::Required);
        scope.add_dependency("crates/rhema-ai".to_string(), dep);
        lock.add_scope("crates/rhema-core".to_string(), scope);
        
        // Set a valid checksum format manually
        lock.checksum = "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456".to_string();
        
        lock
    }

    #[test]
    fn test_validator_creation() {
        let validator = LockValidator::new();
        assert_eq!(validator.mode, ValidationMode::Strict);
        assert!(validator.validate_checksums);
        assert!(validator.check_circular_dependencies);
        assert!(validator.validate_scope_existence);
    }

    #[test]
    fn test_validator_builder_pattern() {
        let validator = LockValidator::new()
            .with_mode(ValidationMode::Lenient)
            .with_checksum_validation(false)
            .with_circular_dependency_check(false)
            .with_scope_existence_validation(false)
            .with_max_lock_age(Some(48))
            .with_repo_path(Some("/test/path".to_string()));

        assert_eq!(validator.mode, ValidationMode::Lenient);
        assert!(!validator.validate_checksums);
        assert!(!validator.check_circular_dependencies);
        assert!(!validator.validate_scope_existence);
        assert_eq!(validator.max_lock_age_hours, Some(48));
        assert_eq!(validator.repo_path, Some("/test/path".to_string()));
    }

    #[test]
    fn test_version_format_validation() {
        let validator = LockValidator::new();
        
        assert!(validator.is_valid_version_format("1.0.0"));
        assert!(validator.is_valid_version_format("0.1.0"));
        assert!(validator.is_valid_version_format("10.20.30"));
        
        assert!(!validator.is_valid_version_format("1.0"));
        assert!(!validator.is_valid_version_format("1.0.0.0"));
        assert!(!validator.is_valid_version_format("1.0.0-alpha"));
        assert!(!validator.is_valid_version_format("invalid"));
    }

    #[test]
    fn test_checksum_format_validation() {
        let validator = LockValidator::new();
        
        assert!(validator.is_valid_checksum_format(
            "a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456"
        ));
        assert!(validator.is_valid_checksum_format(
            "A1B2C3D4E5F6789012345678901234567890ABCDEF1234567890ABCDEF123456"
        ));
        
        assert!(!validator.is_valid_checksum_format("invalid"));
        assert!(!validator.is_valid_checksum_format("a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef12345"));
        assert!(!validator.is_valid_checksum_format("a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567"));
    }

    #[test]
    fn test_basic_validation() {
        let lock_file = create_test_lock_file();
        let validator = LockValidator::new()
            .with_checksum_validation(false)
            .with_scope_existence_validation(false);
        
        // Should not panic or return error for valid lock file
        assert!(validator.validate(&lock_file).is_ok());
    }

    #[test]
    fn test_validation_with_invalid_version() {
        let mut lock_file = create_test_lock_file();
        lock_file.lockfile_version = "invalid".to_string();
        
        let validator = LockValidator::new()
            .with_checksum_validation(false)
            .with_scope_existence_validation(false);
        
        // Should return error for invalid lock file
        assert!(validator.validate(&lock_file).is_err());
    }

    #[test]
    fn test_convenience_functions() {
        let lock_file = create_test_lock_file();
        
        // Test with custom validators that disable checksum validation
        let strict_validator = LockValidator::new()
            .with_mode(ValidationMode::Strict)
            .with_checksum_validation(false)
            .with_scope_existence_validation(false);
        
        let lenient_validator = LockValidator::new()
            .with_mode(ValidationMode::Lenient)
            .with_checksum_validation(false)
            .with_scope_existence_validation(false);
        
        // Both should succeed for valid lock file when checksum validation is disabled
        assert!(strict_validator.validate(&lock_file).is_ok());
        assert!(lenient_validator.validate(&lock_file).is_ok());
    }
} 