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

//! Coordination client integration for Rhema Core
//!
//! This module provides integration between Rhema Core and the coordination client,
//! enabling agents to communicate and coordinate their activities.

use crate::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[cfg(feature = "coordination")]
// Re-export syneidesis-grpc types for convenience
pub use syneidesis_grpc::{CoordinationClient as SyneidesisClient, GrpcClientConfig};



// Note: Full coordination client integration requires the syneidesis-grpc crate
// This module provides the core types and interfaces for coordination

/// Configuration for coordination client integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    /// Whether coordination is enabled
    pub enabled: bool,
    /// gRPC server endpoint
    pub server_endpoint: String,
    /// Client timeout in seconds
    pub timeout_seconds: u64,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Health check configuration
    pub health_check_config: HealthCheckConfig,
    /// TLS configuration
    pub tls_config: Option<TlsConfig>,
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_endpoint: "http://localhost:50051".to_string(),
            timeout_seconds: 30,
            retry_config: RetryConfig::default(),
            health_check_config: HealthCheckConfig::default(),
            tls_config: None,
        }
    }
}

/// Retry configuration for coordination operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial backoff delay in milliseconds
    pub initial_delay_ms: u64,
    /// Maximum backoff delay in milliseconds
    pub max_delay_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Whether health checks are enabled
    pub enabled: bool,
    /// Health check interval in seconds
    pub interval_seconds: u64,
    /// Health check timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 30,
            timeout_seconds: 5,
        }
    }
}

/// TLS configuration for secure connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate authority file path
    pub ca_cert_path: String,
    /// Client certificate file path
    pub client_cert_path: String,
    /// Client key file path
    pub client_key_path: String,
}

/// Agent status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Busy,
    Working,
    Blocked,
    Collaborating,
    Offline,
}

/// Agent performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformanceMetrics {
    /// Tasks completed
    pub tasks_completed: usize,
    /// Tasks failed
    pub tasks_failed: usize,
    /// Average task completion time (seconds)
    pub avg_completion_time_seconds: f64,
    /// Success rate (0.0-1.0)
    pub success_rate: f64,
    /// Collaboration score (0.0-1.0)
    pub collaboration_score: f64,
    /// Response time (milliseconds)
    pub avg_response_time_ms: f64,
}

impl Default for AgentPerformanceMetrics {
    fn default() -> Self {
        Self {
            tasks_completed: 0,
            tasks_failed: 0,
            avg_completion_time_seconds: 0.0,
            success_rate: 1.0,
            collaboration_score: 0.5,
            avg_response_time_ms: 100.0,
        }
    }
}

