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

use crate::schema::validation::Validatable;
use crate::{
    fileops::{read_yaml_file, write_yaml_file},
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
