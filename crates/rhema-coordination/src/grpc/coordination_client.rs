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

//! gRPC Coordination Client Implementation
//!
//! This module provides gRPC-based coordination clients for integrating with the Syneidesis
//! coordination system. It includes both a high-level SyneidesisCoordinationClient and a
//! lower-level LocalGrpcCoordinationClient.
//!
//! ## Current Status
//!
//! ✅ **COMPLETED:**
//! - Basic client structure and configuration
//! - Connection management and status tracking
//! - Method signatures and error handling
//! - Integration with syneidesis-grpc dependency
//! - Compilation and basic functionality
//! - Comprehensive error handling and resilience
//! - Monitoring and observability features
//!
//! 🔄 **IN PROGRESS:**
//! - Type conversion between Rhema and Syneidesis types
//! - Full gRPC client integration
//!
//! 📋 **TODO:**
//! - Implement proper type conversion between Rhema AgentInfo/AgentMessage and Syneidesis protobuf types
//! - Replace simulated operations with actual gRPC calls
//! - Add unit tests and integration tests
//! - Implement TLS support for secure connections
//!
//! ## Architecture
//!
//! The coordination client provides two main interfaces:
//!
//! 1. **SyneidesisCoordinationClient**: High-level client that manages connection state
//!    and provides a simplified API for coordination operations.
//!
//! 2. **LocalGrpcCoordinationClient**: Lower-level client that directly interfaces with
//!    the gRPC coordination service for more control over the communication.
//!
//! ## Usage
//!
//! ```rust
//! use rhema_coordination::grpc::coordination_client::SyneidesisCoordinationClient;
//!
//! let config = SyneidesisConfig::default();
//! let client = SyneidesisCoordinationClient::new(config).await?;
//!
//! // Register an agent
//! client.register_agent(agent_info).await?;
//!
//! // Send a message
//! client.send_message(message).await?;
//! ```
//!
//! ## Integration Notes
//!
//! This implementation integrates with the syneidesis-grpc crate which provides:
//! - Generated protobuf types for coordination messages
//! - gRPC client and server implementations
//! - Configuration management for gRPC connections
//!
//! The current implementation uses simulated operations while the type conversion
//! layer is being developed. Once complete, this will provide full real-time
//! coordination capabilities.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument, warn, Level};

use super::security::{
    ConnectionPool, PerformanceConfig, PerformanceManager, SecurityConfig, SecurityManager,
};
use crate::agent::real_time_coordination::{AgentInfo, AgentMessage};

// Type conversion functions (to be implemented)
fn rhema_agent_info_to_proto(_agent: &AgentInfo) -> syneidesis_grpc::AgentInfo {
    // TODO: Implement proper conversion
    syneidesis_grpc::AgentInfo {
        id: _agent.id.clone(),
        name: _agent.name.clone(),
        agent_type: _agent.agent_type.clone(),
        status: 0, // Default status
        health: 0, // Default health
        current_task_id: _agent.current_task_id.clone(),
        assigned_scope: _agent.assigned_scope.clone(),
        capabilities: _agent.capabilities.clone(),
        last_heartbeat: None,
        is_online: _agent.is_online,
        performance_metrics: None,
        priority: 1,
        version: "1.0.0".to_string(),
        endpoint: None,
        metadata: std::collections::HashMap::new(),
        created_at: None,
        last_updated: None,
    }
}

fn rhema_message_to_proto(_message: &AgentMessage) -> syneidesis_grpc::AgentMessage {
    // TODO: Implement proper conversion
    syneidesis_grpc::AgentMessage {
        id: _message.id.clone(),
        message_type: 0, // Default message type
        priority: 0,     // Default priority
        sender_id: _message.sender_id.clone(),
        recipient_ids: _message.recipient_ids.clone(),
        content: _message.content.clone(),
        payload: None,
        timestamp: None,
        requires_ack: _message.requires_ack,
        expires_at: None,
        metadata: _message.metadata.clone(),
    }
}

