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
use crate::agent::real_time_coordination::{AgentInfo, AgentStatus};
use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// State manager for persisting general system state
pub struct StateManager {
    config: PersistenceConfig,
    agent_states: Arc<RwLock<HashMap<String, StoredAgentState>>>,
    system_metrics: Arc<RwLock<SystemMetrics>>,
    configuration_snapshots: Arc<RwLock<HashMap<String, StoredConfiguration>>>,
    file_path: Option<PathBuf>,
}

/// Stored agent state with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAgentState {
    pub agent_info: AgentInfo,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub size_bytes: u64,
    pub state_transitions: Vec<StateTransition>,
}

/// Agent state transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from_status: AgentStatus,
    pub to_status: AgentStatus,
    pub timestamp: DateTime<Utc>,
    pub reason: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub total_messages: u64,
    pub messages_per_second: f64,
    pub average_response_time_ms: f64,
    pub system_uptime_seconds: u64,
    pub memory_usage_bytes: u64,
    pub cpu_usage_percent: f64,
    pub last_updated: DateTime<Utc>,
}

/// Stored configuration with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConfiguration {
    pub name: String,
    pub config_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: String,
    pub description: Option<String>,
    pub size_bytes: u64,
}

impl StateManager {
    /// Create a new state manager
    pub async fn new(config: PersistenceConfig) -> RhemaResult<Self> {
        let file_path = match &config.backend {
            StorageBackend::File => {
                let path = config
                    .storage_path
                    .as_ref()
                    .map(|p| p.join("state"))
                    .unwrap_or_else(|| PathBuf::from("./data/state"));

                // Create directory if it doesn't exist
                if let Some(parent) = path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                Some(path)
            }
            _ => None,
        };

        let mut manager = Self {
            config,
            agent_states: Arc::new(RwLock::new(HashMap::new())),
            system_metrics: Arc::new(RwLock::new(SystemMetrics::default())),
            configuration_snapshots: Arc::new(RwLock::new(HashMap::new())),
            file_path,
        };

        // Load existing data
        manager.load().await?;

        Ok(manager)
    }

