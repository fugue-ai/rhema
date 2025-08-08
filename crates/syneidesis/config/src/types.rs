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

//! Configuration types for the Syneidesis ecosystem

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use validator::Validate;

/// Main configuration structure for the entire Syneidesis system
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SyneidesisConfig {
    /// System-level configuration
    #[serde(default)]
    pub system: Option<SystemConfig>,

    /// Agent configuration
    #[serde(default)]
    pub agent: Option<AgentConfig>,

    /// Coordination configuration
    #[serde(default)]
    pub coordination: Option<CoordinationConfig>,

    /// gRPC configuration
    #[serde(default)]
    pub grpc: Option<GrpcConfig>,

    /// HTTP configuration
    #[serde(default)]
    pub http: Option<HttpConfig>,

    /// Network configuration
    #[serde(default)]
    pub network: Option<NetworkConfig>,

    /// Security configuration
    #[serde(default)]
    pub security: Option<SecurityConfig>,

    /// Logging configuration
    #[serde(default)]
    pub logging: Option<LoggingConfig>,

    /// Validation configuration
    #[serde(default)]
    pub validation: Option<ValidationConfig>,

    /// Additional custom configuration
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// System-level configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SystemConfig {
    /// System name
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    /// System version
    #[validate(length(min = 1, max = 50))]
    pub version: String,

    /// System description
    #[serde(default)]
    pub description: Option<String>,

    /// System environment (development, staging, production)
    #[serde(default = "default_environment")]
    pub environment: String,

    /// System timezone
    #[serde(default = "default_timezone")]
    pub timezone: String,

    /// System locale
    #[serde(default = "default_locale")]
    pub locale: String,

    /// Maximum number of concurrent operations
    #[serde(default = "default_max_concurrent_ops")]
    #[validate(range(min = 1, max = 10000))]
    pub max_concurrent_operations: usize,

    /// Operation timeout
    #[serde(default = "default_operation_timeout")]
    pub operation_timeout: Duration,

    /// Enable debug mode
    #[serde(default)]
    pub debug: bool,

    /// Enable profiling
    #[serde(default)]
    pub profiling: bool,

    /// Enable metrics collection
    #[serde(default = "default_metrics_enabled")]
    pub metrics_enabled: bool,

    /// Metrics collection interval
    #[serde(default = "default_metrics_interval")]
    pub metrics_interval: Duration,

    /// Health check interval
    #[serde(default = "default_health_check_interval")]
    pub health_check_interval: Duration,

    /// Graceful shutdown timeout
    #[serde(default = "default_shutdown_timeout")]
    pub shutdown_timeout: Duration,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgentConfig {
    /// Maximum number of agents
    #[serde(default = "default_max_agents")]
    #[validate(range(min = 1, max = 10000))]
    pub max_agents: usize,

    /// Agent heartbeat interval
    #[serde(default = "default_heartbeat_interval")]
    pub heartbeat_interval: Duration,

    /// Agent timeout
    #[serde(default = "default_agent_timeout")]
    pub agent_timeout: Duration,

    /// Agent registration timeout
    #[serde(default = "default_registration_timeout")]
    pub registration_timeout: Duration,

    /// Enable agent auto-discovery
    #[serde(default = "default_auto_discovery")]
    pub auto_discovery: bool,

    /// Agent discovery interval
    #[serde(default = "default_discovery_interval")]
    pub discovery_interval: Duration,

    /// Agent cleanup interval
    #[serde(default = "default_cleanup_interval")]
    pub cleanup_interval: Duration,

    /// Enable agent load balancing
    #[serde(default = "default_load_balancing")]
    pub load_balancing: bool,

    /// Load balancing strategy
    #[serde(default = "default_load_balancing_strategy")]
    pub load_balancing_strategy: String,

    /// Agent priority levels
    #[serde(default = "default_priority_levels")]
    pub priority_levels: usize,

    /// Enable agent failover
    #[serde(default = "default_failover")]
    pub failover: bool,

    /// Failover timeout
    #[serde(default = "default_failover_timeout")]
    pub failover_timeout: Duration,

    /// Agent resource limits
    #[serde(default)]
    pub resource_limits: Option<AgentResourceLimits>,
}

/// Agent resource limits
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgentResourceLimits {
    /// Maximum CPU usage percentage
    #[serde(default = "default_max_cpu_usage")]
    #[validate(range(min = 1, max = 100))]
    pub max_cpu_usage: f64,

    /// Maximum memory usage in bytes
    #[serde(default = "default_max_memory_usage")]
    pub max_memory_usage: u64,

    /// Maximum disk usage in bytes
    #[serde(default = "default_max_disk_usage")]
    pub max_disk_usage: u64,

    /// Maximum network bandwidth in bytes per second
    #[serde(default = "default_max_network_bandwidth")]
    pub max_network_bandwidth: u64,

    /// Maximum concurrent tasks
    #[serde(default = "default_max_concurrent_tasks")]
    pub max_concurrent_tasks: usize,
}

/// Coordination configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CoordinationConfig {
    /// Maximum number of agents
    #[serde(default = "default_max_agents")]
    #[validate(range(min = 1, max = 10000))]
    pub max_agents: usize,

    /// Heartbeat interval in seconds
    #[serde(default = "default_heartbeat_interval_secs")]
    pub heartbeat_interval: u64,

    /// Agent timeout in seconds
    #[serde(default = "default_agent_timeout_secs")]
    pub agent_timeout: u64,

    /// Enable metrics collection
    #[serde(default = "default_metrics_enabled")]
    pub enable_metrics: bool,

    /// Enable conflict resolution
    #[serde(default = "default_conflict_resolution")]
    pub enable_conflict_resolution: bool,

    /// Conflict resolution strategy
    #[serde(default = "default_conflict_resolution_strategy")]
    pub conflict_resolution_strategy: String,

    /// Enable state synchronization
    #[serde(default = "default_state_sync")]
    pub enable_state_sync: bool,

    /// State sync interval in seconds
    #[serde(default = "default_state_sync_interval")]
    pub state_sync_interval: u64,

    /// Enable message persistence
    #[serde(default = "default_message_persistence")]
    pub enable_message_persistence: bool,

    /// Message retention period in seconds
    #[serde(default = "default_message_retention")]
    pub message_retention_period: u64,

    /// Maximum message queue size
    #[serde(default = "default_max_message_queue")]
    pub max_message_queue_size: usize,

    /// Enable message encryption
    #[serde(default = "default_message_encryption")]
    pub enable_message_encryption: bool,

    /// Enable message compression
    #[serde(default = "default_message_compression")]
    pub enable_message_compression: bool,
}

