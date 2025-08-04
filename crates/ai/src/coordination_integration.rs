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

use crate::agent::real_time_coordination::{
    AgentInfo, AgentMessage, AgentStatus, MessageType, MessagePriority,
    RealTimeCoordinationSystem
};
use crate::grpc::coordination_client::{SyneidesisCoordinationClient, SyneidesisConfig, ConnectionStatus};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

/// Integration layer between Rhema's coordination system and Syneidesis
pub struct CoordinationIntegration {
    /// Bridge to existing Rhema coordination system
    rhema_coordination: Arc<RwLock<RealTimeCoordinationSystem>>,
    /// Syneidesis coordination client
    syneidesis_client: Option<SyneidesisCoordinationClient>,
    /// Integration configuration
    config: CoordinationConfig,
    /// Integration statistics
    stats: Arc<RwLock<IntegrationStats>>,
}

/// Configuration for coordination integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    /// Whether to run a local coordination server
    pub run_local_server: bool,
    /// Server address (if connecting to remote server)
    pub server_address: Option<String>,
    /// Agent registration settings
    pub auto_register_agents: bool,
    /// Message synchronization settings
    pub sync_messages: bool,
    /// Task synchronization settings
    pub sync_tasks: bool,
    /// Health monitoring settings
    pub enable_health_monitoring: bool,
    /// Syneidesis integration settings
    pub syneidesis: Option<SyneidesisConfig>,
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            run_local_server: true,
            server_address: None,
            auto_register_agents: true,
            sync_messages: true,
            sync_tasks: true,
            enable_health_monitoring: true,
            syneidesis: None,
        }
    }
}

/// Bridge message types for converting between Rhema and Syneidesis formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeMessage {
    /// Rhema agent message
    RhemaMessage(AgentMessage),
    /// Health update
    HealthUpdate(String),
    /// Status update
    StatusUpdate(AgentStatus),
}

