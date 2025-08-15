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

use crate::coordination::client::{
    CoordinationClient, GrpcCoordinationClient, MockCoordinationClient,
};
use crate::coordination::config::CoordinationConfig;
use crate::coordination::types::{AgentInfo, AgentMessage, ConnectionStats};
use crate::RhemaResult;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Coordination manager for integrating with the coordination client
pub struct CoordinationManager {
    config: CoordinationConfig,
    client: Option<Box<dyn CoordinationClient>>,
    connection_stats: Arc<RwLock<ConnectionStats>>,
}

impl CoordinationManager {
    /// Create a new coordination manager
    pub fn new(config: CoordinationConfig) -> Self {
        Self {
            config,
            client: None,
            connection_stats: Arc::new(RwLock::new(ConnectionStats {
                is_connected: false,
                uptime_seconds: 0,
                messages_sent: 0,
                messages_received: 0,
                last_heartbeat: None,
                latency_ms: None,
            })),
        }
    }

    /// Initialize the coordination manager
    pub async fn initialize(&mut self) -> RhemaResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Initialize the coordination client
        tracing::info!(
            "Initializing coordination manager with endpoint: {}",
            self.config.server_endpoint
        );

        #[cfg(feature = "coordination")]
        {
            // Use the real gRPC client when coordination feature is enabled
            let grpc_client = GrpcCoordinationClient::new(&self.config).await?;
            self.client = Some(Box::new(grpc_client));
        }

        #[cfg(not(feature = "coordination"))]
        {
            // Use the mock client when coordination feature is not enabled
            let mock_client = MockCoordinationClient::new();
            self.client = Some(Box::new(mock_client));
        }

        Ok(())
    }

    /// Register an agent
    pub async fn register_agent(&mut self, agent_info: AgentInfo) -> RhemaResult<()> {
        if let Some(client) = &mut self.client {
            client.register_agent(agent_info).await?;
            self.update_stats(|stats| stats.messages_sent += 1).await;
        }
        Ok(())
    }

    /// Send a message
    pub async fn send_message(&mut self, message: AgentMessage) -> RhemaResult<()> {
        if let Some(client) = &mut self.client {
            client.send_message(message).await?;
            self.update_stats(|stats| stats.messages_sent += 1).await;
        }
        Ok(())
    }

    /// Get connection statistics
    pub async fn get_connection_stats(&self) -> ConnectionStats {
        self.connection_stats.read().await.clone()
    }

    /// Update connection statistics
    async fn update_stats<F>(&self, f: F)
    where
        F: FnOnce(&mut ConnectionStats),
    {
        let mut stats = self.connection_stats.write().await;
        f(&mut stats);
    }

    /// Check if coordination is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Get the configuration
    pub fn config(&self) -> &CoordinationConfig {
        &self.config
    }
}

/// Create a coordination manager from configuration
pub async fn create_coordination_manager(
    config: CoordinationConfig,
) -> RhemaResult<CoordinationManager> {
    let mut manager = CoordinationManager::new(config);
    manager.initialize().await?;
    Ok(manager)
}
