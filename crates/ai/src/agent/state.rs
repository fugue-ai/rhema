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

use chrono::{DateTime, Utc};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::time::interval;

/// Agent states as defined in the TLA+ specification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Working,
    Blocked,
    Completed,
}

impl std::fmt::Display for AgentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentState::Idle => write!(f, "idle"),
            AgentState::Working => write!(f, "working"),
            AgentState::Blocked => write!(f, "blocked"),
            AgentState::Completed => write!(f, "completed"),
        }
    }
}

/// Agent health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl std::fmt::Display for AgentHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentHealth::Healthy => write!(f, "healthy"),
            AgentHealth::Degraded => write!(f, "degraded"),
            AgentHealth::Unhealthy => write!(f, "unhealthy"),
            AgentHealth::Unknown => write!(f, "unknown"),
        }
    }
}

/// Agent metadata for tracking additional information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// When the agent joined the system
    pub joined_at: DateTime<Utc>,

    /// When the agent was last active
    pub last_active: DateTime<Utc>,

    /// Current scope the agent is working on (if any)
    pub current_scope: Option<String>,

    /// Number of operations performed
    pub operations_count: usize,

    /// Total time spent working
    pub total_work_time: Duration,

    /// Time when the agent was blocked (if currently blocked)
    pub blocked_since: Option<DateTime<Utc>>,

    /// Retry count for failed operations
    pub retry_count: usize,

    /// Agent health status
    pub health: AgentHealth,

    /// Last heartbeat timestamp
    pub last_heartbeat: Option<DateTime<Utc>>,

    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,

    /// Custom metadata
    pub custom_data: HashMap<String, String>,
}

impl Default for AgentMetadata {
    fn default() -> Self {
        Self {
            joined_at: Utc::now(),
            last_active: Utc::now(),
            current_scope: None,
            operations_count: 0,
            total_work_time: Duration::ZERO,
            blocked_since: None,
            retry_count: 0,
            health: AgentHealth::Unknown,
            last_heartbeat: None,
            performance_metrics: PerformanceMetrics::default(),
            custom_data: HashMap::new(),
        }
    }
}

/// Performance metrics for agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time
    pub avg_response_time: Duration,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Throughput (operations per second)
    pub throughput: f64,
    /// Error count
    pub error_count: usize,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_response_time: Duration::ZERO,
            success_rate: 1.0,
            throughput: 0.0,
            error_count: 0,
            last_updated: Utc::now(),
        }
    }
}

/// Agent state persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Directory to store agent state files
    pub state_dir: String,
    /// How often to persist state (seconds)
    pub persist_interval: Duration,
    /// Maximum number of state files to keep
    pub max_state_files: usize,
    /// Whether to enable compression
    pub enable_compression: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            state_dir: ".rhema/agent_states".to_string(),
            persist_interval: Duration::from_secs(30),
            max_state_files: 10,
            enable_compression: true,
        }
    }
}

/// Agent manager for handling agent state transitions with persistence
pub struct AgentManager {
    /// Map of agent ID to current state
    agents: HashMap<String, AgentState>,

    /// Map of agent ID to metadata
    agent_metadata: HashMap<String, AgentMetadata>,

    /// Maximum number of concurrent agents allowed
    max_concurrent_agents: usize,

    /// Maximum time an agent can be blocked
    max_block_time: Duration,

    /// Maximum time without heartbeat before considering agent unhealthy
    max_heartbeat_interval: Duration,

    /// State transition history for auditing
    state_history: Vec<StateTransition>,

    /// Persistence configuration
    persistence_config: PersistenceConfig,

    /// Last persistence timestamp
    last_persist: Instant,

    /// Health monitoring interval
    health_check_interval: Duration,

    /// Cleanup interval
    cleanup_interval: Duration,
}

/// State transition record for auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub timestamp: DateTime<Utc>,
    pub agent_id: String,
    pub from_state: Option<AgentState>,
    pub to_state: AgentState,
    pub reason: String,
}

