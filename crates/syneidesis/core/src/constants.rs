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

//! Constants and configuration defaults for the Syneidesis coordination ecosystem

use std::time::Duration;

// ============================================================================
// Network and Communication Constants
// ============================================================================

/// Default gRPC server port
pub const DEFAULT_GRPC_PORT: u16 = 50051;

/// Default gRPC server address
pub const DEFAULT_GRPC_ADDR: &str = "127.0.0.1:50051";

/// Default WebSocket server port
pub const DEFAULT_WEBSOCKET_PORT: u16 = 8080;

/// Default WebSocket server address
pub const DEFAULT_WEBSOCKET_ADDR: &str = "127.0.0.1:8080";

/// Default HTTP server port
pub const DEFAULT_HTTP_PORT: u16 = 3000;

/// Default HTTP server address
pub const DEFAULT_HTTP_ADDR: &str = "127.0.0.1:3000";

/// Default maximum message size (10MB)
pub const DEFAULT_MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024;

/// Default connection timeout (30 seconds)
pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Default request timeout (60 seconds)
pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Default keep-alive interval (30 seconds)
pub const DEFAULT_KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(30);

/// Default keep-alive timeout (10 seconds)
pub const DEFAULT_KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(10);

// ============================================================================
// Agent Management Constants
// ============================================================================

/// Default maximum number of agents
pub const DEFAULT_MAX_AGENTS: usize = 1000;

/// Default agent heartbeat interval (30 seconds)
pub const DEFAULT_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

/// Default agent heartbeat timeout (10 seconds)
pub const DEFAULT_HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(10);

/// Default agent offline timeout (5 minutes)
pub const DEFAULT_OFFLINE_TIMEOUT: Duration = Duration::from_secs(300);

/// Default agent priority
pub const DEFAULT_AGENT_PRIORITY: u8 = 128;

/// Default maximum concurrent tasks per agent
pub const DEFAULT_MAX_CONCURRENT_TASKS: usize = 10;

/// Default agent registration timeout (60 seconds)
pub const DEFAULT_REGISTRATION_TIMEOUT: Duration = Duration::from_secs(60);

// ============================================================================
// Task Management Constants
// ============================================================================

/// Default task queue size
pub const DEFAULT_TASK_QUEUE_SIZE: usize = 1000;

/// Default task timeout (5 minutes)
pub const DEFAULT_TASK_TIMEOUT: Duration = Duration::from_secs(300);

/// Default task retry attempts
pub const DEFAULT_TASK_RETRY_ATTEMPTS: usize = 3;

/// Default task retry delay (30 seconds)
pub const DEFAULT_TASK_RETRY_DELAY: Duration = Duration::from_secs(30);

/// Default task cleanup interval (1 hour)
pub const DEFAULT_TASK_CLEANUP_INTERVAL: Duration = Duration::from_secs(3600);

/// Default task history retention (24 hours)
pub const DEFAULT_TASK_HISTORY_RETENTION: Duration = Duration::from_secs(86400);

// ============================================================================
// Conflict Resolution Constants
// ============================================================================

/// Default conflict resolution timeout (5 minutes)
pub const DEFAULT_CONFLICT_RESOLUTION_TIMEOUT: Duration = Duration::from_secs(300);

/// Default conflict detection interval (10 seconds)
pub const DEFAULT_CONFLICT_DETECTION_INTERVAL: Duration = Duration::from_secs(10);

/// Default conflict history retention (1 hour)
pub const DEFAULT_CONFLICT_HISTORY_RETENTION: Duration = Duration::from_secs(3600);

/// Default maximum conflicts to track
pub const DEFAULT_MAX_CONFLICTS: usize = 100;

// ============================================================================
// Performance and Resource Constants
// ============================================================================

/// Default worker thread count
pub const DEFAULT_WORKER_THREADS: usize = 4;

/// Default channel buffer size
pub const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 1000;

/// Default event buffer size
pub const DEFAULT_EVENT_BUFFER_SIZE: usize = 10000;

