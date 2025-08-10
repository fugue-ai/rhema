/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
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

pub mod compression;
pub mod encryption;
pub mod key_management;

pub use compression::{CompressionAlgorithm, CompressionConfig, MessageCompressor};
pub use encryption::{EncryptionAlgorithm, EncryptionConfig, MessageEncryption};
pub use key_management::{KeyManager, KeyRotationPolicy, KeyStorage};

use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Advanced features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFeaturesConfig {
    /// Compression configuration
    pub compression: CompressionConfig,
    /// Key management configuration
    pub key_management: KeyManagementConfig,
    /// Encryption configuration
    pub encryption: EncryptionConfig,
    /// Performance monitoring configuration
    pub performance_monitoring: PerformanceMonitoringConfig,
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Key rotation policy
    pub rotation_policy: KeyRotationPolicy,
    /// Key storage configuration
    pub storage: KeyStorage,
    /// Key backup configuration
    pub backup: KeyBackupConfig,
    /// Key recovery configuration
    pub recovery: KeyRecoveryConfig,
}

/// Key backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBackupConfig {
    /// Enable automatic key backup
    pub enabled: bool,
    /// Backup location
    pub location: PathBuf,
    /// Backup encryption enabled
    pub encryption_enabled: bool,
    /// Backup frequency in hours
    pub frequency_hours: u64,
    /// Backup retention days
    pub retention_days: u64,
}

/// Key recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRecoveryConfig {
    /// Enable key recovery
    pub enabled: bool,
    /// Recovery method
    pub method: KeyRecoveryMethod,
    /// Recovery verification enabled
    pub verification_enabled: bool,
    /// Recovery timeout in seconds
    pub timeout_seconds: u64,
}

/// Key recovery methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyRecoveryMethod {
    /// Shamir's Secret Sharing
    ShamirSecretSharing,
    /// Hardware Security Module (HSM)
    Hsm,
    /// Cloud Key Management Service (KMS)
    CloudKms,
    /// Manual recovery
    Manual,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_seconds: u64,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
    /// Enable performance alerts
    pub enable_alerts: bool,
}

/// Performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum compression ratio
    pub max_compression_ratio: f64,
    /// Maximum encryption overhead percentage
    pub max_encryption_overhead_percent: f64,
    /// Maximum key rotation time in seconds
    pub max_key_rotation_time_seconds: u64,
    /// Maximum message processing time in milliseconds
    pub max_message_processing_time_ms: u64,
}

impl Default for AdvancedFeaturesConfig {
    fn default() -> Self {
        Self {
            compression: CompressionConfig {
                algorithm: compression::CompressionAlgorithm::Lz4,
                level: 6,
                threshold_bytes: 1024,
                enable_adaptive: true,
                enable_metrics: true,
            },
            key_management: KeyManagementConfig {
                rotation_policy: KeyRotationPolicy {
                    enabled: true,
                    interval_hours: 24 * 7, // 1 week
                    method: key_management::KeyRotationMethod::Automatic,
                    notification_enabled: true,
                },
                storage: KeyStorage::File(PathBuf::from("./keys")),
                backup: KeyBackupConfig {
                    enabled: true,
                    location: PathBuf::from("./backups/keys"),
                    encryption_enabled: true,
                    frequency_hours: 24,
                    retention_days: 30,
                },
                recovery: KeyRecoveryConfig {
                    enabled: true,
                    method: KeyRecoveryMethod::ShamirSecretSharing,
                    verification_enabled: true,
                    timeout_seconds: 300,
                },
            },
            encryption: EncryptionConfig {
                algorithm: encryption::EncryptionAlgorithm::AES256,
                key_rotation_hours: 24 * 7, // 1 week
                enable_e2e_encryption: true,
                certificate_path: None,
                private_key_path: None,
            },
            performance_monitoring: PerformanceMonitoringConfig {
                enabled: true,
                metrics_interval_seconds: 60,
                thresholds: PerformanceThresholds {
                    max_compression_ratio: 0.8,
                    max_encryption_overhead_percent: 10.0,
                    max_key_rotation_time_seconds: 60,
                    max_message_processing_time_ms: 1000,
                },
                enable_alerts: true,
            },
        }
    }
}

/// Advanced features manager
pub struct AdvancedFeaturesManager {
    config: AdvancedFeaturesConfig,
    compressor: MessageCompressor,
    key_manager: KeyManager,
    encryption: MessageEncryption,
    performance_monitor: PerformanceMonitor,
}

/// Performance monitor
pub struct PerformanceMonitor {
    config: PerformanceMonitoringConfig,
    metrics: HashMap<String, PerformanceMetric>,
    alerts: Vec<PerformanceAlert>,
}

/// Performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub id: String,
    pub alert_type: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl AdvancedFeaturesManager {
    /// Create a new advanced features manager
    pub async fn new(config: AdvancedFeaturesConfig) -> RhemaResult<Self> {
        let compressor = MessageCompressor::new(config.compression.clone()).await?;
        let key_manager = KeyManager::new(config.key_management.clone()).await?;
        let encryption = MessageEncryption::new(config.encryption.clone()).await?;
        let performance_monitor = PerformanceMonitor::new(config.performance_monitoring.clone());

        Ok(Self {
            config,
            compressor,
            key_manager,
            encryption,
            performance_monitor,
        })
    }

    /// Compress message
    pub async fn compress_message(&mut self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        let start_time = std::time::Instant::now();
        let compressed = self.compressor.compress(data).await?;
        let processing_time = start_time.elapsed();

        // Record performance metric
        self.performance_monitor.record_metric(
            "compression_time_ms",
            processing_time.as_millis() as f64,
            "milliseconds",
            HashMap::new(),
        );

        // Check performance threshold
        let compression_ratio = compressed.len() as f64 / data.len() as f64;
        if compression_ratio
            > self
                .config
                .performance_monitoring
                .thresholds
                .max_compression_ratio
        {
            self.performance_monitor.create_alert(
                "compression_ratio_exceeded",
                format!(
                    "Compression ratio {} exceeds threshold {}",
                    compression_ratio,
                    self.config
                        .performance_monitoring
                        .thresholds
                        .max_compression_ratio
                ),
                AlertSeverity::Medium,
            );
        }

        Ok(compressed)
    }

    /// Decompress message
    pub async fn decompress_message(&mut self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        let start_time = std::time::Instant::now();
        let decompressed = self.compressor.decompress(data).await?;
        let processing_time = start_time.elapsed();

        // Record performance metric
        self.performance_monitor.record_metric(
            "decompression_time_ms",
            processing_time.as_millis() as f64,
            "milliseconds",
            HashMap::new(),
        );

        Ok(decompressed)
    }

    /// Encrypt message
    pub async fn encrypt_message(&mut self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        let start_time = std::time::Instant::now();
        let encrypted = self.encryption.encrypt(data).await?;
        let processing_time = start_time.elapsed();

        // Record performance metric
        self.performance_monitor.record_metric(
            "encryption_time_ms",
            processing_time.as_millis() as f64,
            "milliseconds",
            HashMap::new(),
        );

        // Check encryption overhead
        let overhead_percent = (encrypted.len() - data.len()) as f64 / data.len() as f64 * 100.0;
        if overhead_percent
            > self
                .config
                .performance_monitoring
                .thresholds
                .max_encryption_overhead_percent
        {
            self.performance_monitor.create_alert(
                "encryption_overhead_exceeded",
                format!(
                    "Encryption overhead {}% exceeds threshold {}%",
                    overhead_percent,
                    self.config
                        .performance_monitoring
                        .thresholds
                        .max_encryption_overhead_percent
                ),
                AlertSeverity::Medium,
            );
        }

        Ok(encrypted)
    }

    /// Decrypt message
    pub async fn decrypt_message(&mut self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        let start_time = std::time::Instant::now();
        let decrypted = self.encryption.decrypt(data).await?;
        let processing_time = start_time.elapsed();

        // Record performance metric
        self.performance_monitor.record_metric(
            "decryption_time_ms",
            processing_time.as_millis() as f64,
            "milliseconds",
            HashMap::new(),
        );

        Ok(decrypted)
    }

    /// Rotate encryption keys
    pub async fn rotate_keys(&mut self) -> RhemaResult<()> {
        let start_time = std::time::Instant::now();
        self.key_manager.rotate_keys().await?;
        let processing_time = start_time.elapsed();

        // Record performance metric
        self.performance_monitor.record_metric(
            "key_rotation_time_seconds",
            processing_time.as_secs() as f64,
            "seconds",
            HashMap::new(),
        );

        // Check key rotation time
        if processing_time.as_secs()
            > self
                .config
                .performance_monitoring
                .thresholds
                .max_key_rotation_time_seconds
        {
            self.performance_monitor.create_alert(
                "key_rotation_time_exceeded",
                format!(
                    "Key rotation took {} seconds, exceeds threshold {} seconds",
                    processing_time.as_secs(),
                    self.config
                        .performance_monitoring
                        .thresholds
                        .max_key_rotation_time_seconds
                ),
                AlertSeverity::High,
            );
        }

        Ok(())
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> Vec<PerformanceMetric> {
        self.performance_monitor.get_metrics()
    }

    /// Get performance alerts
    pub fn get_performance_alerts(&self) -> Vec<PerformanceAlert> {
        self.performance_monitor.get_alerts()
    }

    /// Backup keys
    pub async fn backup_keys(&self) -> RhemaResult<()> {
        self.key_manager.backup_keys().await
    }

    /// Restore keys from backup
    pub async fn restore_keys(&self, backup_path: &PathBuf) -> RhemaResult<()> {
        self.key_manager.restore_keys(backup_path).await
    }

    /// Get key statistics
    pub async fn get_key_stats(&self) -> RhemaResult<key_management::KeyStats> {
        self.key_manager.get_stats().await
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(config: PerformanceMonitoringConfig) -> Self {
        Self {
            config,
            metrics: HashMap::new(),
            alerts: Vec::new(),
        }
    }

    /// Record a performance metric
    pub fn record_metric(
        &mut self,
        name: &str,
        value: f64,
        unit: &str,
        metadata: HashMap<String, String>,
    ) {
        let metric = PerformanceMetric {
            name: name.to_string(),
            value,
            unit: unit.to_string(),
            timestamp: Utc::now(),
            metadata,
        };
        self.metrics.insert(name.to_string(), metric);
    }

    /// Create a performance alert
    pub fn create_alert(&mut self, alert_type: &str, message: String, severity: AlertSeverity) {
        let alert = PerformanceAlert {
            id: uuid::Uuid::new_v4().to_string(),
            alert_type: alert_type.to_string(),
            message,
            severity,
            timestamp: Utc::now(),
            resolved: false,
        };
        self.alerts.push(alert);
    }

    /// Get all performance metrics
    pub fn get_metrics(&self) -> Vec<PerformanceMetric> {
        self.metrics.values().cloned().collect()
    }

    /// Get all performance alerts
    pub fn get_alerts(&self) -> Vec<PerformanceAlert> {
        self.alerts.clone()
    }

    /// Resolve an alert
    pub fn resolve_alert(&mut self, alert_id: &str) {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
        }
    }
}
