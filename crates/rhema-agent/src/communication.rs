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

use crate::agent::{AgentId, AgentMessage, AgentRequest, AgentResponse};
use crate::error::{AgentError, AgentResult};
use crate::registry::AgentRegistry;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{Duration, Instant};
use uuid::Uuid;

/// Message types for agent communication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// Task request message
    TaskRequest,
    /// Task response message
    TaskResponse,
    /// Status update message
    StatusUpdate,
    /// Heartbeat message
    Heartbeat,
    /// Coordination message
    Coordination,
    /// Error message
    Error,
    /// Custom message
    Custom,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::TaskRequest => write!(f, "TaskRequest"),
            MessageType::TaskResponse => write!(f, "TaskResponse"),
            MessageType::StatusUpdate => write!(f, "StatusUpdate"),
            MessageType::Heartbeat => write!(f, "Heartbeat"),
            MessageType::Coordination => write!(f, "Coordination"),
            MessageType::Error => write!(f, "Error"),
            MessageType::Custom => write!(f, "Custom"),
        }
    }
}

/// Message priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessagePriority {
    /// Low priority
    Low = 1,
    /// Normal priority
    Normal = 2,
    /// High priority
    High = 3,
    /// Critical priority
    Critical = 4,
}

impl Default for MessagePriority {
    fn default() -> Self {
        MessagePriority::Normal
    }
}

impl std::fmt::Display for MessagePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagePriority::Low => write!(f, "Low"),
            MessagePriority::Normal => write!(f, "Normal"),
            MessagePriority::High => write!(f, "High"),
            MessagePriority::Critical => write!(f, "Critical"),
        }
    }
}

/// Message handler trait for processing messages
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handle a message
    async fn handle_message(&self, message: AgentMessage) -> AgentResult<Option<AgentMessage>>;

    /// Get handler name
    fn name(&self) -> &str;

    /// Get supported message types
    fn supported_message_types(&self) -> &[MessageType];
}

/// Message broker for agent communication
#[derive(Clone)]
pub struct MessageBroker {
    /// Agent registry
    registry: AgentRegistry,
    /// Message handlers
    handlers: Arc<RwLock<HashMap<String, Box<dyn MessageHandler>>>>,
    /// Message queue
    message_queue: Arc<RwLock<VecDeque<AgentMessage>>>,
    /// Message statistics
    stats: Arc<RwLock<MessageStats>>,
    /// Message broker configuration
    config: MessageBrokerConfig,
}

/// Message broker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBrokerConfig {
    /// Maximum queue size per agent
    pub max_queue_size: usize,
    /// Message timeout in seconds
    pub message_timeout: u64,
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
    /// Whether to enable message persistence
    pub enable_persistence: bool,
    /// Whether to enable message routing
    pub enable_routing: bool,
    /// Whether to enable message filtering
    pub enable_filtering: bool,
}

impl Default for MessageBrokerConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 1000,
            message_timeout: 30,
            heartbeat_interval: 30,
            enable_persistence: false,
            enable_routing: true,
            enable_filtering: true,
        }
    }
}

/// Message statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStats {
    /// Total messages sent
    pub total_messages_sent: usize,
    /// Total messages received
    pub total_messages_received: usize,
    /// Messages by type
    pub messages_by_type: HashMap<String, usize>,
    /// Messages by priority
    pub messages_by_priority: HashMap<String, usize>,
    /// Failed messages
    pub failed_messages: usize,
    /// Average message processing time in milliseconds
    pub avg_processing_time: u64,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl Default for MessageStats {
    fn default() -> Self {
        Self {
            total_messages_sent: 0,
            total_messages_received: 0,
            messages_by_type: HashMap::new(),
            messages_by_priority: HashMap::new(),
            failed_messages: 0,
            avg_processing_time: 0,
            last_update: Utc::now(),
        }
    }
}

