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

//! Real-time communication framework for agent coordination

use super::AgentState;
use crate::config::WebSocketConfig;
use crate::error::CoordinationError;
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use syneidesis_core::types::{AgentEvent, AgentId, EventType, TaskId};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{connect_async, WebSocketStream};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    /// Connection is connecting
    Connecting,

    /// Connection is connected
    Connected,

    /// Connection is disconnected
    Disconnected,

    /// Connection is reconnecting
    Reconnecting,

    /// Connection is in error state
    Error,
}

/// Message types for communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Agent registration message
    AgentRegister { agent: AgentState },

    /// Agent heartbeat message
    AgentHeartbeat {
        agent_id: AgentId,
        timestamp: DateTime<Utc>,
    },

    /// Task assignment message
    TaskAssign {
        task_id: TaskId,
        agent_id: AgentId,
        task: serde_json::Value,
    },

    /// Task completion message
    TaskComplete {
        task_id: TaskId,
        agent_id: AgentId,
        result: serde_json::Value,
    },

    /// Task failure message
    TaskFail {
        task_id: TaskId,
        agent_id: AgentId,
        error: String,
    },

    /// Agent status update message
    AgentStatusUpdate { agent_id: AgentId, status: String },

    /// Agent health update message
    AgentHealthUpdate { agent_id: AgentId, health: String },

    /// Event message
    Event { event: AgentEvent },

    /// Ping message
    Ping { timestamp: DateTime<Utc> },

    /// Pong message
    Pong { timestamp: DateTime<Utc> },

    /// Error message
    Error { error: String },

    /// Custom message
    Custom {
        message_type: String,
        data: serde_json::Value,
    },
}

/// Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub id: String,

    /// Message type
    pub message_type: MessageType,

    /// Source agent ID
    pub source: AgentId,

    /// Target agent ID (optional)
    pub target: Option<AgentId>,

    /// Message timestamp
    pub timestamp: DateTime<Utc>,

    /// Message metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Message {
    /// Create a new message
    pub fn new(message_type: MessageType, source: AgentId) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message_type,
            source,
            target: None,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Set target agent
    pub fn with_target(mut self, target: AgentId) -> Self {
        self.target = Some(target);
        self
    }

    /// Add metadata to message
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    /// Handle incoming message
    fn handle_message<'a>(
        &'a self,
        message: Message,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CoordinationError>> + Send + 'a>,
    >;

    /// Handle connection state change
    fn handle_connection_state_change<'a>(
        &'a self,
        state: ConnectionState,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CoordinationError>> + Send + 'a>,
    >;

    /// Handle error
    fn handle_error<'a>(
        &'a self,
        error: CoordinationError,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CoordinationError>> + Send + 'a>,
    >;
}

/// WebSocket connection manager
pub struct WebSocketManager {
    /// WebSocket configuration
    config: WebSocketConfig,

    /// Connection state
    connection_state: Arc<RwLock<ConnectionState>>,

    /// Message sender
    message_sender: Option<mpsc::UnboundedSender<Message>>,

    /// Event handler
    event_handler: Option<Arc<dyn EventHandler>>,

    /// Connection URL
    url: String,

    /// Reconnection attempts
    reconnection_attempts: u32,

    /// Maximum reconnection attempts
    max_reconnection_attempts: u32,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new(config: WebSocketConfig, url: String) -> Self {
        Self {
            config,
            connection_state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            message_sender: None,
            event_handler: None,
            url,
            reconnection_attempts: 0,
            max_reconnection_attempts: 5,
        }
    }

    /// Set event handler
    pub fn with_event_handler(mut self, handler: Arc<dyn EventHandler>) -> Self {
        self.event_handler = Some(handler);
        self
    }

    /// Connect to WebSocket server
    pub async fn connect(&mut self) -> Result<(), CoordinationError> {
        info!("Connecting to WebSocket server: {}", self.url);

        // Update connection state
        {
            let mut state = self.connection_state.write();
            *state = ConnectionState::Connecting;
        }

        // Attempt connection
        match connect_async(&self.url).await {
            Ok((ws_stream, _)) => {
                info!("WebSocket connected successfully");

                // Update connection state
                {
                    let mut state = self.connection_state.write();
                    *state = ConnectionState::Connected;
                }

                // Reset reconnection attempts
                self.reconnection_attempts = 0;

                // Set up message handling
                self.setup_message_handling(ws_stream).await?;

                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to WebSocket: {}", e);

                // Update connection state
                {
                    let mut state = self.connection_state.write();
                    *state = ConnectionState::Error;
                }

                Err(CoordinationError::Communication {
                    message: format!("WebSocket connection failed: {e}"),
                })
            }
        }
    }

