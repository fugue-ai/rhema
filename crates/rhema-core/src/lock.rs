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
    file_ops::{read_yaml_file, write_yaml_file},
    ConflictResolution, LockPerformanceMetrics, LockedDependency, LockedScope, RhemaError,
    RhemaLock, RhemaResult,
};
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Default lock file name
pub const DEFAULT_LOCK_FILE: &str = "rhema.lock";

/// Default lock file version
pub const DEFAULT_LOCK_VERSION: &str = "1.0.0";

/// Lock file operations module
pub struct LockFileOps;

impl LockFileOps {
    /// Read a lock file from the specified path
    pub fn read_lock_file(lock_path: &Path) -> RhemaResult<RhemaLock> {
        if !lock_path.exists() {
            return Err(RhemaError::FileNotFound(format!(
                "Lock file not found: {}",
                lock_path.display()
            )));
        }

        let lock_data: RhemaLock = read_yaml_file(lock_path)?;

        // Validate the lock file after reading
        lock_data.validate()?;

        Ok(lock_data)
    }

    /// Write a lock file to the specified path
    pub fn write_lock_file(lock_path: &Path, lock_data: &RhemaLock) -> RhemaResult<()> {
        // Create a mutable copy to update checksum
        let mut lock_data = lock_data.clone();
        lock_data.update_checksum();

        // Validate the lock data after updating checksum
        lock_data.validate()?;

        write_yaml_file(lock_path, &lock_data)
    }

    /// Create a new lock file at the specified path
    pub fn create_lock_file(lock_path: &Path, generated_by: &str) -> RhemaResult<RhemaLock> {
        let mut lock_data = RhemaLock::new(generated_by);

        // Update checksum before writing
        lock_data.update_checksum();

        Self::write_lock_file(lock_path, &lock_data)?;

        Ok(lock_data)
    }

    /// Get or create a lock file at the specified path
    pub fn get_or_create_lock_file(lock_path: &Path, generated_by: &str) -> RhemaResult<RhemaLock> {
        if lock_path.exists() {
            Self::read_lock_file(lock_path)
        } else {
            Self::create_lock_file(lock_path, generated_by)
        }
    }