impl MessageBroker {
    /// Create a new message broker
    pub fn new(registry: crate::registry::AgentRegistry) -> Self {
        Self {
            registry,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            message_queue: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(MessageStats::default())),
            config: MessageBrokerConfig::default(),
        }
    }

    /// Initialize the message broker
    pub async fn initialize(&self) -> AgentResult<()> {
        // Start heartbeat monitoring
        self.start_heartbeat_monitoring().await;

        Ok(())
    }

    /// Start the message broker
    pub async fn start(&self) -> AgentResult<()> {
        let registry = self.registry.clone();
        let message_queue = self.message_queue.clone();
        let handlers = self.handlers.clone();

        tokio::spawn(async move {
            loop {
                let mut queue = message_queue.write().await;
                if let Some(message) = queue.pop_front() {
                    drop(queue);

                    if let Err(error) = Self::process_message(&registry, &handlers, &message).await
                    {
                        eprintln!("Error processing message: {:?}", error);
                    }
                } else {
                    drop(queue);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        });

        Ok(())
    }

    /// Register an agent with the message broker
    pub async fn register_agent(&self, agent_id: &AgentId) -> AgentResult<()> {
        // Store the sender
        {
            let mut queues = self.message_queue.write().await;
            queues.push_back(AgentMessage::StatusUpdate(crate::agent::AgentStatus {
                agent_id: agent_id.clone(),
                state: crate::agent::AgentState::Ready,
                current_task: None,
                health: crate::agent::HealthStatus::Healthy,
                resources: crate::agent::ResourceUsage::default(),
                timestamp: Utc::now(),
            }));
        }

        Ok(())
    }

    /// Unregister an agent from the message broker
    pub async fn unregister_agent(&self, agent_id: &AgentId) -> AgentResult<()> {
        let mut queues = self.message_queue.write().await;
        queues.push_back(AgentMessage::StatusUpdate(crate::agent::AgentStatus {
            agent_id: agent_id.clone(),
            state: crate::agent::AgentState::Stopped,
            current_task: None,
            health: crate::agent::HealthStatus::Unknown,
            resources: crate::agent::ResourceUsage::default(),
            timestamp: Utc::now(),
        }));

        Ok(())
    }

    /// Send a message to an agent
    pub async fn send_message(&self, agent_id: &AgentId, message: AgentMessage) -> AgentResult<()> {
        let mut queue = self.message_queue.write().await;
        queue.push_back(message);
        drop(queue);

        let mut stats = self.stats.write().await;
        stats.total_messages_sent += 1;
        Ok(())
    }

    /// Broadcast a message to all agents
    pub async fn broadcast_message(&self, message: AgentMessage) -> AgentResult<()> {
        let mut queue = self.message_queue.write().await;
        queue.push_back(message);
        drop(queue);

        let mut stats = self.stats.write().await;
        stats.total_messages_sent += 1;
        Ok(())
    }

    /// Send a message to multiple agents
    pub async fn send_to_multiple(
        &self,
        agent_ids: &[AgentId],
        message: AgentMessage,
    ) -> AgentResult<()> {
        let mut errors = Vec::new();

        for agent_id in agent_ids {
            if let Err(_) = self.send_message(agent_id, message.clone()).await {
                errors.push(agent_id.clone());
            }
        }

        if !errors.is_empty() {
            return Err(AgentError::CommunicationFailed {
                reason: format!("Failed to send message to agents: {:?}", errors),
            });
        }

        Ok(())
    }

    /// Register a message handler
    pub async fn register_handler(&self, handler: Box<dyn MessageHandler>) -> AgentResult<()> {
        let mut handlers = self.handlers.write().await;
        handlers.insert(handler.name().to_string(), handler);
        Ok(())
    }

    /// Unregister a message handler
    pub async fn unregister_handler(&self, handler_name: &str) -> AgentResult<()> {
        let mut handlers = self.handlers.write().await;
        handlers.remove(handler_name);

        Ok(())
    }

    /// Get message count
    pub async fn get_message_count(&self) -> usize {
        let stats = self.stats.read().await;
        stats.total_messages_sent + stats.total_messages_received
    }

    /// Get message statistics
    pub async fn get_stats(&self) -> MessageStats {
        self.stats.read().await.clone()
    }

    /// Start heartbeat monitoring
    async fn start_heartbeat_monitoring(&self) {
        let registry = self.registry.clone();
        let message_queue = self.message_queue.clone();
        let heartbeat_interval = self.config.heartbeat_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(heartbeat_interval));

            loop {
                interval.tick().await;

                // Send heartbeat to all agents
                let mut queue = message_queue.write().await;
                queue.push_back(AgentMessage::Heartbeat(crate::agent::AgentHeartbeat {
                    agent_id: "system".to_string(),
                    timestamp: Utc::now(),
                    status: crate::agent::AgentStatus {
                        agent_id: "system".to_string(),
                        state: crate::agent::AgentState::Ready,
                        current_task: None,
                        health: crate::agent::HealthStatus::Healthy,
                        resources: crate::agent::ResourceUsage::default(),
                        timestamp: Utc::now(),
                    },
                }));
            }
        });
    }

    /// Process a message
    async fn process_message(
        registry: &AgentRegistry,
        handlers: &Arc<RwLock<HashMap<String, Box<dyn MessageHandler>>>>,
        message: &AgentMessage,
    ) -> AgentResult<()> {
        // Try to find a handler for this message type
        let message_type = match message {
            AgentMessage::TaskRequest(_) => "task_request",
            AgentMessage::TaskResponse(_) => "task_response",
            AgentMessage::StatusUpdate(_) => "status_update",
            AgentMessage::Heartbeat(_) => "heartbeat",
            AgentMessage::Coordination(_) => "coordination",
            AgentMessage::Error(_) => "error",
            AgentMessage::Custom(_) => "custom",
        };

        let handler_guard = handlers.read().await;
        if let Some(handler) = handler_guard.get(message_type) {
            let response = handler.handle_message(message.clone()).await?;
            if let Some(_response_message) = response {
                // Send response back to sender
                // This would need to be implemented based on your message routing logic
            }
        }
        Ok(())
    }

    /// Get message type from agent message
    fn get_message_type(message: &AgentMessage) -> MessageType {
        match message {
            AgentMessage::TaskRequest(_) => MessageType::TaskRequest,
            AgentMessage::TaskResponse(_) => MessageType::TaskResponse,
            AgentMessage::StatusUpdate(_) => MessageType::StatusUpdate,
            AgentMessage::Heartbeat(_) => MessageType::Heartbeat,
            AgentMessage::Coordination(_) => MessageType::Coordination,
            AgentMessage::Error(_) => MessageType::Error,
            AgentMessage::Custom(_) => MessageType::Custom,
        }
    }

    /// Shutdown the message broker
    pub async fn shutdown(&self) -> AgentResult<()> {
        // Clear all message queues
        let mut queues = self.message_queue.write().await;
        queues.clear();

        // Clear all handlers
        let mut handlers = self.handlers.write().await;
        handlers.clear();

        Ok(())
    }
}

