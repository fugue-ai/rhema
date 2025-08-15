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
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;

/// Dependency type enumeration for lock files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Peer,
    Development,
    Build,
}

/// Validation status for lock files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Valid,
    Invalid,
    Warning,
    Pending,
}

/// Resolution strategy for dependency conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionStrategy {
    Latest,
    Earliest,
    Pinned,
    Range,
    Compatible,
}

/// Conflict resolution method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolution {
    Manual,
    Automatic,
    Prompt,
    Skip,
    Fail,
}

/// Main lock file structure for the Rhema lock file system
///
/// This structure represents the complete state of a Rhema project's dependencies
/// and provides deterministic, reproducible builds by locking all dependency
/// versions and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhemaLock {
    /// Version of the lock file format
    pub lockfile_version: String,

    /// Timestamp when the lock file was generated
    pub generated_at: DateTime<Utc>,

    /// Information about what generated this lock file
    pub generated_by: String,

    /// Checksum of the lock file contents for integrity verification
    pub checksum: String,

    /// Locked scopes in the project
    pub scopes: HashMap<String, LockedScope>,

    /// Metadata about the lock file
    pub metadata: LockMetadata,
}

/// Individual locked scope structure
///
/// Represents a single scope with its locked dependencies and metadata.
/// Each scope is identified by its path and contains all resolved dependencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedScope {
    /// Version of the scope that was locked
    pub version: String,

    /// Path to the scope in the project
    pub path: String,

    /// All dependencies for this scope with their resolved versions
    pub dependencies: HashMap<String, LockedDependency>,

    /// Checksum of the scope's source for integrity verification
    pub source_checksum: Option<String>,

    /// Timestamp when this scope was last resolved
    pub resolved_at: DateTime<Utc>,

    /// Whether this scope has any circular dependencies
    pub has_circular_dependencies: bool,

    /// Custom metadata for the scope
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Individual locked dependency structure
///
/// Represents a single dependency with its resolved version and metadata.
/// This ensures deterministic builds by locking the exact version and path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedDependency {
    /// Resolved version of the dependency
    pub version: String,

    /// Path to the dependency
    pub path: String,

    /// Timestamp when this dependency was resolved
    pub resolved_at: DateTime<Utc>,

    /// Checksum of the dependency for integrity verification
    pub checksum: String,

    /// Type of dependency (required, optional, peer, etc.)
    pub dependency_type: DependencyType,

    /// Original version constraint that was resolved
    pub original_constraint: Option<String>,

    /// Whether this dependency is transitive (dependency of a dependency)
    pub is_transitive: bool,

    /// Direct dependencies of this dependency
    pub dependencies: Option<Vec<String>>,

    /// Custom metadata for the dependency
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Metadata structure for lock files
///
/// Contains aggregate information about the lock file and its contents,
/// useful for validation, analysis, and debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockMetadata {
    /// Total number of scopes in the lock file
    pub total_scopes: u32,

    /// Total number of dependencies across all scopes
    pub total_dependencies: u32,

    /// Number of circular dependencies detected
    pub circular_dependencies: u32,

    /// Overall validation status of the lock file
    pub validation_status: ValidationStatus,

    /// Strategy used for dependency resolution
    pub resolution_strategy: ResolutionStrategy,

    /// Method used for conflict resolution
    pub conflict_resolution: ConflictResolution,

    /// Timestamp when the lock file was last validated
    pub last_validated: Option<DateTime<Utc>>,

    /// List of validation warnings or errors
    pub validation_messages: Option<Vec<String>>,

    /// Performance metrics for lock file operations
    pub performance_metrics: Option<LockPerformanceMetrics>,

    /// Custom metadata
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

/// Performance metrics for lock file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockPerformanceMetrics {
    /// Time taken to generate the lock file in milliseconds
    pub generation_time_ms: u64,

    /// Time taken to validate the lock file in milliseconds
    pub validation_time_ms: Option<u64>,

    /// Memory usage during lock file operations in bytes
    pub memory_usage_bytes: Option<u64>,

    /// Number of dependency resolution attempts
    pub resolution_attempts: u32,

    /// Number of cache hits during resolution
    pub cache_hits: u32,

    /// Number of cache misses during resolution
    pub cache_misses: u32,
}

impl LockMetadata {
    /// Create a new lock metadata instance
    pub fn new() -> Self {
        Self {
            total_scopes: 0,
            total_dependencies: 0,
            circular_dependencies: 0,
            validation_status: ValidationStatus::Pending,
            resolution_strategy: ResolutionStrategy::Latest,
            conflict_resolution: ConflictResolution::Automatic,
            last_validated: None,
            validation_messages: None,
            performance_metrics: None,
            custom: HashMap::new(),
        }
    }

    /// Set performance metrics
    pub fn set_performance_metrics(&mut self, metrics: LockPerformanceMetrics) {
        self.performance_metrics = Some(metrics);
    }

    /// Validate the metadata
    pub fn validate(&self) -> crate::RhemaResult<()> {
        // Basic validation - in a real implementation, this would be more comprehensive
        Ok(())
    }
}

