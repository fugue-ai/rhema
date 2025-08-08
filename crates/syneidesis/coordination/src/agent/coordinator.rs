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

//! Agent coordinator for managing multi-agent coordination

use super::{AgentEvent, AgentHealth, AgentState, AgentStatus, EventType, Task, TaskStatus};
use crate::config::CoordinationConfig;
use crate::error::{AgentError, CoordinationError};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Coordinator state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoordinatorState {
    /// Coordinator is starting up
    Starting,

    /// Coordinator is running
    Running,

    /// Coordinator is shutting down
    ShuttingDown,

    /// Coordinator is stopped
    Stopped,

    /// Coordinator is in error state
    Error,
}

/// Coordinator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    /// Total number of agents
    pub total_agents: usize,

    /// Number of healthy agents
    pub healthy_agents: usize,

    /// Number of available agents
    pub available_agents: usize,

    /// Total number of tasks
    pub total_tasks: usize,

    /// Number of pending tasks
    pub pending_tasks: usize,

    /// Number of running tasks
    pub running_tasks: usize,

    /// Number of completed tasks
    pub completed_tasks: usize,

    /// Number of failed tasks
    pub failed_tasks: usize,

    /// Average task completion time in milliseconds
    pub avg_task_time: u64,

    /// Success rate percentage
    pub success_rate: f64,

    /// Coordinator uptime in seconds
    pub uptime: u64,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            total_agents: 0,
            healthy_agents: 0,
            available_agents: 0,
            total_tasks: 0,
            pending_tasks: 0,
            running_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            avg_task_time: 0,
            success_rate: 0.0,
            uptime: 0,
            last_updated: Utc::now(),
        }
    }
}

/// Main agent coordinator
#[derive(Debug)]
pub struct AgentCoordinator {
    /// Registered agents
    agents: DashMap<String, AgentState>,

    /// Configuration
    config: CoordinationConfig,

    /// Task queue
    task_queue: Arc<RwLock<Vec<Task>>>,

    /// Event sender
    event_sender: Option<mpsc::UnboundedSender<AgentEvent>>,

    /// Health check interval
    health_check_interval: Duration,

    /// Coordinator state
    state: CoordinatorState,

    /// Statistics
    statistics: Arc<RwLock<Statistics>>,

    /// Start time
    start_time: Option<DateTime<Utc>>,
}

impl AgentCoordinator {
    /// Create a new agent coordinator
    pub fn new() -> Self {
        Self {
            agents: DashMap::new(),
            config: CoordinationConfig::default(),
            task_queue: Arc::new(RwLock::new(Vec::new())),
            event_sender: None,
            health_check_interval: Duration::from_secs(30),
            state: CoordinatorState::Stopped,
            statistics: Arc::new(RwLock::new(Statistics::default())),
            start_time: None,
        }
    }

    /// Create a new agent coordinator with configuration
    pub fn with_config(config: CoordinationConfig) -> Self {
        Self {
            agents: DashMap::new(),
            config,
            task_queue: Arc::new(RwLock::new(Vec::new())),
            event_sender: None,
            health_check_interval: Duration::from_secs(30),
            state: CoordinatorState::Stopped,
            statistics: Arc::new(RwLock::new(Statistics::default())),
            start_time: None,
        }
    }

    /// Start the coordinator
    pub async fn start(&mut self) -> Result<(), CoordinationError> {
        if self.state == CoordinatorState::Running {
            return Ok(());
        }

        self.state = CoordinatorState::Starting;
        info!("Starting agent coordinator");

        // Set up event channel
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        self.event_sender = Some(event_sender);

        // Start event handler
        self.start_event_handler(event_receiver).await?;

        // Start health checker
        self.start_health_checker().await?;

        // Start task scheduler
        self.start_task_scheduler().await?;

        self.state = CoordinatorState::Running;
        self.start_time = Some(Utc::now());

        info!("Agent coordinator started successfully");
        Ok(())
    }

    /// Stop the coordinator
    pub async fn stop(&mut self) -> Result<(), CoordinationError> {
        if self.state == CoordinatorState::Stopped {
            return Ok(());
        }

        self.state = CoordinatorState::ShuttingDown;
        info!("Stopping agent coordinator");

        // Clean up resources
        self.agents.clear();
        self.task_queue.write().clear();

        self.state = CoordinatorState::Stopped;
        info!("Agent coordinator stopped");

        Ok(())
    }

