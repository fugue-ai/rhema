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

// Import types from other modules
use crate::auth::AuthManager;
use crate::cache::CacheConfig as CacheManagerConfig;
use crate::cache::{
    AnalyticsConfig, CacheManager, CoherencyConfig, CompressionConfig, EvictionPolicy,
    HealthConfig, MonitoringConfig, OptimizationConfig, PartitioningConfig, PersistenceConfig,
    PrefetchingConfig, ValidationConfig, WarmingStrategy,
};
use crate::context::ContextProvider;
use crate::http_server::HttpServer;
use crate::official_sdk::OfficialRhemaMcpServer;
use crate::sdk::{
    ContextProviderExt, Prompt as SdkPrompt, Resource as SdkResource, RhemaMcpServer,
    Tool as SdkTool, ToolResult as SdkToolResult,
};
use crate::watcher::FileWatcher;

use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// MCP Daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Daemon host address
    pub host: String,

    /// Daemon port
    pub port: u16,

    /// Unix socket path for local communication
    pub unix_socket: Option<PathBuf>,

    /// Redis connection URL for distributed caching
    pub redis_url: Option<String>,

    /// Authentication settings
    pub auth: AuthConfig,

    /// File system watching settings
    pub watcher: WatcherConfig,

    /// Cache settings
    pub cache: CacheConfig,

    /// Logging settings
    pub logging: LoggingConfig,

    /// Use official MCP SDK (default: true)
    pub use_official_sdk: bool,

    /// Daemon startup settings
    pub startup: StartupConfig,

    /// Maximum concurrent connections
    pub max_connections: Option<usize>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Enable authentication
    pub enabled: bool,

    /// API key for client authentication
    pub api_key: Option<String>,

    /// JWT secret for token-based auth
    pub jwt_secret: Option<String>,

    /// Allowed origins for CORS
    pub allowed_origins: Vec<String>,

    /// Rate limiting settings
    pub rate_limiting: RateLimitConfig,

    /// Audit logging settings
    pub audit_logging: AuditLoggingConfig,

    /// Security settings
    pub security: SecurityConfig,
}

/// Audit logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLoggingConfig {
    /// Enable audit logging
    pub enabled: bool,

    /// Audit log file path
    pub log_file: Option<PathBuf>,

    /// Audit log level
    pub log_level: String,

    /// Events to audit
    pub events: Vec<String>,
}

impl Default for AuditLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_file: None,
            log_level: "info".to_string(),
            events: vec![
                "authentication".to_string(),
                "authorization".to_string(),
                "security_violation".to_string(),
                "rate_limit_violation".to_string(),
            ],
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable brute force protection
    pub brute_force_protection: bool,

    /// Maximum failed attempts before lockout
    pub max_failed_attempts: u32,

    /// Lockout duration in seconds
    pub lockout_duration_seconds: u64,

    /// Enable security monitoring
    pub security_monitoring: bool,

    /// Enable token encryption at rest
    pub token_encryption: bool,

    /// Enable secure headers
    pub secure_headers: bool,

    /// Enable input sanitization
    pub input_sanitization: bool,

    /// Invalidate session on IP address change
    pub invalidate_session_on_ip_change: bool,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute for HTTP API
    pub http_requests_per_minute: u32,

    /// Messages per minute for WebSocket
    pub websocket_messages_per_minute: u32,

    /// Messages per minute for Unix socket
    pub unix_socket_messages_per_minute: u32,
}

/// File system watcher configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherConfig {
    /// Enable file system watching
    pub enabled: bool,

    /// Watch directories
    pub watch_dirs: Vec<PathBuf>,

    /// File patterns to watch
    pub file_patterns: Vec<String>,

    /// Debounce interval in milliseconds
    pub debounce_ms: u64,

    /// Watch directories recursively
    pub recursive: bool,

    /// Ignore hidden files and directories
    pub ignore_hidden: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable in-memory caching
    pub memory_enabled: bool,

    /// Enable Redis caching
    pub redis_enabled: bool,

    /// Redis connection URL for distributed caching
    pub redis_url: Option<String>,

    /// Cache TTL in seconds
    pub ttl_seconds: u64,

    /// Maximum cache size
    pub max_size: usize,

    /// Cache compression enabled
    pub compression_enabled: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,

    /// Enable structured logging
    pub structured: bool,

    /// Log file path
    pub file: Option<PathBuf>,

    /// Enable JSON logging
    pub json: bool,
}

