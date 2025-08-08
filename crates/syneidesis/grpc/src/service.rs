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

//! gRPC service implementation for real-time coordination
//!
//! This module provides the implementation of the RealTimeCoordinationService
//! that handles agent registration, message passing, session management,
//! resource management, and conflict resolution.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use tracing::{debug, error, info, warn};

use crate::types::{
    AgentCoordinator, AgentHealth, AgentMetrics, AgentState, AgentStatus, CoordinationConfig,
    GrpcError,
};

use super::coordination::{
    real_time_coordination_service_server::RealTimeCoordinationService, AgentInfo, AgentMessage,
    AgentPerformanceMetrics, Conflict, ConflictStrategy, CoordinationSession, CoordinationStats,
    CreateSessionRequest, CreateSessionResponse, DetectConflictRequest, DetectConflictResponse,
    GetAgentInfoRequest, GetAgentInfoResponse, GetAllAgentsRequest, GetAllAgentsResponse,
    GetConflictsRequest, GetConflictsResponse, GetMessageHistoryRequest, GetMessageHistoryResponse,
    GetMessageStreamRequest, GetStatsRequest, GetStatsResponse, HeartbeatRequest,
    HeartbeatResponse, JoinSessionRequest, JoinSessionResponse, LeaveSessionRequest,
    LeaveSessionResponse, RegisterAgentRequest, RegisterAgentResponse, ReleaseResourceRequest,
    ReleaseResourceResponse, RequestResourceRequest, RequestResourceResponse,
    ResolveConflictRequest, ResolveConflictResponse, SendMessageRequest, SendMessageResponse,
    SendSessionMessageRequest, SendSessionMessageResponse, SessionStatus, UnregisterAgentRequest,
    UnregisterAgentResponse, UpdateAgentStatusRequest, UpdateAgentStatusResponse,
};

/// Resource manager for handling resource allocation and management
#[derive(Debug)]
pub struct ResourceManager {
    /// Available resources
    resources: Arc<RwLock<HashMap<String, Resource>>>,
    /// Resource locks
    locks: Arc<RwLock<HashMap<String, ResourceLock>>>,
}

/// Resource representation
#[derive(Debug, Clone)]
pub struct Resource {
    /// Resource ID
    pub id: String,
    /// Resource type
    pub resource_type: String,
    /// Resource capacity
    pub capacity: u32,
    /// Current usage
    pub current_usage: u32,
    /// Resource metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Resource lock for tracking resource usage
#[derive(Debug, Clone)]
pub struct ResourceLock {
    /// Resource ID
    pub resource_id: String,
    /// Agent ID that holds the lock
    pub agent_id: String,
    /// Lock timestamp
    pub locked_at: DateTime<Utc>,
    /// Lock timeout
    pub timeout: Duration,
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
            locks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Request a resource
    pub async fn request_resource(
        &self,
        resource_id: &str,
        agent_id: &str,
        timeout_seconds: u32,
    ) -> Result<bool, String> {
        let mut resources = self.resources.write().await;
        let mut locks = self.locks.write().await;

        // Check if resource exists
        if let Some(resource) = resources.get_mut(resource_id) {
            // Check if resource has available capacity
            if resource.current_usage < resource.capacity {
                // Check if resource is already locked by this agent
                if let Some(lock) = locks.get(resource_id) {
                    if lock.agent_id == agent_id {
                        return Ok(true); // Agent already has the resource
                    }
                }

                // Try to acquire the resource
                resource.current_usage += 1;
                locks.insert(
                    resource_id.to_string(),
                    ResourceLock {
                        resource_id: resource_id.to_string(),
                        agent_id: agent_id.to_string(),
                        locked_at: Utc::now(),
                        timeout: Duration::from_secs(timeout_seconds as u64),
                    },
                );

                Ok(true)
            } else {
                Ok(false) // Resource is at capacity
            }
        } else {
            Err(format!("Resource {resource_id} not found"))
        }
    }

    /// Release a resource
    pub async fn release_resource(&self, resource_id: &str, agent_id: &str) -> Result<(), String> {
        let mut resources = self.resources.write().await;
        let mut locks = self.locks.write().await;

        // Check if resource exists
        if let Some(resource) = resources.get_mut(resource_id) {
            // Check if agent holds the lock
            if let Some(lock) = locks.get(resource_id) {
                if lock.agent_id == agent_id {
                    // Release the resource
                    resource.current_usage = resource.current_usage.saturating_sub(1);
                    locks.remove(resource_id);
                    Ok(())
                } else {
                    Err(format!(
                        "Agent {agent_id} does not hold lock on resource {resource_id}"
                    ))
                }
            } else {
                Err(format!("Resource {resource_id} is not locked"))
            }
        } else {
            Err(format!("Resource {resource_id} not found"))
        }
    }

    /// Add a resource
    pub async fn add_resource(&self, resource: Resource) {
        let mut resources = self.resources.write().await;
        resources.insert(resource.id.clone(), resource);
    }

    /// Remove a resource
    pub async fn remove_resource(&self, resource_id: &str) -> Result<(), String> {
        let mut resources = self.resources.write().await;
        let mut locks = self.locks.write().await;

        if resources.remove(resource_id).is_some() {
            locks.remove(resource_id);
            Ok(())
        } else {
            Err(format!("Resource {resource_id} not found"))
        }
    }

