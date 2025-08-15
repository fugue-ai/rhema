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

use crate::coordination::types::{AgentInfo, AgentMessage, ConnectionStats};
use crate::{RhemaError, RhemaResult};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Coordination client interface
#[async_trait::async_trait]
pub trait CoordinationClient: Send + Sync {
    /// Register an agent
    async fn register_agent(&mut self, agent_info: AgentInfo) -> RhemaResult<()>;

    /// Unregister an agent
    async fn unregister_agent(&mut self, agent_id: &str) -> RhemaResult<()>;

    /// Send a message
    async fn send_message(&mut self, message: AgentMessage) -> RhemaResult<()>;

    /// Get agent information
    async fn get_agent_info(&mut self, agent_id: &str) -> RhemaResult<Option<AgentInfo>>;

    /// Get all registered agents
    async fn get_all_agents(&mut self) -> RhemaResult<Vec<AgentInfo>>;

    /// Create a coordination session
    async fn create_session(
        &mut self,
        topic: String,
        participants: Vec<String>,
    ) -> RhemaResult<String>;

    /// Join a coordination session
    async fn join_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()>;

    /// Leave a coordination session
    async fn leave_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()>;

    /// Send a session message
    async fn send_session_message(
        &mut self,
        session_id: &str,
        message: AgentMessage,
    ) -> RhemaResult<()>;

    /// Check if the client is connected
    async fn is_connected(&self) -> bool;

    /// Get connection statistics
    async fn get_connection_stats(&mut self) -> RhemaResult<ConnectionStats>;
}

#[cfg(feature = "coordination")]
/// Real gRPC coordination client implementation
#[derive(Clone)]
pub struct GrpcCoordinationClient {
    client: crate::coordination::SyneidesisClient,
    connection_stats: Arc<RwLock<ConnectionStats>>,
}

#[cfg(feature = "coordination")]
impl GrpcCoordinationClient {
    /// Create a new gRPC coordination client
    pub async fn new(config: &crate::coordination::CoordinationConfig) -> RhemaResult<Self> {
        let grpc_config = syneidesis_grpc::GrpcClientConfig {
            server_addr: config.server_endpoint.clone(),
            request_timeout: config.timeout_seconds,
            connection_timeout: config.timeout_seconds,
            max_retries: config.retry_config.max_retries,
            retry_backoff: config.retry_config.initial_delay_ms / 1000, // Convert to seconds
            enable_retry: true,
            max_message_size: 1024 * 1024, // 1MB
            enable_pooling: true,
            pool_size: 10,
            tls: None, // Simplified for now
        };

        let client = crate::coordination::SyneidesisClient::new(grpc_config)
            .await
            .map_err(|e| {
                RhemaError::CoordinationError(format!("Failed to create gRPC client: {}", e))
            })?;

        Ok(Self {
            client,
            connection_stats: Arc::new(RwLock::new(ConnectionStats {
                is_connected: false,
                uptime_seconds: 0,
                messages_sent: 0,
                messages_received: 0,
                last_heartbeat: None,
                latency_ms: None,
            })),
        })
    }

    /// Convert core AgentInfo to syneidesis AgentInfo
    fn convert_agent_info(&self, agent_info: &AgentInfo) -> syneidesis_grpc::AgentInfo {
        let status = match agent_info.status {
            crate::coordination::types::AgentStatus::Idle => {
                syneidesis_grpc::AgentStatus::Idle as i32
            }
            crate::coordination::types::AgentStatus::Busy => {
                syneidesis_grpc::AgentStatus::Busy as i32
            }
            crate::coordination::types::AgentStatus::Working => {
                syneidesis_grpc::AgentStatus::Working as i32
            }
            crate::coordination::types::AgentStatus::Blocked => {
                syneidesis_grpc::AgentStatus::Blocked as i32
            }
            crate::coordination::types::AgentStatus::Collaborating => {
                syneidesis_grpc::AgentStatus::Collaborating as i32
            }
            crate::coordination::types::AgentStatus::Offline => {
                syneidesis_grpc::AgentStatus::Offline as i32
            }
        };

        let health = if agent_info.is_online {
            syneidesis_grpc::AgentHealth::Healthy as i32
        } else {
            syneidesis_grpc::AgentHealth::Offline as i32
        };

        syneidesis_grpc::AgentInfo {
            id: agent_info.id.clone(),
            name: agent_info.name.clone(),
            agent_type: agent_info.agent_type.clone(),
            status,
            health,
            current_task_id: agent_info.current_task_id.clone(),
            assigned_scope: agent_info.assigned_scope.clone().unwrap_or_default(),
            capabilities: agent_info.capabilities.clone(),
            last_heartbeat: None, // Simplified for now
            is_online: agent_info.is_online,
            performance_metrics: Some(syneidesis_grpc::AgentPerformanceMetrics {
                tasks_completed: 0,
                tasks_failed: 0,
                avg_completion_time_seconds: 0.0,
                success_rate: 1.0,
                collaboration_score: 0.5,
                avg_response_time_ms: 100.0,
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0.0,
                active_connections: 0,
            }),
            priority: 1,
            version: "1.0.0".to_string(),
            endpoint: None,
            metadata: agent_info.metadata.clone(),
            created_at: None,   // Simplified for now
            last_updated: None, // Simplified for now
        }
    }

