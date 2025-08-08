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
use tracing::{info, warn};
use uuid::Uuid;

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
    pub id: String,

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
            id: Uuid::new_v4().to_string(),
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

    /// Add metadata to conflict
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if conflict is resolved
    pub fn is_resolved(&self) -> bool {
        self.resolved_at.is_some()
    }

    /// Get conflict age
    pub fn age(&self) -> std::time::Duration {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.created_at);
        duration.to_std().unwrap_or_default()
    }
}

/// Conflict resolution result
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

/// Conflict record for history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRecord {
    /// Conflict ID
    pub conflict_id: String,

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

/// Conflict handler trait
#[async_trait::async_trait]
pub trait ConflictHandler: Send + Sync {
    /// Handle conflict resolution
    async fn resolve_conflict(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError>;

    /// Get handler name
    fn name(&self) -> &str;

    /// Get supported conflict types
    fn supported_conflict_types(&self) -> Vec<ConflictType>;
}

/// Default conflict resolver
pub struct ConflictResolver {
    /// Resolution strategy
    strategy: ConflictStrategy,

    /// Active conflicts
    conflicts: Arc<RwLock<HashMap<String, Conflict>>>,

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

impl ConflictResolver {
    /// Create a new conflict resolver with strategy
    pub fn with_strategy(strategy: ConflictStrategy) -> Self {
        Self {
            strategy,
            conflicts: Arc::new(RwLock::new(HashMap::new())),
            conflict_history: Arc::new(RwLock::new(Vec::new())),
            resolution_handlers: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(ConflictStatistics::default())),
        }
    }

    /// Add custom conflict handler
    pub fn add_handler(
        &self,
        handler: Box<dyn ConflictHandler + Send + Sync>,
    ) -> Result<(), ConflictError> {
        let name = handler.name().to_string();
        let mut handlers = self.resolution_handlers.write();

        if handlers.contains_key(&name) {
            return Err(ConflictError::HandlerNotFound { handler_name: name });
        }

        handlers.insert(name, handler);
        Ok(())
    }

    /// Remove custom conflict handler
    pub fn remove_handler(&self, handler_name: &str) -> Result<(), ConflictError> {
        let mut handlers = self.resolution_handlers.write();

        if handlers.remove(handler_name).is_none() {
            return Err(ConflictError::HandlerNotFound {
                handler_name: handler_name.to_string(),
            });
        }

        Ok(())
    }

    /// Detect conflict between local and remote states
    pub async fn detect_conflict(
        &self,
        conflict_type: ConflictType,
        local_state: serde_json::Value,
        remote_state: serde_json::Value,
    ) -> Result<Option<Conflict>, ConflictError> {
        // Check if states are different
        if local_state == remote_state {
            return Ok(None);
        }

        // Create conflict
        let conflict = Conflict::new(
            conflict_type,
            ConflictSeverity::Medium, // Default severity
            "Conflict detected between local and remote states".to_string(),
            local_state,
            remote_state,
        );

        // Store conflict
        self.conflicts
            .write()
            .insert(conflict.id.clone(), conflict.clone());

        // Update statistics
        self.update_statistics(&conflict, None).await;

        info!("Conflict detected: {}", conflict.id);
        Ok(Some(conflict))
    }

    /// Attempt to resolve a conflict
    pub async fn attempt_resolution(
        &self,
        conflict_id: &str,
    ) -> Result<ResolutionResult, ConflictError> {
        let conflict = {
            let conflicts = self.conflicts.read();
            conflicts
                .get(conflict_id)
                .cloned()
                .ok_or_else(|| ConflictError::DetectionFailed {
                    message: format!("Conflict not found: {conflict_id}"),
                })?
        };

        if conflict.is_resolved() {
            return Err(ConflictError::ResolutionFailed {
                message: "Conflict already resolved".to_string(),
            });
        }

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
        }?;

        // Update conflict with resolution
        {
            let mut conflicts = self.conflicts.write();
            if let Some(conflict) = conflicts.get_mut(conflict_id) {
                conflict.resolved_at = Some(Utc::now());
                conflict.resolution_strategy = Some(self.strategy.clone());
                conflict.resolution_result = Some(result.resolved_state.clone());
            }
        }

        // Record in history
        self.record_conflict_resolution(conflict_id, &result).await;

