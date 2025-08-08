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

//! Conflict resolution system for agent coordination

use crate::error::ConflictError;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use syneidesis_core::types::ConflictId;
use tracing::info;

/// Conflict resolution strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictStrategy {
    /// Automatically merge conflicts
    AutoMerge,

    /// Keep local changes
    KeepLocal,

    /// Keep remote changes
    KeepRemote,

    /// Require manual resolution
    Manual,

    /// Use last writer wins
    LastWriterWins,

    /// Use custom handler
    Custom { handler_name: String },
}

/// Conflict types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Agent state conflict
    AgentState,

    /// Task assignment conflict
    TaskAssignment,

    /// Resource conflict
    Resource,

    /// Configuration conflict
    Configuration,

    /// Communication conflict
    Communication,

    /// Custom conflict type
    Custom { conflict_type: String },
}

/// Conflict severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConflictSeverity {
    /// Low severity conflict
    Low,

    /// Medium severity conflict
    Medium,

    /// High severity conflict
    High,

    /// Critical severity conflict
    Critical,
}

/// Conflict representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    /// Unique conflict ID
    pub id: ConflictId,

    /// Conflict type
    pub conflict_type: ConflictType,

    /// Conflict severity
    pub severity: ConflictSeverity,

    /// Conflict description
    pub description: String,

    /// Local state
    pub local_state: serde_json::Value,

    /// Remote state
    pub remote_state: serde_json::Value,

    /// Conflict creation time
    pub created_at: DateTime<Utc>,

    /// Conflict resolution time
    pub resolved_at: Option<DateTime<Utc>>,

    /// Resolution strategy used
    pub resolution_strategy: Option<ConflictStrategy>,

    /// Resolution result
    pub resolution_result: Option<serde_json::Value>,

    /// Conflict metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Conflict {
    /// Create a new conflict
    pub fn new(
        conflict_type: ConflictType,
        severity: ConflictSeverity,
        description: String,
        local_state: serde_json::Value,
        remote_state: serde_json::Value,
    ) -> Self {
        Self {
            id: ConflictId::generate(),
            conflict_type,
            severity,
            description,
            local_state,
            remote_state,
            created_at: Utc::now(),
            resolved_at: None,
            resolution_strategy: None,
            resolution_result: None,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the conflict
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if the conflict is resolved
    pub fn is_resolved(&self) -> bool {
        self.resolved_at.is_some()
    }

    /// Get the age of the conflict
    pub fn age(&self) -> std::time::Duration {
        let now = Utc::now();
        now.signed_duration_since(self.created_at)
            .to_std()
            .unwrap_or_default()
    }
}

/// Result of conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    /// Whether resolution was successful
    pub success: bool,

    /// Resolution strategy used
    pub strategy: ConflictStrategy,

    /// Resolved state
    pub resolved_state: serde_json::Value,

    /// Resolution message
    pub message: String,

    /// Resolution timestamp
    pub timestamp: DateTime<Utc>,

    /// Resolution metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ResolutionResult {
    /// Create a successful resolution result
    pub fn success(
        strategy: ConflictStrategy,
        resolved_state: serde_json::Value,
        message: String,
    ) -> Self {
        Self {
            success: true,
            strategy,
            resolved_state,
            message,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a failed resolution result
    pub fn failure(strategy: ConflictStrategy, message: String) -> Self {
        Self {
            success: false,
            strategy,
            resolved_state: serde_json::Value::Null,
            message,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

/// Conflict resolution record for history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRecord {
    /// Conflict ID
    pub conflict_id: ConflictId,

    /// Conflict type
    pub conflict_type: ConflictType,

    /// Resolution strategy used
    pub resolution_strategy: ConflictStrategy,

    /// Resolution success
    pub resolution_success: bool,

    /// Resolution time in milliseconds
    pub resolution_time_ms: u64,

    /// Record timestamp
    pub timestamp: DateTime<Utc>,
}

/// Trait for conflict handlers
pub trait ConflictHandler: Send + Sync {
    /// Handle conflict resolution
    fn resolve_conflict<'a>(
        &'a self,
        conflict: &'a Conflict,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<ResolutionResult, ConflictError>> + Send + 'a>,
    >;

    /// Get handler name
    fn name(&self) -> &str;

    /// Get supported conflict types
    fn supported_conflict_types(&self) -> Vec<ConflictType>;
}

/// Main conflict resolver
pub struct ConflictResolver {
    /// Resolution strategy
    strategy: ConflictStrategy,

    /// Active conflicts
    conflicts: Arc<RwLock<HashMap<ConflictId, Conflict>>>,

    /// Conflict history
    conflict_history: Arc<RwLock<Vec<ConflictRecord>>>,

    /// Custom conflict handlers
    resolution_handlers: Arc<RwLock<HashMap<String, Box<dyn ConflictHandler + Send + Sync>>>>,

    /// Statistics
    statistics: Arc<RwLock<ConflictStatistics>>,
}

/// Conflict resolution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictStatistics {
    /// Total conflicts detected
    pub total_conflicts: u64,

    /// Total conflicts resolved
    pub total_resolved: u64,

    /// Total conflicts failed
    pub total_failed: u64,

    /// Average resolution time in milliseconds
    pub avg_resolution_time_ms: u64,

    /// Resolution success rate
    pub success_rate: f64,

    /// Conflicts by type
    pub conflicts_by_type: HashMap<String, u64>,

    /// Conflicts by strategy
    pub conflicts_by_strategy: HashMap<String, u64>,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for ConflictStatistics {
    fn default() -> Self {
        Self {
            total_conflicts: 0,
            total_resolved: 0,
            total_failed: 0,
            avg_resolution_time_ms: 0,
            success_rate: 0.0,
            conflicts_by_type: HashMap::new(),
            conflicts_by_strategy: HashMap::new(),
            last_updated: Utc::now(),
        }
    }
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ConflictResolver {
    /// Create a new conflict resolver with default strategy
    pub fn new() -> Self {
        Self::with_strategy(ConflictStrategy::AutoMerge)
    }

    /// Create a new conflict resolver with specified strategy
    pub fn with_strategy(strategy: ConflictStrategy) -> Self {
        Self {
            strategy,
            conflicts: Arc::new(RwLock::new(HashMap::new())),
            conflict_history: Arc::new(RwLock::new(Vec::new())),
            resolution_handlers: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(ConflictStatistics::default())),
        }
    }

    /// Add a custom conflict handler
    pub fn add_handler(
        &self,
        handler: Box<dyn ConflictHandler + Send + Sync>,
    ) -> Result<(), ConflictError> {
        let handler_name = handler.name().to_string();
        let mut handlers = self.resolution_handlers.write();

        if handlers.contains_key(&handler_name) {
            return Err(ConflictError::HandlerNotFound { handler_name });
        }

        handlers.insert(handler_name, handler);
        Ok(())
    }

    /// Remove a custom conflict handler
    pub fn remove_handler(&self, handler_name: &str) -> Result<(), ConflictError> {
        let mut handlers = self.resolution_handlers.write();

        if handlers.remove(handler_name).is_none() {
            return Err(ConflictError::HandlerNotFound {
                handler_name: handler_name.to_string(),
            });
        }

        Ok(())
    }

    /// Detect a conflict between local and remote states
    pub async fn detect_conflict(
        &self,
        conflict_type: ConflictType,
        local_state: serde_json::Value,
        remote_state: serde_json::Value,
    ) -> Result<Option<Conflict>, ConflictError> {
        // Simple conflict detection: if states are different, there's a conflict
        if local_state != remote_state {
            let conflict = Conflict::new(
                conflict_type,
                ConflictSeverity::Medium,
                "State conflict detected".to_string(),
                local_state,
                remote_state,
            );

            // Store the conflict
            {
                let mut conflicts = self.conflicts.write();
                conflicts.insert(conflict.id.clone(), conflict.clone());
            }

            // Update statistics
            self.update_statistics(&conflict, None).await;

            info!("Conflict detected: {}", conflict.id);
            Ok(Some(conflict))
        } else {
            Ok(None)
        }
    }

    /// Attempt to resolve a conflict
    pub async fn attempt_resolution(
        &self,
        conflict_id: &ConflictId,
    ) -> Result<ResolutionResult, ConflictError> {
        // Get the conflict
        let conflict = {
            let conflicts = self.conflicts.read();
            conflicts.get(conflict_id).cloned()
        };

        let conflict = conflict.ok_or_else(|| ConflictError::HistoryError {
            message: format!("Conflict not found: {conflict_id}"),
        })?;

        let start_time = std::time::Instant::now();

        // Attempt resolution based on strategy
        let result = match &self.strategy {
            ConflictStrategy::AutoMerge => self.resolve_auto_merge(&conflict).await,
            ConflictStrategy::KeepLocal => self.resolve_keep_local(&conflict).await,
            ConflictStrategy::KeepRemote => self.resolve_keep_remote(&conflict).await,
            ConflictStrategy::Manual => self.resolve_manual(&conflict).await,
            ConflictStrategy::LastWriterWins => self.resolve_last_writer_wins(&conflict).await,
            ConflictStrategy::Custom { handler_name } => {
                self.resolve_custom(&conflict, handler_name).await
            }
        };

        // Record resolution
        if let Ok(ref resolution) = result {
            self.record_conflict_resolution(conflict_id, resolution)
                .await;
        }

        // Update statistics
        self.update_statistics(&conflict, result.as_ref().ok())
            .await;

        // Remove resolved conflict
        if result.as_ref().map(|r| r.success).unwrap_or(false) {
            let mut conflicts = self.conflicts.write();
            conflicts.remove(conflict_id);
        }

        result
    }

    /// Get a specific conflict
    pub async fn get_conflict(&self, conflict_id: &ConflictId) -> Option<Conflict> {
        let conflicts = self.conflicts.read();
        conflicts.get(conflict_id).cloned()
    }

    /// Get all active conflicts
    pub async fn get_active_conflicts(&self) -> Vec<Conflict> {
        let conflicts = self.conflicts.read();
        conflicts.values().cloned().collect()
    }

    /// Get conflict resolution statistics
    pub async fn get_statistics(&self) -> ConflictStatistics {
        let stats = self.statistics.read();
        stats.clone()
    }

    /// Get conflict resolution history
    pub async fn get_conflict_history(&self) -> Vec<ConflictRecord> {
        let history = self.conflict_history.read();
        history.clone()
    }

    /// Manually add a conflict to the resolver
    pub async fn add_conflict(&self, conflict: Conflict) {
        let mut conflicts = self.conflicts.write();
        conflicts.insert(conflict.id.clone(), conflict.clone());

        // Update statistics
        self.update_statistics(&conflict, None).await;
    }

    /// Resolve conflict using auto-merge strategy
    async fn resolve_auto_merge(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError> {
        // Simple auto-merge: merge objects, keep arrays from local
        let resolved_state =
            if conflict.local_state.is_object() && conflict.remote_state.is_object() {
                let mut merged = conflict.local_state.as_object().unwrap().clone();
                if let Some(remote_obj) = conflict.remote_state.as_object() {
                    for (key, value) in remote_obj {
                        merged.insert(key.clone(), value.clone());
                    }
                }
                serde_json::Value::Object(merged)
            } else {
                // Default to local state for non-objects
                conflict.local_state.clone()
            };

        Ok(ResolutionResult::success(
            ConflictStrategy::AutoMerge,
            resolved_state,
            "Auto-merged local and remote states".to_string(),
        ))
    }

    /// Resolve conflict by keeping local state
    async fn resolve_keep_local(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError> {
        Ok(ResolutionResult::success(
            ConflictStrategy::KeepLocal,
            conflict.local_state.clone(),
            "Kept local state".to_string(),
        ))
    }

    /// Resolve conflict by keeping remote state
    async fn resolve_keep_remote(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError> {
        Ok(ResolutionResult::success(
            ConflictStrategy::KeepRemote,
            conflict.remote_state.clone(),
            "Kept remote state".to_string(),
        ))
    }

    /// Resolve conflict manually (requires manual intervention)
    async fn resolve_manual(&self, conflict: &Conflict) -> Result<ResolutionResult, ConflictError> {
        Err(ConflictError::ManualResolutionRequired {
            conflict_id: conflict.id.as_str().to_string(),
        })
    }

    /// Resolve conflict using last writer wins strategy
    async fn resolve_last_writer_wins(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError> {
        // This is a simplified implementation
        // In a real system, you'd compare timestamps or version numbers
        let resolved_state =
            if conflict.local_state.is_object() && conflict.remote_state.is_object() {
                // For objects, merge but prefer remote for conflicts
                let mut merged = conflict.local_state.as_object().unwrap().clone();
                if let Some(remote_obj) = conflict.remote_state.as_object() {
                    for (key, value) in remote_obj {
                        merged.insert(key.clone(), value.clone());
                    }
                }
                serde_json::Value::Object(merged)
            } else {
                // For non-objects, prefer remote
                conflict.remote_state.clone()
            };

        Ok(ResolutionResult::success(
            ConflictStrategy::LastWriterWins,
            resolved_state,
            "Applied last writer wins strategy".to_string(),
        ))
    }

    /// Resolve conflict using custom handler
    async fn resolve_custom(
        &self,
        conflict: &Conflict,
        handler_name: &str,
    ) -> Result<ResolutionResult, ConflictError> {
        let handlers = self.resolution_handlers.read();
        let handler = handlers
            .get(handler_name)
            .ok_or_else(|| ConflictError::HandlerNotFound {
                handler_name: handler_name.to_string(),
            })?;

        handler.resolve_conflict(conflict).await
    }

    /// Record conflict resolution for history
    async fn record_conflict_resolution(
        &self,
        conflict_id: &ConflictId,
        result: &ResolutionResult,
    ) {
        let resolution_time_ms = result
            .timestamp
            .signed_duration_since(Utc::now())
            .num_milliseconds() as u64;

        let record = ConflictRecord {
            conflict_id: conflict_id.clone(),
            conflict_type: ConflictType::AgentState, // This should come from the conflict
            resolution_strategy: result.strategy.clone(),
            resolution_success: result.success,
            resolution_time_ms,
            timestamp: Utc::now(),
        };

        let mut history = self.conflict_history.write();
        history.push(record);
    }

    /// Update conflict resolution statistics
    async fn update_statistics(&self, conflict: &Conflict, resolution: Option<&ResolutionResult>) {
        let mut stats = self.statistics.write();

        stats.total_conflicts += 1;

        // Update conflicts by type
        let type_key = match &conflict.conflict_type {
            ConflictType::AgentState => "agent_state".to_string(),
            ConflictType::TaskAssignment => "task_assignment".to_string(),
            ConflictType::Resource => "resource".to_string(),
            ConflictType::Configuration => "configuration".to_string(),
            ConflictType::Communication => "communication".to_string(),
            ConflictType::Custom { conflict_type } => format!("custom_{conflict_type}"),
        };
        *stats.conflicts_by_type.entry(type_key).or_insert(0) += 1;

        if let Some(resolution) = resolution {
            if resolution.success {
                stats.total_resolved += 1;
            } else {
                stats.total_failed += 1;
            }

            // Update conflicts by strategy
            let strategy_key = match &resolution.strategy {
                ConflictStrategy::AutoMerge => "auto_merge".to_string(),
                ConflictStrategy::KeepLocal => "keep_local".to_string(),
                ConflictStrategy::KeepRemote => "keep_remote".to_string(),
                ConflictStrategy::Manual => "manual".to_string(),
                ConflictStrategy::LastWriterWins => "last_writer_wins".to_string(),
                ConflictStrategy::Custom { handler_name } => format!("custom_{handler_name}"),
            };
            *stats.conflicts_by_strategy.entry(strategy_key).or_insert(0) += 1;
        }

        // Calculate success rate
        if stats.total_conflicts > 0 {
            stats.success_rate =
                (stats.total_resolved as f64 / stats.total_conflicts as f64) * 100.0;
        }

        stats.last_updated = Utc::now();
    }
}

/// Default agent state conflict handler
pub struct AgentStateConflictHandler;

impl ConflictHandler for AgentStateConflictHandler {
    fn resolve_conflict<'a>(
        &'a self,
        conflict: &'a Conflict,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<ResolutionResult, ConflictError>> + Send + 'a>,
    > {
        Box::pin(async move {
            // Specialized handling for agent state conflicts
            match &conflict.conflict_type {
                ConflictType::AgentState => {
                    // For agent state conflicts, prefer the state with higher health score
                    let local_health = conflict.local_state.get("health").and_then(|h| h.as_str());
                    let remote_health =
                        conflict.remote_state.get("health").and_then(|h| h.as_str());

                    let resolved_state = match (local_health, remote_health) {
                        (Some("healthy"), _) => conflict.local_state.clone(),
                        (_, Some("healthy")) => conflict.remote_state.clone(),
                        (Some("degraded"), _) => conflict.local_state.clone(),
                        (_, Some("degraded")) => conflict.remote_state.clone(),
                        _ => conflict.local_state.clone(), // Default to local
                    };

                    Ok(ResolutionResult::success(
                        ConflictStrategy::Custom {
                            handler_name: "agent_state".to_string(),
                        },
                        resolved_state,
                        "Resolved agent state conflict based on health".to_string(),
                    ))
                }
                _ => Err(ConflictError::UnsupportedStrategy {
                    strategy: "agent_state_handler".to_string(),
                }),
            }
        })
    }

    fn name(&self) -> &str {
        "agent_state_handler"
    }

    fn supported_conflict_types(&self) -> Vec<ConflictType> {
        vec![ConflictType::AgentState]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_creation() {
        let conflict = Conflict::new(
            ConflictType::AgentState,
            ConflictSeverity::Medium,
            "Test conflict".to_string(),
            serde_json::json!({"status": "idle"}),
            serde_json::json!({"status": "busy"}),
        );

        assert_eq!(conflict.conflict_type, ConflictType::AgentState);
        assert_eq!(conflict.severity, ConflictSeverity::Medium);
        assert!(!conflict.is_resolved());
    }

    #[tokio::test]
    async fn test_conflict_detection() {
        let resolver = ConflictResolver::new();

        let local_state = serde_json::json!({"status": "idle"});
        let remote_state = serde_json::json!({"status": "busy"});

        let result = resolver
            .detect_conflict(ConflictType::AgentState, local_state, remote_state)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_conflict_resolution() {
        let resolver = ConflictResolver::with_strategy(ConflictStrategy::KeepLocal);

        let conflict = Conflict::new(
            ConflictType::AgentState,
            ConflictSeverity::Medium,
            "Test conflict".to_string(),
            serde_json::json!({"status": "idle"}),
            serde_json::json!({"status": "busy"}),
        );

        // Add the conflict to the resolver
        resolver.add_conflict(conflict.clone()).await;

        let result = resolver.attempt_resolution(&conflict.id).await;
        assert!(result.is_ok());

        let resolution = result.unwrap();
        assert!(resolution.success);
        assert_eq!(resolution.strategy, ConflictStrategy::KeepLocal);
    }

    #[tokio::test]
    async fn test_custom_handler() {
        let resolver = ConflictResolver::new();
        let handler = Box::new(AgentStateConflictHandler);

        resolver.add_handler(handler).unwrap();

        let conflict = Conflict::new(
            ConflictType::AgentState,
            ConflictSeverity::Medium,
            "Test conflict".to_string(),
            serde_json::json!({"health": "healthy"}),
            serde_json::json!({"health": "degraded"}),
        );

        // Add the conflict to the resolver
        resolver.add_conflict(conflict.clone()).await;

        let result = resolver.attempt_resolution(&conflict.id).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolution_result() {
        let result = ResolutionResult::success(
            ConflictStrategy::AutoMerge,
            serde_json::json!({"status": "resolved"}),
            "Test resolution".to_string(),
        );

        assert!(result.success);
        assert_eq!(result.strategy, ConflictStrategy::AutoMerge);
    }
}
