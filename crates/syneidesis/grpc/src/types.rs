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

//! Types and configurations for gRPC services
//!
//! This module provides the types and configurations needed by the gRPC services
//! without depending on the coordination crate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Re-export configuration types from syneidesis-config
pub use syneidesis_config::types::{CoordinationConfig, GrpcClientConfig, GrpcConfig};

// Note: The following local config types have been removed and replaced with
// centralized types from syneidesis-config:
// - CoordinationConfig -> syneidesis_config::types::CoordinationConfig
// - GrpcConfig -> syneidesis_config::types::GrpcConfig
// - GrpcClientConfig -> syneidesis_config::types::GrpcClientConfig

/// Agent status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Agent is idle and ready for tasks
    Idle = 0,
    /// Agent is busy processing a task
    Busy = 1,
    /// Agent is offline or unavailable
    Offline = 2,
    /// Agent is in error state
    Error = 3,
    /// Agent is in maintenance mode
    Maintenance = 4,
    /// Agent is shutting down
    ShuttingDown = 5,
    /// Agent is starting up
    Starting = 6,
}

/// Agent health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentHealth {
    /// Agent is healthy
    Healthy = 0,
    /// Agent is degraded
    Degraded = 1,
    /// Agent is unhealthy
    Unhealthy = 2,
    /// Agent is offline
    Offline = 3,
}

/// Agent state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Agent identifier
    pub id: String,
    /// Agent name
    pub name: String,
    /// Agent type
    pub agent_type: String,
    /// Current status
    pub status: AgentStatus,
    /// Health status
    pub health: AgentHealth,
    /// Current task ID if any
    pub current_task: Option<String>,
    /// Assigned scope
    pub assigned_scope: String,
    /// Agent capabilities
    pub capabilities: Vec<String>,
    /// Last heartbeat timestamp
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether agent is online
    pub is_online: bool,
    /// Performance metrics
    pub metrics: AgentMetrics,
    /// Priority level
    pub priority: i32,
    /// Agent version
    pub version: String,
    /// Agent endpoint
    pub endpoint: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl AgentState {
    /// Create a new agent state
    pub fn new(id: String, name: String, agent_type: String, capabilities: Vec<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            agent_type,
            status: AgentStatus::Idle,
            health: AgentHealth::Healthy,
            current_task: None,
            assigned_scope: "default".to_string(),
            capabilities,
            last_heartbeat: None,
            is_online: true,
            metrics: AgentMetrics::default(),
            priority: 1,
            version: "1.0.0".to_string(),
            endpoint: None,
            metadata: HashMap::new(),
            created_at: now,
            last_updated: now,
        }
    }

    /// Check if agent is operational
    pub fn is_operational(&self) -> bool {
        self.is_online && self.health == AgentHealth::Healthy
    }

    /// Update agent status
    pub fn update_status(&mut self, status: AgentStatus) {
        self.status = status;
        self.last_updated = chrono::Utc::now();
    }

    /// Update agent health
    pub fn update_health(&mut self, health: AgentHealth) {
        self.health = health;
        self.last_updated = chrono::Utc::now();
    }

    /// Update agent metrics
    pub fn update_metrics(&mut self, metrics: AgentMetrics) {
        self.metrics = metrics;
        self.last_updated = chrono::Utc::now();
    }
}

/// Agent performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Number of tasks completed
    pub tasks_completed: u64,
    /// Number of tasks failed
    pub tasks_failed: u64,
    /// Average task completion time
    pub avg_task_time: Duration,
    /// Last update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Statistics for the coordination service
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Statistics {
    /// Total number of agents
    pub total_agents: usize,
    /// Number of online agents
    pub online_agents: usize,
    /// Number of busy agents
    pub busy_agents: usize,
    /// Number of idle agents
    pub idle_agents: usize,
    /// Total messages sent
    pub total_messages: u64,
    /// Total tasks completed
    pub total_tasks: u64,
    /// Average response time
    pub avg_response_time: Duration,
    /// Last update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Simplified agent coordinator interface for gRPC services
#[derive(Debug, Clone)]
pub struct AgentCoordinator {
    /// Agent states
    pub agents: std::collections::HashMap<String, AgentState>,
    /// Statistics
    pub statistics: Statistics,
}

impl AgentCoordinator {
    /// Create a new agent coordinator
    pub fn new() -> Self {
        Self {
            agents: std::collections::HashMap::new(),
            statistics: Statistics::default(),
        }
    }

    /// Register an agent
    pub async fn register_agent(&mut self, agent: AgentState) {
        self.agents.insert(agent.id.clone(), agent);
    }

    /// Unregister an agent
    pub async fn unregister_agent(&mut self, agent_id: &str) {
        self.agents.remove(agent_id);
    }

    /// Get agent state
    pub async fn get_agent(&self, agent_id: &str) -> Option<AgentState> {
        self.agents.get(agent_id).cloned()
    }

    /// Get all agents
    pub async fn get_all_agents(&self) -> Vec<AgentState> {
        self.agents.values().cloned().collect()
    }

    /// Update agent status
    pub async fn update_agent_status(
        &mut self,
        agent_id: &str,
        status: AgentStatus,
        health: AgentHealth,
    ) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = status;
            agent.health = health;
            agent.last_updated = chrono::Utc::now();
        }
    }

    /// Update agent state
    pub async fn update_agent_state(&mut self, agent_id: &str, agent: AgentState) {
        self.agents.insert(agent_id.to_string(), agent);
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> Statistics {
        self.statistics.clone()
    }
}

impl Default for AgentCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Error types for gRPC services
#[derive(Debug, thiserror::Error)]
pub enum GrpcError {
    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Agent error: {message}")]
    Agent { message: String },

    #[error("Communication error: {message}")]
    Communication { message: String },

    #[error("State error: {message}")]
    State { message: String },

    #[error("Conflict error: {message}")]
    Conflict { message: String },

    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl From<GrpcError> for tonic::Status {
    fn from(error: GrpcError) -> Self {
        match error {
            GrpcError::Configuration { message } => tonic::Status::invalid_argument(message),
            GrpcError::Agent { message } => tonic::Status::failed_precondition(message),
            GrpcError::Communication { message } => tonic::Status::unavailable(message),
            GrpcError::State { message } => tonic::Status::data_loss(message),
            GrpcError::Conflict { message } => tonic::Status::aborted(message),
            GrpcError::Internal { message } => tonic::Status::internal(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_config_validation() {
        let coordination_config = CoordinationConfig::default();
        let grpc_config = GrpcConfig::default();
        let grpc_client_config = GrpcClientConfig::default();

        assert!(coordination_config.validate().is_ok());
        assert!(grpc_config.validate().is_ok());
        assert!(grpc_client_config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = GrpcConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: GrpcConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.addr, deserialized.addr);
    }
}