    /// Disconnect from WebSocket server
    pub async fn disconnect(&mut self) -> Result<(), CoordinationError> {
        info!("Disconnecting from WebSocket server");

        // Update connection state
        {
            let mut state = self.connection_state.write();
            *state = ConnectionState::Disconnected;
        }

        // Clear message sender
        self.message_sender = None;

        Ok(())
    }

    /// Send a message
    pub async fn send_message(&self, message: Message) -> Result<(), CoordinationError> {
        if let Some(sender) = &self.message_sender {
            sender
                .send(message)
                .map_err(|e| CoordinationError::Communication {
                    message: format!("Failed to send message: {e}"),
                })?;
        } else {
            return Err(CoordinationError::Communication {
                message: "No message sender available".to_string(),
            });
        }

        Ok(())
    }

    /// Get current connection state
    pub async fn get_connection_state(&self) -> ConnectionState {
        let state = self.connection_state.read();
        state.clone()
    }

    /// Set up message handling for WebSocket stream
    async fn setup_message_handling(
        &mut self,
        ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    ) -> Result<(), CoordinationError> {
        let (mut write, mut read) = ws_stream.split();

        // Set up message channel
        let (message_sender, mut message_receiver) = mpsc::unbounded_channel();
        self.message_sender = Some(message_sender);

        // Set up ping/pong channel
        let (ping_sender, mut ping_receiver) = mpsc::unbounded_channel();

        // Spawn message sender task
        let _sender_task = tokio::spawn(async move {
            while let Some(message) = message_receiver.recv().await {
                let message_json = serde_json::to_string(&message).unwrap_or_default();
                let ws_message = WsMessage::Text(message_json);

                if let Err(e) = write.send(ws_message).await {
                    error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
        });

        // Spawn ping/pong task
        let _ping_task = tokio::spawn(async move {
            while let Some(pong_data) = ping_receiver.recv().await {
                // Note: This will fail because write is moved, but we'll handle it differently
                debug!("Received ping, would send pong: {:?}", pong_data);
            }
        });

        // Spawn message receiver task
        let event_handler = self.event_handler.clone();
        let _receiver_task = tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(WsMessage::Text(text)) => match serde_json::from_str::<Message>(&text) {
                        Ok(message) => {
                            if let Some(handler) = &event_handler {
                                if let Err(e) = handler.handle_message(message).await {
                                    error!("Failed to handle message: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse message: {}", e);
                        }
                    },
                    Ok(WsMessage::Close(_)) => {
                        info!("WebSocket connection closed");
                        break;
                    }
                    Ok(WsMessage::Ping(data)) => {
                        if let Err(e) = ping_sender.send(data) {
                            error!("Failed to send ping to pong task: {}", e);
                            break;
                        }
                    }
                    Ok(WsMessage::Pong(_)) => {
                        debug!("Received pong");
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {
                        debug!("Received other WebSocket message");
                    }
                }
            }
        });

        // Start ping/pong
        self.start_ping_pong().await?;

        Ok(())
    }

    /// Start ping/pong heartbeat
    async fn start_ping_pong(&self) -> Result<(), CoordinationError> {
        let ping_interval = Duration::from_secs(self.config.ping_interval);
        let message_sender = self.message_sender.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(ping_interval);
            loop {
                interval.tick().await;

                if let Some(sender) = &message_sender {
                    let ping_message = Message::new(
                        MessageType::Ping {
                            timestamp: Utc::now(),
                        },
                        AgentId::new("system"),
                    );

                    if let Err(e) = sender.send(ping_message) {
                        error!("Failed to send ping: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }
}

/// Communication manager for multiple WebSocket connections
pub struct CommunicationManager {
    /// WebSocket managers
    websocket_managers: HashMap<String, WebSocketManager>,

    /// Message handlers
    message_handlers: HashMap<String, Arc<dyn EventHandler>>,

    /// Event sender
    event_sender: Option<mpsc::UnboundedSender<AgentEvent>>,

    /// Configuration
    config: WebSocketConfig,
}

impl CommunicationManager {
    /// Create a new communication manager
    pub fn new(config: WebSocketConfig) -> Self {
        Self {
            websocket_managers: HashMap::new(),
            message_handlers: HashMap::new(),
            event_sender: None,
            config,
        }
    }

    /// Add a WebSocket manager
    pub fn add_websocket_manager(&mut self, id: String, manager: WebSocketManager) {
        self.websocket_managers.insert(id, manager);
    }

    /// Add a message handler
    pub fn add_message_handler(&mut self, message_type: String, handler: Arc<dyn EventHandler>) {
        self.message_handlers.insert(message_type, handler);
    }

    /// Set event sender
    pub fn set_event_sender(&mut self, sender: mpsc::UnboundedSender<AgentEvent>) {
        self.event_sender = Some(sender);
    }

    /// Broadcast message to all connections
    pub async fn broadcast_message(&self, message: Message) -> Result<(), CoordinationError> {
        for manager in self.websocket_managers.values() {
            if let Err(e) = manager.send_message(message.clone()).await {
                warn!("Failed to broadcast message: {}", e);
            }
        }
        Ok(())
    }

    /// Send message to specific manager
    pub async fn send_message_to(
        &self,
        manager_id: &str,
        message: Message,
    ) -> Result<(), CoordinationError> {
        if let Some(manager) = self.websocket_managers.get(manager_id) {
            manager.send_message(message).await
        } else {
            Err(CoordinationError::Communication {
                message: format!("Manager not found: {manager_id}"),
            })
        }
    }

    /// Get connection states for all managers
    pub async fn get_connection_states(&self) -> HashMap<String, ConnectionState> {
        let mut states = HashMap::new();
        for (id, manager) in &self.websocket_managers {
            let state = manager.get_connection_state().await;
            states.insert(id.clone(), state);
        }
        states
    }

    /// Start all WebSocket managers
    pub async fn start_all(&mut self) -> Result<(), CoordinationError> {
        for (id, manager) in &mut self.websocket_managers {
            info!("Starting WebSocket manager: {}", id);
            if let Err(e) = manager.connect().await {
                error!("Failed to start manager {}: {}", id, e);
            }
        }
        Ok(())
    }

    /// Stop all WebSocket managers
    pub async fn stop_all(&mut self) -> Result<(), CoordinationError> {
        for (id, manager) in &mut self.websocket_managers {
            info!("Stopping WebSocket manager: {}", id);
            if let Err(e) = manager.disconnect().await {
                error!("Failed to stop manager {}: {}", id, e);
            }
        }
        Ok(())
    }
}

/// Default event handler implementation
#[derive(Debug)]
pub struct DefaultEventHandler {
    /// Event sender
    event_sender: Option<mpsc::UnboundedSender<AgentEvent>>,
}

impl DefaultEventHandler {
    /// Create a new default event handler
    pub fn new() -> Self {
        Self { event_sender: None }
    }

    /// Set event sender
    pub fn with_event_sender(mut self, sender: mpsc::UnboundedSender<AgentEvent>) -> Self {
        self.event_sender = Some(sender);
        self
    }
}

impl Default for DefaultEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for DefaultEventHandler {
    fn handle_message<'a>(
        &'a self,
        message: Message,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CoordinationError>> + Send + 'a>,
    > {
        Box::pin(async move {
            debug!("Handling message: {:?}", message.message_type);

            match message.message_type {
                MessageType::AgentRegister { agent } => {
                    info!("Agent registered: {}", agent.id);

                    // Send registration event
                    if let Some(sender) = &self.event_sender {
                        let event = AgentEvent::new(
                            EventType::AgentRegistered {
                                agent_id: agent.id.as_str().to_string(),
                            },
                            "communication".to_string(),
                        );
                        if let Err(e) = sender.send(event) {
                            error!("Failed to send registration event: {}", e);
                        }
                    }
                }
                MessageType::AgentHeartbeat {
                    agent_id,
                    timestamp,
                } => {
                    debug!("Agent heartbeat: {} at {}", agent_id, timestamp);
                }
                MessageType::TaskAssign {
                    task_id,
                    agent_id,
                    task,
                } => {
                    info!("Task assigned: {} to {}", task_id, agent_id);

                    // Send task assignment event
                    if let Some(sender) = &self.event_sender {
                        let event = AgentEvent::new(
                            EventType::TaskAssigned {
                                task_id: task_id.as_str().to_string(),
                                agent_id: agent_id.as_str().to_string(),
                            },
                            "communication".to_string(),
                        );
                        if let Err(e) = sender.send(event) {
                            error!("Failed to send task assignment event: {}", e);
                        }
                    }
                }
                MessageType::TaskComplete {
                    task_id,
                    agent_id,
                    result,
                } => {
                    info!("Task completed: {} by {}", task_id, agent_id);

                    // Send task completion event
                    if let Some(sender) = &self.event_sender {
                        let event = AgentEvent::new(
                            EventType::TaskCompleted {
                                task_id: task_id.as_str().to_string(),
                                agent_id: agent_id.as_str().to_string(),
                            },
                            "communication".to_string(),
                        );
                        if let Err(e) = sender.send(event) {
                            error!("Failed to send task completion event: {}", e);
                        }
                    }
                }
                MessageType::TaskFail {
                    task_id,
                    agent_id,
                    error,
                } => {
                    error!("Task failed: {} by {}: {}", task_id, agent_id, error);

                    // Send task failure event
                    if let Some(sender) = &self.event_sender {
                        let event = AgentEvent::new(
                            EventType::TaskFailed {
                                task_id: task_id.as_str().to_string(),
                                agent_id: agent_id.as_str().to_string(),
                                error,
                            },
                            "communication".to_string(),
                        );
                        if let Err(e) = sender.send(event) {
                            error!("Failed to send task failure event: {}", e);
                        }
                    }
                }
                MessageType::AgentStatusUpdate { agent_id, status } => {
                    info!("Agent status update: {} -> {}", agent_id, status);
                }
                MessageType::AgentHealthUpdate { agent_id, health } => {
                    info!("Agent health update: {} -> {}", agent_id, health);
                }
                MessageType::Event { event } => {
                    // Forward event
                    if let Some(sender) = &self.event_sender {
                        if let Err(e) = sender.send(event) {
                            error!("Failed to forward event: {}", e);
                        }
                    }
                }
                MessageType::Ping { timestamp } => {
                    debug!("Received ping at {}", timestamp);
                }
                MessageType::Pong { timestamp } => {
                    debug!("Received pong at {}", timestamp);
                }
                MessageType::Error { error } => {
                    error!("Received error message: {}", error);
                }
                MessageType::Custom { message_type, data } => {
                    debug!("Received custom message: {} - {:?}", message_type, data);
                }
            }

            Ok(())
        })
    }

    fn handle_connection_state_change<'a>(
        &'a self,
        state: ConnectionState,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CoordinationError>> + Send + 'a>,
    > {
        Box::pin(async move {
            info!("Connection state changed: {:?}", state);
            Ok(())
        })
    }

    fn handle_error<'a>(
        &'a self,
        error: CoordinationError,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), CoordinationError>> + Send + 'a>,
    > {
        Box::pin(async move {
            error!("Communication error: {:?}", error);
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new(
            MessageType::Ping {
                timestamp: Utc::now(),
            },
            AgentId::new("test-agent"),
        );

        assert_eq!(message.source, AgentId::new("test-agent"));
        assert!(matches!(message.message_type, MessageType::Ping { .. }));
    }

    #[test]
    fn test_message_with_target() {
        let message = Message::new(
            MessageType::Ping {
                timestamp: Utc::now(),
            },
            AgentId::new("source"),
        )
        .with_target(AgentId::new("target"));

        assert_eq!(message.target, Some(AgentId::new("target")));
    }

    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config, "ws://localhost:8080".to_string());

        assert_eq!(manager.url, "ws://localhost:8080");
    }

    #[tokio::test]
    async fn test_communication_manager_creation() {
        let config = WebSocketConfig::default();
        let manager = CommunicationManager::new(config);

        assert!(manager.websocket_managers.is_empty());
    }

    #[tokio::test]
    async fn test_default_event_handler() {
        let handler = DefaultEventHandler::new();
        let message = Message::new(
            MessageType::Ping {
                timestamp: Utc::now(),
            },
            AgentId::new("test"),
        );

        let result = handler.handle_message(message).await;
        assert!(result.is_ok());
    }
}
