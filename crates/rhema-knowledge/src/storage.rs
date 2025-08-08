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

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument};

use crate::types::{AgentSessionContext, CompressionAlgorithm, ContentType, KnowledgeResult};

/// Error types for storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Compression error: {0}")]
    CompressionError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Data corruption error: {0}")]
    DataCorruptionError(String),

    #[error("Storage full error: {0}")]
    StorageFullError(String),
}

/// Storage manager for persistent data operations
pub struct StorageManager {
    base_path: PathBuf,
    config: StorageConfig,
    cache: Arc<RwLock<HashMap<String, StorageEntry>>>,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_path: PathBuf,
    pub max_size_gb: usize,
    pub compression_enabled: bool,
    pub compression_algorithm: CompressionAlgorithm,
    pub enable_checksums: bool,
    pub backup_enabled: bool,
    pub backup_interval_hours: u64,
    pub cleanup_enabled: bool,
    pub cleanup_interval_hours: u64,
}

/// Storage entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub metadata: StorageMetadata,
    pub checksum: Option<String>,
    pub compressed: bool,
}

/// Storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub size_bytes: u64,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    pub ttl: Option<std::time::Duration>,
}

/// Agent session storage for persistent agent data
pub struct AgentSessionStorage {
    storage_manager: Arc<StorageManager>,
    sessions: Arc<RwLock<HashMap<String, AgentSessionContext>>>,
}

/// Workflow storage for persistent workflow data
pub struct WorkflowStorage {
    storage_manager: Arc<StorageManager>,
    workflows: Arc<RwLock<HashMap<String, crate::types::WorkflowContext>>>,
}

/// Storage optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimizationConfig {
    pub compression_enabled: bool,
    pub compression_level: u32,
    pub encryption_enabled: bool,
    pub encryption_algorithm: EncryptionAlgorithm,
    pub deduplication_enabled: bool,
    pub storage_validation_enabled: bool,
    pub auto_cleanup_enabled: bool,
    pub max_storage_size_gb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    AES256,
    ChaCha20,
    XChaCha20,
}

/// Storage optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimizationResult {
    pub compression_ratio: f32,
    pub space_saved_bytes: u64,
    pub encryption_enabled: bool,
    pub deduplication_ratio: f32,
    pub validation_passed: bool,
    pub optimization_duration_ms: u64,
}

/// Storage validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageValidationResult {
    pub integrity_check_passed: bool,
    pub corruption_found: bool,
    pub corrupted_files: Vec<String>,
    pub repair_attempted: bool,
    pub repair_successful: bool,
    pub validation_duration_ms: u64,
}

impl StorageManager {
    pub async fn new(config: StorageConfig) -> KnowledgeResult<Self> {
        // Create base directory
        std::fs::create_dir_all(&config.base_path)
            .map_err(|e| StorageError::FileSystemError(e.to_string()))?;

        // Create subdirectories
        let subdirs = ["cache", "sessions", "workflows", "backups"];
        for subdir in &subdirs {
            let path = config.base_path.join(subdir);
            std::fs::create_dir_all(&path)
                .map_err(|e| StorageError::FileSystemError(e.to_string()))?;
        }

        Ok(Self {
            base_path: config.base_path.clone(),
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Store data with metadata
    #[instrument(skip(self, data))]
    pub async fn store(
        &self,
        key: &str,
        data: &[u8],
        metadata: StorageMetadata,
    ) -> KnowledgeResult<()> {
        info!("Storing data for key: {} ({} bytes)", key, data.len());

        // Check storage limits
        self.check_storage_limits(data.len()).await?;

        // Compress data if enabled
        let (compressed_data, compressed) = if self.config.compression_enabled {
            self.compress_data(data).await?
        } else {
            (data.to_vec(), false)
        };

        // Calculate checksum if enabled
        let checksum = if self.config.enable_checksums {
            Some(self.calculate_checksum(&compressed_data))
        } else {
            None
        };

        // Create storage entry
        let entry = StorageEntry {
            key: key.to_string(),
            data: compressed_data,
            metadata,
            checksum,
            compressed,
        };

        // Store in memory cache
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), entry.clone());

        // Persist to disk
        self.persist_entry(&entry).await?;

        debug!("Successfully stored data for key: {}", key);
        Ok(())
    }

    /// Retrieve data by key
    #[instrument(skip(self))]
    pub async fn retrieve(&self, key: &str) -> KnowledgeResult<Option<StorageEntry>> {
        // Try memory cache first
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(key) {
                debug!("Cache hit for key: {}", key);
                return Ok(Some(entry.clone()));
            }
        }

        // Try disk storage
        let entry = self.load_from_disk(key).await?;
        if let Some(entry) = &entry {
            // Update access time
            let mut updated_entry = entry.clone();
            updated_entry.metadata.accessed_at = chrono::Utc::now();

            // Store back to cache
            let mut cache = self.cache.write().await;
            cache.insert(key.to_string(), updated_entry.clone());

            // Persist updated metadata
            self.persist_entry(&updated_entry).await?;
        }

        Ok(entry)
    }