/// gRPC configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GrpcConfig {
    /// Server address
    #[serde(default = "default_grpc_addr")]
    pub addr: String,

    /// Maximum message size in bytes
    #[serde(default = "default_max_message_size")]
    pub max_message_size: usize,

    /// Connection timeout in seconds
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,

    /// Keep alive interval in seconds
    #[serde(default = "default_keep_alive_interval")]
    pub keep_alive_interval: u64,

    /// Keep alive timeout in seconds
    #[serde(default = "default_keep_alive_timeout")]
    pub keep_alive_timeout: u64,

    /// Maximum concurrent streams
    #[serde(default = "default_max_concurrent_streams")]
    pub max_concurrent_streams: u32,

    /// Enable reflection
    #[serde(default = "default_reflection_enabled")]
    pub enable_reflection: bool,

    /// Enable health checks
    #[serde(default = "default_health_checks_enabled")]
    pub enable_health_checks: bool,

    /// Enable metrics
    #[serde(default = "default_metrics_enabled")]
    pub enable_metrics: bool,

    /// Enable tracing
    #[serde(default = "default_tracing_enabled")]
    pub enable_tracing: bool,

    /// TLS configuration
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TlsConfig {
    /// Certificate file path
    #[validate(length(min = 1))]
    pub cert_file: String,

    /// Private key file path
    #[validate(length(min = 1))]
    pub key_file: String,

    /// CA certificate file path
    #[serde(default)]
    pub ca_file: Option<String>,

    /// Enable client authentication
    #[serde(default)]
    pub client_auth: bool,

    /// Minimum TLS version
    #[serde(default = "default_min_tls_version")]
    pub min_tls_version: String,

    /// Maximum TLS version
    #[serde(default = "default_max_tls_version")]
    pub max_tls_version: String,
}

/// HTTP configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct HttpConfig {
    /// Server address
    #[serde(default = "default_http_addr")]
    pub addr: String,

    /// Port
    #[serde(default = "default_http_port")]
    pub port: u16,

    /// Maximum request size in bytes
    #[serde(default = "default_max_request_size")]
    pub max_request_size: usize,

    /// Request timeout in seconds
    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,

    /// Enable CORS
    #[serde(default = "default_cors_enabled")]
    pub enable_cors: bool,

    /// CORS origins
    #[serde(default = "default_cors_origins")]
    pub cors_origins: Vec<String>,

    /// Enable rate limiting
    #[serde(default = "default_rate_limiting")]
    pub enable_rate_limiting: bool,

    /// Rate limit requests per minute
    #[serde(default = "default_rate_limit")]
    pub rate_limit: u32,

    /// Enable compression
    #[serde(default = "default_compression_enabled")]
    pub enable_compression: bool,

    /// Enable WebSocket support
    #[serde(default = "default_websocket_enabled")]
    pub enable_websocket: bool,

    /// WebSocket configuration
    #[serde(default)]
    pub websocket: WebSocketConfig,

    /// Enable static file serving
    #[serde(default = "default_static_files_enabled")]
    pub enable_static_files: bool,

    /// Static files directory
    #[serde(default = "default_static_files_dir")]
    pub static_files_dir: String,

    /// Enable API documentation
    #[serde(default = "default_api_docs_enabled")]
    pub enable_api_docs: bool,

    /// API documentation path
    #[serde(default = "default_api_docs_path")]
    pub api_docs_path: String,
}

