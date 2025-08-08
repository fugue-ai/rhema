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

use crate::agent::{Agent, AgentId, AgentState};
use crate::error::{AgentError, AgentResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Agent lifecycle states
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum LifecycleState {
    /// Agent is being created
    Creating,
    /// Agent is initializing
    Initializing,
    /// Agent is ready to start
    Ready,
    /// Agent is starting
    Starting,
    /// Agent is running
    Running,
    /// Agent is stopping
    Stopping,
    /// Agent is stopped
    Stopped,
    /// Agent is restarting
    Restarting,
    /// Agent is in error state
    Error,
    /// Agent is being destroyed
    Destroying,
    /// Agent is destroyed
    Destroyed,
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LifecycleState::Creating => write!(f, "Creating"),
            LifecycleState::Initializing => write!(f, "Initializing"),
            LifecycleState::Ready => write!(f, "Ready"),
            LifecycleState::Starting => write!(f, "Starting"),
            LifecycleState::Running => write!(f, "Running"),
            LifecycleState::Stopping => write!(f, "Stopping"),
            LifecycleState::Stopped => write!(f, "Stopped"),
            LifecycleState::Restarting => write!(f, "Restarting"),
            LifecycleState::Error => write!(f, "Error"),
            LifecycleState::Destroying => write!(f, "Destroying"),
            LifecycleState::Destroyed => write!(f, "Destroyed"),
        }
    }
}

/// Lifecycle events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleEvent {
    /// Agent created
    Created {
        agent_id: AgentId,
        timestamp: DateTime<Utc>,
    },
    /// Agent initialized
    Initialized {
        agent_id: AgentId,
        timestamp: DateTime<Utc>,
    },
    /// Agent started
    Started {
        agent_id: AgentId,
        timestamp: DateTime<Utc>,
    },
    /// Agent stopped
    Stopped {
        agent_id: AgentId,
        timestamp: DateTime<Utc>,
    },
    /// Agent restarted
    Restarted {
        agent_id: AgentId,
        timestamp: DateTime<Utc>,
    },
    /// Agent error occurred
    Error {
        agent_id: AgentId,
        error: String,
        timestamp: DateTime<Utc>,
    },
    /// Agent destroyed
    Destroyed {
        agent_id: AgentId,
        timestamp: DateTime<Utc>,
    },
    /// Custom lifecycle event
    Custom {
        agent_id: AgentId,
        event_type: String,
        data: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
}

impl LifecycleEvent {
    pub fn agent_id(&self) -> &AgentId {
        match self {
            LifecycleEvent::Created { agent_id, .. } => agent_id,
            LifecycleEvent::Initialized { agent_id, .. } => agent_id,
            LifecycleEvent::Started { agent_id, .. } => agent_id,
            LifecycleEvent::Stopped { agent_id, .. } => agent_id,
            LifecycleEvent::Restarted { agent_id, .. } => agent_id,
            LifecycleEvent::Error { agent_id, .. } => agent_id,
            LifecycleEvent::Destroyed { agent_id, .. } => agent_id,
            LifecycleEvent::Custom { agent_id, .. } => agent_id,
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            LifecycleEvent::Created { timestamp, .. } => *timestamp,
            LifecycleEvent::Initialized { timestamp, .. } => *timestamp,
            LifecycleEvent::Started { timestamp, .. } => *timestamp,
            LifecycleEvent::Stopped { timestamp, .. } => *timestamp,
            LifecycleEvent::Restarted { timestamp, .. } => *timestamp,
            LifecycleEvent::Error { timestamp, .. } => *timestamp,
            LifecycleEvent::Destroyed { timestamp, .. } => *timestamp,
            LifecycleEvent::Custom { timestamp, .. } => *timestamp,
        }
    }
}