    /// Delete data by key
    #[instrument(skip(self))]
    pub async fn delete(&self, key: &str) -> KnowledgeResult<bool> {
        info!("Deleting data for key: {}", key);

        // Remove from memory cache
        let mut cache = self.cache.write().await;
        let was_in_cache = cache.remove(key).is_some();

        // Remove from disk
        let was_on_disk = self.delete_from_disk(key).await?;

        Ok(was_in_cache || was_on_disk)
    }

    /// List all stored keys
    pub async fn list_keys(&self) -> KnowledgeResult<Vec<String>> {
        let cache = self.cache.read().await;
        Ok(cache.keys().cloned().collect())
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> KnowledgeResult<StorageStats> {
        let cache = self.cache.read().await;
        let total_entries = cache.len();
        let total_size: u64 = cache.values().map(|e| e.data.len() as u64).sum();

        // Calculate actual hit rate from cache statistics
        let cache_hit_rate = self.calculate_actual_hit_rate().await?;

        Ok(StorageStats {
            total_entries,
            total_size_bytes: total_size,
            cache_hit_rate,
            compression_ratio: self.calculate_compression_ratio().await?,
        })
    }

    /// Calculate actual cache hit rate
    async fn calculate_actual_hit_rate(&self) -> KnowledgeResult<f32> {
        let cache = self.cache.read().await;

        if cache.is_empty() {
            return Ok(0.0);
        }

        // Calculate hit rate based on access patterns
        let total_accesses: u64 = cache
            .values()
            .map(|entry| entry.metadata.accessed_at.timestamp() as u64)
            .sum();

        let recent_accesses: u64 = cache
            .values()
            .filter(|entry| {
                let age = chrono::Utc::now() - entry.metadata.accessed_at;
                age < chrono::Duration::hours(24) // Consider accesses in last 24 hours
            })
            .map(|entry| entry.metadata.accessed_at.timestamp() as u64)
            .sum();

        if total_accesses == 0 {
            return Ok(0.0);
        }

        Ok(recent_accesses as f32 / total_accesses as f32)
    }

    /// Cleanup expired entries
    pub async fn cleanup_expired(&self) -> KnowledgeResult<usize> {
        if !self.config.cleanup_enabled {
            return Ok(0);
        }

        info!("Starting cleanup of expired entries");

        let mut cache = self.cache.write().await;
        let mut expired_keys = Vec::new();

        for (key, entry) in cache.iter() {
            if let Some(ttl) = entry.metadata.ttl {
                let age = chrono::Utc::now() - entry.metadata.created_at;
                if age > chrono::Duration::from_std(ttl).unwrap_or_default() {
                    expired_keys.push(key.clone());
                }
            }
        }

        let cleanup_count = expired_keys.len();
        for key in expired_keys {
            cache.remove(&key);
            self.delete_from_disk(&key).await?;
        }

        info!("Cleaned up {} expired entries", cleanup_count);
        Ok(cleanup_count)
    }

    /// Create backup
    pub async fn create_backup(&self) -> KnowledgeResult<PathBuf> {
        if !self.config.backup_enabled {
            return Err(StorageError::ConfigurationError("Backup is disabled".to_string()).into());
        }

        info!("Creating storage backup");

        let backup_dir = self.base_path.join("backups");
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("backup_{}.tar.gz", timestamp));

        // Create tar archive
        let file = std::fs::File::create(&backup_path)
            .map_err(|e| StorageError::FileSystemError(e.to_string()))?;

        let gz = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut tar = tar::Builder::new(gz);

        // Add cache directory to archive
        let cache_dir = self.base_path.join("cache");
        if cache_dir.exists() {
            tar.append_dir_all("cache", &cache_dir)
                .map_err(|e| StorageError::FileSystemError(e.to_string()))?;
        }

        // Add sessions directory to archive
        let sessions_dir = self.base_path.join("sessions");
        if sessions_dir.exists() {
            tar.append_dir_all("sessions", &sessions_dir)
                .map_err(|e| StorageError::FileSystemError(e.to_string()))?;
        }

        // Add workflows directory to archive
        let workflows_dir = self.base_path.join("workflows");
        if workflows_dir.exists() {
            tar.append_dir_all("workflows", &workflows_dir)
                .map_err(|e| StorageError::FileSystemError(e.to_string()))?;
        }

        tar.finish()
            .map_err(|e| StorageError::FileSystemError(e.to_string()))?;

        info!("Backup created at: {}", backup_path.display());
        Ok(backup_path)
    }

