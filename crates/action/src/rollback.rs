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

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::{ActionIntent, RollbackInfo};
use crate::error::{ActionError, ActionResult};

/// Backup information
#[derive(Debug, Clone)]
pub struct Backup {
    pub id: String,
    pub intent_id: String,
    pub created_at: DateTime<Utc>,
    pub backup_path: PathBuf,
    pub files_backed_up: Vec<String>,
    pub backup_size: u64,
    pub backup_method: BackupMethod,
}

/// Backup methods
#[derive(Debug, Clone)]
pub enum BackupMethod {
    Git,
    FileCopy,
    Archive,
    Snapshot,
}

/// Rollback manager for handling backups and rollbacks
pub struct RollbackManager {
    backups: Arc<RwLock<HashMap<String, Backup>>>,
    backup_directory: PathBuf,
    max_backups: usize,
}

impl RollbackManager {
    /// Create a new rollback manager
    pub async fn new() -> ActionResult<Self> {
        info!("Initializing Rollback Manager");
        
        let backup_directory = Self::get_backup_directory().await?;
        
        // Create backup directory if it doesn't exist
        tokio::fs::create_dir_all(&backup_directory).await.map_err(|e| {
            ActionError::file_operation(
                backup_directory.clone(),
                format!("Failed to create backup directory: {}", e)
            )
        })?;
        
        let manager = Self {
            backups: Arc::new(RwLock::new(HashMap::new())),
            backup_directory,
            max_backups: 100, // Keep last 100 backups
        };
        
        info!("Rollback Manager initialized successfully");
        Ok(manager)
    }

    /// Initialize the rollback manager (stub)
    pub async fn initialize() -> ActionResult<()> {
        info!("RollbackManager initialized (stub)");
        Ok(())
    }

    /// Shutdown the rollback manager (stub)
    pub async fn shutdown() -> ActionResult<()> {
        info!("RollbackManager shutdown (stub)");
        Ok(())
    }
    