/// Startup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupConfig {
    /// Graceful shutdown timeout in seconds
    pub graceful_shutdown_timeout: u64,

    /// Health check interval in seconds
    pub health_check_interval: u64,

    /// Connection timeout in seconds
    pub connection_timeout: u64,

    /// Enable auto-restart on failure
    pub auto_restart: bool,

    /// Maximum restart attempts
    pub max_restart_attempts: u32,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            unix_socket: None,
            redis_url: None,
            max_connections: Some(1000),
            auth: AuthConfig::default(),
            watcher: WatcherConfig::default(),
            cache: CacheConfig::default(),
            logging: LoggingConfig::default(),
            use_official_sdk: true,
            startup: StartupConfig::default(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: None,
            jwt_secret: None,
            allowed_origins: vec!["*".to_string()],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: AuditLoggingConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            brute_force_protection: true,
            max_failed_attempts: 5,
            lockout_duration_seconds: 300, // 5 minutes
            security_monitoring: true,
            token_encryption: true,
            secure_headers: true,
            input_sanitization: true,
            invalidate_session_on_ip_change: true,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            http_requests_per_minute: 1000,
            websocket_messages_per_minute: 100,
            unix_socket_messages_per_minute: 1000,
        }
    }
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_dirs: vec![PathBuf::from(".rhema")],
            file_patterns: vec!["*.yaml".to_string(), "*.yml".to_string()],
            debounce_ms: 100,
            recursive: true,
            ignore_hidden: true,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            memory_enabled: true,
            redis_enabled: false,
            redis_url: None,
            ttl_seconds: 3600,
            max_size: 10000,
            compression_enabled: true,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            structured: true,
            file: None,
            json: false,
        }
    }
}

impl Default for StartupConfig {
    fn default() -> Self {
        Self {
            graceful_shutdown_timeout: 30,
            health_check_interval: 30,
            connection_timeout: 60,
            auto_restart: false,
            max_restart_attempts: 3,
        }
    }
}

#[derive(Clone)]
pub struct McpDaemon {
    config: McpConfig,
    context_provider: Arc<ContextProvider>,
    cache_manager: Arc<CacheManager>,
    file_watcher: Arc<FileWatcher>,
    auth_manager: Arc<AuthManager>,
    connections: Arc<RwLock<HashMap<String, ClientConnection>>>,
    official_sdk_server: Option<OfficialRhemaMcpServer>,
    http_server: Option<HttpServer>,
    // Daemon state tracking
    start_time: Instant,
    uptime: Arc<RwLock<Duration>>,
    is_running: Arc<RwLock<bool>>,
    restart_count: Arc<RwLock<u32>>,
    // Performance metrics
    request_count: Arc<RwLock<u64>>,
    error_count: Arc<RwLock<u64>>,
    last_health_check: Arc<RwLock<Instant>>,
}