    /// Get resource status
    pub async fn get_resource_status(&self, resource_id: &str) -> Option<Resource> {
        let resources = self.resources.read().await;
        resources.get(resource_id).cloned()
    }
}

/// Message tracker for delivery and performance metrics
#[derive(Debug)]
pub struct MessageTracker {
    /// Message delivery records
    deliveries: Arc<RwLock<HashMap<String, MessageDeliveryRecord>>>,
    /// Message statistics
    statistics: Arc<RwLock<MessageStatistics>>,
}

/// Message delivery record
#[derive(Debug, Clone)]
pub struct MessageDeliveryRecord {
    /// Message ID
    pub message_id: String,
    /// Sender ID
    pub sender_id: String,
    /// Recipient ID
    pub recipient_id: String,
    /// Message sent timestamp
    pub sent_at: DateTime<Utc>,
    /// Message delivered timestamp
    pub delivered_at: Option<DateTime<Utc>>,
    /// Message failed timestamp
    pub failed_at: Option<DateTime<Utc>>,
    /// Response received timestamp
    pub response_at: Option<DateTime<Utc>>,
    /// Delivery status
    pub status: MessageDeliveryStatus,
    /// Error message if failed
    pub error_message: Option<String>,
}

/// Message delivery status
#[derive(Debug, Clone, PartialEq)]
pub enum MessageDeliveryStatus {
    /// Message sent
    Sent,
    /// Message delivered
    Delivered,
    /// Message failed
    Failed,
    /// Response received
    Responded,
}

/// Message statistics
#[derive(Debug, Clone)]
pub struct MessageStatistics {
    /// Total messages sent
    pub total_sent: u32,
    /// Messages delivered
    pub delivered: u32,
    /// Messages failed
    pub failed: u32,
    /// Messages responded to
    pub responded: u32,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Coordination efficiency (delivered / sent)
    pub efficiency: f64,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for MessageStatistics {
    fn default() -> Self {
        Self {
            total_sent: 0,
            delivered: 0,
            failed: 0,
            responded: 0,
            avg_response_time_ms: 0.0,
            efficiency: 0.0,
            last_updated: Utc::now(),
        }
    }
}

impl Default for MessageTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageTracker {
    /// Create a new message tracker
    pub fn new() -> Self {
        Self {
            deliveries: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(MessageStatistics::default())),
        }
    }

    /// Record message sent
    pub async fn record_message_sent(&self, message_id: &str, sender_id: &str, recipient_id: &str) {
        let record = MessageDeliveryRecord {
            message_id: message_id.to_string(),
            sender_id: sender_id.to_string(),
            recipient_id: recipient_id.to_string(),
            sent_at: Utc::now(),
            delivered_at: None,
            failed_at: None,
            response_at: None,
            status: MessageDeliveryStatus::Sent,
            error_message: None,
        };

        self.deliveries
            .write()
            .await
            .insert(message_id.to_string(), record);
        self.update_statistics().await;
    }

    /// Record message delivered
    pub async fn record_message_delivered(&self, message_id: &str) {
        if let Some(record) = self.deliveries.write().await.get_mut(message_id) {
            record.delivered_at = Some(Utc::now());
            record.status = MessageDeliveryStatus::Delivered;
        }
        self.update_statistics().await;
    }

    /// Record message failed
    pub async fn record_message_failed(&self, message_id: &str, error: &str) {
        if let Some(record) = self.deliveries.write().await.get_mut(message_id) {
            record.failed_at = Some(Utc::now());
            record.status = MessageDeliveryStatus::Failed;
            record.error_message = Some(error.to_string());
        }
        self.update_statistics().await;
    }

    /// Record response received
    pub async fn record_response_received(&self, message_id: &str) {
        if let Some(record) = self.deliveries.write().await.get_mut(message_id) {
            record.response_at = Some(Utc::now());
            record.status = MessageDeliveryStatus::Responded;
        }
        self.update_statistics().await;
    }

    /// Get message statistics
    pub async fn get_statistics(&self) -> MessageStatistics {
        self.statistics.read().await.clone()
    }

    /// Update statistics
    async fn update_statistics(&self) {
        let deliveries = self.deliveries.read().await;
        let mut stats = self.statistics.write().await;

        stats.total_sent = deliveries.len() as u32;
        stats.delivered = deliveries
            .values()
            .filter(|r| {
                r.status == MessageDeliveryStatus::Delivered
                    || r.status == MessageDeliveryStatus::Responded
            })
            .count() as u32;
        stats.failed = deliveries
            .values()
            .filter(|r| r.status == MessageDeliveryStatus::Failed)
            .count() as u32;
        stats.responded = deliveries
            .values()
            .filter(|r| r.status == MessageDeliveryStatus::Responded)
            .count() as u32;

        // Calculate average response time
        let response_times: Vec<f64> = deliveries
            .values()
            .filter_map(|r| {
                r.response_at.map(|response| {
                    response.signed_duration_since(r.sent_at).num_milliseconds() as f64
                })
            })
            .collect();

        if !response_times.is_empty() {
            stats.avg_response_time_ms =
                response_times.iter().sum::<f64>() / response_times.len() as f64;
        }

        // Calculate efficiency
        if stats.total_sent > 0 {
            stats.efficiency = stats.delivered as f64 / stats.total_sent as f64;
        }

        stats.last_updated = Utc::now();
    }
}

/// Implementation of the RealTimeCoordinationService
#[derive(Debug, Clone)]
pub struct RealTimeCoordinationServiceImpl {
    /// Agent coordinator for managing agent state and operations
    coordinator: Arc<RwLock<AgentCoordinator>>,
    /// Configuration for the coordination service
    config: CoordinationConfig,
    /// Active sessions
    sessions: Arc<DashMap<String, CoordinationSession>>,
    /// Message history
    message_history: Arc<RwLock<Vec<AgentMessage>>>,
    /// Conflict tracking
    conflicts: Arc<DashMap<String, Conflict>>,
    /// Event sender for real-time updates
    event_sender: Option<mpsc::UnboundedSender<AgentMessage>>,
    /// Agent-specific message streams
    agent_message_streams:
        Arc<DashMap<String, mpsc::UnboundedSender<Result<AgentMessage, Status>>>>,
    /// Resource manager for handling resource allocation
    resource_manager: Arc<ResourceManager>,
    /// Message tracker for delivery and performance metrics
    message_tracker: Arc<MessageTracker>,
}

impl RealTimeCoordinationServiceImpl {
    /// Create a new coordination service implementation
    pub fn new(config: CoordinationConfig) -> Result<Self, GrpcError> {
        let coordinator = Arc::new(RwLock::new(AgentCoordinator::new()));
        let event_sender = None; // Will be set when streaming is implemented
        let resource_manager = Arc::new(ResourceManager::new());
        let message_tracker = Arc::new(MessageTracker::new());

        Ok(Self {
            coordinator,
            config,
            sessions: Arc::new(DashMap::new()),
            message_history: Arc::new(RwLock::new(Vec::new())),
            conflicts: Arc::new(DashMap::new()),
            event_sender,
            agent_message_streams: Arc::new(DashMap::new()),
            resource_manager,
            message_tracker,
        })
    }

    /// Start the coordination service
    pub async fn start(&mut self) -> Result<(), GrpcError> {
        info!("Starting coordination service");
        // No specific start logic needed for simplified coordinator
        info!("Coordination service started successfully");
        Ok(())
    }

