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

use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Lock event types for auditing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockEventType {
    Acquired,
    Released,
    Timeout,
    ForceReleased,
}

/// Lock event for auditing
#[derive(Debug, Clone)]
pub struct LockEvent {
    pub timestamp: DateTime<Utc>,
    pub scope_path: String,
    pub agent_id: String,
    pub event_type: LockEventType,
    pub reason: Option<String>,
}

/// Lock information
#[derive(Debug, Clone)]
pub struct LockInfo {
    pub agent_id: String,
    pub acquired_at: DateTime<Utc>,
    pub expires_at: Instant,
    pub scope_path: String,
}

/// Lock manager for handling resource locks
pub struct LockManager {
    /// Map of scope path to agent ID (None means unlocked)
    locks: HashMap<String, Option<String>>,
    
    /// Detailed lock information
    lock_info: HashMap<String, LockInfo>,
    
    /// Lock event history for auditing
    lock_history: Vec<LockEvent>,
    
    /// Lock timeout duration
    lock_timeout: Duration,
    
    /// Maximum number of locks per agent
    max_locks_per_agent: usize,
}

/// Lock-related errors
#[derive(Debug, Error)]
pub enum LockError {
    #[error("Lock already held by agent: {0}")]
    LockAlreadyHeld(String),
    
    #[error("Lock not held by agent: {0}")]
    LockNotHeld(String),
    
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Scope not found: {0}")]
    ScopeNotFound(String),
    
    #[error("Lock timeout: {0}")]
    LockTimeout(String),
    
    #[error("Too many locks per agent: {0}")]
    TooManyLocksPerAgent(String),
    
    #[error("Lock consistency violation: {0}")]
    LockConsistencyViolation(String),
}

impl LockManager {
    /// Create a new lock manager
    pub fn new(lock_timeout: Duration) -> Self {
        Self {
            locks: HashMap::new(),
            lock_info: HashMap::new(),
            lock_history: Vec::new(),
            lock_timeout,
            max_locks_per_agent: 10, // Allow multiple locks per agent
        }
    }

    /// Get all locks
    pub fn locks(&self) -> &HashMap<String, Option<String>> {
        &self.locks
    }

    /// Get active locks (locks that are currently held)
    pub fn active_locks(&self) -> HashMap<String, String> {
        self.locks
            .iter()
            .filter_map(|(scope, agent_id)| agent_id.as_ref().map(|id| (scope.clone(), id.clone())))
            .collect()
    }

