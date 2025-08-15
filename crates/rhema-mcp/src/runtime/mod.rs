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

use rhema_core::RhemaResult;
use std::sync::Arc;

use crate::mcp::McpConfig;
use crate::{AuthManager, CacheManager, ContextProvider, FileWatcher};

pub mod http_server;
pub mod official_sdk;
pub mod sdk;

// Re-export the main types from each module
pub use http_server::{
    ConnectionGuard, ConnectionPool, ConnectionPoolStats, EnhancedConnectionGuard,
    EnhancedConnectionPool, HttpServer, PerformanceMetrics, StringCache,
};
pub use official_sdk::{OfficialRhemaMcpServer, MCP_VERSION, SUPPORTED_VERSIONS};
pub use sdk::{
    ContextProviderExt, Prompt, PromptSegment, Resource, RhemaMcpServer, Tool, ToolResult,
};

/// Runtime type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeType {
    /// HTTP Server runtime for web-based communication
    HTTP,
    /// Official MCP SDK runtime for protocol compliance
    RMCP,
    /// Custom SDK runtime for simplified implementation
    INTERNAL,
}

/// Runtime selection criteria
#[derive(Debug, Clone)]
pub struct RuntimeSelection {
    /// The type of runtime to use
    pub runtime_type: RuntimeType,
    /// Whether to enable HTTP server capabilities
    pub enable_http: bool,
    /// Whether to use official MCP protocol
    pub use_official_protocol: bool,
    /// Whether to enable performance monitoring
    pub enable_monitoring: bool,
    /// Whether to enable caching
    pub enable_caching: bool,
    /// Maximum concurrent connections (for HTTP server)
    pub max_connections: Option<usize>,
}

impl Default for RuntimeSelection {
    fn default() -> Self {
        Self {
            runtime_type: RuntimeType::RMCP,
            enable_http: true,
            use_official_protocol: true,
            enable_monitoring: true,
            enable_caching: true,
            max_connections: None,
        }
    }
}

/// Helper function to choose the appropriate runtime based on configuration
///
/// This function analyzes the MCP configuration and determines the best runtime
/// to use based on the following criteria:
///
/// - If `port > 0`: Uses HTTP Server runtime for web-based communication
/// - If `use_official_sdk = true`: Uses Official SDK runtime for protocol compliance
/// - Otherwise: Uses Custom SDK runtime for simplified implementation
///
/// # Examples
///
/// ```rust
/// use rhema_mcp::runtime::{choose_runtime, RuntimeType};
/// use rhema_mcp::mcp::McpConfig;
///
/// let config = McpConfig {
///     port: 8080,
///     use_official_sdk: false,
///     // ... other fields
/// };
///
/// let selection = choose_runtime(&config);
/// assert_eq!(selection.runtime_type, RuntimeType::HttpServer);
/// ```
pub fn choose_runtime(config: &McpConfig) -> RuntimeSelection {
    let mut selection = RuntimeSelection::default();

    // Determine runtime type based on configuration
    if config.port > 0 {
        // If HTTP port is configured, prefer HTTP server
        selection.runtime_type = RuntimeType::HTTP;
        selection.enable_http = true;
        selection.max_connections = config.max_connections;
    } else if config.use_official_sdk {
        // If official SDK is enabled, use official SDK runtime
        selection.runtime_type = RuntimeType::RMCP;
        selection.use_official_protocol = true;
    } else {
        // Default to custom SDK runtime
        selection.runtime_type = RuntimeType::INTERNAL;
        selection.use_official_protocol = false;
    }

    // Enable monitoring if logging is configured
    selection.enable_monitoring = config.logging.level != "off";

    // Enable caching if Redis is configured or cache is enabled
    selection.enable_caching = config.redis_url.is_some() || config.cache.memory_enabled;

    selection
}

