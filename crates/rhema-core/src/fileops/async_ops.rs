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

use crate::audit::log_file_operation;
use crate::cache::AsyncCache;
use crate::schema::knowledge::{TodoEntry, Todos};
use crate::validation::ValidationRules;
use crate::{RhemaError, RhemaResult};
use chrono::Utc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;

/// Async file operations with caching and validation
pub struct AsyncFileOps {
    cache: Arc<AsyncCache<String, Vec<u8>>>,
    yaml_cache: Arc<AsyncCache<String, serde_yaml::Value>>,
}

impl AsyncFileOps {
    /// Create a new async file operations instance
    pub fn new() -> Self {
        Self {
            cache: Arc::new(AsyncCache::new(
                1000,
                Some(std::time::Duration::from_secs(300)),
            )),
            yaml_cache: Arc::new(AsyncCache::new(
                500,
                Some(std::time::Duration::from_secs(600)),
            )),
        }
    }

    /// Read a file asynchronously with caching
    pub async fn read_file(&self, file_path: &Path) -> RhemaResult<Vec<u8>> {
        // Validate file path
        ValidationRules::validate_file_path_static(file_path)?;

        let path_str = file_path.to_string_lossy().to_string();

        // Check cache first
        if let Some(cached_content) = self.cache.get(&path_str).await {
            log_file_operation("read", file_path, true, None)?;
            return Ok(cached_content);
        }

        // Read from disk
        let content = match fs::read(file_path).await {
            Ok(content) => {
                log_file_operation("read", file_path, true, None)?;
                content
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("read", file_path, false, Some(error_msg.clone()))?;
                return Err(RhemaError::IoError(e));
            }
        };

        // Cache the content
        if let Err(e) = self.cache.set(path_str.clone(), content.clone()).await {
            // Log cache error but don't fail the operation
            tracing::warn!("Failed to cache file content: {}", e);
        }

        Ok(content)
    }

    /// Read a YAML file asynchronously with caching
    pub async fn read_yaml_file<T>(&self, file_path: &Path) -> RhemaResult<T>
    where
        T: DeserializeOwned + Clone + Send + Sync,
    {
        // Validate file path
        ValidationRules::validate_file_path_static(file_path)?;

        let path_str = file_path.to_string_lossy().to_string();

        // Check YAML cache first
        if let Some(cached_yaml) = self.yaml_cache.get(&path_str).await {
            let data: T = serde_yaml::from_value(cached_yaml.clone()).map_err(|e| {
                RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                }
            })?;

            log_file_operation("read", file_path, true, None)?;
            return Ok(data);
        }

        // Read file content
        let content = self.read_file(file_path).await?;

        // Parse YAML
        let yaml_value: serde_yaml::Value =
            serde_yaml::from_slice(&content).map_err(|e| RhemaError::InvalidYaml {
                file: file_path.display().to_string(),
                message: e.to_string(),
            })?;

        let data: T =
            serde_yaml::from_value(yaml_value.clone()).map_err(|e| RhemaError::InvalidYaml {
                file: file_path.display().to_string(),
                message: e.to_string(),
            })?;

        // Cache the YAML value
        if let Err(e) = self.yaml_cache.set(path_str, yaml_value).await {
            // Log cache error but don't fail the operation
            tracing::warn!("Failed to cache YAML content: {}", e);
        }