impl CoordinationIntegration {
    /// Create a new Coordination integration
    pub async fn new(
        rhema_coordination: RealTimeCoordinationSystem,
        config: Option<CoordinationConfig>,
    ) -> RhemaResult<Self> {
        let config = config.unwrap_or_default();
        
        info!("Initializing Coordination integration with config: {:?}", config);

        // Initialize Syneidesis client if configured
        let syneidesis_client = if let Some(syneidesis_config) = &config.syneidesis {
            if syneidesis_config.enabled {
                match SyneidesisCoordinationClient::new(syneidesis_config.clone()).await {
                    Ok(client) => {
                        info!("✅ Syneidesis coordination client initialized successfully");
                        Some(client)
                    }
                    Err(e) => {
                        error!("Failed to initialize Syneidesis coordination client: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        let integration = Self {
            rhema_coordination: Arc::new(RwLock::new(rhema_coordination)),
            syneidesis_client,
            config,
            stats: Arc::new(RwLock::new(IntegrationStats::default())),
        };

        info!("✅ Coordination integration initialized successfully");
        Ok(integration)
    }

    /// Register a Rhema agent with both Rhema and Syneidesis coordination
    pub async fn register_rhema_agent(&self, rhema_agent: &AgentInfo) -> RhemaResult<()> {
        // Register with Rhema coordination system
        self.rhema_coordination.write().await.register_agent(rhema_agent.clone()).await?;
        info!("✅ Registered Rhema agent '{}' with Rhema coordination", rhema_agent.id);
        
        // Register with Syneidesis if available
        if let Some(syneidesis_client) = &self.syneidesis_client {
            match syneidesis_client.register_agent(rhema_agent.clone()).await {
                Ok(()) => {
                    info!("✅ Registered Rhema agent '{}' with Syneidesis coordination", rhema_agent.id);
                }
                Err(e) => {
                    warn!("Failed to register agent '{}' with Syneidesis: {}", rhema_agent.id, e);
                }
            }
        }
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.rhema_agents += 1;
        if self.syneidesis_client.is_some() {
            stats.syneidesis_agents += 1;
        }
        
        Ok(())
    }

    /// Bridge a Rhema message to Syneidesis
    pub async fn bridge_rhema_message(&self, message: &AgentMessage) -> RhemaResult<()> {
        if !self.config.sync_messages {
            return Ok(());
        }

        // Send message via Rhema coordination system
        self.rhema_coordination.write().await.send_message(message.clone()).await?;
        info!("Bridged Rhema message to Rhema coordination: {:?}", message.message_type);
        
        // Bridge message to Syneidesis if available
        if let Some(syneidesis_client) = &self.syneidesis_client {
            match syneidesis_client.send_message(message.clone()).await {
                Ok(()) => {
                    info!("✅ Bridged Rhema message to Syneidesis: {:?}", message.message_type);
                }
                Err(e) => {
                    warn!("Failed to bridge message to Syneidesis: {}", e);
                }
            }
        }
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.bridge_messages_sent += 1;
        
        Ok(())
    }

    /// Create a coordination session with both systems
    pub async fn create_session(&self, topic: String, participants: Vec<String>) -> RhemaResult<String> {
        // Create session in Rhema coordination system
        let rhema_session_id = self.rhema_coordination.write().await.create_session(
            topic.clone(),
            participants.clone(),
        ).await?;
        
        info!("✅ Created Rhema coordination session: {}", rhema_session_id);
        
        // Create session in Syneidesis if available
        if let Some(syneidesis_client) = &self.syneidesis_client {
            match syneidesis_client.create_session(topic, participants).await {
                Ok(syneidesis_session_id) => {
                    info!("✅ Created Syneidesis coordination session: {}", syneidesis_session_id);
                }
                Err(e) => {
                    warn!("Failed to create Syneidesis session: {}", e);
                }
            }
        }
        
        Ok(rhema_session_id)
    }

    /// Join a coordination session with both systems
    pub async fn join_session(&self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        // Join session in Rhema coordination system
        self.rhema_coordination.write().await.join_session(session_id, agent_id).await?;
        info!("✅ Agent '{}' joined Rhema session: {}", agent_id, session_id);
        
        // Join session in Syneidesis if available
        if let Some(syneidesis_client) = &self.syneidesis_client {
            match syneidesis_client.join_session(session_id, agent_id).await {
                Ok(()) => {
                    info!("✅ Agent '{}' joined Syneidesis session: {}", agent_id, session_id);
                }
                Err(e) => {
                    warn!("Failed to join Syneidesis session: {}", e);
                }
            }
        }
        
        Ok(())
    }

    /// Send a session message with both systems
    pub async fn send_session_message(&self, session_id: &str, message: AgentMessage) -> RhemaResult<()> {
        // Send message in Rhema coordination system
        self.rhema_coordination.write().await.send_session_message(session_id, message.clone()).await?;
        info!("✅ Sent session message to Rhema coordination: {}", message.id);
        
        // Send message in Syneidesis if available
        if let Some(syneidesis_client) = &self.syneidesis_client {
            match syneidesis_client.send_session_message(session_id, message.clone()).await {
                Ok(()) => {
                    info!("✅ Sent session message to Syneidesis: {}", message.id);
                }
                Err(e) => {
                    warn!("Failed to send session message to Syneidesis: {}", e);
                }
            }
        }
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.bridge_messages_sent += 1;
        
        Ok(())
    }

    /// Get integration statistics
    pub async fn get_integration_stats(&self) -> IntegrationStats {
        let rhema_stats = self.rhema_coordination.read().await.get_stats();
        let bridge_stats = self.stats.read().await;
        
        IntegrationStats {
            rhema_agents: rhema_stats.active_agents,
            rhema_messages: rhema_stats.total_messages,
            syneidesis_agents: bridge_stats.syneidesis_agents,
            syneidesis_tasks: 0, // TODO: Implement task counting
            bridge_messages_sent: bridge_stats.bridge_messages_sent,
            bridge_messages_received: bridge_stats.bridge_messages_received,
        }
    }

    /// Get Syneidesis connection status
    pub async fn get_syneidesis_status(&self) -> Option<ConnectionStatus> {
        if let Some(syneidesis_client) = &self.syneidesis_client {
            Some(syneidesis_client.get_connection_status().await)
        } else {
            None
        }
    }

    /// Check if Syneidesis integration is enabled
    pub fn has_syneidesis_integration(&self) -> bool {
        self.syneidesis_client.is_some()
    }

    /// Start health monitoring
    pub async fn start_health_monitoring(&self) -> RhemaResult<()> {
        if !self.config.enable_health_monitoring {
            return Ok(());
        }

        let stats = self.stats.clone();
        let syneidesis_client = self.syneidesis_client.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                
                // Monitor integration health
                let bridge_stats = stats.read().await;
                info!("Coordination integration health check - Messages sent: {}", bridge_stats.bridge_messages_sent);
                
                // Monitor Syneidesis health if available
                if let Some(client) = &syneidesis_client {
                    match client.health_check().await {
                        Ok(()) => {
                            info!("✅ Syneidesis coordination health check passed");
                        }
                        Err(e) => {
                            warn!("❌ Syneidesis coordination health check failed: {}", e);
                        }
                    }
                }
            }
        });

        info!("✅ Started Coordination health monitoring");
        Ok(())
    }

    /// Shutdown the integration
    pub async fn shutdown(&self) -> RhemaResult<()> {
        info!("Shutting down Coordination integration...");
        
        // Shutdown Syneidesis client if available
        if let Some(syneidesis_client) = &self.syneidesis_client {
            if let Err(e) = syneidesis_client.shutdown().await {
                warn!("Error shutting down Syneidesis client: {}", e);
            }
        }
        
        info!("✅ Coordination integration shutdown complete");
        Ok(())
    }
}

/// Statistics for the Coordination integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStats {
    pub rhema_agents: usize,
    pub rhema_messages: usize,
    pub syneidesis_agents: usize,
    pub syneidesis_tasks: usize,
    pub bridge_messages_sent: usize,
    pub bridge_messages_received: usize,
}