/// Create the appropriate runtime instance based on selection
///
/// This function creates a runtime instance based on the provided selection.
/// Note that HTTP Server runtime requires a daemon instance, so use
/// `create_runtime_with_daemon` if you need HTTP server support.
///
/// # Arguments
///
/// * `selection` - The runtime selection criteria
/// * `context_provider` - Context provider for the runtime
/// * `cache_manager` - Cache manager for the runtime
/// * `file_watcher` - File watcher for the runtime
/// * `auth_manager` - Authentication manager for the runtime
/// * `config` - MCP configuration
///
/// # Returns
///
/// Returns a `RuntimeInstance` enum containing the appropriate runtime.
/// For HTTP Server runtime, returns an error indicating that a daemon is required.
///
/// # Examples
///
/// ```rust
/// use rhema_mcp::runtime::{choose_runtime, create_runtime, RuntimeType};
/// use rhema_mcp::mcp::McpConfig;
///
/// let config = McpConfig {
///     port: 0,
///     use_official_sdk: true,
///     // ... other fields
/// };
///
/// let selection = choose_runtime(&config);
/// let runtime = create_runtime(
///     &selection,
///     context_provider,
///     cache_manager,
///     file_watcher,
///     auth_manager,
///     &config
/// ).await?;
///
/// assert_eq!(runtime.runtime_type(), RuntimeType::OfficialSdk);
/// ```
pub async fn create_runtime(
    selection: &RuntimeSelection,
    context_provider: Arc<ContextProvider>,
    cache_manager: Arc<CacheManager>,
    file_watcher: Arc<FileWatcher>,
    auth_manager: Arc<AuthManager>,
    config: &McpConfig,
) -> RhemaResult<RuntimeInstance> {
    match selection.runtime_type {
        RuntimeType::HTTP => {
            // For HTTP server, we need a daemon instance
            // This helper function assumes the daemon is created separately
            // Use create_http_runtime_with_daemon if you have a daemon instance
            Err(rhema_core::RhemaError::ConfigError(
                "HTTP server runtime requires a daemon instance. Use create_http_runtime_with_daemon instead.".to_string()
            ))
        }
        RuntimeType::RMCP => {
            let official_sdk = OfficialRhemaMcpServer::new(
                context_provider,
                cache_manager,
                file_watcher,
                auth_manager,
                config,
            )
            .await?;
            Ok(RuntimeInstance::OfficialSdk(official_sdk))
        }
        RuntimeType::INTERNAL => {
            let sdk =
                RhemaMcpServer::new(context_provider, cache_manager, file_watcher, auth_manager)?;
            Ok(RuntimeInstance::Sdk(sdk))
        }
    }
}

/// Create HTTP server runtime with an existing daemon instance
///
/// This function creates an HTTP server runtime when you already have a daemon instance.
///
/// # Arguments
///
/// * `config` - MCP configuration
/// * `daemon` - Arc-wrapped MCP daemon instance
///
/// # Returns
///
/// Returns a `RuntimeInstance::HttpServer` containing the HTTP server.
///
/// # Examples
///
/// ```rust
/// use rhema_mcp::runtime::{create_http_runtime_with_daemon, RuntimeType};
/// use rhema_mcp::mcp::{McpConfig, McpDaemon};
///
/// let config = McpConfig { /* ... */ };
/// let daemon = Arc::new(McpDaemon::new(config.clone(), repo_root).await?);
///
/// let runtime = create_http_runtime_with_daemon(config, daemon);
/// assert_eq!(runtime.runtime_type(), RuntimeType::HttpServer);
/// ```
pub fn create_http_runtime_with_daemon(
    config: McpConfig,
    daemon: Arc<crate::mcp::McpDaemon>,
) -> RuntimeInstance {
    let http_server = HttpServer::new(config, daemon);
    RuntimeInstance::HttpServer(http_server)
}

/// Create the appropriate runtime instance with optional daemon for HTTP server
///
/// This function creates a runtime instance and can handle HTTP server runtime
/// when a daemon instance is provided.
///
/// # Arguments
///
/// * `selection` - The runtime selection criteria
/// * `context_provider` - Context provider for the runtime
/// * `cache_manager` - Cache manager for the runtime
/// * `file_watcher` - File watcher for the runtime
/// * `auth_manager` - Authentication manager for the runtime
/// * `config` - MCP configuration
/// * `daemon` - Optional daemon instance (required for HTTP server runtime)
///
/// # Returns
///
/// Returns a `RuntimeInstance` enum containing the appropriate runtime.
///
/// # Examples
///
/// ```rust
/// use rhema_mcp::runtime::{choose_runtime, create_runtime_with_daemon, RuntimeType};
/// use rhema_mcp::mcp::{McpConfig, McpDaemon};
///
/// let config = McpConfig {
///     port: 8080,
///     use_official_sdk: false,
///     // ... other fields
/// };
///
/// let selection = choose_runtime(&config);
/// let daemon = Arc::new(McpDaemon::new(config.clone(), repo_root).await?);
///
/// let runtime = create_runtime_with_daemon(
///     &selection,
///     context_provider,
///     cache_manager,
///     file_watcher,
///     auth_manager,
///     &config,
///     Some(daemon)
/// ).await?;
///
/// assert_eq!(runtime.runtime_type(), RuntimeType::HttpServer);
/// ```
pub async fn create_runtime_with_daemon(
    selection: &RuntimeSelection,
    context_provider: Arc<ContextProvider>,
    cache_manager: Arc<CacheManager>,
    file_watcher: Arc<FileWatcher>,
    auth_manager: Arc<AuthManager>,
    config: &McpConfig,
    daemon: Option<Arc<crate::mcp::McpDaemon>>,
) -> RhemaResult<RuntimeInstance> {
    match selection.runtime_type {
        RuntimeType::HTTP => {
            if let Some(daemon) = daemon {
                Ok(create_http_runtime_with_daemon(config.clone(), daemon))
            } else {
                Err(rhema_core::RhemaError::ConfigError(
                    "HTTP server runtime requires a daemon instance.".to_string(),
                ))
            }
        }
        RuntimeType::RMCP => {
            let official_sdk = OfficialRhemaMcpServer::new(
                context_provider,
                cache_manager,
                file_watcher,
                auth_manager,
                config,
            )
            .await?;
            Ok(RuntimeInstance::OfficialSdk(official_sdk))
        }
        RuntimeType::INTERNAL => {
            let sdk =
                RhemaMcpServer::new(context_provider, cache_manager, file_watcher, auth_manager)?;
            Ok(RuntimeInstance::Sdk(sdk))
        }
    }
}