    /// Validate a lock file's integrity
    pub fn validate_lock_file_integrity(lock_path: &Path) -> RhemaResult<ValidationResult> {
        let start_time = Instant::now();
        let mut validation_messages = Vec::new();
        let mut is_valid = true;

        // Read and validate the lock file
        let lock_data = match Self::read_lock_file(lock_path) {
            Ok(data) => data,
            Err(e) => {
                validation_messages.push(format!("Failed to read lock file: {}", e));
                return Ok(ValidationResult {
                    is_valid: false,
                    messages: validation_messages,
                    validation_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        };

        // Validate checksum
        let expected_checksum = lock_data.calculate_checksum();
        if lock_data.checksum != expected_checksum {
            validation_messages.push("Lock file checksum mismatch detected".to_string());
            is_valid = false;
        }

        // Validate each scope
        for (scope_path, scope) in &lock_data.scopes {
            if let Err(e) = scope.validate() {
                validation_messages.push(format!("Scope {} validation failed: {}", scope_path, e));
                is_valid = false;
            }

            // Validate dependencies
            for (dep_name, dep) in &scope.dependencies {
                if let Err(e) = dep.validate() {
                    validation_messages.push(format!(
                        "Dependency {} in scope {} validation failed: {}",
                        dep_name, scope_path, e
                    ));
                    is_valid = false;
                }
            }
        }

        // Validate metadata
        if let Err(e) = lock_data.metadata.validate() {
            validation_messages.push(format!("Metadata validation failed: {}", e));
            is_valid = false;
        }

        let validation_time = start_time.elapsed().as_millis() as u64;

        Ok(ValidationResult {
            is_valid,
            messages: validation_messages,
            validation_time_ms: validation_time,
        })
    }

    /// Update a lock file with new scope information
    pub fn update_lock_file_scope(
        lock_path: &Path,
        scope_path: &str,
        scope_data: LockedScope,
    ) -> RhemaResult<RhemaLock> {
        let mut lock_data = Self::read_lock_file(lock_path)?;

        lock_data.add_scope(scope_path.to_string(), scope_data);

        Self::write_lock_file(lock_path, &lock_data)?;

        Ok(lock_data)
    }

    /// Remove a scope from a lock file
    pub fn remove_lock_file_scope(lock_path: &Path, scope_path: &str) -> RhemaResult<RhemaLock> {
        let mut lock_data = Self::read_lock_file(lock_path)?;

        if lock_data.remove_scope(scope_path).is_none() {
            return Err(RhemaError::LockError(format!(
                "Scope {} not found in lock file",
                scope_path
            )));
        }

        Self::write_lock_file(lock_path, &lock_data)?;

        Ok(lock_data)
    }

    /// Add a dependency to a scope in the lock file
    pub fn add_scope_dependency(
        lock_path: &Path,
        scope_path: &str,
        dep_name: &str,
        dependency: LockedDependency,
    ) -> RhemaResult<RhemaLock> {
        let mut lock_data = Self::read_lock_file(lock_path)?;

        if let Some(scope) = lock_data.scopes.get_mut(scope_path) {
            scope.add_dependency(dep_name.to_string(), dependency);
        } else {
            return Err(RhemaError::LockError(format!(
                "Scope {} not found in lock file",
                scope_path
            )));
        }

        Self::write_lock_file(lock_path, &lock_data)?;

        Ok(lock_data)
    }

    /// Remove a dependency from a scope in the lock file
    pub fn remove_scope_dependency(
        lock_path: &Path,
        scope_path: &str,
        dep_name: &str,
    ) -> RhemaResult<RhemaLock> {
        let mut lock_data = Self::read_lock_file(lock_path)?;

        if let Some(scope) = lock_data.scopes.get_mut(scope_path) {
            if scope.remove_dependency(dep_name).is_none() {
                return Err(RhemaError::LockError(format!(
                    "Dependency {} not found in scope {}",
                    dep_name, scope_path
                )));
            }
        } else {
            return Err(RhemaError::LockError(format!(
                "Scope {} not found in lock file",
                scope_path
            )));
        }

        Self::write_lock_file(lock_path, &lock_data)?;

        Ok(lock_data)
    }

    /// Calculate a checksum for file content
    pub fn calculate_file_checksum(file_path: &Path) -> RhemaResult<String> {
        if !file_path.exists() {
            return Err(RhemaError::FileNotFound(format!(
                "File not found: {}",
                file_path.display()
            )));
        }

        let content = std::fs::read(file_path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }

    /// Calculate a checksum for string content
    pub fn calculate_content_checksum(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    /// Check if a lock file is outdated compared to source files
    pub fn is_lock_file_outdated(lock_path: &Path, source_paths: &[PathBuf]) -> RhemaResult<bool> {
        if !lock_path.exists() {
            return Ok(true); // Lock file doesn't exist, so it's outdated
        }

        let lock_data = Self::read_lock_file(lock_path)?;

        for source_path in source_paths {
            if !source_path.exists() {
                continue; // Skip non-existent files
            }

            let current_checksum = Self::calculate_file_checksum(source_path)?;

            // Check if this source file is tracked in the lock file
            if let Some(scope) = lock_data.get_scope(source_path.to_str().unwrap()) {
                if let Some(expected_checksum) = &scope.source_checksum {
                    if current_checksum != *expected_checksum {
                        return Ok(true); // Checksum mismatch, lock file is outdated
                    }
                }
            }
        }

        Ok(false) // Lock file is up to date
    }

    /// Generate a new lock file from source files
    pub fn generate_lock_file(
        lock_path: &Path,
        generated_by: &str,
        scopes: HashMap<String, LockedScope>,
    ) -> RhemaResult<RhemaLock> {
        let start_time = Instant::now();

        let mut lock_data = RhemaLock::new(generated_by);

        // Add all scopes
        for (scope_path, scope) in scopes {
            lock_data.add_scope(scope_path, scope);
        }

        // Update performance metrics
        let generation_time = start_time.elapsed().as_millis() as u64;
        let metrics = LockPerformanceMetrics::new(generation_time);
        lock_data.metadata.set_performance_metrics(metrics);

        // Write the lock file
        Self::write_lock_file(lock_path, &lock_data)?;

        Ok(lock_data)
    }

    /// Merge multiple lock files into one
    pub fn merge_lock_files(
        output_path: &Path,
        lock_files: &[PathBuf],
        conflict_resolution: ConflictResolution,
    ) -> RhemaResult<RhemaLock> {
        if lock_files.is_empty() {
            return Err(RhemaError::LockError(
                "No lock files provided for merging".to_string(),
            ));
        }

        let mut merged_lock = Self::read_lock_file(&lock_files[0])?;

        for lock_file in &lock_files[1..] {
            let lock_data = Self::read_lock_file(lock_file)?;

            // Merge scopes
            for (scope_path, scope) in lock_data.scopes {
                match conflict_resolution {
                    ConflictResolution::Manual => {
                        return Err(RhemaError::LockError(format!(
                            "Manual conflict resolution required for scope {}",
                            scope_path
                        )));
                    }
                    ConflictResolution::Automatic => {
                        // Use the latest scope (based on resolved_at timestamp)
                        if let Some(existing_scope) = merged_lock.get_scope(&scope_path) {
                            if scope.resolved_at > existing_scope.resolved_at {
                                merged_lock.add_scope(scope_path, scope);
                            }
                        } else {
                            merged_lock.add_scope(scope_path, scope);
                        }
                    }
                    ConflictResolution::Prompt => {
                        // For now, treat as manual
                        return Err(RhemaError::LockError(format!(
                            "Prompt conflict resolution required for scope {}",
                            scope_path
                        )));
                    }
                    ConflictResolution::Skip => {
                        // Skip conflicting scopes
                        if !merged_lock.scopes.contains_key(&scope_path) {
                            merged_lock.add_scope(scope_path, scope);
                        }
                    }
                    ConflictResolution::Fail => {
                        if merged_lock.scopes.contains_key(&scope_path) {
                            return Err(RhemaError::LockError(format!(
                                "Conflict detected for scope {}",
                                scope_path
                            )));
                        }
                        merged_lock.add_scope(scope_path, scope);
                    }
                }
            }
        }

        Self::write_lock_file(output_path, &merged_lock)?;

        Ok(merged_lock)
    }

    /// Backup a lock file
    pub fn backup_lock_file(lock_path: &Path) -> RhemaResult<PathBuf> {
        if !lock_path.exists() {
            return Err(RhemaError::FileNotFound(format!(
                "Lock file not found: {}",
                lock_path.display()
            )));
        }

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!(
            "{}.backup.{}",
            lock_path.file_name().unwrap().to_str().unwrap(),
            timestamp
        );
        let backup_path = lock_path.parent().unwrap().join(backup_name);

        std::fs::copy(lock_path, &backup_path)?;

        Ok(backup_path)
    }

    /// Restore a lock file from backup
    pub fn restore_lock_file(backup_path: &Path, target_path: &Path) -> RhemaResult<()> {
        if !backup_path.exists() {
            return Err(RhemaError::FileNotFound(format!(
                "Backup file not found: {}",
                backup_path.display()
            )));
        }

        // Validate the backup file
        Self::read_lock_file(backup_path)?;

        // Copy backup to target
        std::fs::copy(backup_path, target_path)?;

        Ok(())
    }
}

/// Result of lock file validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the lock file is valid
    pub is_valid: bool,

    /// Validation messages (warnings, errors)
    pub messages: Vec<String>,

    /// Time taken for validation in milliseconds
    pub validation_time_ms: u64,
}

impl ValidationResult {
    /// Check if validation was successful
    pub fn is_success(&self) -> bool {
        self.is_valid
    }

    /// Get validation messages as a single string
    pub fn messages_as_string(&self) -> String {
        self.messages.join("\n")
    }

    /// Get the number of validation messages
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

/// Lock file statistics
#[derive(Debug, Clone)]
pub struct LockFileStats {
    /// Total number of scopes in the lock file
    pub total_scopes: usize,

    /// Total number of dependencies across all scopes
    pub total_dependencies: usize,

    /// Number of circular dependencies
    pub circular_dependencies: usize,

    /// Lock file size in bytes
    pub file_size_bytes: u64,

    /// Last modified timestamp
    pub last_modified: chrono::DateTime<Utc>,

    /// Generation time in milliseconds
    pub generation_time_ms: u64,
}

impl LockFileStats {
    /// Calculate the lock file statistics
    pub fn from_lock_file(lock_path: &Path) -> RhemaResult<Self> {
        let lock_data = LockFileOps::read_lock_file(lock_path)?;
        let metadata = std::fs::metadata(lock_path)?;

        Ok(Self {
            total_scopes: lock_data.scopes.len(),
            total_dependencies: lock_data
                .scopes
                .values()
                .map(|s| s.dependencies.len())
                .sum(),
            circular_dependencies: lock_data.metadata.circular_dependencies as usize,
            file_size_bytes: metadata.len(),
            last_modified: chrono::DateTime::from(metadata.modified()?),
            generation_time_ms: lock_data
                .metadata
                .performance_metrics
                .as_ref()
                .map(|m| m.generation_time_ms)
                .unwrap_or(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_and_read_lock_file() {
        let temp_dir = tempdir().unwrap();
        let lock_path = temp_dir.path().join("test.lock");

        // Create a new lock file
        let lock_data = LockFileOps::create_lock_file(&lock_path, "test").unwrap();
        assert_eq!(lock_data.generated_by, "test");
        assert!(lock_path.exists());

        // Read the lock file
        let read_data = LockFileOps::read_lock_file(&lock_path).unwrap();
        assert_eq!(read_data.generated_by, lock_data.generated_by);
        assert_eq!(read_data.checksum, lock_data.checksum);
    }

    #[test]
    fn test_get_or_create_lock_file() {
        let temp_dir = tempdir().unwrap();
        let lock_path = temp_dir.path().join("test.lock");

        // Should create new file
        let lock_data = LockFileOps::get_or_create_lock_file(&lock_path, "test").unwrap();
        assert_eq!(lock_data.generated_by, "test");

        // Should read existing file
        let read_data = LockFileOps::get_or_create_lock_file(&lock_path, "test2").unwrap();
        assert_eq!(read_data.generated_by, "test"); // Should keep original
    }

    #[test]
    fn test_validate_lock_file_integrity() {
        let temp_dir = tempdir().unwrap();
        let lock_path = temp_dir.path().join("test.lock");

        // Create a valid lock file
        LockFileOps::create_lock_file(&lock_path, "test").unwrap();

        // Validate it
        let result = LockFileOps::validate_lock_file_integrity(&lock_path).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.message_count(), 0);
    }

    #[test]
    fn test_calculate_checksums() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        std::fs::write(&test_file, "test content").unwrap();

        // Test file checksum
        let file_checksum = LockFileOps::calculate_file_checksum(&test_file).unwrap();
        assert!(!file_checksum.is_empty());

        // Test content checksum
        let content_checksum = LockFileOps::calculate_content_checksum("test content");
        assert_eq!(file_checksum, content_checksum);
    }

    #[test]
    fn test_lock_file_stats() {
        let temp_dir = tempdir().unwrap();
        let lock_path = temp_dir.path().join("test.lock");

        // Create a lock file with some scopes
        let mut lock_data = RhemaLock::new("test");
        let scope = LockedScope::new("1.0.0", "/test/scope");
        lock_data.add_scope("/test/scope".to_string(), scope);

        LockFileOps::write_lock_file(&lock_path, &lock_data).unwrap();

        // Get stats
        let stats = LockFileStats::from_lock_file(&lock_path).unwrap();
        assert_eq!(stats.total_scopes, 1);
        assert_eq!(stats.total_dependencies, 0);
        assert!(stats.file_size_bytes > 0);
    }
}