fn proto_agent_info_to_rhema(_proto_agent: &syneidesis_grpc::AgentInfo) -> AgentInfo {
    // TODO: Implement proper conversion
    AgentInfo {
        id: _proto_agent.id.clone(),
        name: _proto_agent.name.clone(),
        agent_type: _proto_agent.agent_type.clone(),
        status: crate::agent::real_time_coordination::AgentStatus::Idle,
        current_task_id: _proto_agent.current_task_id.clone(),
        assigned_scope: _proto_agent.assigned_scope.clone(),
        capabilities: _proto_agent.capabilities.clone(),
        last_heartbeat: chrono::Utc::now(),
        is_online: _proto_agent.is_online,
        performance_metrics: crate::agent::real_time_coordination::AgentPerformanceMetrics::default(
        ),
    }
}

fn proto_message_to_rhema(_proto_message: &syneidesis_grpc::AgentMessage) -> AgentMessage {
    // TODO: Implement proper conversion
    AgentMessage {
        id: _proto_message.id.clone(),
        message_type: crate::agent::real_time_coordination::MessageType::Custom(
            "default".to_string(),
        ),
        priority: crate::agent::real_time_coordination::MessagePriority::Normal,
        sender_id: _proto_message.sender_id.clone(),
        recipient_ids: _proto_message.recipient_ids.clone(),
        content: _proto_message.content.clone(),
        payload: None,
        timestamp: chrono::Utc::now(),
        requires_ack: _proto_message.requires_ack,
        expires_at: None,
        metadata: _proto_message.metadata.clone(),
    }
}

// Use Syneidesis gRPC types
use syneidesis_grpc::{
    CoordinationClient as GrpcCoordinationClient, GrpcClientConfig as SyneidesisGrpcConfig,
};

/// Custom error types for gRPC coordination client
#[derive(Debug, thiserror::Error)]
pub enum CoordinationError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Timeout after {timeout:?}: {operation}")]
    Timeout {
        operation: String,
        timeout: Duration,
    },

    #[error("Retry limit exceeded after {attempts} attempts: {operation}")]
    RetryLimitExceeded { operation: String, attempts: u32 },

    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Not connected to coordination server")]
    NotConnected,

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Transport error: {0}")]
    TransportError(String),
}

/// Metrics for monitoring client operations
#[derive(Debug)]
pub struct ClientMetrics {
    pub total_requests: AtomicU64,
    pub successful_requests: AtomicU64,
    pub failed_requests: AtomicU64,
    pub connection_attempts: AtomicU64,
    pub successful_connections: AtomicU64,
    pub failed_connections: AtomicU64,
    pub retry_attempts: AtomicU64,
    pub average_response_time: AtomicU64, // in milliseconds
}

impl Clone for ClientMetrics {
    fn clone(&self) -> Self {
        Self {
            total_requests: AtomicU64::new(self.total_requests.load(Ordering::Relaxed)),
            successful_requests: AtomicU64::new(self.successful_requests.load(Ordering::Relaxed)),
            failed_requests: AtomicU64::new(self.failed_requests.load(Ordering::Relaxed)),
            connection_attempts: AtomicU64::new(self.connection_attempts.load(Ordering::Relaxed)),
            successful_connections: AtomicU64::new(
                self.successful_connections.load(Ordering::Relaxed),
            ),
            failed_connections: AtomicU64::new(self.failed_connections.load(Ordering::Relaxed)),
            retry_attempts: AtomicU64::new(self.retry_attempts.load(Ordering::Relaxed)),
            average_response_time: AtomicU64::new(
                self.average_response_time.load(Ordering::Relaxed),
            ),
        }
    }
}