    /// Stop the coordination service
    pub async fn stop(&self) -> Result<(), GrpcError> {
        info!("Stopping coordination service");
        // Cleanup operations
        Ok(())
    }

    /// Convert internal AgentState to protobuf AgentInfo
    fn agent_state_to_info(&self, agent: &AgentState) -> AgentInfo {
        // Convert metadata from serde_json::Value to String
        let mut string_metadata = std::collections::HashMap::new();
        for (key, value) in &agent.metadata {
            string_metadata.insert(key.clone(), value.to_string());
        }

        AgentInfo {
            id: agent.id.as_str().to_string(),
            name: agent.name.clone(),
            agent_type: agent.agent_type.clone(),
            status: self.status_to_proto(agent.status),
            health: self.health_to_proto(agent.health),
            current_task_id: agent
                .current_task
                .as_ref()
                .map(|id| id.as_str().to_string()),
            assigned_scope: "default".to_string(), // Not available in AgentState
            capabilities: agent.capabilities.clone(),
            last_heartbeat: agent.last_heartbeat.map(|t| SystemTime::from(t).into()),
            is_online: agent.is_operational(),
            performance_metrics: Some(self.metrics_to_proto(&agent.metrics)),
            priority: agent.priority as u32,
            version: agent.version.clone(),
            endpoint: agent.endpoint.clone(),
            metadata: string_metadata,
            created_at: Some(SystemTime::from(agent.created_at).into()),
            last_updated: Some(SystemTime::from(agent.last_updated).into()),
        }
    }

    /// Convert protobuf AgentStatus to internal AgentStatus
    fn proto_to_status(&self, status: i32) -> AgentStatus {
        match status {
            0 => AgentStatus::Idle, // Unspecified
            1 => AgentStatus::Idle,
            2 => AgentStatus::Busy,
            3 => AgentStatus::Maintenance,
            4 => AgentStatus::ShuttingDown,
            5 => AgentStatus::Starting,
            6 => AgentStatus::Error,
            _ => AgentStatus::Idle,
        }
    }

    /// Convert internal AgentStatus to protobuf AgentStatus
    fn status_to_proto(&self, status: AgentStatus) -> i32 {
        match status {
            AgentStatus::Idle => 1,
            AgentStatus::Busy => 2,
            AgentStatus::Offline => 2, // Map to Busy for now
            AgentStatus::Error => 6,
            AgentStatus::Maintenance => 3,
            AgentStatus::ShuttingDown => 4,
            AgentStatus::Starting => 5,
        }
    }

    /// Convert protobuf AgentHealth to internal AgentHealth
    fn proto_to_health(&self, health: i32) -> AgentHealth {
        match health {
            0 => AgentHealth::Healthy, // Unspecified
            1 => AgentHealth::Healthy,
            2 => AgentHealth::Degraded,
            3 => AgentHealth::Unhealthy,
            4 => AgentHealth::Offline,
            _ => AgentHealth::Healthy,
        }
    }

    /// Convert internal AgentHealth to protobuf AgentHealth
    fn health_to_proto(&self, health: AgentHealth) -> i32 {
        match health {
            AgentHealth::Healthy => 1,
            AgentHealth::Degraded => 2,
            AgentHealth::Unhealthy => 3,
            AgentHealth::Offline => 4,
        }
    }

    /// Convert internal metrics to protobuf metrics
    fn metrics_to_proto(&self, metrics: &AgentMetrics) -> AgentPerformanceMetrics {
        AgentPerformanceMetrics {
            tasks_completed: metrics.tasks_completed as u32,
            tasks_failed: metrics.tasks_failed as u32,
            avg_completion_time_seconds: metrics.avg_task_time.as_secs_f64(), // Convert Duration to seconds
            success_rate: (metrics.tasks_completed as f64
                / (metrics.tasks_completed + metrics.tasks_failed) as f64)
                * 100.0,
            collaboration_score: 0.0,  // Not available in internal metrics
            avg_response_time_ms: 0.0, // Not available in internal metrics
            cpu_usage_percent: metrics.cpu_usage,
            memory_usage_mb: metrics.memory_usage as f64 / (1024.0 * 1024.0), // Convert bytes to MB
            active_connections: 0, // Not available in internal metrics
        }
    }

    /// Convert protobuf metrics to internal metrics
    fn proto_to_metrics(&self, metrics: &AgentPerformanceMetrics) -> AgentMetrics {
        AgentMetrics {
            tasks_completed: metrics.tasks_completed as u64,
            tasks_failed: metrics.tasks_failed as u64,
            avg_task_time: Duration::from_secs_f64(metrics.avg_completion_time_seconds), // Convert seconds to Duration
            cpu_usage: metrics.cpu_usage_percent,
            memory_usage: (metrics.memory_usage_mb * 1024.0 * 1024.0) as u64, // Convert MB to bytes
            ..Default::default()
        }
    }

    /// Get coordinator reference
    pub fn coordinator(&self) -> &Arc<RwLock<AgentCoordinator>> {
        &self.coordinator
    }
}

#[tonic::async_trait]
impl RealTimeCoordinationService for RealTimeCoordinationServiceImpl {
    /// Register a new agent
    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<RegisterAgentResponse>, Status> {
        let req = request.into_inner();
        let agent_info = req
            .agent_info
            .ok_or_else(|| Status::invalid_argument("Agent info is required"))?;

        debug!("Registering agent: {}", agent_info.id);

        let agent_state = AgentState::new(
            agent_info.id.clone(),
            agent_info.name.clone(),
            agent_info.agent_type.clone(),
            vec![], // capabilities
        );

        self.coordinator
            .write()
            .await
            .register_agent(agent_state)
            .await;

        info!("Agent registered successfully: {}", agent_info.id);
        Ok(Response::new(RegisterAgentResponse {
            success: true,
            message: "Agent registered successfully".to_string(),
        }))
    }

    /// Unregister an agent
    async fn unregister_agent(
        &self,
        request: Request<UnregisterAgentRequest>,
    ) -> Result<Response<UnregisterAgentResponse>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id;

        debug!("Unregistering agent: {}", agent_id);

        self.coordinator
            .write()
            .await
            .unregister_agent(&agent_id)
            .await;

        info!("Agent unregistered successfully: {}", agent_id);
        Ok(Response::new(UnregisterAgentResponse {
            success: true,
            message: "Agent unregistered successfully".to_string(),
        }))
    }

