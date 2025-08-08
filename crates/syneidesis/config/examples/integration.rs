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

//! Integration example showing how to use the centralized configuration
//! with other Syneidesis crates
//!
//! This example demonstrates:
//! - Loading configuration for multiple services
//! - Converting between crate-specific and centralized configs
//! - Starting services with centralized configuration

use std::fs;
use std::sync::Arc;
use syneidesis_config::{
    types::{CoordinationConfig, GrpcConfig, HttpConfig, SyneidesisConfig},
    ConfigBuilder,
};
use tempfile::NamedTempFile;
use tokio::sync::RwLock;

// Simulate the HTTP crate's configuration structure
#[derive(Debug, Clone)]
struct HttpServerConfig {
    bind_address: String,
    port: u16,
    max_body_size: usize,
    enable_websocket: bool,
    websocket_port: u16,
}

impl From<&HttpConfig> for HttpServerConfig {
    fn from(config: &HttpConfig) -> Self {
        Self {
            bind_address: config.addr.clone(),
            port: config.port,
            max_body_size: config.max_request_size,
            enable_websocket: config.enable_websocket,
            websocket_port: config.websocket.port,
        }
    }
}

// Simulate the gRPC crate's configuration structure
#[derive(Debug, Clone)]
struct GrpcServerConfig {
    addr: String,
    max_message_size: usize,
    connection_timeout: u64,
    enable_reflection: bool,
    enable_health_checks: bool,
}

impl From<&GrpcConfig> for GrpcServerConfig {
    fn from(config: &GrpcConfig) -> Self {
        Self {
            addr: config.addr.clone(),
            max_message_size: config.max_message_size,
            connection_timeout: config.connection_timeout,
            enable_reflection: config.enable_reflection,
            enable_health_checks: config.enable_health_checks,
        }
    }
}

// Simulate the coordination crate's configuration structure
#[derive(Debug, Clone)]
struct CoordinationServerConfig {
    max_agents: usize,
    heartbeat_interval: u64,
    agent_timeout: u64,
    enable_conflict_resolution: bool,
    enable_metrics: bool,
}

impl From<&CoordinationConfig> for CoordinationServerConfig {
    fn from(config: &CoordinationConfig) -> Self {
        Self {
            max_agents: config.max_agents,
            heartbeat_interval: config.heartbeat_interval,
            agent_timeout: config.agent_timeout,
            enable_conflict_resolution: config.enable_conflict_resolution,
            enable_metrics: config.enable_metrics,
        }
    }
}

// Simulate service implementations
struct HttpServer {
    config: HttpServerConfig,
    running: bool,
}

impl HttpServer {
    fn new(config: HttpServerConfig) -> Self {
        Self {
            config,
            running: false,
        }
    }

    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Starting HTTP server on {}:{}",
            self.config.bind_address, self.config.port
        );
        if self.config.enable_websocket {
            println!(
                "WebSocket server enabled on port {}",
                self.config.websocket_port
            );
        }
        self.running = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping HTTP server");
        self.running = false;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

struct GrpcServer {
    config: GrpcServerConfig,
    running: bool,
}

impl GrpcServer {
    fn new(config: GrpcServerConfig) -> Self {
        Self {
            config,
            running: false,
        }
    }

    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting gRPC server on {}", self.config.addr);
        if self.config.enable_reflection {
            println!("gRPC reflection enabled");
        }
        if self.config.enable_health_checks {
            println!("gRPC health checks enabled");
        }
        self.running = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping gRPC server");
        self.running = false;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

struct CoordinationServer {
    config: CoordinationServerConfig,
    running: bool,
}

impl CoordinationServer {
    fn new(config: CoordinationServerConfig) -> Self {
        Self {
            config,
            running: false,
        }
    }

    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting coordination server");
        println!("Max agents: {}", self.config.max_agents);
        println!("Heartbeat interval: {}s", self.config.heartbeat_interval);
        if self.config.enable_conflict_resolution {
            println!("Conflict resolution enabled");
        }
        if self.config.enable_metrics {
            println!("Metrics collection enabled");
        }
        self.running = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping coordination server");
        self.running = false;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

// Main application that orchestrates all services
struct SyneidesisApplication {
    config: Arc<RwLock<SyneidesisConfig>>,
    http_server: Option<HttpServer>,
    grpc_server: Option<GrpcServer>,
    coordination_server: Option<CoordinationServer>,
}

impl SyneidesisApplication {
    fn new(config: SyneidesisConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            http_server: None,
            grpc_server: None,
            coordination_server: None,
        }
    }