/// Default message handler implementation
pub struct DefaultMessageHandler {
    name: String,
    supported_types: Vec<MessageType>,
}

impl DefaultMessageHandler {
    pub fn new(name: String, supported_types: Vec<MessageType>) -> Self {
        Self {
            name,
            supported_types,
        }
    }
}

#[async_trait]
impl MessageHandler for DefaultMessageHandler {
    async fn handle_message(&self, _message: AgentMessage) -> AgentResult<Option<AgentMessage>> {
        // Default implementation does nothing
        Ok(None)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn supported_message_types(&self) -> &[MessageType] {
        &self.supported_types
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentCapability, AgentConfig, AgentType, BaseAgent};

    #[tokio::test]
    async fn test_message_broker_creation() {
        let registry = AgentRegistry::new();
        let broker = MessageBroker::new(registry);

        assert_eq!(broker.get_message_count().await, 0);
    }

    #[tokio::test]
    async fn test_agent_registration() {
        let registry = AgentRegistry::new();
        let broker = MessageBroker::new(registry);

        broker.initialize().await.unwrap();
        assert!(broker
            .register_agent(&"test-agent".to_string())
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_message_sending() {
        let registry = AgentRegistry::new();
        let broker = MessageBroker::new(registry);

        broker.initialize().await.unwrap();
        broker
            .register_agent(&"test-agent".to_string())
            .await
            .unwrap();

        let message =
            AgentMessage::TaskRequest(AgentRequest::new("test".to_string(), serde_json::json!({})));

        assert!(broker
            .send_message(&"test-agent".to_string(), message)
            .await
            .is_ok());
    }

    #[test]
    fn test_message_priority() {
        assert!(MessagePriority::High > MessagePriority::Normal);
        assert!(MessagePriority::Critical > MessagePriority::High);
    }

    #[test]
    fn test_message_type_display() {
        assert_eq!(MessageType::TaskRequest.to_string(), "TaskRequest");
        assert_eq!(MessageType::Heartbeat.to_string(), "Heartbeat");
    }
}
