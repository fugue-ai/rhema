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

//! Agent state management for the Syneidesis coordination library

use crate::error::AgentError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Agent health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentHealth {
    /// Agent is healthy and functioning normally
    Healthy,

    /// Agent is experiencing issues but still functional
    Degraded,

    /// Agent is unhealthy and may not be functioning properly
    Unhealthy,

    /// Agent is offline or unreachable
    Offline,
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
        }
    }
}

/// Agent status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Agent is idle and ready for tasks
    Idle,

    /// Agent is busy working on a task
    Busy,

    /// Agent is in maintenance mode
    Maintenance,

    /// Agent is shutting down
    ShuttingDown,

    /// Agent is starting up
    Starting,

    /// Agent is in error state
    Error,
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

/// Agent metrics for monitoring and performance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,

    /// Memory usage in bytes
    pub memory_usage: u64,

    /// Memory usage percentage
    pub memory_usage_percent: f64,

    /// Number of tasks completed
    pub tasks_completed: u64,

    /// Number of tasks failed
    pub tasks_failed: u64,

    /// Number of tasks currently running
    pub tasks_running: u64,

    /// Average task completion time in milliseconds
    pub avg_task_time: u64,

    /// Uptime in seconds
    pub uptime: u64,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,

    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl Default for AgentMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_usage_percent: 0.0,
            tasks_completed: 0,
            tasks_failed: 0,
            tasks_running: 0,
            avg_task_time: 0,
            uptime: 0,
            last_updated: Utc::now(),
            custom_metrics: HashMap::new(),
        }
    }
}

impl AgentMetrics {
    /// Update CPU usage
    pub fn update_cpu_usage(&mut self, usage: f64) {
        self.cpu_usage = usage;
        self.last_updated = Utc::now();
    }

    /// Update memory usage
    pub fn update_memory_usage(&mut self, usage: u64, total: u64) {
        self.memory_usage = usage;
        self.memory_usage_percent = if total > 0 {
            (usage as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        self.last_updated = Utc::now();
    }

    /// Record task completion
    pub fn record_task_completion(&mut self, duration_ms: u64) {
        self.tasks_completed += 1;
        self.tasks_running = self.tasks_running.saturating_sub(1);

        // Update average task time
        if self.tasks_completed > 0 {
            let total_time = self.avg_task_time * (self.tasks_completed - 1) + duration_ms;
            self.avg_task_time = total_time / self.tasks_completed;
        }

        self.last_updated = Utc::now();
    }

    /// Record task failure
    pub fn record_task_failure(&mut self) {
        self.tasks_failed += 1;
        self.tasks_running = self.tasks_running.saturating_sub(1);
        self.last_updated = Utc::now();
    }

    /// Start a new task
    pub fn start_task(&mut self) {
        self.tasks_running += 1;
        self.last_updated = Utc::now();
    }

    /// Update uptime
    pub fn update_uptime(&mut self, uptime: u64) {
        self.uptime = uptime;
        self.last_updated = Utc::now();
    }

    /// Add custom metric
    pub fn add_custom_metric(&mut self, key: String, value: f64) {
        self.custom_metrics.insert(key, value);
        self.last_updated = Utc::now();
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        let total = self.tasks_completed + self.tasks_failed;
        if total == 0 {
            0.0
        } else {
            (self.tasks_completed as f64 / total as f64) * 100.0
        }
    }

    /// Get task throughput (tasks per hour)
    pub fn throughput(&self) -> f64 {
        if self.uptime == 0 {
            0.0
        } else {
            self.tasks_completed as f64 / (self.uptime as f64 / 3600.0)
        }
    }
}

/// Agent state representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Unique agent ID
    pub id: String,

    /// Agent name
    pub name: String,

    /// Agent type
    pub agent_type: String,

    /// Agent capabilities
    pub capabilities: Vec<String>,

    /// Agent health status
    pub health: AgentHealth,

    /// Agent metrics
    pub metrics: AgentMetrics,

    /// Current task ID
    pub current_task: Option<String>,

    /// Agent status
    pub status: AgentStatus,

    /// Agent priority (0-255, higher is more important)
    pub priority: u8,

    /// Agent version
    pub version: String,

    /// Agent endpoint URL
    pub endpoint: Option<String>,

    /// Agent metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// Agent configuration
    pub config: AgentConfig,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,

    /// Last heartbeat timestamp
    pub last_heartbeat: Option<DateTime<Utc>>,
}

impl AgentState {
    /// Create a new agent state
    pub fn new(id: String, name: String, agent_type: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            agent_type,
            capabilities: Vec::new(),
            health: AgentHealth::Healthy,
            metrics: AgentMetrics::default(),
            current_task: None,
            status: AgentStatus::Idle,
            priority: 128,
            version: "1.0.0".to_string(),
            endpoint: None,
            metadata: HashMap::new(),
            config: AgentConfig::default(),
            created_at: now,
            last_updated: now,
            last_heartbeat: Some(now),
        }
    }

