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
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::error::{ActionError, ActionResult};
use crate::schema::{ActionIntent, RollbackInfo};

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
        tokio::fs::create_dir_all(&backup_directory)
            .await
            .map_err(|e| {
                ActionError::file_operation(
                    backup_directory.clone(),
                    format!("Failed to create backup directory: {}", e),
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
        let home_dir = dirs::home_dir()
            .ok_or_else(|| ActionError::configuration("Could not determine home directory"))?;

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
                format!("Failed to create backup directory: {}", e),
            )
        })?;

        let backup_method = self.determine_backup_method(intent);
        let files_backed_up = self
            .backup_files(intent, &backup_path, &backup_method)
            .await?;
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

        info!(
            "Backup created successfully for intent: {} (ID: {})",
            intent.id, backup_id
        );
        Ok(backup)
    }

    /// Determine the best backup method for the intent
    fn determine_backup_method(&self, intent: &ActionIntent) -> BackupMethod {
        // Check if we're in a Git repository
        if self.is_git_repository().unwrap_or(false) {
            // Use Git backup for Git repositories
            return BackupMethod::Git;
        }

        // Analyze file sizes and types
        let total_size = self.calculate_intent_scope_size(intent).unwrap_or(0);
        let file_count = intent.scope.len();

        // Use archive for large numbers of files or large total size
        if file_count > 50 || total_size > 100 * 1024 * 1024 {
            // 100MB
            return BackupMethod::Archive;
        }

        // Use snapshot for filesystem operations if supported
        if self.supports_snapshots().unwrap_or(false) && self.is_filesystem_operation(intent) {
            return BackupMethod::Snapshot;
        }

        // Default to file copy for small operations
        BackupMethod::FileCopy
    }

    /// Check if current directory is a Git repository
    fn is_git_repository(&self) -> Result<bool, std::io::Error> {
        let current_dir = std::env::current_dir()?;
        let git_dir = current_dir.join(".git");
        Ok(git_dir.exists() && git_dir.is_dir())
    }

    /// Calculate total size of files in intent scope
    fn calculate_intent_scope_size(&self, intent: &ActionIntent) -> Result<u64, std::io::Error> {
        let mut total_size = 0u64;

        for scope_path in &intent.scope {
            let path = std::path::Path::new(scope_path);
            if path.exists() {
                if path.is_file() {
                    total_size += path.metadata()?.len();
                } else if path.is_dir() {
                    total_size += self.calculate_directory_size_sync(path)?;
                }
            }
        }

        Ok(total_size)
    }

    /// Calculate directory size synchronously
    fn calculate_directory_size_sync(
        &self,
        dir_path: &std::path::Path,
    ) -> Result<u64, std::io::Error> {
        let mut total_size = 0u64;

        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                total_size += entry_path.metadata()?.len();
            } else if entry_path.is_dir() {
                total_size += self.calculate_directory_size_sync(&entry_path)?;
            }
        }

        Ok(total_size)
    }

    /// Check if filesystem snapshots are supported
    fn supports_snapshots(&self) -> Result<bool, std::io::Error> {
        // Check for common snapshot-capable filesystems
        let _current_dir = std::env::current_dir()?;

        // This is a simplified check - in practice, you'd want to check the actual filesystem type
        // For now, we'll assume snapshots are not supported by default
        Ok(false)
    }

    /// Check if the intent involves filesystem operations
    fn is_filesystem_operation(&self, intent: &ActionIntent) -> bool {
        // Check if the action type involves filesystem changes
        matches!(
            intent.action_type,
            crate::schema::ActionType::Refactor
                | crate::schema::ActionType::Feature
                | crate::schema::ActionType::BugFix
                | crate::schema::ActionType::Documentation
                | crate::schema::ActionType::Test
                | crate::schema::ActionType::Configuration
                | crate::schema::ActionType::Dependency
                | crate::schema::ActionType::Security
                | crate::schema::ActionType::Performance
                | crate::schema::ActionType::Cleanup
                | crate::schema::ActionType::Migration
        )
    }

    /// Backup files for the intent
    async fn backup_files(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
        method: &BackupMethod,
    ) -> ActionResult<Vec<String>> {
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
    async fn backup_git(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
    ) -> ActionResult<Vec<String>> {
        info!("Creating Git backup for intent: {}", intent.id);

        let mut files_backed_up = Vec::new();

        // Create backup metadata file
        let metadata_path = backup_path.join("backup_metadata.json");
        let metadata = serde_json::json!({
            "intent_id": intent.id,
            "created_at": Utc::now().to_rfc3339(),
            "action_type": format!("{:?}", intent.action_type),
            "description": intent.description,
            "scope": intent.scope,
            "safety_level": format!("{:?}", intent.safety_level)
        });

        tokio::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)
            .await
            .map_err(|e| {
                ActionError::file_operation(
                    metadata_path.clone(),
                    format!("Failed to write backup metadata: {}", e),
                )
            })?;

        files_backed_up.push(metadata_path.to_string_lossy().to_string());

        // Execute Git commands to create backup
        let backup_branch_name = format!("backup-{}", intent.id);

        // Create and switch to backup branch
        let create_branch_result = tokio::process::Command::new("git")
            .args(&["checkout", "-b", &backup_branch_name])
            .output()
            .await;

        match create_branch_result {
            Ok(output) => {
                if output.status.success() {
                    info!("Created backup branch: {}", backup_branch_name);
                } else {
                    // Branch might already exist, try to switch to it
                    let switch_result = tokio::process::Command::new("git")
                        .args(&["checkout", &backup_branch_name])
                        .output()
                        .await;

                    if let Ok(switch_output) = switch_result {
                        if !switch_output.status.success() {
                            warn!(
                                "Failed to switch to backup branch: {}",
                                String::from_utf8_lossy(&switch_output.stderr)
                            );
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Failed to create backup branch: {}", e);
            }
        }

        // Add all files in scope to Git
        for scope_path in &intent.scope {
            let path = Path::new(scope_path);
            if path.exists() {
                let add_result = tokio::process::Command::new("git")
                    .args(&["add", scope_path])
                    .output()
                    .await;

                if let Ok(add_output) = add_result {
                    if add_output.status.success() {
                        files_backed_up.push(scope_path.clone());
                    } else {
                        warn!(
                            "Failed to add {} to Git: {}",
                            scope_path,
                            String::from_utf8_lossy(&add_output.stderr)
                        );
                    }
                }
            }
        }

        // Commit the backup
        let commit_message = format!("Backup for intent {}: {}", intent.id, intent.description);
        let commit_result = tokio::process::Command::new("git")
            .args(&["commit", "-m", &commit_message])
            .output()
            .await;

        if let Ok(commit_output) = commit_result {
            if commit_output.status.success() {
                info!("Git backup committed successfully");

                // Store branch reference in backup metadata
                let branch_ref_path = backup_path.join("branch_ref.txt");
                tokio::fs::write(&branch_ref_path, &backup_branch_name)
                    .await
                    .map_err(|e| {
                        ActionError::file_operation(
                            branch_ref_path.clone(),
                            format!("Failed to write branch reference: {}", e),
                        )
                    })?;

                files_backed_up.push(branch_ref_path.to_string_lossy().to_string());
            } else {
                warn!(
                    "Failed to commit Git backup: {}",
                    String::from_utf8_lossy(&commit_output.stderr)
                );
            }
        }

        // Switch back to original branch (usually main/master)
        let _ = tokio::process::Command::new("git")
            .args(&["checkout", "-"])
            .output()
            .await;

        Ok(files_backed_up)
    }

    /// Backup using file copy
    async fn backup_file_copy(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
    ) -> ActionResult<Vec<String>> {
        info!("Creating file copy backup for intent: {}", intent.id);

        let mut files_backed_up = Vec::new();

        for scope_path in &intent.scope {
            let scope_path = Path::new(scope_path);
            if scope_path.exists() {
                if scope_path.is_file() {
                    // Copy single file
                    let file_name = scope_path.file_name().unwrap().to_string_lossy();
                    let backup_file_path = backup_path.join(&*file_name);

                    tokio::fs::copy(scope_path, &backup_file_path)
                        .await
                        .map_err(|e| {
                            ActionError::file_operation(
                                scope_path.to_path_buf(),
                                format!("Failed to copy file: {}", e),
                            )
                        })?;

                    files_backed_up.push(scope_path.to_string_lossy().to_string());
                } else if scope_path.is_dir() {
                    // Copy directory recursively
                    self.copy_directory_recursive(scope_path, backup_path)
                        .await?;
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
                "Source is not a directory".to_string(),
            ));
        }

        // Create destination directory
        tokio::fs::create_dir_all(dst).await.map_err(|e| {
            ActionError::file_operation(
                dst.to_path_buf(),
                format!("Failed to create directory: {}", e),
            )
        })?;

        // Read source directory
        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| {
            ActionError::file_operation(
                src.to_path_buf(),
                format!("Failed to read directory: {}", e),
            )
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ActionError::file_operation(
                src.to_path_buf(),
                format!("Failed to read directory entry: {}", e),
            )
        })? {
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let dst_path = dst.join(file_name);

            if entry_path.is_file() {
                tokio::fs::copy(&entry_path, &dst_path).await.map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to copy file: {}", e),
                    )
                })?;
            } else if entry_path.is_dir() {
                Box::pin(self.copy_directory_recursive_impl(&entry_path, &dst_path)).await?;
            }
        }

        Ok(())
    }

    /// Backup using archive
    async fn backup_archive(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
    ) -> ActionResult<Vec<String>> {
        info!("Creating archive backup for intent: {}", intent.id);

        let mut files_backed_up = Vec::new();
        let archive_path = backup_path.join("backup.tar.gz");

        // Create a temporary directory for staging files
        let temp_dir = tempfile::tempdir().map_err(|e| {
            ActionError::file_operation(
                backup_path.to_path_buf(),
                format!("Failed to create temporary directory: {}", e),
            )
        })?;

        // Copy files to temporary directory
        for scope_path in &intent.scope {
            let path = Path::new(scope_path);
            if path.exists() {
                let temp_path = temp_dir.path().join(path.file_name().unwrap_or_default());

                if path.is_file() {
                    tokio::fs::copy(path, &temp_path).await.map_err(|e| {
                        ActionError::file_operation(
                            path.to_path_buf(),
                            format!("Failed to copy file to temp directory: {}", e),
                        )
                    })?;
                    files_backed_up.push(scope_path.clone());
                } else if path.is_dir() {
                    self.copy_directory_recursive(path, temp_dir.path()).await?;
                    files_backed_up.push(scope_path.clone());
                }
            }
        }

        // Create tar.gz archive using synchronous I/O
        let archive_file = std::fs::File::create(&archive_path).map_err(|e| {
            ActionError::file_operation(
                archive_path.clone(),
                format!("Failed to create archive file: {}", e),
            )
        })?;

        let gz_encoder = flate2::write::GzEncoder::new(
            std::io::BufWriter::new(archive_file),
            flate2::Compression::default(),
        );

        let mut tar_builder = tar::Builder::new(gz_encoder);

        // Add files from temp directory to archive
        for entry in std::fs::read_dir(temp_dir.path()).map_err(|e| {
            ActionError::file_operation(
                temp_dir.path().to_path_buf(),
                format!("Failed to read temp directory: {}", e),
            )
        })? {
            let entry = entry.map_err(|e| {
                ActionError::file_operation(
                    temp_dir.path().to_path_buf(),
                    format!("Failed to read temp directory entry: {}", e),
                )
            })?;

            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap().to_string_lossy();

            if entry_path.is_file() {
                let mut file = std::fs::File::open(&entry_path).map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to open file for archiving: {}", e),
                    )
                })?;

                let mut header = tar::Header::new_gnu();
                header.set_path(&*file_name).map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to set archive path: {}", e),
                    )
                })?;

                let metadata = file.metadata().map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to get file metadata: {}", e),
                    )
                })?;

                header.set_size(metadata.len());
                header.set_mode(0o644);
                header.set_cksum();

                tar_builder.append(&header, &mut file).map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to append file to archive: {}", e),
                    )
                })?;
            }
        }

        // Finish the archive
        tar_builder.finish().map_err(|e| {
            ActionError::file_operation(
                archive_path.clone(),
                format!("Failed to finish archive: {}", e),
            )
        })?;

        // Create backup metadata
        let metadata_path = backup_path.join("backup_metadata.json");
        let metadata = serde_json::json!({
            "intent_id": intent.id,
            "created_at": Utc::now().to_rfc3339(),
            "action_type": format!("{:?}", intent.action_type),
            "description": intent.description,
            "scope": intent.scope,
            "safety_level": format!("{:?}", intent.safety_level),
            "archive_path": archive_path.to_string_lossy(),
            "files_backed_up": files_backed_up
        });

        tokio::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)
            .await
            .map_err(|e| {
                ActionError::file_operation(
                    metadata_path.clone(),
                    format!("Failed to write backup metadata: {}", e),
                )
            })?;

        files_backed_up.push(metadata_path.to_string_lossy().to_string());
        files_backed_up.push(archive_path.to_string_lossy().to_string());

        info!(
            "Archive backup created successfully: {}",
            archive_path.display()
        );
        Ok(files_backed_up)
    }

    /// Backup using snapshot
    async fn backup_snapshot(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
    ) -> ActionResult<Vec<String>> {
        info!("Creating snapshot backup for intent: {}", intent.id);

        let mut files_backed_up = Vec::new();

        // Create backup metadata
        let metadata_path = backup_path.join("backup_metadata.json");
        let metadata = serde_json::json!({
            "intent_id": intent.id,
            "created_at": Utc::now().to_rfc3339(),
            "action_type": format!("{:?}", intent.action_type),
            "description": intent.description,
            "scope": intent.scope,
            "safety_level": format!("{:?}", intent.safety_level),
            "backup_method": "snapshot"
        });

        tokio::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)
            .await
            .map_err(|e| {
                ActionError::file_operation(
                    metadata_path.clone(),
                    format!("Failed to write backup metadata: {}", e),
                )
            })?;

        files_backed_up.push(metadata_path.to_string_lossy().to_string());

        // Try to create filesystem snapshot
        let snapshot_result = self.create_filesystem_snapshot(intent, backup_path).await;

        match snapshot_result {
            Ok(snapshot_info) => {
                // Store snapshot information
                let snapshot_info_path = backup_path.join("snapshot_info.json");
                tokio::fs::write(
                    &snapshot_info_path,
                    serde_json::to_string_pretty(&snapshot_info)?,
                )
                .await
                .map_err(|e| {
                    ActionError::file_operation(
                        snapshot_info_path.clone(),
                        format!("Failed to write snapshot info: {}", e),
                    )
                })?;

                files_backed_up.push(snapshot_info_path.to_string_lossy().to_string());
                info!("Filesystem snapshot created successfully");
            }
            Err(e) => {
                warn!(
                    "Failed to create filesystem snapshot: {}. Falling back to file copy.",
                    e
                );

                // Fall back to file copy if snapshot fails
                let fallback_files = self.backup_file_copy(intent, backup_path).await?;
                files_backed_up.extend(fallback_files);
            }
        }

        Ok(files_backed_up)
    }

    /// Create filesystem snapshot
    async fn create_filesystem_snapshot(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
    ) -> ActionResult<serde_json::Value> {
        // Check if we're on a supported platform/filesystem
        #[cfg(target_os = "linux")]
        {
            // Try to use btrfs snapshots if available
            if self.is_btrfs_filesystem().await? {
                return self.create_btrfs_snapshot(intent, backup_path).await;
            }
        }

        #[cfg(target_os = "macos")]
        {
            // Try to use APFS snapshots if available
            if self.is_apfs_filesystem().await? {
                return self.create_apfs_snapshot(intent, backup_path).await;
            }
        }

        #[cfg(target_os = "windows")]
        {
            // Try to use Volume Shadow Copy Service (VSS) if available
            return self.create_vss_snapshot(intent, backup_path).await;
        }

        // No supported snapshot mechanism found
        Err(ActionError::configuration(
            "No supported filesystem snapshot mechanism available",
        ))
    }

    #[cfg(target_os = "linux")]
    async fn is_btrfs_filesystem(&self) -> ActionResult<bool> {
        // Check if current directory is on a btrfs filesystem
        let output = tokio::process::Command::new("df")
            .args(&["-T", "."])
            .output()
            .await
            .map_err(|e| {
                ActionError::configuration(format!("Failed to check filesystem: {}", e))
            })?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            Ok(output_str.contains("btrfs"))
        } else {
            Ok(false)
        }
    }

    #[cfg(target_os = "linux")]
    async fn create_btrfs_snapshot(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
    ) -> ActionResult<serde_json::Value> {
        let snapshot_name = format!("backup-{}", intent.id);
        let current_dir = std::env::current_dir().map_err(|e| {
            ActionError::configuration(format!("Failed to get current directory: {}", e))
        })?;

        // Create btrfs snapshot
        let output = tokio::process::Command::new("btrfs")
            .args(&[
                "subvolume",
                "snapshot",
                current_dir.to_str().unwrap(),
                &snapshot_name,
            ])
            .output()
            .await
            .map_err(|e| {
                ActionError::configuration(format!("Failed to create btrfs snapshot: {}", e))
            })?;

        if output.status.success() {
            let snapshot_info = serde_json::json!({
                "snapshot_type": "btrfs",
                "snapshot_name": snapshot_name,
                "source_path": current_dir.to_string_lossy(),
                "created_at": Utc::now().to_rfc3339()
            });
            Ok(snapshot_info)
        } else {
            Err(ActionError::configuration(format!(
                "btrfs snapshot creation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(target_os = "macos")]
    async fn is_apfs_filesystem(&self) -> ActionResult<bool> {
        // Check if current directory is on an APFS filesystem
        let output = tokio::process::Command::new("diskutil")
            .args(&["info", "."])
            .output()
            .await
            .map_err(|e| {
                ActionError::configuration(format!("Failed to check filesystem: {}", e))
            })?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            Ok(output_str.contains("APFS"))
        } else {
            Ok(false)
        }
    }

    #[cfg(target_os = "macos")]
    async fn create_apfs_snapshot(
        &self,
        intent: &ActionIntent,
        _backup_path: &Path,
    ) -> ActionResult<serde_json::Value> {
        let snapshot_name = format!("backup-{}", intent.id);

        // Get the APFS volume
        let output = tokio::process::Command::new("diskutil")
            .args(&["apfs", "listSnapshots", "."])
            .output()
            .await
            .map_err(|e| {
                ActionError::configuration(format!("Failed to list APFS snapshots: {}", e))
            })?;

        if output.status.success() {
            let snapshot_info = serde_json::json!({
                "snapshot_type": "apfs",
                "snapshot_name": snapshot_name,
                "created_at": Utc::now().to_rfc3339(),
                "note": "APFS snapshots are typically created automatically by Time Machine or manually via tmutil"
            });
            Ok(snapshot_info)
        } else {
            Err(ActionError::configuration(
                "APFS snapshot creation not supported in this environment",
            ))
        }
    }

    #[cfg(target_os = "windows")]
    async fn create_vss_snapshot(
        &self,
        intent: &ActionIntent,
        backup_path: &Path,
    ) -> ActionResult<serde_json::Value> {
        // Windows VSS implementation would go here
        // This is a placeholder - actual VSS implementation would require Windows-specific APIs
        let snapshot_info = serde_json::json!({
            "snapshot_type": "vss",
            "snapshot_name": format!("backup-{}", intent.id),
            "created_at": Utc::now().to_rfc3339(),
            "note": "VSS snapshot creation requires Windows-specific implementation"
        });
        Ok(snapshot_info)
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    async fn create_filesystem_snapshot(
        &self,
        _intent: &ActionIntent,
        _backup_path: &Path,
    ) -> ActionResult<serde_json::Value> {
        Err(ActionError::configuration(
            "Filesystem snapshots not supported on this platform",
        ))
    }

    /// Calculate backup size
    async fn calculate_backup_size(&self, backup_path: &Path) -> ActionResult<u64> {
        let mut total_size = 0u64;

        if backup_path.is_file() {
            let metadata = tokio::fs::metadata(backup_path).await.map_err(|e| {
                ActionError::file_operation(
                    backup_path.to_path_buf(),
                    format!("Failed to get file metadata: {}", e),
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
                format!("Failed to read directory: {}", e),
            )
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ActionError::file_operation(
                dir_path.to_path_buf(),
                format!("Failed to read directory entry: {}", e),
            )
        })? {
            let entry_path = entry.path();

            if entry_path.is_file() {
                let metadata = tokio::fs::metadata(&entry_path).await.map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to get file metadata: {}", e),
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
        let errors = Vec::new();

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

        let mut files_restored = Vec::new();

        // Read backup metadata to get branch information
        let metadata_path = backup.backup_path.join("backup_metadata.json");
        let branch_ref_path = backup.backup_path.join("branch_ref.txt");

        if let Ok(branch_name) = tokio::fs::read_to_string(&branch_ref_path).await {
            let branch_name = branch_name.trim();

            // Stash any current changes
            let stash_result = tokio::process::Command::new("git")
                .args(&[
                    "stash",
                    "push",
                    "-m",
                    &format!("Pre-rollback stash for backup {}", backup.id),
                ])
                .output()
                .await;

            if let Ok(stash_output) = stash_result {
                if !stash_output.status.success() {
                    warn!(
                        "Failed to stash changes: {}",
                        String::from_utf8_lossy(&stash_output.stderr)
                    );
                }
            }

            // Checkout the backup branch
            let checkout_result = tokio::process::Command::new("git")
                .args(&["checkout", branch_name])
                .output()
                .await;

            match checkout_result {
                Ok(checkout_output) => {
                    if checkout_output.status.success() {
                        info!("Successfully checked out backup branch: {}", branch_name);

                        // Get the list of files that were backed up
                        let status_result = tokio::process::Command::new("git")
                            .args(&["ls-files"])
                            .output()
                            .await;

                        if let Ok(status_output) = status_result {
                            if status_output.status.success() {
                                let files = String::from_utf8_lossy(&status_output.stdout);
                                for line in files.lines() {
                                    if !line.trim().is_empty() {
                                        files_restored.push(line.trim().to_string());
                                    }
                                }
                            }
                        }

                        // Reset to the backup commit (HEAD of the backup branch)
                        let reset_result = tokio::process::Command::new("git")
                            .args(&["reset", "--hard", "HEAD"])
                            .output()
                            .await;

                        if let Ok(reset_output) = reset_result {
                            if reset_output.status.success() {
                                info!("Successfully reset to backup commit");
                            } else {
                                warn!(
                                    "Failed to reset to backup commit: {}",
                                    String::from_utf8_lossy(&reset_output.stderr)
                                );
                            }
                        }

                        // Switch back to the original branch (usually main/master)
                        let _ = tokio::process::Command::new("git")
                            .args(&["checkout", "-"])
                            .output()
                            .await;

                        // Apply the stashed changes if they exist
                        let stash_list_result = tokio::process::Command::new("git")
                            .args(&["stash", "list"])
                            .output()
                            .await;

                        if let Ok(stash_list_output) = stash_list_result {
                            if stash_list_output.status.success() {
                                let stash_list = String::from_utf8_lossy(&stash_list_output.stdout);
                                if !stash_list.trim().is_empty() {
                                    let pop_result = tokio::process::Command::new("git")
                                        .args(&["stash", "pop"])
                                        .output()
                                        .await;

                                    if let Ok(pop_output) = pop_result {
                                        if !pop_output.status.success() {
                                            warn!(
                                                "Failed to pop stashed changes: {}",
                                                String::from_utf8_lossy(&pop_output.stderr)
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        warn!(
                            "Failed to checkout backup branch: {}",
                            String::from_utf8_lossy(&checkout_output.stderr)
                        );

                        // Try to restore files directly from the backup
                        files_restored = self.rollback_file_copy(backup).await?;
                    }
                }
                Err(e) => {
                    warn!(
                        "Failed to execute git checkout: {}. Falling back to file copy.",
                        e
                    );
                    files_restored = self.rollback_file_copy(backup).await?;
                }
            }
        } else {
            warn!("Could not read branch reference. Falling back to file copy.");
            files_restored = self.rollback_file_copy(backup).await?;
        }

        Ok(files_restored)
    }

    /// Rollback using file copy
    async fn rollback_file_copy(&self, backup: &Backup) -> ActionResult<Vec<String>> {
        info!("Rolling back using file copy for backup: {}", backup.id);

        let mut files_restored = Vec::new();

        // Restore files from backup
        if backup.backup_path.is_dir() {
            let mut entries = tokio::fs::read_dir(&backup.backup_path)
                .await
                .map_err(|e| {
                    ActionError::file_operation(
                        backup.backup_path.clone(),
                        format!("Failed to read backup directory: {}", e),
                    )
                })?;

            while let Some(entry) = entries.next_entry().await.map_err(|e| {
                ActionError::file_operation(
                    backup.backup_path.clone(),
                    format!("Failed to read backup directory entry: {}", e),
                )
            })? {
                let entry_path = entry.path();
                let file_name = entry_path.file_name().unwrap();

                // Restore to original location (assuming same structure)
                let restore_path = Path::new(file_name);

                if entry_path.is_file() {
                    tokio::fs::copy(&entry_path, restore_path)
                        .await
                        .map_err(|e| {
                            ActionError::file_operation(
                                entry_path.clone(),
                                format!("Failed to restore file: {}", e),
                            )
                        })?;

                    files_restored.push(restore_path.to_string_lossy().to_string());
                } else if entry_path.is_dir() {
                    // Restore directory recursively
                    let restore_dir_path = Path::new(file_name);
                    self.restore_directory_recursive(&entry_path, restore_dir_path)
                        .await?;
                    files_restored.push(restore_dir_path.to_string_lossy().to_string());
                }
            }
        }

        Ok(files_restored)
    }

    /// Rollback using archive
    async fn rollback_archive(&self, backup: &Backup) -> ActionResult<Vec<String>> {
        info!("Rolling back using archive for backup: {}", backup.id);

        let mut files_restored = Vec::new();

        // Find the archive file
        let archive_path = backup.backup_path.join("backup.tar.gz");
        if !archive_path.exists() {
            return Err(ActionError::file_operation(
                archive_path.clone(),
                "Archive file not found".to_string(),
            ));
        }

        // Read backup metadata to get original file paths
        let _metadata_path = backup.backup_path.join("backup_metadata.json");
        let metadata_content = tokio::fs::read_to_string(&_metadata_path)
            .await
            .map_err(|e| {
                ActionError::file_operation(
                    _metadata_path.clone(),
                    format!("Failed to read backup metadata: {}", e),
                )
            })?;

        let _metadata: serde_json::Value =
            serde_json::from_str(&metadata_content).map_err(|e| {
                ActionError::file_operation(
                    _metadata_path.clone(),
                    format!("Failed to parse backup metadata: {}", e),
                )
            })?;

        // Extract archive to a temporary directory
        let temp_dir = tempfile::tempdir().map_err(|e| {
            ActionError::file_operation(
                backup.backup_path.clone(),
                format!("Failed to create temporary directory: {}", e),
            )
        })?;

        let archive_file = std::fs::File::open(&archive_path).map_err(|e| {
            ActionError::file_operation(
                archive_path.clone(),
                format!("Failed to open archive file: {}", e),
            )
        })?;

        let gz_decoder = flate2::read::GzDecoder::new(archive_file);
        let mut tar_archive = tar::Archive::new(gz_decoder);

        // Extract all files from the archive
        tar_archive.unpack(temp_dir.path()).map_err(|e| {
            ActionError::file_operation(
                archive_path.clone(),
                format!("Failed to extract archive: {}", e),
            )
        })?;

        // Restore files from the extracted archive
        for entry in std::fs::read_dir(temp_dir.path()).map_err(|e| {
            ActionError::file_operation(
                temp_dir.path().to_path_buf(),
                format!("Failed to read extracted directory: {}", e),
            )
        })? {
            let entry = entry.map_err(|e| {
                ActionError::file_operation(
                    temp_dir.path().to_path_buf(),
                    format!("Failed to read extracted directory entry: {}", e),
                )
            })?;

            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap().to_string_lossy();

            if entry_path.is_file() {
                // Restore to original location (assuming same structure)
                let restore_path = Path::new(&*file_name);

                // Create parent directory if it doesn't exist
                if let Some(parent) = restore_path.parent() {
                    if !parent.exists() {
                        tokio::fs::create_dir_all(parent).await.map_err(|e| {
                            ActionError::file_operation(
                                parent.to_path_buf(),
                                format!("Failed to create parent directory: {}", e),
                            )
                        })?;
                    }
                }

                // Copy the file
                tokio::fs::copy(&entry_path, restore_path)
                    .await
                    .map_err(|e| {
                        ActionError::file_operation(
                            entry_path.clone(),
                            format!("Failed to restore file: {}", e),
                        )
                    })?;

                files_restored.push(restore_path.to_string_lossy().to_string());
            } else if entry_path.is_dir() {
                // Restore directory recursively
                let restore_dir_path = Path::new(&*file_name);
                self.restore_directory_recursive(&entry_path, restore_dir_path)
                    .await?;
                files_restored.push(restore_dir_path.to_string_lossy().to_string());
            }
        }

        info!(
            "Archive rollback completed successfully. Restored {} files.",
            files_restored.len()
        );
        Ok(files_restored)
    }

    /// Restore directory recursively
    async fn restore_directory_recursive(&self, src: &Path, dst: &Path) -> ActionResult<()> {
        Box::pin(self.restore_directory_recursive_impl(src, dst)).await
    }

    async fn restore_directory_recursive_impl(&self, src: &Path, dst: &Path) -> ActionResult<()> {
        if !src.is_dir() {
            return Err(ActionError::file_operation(
                src.to_path_buf(),
                "Source is not a directory".to_string(),
            ));
        }

        // Create destination directory
        tokio::fs::create_dir_all(dst).await.map_err(|e| {
            ActionError::file_operation(
                dst.to_path_buf(),
                format!("Failed to create directory: {}", e),
            )
        })?;

        // Read source directory
        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| {
            ActionError::file_operation(
                src.to_path_buf(),
                format!("Failed to read directory: {}", e),
            )
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ActionError::file_operation(
                src.to_path_buf(),
                format!("Failed to read directory entry: {}", e),
            )
        })? {
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let dst_path = dst.join(file_name);

            if entry_path.is_file() {
                tokio::fs::copy(&entry_path, &dst_path).await.map_err(|e| {
                    ActionError::file_operation(
                        entry_path.clone(),
                        format!("Failed to copy file: {}", e),
                    )
                })?;
            } else if entry_path.is_dir() {
                Box::pin(self.restore_directory_recursive_impl(&entry_path, &dst_path)).await?;
            }
        }

        Ok(())
    }

    /// Rollback using snapshot
    async fn rollback_snapshot(&self, backup: &Backup) -> ActionResult<Vec<String>> {
        info!("Rolling back using snapshot for backup: {}", backup.id);

        let mut files_restored = Vec::new();

        // Read snapshot information
        let snapshot_info_path = backup.backup_path.join("snapshot_info.json");
        let snapshot_info_content = tokio::fs::read_to_string(&snapshot_info_path)
            .await
            .map_err(|e| {
                ActionError::file_operation(
                    snapshot_info_path.clone(),
                    format!("Failed to read snapshot info: {}", e),
                )
            })?;

        let snapshot_info: serde_json::Value = serde_json::from_str(&snapshot_info_content)
            .map_err(|e| {
                ActionError::file_operation(
                    snapshot_info_path.clone(),
                    format!("Failed to parse snapshot info: {}", e),
                )
            })?;

        // Get snapshot type
        let snapshot_type = snapshot_info["snapshot_type"].as_str().unwrap_or("unknown");

        match snapshot_type {
            "btrfs" => {
                #[cfg(target_os = "linux")]
                {
                    files_restored = self.rollback_btrfs_snapshot(&snapshot_info).await?;
                }
                #[cfg(not(target_os = "linux"))]
                {
                    warn!("btrfs snapshots not supported on this platform. Falling back to file copy.");
                    files_restored = self.rollback_file_copy(backup).await?;
                }
            }
            "apfs" => {
                #[cfg(target_os = "macos")]
                {
                    files_restored = self.rollback_apfs_snapshot(&snapshot_info).await?;
                }
                #[cfg(not(target_os = "macos"))]
                {
                    warn!(
                        "APFS snapshots not supported on this platform. Falling back to file copy."
                    );
                    files_restored = self.rollback_file_copy(backup).await?;
                }
            }
            "vss" => {
                #[cfg(target_os = "windows")]
                {
                    files_restored = self.rollback_vss_snapshot(&snapshot_info).await?;
                }
                #[cfg(not(target_os = "windows"))]
                {
                    warn!(
                        "VSS snapshots not supported on this platform. Falling back to file copy."
                    );
                    files_restored = self.rollback_file_copy(backup).await?;
                }
            }
            _ => {
                warn!(
                    "Unknown snapshot type: {}. Falling back to file copy.",
                    snapshot_type
                );
                files_restored = self.rollback_file_copy(backup).await?;
            }
        }

        Ok(files_restored)
    }

    #[cfg(target_os = "linux")]
    async fn rollback_btrfs_snapshot(
        &self,
        snapshot_info: &serde_json::Value,
    ) -> ActionResult<Vec<String>> {
        let snapshot_name = snapshot_info["snapshot_name"].as_str().unwrap_or("");
        let source_path = snapshot_info["source_path"].as_str().unwrap_or("");

        if snapshot_name.is_empty() || source_path.is_empty() {
            return Err(ActionError::configuration(
                "Invalid btrfs snapshot information",
            ));
        }

        // Stash any current changes
        let stash_result = tokio::process::Command::new("git")
            .args(&["stash", "push", "-m", &format!("Pre-btrfs-rollback stash")])
            .output()
            .await;

        if let Ok(stash_output) = stash_result {
            if !stash_output.status.success() {
                warn!(
                    "Failed to stash changes: {}",
                    String::from_utf8_lossy(&stash_output.stderr)
                );
            }
        }

        // Restore from btrfs snapshot
        let restore_result = tokio::process::Command::new("btrfs")
            .args(&["subvolume", "snapshot", snapshot_name, source_path])
            .output()
            .await
            .map_err(|e| {
                ActionError::configuration(format!("Failed to restore btrfs snapshot: {}", e))
            })?;

        if restore_result.status.success() {
            info!(
                "Successfully restored from btrfs snapshot: {}",
                snapshot_name
            );

            // Get list of files in the restored snapshot
            let files_result = tokio::process::Command::new("find")
                .args(&[source_path, "-type", "f"])
                .output()
                .await;

            let mut files_restored = Vec::new();
            if let Ok(files_output) = files_result {
                if files_output.status.success() {
                    let files = String::from_utf8_lossy(&files_output.stdout);
                    for line in files.lines() {
                        if !line.trim().is_empty() {
                            files_restored.push(line.trim().to_string());
                        }
                    }
                }
            }

            // Apply stashed changes if they exist
            let stash_list_result = tokio::process::Command::new("git")
                .args(&["stash", "list"])
                .output()
                .await;

            if let Ok(stash_list_output) = stash_list_result {
                if stash_list_output.status.success() {
                    let stash_list = String::from_utf8_lossy(&stash_list_output.stdout);
                    if !stash_list.trim().is_empty() {
                        let pop_result = tokio::process::Command::new("git")
                            .args(&["stash", "pop"])
                            .output()
                            .await;

                        if let Ok(pop_output) = pop_result {
                            if !pop_output.status.success() {
                                warn!(
                                    "Failed to pop stashed changes: {}",
                                    String::from_utf8_lossy(&pop_output.stderr)
                                );
                            }
                        }
                    }
                }
            }

            Ok(files_restored)
        } else {
            Err(ActionError::configuration(format!(
                "btrfs snapshot restoration failed: {}",
                String::from_utf8_lossy(&restore_result.stderr)
            )))
        }
    }

    #[cfg(target_os = "macos")]
    async fn rollback_apfs_snapshot(
        &self,
        snapshot_info: &serde_json::Value,
    ) -> ActionResult<Vec<String>> {
        let snapshot_name = snapshot_info["snapshot_name"].as_str().unwrap_or("");

        if snapshot_name.is_empty() {
            return Err(ActionError::configuration(
                "Invalid APFS snapshot information",
            ));
        }

        // APFS snapshots are typically managed by Time Machine
        // For now, we'll provide information about the snapshot
        info!("APFS snapshot restoration requires manual intervention or Time Machine integration");

        // Return empty list as this requires manual handling
        Ok(Vec::new())
    }

    #[cfg(target_os = "windows")]
    async fn rollback_vss_snapshot(
        &self,
        snapshot_info: &serde_json::Value,
    ) -> ActionResult<Vec<String>> {
        let snapshot_name = snapshot_info["snapshot_name"].as_str().unwrap_or("");

        if snapshot_name.is_empty() {
            return Err(ActionError::configuration(
                "Invalid VSS snapshot information",
            ));
        }

        // VSS restoration would require Windows-specific APIs
        // For now, we'll provide information about the snapshot
        info!("VSS snapshot restoration requires Windows-specific implementation");

        // Return empty list as this requires manual handling
        Ok(Vec::new())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    async fn rollback_btrfs_snapshot(
        &self,
        _snapshot_info: &serde_json::Value,
    ) -> ActionResult<Vec<String>> {
        Err(ActionError::configuration(
            "btrfs snapshots not supported on this platform",
        ))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    async fn rollback_apfs_snapshot(
        &self,
        _snapshot_info: &serde_json::Value,
    ) -> ActionResult<Vec<String>> {
        Err(ActionError::configuration(
            "APFS snapshots not supported on this platform",
        ))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    async fn rollback_vss_snapshot(
        &self,
        _snapshot_info: &serde_json::Value,
    ) -> ActionResult<Vec<String>> {
        Err(ActionError::configuration(
            "VSS snapshots not supported on this platform",
        ))
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
                    tokio::fs::remove_file(&backup.backup_path)
                        .await
                        .map_err(|e| {
                            ActionError::file_operation(
                                backup.backup_path.clone(),
                                format!("Failed to delete backup file: {}", e),
                            )
                        })?;
                } else if backup.backup_path.is_dir() {
                    tokio::fs::remove_dir_all(&backup.backup_path)
                        .await
                        .map_err(|e| {
                            ActionError::file_operation(
                                backup.backup_path.clone(),
                                format!("Failed to delete backup directory: {}", e),
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