    /// Get locks held by a specific agent
    pub fn get_agent_locks(&self, agent_id: &str) -> Vec<String> {
        self.locks
            .iter()
            .filter_map(|(scope, lock_agent_id)| {
                if lock_agent_id.as_deref() == Some(agent_id) {
                    Some(scope.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if an agent has any locks
    pub fn has_agent_locks(&self, agent_id: &str) -> bool {
        self.locks.values().any(|lock_agent_id| lock_agent_id.as_deref() == Some(agent_id))
    }

    /// Get lock information for a scope
    pub fn get_lock_info(&self, scope_path: &str) -> Option<&LockInfo> {
        self.lock_info.get(scope_path)
    }

    /// Acquire a lock for a scope
    pub async fn acquire_lock(&mut self, scope_path: &str, agent_id: &str) -> RhemaResult<bool> {
        // Check if lock is already held
        if let Some(current_agent) = self.locks.get(scope_path) {
            if let Some(current_agent_id) = current_agent {
                if current_agent_id == agent_id {
                    // Agent already holds this lock
                    return Ok(true);
                } else {
                    // Lock held by different agent
                    return Ok(false);
                }
            }
        }

        // Check if agent already has too many locks
        let agent_locks = self.get_agent_locks(agent_id);
        if agent_locks.len() >= self.max_locks_per_agent {
            return Err(LockError::TooManyLocksPerAgent(agent_id.to_string()).into());
        }

        // Acquire the lock
        self.locks.insert(scope_path.to_string(), Some(agent_id.to_string()));
        
        // Create lock info
        let lock_info = LockInfo {
            agent_id: agent_id.to_string(),
            acquired_at: Utc::now(),
            expires_at: Instant::now() + self.lock_timeout,
            scope_path: scope_path.to_string(),
        };
        self.lock_info.insert(scope_path.to_string(), lock_info);

        // Record lock event
        self.record_lock_event(scope_path, agent_id, LockEventType::Acquired, None);

        Ok(true)
    }

    /// Release a lock for a scope
    pub async fn release_lock(&mut self, scope_path: &str, agent_id: &str) -> RhemaResult<()> {
        // Check if lock is held by this agent
        let current_agent = self.locks.get(scope_path)
            .ok_or_else(|| LockError::ScopeNotFound(scope_path.to_string()))?;

        match current_agent {
            Some(current_agent_id) if current_agent_id == agent_id => {
                // Release the lock
                self.locks.insert(scope_path.to_string(), None);
                self.lock_info.remove(scope_path);

                // Record lock event
                self.record_lock_event(scope_path, agent_id, LockEventType::Released, None);

                Ok(())
            }
            Some(current_agent_id) => {
                Err(LockError::LockNotHeld(current_agent_id.clone()).into())
            }
            None => {
                Err(LockError::LockNotHeld("no agent".to_string()).into())
            }
        }
    }

    /// Release all locks held by an agent
    pub async fn release_agent_locks(&mut self, agent_id: &str) -> RhemaResult<()> {
        let agent_locks = self.get_agent_locks(agent_id);
        
        for scope_path in agent_locks {
            self.release_lock(&scope_path, agent_id).await?;
        }

        Ok(())
    }

    /// Force release a lock (for cleanup or error recovery)
    pub async fn force_release_lock(&mut self, scope_path: &str, reason: &str) -> RhemaResult<()> {
        if let Some(agent_id) = self.locks.get(scope_path).and_then(|id| id.as_ref()) {
            let agent_id = agent_id.clone();
            
            // Release the lock
            self.locks.insert(scope_path.to_string(), None);
            self.lock_info.remove(scope_path);

            // Record force release event
            self.record_lock_event(scope_path, &agent_id, LockEventType::ForceReleased, Some(reason.to_string()));
        }

        Ok(())
    }

    /// Check if a lock is held
    pub fn is_locked(&self, scope_path: &str) -> bool {
        self.locks.get(scope_path).map_or(false, |agent_id| agent_id.is_some())
    }

    /// Check if a lock is held by a specific agent
    pub fn is_locked_by(&self, scope_path: &str, agent_id: &str) -> bool {
        self.locks.get(scope_path).map_or(false, |lock_agent_id| {
            lock_agent_id.as_deref() == Some(agent_id)
        })
    }

    /// Get the agent holding a lock
    pub fn get_lock_holder(&self, scope_path: &str) -> Option<&String> {
        self.locks.get(scope_path).and_then(|agent_id| agent_id.as_ref())
    }

    /// Cleanup expired locks
    pub async fn cleanup_expired_locks(&mut self) -> RhemaResult<()> {
        let now = Instant::now();
        let mut expired_locks = Vec::new();

        // Find expired locks
        for (scope_path, lock_info) in &self.lock_info {
            if lock_info.expires_at <= now {
                expired_locks.push(scope_path.clone());
            }
        }

        // Release expired locks
        for scope_path in expired_locks {
            if let Some(agent_id) = self.locks.get(&scope_path).and_then(|id| id.as_ref()) {
                let agent_id = agent_id.clone();
                
                // Release the lock
                self.locks.insert(scope_path.clone(), None);
                self.lock_info.remove(&scope_path);

                // Record timeout event
                self.record_lock_event(&scope_path, &agent_id, LockEventType::Timeout, Some("Lock expired".to_string()));
            }
        }

        Ok(())
    }

    /// Extend lock timeout
    pub async fn extend_lock_timeout(&mut self, scope_path: &str, agent_id: &str) -> RhemaResult<()> {
        if let Some(lock_info) = self.lock_info.get_mut(scope_path) {
            if lock_info.agent_id == agent_id {
                lock_info.expires_at = Instant::now() + self.lock_timeout;
                return Ok(());
            }
        }

        Err(LockError::LockNotHeld(agent_id.to_string()).into())
    }

    /// Get lock statistics
    pub fn get_lock_statistics(&self) -> LockStatistics {
        let mut stats = LockStatistics::default();
        
        stats.total_scopes = self.locks.len();
        stats.locked_scopes = self.locks.values().filter(|agent_id| agent_id.is_some()).count();
        stats.unlocked_scopes = stats.total_scopes - stats.locked_scopes;
        
        // Count locks per agent
        let mut agent_lock_counts = HashMap::new();
        for agent_id in self.locks.values().filter_map(|id| id.as_ref()) {
            *agent_lock_counts.entry(agent_id.clone()).or_insert(0) += 1;
        }
        
        stats.max_locks_per_agent = agent_lock_counts.values().max().copied().unwrap_or(0);
        stats.agents_with_locks = agent_lock_counts.len();
        
        stats
    }

    /// Get lock history
    pub fn lock_history(&self) -> &[LockEvent] {
        &self.lock_history
    }

    /// Get recent lock events for a scope
    pub fn get_scope_history(&self, scope_path: &str, limit: usize) -> Vec<&LockEvent> {
        self.lock_history
            .iter()
            .filter(|event| event.scope_path == scope_path)
            .rev()
            .take(limit)
            .collect()
    }

    /// Get recent lock events for an agent
    pub fn get_agent_history(&self, agent_id: &str, limit: usize) -> Vec<&LockEvent> {
        self.lock_history
            .iter()
            .filter(|event| event.agent_id == agent_id)
            .rev()
            .take(limit)
            .collect()
    }

    /// Clean up old lock history
    pub fn cleanup_history(&mut self, max_history_size: usize) {
        if self.lock_history.len() > max_history_size {
            self.lock_history.drain(0..self.lock_history.len() - max_history_size);
        }
    }

    /// Record lock event for auditing
    fn record_lock_event(&mut self, scope_path: &str, agent_id: &str, event_type: LockEventType, reason: Option<String>) {
        let event = LockEvent {
            timestamp: Utc::now(),
            scope_path: scope_path.to_string(),
            agent_id: agent_id.to_string(),
            event_type,
            reason,
        };
        self.lock_history.push(event);
    }

    /// Validate lock consistency
    pub fn validate_consistency(&self) -> RhemaResult<()> {
        // Check that lock_info matches locks
        for (scope_path, agent_id) in &self.locks {
            match agent_id {
                Some(agent_id) => {
                    // Should have lock info
                    if !self.lock_info.contains_key(scope_path) {
                        return Err(LockError::LockConsistencyViolation(
                            format!("Lock info missing for scope {}", scope_path)
                        ).into());
                    }
                    
                    // Agent ID should match
                    let lock_info = &self.lock_info[scope_path];
                    if lock_info.agent_id != *agent_id {
                        return Err(LockError::LockConsistencyViolation(
                            format!("Agent ID mismatch for scope {}", scope_path)
                        ).into());
                    }
                }
                None => {
                    // Should not have lock info
                    if self.lock_info.contains_key(scope_path) {
                        return Err(LockError::LockConsistencyViolation(
                            format!("Lock info exists for unlocked scope {}", scope_path)
                        ).into());
                    }
                }
            }
        }

        // Check that all lock_info entries have corresponding locks
        for scope_path in self.lock_info.keys() {
            if !self.locks.contains_key(scope_path) {
                return Err(LockError::LockConsistencyViolation(
                    format!("Lock info exists for non-existent scope {}", scope_path)
                ).into());
            }
        }

        Ok(())
    }
}

/// Lock statistics
#[derive(Debug, Clone, Default)]
pub struct LockStatistics {
    pub total_scopes: usize,
    pub locked_scopes: usize,
    pub unlocked_scopes: usize,
    pub max_locks_per_agent: usize,
    pub agents_with_locks: usize,
}

impl std::fmt::Display for LockStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Total: {}, Locked: {}, Unlocked: {}, Max per agent: {}, Agents with locks: {}", 
            self.total_scopes, self.locked_scopes, self.unlocked_scopes, self.max_locks_per_agent, self.agents_with_locks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_acquire_lock() {
        let mut manager = LockManager::new(Duration::from_secs(60));
        
        // Test successful lock acquisition
        assert!(manager.acquire_lock("scope1", "agent1").await.unwrap());
        assert!(manager.is_locked("scope1"));
        assert!(manager.is_locked_by("scope1", "agent1"));
        
        // Test duplicate acquisition by same agent
        assert!(manager.acquire_lock("scope1", "agent1").await.unwrap());
        
        // Test acquisition by different agent
        assert!(!manager.acquire_lock("scope1", "agent2").await.unwrap());
    }

    #[tokio::test]
    async fn test_release_lock() {
        let mut manager = LockManager::new(Duration::from_secs(60));
        
        manager.acquire_lock("scope1", "agent1").await.unwrap();
        
        // Test successful release
        assert!(manager.release_lock("scope1", "agent1").await.is_ok());
        assert!(!manager.is_locked("scope1"));
        
        // Test release by wrong agent
        assert!(manager.release_lock("scope1", "agent2").await.is_err());
    }

    #[tokio::test]
    async fn test_agent_locks() {
        let mut manager = LockManager::new(Duration::from_secs(60));
        
        manager.acquire_lock("scope1", "agent1").await.unwrap();
        manager.acquire_lock("scope2", "agent1").await.unwrap();
        
        let agent_locks = manager.get_agent_locks("agent1");
        assert_eq!(agent_locks.len(), 2);
        assert!(agent_locks.contains(&"scope1".to_string()));
        assert!(agent_locks.contains(&"scope2".to_string()));
        
        assert!(manager.has_agent_locks("agent1"));
        assert!(!manager.has_agent_locks("agent2"));
    }

    #[tokio::test]
    async fn test_release_agent_locks() {
        let mut manager = LockManager::new(Duration::from_secs(60));
        
        manager.acquire_lock("scope1", "agent1").await.unwrap();
        manager.acquire_lock("scope2", "agent1").await.unwrap();
        
        assert!(manager.release_agent_locks("agent1").await.is_ok());
        assert!(!manager.has_agent_locks("agent1"));
        assert!(!manager.is_locked("scope1"));
        assert!(!manager.is_locked("scope2"));
    }

    #[tokio::test]
    async fn test_lock_consistency() {
        let mut manager = LockManager::new(Duration::from_secs(60));
        
        manager.acquire_lock("scope1", "agent1").await.unwrap();
        assert!(manager.validate_consistency().is_ok());
        
        // Manually corrupt the state
        manager.locks.insert("scope1".to_string(), None);
        assert!(manager.validate_consistency().is_err());
    }

    #[tokio::test]
    async fn test_lock_statistics() {
        let mut manager = LockManager::new(Duration::from_secs(60));
        
        manager.acquire_lock("scope1", "agent1").await.unwrap();
        manager.acquire_lock("scope2", "agent2").await.unwrap();
        
        let stats = manager.get_lock_statistics();
        assert_eq!(stats.total_scopes, 2);
        assert_eq!(stats.locked_scopes, 2);
        assert_eq!(stats.unlocked_scopes, 0);
        assert_eq!(stats.max_locks_per_agent, 1);
        assert_eq!(stats.agents_with_locks, 2);
    }
} 