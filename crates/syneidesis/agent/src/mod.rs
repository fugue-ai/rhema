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
pub use communication::{
    CommunicationManager, ConnectionState, EventHandler, Message, MessageType, WebSocketManager,
};
pub use conflict::{
    Conflict, ConflictHandler, ConflictRecord, ConflictResolver, ConflictStrategy, ResolutionResult,
};
pub use coordinator::{AgentCoordinator, CoordinatorState, Statistics};
pub use state::{
    AgentCapability, AgentConfig, AgentMetadata, AgentMetrics, AgentState,
};

// Re-export types from syneidesis-core
pub use syneidesis_core::types::{
    AgentId, TaskId, SessionId, ConflictId, AgentStatus, AgentHealth, TaskPriority, TaskStatus,
    EventType, AgentEvent, Task,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

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
