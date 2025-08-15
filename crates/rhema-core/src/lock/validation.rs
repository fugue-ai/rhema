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

use crate::schema::lock::{LockedDependency, LockedScope, RhemaLock};
use crate::schema::validation::Validatable;
use crate::RhemaResult;
use std::path::Path;
use std::time::Instant;

use super::ops::LockFileOps;

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

/// Extension trait for lock file validation operations
pub trait LockFileValidation {
    /// Validate a lock file's integrity
    fn validate_lock_file_integrity(lock_path: &Path) -> RhemaResult<ValidationResult>;
}

impl LockFileValidation for LockFileOps {
    /// Validate a lock file's integrity
    fn validate_lock_file_integrity(lock_path: &Path) -> RhemaResult<ValidationResult> {
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
}