/// Enum representing the different runtime instances
///
/// This enum provides a unified interface for all three runtime types:
/// - HTTP Server runtime for web-based communication
/// - Official SDK runtime for protocol compliance
/// - Custom SDK runtime for simplified implementation
///
/// # Examples
///
/// ```rust
/// use rhema_mcp::runtime::{RuntimeInstance, RuntimeType};
///
/// // You can match on the runtime type
/// match runtime {
///     RuntimeInstance::HttpServer(server) => {
///         println!("HTTP Server runtime");
///     }
///     RuntimeInstance::OfficialSdk(server) => {
///         println!("Official SDK runtime");
///     }
///     RuntimeInstance::Sdk(server) => {
///         println!("Custom SDK runtime");
///     }
/// }
///
/// // Or use the runtime_type() method
/// assert_eq!(runtime.runtime_type(), RuntimeType::HttpServer);
/// ```
pub enum RuntimeInstance {
    HttpServer(HttpServer),
    OfficialSdk(OfficialRhemaMcpServer),
    Sdk(RhemaMcpServer),
}

impl RuntimeInstance {
    /// Get the runtime type of this instance
    ///
    /// Returns the `RuntimeType` enum variant that corresponds to this runtime instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rhema_mcp::runtime::{RuntimeInstance, RuntimeType};
    ///
    /// let runtime_type = runtime.runtime_type();
    /// match runtime_type {
    ///     RuntimeType::HttpServer => println!("HTTP Server"),
    ///     RuntimeType::OfficialSdk => println!("Official SDK"),
    ///     RuntimeType::Sdk => println!("Custom SDK"),
    /// }
    /// ```
    pub fn runtime_type(&self) -> RuntimeType {
        match self {
            RuntimeInstance::HttpServer(_) => RuntimeType::HTTP,
            RuntimeInstance::OfficialSdk(_) => RuntimeType::RMCP,
            RuntimeInstance::Sdk(_) => RuntimeType::INTERNAL,
        }
    }

    /// Start the runtime
    ///
    /// Starts the runtime instance. Note that HTTP Server and Custom SDK runtimes
    /// don't require explicit starting, so this method returns `Ok(())` for them.
    ///
    /// # Arguments
    ///
    /// * `config` - MCP configuration for the runtime
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if starting failed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rhema_mcp::runtime::RuntimeInstance;
    ///
    /// let mut runtime = create_runtime(&selection, ...).await?;
    /// runtime.start(&config).await?;
    /// ```
    pub async fn start(&mut self, config: &McpConfig) -> RhemaResult<()> {
        match self {
            RuntimeInstance::HttpServer(server) => {
                // HTTP server is typically started separately
                Ok(())
            }
            RuntimeInstance::OfficialSdk(server) => server.start(config).await,
            RuntimeInstance::Sdk(_) => {
                // SDK runtime doesn't need explicit start
                Ok(())
            }
        }
    }

    /// Stop the runtime
    ///
    /// Stops the runtime instance. Note that HTTP Server and Custom SDK runtimes
    /// don't require explicit stopping, so this method returns `Ok(())` for them.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if stopping failed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rhema_mcp::runtime::RuntimeInstance;
    ///
    /// let mut runtime = create_runtime(&selection, ...).await?;
    /// // ... use runtime ...
    /// runtime.stop().await?;
    /// ```
    pub async fn stop(&mut self) -> RhemaResult<()> {
        match self {
            RuntimeInstance::HttpServer(_) => {
                // HTTP server is typically stopped separately
                Ok(())
            }
            RuntimeInstance::OfficialSdk(server) => server.stop().await,
            RuntimeInstance::Sdk(_) => {
                // SDK runtime doesn't need explicit stop
                Ok(())
            }
        }
    }
}