/// Default memory pool size (100MB)
pub const DEFAULT_MEMORY_POOL_SIZE: usize = 100 * 1024 * 1024;

/// Default performance monitoring interval (30 seconds)
pub const DEFAULT_PERFORMANCE_MONITORING_INTERVAL: Duration = Duration::from_secs(30);

// ============================================================================
// Metrics and Monitoring Constants
// ============================================================================

/// Default metrics collection interval (30 seconds)
pub const DEFAULT_METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(30);

/// Default metrics retention period (24 hours)
pub const DEFAULT_METRICS_RETENTION_PERIOD: Duration = Duration::from_secs(86400);

/// Default maximum metrics points to store
pub const DEFAULT_MAX_METRICS_POINTS: usize = 10000;

/// Default metrics export interval (5 minutes)
pub const DEFAULT_METRICS_EXPORT_INTERVAL: Duration = Duration::from_secs(300);

// ============================================================================
// Security Constants
// ============================================================================

/// Default rate limiting requests per minute
pub const DEFAULT_RATE_LIMIT_REQUESTS_PER_MINUTE: usize = 1000;

/// Default rate limiting burst size
pub const DEFAULT_RATE_LIMIT_BURST_SIZE: usize = 100;

/// Default rate limiting window size (1 minute)
pub const DEFAULT_RATE_LIMIT_WINDOW_SIZE: Duration = Duration::from_secs(60);

/// Default key rotation interval (24 hours)
pub const DEFAULT_KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400);

// ============================================================================
// Persistence Constants
// ============================================================================

/// Default persistence interval (5 minutes)
pub const DEFAULT_PERSISTENCE_INTERVAL: Duration = Duration::from_secs(300);

/// Default backup retention period (7 days)
pub const DEFAULT_BACKUP_RETENTION_PERIOD: Duration = Duration::from_secs(7 * 86400);

/// Default maximum backup files
pub const DEFAULT_MAX_BACKUP_FILES: usize = 10;

// ============================================================================
// Logging and Debugging Constants
// ============================================================================

/// Default log level
pub const DEFAULT_LOG_LEVEL: &str = "info";

/// Default log format
pub const DEFAULT_LOG_FORMAT: &str = "json";

/// Default log file path
pub const DEFAULT_LOG_FILE_PATH: &str = "logs/syneidesis.log";

/// Default log rotation size (100MB)
pub const DEFAULT_LOG_ROTATION_SIZE: usize = 100 * 1024 * 1024;

/// Default log retention period (30 days)
pub const DEFAULT_LOG_RETENTION_PERIOD: Duration = Duration::from_secs(30 * 86400);

// ============================================================================
// Feature Flags
// ============================================================================

/// Default feature flags configuration
pub const DEFAULT_FEATURE_FLAGS: FeatureFlags = FeatureFlags {
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
};

/// Feature flags configuration
#[derive(Debug, Clone, Copy)]
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
        DEFAULT_FEATURE_FLAGS
    }
}

// ============================================================================
// Error and Status Constants
// ============================================================================

/// Default error message for unknown errors
pub const DEFAULT_UNKNOWN_ERROR_MESSAGE: &str = "An unknown error occurred";

/// Default error message for validation failures
pub const DEFAULT_VALIDATION_ERROR_MESSAGE: &str = "Validation failed";

/// Default error message for timeout errors
pub const DEFAULT_TIMEOUT_ERROR_MESSAGE: &str = "Operation timed out";

/// Default error message for not found errors
pub const DEFAULT_NOT_FOUND_ERROR_MESSAGE: &str = "Resource not found";

/// Default error message for configuration errors
pub const DEFAULT_CONFIGURATION_ERROR_MESSAGE: &str = "Configuration error";

// ============================================================================
// Protocol and Format Constants
// ============================================================================

/// Default protocol version
pub const DEFAULT_PROTOCOL_VERSION: &str = "1.0.0";

/// Default API version
pub const DEFAULT_API_VERSION: &str = "v1";

/// Default content type for JSON
pub const DEFAULT_JSON_CONTENT_TYPE: &str = "application/json";

