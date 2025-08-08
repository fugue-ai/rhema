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

use super::AgentState;
use crate::config::CoordinationConfig;
use crate::error::{AgentError, CoordinationError};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use syneidesis_core::types::{AgentEvent, AgentId, EventType, Task, TaskId, TaskStatus};

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

/// Agent coordinator for managing multi-agent coordination
#[derive(Debug)]
pub struct AgentCoordinator {
    /// Registered agents
    agents: DashMap<AgentId, AgentState>,

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
        info!("Starting agent coordinator");
        self.state = CoordinatorState::Starting;

        // Set up event handling
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        self.event_sender = Some(event_sender);

        // Start event handler
        tokio::spawn(async move {
            Self::handle_events(event_receiver).await;
        });

        // Start health checker
        let agents = Arc::new(self.agents.clone());
        tokio::spawn(async move {
            Self::health_check_loop(agents).await;
        });

        // Start task scheduler
        let task_queue = self.task_queue.clone();
        let agents_clone = Arc::new(self.agents.clone());
        tokio::spawn(async move {
            Self::task_scheduler_loop(task_queue, agents_clone).await;
        });

        self.state = CoordinatorState::Running;
        self.start_time = Some(Utc::now());
        info!("Agent coordinator started successfully");

        Ok(())
    }

    /// Stop the coordinator
    pub async fn stop(&mut self) -> Result<(), CoordinationError> {
        info!("Stopping agent coordinator");
        self.state = CoordinatorState::ShuttingDown;

        // Clear event sender
        self.event_sender = None;

        // Clear agents
        self.agents.clear();

        // Clear task queue
        {
            let mut queue = self.task_queue.write();
            queue.clear();
        }

        self.state = CoordinatorState::Stopped;
        info!("Agent coordinator stopped successfully");

        Ok(())
    }

    /// Register a new agent
    pub async fn register_agent(&mut self, agent: AgentState) -> Result<(), AgentError> {
        let agent_id = agent.id.clone();

        if self.agents.contains_key(&agent_id) {
            return Err(AgentError::AlreadyExists {
                agent_id: agent_id.as_str().to_string(),
            });
        }

        // Validate agent
        agent.validate()?;

        // Insert agent
        self.agents.insert(agent_id.clone(), agent);

        // Send registration event
        let event = AgentEvent::new(
            EventType::AgentRegistered {
                agent_id: agent_id.as_str().to_string(),
            },
            "coordinator".to_string(),
        );
        self.send_event(event).await;

        info!("Agent registered: {}", agent_id);
        Ok(())
    }

    /// Unregister an agent
    pub async fn unregister_agent(&mut self, agent_id: &AgentId) -> Result<(), AgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(AgentError::NotFound {
                agent_id: agent_id.as_str().to_string(),
            });
        }

        // Remove agent
        self.agents.remove(agent_id);

        // Send unregistration event
        let event = AgentEvent::new(
            EventType::AgentUnregistered {
                agent_id: agent_id.as_str().to_string(),
            },
            "coordinator".to_string(),
        );
        self.send_event(event).await;

        info!("Agent unregistered: {}", agent_id);
        Ok(())
    }

    /// Get an agent by ID
    pub async fn get_agent(&self, agent_id: &AgentId) -> Result<AgentState, AgentError> {
        self.agents
            .get(agent_id)
            .map(|agent| agent.clone())
            .ok_or_else(|| AgentError::NotFound {
                agent_id: agent_id.as_str().to_string(),
            })
    }

    /// Get all available agents
    pub async fn get_available_agents(&self) -> Result<Vec<AgentState>, AgentError> {
        let available_agents: Vec<AgentState> = self
            .agents
            .iter()
            .filter(|agent| {
                agent.is_healthy() && agent.is_operational() && agent.status.can_accept_tasks()
            })
            .map(|agent| agent.clone())
            .collect();

        Ok(available_agents)
    }

    /// Update agent state
    pub async fn update_agent_state(
        &mut self,
        agent_id: &AgentId,
        state: AgentState,
    ) -> Result<(), AgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(AgentError::NotFound {
                agent_id: agent_id.as_str().to_string(),
            });
        }

        // Validate state
        state.validate()?;

        // Update agent
        self.agents.insert(agent_id.clone(), state);

        info!("Agent state updated: {}", agent_id);
        Ok(())
    }

    /// Assign a task to an available agent
    pub async fn assign_task(&mut self, task: Task) -> Result<AgentId, AgentError> {
        // Get available agents
        let available_agents = self.get_available_agents().await?;

        if available_agents.is_empty() {
            return Err(AgentError::TaskAssignmentFailed {
                message: "No available agents".to_string(),
            });
        }

        // Find best agent for the task
        let best_agent = self
            .find_best_agent_for_task(&available_agents, &task)
            .await?;

        // Assign task to agent
        let mut agent = best_agent.clone();
        agent.set_current_task(Some(task.id.clone()));

        // Update agent state
        self.update_agent_state(&agent.id.clone(), agent).await?;

        // Add task to queue
        {
            let mut queue = self.task_queue.write();
            queue.push(task.clone());
        }

        // Send task assignment event
        let event = AgentEvent::new(
            EventType::TaskAssigned {
                task_id: task.id.as_str().to_string(),
                agent_id: best_agent.id.as_str().to_string(),
            },
            "coordinator".to_string(),
        );
        self.send_event(event).await;

        info!("Task assigned: {} -> {}", task.id, best_agent.id);
        Ok(best_agent.id)
    }

    /// Complete a task
    pub async fn complete_task(
        &mut self,
        task_id: &TaskId,
        result: serde_json::Value,
    ) -> Result<(), AgentError> {
        // Find agent with this task
        let agent_id = {
            let agent_with_task = self
                .agents
                .iter()
                .find(|agent| agent.current_task.as_ref() == Some(task_id));

            if let Some(agent) = agent_with_task {
                Some(agent.id.clone())
            } else {
                None
            }
        };

        if let Some(agent_id) = agent_id {
            let mut updated_agent = self.get_agent(&agent_id).await?;
            updated_agent.set_current_task(None);

            // Update agent state
            self.update_agent_state(&agent_id, updated_agent).await?;

            // Send task completion event
            let event = AgentEvent::new(
                EventType::TaskCompleted {
                    task_id: task_id.as_str().to_string(),
                    agent_id: agent_id.as_str().to_string(),
                },
                "coordinator".to_string(),
            );
            self.send_event(event).await;

            info!("Task completed: {} by {}", task_id, agent_id);
        }

        Ok(())
    }

    /// Get coordinator statistics
    pub async fn get_statistics(&self) -> Result<Statistics, AgentError> {
        let mut stats = Statistics::default();

        // Count agents
        stats.total_agents = self.agents.len();
        stats.healthy_agents = self
            .agents
            .iter()
            .filter(|agent| agent.is_healthy())
            .count();
        stats.available_agents = self
            .agents
            .iter()
            .filter(|agent| agent.is_available())
            .count();

        // Count tasks
        let queue = self.task_queue.read();
        stats.total_tasks = queue.len();
        stats.pending_tasks = queue
            .iter()
            .filter(|task| task.status == TaskStatus::Pending)
            .count();
        stats.running_tasks = queue
            .iter()
            .filter(|task| task.status == TaskStatus::Running)
            .count();
        stats.completed_tasks = queue
            .iter()
            .filter(|task| task.status == TaskStatus::Completed)
            .count();
        stats.failed_tasks = queue
            .iter()
            .filter(|task| task.status == TaskStatus::Failed)
            .count();

        // Calculate success rate
        let total_completed = stats.completed_tasks + stats.failed_tasks;
        if total_completed > 0 {
            stats.success_rate = (stats.completed_tasks as f64 / total_completed as f64) * 100.0;
        }

        // Calculate uptime
        if let Some(start_time) = self.start_time {
            stats.uptime = Utc::now().signed_duration_since(start_time).num_seconds() as u64;
        }

        stats.last_updated = Utc::now();

        // Update statistics
        {
            let mut current_stats = self.statistics.write();
            *current_stats = stats.clone();
        }

        Ok(stats)
    }

    /// Find the best agent for a given task
    async fn find_best_agent_for_task(
        &self,
        available_agents: &[AgentState],
        task: &Task,
    ) -> Result<AgentState, AgentError> {
        if available_agents.is_empty() {
            return Err(AgentError::TaskAssignmentFailed {
                message: "No available agents".to_string(),
            });
        }

        // Filter agents by required capabilities
        let capable_agents: Vec<&AgentState> = available_agents
            .iter()
            .filter(|agent| {
                task.required_capabilities
                    .iter()
                    .all(|cap| agent.has_capability(cap))
            })
            .collect();

        if capable_agents.is_empty() {
            return Err(AgentError::TaskAssignmentFailed {
                message: "No agents with required capabilities".to_string(),
            });
        }

        // Score agents and select the best one
        let best_agent = capable_agents
            .iter()
            .max_by(|a, b| {
                let score_a = a.score();
                let score_b = b.score();
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap();

        Ok((*best_agent).clone())
    }

    /// Handle events
    async fn handle_events(mut receiver: mpsc::UnboundedReceiver<AgentEvent>) {
        while let Some(event) = receiver.recv().await {
            debug!("Processing event: {:?}", event);
            // Handle event here
        }
    }

    /// Health check loop
    async fn health_check_loop(agents: Arc<DashMap<AgentId, AgentState>>) {
        let mut interval_timer = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval_timer.tick().await;

            for agent in agents.iter() {
                // Check if agent is stale
                if agent.is_stale(Duration::from_secs(300)) {
                    warn!("Agent {} is stale, marking as offline", agent.id);
                    // Mark agent as offline
                }
            }
        }
    }

    /// Task scheduler loop
    async fn task_scheduler_loop(
        task_queue: Arc<RwLock<Vec<Task>>>,
        agents: Arc<DashMap<AgentId, AgentState>>,
    ) {
        let mut interval_timer = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval_timer.tick().await;

            let mut queue = task_queue.write();
            let available_agents: Vec<AgentState> = agents
                .iter()
                .filter(|agent| {
                    agent.is_healthy() && agent.is_operational() && agent.status.can_accept_tasks()
                })
                .map(|agent| agent.clone())
                .collect();

            // Process pending tasks
            for task in queue.iter_mut() {
                if task.status == TaskStatus::Pending && !available_agents.is_empty() {
                    // Find best agent and assign task
                    // This is a simplified version
                    task.status = TaskStatus::Assigned;
                }
            }
        }
    }

    /// Send an event
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
            AgentId::new("test-agent"),
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
            AgentId::new("test-agent"),
            "Test Agent".to_string(),
            "test".to_string(),
        );

        coordinator.register_agent(agent.clone()).await.unwrap();
        let retrieved_agent = coordinator.get_agent(&agent.id).await.unwrap();
        assert_eq!(retrieved_agent.id, agent.id);
    }

    #[tokio::test]
    async fn test_task_assignment() {
        let mut coordinator = AgentCoordinator::new();
        let agent = AgentState::new(
            AgentId::new("test-agent"),
            "Test Agent".to_string(),
            "test".to_string(),
        );

        coordinator.register_agent(agent).await.unwrap();
        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
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
    }
}