    /// Convert core AgentMessage to syneidesis AgentMessage
    fn convert_message(&self, message: &AgentMessage) -> syneidesis_grpc::AgentMessage {
        let message_type = match message.message_type {
            crate::coordination::types::MessageType::TaskAssignment => {
                syneidesis_grpc::MessageType::TaskAssignment as i32
            }
            crate::coordination::types::MessageType::TaskCompletion => {
                syneidesis_grpc::MessageType::TaskCompletion as i32
            }
            crate::coordination::types::MessageType::TaskFailure => {
                syneidesis_grpc::MessageType::TaskBlocked as i32
            }
            crate::coordination::types::MessageType::StatusUpdate => {
                syneidesis_grpc::MessageType::StatusUpdate as i32
            }
            crate::coordination::types::MessageType::Heartbeat => {
                syneidesis_grpc::MessageType::Custom as i32
            }
            crate::coordination::types::MessageType::CoordinationRequest => {
                syneidesis_grpc::MessageType::CoordinationRequest as i32
            }
            crate::coordination::types::MessageType::CoordinationResponse => {
                syneidesis_grpc::MessageType::DecisionResponse as i32
            }
            crate::coordination::types::MessageType::ErrorNotification => {
                syneidesis_grpc::MessageType::ConflictNotification as i32
            }
            crate::coordination::types::MessageType::Custom(_) => {
                syneidesis_grpc::MessageType::Custom as i32
            }
        };

        let priority = match message.priority {
            crate::coordination::types::MessagePriority::Low => {
                syneidesis_grpc::MessagePriority::Low as i32
            }
            crate::coordination::types::MessagePriority::Normal => {
                syneidesis_grpc::MessagePriority::Normal as i32
            }
            crate::coordination::types::MessagePriority::High => {
                syneidesis_grpc::MessagePriority::High as i32
            }
            crate::coordination::types::MessagePriority::Critical => {
                syneidesis_grpc::MessagePriority::Critical as i32
            }
        };

        syneidesis_grpc::AgentMessage {
            id: message.id.clone(),
            message_type,
            priority,
            sender_id: message.sender_id.clone(),
            recipient_ids: message.recipient_ids.clone(),
            content: message.content.clone(),
            payload: None,
            timestamp: None, // Simplified for now
            requires_ack: false,
            expires_at: None,
            metadata: message.metadata.clone(),
        }
    }

    /// Convert syneidesis AgentInfo to core AgentInfo
    fn convert_syneidesis_agent_info(&self, agent_info: &syneidesis_grpc::AgentInfo) -> AgentInfo {
        let status = match agent_info.status {
            1 => crate::coordination::types::AgentStatus::Idle,
            2 => crate::coordination::types::AgentStatus::Busy,
            3 => crate::coordination::types::AgentStatus::Working,
            4 => crate::coordination::types::AgentStatus::Blocked,
            5 => crate::coordination::types::AgentStatus::Collaborating,
            6 => crate::coordination::types::AgentStatus::Offline,
            _ => crate::coordination::types::AgentStatus::Idle,
        };

        let last_heartbeat = agent_info.last_heartbeat.as_ref().map(|t| {
            chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32)
                .unwrap_or_else(|| chrono::Utc::now())
        });

