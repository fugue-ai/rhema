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

//! Core types for the Syneidesis coordination ecosystem

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::CoreError;
use crate::utils::{generate_id, validate_id};

/// Agent identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(String);

/// Task identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(String);

/// Session identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

/// Conflict identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConflictId(String);

impl AgentId {
    /// Create a new agent ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Create a new agent ID with validation
    pub fn new_validated(id: impl Into<String>) -> Result<Self, CoreError> {
        let id = id.into();
        validate_id(&id)?;
        Ok(Self(id))
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generate a new random agent ID
    pub fn generate() -> Self {
        Self(generate_id("agent"))
    }
}

impl TaskId {
    /// Create a new task ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Create a new task ID with validation
    pub fn new_validated(id: impl Into<String>) -> Result<Self, CoreError> {
        let id = id.into();
        validate_id(&id)?;
        Ok(Self(id))
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generate a new random task ID
    pub fn generate() -> Self {
        Self(generate_id("task"))
    }
}

impl SessionId {
    /// Create a new session ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Create a new session ID with validation
    pub fn new_validated(id: impl Into<String>) -> Result<Self, CoreError> {
        let id = id.into();
        validate_id(&id)?;
        Ok(Self(id))
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generate a new random session ID
    pub fn generate() -> Self {
        Self(generate_id("session"))
    }
}

impl ConflictId {
    /// Create a new conflict ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Create a new conflict ID with validation
    pub fn new_validated(id: impl Into<String>) -> Result<Self, CoreError> {
        let id = id.into();
        validate_id(&id)?;
        Ok(Self(id))
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generate a new random conflict ID
    pub fn generate() -> Self {
        Self(generate_id("conflict"))
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for ConflictId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AgentStatus {
    /// Check if the agent can accept new tasks
    pub fn can_accept_tasks(&self) -> bool {
        matches!(self, AgentStatus::Idle)
    }

    /// Check if the agent is operational
    pub fn is_operational(&self) -> bool {
        matches!(self, AgentStatus::Idle | AgentStatus::Busy)
    }
}

impl AgentHealth {
    /// Check if the agent is healthy enough to receive tasks
    pub fn is_healthy(&self) -> bool {
        matches!(self, AgentHealth::Healthy | AgentHealth::Degraded)
    }

    /// Check if the agent is available for tasks
    pub fn is_available(&self) -> bool {
        matches!(self, AgentHealth::Healthy)
    }

    /// Get health score (0-100)
    pub fn score(&self) -> u8 {
        match self {
            AgentHealth::Healthy => 100,
            AgentHealth::Degraded => 75,
            AgentHealth::Unhealthy => 25,
            AgentHealth::Offline => 0,
            AgentHealth::Unknown => 0,
        }
    }
}

impl AgentId {
    /// Check if the agent ID is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Agent status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Agent is idle and ready for tasks
    Idle,
    /// Agent is busy working on a task
    Busy,
    /// Agent is offline
    Offline,
    /// Agent is in maintenance mode
    Maintenance,
    /// Agent is shutting down
    ShuttingDown,
    /// Agent is in an error state
    Error,
}

/// Agent health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentHealth {
    /// Agent is healthy
    Healthy,
    /// Agent is degraded but functional
    Degraded,
    /// Agent is unhealthy
    Unhealthy,
    /// Agent is offline or unreachable
    Offline,
    /// Agent health is unknown
    Unknown,
}

/// Task priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

/// Task status enumeration
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

/// Agent capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    /// Capability name
    pub name: String,
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: Option<String>,
    /// Capability parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Agent metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Agent name
    pub name: String,
    /// Agent type
    pub agent_type: String,
    /// Agent version
    pub version: String,
    /// Agent description
    pub description: Option<String>,
    /// Agent tags
    pub tags: Vec<String>,
    /// Custom metadata
    pub custom: HashMap<String, serde_json::Value>,
}

/// Agent performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Number of tasks completed
    pub tasks_completed: u32,
    /// Number of tasks failed
    pub tasks_failed: u32,
    /// Number of tasks currently running
    pub tasks_running: u32,
    /// Average task completion time in milliseconds
    pub avg_task_time_ms: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for AgentMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            tasks_running: 0,
            avg_task_time_ms: 0.0,
            uptime_seconds: 0,
            last_updated: Utc::now(),
        }
    }
}

impl AgentMetrics {
    /// Record a task completion with duration
    pub fn record_task_completion(&mut self, duration_ms: u64) {
        self.tasks_completed += 1;

        // Update average task time
        let total_time =
            self.avg_task_time_ms * (self.tasks_completed - 1) as f64 + duration_ms as f64;
        self.avg_task_time_ms = total_time / self.tasks_completed as f64;

        self.last_updated = Utc::now();
    }

    /// Record a task failure
    pub fn record_task_failure(&mut self) {
        self.tasks_failed += 1;
        self.last_updated = Utc::now();
    }

    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        let total_tasks = self.tasks_completed + self.tasks_failed;
        if total_tasks == 0 {
            0.0
        } else {
            (self.tasks_completed as f64 / total_tasks as f64) * 100.0
        }
    }

    /// Update CPU usage
    pub fn update_cpu_usage(&mut self, usage: f64) {
        self.cpu_usage = usage;
        self.last_updated = Utc::now();
    }

    /// Update memory usage
    pub fn update_memory_usage(&mut self, usage: u64) {
        self.memory_usage = usage;
        self.last_updated = Utc::now();
    }

    /// Update uptime
    pub fn update_uptime(&mut self, uptime: u64) {
        self.uptime_seconds = uptime;
        self.last_updated = Utc::now();
    }
}

/// Event types for agent coordination
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
    /// Event ID
    pub id: String,
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
            id: Uuid::new_v4().to_string(),
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

/// Task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task ID
    pub id: TaskId,
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
    pub assigned_agent: Option<AgentId>,
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
            id: TaskId::generate(),
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

    /// Set task deadline
    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Add metadata to the task
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id_creation() {
        let id = AgentId::new("agent-1");
        assert_eq!(id.as_str(), "agent-1");
    }

    #[test]
    fn test_task_id_creation() {
        let id = TaskId::new("task-1");
        assert_eq!(id.as_str(), "task-1");
    }

    #[test]
    fn test_session_id_creation() {
        let id = SessionId::new("session-1");
        assert_eq!(id.as_str(), "session-1");
    }

    #[test]
    fn test_conflict_id_creation() {
        let id = ConflictId::new("conflict-1");
        assert_eq!(id.as_str(), "conflict-1");
    }

    #[test]
    fn test_task_creation() {
        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "test".to_string(),
            serde_json::json!({"test": "data"}),
        );

        assert_eq!(task.name, "Test Task");
        assert_eq!(task.description, "A test task");
        assert_eq!(task.task_type, "test");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::Normal);
    }

    #[test]
    fn test_task_with_priority() {
        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
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
                agent_id: "agent-1".to_string(),
            },
            "test".to_string(),
        );

        assert!(matches!(
            event.event_type,
            EventType::AgentRegistered { agent_id } if agent_id == "agent-1"
        ));
        assert_eq!(event.source, "test");
    }
}
