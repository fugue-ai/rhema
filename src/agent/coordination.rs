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
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Sync status as defined in the TLA+ specification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Completed,
    Failed,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncStatus::Idle => write!(f, "idle"),
            SyncStatus::Syncing => write!(f, "syncing"),
            SyncStatus::Completed => write!(f, "completed"),
            SyncStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Sync operation for queue management
#[derive(Debug, Clone)]
pub struct SyncOperation {
    pub scope_path: String,
    pub priority: SyncPriority,
    pub created_at: DateTime<Utc>,
    pub dependencies: Vec<String>,
    pub retry_count: usize,
}

/// Sync priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Sync event for auditing
#[derive(Debug, Clone)]
pub struct SyncEvent {
    pub timestamp: DateTime<Utc>,
    pub scope_path: String,
    pub from_status: Option<SyncStatus>,
    pub to_status: SyncStatus,
    pub reason: String,
    pub error: Option<String>,
}

/// Sync coordinator for managing cross-scope synchronization
pub struct SyncCoordinator {
    /// Map of scope path to sync status
    sync_status: HashMap<String, SyncStatus>,
    
    /// Map of scope path to its dependencies
    sync_dependencies: HashMap<String, Vec<String>>,
    
    /// Queue of pending sync operations
    sync_queue: VecDeque<SyncOperation>,
    
    /// Sync event history for auditing
    sync_history: Vec<SyncEvent>,
    
    /// Maximum retry attempts for failed syncs
    max_retry_attempts: usize,
    
    /// Maximum sync queue size
    max_queue_size: usize,
}

/// Sync-related errors
#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Scope not found: {0}")]
    ScopeNotFound(String),
    
    #[error("Invalid sync status transition: {0} -> {1:?}")]
    InvalidStatusTransition(String, SyncStatus),
    
    #[error("Dependencies not ready: {0}")]
    DependenciesNotReady(String),
    
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),
    
    #[error("Sync queue full: {0}")]
    SyncQueueFull(usize),
    
    #[error("Max retry attempts exceeded: {0}")]
    MaxRetryAttemptsExceeded(String),
    
    #[error("Sync already in progress: {0}")]
    SyncAlreadyInProgress(String),
}

impl SyncCoordinator {
    /// Create a new sync coordinator
    pub fn new() -> Self {
        Self {
            sync_status: HashMap::new(),
            sync_dependencies: HashMap::new(),
            sync_queue: VecDeque::new(),
            sync_history: Vec::new(),
            max_retry_attempts: 3,
            max_queue_size: 1000,
        }
    }

    /// Get sync status for all scopes
    pub fn sync_status(&self) -> &HashMap<String, SyncStatus> {
        &self.sync_status
    }

    /// Get sync dependencies
    pub fn sync_dependencies(&self) -> &HashMap<String, Vec<String>> {
        &self.sync_dependencies
    }