    async fn initialize_services(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.read().await;

        // Initialize HTTP server if configuration is provided
        if let Some(http_config) = &config.http {
            let server_config = HttpServerConfig::from(http_config);
            self.http_server = Some(HttpServer::new(server_config));
            println!("HTTP server initialized");
        }

        // Initialize gRPC server if configuration is provided
        if let Some(grpc_config) = &config.grpc {
            let server_config = GrpcServerConfig::from(grpc_config);
            self.grpc_server = Some(GrpcServer::new(server_config));
            println!("gRPC server initialized");
        }

        // Initialize coordination server if configuration is provided
        if let Some(coordination_config) = &config.coordination {
            let server_config = CoordinationServerConfig::from(coordination_config);
            self.coordination_server = Some(CoordinationServer::new(server_config));
            println!("Coordination server initialized");
        }

        Ok(())
    }

    async fn start_all_services(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Starting all services ===");

        // Start HTTP server
        if let Some(server) = &mut self.http_server {
            server.start().await?;
        }

        // Start gRPC server
        if let Some(server) = &mut self.grpc_server {
            server.start().await?;
        }

        // Start coordination server
        if let Some(server) = &mut self.coordination_server {
            server.start().await?;
        }

        println!("All services started successfully!");
        Ok(())
    }

    async fn stop_all_services(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Stopping all services ===");

        // Stop HTTP server
        if let Some(server) = &mut self.http_server {
            server.stop().await?;
        }

        // Stop gRPC server
        if let Some(server) = &mut self.grpc_server {
            server.stop().await?;
        }

        // Stop coordination server
        if let Some(server) = &mut self.coordination_server {
            server.stop().await?;
        }

        println!("All services stopped successfully!");
        Ok(())
    }

    async fn update_configuration(
        &mut self,
        new_config: SyneidesisConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Updating configuration ===");

        // Update the configuration
        {
            let mut config = self.config.write().await;
            *config = new_config;
        }

        // Reinitialize services with new configuration
        self.initialize_services().await?;

        println!("Configuration updated successfully!");
        Ok(())
    }

