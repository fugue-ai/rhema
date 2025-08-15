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

use crate::RhemaResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub fn new(sender_id: String, message_type: MessageType, content: String) -> Self {
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