    /// Get scopes currently syncing
    pub fn syncing_scopes(&self) -> Vec<String> {
        self.sync_status
            .iter()
            .filter_map(|(scope, status)| {
                if *status == SyncStatus::Syncing {
                    Some(scope.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get scopes with completed syncs
    pub fn completed_syncs(&self) -> Vec<String> {
        self.sync_status
            .iter()
            .filter_map(|(scope, status)| {
                if *status == SyncStatus::Completed {
                    Some(scope.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get scopes with failed syncs
    pub fn failed_syncs(&self) -> Vec<String> {
        self.sync_status
            .iter()
            .filter_map(|(scope, status)| {
                if *status == SyncStatus::Failed {
                    Some(scope.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get sync status for a specific scope
    pub fn get_sync_status(&self, scope_path: &str) -> Option<SyncStatus> {
        self.sync_status.get(scope_path).cloned()
    }

    /// Get dependencies for a scope
    pub fn get_dependencies(&self, scope_path: &str) -> Vec<String> {
        self.sync_dependencies.get(scope_path).cloned().unwrap_or_default()
    }

    /// Add a scope to the sync system
    pub fn add_scope(&mut self, scope_path: String, dependencies: Vec<String>) -> RhemaResult<()> {
        // Validate dependencies exist
        for dep in &dependencies {
            if !self.sync_status.contains_key(dep) {
                return Err(SyncError::ScopeNotFound(dep.clone()).into());
            }
        }

        // Check for circular dependencies
        if self.has_circular_dependency(&scope_path, &dependencies) {
            return Err(SyncError::CircularDependency(scope_path).into());
        }

        // Add scope with idle status
        self.sync_status.insert(scope_path.clone(), SyncStatus::Idle);
        self.sync_dependencies.insert(scope_path, dependencies);

        Ok(())
    }

    /// Remove a scope from the sync system
    pub fn remove_scope(&mut self, scope_path: &str) -> RhemaResult<()> {
        // Check if scope is currently syncing
        if let Some(status) = self.sync_status.get(scope_path) {
            if *status == SyncStatus::Syncing {
                return Err(SyncError::SyncAlreadyInProgress(scope_path.to_string()).into());
            }
        }

        // Remove from all dependency lists
        for deps in self.sync_dependencies.values_mut() {
            deps.retain(|dep| dep != scope_path);
        }

        // Remove scope
        self.sync_status.remove(scope_path);
        self.sync_dependencies.remove(scope_path);

        // Remove from queue
        self.sync_queue.retain(|op| op.scope_path != scope_path);

        Ok(())
    }

    /// Start sync for a scope
    pub async fn start_sync(&mut self, scope_path: &str) -> RhemaResult<()> {
        let current_status = self.sync_status.get(scope_path)
            .ok_or_else(|| SyncError::ScopeNotFound(scope_path.to_string()))?;

        // Validate status transition
        if *current_status != SyncStatus::Idle {
            return Err(SyncError::InvalidStatusTransition(
                scope_path.to_string(),
                current_status.clone()
            ).into());
        }

        // Check dependencies
        if !self.check_dependencies_ready(scope_path) {
            return Err(SyncError::DependenciesNotReady(scope_path.to_string()).into());
        }

        // Update status
        self.record_sync_event(scope_path, Some(current_status.clone()), SyncStatus::Syncing, "Sync started".to_string(), None);
        self.sync_status.insert(scope_path.to_string(), SyncStatus::Syncing);

        Ok(())
    }

    /// Complete sync for a scope
    pub async fn complete_sync(&mut self, scope_path: &str) -> RhemaResult<()> {
        let current_status = self.sync_status.get(scope_path)
            .ok_or_else(|| SyncError::ScopeNotFound(scope_path.to_string()))?;

        // Validate status transition
        if *current_status != SyncStatus::Syncing {
            return Err(SyncError::InvalidStatusTransition(
                scope_path.to_string(),
                current_status.clone()
            ).into());
        }

        // Update status
        self.record_sync_event(scope_path, Some(current_status.clone()), SyncStatus::Completed, "Sync completed".to_string(), None);
        self.sync_status.insert(scope_path.to_string(), SyncStatus::Completed);

        // Process queue for dependent scopes
        self.process_queue_for_dependents(scope_path).await?;

        Ok(())
    }

    /// Fail sync for a scope
    pub async fn fail_sync(&mut self, scope_path: &str, error: String) -> RhemaResult<()> {
        let current_status = self.sync_status.get(scope_path)
            .ok_or_else(|| SyncError::ScopeNotFound(scope_path.to_string()))?;

        // Validate status transition
        if *current_status != SyncStatus::Syncing {
            return Err(SyncError::InvalidStatusTransition(
                scope_path.to_string(),
                current_status.clone()
            ).into());
        }

        // Update status
        self.record_sync_event(scope_path, Some(current_status.clone()), SyncStatus::Failed, "Sync failed".to_string(), Some(error.clone()));
        self.sync_status.insert(scope_path.to_string(), SyncStatus::Failed);

        // Check if we should retry
        if let Some(operation) = self.find_queue_operation(scope_path) {
            if operation.retry_count < self.max_retry_attempts {
                // Retry the sync
                let mut new_operation = operation.clone();
                new_operation.retry_count += 1;
                self.add_to_queue(new_operation).await?;
            }
        }

        Ok(())
    }

    /// Reset sync status to idle (for retry)
    pub async fn reset_sync(&mut self, scope_path: &str) -> RhemaResult<()> {
        let current_status = self.sync_status.get(scope_path)
            .ok_or_else(|| SyncError::ScopeNotFound(scope_path.to_string()))?;

        // Only allow reset from failed status
        if *current_status != SyncStatus::Failed {
            return Err(SyncError::InvalidStatusTransition(
                scope_path.to_string(),
                current_status.clone()
            ).into());
        }

        // Update status
        self.record_sync_event(scope_path, Some(current_status.clone()), SyncStatus::Idle, "Sync reset".to_string(), None);
        self.sync_status.insert(scope_path.to_string(), SyncStatus::Idle);

        Ok(())
    }

    /// Check if dependencies are ready for a scope
    pub async fn check_sync_dependencies(&self, scope_path: &str) -> RhemaResult<bool> {
        Ok(self.check_dependencies_ready(scope_path))
    }

    /// Add sync operation to queue
    pub async fn queue_sync(&mut self, scope_path: String, priority: SyncPriority) -> RhemaResult<()> {
        let dependencies = self.get_dependencies(&scope_path);
        
        let operation = SyncOperation {
            scope_path: scope_path.clone(),
            priority,
            created_at: Utc::now(),
            dependencies,
            retry_count: 0,
        };

        self.add_to_queue(operation).await?;
        Ok(())
    }

    /// Get next sync operation from queue
    pub fn get_next_sync_operation(&mut self) -> Option<SyncOperation> {
        // Find the highest priority operation with ready dependencies
        let mut best_operation: Option<(usize, SyncOperation)> = None;

        for (index, operation) in self.sync_queue.iter().enumerate() {
            if self.check_dependencies_ready(&operation.scope_path) {
                if let Some((_, ref best)) = best_operation {
                    if operation.priority > best.priority {
                        best_operation = Some((index, operation.clone()));
                    }
                } else {
                    best_operation = Some((index, operation.clone()));
                }
            }
        }

        // Remove and return the best operation
        if let Some((index, operation)) = best_operation {
            self.sync_queue.remove(index);
            Some(operation)
        } else {
            None
        }
    }

    /// Get sync statistics
    pub fn get_sync_statistics(&self) -> SyncStatistics {
        let mut stats = SyncStatistics::default();
        
        for status in self.sync_status.values() {
            match status {
                SyncStatus::Idle => stats.idle_count += 1,
                SyncStatus::Syncing => stats.syncing_count += 1,
                SyncStatus::Completed => stats.completed_count += 1,
                SyncStatus::Failed => stats.failed_count += 1,
            }
        }
        
        stats.total_scopes = self.sync_status.len();
        stats.queue_size = self.sync_queue.len();
        
        stats
    }

    /// Get sync history
    pub fn sync_history(&self) -> &[SyncEvent] {
        &self.sync_history
    }

    /// Get recent sync events for a scope
    pub fn get_scope_history(&self, scope_path: &str, limit: usize) -> Vec<&SyncEvent> {
        self.sync_history
            .iter()
            .filter(|event| event.scope_path == scope_path)
            .rev()
            .take(limit)
            .collect()
    }

    /// Clean up old sync history
    pub fn cleanup_history(&mut self, max_history_size: usize) {
        if self.sync_history.len() > max_history_size {
            self.sync_history.drain(0..self.sync_history.len() - max_history_size);
        }
    }

    /// Check if dependencies are ready for a scope
    fn check_dependencies_ready(&self, scope_path: &str) -> bool {
        let dependencies = self.get_dependencies(scope_path);
        
        for dep in dependencies {
            if let Some(status) = self.sync_status.get(&dep) {
                if *status != SyncStatus::Completed {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }

    /// Check for circular dependencies
    fn has_circular_dependency(&self, scope_path: &str, dependencies: &[String]) -> bool {
        // Create a temporary graph with the new dependencies
        let mut temp_graph = self.sync_dependencies.clone();
        temp_graph.insert(scope_path.to_string(), dependencies.to_vec());
        
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        // Check if adding this scope creates a cycle
        for node in temp_graph.keys() {
            if !visited.contains(node) {
                if self.dfs_check_cycle_with_graph(&temp_graph, node, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        
        false
    }

    /// DFS to check for cycles in dependency graph
    fn dfs_check_cycle(&self, scope_path: &str, visited: &mut std::collections::HashSet<String>, rec_stack: &mut std::collections::HashSet<String>) -> bool {
        if rec_stack.contains(scope_path) {
            return true; // Back edge found - cycle detected
        }
        
        if visited.contains(scope_path) {
            return false; // Already processed
        }
        
        visited.insert(scope_path.to_string());
        rec_stack.insert(scope_path.to_string());
        
        let dependencies = self.get_dependencies(scope_path);
        for dep in dependencies {
            if self.dfs_check_cycle(&dep, visited, rec_stack) {
                return true;
            }
        }
        
        rec_stack.remove(scope_path);
        false
    }

    /// DFS to check for cycles in a given graph
    fn dfs_check_cycle_with_graph(&self, graph: &HashMap<String, Vec<String>>, scope_path: &str, visited: &mut std::collections::HashSet<String>, rec_stack: &mut std::collections::HashSet<String>) -> bool {
        if rec_stack.contains(scope_path) {
            return true; // Back edge found - cycle detected
        }
        
        if visited.contains(scope_path) {
            return false; // Already processed
        }
        
        visited.insert(scope_path.to_string());
        rec_stack.insert(scope_path.to_string());
        
        if let Some(dependencies) = graph.get(scope_path) {
            for dep in dependencies {
                if self.dfs_check_cycle_with_graph(graph, dep, visited, rec_stack) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(scope_path);
        false
    }

    /// Process queue for dependent scopes
    async fn process_queue_for_dependents(&mut self, completed_scope: &str) -> RhemaResult<()> {
        // Find scopes that depend on the completed scope
        let mut dependent_scopes = Vec::new();
        
        for (scope_path, deps) in &self.sync_dependencies {
            if deps.contains(&completed_scope.to_string()) {
                dependent_scopes.push(scope_path.clone());
            }
        }

        // Try to start sync for dependent scopes
        for scope_path in dependent_scopes {
            if self.check_dependencies_ready(&scope_path) {
                if let Some(status) = self.sync_status.get(&scope_path) {
                    if *status == SyncStatus::Idle {
                        self.start_sync(&scope_path).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Find operation in queue
    fn find_queue_operation(&self, scope_path: &str) -> Option<&SyncOperation> {
        self.sync_queue.iter().find(|op| op.scope_path == scope_path)
    }

    /// Add operation to queue
    async fn add_to_queue(&mut self, operation: SyncOperation) -> RhemaResult<()> {
        if self.sync_queue.len() >= self.max_queue_size {
            return Err(SyncError::SyncQueueFull(self.max_queue_size).into());
        }

        self.sync_queue.push_back(operation);
        Ok(())
    }

    /// Record sync event for auditing
    fn record_sync_event(&mut self, scope_path: &str, from_status: Option<SyncStatus>, to_status: SyncStatus, reason: String, error: Option<String>) {
        let event = SyncEvent {
            timestamp: Utc::now(),
            scope_path: scope_path.to_string(),
            from_status,
            to_status,
            reason,
            error,
        };
        self.sync_history.push(event);
    }
}

/// Sync statistics
#[derive(Debug, Clone, Default)]
pub struct SyncStatistics {
    pub total_scopes: usize,
    pub idle_count: usize,
    pub syncing_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub queue_size: usize,
}

impl std::fmt::Display for SyncStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Total: {}, Idle: {}, Syncing: {}, Completed: {}, Failed: {}, Queue: {}", 
            self.total_scopes, self.idle_count, self.syncing_count, self.completed_count, self.failed_count, self.queue_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_scope() {
        let mut coordinator = SyncCoordinator::new();
        
        // Test adding scope without dependencies
        assert!(coordinator.add_scope("scope1".to_string(), vec![]).is_ok());
        assert_eq!(coordinator.get_sync_status("scope1"), Some(SyncStatus::Idle));
        
        // Test adding scope with dependencies
        assert!(coordinator.add_scope("scope2".to_string(), vec!["scope1".to_string()]).is_ok());
        
        // Test adding scope with non-existent dependency
        assert!(coordinator.add_scope("scope3".to_string(), vec!["nonexistent".to_string()]).is_err());
    }

    #[tokio::test]
    async fn test_circular_dependency() {
        let mut coordinator = SyncCoordinator::new();
        
        coordinator.add_scope("scope1".to_string(), vec![]).unwrap();
        coordinator.add_scope("scope2".to_string(), vec!["scope1".to_string()]).unwrap();
        
        // Test circular dependency
        assert!(coordinator.add_scope("scope1".to_string(), vec!["scope2".to_string()]).is_err());
    }

    #[tokio::test]
    async fn test_sync_lifecycle() {
        let mut coordinator = SyncCoordinator::new();
        
        coordinator.add_scope("scope1".to_string(), vec![]).unwrap();
        
        // Test start sync
        assert!(coordinator.start_sync("scope1").await.is_ok());
        assert_eq!(coordinator.get_sync_status("scope1"), Some(SyncStatus::Syncing));
        
        // Test complete sync
        assert!(coordinator.complete_sync("scope1").await.is_ok());
        assert_eq!(coordinator.get_sync_status("scope1"), Some(SyncStatus::Completed));
    }

    #[tokio::test]
    async fn test_dependency_checking() {
        let mut coordinator = SyncCoordinator::new();
        
        coordinator.add_scope("scope1".to_string(), vec![]).unwrap();
        coordinator.add_scope("scope2".to_string(), vec!["scope1".to_string()]).unwrap();
        
        // scope2 should not be able to start sync until scope1 is completed
        assert!(coordinator.start_sync("scope2").await.is_err());
        
        // Complete scope1
        coordinator.start_sync("scope1").await.unwrap();
        coordinator.complete_sync("scope1").await.unwrap();
        
        // Check scope2 status - it should have been automatically started
        let scope2_status = coordinator.get_sync_status("scope2");
        println!("Scope2 status after completing scope1: {:?}", scope2_status);
        
        // scope2 should have been automatically started when scope1 completed
        assert_eq!(scope2_status, Some(SyncStatus::Syncing));
        
        // Try to start scope2 again - this should fail since it's already syncing
        let result = coordinator.start_sync("scope2").await;
        assert!(result.is_err());
        
        // Complete scope2
        coordinator.complete_sync("scope2").await.unwrap();
        assert_eq!(coordinator.get_sync_status("scope2"), Some(SyncStatus::Completed));
    }

    #[tokio::test]
    async fn test_sync_queue() {
        let mut coordinator = SyncCoordinator::new();
        
        coordinator.add_scope("scope1".to_string(), vec![]).unwrap();
        coordinator.add_scope("scope2".to_string(), vec!["scope1".to_string()]).unwrap();
        
        // Queue sync operations
        assert!(coordinator.queue_sync("scope1".to_string(), SyncPriority::Normal).await.is_ok());
        assert!(coordinator.queue_sync("scope2".to_string(), SyncPriority::High).await.is_ok());
        
        // Get next operation (should be scope1 since scope2 depends on it)
        let operation = coordinator.get_next_sync_operation();
        assert!(operation.is_some());
        assert_eq!(operation.unwrap().scope_path, "scope1");
    }

    #[tokio::test]
    async fn test_sync_statistics() {
        let mut coordinator = SyncCoordinator::new();
        
        coordinator.add_scope("scope1".to_string(), vec![]).unwrap();
        coordinator.add_scope("scope2".to_string(), vec![]).unwrap();
        
        coordinator.start_sync("scope1").await.unwrap();
        coordinator.complete_sync("scope1").await.unwrap();
        coordinator.start_sync("scope2").await.unwrap();
        coordinator.fail_sync("scope2", "Test error".to_string()).await.unwrap();
        
        let stats = coordinator.get_sync_statistics();
        assert_eq!(stats.total_scopes, 2);
        assert_eq!(stats.completed_count, 1);
        assert_eq!(stats.failed_count, 1);
        assert_eq!(stats.syncing_count, 0);
        assert_eq!(stats.idle_count, 0);
    }
} 