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

//! # Syneidesis Coordination Library
//!
//! A real-time agent coordination library for the Syneidesis ecosystem.
//! This library provides comprehensive multi-agent coordination capabilities
//! including state management, communication, conflict resolution, and more.
//!
//! ## Features
//!
//! - **Agent State Management**: Track agent health, metrics, and capabilities
//! - **Multi-Agent Coordination**: Register, discover, and coordinate agents
//! - **Real-time Communication**: gRPC-based communication framework
//! - **Conflict Resolution**: Multiple strategies for handling conflicts
//! - **Configuration Management**: Comprehensive configuration system
//!
//! ## Quick Start
//!
//! ```rust
//! use syneidesis_coordination::{
//!     init, CoordinationConfig, CoordinationServer, CoordinationClient
//! };
//! use syneidesis_grpc::AgentInfo;
//! use syneidesis_grpc::AgentStatus;
//! use syneidesis_grpc::AgentHealth;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize coordination library
//!     init().await?;
//!     
//!     // Start coordination server
//!     let config = CoordinationConfig::default();
//!     let mut server = CoordinationServer::new(config)?;
//!     server.start().await?;
//!     
//!     // Connect client
//!     let mut client = CoordinationClient::new_default().await?;
//!     
//!     // Register agents
//!     let agent_info = AgentInfo {
//!         id: "agent-1".to_string(),
//!         name: "Test Agent".to_string(),
//!         agent_type: "verification".to_string(),
//!         status: AgentStatus::Idle as i32,
//!         health: AgentHealth::Healthy as i32,
//!         current_task_id: None,
//!         assigned_scope: "default".to_string(),
//!         capabilities: vec!["verification".to_string()],
//!         last_heartbeat: None,
//!         is_online: true,
//!         performance_metrics: None,
//!         priority: 1,
//!         version: "1.0.0".to_string(),
//!         endpoint: None,
//!         metadata: std::collections::HashMap::new(),
//!         created_at: None,
//!         last_updated: None,
//!     };
//!     
//!     client.register_agent(agent_info).await?;
//!     
//!     println!("✅ Coordination library ready!");
//!     Ok(())
//! }
//! ```

pub mod agent;
pub mod config;
pub mod error;

// Re-export main types for easy access
pub use agent::{
    AgentConfig, AgentCoordinator, AgentEvent, AgentHealth, AgentMetrics, AgentState, AgentStatus,
    Task, TaskStatus,
};
pub use config::{CoordinationConfig, MCPConfig, MetricsConfig};
pub use error::{AgentError, ConflictError, CoordinationError, StateSyncError};

// Re-export gRPC types from the grpc crate
pub use syneidesis_grpc::{
    CoordinationClient, CoordinationServer, GrpcClientConfig, GrpcConfig, DEFAULT_GRPC_ADDR,
    DEFAULT_GRPC_PORT,
};

use std::sync::Once;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

static INIT: Once = Once::new();

/// Initialize the coordination library
///
/// This function sets up logging, configuration, and other global state.
/// It should be called once at the start of your application.
pub async fn init() -> Result<(), CoordinationError> {
    INIT.call_once(|| {
        // Set up logging
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_target(false)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .init();

        info!("Syneidesis Coordination Library initialized");
    });

    Ok(())
}

/// Initialize the coordination library with custom configuration
pub async fn init_with_config(config: CoordinationConfig) -> Result<(), CoordinationError> {
    init().await?;

    // Apply custom configuration
    info!("Applied custom configuration: {:?}", config);

    Ok(())
}

/// Start a coordination server with default configuration
pub async fn start_server() -> Result<CoordinationServer, CoordinationError> {
    let config = CoordinationConfig::default();
    let mut server = CoordinationServer::new(config)?;
    server.start().await?;
    Ok(server)
}

/// Start a coordination server with custom configuration
pub async fn start_server_with_config(
    config: CoordinationConfig,
    grpc_config: Option<GrpcConfig>,
) -> Result<CoordinationServer, CoordinationError> {
    let mut server = if let Some(grpc_config) = grpc_config {
        CoordinationServer::with_grpc_config(config, grpc_config)?
    } else {
        CoordinationServer::new(config)?
    };

    server.start().await?;
    Ok(server)
}

/// Create a coordination client with default configuration
pub async fn create_client(
) -> Result<CoordinationClient, syneidesis_agent::error::CoordinationError> {
    CoordinationClient::new_default().await
}

/// Create a coordination client with custom server address
pub async fn create_client_with_addr(
    server_addr: String,
) -> Result<CoordinationClient, syneidesis_agent::error::CoordinationError> {
    CoordinationClient::new_with_addr(server_addr).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() {
        let result = init().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_init_with_config() {
        let config = CoordinationConfig::default();
        let result = init_with_config(config).await;
        assert!(result.is_ok());
    }
}
