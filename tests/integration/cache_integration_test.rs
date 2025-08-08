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

use rhema_core::{RhemaError, RhemaResult};
use rhema_core::lock::LockFileOps;
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;
use std::collections::HashMap;

#[test]
fn test_lock_file_initialization() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let lock_path = temp_dir.path().join("test.lock");
    
    // Create a new lock file
    let lock_data = LockFileOps::create_lock_file(&lock_path, "test")?;
    assert_eq!(lock_data.generated_by, "test");
    assert!(lock_path.exists());
    
    Ok(())
}

#[test]
fn test_lock_file_operations() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let lock_path = temp_dir.path().join("test.lock");
    
    // Create a new lock file
    let lock_data = LockFileOps::create_lock_file(&lock_path, "test")?;
    
    // Read the lock file
    let read_data = LockFileOps::read_lock_file(&lock_path)?;
    assert_eq!(read_data.generated_by, lock_data.generated_by);
    
    // Validate lock file integrity
    let validation = LockFileOps::validate_lock_file_integrity(&lock_path)?;
    assert!(validation.is_valid);
    
    Ok(())
}

#[test]
fn test_lock_file_with_temp_directory() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let lock_path = temp_dir.path().join("rhema.lock");
    
    // Test get or create functionality
    let lock_data = LockFileOps::get_or_create_lock_file(&lock_path, "test")?;
    assert_eq!(lock_data.generated_by, "test");
    assert!(lock_path.exists());
    
    // Test that subsequent calls return the same data
    let lock_data2 = LockFileOps::get_or_create_lock_file(&lock_path, "test2")?;
    assert_eq!(lock_data2.generated_by, "test"); // Should be the original
    
    Ok(())
}

#[test]
fn test_lock_file_validation() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let lock_path = temp_dir.path().join("test.lock");
    
    // Create a valid lock file
    LockFileOps::create_lock_file(&lock_path, "test")?;
    
    // Validate it
    let validation = LockFileOps::validate_lock_file_integrity(&lock_path)?;
    assert!(validation.is_valid);
    assert!(validation.messages.is_empty());
    
    Ok(())
}

#[test]
fn test_lock_file_checksum_calculation() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let test_file = temp_dir.path().join("test.txt");
    
    // Create a test file
    fs::write(&test_file, "test content")?;
    
    // Calculate file checksum
    let checksum = LockFileOps::calculate_file_checksum(&test_file)?;
    assert!(!checksum.is_empty());
    
    // Calculate content checksum
    let content_checksum = LockFileOps::calculate_content_checksum("test content");
    assert!(!content_checksum.is_empty());
    
    Ok(())
}

#[test]
fn test_lock_file_backup_and_restore() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let lock_path = temp_dir.path().join("test.lock");
    
    // Create a lock file
    LockFileOps::create_lock_file(&lock_path, "test")?;
    
    // Backup the lock file
    let backup_path = LockFileOps::backup_lock_file(&lock_path)?;
    assert!(backup_path.exists());
    
    // Remove the original
    fs::remove_file(&lock_path)?;
    assert!(!lock_path.exists());
    
    // Restore from backup
    LockFileOps::restore_lock_file(&backup_path, &lock_path)?;
    assert!(lock_path.exists());
    
    // Verify the restored file is valid
    let validation = LockFileOps::validate_lock_file_integrity(&lock_path)?;
    assert!(validation.is_valid);
    
    Ok(())
}

#[test]
fn test_lock_file_error_handling() {
    let temp_dir = tempfile::tempdir().unwrap();
    let non_existent_path = temp_dir.path().join("non_existent.lock");
    
    // Try to read a non-existent lock file
    let result = LockFileOps::read_lock_file(&non_existent_path);
    assert!(result.is_err());
    
    // Try to validate a non-existent lock file
    let result = LockFileOps::validate_lock_file_integrity(&non_existent_path);
    assert!(result.is_err());
}

#[test]
fn test_lock_file_concurrent_access() -> RhemaResult<()> {
    let temp_dir = tempfile::tempdir()?;
    let lock_path = temp_dir.path().join("test.lock");
    
    // Create a lock file
    LockFileOps::create_lock_file(&lock_path, "test")?;
    
    // Simulate concurrent access by reading multiple times
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let path = lock_path.clone();
        let handle = std::thread::spawn(move || {
            LockFileOps::read_lock_file(&path)
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result.is_ok());
    }
    
    Ok(())
} 