        AgentInfo {
            id: agent_info.id.clone(),
            name: agent_info.name.clone(),
            agent_type: agent_info.agent_type.clone(),
            status,
            current_task_id: agent_info.current_task_id.clone(),
            assigned_scope: Some(agent_info.assigned_scope.clone()),
            capabilities: agent_info.capabilities.clone(),
            last_heartbeat,
            is_online: agent_info.is_online,
            performance_metrics: agent_info
                .performance_metrics
                .as_ref()
                .map(|m| crate::coordination::types::AgentPerformanceMetrics {
                    tasks_completed: m.tasks_completed as usize,
                    tasks_failed: m.tasks_failed as usize,
                    avg_completion_time_seconds: m.avg_completion_time_seconds,
                    success_rate: m.success_rate,
                    collaboration_score: m.collaboration_score,
                    avg_response_time_ms: m.avg_response_time_ms,
                })
                .unwrap_or_default(),
            metadata: agent_info.metadata.clone(),
        }
    }

    /// Convert syneidesis AgentMessage to core AgentMessage
    fn convert_syneidesis_message(&self, message: &syneidesis_grpc::AgentMessage) -> AgentMessage {
        let message_type = match message.message_type {
            1 => crate::coordination::types::MessageType::TaskAssignment,
            2 => crate::coordination::types::MessageType::TaskCompletion,
            3 => crate::coordination::types::MessageType::TaskFailure,
            8 => crate::coordination::types::MessageType::StatusUpdate,
            7 => crate::coordination::types::MessageType::CoordinationRequest,
            11 => crate::coordination::types::MessageType::CoordinationResponse,
            6 => crate::coordination::types::MessageType::ErrorNotification,
            12 => crate::coordination::types::MessageType::Custom("custom".to_string()),
            _ => crate::coordination::types::MessageType::Custom("unknown".to_string()),
        };

        let priority = match message.priority {
            1 => crate::coordination::types::MessagePriority::Low,
            2 => crate::coordination::types::MessagePriority::Normal,
            3 => crate::coordination::types::MessagePriority::High,
            4 => crate::coordination::types::MessagePriority::Critical,
            _ => crate::coordination::types::MessagePriority::Normal,
        };

        let timestamp = message
            .timestamp
            .as_ref()
            .map(|t| {
                chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32)
                    .unwrap_or_else(|| chrono::Utc::now())
            })
            .unwrap_or_else(|| chrono::Utc::now());

        AgentMessage {
            id: message.id.clone(),
            message_type,
            priority,
            sender_id: message.sender_id.clone(),
            recipient_ids: message.recipient_ids.clone(),
            content: message.content.clone(),
            timestamp,
            metadata: message.metadata.clone(),
        }
    }

    /// Update connection statistics
    async fn update_stats<F>(&self, f: F)
    where
        F: FnOnce(&mut ConnectionStats),
    {
        let mut stats = self.connection_stats.write().await;
        f(&mut stats);
    }
}

#[cfg(feature = "coordination")]
#[async_trait::async_trait]
impl CoordinationClient for GrpcCoordinationClient {
    async fn register_agent(&mut self, agent_info: AgentInfo) -> RhemaResult<()> {
        let syneidesis_agent_info = self.convert_agent_info(&agent_info);

        self.client
            .register_agent(syneidesis_agent_info)
            .await
            .map_err(|e| {
                RhemaError::CoordinationError(format!("Failed to register agent: {}", e))
            })?;

        self.update_stats(|stats| {
            stats.messages_sent += 1;
            stats.is_connected = true;
        })
        .await;

        Ok(())
    }

    async fn unregister_agent(&mut self, agent_id: &str) -> RhemaResult<()> {
        self.client
            .unregister_agent(agent_id.to_string())
            .await
            .map_err(|e| {
                RhemaError::CoordinationError(format!("Failed to unregister agent: {}", e))
            })?;

        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn send_message(&mut self, message: AgentMessage) -> RhemaResult<()> {
        let syneidesis_message = self.convert_message(&message);

        self.client
            .send_message(syneidesis_message)
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to send message: {}", e)))?;

        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn get_agent_info(&mut self, agent_id: &str) -> RhemaResult<Option<AgentInfo>> {
        let syneidesis_agent_info = self
            .client
            .get_agent_info(agent_id.to_string())
            .await
            .map_err(|e| {
                RhemaError::CoordinationError(format!("Failed to get agent info: {}", e))
            })?;

        Ok(syneidesis_agent_info.map(|info| self.convert_syneidesis_agent_info(&info)))
    }

    async fn get_all_agents(&mut self) -> RhemaResult<Vec<AgentInfo>> {
        let syneidesis_agents = self.client.get_all_agents().await.map_err(|e| {
            RhemaError::CoordinationError(format!("Failed to get all agents: {}", e))
        })?;

        Ok(syneidesis_agents
            .into_iter()
            .map(|agent| self.convert_syneidesis_agent_info(&agent))
            .collect())
    }

    async fn create_session(
        &mut self,
        topic: String,
        participants: Vec<String>,
    ) -> RhemaResult<String> {
        let session_id = self
            .client
            .create_session(topic, participants)
            .await
            .map_err(|e| {
                RhemaError::CoordinationError(format!("Failed to create session: {}", e))
            })?;

        Ok(session_id)
    }

    async fn join_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        self.client
            .join_session(session_id.to_string(), agent_id.to_string())
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to join session: {}", e)))?;

        Ok(())
    }

    async fn leave_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        self.client
            .leave_session(session_id.to_string(), agent_id.to_string())
            .await
            .map_err(|e| {
                RhemaError::CoordinationError(format!("Failed to leave session: {}", e))
            })?;

        Ok(())
    }

