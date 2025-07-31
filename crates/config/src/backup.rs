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

use crate::config::{SafetyValidator, SafetyViolation};
use crate::{Config, ConfigError, CURRENT_CONFIG_VERSION};
use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        fs::create_dir_all(&backup_dir).map_err(|e| ConfigError::IoError(e))?;

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
            let content = fs::read_to_string(&history_file).map_err(|e| ConfigError::IoError(e))?;

            self.backup_history =
                serde_json::from_str(&content).map_err(|e| ConfigError::SerializationError(e))?;
        }

        Ok(())
    }

    /// Save backup history
    #[allow(dead_code)]
    fn save_backup_history(&self) -> RhemaResult<()> {
        let history_file = self.backup_directory.join("backup_history.json");
        let content = serde_json::to_string_pretty(&self.backup_history)
            .map_err(|e| ConfigError::SerializationError(e))?;

        fs::write(&history_file, content).map_err(|e| ConfigError::IoError(e))?;

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
        fs::write(&backup_path, &final_content).map_err(|e| ConfigError::IoError(e))?;

        // Calculate checksum
        let checksum = self.calculate_checksum(&final_content);

        // Get file size
        let size_bytes = final_content.len() as u64;

        let record = BackupRecord {
            backup_id: backup_id.clone(),
            original_path: PathBuf::from(context),
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
            .entry(PathBuf::from(context))
            .or_insert_with(Vec::new);
        history.push(record.clone());

        // Cleanup old backups
        self.cleanup_old_backups(context)?;

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
                let content =
                    serde_yaml::to_string(config).map_err(|e| ConfigError::YamlError(e))?;
                Ok(content.into_bytes())
            }
            BackupFormat::JSON => {
                let content = serde_json::to_string_pretty(config)
                    .map_err(|e| ConfigError::SerializationError(e))?;
                Ok(content.into_bytes())
            }
            BackupFormat::TOML => {
                let content = toml::to_string_pretty(config).map_err(|e| {
                    ConfigError::SerializationError(serde_json::Error::io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        e.to_string(),
                    )))
                })?;
                Ok(content.into_bytes())
            }
            BackupFormat::Binary => bincode::serialize(config)
                .map_err(|e| ConfigError::BincodeError(e))
                .map_err(|e| e.into()),
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
            .map_err(|e| ConfigError::IoError(e))?;

        Ok(encoder.finish().map_err(|e| ConfigError::IoError(e))?)
    }

    /// Decompress content
    fn decompress_content(&self, content: &[u8]) -> RhemaResult<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(content);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| ConfigError::IoError(e))?;

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
                            .map_err(|e| ConfigError::IoError(e))?;
                    }
                }
            }

            // Remove backups older than retention_days
            let cutoff_time = Utc::now() - chrono::Duration::days(self.retention_days as i64);
            for backup in &sorted_history {
                if backup.timestamp < cutoff_time && backup.backup_path.exists() {
                    fs::remove_file(&backup.backup_path).map_err(|e| ConfigError::IoError(e))?;
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
        let content = fs::read(&backup_record.backup_path).map_err(|e| ConfigError::IoError(e))?;

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
            BackupFormat::YAML => serde_yaml::from_str(&content_str)
                .map_err(|e| ConfigError::YamlError(e))
                .map_err(|e| e.into()),
            BackupFormat::JSON => serde_json::from_str(&content_str)
                .map_err(|e| ConfigError::SerializationError(e))
                .map_err(|e| e.into()),
            BackupFormat::TOML => toml::from_str(&content_str)
                .map_err(|e| ConfigError::TomlError(e))
                .map_err(|e| e.into()),
            BackupFormat::Binary => bincode::deserialize(content)
                .map_err(|e| ConfigError::BincodeError(e))
                .map_err(|e| e.into()),
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
}