    fn get_service_status(&self) -> Vec<(&str, bool)> {
        let mut status = Vec::new();

        if let Some(server) = &self.http_server {
            status.push(("HTTP Server", server.is_running()));
        }

        if let Some(server) = &self.grpc_server {
            status.push(("gRPC Server", server.is_running()));
        }

        if let Some(server) = &self.coordination_server {
            status.push(("Coordination Server", server.is_running()));
        }

        status
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Syneidesis Configuration Integration Example ===");

    // Create a comprehensive configuration file
    let config_content = r#"
system:
  name: "syneidesis-integration-example"
  version: "1.0.0"
  environment: "development"
  debug: true

http:
  addr: "0.0.0.0"
  port: 8080
  max_request_size: 10485760  # 10MB
  enable_cors: true
  enable_websocket: true
  websocket:
    bind_address: "0.0.0.0"
    port: 8081
    max_message_size: 1048576  # 1MB
    max_connections: 1000
    enable_compression: true

grpc:
  addr: "0.0.0.0:50051"
  max_message_size: 10485760  # 10MB
  connection_timeout: 30
  keep_alive_interval: 30
  keep_alive_timeout: 10
  max_concurrent_streams: 1000
  enable_reflection: true
  enable_health_checks: true
  enable_metrics: true
  enable_tracing: false

coordination:
  max_agents: 100
  heartbeat_interval: 30
  agent_timeout: 120
  enable_metrics: true
  enable_conflict_resolution: true
  conflict_resolution_strategy: "priority"
  enable_state_sync: true
  state_sync_interval: 60
  enable_message_persistence: false
  message_retention_period: 86400
  max_message_queue_size: 10000
  enable_message_encryption: false
  enable_message_compression: true

agent:
  max_agents: 50
  heartbeat_interval: 30
  agent_timeout: 120
  registration_timeout: 60
  auto_discovery: true
  discovery_interval: 300
  cleanup_interval: 600
  load_balancing: true
  load_balancing_strategy: "round_robin"
  priority_levels: 10
  failover: true
  failover_timeout: 30

security:
  enable_auth: false
  auth_method: "jwt"
  enable_authorization: false
  enable_encryption: false
  enable_rate_limiting: true
  rate_limit_window: 60
  rate_limit_max_requests: 100

logging:
  level: "info"
  format: "text"
  enable_console: true
  enable_file: false
  enable_structured: false
  enable_json: false
  enable_rotation: true
  rotation_size: 104857600  # 100MB
  rotation_count: 5
  enable_compression: true
  enable_color: true
"#;

    // Create a temporary file with the configuration
    let temp_file = NamedTempFile::new()?;
    fs::write(&temp_file, config_content)?;

    // Load configuration using the centralized config system
    let config_manager = ConfigBuilder::new()
        .with_file(temp_file.path().to_str().unwrap())
        .with_env_prefix("SYNEIDESIS")
        .with_defaults()
        .build()
        .await?;

    let config = config_manager.load().await?;

    println!("Configuration loaded successfully!");
    println!(
        "System: {} v{} ({})",
        config.system.as_ref().unwrap().name,
        config.system.as_ref().unwrap().version,
        config.system.as_ref().unwrap().environment
    );

    // Create and initialize the application
    let mut app = SyneidesisApplication::new(config);
    app.initialize_services().await?;

    // Start all services
    app.start_all_services().await?;

    // Display service status
    println!("\n=== Service Status ===");
    for (service_name, is_running) in app.get_service_status() {
        println!(
            "{}: {}",
            service_name,
            if is_running { "Running" } else { "Stopped" }
        );
    }

    // Simulate running for a bit
    println!("\nSimulating service operation for 2 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Demonstrate configuration update
    println!("\n=== Demonstrating Configuration Update ===");

    let updated_config_content = r#"
system:
  name: "syneidesis-integration-example"
  version: "1.1.0"
  environment: "development"
  debug: false

http:
  addr: "0.0.0.0"
  port: 9090
  max_request_size: 20971520  # 20MB
  enable_cors: true
  enable_websocket: true
  websocket:
    bind_address: "0.0.0.0"
    port: 9091
    max_message_size: 2097152  # 2MB
    max_connections: 2000
    enable_compression: true

grpc:
  addr: "0.0.0.0:50052"
  max_message_size: 20971520  # 20MB
  connection_timeout: 60
  keep_alive_interval: 60
  keep_alive_timeout: 20
  max_concurrent_streams: 2000
  enable_reflection: true
  enable_health_checks: true
  enable_metrics: true
  enable_tracing: true

coordination:
  max_agents: 200
  heartbeat_interval: 60
  agent_timeout: 300
  enable_metrics: true
  enable_conflict_resolution: true
  conflict_resolution_strategy: "timestamp"
  enable_state_sync: true
  state_sync_interval: 120
  enable_message_persistence: true
  message_retention_period: 172800
  max_message_queue_size: 20000
  enable_message_encryption: true
  enable_message_compression: true
"#;

    fs::write(&temp_file, updated_config_content)?;
    let updated_config = config_manager.load().await?;

    app.update_configuration(updated_config).await?;
    app.start_all_services().await?;

    // Display updated service status
    println!("\n=== Updated Service Status ===");
    for (service_name, is_running) in app.get_service_status() {
        println!(
            "{}: {}",
            service_name,
            if is_running { "Running" } else { "Stopped" }
        );
    }

    // Simulate running for a bit more
    println!("\nSimulating service operation for 2 more seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Stop all services
    app.stop_all_services().await?;

    println!("\n=== Integration example completed successfully! ===");
    Ok(())
}