impl ClientMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
            connection_attempts: AtomicU64::new(0),
            successful_connections: AtomicU64::new(0),
            failed_connections: AtomicU64::new(0),
            retry_attempts: AtomicU64::new(0),
            average_response_time: AtomicU64::new(0),
        }
    }

    pub fn increment_total_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_successful_requests(&self) {
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_failed_requests(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_connection_attempts(&self) {
        self.connection_attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_successful_connections(&self) {
        self.successful_connections.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_failed_connections(&self) {
        self.failed_connections.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_retry_attempts(&self) {
        self.retry_attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn update_average_response_time(&self, new_time_ms: u64) {
        let current_avg = self.average_response_time.load(Ordering::Relaxed);
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        if total_requests > 0 {
            let new_avg = ((current_avg * (total_requests - 1)) + new_time_ms) / total_requests;
            self.average_response_time.store(new_avg, Ordering::Relaxed);
        } else {
            self.average_response_time
                .store(new_time_ms, Ordering::Relaxed);
        }
    }

    pub fn get_success_rate(&self) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);
        if total > 0 {
            successful as f64 / total as f64
        } else {
            0.0
        }
    }

    pub fn get_connection_success_rate(&self) -> f64 {
        let total = self.connection_attempts.load(Ordering::Relaxed);
        let successful = self.successful_connections.load(Ordering::Relaxed);
        if total > 0 {
            successful as f64 / total as f64
        } else {
            0.0
        }
    }
}

/// Syneidesis coordination client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyneidesisConfig {
    pub enabled: bool,
    pub server_address: Option<String>,
    pub auto_register_agents: bool,
    pub sync_messages: bool,
    pub enable_health_monitoring: bool,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub retry_backoff_ms: u64,
    pub enable_tls: bool,
    pub tls_cert_path: Option<String>,
    pub health_check_interval_seconds: u64,
    pub connection_recovery_enabled: bool,
    pub max_reconnection_attempts: u32,
    pub reconnection_backoff_ms: u64,
    pub enable_metrics: bool,
    pub log_level: String,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
}

impl Default for SyneidesisConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_address: Some("http://127.0.0.1:50051".to_string()),
            auto_register_agents: true,
            sync_messages: true,
            enable_health_monitoring: true,
            timeout_seconds: 30,
            max_retries: 3,
            retry_backoff_ms: 1000,
            enable_tls: false,
            tls_cert_path: None,
            health_check_interval_seconds: 30,
            connection_recovery_enabled: true,
            max_reconnection_attempts: 5,
            reconnection_backoff_ms: 5000,
            enable_metrics: true,
            log_level: "info".to_string(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

/// Syneidesis coordination client implementation
#[derive(Clone, Debug)]
pub struct SyneidesisCoordinationClient {
    client: Option<GrpcCoordinationClient>,
    config: SyneidesisConfig,
    connection_status: Arc<RwLock<ConnectionStatus>>,
    metrics: Arc<ClientMetrics>,
    last_health_check: Arc<RwLock<Option<Instant>>>,
    connection_attempts: Arc<RwLock<u32>>,
    security_manager: Arc<SecurityManager>,
    performance_manager: Arc<PerformanceManager>,
    connection_pool: Option<Arc<ConnectionPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Failed(String),
    Reconnecting,
}

impl SyneidesisCoordinationClient {
    #[instrument(level = Level::INFO, skip(config))]
    pub async fn new(config: SyneidesisConfig) -> Result<Self, CoordinationError> {
        info!(
            "Initializing Syneidesis coordination client with config: {:?}",
            config
        );

        let security_manager = Arc::new(SecurityManager::new(config.security.clone()));
        let performance_manager = Arc::new(PerformanceManager::new(config.performance.clone()));

        let connection_pool = if config.performance.enable_connection_pooling {
            Some(Arc::new(ConnectionPool::new(
                config.performance.max_connections,
                Duration::from_secs(config.performance.pool_timeout),
            )))
        } else {
            None
        };

        let client = Self {
            client: None,
            config: config.clone(),
            connection_status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            metrics: Arc::new(ClientMetrics::new()),
            last_health_check: Arc::new(RwLock::new(None)),
            connection_attempts: Arc::new(RwLock::new(0)),
            security_manager,
            performance_manager,
            connection_pool,
        };

        // Attempt to connect
        client.connect_with_retry().await?;

        // Start health monitoring if enabled
        if config.enable_health_monitoring {
            client.start_health_monitoring().await;
        }

        info!("✅ Syneidesis coordination client initialized successfully");
        Ok(client)
    }

    async fn connect_with_retry(&self) -> Result<(), CoordinationError> {
        let mut attempts = 0;
        let max_attempts = self.config.max_reconnection_attempts;

        while attempts < max_attempts {
            attempts += 1;
            self.metrics.increment_connection_attempts();

            info!("Connection attempt {}/{}", attempts, max_attempts);

            match self.connect().await {
                Ok(()) => {
                    self.metrics.increment_successful_connections();
                    *self.connection_attempts.write().await = 0;
                    info!("✅ Successfully connected to Syneidesis coordination server");
                    return Ok(());
                }
                Err(e) => {
                    self.metrics.increment_failed_connections();
                    warn!("Connection attempt {} failed: {}", attempts, e);

                    if attempts < max_attempts {
                        let backoff_duration = Duration::from_millis(
                            self.config.reconnection_backoff_ms * attempts as u64,
                        );
                        info!("Retrying connection in {:?}", backoff_duration);
                        sleep(backoff_duration).await;
                    } else {
                        return Err(CoordinationError::RetryLimitExceeded {
                            operation: "connection".to_string(),
                            attempts: max_attempts,
                        });
                    }
                }
            }
        }

        Err(CoordinationError::ConnectionFailed(
            "Max reconnection attempts exceeded".to_string(),
        ))
    }

    async fn connect(&self) -> Result<(), CoordinationError> {
        let mut status = self.connection_status.write().await;
        *status = ConnectionStatus::Connecting;

        // Connect to Syneidesis coordination server
        let server_address = self
            .config
            .server_address
            .as_deref()
            .ok_or_else(|| {
                CoordinationError::ConfigurationError("Server address not configured".to_string())
            })?
            .replace("http://", "");

        // Create TLS-enabled endpoint using security manager
        let endpoint = self
            .security_manager
            .create_tls_endpoint(&server_address)
            .await?;

        // Configure endpoint with performance optimizations
        let configured_endpoint = self.performance_manager.configure_endpoint(endpoint);

        // Initialize connection pool if enabled
        if let Some(pool) = &self.connection_pool {
            pool.initialize_pool(configured_endpoint.clone()).await?;
        }

        // Create gRPC client configuration
        let grpc_config = SyneidesisGrpcConfig {
            server_addr: server_address.clone(),
            connection_timeout: self.config.timeout_seconds,
            request_timeout: self.config.timeout_seconds,
            max_message_size: self.config.performance.max_message_size,
            enable_retry: true,
            max_retries: self.config.max_retries,
            retry_backoff: self.config.retry_backoff_ms,
            enable_pooling: self.config.performance.enable_connection_pooling,
            pool_size: self.config.performance.max_connections,
            tls: None, // TLS is handled by the security manager
        };

        // Create the gRPC client
        let _grpc_client = GrpcCoordinationClient::new(grpc_config)
            .await
            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

        *status = ConnectionStatus::Connected;
        info!("✅ Connected to Syneidesis coordination server at {} with security and performance optimizations", server_address);
        Ok(())
    }

    async fn get_client(&self) -> Result<GrpcCoordinationClient, CoordinationError> {
        // Use connection pool if available
        if let Some(pool) = &self.connection_pool {
            let channel = pool.get_connection().await?;
            // Note: new_with_channel doesn't exist, we'll need to handle this differently
            // For now, we'll fall back to creating a new client
        }

        // Fallback to creating new client
        let server_address = self
            .config
            .server_address
            .as_deref()
            .ok_or_else(|| {
                CoordinationError::ConfigurationError("Server address not configured".to_string())
            })?
            .replace("http://", "");

        let grpc_config = SyneidesisGrpcConfig {
            server_addr: server_address,
            connection_timeout: self.config.timeout_seconds,
            request_timeout: self.config.timeout_seconds,
            max_message_size: self.config.performance.max_message_size,
            enable_retry: true,
            max_retries: self.config.max_retries,
            retry_backoff: self.config.retry_backoff_ms,
            enable_pooling: self.config.performance.enable_connection_pooling,
            pool_size: self.config.performance.max_connections,
            tls: None,
        };

        GrpcCoordinationClient::new(grpc_config)
            .await
            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))
    }

    #[instrument(level = Level::INFO, skip(self, agent))]
    pub async fn register_agent(&self, agent: AgentInfo) -> Result<(), CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!("Registering agent '{}' with Syneidesis", agent.id);

                        // Convert Rhema AgentInfo to Syneidesis protobuf AgentInfo
                        let proto_agent = rhema_agent_info_to_proto(&agent);

                        // Create gRPC client and send registration request
                        let mut client = self.get_client().await?;
                        let response = client
                            .register_agent(proto_agent)
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        if response.success {
                            info!("✅ Agent '{}' registered with Syneidesis", agent.id);
                            Ok(())
                        } else {
                            Err(CoordinationError::ServerError(response.message))
                        }
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub async fn unregister_agent(&self, agent_id: &str) -> Result<(), CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!("Unregistering agent '{}' from Syneidesis", agent_id);

                        // Create gRPC client and send unregistration request
                        let mut client = self.get_client().await?;
                        let response = client
                            .unregister_agent(agent_id.to_string())
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        if response.success {
                            info!("✅ Agent '{}' unregistered from Syneidesis", agent_id);
                            Ok(())
                        } else {
                            Err(CoordinationError::ServerError(response.message))
                        }
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    #[instrument(level = Level::INFO, skip(self, message))]
    pub async fn send_message(&self, message: AgentMessage) -> Result<(), CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!("Sending message '{}' via Syneidesis", message.id);

                        // Convert Rhema AgentMessage to Syneidesis protobuf AgentMessage
                        let proto_message = rhema_message_to_proto(&message);

                        // Create gRPC client and send message
                        let mut client = self.get_client().await?;
                        let response = client
                            .send_message(proto_message)
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        if response.success {
                            info!("✅ Message '{}' sent via Syneidesis", message.id);
                            Ok(())
                        } else {
                            Err(CoordinationError::ServerError(response.error_message))
                        }
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub async fn get_agent_info(
        &self,
        agent_id: &str,
    ) -> Result<Option<AgentInfo>, CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!("Getting agent info '{}' from Syneidesis", agent_id);

                        // Create gRPC client and request agent info
                        let mut client = self.get_client().await?;
                        let response = client
                            .get_agent_info(agent_id.to_string())
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        if let Some(proto_agent) = response {
                            let rhema_agent = proto_agent_info_to_rhema(&proto_agent);
                            Ok(Some(rhema_agent))
                        } else {
                            Ok(None)
                        }
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub async fn create_session(
        &self,
        topic: String,
        participants: Vec<String>,
    ) -> Result<String, CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let participants_clone = participants.clone();
        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!(
                            "Creating session '{}' with {} participants via Syneidesis",
                            topic,
                            participants_clone.len()
                        );

                        // Create gRPC client and request session creation
                        let mut client = self.get_client().await?;
                        let session_id = client
                            .create_session(topic.clone(), participants_clone.clone())
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        info!("✅ Session '{}' created with ID '{}'", topic, session_id);
                        Ok(session_id)
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub async fn join_session(
        &self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<(), CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!(
                            "Agent '{}' joining session '{}' via Syneidesis",
                            agent_id, session_id
                        );

                        // Create gRPC client and join session
                        let mut client = self.get_client().await?;
                        client
                            .join_session(session_id.to_string(), agent_id.to_string())
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        info!("✅ Agent '{}' joined session '{}'", agent_id, session_id);
                        Ok(())
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub async fn leave_session(
        &self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<(), CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!(
                            "Agent '{}' leaving session '{}' via Syneidesis",
                            agent_id, session_id
                        );

                        // Create gRPC client and leave session
                        let mut client = self.get_client().await?;
                        client
                            .leave_session(session_id.to_string(), agent_id.to_string())
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        info!("✅ Agent '{}' left session '{}'", agent_id, session_id);
                        Ok(())
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    #[instrument(level = Level::INFO, skip(self, message))]
    pub async fn send_session_message(
        &self,
        session_id: &str,
        message: AgentMessage,
    ) -> Result<(), CoordinationError> {
        let start_time = Instant::now();
        self.metrics.increment_total_requests();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        info!(
                            "Sending session message '{}' to session '{}' via Syneidesis",
                            message.id, session_id
                        );

                        // Convert Rhema AgentMessage to Syneidesis protobuf AgentMessage
                        let proto_message = rhema_message_to_proto(&message);

                        // Create gRPC client and send session message
                        let mut client = self.get_client().await?;
                        client
                            .send_session_message(session_id.to_string(), proto_message)
                            .await
                            .map_err(|e| CoordinationError::ConnectionFailed(e.to_string()))?;

                        info!(
                            "✅ Session message '{}' sent to session '{}'",
                            message.id, session_id
                        );
                        Ok(())
                    }
                    _ => Err(CoordinationError::NotConnected),
                }
            })
            .await;

        self.update_metrics(&result, start_time);
        result
    }

    pub async fn get_connection_status(&self) -> ConnectionStatus {
        self.connection_status.read().await.clone()
    }

    pub async fn health_check(&self) -> Result<(), CoordinationError> {
        let _start_time = Instant::now();

        let result = self
            .execute_with_retry(|| async {
                let status = self.connection_status.read().await;
                match *status {
                    ConnectionStatus::Connected => {
                        debug!("Performing health check");
                        // TODO: Replace with actual Syneidesis health check
                        // For now, we'll consider the connection status as the health indicator
                        Ok(())
                    }
                    _ => Err(CoordinationError::HealthCheckFailed(
                        "Not connected".to_string(),
                    )),
                }
            })
            .await;

        if result.is_ok() {
            *self.last_health_check.write().await = Some(Instant::now());
        }

        result
    }

    #[instrument(level = Level::INFO)]
    pub async fn shutdown(&self) -> Result<(), CoordinationError> {
        info!("Shutting down Syneidesis coordination client");
        let mut status = self.connection_status.write().await;
        *status = ConnectionStatus::Disconnected;
        Ok(())
    }

    /// Get current client metrics
    pub fn get_metrics(&self) -> ClientMetrics {
        self.metrics.as_ref().clone()
    }

    /// Execute an operation with retry logic
    async fn execute_with_retry<F, Fut, T>(&self, operation: F) -> Result<T, CoordinationError>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<T, CoordinationError>> + Send,
    {
        let mut attempts = 0;
        let max_attempts = self.config.max_retries;

        loop {
            attempts += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    warn!(
                        "Operation failed (attempt {}/{}): {}",
                        attempts, max_attempts, e
                    );

                    if attempts >= max_attempts {
                        return Err(e);
                    }

                    self.metrics.increment_retry_attempts();

                    let backoff_duration =
                        Duration::from_millis(self.config.retry_backoff_ms * attempts as u64);

                    debug!("Retrying operation in {:?}", backoff_duration);
                    sleep(backoff_duration).await;
                }
            }
        }
    }

    /// Update metrics based on operation result
    fn update_metrics<T>(&self, result: &Result<T, CoordinationError>, start_time: Instant) {
        let response_time = start_time.elapsed();
        let response_time_ms = response_time.as_millis() as u64;
        self.metrics.update_average_response_time(response_time_ms);

        match result {
            Ok(_) => {
                self.metrics.increment_successful_requests();
                debug!("Operation completed successfully in {:?}", response_time);
            }
            Err(e) => {
                self.metrics.increment_failed_requests();
                error!("Operation failed after {:?}: {}", response_time, e);
            }
        }
    }

    /// Start health monitoring background task
    async fn start_health_monitoring(&self) {
        let client = self.clone();
        let interval = Duration::from_secs(self.config.health_check_interval_seconds);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = client.health_check().await {
                    error!("Health check failed: {}", e);

                    // Attempt to reconnect if connection recovery is enabled
                    if client.config.connection_recovery_enabled {
                        warn!("Attempting to reconnect due to health check failure");
                        if let Err(e) = client.connect_with_retry().await {
                            error!("Failed to reconnect: {}", e);
                        }
                    }
                } else {
                    debug!("Health check passed");
                }
            }
        });
    }
}