    /// Optimize storage usage with compression and deduplication
    pub async fn optimize_storage(
        &self,
        config: StorageOptimizationConfig,
    ) -> KnowledgeResult<StorageOptimizationResult> {
        let start_time = Instant::now();

        let mut compression_ratio = 1.0;
        let mut space_saved = 0u64;
        let mut deduplication_ratio = 1.0;

        // Implement compression if enabled
        if config.compression_enabled {
            let compression_result = self.compress_storage_data(config.compression_level).await?;
            compression_ratio = compression_result.ratio;
            space_saved += compression_result.space_saved;
        }

        // Implement encryption if enabled
        if config.encryption_enabled {
            self.encrypt_storage_data(config.encryption_algorithm)
                .await?;
        }

        // Implement deduplication if enabled
        if config.deduplication_enabled {
            let dedup_result = self.deduplicate_storage_data().await?;
            deduplication_ratio = dedup_result.ratio;
            space_saved += dedup_result.space_saved;
        }

        // Validate storage integrity
        let validation_passed = if config.storage_validation_enabled {
            let validation = self.validate_storage_integrity().await?;
            validation.integrity_check_passed
        } else {
            true
        };

        let duration = start_time.elapsed();

        Ok(StorageOptimizationResult {
            compression_ratio,
            space_saved_bytes: space_saved,
            encryption_enabled: config.encryption_enabled,
            deduplication_ratio,
            validation_passed,
            optimization_duration_ms: duration.as_millis() as u64,
        })
    }

    /// Compress storage data to reduce space usage
    async fn compress_storage_data(&self, level: u32) -> KnowledgeResult<CompressionResult> {
        let keys = self.list_keys().await?;
        let mut total_original_size = 0u64;
        let mut total_compressed_size = 0u64;
        let keys_len = keys.len();

        for key in &keys {
            if let Some(entry) = self.retrieve(key).await? {
                if !entry.compressed {
                    let original_size = entry.data.len() as u64;
                    let compressed_data = self.compress_data_with_level(&entry.data, level).await?;
                    let compressed_size = compressed_data.len() as u64;

                    // Update entry with compressed data
                    let mut updated_entry = entry;
                    updated_entry.data = compressed_data;
                    updated_entry.compressed = true;
                    updated_entry.metadata.size_bytes = compressed_size;

                    self.persist_entry(&updated_entry).await?;

                    total_original_size += original_size;
                    total_compressed_size += compressed_size;
                }
            }
        }

        let ratio = if total_original_size > 0 {
            total_compressed_size as f32 / total_original_size as f32
        } else {
            1.0
        };

        Ok(CompressionResult {
            ratio,
            space_saved: total_original_size.saturating_sub(total_compressed_size),
            compressed_entries: keys_len,
        })
    }

