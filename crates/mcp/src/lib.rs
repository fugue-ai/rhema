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

pub mod auth;
pub mod cache;
pub mod context;
pub mod http_server;
pub mod mcp;
pub mod official_sdk;
pub mod sdk;
pub mod watcher;

// Re-export configuration types
pub use mcp::{
    McpConfig, McpDaemon, HealthStatus, MemoryUsage, ClientConnection, ClientType,
    DaemonStatistics, RateLimitConfig, StartupConfig, AuthConfig, WatcherConfig, CacheConfig, LoggingConfig
};

pub use auth::{AuthManager, AuthResult, AuthToken, AuthStats, ClientInfo, SecurityEventType, SecuritySeverity, AuditEventType, AuditResult};
pub use cache::{CacheManager, CacheStatistics};
pub use context::ContextProvider;
pub use http_server::{HttpServer, PerformanceMetrics, ConnectionPool, ConnectionGuard, StringCache, EnhancedConnectionPool, EnhancedConnectionGuard, ConnectionPoolStats};
pub use official_sdk::{OfficialRhemaMcpServer, MCP_VERSION, SUPPORTED_VERSIONS};
pub use sdk::{RhemaMcpServer, Resource, Tool, Prompt, PromptSegment, ToolResult, ContextProviderExt};
pub use watcher::{FileWatcher, WatcherConfig as FileWatcherConfig};

/// Main MCP service that coordinates all components
pub struct RhemaMcpService {
    daemon: McpDaemon,
    http_server: Option<HttpServer>,
    official_sdk_server: Option<OfficialRhemaMcpServer>,
}

impl RhemaMcpService {
    /// Create a new MCP service
    pub async fn new(config: McpConfig, repo_root: std::path::PathBuf) -> rhema_core::RhemaResult<Self> {
        let daemon = McpDaemon::new(config.clone(), repo_root).await?;
        
        Ok(Self {
            daemon,
            http_server: None,
            official_sdk_server: None,
        })
    }

    /// Start the MCP service
    pub async fn start(&mut self) -> rhema_core::RhemaResult<()> {
        // Start the daemon
        self.daemon.start().await?;

        // Start HTTP server if enabled
        if self.daemon.config().port > 0 {
            let http_server = HttpServer::new(self.daemon.config().clone(), Arc::new(self.daemon.clone()));
            self.http_server = Some(http_server);
        }

        // Start official SDK server if enabled
        if self.daemon.config().use_official_sdk {
            let official_sdk_server = OfficialRhemaMcpServer::new(
                Arc::new(self.daemon.get_context_provider().clone()),
                Arc::new(self.daemon.get_cache_manager().clone()),
                Arc::new(self.daemon.get_file_watcher().clone()),
                Arc::new(self.daemon.get_auth_manager().clone()),
                self.daemon.config(),
            ).await?;
            
            self.official_sdk_server = Some(official_sdk_server);
            
            // Start the official SDK server
            if let Some(ref mut server) = self.official_sdk_server {
                server.start(self.daemon.config()).await?;
            }
            
            tracing::info!("Official SDK server started successfully");
        }

        Ok(())
    }

    /// Stop the MCP service
    pub async fn stop(&mut self) -> rhema_core::RhemaResult<()> {
        // Stop the official SDK server if running
        if let Some(ref mut server) = self.official_sdk_server {
            server.stop().await?;
        }
        
        // Stop the daemon
        self.daemon.stop().await?;
        
        // Clean up servers
        self.http_server = None;
        self.official_sdk_server = None;
        
        Ok(())
    }

    /// Get the daemon instance
    pub fn daemon(&self) -> &McpDaemon {
        &self.daemon
    }

    /// Get the daemon instance mutably
    pub fn daemon_mut(&mut self) -> &mut McpDaemon {
        &mut self.daemon
    }

    /// Check if the service is running
    pub async fn is_running(&self) -> bool {
        self.daemon.is_running().await
    }

    /// Get health status
    pub async fn health(&self) -> HealthStatus {
        self.daemon.health().await
    }

    /// Get service statistics
    pub async fn statistics(&self) -> DaemonStatistics {
        self.daemon.get_statistics().await
    }
}

use std::sync::Arc;

#[cfg(test)]
mod tests;