/// Agent information for coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// Unique agent identifier
    pub id: String,
    /// Agent name
    pub name: String,
    /// Agent type
    pub agent_type: String,
    /// Agent status
    pub status: AgentStatus,
    /// Current task ID
    pub current_task_id: Option<String>,
    /// Assigned scope
    pub assigned_scope: Option<String>,
    /// Agent capabilities
    pub capabilities: Vec<String>,
    /// Last heartbeat timestamp
    pub last_heartbeat: Option<DateTime<Utc>>,
    /// Whether agent is online
    pub is_online: bool,
    /// Agent performance metrics
    pub performance_metrics: AgentPerformanceMetrics,
    /// Agent metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl AgentInfo {
    /// Create a new agent info
    pub fn new(name: String, agent_type: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            agent_type,
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: None,
            capabilities: Vec::new(),
            last_heartbeat: None,
            is_online: true,
            performance_metrics: AgentPerformanceMetrics::default(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set the current task ID
    pub fn with_task_id(mut self, task_id: String) -> Self {
        self.current_task_id = Some(task_id);
        self
    }

    /// Set the assigned scope
    pub fn with_scope(mut self, scope: String) -> Self {
        self.assigned_scope = Some(scope);
        self
    }

    /// Add a capability
    pub fn with_capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Agent message for coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Message ID
    pub id: String,
    /// Sender agent ID
    pub sender_id: String,
    /// Recipient agent IDs (empty for broadcast)
    pub recipient_ids: Vec<String>,
    /// Message type
    pub message_type: MessageType,
    /// Message priority
    pub priority: MessagePriority,
    /// Message content
    pub content: String,
    /// Message metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl AgentMessage {
    /// Create a new agent message
    pub fn new(
        sender_id: String,
        message_type: MessageType,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sender_id,
            recipient_ids: Vec::new(),
            message_type,
            priority: MessagePriority::Normal,
            content,
            metadata: std::collections::HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    /// Set recipient IDs
    pub fn to_recipients(mut self, recipient_ids: Vec<String>) -> Self {
        self.recipient_ids = recipient_ids;
        self
    }

    /// Set message priority
    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Message types for coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Task assignment
    TaskAssignment,
    /// Task completion
    TaskCompletion,
    /// Task failure
    TaskFailure,
    /// Status update
    StatusUpdate,
    /// Heartbeat
    Heartbeat,
    /// Coordination request
    CoordinationRequest,
    /// Coordination response
    CoordinationResponse,
    /// Error notification
    ErrorNotification,
    /// Custom message
    Custom(String),
}

/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

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
    async fn create_session(&mut self, topic: String, participants: Vec<String>) -> RhemaResult<String>;
    
    /// Join a coordination session
    async fn join_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()>;
    
    /// Leave a coordination session
    async fn leave_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()>;
    
    /// Send a session message
    async fn send_session_message(&mut self, session_id: &str, message: AgentMessage) -> RhemaResult<()>;
    
    /// Check if the client is connected
    async fn is_connected(&self) -> bool;
    
    /// Get connection statistics
    async fn get_connection_stats(&mut self) -> RhemaResult<ConnectionStats>;
}

/// Connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    /// Whether the connection is active
    pub is_connected: bool,
    /// Connection uptime in seconds
    pub uptime_seconds: u64,
    /// Total messages sent
    pub messages_sent: u64,
    /// Total messages received
    pub messages_received: u64,
    /// Last heartbeat timestamp
    pub last_heartbeat: Option<DateTime<Utc>>,
    /// Connection latency in milliseconds
    pub latency_ms: Option<u64>,
}

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
        tracing::info!("Initializing coordination manager with endpoint: {}", self.config.server_endpoint);
        
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
pub async fn create_coordination_manager(config: CoordinationConfig) -> RhemaResult<CoordinationManager> {
    let mut manager = CoordinationManager::new(config);
    manager.initialize().await?;
    Ok(manager)
}

#[cfg(feature = "coordination")]
/// Real gRPC coordination client implementation
#[derive(Clone)]
pub struct GrpcCoordinationClient {
    client: SyneidesisClient,
    connection_stats: Arc<RwLock<ConnectionStats>>,
}

