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

//! Configuration management for the Syneidesis coordination library

use serde::{Deserialize, Serialize};
use std::time::Duration;

// Re-export configuration types from syneidesis-config
pub use syneidesis_config::types::{
    CoordinationConfig, LoggingConfig, NetworkConfig, SecurityConfig,
};

// Note: The following local config types have been removed and replaced with
// centralized types from syneidesis-config:
// - CoordinationConfig -> syneidesis_config::types::CoordinationConfig
// - SecurityConfig -> syneidesis_config::types::SecurityConfig
// - LoggingConfig -> syneidesis_config::types::LoggingConfig
// - NetworkConfig -> syneidesis_config::types::NetworkConfig

// Local configuration types that are specific to the coordination crate

/// MCP (Model Context Protocol) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPConfig {
    /// Enable MCP support
    pub enabled: bool,

    /// MCP server address
    pub server_address: String,

    /// MCP server port
    pub server_port: u16,

    /// MCP connection timeout
    pub connection_timeout: Duration,

    /// MCP request timeout
    pub request_timeout: Duration,

    /// MCP authentication token
    pub auth_token: Option<String>,

    /// MCP capabilities
    pub capabilities: Vec<String>,
}

impl Default for MCPConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_address: "localhost".to_string(),
            server_port: 3000,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            auth_token: None,
            capabilities: vec!["tools".to_string(), "resources".to_string()],
        }
    }
}

/// Metrics and monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,

    /// Metrics collection interval
    pub collection_interval: Duration,

    /// Metrics retention period
    pub retention_period: Duration,

    /// Maximum number of metrics points to store
    pub max_metrics_points: usize,

    /// Enable detailed agent metrics
    pub detailed_agent_metrics: bool,

    /// Enable performance metrics
    pub performance_metrics: bool,

    /// Metrics export configuration
    pub export: MetricsExportConfig,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(24 * 60 * 60), // 24 hours
            max_metrics_points: 10000,
            detailed_agent_metrics: true,
            performance_metrics: true,
            export: MetricsExportConfig::default(),
        }
    }
}

/// Metrics export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsExportConfig {
    /// Enable metrics export
    pub enabled: bool,

    /// Export format (json, prometheus, etc.)
    pub format: String,

    /// Export endpoint
    pub endpoint: Option<String>,

    /// Export interval
    pub interval: Duration,
}

impl Default for MetricsExportConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            format: "json".to_string(),
            endpoint: None,
            interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Performance tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Number of worker threads for async operations
    pub worker_threads: usize,

    /// Channel buffer sizes
    pub channel_buffer_size: usize,

    /// Task queue size
    pub task_queue_size: usize,

    /// Event buffer size
    pub event_buffer_size: usize,

    /// Memory pool size
    pub memory_pool_size: usize,

    /// Enable performance profiling
    pub enable_profiling: bool,

    /// Performance monitoring interval
    pub monitoring_interval: Duration,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: 4, // Default to 4 threads instead of num_cpus::get()
            channel_buffer_size: 1000,
            task_queue_size: 10000,
            event_buffer_size: 5000,
            memory_pool_size: 1024 * 1024 * 100, // 100MB
            enable_profiling: false,
            monitoring_interval: Duration::from_secs(60),
        }
    }
}

/// Rate limiting configuration specific to coordination services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,

    /// Requests per minute
    pub requests_per_minute: usize,

    /// Burst size
    pub burst_size: usize,

    /// Rate limit window
    pub window_size: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_minute: 1000,
            burst_size: 100,
            window_size: Duration::from_secs(60),
        }
    }
}

/// Encryption configuration specific to coordination services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Enable message encryption
    pub enabled: bool,

    /// Encryption algorithm
    pub algorithm: String,

    /// Encryption key
    pub key: Option<String>,

    /// Key rotation interval
    pub key_rotation_interval: Duration,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "AES-256-GCM".to_string(),
            key: None,
            key_rotation_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
        }
    }
}

/// Persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Enable state persistence
    pub enabled: bool,

    /// Persistence backend (memory, file, database)
    pub backend: String,

    /// Persistence file path (for file backend)
    pub file_path: Option<String>,

    /// Database connection string (for database backend)
    pub database_url: Option<String>,

    /// Persistence interval
    pub interval: Duration,

    /// Enable automatic backup
    pub auto_backup: bool,

    /// Backup retention period
    pub backup_retention: Duration,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: "memory".to_string(),
            file_path: Some("agent_state.json".to_string()),
            database_url: None,
            interval: Duration::from_secs(300), // 5 minutes
            auto_backup: false,
            backup_retention: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
        }
    }
}

/// Feature flags for enabling/disabling specific features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Enable real-time communication
    pub real_time_communication: bool,

    /// Enable conflict resolution
    pub conflict_resolution: bool,

    /// Enable load balancing
    pub load_balancing: bool,

    /// Enable agent discovery
    pub agent_discovery: bool,

    /// Enable task scheduling
    pub task_scheduling: bool,

    /// Enable health monitoring
    pub health_monitoring: bool,

    /// Enable metrics collection
    pub metrics_collection: bool,

    /// Enable state synchronization
    pub state_synchronization: bool,

    /// Enable event logging
    pub event_logging: bool,

    /// Enable performance optimization
    pub performance_optimization: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            real_time_communication: true,
            conflict_resolution: true,
            load_balancing: true,
            agent_discovery: true,
            task_scheduling: true,
            health_monitoring: true,
            metrics_collection: true,
            state_synchronization: true,
            event_logging: true,
            performance_optimization: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_default_config() {
        let config = CoordinationConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let config = CoordinationConfig::default();

        // Test that default config is valid
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = CoordinationConfig::default();
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: CoordinationConfig = serde_yaml::from_str(&yaml).unwrap();
        // Verify deserialization works
        assert_eq!(config.max_agents, deserialized.max_agents);
    }

    #[test]
    fn test_feature_flags() {
        let flags = FeatureFlags::default();
        assert!(flags.real_time_communication);
        assert!(flags.conflict_resolution);
        assert!(flags.load_balancing);
    }
}
