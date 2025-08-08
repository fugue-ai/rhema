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

use crate::agent::{AgentCapability, AgentId, AgentState, AgentType};
use crate::error::{AgentError, AgentResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Agent metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    /// Agent ID
    pub agent_id: AgentId,
    /// Agent type
    pub agent_type: AgentType,
    /// Current state
    pub state: AgentState,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Resource usage metrics
    pub resources: ResourceMetrics,
    /// Task metrics
    pub tasks: TaskMetrics,
    /// Communication metrics
    pub communication: CommunicationMetrics,
    /// Error metrics
    pub errors: ErrorMetrics,
    /// Custom metrics
    pub custom: HashMap<String, f64>,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time: f64,
    /// Throughput (tasks per second)
    pub throughput: f64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Uptime percentage (0.0 to 1.0)
    pub uptime: f64,
    /// Total execution time in seconds
    pub total_execution_time: f64,
    /// Average task duration in milliseconds
    pub avg_task_duration: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_response_time: 0.0,
            throughput: 0.0,
            success_rate: 1.0,
            uptime: 1.0,
            total_execution_time: 0.0,
            avg_task_duration: 0.0,
        }
    }
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in MB
    pub memory_usage: u64,
    /// Disk usage in MB
    pub disk_usage: u64,
    /// Network usage in MB/s
    pub network_usage: f64,
    /// Peak CPU usage
    pub peak_cpu_usage: f64,
    /// Peak memory usage
    pub peak_memory_usage: u64,
}

impl Default for ResourceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            disk_usage: 0,
            network_usage: 0.0,
            peak_cpu_usage: 0.0,
            peak_memory_usage: 0,
        }
    }
}

/// Task metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetrics {
    /// Total tasks executed
    pub total_tasks: u64,
    /// Successful tasks
    pub successful_tasks: u64,
    /// Failed tasks
    pub failed_tasks: u64,
    /// Pending tasks
    pub pending_tasks: u64,
    /// Tasks by type
    pub tasks_by_type: HashMap<String, u64>,
    /// Average task queue time in milliseconds
    pub avg_queue_time: f64,
    /// Tasks per minute
    pub tasks_per_minute: f64,
}

impl Default for TaskMetrics {
    fn default() -> Self {
        Self {
            total_tasks: 0,
            successful_tasks: 0,
            failed_tasks: 0,
            pending_tasks: 0,
            tasks_by_type: HashMap::new(),
            avg_queue_time: 0.0,
            tasks_per_minute: 0.0,
        }
    }
}

/// Communication metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMetrics {
    /// Total messages sent
    pub messages_sent: u64,
    /// Total messages received
    pub messages_received: u64,
    /// Messages by type
    pub messages_by_type: HashMap<String, u64>,
    /// Average message processing time in milliseconds
    pub avg_message_processing_time: f64,
    /// Failed messages
    pub failed_messages: u64,
    /// Messages per minute
    pub messages_per_minute: f64,
}

impl Default for CommunicationMetrics {
    fn default() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            messages_by_type: HashMap::new(),
            avg_message_processing_time: 0.0,
            failed_messages: 0,
            messages_per_minute: 0.0,
        }
    }
}

/// Error metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Total errors
    pub total_errors: u64,
    /// Errors by type
    pub errors_by_type: HashMap<String, u64>,
    /// Error rate (errors per minute)
    pub error_rate: f64,
    /// Last error timestamp
    pub last_error: Option<DateTime<Utc>>,
    /// Error recovery time in seconds
    pub avg_recovery_time: f64,
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            errors_by_type: HashMap::new(),
            error_rate: 0.0,
            last_error: None,
            avg_recovery_time: 0.0,
        }
    }
}

/// Metrics collector for collecting and managing agent metrics
pub struct MetricsCollector {
    /// Agent metrics storage
    agent_metrics: Arc<RwLock<HashMap<AgentId, AgentMetrics>>>,
    /// Global metrics
    global_metrics: Arc<RwLock<GlobalMetrics>>,
    /// Metrics history
    metrics_history: Arc<RwLock<Vec<MetricsSnapshot>>>,
    /// Collection interval in seconds
    collection_interval: u64,
}

