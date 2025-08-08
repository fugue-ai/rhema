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

//! gRPC client implementation for real-time coordination
//!
//! This module provides the client implementation for connecting to the coordination service,
//! including connection management, retry logic, and client utilities.

use std::time::Duration;

use tonic::{transport::Channel, Request};
use tracing::{debug, info};

use crate::GrpcClientConfig;
use syneidesis_agent::error::CoordinationError;

use super::coordination::{
    real_time_coordination_service_client::RealTimeCoordinationServiceClient, AgentHealth,
    AgentInfo, AgentMessage, AgentPerformanceMetrics, AgentStatus, Conflict, ConflictStrategy,
    CoordinationStats, CreateSessionRequest, DetectConflictRequest, GetAgentInfoRequest,
    GetAllAgentsRequest, GetConflictsRequest, GetMessageHistoryRequest, GetStatsRequest,
    HeartbeatRequest, JoinSessionRequest, LeaveSessionRequest, RegisterAgentRequest,
    RegisterAgentResponse, ReleaseResourceRequest, RequestResourceRequest, ResolveConflictRequest,
    SendMessageRequest, SendMessageResponse, SendSessionMessageRequest, UnregisterAgentRequest,
    UnregisterAgentResponse, UpdateAgentStatusRequest, UpdateAgentStatusResponse,
};

/// gRPC client for coordination service
#[derive(Debug, Clone)]
pub struct CoordinationClient {
    /// Client configuration
    config: GrpcClientConfig,
    /// gRPC client
    client: RealTimeCoordinationServiceClient<Channel>,
}

impl CoordinationClient {
    /// Create a new coordination client
    pub async fn new(config: GrpcClientConfig) -> Result<Self, CoordinationError> {
        let channel = Channel::from_shared(format!("http://{}", config.server_addr))
            .map_err(|e| CoordinationError::Configuration {
                message: format!("Invalid server address: {e}"),
            })?
            .timeout(Duration::from_secs(config.request_timeout))
            .connect_timeout(Duration::from_secs(config.connection_timeout))
            .connect()
            .await
            .map_err(|e| CoordinationError::Configuration {
                message: format!("Failed to connect to server: {e}"),
            })?;

        let client = RealTimeCoordinationServiceClient::new(channel);

        Ok(Self { config, client })
    }

    /// Create a new coordination client with default configuration
    pub async fn new_default() -> Result<Self, CoordinationError> {
        Self::new(GrpcClientConfig::default()).await
    }

    /// Create a new coordination client with custom server address
    pub async fn new_with_addr(server_addr: String) -> Result<Self, CoordinationError> {
        let config = GrpcClientConfig {
            server_addr,
            ..Default::default()
        };
        Self::new(config).await
    }

