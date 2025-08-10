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

use super::{PersistenceConfig, StorageBackend, StoreStats};
use crate::agent::real_time_coordination::{
    ConsensusAlgorithm, ConsensusConfig, ConsensusEntry, ConsensusState,
};
use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Consensus store for persisting consensus state
pub struct ConsensusStore {
    config: PersistenceConfig,
    consensus_states: Arc<RwLock<HashMap<String, StoredConsensusState>>>,
    consensus_logs: Arc<RwLock<HashMap<String, Vec<StoredConsensusEntry>>>>,
    consensus_configs: Arc<RwLock<HashMap<String, ConsensusConfig>>>,
    file_path: Option<PathBuf>,
}

/// Stored consensus state with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConsensusState {
    pub state: ConsensusState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub size_bytes: u64,
    pub term_count: u64,
    pub leader_changes: u64,
}

/// Stored consensus entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConsensusEntry {
    pub entry: ConsensusEntry,
    pub created_at: DateTime<Utc>,
    pub committed_at: Option<DateTime<Utc>>,
    pub applied_at: Option<DateTime<Utc>>,
    pub size_bytes: u64,
}

impl ConsensusStore {
    /// Create a new consensus store
    pub async fn new(config: PersistenceConfig) -> RhemaResult<Self> {
        let file_path = match &config.backend {
            StorageBackend::File => {
                let path = config
                    .storage_path
                    .as_ref()
                    .map(|p| p.join("consensus"))
                    .unwrap_or_else(|| PathBuf::from("./data/consensus"));

                // Create directory if it doesn't exist
                if let Some(parent) = path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                Some(path)
            }
            _ => None,
        };

        let mut store = Self {
            config,
            consensus_states: Arc::new(RwLock::new(HashMap::new())),
            consensus_logs: Arc::new(RwLock::new(HashMap::new())),
            consensus_configs: Arc::new(RwLock::new(HashMap::new())),
            file_path,
        };

        // Load existing data
        store.load().await?;

        Ok(store)
    }

