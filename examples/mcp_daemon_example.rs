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

use rhema_mcp::{
    McpConfig, McpDaemon, AuthConfig, WatcherConfig, CacheConfig, LoggingConfig,
    RateLimitConfig, StartupConfig,
};
use std::path::PathBuf;
use tokio::signal;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("Starting Rhema MCP Daemon Example");

    // Create configuration
    let config = create_daemon_config();

    // Get repository root (current directory for this example)
    let repo_root = std::env::current_dir()?;

    // Create and start the daemon
    let mut daemon = McpDaemon::new(config, repo_root).await?;
    
    info!("MCP Daemon created successfully");

    // Start the daemon
    daemon.start().await?;
    
    info!("MCP Daemon started successfully");

    // Wait for shutdown signal
    wait_for_shutdown().await;

    // Stop the daemon gracefully
    info!("Shutting down MCP Daemon...");
    daemon.stop().await?;
    
    info!("MCP Daemon stopped successfully");
    Ok(())
}

/// Create a comprehensive daemon configuration
fn create_daemon_config() -> McpConfig {
    McpConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        unix_socket: Some(PathBuf::from("/tmp/rhema-mcp.sock")),
        redis_url: Some("redis://localhost:6379".to_string()),
        
        auth: AuthConfig {
            enabled: true,
            api_key: Some("your-secret-api-key-here".to_string()),
            jwt_secret: Some("your-jwt-secret-here".to_string()),
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "https://your-app.example.com".to_string(),
            ],
            rate_limiting: RateLimitConfig {
                http_requests_per_minute: 1000,
                websocket_messages_per_minute: 100,
                unix_socket_messages_per_minute: 1000,
            },
        },
        
        watcher: WatcherConfig {
            enabled: true,
            watch_dirs: vec![
                PathBuf::from(".rhema"),
                PathBuf::from("src"),
                PathBuf::from("docs"),
            ],
            file_patterns: vec![
                "*.yaml".to_string(),
                "*.yml".to_string(),
                "*.json".to_string(),
                "*.md".to_string(),
            ],
            debounce_ms: 100,
            recursive: true,
            ignore_hidden: true,
        },
        
        cache: CacheConfig {
            memory_enabled: true,
            redis_enabled: true,
            redis_url: Some("redis://localhost:6379".to_string()),
            ttl_seconds: 3600,
            max_size: 10000,
            compression_enabled: true,
        },
        
        logging: LoggingConfig {
            level: "info".to_string(),
            structured: true,
            file: Some(PathBuf::from("/var/log/rhema-mcp.log")),
            json: true,
        },
        
        use_official_sdk: true,
        
        startup: StartupConfig {
            graceful_shutdown_timeout: 30,
            health_check_interval: 30,
            connection_timeout: 60,
            auto_restart: true,
            max_restart_attempts: 3,
        },
    }
}

/// Wait for shutdown signal (Ctrl+C or SIGTERM)
async fn wait_for_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down...");
        }
        _ = terminate => {
            info!("Received SIGTERM, shutting down...");
        }
    }
}

/// Example client code for interacting with the MCP daemon
#[allow(dead_code)]
async fn example_client_usage() -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::Client;
    use serde_json::json;

    let client = Client::new();
    let base_url = "http://localhost:8080";

    // Health check
    let health_response = client
        .get(&format!("{}/health", base_url))
        .header("Authorization", "Bearer your-secret-api-key-here")
        .send()
        .await?;

    if health_response.status().is_success() {
        let health: serde_json::Value = health_response.json().await?;
        info!("Daemon health: {:?}", health);
    }

    // Execute a query
    let query_response = client
        .post(&format!("{}/query", base_url))
        .header("Authorization", "Bearer your-secret-api-key-here")
        .header("Content-Type", "application/json")
        .json(&json!({
            "query": "SELECT * FROM scopes WHERE type = 'service'",
            "parameters": {
                "type": "service"
            },
            "timeout_ms": 5000
        }))
        .send()
        .await?;

    if query_response.status().is_success() {
        let results: serde_json::Value = query_response.json().await?;
        info!("Query results: {:?}", results);
    }

    // List resources
    let resources_response = client
        .get(&format!("{}/resources", base_url))
        .header("Authorization", "Bearer your-secret-api-key-here")
        .query(&[("uri", "rhema://scopes")])
        .send()
        .await?;

    if resources_response.status().is_success() {
        let resources: serde_json::Value = resources_response.json().await?;
        info!("Resources: {:?}", resources);
    }

    // Get daemon statistics
    let stats_response = client
        .get(&format!("{}/stats", base_url))
        .header("Authorization", "Bearer your-secret-api-key-here")
        .send()
        .await?;

    if stats_response.status().is_success() {
        let stats: serde_json::Value = stats_response.json().await?;
        info!("Daemon statistics: {:?}", stats);
    }

    Ok(())
}

