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

use crate::RhemaResult;
use chrono::Utc;
use std::path::Path;

use super::ops::LockFileOps;

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