/// Agent-related errors
#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Agent already exists: {0}")]
    AgentAlreadyExists(String),

    #[error("Invalid agent state: {0} -> {1:?}")]
    InvalidAgentState(String, AgentState),

    #[error("Invalid state transition: {0} -> {1:?}")]
    InvalidStateTransition(String, AgentState),

    #[error("Maximum concurrent agents exceeded: {0}")]
    MaxConcurrentAgentsExceeded(usize),

    #[error("Agent blocked for too long: {0}")]
    AgentBlockedTooLong(String),

    #[error("Agent has active locks and cannot leave: {0}")]
    AgentHasActiveLocks(String),

    #[error("Persistence error: {0}")]
    PersistenceError(String),

    #[error("Recovery error: {0}")]
    RecoveryError(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl AgentManager {
    /// Create a new agent manager
    pub fn new(
        max_concurrent_agents: usize,
        max_block_time: Duration,
        persistence_config: Option<PersistenceConfig>,
    ) -> Self {
        Self {
            agents: HashMap::new(),
            agent_metadata: HashMap::new(),
            max_concurrent_agents,
            max_block_time,
            max_heartbeat_interval: Duration::from_secs(60),
            state_history: Vec::new(),
            persistence_config: persistence_config.unwrap_or_default(),
            last_persist: Instant::now(),
            health_check_interval: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Get all agents and their states
    pub fn agents(&self) -> &HashMap<String, AgentState> {
        &self.agents
    }

    /// Get all agent IDs
    pub fn agent_ids(&self) -> Vec<String> {
        self.agents.keys().cloned().collect()
    }

    /// Get active agents (not idle or completed)
    pub fn active_agents(&self) -> Vec<String> {
        self.agents
            .iter()
            .filter(|(_, state)| matches!(state, AgentState::Working | AgentState::Blocked))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get agents in a specific state
    pub fn agents_in_state(&self, state: &AgentState) -> Vec<String> {
        self.agents
            .iter()
            .filter(|(_, agent_state)| *agent_state == state)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get agent state
    pub fn get_agent_state(&self, agent_id: &str) -> Option<AgentState> {
        self.agents.get(agent_id).cloned()
    }

    /// Get agent metadata
    pub fn get_agent_metadata(&self, agent_id: &str) -> Option<&AgentMetadata> {
        self.agent_metadata.get(agent_id)
    }

    /// Get agent metadata mutable
    pub fn get_agent_metadata_mut(&mut self, agent_id: &str) -> Option<&mut AgentMetadata> {
        self.agent_metadata.get_mut(agent_id)
    }

    /// Agent joins the system
    pub async fn agent_join(&mut self, agent_id: String) -> RhemaResult<()> {
        if self.agents.contains_key(&agent_id) {
            return Err(AgentError::AgentAlreadyExists(agent_id).into());
        }

        if self.agents.len() >= self.max_concurrent_agents {
            return Err(AgentError::MaxConcurrentAgentsExceeded(self.max_concurrent_agents).into());
        }

        // Add agent with idle state
        self.agents.insert(agent_id.clone(), AgentState::Idle);

        // Create metadata
        let mut metadata = AgentMetadata::default();
        metadata.last_heartbeat = Some(Utc::now());
        metadata.health = AgentHealth::Healthy;
        self.agent_metadata.insert(agent_id.clone(), metadata);

        // Record state transition
        self.record_state_transition(&agent_id, None, AgentState::Idle, "Agent joined system");

        Ok(())
    }

    /// Agent leaves the system
    pub async fn agent_leave(&mut self, agent_id: String) -> RhemaResult<()> {
        let current_state = self
            .agents
            .get(&agent_id)
            .ok_or_else(|| AgentError::AgentNotFound(agent_id.clone()))?;

        // Check if agent can leave (not working or blocked)
        if matches!(current_state, AgentState::Working | AgentState::Blocked) {
            return Err(AgentError::AgentHasActiveLocks(agent_id).into());
        }

        // Record state transition before removal
        self.record_state_transition(
            &agent_id,
            Some(current_state.clone()),
            AgentState::Completed,
            "Agent left system",
        );

        // Remove agent
        self.agents.remove(&agent_id);
        self.agent_metadata.remove(&agent_id);

        Ok(())
    }

    /// Set agent state
    pub async fn set_agent_state(
        &mut self,
        agent_id: &str,
        new_state: AgentState,
    ) -> RhemaResult<()> {
        let current_state = self
            .agents
            .get(agent_id)
            .ok_or_else(|| AgentError::AgentNotFound(agent_id.to_string()))?;

        // Validate state transition
        self.validate_state_transition(agent_id, current_state, &new_state)?;

        // Update metadata
        if let Some(metadata) = self.agent_metadata.get_mut(agent_id) {
            metadata.last_active = Utc::now();

            match new_state {
                AgentState::Working => {
                    metadata.operations_count += 1;
                    metadata.blocked_since = None;
                }
                AgentState::Blocked => {
                    metadata.blocked_since = Some(Utc::now());
                }
                AgentState::Completed => {
                    metadata.blocked_since = None;
                }
                _ => {}
            }
        }

        // Record state transition
        self.record_state_transition(
            agent_id,
            Some(current_state.clone()),
            new_state.clone(),
            "State change",
        );

        // Update state
        self.agents.insert(agent_id.to_string(), new_state);

        Ok(())
    }

    /// Update agent heartbeat
    pub async fn update_heartbeat(&mut self, agent_id: &str) -> RhemaResult<()> {
        if let Some(metadata) = self.agent_metadata.get_mut(agent_id) {
            metadata.last_heartbeat = Some(Utc::now());
            metadata.health = AgentHealth::Healthy;
        }
        Ok(())
    }

    /// Check agent health
    pub async fn check_agent_health(&mut self) -> RhemaResult<()> {
        let now = Utc::now();
        let mut unhealthy_agents = Vec::new();

        for (agent_id, metadata) in &self.agent_metadata {
            if let Some(last_heartbeat) = metadata.last_heartbeat {
                let time_since_heartbeat = now.signed_duration_since(last_heartbeat);
                let heartbeat_secs = self.max_heartbeat_interval.as_secs() as i64;

                if time_since_heartbeat.num_seconds() > heartbeat_secs {
                    unhealthy_agents.push(agent_id.clone());
                }
            }
        }

        // Mark unhealthy agents
        for agent_id in unhealthy_agents {
            if let Some(metadata) = self.agent_metadata.get_mut(&agent_id) {
                metadata.health = AgentHealth::Unhealthy;
            }
        }

        Ok(())
    }

    /// Check agent progress and handle timeouts
    pub async fn check_agent_progress(&mut self) -> RhemaResult<()> {
        let now = Utc::now();
        let mut agents_to_unblock = Vec::new();

        for (agent_id, state) in &self.agents {
            if let AgentState::Blocked = state {
                if let Some(metadata) = self.agent_metadata.get(agent_id) {
                    if let Some(blocked_since) = metadata.blocked_since {
                        let blocked_duration = now.signed_duration_since(blocked_since);
                        if blocked_duration.num_seconds() as u64 > self.max_block_time.as_secs() {
                            agents_to_unblock.push(agent_id.clone());
                        }
                    }
                }
            }
        }

        // Unblock agents that have been blocked too long
        for agent_id in agents_to_unblock {
            self.set_agent_state(&agent_id, AgentState::Idle).await?;
        }

        Ok(())
    }

    /// Persist agent states to disk
    pub async fn persist_state(&self) -> RhemaResult<()> {
        let state_data = AgentStateData {
            agents: self.agents.clone(),
            agent_metadata: self.agent_metadata.clone(),
            state_history: self.state_history.clone(),
            timestamp: Utc::now(),
        };

        let state_dir = Path::new(&self.persistence_config.state_dir);
        if !state_dir.exists() {
            std::fs::create_dir_all(state_dir)
                .map_err(|e| AgentError::PersistenceError(format!("Failed to create state directory: {}", e)))?;
        }

        let filename = format!(
            "agent_states_{}.json",
            state_data.timestamp.format("%Y%m%d_%H%M%S")
        );
        let filepath = state_dir.join(filename);

        let json = serde_json::to_string_pretty(&state_data)
            .map_err(|e| AgentError::PersistenceError(format!("Failed to serialize state: {}", e)))?;

        tokio::fs::write(&filepath, json)
            .await
            .map_err(|e| AgentError::PersistenceError(format!("Failed to write state file: {}", e)))?;

        // Clean up old state files
        self.cleanup_old_state_files().await?;

        Ok(())
    }

    /// Load agent states from disk
    pub async fn load_state(&mut self) -> RhemaResult<()> {
        let state_dir = Path::new(&self.persistence_config.state_dir);
        if !state_dir.exists() {
            return Ok(()); // No state to load
        }

        // Find the most recent state file
        let mut state_files = Vec::new();
        let mut entries = tokio::fs::read_dir(state_dir)
            .await
            .map_err(|e| AgentError::RecoveryError(format!("Failed to read state directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| AgentError::RecoveryError(format!("Failed to read directory entry: {}", e)))?
        {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    state_files.push(entry.path());
                }
            }
        }

        if state_files.is_empty() {
            return Ok(()); // No state files found
        }

        // Sort by modification time and get the most recent
        state_files.sort_by(|a, b| {
            let a_meta = std::fs::metadata(a).unwrap_or_else(|_| std::fs::metadata(".").unwrap());
            let b_meta = std::fs::metadata(b).unwrap_or_else(|_| std::fs::metadata(".").unwrap());
            let a_modified = a_meta.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            let b_modified = b_meta.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            b_modified.cmp(&a_modified)
        });

        let latest_file = &state_files[0];
        let json = tokio::fs::read_to_string(latest_file)
            .await
            .map_err(|e| AgentError::RecoveryError(format!("Failed to read state file: {}", e)))?;

        let state_data: AgentStateData = serde_json::from_str(&json)
            .map_err(|e| AgentError::RecoveryError(format!("Failed to deserialize state: {}", e)))?;

        // Restore state
        self.agents = state_data.agents;
        self.agent_metadata = state_data.agent_metadata;
        self.state_history = state_data.state_history;

        Ok(())
    }

    /// Clean up old state files
    async fn cleanup_old_state_files(&self) -> RhemaResult<()> {
        let state_dir = Path::new(&self.persistence_config.state_dir);
        if !state_dir.exists() {
            return Ok(());
        }

        let mut state_files = Vec::new();
        let mut entries = tokio::fs::read_dir(state_dir)
            .await
            .map_err(|e| AgentError::PersistenceError(format!("Failed to read state directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| AgentError::PersistenceError(format!("Failed to read directory entry: {}", e)))?
        {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    state_files.push(entry.path());
                }
            }
        }

        if state_files.len() <= self.persistence_config.max_state_files {
            return Ok(());
        }

        // Sort by modification time
        state_files.sort_by(|a, b| {
            let a_meta = std::fs::metadata(a).unwrap_or_else(|_| std::fs::metadata(".").unwrap());
            let b_meta = std::fs::metadata(b).unwrap_or_else(|_| std::fs::metadata(".").unwrap());
            let a_modified = a_meta.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            let b_modified = b_meta.modified().unwrap_or_else(|_| std::time::SystemTime::now());
            a_modified.cmp(&b_modified)
        });

        // Remove oldest files
        let files_to_remove = state_files.len() - self.persistence_config.max_state_files;
        for file in state_files.iter().take(files_to_remove) {
            tokio::fs::remove_file(file)
                .await
                .map_err(|e| AgentError::PersistenceError(format!("Failed to remove old state file: {}", e)))?;
        }

        Ok(())
    }

    /// Validate agent state consistency
    pub async fn validate_state(&self) -> RhemaResult<()> {
        for (agent_id, state) in &self.agents {
            // Check if metadata exists for all agents
            if !self.agent_metadata.contains_key(agent_id) {
                return Err(AgentError::ValidationError(format!(
                    "Agent {} has state but no metadata",
                    agent_id
                ))
                .into());
            }

            // Validate state-specific constraints
            match state {
                AgentState::Blocked => {
                    if let Some(metadata) = self.agent_metadata.get(agent_id) {
                        if metadata.blocked_since.is_none() {
                            return Err(AgentError::ValidationError(format!(
                                "Agent {} is blocked but has no blocked_since timestamp",
                                agent_id
                            ))
                            .into());
                        }
                    }
                }
                AgentState::Working => {
                    if let Some(metadata) = self.agent_metadata.get(agent_id) {
                        if metadata.current_scope.is_none() {
                            return Err(AgentError::ValidationError(format!(
                                "Agent {} is working but has no current scope",
                                agent_id
                            ))
                            .into());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Clean up stale agent states
    pub async fn cleanup_stale_states(&mut self) -> RhemaResult<()> {
        let now = Utc::now();
        let mut agents_to_remove = Vec::new();

        for (agent_id, metadata) in &self.agent_metadata {
            // Remove agents that haven't had a heartbeat in too long
            if let Some(last_heartbeat) = metadata.last_heartbeat {
                let time_since_heartbeat = now.signed_duration_since(last_heartbeat);
                let max_heartbeat_secs = (self.max_heartbeat_interval.as_secs() * 3) as i64; // 3x the normal interval

                if time_since_heartbeat.num_seconds() > max_heartbeat_secs {
                    agents_to_remove.push(agent_id.clone());
                }
            }
        }

        // Remove stale agents
        for agent_id in agents_to_remove {
            self.agents.remove(&agent_id);
            self.agent_metadata.remove(&agent_id);
        }

        Ok(())
    }

    /// Get state transition history
    pub fn state_history(&self) -> &[StateTransition] {
        &self.state_history
    }

    /// Get recent state transitions for an agent
    pub fn get_agent_history(&self, agent_id: &str, limit: usize) -> Vec<&StateTransition> {
        self.state_history
            .iter()
            .filter(|transition| transition.agent_id == agent_id)
            .rev()
            .take(limit)
            .collect()
    }

    /// Validate state transition
    fn validate_state_transition(
        &self,
        agent_id: &str,
        current_state: &AgentState,
        new_state: &AgentState,
    ) -> RhemaResult<()> {
        match (current_state, new_state) {
            // Valid transitions
            (AgentState::Idle, AgentState::Working) => Ok(()),
            (AgentState::Idle, AgentState::Completed) => Ok(()),
            (AgentState::Working, AgentState::Idle) => Ok(()),
            (AgentState::Working, AgentState::Blocked) => Ok(()),
            (AgentState::Working, AgentState::Completed) => Ok(()),
            (AgentState::Blocked, AgentState::Idle) => Ok(()),
            (AgentState::Blocked, AgentState::Working) => Ok(()),
            (AgentState::Blocked, AgentState::Completed) => Ok(()),

            // Invalid transitions
            (AgentState::Completed, _) => Err(AgentError::InvalidStateTransition(
                agent_id.to_string(),
                new_state.clone(),
            )
            .into()),
            (_, AgentState::Idle) if current_state == &AgentState::Idle => Err(
                AgentError::InvalidStateTransition(agent_id.to_string(), new_state.clone()).into(),
            ),
            _ => Ok(()),
        }
    }

    /// Record state transition for auditing
    fn record_state_transition(
        &mut self,
        agent_id: &str,
        from_state: Option<AgentState>,
        to_state: AgentState,
        reason: &str,
    ) {
        let transition = StateTransition {
            timestamp: Utc::now(),
            agent_id: agent_id.to_string(),
            from_state,
            to_state,
            reason: reason.to_string(),
        };
        self.state_history.push(transition);
    }

    /// Get statistics about agent states
    pub fn get_statistics(&self) -> AgentStatistics {
        let mut stats = AgentStatistics::default();

        for state in self.agents.values() {
            match state {
                AgentState::Idle => stats.idle_count += 1,
                AgentState::Working => stats.working_count += 1,
                AgentState::Blocked => stats.blocked_count += 1,
                AgentState::Completed => stats.completed_count += 1,
            }
        }

        stats.total_count = self.agents.len();
        stats
    }

    /// Get health statistics
    pub fn get_health_statistics(&self) -> HealthStatistics {
        let mut stats = HealthStatistics::default();

        for metadata in self.agent_metadata.values() {
            match metadata.health {
                AgentHealth::Healthy => stats.healthy_count += 1,
                AgentHealth::Degraded => stats.degraded_count += 1,
                AgentHealth::Unhealthy => stats.unhealthy_count += 1,
                AgentHealth::Unknown => stats.unknown_count += 1,
            }
        }

        stats.total_count = self.agent_metadata.len();
        stats
    }

    /// Clean up old state history
    pub fn cleanup_history(&mut self, max_history_size: usize) {
        if self.state_history.len() > max_history_size {
            self.state_history
                .drain(0..self.state_history.len() - max_history_size);
        }
    }

    /// Start background monitoring tasks
    pub async fn start_monitoring(&mut self) -> RhemaResult<()> {
        let health_interval = self.health_check_interval;
        let cleanup_interval = self.cleanup_interval;
        let persist_interval = self.persistence_config.persist_interval;

        // Health monitoring task
        let mut health_interval_timer = interval(health_interval);
        let mut cleanup_interval_timer = interval(cleanup_interval);
        let mut persist_interval_timer = interval(persist_interval);

        loop {
            tokio::select! {
                _ = health_interval_timer.tick() => {
                    self.check_agent_health().await?;
                    self.check_agent_progress().await?;
                }
                _ = cleanup_interval_timer.tick() => {
                    self.cleanup_stale_states().await?;
                    self.cleanup_history(1000); // Keep last 1000 transitions
                }
                _ = persist_interval_timer.tick() => {
                    self.persist_state().await?;
                }
            }
        }
    }
}

/// Agent state data for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentStateData {
    agents: HashMap<String, AgentState>,
    agent_metadata: HashMap<String, AgentMetadata>,
    state_history: Vec<StateTransition>,
    timestamp: DateTime<Utc>,
}

/// Agent statistics
#[derive(Debug, Clone, Default)]
pub struct AgentStatistics {
    pub total_count: usize,
    pub idle_count: usize,
    pub working_count: usize,
    pub blocked_count: usize,
    pub completed_count: usize,
}

impl std::fmt::Display for AgentStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total: {}, Idle: {}, Working: {}, Blocked: {}, Completed: {}",
            self.total_count,
            self.idle_count,
            self.working_count,
            self.blocked_count,
            self.completed_count
        )
    }
}

/// Health statistics
#[derive(Debug, Clone, Default)]
pub struct HealthStatistics {
    pub total_count: usize,
    pub healthy_count: usize,
    pub degraded_count: usize,
    pub unhealthy_count: usize,
    pub unknown_count: usize,
}

impl std::fmt::Display for HealthStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total: {}, Healthy: {}, Degraded: {}, Unhealthy: {}, Unknown: {}",
            self.total_count,
            self.healthy_count,
            self.degraded_count,
            self.unhealthy_count,
            self.unknown_count
        )
    }
}

impl From<AgentError> for RhemaError {
    fn from(err: AgentError) -> Self {
        RhemaError::AgentError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_join() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60), None);

        // Test successful join
        assert!(manager.agent_join("agent1".to_string()).await.is_ok());
        assert_eq!(manager.get_agent_state("agent1"), Some(AgentState::Idle));

        // Test duplicate join
        assert!(manager.agent_join("agent1".to_string()).await.is_err());

        // Test max concurrent agents
        assert!(manager.agent_join("agent2".to_string()).await.is_ok());
        assert!(manager.agent_join("agent3".to_string()).await.is_ok());
        assert!(manager.agent_join("agent4".to_string()).await.is_err());
    }

    #[tokio::test]
    async fn test_agent_leave() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60), None);

        manager.agent_join("agent1".to_string()).await.unwrap();

        // Test successful leave
        assert!(manager.agent_leave("agent1".to_string()).await.is_ok());
        assert_eq!(manager.get_agent_state("agent1"), None);

        // Test leave non-existent agent
        assert!(manager.agent_leave("agent2".to_string()).await.is_err());
    }

    #[tokio::test]
    async fn test_state_transitions() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60), None);
        manager.agent_join("agent1".to_string()).await.unwrap();

        // Test valid transitions
        assert!(manager
            .set_agent_state("agent1", AgentState::Working)
            .await
            .is_ok());
        assert!(manager
            .set_agent_state("agent1", AgentState::Blocked)
            .await
            .is_ok());
        assert!(manager
            .set_agent_state("agent1", AgentState::Idle)
            .await
            .is_ok());
        assert!(manager
            .set_agent_state("agent1", AgentState::Completed)
            .await
            .is_ok());

        // Test invalid transition from completed
        assert!(manager
            .set_agent_state("agent1", AgentState::Working)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_agent_statistics() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60), None);

        manager.agent_join("agent1".to_string()).await.unwrap();
        manager.agent_join("agent2".to_string()).await.unwrap();

        manager
            .set_agent_state("agent1", AgentState::Working)
            .await
            .unwrap();
        manager
            .set_agent_state("agent2", AgentState::Blocked)
            .await
            .unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_count, 2);
        assert_eq!(stats.working_count, 1);
        assert_eq!(stats.blocked_count, 1);
        assert_eq!(stats.idle_count, 0);
        assert_eq!(stats.completed_count, 0);
    }

    #[tokio::test]
    async fn test_heartbeat_and_health() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60), None);
        manager.agent_join("agent1".to_string()).await.unwrap();

        // Test heartbeat update
        assert!(manager.update_heartbeat("agent1").await.is_ok());

        // Test health check
        assert!(manager.check_agent_health().await.is_ok());

        let metadata = manager.get_agent_metadata("agent1").unwrap();
        assert_eq!(metadata.health, AgentHealth::Healthy);
    }

    #[tokio::test]
    async fn test_persistence() {
        let config = PersistenceConfig {
            state_dir: "/tmp/rhema_test_states".to_string(),
            persist_interval: Duration::from_secs(1),
            max_state_files: 2,
            enable_compression: false,
        };

        let mut manager = AgentManager::new(3, Duration::from_secs(60), Some(config.clone()));
        manager.agent_join("agent1".to_string()).await.unwrap();
        manager
            .set_agent_state("agent1", AgentState::Working)
            .await
            .unwrap();

        // Test persistence
        assert!(manager.persist_state().await.is_ok());

        // Create new manager and load state
        let mut new_manager = AgentManager::new(3, Duration::from_secs(60), Some(config));
        assert!(new_manager.load_state().await.is_ok());

        // Verify state was restored
        assert_eq!(new_manager.get_agent_state("agent1"), Some(AgentState::Working));
    }
}