        // Update statistics
        self.update_statistics(&conflict, Some(&result)).await;

        info!(
            "Conflict resolved: {} with strategy: {:?}",
            conflict_id, self.strategy
        );
        Ok(result)
    }

    /// Get conflict by ID
    pub async fn get_conflict(&self, conflict_id: &str) -> Option<Conflict> {
        self.conflicts.read().get(conflict_id).cloned()
    }

    /// Get all active conflicts
    pub async fn get_active_conflicts(&self) -> Vec<Conflict> {
        self.conflicts
            .read()
            .values()
            .filter(|c| !c.is_resolved())
            .cloned()
            .collect()
    }

    /// Get conflict statistics
    pub async fn get_statistics(&self) -> ConflictStatistics {
        self.statistics.read().clone()
    }

    /// Get conflict history
    pub async fn get_conflict_history(&self) -> Vec<ConflictRecord> {
        self.conflict_history.read().clone()
    }

    /// Auto-merge resolution strategy
    async fn resolve_auto_merge(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError> {
        // Simple merge strategy - combine both states
        let mut merged_state = conflict.local_state.clone();

        if let (serde_json::Value::Object(mut local), serde_json::Value::Object(remote)) =
            (merged_state.clone(), conflict.remote_state.clone())
        {
            for (key, value) in remote {
                local.insert(key, value);
            }
            merged_state = serde_json::Value::Object(local);
        }

        Ok(ResolutionResult::success(
            ConflictStrategy::AutoMerge,
            merged_state,
            "Auto-merged local and remote states".to_string(),
        ))
    }

    /// Keep local resolution strategy
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

    /// Keep remote resolution strategy
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

    /// Manual resolution strategy
    async fn resolve_manual(&self, conflict: &Conflict) -> Result<ResolutionResult, ConflictError> {
        Err(ConflictError::ManualResolutionRequired {
            conflict_id: conflict.id.clone(),
        })
    }

    /// Last writer wins resolution strategy
    async fn resolve_last_writer_wins(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError> {
        // Use timestamp to determine winner
        let local_timestamp = conflict
            .metadata
            .get("local_timestamp")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(conflict.created_at);

        let remote_timestamp = conflict
            .metadata
            .get("remote_timestamp")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(conflict.created_at);

        if local_timestamp > remote_timestamp {
            Ok(ResolutionResult::success(
                ConflictStrategy::LastWriterWins,
                conflict.local_state.clone(),
                "Local state wins (last writer)".to_string(),
            ))
        } else {
            Ok(ResolutionResult::success(
                ConflictStrategy::LastWriterWins,
                conflict.remote_state.clone(),
                "Remote state wins (last writer)".to_string(),
            ))
        }
    }

    /// Custom resolution strategy
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

    /// Record conflict resolution in history
    async fn record_conflict_resolution(&self, conflict_id: &str, result: &ResolutionResult) {
        // Get the actual conflict to extract conflict type and calculate resolution time
        let conflict = if let Some(conflict) = self.conflicts.read().get(conflict_id) {
            conflict.clone()
        } else {
            warn!(
                "Conflict {} not found when recording resolution",
                conflict_id
            );
            return;
        };

        // Calculate actual resolution time
        let resolution_time_ms = if let Some(resolved_at) = conflict.resolved_at {
            resolved_at
                .signed_duration_since(conflict.created_at)
                .num_milliseconds() as u64
        } else {
            // Fallback to current time if resolved_at is not set
            Utc::now()
                .signed_duration_since(conflict.created_at)
                .num_milliseconds() as u64
        };

        let record = ConflictRecord {
            conflict_id: conflict_id.to_string(),
            conflict_type: conflict.conflict_type.clone(),
            resolution_strategy: result.strategy.clone(),
            resolution_success: result.success,
            resolution_time_ms,
            timestamp: Utc::now(),
        };

        self.conflict_history.write().push(record);
    }

    /// Update conflict statistics
    async fn update_statistics(&self, conflict: &Conflict, resolution: Option<&ResolutionResult>) {
        let mut stats = self.statistics.write();

        stats.total_conflicts += 1;

        if let Some(resolution) = resolution {
            if resolution.success {
                stats.total_resolved += 1;
            } else {
                stats.total_failed += 1;
            }
        }

        // Update conflicts by type
        let type_key = format!("{:?}", conflict.conflict_type);
        *stats.conflicts_by_type.entry(type_key).or_insert(0) += 1;

        // Update conflicts by strategy
        if let Some(resolution) = resolution {
            let strategy_key = format!("{:?}", resolution.strategy);
            *stats.conflicts_by_strategy.entry(strategy_key).or_insert(0) += 1;
        }

        // Calculate success rate
        let total_attempted = stats.total_resolved + stats.total_failed;
        if total_attempted > 0 {
            stats.success_rate = (stats.total_resolved as f64 / total_attempted as f64) * 100.0;
        }

        stats.last_updated = Utc::now();
    }
}

/// Default conflict handler for agent state conflicts
pub struct AgentStateConflictHandler;

#[async_trait::async_trait]
impl ConflictHandler for AgentStateConflictHandler {
    async fn resolve_conflict(
        &self,
        conflict: &Conflict,
    ) -> Result<ResolutionResult, ConflictError> {
        match conflict.conflict_type {
            ConflictType::AgentState => {
                // For agent state conflicts, prefer the more recent state
                let local_timestamp = conflict
                    .metadata
                    .get("local_timestamp")
                    .and_then(|v| v.as_str())
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or(conflict.created_at);

                let remote_timestamp = conflict
                    .metadata
                    .get("remote_timestamp")
                    .and_then(|v| v.as_str())
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or(conflict.created_at);

                if local_timestamp > remote_timestamp {
                    Ok(ResolutionResult::success(
                        ConflictStrategy::LastWriterWins,
                        conflict.local_state.clone(),
                        "Local agent state is more recent".to_string(),
                    ))
                } else {
                    Ok(ResolutionResult::success(
                        ConflictStrategy::LastWriterWins,
                        conflict.remote_state.clone(),
                        "Remote agent state is more recent".to_string(),
                    ))
                }
            }
            _ => Err(ConflictError::ResolutionFailed {
                message: "Unsupported conflict type for agent state handler".to_string(),
            }),
        }
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
            serde_json::json!({"local": "value"}),
            serde_json::json!({"remote": "value"}),
        );

        assert!(!conflict.is_resolved());
        assert_eq!(conflict.conflict_type, ConflictType::AgentState);
    }

    #[tokio::test]
    async fn test_conflict_detection() {
        let resolver = ConflictResolver::with_strategy(ConflictStrategy::AutoMerge);

        let local_state = serde_json::json!({"key": "local_value"});
        let remote_state = serde_json::json!({"key": "remote_value"});

        let conflict = resolver
            .detect_conflict(ConflictType::AgentState, local_state, remote_state)
            .await
            .unwrap();

        assert!(conflict.is_some());
    }

    #[tokio::test]
    async fn test_conflict_resolution() {
        let resolver = ConflictResolver::with_strategy(ConflictStrategy::KeepLocal);

        let conflict = Conflict::new(
            ConflictType::AgentState,
            ConflictSeverity::Medium,
            "Test conflict".to_string(),
            serde_json::json!({"local": "value"}),
            serde_json::json!({"remote": "value"}),
        );

        // Store conflict
        resolver
            .conflicts
            .write()
            .insert(conflict.id.clone(), conflict.clone());

        let result = resolver.attempt_resolution(&conflict.id).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_custom_handler() {
        let resolver = ConflictResolver::with_strategy(ConflictStrategy::Custom {
            handler_name: "agent_state_handler".to_string(),
        });

        let handler = Box::new(AgentStateConflictHandler);
        resolver.add_handler(handler).unwrap();

        let conflict = Conflict::new(
            ConflictType::AgentState,
            ConflictSeverity::Medium,
            "Test conflict".to_string(),
            serde_json::json!({"local": "value"}),
            serde_json::json!({"remote": "value"}),
        );

        // Store conflict
        resolver
            .conflicts
            .write()
            .insert(conflict.id.clone(), conflict.clone());

        let result = resolver.attempt_resolution(&conflict.id).await.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_resolution_result() {
        let result = ResolutionResult::success(
            ConflictStrategy::KeepLocal,
            serde_json::json!({"resolved": "value"}),
            "Test resolution".to_string(),
        );

        assert!(result.success);
        assert_eq!(result.strategy, ConflictStrategy::KeepLocal);
    }
}