    async fn send_session_message(
        &mut self,
        session_id: &str,
        message: AgentMessage,
    ) -> RhemaResult<()> {
        let syneidesis_message = self.convert_message(&message);

        self.client
            .send_session_message(session_id.to_string(), syneidesis_message)
            .await
            .map_err(|e| {
                RhemaError::CoordinationError(format!("Failed to send session message: {}", e))
            })?;

        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        // For now, assume connected if we can make a simple request
        // In a real implementation, this would check the actual connection status
        true
    }

    async fn get_connection_stats(&mut self) -> RhemaResult<ConnectionStats> {
        let stats = self.client.get_stats().await.map_err(|e| {
            RhemaError::CoordinationError(format!("Failed to get connection stats: {}", e))
        })?;

        let mut connection_stats = self.connection_stats.read().await.clone();
        connection_stats.is_connected = true;
        connection_stats.messages_sent = stats.total_messages as u64;
        connection_stats.messages_received = stats.messages_delivered as u64;
        connection_stats.latency_ms = Some(stats.avg_response_time_ms as u64);

        Ok(connection_stats)
    }
}

/// Mock coordination client for testing and when coordination is disabled
pub struct MockCoordinationClient {
    connection_stats: Arc<RwLock<ConnectionStats>>,
}

impl MockCoordinationClient {
    /// Create a new mock coordination client
    pub fn new() -> Self {
        Self {
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

    /// Update connection statistics
    async fn update_stats<F>(&self, f: F)
    where
        F: FnOnce(&mut ConnectionStats),
    {
        let mut stats = self.connection_stats.write().await;
        f(&mut stats);
    }
}

#[async_trait::async_trait]
impl CoordinationClient for MockCoordinationClient {
    async fn register_agent(&mut self, _agent_info: AgentInfo) -> RhemaResult<()> {
        tracing::info!("Mock: Agent registration requested");
        self.update_stats(|stats| {
            stats.messages_sent += 1;
            stats.is_connected = true;
        })
        .await;
        Ok(())
    }

    async fn unregister_agent(&mut self, agent_id: &str) -> RhemaResult<()> {
        tracing::info!("Mock: Agent unregistration requested for {}", agent_id);
        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn send_message(&mut self, message: AgentMessage) -> RhemaResult<()> {
        tracing::info!(
            "Mock: Message sent from {} to {:?}",
            message.sender_id,
            message.recipient_ids
        );
        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn get_agent_info(&mut self, _agent_id: &str) -> RhemaResult<Option<AgentInfo>> {
        tracing::info!("Mock: Agent info requested");
        Ok(None)
    }

    async fn get_all_agents(&mut self) -> RhemaResult<Vec<AgentInfo>> {
        tracing::info!("Mock: All agents requested");
        Ok(Vec::new())
    }

    async fn create_session(
        &mut self,
        topic: String,
        participants: Vec<String>,
    ) -> RhemaResult<String> {
        tracing::info!(
            "Mock: Session creation requested for topic '{}' with participants {:?}",
            topic,
            participants
        );
        Ok("mock-session-id".to_string())
    }

    async fn join_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        tracing::info!(
            "Mock: Join session requested for session {} by agent {}",
            session_id,
            agent_id
        );
        Ok(())
    }

    async fn leave_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        tracing::info!(
            "Mock: Leave session requested for session {} by agent {}",
            session_id,
            agent_id
        );
        Ok(())
    }

    async fn send_session_message(
        &mut self,
        session_id: &str,
        message: AgentMessage,
    ) -> RhemaResult<()> {
        tracing::info!(
            "Mock: Session message sent to session {} from {}",
            session_id,
            message.sender_id
        );
        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        true // Mock is always "connected"
    }

    async fn get_connection_stats(&mut self) -> RhemaResult<ConnectionStats> {
        Ok(self.connection_stats.read().await.clone())
    }
}
