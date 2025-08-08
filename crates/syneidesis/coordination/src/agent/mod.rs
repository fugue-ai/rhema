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

//! Agent coordination module for the Syneidesis library

pub mod communication;
pub mod conflict;
pub mod coordinator;
pub mod state;

// Re-export main types
pub use conflict::{
    Conflict, ConflictHandler, ConflictRecord, ConflictResolver, ConflictStrategy, ResolutionResult,
};
pub use coordinator::{AgentCoordinator, CoordinatorState, Statistics};
pub use state::{
    AgentCapability, AgentConfig, AgentHealth, AgentMetadata, AgentMetrics, AgentState, AgentStatus,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// Agent registered
    AgentRegistered { agent_id: String },

    /// Agent unregistered
    AgentUnregistered { agent_id: String },

    /// Agent health changed
    AgentHealthChanged {
        agent_id: String,
        health: AgentHealth,
    },

    /// Agent status changed
    AgentStatusChanged {
        agent_id: String,
        status: AgentStatus,
    },

    /// Task assigned
    TaskAssigned { task_id: String, agent_id: String },

    /// Task completed
    TaskCompleted { task_id: String, agent_id: String },

    /// Task failed
    TaskFailed {
        task_id: String,
        agent_id: String,
        error: String,
    },

    /// Conflict detected
    ConflictDetected {
        conflict_id: String,
        description: String,
    },

    /// Conflict resolved
    ConflictResolved {
        conflict_id: String,
        resolution: String,
    },

    /// Communication event
    CommunicationEvent {
        event: String,
        data: serde_json::Value,
    },

    /// System event
    SystemEvent {
        event: String,
        data: serde_json::Value,
    },
}

/// Agent event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    /// Event type
    pub event_type: EventType,

    /// Event timestamp
    pub timestamp: DateTime<Utc>,

    /// Event source
    pub source: String,

    /// Event metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AgentEvent {
    /// Create a new agent event
    pub fn new(event_type: EventType, source: String) -> Self {
        Self {
            event_type,
            timestamp: Utc::now(),
            source,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the event
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Task representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task ID
    pub id: String,

    /// Task name
    pub name: String,

    /// Task description
    pub description: String,

    /// Task type
    pub task_type: String,

    /// Task priority
    pub priority: TaskPriority,

    /// Task status
    pub status: TaskStatus,

    /// Task payload
    pub payload: serde_json::Value,

    /// Required agent capabilities
    pub required_capabilities: Vec<String>,

    /// Assigned agent ID
    pub assigned_agent: Option<String>,

    /// Task creation time
    pub created_at: DateTime<Utc>,

    /// Task deadline
    pub deadline: Option<DateTime<Utc>>,

    /// Task result
    pub result: Option<serde_json::Value>,

    /// Task error
    pub error: Option<String>,

    /// Task metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Task {
    /// Create a new task
    pub fn new(
        name: String,
        description: String,
        task_type: String,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            task_type,
            priority: TaskPriority::Normal,
            status: TaskStatus::Pending,
            payload,
            required_capabilities: Vec::new(),
            assigned_agent: None,
            created_at: Utc::now(),
            deadline: None,
            result: None,
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Set task priority
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set required capabilities
    pub fn with_capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.required_capabilities = capabilities;
        self
    }

    /// Set deadline
    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Low priority
    Low = 1,

    /// Normal priority
    Normal = 2,

    /// High priority
    High = 3,

    /// Critical priority
    Critical = 4,

    /// Emergency priority
    Emergency = 5,
}

/// Task status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending assignment
    Pending,

    /// Task is assigned to an agent
    Assigned,

    /// Task is currently running
    Running,

    /// Task completed successfully
    Completed,

    /// Task failed
    Failed,

    /// Task was cancelled
    Cancelled,

    /// Task timed out
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(
            "test_task".to_string(),
            "Test task".to_string(),
            "test".to_string(),
            serde_json::json!({"test": "data"}),
        );

        assert_eq!(task.name, "test_task");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::Normal);
    }

    #[test]
    fn test_task_with_priority() {
        let task = Task::new(
            "test_task".to_string(),
            "Test task".to_string(),
            "test".to_string(),
            serde_json::json!({}),
        )
        .with_priority(TaskPriority::High);

        assert_eq!(task.priority, TaskPriority::High);
    }

    #[test]
    fn test_agent_event_creation() {
        let event = AgentEvent::new(
            EventType::AgentRegistered {
                agent_id: "test".to_string(),
            },
            "test_source".to_string(),
        );

        assert_eq!(event.source, "test_source");
    }
}