/// Lifecycle transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleTransition {
    /// From state
    pub from: LifecycleState,
    /// To state
    pub to: LifecycleState,
    /// Transition timestamp
    pub timestamp: DateTime<Utc>,
    /// Transition reason
    pub reason: Option<String>,
    /// Transition metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl LifecycleTransition {
    pub fn new(from: LifecycleState, to: LifecycleState) -> Self {
        Self {
            from,
            to,
            timestamp: Utc::now(),
            reason: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Agent lifecycle manager
pub struct AgentLifecycle {
    /// Current state
    state: LifecycleState,
    /// State transitions history
    transitions: Vec<LifecycleTransition>,
    /// Lifecycle events
    events: Vec<LifecycleEvent>,
    /// State change callbacks
    callbacks: HashMap<LifecycleState, Vec<Box<dyn Fn(&AgentId, &LifecycleState) + Send + Sync>>>,
    /// Agent ID
    agent_id: AgentId,
}

impl AgentLifecycle {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            state: LifecycleState::Creating,
            transitions: Vec::new(),
            events: Vec::new(),
            callbacks: HashMap::new(),
            agent_id,
        }
    }

    /// Get current state
    pub fn current_state(&self) -> &LifecycleState {
        &self.state
    }

    /// Check if agent is in a specific state
    pub fn is_in_state(&self, state: &LifecycleState) -> bool {
        &self.state == state
    }

    /// Check if agent is running
    pub fn is_running(&self) -> bool {
        self.is_in_state(&LifecycleState::Running)
    }

    /// Check if agent is stopped
    pub fn is_stopped(&self) -> bool {
        self.is_in_state(&LifecycleState::Stopped)
    }

    /// Check if agent is in error state
    pub fn is_error(&self) -> bool {
        self.is_in_state(&LifecycleState::Error)
    }

    /// Transition to a new state
    pub async fn transition_to(&mut self, new_state: LifecycleState) -> AgentResult<()> {
        self.transition_to_with_reason(new_state, None).await
    }

    /// Transition to a new state with reason
    pub async fn transition_to_with_reason(
        &mut self,
        new_state: LifecycleState,
        reason: Option<String>,
    ) -> AgentResult<()> {
        let old_state = self.state.clone();

        // Validate transition
        if !self.is_valid_transition(&old_state, &new_state) {
            return Err(AgentError::LifecycleError {
                reason: format!("Invalid transition from {:?} to {:?}", old_state, new_state),
            });
        }

        // Create transition
        let transition = LifecycleTransition::new(old_state.clone(), new_state.clone())
            .with_reason(reason.unwrap_or_else(|| "State transition".to_string()));

        // Update state
        self.state = new_state.clone();
        self.transitions.push(transition);

        // Create and emit event
        let event = self.create_event_for_transition(&old_state, &new_state);
        self.events.push(event.clone());

        // Notify callbacks
        self.notify_callbacks(&new_state).await;

        Ok(())
    }

    /// Add a callback for state changes
    pub fn add_state_change_callback<F>(&mut self, state: LifecycleState, callback: F)
    where
        F: Fn(&AgentId, &LifecycleState) + Send + Sync + 'static,
    {
        self.callbacks
            .entry(state)
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    /// Get transition history
    pub fn get_transitions(&self) -> &[LifecycleTransition] {
        &self.transitions
    }

    /// Get lifecycle events
    pub fn get_events(&self) -> &[LifecycleEvent] {
        &self.events
    }

    /// Get recent events
    pub fn get_recent_events(&self, limit: usize) -> Vec<&LifecycleEvent> {
        self.events.iter().rev().take(limit).collect()
    }

    /// Get events for a specific agent
    pub fn get_events_for_agent(&self, agent_id: &AgentId) -> Vec<&LifecycleEvent> {
        self.events
            .iter()
            .filter(|event| event.agent_id() == agent_id)
            .collect()
    }

    /// Clear old events
    pub fn clear_old_events(&mut self, older_than: DateTime<Utc>) {
        self.events.retain(|event| event.timestamp() >= older_than);
    }

    /// Get lifecycle statistics
    pub fn get_lifecycle_stats(&self) -> LifecycleStats {
        let total_transitions = self.transitions.len();
        let total_events = self.events.len();
        let current_state_duration = if let Some(last_transition) = self.transitions.last() {
            (Utc::now() - last_transition.timestamp).num_seconds() as u64
        } else {
            0
        };

        let state_counts = self
            .transitions
            .iter()
            .fold(HashMap::new(), |mut acc, transition| {
                *acc.entry(transition.to.clone()).or_insert(0) += 1;
                acc
            });

        LifecycleStats {
            current_state: self.state.clone(),
            total_transitions,
            total_events,
            current_state_duration,
            state_counts,
            last_update: Utc::now(),
        }
    }

    /// Check if transition is valid
    fn is_valid_transition(&self, from: &LifecycleState, to: &LifecycleState) -> bool {
        match (from, to) {
            // Valid transitions
            (LifecycleState::Creating, LifecycleState::Initializing) => true,
            (LifecycleState::Initializing, LifecycleState::Ready) => true,
            (LifecycleState::Initializing, LifecycleState::Error) => true,
            (LifecycleState::Ready, LifecycleState::Starting) => true,
            (LifecycleState::Starting, LifecycleState::Running) => true,
            (LifecycleState::Starting, LifecycleState::Error) => true,
            (LifecycleState::Running, LifecycleState::Stopping) => true,
            (LifecycleState::Running, LifecycleState::Error) => true,
            (LifecycleState::Stopping, LifecycleState::Stopped) => true,
            (LifecycleState::Stopped, LifecycleState::Starting) => true,
            (LifecycleState::Stopped, LifecycleState::Destroying) => true,
            (LifecycleState::Error, LifecycleState::Starting) => true,
            (LifecycleState::Error, LifecycleState::Destroying) => true,
            (LifecycleState::Destroying, LifecycleState::Destroyed) => true,
            // Self-transitions (no-op)
            (from, to) if from == to => true,
            // Invalid transitions
            _ => false,
        }
    }

    /// Create event for transition
    fn create_event_for_transition(
        &self,
        from: &LifecycleState,
        to: &LifecycleState,
    ) -> LifecycleEvent {
        match to {
            LifecycleState::Initializing => LifecycleEvent::Created {
                agent_id: self.agent_id.clone(),
                timestamp: Utc::now(),
            },
            LifecycleState::Ready => LifecycleEvent::Initialized {
                agent_id: self.agent_id.clone(),
                timestamp: Utc::now(),
            },
            LifecycleState::Running => LifecycleEvent::Started {
                agent_id: self.agent_id.clone(),
                timestamp: Utc::now(),
            },
            LifecycleState::Stopped => LifecycleEvent::Stopped {
                agent_id: self.agent_id.clone(),
                timestamp: Utc::now(),
            },
            LifecycleState::Error => LifecycleEvent::Error {
                agent_id: self.agent_id.clone(),
                error: "State transition error".to_string(),
                timestamp: Utc::now(),
            },
            LifecycleState::Destroyed => LifecycleEvent::Destroyed {
                agent_id: self.agent_id.clone(),
                timestamp: Utc::now(),
            },
            _ => LifecycleEvent::Custom {
                agent_id: self.agent_id.clone(),
                event_type: format!("transition_to_{:?}", to).to_lowercase(),
                data: serde_json::json!({
                    "from": format!("{:?}", from),
                    "to": format!("{:?}", to)
                }),
                timestamp: Utc::now(),
            },
        }
    }

    /// Notify callbacks for state change
    async fn notify_callbacks(&self, state: &LifecycleState) {
        if let Some(callbacks) = self.callbacks.get(state) {
            for callback in callbacks {
                callback(&self.agent_id, state);
            }
        }
    }
}

/// Lifecycle statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleStats {
    /// Current state
    pub current_state: LifecycleState,
    /// Total transitions
    pub total_transitions: usize,
    /// Total events
    pub total_events: usize,
    /// Current state duration in seconds
    pub current_state_duration: u64,
    /// State transition counts
    pub state_counts: HashMap<LifecycleState, usize>,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl std::fmt::Display for LifecycleStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State: {} | Transitions: {} | Events: {} | Duration: {}s",
            self.current_state,
            self.total_transitions,
            self.total_events,
            self.current_state_duration
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lifecycle_creation() {
        let lifecycle = AgentLifecycle::new("test-agent".to_string());
        assert_eq!(lifecycle.current_state(), &LifecycleState::Creating);
    }

    #[tokio::test]
    async fn test_valid_transition() {
        let mut lifecycle = AgentLifecycle::new("test-agent".to_string());
        assert!(lifecycle
            .transition_to(LifecycleState::Initializing)
            .await
            .is_ok());
        assert_eq!(lifecycle.current_state(), &LifecycleState::Initializing);
    }

    #[tokio::test]
    async fn test_invalid_transition() {
        let mut lifecycle = AgentLifecycle::new("test-agent".to_string());
        assert!(lifecycle
            .transition_to(LifecycleState::Running)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_lifecycle_stats() {
        let mut lifecycle = AgentLifecycle::new("test-agent".to_string());
        lifecycle
            .transition_to(LifecycleState::Initializing)
            .await
            .unwrap();
        lifecycle
            .transition_to(LifecycleState::Ready)
            .await
            .unwrap();

        let stats = lifecycle.get_lifecycle_stats();
        assert_eq!(stats.total_transitions, 2);
        assert_eq!(stats.current_state, LifecycleState::Ready);
    }

    #[test]
    fn test_lifecycle_state_display() {
        assert_eq!(LifecycleState::Running.to_string(), "Running");
        assert_eq!(LifecycleState::Error.to_string(), "Error");
    }
}