/// Default content type for protobuf
pub const DEFAULT_PROTOBUF_CONTENT_TYPE: &str = "application/x-protobuf";

/// Default encoding
pub const DEFAULT_ENCODING: &str = "utf-8";

// ============================================================================
// Time and Date Constants
// ============================================================================

/// Default timezone
pub const DEFAULT_TIMEZONE: &str = "UTC";

/// Default date format
pub const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";

/// Default time format
pub const DEFAULT_TIME_FORMAT: &str = "%H:%M:%S";

/// Default datetime format
pub const DEFAULT_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Default timestamp format
pub const DEFAULT_TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

// ============================================================================
// Validation Constants
// ============================================================================

/// Maximum identifier length
pub const MAX_IDENTIFIER_LENGTH: usize = 255;

/// Minimum identifier length
pub const MIN_IDENTIFIER_LENGTH: usize = 1;

/// Maximum name length
pub const MAX_NAME_LENGTH: usize = 100;

/// Minimum name length
pub const MIN_NAME_LENGTH: usize = 1;

/// Maximum description length
pub const MAX_DESCRIPTION_LENGTH: usize = 1000;

/// Maximum metadata key length
pub const MAX_METADATA_KEY_LENGTH: usize = 100;

/// Maximum metadata value length
pub const MAX_METADATA_VALUE_LENGTH: usize = 10000;

/// Maximum tags count
pub const MAX_TAGS_COUNT: usize = 50;

/// Maximum tag length
pub const MAX_TAG_LENGTH: usize = 50;

// ============================================================================
// Cache and Storage Constants
// ============================================================================

/// Default cache size
pub const DEFAULT_CACHE_SIZE: usize = 1000;

/// Default cache TTL (1 hour)
pub const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(3600);

/// Default cache cleanup interval (10 minutes)
pub const DEFAULT_CACHE_CLEANUP_INTERVAL: Duration = Duration::from_secs(600);

/// Default storage path
pub const DEFAULT_STORAGE_PATH: &str = "./data";

/// Default temporary directory
pub const DEFAULT_TEMP_DIR: &str = "./temp";

// ============================================================================
// Testing Constants
// ============================================================================

/// Default test timeout (30 seconds)
pub const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Default test retry attempts
pub const DEFAULT_TEST_RETRY_ATTEMPTS: usize = 3;

/// Default test retry delay (1 second)
pub const DEFAULT_TEST_RETRY_DELAY: Duration = Duration::from_secs(1);

/// Default test data directory
pub const DEFAULT_TEST_DATA_DIR: &str = "./test_data";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_flags_default() {
        let flags = FeatureFlags::default();
        assert!(flags.real_time_communication);
        assert!(flags.conflict_resolution);
        assert!(flags.load_balancing);
        assert!(flags.agent_discovery);
        assert!(flags.task_scheduling);
        assert!(flags.health_monitoring);
        assert!(flags.metrics_collection);
        assert!(flags.state_synchronization);
        assert!(flags.event_logging);
        assert!(flags.performance_optimization);
    }

    #[test]
    fn test_validation_constants() {
        assert!(MAX_IDENTIFIER_LENGTH > MIN_IDENTIFIER_LENGTH);
        assert!(MAX_NAME_LENGTH > MIN_NAME_LENGTH);
        assert!(MAX_TAGS_COUNT > 0);
        assert!(MAX_TAG_LENGTH > 0);
    }

    #[test]
    fn test_duration_constants() {
        assert!(DEFAULT_HEARTBEAT_INTERVAL > Duration::from_secs(0));
        assert!(DEFAULT_TASK_TIMEOUT > Duration::from_secs(0));
        assert!(DEFAULT_CONFLICT_RESOLUTION_TIMEOUT > Duration::from_secs(0));
    }

    #[test]
    fn test_network_constants() {
        assert!(DEFAULT_GRPC_PORT > 0);
        assert!(DEFAULT_WEBSOCKET_PORT > 0);
        assert!(DEFAULT_HTTP_PORT > 0);
        assert!(DEFAULT_MAX_MESSAGE_SIZE > 0);
    }
}
