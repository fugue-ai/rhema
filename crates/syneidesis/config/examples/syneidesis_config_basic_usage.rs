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

//! Basic usage example for the Syneidesis configuration system
//!
//! This example demonstrates how to:
//! - Load configuration from files
//! - Use environment variables
//! - Validate configuration
//! - Access different configuration sections

use std::fs;
use syneidesis_config::{
    types::{GrpcConfig, HttpConfig, SyneidesisConfig, SystemConfig},
    ConfigBuilder,
};
use tempfile::NamedTempFile;
use validator::Validate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Load configuration from a YAML file
    println!("=== Example 1: Loading from YAML file ===");

    let config_content = r#"
system:
  name: "syneidesis-example"
  version: "1.0.0"
  environment: "development"
  debug: true

http:
  addr: "0.0.0.0"
  port: 8080
  enable_cors: true
  enable_websocket: true
  websocket:
    port: 8081
    max_connections: 1000

grpc:
  addr: "0.0.0.0:50051"
  enable_reflection: true
  enable_health_checks: true

coordination:
  max_agents: 100
  heartbeat_interval: 30
  enable_conflict_resolution: true

agent:
  max_agents: 50
  auto_discovery: true
  load_balancing: true
"#;

    // Create a temporary file with the configuration
    let temp_file = NamedTempFile::new()?;
    fs::write(&temp_file, config_content)?;

    let config_manager = ConfigBuilder::new()
        .with_file(temp_file.path().to_str().unwrap())
        .with_env_prefix("SYNEIDESIS")
        .with_defaults()
        .build()
        .await?;

    let config = config_manager.load().await?;

    println!("System name: {}", config.system.as_ref().unwrap().name);
    println!("HTTP port: {}", config.http.as_ref().unwrap().port);
    println!("gRPC address: {}", config.grpc.as_ref().unwrap().addr);
    println!(
        "Max agents: {}",
        config.coordination.as_ref().unwrap().max_agents
    );

    // Example 2: Environment variable overrides
    println!("\n=== Example 2: Environment variable overrides ===");

    // Set environment variables
    std::env::set_var("SYNEIDESIS_HTTP_PORT", "9090");
    std::env::set_var("SYNEIDESIS_SYSTEM_DEBUG", "false");

    let config = config_manager.load().await?;

    println!(
        "HTTP port (from env): {}",
        config.http.as_ref().unwrap().port
    );
    println!(
        "System debug (from env): {}",
        config.system.as_ref().unwrap().debug
    );

    // Example 3: Programmatic configuration
    println!("\n=== Example 3: Programmatic configuration ===");

    let mut config = SyneidesisConfig::default();

    // Set system configuration
    config.system = Some(SystemConfig {
        name: "programmatic-example".to_string(),
        version: "2.0.0".to_string(),
        environment: "production".to_string(),
        debug: false,
        ..Default::default()
    });

    // Set HTTP configuration
    config.http = Some(HttpConfig {
        addr: "127.0.0.1".to_string(),
        port: 3000,
        enable_cors: true,
        enable_websocket: true,
        ..Default::default()
    });

    // Set gRPC configuration
    config.grpc = Some(GrpcConfig {
        addr: "127.0.0.1:50052".to_string(),
        enable_reflection: true,
        enable_health_checks: true,
        ..Default::default()
    });

    println!("System name: {}", config.system.as_ref().unwrap().name);
    println!("HTTP port: {}", config.http.as_ref().unwrap().port);
    println!("gRPC address: {}", config.grpc.as_ref().unwrap().addr);

    // Example 4: Configuration validation
    println!("\n=== Example 4: Configuration validation ===");

    let validation_result = config.validate();
    match validation_result {
        Ok(_) => println!("Configuration is valid!"),
        Err(errors) => {
            println!("Configuration validation failed:");
            for (field, errors) in errors.field_errors() {
                for error in errors {
                    println!(
                        "  - {}: {}",
                        field,
                        error
                            .message
                            .as_ref()
                            .unwrap_or(&std::borrow::Cow::Borrowed("validation error"))
                    );
                }
            }
        }
    }

    // Example 5: Hot reloading (simulated)
    println!("\n=== Example 5: Hot reloading simulation ===");

    let config_manager = ConfigBuilder::new()
        .with_file(temp_file.path().to_str().unwrap())
        .with_env_prefix("SYNEIDESIS")
        .with_defaults()
        .build()
        .await?;

    // Simulate configuration change
    let updated_config_content = r#"
system:
  name: "syneidesis-example-updated"
  version: "1.1.0"
  environment: "development"
  debug: true

http:
  addr: "0.0.0.0"
  port: 8082
  enable_cors: true
  enable_websocket: true
"#;

    fs::write(&temp_file, updated_config_content)?;

    // In a real application, the config manager would automatically reload
    // For this example, we manually reload
    let config = config_manager.load().await?;

    println!(
        "Updated system name: {}",
        config.system.as_ref().unwrap().name
    );
    println!("Updated HTTP port: {}", config.http.as_ref().unwrap().port);

    // Example 6: Configuration serialization
    println!("\n=== Example 6: Configuration serialization ===");

    let yaml = serde_yaml::to_string(&config)?;
    println!("Configuration as YAML:");
    println!("{yaml}");

    let json = serde_json::to_string_pretty(&config)?;
    println!("\nConfiguration as JSON:");
    println!("{json}");

    // Example 7: Accessing nested configuration
    println!("\n=== Example 7: Accessing nested configuration ===");

    if let Some(http_config) = &config.http {
        let websocket_config = &http_config.websocket;
        println!("WebSocket port: {}", websocket_config.port);
        println!(
            "WebSocket max connections: {}",
            websocket_config.max_connections
        );
        println!(
            "WebSocket compression enabled: {}",
            websocket_config.enable_compression
        );
    }

    if let Some(grpc_config) = &config.grpc {
        println!("gRPC reflection enabled: {}", grpc_config.enable_reflection);
        println!(
            "gRPC health checks enabled: {}",
            grpc_config.enable_health_checks
        );
        println!(
            "gRPC max message size: {} bytes",
            grpc_config.max_message_size
        );
    }

    println!("\n=== Configuration system demonstration complete ===");
    Ok(())
}