        Ok(data)
    }

    /// Write a file asynchronously
    pub async fn write_file(&self, file_path: &Path, content: &[u8]) -> RhemaResult<()> {
        // Validate file path
        ValidationRules::validate_file_path_static(file_path)?;

        // Ensure the directory exists
        if let Some(parent) = file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                let error_msg = e.to_string();
                log_file_operation("write", file_path, false, Some(error_msg.clone()))?;
                return Err(RhemaError::IoError(e));
            }
        }

        // Write the file
        match fs::write(file_path, content).await {
            Ok(_) => {
                log_file_operation("write", file_path, true, None)?;

                // Update cache
                let path_str = file_path.to_string_lossy().to_string();
                if let Err(e) = self.cache.set(path_str, content.to_vec()).await {
                    tracing::warn!("Failed to cache file content: {}", e);
                }

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("write", file_path, false, Some(error_msg.clone()))?;
                Err(RhemaError::IoError(e))
            }
        }
    }

    /// Write a YAML file asynchronously
    pub async fn write_yaml_file<T>(&self, file_path: &Path, data: &T) -> RhemaResult<()>
    where
        T: Serialize + Clone + Send + Sync,
    {
        // Validate file path
        ValidationRules::validate_file_path_static(file_path)?;

        // Ensure the directory exists
        if let Some(parent) = file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                let error_msg = e.to_string();
                log_file_operation("write", file_path, false, Some(error_msg.clone()))?;
                return Err(RhemaError::IoError(e));
            }
        }

        // Serialize to YAML
        let yaml_value = serde_yaml::to_value(data).map_err(|e| RhemaError::InvalidYaml {
            file: file_path.display().to_string(),
            message: e.to_string(),
        })?;

        let content = serde_yaml::to_string(&yaml_value).map_err(|e| RhemaError::InvalidYaml {
            file: file_path.display().to_string(),
            message: e.to_string(),
        })?;

        // Write the file
        match fs::write(file_path, content).await {
            Ok(_) => {
                log_file_operation("write", file_path, true, None)?;

                // Update caches
                let path_str = file_path.to_string_lossy().to_string();
                if let Err(e) = self.yaml_cache.set(path_str.clone(), yaml_value).await {
                    tracing::warn!("Failed to cache YAML content: {}", e);
                }

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("write", file_path, false, Some(error_msg.clone()))?;
                Err(RhemaError::IoError(e))
            }
        }
    }

    /// Check if a file exists asynchronously
    pub async fn file_exists(&self, file_path: &Path) -> RhemaResult<bool> {
        // Validate file path
        ValidationRules::validate_file_path_static(file_path)?;

        match fs::metadata(file_path).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(RhemaError::IoError(e)),
        }
    }

    /// Get file metadata asynchronously
    pub async fn get_file_metadata(&self, file_path: &Path) -> RhemaResult<Metadata> {
        // Validate file path
        ValidationRules::validate_file_path_static(file_path)?;

        match fs::metadata(file_path).await {
            Ok(metadata) => Ok(metadata),
            Err(e) => Err(RhemaError::IoError(e)),
        }
    }

    /// Delete a file asynchronously
    pub async fn delete_file(&self, file_path: &Path) -> RhemaResult<()> {
        // Validate file path
        ValidationRules::validate_file_path_static(file_path)?;

        match fs::remove_file(file_path).await {
            Ok(_) => {
                log_file_operation("delete", file_path, true, None)?;

                // Remove from cache
                let path_str = file_path.to_string_lossy().to_string();
                self.cache.remove(&path_str).await;
                self.yaml_cache.remove(&path_str).await;

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("delete", file_path, false, Some(error_msg.clone()))?;
                Err(RhemaError::IoError(e))
            }
        }
    }

    /// Copy a file asynchronously
    pub async fn copy_file(&self, source: &Path, destination: &Path) -> RhemaResult<()> {
        // Validate file paths
        ValidationRules::validate_file_path_static(source)?;
        ValidationRules::validate_file_path_static(destination)?;

        // Ensure destination directory exists
        if let Some(parent) = destination.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                let error_msg = e.to_string();
                log_file_operation("copy", destination, false, Some(error_msg.clone()))?;
                return Err(RhemaError::IoError(e));
            }
        }

        match fs::copy(source, destination).await {
            Ok(_) => {
                log_file_operation("copy", destination, true, None)?;

                // Update cache for destination
                let dest_path_str = destination.to_string_lossy().to_string();
                if let Some(source_content) =
                    self.cache.get(&source.to_string_lossy().to_string()).await
                {
                    if let Err(e) = self.cache.set(dest_path_str, source_content).await {
                        tracing::warn!("Failed to cache copied file content: {}", e);
                    }
                }

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("copy", destination, false, Some(error_msg.clone()))?;
                Err(RhemaError::IoError(e))
            }
        }
    }

    /// Move a file asynchronously
    pub async fn move_file(&self, source: &Path, destination: &Path) -> RhemaResult<()> {
        // Validate file paths
        ValidationRules::validate_file_path_static(source)?;
        ValidationRules::validate_file_path_static(destination)?;

        // Ensure destination directory exists
        if let Some(parent) = destination.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                let error_msg = e.to_string();
                log_file_operation("move", destination, false, Some(error_msg.clone()))?;
                return Err(RhemaError::IoError(e));
            }
        }

        match fs::rename(source, destination).await {
            Ok(_) => {
                log_file_operation("move", destination, true, None)?;

                // Update cache
                let source_path_str = source.to_string_lossy().to_string();
                let dest_path_str = destination.to_string_lossy().to_string();

                if let Some(source_content) = self.cache.get(&source_path_str).await {
                    if let Err(e) = self.cache.set(dest_path_str.clone(), source_content).await {
                        tracing::warn!("Failed to cache moved file content: {}", e);
                    }
                }

                if let Some(source_yaml) = self.yaml_cache.get(&source_path_str).await {
                    if let Err(e) = self.yaml_cache.set(dest_path_str, source_yaml).await {
                        tracing::warn!("Failed to cache moved YAML content: {}", e);
                    }
                }

                // Remove source from cache
                self.cache.remove(&source_path_str).await;
                self.yaml_cache.remove(&source_path_str).await;

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("move", destination, false, Some(error_msg.clone()))?;
                Err(RhemaError::IoError(e))
            }
        }
    }

    /// List directory contents asynchronously
    pub async fn list_directory(&self, dir_path: &Path) -> RhemaResult<Vec<PathBuf>> {
        // Validate directory path
        ValidationRules::validate_file_path_static(dir_path)?;

        let mut entries = Vec::new();
        let mut read_dir = match fs::read_dir(dir_path).await {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(RhemaError::IoError(e)),
        };

        while let Some(entry) = read_dir.next_entry().await? {
            entries.push(entry.path());
        }

        Ok(entries)
    }

    /// Create a directory asynchronously
    pub async fn create_directory(&self, dir_path: &Path) -> RhemaResult<()> {
        // Validate directory path
        ValidationRules::validate_file_path_static(dir_path)?;

        match fs::create_dir_all(dir_path).await {
            Ok(_) => {
                log_file_operation("create_directory", dir_path, true, None)?;
                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("create_directory", dir_path, false, Some(error_msg.clone()))?;
                Err(RhemaError::IoError(e))
            }
        }
    }

    /// Remove a directory asynchronously
    pub async fn remove_directory(&self, dir_path: &Path) -> RhemaResult<()> {
        // Validate directory path
        ValidationRules::validate_file_path_static(dir_path)?;

        match fs::remove_dir_all(dir_path).await {
            Ok(_) => {
                log_file_operation("remove_directory", dir_path, true, None)?;
                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                log_file_operation("remove_directory", dir_path, false, Some(error_msg.clone()))?;
                Err(RhemaError::IoError(e))
            }
        }
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let file_cache_size = self.cache.len().await;
        let yaml_cache_size = self.yaml_cache.len().await;
        (file_cache_size, yaml_cache_size)
    }

    /// Clear all caches
    pub async fn clear_caches(&self) {
        self.cache.clear().await;
        self.yaml_cache.clear().await;
    }
}