impl McpDaemon {
    /// Create a new MCP daemon instance
    pub async fn new(config: McpConfig, repo_root: PathBuf) -> RhemaResult<Self> {
        let context_provider = Arc::new(ContextProvider::new(repo_root.clone())?);

        // Convert config types
        let cache_config = CacheManagerConfig {
            memory_enabled: config.cache.memory_enabled,
            redis_enabled: config.cache.redis_enabled,
            redis_url: config.cache.redis_url.clone(),
            ttl_seconds: config.cache.ttl_seconds,
            max_size: config.cache.max_size,
            compression_enabled: config.cache.compression_enabled,
            eviction_policy: EvictionPolicy::LRU,
            warming: WarmingStrategy::default(),
            monitoring: MonitoringConfig::default(),
            optimization: OptimizationConfig::default(),
            validation: ValidationConfig::default(),
            persistence: PersistenceConfig::default(),
            compression: CompressionConfig::default(),
            partitioning: PartitioningConfig::default(),
            coherency: CoherencyConfig::default(),
            prefetching: PrefetchingConfig::default(),
            analytics: AnalyticsConfig::default(),
            health: HealthConfig::default(),
        };

        let watcher_config = super::FileWatcherConfig {
            enabled: config.watcher.enabled,
            watch_dirs: config.watcher.watch_dirs.clone(),
            file_patterns: config.watcher.file_patterns.clone(),
            debounce_ms: config.watcher.debounce_ms,
            recursive: config.watcher.recursive,
            ignore_hidden: config.watcher.ignore_hidden,
        };

        let cache_manager = Arc::new(CacheManager::new(&cache_config).await?);
        let file_watcher = Arc::new(FileWatcher::new(&watcher_config, repo_root).await?);
        let auth_manager = Arc::new(AuthManager::new(&config.auth)?);
        let connections = Arc::new(RwLock::new(HashMap::new()));

        // Initialize official SDK server if enabled
        let official_sdk_server = if config.use_official_sdk {
            Some(
                OfficialRhemaMcpServer::new(
                    context_provider.clone(),
                    cache_manager.clone(),
                    file_watcher.clone(),
                    auth_manager.clone(),
                    &config,
                )
                .await?,
            )
        } else {
            None
        };

        Ok(Self {
            config,
            context_provider,
            cache_manager,
            file_watcher,
            auth_manager,
            connections,
            official_sdk_server,
            http_server: None, // Will be initialized in start()
            start_time: Instant::now(),
            uptime: Arc::new(RwLock::new(Duration::ZERO)),
            is_running: Arc::new(RwLock::new(false)),
            restart_count: Arc::new(RwLock::new(0)),
            request_count: Arc::new(RwLock::new(0)),
            error_count: Arc::new(RwLock::new(0)),
            last_health_check: Arc::new(RwLock::new(Instant::now())),
        })
    }

    /// Start the MCP daemon
    pub async fn start(&mut self) -> RhemaResult<()> {
        info!(
            "Starting MCP daemon on {}:{}",
            self.config.host, self.config.port
        );

        // Mark daemon as running
        *self.is_running.write().await = true;

        // Start uptime tracking
        self.start_uptime_tracking().await;

        // Start file watcher
        if self.config.watcher.enabled {
            self.file_watcher.start().await?;
        }

        // Start official SDK server
        self.start_official_sdk_server().await?;

        // Start health check monitoring
        self.start_health_monitoring().await;

        // Start HTTP server
        self.start_http_server().await?;

        info!("MCP daemon started successfully");
        Ok(())
    }

    /// Stop the MCP daemon
    pub async fn stop(&mut self) -> RhemaResult<()> {
        info!("Stopping MCP daemon");

        // Mark daemon as not running
        *self.is_running.write().await = false;

        // Stop file watcher
        self.file_watcher.stop().await?;

        // Close all connections
        let mut connections = self.connections.write().await;
        for (_, connection) in connections.drain() {
            connection.close().await?;
        }

        // Stop official SDK server
        if let Some(server) = &mut self.official_sdk_server {
            server.stop().await?;
        }

        // Stop HTTP server
        if let Some(server) = &mut self.http_server {
            server.stop().await?;
        }

        info!("MCP daemon stopped successfully");
        Ok(())
    }

    /// Restart the MCP daemon
    pub async fn restart(&mut self) -> RhemaResult<()> {
        info!("Restarting MCP daemon");

        let current_restart_count = *self.restart_count.read().await;
        if current_restart_count >= self.config.startup.max_restart_attempts {
            error!(
                "Maximum restart attempts reached ({})",
                self.config.startup.max_restart_attempts
            );
            return Err(rhema_core::RhemaError::DaemonError(
                "Maximum restart attempts reached".to_string(),
            ));
        }

        // Increment restart count
        *self.restart_count.write().await = current_restart_count + 1;

        // Stop the daemon
        self.stop().await?;

        // Wait a moment before restarting
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Start the daemon
        self.start().await?;

        info!("MCP daemon restarted successfully");
        Ok(())
    }