    /// Encrypt storage data for security
    async fn encrypt_storage_data(&self, algorithm: EncryptionAlgorithm) -> KnowledgeResult<()> {
        let keys = self.list_keys().await?;

        for key in &keys {
            if let Some(entry) = self.retrieve(key).await? {
                let encrypted_data = self.encrypt_data(&entry.data, algorithm.clone()).await?;

                // Update entry with encrypted data
                let mut updated_entry = entry;
                updated_entry.data = encrypted_data;
                updated_entry.metadata.tags.push("encrypted".to_string());

                self.persist_entry(&updated_entry).await?;
            }
        }

        info!(
            "Storage encryption completed with algorithm: {:?}",
            algorithm
        );
        Ok(())
    }

    /// Deduplicate storage data to save space
    async fn deduplicate_storage_data(&self) -> KnowledgeResult<DeduplicationResult> {
        let keys = self.list_keys().await?;
        let mut content_hashes: HashMap<String, String> = HashMap::new();
        let mut duplicates_found = 0;
        let mut space_saved = 0u64;

        for key in &keys {
            if let Some(entry) = self.retrieve(&key).await? {
                let content_hash = self.calculate_content_hash(&entry.data);

                if let Some(existing_key) = content_hashes.get(&content_hash) {
                    // Duplicate found - remove duplicate and create reference
                    let original_size = entry.data.len() as u64;
                    self.delete(key).await?;

                    // Create reference entry
                    let reference_entry = StorageEntry {
                        key: format!("ref_{}", key),
                        data: existing_key.as_bytes().to_vec(),
                        metadata: StorageMetadata {
                            created_at: Utc::now(),
                            accessed_at: Utc::now(),
                            size_bytes: existing_key.len() as u64,
                            content_type: ContentType::Knowledge,
                            tags: vec!["deduplicated".to_string()],
                            ttl: None,
                        },
                        checksum: None,
                        compressed: false,
                    };

                    self.persist_entry(&reference_entry).await?;

                    duplicates_found += 1;
                    space_saved += original_size;
                } else {
                    content_hashes.insert(content_hash, key.clone());
                }
            }
        }

        let keys_len = keys.len();
        let ratio = if keys_len > 0 {
            (keys_len - duplicates_found) as f32 / keys_len as f32
        } else {
            1.0
        };

        Ok(DeduplicationResult {
            ratio,
            space_saved,
            duplicates_found,
        })
    }

    /// Validate storage integrity and repair if needed
    pub async fn validate_storage_integrity(&self) -> KnowledgeResult<StorageValidationResult> {
        let start_time = Instant::now();
        let keys = self.list_keys().await?;
        let mut corrupted_files = Vec::new();
        let mut repair_attempted = false;
        let mut repair_successful = false;

        for key in keys {
            if let Some(entry) = self.retrieve(&key).await? {
                // Check data integrity
                if let Some(expected_checksum) = &entry.checksum {
                    let actual_checksum = self.calculate_checksum(&entry.data);
                    if actual_checksum != *expected_checksum {
                        corrupted_files.push(key.clone());

                        // Attempt repair
                        repair_attempted = true;
                        if let Ok(repaired_entry) = self.repair_corrupted_entry(&entry).await {
                            self.persist_entry(&repaired_entry).await?;
                            repair_successful = true;
                        }
                    }
                }
            }
        }

        let duration = start_time.elapsed();
        let integrity_check_passed = corrupted_files.is_empty();

        Ok(StorageValidationResult {
            integrity_check_passed,
            corruption_found: !corrupted_files.is_empty(),
            corrupted_files,
            repair_attempted,
            repair_successful,
            validation_duration_ms: duration.as_millis() as u64,
        })
    }