    /// Update agent health
    pub fn update_health(&mut self, health: AgentHealth) {
        self.health = health;
        self.last_updated = Utc::now();
    }

    /// Update agent status
    pub fn update_status(&mut self, status: AgentStatus) {
        self.status = status;
        self.last_updated = Utc::now();
    }

    /// Update heartbeat
    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = Some(Utc::now());
        self.last_updated = Utc::now();
    }

    /// Check if agent is healthy
    pub fn is_healthy(&self) -> bool {
        self.health.is_healthy()
    }

    /// Check if agent is available for tasks
    pub fn is_available(&self) -> bool {
        self.health.is_available() && self.status.can_accept_tasks()
    }

    /// Check if agent is operational
    pub fn is_operational(&self) -> bool {
        self.health.is_healthy() && self.status.is_operational()
    }

    /// Add capability
    pub fn add_capability(&mut self, capability: String) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
            self.last_updated = Utc::now();
        }
    }

    /// Remove capability
    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
        self.last_updated = Utc::now();
    }

    /// Check if agent has capability
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&capability.to_string())
    }

    /// Set current task
    pub fn set_current_task(&mut self, task_id: Option<String>) {
        self.current_task = task_id;
        self.last_updated = Utc::now();
    }

    /// Update metrics
    pub fn update_metrics(&mut self, metrics: AgentMetrics) {
        self.metrics = metrics;
        self.last_updated = Utc::now();
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
        self.last_updated = Utc::now();
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    /// Validate agent state
    pub fn validate(&self) -> Result<(), AgentError> {
        if self.id.is_empty() {
            return Err(AgentError::ValidationFailed {
                message: "Agent ID cannot be empty".to_string(),
            });
        }

        if self.name.is_empty() {
            return Err(AgentError::ValidationFailed {
                message: "Agent name cannot be empty".to_string(),
            });
        }

        if self.agent_type.is_empty() {
            return Err(AgentError::ValidationFailed {
                message: "Agent type cannot be empty".to_string(),
            });
        }

        // Priority is already constrained to u8 (0-255) by the type system
        // No additional validation needed

        if self.version.is_empty() {
            return Err(AgentError::ValidationFailed {
                message: "Agent version cannot be empty".to_string(),
            });
        }

        Ok(())
    }

    /// Check if agent is stale (no heartbeat for too long)
    pub fn is_stale(&self, timeout: Duration) -> bool {
        if let Some(last_heartbeat) = self.last_heartbeat {
            let now = Utc::now();
            let duration = now.signed_duration_since(last_heartbeat);
            duration > chrono::Duration::from_std(timeout).unwrap_or_default()
        } else {
            true
        }
    }

    /// Get agent age
    pub fn age(&self) -> Duration {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.created_at);
        duration.to_std().unwrap_or_default()
    }

    /// Get time since last heartbeat
    pub fn time_since_heartbeat(&self) -> Option<Duration> {
        self.last_heartbeat.map(|heartbeat| {
            let now = Utc::now();
            let duration = now.signed_duration_since(heartbeat);
            duration.to_std().unwrap_or_default()
        })
    }

    /// Get agent score for load balancing
    pub fn score(&self) -> f64 {
        let health_score = self.health.score() as f64;
        let priority_score = self.priority as f64;
        let load_score = if self.metrics.tasks_running > 0 {
            100.0 / (self.metrics.tasks_running as f64 + 1.0)
        } else {
            100.0
        };

        (health_score * 0.4) + (priority_score * 0.3) + (load_score * 0.3)
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Maximum number of concurrent tasks
    pub max_concurrent_tasks: usize,

    /// Heartbeat interval
    pub heartbeat_interval: Duration,

    /// Task timeout
    pub task_timeout: Duration,

    /// Enable auto-scaling
    pub auto_scaling: bool,

    /// Minimum instances
    pub min_instances: usize,

    /// Maximum instances
    pub max_instances: usize,

    /// Resource limits
    pub resource_limits: ResourceLimits,

    /// Retry configuration
    pub retry_config: RetryConfig,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 10,
            heartbeat_interval: Duration::from_secs(30),
            task_timeout: Duration::from_secs(300),
            auto_scaling: false,
            min_instances: 1,
            max_instances: 10,
            resource_limits: ResourceLimits::default(),
            retry_config: RetryConfig::default(),
        }
    }
}

