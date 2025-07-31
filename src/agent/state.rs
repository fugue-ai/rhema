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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc};
use thiserror::Error;

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
            custom_data: HashMap::new(),
        }
    }
}

/// Agent manager for handling agent state transitions
pub struct AgentManager {
    /// Map of agent ID to current state
    agents: HashMap<String, AgentState>,
    
    /// Map of agent ID to metadata
    agent_metadata: HashMap<String, AgentMetadata>,
    
    /// Maximum number of concurrent agents allowed
    max_concurrent_agents: usize,
    
    /// Maximum time an agent can be blocked
    max_block_time: Duration,
    
    /// State transition history for auditing
    state_history: Vec<StateTransition>,
}

/// State transition record for auditing
#[derive(Debug, Clone)]
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
}

impl AgentManager {
    /// Create a new agent manager
    pub fn new(max_concurrent_agents: usize, max_block_time: Duration) -> Self {
        Self {
            agents: HashMap::new(),
            agent_metadata: HashMap::new(),
            max_concurrent_agents,
            max_block_time,
            state_history: Vec::new(),
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
        let metadata = AgentMetadata::default();
        self.agent_metadata.insert(agent_id.clone(), metadata);

        // Record state transition
        self.record_state_transition(&agent_id, None, AgentState::Idle, "Agent joined system");

        Ok(())
    }

    /// Agent leaves the system
    pub async fn agent_leave(&mut self, agent_id: String) -> RhemaResult<()> {
        let current_state = self.agents.get(&agent_id)
            .ok_or_else(|| AgentError::AgentNotFound(agent_id.clone()))?;

        // Check if agent can leave (not working or blocked)
        if matches!(current_state, AgentState::Working | AgentState::Blocked) {
            return Err(AgentError::AgentHasActiveLocks(agent_id).into());
        }

        // Record state transition before removal
        self.record_state_transition(&agent_id, Some(current_state.clone()), AgentState::Completed, "Agent left system");

        // Remove agent
        self.agents.remove(&agent_id);
        self.agent_metadata.remove(&agent_id);

        Ok(())
    }

    /// Set agent state
    pub async fn set_agent_state(&mut self, agent_id: &str, new_state: AgentState) -> RhemaResult<()> {
        let current_state = self.agents.get(agent_id)
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
        self.record_state_transition(agent_id, Some(current_state.clone()), new_state.clone(), "State change");

        // Update state
        self.agents.insert(agent_id.to_string(), new_state);

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
    fn validate_state_transition(&self, agent_id: &str, current_state: &AgentState, new_state: &AgentState) -> RhemaResult<()> {
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
            (AgentState::Completed, _) => {
                Err(AgentError::InvalidStateTransition(agent_id.to_string(), new_state.clone()).into())
            }
            (_, AgentState::Idle) if current_state == &AgentState::Idle => {
                Err(AgentError::InvalidStateTransition(agent_id.to_string(), new_state.clone()).into())
            }
            _ => Ok(()),
        }
    }

    /// Record state transition for auditing
    fn record_state_transition(&mut self, agent_id: &str, from_state: Option<AgentState>, to_state: AgentState, reason: &str) {
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

    /// Clean up old state history
    pub fn cleanup_history(&mut self, max_history_size: usize) {
        if self.state_history.len() > max_history_size {
            self.state_history.drain(0..self.state_history.len() - max_history_size);
        }
    }
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
        write!(f, "Total: {}, Idle: {}, Working: {}, Blocked: {}, Completed: {}", 
            self.total_count, self.idle_count, self.working_count, self.blocked_count, self.completed_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_join() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60));
        
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
        let mut manager = AgentManager::new(3, Duration::from_secs(60));
        
        manager.agent_join("agent1".to_string()).await.unwrap();
        
        // Test successful leave
        assert!(manager.agent_leave("agent1".to_string()).await.is_ok());
        assert_eq!(manager.get_agent_state("agent1"), None);
        
        // Test leave non-existent agent
        assert!(manager.agent_leave("agent2".to_string()).await.is_err());
    }

    #[tokio::test]
    async fn test_state_transitions() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60));
        manager.agent_join("agent1".to_string()).await.unwrap();
        
        // Test valid transitions
        assert!(manager.set_agent_state("agent1", AgentState::Working).await.is_ok());
        assert!(manager.set_agent_state("agent1", AgentState::Blocked).await.is_ok());
        assert!(manager.set_agent_state("agent1", AgentState::Idle).await.is_ok());
        assert!(manager.set_agent_state("agent1", AgentState::Completed).await.is_ok());
        
        // Test invalid transition from completed
        assert!(manager.set_agent_state("agent1", AgentState::Working).await.is_err());
    }

    #[tokio::test]
    async fn test_agent_statistics() {
        let mut manager = AgentManager::new(3, Duration::from_secs(60));
        
        manager.agent_join("agent1".to_string()).await.unwrap();
        manager.agent_join("agent2".to_string()).await.unwrap();
        
        manager.set_agent_state("agent1", AgentState::Working).await.unwrap();
        manager.set_agent_state("agent2", AgentState::Blocked).await.unwrap();
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_count, 2);
        assert_eq!(stats.working_count, 1);
        assert_eq!(stats.blocked_count, 1);
        assert_eq!(stats.idle_count, 0);
        assert_eq!(stats.completed_count, 0);
    }
} 