    /// Get daemon health status
    pub async fn health(&self) -> HealthStatus {
        let uptime = *self.uptime.read().await;
        let connections = self.connections.read().await.len();
        let cache_hit_rate = self.cache_manager.hit_rate().await;
        let memory_usage = self.get_memory_usage().await;
        let request_count = *self.request_count.read().await;
        let error_count = *self.error_count.read().await;
        let is_running = *self.is_running.read().await;

        HealthStatus {
            status: if is_running {
                "healthy".to_string()
            } else {
                "stopped".to_string()
            },
            uptime: uptime.as_secs(),
            connections,
            cache_hit_rate,
            memory_usage,
            request_count,
            error_count,
            error_rate: if request_count > 0 {
                error_count as f64 / request_count as f64
            } else {
                0.0
            },
            restart_count: *self.restart_count.read().await,
        }
    }

    /// Get daemon uptime
    pub async fn get_uptime(&self) -> Duration {
        *self.uptime.read().await
    }

    /// Track a new client connection
    pub async fn track_connection(
        &self,
        client_id: String,
        client_type: ClientType,
    ) -> RhemaResult<()> {
        let connection = ClientConnection {
            id: client_id.clone(),
            client_type,
            connected_at: Instant::now(),
            last_activity: Instant::now(),
        };

        self.connections.write().await.insert(client_id, connection);
        Ok(())
    }

    /// Remove a client connection
    pub async fn remove_connection(&self, client_id: &str) -> RhemaResult<()> {
        self.connections.write().await.remove(client_id);
        Ok(())
    }

    /// Update client activity
    pub async fn update_client_activity(&self, client_id: &str) -> RhemaResult<()> {
        if let Some(connection) = self.connections.write().await.get_mut(client_id) {
            connection.last_activity = Instant::now();
        }
        Ok(())
    }

    /// Get connection count
    pub async fn get_connection_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// Increment request count
    pub async fn increment_request_count(&self) {
        *self.request_count.write().await += 1;
    }

    /// Increment error count
    pub async fn increment_error_count(&self) {
        *self.error_count.write().await += 1;
    }