    /// Register an agent
    pub async fn register_agent(
        &mut self,
        agent_info: AgentInfo,
    ) -> Result<RegisterAgentResponse, CoordinationError> {
        debug!("Registering agent: {}", agent_info.id);

        let request = Request::new(RegisterAgentRequest {
            agent_info: Some(agent_info),
        });

        let response = self
            .client
            .clone()
            .register_agent(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to register agent: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to register agent: {}", response.message),
            });
        }

        info!("Agent registered successfully");
        Ok(response)
    }

    /// Unregister an agent
    pub async fn unregister_agent(
        &mut self,
        agent_id: String,
    ) -> Result<UnregisterAgentResponse, CoordinationError> {
        debug!("Unregistering agent: {}", agent_id);

        let request = Request::new(UnregisterAgentRequest { agent_id });

        let response = self
            .client
            .clone()
            .unregister_agent(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to unregister agent: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to unregister agent: {}", response.message),
            });
        }

        info!("Agent unregistered successfully");
        Ok(response)
    }

    /// Update agent status
    pub async fn update_agent_status(
        &mut self,
        agent_id: String,
        status: AgentStatus,
        health: AgentHealth,
        current_task_id: Option<String>,
    ) -> Result<UpdateAgentStatusResponse, CoordinationError> {
        debug!("Updating agent status: {} -> {:?}", agent_id, status);

        let request = Request::new(UpdateAgentStatusRequest {
            agent_id,
            status: status as i32,
            health: health as i32,
            current_task_id,
        });

        let response = self
            .client
            .clone()
            .update_agent_status(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to update agent status: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to update agent status: {}", response.error_message),
            });
        }

        Ok(response)
    }

    /// Get agent information
    pub async fn get_agent_info(
        &mut self,
        agent_id: String,
    ) -> Result<Option<AgentInfo>, CoordinationError> {
        debug!("Getting agent info: {}", agent_id);

        let request = Request::new(GetAgentInfoRequest { agent_id });

        let response = self
            .client
            .clone()
            .get_agent_info(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to get agent info: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::NotFound {
                resource: "Agent: unknown".to_string(),
            });
        }

        Ok(response.agent_info)
    }

    /// Get all agents
    pub async fn get_all_agents(&mut self) -> Result<Vec<AgentInfo>, CoordinationError> {
        debug!("Getting all agents");

        let request = Request::new(GetAllAgentsRequest {});

        let response = self
            .client
            .clone()
            .get_all_agents(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to get all agents: {e}"),
            })?;

        Ok(response.into_inner().agents)
    }

    /// Send a message
    pub async fn send_message(
        &mut self,
        message: AgentMessage,
    ) -> Result<SendMessageResponse, CoordinationError> {
        debug!(
            "Sending message: {} from {} to {:?}",
            message.id, message.sender_id, message.recipient_ids
        );

        let request = Request::new(SendMessageRequest {
            message: Some(message),
        });

        let response = self
            .client
            .clone()
            .send_message(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to send message: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to send message: {}", response.error_message),
            });
        }

        Ok(response)
    }

    /// Get message history
    pub async fn get_message_history(
        &mut self,
        limit: u32,
        agent_id: Option<String>,
    ) -> Result<Vec<AgentMessage>, CoordinationError> {
        debug!(
            "Getting message history, limit: {}, agent: {:?}",
            limit, agent_id
        );

        let request = Request::new(GetMessageHistoryRequest { limit, agent_id });

        let response = self
            .client
            .clone()
            .get_message_history(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to get message history: {e}"),
            })?;

        Ok(response.into_inner().messages)
    }

    /// Create a coordination session
    pub async fn create_session(
        &mut self,
        topic: String,
        participants: Vec<String>,
    ) -> Result<String, CoordinationError> {
        debug!(
            "Creating session: {} with participants: {:?}",
            topic, participants
        );

        let request = Request::new(CreateSessionRequest {
            topic,
            participants,
        });

        let response = self
            .client
            .clone()
            .create_session(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to create session: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to create session: {}", response.error_message),
            });
        }

        Ok(response.session_id)
    }

    /// Join a coordination session
    pub async fn join_session(
        &mut self,
        session_id: String,
        agent_id: String,
    ) -> Result<(), CoordinationError> {
        debug!("Agent {} joining session: {}", agent_id, session_id);

        let request = Request::new(JoinSessionRequest {
            session_id,
            agent_id,
        });

        let response = self
            .client
            .clone()
            .join_session(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to join session: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to join session: {}", response.error_message),
            });
        }

        Ok(())
    }

    /// Leave a coordination session
    pub async fn leave_session(
        &mut self,
        session_id: String,
        agent_id: String,
    ) -> Result<(), CoordinationError> {
        debug!("Agent {} leaving session: {}", agent_id, session_id);

        let request = Request::new(LeaveSessionRequest {
            session_id,
            agent_id,
        });

        let response = self
            .client
            .clone()
            .leave_session(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to leave session: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to leave session: {}", response.error_message),
            });
        }

        Ok(())
    }

    /// Send a session message
    pub async fn send_session_message(
        &mut self,
        session_id: String,
        message: AgentMessage,
    ) -> Result<(), CoordinationError> {
        debug!(
            "Sending session message: {} to session: {}",
            message.id, session_id
        );

        let request = Request::new(SendSessionMessageRequest {
            session_id,
            message: Some(message),
        });

        let response = self
            .client
            .clone()
            .send_session_message(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to send session message: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to send session message: {}", response.error_message),
            });
        }

        Ok(())
    }

    /// Request a resource
    pub async fn request_resource(
        &mut self,
        resource_id: String,
        agent_id: String,
        timeout_seconds: Option<u32>,
    ) -> Result<bool, CoordinationError> {
        debug!("Agent {} requesting resource: {}", agent_id, resource_id);

        let request = Request::new(RequestResourceRequest {
            resource_id,
            agent_id,
            timeout_seconds,
        });

        let response = self
            .client
            .clone()
            .request_resource(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to request resource: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to request resource: {}", response.error_message),
            });
        }

        Ok(response.resource_acquired)
    }

    /// Release a resource
    pub async fn release_resource(
        &mut self,
        resource_id: String,
        agent_id: String,
    ) -> Result<(), CoordinationError> {
        debug!("Agent {} releasing resource: {}", agent_id, resource_id);

        let request = Request::new(ReleaseResourceRequest {
            resource_id,
            agent_id,
        });

        let response = self
            .client
            .clone()
            .release_resource(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to release resource: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to release resource: {}", response.error_message),
            });
        }

        Ok(())
    }

    /// Detect a conflict
    pub async fn detect_conflict(
        &mut self,
        resource_id: String,
        agent_ids: Vec<String>,
        conflict_type: String,
        description: String,
    ) -> Result<Option<Conflict>, CoordinationError> {
        debug!(
            "Detecting conflict: {} for resource: {}",
            conflict_type, resource_id
        );

        let request = Request::new(DetectConflictRequest {
            resource_id,
            agent_ids,
            conflict_type,
            description,
        });

        let response = self
            .client
            .clone()
            .detect_conflict(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to detect conflict: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to detect conflict: {}", response.error_message),
            });
        }

        Ok(response.conflict)
    }

    /// Resolve a conflict
    pub async fn resolve_conflict(
        &mut self,
        conflict_id: String,
        strategy: ConflictStrategy,
        resolution_data: std::collections::HashMap<String, String>,
    ) -> Result<(), CoordinationError> {
        debug!(
            "Resolving conflict: {} with strategy: {:?}",
            conflict_id, strategy
        );

        let request = Request::new(ResolveConflictRequest {
            conflict_id,
            strategy: strategy as i32,
            resolution_data,
        });

        let response = self
            .client
            .clone()
            .resolve_conflict(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to resolve conflict: {e}"),
            })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: format!("Failed to resolve conflict: {}", response.error_message),
            });
        }

        Ok(())
    }

    /// Get conflicts
    pub async fn get_conflicts(
        &mut self,
        resource_id: Option<String>,
        agent_id: Option<String>,
    ) -> Result<Vec<Conflict>, CoordinationError> {
        debug!(
            "Getting conflicts, resource: {:?}, agent: {:?}",
            resource_id, agent_id
        );

        let request = Request::new(GetConflictsRequest {
            resource_id,
            agent_id,
        });

        let response = self
            .client
            .clone()
            .get_conflicts(request)
            .await
            .map_err(|e| CoordinationError::Communication {
                message: format!("Failed to get conflicts: {e}"),
            })?;

        Ok(response.into_inner().conflicts)
    }

    /// Get coordination statistics
    pub async fn get_stats(&mut self) -> Result<CoordinationStats, CoordinationError> {
        debug!("Getting coordination statistics");

        let request = Request::new(GetStatsRequest {});

        let response = self.client.clone().get_stats(request).await.map_err(|e| {
            CoordinationError::Communication {
                message: format!("Failed to get stats: {e}"),
            }
        })?;

        Ok(response
            .into_inner()
            .stats
            .expect("Stats should be present"))
    }

    /// Send heartbeat
    pub async fn heartbeat(
        &mut self,
        agent_id: String,
        status: AgentStatus,
        health: AgentHealth,
        current_task_id: Option<String>,
        metrics: Option<AgentPerformanceMetrics>,
    ) -> Result<Vec<AgentMessage>, CoordinationError> {
        debug!(
            "Sending heartbeat from agent: {} - status: {:?}, health: {:?}",
            agent_id, status, health
        );

        let request = Request::new(HeartbeatRequest {
            agent_id,
            status: status as i32,
            health: health as i32,
            current_task_id,
            metrics,
        });

        let response = self.client.clone().heartbeat(request).await.map_err(|e| {
            CoordinationError::Communication {
                message: format!("Failed to send heartbeat: {e}"),
            }
        })?;

        let response = response.into_inner();
        if !response.success {
            return Err(CoordinationError::Communication {
                message: "Failed to send heartbeat".to_string(),
            });
        }

        Ok(response.pending_messages)
    }

    /// Get client configuration
    pub fn config(&self) -> &GrpcClientConfig {
        &self.config
    }
}