    /// Update agent status
    async fn update_agent_status(
        &self,
        request: Request<UpdateAgentStatusRequest>,
    ) -> Result<Response<UpdateAgentStatusResponse>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id;
        let status = self.proto_to_status(req.status);
        let health = self.proto_to_health(req.health);
        let current_task_id = req.current_task_id;

        debug!("Updating agent status: {} -> {:?}", agent_id, status);

        // Create a new agent state with updated status and health
        let mut agent_state = self
            .coordinator
            .read()
            .await
            .get_agent(&agent_id)
            .await
            .ok_or_else(|| Status::not_found(format!("Agent not found: {agent_id}")))?;
        agent_state.update_status(status);
        agent_state.update_health(health);
        if let Some(task_id) = current_task_id {
            agent_state.current_task = Some(task_id);
        }

        self.coordinator
            .write()
            .await
            .update_agent_state(&agent_id, agent_state)
            .await;

        info!("Agent status updated successfully: {}", agent_id);
        Ok(Response::new(UpdateAgentStatusResponse {
            success: true,
            error_message: String::new(),
        }))
    }

    /// Get agent information
    async fn get_agent_info(
        &self,
        request: Request<GetAgentInfoRequest>,
    ) -> Result<Response<GetAgentInfoResponse>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id;

        debug!("Getting agent info: {}", agent_id);

        match self.coordinator.read().await.get_agent(&agent_id).await {
            Some(agent) => {
                let agent_info = self.agent_state_to_info(&agent);
                Ok(Response::new(GetAgentInfoResponse {
                    success: true,
                    agent_info: Some(agent_info),
                    error_message: String::new(),
                }))
            }
            None => {
                warn!("Agent not found: {}", agent_id);
                Ok(Response::new(GetAgentInfoResponse {
                    success: false,
                    agent_info: None,
                    error_message: "Agent not found".to_string(),
                }))
            }
        }
    }

    /// Get all agents
    async fn get_all_agents(
        &self,
        _request: Request<GetAllAgentsRequest>,
    ) -> Result<Response<GetAllAgentsResponse>, Status> {
        debug!("Getting all agents");

        let agents = self.coordinator.read().await.get_all_agents().await;
        let agent_infos: Vec<AgentInfo> = agents
            .into_iter()
            .map(|agent| self.agent_state_to_info(&agent))
            .collect();

        Ok(Response::new(GetAllAgentsResponse {
            agents: agent_infos,
        }))
    }

    /// Send a message
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        let message = req
            .message
            .ok_or_else(|| Status::invalid_argument("Message is required"))?;

        debug!(
            "Sending message: {} from {} to {:?}",
            message.id, message.sender_id, message.recipient_ids
        );

        // Validate sender exists
        let coordinator = self.coordinator.read().await;
        let sender_exists = coordinator.get_agent(&message.sender_id).await.is_some();
        if !sender_exists {
            return Ok(Response::new(SendMessageResponse {
                success: false,
                message_id: message.id.clone(),
                error_message: "Sender agent not found".to_string(),
            }));
        }

        // Validate recipients exist
        let mut valid_recipients = Vec::new();
        let mut invalid_recipients = Vec::new();

        for recipient_id in &message.recipient_ids {
            match coordinator.get_agent(recipient_id).await {
                Some(_) => valid_recipients.push(recipient_id.clone()),
                None => invalid_recipients.push(recipient_id.clone()),
            }
        }

        if valid_recipients.is_empty() {
            return Ok(Response::new(SendMessageResponse {
                success: false,
                message_id: message.id.clone(),
                error_message: format!(
                    "No valid recipients found. Invalid recipients: {invalid_recipients:?}"
                ),
            }));
        }

        // Store message in history
        {
            let mut history = self.message_history.write().await;
            history.push(message.clone());
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        // Route message to valid recipients
        let message_id = message.id.clone();
        let sender_id = message.sender_id.clone();
        let mut delivery_results = Vec::new();

        for recipient_id in valid_recipients {
            let recipient_id_clone = recipient_id.clone();

            // Record message sent for tracking
            self.message_tracker
                .record_message_sent(&message_id, &sender_id, &recipient_id)
                .await;

            match self.route_message_to_agent(&message, &recipient_id).await {
                Ok(_) => {
                    delivery_results.push((recipient_id, true, String::new()));
                    debug!(
                        "Message {} successfully routed to agent {}",
                        message_id, recipient_id_clone
                    );

                    // Record successful delivery
                    self.message_tracker
                        .record_message_delivered(&message_id)
                        .await;
                }
                Err(e) => {
                    delivery_results.push((recipient_id, false, e.to_string()));
                    warn!(
                        "Failed to route message {} to agent {}: {}",
                        message_id, recipient_id_clone, e
                    );

                    // Record failed delivery
                    self.message_tracker
                        .record_message_failed(&message_id, &e.to_string())
                        .await;
                }
            }
        }

        // Calculate delivery statistics
        let successful_deliveries = delivery_results
            .iter()
            .filter(|(_, success, _)| *success)
            .count();
        let failed_deliveries = delivery_results.len() - successful_deliveries;

        let success = successful_deliveries > 0;
        let error_message = if failed_deliveries > 0 {
            let failed_recipients: Vec<_> = delivery_results
                .iter()
                .filter(|(_, success, _)| !*success)
                .map(|(id, _, error)| format!("{id}: {error}"))
                .collect();
            format!(
                "Failed to deliver to {} recipients: {}",
                failed_deliveries,
                failed_recipients.join(", ")
            )
        } else {
            String::new()
        };

        info!(
            "Message {} routing completed: {}/{} successful deliveries",
            message_id,
            successful_deliveries,
            delivery_results.len()
        );

        Ok(Response::new(SendMessageResponse {
            success,
            message_id,
            error_message,
        }))
    }

    /// Get message stream for an agent
    type GetMessageStreamStream =
        Pin<Box<dyn tokio_stream::Stream<Item = Result<AgentMessage, Status>> + Send>>;

    async fn get_message_stream(
        &self,
        request: Request<GetMessageStreamRequest>,
    ) -> Result<Response<Self::GetMessageStreamStream>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id;

        debug!("Getting message stream for agent: {}", agent_id);

        // Validate that the agent exists and is operational
        let coordinator = self.coordinator.read().await;
        let agent = coordinator
            .get_agent(&agent_id)
            .await
            .ok_or_else(|| Status::not_found(format!("Agent not found: {agent_id}")))?;

        if !agent.is_operational() {
            return Err(Status::failed_precondition(format!(
                "Agent {} is not operational (status: {:?})",
                agent_id, agent.status
            )));
        }

        // Create a dedicated message stream for this agent
        let (tx, rx) = mpsc::unbounded_channel();
        let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

        // Store the sender for this agent's message stream
        // Implement proper agent-specific message stream management
        let agent_streams = Arc::clone(&self.agent_message_streams);

        // Store the sender in the agent streams map
        agent_streams.insert(agent_id.clone(), tx.clone());

        // Clean up the stream when the agent disconnects
        let _cleanup_streams = Arc::clone(&self.agent_message_streams);
        let cleanup_agent_id = agent_id.clone();
        tokio::spawn(async move {
            // Wait for the stream to close
            // Note: We can't easily detect when the stream closes, so we'll rely on
            // the agent to properly disconnect and clean up will happen when they reconnect
            debug!("Message stream established for agent: {}", cleanup_agent_id);
        });

        // Send any pending messages for this agent
        let pending_messages = self.get_pending_messages_for_agent(&agent_id).await;
        for message in pending_messages {
            if let Err(e) = tx.send(Ok(message)) {
                warn!(
                    "Failed to send pending message to agent {}: {}",
                    agent_id, e
                );
                break;
            }
        }

        info!("Message stream established for agent: {}", agent_id);

        Ok(Response::new(
            Box::pin(stream) as Self::GetMessageStreamStream
        ))
    }

    /// Get message history
    async fn get_message_history(
        &self,
        request: Request<GetMessageHistoryRequest>,
    ) -> Result<Response<GetMessageHistoryResponse>, Status> {
        let req = request.into_inner();
        let limit = req.limit as usize;
        let agent_id = req.agent_id;

        debug!(
            "Getting message history, limit: {}, agent: {:?}",
            limit, agent_id
        );

        let history = self.message_history.read().await;
        let messages = if let Some(agent_id) = agent_id {
            history
                .iter()
                .filter(|msg| msg.sender_id == agent_id || msg.recipient_ids.contains(&agent_id))
                .take(limit)
                .cloned()
                .collect()
        } else {
            history.iter().take(limit).cloned().collect()
        };

        Ok(Response::new(GetMessageHistoryResponse { messages }))
    }

    /// Create a coordination session
    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<CreateSessionResponse>, Status> {
        let req = request.into_inner();
        let topic = req.topic;
        let participants = req.participants;

        debug!(
            "Creating session: {} with participants: {:?}",
            topic, participants
        );

        let session_id = uuid::Uuid::new_v4().to_string();
        let session = CoordinationSession {
            id: session_id.clone(),
            topic,
            participants,
            status: SessionStatus::Active.into(),
            started_at: Some(SystemTime::now().into()),
            ended_at: None,
            messages: Vec::new(),
            decisions: Vec::new(),
        };

        self.sessions.insert(session_id.clone(), session);

        Ok(Response::new(CreateSessionResponse {
            success: true,
            session_id,
            error_message: String::new(),
        }))
    }

    /// Join a coordination session
    async fn join_session(
        &self,
        request: Request<JoinSessionRequest>,
    ) -> Result<Response<JoinSessionResponse>, Status> {
        let req = request.into_inner();
        let session_id = req.session_id;
        let agent_id = req.agent_id;

        debug!("Agent {} joining session: {}", agent_id, session_id);

        if let Some(mut session) = self.sessions.get_mut(&session_id) {
            if !session.participants.contains(&agent_id) {
                session.participants.push(agent_id.clone());
            }
            Ok(Response::new(JoinSessionResponse {
                success: true,
                error_message: String::new(),
            }))
        } else {
            Ok(Response::new(JoinSessionResponse {
                success: false,
                error_message: "Session not found".to_string(),
            }))
        }
    }

    /// Leave a coordination session
    async fn leave_session(
        &self,
        request: Request<LeaveSessionRequest>,
    ) -> Result<Response<LeaveSessionResponse>, Status> {
        let req = request.into_inner();
        let session_id = req.session_id;
        let agent_id = req.agent_id;

        debug!("Agent {} leaving session: {}", agent_id, session_id);

        if let Some(mut session) = self.sessions.get_mut(&session_id) {
            session.participants.retain(|id| id != &agent_id);
            Ok(Response::new(LeaveSessionResponse {
                success: true,
                error_message: String::new(),
            }))
        } else {
            Ok(Response::new(LeaveSessionResponse {
                success: false,
                error_message: "Session not found".to_string(),
            }))
        }
    }

    /// Send a session message
    async fn send_session_message(
        &self,
        request: Request<SendSessionMessageRequest>,
    ) -> Result<Response<SendSessionMessageResponse>, Status> {
        let req = request.into_inner();
        let session_id = req.session_id;
        let message = req
            .message
            .ok_or_else(|| Status::invalid_argument("Message is required"))?;

        debug!(
            "Sending session message: {} to session: {}",
            message.id, session_id
        );

        if let Some(mut session) = self.sessions.get_mut(&session_id) {
            session.messages.push(message);
            Ok(Response::new(SendSessionMessageResponse {
                success: true,
                error_message: String::new(),
            }))
        } else {
            Ok(Response::new(SendSessionMessageResponse {
                success: false,
                error_message: "Session not found".to_string(),
            }))
        }
    }

    /// Request a resource
    async fn request_resource(
        &self,
        request: Request<RequestResourceRequest>,
    ) -> Result<Response<RequestResourceResponse>, Status> {
        let req = request.into_inner();
        let resource_id = req.resource_id;
        let agent_id = req.agent_id;
        let timeout_seconds = req.timeout_seconds;

        debug!("Agent {} requesting resource: {}", agent_id, resource_id);

        // Implement actual resource management
        let resource_manager = Arc::clone(&self.resource_manager);
        match resource_manager
            .request_resource(&resource_id, &agent_id, timeout_seconds.unwrap_or(30))
            .await
        {
            Ok(acquired) => Ok(Response::new(RequestResourceResponse {
                success: true,
                resource_acquired: acquired,
                error_message: String::new(),
            })),
            Err(e) => Ok(Response::new(RequestResourceResponse {
                success: false,
                resource_acquired: false,
                error_message: format!("Failed to acquire resource: {e}"),
            })),
        }
    }

    /// Release a resource
    async fn release_resource(
        &self,
        request: Request<ReleaseResourceRequest>,
    ) -> Result<Response<ReleaseResourceResponse>, Status> {
        let req = request.into_inner();
        let resource_id = req.resource_id;
        let agent_id = req.agent_id;

        debug!("Agent {} releasing resource: {}", agent_id, resource_id);

        // Implement actual resource management
        let resource_manager = Arc::clone(&self.resource_manager);
        match resource_manager
            .release_resource(&resource_id, &agent_id)
            .await
        {
            Ok(_) => Ok(Response::new(ReleaseResourceResponse {
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Ok(Response::new(ReleaseResourceResponse {
                success: false,
                error_message: format!("Failed to release resource: {e}"),
            })),
        }
    }

    /// Detect a conflict
    async fn detect_conflict(
        &self,
        request: Request<DetectConflictRequest>,
    ) -> Result<Response<DetectConflictResponse>, Status> {
        let req = request.into_inner();
        let resource_id = req.resource_id;
        let agent_ids = req.agent_ids;
        let conflict_type = req.conflict_type;
        let description = req.description;

        debug!(
            "Detecting conflict: {} for resource: {}",
            conflict_type, resource_id
        );

        let conflict_id = uuid::Uuid::new_v4().to_string();
        let conflict = Conflict {
            id: conflict_id,
            resource_id,
            conflicting_agents: agent_ids,
            conflict_type,
            description,
            resolution_strategy: ConflictStrategy::Manual.into(),
            detected_at: Some(SystemTime::now().into()),
            resolved_at: None,
            resolution_data: HashMap::new(),
        };

        self.conflicts.insert(conflict.id.clone(), conflict.clone());

        Ok(Response::new(DetectConflictResponse {
            success: true,
            conflict: Some(conflict),
            error_message: String::new(),
        }))
    }

    /// Resolve a conflict
    async fn resolve_conflict(
        &self,
        request: Request<ResolveConflictRequest>,
    ) -> Result<Response<ResolveConflictResponse>, Status> {
        let req = request.into_inner();
        let conflict_id = req.conflict_id;
        let strategy = req.strategy;
        let resolution_data = req.resolution_data;

        debug!(
            "Resolving conflict: {} with strategy: {:?}",
            conflict_id, strategy
        );

        if let Some(mut conflict) = self.conflicts.get_mut(&conflict_id) {
            conflict.resolution_strategy = strategy;
            conflict.resolved_at = Some(SystemTime::now().into());
            conflict.resolution_data = resolution_data;
            Ok(Response::new(ResolveConflictResponse {
                success: true,
                error_message: String::new(),
            }))
        } else {
            Ok(Response::new(ResolveConflictResponse {
                success: false,
                error_message: "Conflict not found".to_string(),
            }))
        }
    }

    /// Get conflicts
    async fn get_conflicts(
        &self,
        request: Request<GetConflictsRequest>,
    ) -> Result<Response<GetConflictsResponse>, Status> {
        let req = request.into_inner();
        let resource_id = req.resource_id;
        let agent_id = req.agent_id;

        debug!(
            "Getting conflicts, resource: {:?}, agent: {:?}",
            resource_id, agent_id
        );

        let conflicts: Vec<Conflict> = self
            .conflicts
            .iter()
            .filter(|conflict| {
                let matches_resource = resource_id
                    .as_ref()
                    .is_none_or(|rid| conflict.resource_id == *rid);
                let matches_agent = agent_id
                    .as_ref()
                    .is_none_or(|aid| conflict.conflicting_agents.contains(aid));
                matches_resource && matches_agent
            })
            .map(|conflict| conflict.clone())
            .collect();

        Ok(Response::new(GetConflictsResponse { conflicts }))
    }

    /// Get coordination statistics
    async fn get_stats(
        &self,
        _request: Request<GetStatsRequest>,
    ) -> Result<Response<GetStatsResponse>, Status> {
        debug!("Getting coordination statistics");

        let agents = self.coordinator.read().await.get_all_agents().await;
        let active_agents = agents.len() as u32;
        let active_sessions = self.sessions.len() as u32;
        let total_conflicts = self.conflicts.len() as u32;
        let resolved_conflicts = self
            .conflicts
            .iter()
            .filter(|conflict| conflict.resolved_at.is_some())
            .count() as u32;

        // Get message tracking statistics
        let message_stats = self.message_tracker.get_statistics().await;

        let stats = CoordinationStats {
            total_messages: self.message_history.read().await.len() as u32,
            messages_delivered: message_stats.delivered,
            messages_failed: message_stats.failed,
            active_agents,
            active_sessions,
            avg_response_time_ms: message_stats.avg_response_time_ms,
            coordination_efficiency: message_stats.efficiency,
            total_conflicts,
            resolved_conflicts,
            conflict_resolution_rate: if total_conflicts > 0 {
                resolved_conflicts as f64 / total_conflicts as f64
            } else {
                0.0
            },
        };

        Ok(Response::new(GetStatsResponse { stats: Some(stats) }))
    }

    /// Handle agent heartbeat
    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id;
        let status = self.proto_to_status(req.status);
        let health = self.proto_to_health(req.health);
        let _current_task_id = req.current_task_id;
        let metrics = req.metrics;

        debug!(
            "Heartbeat from agent: {} - status: {:?}, health: {:?}",
            agent_id, status, health
        );

        // Update agent status and metrics
        if let Some(metrics) = metrics {
            let internal_metrics = self.proto_to_metrics(&metrics);

            // Update agent metrics in coordinator
            let mut coordinator = self.coordinator.write().await;
            if let Some(mut agent) = coordinator.get_agent(&agent_id).await {
                agent.update_metrics(internal_metrics);
                coordinator.update_agent_state(&agent_id, agent).await;
            }
        }

        // Get pending messages for the agent
        let pending_messages = self.get_pending_messages_for_agent(&agent_id).await;

        Ok(Response::new(HeartbeatResponse {
            success: true,
            pending_messages,
        }))
    }

    /// Stream updates bidirectionally
    type StreamUpdatesStream =
        Pin<Box<dyn tokio_stream::Stream<Item = Result<AgentMessage, Status>> + Send>>;

    async fn stream_updates(
        &self,
        request: Request<Streaming<AgentMessage>>,
    ) -> Result<Response<Self::StreamUpdatesStream>, Status> {
        let mut incoming_stream = request.into_inner();

        debug!("Starting bidirectional update stream");

        let (_tx, rx) = mpsc::channel(100);
        let outgoing_stream = ReceiverStream::new(rx);

        // Implement bidirectional streaming
        // Spawn a task to handle incoming messages and forward them to appropriate recipients
        let message_tracker = Arc::clone(&self.message_tracker);
        let agent_message_streams = Arc::clone(&self.agent_message_streams);

        tokio::spawn(async move {
            while let Some(message_result) = incoming_stream.next().await {
                match message_result {
                    Ok(message) => {
                        debug!(
                            "Received message in bidirectional stream: {} from {} to {:?}",
                            message.id, message.sender_id, message.recipient_ids
                        );

                        // Record message sent for tracking
                        let message_id = message.id.clone();
                        let sender_id = message.sender_id.clone();

                        for recipient_id in &message.recipient_ids {
                            message_tracker
                                .record_message_sent(&message_id, &sender_id, recipient_id)
                                .await;

                            // Try to send to agent's message stream
                            if let Some(agent_tx) = agent_message_streams.get(recipient_id) {
                                match agent_tx.send(Ok(message.clone())) {
                                    Ok(_) => {
                                        debug!(
                                            "Message forwarded to agent {} via stream",
                                            recipient_id
                                        );
                                        message_tracker.record_message_delivered(&message_id).await;
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Failed to forward message to agent {}: {}",
                                            recipient_id, e
                                        );
                                        message_tracker
                                            .record_message_failed(&message_id, &e.to_string())
                                            .await;
                                    }
                                }
                            } else {
                                warn!("Agent {} not connected via stream", recipient_id);
                                message_tracker
                                    .record_message_failed(&message_id, "Agent not connected")
                                    .await;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error in bidirectional stream: {}", e);
                        break;
                    }
                }
            }
            debug!("Bidirectional stream closed");
        });

        Ok(Response::new(
            Box::pin(outgoing_stream) as Self::StreamUpdatesStream
        ))
    }
}

impl RealTimeCoordinationServiceImpl {
    /// Route a message to a specific agent
    async fn route_message_to_agent(
        &self,
        message: &AgentMessage,
        recipient_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check if agent is online and available
        let coordinator = self.coordinator.read().await;
        let agent = coordinator
            .get_agent(recipient_id)
            .await
            .ok_or_else(|| format!("Agent {recipient_id} not found"))?;

        if !agent.is_operational() {
            return Err(format!(
                "Agent {} is not operational (status: {:?})",
                recipient_id, agent.status
            )
            .into());
        }

        // Store message in agent's pending queue (this will be retrieved during heartbeat)
        // For now, we'll use the message history as a simple queue
        // In a production system, this would be a proper message queue per agent

        // If the agent has an active message stream, send directly
        if let Some(event_sender) = &self.event_sender {
            if let Err(e) = event_sender.send(message.clone()) {
                warn!(
                    "Failed to send message to agent {} via event stream: {}",
                    recipient_id, e
                );
                // Fall back to storing in pending queue
            } else {
                debug!("Message sent to agent {} via event stream", recipient_id);
                return Ok(());
            }
        }

        // Store in pending messages for retrieval during heartbeat
        // This is a simplified implementation - in production, use a proper message queue
        debug!(
            "Message stored for agent {} to retrieve during heartbeat",
            recipient_id
        );

        Ok(())
    }

    /// Get pending messages for a specific agent
    async fn get_pending_messages_for_agent(&self, agent_id: &str) -> Vec<AgentMessage> {
        let history = self.message_history.read().await;

        // Filter messages that are intended for this agent and haven't been delivered yet
        // In a production system, this would check a proper delivery status tracking system
        history
            .iter()
            .filter(|msg| {
                msg.recipient_ids.contains(&agent_id.to_string()) && msg.sender_id != agent_id
                // Don't send back messages from the same agent
            })
            .take(50) // Limit to prevent overwhelming the agent
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // TODO: Fix these tests after refactoring is complete
    /*
    use syneidesis_agent::communication::MessageType;
    use super::*;
    use crate::types::{AgentHealth, AgentState, AgentStatus};
    use std::time::SystemTime;
    use tonic::Request;

    fn create_test_agent(id: &str, status: AgentStatus, health: AgentHealth) -> AgentState {
        let mut agent = AgentState::new(
            id.to_string(),
            format!("Test Agent {id}"),
            "test".to_string(),
            vec!["test".to_string()],
        );
        agent.update_status(status);
        agent.update_health(health);
        agent
    }

    fn create_test_message(sender_id: &str, recipient_ids: Vec<String>) -> AgentMessage {
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::Custom as i32,
            priority: 0,
            sender_id: sender_id.to_string(),
            recipient_ids,
            content: "Test message".to_string(),
            payload: None,
            timestamp: Some(SystemTime::now().into()),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_send_message_success() -> Result<(), Box<dyn std::error::Error>> {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register sender agent
        let sender = create_test_agent("sender-1", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(sender)
            .await?;

        // Register recipient agent
        let recipient = create_test_agent("recipient-1", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(recipient)
            .await?;

        // Send message
        let message = create_test_message("sender-1", vec!["recipient-1".to_string()]);
        let request = Request::new(SendMessageRequest {
            message: Some(message),
        });

        let response = service.send_message(request).await.unwrap();
        let response = response.into_inner();

        assert!(response.success);
        assert!(response.error_message.is_empty());
    }

    #[tokio::test]
    async fn test_send_message_invalid_sender() -> Result<(), Box<dyn std::error::Error>> {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register recipient agent (but not sender)
        let recipient = create_test_agent("recipient-1", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(recipient)
            .await?;

        // Send message from non-existent sender
        let message = create_test_message("non-existent-sender", vec!["recipient-1".to_string()]);
        let request = Request::new(SendMessageRequest {
            message: Some(message),
        });

        let response = service.send_message(request).await.unwrap();
        let response = response.into_inner();

        assert!(!response.success);
        assert!(response.error_message.contains("Sender agent not found"));
    }

    #[tokio::test]
    async fn test_send_message_invalid_recipients() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register sender agent
        let sender = create_test_agent("sender-1", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(sender)
            .await
            .unwrap();

        // Send message to non-existent recipients
        let message = create_test_message(
            "sender-1",
            vec!["non-existent-1".to_string(), "non-existent-2".to_string()],
        );
        let request = Request::new(SendMessageRequest {
            message: Some(message),
        });

        let response = service.send_message(request).await.unwrap();
        let response = response.into_inner();

        assert!(!response.success);
        assert!(response.error_message.contains("No valid recipients found"));
        assert!(response.error_message.contains("non-existent-1"));
        assert!(response.error_message.contains("non-existent-2"));
    }

    #[tokio::test]
    async fn test_send_message_mixed_valid_invalid_recipients() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register sender and one valid recipient
        let sender = create_test_agent("sender-1", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(sender)
            .await
            .unwrap();

        let valid_recipient =
            create_test_agent("valid-recipient", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(valid_recipient)
            .await
            .unwrap();

        // Send message to both valid and invalid recipients
        let message = create_test_message(
            "sender-1",
            vec![
                "valid-recipient".to_string(),
                "invalid-recipient".to_string(),
            ],
        );
        let request = Request::new(SendMessageRequest {
            message: Some(message),
        });

        let response = service.send_message(request).await.unwrap();
        let response = response.into_inner();

        // Should succeed completely since invalid recipient is filtered out before routing
        assert!(response.success);
        assert!(response.error_message.is_empty()); // No delivery failures since invalid recipient was filtered out
    }

    #[tokio::test]
    async fn test_send_message_non_operational_recipient() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register sender agent
        let sender = create_test_agent("sender-1", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(sender)
            .await
            .unwrap();

        // Register recipient agent that is not operational
        let recipient =
            create_test_agent("recipient-1", AgentStatus::Error, AgentHealth::Unhealthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(recipient)
            .await
            .unwrap();

        // Send message to non-operational recipient
        let message = create_test_message("sender-1", vec!["recipient-1".to_string()]);
        let request = Request::new(SendMessageRequest {
            message: Some(message),
        });

        let response = service.send_message(request).await.unwrap();
        let response = response.into_inner();

        // Should fail completely since all recipients are non-operational
        assert!(!response.success); // Complete failure since all recipients failed
        assert!(response
            .error_message
            .contains("Failed to deliver to 1 recipients"));
        assert!(response.error_message.contains("not operational"));
    }

    #[tokio::test]
    async fn test_get_message_stream_valid_agent() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register operational agent
        let agent = create_test_agent("test-agent", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(agent)
            .await
            .unwrap();

        let request = Request::new(GetMessageStreamRequest {
            agent_id: "test-agent".to_string(),
        });

        let response = service.get_message_stream(request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_message_stream_invalid_agent() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        let request = Request::new(GetMessageStreamRequest {
            agent_id: "non-existent-agent".to_string(),
        });

        let response = service.get_message_stream(request).await;
        assert!(response.is_err());
        let status = response.err().unwrap();
        assert_eq!(status.code(), tonic::Code::NotFound);
    }

    #[tokio::test]
    async fn test_get_message_stream_non_operational_agent() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register non-operational agent
        let agent = create_test_agent("test-agent", AgentStatus::Error, AgentHealth::Unhealthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(agent)
            .await
            .unwrap();

        let request = Request::new(GetMessageStreamRequest {
            agent_id: "test-agent".to_string(),
        });

        let response = service.get_message_stream(request).await;
        assert!(response.is_err());
        let status = response.err().unwrap();
        assert_eq!(status.code(), tonic::Code::FailedPrecondition);
    }

    #[tokio::test]
    async fn test_get_pending_messages_for_agent() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Add some messages to history
        let message1 = create_test_message("sender-1", vec!["recipient-1".to_string()]);
        let message2 = create_test_message(
            "sender-2",
            vec!["recipient-1".to_string(), "recipient-2".to_string()],
        );
        let message3 = create_test_message("recipient-1", vec!["sender-1".to_string()]); // Should be filtered out

        {
            let mut history = service.message_history.write().await;
            history.push(message1);
            history.push(message2);
            history.push(message3);
        }

        // Get pending messages for recipient-1
        let pending_messages = service.get_pending_messages_for_agent("recipient-1").await;

        // Should get 2 messages (message1 and message2), but not message3 (from same agent)
        assert_eq!(pending_messages.len(), 2);
        assert!(pending_messages
            .iter()
            .all(|msg| msg.recipient_ids.contains(&"recipient-1".to_string())));
        assert!(pending_messages
            .iter()
            .all(|msg| msg.sender_id != "recipient-1"));
    }

    #[tokio::test]
    async fn test_heartbeat_with_pending_messages() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register agent
        let agent = create_test_agent("test-agent", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(agent)
            .await
            .unwrap();

        // Add a message for the agent
        let message = create_test_message("sender-1", vec!["test-agent".to_string()]);
        {
            let mut history = service.message_history.write().await;
            history.push(message);
        }

        let request = Request::new(HeartbeatRequest {
            agent_id: "test-agent".to_string(),
            status: AgentStatus::Idle as i32,
            health: AgentHealth::Healthy as i32,
            current_task_id: None,
            metrics: None,
        });

        let response = service.heartbeat(request).await.unwrap();
        let response = response.into_inner();

        assert!(response.success);
        assert_eq!(response.pending_messages.len(), 1);
        assert_eq!(response.pending_messages[0].sender_id, "sender-1");
    }

    #[tokio::test]
    async fn test_route_message_to_agent_success() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register operational agent
        let agent = create_test_agent("test-agent", AgentStatus::Idle, AgentHealth::Healthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(agent)
            .await
            .unwrap();

        let message = create_test_message("sender-1", vec!["test-agent".to_string()]);
        let result = service.route_message_to_agent(&message, "test-agent").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_route_message_to_agent_non_operational() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        // Register non-operational agent
        let agent = create_test_agent("test-agent", AgentStatus::Error, AgentHealth::Unhealthy);
        service
            .coordinator
            .write()
            .await
            .register_agent(agent)
            .await
            .unwrap();

        let message = create_test_message("sender-1", vec!["test-agent".to_string()]);
        let result = service.route_message_to_agent(&message, "test-agent").await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert!(error.to_string().contains("not operational"));
    }

    #[tokio::test]
    async fn test_route_message_to_agent_not_found() {
        let config = CoordinationConfig::default();
        let mut service = RealTimeCoordinationServiceImpl::new(config).unwrap();
        service.start().await.unwrap();

        let message = create_test_message("sender-1", vec!["non-existent-agent".to_string()]);
        let result = service
            .route_message_to_agent(&message, "non-existent-agent")
            .await;
        assert!(result.is_err());
    }
    */
}
