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

pub mod coordination_client;
pub mod coordination_service;
pub mod server;

// Re-export main types
pub use coordination_client::{GrpcClientConfig, GrpcCoordinationClient};
pub use coordination_service::CoordinationService;
pub use server::{GrpcCoordinationServer, GrpcServerConfig};

// Temporarily comment out the protobuf module until we fix the dependencies
/*
pub mod coordination {
    tonic::include_proto!("rhema.coordination.v1");
}
*/

/// Example usage of the gRPC coordination system
pub fn example_usage() {
    println!("gRPC coordination system example:");
    println!("1. Create coordination system");
    println!("2. Start gRPC server");
    println!("3. Create gRPC client");
    println!("4. Register agents");
    println!("5. Send messages");
    println!("6. Create sessions");
}

/// Configuration for the gRPC coordination system
#[derive(Debug, Clone)]
pub struct GrpcCoordinationConfig {
    pub server_config: GrpcServerConfig,
    pub client_config: GrpcClientConfig,
    pub enable_health_checks: bool,
    pub enable_metrics: bool,
    pub enable_logging: bool,
}

impl Default for GrpcCoordinationConfig {
    fn default() -> Self {
        Self {
            server_config: GrpcServerConfig::default(),
            client_config: GrpcClientConfig::default(),
            enable_health_checks: true,
            enable_metrics: true,
            enable_logging: true,
        }
    }
}

/// Example of creating a simple coordination setup
pub async fn create_example_setup() -> Result<(), Box<dyn std::error::Error>> {
    use crate::agent::real_time_coordination::RealTimeCoordinationSystem;

    // Create coordination system
    let coordination_system = RealTimeCoordinationSystem::new();

    // Create server configuration
    let server_config = GrpcServerConfig::default();

    // Create server
    let _server = GrpcCoordinationServer::new(coordination_system, server_config);

    // Start server (for now, just log)
    println!("Would start gRPC coordination server");

    // Create client configuration
    let client_config = GrpcClientConfig::default();

    // Create client
    let _client = GrpcCoordinationClient::new(client_config).await?;

    println!("✅ Example gRPC coordination setup created successfully");

    Ok(())
}
