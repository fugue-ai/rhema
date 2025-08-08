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

//! gRPC server implementation for real-time coordination
//!
//! This module provides the server implementation for the coordination service,
//! including server startup, shutdown, and configuration management.

use std::time::Duration;

use tonic::transport::Server;
use tracing::{error, info, warn};

use crate::types::{CoordinationConfig, GrpcError};
use crate::GrpcConfig;

use super::coordination::real_time_coordination_service_server::RealTimeCoordinationServiceServer;
use super::service::RealTimeCoordinationServiceImpl;

/// gRPC server for coordination service
#[derive(Debug)]
pub struct CoordinationServer {
    /// Server configuration
    config: GrpcConfig,
    /// Coordination service implementation
    service: RealTimeCoordinationServiceImpl,
    /// Server handle for graceful shutdown
    server_handle: Option<tokio::task::JoinHandle<()>>,
}

impl CoordinationServer {
    /// Create a new coordination server
    pub fn new(config: CoordinationConfig) -> Result<Self, GrpcError> {
        let grpc_config = GrpcConfig::default();
        let service = RealTimeCoordinationServiceImpl::new(config)?;

        Ok(Self {
            config: grpc_config,
            service,
            server_handle: None,
        })
    }

    /// Create a new coordination server with custom gRPC configuration
    pub fn with_grpc_config(
        config: CoordinationConfig,
        grpc_config: GrpcConfig,
    ) -> Result<Self, GrpcError> {
        let service = RealTimeCoordinationServiceImpl::new(config)?;

        Ok(Self {
            config: grpc_config,
            service,
            server_handle: None,
        })
    }

    /// Start the coordination server
    pub async fn start(&mut self) -> Result<(), GrpcError> {
        info!("Starting coordination server on {}", self.config.addr);

        // Start the coordination service
        self.service.start().await?;

        // Parse the address
        let addr = self
            .config
            .addr
            .parse()
            .map_err(|e| GrpcError::Configuration {
                message: format!("Invalid server address: {e}"),
            })?;

        // Create the server with custom configuration
        let server = Server::builder()
            .timeout(Duration::from_secs(self.config.connection_timeout))
            .max_concurrent_streams(Some(1000))
            .max_frame_size(Some(self.config.max_message_size as u32))
            .add_service(RealTimeCoordinationServiceServer::new(self.service.clone()));

        // Start the server
        let server_handle = tokio::spawn(async move {
            match server.serve(addr).await {
                Ok(_) => info!("Coordination server stopped gracefully"),
                Err(e) => error!("Coordination server error: {}", e),
            }
        });

        self.server_handle = Some(server_handle);

        info!(
            "Coordination server started successfully on {}",
            self.config.addr
        );
        Ok(())
    }

    /// Stop the coordination server
    pub async fn stop(&mut self) -> Result<(), GrpcError> {
        info!("Stopping coordination server");

        // Stop the coordination service
        self.service.stop().await?;

        // Cancel the server task if it exists
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
            if let Err(e) = handle.await {
                if !e.is_cancelled() {
                    warn!("Server task error during shutdown: {:?}", e);
                }
            }
        }

        info!("Coordination server stopped successfully");
        Ok(())
    }

    /// Get the server configuration
    pub fn config(&self) -> &GrpcConfig {
        &self.config
    }

    /// Get the coordination service implementation
    pub fn service(&self) -> &RealTimeCoordinationServiceImpl {
        &self.service
    }

    /// Get a mutable reference to the coordination service implementation
    pub fn service_mut(&mut self) -> &mut RealTimeCoordinationServiceImpl {
        &mut self.service
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.server_handle.is_some()
    }

    /// Get server statistics
    pub async fn get_statistics(&self) -> Result<crate::types::Statistics, GrpcError> {
        // Get actual statistics from the coordinator
        let coordinator = self.service.coordinator().read().await;
        let stats = coordinator.get_statistics().await;
        Ok(stats)
    }
}

impl Drop for CoordinationServer {
    fn drop(&mut self) {
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
    }
}

/// Builder for creating coordination servers
#[derive(Debug)]
pub struct CoordinationServerBuilder {
    config: Option<CoordinationConfig>,
    grpc_config: Option<GrpcConfig>,
}

impl CoordinationServerBuilder {
    /// Create a new server builder
    pub fn new() -> Self {
        Self {
            config: None,
            grpc_config: None,
        }
    }

    /// Set the coordination configuration
    pub fn with_config(mut self, config: CoordinationConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the gRPC configuration
    pub fn with_grpc_config(mut self, grpc_config: GrpcConfig) -> Self {
        self.grpc_config = Some(grpc_config);
        self
    }

    /// Set the server address
    pub fn with_addr(mut self, addr: String) -> Self {
        let mut grpc_config = self.grpc_config.unwrap_or_default();
        grpc_config.addr = addr;
        self.grpc_config = Some(grpc_config);
        self
    }

    /// Set the maximum message size
    pub fn with_max_message_size(mut self, max_message_size: usize) -> Self {
        let mut grpc_config = self.grpc_config.unwrap_or_default();
        grpc_config.max_message_size = max_message_size;
        self.grpc_config = Some(grpc_config);
        self
    }

    /// Set the connection timeout
    pub fn with_connection_timeout(mut self, timeout: u64) -> Self {
        let mut grpc_config = self.grpc_config.unwrap_or_default();
        grpc_config.connection_timeout = timeout;
        self.grpc_config = Some(grpc_config);
        self
    }

    /// Build the coordination server
    pub fn build(self) -> Result<CoordinationServer, GrpcError> {
        let config = self.config.ok_or_else(|| GrpcError::Configuration {
            message: "Coordination configuration is required".to_string(),
        })?;

        let grpc_config = self.grpc_config.unwrap_or_default();

        CoordinationServer::with_grpc_config(config, grpc_config)
    }
}

impl Default for CoordinationServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to start a coordination server with default configuration
pub async fn start_server() -> Result<CoordinationServer, GrpcError> {
    let config = CoordinationConfig::default();
    let mut server = CoordinationServer::new(config)?;
    server.start().await?;
    Ok(server)
}

/// Utility function to start a coordination server with custom configuration
pub async fn start_server_with_config(
    config: CoordinationConfig,
    grpc_config: Option<GrpcConfig>,
) -> Result<CoordinationServer, GrpcError> {
    let mut server = if let Some(grpc_config) = grpc_config {
        CoordinationServer::with_grpc_config(config, grpc_config)?
    } else {
        CoordinationServer::new(config)?
    };

    server.start().await?;
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CoordinationConfig;
    use crate::types::{AgentHealth, AgentState, AgentStatus};

    #[tokio::test]
    async fn test_server_builder() {
        let config = CoordinationConfig::default();
        let grpc_config = GrpcConfig {
            addr: "127.0.0.1:0".to_string(),
            ..Default::default()
        };

        let server = CoordinationServerBuilder::new()
            .with_config(config)
            .with_grpc_config(grpc_config)
            .build();

        assert!(server.is_ok());
    }

    #[tokio::test]
    async fn test_server_builder_with_addr() {
        let config = CoordinationConfig::default();
        let server = CoordinationServerBuilder::new()
            .with_config(config)
            .with_addr("127.0.0.1:0".to_string())
            .build();

        assert!(server.is_ok());
    }
}