/// Global metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMetrics {
    /// Total agents
    pub total_agents: usize,
    /// Active agents
    pub active_agents: usize,
    /// Agents by type
    pub agents_by_type: HashMap<String, usize>,
    /// Agents by state
    pub agents_by_state: HashMap<String, usize>,
    /// Total tasks executed
    pub total_tasks: u64,
    /// Total messages
    pub total_messages: u64,
    /// Total errors
    pub total_errors: u64,
    /// System uptime in seconds
    pub system_uptime: u64,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl Default for GlobalMetrics {
    fn default() -> Self {
        Self {
            total_agents: 0,
            active_agents: 0,
            agents_by_type: HashMap::new(),
            agents_by_state: HashMap::new(),
            total_tasks: 0,
            total_messages: 0,
            total_errors: 0,
            system_uptime: 0,
            last_update: Utc::now(),
        }
    }
}

/// Metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    /// Snapshot ID
    pub snapshot_id: String,
    /// Snapshot timestamp
    pub timestamp: DateTime<Utc>,
    /// Agent metrics
    pub agent_metrics: HashMap<AgentId, AgentMetrics>,
    /// Global metrics
    pub global_metrics: GlobalMetrics,
}

impl MetricsSnapshot {
    pub fn new(
        agent_metrics: HashMap<AgentId, AgentMetrics>,
        global_metrics: GlobalMetrics,
    ) -> Self {
        Self {
            snapshot_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            agent_metrics,
            global_metrics,
        }
    }
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            agent_metrics: Arc::new(RwLock::new(HashMap::new())),
            global_metrics: Arc::new(RwLock::new(GlobalMetrics::default())),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            collection_interval: 30, // 30 seconds
        }
    }

    /// Initialize the metrics collector
    pub async fn initialize(&self) -> AgentResult<()> {
        // Start metrics collection
        self.start_metrics_collection().await;

        Ok(())
    }

    /// Update agent metrics
    pub async fn update_agent_metrics(
        &self,
        agent_id: &AgentId,
        metrics: AgentMetrics,
    ) -> AgentResult<()> {
        let mut agent_metrics = self.agent_metrics.write().await;
        agent_metrics.insert(agent_id.clone(), metrics);

        Ok(())
    }

    /// Get agent metrics
    pub async fn get_agent_metrics(&self, agent_id: &AgentId) -> AgentResult<AgentMetrics> {
        let agent_metrics = self.agent_metrics.read().await;

        agent_metrics
            .get(agent_id)
            .cloned()
            .ok_or_else(|| AgentError::AgentNotFound {
                agent_id: agent_id.clone(),
            })
    }

    /// Get all agent metrics
    pub async fn get_all_agent_metrics(&self) -> HashMap<AgentId, AgentMetrics> {
        self.agent_metrics.read().await.clone()
    }

    /// Update performance metrics
    pub async fn update_performance_metrics(
        &self,
        agent_id: &AgentId,
        performance: PerformanceMetrics,
    ) -> AgentResult<()> {
        let mut agent_metrics = self.agent_metrics.write().await;

        if let Some(metrics) = agent_metrics.get_mut(agent_id) {
            metrics.performance = performance;
            metrics.last_update = Utc::now();
        }

        Ok(())
    }

    /// Update resource metrics
    pub async fn update_resource_metrics(
        &self,
        agent_id: &AgentId,
        resources: ResourceMetrics,
    ) -> AgentResult<()> {
        let mut agent_metrics = self.agent_metrics.write().await;

        if let Some(metrics) = agent_metrics.get_mut(agent_id) {
            metrics.resources = resources;
            metrics.last_update = Utc::now();
        }

        Ok(())
    }

    /// Update task metrics
    pub async fn update_task_metrics(
        &self,
        agent_id: &AgentId,
        tasks: TaskMetrics,
    ) -> AgentResult<()> {
        let mut agent_metrics = self.agent_metrics.write().await;

        if let Some(metrics) = agent_metrics.get_mut(agent_id) {
            metrics.tasks = tasks;
            metrics.last_update = Utc::now();
        }

        Ok(())
    }

    /// Update communication metrics
    pub async fn update_communication_metrics(
        &self,
        agent_id: &AgentId,
        communication: CommunicationMetrics,
    ) -> AgentResult<()> {
        let mut agent_metrics = self.agent_metrics.write().await;

        if let Some(metrics) = agent_metrics.get_mut(agent_id) {
            metrics.communication = communication;
            metrics.last_update = Utc::now();
        }

        Ok(())
    }

    /// Update error metrics
    pub async fn update_error_metrics(
        &self,
        agent_id: &AgentId,
        errors: ErrorMetrics,
    ) -> AgentResult<()> {
        let mut agent_metrics = self.agent_metrics.write().await;

        if let Some(metrics) = agent_metrics.get_mut(agent_id) {
            metrics.errors = errors;
            metrics.last_update = Utc::now();
        }

        Ok(())
    }

    /// Add custom metric
    pub async fn add_custom_metric(
        &self,
        agent_id: &AgentId,
        key: String,
        value: f64,
    ) -> AgentResult<()> {
        let mut agent_metrics = self.agent_metrics.write().await;

        if let Some(metrics) = agent_metrics.get_mut(agent_id) {
            metrics.custom.insert(key, value);
            metrics.last_update = Utc::now();
        }

        Ok(())
    }

    /// Get custom metric
    pub async fn get_custom_metric(&self, agent_id: &AgentId, key: &str) -> Option<f64> {
        let agent_metrics = self.agent_metrics.read().await;

        agent_metrics
            .get(agent_id)
            .and_then(|metrics| metrics.custom.get(key))
            .copied()
    }

    /// Get global metrics
    pub async fn get_global_metrics(&self) -> GlobalMetrics {
        self.global_metrics.read().await.clone()
    }

    /// Update global metrics
    pub async fn update_global_metrics(&self, global_metrics: GlobalMetrics) -> AgentResult<()> {
        let mut global = self.global_metrics.write().await;
        *global = global_metrics;

        Ok(())
    }

    /// Get metrics history
    pub async fn get_metrics_history(&self, limit: Option<usize>) -> Vec<MetricsSnapshot> {
        let history = self.metrics_history.read().await;

        if let Some(limit) = limit {
            history.iter().rev().take(limit).cloned().collect()
        } else {
            history.iter().rev().cloned().collect()
        }
    }

    /// Create metrics snapshot
    pub async fn create_snapshot(&self) -> AgentResult<()> {
        let agent_metrics = self.agent_metrics.read().await.clone();
        let global_metrics = self.global_metrics.read().await.clone();

        let snapshot = MetricsSnapshot::new(agent_metrics, global_metrics);

        let mut history = self.metrics_history.write().await;
        history.push(snapshot);

        // Keep only last 1000 snapshots
        if history.len() > 1000 {
            history.remove(0);
        }

        Ok(())
    }

    /// Get metrics summary
    pub async fn get_metrics_summary(&self) -> MetricsSummary {
        let agent_metrics = self.agent_metrics.read().await;
        let global_metrics = self.global_metrics.read().await;

        let total_agents = agent_metrics.len();
        let active_agents = agent_metrics
            .values()
            .filter(|m| m.state == AgentState::Ready)
            .count();

        let total_tasks: u64 = agent_metrics.values().map(|m| m.tasks.total_tasks).sum();
        let total_messages: u64 = agent_metrics
            .values()
            .map(|m| m.communication.messages_sent + m.communication.messages_received)
            .sum();
        let total_errors: u64 = agent_metrics.values().map(|m| m.errors.total_errors).sum();

        let avg_response_time = if total_agents > 0 {
            agent_metrics
                .values()
                .map(|m| m.performance.avg_response_time)
                .sum::<f64>()
                / total_agents as f64
        } else {
            0.0
        };

        let avg_success_rate = if total_agents > 0 {
            agent_metrics
                .values()
                .map(|m| m.performance.success_rate)
                .sum::<f64>()
                / total_agents as f64
        } else {
            1.0
        };

        MetricsSummary {
            total_agents,
            active_agents,
            total_tasks,
            total_messages,
            total_errors,
            avg_response_time,
            avg_success_rate,
            system_uptime: global_metrics.system_uptime,
            last_update: global_metrics.last_update,
        }
    }

    /// Start metrics collection
    async fn start_metrics_collection(&self) {
        let agent_metrics = self.agent_metrics.clone();
        let global_metrics = self.global_metrics.clone();
        let metrics_history = self.metrics_history.clone();
        let collection_interval = self.collection_interval;

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(collection_interval));

            loop {
                interval.tick().await;

                // Update global metrics
                {
                    let agent_metrics_guard = agent_metrics.read().await;
                    let mut global_metrics_guard = global_metrics.write().await;

                    global_metrics_guard.total_agents = agent_metrics_guard.len();
                    global_metrics_guard.active_agents = agent_metrics_guard
                        .values()
                        .filter(|m| m.state == AgentState::Ready)
                        .count();

                    global_metrics_guard.total_tasks = agent_metrics_guard
                        .values()
                        .map(|m| m.tasks.total_tasks)
                        .sum();
                    global_metrics_guard.total_messages = agent_metrics_guard
                        .values()
                        .map(|m| m.communication.messages_sent + m.communication.messages_received)
                        .sum();
                    global_metrics_guard.total_errors = agent_metrics_guard
                        .values()
                        .map(|m| m.errors.total_errors)
                        .sum();

                    global_metrics_guard.last_update = Utc::now();
                }

                // Create snapshot
                {
                    let agent_metrics_guard = agent_metrics.read().await.clone();
                    let global_metrics_guard = global_metrics.read().await.clone();

                    let snapshot = MetricsSnapshot::new(agent_metrics_guard, global_metrics_guard);

                    let mut history = metrics_history.write().await;
                    history.push(snapshot);

                    // Keep only last 1000 snapshots
                    if history.len() > 1000 {
                        history.remove(0);
                    }
                }
            }
        });
    }

    /// Shutdown the metrics collector
    pub async fn shutdown(&self) -> AgentResult<()> {
        // Create final snapshot
        self.create_snapshot().await?;

        Ok(())
    }
}

/// Metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    /// Total agents
    pub total_agents: usize,
    /// Active agents
    pub active_agents: usize,
    /// Total tasks executed
    pub total_tasks: u64,
    /// Total messages
    pub total_messages: u64,
    /// Total errors
    pub total_errors: u64,
    /// Average response time in milliseconds
    pub avg_response_time: f64,
    /// Average success rate
    pub avg_success_rate: f64,
    /// System uptime in seconds
    pub system_uptime: u64,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl std::fmt::Display for MetricsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Agents: {}/{} active | Tasks: {} | Messages: {} | Errors: {} | Avg Response: {:.2}ms | Success Rate: {:.2}%",
            self.active_agents,
            self.total_agents,
            self.total_tasks,
            self.total_messages,
            self.total_errors,
            self.avg_response_time,
            self.avg_success_rate * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentState, AgentType};

    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new();
        assert!(collector.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_agent_metrics_update() {
        let collector = MetricsCollector::new();
        collector.initialize().await.unwrap();

        let agent_id = "test-agent".to_string();
        let metrics = AgentMetrics {
            agent_id: agent_id.clone(),
            agent_type: AgentType::Development,
            state: AgentState::Ready,
            performance: PerformanceMetrics::default(),
            resources: ResourceMetrics::default(),
            tasks: TaskMetrics::default(),
            communication: CommunicationMetrics::default(),
            errors: ErrorMetrics::default(),
            custom: HashMap::new(),
            last_update: Utc::now(),
        };

        assert!(collector
            .update_agent_metrics(&agent_id, metrics)
            .await
            .is_ok());

        let retrieved_metrics = collector.get_agent_metrics(&agent_id).await.unwrap();
        assert_eq!(retrieved_metrics.agent_id, agent_id);
        assert_eq!(retrieved_metrics.agent_type, AgentType::Development);
    }

    #[tokio::test]
    async fn test_performance_metrics_update() {
        let collector = MetricsCollector::new();
        collector.initialize().await.unwrap();

        let agent_id = "test-agent".to_string();
        let performance = PerformanceMetrics {
            avg_response_time: 100.0,
            throughput: 10.0,
            success_rate: 0.95,
            uptime: 0.99,
            total_execution_time: 3600.0,
            avg_task_duration: 50.0,
        };

        assert!(collector
            .update_performance_metrics(&agent_id, performance)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_custom_metrics() {
        let collector = MetricsCollector::new();
        collector.initialize().await.unwrap();

        let agent_id = "test-agent".to_string();
        assert!(collector
            .add_custom_metric(&agent_id, "custom_metric".to_string(), 42.0)
            .await
            .is_ok());

        let value = collector
            .get_custom_metric(&agent_id, "custom_metric")
            .await;
        assert_eq!(value, Some(42.0));
    }

    #[tokio::test]
    async fn test_metrics_summary() {
        let collector = MetricsCollector::new();
        collector.initialize().await.unwrap();

        let summary = collector.get_metrics_summary().await;
        assert_eq!(summary.total_agents, 0);
        assert_eq!(summary.active_agents, 0);
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.avg_response_time, 0.0);
        assert_eq!(metrics.success_rate, 1.0);
        assert_eq!(metrics.uptime, 1.0);
    }

    #[test]
    fn test_metrics_summary_display() {
        let summary = MetricsSummary {
            total_agents: 10,
            active_agents: 8,
            total_tasks: 1000,
            total_messages: 5000,
            total_errors: 10,
            avg_response_time: 150.0,
            avg_success_rate: 0.95,
            system_uptime: 3600,
            last_update: Utc::now(),
        };

        let display = summary.to_string();
        assert!(display.contains("8/10 active"));
        assert!(display.contains("1000"));
        assert!(display.contains("150.00ms"));
    }
}
