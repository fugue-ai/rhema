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

use super::{AgentEvent, AgentState};
use crate::error::CoordinationError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        agent_id: String,
        timestamp: DateTime<Utc>,
    },

    /// Task assignment message
    TaskAssign {
        task_id: String,
        agent_id: String,
        task: serde_json::Value,
    },

    /// Task completion message
    TaskComplete {
        task_id: String,
        agent_id: String,
        result: serde_json::Value,
    },

    /// Task failure message
    TaskFail {
        task_id: String,
        agent_id: String,
        error: String,
    },

    /// Agent status update message
    AgentStatusUpdate { agent_id: String, status: String },

    /// Agent health update message
    AgentHealthUpdate { agent_id: String, health: String },

    /// Event message
    Event { event: AgentEvent },

    /// Ping message
    Ping { timestamp: DateTime<Utc> },

    /// Pong message
    Pong { timestamp: DateTime<Utc> },

    /// Error message
    Error { error: String },
}

/// Communication message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub id: String,

    /// Message type
    pub message_type: MessageType,

    /// Source agent ID
    pub source: String,

    /// Target agent ID (optional)
    pub target: Option<String>,

    /// Message timestamp
    pub timestamp: DateTime<Utc>,

    /// Message metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Message {
    /// Create a new message
    pub fn new(message_type: MessageType, source: String) -> Self {
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
    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Event handler trait
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle incoming message
    async fn handle_message(&self, message: Message) -> Result<(), CoordinationError>;

    /// Handle connection state change
    async fn handle_connection_state_change(
        &self,
        state: ConnectionState,
    ) -> Result<(), CoordinationError>;

    /// Handle error
    async fn handle_error(&self, error: CoordinationError) -> Result<(), CoordinationError>;
}

// All mpsc and event_sender logic removed after WebSocket extraction.

/// Default event handler implementation
pub struct DefaultEventHandler {
    // event_sender field and with_event_sender method removed after WebSocket extraction.
}

impl DefaultEventHandler {
    /// Create a new default event handler
    pub fn new() -> Self {
        Self {
            // event_sender field and with_event_sender method removed after WebSocket extraction.
        }
    }

    // with_event_sender method removed after WebSocket extraction.
}

impl Default for DefaultEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl EventHandler for DefaultEventHandler {
    async fn handle_message(&self, message: Message) -> Result<(), CoordinationError> {
        debug!("Handling message: {:?}", message.message_type);

        match message.message_type {
            MessageType::AgentRegister { agent } => {
                info!("Agent registration received: {}", agent.id);
            }
            MessageType::AgentHeartbeat {
                agent_id,
                timestamp,
            } => {
                debug!("Agent heartbeat received: {} at {}", agent_id, timestamp);
            }
            MessageType::TaskAssign {
                task_id,
                agent_id,
                task: _,
            } => {
                info!("Task assignment received: {} to {}", task_id, agent_id);
            }
            MessageType::TaskComplete {
                task_id,
                agent_id,
                result: _,
            } => {
                info!("Task completion received: {} by {}", task_id, agent_id);
            }
            MessageType::TaskFail {
                task_id,
                agent_id,
                error,
            } => {
                warn!(
                    "Task failure received: {} by {}: {}",
                    task_id, agent_id, error
                );
            }
            MessageType::Event { event } => {
                // event_sender field and with_event_sender method removed after WebSocket extraction.
            }
            MessageType::Ping { timestamp } => {
                debug!("Ping received at {}", timestamp);
            }
            MessageType::Pong { timestamp } => {
                debug!("Pong received at {}", timestamp);
            }
            MessageType::Error { error } => {
                error!("Error message received: {}", error);
            }
            _ => {
                debug!("Unhandled message type: {:?}", message.message_type);
            }
        }

        Ok(())
    }

    async fn handle_connection_state_change(
        &self,
        state: ConnectionState,
    ) -> Result<(), CoordinationError> {
        info!("Connection state changed to: {:?}", state);
        Ok(())
    }

    async fn handle_error(&self, error: CoordinationError) -> Result<(), CoordinationError> {
        error!("Communication error: {}", error);
        Ok(())
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
            "test".to_string(),
        );

        assert_eq!(message.source, "test");
        assert!(!message.id.is_empty());
    }

    #[test]
    fn test_message_with_target() {
        let message = Message::new(
            MessageType::Ping {
                timestamp: Utc::now(),
            },
            "test".to_string(),
        )
        .with_target("target".to_string());

        assert_eq!(message.target, Some("target".to_string()));
    }

    #[tokio::test]
    async fn test_default_event_handler() {
        let handler = DefaultEventHandler::new();

        let message = Message::new(
            MessageType::Ping {
                timestamp: Utc::now(),
            },
            "test".to_string(),
        );

        let result = handler.handle_message(message).await;
        assert!(result.is_ok());
    }
}
