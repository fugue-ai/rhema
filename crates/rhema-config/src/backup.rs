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

use crate::{Config, ConfigError};
use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Backup schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    pub frequency: BackupFrequency,
    pub time: String,                // HH:MM format
    pub day_of_week: Option<String>, // For weekly backups
    pub day_of_month: Option<u32>,   // For monthly backups
    pub enabled: bool,
}

/// Backup frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupFrequency {
    Daily,
    Weekly,
    Monthly,
    Custom(String), // Cron-like expression
}

/// Detailed backup statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedBackupStats {
    pub total_backups: usize,
    pub total_size_bytes: u64,
    pub total_compressed_size_bytes: u64,
    pub compression_ratio: f64,
    pub format_distribution: std::collections::HashMap<BackupFormat, usize>,
    pub age_distribution: std::collections::HashMap<String, usize>,
    pub last_backup: Option<DateTime<Utc>>,
}

/// Backup manager for configuration backups and restoration
pub struct BackupManager {
    backup_directory: PathBuf,
    backup_format: BackupFormat,
    compression_enabled: bool,
    encryption_enabled: bool,
    max_backups: usize,
    retention_days: u32,
    backup_history: HashMap<PathBuf, Vec<BackupRecord>>,
}

/// Backup format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackupFormat {
    YAML,
    JSON,
    TOML,
    Binary,
}

/// Backup record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRecord {
    pub backup_id: String,
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub timestamp: DateTime<Utc>,
    pub format: BackupFormat,
    pub size_bytes: u64,
    pub checksum: String,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

/// Backup report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupReport {
    pub backups_created: Vec<BackupRecord>,
    pub backups_failed: Vec<BackupError>,
    pub summary: BackupSummary,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
}

/// Backup error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupError {
    pub path: PathBuf,
    pub error: String,
    pub timestamp: DateTime<Utc>,
}

/// Backup summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSummary {
    pub total_backups: usize,
    pub successful_backups: usize,
    pub failed_backups: usize,
    pub total_size_bytes: u64,
    pub compression_ratio: f64,
}

/// Restore report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreReport {
    pub restored_configs: Vec<RestoredConfig>,
    pub restore_errors: Vec<RestoreError>,
    pub summary: RestoreSummary,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
}

/// Restored config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoredConfig {
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub backup_timestamp: DateTime<Utc>,
    pub restore_timestamp: DateTime<Utc>,
    pub success: bool,
}

/// Restore error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreError {
    pub path: PathBuf,
    pub error: String,
    pub timestamp: DateTime<Utc>,
}

/// Restore summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSummary {
    pub total_configs: usize,
    pub successful_restores: usize,
    pub failed_restores: usize,
}

impl BackupManager {
    /// Create a new backup manager
    pub fn new(global_config: &super::GlobalConfig) -> RhemaResult<Self> {
        let backup_dir = global_config.environment.paths.data.join("backups");

        // Ensure backup directory exists
        fs::create_dir_all(&backup_dir).map_err(|e| ConfigError::IoError(e.to_string()))?;

        let mut manager = Self {
            backup_directory: backup_dir,
            backup_format: BackupFormat::YAML,
            compression_enabled: true,
            encryption_enabled: false,
            max_backups: 10,
            retention_days: 30,
            backup_history: HashMap::new(),
        };

        manager.load_backup_history()?;

        Ok(manager)
    }

    /// Load backup history
    fn load_backup_history(&mut self) -> RhemaResult<()> {
        let history_file = self.backup_directory.join("backup_history.json");

        if history_file.exists() {
            let content = fs::read_to_string(&history_file)
                .map_err(|e| ConfigError::IoError(e.to_string()))?;

            self.backup_history = serde_json::from_str(&content)
                .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
        }

        Ok(())
    }