// Global async file operation functions for convenience

/// Read a file asynchronously
pub async fn read_file_async(file_path: &Path) -> RhemaResult<Vec<u8>> {
    let file_ops = AsyncFileOps::new();
    file_ops.read_file(file_path).await
}

/// Write a file asynchronously
pub async fn write_file_async(file_path: &Path, content: &[u8]) -> RhemaResult<()> {
    let file_ops = AsyncFileOps::new();
    file_ops.write_file(file_path, content).await
}

/// Read a YAML file asynchronously
pub async fn read_yaml_file_async<T>(file_path: &Path) -> RhemaResult<T>
where
    T: DeserializeOwned + Clone + Send + Sync,
{
    let file_ops = AsyncFileOps::new();
    file_ops.read_yaml_file(file_path).await
}

/// Write a YAML file asynchronously
pub async fn write_yaml_file_async<T>(file_path: &Path, data: &T) -> RhemaResult<()>
where
    T: Serialize + Clone + Send + Sync,
{
    let file_ops = AsyncFileOps::new();
    file_ops.write_yaml_file(file_path, data).await
}

/// Delete a file asynchronously
pub async fn delete_file_async(file_path: &Path) -> RhemaResult<()> {
    let file_ops = AsyncFileOps::new();
    file_ops.delete_file(file_path).await
}