/// Resource limits for agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,

    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,

    /// Maximum disk usage in bytes
    pub max_disk_bytes: u64,

    /// Maximum network bandwidth in bytes per second
    pub max_network_bandwidth: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_percent: 80.0,
            max_memory_bytes: 1024 * 1024 * 1024,     // 1GB
            max_disk_bytes: 10 * 1024 * 1024 * 1024,  // 10GB
            max_network_bandwidth: 100 * 1024 * 1024, // 100MB/s
        }
    }
}

/// Retry configuration for failed tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,

    /// Retry delay
    pub retry_delay: Duration,

    /// Exponential backoff
    pub exponential_backoff: bool,

    /// Maximum retry delay
    pub max_retry_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
            exponential_backoff: true,
            max_retry_delay: Duration::from_secs(60),
        }
    }
}

/// Agent capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    /// Capability name
    pub name: String,

    /// Capability version
    pub version: String,

    /// Capability description
    pub description: String,

    /// Capability parameters
    pub parameters: HashMap<String, serde_json::Value>,

    /// Capability enabled
    pub enabled: bool,
}

impl AgentCapability {
    /// Create a new capability
    pub fn new(name: String, version: String, description: String) -> Self {
        Self {
            name,
            version,
            description,
            parameters: HashMap::new(),
            enabled: true,
        }
    }

    /// Add parameter to capability
    pub fn with_parameter(mut self, key: String, value: serde_json::Value) -> Self {
        self.parameters.insert(key, value);
        self
    }
}

/// Agent metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Agent version
    pub version: String,

    /// Agent build information
    pub build_info: HashMap<String, String>,

    /// Agent configuration
    pub config: HashMap<String, serde_json::Value>,

    /// Agent tags
    pub tags: Vec<String>,

    /// Agent description
    pub description: Option<String>,

    /// Agent contact information
    pub contact: Option<HashMap<String, String>>,
}

impl Default for AgentMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            build_info: HashMap::new(),
            config: HashMap::new(),
            tags: Vec::new(),
            description: None,
            contact: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_state_creation() {
        let agent = AgentState::new(
            "test-agent".to_string(),
            "Test Agent".to_string(),
            "test".to_string(),
        );

        assert_eq!(agent.id, "test-agent");
        assert_eq!(agent.health, AgentHealth::Healthy);
        assert_eq!(agent.status, AgentStatus::Idle);
        assert!(agent.is_available());
    }

    #[test]
    fn test_agent_health() {
        assert!(AgentHealth::Healthy.is_healthy());
        assert!(AgentHealth::Degraded.is_healthy());
        assert!(!AgentHealth::Unhealthy.is_healthy());
        assert!(!AgentHealth::Offline.is_healthy());

        assert!(AgentHealth::Healthy.is_available());
        assert!(!AgentHealth::Degraded.is_available());
    }

    #[test]
    fn test_agent_status() {
        assert!(AgentStatus::Idle.can_accept_tasks());
        assert!(!AgentStatus::Busy.can_accept_tasks());

        assert!(AgentStatus::Idle.is_operational());
        assert!(AgentStatus::Busy.is_operational());
        assert!(!AgentStatus::Error.is_operational());
    }

    #[test]
    fn test_agent_metrics() {
        let mut metrics = AgentMetrics::default();

        metrics.record_task_completion(1000);
        assert_eq!(metrics.tasks_completed, 1);
        assert_eq!(metrics.avg_task_time, 1000);

        metrics.record_task_failure();
        assert_eq!(metrics.tasks_failed, 1);

        assert_eq!(metrics.success_rate(), 50.0);
    }

    #[test]
    fn test_agent_capabilities() {
        let mut agent = AgentState::new("test".to_string(), "Test".to_string(), "test".to_string());

        agent.add_capability("test_cap".to_string());
        assert!(agent.has_capability("test_cap"));

        agent.remove_capability("test_cap");
        assert!(!agent.has_capability("test_cap"));
    }

    #[test]
    fn test_agent_validation() {
        let mut agent = AgentState::new("test".to_string(), "Test".to_string(), "test".to_string());

        assert!(agent.validate().is_ok());

        agent.id = "".to_string();
        assert!(agent.validate().is_err());
    }

    #[test]
    fn test_agent_score() {
        let agent = AgentState::new("test".to_string(), "Test".to_string(), "test".to_string());

        let score = agent.score();
        assert!(score > 0.0);
    }
}