impl Default for IntegrationStats {
    fn default() -> Self {
        Self {
            rhema_agents: 0,
            rhema_messages: 0,
            syneidesis_agents: 0,
            syneidesis_tasks: 0,
            bridge_messages_sent: 0,
            bridge_messages_received: 0,
        }
    }
}

impl std::fmt::Display for IntegrationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordination Integration Stats:\n")?;
        write!(f, "  Rhema Agents: {}\n", self.rhema_agents)?;
        write!(f, "  Rhema Messages: {}\n", self.rhema_messages)?;
        write!(f, "  Syneidesis Agents: {}\n", self.syneidesis_agents)?;
        write!(f, "  Syneidesis Tasks: {}\n", self.syneidesis_tasks)?;
        write!(f, "  Bridge Messages Sent: {}\n", self.bridge_messages_sent)?;
        write!(f, "  Bridge Messages Received: {}", self.bridge_messages_received)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::real_time_coordination::{CoordinationConfig as OldCoordinationConfig, RealTimeCoordinationSystem};

    #[tokio::test]
    async fn test_coordination_integration_creation() {
        let rhema_coordination = RealTimeCoordinationSystem::with_config(OldCoordinationConfig::default());
        let config = CoordinationConfig::default();
        
        let integration = CoordinationIntegration::new(rhema_coordination, Some(config)).await;
        assert!(integration.is_ok());
    }

    #[tokio::test]
    async fn test_agent_registration() {
        let rhema_coordination = RealTimeCoordinationSystem::new();
        let integration = CoordinationIntegration::new(rhema_coordination, None).await.unwrap();

        let rhema_agent = AgentInfo {
            id: "test-agent".to_string(),
            name: "Test Agent".to_string(),
            agent_type: "test".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "default".to_string(),
            capabilities: vec!["test".to_string()],
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: crate::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        };

        let result = integration.register_rhema_agent(&rhema_agent).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_syneidesis_integration_creation() {
        let rhema_coordination = RealTimeCoordinationSystem::new();
        let mut config = CoordinationConfig::default();
        config.syneidesis = Some(SyneidesisConfig {
            enabled: true,
            ..Default::default()
        });
        
        let integration = CoordinationIntegration::new(rhema_coordination, Some(config)).await;
        assert!(integration.is_ok());
        
        let integration = integration.unwrap();
        assert!(integration.has_syneidesis_integration());
    }
}