impl Default for LockMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl LockPerformanceMetrics {
    /// Create new performance metrics
    pub fn new(generation_time_ms: u64) -> Self {
        Self {
            generation_time_ms,
            validation_time_ms: None,
            memory_usage_bytes: None,
            resolution_attempts: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}

impl RhemaLock {
    /// Create a new RhemaLock with default values
    pub fn new(generated_by: &str) -> Self {
        Self {
            lockfile_version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            generated_by: generated_by.to_string(),
            checksum: String::new(), // Will be calculated later
            scopes: HashMap::new(),
            metadata: LockMetadata::new(),
        }
    }

    /// Calculate the checksum of the lock file contents
    pub fn calculate_checksum(&self) -> String {
        use sha2::{Digest, Sha256};

        // Create a hash of the lock file contents
        let mut hasher = Sha256::new();

        // Hash the lock file version
        hasher.update(self.lockfile_version.as_bytes());

        // Hash the generated timestamp
        hasher.update(self.generated_at.timestamp().to_string().as_bytes());

        // Hash the generated by field
        hasher.update(self.generated_by.as_bytes());

        // Hash all scopes and their dependencies
        let mut scope_paths: Vec<_> = self.scopes.keys().collect();
        scope_paths.sort(); // Ensure deterministic ordering

        for path in scope_paths {
            hasher.update(path.as_bytes());
            if let Some(scope) = self.scopes.get(path) {
                hasher.update(scope.version.as_bytes());
                hasher.update(scope.path.as_bytes());

                // Hash dependencies in sorted order
                let mut dep_names: Vec<_> = scope.dependencies.keys().collect();
                dep_names.sort();
                for dep_name in dep_names {
                    hasher.update(dep_name.as_bytes());
                    if let Some(dep) = scope.dependencies.get(dep_name) {
                        hasher.update(dep.version.as_bytes());
                        hasher.update(dep.path.as_bytes());
                        hasher.update(dep.checksum.as_bytes());
                    }
                }
            }
        }

        // Convert to hex string
        format!("{:x}", hasher.finalize())
    }

    /// Update the checksum field
    pub fn update_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }

    /// Add a locked scope to the lock file
    pub fn add_scope(&mut self, path: String, scope: LockedScope) {
        self.scopes.insert(path, scope);
        self.update_metadata();
    }

    /// Get a locked scope by path
    pub fn get_scope(&self, path: &str) -> Option<&LockedScope> {
        self.scopes.get(path)
    }

    /// Remove a scope from the lock file
    pub fn remove_scope(&mut self, path: &str) -> Option<LockedScope> {
        let scope = self.scopes.remove(path);
        self.update_metadata();
        scope
    }

    /// Update metadata based on current scopes
    fn update_metadata(&mut self) {
        let total_scopes = self.scopes.len() as u32;
        let total_dependencies: u32 = self
            .scopes
            .values()
            .map(|scope| scope.dependencies.len() as u32)
            .sum();
        let circular_dependencies: u32 = self
            .scopes
            .values()
            .filter(|scope| scope.has_circular_dependencies)
            .count() as u32;

        self.metadata.total_scopes = total_scopes;
        self.metadata.total_dependencies = total_dependencies;
        self.metadata.circular_dependencies = circular_dependencies;
    }
}

impl LockedScope {
    /// Create a new LockedScope
    pub fn new(version: &str, path: &str) -> Self {
        Self {
            version: version.to_string(),
            path: path.to_string(),
            dependencies: HashMap::new(),
            source_checksum: None,
            resolved_at: Utc::now(),
            has_circular_dependencies: false,
            custom: HashMap::new(),
        }
    }

    /// Add a dependency to the scope
    pub fn add_dependency(&mut self, name: String, dependency: LockedDependency) {
        self.dependencies.insert(name, dependency);
    }

    /// Get a dependency by name
    pub fn get_dependency(&self, name: &str) -> Option<&LockedDependency> {
        self.dependencies.get(name)
    }

    /// Remove a dependency from the scope
    pub fn remove_dependency(&mut self, name: &str) -> Option<LockedDependency> {
        self.dependencies.remove(name)
    }

    /// Check if the scope has any dependencies
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    /// Get the number of dependencies
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }
}

impl LockedDependency {
    /// Create a new LockedDependency
    pub fn new(version: &str, path: &str, dependency_type: DependencyType) -> Self {
        Self {
            version: version.to_string(),
            path: path.to_string(),
            resolved_at: Utc::now(),
            checksum: String::new(), // Will be calculated later
            dependency_type,
            original_constraint: None,
            is_transitive: false,
            dependencies: None,
            custom: HashMap::new(),
        }
    }

    /// Calculate the checksum of the dependency
    pub fn calculate_checksum(&self) -> String {
        use sha2::{Digest, Sha256};

        // Create a hash of the dependency contents
        let mut hasher = Sha256::new();

        // Hash the version
        hasher.update(self.version.as_bytes());

        // Hash the path
        hasher.update(self.path.as_bytes());

        // Hash the dependency type
        hasher.update(format!("{:?}", self.dependency_type).as_bytes());

        // Hash the original constraint if present
        if let Some(constraint) = &self.original_constraint {
            hasher.update(constraint.as_bytes());
        }

        // Hash the transitive flag
        hasher.update(self.is_transitive.to_string().as_bytes());

        // Hash dependencies if present
        if let Some(deps) = &self.dependencies {
            let mut sorted_deps = deps.clone();
            sorted_deps.sort(); // Ensure deterministic ordering
            for dep in sorted_deps {
                hasher.update(dep.as_bytes());
            }
        }

        // Convert to hex string
        format!("{:x}", hasher.finalize())
    }

    /// Update the checksum field
    pub fn update_checksum(&mut self) {
        self.checksum = self.calculate_checksum();
    }
}
