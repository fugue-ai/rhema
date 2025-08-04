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

use crate::agent::{AgentId, AgentState, AgentType, AgentCapability};
use crate::error::{AgentError, AgentResult};
use crate::registry::AgentRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Coordination policy for agent interactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationPolicy {
    /// Whether to enable automatic coordination
    pub enable_automatic: bool,
    /// Maximum coordination sessions
    pub max_sessions: usize,
    /// Session timeout in seconds
    pub session_timeout: u64,
    /// Whether to enable conflict resolution
    pub enable_conflict_resolution: bool,
    /// Whether to enable load balancing
    pub enable_load_balancing: bool,
    /// Coordination strategy
    pub strategy: CoordinationStrategy,
    /// Custom policy parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

impl Default for CoordinationPolicy {
    fn default() -> Self {
        Self {
            enable_automatic: true,
            max_sessions: 100,
            session_timeout: 300, // 5 minutes
            enable_conflict_resolution: true,
            enable_load_balancing: true,
            strategy: CoordinationStrategy::RoundRobin,
            parameters: HashMap::new(),
        }
    }
}

/// Coordination strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationStrategy {
    /// Round-robin coordination
    RoundRobin,
    /// Load-based coordination
    LoadBased,
    /// Priority-based coordination
    PriorityBased,
    /// Capability-based coordination
    CapabilityBased,
    /// Custom coordination strategy
    Custom(String),
}

impl std::fmt::Display for CoordinationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinationStrategy::RoundRobin => write!(f, "RoundRobin"),
            CoordinationStrategy::LoadBased => write!(f, "LoadBased"),
            CoordinationStrategy::PriorityBased => write!(f, "PriorityBased"),
            CoordinationStrategy::CapabilityBased => write!(f, "CapabilityBased"),
            CoordinationStrategy::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Coordination result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationResult {
    /// Coordination ID
    pub coordination_id: String,
    /// Participating agents
    pub agents: Vec<AgentId>,
    /// Coordination status
    pub status: CoordinationStatus,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Result data
    pub result: Option<serde_json::Value>,
    /// Error message if any
    pub error: Option<String>,
}

/// Coordination status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationStatus {
    /// Coordination is pending
    Pending,
    /// Coordination is in progress
    InProgress,
    /// Coordination completed successfully
    Completed,
    /// Coordination failed
    Failed,
    /// Coordination was cancelled
    Cancelled,
}

impl std::fmt::Display for CoordinationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinationStatus::Pending => write!(f, "Pending"),
            CoordinationStatus::InProgress => write!(f, "InProgress"),
            CoordinationStatus::Completed => write!(f, "Completed"),
            CoordinationStatus::Failed => write!(f, "Failed"),
            CoordinationStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Coordination session
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationSession {
    /// Session ID
    pub session_id: String,
    /// Session topic
    pub topic: String,
    /// Participating agents
    pub participants: Vec<AgentId>,
    /// Session status
    pub status: CoordinationStatus,
    /// Session policy
    pub policy: CoordinationPolicy,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Session data
    pub data: HashMap<String, serde_json::Value>,
}

impl CoordinationSession {
    pub fn new(topic: String, participants: Vec<AgentId>, policy: CoordinationPolicy) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            topic,
            participants,
            status: CoordinationStatus::Pending,
            policy,
            start_time: Utc::now(),
            end_time: None,
            data: HashMap::new(),
        }
    }

    pub fn add_data(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, CoordinationStatus::Pending | CoordinationStatus::InProgress)
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, CoordinationStatus::Completed)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.status, CoordinationStatus::Failed | CoordinationStatus::Cancelled)
    }
}

/// Agent coordinator for managing agent interactions
#[derive(Clone)]
pub struct AgentCoordinator {
    /// Agent registry
    registry: AgentRegistry,
    /// Active coordination sessions
    sessions: Arc<RwLock<HashMap<String, CoordinationSession>>>,
    /// Coordination policy
    policy: CoordinationPolicy,
    /// Coordination statistics
    stats: Arc<RwLock<CoordinationStats>>,
}

/// Coordination statistics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationStats {
    /// Total coordination sessions
    pub total_sessions: usize,
    /// Active sessions
    pub active_sessions: usize,
    /// Completed sessions
    pub completed_sessions: usize,
    /// Failed sessions
    pub failed_sessions: usize,
    /// Average session duration in seconds
    pub avg_session_duration: u64,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl Default for CoordinationStats {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            active_sessions: 0,
            completed_sessions: 0,
            failed_sessions: 0,
            avg_session_duration: 0,
            last_update: Utc::now(),
        }
    }
}

impl AgentCoordinator {
    pub fn new() -> Self {
        Self {
            registry: AgentRegistry::new(),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            policy: CoordinationPolicy::default(),
            stats: Arc::new(RwLock::new(CoordinationStats::default())),
        }
    }