#[cfg(feature = "coordination")]
impl GrpcCoordinationClient {
    /// Create a new gRPC coordination client
    pub async fn new(config: &CoordinationConfig) -> RhemaResult<Self> {
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

        let client = SyneidesisClient::new(grpc_config)
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to create gRPC client: {}", e)))?;

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
            AgentStatus::Idle => syneidesis_grpc::AgentStatus::Idle as i32,
            AgentStatus::Busy => syneidesis_grpc::AgentStatus::Busy as i32,
            AgentStatus::Working => syneidesis_grpc::AgentStatus::Working as i32,
            AgentStatus::Blocked => syneidesis_grpc::AgentStatus::Blocked as i32,
            AgentStatus::Collaborating => syneidesis_grpc::AgentStatus::Collaborating as i32,
            AgentStatus::Offline => syneidesis_grpc::AgentStatus::Offline as i32,
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
            created_at: None, // Simplified for now
            last_updated: None, // Simplified for now
        }
    }

    /// Convert core AgentMessage to syneidesis AgentMessage
    fn convert_message(&self, message: &AgentMessage) -> syneidesis_grpc::AgentMessage {
        let message_type = match message.message_type {
            MessageType::TaskAssignment => syneidesis_grpc::MessageType::TaskAssignment as i32,
            MessageType::TaskCompletion => syneidesis_grpc::MessageType::TaskCompletion as i32,
            MessageType::TaskFailure => syneidesis_grpc::MessageType::TaskBlocked as i32,
            MessageType::StatusUpdate => syneidesis_grpc::MessageType::StatusUpdate as i32,
            MessageType::Heartbeat => syneidesis_grpc::MessageType::Custom as i32,
            MessageType::CoordinationRequest => syneidesis_grpc::MessageType::CoordinationRequest as i32,
            MessageType::CoordinationResponse => syneidesis_grpc::MessageType::DecisionResponse as i32,
            MessageType::ErrorNotification => syneidesis_grpc::MessageType::ConflictNotification as i32,
            MessageType::Custom(_) => syneidesis_grpc::MessageType::Custom as i32,
        };

        let priority = match message.priority {
            MessagePriority::Low => syneidesis_grpc::MessagePriority::Low as i32,
            MessagePriority::Normal => syneidesis_grpc::MessagePriority::Normal as i32,
            MessagePriority::High => syneidesis_grpc::MessagePriority::High as i32,
            MessagePriority::Critical => syneidesis_grpc::MessagePriority::Critical as i32,
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
            1 => AgentStatus::Idle,
            2 => AgentStatus::Busy,
            3 => AgentStatus::Working,
            4 => AgentStatus::Blocked,
            5 => AgentStatus::Collaborating,
            6 => AgentStatus::Offline,
            _ => AgentStatus::Idle,
        };

        let last_heartbeat = agent_info.last_heartbeat.as_ref().map(|t| {
            chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32).unwrap_or_else(|| chrono::Utc::now())
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
            performance_metrics: agent_info.performance_metrics.as_ref().map(|m| {
                AgentPerformanceMetrics {
                    tasks_completed: m.tasks_completed as usize,
                    tasks_failed: m.tasks_failed as usize,
                    avg_completion_time_seconds: m.avg_completion_time_seconds,
                    success_rate: m.success_rate,
                    collaboration_score: m.collaboration_score,
                    avg_response_time_ms: m.avg_response_time_ms,
                }
            }).unwrap_or_default(),
            metadata: agent_info.metadata.clone(),
        }
    }

    /// Convert syneidesis AgentMessage to core AgentMessage
    fn convert_syneidesis_message(&self, message: &syneidesis_grpc::AgentMessage) -> AgentMessage {
        let message_type = match message.message_type {
            1 => MessageType::TaskAssignment,
            2 => MessageType::TaskCompletion,
            3 => MessageType::TaskFailure,
            8 => MessageType::StatusUpdate,
            7 => MessageType::CoordinationRequest,
            11 => MessageType::CoordinationResponse,
            6 => MessageType::ErrorNotification,
            12 => MessageType::Custom("custom".to_string()),
            _ => MessageType::Custom("unknown".to_string()),
        };

        let priority = match message.priority {
            1 => MessagePriority::Low,
            2 => MessagePriority::Normal,
            3 => MessagePriority::High,
            4 => MessagePriority::Critical,
            _ => MessagePriority::Normal,
        };

        let timestamp = message.timestamp.as_ref().map(|t| {
            chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32).unwrap_or_else(|| chrono::Utc::now())
        }).unwrap_or_else(|| chrono::Utc::now());

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
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to register agent: {}", e)))?;
        
        self.update_stats(|stats| {
            stats.messages_sent += 1;
            stats.is_connected = true;
        }).await;
        
        Ok(())
    }

    async fn unregister_agent(&mut self, agent_id: &str) -> RhemaResult<()> {
        self.client
            .unregister_agent(agent_id.to_string())
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to unregister agent: {}", e)))?;
        
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
        let syneidesis_agent_info = self.client
            .get_agent_info(agent_id.to_string())
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to get agent info: {}", e)))?;
        
        Ok(syneidesis_agent_info.map(|info| self.convert_syneidesis_agent_info(&info)))
    }

    async fn get_all_agents(&mut self) -> RhemaResult<Vec<AgentInfo>> {
        let syneidesis_agents = self.client
            .get_all_agents()
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to get all agents: {}", e)))?;
        
        Ok(syneidesis_agents.into_iter().map(|agent| self.convert_syneidesis_agent_info(&agent)).collect())
    }

    async fn create_session(&mut self, topic: String, participants: Vec<String>) -> RhemaResult<String> {
        let session_id = self.client
            .create_session(topic, participants)
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to create session: {}", e)))?;
        
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
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to leave session: {}", e)))?;
        
        Ok(())
    }

    async fn send_session_message(&mut self, session_id: &str, message: AgentMessage) -> RhemaResult<()> {
        let syneidesis_message = self.convert_message(&message);
        
        self.client
            .send_session_message(session_id.to_string(), syneidesis_message)
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to send session message: {}", e)))?;
        
        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        // For now, assume connected if we can make a simple request
        // In a real implementation, this would check the actual connection status
        true
    }

    async fn get_connection_stats(&mut self) -> RhemaResult<ConnectionStats> {
        let stats = self.client
            .get_stats()
            .await
            .map_err(|e| RhemaError::CoordinationError(format!("Failed to get connection stats: {}", e)))?;
        
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
        }).await;
        Ok(())
    }

    async fn unregister_agent(&mut self, agent_id: &str) -> RhemaResult<()> {
        tracing::info!("Mock: Agent unregistration requested for {}", agent_id);
        self.update_stats(|stats| stats.messages_sent += 1).await;
        Ok(())
    }

    async fn send_message(&mut self, message: AgentMessage) -> RhemaResult<()> {
        tracing::info!("Mock: Message sent from {} to {:?}", message.sender_id, message.recipient_ids);
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

    async fn create_session(&mut self, topic: String, participants: Vec<String>) -> RhemaResult<String> {
        tracing::info!("Mock: Session creation requested for topic '{}' with participants {:?}", topic, participants);
        Ok("mock-session-id".to_string())
    }

    async fn join_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        tracing::info!("Mock: Join session requested for session {} by agent {}", session_id, agent_id);
        Ok(())
    }

    async fn leave_session(&mut self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        tracing::info!("Mock: Leave session requested for session {} by agent {}", session_id, agent_id);
        Ok(())
    }

    async fn send_session_message(&mut self, session_id: &str, message: AgentMessage) -> RhemaResult<()> {
        tracing::info!("Mock: Session message sent to session {} from {}", session_id, message.sender_id);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_coordination_config_default() {
        let config = CoordinationConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.server_endpoint, "http://localhost:50051");
        assert_eq!(config.timeout_seconds, 30);
    }

    #[tokio::test]
    async fn test_agent_info_creation() {
        let agent_info = AgentInfo::new("test-agent".to_string(), "test-type".to_string())
            .with_task_id("task-123".to_string())
            .with_scope("test-scope".to_string())
            .with_capability("test-capability".to_string());

        assert_eq!(agent_info.name, "test-agent");
        assert_eq!(agent_info.agent_type, "test-type");
        assert_eq!(agent_info.current_task_id, Some("task-123".to_string()));
        assert_eq!(agent_info.assigned_scope, Some("test-scope".to_string()));
        assert_eq!(agent_info.capabilities, vec!["test-capability"]);
    }

    #[tokio::test]
    async fn test_agent_message_creation() {
        let message = AgentMessage::new(
            "sender-123".to_string(),
            MessageType::TaskAssignment,
            "Test message content".to_string(),
        )
        .to_recipients(vec!["recipient-1".to_string(), "recipient-2".to_string()])
        .with_priority(MessagePriority::High);

        assert_eq!(message.sender_id, "sender-123");
        assert_eq!(message.recipient_ids, vec!["recipient-1", "recipient-2"]);
        assert!(matches!(message.priority, MessagePriority::High));
        assert_eq!(message.content, "Test message content");
    }

    #[tokio::test]
    async fn test_coordination_manager_creation() {
        let config = CoordinationConfig::default();
        let manager = CoordinationManager::new(config);
        
        assert!(!manager.is_enabled());
        assert_eq!(manager.get_connection_stats().await.is_connected, false);
    }
}