    /// Get the backup directory path
    async fn get_backup_directory() -> ActionResult<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            ActionError::configuration("Could not determine home directory")
        })?;
        
        let backup_dir = home_dir.join(".rhema").join("backups");
        Ok(backup_dir)
    }
    
    /// Create a backup for an action intent
    pub async fn create_backup(&self, intent: &ActionIntent) -> ActionResult<Backup> {
        info!("Creating backup for intent: {}", intent.id);
        
        let backup_id = Uuid::new_v4().simple().to_string();
        let backup_path = self.backup_directory.join(&backup_id);
        
        // Create backup directory
        tokio::fs::create_dir_all(&backup_path).await.map_err(|e| {
            ActionError::file_operation(
                backup_path.clone(),
                format!("Failed to create backup directory: {}", e)
            )
        })?;
        
        let backup_method = self.determine_backup_method(intent);
        let files_backed_up = self.backup_files(intent, &backup_path, &backup_method).await?;
        let backup_size = self.calculate_backup_size(&backup_path).await?;
        
        let backup = Backup {
            id: backup_id.clone(),
            intent_id: intent.id.clone(),
            created_at: Utc::now(),
            backup_path,
            files_backed_up,
            backup_size,
            backup_method,
        };
        
        // Store backup information
        {
            let mut backups = self.backups.write().await;
            backups.insert(backup_id.clone(), backup.clone());
            
            // Clean up old backups if we exceed the limit
            if backups.len() > self.max_backups {
                self.cleanup_old_backups(&mut backups).await?;
            }
        }
        
        info!("Backup created successfully for intent: {} (ID: {})", intent.id, backup_id);
        Ok(backup)
    }
    
    /// Determine the best backup method for the intent
    fn determine_backup_method(&self, intent: &ActionIntent) -> BackupMethod {
        // For now, use file copy as default
        // TODO: Implement intelligent backup method selection based on:
        // - Git repository status
        // - File sizes and types
        // - Intent scope
        BackupMethod::FileCopy
    }
    
    /// Backup files for the intent
    async fn backup_files(&self, intent: &ActionIntent, backup_path: &Path, method: &BackupMethod) -> ActionResult<Vec<String>> {
        let mut files_backed_up = Vec::new();
        
        match method {
            BackupMethod::Git => {
                files_backed_up = self.backup_git(intent, backup_path).await?;
            }
            BackupMethod::FileCopy => {
                files_backed_up = self.backup_file_copy(intent, backup_path).await?;
            }
            BackupMethod::Archive => {
                files_backed_up = self.backup_archive(intent, backup_path).await?;
            }
            BackupMethod::Snapshot => {
                files_backed_up = self.backup_snapshot(intent, backup_path).await?;
            }
        }
        
        Ok(files_backed_up)
    }
    
    /// Backup using Git
    async fn backup_git(&self, intent: &ActionIntent, backup_path: &Path) -> ActionResult<Vec<String>> {
        info!("Creating Git backup for intent: {}", intent.id);
        
        // TODO: Implement Git-based backup
        // - Create a new branch
        // - Commit current state
        // - Store branch reference
        
        Ok(vec!["Git backup created".to_string()])
    }
    
    /// Backup using file copy
    async fn backup_file_copy(&self, intent: &ActionIntent, backup_path: &Path) -> ActionResult<Vec<String>> {
        info!("Creating file copy backup for intent: {}", intent.id);
        
        let mut files_backed_up = Vec::new();
        
        for scope_path in &intent.scope {
            let scope_path = Path::new(scope_path);
            if scope_path.exists() {
                if scope_path.is_file() {
                    // Copy single file
                    let file_name = scope_path.file_name().unwrap().to_string_lossy();
                    let backup_file_path = backup_path.join(&*file_name);
                    
                    tokio::fs::copy(scope_path, &backup_file_path).await.map_err(|e| {
                        ActionError::file_operation(
                            scope_path.to_path_buf(),
                            format!("Failed to copy file: {}", e)
                        )
                    })?;
                    
                    files_backed_up.push(scope_path.to_string_lossy().to_string());
                } else if scope_path.is_dir() {
                    // Copy directory recursively
                    self.copy_directory_recursive(scope_path, backup_path).await?;
                    files_backed_up.push(scope_path.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(files_backed_up)
    }
    
    /// Copy directory recursively
    async fn copy_directory_recursive(&self, src: &Path, dst: &Path) -> ActionResult<()> {
        Box::pin(self.copy_directory_recursive_impl(src, dst)).await
    }

    async fn copy_directory_recursive_impl(&self, src: &Path, dst: &Path) -> ActionResult<()> {
        if !src.is_dir() {
            return Err(ActionError::file_operation(
                src.to_path_buf(),
                "Source is not a directory".to_string()
            ));
        }
        
        // Create destination directory
        tokio::fs::create_dir_all(dst).await.map_err(|e| {
            ActionError::file_operation(
                dst.to_path_buf(),
                format!("Failed to create directory: {}", e)
            )
        })?;
        
        // Read source directory
        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| {
            ActionError::file_operation(
                src.to_path_buf(),
                format!("Failed to read directory: {}", e)
            )
        })?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ActionError::file_operation(
                src.to_path_buf(),
                format!("Failed to read directory entry: {}", e)
            )
        })? {
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let dst_path = dst.join(file_name);
            
            if entry_path.is_file() {
                tokio::fs::copy(&entry_path, &dst_path).await.map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to copy file: {}", e)
                    )
                })?;
            } else if entry_path.is_dir() {
                Box::pin(self.copy_directory_recursive_impl(&entry_path, &dst_path)).await?;
            }
        }
        
        Ok(())
    }
    
    /// Backup using archive
    async fn backup_archive(&self, intent: &ActionIntent, backup_path: &Path) -> ActionResult<Vec<String>> {
        info!("Creating archive backup for intent: {}", intent.id);
        
        // TODO: Implement archive-based backup
        // - Create tar.gz archive
        // - Include all files in scope
        
        Ok(vec!["Archive backup created".to_string()])
    }
    
    /// Backup using snapshot
    async fn backup_snapshot(&self, intent: &ActionIntent, backup_path: &Path) -> ActionResult<Vec<String>> {
        info!("Creating snapshot backup for intent: {}", intent.id);
        
        // TODO: Implement snapshot-based backup
        // - Create filesystem snapshot if supported
        // - Store snapshot reference
        
        Ok(vec!["Snapshot backup created".to_string()])
    }
    
    /// Calculate backup size
    async fn calculate_backup_size(&self, backup_path: &Path) -> ActionResult<u64> {
        let mut total_size = 0u64;
        
        if backup_path.is_file() {
            let metadata = tokio::fs::metadata(backup_path).await.map_err(|e| {
                ActionError::file_operation(
                    backup_path.to_path_buf(),
                    format!("Failed to get file metadata: {}", e)
                )
            })?;
            total_size = metadata.len();
        } else if backup_path.is_dir() {
            total_size = self.calculate_directory_size(backup_path).await?;
        }
        
        Ok(total_size)
    }
    
    /// Calculate directory size recursively
    async fn calculate_directory_size(&self, dir_path: &Path) -> ActionResult<u64> {
        Box::pin(self.calculate_directory_size_impl(dir_path)).await
    }

    async fn calculate_directory_size_impl(&self, dir_path: &Path) -> ActionResult<u64> {
        let mut total_size = 0u64;
        
        let mut entries = tokio::fs::read_dir(dir_path).await.map_err(|e| {
            ActionError::file_operation(
                dir_path.to_path_buf(),
                format!("Failed to read directory: {}", e)
            )
        })?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ActionError::file_operation(
                dir_path.to_path_buf(),
                format!("Failed to read directory entry: {}", e)
            )
        })? {
            let entry_path = entry.path();
            
            if entry_path.is_file() {
                let metadata = tokio::fs::metadata(&entry_path).await.map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to get file metadata: {}", e)
                    )
                })?;
                total_size += metadata.len();
            } else if entry_path.is_dir() {
                total_size += Box::pin(self.calculate_directory_size_impl(&entry_path)).await?;
            }
        }
        
        Ok(total_size)
    }
    
    /// Rollback to a backup
    pub async fn rollback(&self, backup: &Backup) -> ActionResult<RollbackInfo> {
        info!("Rolling back to backup: {}", backup.id);
        
        let start_time = std::time::Instant::now();
        let mut files_restored = Vec::new();
        let mut errors = Vec::new();
        
        match backup.backup_method {
            BackupMethod::Git => {
                files_restored = self.rollback_git(backup).await?;
            }
            BackupMethod::FileCopy => {
                files_restored = self.rollback_file_copy(backup).await?;
            }
            BackupMethod::Archive => {
                files_restored = self.rollback_archive(backup).await?;
            }
            BackupMethod::Snapshot => {
                files_restored = self.rollback_snapshot(backup).await?;
            }
        }
        
        let duration = start_time.elapsed();
        let success = errors.is_empty();
        
        let rollback_info = RollbackInfo {
            method: format!("{:?}", backup.backup_method),
            duration,
            files_restored,
            success,
            errors,
        };
        
        if success {
            info!("Rollback completed successfully for backup: {}", backup.id);
        } else {
            error!("Rollback failed for backup: {}", backup.id);
        }
        
        Ok(rollback_info)
    }
    
    /// Rollback using Git
    async fn rollback_git(&self, backup: &Backup) -> ActionResult<Vec<String>> {
        info!("Rolling back using Git for backup: {}", backup.id);
        
        // TODO: Implement Git-based rollback
        // - Checkout the backup branch
        // - Reset to the backup commit
        
        Ok(vec!["Git rollback completed".to_string()])
    }
    
    /// Rollback using file copy
    async fn rollback_file_copy(&self, backup: &Backup) -> ActionResult<Vec<String>> {
        info!("Rolling back using file copy for backup: {}", backup.id);
        
        let mut files_restored = Vec::new();
        
        // Restore files from backup
        if backup.backup_path.is_dir() {
            let mut entries = tokio::fs::read_dir(&backup.backup_path).await.map_err(|e| {
                ActionError::file_operation(
                    backup.backup_path.clone(),
                    format!("Failed to read backup directory: {}", e)
                )
            })?;
            
            while let Some(entry) = entries.next_entry().await.map_err(|e| {
                ActionError::file_operation(
                    backup.backup_path.clone(),
                    format!("Failed to read backup directory entry: {}", e)
                )
            })? {
                let entry_path = entry.path();
                let file_name = entry_path.file_name().unwrap();
                
                // Restore to original location (assuming same structure)
                let restore_path = Path::new(file_name);
                
                if entry_path.is_file() {
                    tokio::fs::copy(&entry_path, restore_path).await.map_err(|e| {
                        ActionError::file_operation(
                            entry_path.clone(),
                            format!("Failed to restore file: {}", e)
                        )
                    })?;
                    
                    files_restored.push(restore_path.to_string_lossy().to_string());
                } else if entry_path.is_dir() {
                    // TODO: Implement directory restoration
                    files_restored.push(restore_path.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(files_restored)
    }
    
    /// Rollback using archive
    async fn rollback_archive(&self, backup: &Backup) -> ActionResult<Vec<String>> {
        info!("Rolling back using archive for backup: {}", backup.id);
        
        // TODO: Implement archive-based rollback
        // - Extract archive
        // - Restore files
        
        Ok(vec!["Archive rollback completed".to_string()])
    }
    
    /// Rollback using snapshot
    async fn rollback_snapshot(&self, backup: &Backup) -> ActionResult<Vec<String>> {
        info!("Rolling back using snapshot for backup: {}", backup.id);
        
        // TODO: Implement snapshot-based rollback
        // - Restore from snapshot
        
        Ok(vec!["Snapshot rollback completed".to_string()])
    }
    
    /// Get backup by ID
    pub async fn get_backup(&self, backup_id: &str) -> Option<Backup> {
        let backups = self.backups.read().await;
        backups.get(backup_id).cloned()
    }
    
    /// List all backups
    pub async fn list_backups(&self) -> Vec<Backup> {
        let backups = self.backups.read().await;
        backups.values().cloned().collect()
    }
    
    /// List backups for an intent
    pub async fn list_backups_for_intent(&self, intent_id: &str) -> Vec<Backup> {
        let backups = self.backups.read().await;
        backups
            .values()
            .filter(|backup| backup.intent_id == intent_id)
            .cloned()
            .collect()
    }
    
    /// Delete a backup
    pub async fn delete_backup(&self, backup_id: &str) -> ActionResult<()> {
        info!("Deleting backup: {}", backup_id);
        
        let backup = {
            let mut backups = self.backups.write().await;
            backups.remove(backup_id)
        };
        
        if let Some(backup) = backup {
            // Remove backup files
            if backup.backup_path.exists() {
                if backup.backup_path.is_file() {
                    tokio::fs::remove_file(&backup.backup_path).await.map_err(|e| {
                        ActionError::file_operation(
                            backup.backup_path.clone(),
                            format!("Failed to delete backup file: {}", e)
                        )
                    })?;
                } else if backup.backup_path.is_dir() {
                    tokio::fs::remove_dir_all(&backup.backup_path).await.map_err(|e| {
                        ActionError::file_operation(
                            backup.backup_path.clone(),
                            format!("Failed to delete backup directory: {}", e)
                        )
                    })?;
                }
            }
            
            info!("Backup deleted successfully: {}", backup_id);
        } else {
            warn!("Backup not found: {}", backup_id);
        }
        
        Ok(())
    }
    
    /// Clean up old backups
    async fn cleanup_old_backups(&self, backups: &mut HashMap<String, Backup>) -> ActionResult<()> {
        info!("Cleaning up old backups");
        
        // Sort backups by creation time (oldest first)
        let mut backup_list: Vec<_> = backups.values().cloned().collect();
        backup_list.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        // Remove oldest backups
        let to_remove = backup_list.len().saturating_sub(self.max_backups);
        for backup in backup_list.into_iter().take(to_remove) {
            self.delete_backup(&backup.id).await?;
            backups.remove(&backup.id);
        }
        
        info!("Cleaned up {} old backups", to_remove);
        Ok(())
    }
    
    /// Get backup statistics
    pub async fn get_backup_stats(&self) -> BackupStats {
        let backups = self.backups.read().await;
        
        let total_backups = backups.len();
        let total_size: u64 = backups.values().map(|b| b.backup_size).sum();
        let oldest_backup = backups.values().map(|b| b.created_at).min();
        let newest_backup = backups.values().map(|b| b.created_at).max();
        
        BackupStats {
            total_backups,
            total_size,
            oldest_backup,
            newest_backup,
            max_backups: self.max_backups,
        }
    }
}

/// Backup statistics
#[derive(Debug, Clone)]
pub struct BackupStats {
    pub total_backups: usize,
    pub total_size: u64,
    pub oldest_backup: Option<DateTime<Utc>>,
    pub newest_backup: Option<DateTime<Utc>>,
    pub max_backups: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_rollback_manager_creation() {
        let manager = RollbackManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_backup_creation() {
        let manager = RollbackManager::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-backup",
            ActionType::Test,
            "Test backup creation",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        let backup = manager.create_backup(&intent).await;
        assert!(backup.is_ok());
        
        let backup = backup.unwrap();
        assert_eq!(backup.intent_id, "test-backup");
        assert!(backup.backup_path.exists());
    }

    #[tokio::test]
    async fn test_backup_retrieval() {
        let manager = RollbackManager::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-retrieval",
            ActionType::Test,
            "Test backup retrieval",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        let backup = manager.create_backup(&intent).await.unwrap();
        
        let retrieved_backup = manager.get_backup(&backup.id).await;
        assert!(retrieved_backup.is_some());
        
        let retrieved_backup = retrieved_backup.unwrap();
        assert_eq!(retrieved_backup.id, backup.id);
        assert_eq!(retrieved_backup.intent_id, backup.intent_id);
    }

    #[tokio::test]
    async fn test_backup_listing() {
        let manager = RollbackManager::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-listing",
            ActionType::Test,
            "Test backup listing",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        let _backup = manager.create_backup(&intent).await.unwrap();
        
        let backups = manager.list_backups().await;
        assert!(!backups.is_empty());
        
        let intent_backups = manager.list_backups_for_intent("test-listing").await;
        assert!(!intent_backups.is_empty());
    }

    #[tokio::test]
    async fn test_backup_stats() {
        let manager = RollbackManager::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-stats",
            ActionType::Test,
            "Test backup stats",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        let _backup = manager.create_backup(&intent).await.unwrap();
        
        let stats = manager.get_backup_stats().await;
        assert!(stats.total_backups > 0);
        assert!(stats.total_size > 0);
    }
} 