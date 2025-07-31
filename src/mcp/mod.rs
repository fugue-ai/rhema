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

// Core MCP modules
pub mod context;
pub mod cache;
pub mod watcher;
pub mod auth;

// New official MCP SDK implementation
pub mod sdk;

pub use context::*;
pub use cache::*;
pub use watcher::*;
pub use auth::*;

// Re-export new SDK implementation
pub use sdk::{RhemaMcpServer, ContextProviderExt, Resource as SdkResource, Tool as SdkTool, Prompt as SdkPrompt, ToolResult as SdkToolResult};

use crate::RhemaResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

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
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            unix_socket: None,
            redis_url: None,
            auth: AuthConfig::default(),
            watcher: WatcherConfig::default(),
            cache: CacheConfig::default(),
            logging: LoggingConfig::default(),
            use_official_sdk: true,
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
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            structured: true,
            file: None,
        }
    }
}

/// MCP Daemon instance
#[derive(Clone)]
pub struct McpDaemon {
    config: McpConfig,
    #[allow(dead_code)]
    context_provider: Arc<ContextProvider>,
    cache_manager: Arc<CacheManager>,
    file_watcher: Arc<FileWatcher>,
    #[allow(dead_code)]
    auth_manager: Arc<AuthManager>,
    connections: Arc<RwLock<HashMap<String, ClientConnection>>>,
    // New official SDK server
    sdk_server: Option<Arc<RhemaMcpServer>>,
}

impl McpDaemon {
    /// Create a new MCP daemon instance
    pub async fn new(config: McpConfig, repo_root: PathBuf) -> RhemaResult<Self> {
        let context_provider = Arc::new(ContextProvider::new(repo_root.clone())?);
        let cache_manager = Arc::new(CacheManager::new(&config.cache).await?);
        let file_watcher = Arc::new(FileWatcher::new(&config.watcher, repo_root).await?);
        let auth_manager = Arc::new(AuthManager::new(&config.auth)?);
        let connections = Arc::new(RwLock::new(HashMap::new()));

        // Initialize official SDK server if enabled
        let sdk_server = if config.use_official_sdk {
            Some(Arc::new(RhemaMcpServer::new(
                context_provider.clone(),
                cache_manager.clone(),
                file_watcher.clone(),
                auth_manager.clone(),
            )?))
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
            sdk_server,
        })
    }

    /// Start the MCP daemon
    pub async fn start(&self) -> RhemaResult<()> {
        tracing::info!("Starting MCP daemon on {}:{}", self.config.host, self.config.port);
        
        // Start file watcher
        if self.config.watcher.enabled {
            self.file_watcher.start().await?;
        }
        
        // Start official SDK server
        self.start_official_sdk_server().await?;
        
        Ok(())
    }

    /// Stop the MCP daemon
    pub async fn stop(&self) -> RhemaResult<()> {
        tracing::info!("Stopping MCP daemon");
        
        // Stop file watcher
        self.file_watcher.stop().await?;
        
        // Close all connections
        let mut connections = self.connections.write().await;
        for (_, connection) in connections.drain() {
            connection.close().await?;
        }
        
        Ok(())
    }

    /// Get daemon health status
    pub async fn health(&self) -> HealthStatus {
        HealthStatus {
            status: "healthy".to_string(),
            uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            connections: self.connections.read().await.len(),
            cache_hit_rate: self.cache_manager.hit_rate().await,
            memory_usage: self.get_memory_usage().await,
        }
    }

    async fn start_official_sdk_server(&self) -> RhemaResult<()> {
        if let Some(server) = &self.sdk_server {
            tracing::info!("Starting official MCP SDK server on {}:{}", self.config.host, self.config.port);
            server.start(&self.config).await?;
        }
        Ok(())
    }

    async fn get_memory_usage(&self) -> MemoryUsage {
        // Implementation will be added
        MemoryUsage {
            used: 0,
            total: 0,
            cache_size: 0,
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
}

/// Memory usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub used: u64,
    pub total: u64,
    pub cache_size: u64,
}

/// Client connection information
#[derive(Debug)]
pub struct ClientConnection {
    pub id: String,
    pub client_type: ClientType,
    pub connected_at: std::time::Instant,
    pub last_activity: std::time::Instant,
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
        // Implementation will be added
        Ok(())
    }
} 