/// Check if a file exists asynchronously
pub async fn file_exists_async(file_path: &Path) -> RhemaResult<bool> {
    let file_ops = AsyncFileOps::new();
    file_ops.file_exists(file_path).await
}

/// Copy a file asynchronously
pub async fn copy_file_async(source: &Path, destination: &Path) -> RhemaResult<()> {
    let file_ops = AsyncFileOps::new();
    file_ops.copy_file(source, destination).await
}

/// Move a file asynchronously
pub async fn move_file_async(source: &Path, destination: &Path) -> RhemaResult<()> {
    let file_ops = AsyncFileOps::new();
    file_ops.move_file(source, destination).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::knowledge::{Priority, TodoEntry, TodoStatus, Todos};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_async_file_ops_basic() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = b"Test content";

        let file_ops = AsyncFileOps::new();

        // Test write
        assert!(file_ops.write_file(&file_path, content).await.is_ok());

        // Test read
        let read_content = file_ops.read_file(&file_path).await.unwrap();
        assert_eq!(read_content, content);

        // Test exists
        assert!(file_ops.file_exists(&file_path).await.unwrap());

        // Test delete
        assert!(file_ops.delete_file(&file_path).await.is_ok());
        assert!(!file_ops.file_exists(&file_path).await.unwrap());
    }

    #[tokio::test]
    async fn test_async_yaml_operations() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("todos.yaml");

        let file_ops = AsyncFileOps::new();

        // Create test data
        let todos = Todos {
            todos: vec![TodoEntry {
                id: "test-1".to_string(),
                title: "Test Todo".to_string(),
                description: Some("Test Description".to_string()),
                status: TodoStatus::Pending,
                priority: Priority::Medium,
                assigned_to: None,
                due_date: None,
                created_at: Utc::now(),
                completed_at: None,
                outcome: None,
                related_knowledge: None,
                custom: std::collections::HashMap::new(),
            }],
            custom: std::collections::HashMap::new(),
        };

        // Test write YAML
        assert!(file_ops.write_yaml_file(&file_path, &todos).await.is_ok());

        // Test read YAML
        let read_todos: Todos = file_ops.read_yaml_file(&file_path).await.unwrap();
        assert_eq!(read_todos.todos.len(), 1);
        assert_eq!(read_todos.todos[0].title, "Test Todo");

        // Test cache
        let (file_cache_size, yaml_cache_size) = file_ops.get_cache_stats().await;
        assert!(file_cache_size > 0);
        assert!(yaml_cache_size > 0);
    }

    #[tokio::test]
    async fn test_async_file_ops_validation() {
        let file_ops = AsyncFileOps::new();

        // Test invalid path
        let invalid_path = Path::new("../../../etc/passwd");
        assert!(file_ops.read_file(invalid_path).await.is_err());

        // Test absolute path
        let absolute_path = Path::new("/absolute/path");
        assert!(file_ops.read_file(absolute_path).await.is_err());
    }

    #[tokio::test]
    async fn test_async_file_ops_copy_move() {
        let temp_dir = tempdir().unwrap();
        let source_path = temp_dir.path().join("source.txt");
        let dest_path = temp_dir.path().join("dest.txt");
        let move_path = temp_dir.path().join("moved.txt");

        let file_ops = AsyncFileOps::new();
        let content = b"Test content";

        // Create source file
        assert!(file_ops.write_file(&source_path, content).await.is_ok());

        // Test copy
        assert!(file_ops.copy_file(&source_path, &dest_path).await.is_ok());
        assert!(file_ops.file_exists(&dest_path).await.unwrap());

        // Test move
        assert!(file_ops.move_file(&dest_path, &move_path).await.is_ok());
        assert!(!file_ops.file_exists(&dest_path).await.unwrap());
        assert!(file_ops.file_exists(&move_path).await.unwrap());
    }

    #[tokio::test]
    async fn test_global_async_file_ops() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("global_test.txt");
        let content = b"Global test content";

        // Test global functions
        assert!(write_file_async(&file_path, content).await.is_ok());

        let read_content = read_file_async(&file_path).await.unwrap();
        assert_eq!(read_content, content);

        assert!(delete_file_async(&file_path).await.is_ok());
    }
}