/// Builder for creating coordination clients
#[derive(Debug)]
pub struct CoordinationClientBuilder {
    config: GrpcClientConfig,
}

impl CoordinationClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            config: GrpcClientConfig::default(),
        }
    }

    /// Set the server address
    pub fn with_server_addr(mut self, server_addr: String) -> Self {
        self.config.server_addr = server_addr;
        self
    }

    /// Set the connection timeout
    pub fn with_connection_timeout(mut self, timeout: u64) -> Self {
        self.config.connection_timeout = timeout;
        self
    }

    /// Set the request timeout
    pub fn with_request_timeout(mut self, timeout: u64) -> Self {
        self.config.request_timeout = timeout;
        self
    }

    /// Set the maximum message size
    pub fn with_max_message_size(mut self, max_message_size: usize) -> Self {
        self.config.max_message_size = max_message_size;
        self
    }

    /// Build the coordination client
    pub async fn build(self) -> Result<CoordinationClient, CoordinationError> {
        CoordinationClient::new(self.config).await
    }
}

impl Default for CoordinationClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_builder() {
        let client = CoordinationClientBuilder::new()
            .with_server_addr("127.0.0.1:50051".to_string())
            .with_connection_timeout(30)
            .with_request_timeout(60)
            .build()
            .await;

        // This will fail because there's no server running, but we can test the builder
        assert!(client.is_err());
    }
}