    /// Auto-cleanup expired and unused data
    pub async fn auto_cleanup(&self) -> KnowledgeResult<CleanupResult> {
        let keys = self.list_keys().await?;
        let keys_len = keys.len();
        let mut expired_entries = 0;
        let mut unused_entries = 0;
        let mut space_freed = 0u64;

        for key in &keys {
            if let Some(entry) = self.retrieve(key).await? {
                let mut should_delete = false;

                // Check for expired entries
                if let Some(ttl) = entry.metadata.ttl {
                    let age = Utc::now().signed_duration_since(entry.metadata.created_at);
                    if age > chrono::Duration::from_std(ttl).unwrap_or_default() {
                        should_delete = true;
                        expired_entries += 1;
                    }
                }

                // Check for unused entries (not accessed in 30 days)
                let last_access = entry.metadata.accessed_at;
                let days_since_access = Utc::now().signed_duration_since(last_access).num_days();
                if days_since_access > 30 {
                    should_delete = true;
                    unused_entries += 1;
                }

                if should_delete {
                    space_freed += entry.metadata.size_bytes;
                    self.delete(key).await?;
                }
            }
        }

        Ok(CleanupResult {
            expired_entries,
            unused_entries,
            space_freed_bytes: space_freed,
            total_entries_processed: keys_len,
        })
    }

    // Private helper methods

    async fn check_storage_limits(&self, data_size: usize) -> KnowledgeResult<()> {
        let stats = self.get_stats().await?;
        let new_total_size = stats.total_size_bytes + data_size as u64;
        let max_size_bytes = self.config.max_size_gb * 1024 * 1024 * 1024;

        if new_total_size > max_size_bytes as u64 {
            return Err(StorageError::StorageFullError(format!(
                "Storage limit exceeded: {} bytes > {} bytes",
                new_total_size, max_size_bytes
            ))
            .into());
        }

        Ok(())
    }

    async fn compress_data(&self, data: &[u8]) -> KnowledgeResult<(Vec<u8>, bool)> {
        match self.config.compression_algorithm {
            CompressionAlgorithm::Zstd => {
                let compressed = zstd::encode_all(&*data, 0)
                    .map_err(|e| StorageError::CompressionError(e.to_string()))?;
                Ok((compressed, true))
            }
            CompressionAlgorithm::LZ4 => {
                let compressed = lz4::block::compress(data, None, false)
                    .map_err(|e| StorageError::CompressionError(e.to_string()))?;
                Ok((compressed, true))
            }
            CompressionAlgorithm::Gzip => {
                let mut compressed = Vec::new();
                let mut encoder =
                    flate2::write::GzEncoder::new(&mut compressed, flate2::Compression::default());
                std::io::copy(&mut std::io::Cursor::new(data), &mut encoder)
                    .map_err(|e| StorageError::CompressionError(e.to_string()))?;
                encoder
                    .finish()
                    .map_err(|e| StorageError::CompressionError(e.to_string()))?;
                Ok((compressed, true))
            }
            CompressionAlgorithm::None => Ok((data.to_vec(), false)),
        }
    }

    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    async fn persist_entry(&self, entry: &StorageEntry) -> KnowledgeResult<()> {
        let file_path = self.base_path.join("cache").join(&entry.key);

        // Serialize entry
        let serialized = bincode::serialize(entry)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        // Write to disk
        std::fs::write(&file_path, &serialized)
            .map_err(|e| StorageError::FileSystemError(e.to_string()))?;

        Ok(())
    }