    /// Save backup history
    #[allow(dead_code)]
    fn save_backup_history(&self) -> RhemaResult<()> {
        let history_file = self.backup_directory.join("backup_history.json");
        let content = serde_json::to_string_pretty(&self.backup_history)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;

        fs::write(&history_file, content).map_err(|e| ConfigError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Backup all configurations
    pub fn backup_all(
        &mut self,
        global_config: &super::GlobalConfig,
        repository_configs: &HashMap<PathBuf, super::RepositoryConfig>,
        scope_configs: &HashMap<PathBuf, super::ScopeConfig>,
    ) -> RhemaResult<BackupReport> {
        let start_time = Utc::now();
        let mut backups_created = Vec::new();
        let mut backups_failed = Vec::new();

        // Backup global config
        match self.backup_config(global_config, "global") {
            Ok(record) => backups_created.push(record),
            Err(e) => backups_failed.push(BackupError {
                path: PathBuf::from("global"),
                error: e.to_string(),
                timestamp: Utc::now(),
            }),
        }

        // Backup repository configs
        for (path, config) in repository_configs {
            match self.backup_config(config, &format!("repository:{}", path.display())) {
                Ok(record) => backups_created.push(record),
                Err(e) => backups_failed.push(BackupError {
                    path: path.clone(),
                    error: e.to_string(),
                    timestamp: Utc::now(),
                }),
            }
        }

        // Backup scope configs
        for (path, config) in scope_configs {
            match self.backup_config(config, &format!("scope:{}", path.display())) {
                Ok(record) => backups_created.push(record),
                Err(e) => backups_failed.push(BackupError {
                    path: path.clone(),
                    error: e.to_string(),
                    timestamp: Utc::now(),
                }),
            }
        }

        let end_time = Utc::now();
        let duration = end_time.signed_duration_since(start_time);

        let total_size: u64 = backups_created.iter().map(|b| b.size_bytes).sum();
        let compression_ratio = if self.compression_enabled { 0.7 } else { 1.0 };

        let summary = BackupSummary {
            total_backups: backups_created.len() + backups_failed.len(),
            successful_backups: backups_created.len(),
            failed_backups: backups_failed.len(),
            total_size_bytes: total_size,
            compression_ratio,
        };

        Ok(BackupReport {
            backups_created,
            backups_failed,
            summary,
            timestamp: end_time,
            duration_ms: duration.num_milliseconds() as u64,
        })
    }

    /// Backup a single configuration
    pub fn backup_config<T: Config>(
        &mut self,
        config: &T,
        context: &str,
    ) -> RhemaResult<BackupRecord> {
        let backup_id = self.generate_backup_id();
        let timestamp = Utc::now();

        // Create backup filename
        let filename = format!(
            "{}_{}.{}",
            context.replace(":", "_").replace("/", "_"),
            timestamp.format("%Y%m%d_%H%M%S"),
            self.get_file_extension()
        );

        let backup_path = self.backup_directory.join(&filename);

        // Serialize configuration
        let content = self.serialize_config(config)?;

        // Compress if enabled
        let final_content = if self.compression_enabled {
            self.compress_content(&content)?
        } else {
            content
        };

        // Encrypt if enabled
        let final_content = if self.encryption_enabled {
            self.encrypt_content(&final_content)?
        } else {
            final_content
        };

        // Write backup file
        fs::write(&backup_path, &final_content).map_err(|e| ConfigError::IoError(e.to_string()))?;

        // Calculate checksum
        let checksum = self.calculate_checksum(&final_content);

        // Get file size
        let size_bytes = final_content.len() as u64;

        // Determine configuration type from the type name
        let config_type = self.get_config_type::<T>();

        let record = BackupRecord {
            backup_id: backup_id.clone(),
            original_path: PathBuf::from(config_type.clone()),
            backup_path: backup_path.clone(),
            timestamp,
            format: self.backup_format.clone(),
            size_bytes,
            checksum,
            compression_enabled: self.compression_enabled,
            encryption_enabled: self.encryption_enabled,
            description: None,
            tags: Vec::new(),
        };

        // Update backup history
        let history = self
            .backup_history
            .entry(PathBuf::from(config_type.clone()))
            .or_insert_with(Vec::new);
        history.push(record.clone());

        // Cleanup old backups
        self.cleanup_old_backups(&config_type)?;

        Ok(record)
    }

    /// Generate backup ID
    fn generate_backup_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        format!("backup_{}", timestamp)
    }

    /// Get file extension for backup format
    fn get_file_extension(&self) -> &str {
        match self.backup_format {
            BackupFormat::YAML => "yaml",
            BackupFormat::JSON => "json",
            BackupFormat::TOML => "toml",
            BackupFormat::Binary => "bin",
        }
    }

    /// Serialize configuration
    fn serialize_config<T: Config>(&self, config: &T) -> RhemaResult<Vec<u8>> {
        match self.backup_format {
            BackupFormat::YAML => {
                let content = serde_yaml::to_string(config)
                    .map_err(|e| ConfigError::YamlError(e.to_string()))?;
                Ok(content.into_bytes())
            }
            BackupFormat::JSON => {
                let content = serde_json::to_string_pretty(config)
                    .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
                Ok(content.into_bytes())
            }
            BackupFormat::TOML => {
                let content = toml::to_string_pretty(config)
                    .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
                Ok(content.into_bytes())
            }
            BackupFormat::Binary => {
                Ok(bincode::serialize(config)
                    .map_err(|e| ConfigError::BincodeError(e.to_string()))?)
            }
        }
    }

    /// Compress content
    fn compress_content(&self, content: &[u8]) -> RhemaResult<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(content)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;

        Ok(encoder
            .finish()
            .map_err(|e| ConfigError::IoError(e.to_string()))?)
    }

