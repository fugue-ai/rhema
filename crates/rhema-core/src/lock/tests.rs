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

use super::*;
use crate::{LockedScope, RhemaLock};
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

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