    /// Initialize the coordinator
    pub async fn initialize(&self) -> AgentResult<()> {
        // Start session monitoring
        self.start_session_monitoring().await;
        
        Ok(())
    }

    /// Register an agent with the coordinator
    pub async fn register_agent(&self, agent_id: &AgentId) -> AgentResult<()> {
        // For now, just log the registration
        // In a full implementation, this would update coordination state
        tracing::debug!("Agent {} registered with coordinator", agent_id);
        
        Ok(())
    }

    /// Agent started notification
    pub async fn agent_started(&self, agent_id: &AgentId) -> AgentResult<()> {
        // Update any sessions that include this agent
        let mut sessions = self.sessions.write().await;
        
        for session in sessions.values_mut() {
            if session.participants.contains(agent_id) {
                // Check if all participants are now ready
                if self.all_participants_ready(&session.participants).await {
                    session.status = CoordinationStatus::InProgress;
                }
            }
        }
        
        Ok(())
    }

    /// Agent stopped notification
    pub async fn agent_stopped(&self, agent_id: &AgentId) -> AgentResult<()> {
        // Update any sessions that include this agent
        let mut sessions = self.sessions.write().await;
        
        for session in sessions.values_mut() {
            if session.participants.contains(agent_id) && session.is_active() {
                session.status = CoordinationStatus::Failed;
                session.end_time = Some(Utc::now());
            }
        }
        
        // Update statistics
        self.update_stats().await;
        
        Ok(())
    }

    /// Create a coordination session
    pub async fn create_session(
        &self,
        topic: String,
        participants: Vec<AgentId>,
        policy: Option<CoordinationPolicy>,
    ) -> AgentResult<String> {
        // Validate participants
        for agent_id in &participants {
            if !self.registry.get_agent(agent_id).await.is_ok() {
                return Err(AgentError::AgentNotFound {
                    agent_id: agent_id.clone(),
                });
            }
        }
        
        // Check session limits
        let sessions = self.sessions.read().await;
        if sessions.len() >= self.policy.max_sessions {
            return Err(AgentError::CoordinationFailed {
                reason: "Maximum number of coordination sessions reached".to_string(),
            });
        }
        
        // Create session
        let session_policy = policy.unwrap_or_else(|| self.policy.clone());
        let session = CoordinationSession::new(topic, participants, session_policy);
        let session_id = session.session_id.clone();
        
        // Add session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.clone(), session);
        }
        
        // Update statistics
        self.update_stats().await;
        