    /// Store agent state
    pub async fn store_agent_state(&self, agent_info: AgentInfo) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&agent_info)?.len() as u64;
        let now = Utc::now();

        let mut states = self.agent_states.write().await;

        let _stored_state = if let Some(existing) = states.get_mut(&agent_info.id) {
            // Record state transition if status changed
            if existing.agent_info.status != agent_info.status {
                let transition = StateTransition {
                    from_status: existing.agent_info.status.clone(),
                    to_status: agent_info.status.clone(),
                    timestamp: now,
                    reason: None,
                    metadata: HashMap::new(),
                };
                existing.state_transitions.push(transition);
            }

            existing.agent_info = agent_info;
            existing.updated_at = now;
            existing.size_bytes = size_bytes;
            existing.access_count += 1;
            existing.last_accessed = now;
            existing.clone()
        } else {
            let stored_state = StoredAgentState {
                agent_info,
                created_at: now,
                updated_at: now,
                access_count: 1,
                last_accessed: now,
                size_bytes,
                state_transitions: Vec::new(),
            };
            states.insert(stored_state.agent_info.id.clone(), stored_state.clone());
            stored_state
        };

        self.save().await?;
        Ok(())
    }

    /// Retrieve agent state
    pub async fn get_agent_state(&self, agent_id: &str) -> Option<AgentInfo> {
        let mut states = self.agent_states.write().await;

        if let Some(stored_state) = states.get_mut(agent_id) {
            stored_state.access_count += 1;
            stored_state.last_accessed = Utc::now();
            Some(stored_state.agent_info.clone())
        } else {
            None
        }
    }

    /// Get agent state transitions
    pub async fn get_agent_transitions(&self, agent_id: &str) -> Vec<StateTransition> {
        let states = self.agent_states.read().await;
        states
            .get(agent_id)
            .map(|stored_state| stored_state.state_transitions.clone())
            .unwrap_or_default()
    }

    /// Update system metrics
    pub async fn update_system_metrics(&self, metrics: SystemMetrics) -> RhemaResult<()> {
        {
            let mut system_metrics = self.system_metrics.write().await;
            *system_metrics = metrics;
        }

        self.save().await?;
        Ok(())
    }

    /// Get system metrics
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        let metrics = self.system_metrics.read().await;
        metrics.clone()
    }

    /// Store configuration snapshot
    pub async fn store_configuration(
        &self,
        name: String,
        config_data: serde_json::Value,
        version: String,
        description: Option<String>,
    ) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&config_data)?.len() as u64;
        let now = Utc::now();

        let stored_config = StoredConfiguration {
            name: name.clone(),
            config_data,
            created_at: now,
            updated_at: now,
            version,
            description,
            size_bytes,
        };

        {
            let mut configs = self.configuration_snapshots.write().await;
            configs.insert(name, stored_config);
        }

        self.save().await?;
        Ok(())
    }

    /// Retrieve configuration snapshot
    pub async fn get_configuration(&self, name: &str) -> Option<StoredConfiguration> {
        let configs = self.configuration_snapshots.read().await;
        configs.get(name).cloned()
    }

    /// List all configurations
    pub async fn list_configurations(&self) -> Vec<String> {
        let configs = self.configuration_snapshots.read().await;
        configs.keys().cloned().collect()
    }

    /// Get agents by status
    pub async fn get_agents_by_status(&self, status: AgentStatus) -> Vec<AgentInfo> {
        let states = self.agent_states.read().await;
        states
            .values()
            .filter(|s| s.agent_info.status == status)
            .map(|s| s.agent_info.clone())
            .collect()
    }

    /// Get agents by capability
    pub async fn get_agents_by_capability(&self, capability: &str) -> Vec<AgentInfo> {
        let states = self.agent_states.read().await;
        states
            .values()
            .filter(|s| s.agent_info.capabilities.contains(&capability.to_string()))
            .map(|s| s.agent_info.clone())
            .collect()
    }

    /// Get agent performance statistics
    pub async fn get_agent_performance_stats(&self) -> AgentPerformanceStats {
        let states = self.agent_states.read().await;

        let total_agents = states.len();
        let active_agents = states.values().filter(|s| s.agent_info.is_online).count();
        let avg_tasks_completed = states
            .values()
            .map(|s| s.agent_info.performance_metrics.tasks_completed)
            .sum::<usize>() as f64
            / total_agents.max(1) as f64;
        let avg_success_rate = states
            .values()
            .map(|s| s.agent_info.performance_metrics.success_rate)
            .sum::<f64>()
            / total_agents.max(1) as f64;
        let avg_response_time = states
            .values()
            .map(|s| s.agent_info.performance_metrics.avg_response_time_ms)
            .sum::<f64>()
            / total_agents.max(1) as f64;

        AgentPerformanceStats {
            total_agents,
            active_agents,
            avg_tasks_completed,
            avg_success_rate,
            avg_response_time,
            last_updated: Utc::now(),
        }
    }

    /// Load data from storage
    async fn load(&mut self) -> RhemaResult<()> {
        match &self.config.backend {
            StorageBackend::File => {
                if let Some(path) = &self.file_path {
                    if path.exists() {
                        let data = tokio::fs::read_to_string(path).await?;
                        let stored_data: StoredStateData = serde_json::from_str(&data)?;

                        let agent_states_count = stored_data.agent_states.len();
                        let configurations_count = stored_data.configurations.len();

                        *self.agent_states.write().await = stored_data.agent_states;
                        *self.system_metrics.write().await = stored_data.system_metrics;
                        *self.configuration_snapshots.write().await = stored_data.configurations;

                        info!(
                            "Loaded {} agent states and {} configurations from storage",
                            agent_states_count, configurations_count
                        );
                    }
                }
            }
            _ => {
                // For other backends, start with empty storage
                info!("Using in-memory state storage");
            }
        }
        Ok(())
    }

    /// Save data to storage
    async fn save(&self) -> RhemaResult<()> {
        match &self.config.backend {
            StorageBackend::File => {
                if let Some(path) = &self.file_path {
                    let agent_states = self.agent_states.read().await;
                    let system_metrics = self.system_metrics.read().await;
                    let configurations = self.configuration_snapshots.read().await;

                    let stored_data = StoredStateData {
                        agent_states: agent_states.clone(),
                        system_metrics: system_metrics.clone(),
                        configurations: configurations.clone(),
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
                .unwrap_or_else(|| PathBuf::from("./data/state.backup"));

            let agent_states = self.agent_states.read().await;
            let system_metrics = self.system_metrics.read().await;
            let configurations = self.configuration_snapshots.read().await;

            let stored_data = StoredStateData {
                agent_states: agent_states.clone(),
                system_metrics: system_metrics.clone(),
                configurations: configurations.clone(),
            };

            let data = serde_json::to_string_pretty(&stored_data)?;
            tokio::fs::write(backup_path, data).await?;

            info!("State backup completed");
        }
        Ok(())
    }

    /// Perform cleanup
    pub async fn cleanup(&self) -> RhemaResult<()> {
        if self.config.enable_cleanup {
            let cutoff_date =
                Utc::now() - chrono::Duration::days(self.config.data_retention_days as i64);

            {
                let mut states = self.agent_states.write().await;
                states.retain(|_, stored_state| stored_state.updated_at > cutoff_date);
            }

            {
                let mut configs = self.configuration_snapshots.write().await;
                configs.retain(|_, stored_config| stored_config.updated_at > cutoff_date);
            }

            self.save().await?;
            info!("State cleanup completed");
        }
        Ok(())
    }

    /// Validate stored data
    pub async fn validate(&self) -> RhemaResult<()> {
        if self.config.enable_validation {
            let states = self.agent_states.read().await;

            for (id, stored_state) in states.iter() {
                if stored_state.agent_info.id != *id {
                    return Err(rhema_core::RhemaError::Validation(format!(
                        "Agent ID mismatch: {}",
                        id
                    )));
                }

                // Validate performance metrics
                let metrics = &stored_state.agent_info.performance_metrics;
                if metrics.success_rate < 0.0 || metrics.success_rate > 1.0 {
                    return Err(rhema_core::RhemaError::Validation(format!(
                        "Invalid success rate for agent {}: {}",
                        id, metrics.success_rate
                    )));
                }
            }

            info!("State validation completed successfully");
        }
        Ok(())
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> RhemaResult<StoreStats> {
        let agent_states = self.agent_states.read().await;
        let configurations = self.configuration_snapshots.read().await;

        let total_entries = agent_states.len() + configurations.len();
        let size_bytes = agent_states.values().map(|s| s.size_bytes).sum::<u64>()
            + configurations.values().map(|c| c.size_bytes).sum::<u64>();

        Ok(StoreStats {
            total_entries,
            size_bytes,
            last_backup: None,    // TODO: Track backup timestamps
            last_cleanup: None,   // TODO: Track cleanup timestamps
            validation_errors: 0, // TODO: Track validation errors
        })
    }
}

/// Agent performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformanceStats {
    pub total_agents: usize,
    pub active_agents: usize,
    pub avg_tasks_completed: f64,
    pub avg_success_rate: f64,
    pub avg_response_time: f64,
    pub last_updated: DateTime<Utc>,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            total_agents: 0,
            active_agents: 0,
            total_sessions: 0,
            active_sessions: 0,
            total_messages: 0,
            messages_per_second: 0.0,
            average_response_time_ms: 0.0,
            system_uptime_seconds: 0,
            memory_usage_bytes: 0,
            cpu_usage_percent: 0.0,
            last_updated: Utc::now(),
        }
    }
}

/// Stored state data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredStateData {
    agent_states: HashMap<String, StoredAgentState>,
    system_metrics: SystemMetrics,
    configurations: HashMap<String, StoredConfiguration>,
}
