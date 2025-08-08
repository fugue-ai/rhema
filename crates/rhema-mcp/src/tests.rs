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

use crate::{
    auth::AuthManager,
    cache::CacheManager,
    context::ContextProvider,
    mcp::{AuthConfig, CacheConfig, McpConfig, McpDaemon, WatcherConfig},
    watcher::FileWatcher,
    RhemaMcpService,
};
use rhema_core::RhemaResult;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

#[tokio::test]
async fn test_mcp_daemon_creation() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let daemon = McpDaemon::new(config, temp_dir.path().to_path_buf()).await?;

    assert!(!daemon.is_running().await);
    Ok(())
}

#[tokio::test]
async fn test_mcp_daemon_health() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let mut daemon = McpDaemon::new(config, temp_dir.path().to_path_buf()).await?;

    // Start the daemon first
    daemon.start().await?;

    let health = daemon.health().await;
    assert_eq!(health.status, "healthy");
    assert_eq!(health.connections, 0);

    Ok(())
}

#[tokio::test]
async fn test_mcp_daemon_uptime() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let daemon = McpDaemon::new(config, temp_dir.path().to_path_buf()).await?;

    let uptime = daemon.get_uptime().await;
    assert!(uptime.as_secs() >= 0);

    Ok(())
}

#[tokio::test]
async fn test_mcp_daemon_connection_tracking() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let daemon = Arc::new(McpDaemon::new(config, temp_dir.path().to_path_buf()).await?);

    // Track a connection
    daemon
        .track_connection("test_client".to_string(), crate::mcp::ClientType::Http)
        .await?;
    assert_eq!(daemon.get_connection_count().await, 1);

    // Remove the connection
    daemon.remove_connection("test_client").await?;
    assert_eq!(daemon.get_connection_count().await, 0);

    Ok(())
}

#[tokio::test]
async fn test_mcp_daemon_statistics() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let daemon = Arc::new(McpDaemon::new(config, temp_dir.path().to_path_buf()).await?);

    let stats = daemon.get_statistics().await;
    assert_eq!(stats.connection_count, 0);
    assert_eq!(stats.request_count, 0);
    assert_eq!(stats.error_count, 0);

    Ok(())
}

#[tokio::test]
async fn test_mcp_daemon_memory_usage() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let daemon = Arc::new(McpDaemon::new(config, temp_dir.path().to_path_buf()).await?);

    let memory_usage = daemon.get_memory_usage().await;
    assert!(memory_usage.total > 0);
    assert!(memory_usage.used >= 0);
    assert!(memory_usage.cache_size >= 0);

    Ok(())
}

#[tokio::test]
async fn test_context_provider_creation() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let context_provider = ContextProvider::new(temp_dir.path().to_path_buf())?;
    // Test that it was created successfully
    assert!(context_provider.repo_root() == temp_dir.path());
    Ok(())
}

#[tokio::test]
async fn test_auth_manager_creation() -> RhemaResult<()> {
    let config = AuthConfig::default();
    let auth_manager = AuthManager::new(&config)?;
    // Test that it was created successfully
    let stats = auth_manager.stats().await;
    assert_eq!(stats.total_requests, 0);
    Ok(())
}

#[tokio::test]
async fn test_mcp_service_creation() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let service = RhemaMcpService::new(config, temp_dir.path().to_path_buf()).await?;

    assert!(!service.is_running().await);
    Ok(())
}

#[tokio::test]
async fn test_mcp_service_health() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let mut service = RhemaMcpService::new(config, temp_dir.path().to_path_buf()).await?;

    // Start the service first
    service.start().await?;

    let health = service.health().await;
    assert_eq!(health.status, "healthy");
    Ok(())
}

#[tokio::test]
async fn test_mcp_daemon_request_counting() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let daemon = Arc::new(McpDaemon::new(config, temp_dir.path().to_path_buf()).await?);

    // Increment request count
    daemon.increment_request_count().await;
    daemon.increment_request_count().await;

    let stats = daemon.get_statistics().await;
    assert_eq!(stats.request_count, 2);

    Ok(())
}

#[tokio::test]
async fn test_mcp_daemon_error_counting() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let config = McpConfig::default();
    let daemon = Arc::new(McpDaemon::new(config, temp_dir.path().to_path_buf()).await?);

    // Increment error count
    daemon.increment_error_count().await;

    let stats = daemon.get_statistics().await;
    assert_eq!(stats.error_count, 1);

    Ok(())
}