    /// Start uptime tracking
    async fn start_uptime_tracking(&self) {
        let uptime = self.uptime.clone();
        let start_time = self.start_time;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                *uptime.write().await = start_time.elapsed();
            }
        });
    }

    /// Start health monitoring
    async fn start_health_monitoring(&self) {
        let health_interval = self.config.startup.health_check_interval;
        let daemon = self.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(health_interval)).await;

                // Update last health check time
                *daemon.last_health_check.write().await = Instant::now();

                // Perform health checks
                let health = daemon.health().await;
                if health.status != "healthy" {
                    warn!("Daemon health check failed: {:?}", health);
                }
            }
        });
    }

    async fn start_official_sdk_server(&mut self) -> RhemaResult<()> {
        if let Some(server) = &mut self.official_sdk_server {
            info!(
                "Starting official MCP SDK server on {}:{}",
                self.config.host, self.config.port
            );
            server.start(&self.config).await?;
        }
        Ok(())
    }

    async fn start_http_server(&mut self) -> RhemaResult<()> {
        let daemon_arc = Arc::new(self.clone());
        let http_server = HttpServer::new(self.config.clone(), daemon_arc);

        // Start HTTP server in background
        let server_clone = http_server.clone();
        tokio::spawn(async move {
            if let Err(e) = server_clone.start().await {
                error!("HTTP server failed to start: {}", e);
            }
        });

        // Start Unix socket server if configured
        if self.config.unix_socket.is_some() {
            let server_clone = http_server.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.start_unix_socket().await {
                    error!("Unix socket server failed to start: {}", e);
                }
            });
        }

        self.http_server = Some(http_server);
        Ok(())
    }

    /// Get a reference to the configuration
    pub fn config(&self) -> &McpConfig {
        &self.config
    }

    /// Get a reference to the context provider
    pub fn get_context_provider(&self) -> &ContextProvider {
        &self.context_provider
    }

    /// Get a reference to the cache manager
    pub fn get_cache_manager(&self) -> &CacheManager {
        &self.cache_manager
    }

    /// Get a reference to the file watcher
    pub fn get_file_watcher(&self) -> &FileWatcher {
        &self.file_watcher
    }

    /// Get a reference to the auth manager
    pub fn get_auth_manager(&self) -> &AuthManager {
        &self.auth_manager
    }

    /// Get memory usage statistics
    pub async fn get_memory_usage(&self) -> MemoryUsage {
        let mut used = 0u64;
        let mut total = 0u64;
        let mut cache_size = 0u64;

        // Get system memory info
        let mut sys = sysinfo::System::new_all();
        sys.refresh_memory();
        total = sys.total_memory() * 1024; // Convert KB to bytes
        used = (sys.total_memory() - sys.available_memory()) * 1024; // Convert KB to bytes

        // Get cache size
        cache_size = self.cache_manager.get_size().await;

        MemoryUsage {
            used,
            total,
            cache_size,
            used_mb: used / 1024 / 1024, // Convert to MB
        }
    }

    /// Get memory usage in MB format
    pub async fn get_memory_usage_mb(&self) -> MemoryUsage {
        let memory_usage = self.get_memory_usage().await;
        MemoryUsage {
            used: memory_usage.used / 1024 / 1024,   // Convert to MB
            total: memory_usage.total / 1024 / 1024, // Convert to MB
            cache_size: memory_usage.cache_size / 1024 / 1024, // Convert to MB
            used_mb: memory_usage.used_mb,
        }
    }

    /// Check if daemon is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Get daemon statistics
    pub async fn get_statistics(&self) -> DaemonStatistics {
        let health = self.health().await;
        let uptime = self.get_uptime().await;
        let connection_count = self.get_connection_count().await;
        let cache_stats = self.cache_manager.get_statistics().await;

        DaemonStatistics {
            uptime: uptime.as_secs(),
            connection_count,
            request_count: health.request_count,
            error_count: health.error_count,
            error_rate: health.error_rate,
            cache_hit_rate: health.cache_hit_rate,
            cache_stats,
            restart_count: health.restart_count,
            memory_usage: health.memory_usage,
        }
    }
}

/// Health status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime: u64,
    pub connections: usize,
    pub cache_hit_rate: f64,
    pub memory_usage: MemoryUsage,
    pub request_count: u64,
    pub error_count: u64,
    pub error_rate: f64,
    pub restart_count: u32,
}

/// Memory usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub used: u64,
    pub total: u64,
    pub cache_size: u64,
    pub used_mb: u64,
}

/// Daemon statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonStatistics {
    pub uptime: u64,
    pub connection_count: usize,
    pub request_count: u64,
    pub error_count: u64,
    pub error_rate: f64,
    pub cache_hit_rate: f64,
    pub cache_stats: crate::cache::CacheStatistics,
    pub restart_count: u32,
    pub memory_usage: MemoryUsage,
}

/// Client connection information
#[derive(Debug)]
pub struct ClientConnection {
    pub id: String,
    pub client_type: ClientType,
    pub connected_at: Instant,
    pub last_activity: Instant,
}

/// Client type enumeration
#[derive(Debug, Clone)]
pub enum ClientType {
    Http,
    WebSocket,
    UnixSocket,
}

impl ClientConnection {
    pub async fn close(&self) -> RhemaResult<()> {
        // Implementation for closing connection
        // This would typically close the underlying socket/stream
        Ok(())
    }

    pub fn is_idle(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed() > timeout
    }
}