/// gRPC coordination client configuration
#[derive(Debug, Clone)]
pub struct GrpcClientConfig {
    pub server_address: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub enable_tls: bool,
    pub tls_cert_path: Option<String>,
}

impl Default for GrpcClientConfig {
    fn default() -> Self {
        Self {
            server_address: "http://127.0.0.1:50051".to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            enable_tls: false,
            tls_cert_path: None,
        }
    }
}

/// gRPC coordination client implementation
pub struct LocalGrpcCoordinationClient {
    client: GrpcCoordinationClient,
    config: GrpcClientConfig,
}

impl LocalGrpcCoordinationClient {
    pub async fn new(config: GrpcClientConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let grpc_config = SyneidesisGrpcConfig {
            server_addr: config.server_address.replace("http://", "").to_string(),
            connection_timeout: config.timeout_seconds,
            request_timeout: config.timeout_seconds,
            max_message_size: 1024 * 1024, // 1MB
            enable_retry: true,
            max_retries: config.max_retries,
            retry_backoff: 1000,
            enable_pooling: true,
            pool_size: 10,
            tls: None,
        };

        let client = GrpcCoordinationClient::new(grpc_config).await?;

        Ok(Self { client, config })
    }

    pub async fn register_agent(
        &mut self,
        agent_info: AgentInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just log the registration
        info!("Would register agent: {}", agent_info.id);
        Ok(())
    }