    /// Store consensus state
    pub async fn store_consensus_state(
        &self,
        node_id: String,
        state: ConsensusState,
    ) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&state)?.len() as u64;
        let now = Utc::now();

        let mut states = self.consensus_states.write().await;

        let _stored_state = if let Some(existing) = states.get_mut(&node_id) {
            existing.state = state;
            existing.updated_at = now;
            existing.size_bytes = size_bytes;
            existing.access_count += 1;
            existing.last_accessed = now;
            existing.term_count += 1;
            existing.clone()
        } else {
            let stored_state = StoredConsensusState {
                state,
                created_at: now,
                updated_at: now,
                access_count: 1,
                last_accessed: now,
                size_bytes,
                term_count: 1,
                leader_changes: 0,
            };
            states.insert(node_id.clone(), stored_state.clone());
            stored_state
        };

        self.save().await?;
        Ok(())
    }

    /// Store consensus entry
    pub async fn store_consensus_entry(
        &self,
        node_id: String,
        entry: ConsensusEntry,
    ) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&entry)?.len() as u64;
        let now = Utc::now();

        let stored_entry = StoredConsensusEntry {
            entry,
            created_at: now,
            committed_at: None,
            applied_at: None,
            size_bytes,
        };

        {
            let mut logs = self.consensus_logs.write().await;
            let node_log = logs.entry(node_id).or_insert_with(Vec::new);
            node_log.push(stored_entry);
        }

        self.save().await?;
        Ok(())
    }

    /// Store consensus configuration
    pub async fn store_consensus_config(
        &self,
        node_id: String,
        config: ConsensusConfig,
    ) -> RhemaResult<()> {
        {
            let mut configs = self.consensus_configs.write().await;
            configs.insert(node_id, config);
        }

        self.save().await?;
        Ok(())
    }

    /// Retrieve consensus state
    pub async fn get_consensus_state(&self, node_id: &str) -> Option<ConsensusState> {
        let mut states = self.consensus_states.write().await;

        if let Some(stored_state) = states.get_mut(node_id) {
            stored_state.access_count += 1;
            stored_state.last_accessed = Utc::now();
            Some(stored_state.state.clone())
        } else {
            None
        }
    }

    /// Retrieve consensus log
    pub async fn get_consensus_log(&self, node_id: &str) -> Vec<ConsensusEntry> {
        let logs = self.consensus_logs.read().await;
        logs.get(node_id)
            .map(|entries| entries.iter().map(|e| e.entry.clone()).collect())
            .unwrap_or_default()
    }

    /// Retrieve consensus configuration
    pub async fn get_consensus_config(&self, node_id: &str) -> Option<ConsensusConfig> {
        let configs = self.consensus_configs.read().await;
        configs.get(node_id).cloned()
    }

    /// Mark consensus entry as committed
    pub async fn mark_entry_committed(&self, node_id: &str, index: u64) -> RhemaResult<()> {
        let mut logs = self.consensus_logs.write().await;

        if let Some(node_log) = logs.get_mut(node_id) {
            if let Some(entry) = node_log.get_mut(index as usize) {
                entry.committed_at = Some(Utc::now());
            }
        }

        self.save().await?;
        Ok(())
    }

    /// Mark consensus entry as applied
    pub async fn mark_entry_applied(&self, node_id: &str, index: u64) -> RhemaResult<()> {
        let mut logs = self.consensus_logs.write().await;

        if let Some(node_log) = logs.get_mut(node_id) {
            if let Some(entry) = node_log.get_mut(index as usize) {
                entry.applied_at = Some(Utc::now());
            }
        }

        self.save().await?;
        Ok(())
    }

    /// Get consensus statistics
    pub async fn get_consensus_stats(&self, node_id: &str) -> Option<ConsensusStats> {
        let states = self.consensus_states.read().await;
        let logs = self.consensus_logs.read().await;

        if let Some(stored_state) = states.get(node_id) {
            let log_entries = logs.get(node_id).map(|entries| entries.len()).unwrap_or(0);
            let committed_entries = logs
                .get(node_id)
                .map(|entries| entries.iter().filter(|e| e.committed_at.is_some()).count())
                .unwrap_or(0);
            let applied_entries = logs
                .get(node_id)
                .map(|entries| entries.iter().filter(|e| e.applied_at.is_some()).count())
                .unwrap_or(0);

            Some(ConsensusStats {
                node_id: node_id.to_string(),
                term: stored_state.state.term,
                leader_id: stored_state.state.leader_id.clone(),
                state: stored_state.state.state.clone(),
                total_entries: log_entries,
                committed_entries,
                applied_entries,
                term_count: stored_state.term_count,
                leader_changes: stored_state.leader_changes,
                last_updated: stored_state.updated_at,
            })
        } else {
            None
        }
    }

    /// List all consensus nodes
    pub async fn list_consensus_nodes(&self) -> Vec<String> {
        let states = self.consensus_states.read().await;
        states.keys().cloned().collect()
    }

    /// Get consensus state by algorithm
    pub async fn get_consensus_by_algorithm(
        &self,
        algorithm: ConsensusAlgorithm,
    ) -> Vec<ConsensusState> {
        let states = self.consensus_states.read().await;
        let configs = self.consensus_configs.read().await;

        states
            .iter()
            .filter_map(|(node_id, stored_state)| {
                configs
                    .get(node_id)
                    .filter(|config| config.algorithm == algorithm)
                    .map(|_| stored_state.state.clone())
            })
            .collect()
    }

    /// Load data from storage
    async fn load(&mut self) -> RhemaResult<()> {
        match &self.config.backend {
            StorageBackend::File => {
                if let Some(path) = &self.file_path {
                    if path.exists() {
                        let data = tokio::fs::read_to_string(path).await?;
                        let stored_data: StoredConsensusData = serde_json::from_str(&data)?;

                        let states_count = stored_data.states.len();
                        let logs_count = stored_data.logs.len();

                        *self.consensus_states.write().await = stored_data.states;
                        *self.consensus_logs.write().await = stored_data.logs;
                        *self.consensus_configs.write().await = stored_data.configs;

                        info!(
                            "Loaded {} consensus states and {} consensus logs from storage",
                            states_count, logs_count
                        );
                    }
                }
            }
            _ => {
                // For other backends, start with empty storage
                info!("Using in-memory consensus storage");
            }
        }
        Ok(())
    }

    /// Save data to storage
    async fn save(&self) -> RhemaResult<()> {
        match &self.config.backend {
            StorageBackend::File => {
                if let Some(path) = &self.file_path {
                    let states = self.consensus_states.read().await;
                    let logs = self.consensus_logs.read().await;
                    let configs = self.consensus_configs.read().await;

                    let stored_data = StoredConsensusData {
                        states: states.clone(),
                        logs: logs.clone(),
                        configs: configs.clone(),
                    };

                    let data = serde_json::to_string_pretty(&stored_data)?;
                    tokio::fs::write(path, data).await?;
                }
            }
            _ => {
                // For other backends, data is kept in memory
            }
        }
        Ok(())
    }

    /// Perform backup
    pub async fn backup(&self) -> RhemaResult<()> {
        if self.config.enable_backups {
            let backup_path = self
                .file_path
                .as_ref()
                .map(|p| p.with_extension("backup"))
                .unwrap_or_else(|| PathBuf::from("./data/consensus.backup"));

            let states = self.consensus_states.read().await;
            let logs = self.consensus_logs.read().await;
            let configs = self.consensus_configs.read().await;

            let stored_data = StoredConsensusData {
                states: states.clone(),
                logs: logs.clone(),
                configs: configs.clone(),
            };

            let data = serde_json::to_string_pretty(&stored_data)?;
            tokio::fs::write(backup_path, data).await?;

            info!("Consensus backup completed");
        }
        Ok(())
    }

    /// Perform cleanup
    pub async fn cleanup(&self) -> RhemaResult<()> {
        if self.config.enable_cleanup {
            let cutoff_date =
                Utc::now() - chrono::Duration::days(self.config.data_retention_days as i64);

            {
                let mut states = self.consensus_states.write().await;
                states.retain(|_, stored_state| stored_state.updated_at > cutoff_date);
            }

            {
                let mut logs = self.consensus_logs.write().await;
                for node_log in logs.values_mut() {
                    node_log.retain(|entry| entry.created_at > cutoff_date);
                }
            }

            self.save().await?;
            info!("Consensus cleanup completed");
        }
        Ok(())
    }

    /// Validate stored data
    pub async fn validate(&self) -> RhemaResult<()> {
        if self.config.enable_validation {
            let states = self.consensus_states.read().await;
            let logs = self.consensus_logs.read().await;

            for (node_id, stored_state) in states.iter() {
                if stored_state.state.term == 0 && stored_state.term_count > 0 {
                    return Err(rhema_core::RhemaError::Validation(format!(
                        "Invalid term count for node {}: term is 0 but count is {}",
                        node_id, stored_state.term_count
                    )));
                }
            }

            for (node_id, node_log) in logs.iter() {
                for (index, entry) in node_log.iter().enumerate() {
                    if entry.entry.index != index as u64 {
                        return Err(rhema_core::RhemaError::Validation(format!(
                            "Invalid entry index for node {}: expected {}, got {}",
                            node_id, index, entry.entry.index
                        )));
                    }
                }
            }

            info!("Consensus validation completed successfully");
        }
        Ok(())
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> RhemaResult<StoreStats> {
        let states = self.consensus_states.read().await;
        let logs = self.consensus_logs.read().await;
        let configs = self.consensus_configs.read().await;

        let total_entries = states.len()
            + logs.values().map(|entries| entries.len()).sum::<usize>()
            + configs.len();
        let size_bytes = states.values().map(|s| s.size_bytes).sum::<u64>()
            + logs
                .values()
                .flat_map(|entries| entries.iter())
                .map(|e| e.size_bytes)
                .sum::<u64>();

        Ok(StoreStats {
            total_entries,
            size_bytes,
            last_backup: None,    // TODO: Track backup timestamps
            last_cleanup: None,   // TODO: Track cleanup timestamps
            validation_errors: 0, // TODO: Track validation errors
        })
    }
}

/// Consensus statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub node_id: String,
    pub term: u64,
    pub leader_id: Option<String>,
    pub state: crate::agent::real_time_coordination::ConsensusNodeState,
    pub total_entries: usize,
    pub committed_entries: usize,
    pub applied_entries: usize,
    pub term_count: u64,
    pub leader_changes: u64,
    pub last_updated: DateTime<Utc>,
}

/// Stored consensus data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredConsensusData {
    states: HashMap<String, StoredConsensusState>,
    logs: HashMap<String, Vec<StoredConsensusEntry>>,
    configs: HashMap<String, ConsensusConfig>,
}