    /// Decompress content
    fn decompress_content(&self, content: &[u8]) -> RhemaResult<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(content);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;

        Ok(decompressed)
    }

    /// Encrypt content
    fn encrypt_content(&self, content: &[u8]) -> RhemaResult<Vec<u8>> {
        // This is a placeholder implementation
        // In a real implementation, you would use proper encryption
        Ok(content.to_vec())
    }

    /// Decrypt content
    fn decrypt_content(&self, content: &[u8]) -> RhemaResult<Vec<u8>> {
        // This is a placeholder implementation
        // In a real implementation, you would use proper decryption
        Ok(content.to_vec())
    }

    /// Calculate checksum
    fn calculate_checksum(&self, content: &[u8]) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    /// Cleanup old backups
    fn cleanup_old_backups(&self, context: &str) -> RhemaResult<()> {
        if let Some(history) = self.backup_history.get(&PathBuf::from(context)) {
            let mut sorted_history = history.clone();
            sorted_history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

            // Keep only the most recent backups up to max_backups
            if sorted_history.len() > self.max_backups {
                for backup in &sorted_history[self.max_backups..] {
                    if backup.backup_path.exists() {
                        fs::remove_file(&backup.backup_path)
                            .map_err(|e| ConfigError::IoError(e.to_string()))?;
                    }
                }
            }

            // Remove backups older than retention_days
            let cutoff_time = Utc::now() - chrono::Duration::days(self.retention_days as i64);
            for backup in &sorted_history {
                if backup.timestamp < cutoff_time && backup.backup_path.exists() {
                    fs::remove_file(&backup.backup_path)
                        .map_err(|e| ConfigError::IoError(e.to_string()))?;
                }
            }
        }

        Ok(())
    }

    /// Restore configuration from backup
    pub fn restore_config<T: Config>(&self, config_type: &str, backup_id: &str) -> RhemaResult<T> {
        // Find backup record
        let backup_record = self.find_backup_record(config_type, backup_id)?;

        // Read backup file
        let content = fs::read(&backup_record.backup_path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;

        // Decrypt if needed
        let content = if backup_record.encryption_enabled {
            self.decrypt_content(&content)?
        } else {
            content
        };

        // Decompress if needed
        let content = if backup_record.compression_enabled {
            self.decompress_content(&content)?
        } else {
            content
        };

        // Deserialize configuration
        let config = self.deserialize_config::<T>(&content, &backup_record.format)?;

        Ok(config)
    }

    /// Find backup record
    fn find_backup_record(&self, config_type: &str, backup_id: &str) -> RhemaResult<&BackupRecord> {
        let path = PathBuf::from(config_type);

        if let Some(history) = self.backup_history.get(&path) {
            for record in history {
                if record.backup_id == backup_id {
                    return Ok(record);
                }
            }
        }

        Err(ConfigError::BackupFailed(format!(
            "Backup with ID '{}' not found for config type '{}'",
            backup_id, config_type
        ))
        .into())
    }

    /// Deserialize configuration
    fn deserialize_config<T: Config>(
        &self,
        content: &[u8],
        format: &BackupFormat,
    ) -> RhemaResult<T> {
        let content_str = String::from_utf8(content.to_vec())
            .map_err(|e| ConfigError::BackupFailed(e.to_string()))?;

        match format {
            BackupFormat::YAML => {
                let result: T = serde_yaml::from_str(&content_str)
                    .map_err(|e| ConfigError::YamlError(e.to_string()))?;
                Ok(result)
            }
            BackupFormat::JSON => {
                let result: T = serde_json::from_str(&content_str)
                    .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
                Ok(result)
            }
            BackupFormat::TOML => {
                let result: T = toml::from_str(&content_str)
                    .map_err(|e| ConfigError::TomlError(e.to_string()))?;
                Ok(result)
            }
            BackupFormat::Binary => {
                let result: T = bincode::deserialize(content)
                    .map_err(|e| ConfigError::BincodeError(e.to_string()))?;
                Ok(result)
            }
        }
    }

    /// List available backups
    pub fn list_backups(&self, config_type: Option<&str>) -> Vec<&BackupRecord> {
        let mut all_backups = Vec::new();

        for (path, history) in &self.backup_history {
            if let Some(config_type) = config_type {
                if path.to_string_lossy() == config_type {
                    all_backups.extend(history.iter());
                }
            } else {
                all_backups.extend(history.iter());
            }
        }

        all_backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        all_backups
    }

    /// Get backup statistics
    pub fn get_backup_stats(&self) -> BackupSummary {
        let mut total_backups = 0;
        let mut total_size = 0;

        for history in self.backup_history.values() {
            total_backups += history.len();
            total_size += history.iter().map(|b| b.size_bytes).sum::<u64>();
        }

        BackupSummary {
            total_backups,
            successful_backups: total_backups,
            failed_backups: 0,
            total_size_bytes: total_size,
            compression_ratio: if self.compression_enabled { 0.7 } else { 1.0 },
        }
    }

    /// Set backup format
    pub fn set_backup_format(&mut self, format: BackupFormat) {
        self.backup_format = format;
    }

    /// Set compression enabled
    pub fn set_compression_enabled(&mut self, enabled: bool) {
        self.compression_enabled = enabled;
    }

    /// Set encryption enabled
    pub fn set_encryption_enabled(&mut self, enabled: bool) {
        self.encryption_enabled = enabled;
    }

    /// Set max backups
    pub fn set_max_backups(&mut self, max: usize) {
        self.max_backups = max;
    }

    /// Set retention days
    pub fn set_retention_days(&mut self, days: u32) {
        self.retention_days = days;
    }

    /// Get backup directory
    pub fn get_backup_directory(&self) -> &Path {
        &self.backup_directory
    }

    /// Schedule automatic backup
    pub async fn schedule_automatic_backup(&self, schedule: &BackupSchedule) -> RhemaResult<()> {
        tracing::info!("Scheduling automatic backup with schedule: {:?}", schedule);

        // Here we would integrate with a task scheduler
        // For now, we'll just log the schedule
        match schedule.frequency {
            BackupFrequency::Daily => {
                tracing::info!("Daily backup scheduled at {}", schedule.time);
            }
            BackupFrequency::Weekly => {
                tracing::info!(
                    "Weekly backup scheduled on {:?} at {}",
                    schedule.day_of_week,
                    schedule.time
                );
            }
            BackupFrequency::Monthly => {
                tracing::info!(
                    "Monthly backup scheduled on day {:?} at {}",
                    schedule.day_of_month,
                    schedule.time
                );
            }
            BackupFrequency::Custom(ref interval) => {
                tracing::info!("Custom backup scheduled with interval: {}", interval);
            }
        }

        Ok(())
    }

    /// Optimize compression for better performance and size
    pub async fn optimize_compression(&self, content: &[u8]) -> RhemaResult<Vec<u8>> {
        let start_time = std::time::Instant::now();

        // Use different compression levels based on content size
        let compression_level = if content.len() > 1024 * 1024 {
            // Large content: use higher compression
            flate2::Compression::best()
        } else if content.len() > 1024 * 100 {
            // Medium content: use balanced compression
            flate2::Compression::default()
        } else {
            // Small content: use fast compression
            flate2::Compression::fast()
        };

        let mut compressed = Vec::new();
        {
            let mut encoder =
                flate2::write::DeflateEncoder::new(&mut compressed, compression_level);
            std::io::copy(&mut std::io::Cursor::new(content), &mut encoder)
                .map_err(|e| ConfigError::BackupFailed(format!("Compression failed: {}", e)))?;
            encoder.finish().map_err(|e| {
                ConfigError::BackupFailed(format!("Compression finish failed: {}", e))
            })?;
        }

        let duration = start_time.elapsed();
        tracing::debug!(
            "Compression completed in {:?}, original size: {}, compressed size: {}, ratio: {:.2}%",
            duration,
            content.len(),
            compressed.len(),
            (compressed.len() as f64 / content.len() as f64) * 100.0
        );

        Ok(compressed)
    }

    /// Validate backup integrity before restoration
    pub async fn validate_backup_integrity(
        &self,
        backup_record: &BackupRecord,
    ) -> RhemaResult<bool> {
        if !backup_record.backup_path.exists() {
            return Err(ConfigError::BackupFailed("Backup file does not exist".to_string()).into());
        }

        // Read backup file
        let backup_content = std::fs::read(&backup_record.backup_path)
            .map_err(|e| ConfigError::BackupFailed(format!("Failed to read backup file: {}", e)))?;

        // Validate file size
        if backup_content.is_empty() {
            return Err(ConfigError::BackupFailed("Backup file is empty".to_string()).into());
        }

        // Check if file is compressed
        let is_compressed = self.is_compressed_content(&backup_content);

        // Validate checksum using the one from the backup record
        let actual_checksum = self.calculate_checksum(&backup_content);
        if actual_checksum != backup_record.checksum {
            return Err(ConfigError::BackupFailed(format!(
                "Checksum mismatch: expected {}, got {}",
                backup_record.checksum, actual_checksum
            ))
            .into());
        }

        // Try to decompress if compressed
        if is_compressed {
            match self.decompress_content(&backup_content) {
                Ok(_) => {
                    tracing::debug!("Backup integrity validation passed");
                    Ok(true)
                }
                Err(e) => Err(ConfigError::BackupFailed(format!(
                    "Decompression failed during validation: {}",
                    e
                ))
                .into()),
            }
        } else {
            // For uncompressed files, try to parse as JSON/YAML to validate structure
            if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(&backup_content) {
                if json_value.is_object() || json_value.is_array() {
                    tracing::debug!("Backup integrity validation passed");
                    Ok(true)
                } else {
                    Err(ConfigError::BackupFailed(
                        "Backup content is not a valid JSON structure".to_string(),
                    )
                    .into())
                }
            } else {
                // Try YAML parsing
                if let Ok(_) = serde_yaml::from_slice::<serde_yaml::Value>(&backup_content) {
                    tracing::debug!("Backup integrity validation passed");
                    Ok(true)
                } else {
                    Err(ConfigError::BackupFailed(
                        "Backup content is not a valid JSON or YAML structure".to_string(),
                    )
                    .into())
                }
            }
        }
    }

    /// Check if content is compressed
    fn is_compressed_content(&self, content: &[u8]) -> bool {
        // Check for gzip magic number
        if content.len() >= 2 && content[0] == 0x1f && content[1] == 0x8b {
            return true;
        }

        // Check for deflate magic number
        if content.len() >= 2
            && content[0] == 0x78
            && (content[1] == 0x01
                || content[1] == 0x5e
                || content[1] == 0x9c
                || content[1] == 0xda)
        {
            return true;
        }

        false
    }

    /// Extract checksum from filename
    fn extract_checksum_from_filename(&self, path: &Path) -> Option<String> {
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                // Look for checksum in filename pattern: name_checksum.ext
                if let Some(underscore_pos) = name_str.rfind('_') {
                    if underscore_pos > 0 {
                        let checksum_part = &name_str[underscore_pos + 1..];
                        // Remove extension
                        if let Some(dot_pos) = checksum_part.rfind('.') {
                            return Some(checksum_part[..dot_pos].to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// Enhanced backup with integrity validation
    pub async fn backup_with_integrity_check<T: Config>(
        &mut self,
        config: &T,
        context: &str,
    ) -> RhemaResult<BackupRecord> {
        // Create backup
        let backup_record = self.backup_config(config, context)?;

        // Validate backup integrity
        if !self.validate_backup_integrity(&backup_record).await? {
            // Remove invalid backup
            if let Err(e) = std::fs::remove_file(&backup_record.backup_path) {
                tracing::warn!("Failed to remove invalid backup file: {}", e);
            }
            return Err(ConfigError::BackupFailed(
                "Backup integrity validation failed".to_string(),
            )
            .into());
        }

        Ok(backup_record)
    }

    /// Enhanced restore with integrity validation
    pub async fn restore_with_integrity_check<T: Config>(
        &self,
        config_type: &str,
        backup_id: &str,
    ) -> RhemaResult<T> {
        // Find backup record
        let backup_record = self.find_backup_record(config_type, backup_id)?;

        // Validate backup integrity before restoration
        if !self.validate_backup_integrity(backup_record).await? {
            return Err(ConfigError::BackupFailed(
                "Backup integrity validation failed before restoration".to_string(),
            )
            .into());
        }

        // Perform restoration
        self.restore_config(config_type, backup_id)
    }

    /// Get backup statistics with detailed information
    pub async fn get_detailed_backup_stats(&self) -> RhemaResult<DetailedBackupStats> {
        let mut total_size = 0u64;
        let mut total_compressed_size = 0u64;
        let mut backup_count = 0usize;
        let mut format_counts = std::collections::HashMap::new();
        let mut age_distribution = std::collections::HashMap::new();

        for backup in self.list_backups(None) {
            backup_count += 1;
            total_size += backup.size_bytes;

            // Count formats
            *format_counts.entry(backup.format.clone()).or_insert(0) += 1;

            // Calculate age distribution
            let age_days = (chrono::Utc::now() - backup.timestamp).num_days();
            let age_category = match age_days {
                0..=7 => "1 week",
                8..=30 => "1 month",
                31..=90 => "3 months",
                91..=365 => "1 year",
                _ => "1+ years",
            };
            *age_distribution
                .entry(age_category.to_string())
                .or_insert(0) += 1;

            // Calculate compressed size if applicable
            if backup.compression_enabled {
                if let Ok(content) = std::fs::read(&backup.backup_path) {
                    total_compressed_size += content.len() as u64;
                }
            }
        }

        let compression_ratio = if total_size > 0 {
            (total_compressed_size as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        Ok(DetailedBackupStats {
            total_backups: backup_count,
            total_size_bytes: total_size,
            total_compressed_size_bytes: total_compressed_size,
            compression_ratio,
            format_distribution: format_counts,
            age_distribution,
            last_backup: self.list_backups(None).first().map(|b| b.timestamp),
        })
    }

    /// Get configuration type from the generic type
    fn get_config_type<T: Config>(&self) -> String {
        let type_name = std::any::type_name::<T>();

        // Extract the type name from the full path
        // Type names are like "rhema_config::global::GlobalConfig"
        if let Some(last_part) = type_name.split("::").last() {
            // Convert CamelCase to lowercase (e.g., "GlobalConfig" -> "global")
            if last_part.ends_with("Config") {
                let base_name = &last_part[..last_part.len() - 6]; // Remove "Config" suffix
                return base_name.to_lowercase();
            }
        }

        // Fallback to a generic name
        "config".to_string()
    }
}