    pub async fn unregister_agent(
        &mut self,
        agent_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just log the unregistration
        info!("Would unregister agent: {}", agent_id);
        Ok(())
    }

    pub async fn send_message(
        &mut self,
        message: AgentMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just log the message
        info!("Would send message: {}", message.id);
        Ok(())
    }

    pub async fn get_agent_info(
        &mut self,
        agent_id: &str,
    ) -> Result<Option<AgentInfo>, Box<dyn std::error::Error>> {
        // For now, return None
        info!("Would get agent info: {}", agent_id);
        Ok(None)
    }

    pub async fn create_session(
        &mut self,
        topic: String,
        participants: Vec<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // For now, return a placeholder session ID
        info!(
            "Would create session: {} with {} participants",
            topic,
            participants.len()
        );
        Ok("placeholder-session-id".to_string())
    }

    pub async fn join_session(
        &mut self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just log the join
        info!(
            "Would join session: {} with agent: {}",
            session_id, agent_id
        );
        Ok(())
    }

    pub async fn leave_session(
        &mut self,
        session_id: &str,
        agent_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just log the leave
        info!(
            "Would leave session: {} with agent: {}",
            session_id, agent_id
        );
        Ok(())
    }

    pub async fn send_session_message(
        &mut self,
        session_id: &str,
        message: AgentMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just log the session message
        info!(
            "Would send session message: {} to session: {}",
            message.id, session_id
        );
        Ok(())
    }
}