/// Example WebSocket client
#[allow(dead_code)]
async fn example_websocket_client() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
    use url::Url;

    let url = Url::parse("ws://localhost:8080/ws")?;
    let (ws_stream, _) = connect_async(url).await?;

    let (write, read) = ws_stream.split();

    // Send a subscription request
    let subscription_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "resources/subscribe",
        "params": {
            "uri": "rhema://scopes/test"
        }
    });

    // TODO: Implement WebSocket message handling
    info!("WebSocket connection established");

    Ok(())
}

/// Example Unix socket client
#[allow(dead_code)]
async fn example_unix_socket_client() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::net::UnixStream;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let stream = UnixStream::connect("/tmp/rhema-mcp.sock").await?;
    let (mut read, mut write) = stream.into_split();

    // Send a health check request
    let health_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "system/health",
        "params": {}
    });

    let request_bytes = serde_json::to_string(&health_request)? + "\n";
    write.write_all(request_bytes.as_bytes()).await?;

    // Read response
    let mut buffer = [0; 1024];
    let n = read.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    
    info!("Unix socket response: {}", response);

    Ok(())
}

/// Example of monitoring daemon health
#[allow(dead_code)]
async fn monitor_daemon_health() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;

    loop {
        // Get daemon health
        let health = get_daemon_health().await?;
        
        info!("Daemon Health Status:");
        info!("  Status: {}", health.status);
        info!("  Uptime: {} seconds", health.uptime);
        info!("  Connections: {}", health.connections);
        info!("  Cache Hit Rate: {:.2}%", health.cache_hit_rate * 100.0);
        info!("  Request Count: {}", health.request_count);
        info!("  Error Count: {}", health.error_count);
        info!("  Error Rate: {:.2}%", health.error_rate * 100.0);
        info!("  Restart Count: {}", health.restart_count);
        info!("  Memory Usage: {:.2} MB / {:.2} MB", 
              health.memory_usage.used as f64 / 1024.0 / 1024.0,
              health.memory_usage.total as f64 / 1024.0 / 1024.0);

        // Check for issues
        if health.error_rate > 0.1 {
            warn!("High error rate detected: {:.2}%", health.error_rate * 100.0);
        }

        if health.cache_hit_rate < 0.8 {
            warn!("Low cache hit rate: {:.2}%", health.cache_hit_rate * 100.0);
        }

        if health.memory_usage.used as f64 / health.memory_usage.total as f64 > 0.9 {
            warn!("High memory usage: {:.2}%", 
                  health.memory_usage.used as f64 / health.memory_usage.total as f64 * 100.0);
        }

        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

/// Get daemon health via HTTP API
async fn get_daemon_health() -> Result<rhema_mcp::HealthStatus, Box<dyn std::error::Error>> {
    use reqwest::Client;

    let client = Client::new();
    let response = client
        .get("http://localhost:8080/health")
        .header("Authorization", "Bearer your-secret-api-key-here")
        .send()
        .await?;

    if response.status().is_success() {
        let health: rhema_mcp::HealthStatus = response.json().await?;
        Ok(health)
    } else {
        Err(format!("Health check failed: {}", response.status()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_daemon_config_creation() {
        let config = create_daemon_config();
        
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert!(config.auth.enabled);
        assert!(config.watcher.enabled);
        assert!(config.cache.memory_enabled);
        assert!(config.use_official_sdk);
    }

    #[tokio::test]
    async fn test_daemon_creation() {
        let config = create_daemon_config();
        let repo_root = std::env::current_dir().unwrap();
        
        let daemon = McpDaemon::new(config, repo_root).await;
        assert!(daemon.is_ok());
    }
} 