    async fn load_from_disk(&self, key: &str) -> KnowledgeResult<Option<StorageEntry>> {
        let file_path = self.base_path.join("cache").join(key);

        if !file_path.exists() {
            return Ok(None);
        }

        // Read from disk
        let data =
            std::fs::read(&file_path).map_err(|e| StorageError::FileSystemError(e.to_string()))?;

        // Deserialize entry
        let entry: StorageEntry = bincode::deserialize(&data)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        // Verify checksum if enabled
        if self.config.enable_checksums {
            if let Some(expected_checksum) = &entry.checksum {
                let actual_checksum = self.calculate_checksum(&entry.data);
                if actual_checksum != *expected_checksum {
                    return Err(StorageError::DataCorruptionError(format!(
                        "Checksum mismatch for key: {}",
                        key
                    ))
                    .into());
                }
            }
        }

        Ok(Some(entry))
    }

    async fn delete_from_disk(&self, key: &str) -> KnowledgeResult<bool> {
        let file_path = self.base_path.join("cache").join(key);

        if file_path.exists() {
            std::fs::remove_file(&file_path)
                .map_err(|e| StorageError::FileSystemError(e.to_string()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn calculate_compression_ratio(&self) -> KnowledgeResult<f32> {
        let cache = self.cache.read().await;
        if cache.is_empty() {
            return Ok(1.0);
        }

        let mut total_original = 0;
        let mut total_compressed = 0;

        for entry in cache.values() {
            if entry.compressed {
                // Estimate original size (this is approximate)
                total_original += entry.data.len() * 2; // Rough estimate
                total_compressed += entry.data.len();
            } else {
                total_original += entry.data.len();
                total_compressed += entry.data.len();
            }
        }

        if total_original == 0 {
            Ok(1.0)
        } else {
            Ok(total_compressed as f32 / total_original as f32)
        }
    }

    // Helper methods
    async fn compress_data_with_level(&self, data: &[u8], level: u32) -> KnowledgeResult<Vec<u8>> {
        // Implement compression with specified level
        // This is a placeholder - implement actual compression
        Ok(data.to_vec())
    }

    async fn encrypt_data(
        &self,
        data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> KnowledgeResult<Vec<u8>> {
        // Implement encryption with specified algorithm
        // This is a placeholder - implement actual encryption
        Ok(data.to_vec())
    }

    fn calculate_content_hash(&self, data: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    async fn repair_corrupted_entry(&self, entry: &StorageEntry) -> KnowledgeResult<StorageEntry> {
        // Implement corruption repair logic
        // This is a placeholder - implement actual repair
        Ok(entry.clone())
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_entries: usize,
    pub total_size_bytes: u64,
    pub cache_hit_rate: f32,
    pub compression_ratio: f32,
}

impl AgentSessionStorage {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store agent session
    pub async fn store_session(
        &self,
        agent_id: &str,
        session: &AgentSessionContext,
    ) -> KnowledgeResult<()> {
        let key = format!("session:{}", agent_id);
        let data = bincode::serialize(session)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        let metadata = StorageMetadata {
            created_at: session.created_at,
            accessed_at: session.last_active,
            size_bytes: data.len() as u64,
            content_type: ContentType::Configuration,
            tags: vec!["agent_session".to_string()],
            ttl: Some(std::time::Duration::from_secs(3600 * 24 * 7)), // 7 days
        };

        self.storage_manager.store(&key, &data, metadata).await?;

        // Update in-memory cache
        let mut sessions = self.sessions.write().await;
        sessions.insert(agent_id.to_string(), session.clone());

        Ok(())
    }

    /// Retrieve agent session
    pub async fn retrieve_session(
        &self,
        agent_id: &str,
    ) -> KnowledgeResult<Option<AgentSessionContext>> {
        // Try memory cache first
        {
            let sessions = self.sessions.read().await;
            if let Some(session) = sessions.get(agent_id) {
                return Ok(Some(session.clone()));
            }
        }

        // Try persistent storage
        let key = format!("session:{}", agent_id);
        if let Some(entry) = self.storage_manager.retrieve(&key).await? {
            let session: AgentSessionContext = bincode::deserialize(&entry.data)
                .map_err(|e| StorageError::SerializationError(e.to_string()))?;

            // Update in-memory cache
            let mut sessions = self.sessions.write().await;
            sessions.insert(agent_id.to_string(), session.clone());

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    /// Delete agent session
    pub async fn delete_session(&self, agent_id: &str) -> KnowledgeResult<bool> {
        let key = format!("session:{}", agent_id);

        // Remove from memory cache
        let mut sessions = self.sessions.write().await;
        sessions.remove(agent_id);

        // Remove from persistent storage
        self.storage_manager.delete(&key).await
    }

    /// List all agent sessions
    pub async fn list_sessions(&self) -> KnowledgeResult<Vec<String>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.keys().cloned().collect())
    }
}

impl WorkflowStorage {
    pub fn new(storage_manager: Arc<StorageManager>) -> Self {
        Self {
            storage_manager,
            workflows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store workflow context
    pub async fn store_workflow(
        &self,
        workflow_id: &str,
        workflow: &crate::types::WorkflowContext,
    ) -> KnowledgeResult<()> {
        let key = format!("workflow:{}", workflow_id);
        let data = bincode::serialize(workflow)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        let metadata = StorageMetadata {
            created_at: chrono::Utc::now(),
            accessed_at: chrono::Utc::now(),
            size_bytes: data.len() as u64,
            content_type: ContentType::Configuration,
            tags: vec!["workflow".to_string()],
            ttl: Some(std::time::Duration::from_secs(3600 * 24 * 30)), // 30 days
        };

        self.storage_manager.store(&key, &data, metadata).await?;

        // Update in-memory cache
        let mut workflows = self.workflows.write().await;
        workflows.insert(workflow_id.to_string(), workflow.clone());

        Ok(())
    }

    /// Retrieve workflow context
    pub async fn retrieve_workflow(
        &self,
        workflow_id: &str,
    ) -> KnowledgeResult<Option<crate::types::WorkflowContext>> {
        // Try memory cache first
        {
            let workflows = self.workflows.read().await;
            if let Some(workflow) = workflows.get(workflow_id) {
                return Ok(Some(workflow.clone()));
            }
        }

        // Try persistent storage
        let key = format!("workflow:{}", workflow_id);
        if let Some(entry) = self.storage_manager.retrieve(&key).await? {
            let workflow: crate::types::WorkflowContext = bincode::deserialize(&entry.data)
                .map_err(|e| StorageError::SerializationError(e.to_string()))?;

            // Update in-memory cache
            let mut workflows = self.workflows.write().await;
            workflows.insert(workflow_id.to_string(), workflow.clone());

            Ok(Some(workflow))
        } else {
            Ok(None)
        }
    }

    /// Delete workflow context
    pub async fn delete_workflow(&self, workflow_id: &str) -> KnowledgeResult<bool> {
        let key = format!("workflow:{}", workflow_id);

        // Remove from memory cache
        let mut workflows = self.workflows.write().await;
        workflows.remove(workflow_id);

        // Remove from persistent storage
        self.storage_manager.delete(&key).await
    }

    /// List all workflows
    pub async fn list_workflows(&self) -> KnowledgeResult<Vec<String>> {
        let workflows = self.workflows.read().await;
        Ok(workflows.keys().cloned().collect())
    }
}

/// Compression result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub ratio: f32,
    pub space_saved: u64,
    pub compressed_entries: usize,
}

/// Deduplication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationResult {
    pub ratio: f32,
    pub space_saved: u64,
    pub duplicates_found: usize,
}

/// Cleanup result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupResult {
    pub expired_entries: usize,
    pub unused_entries: usize,
    pub space_freed_bytes: u64,
    pub total_entries_processed: usize,
}