        Ok(session_id)
    }

    /// Get coordination session
    pub async fn get_session(&self, session_id: &str) -> AgentResult<CoordinationSession> {
        let sessions = self.sessions.read().await;
        
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| AgentError::CoordinationFailed {
                reason: "Session not found".to_string(),
            })
    }

    /// Update session status
    pub async fn update_session_status(
        &self,
        session_id: &str,
        status: CoordinationStatus,
    ) -> AgentResult<()> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = status;
            
            if session.is_completed() || session.is_failed() {
                session.end_time = Some(Utc::now());
            }
            
            // Update statistics
            self.update_stats().await;
            
            Ok(())
        } else {
            Err(AgentError::CoordinationFailed {
                reason: "Session not found".to_string(),
            })
        }
    }

    /// Add data to session
    pub async fn add_session_data(
        &self,
        session_id: &str,
        key: String,
        value: serde_json::Value,
    ) -> AgentResult<()> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.add_data(key, value);
            Ok(())
        } else {
            Err(AgentError::CoordinationFailed {
                reason: "Session not found".to_string(),
            })
        }
    }

    /// Get session data
    pub async fn get_session_data(&self, session_id: &str, key: &str) -> AgentResult<Option<serde_json::Value>> {
        let sessions = self.sessions.read().await;
        
        if let Some(session) = sessions.get(session_id) {
            Ok(session.get_data(key).cloned())
        } else {
            Err(AgentError::CoordinationFailed {
                reason: "Session not found".to_string(),
            })
        }
    }

    /// Cancel coordination session
    pub async fn cancel_session(&self, session_id: &str) -> AgentResult<()> {
        self.update_session_status(session_id, CoordinationStatus::Cancelled).await
    }

    /// Get all active sessions
    pub async fn get_active_sessions(&self) -> Vec<CoordinationSession> {
        let sessions = self.sessions.read().await;
        
        sessions
            .values()
            .filter(|session| session.is_active())
            .cloned()
            .collect()
    }

    /// Get sessions for an agent
    pub async fn get_agent_sessions(&self, agent_id: &AgentId) -> Vec<CoordinationSession> {
        let sessions = self.sessions.read().await;
        
        sessions
            .values()
            .filter(|session| session.participants.contains(agent_id))
            .cloned()
            .collect()
    }

    /// Get session count
    pub async fn get_session_count(&self) -> usize {
        self.sessions.read().await.len()
    }

    /// Get coordination statistics
    pub async fn get_stats(&self) -> CoordinationStats {
        self.stats.read().await.clone()
    }

    /// Set coordination policy
    pub fn set_policy(&mut self, policy: CoordinationPolicy) {
        self.policy = policy;
    }

    /// Get coordination policy
    pub fn get_policy(&self) -> &CoordinationPolicy {
        &self.policy
    }

    /// Start session monitoring
    async fn start_session_monitoring(&self) {
        let sessions = self.sessions.clone();
        let stats = self.stats.clone();
        let timeout = self.policy.session_timeout;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                let now = Utc::now();
                let mut sessions_guard = sessions.write().await;
                let mut updated = false;
                
                // Check for timed out sessions
                for session in sessions_guard.values_mut() {
                    if session.is_active() {
                        let duration = now - session.start_time;
                        if duration.num_seconds() > timeout as i64 {
                            session.status = CoordinationStatus::Failed;
                            session.end_time = Some(now);
                            updated = true;
                        }
                    }
                }
                
                if updated {
                    // Update statistics
                    let mut stats_guard = stats.write().await;
                    stats_guard.active_sessions = sessions_guard.values().filter(|s| s.is_active()).count();
                    stats_guard.failed_sessions = sessions_guard.values().filter(|s| s.is_failed()).count();
                    stats_guard.completed_sessions = sessions_guard.values().filter(|s| s.is_completed()).count();
                    stats_guard.last_update = now;
                }
            }
        });
    }

    /// Check if all participants are ready
    async fn all_participants_ready(&self, participants: &[AgentId]) -> bool {
        for agent_id in participants {
            if let Ok(agent) = self.registry.get_agent(agent_id).await {
                let agent_guard = agent.read().await;
                if agent_guard.context().state != AgentState::Ready {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Update coordination statistics
    async fn update_stats(&self) {
        let sessions = self.sessions.read().await;
        let mut stats = self.stats.write().await;
        
        stats.total_sessions = sessions.len();
        stats.active_sessions = sessions.values().filter(|s| s.is_active()).count();
        stats.completed_sessions = sessions.values().filter(|s| s.is_completed()).count();
        stats.failed_sessions = sessions.values().filter(|s| s.is_failed()).count();
        stats.last_update = Utc::now();
        
        // Calculate average session duration
        let completed_sessions: Vec<_> = sessions.values().filter(|s| s.is_completed()).collect();
        if !completed_sessions.is_empty() {
            let total_duration: i64 = completed_sessions
                .iter()
                .filter_map(|s| s.end_time.map(|end| (end - s.start_time).num_seconds()))
                .sum();
            stats.avg_session_duration = (total_duration / completed_sessions.len() as i64) as u64;
        }
    }

    /// Shutdown the coordinator
    pub async fn shutdown(&self) -> AgentResult<()> {
        // Cancel all active sessions
        let mut sessions = self.sessions.write().await;
        for session in sessions.values_mut() {
            if session.is_active() {
                session.status = CoordinationStatus::Cancelled;
                session.end_time = Some(Utc::now());
            }
        }
        
        // Clear all sessions
        sessions.clear();
        
        // Reset statistics
        let mut stats = self.stats.write().await;
        *stats = CoordinationStats::default();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{BaseAgent, AgentConfig, AgentType, AgentCapability};

    #[tokio::test]
    async fn test_coordinator_creation() {
        let coordinator = AgentCoordinator::new();
        assert_eq!(coordinator.get_session_count().await, 0);
    }

    #[tokio::test]
    async fn test_session_creation() {
        let coordinator = AgentCoordinator::new();
        coordinator.initialize().await.unwrap();
        
        let participants = vec!["agent1".to_string(), "agent2".to_string()];
        let session_id = coordinator.create_session(
            "test-topic".to_string(),
            participants,
            None,
        ).await.unwrap();
        
        assert!(!session_id.is_empty());
        assert_eq!(coordinator.get_session_count().await, 1);
    }

    #[tokio::test]
    async fn test_session_status_update() {
        let coordinator = AgentCoordinator::new();
        coordinator.initialize().await.unwrap();
        
        let participants = vec!["agent1".to_string(), "agent2".to_string()];
        let session_id = coordinator.create_session(
            "test-topic".to_string(),
            participants,
            None,
        ).await.unwrap();
        
        assert!(coordinator.update_session_status(&session_id, CoordinationStatus::InProgress).await.is_ok());
        
        let session = coordinator.get_session(&session_id).await.unwrap();
        assert_eq!(session.status, CoordinationStatus::InProgress);
    }

    #[test]
    fn test_coordination_policy_default() {
        let policy = CoordinationPolicy::default();
        assert!(policy.enable_automatic);
        assert_eq!(policy.max_sessions, 100);
        assert_eq!(policy.session_timeout, 300);
    }

    #[test]
    fn test_coordination_strategy_display() {
        assert_eq!(CoordinationStrategy::RoundRobin.to_string(), "RoundRobin");
        assert_eq!(CoordinationStrategy::LoadBased.to_string(), "LoadBased");
    }
} 