    /// Register an agent
    pub async fn register_agent(&mut self, agent: AgentState) -> Result<(), AgentError> {
        // Validate agent
        agent.validate()?;

        // Check if agent already exists
        if self.agents.contains_key(&agent.id) {
            return Err(AgentError::AlreadyExists {
                agent_id: agent.id.clone(),
            });
        }

        // Check agent limit
        // self.config.max_agents removed; use a constant MAX_AGENTS for now if needed.
        const MAX_AGENTS: usize = 1000;
        if self.agents.len() >= MAX_AGENTS {
            return Err(AgentError::RegistrationFailed {
                message: "Maximum number of agents reached".to_string(),
            });
        }

        // Register agent
        self.agents.insert(agent.id.clone(), agent.clone());

        // Send event
        self.send_event(AgentEvent::new(
            EventType::AgentRegistered {
                agent_id: agent.id.clone(),
            },
            "coordinator".to_string(),
        ))
        .await;

        info!("Agent registered: {}", agent.id);
        Ok(())
    }

    /// Unregister an agent
    pub async fn unregister_agent(&mut self, agent_id: &str) -> Result<(), AgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(AgentError::NotFound {
                agent_id: agent_id.to_string(),
            });
        }

        self.agents.remove(agent_id);

        // Send event
        self.send_event(AgentEvent::new(
            EventType::AgentUnregistered {
                agent_id: agent_id.to_string(),
            },
            "coordinator".to_string(),
        ))
        .await;

        info!("Agent unregistered: {}", agent_id);
        Ok(())
    }

    /// Get agent by ID
    pub async fn get_agent(&self, agent_id: &str) -> Result<AgentState, AgentError> {
        self.agents
            .get(agent_id)
            .map(|agent| agent.clone())
            .ok_or_else(|| AgentError::NotFound {
                agent_id: agent_id.to_string(),
            })
    }

    /// Get all available agents
    pub async fn get_available_agents(&self) -> Result<Vec<AgentState>, AgentError> {
        let mut available_agents = Vec::new();

        for agent in self.agents.iter() {
            if agent.is_available() {
                available_agents.push(agent.clone());
            }
        }

        Ok(available_agents)
    }

    /// Update agent state
    pub async fn update_agent_state(
        &mut self,
        agent_id: &str,
        state: AgentState,
    ) -> Result<(), AgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(AgentError::NotFound {
                agent_id: agent_id.to_string(),
            });
        }

        // Validate new state
        state.validate()?;

        // Update agent
        self.agents.insert(agent_id.to_string(), state.clone());

        // Send event
        self.send_event(AgentEvent::new(
            EventType::AgentStatusChanged {
                agent_id: agent_id.to_string(),
                status: state.status,
            },
            "coordinator".to_string(),
        ))
        .await;

        Ok(())
    }

    /// Assign task to agent
    pub async fn assign_task(&mut self, task: Task) -> Result<String, AgentError> {
        // Find available agent
        let available_agents = self.get_available_agents().await?;

        if available_agents.is_empty() {
            return Err(AgentError::TaskAssignmentFailed {
                message: "No available agents".to_string(),
            });
        }

        // Find best agent for task
        let best_agent = self
            .find_best_agent_for_task(&available_agents, &task)
            .await?;
        let agent_id = best_agent.id.clone();

        // Update task
        let mut task = task;
        task.assigned_agent = Some(agent_id.clone());
        task.status = TaskStatus::Assigned;

        // Add to task queue
        self.task_queue.write().push(task.clone());

        // Update agent
        let mut agent = best_agent;
        agent.set_current_task(Some(task.id.clone()));
        agent.update_status(AgentStatus::Busy);
        agent.metrics.start_task();

        self.agents.insert(agent.id.clone(), agent);

        // Send event
        self.send_event(AgentEvent::new(
            EventType::TaskAssigned {
                task_id: task.id.clone(),
                agent_id: agent_id.clone(),
            },
            "coordinator".to_string(),
        ))
        .await;

        info!("Task assigned: {} to agent: {}", task.id, agent_id);
        Ok(task.id)
    }

    /// Complete task
    pub async fn complete_task(
        &mut self,
        task_id: &str,
        result: serde_json::Value,
    ) -> Result<(), AgentError> {
        // Find task in queue
        let mut task_queue = self.task_queue.write();
        let task_index = task_queue.iter().position(|t| t.id == task_id);

        if let Some(index) = task_index {
            let mut task = task_queue.remove(index);
            task.status = TaskStatus::Completed;
            task.result = Some(result);

            // Calculate actual task duration
            let task_duration = Utc::now()
                .signed_duration_since(task.created_at)
                .num_milliseconds() as u64;

            // Update agent
            if let Some(agent_id) = &task.assigned_agent {
                if let Some(mut agent) = self.agents.get_mut(agent_id) {
                    agent.set_current_task(None);
                    agent.update_status(AgentStatus::Idle);
                    agent.metrics.record_task_completion(task_duration);
                }
            }

            // Send event
            self.send_event(AgentEvent::new(
                EventType::TaskCompleted {
                    task_id: task_id.to_string(),
                    agent_id: task.assigned_agent.unwrap_or_default(),
                },
                "coordinator".to_string(),
            ))
            .await;

            info!("Task completed: {}", task_id);
        }

        Ok(())
    }

    /// Get coordinator statistics
    pub async fn get_statistics(&self) -> Result<Statistics, AgentError> {
        let mut stats = self.statistics.read().clone();

        // Update statistics
        stats.total_agents = self.agents.len();
        stats.healthy_agents = self.agents.iter().filter(|a| a.is_healthy()).count();
        stats.available_agents = self.agents.iter().filter(|a| a.is_available()).count();

        let task_queue = self.task_queue.read();
        stats.total_tasks = task_queue.len();
        stats.pending_tasks = task_queue
            .iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .count();
        stats.running_tasks = task_queue
            .iter()
            .filter(|t| t.status == TaskStatus::Running)
            .count();
        stats.completed_tasks = task_queue
            .iter()
            .filter(|t| t.status == TaskStatus::Completed)
            .count();
        stats.failed_tasks = task_queue
            .iter()
            .filter(|t| t.status == TaskStatus::Failed)
            .count();

        // Calculate success rate
        let total_completed = stats.completed_tasks + stats.failed_tasks;
        if total_completed > 0 {
            stats.success_rate = (stats.completed_tasks as f64 / total_completed as f64) * 100.0;
        }

        // Calculate uptime
        if let Some(start_time) = self.start_time {
            let now = Utc::now();
            let duration = now.signed_duration_since(start_time);
            stats.uptime = duration.num_seconds() as u64;
        }

        stats.last_updated = Utc::now();

        Ok(stats)
    }

    /// Find best agent for task
    async fn find_best_agent_for_task(
        &self,
        available_agents: &[AgentState],
        task: &Task,
    ) -> Result<AgentState, AgentError> {
        let mut best_agent = None;
        let mut best_score = 0.0;

        for agent in available_agents {
            // Check if agent has required capabilities
            let has_capabilities = task
                .required_capabilities
                .iter()
                .all(|cap| agent.has_capability(cap));

            if !has_capabilities {
                continue;
            }

            // Check if agent can handle more tasks
            if agent.metrics.tasks_running >= agent.config.max_concurrent_tasks as u64 {
                continue;
            }

            // Calculate agent score
            let score = agent.score();

            if score > best_score {
                best_score = score;
                best_agent = Some(agent.clone());
            }
        }

        best_agent.ok_or_else(|| AgentError::TaskAssignmentFailed {
            message: "No suitable agent found".to_string(),
        })
    }

    /// Start event handler
    async fn start_event_handler(
        &self,
        mut receiver: mpsc::UnboundedReceiver<AgentEvent>,
    ) -> Result<(), CoordinationError> {
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                debug!("Processing event: {:?}", event.event_type);

                // Process event based on type
                match event.event_type {
                    EventType::AgentRegistered { agent_id } => {
                        info!("Agent registered: {}", agent_id);
                    }
                    EventType::AgentUnregistered { agent_id } => {
                        info!("Agent unregistered: {}", agent_id);
                    }
                    EventType::TaskAssigned { task_id, agent_id } => {
                        info!("Task assigned: {} to agent: {}", task_id, agent_id);
                    }
                    EventType::TaskCompleted { task_id, agent_id } => {
                        info!("Task completed: {} by agent: {}", task_id, agent_id);
                    }
                    _ => {
                        debug!("Unhandled event: {:?}", event.event_type);
                    }
                }
            }
        });

        Ok(())
    }

    /// Start health checker
    async fn start_health_checker(&self) -> Result<(), CoordinationError> {
        let agents = self.agents.clone();
        let health_check_interval = self.health_check_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(health_check_interval);

            loop {
                interval.tick().await;

                for mut agent in agents.iter_mut() {
                    // Check if agent is stale
                    if agent.is_stale(Duration::from_secs(60)) {
                        warn!("Agent {} is stale, marking as offline", agent.id);
                        agent.update_health(AgentHealth::Offline);
                    }
                }
            }
        });

        Ok(())
    }

    /// Start task scheduler
    async fn start_task_scheduler(&self) -> Result<(), CoordinationError> {
        let task_queue = self.task_queue.clone();
        let agents = self.agents.clone();
        let event_sender = self.event_sender.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));

            loop {
                interval.tick().await;

                // Process pending tasks
                let mut tasks_to_assign = Vec::new();
                {
                    let task_queue = task_queue.read();
                    for task in task_queue.iter() {
                        if task.status == TaskStatus::Pending {
                            tasks_to_assign.push((task.id.clone(), task.clone()));
                        }
                    }
                }

                for (task_id, _task) in tasks_to_assign {
                    // Find available agent
                    let available_agents: Vec<AgentState> = agents
                        .iter()
                        .filter(|a| a.is_available())
                        .map(|a| a.clone())
                        .collect();

                    if !available_agents.is_empty() {
                        // Simple task assignment logic (find first available agent)
                        if let Some(best_agent) = available_agents.first() {
                            let best_agent_id = best_agent.id.clone();

                            // Update task in queue
                            {
                                let mut task_queue = task_queue.write();
                                if let Some(task_to_update) =
                                    task_queue.iter_mut().find(|t| t.id == task_id)
                                {
                                    task_to_update.assigned_agent = Some(best_agent_id.clone());
                                    task_to_update.status = TaskStatus::Assigned;
                                }
                            }

                            // Update agent state
                            if let Some(mut agent) = agents.get_mut(&best_agent_id) {
                                agent.set_current_task(Some(task_id.clone()));
                                agent.update_status(AgentStatus::Busy);
                                agent.metrics.start_task();
                            }

                            // Send event
                            if let Some(sender) = &event_sender {
                                let event = AgentEvent::new(
                                    EventType::TaskAssigned {
                                        task_id: task_id.clone(),
                                        agent_id: best_agent_id.clone(),
                                    },
                                    "coordinator".to_string(),
                                );
                                if let Err(e) = sender.send(event) {
                                    error!("Failed to send event: {}", e);
                                }
                            }

                            info!("Task {} assigned to agent {}", task_id, best_agent_id);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Send event
    async fn send_event(&self, event: AgentEvent) {
        if let Some(sender) = &self.event_sender {
            if let Err(e) = sender.send(event) {
                error!("Failed to send event: {}", e);
            }
        }
    }
}

impl Default for AgentCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_coordinator_creation() {
        let coordinator = AgentCoordinator::new();
        assert_eq!(coordinator.state, CoordinatorState::Stopped);
    }

    #[tokio::test]
    async fn test_agent_registration() {
        let mut coordinator = AgentCoordinator::new();
        let agent = AgentState::new(
            "test-agent".to_string(),
            "Test Agent".to_string(),
            "test".to_string(),
        );

        let result = coordinator.register_agent(agent).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_agent_retrieval() {
        let mut coordinator = AgentCoordinator::new();
        let agent = AgentState::new(
            "test-agent".to_string(),
            "Test Agent".to_string(),
            "test".to_string(),
        );

        coordinator.register_agent(agent.clone()).await.unwrap();

        let retrieved_agent = coordinator.get_agent("test-agent").await.unwrap();
        assert_eq!(retrieved_agent.id, agent.id);
    }

    #[tokio::test]
    async fn test_task_assignment() {
        let mut coordinator = AgentCoordinator::new();
        let agent = AgentState::new(
            "test-agent".to_string(),
            "Test Agent".to_string(),
            "test".to_string(),
        );

        coordinator.register_agent(agent).await.unwrap();

        let task = Task::new(
            "test_task".to_string(),
            "Test task".to_string(),
            "test".to_string(),
            serde_json::json!({}),
        );

        let result = coordinator.assign_task(task).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_statistics() {
        let coordinator = AgentCoordinator::new();
        let stats = coordinator.get_statistics().await.unwrap();

        assert_eq!(stats.total_agents, 0);
        assert_eq!(stats.total_tasks, 0);
    }
}