/// gRPC client configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GrpcClientConfig {
    /// Server address to connect to
    #[serde(default = "default_grpc_client_addr")]
    pub server_addr: String,

    /// Connection timeout in seconds
    #[serde(default = "default_grpc_client_connection_timeout")]
    pub connection_timeout: u64,

    /// Request timeout in seconds
    #[serde(default = "default_grpc_client_request_timeout")]
    pub request_timeout: u64,

    /// Maximum message size in bytes
    #[serde(default = "default_grpc_client_max_message_size")]
    pub max_message_size: usize,

    /// Enable retry on failure
    #[serde(default = "default_grpc_client_retry_enabled")]
    pub enable_retry: bool,

    /// Maximum retry attempts
    #[serde(default = "default_grpc_client_max_retries")]
    pub max_retries: u32,

    /// Retry backoff delay in seconds
    #[serde(default = "default_grpc_client_retry_backoff")]
    pub retry_backoff: u64,

    /// Enable connection pooling
    #[serde(default = "default_grpc_client_pooling_enabled")]
    pub enable_pooling: bool,

    /// Maximum pool size
    #[serde(default = "default_grpc_client_pool_size")]
    pub pool_size: usize,

    /// TLS configuration
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WebSocketConfig {
    /// WebSocket server bind address
    #[serde(default = "default_websocket_addr")]
    pub bind_address: String,

    /// WebSocket server port
    #[serde(default = "default_websocket_port")]
    pub port: u16,

    /// Maximum message size in bytes
    #[serde(default = "default_websocket_max_message_size")]
    pub max_message_size: usize,

    /// Connection timeout in seconds
    #[serde(default = "default_websocket_connection_timeout")]
    pub connection_timeout: u64,

    /// Ping interval for connection health
    #[serde(default = "default_websocket_ping_interval")]
    pub ping_interval: u64,

    /// Pong timeout in seconds
    #[serde(default = "default_websocket_pong_timeout")]
    pub pong_timeout: u64,

    /// Maximum number of concurrent connections
    #[serde(default = "default_websocket_max_connections")]
    pub max_connections: usize,

    /// Enable WebSocket compression
    #[serde(default = "default_websocket_compression_enabled")]
    pub enable_compression: bool,

    /// Enable WebSocket subprotocols
    #[serde(default = "default_websocket_subprotocols_enabled")]
    pub enable_subprotocols: bool,

    /// Allowed subprotocols
    #[serde(default = "default_websocket_subprotocols")]
    pub subprotocols: Vec<String>,

    /// TLS configuration
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NetworkConfig {
    /// Bind address
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// External address
    #[serde(default)]
    pub external_addr: Option<String>,

    /// Network interface
    #[serde(default)]
    pub interface: Option<String>,

    /// Enable IPv6
    #[serde(default = "default_ipv6_enabled")]
    pub enable_ipv6: bool,

    /// TCP keep alive
    #[serde(default = "default_tcp_keep_alive")]
    pub tcp_keep_alive: bool,

    /// TCP keep alive interval in seconds
    #[serde(default = "default_tcp_keep_alive_interval")]
    pub tcp_keep_alive_interval: u64,

    /// TCP keep alive probes
    #[serde(default = "default_tcp_keep_alive_probes")]
    pub tcp_keep_alive_probes: u32,

    /// TCP keep alive time in seconds
    #[serde(default = "default_tcp_keep_alive_time")]
    pub tcp_keep_alive_time: u64,

    /// TCP no delay
    #[serde(default = "default_tcp_no_delay")]
    pub tcp_no_delay: bool,

    /// TCP reuse address
    #[serde(default = "default_tcp_reuse_addr")]
    pub tcp_reuse_addr: bool,

    /// TCP linger
    #[serde(default)]
    pub tcp_linger: Option<u64>,

    /// Socket buffer size
    #[serde(default = "default_socket_buffer_size")]
    pub socket_buffer_size: usize,

    /// Enable multicast
    #[serde(default = "default_multicast_enabled")]
    pub enable_multicast: bool,

    /// Multicast address
    #[serde(default)]
    pub multicast_addr: Option<String>,

    /// Multicast interface
    #[serde(default)]
    pub multicast_interface: Option<String>,

    /// Multicast TTL
    #[serde(default = "default_multicast_ttl")]
    pub multicast_ttl: u32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SecurityConfig {
    /// Enable authentication
    #[serde(default = "default_auth_enabled")]
    pub enable_auth: bool,

    /// Authentication method
    #[serde(default = "default_auth_method")]
    pub auth_method: String,

    /// JWT secret key
    #[serde(default)]
    pub jwt_secret: Option<String>,

    /// JWT expiration time in seconds
    #[serde(default = "default_jwt_expiration")]
    pub jwt_expiration: u64,

    /// JWT refresh expiration time in seconds
    #[serde(default = "default_jwt_refresh_expiration")]
    pub jwt_refresh_expiration: u64,

    /// Enable authorization
    #[serde(default = "default_authorization_enabled")]
    pub enable_authorization: bool,

    /// Authorization policy file
    #[serde(default)]
    pub authorization_policy: Option<String>,

    /// Enable encryption
    #[serde(default = "default_encryption_enabled")]
    pub enable_encryption: bool,

    /// Encryption algorithm
    #[serde(default = "default_encryption_algorithm")]
    pub encryption_algorithm: String,

    /// Encryption key
    #[serde(default)]
    pub encryption_key: Option<String>,

    /// Enable rate limiting
    #[serde(default = "default_security_rate_limiting")]
    pub enable_rate_limiting: bool,

    /// Rate limit window in seconds
    #[serde(default = "default_rate_limit_window")]
    pub rate_limit_window: u64,

    /// Rate limit max requests
    #[serde(default = "default_rate_limit_max_requests")]
    pub rate_limit_max_requests: u32,

    /// Enable IP whitelist
    #[serde(default = "default_ip_whitelist_enabled")]
    pub enable_ip_whitelist: bool,

    /// IP whitelist
    #[serde(default)]
    pub ip_whitelist: Vec<String>,

    /// Enable IP blacklist
    #[serde(default = "default_ip_blacklist_enabled")]
    pub enable_ip_blacklist: bool,

    /// IP blacklist
    #[serde(default)]
    pub ip_blacklist: Vec<String>,

    /// Enable audit logging
    #[serde(default = "default_audit_logging_enabled")]
    pub enable_audit_logging: bool,

    /// Audit log file
    #[serde(default = "default_audit_log_file")]
    pub audit_log_file: String,

    /// Enable session management
    #[serde(default = "default_session_management_enabled")]
    pub enable_session_management: bool,

    /// Session timeout in seconds
    #[serde(default = "default_session_timeout")]
    pub session_timeout: u64,

    /// Session cleanup interval in seconds
    #[serde(default = "default_session_cleanup_interval")]
    pub session_cleanup_interval: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoggingConfig {
    /// Log level
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log format
    #[serde(default = "default_log_format")]
    pub format: String,

    /// Log file path
    #[serde(default)]
    pub file: Option<String>,

    /// Enable console logging
    #[serde(default = "default_console_logging")]
    pub enable_console: bool,

    /// Enable file logging
    #[serde(default = "default_file_logging")]
    pub enable_file: bool,

    /// Enable syslog
    #[serde(default = "default_syslog_enabled")]
    pub enable_syslog: bool,

    /// Syslog facility
    #[serde(default = "default_syslog_facility")]
    pub syslog_facility: String,

    /// Enable structured logging
    #[serde(default = "default_structured_logging")]
    pub enable_structured: bool,

    /// Enable JSON logging
    #[serde(default = "default_json_logging")]
    pub enable_json: bool,

    /// Log rotation enabled
    #[serde(default = "default_log_rotation")]
    pub enable_rotation: bool,

    /// Log rotation size in bytes
    #[serde(default = "default_log_rotation_size")]
    pub rotation_size: u64,

    /// Log rotation count
    #[serde(default = "default_log_rotation_count")]
    pub rotation_count: u32,

    /// Log compression enabled
    #[serde(default = "default_log_compression")]
    pub enable_compression: bool,

    /// Log timestamp format
    #[serde(default = "default_log_timestamp_format")]
    pub timestamp_format: String,

    /// Enable color output
    #[serde(default = "default_log_color")]
    pub enable_color: bool,

    /// Enable thread IDs in logs
    #[serde(default = "default_log_thread_ids")]
    pub enable_thread_ids: bool,

    /// Enable target filtering
    #[serde(default = "default_log_target_filtering")]
    pub enable_target_filtering: bool,

    /// Log targets to include
    #[serde(default)]
    pub include_targets: Vec<String>,

    /// Log targets to exclude
    #[serde(default)]
    pub exclude_targets: Vec<String>,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ValidationConfig {
    /// Enable validation
    #[serde(default = "default_validation_enabled")]
    pub enable_validation: bool,

    /// Validation mode
    #[serde(default = "default_validation_mode")]
    pub mode: String,

    /// Strict validation
    #[serde(default = "default_strict_validation")]
    pub strict: bool,

    /// Enable schema validation
    #[serde(default = "default_schema_validation")]
    pub enable_schema: bool,

    /// Schema file path
    #[serde(default)]
    pub schema_file: Option<String>,

    /// Enable custom validation
    #[serde(default = "default_custom_validation")]
    pub enable_custom: bool,

    /// Custom validation rules file
    #[serde(default)]
    pub custom_rules_file: Option<String>,

    /// Enable cross-field validation
    #[serde(default = "default_cross_field_validation")]
    pub enable_cross_field: bool,

    /// Enable dependency validation
    #[serde(default = "default_dependency_validation")]
    pub enable_dependency: bool,

    /// Enable conditional validation
    #[serde(default = "default_conditional_validation")]
    pub enable_conditional: bool,

    /// Validation error reporting mode
    #[serde(default = "default_validation_error_mode")]
    pub error_mode: String,

    /// Maximum validation errors
    #[serde(default = "default_max_validation_errors")]
    pub max_errors: usize,

    /// Enable validation caching
    #[serde(default = "default_validation_caching")]
    pub enable_caching: bool,

    /// Validation cache size
    #[serde(default = "default_validation_cache_size")]
    pub cache_size: usize,

    /// Validation cache TTL in seconds
    #[serde(default = "default_validation_cache_ttl")]
    pub cache_ttl: u64,
}

// Default value functions
fn default_environment() -> String {
    "development".to_string()
}
fn default_timezone() -> String {
    "UTC".to_string()
}
fn default_locale() -> String {
    "en_US".to_string()
}
fn default_max_concurrent_ops() -> usize {
    1000
}
fn default_operation_timeout() -> Duration {
    Duration::from_secs(30)
}
fn default_metrics_enabled() -> bool {
    true
}
fn default_metrics_interval() -> Duration {
    Duration::from_secs(60)
}
fn default_health_check_interval() -> Duration {
    Duration::from_secs(30)
}
fn default_shutdown_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_max_agents() -> usize {
    1000
}
fn default_heartbeat_interval() -> Duration {
    Duration::from_secs(30)
}
fn default_agent_timeout() -> Duration {
    Duration::from_secs(120)
}
fn default_registration_timeout() -> Duration {
    Duration::from_secs(60)
}
fn default_auto_discovery() -> bool {
    true
}
fn default_discovery_interval() -> Duration {
    Duration::from_secs(300)
}
fn default_cleanup_interval() -> Duration {
    Duration::from_secs(600)
}
fn default_load_balancing() -> bool {
    true
}
fn default_load_balancing_strategy() -> String {
    "round_robin".to_string()
}
fn default_priority_levels() -> usize {
    10
}
fn default_failover() -> bool {
    true
}
fn default_failover_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_max_cpu_usage() -> f64 {
    80.0
}
fn default_max_memory_usage() -> u64 {
    1024 * 1024 * 1024
} // 1GB
fn default_max_disk_usage() -> u64 {
    10 * 1024 * 1024 * 1024
} // 10GB
fn default_max_network_bandwidth() -> u64 {
    100 * 1024 * 1024
} // 100MB/s
fn default_max_concurrent_tasks() -> usize {
    100
}

fn default_heartbeat_interval_secs() -> u64 {
    30
}
fn default_agent_timeout_secs() -> u64 {
    120
}
fn default_conflict_resolution() -> bool {
    true
}
fn default_conflict_resolution_strategy() -> String {
    "priority".to_string()
}
fn default_state_sync() -> bool {
    true
}
fn default_state_sync_interval() -> u64 {
    60
}
fn default_message_persistence() -> bool {
    false
}
fn default_message_retention() -> u64 {
    86400
} // 24 hours
fn default_max_message_queue() -> usize {
    10000
}
fn default_message_encryption() -> bool {
    false
}
fn default_message_compression() -> bool {
    true
}

fn default_grpc_addr() -> String {
    "127.0.0.1:50051".to_string()
}
fn default_max_message_size() -> usize {
    10 * 1024 * 1024
} // 10MB
fn default_connection_timeout() -> u64 {
    30
}
fn default_keep_alive_interval() -> u64 {
    30
}
fn default_keep_alive_timeout() -> u64 {
    10
}
fn default_max_concurrent_streams() -> u32 {
    1000
}
fn default_reflection_enabled() -> bool {
    true
}
fn default_health_checks_enabled() -> bool {
    true
}
fn default_tracing_enabled() -> bool {
    false
}

fn default_min_tls_version() -> String {
    "1.2".to_string()
}
fn default_max_tls_version() -> String {
    "1.3".to_string()
}

fn default_http_addr() -> String {
    "127.0.0.1".to_string()
}
fn default_http_port() -> u16 {
    8080
}
fn default_max_request_size() -> usize {
    10 * 1024 * 1024
} // 10MB
fn default_request_timeout() -> u64 {
    30
}
fn default_cors_enabled() -> bool {
    true
}
fn default_cors_origins() -> Vec<String> {
    vec!["*".to_string()]
}
fn default_rate_limiting() -> bool {
    true
}
fn default_rate_limit() -> u32 {
    1000
}
fn default_compression_enabled() -> bool {
    true
}
fn default_websocket_enabled() -> bool {
    true
}
fn default_static_files_enabled() -> bool {
    true
}
fn default_static_files_dir() -> String {
    "static".to_string()
}
fn default_api_docs_enabled() -> bool {
    true
}
fn default_api_docs_path() -> String {
    "/docs".to_string()
}

fn default_bind_addr() -> String {
    "0.0.0.0".to_string()
}
fn default_ipv6_enabled() -> bool {
    false
}
fn default_tcp_keep_alive() -> bool {
    true
}
fn default_tcp_keep_alive_interval() -> u64 {
    30
}
fn default_tcp_keep_alive_probes() -> u32 {
    3
}
fn default_tcp_keep_alive_time() -> u64 {
    60
}
fn default_tcp_no_delay() -> bool {
    true
}
fn default_tcp_reuse_addr() -> bool {
    true
}
fn default_socket_buffer_size() -> usize {
    64 * 1024
} // 64KB
fn default_multicast_enabled() -> bool {
    false
}
fn default_multicast_ttl() -> u32 {
    1
}

fn default_auth_enabled() -> bool {
    false
}
fn default_auth_method() -> String {
    "jwt".to_string()
}
fn default_jwt_expiration() -> u64 {
    3600
} // 1 hour
fn default_jwt_refresh_expiration() -> u64 {
    86400
} // 24 hours
fn default_authorization_enabled() -> bool {
    false
}
fn default_encryption_enabled() -> bool {
    false
}
fn default_encryption_algorithm() -> String {
    "AES-256-GCM".to_string()
}
fn default_security_rate_limiting() -> bool {
    true
}
fn default_rate_limit_window() -> u64 {
    60
}
fn default_rate_limit_max_requests() -> u32 {
    100
}
fn default_ip_whitelist_enabled() -> bool {
    false
}
fn default_ip_blacklist_enabled() -> bool {
    false
}
fn default_audit_logging_enabled() -> bool {
    false
}
fn default_audit_log_file() -> String {
    "audit.log".to_string()
}
fn default_session_management_enabled() -> bool {
    false
}
fn default_session_timeout() -> u64 {
    3600
} // 1 hour
fn default_session_cleanup_interval() -> u64 {
    300
} // 5 minutes

fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_format() -> String {
    "text".to_string()
}
fn default_console_logging() -> bool {
    true
}
fn default_file_logging() -> bool {
    false
}
fn default_syslog_enabled() -> bool {
    false
}
fn default_syslog_facility() -> String {
    "daemon".to_string()
}
fn default_structured_logging() -> bool {
    false
}
fn default_json_logging() -> bool {
    false
}
fn default_log_rotation() -> bool {
    true
}
fn default_log_rotation_size() -> u64 {
    100 * 1024 * 1024
} // 100MB
fn default_log_rotation_count() -> u32 {
    5
}
fn default_log_compression() -> bool {
    true
}
fn default_log_timestamp_format() -> String {
    "%Y-%m-%d %H:%M:%S".to_string()
}
fn default_log_color() -> bool {
    true
}
fn default_log_thread_ids() -> bool {
    false
}
fn default_log_target_filtering() -> bool {
    false
}

fn default_validation_enabled() -> bool {
    true
}
fn default_validation_mode() -> String {
    "strict".to_string()
}
fn default_strict_validation() -> bool {
    true
}
fn default_schema_validation() -> bool {
    true
}
fn default_custom_validation() -> bool {
    false
}
fn default_cross_field_validation() -> bool {
    true
}
fn default_dependency_validation() -> bool {
    true
}
fn default_conditional_validation() -> bool {
    true
}
fn default_validation_error_mode() -> String {
    "collect".to_string()
}
fn default_max_validation_errors() -> usize {
    100
}
fn default_validation_caching() -> bool {
    true
}
fn default_validation_cache_size() -> usize {
    1000
}
fn default_validation_cache_ttl() -> u64 {
    3600
} // 1 hour

fn default_grpc_client_addr() -> String {
    "127.0.0.1:50051".to_string()
}
fn default_grpc_client_connection_timeout() -> u64 {
    30
}
fn default_grpc_client_request_timeout() -> u64 {
    30
}
fn default_grpc_client_max_message_size() -> usize {
    10 * 1024 * 1024
} // 10MB
fn default_grpc_client_retry_enabled() -> bool {
    true
}
fn default_grpc_client_max_retries() -> u32 {
    3
}
fn default_grpc_client_retry_backoff() -> u64 {
    10
}
fn default_grpc_client_pooling_enabled() -> bool {
    true
}
fn default_grpc_client_pool_size() -> usize {
    10
}

fn default_websocket_addr() -> String {
    "0.0.0.0".to_string()
}
fn default_websocket_port() -> u16 {
    8080
}
fn default_websocket_max_message_size() -> usize {
    10 * 1024 * 1024
} // 10MB
fn default_websocket_connection_timeout() -> u64 {
    30
}
fn default_websocket_ping_interval() -> u64 {
    30
}
fn default_websocket_pong_timeout() -> u64 {
    10
}
fn default_websocket_max_connections() -> usize {
    1000
}
fn default_websocket_compression_enabled() -> bool {
    true
}
fn default_websocket_subprotocols_enabled() -> bool {
    false
}
fn default_websocket_subprotocols() -> Vec<String> {
    vec![]
}

impl Default for SyneidesisConfig {
    fn default() -> Self {
        Self {
            system: Some(SystemConfig::default()),
            agent: Some(AgentConfig::default()),
            coordination: Some(CoordinationConfig::default()),
            grpc: Some(GrpcConfig::default()),
            http: Some(HttpConfig::default()),
            network: Some(NetworkConfig::default()),
            security: Some(SecurityConfig::default()),
            logging: Some(LoggingConfig::default()),
            validation: Some(ValidationConfig::default()),
            custom: HashMap::new(),
        }
    }
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            name: "syneidesis".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            environment: default_environment(),
            timezone: default_timezone(),
            locale: default_locale(),
            max_concurrent_operations: default_max_concurrent_ops(),
            operation_timeout: default_operation_timeout(),
            debug: false,
            profiling: false,
            metrics_enabled: default_metrics_enabled(),
            metrics_interval: default_metrics_interval(),
            health_check_interval: default_health_check_interval(),
            shutdown_timeout: default_shutdown_timeout(),
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_agents: default_max_agents(),
            heartbeat_interval: default_heartbeat_interval(),
            agent_timeout: default_agent_timeout(),
            registration_timeout: default_registration_timeout(),
            auto_discovery: default_auto_discovery(),
            discovery_interval: default_discovery_interval(),
            cleanup_interval: default_cleanup_interval(),
            load_balancing: default_load_balancing(),
            load_balancing_strategy: default_load_balancing_strategy(),
            priority_levels: default_priority_levels(),
            failover: default_failover(),
            failover_timeout: default_failover_timeout(),
            resource_limits: Some(AgentResourceLimits::default()),
        }
    }
}

impl Default for AgentResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_usage: default_max_cpu_usage(),
            max_memory_usage: default_max_memory_usage(),
            max_disk_usage: default_max_disk_usage(),
            max_network_bandwidth: default_max_network_bandwidth(),
            max_concurrent_tasks: default_max_concurrent_tasks(),
        }
    }
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            max_agents: default_max_agents(),
            heartbeat_interval: default_heartbeat_interval_secs(),
            agent_timeout: default_agent_timeout_secs(),
            enable_metrics: default_metrics_enabled(),
            enable_conflict_resolution: default_conflict_resolution(),
            conflict_resolution_strategy: default_conflict_resolution_strategy(),
            enable_state_sync: default_state_sync(),
            state_sync_interval: default_state_sync_interval(),
            enable_message_persistence: default_message_persistence(),
            message_retention_period: default_message_retention(),
            max_message_queue_size: default_max_message_queue(),
            enable_message_encryption: default_message_encryption(),
            enable_message_compression: default_message_compression(),
        }
    }
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            addr: default_grpc_addr(),
            max_message_size: default_max_message_size(),
            connection_timeout: default_connection_timeout(),
            keep_alive_interval: default_keep_alive_interval(),
            keep_alive_timeout: default_keep_alive_timeout(),
            max_concurrent_streams: default_max_concurrent_streams(),
            enable_reflection: default_reflection_enabled(),
            enable_health_checks: default_health_checks_enabled(),
            enable_metrics: default_metrics_enabled(),
            enable_tracing: default_tracing_enabled(),
            tls: None,
        }
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            addr: default_http_addr(),
            port: default_http_port(),
            max_request_size: default_max_request_size(),
            request_timeout: default_request_timeout(),
            enable_cors: default_cors_enabled(),
            cors_origins: default_cors_origins(),
            enable_rate_limiting: default_rate_limiting(),
            rate_limit: default_rate_limit(),
            enable_compression: default_compression_enabled(),
            enable_websocket: default_websocket_enabled(),
            websocket: WebSocketConfig::default(),
            enable_static_files: default_static_files_enabled(),
            static_files_dir: default_static_files_dir(),
            enable_api_docs: default_api_docs_enabled(),
            api_docs_path: default_api_docs_path(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            external_addr: None,
            interface: None,
            enable_ipv6: default_ipv6_enabled(),
            tcp_keep_alive: default_tcp_keep_alive(),
            tcp_keep_alive_interval: default_tcp_keep_alive_interval(),
            tcp_keep_alive_probes: default_tcp_keep_alive_probes(),
            tcp_keep_alive_time: default_tcp_keep_alive_time(),
            tcp_no_delay: default_tcp_no_delay(),
            tcp_reuse_addr: default_tcp_reuse_addr(),
            tcp_linger: None,
            socket_buffer_size: default_socket_buffer_size(),
            enable_multicast: default_multicast_enabled(),
            multicast_addr: None,
            multicast_interface: None,
            multicast_ttl: default_multicast_ttl(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_auth: default_auth_enabled(),
            auth_method: default_auth_method(),
            jwt_secret: None,
            jwt_expiration: default_jwt_expiration(),
            jwt_refresh_expiration: default_jwt_refresh_expiration(),
            enable_authorization: default_authorization_enabled(),
            authorization_policy: None,
            enable_encryption: default_encryption_enabled(),
            encryption_algorithm: default_encryption_algorithm(),
            encryption_key: None,
            enable_rate_limiting: default_security_rate_limiting(),
            rate_limit_window: default_rate_limit_window(),
            rate_limit_max_requests: default_rate_limit_max_requests(),
            enable_ip_whitelist: default_ip_whitelist_enabled(),
            ip_whitelist: Vec::new(),
            enable_ip_blacklist: default_ip_blacklist_enabled(),
            ip_blacklist: Vec::new(),
            enable_audit_logging: default_audit_logging_enabled(),
            audit_log_file: default_audit_log_file(),
            enable_session_management: default_session_management_enabled(),
            session_timeout: default_session_timeout(),
            session_cleanup_interval: default_session_cleanup_interval(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
            file: None,
            enable_console: default_console_logging(),
            enable_file: default_file_logging(),
            enable_syslog: default_syslog_enabled(),
            syslog_facility: default_syslog_facility(),
            enable_structured: default_structured_logging(),
            enable_json: default_json_logging(),
            enable_rotation: default_log_rotation(),
            rotation_size: default_log_rotation_size(),
            rotation_count: default_log_rotation_count(),
            enable_compression: default_log_compression(),
            timestamp_format: default_log_timestamp_format(),
            enable_color: default_log_color(),
            enable_thread_ids: default_log_thread_ids(),
            enable_target_filtering: default_log_target_filtering(),
            include_targets: Vec::new(),
            exclude_targets: Vec::new(),
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enable_validation: default_validation_enabled(),
            mode: default_validation_mode(),
            strict: default_strict_validation(),
            enable_schema: default_schema_validation(),
            schema_file: None,
            enable_custom: default_custom_validation(),
            custom_rules_file: None,
            enable_cross_field: default_cross_field_validation(),
            enable_dependency: default_dependency_validation(),
            enable_conditional: default_conditional_validation(),
            error_mode: default_validation_error_mode(),
            max_errors: default_max_validation_errors(),
            enable_caching: default_validation_caching(),
            cache_size: default_validation_cache_size(),
            cache_ttl: default_validation_cache_ttl(),
        }
    }
}

impl Default for GrpcClientConfig {
    fn default() -> Self {
        Self {
            server_addr: default_grpc_client_addr(),
            connection_timeout: default_grpc_client_connection_timeout(),
            request_timeout: default_grpc_client_request_timeout(),
            max_message_size: default_grpc_client_max_message_size(),
            enable_retry: default_grpc_client_retry_enabled(),
            max_retries: default_grpc_client_max_retries(),
            retry_backoff: default_grpc_client_retry_backoff(),
            enable_pooling: default_grpc_client_pooling_enabled(),
            pool_size: default_grpc_client_pool_size(),
            tls: None,
        }
    }
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            bind_address: default_websocket_addr(),
            port: default_websocket_port(),
            max_message_size: default_websocket_max_message_size(),
            connection_timeout: default_websocket_connection_timeout(),
            ping_interval: default_websocket_ping_interval(),
            pong_timeout: default_websocket_pong_timeout(),
            max_connections: default_websocket_max_connections(),
            enable_compression: default_websocket_compression_enabled(),
            enable_subprotocols: default_websocket_subprotocols_enabled(),
            subprotocols: default_websocket_subprotocols(),
            tls: None,
        }
    }
}
