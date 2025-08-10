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

pub mod consensus_store;
pub mod session_store;
pub mod state_manager;

pub use consensus_store::ConsensusStore;
pub use session_store::SessionStore;
pub use state_manager::StateManager;

use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Persistence configuration for production environments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Storage backend type
    pub backend: StorageBackend,
    /// Storage path for file-based backends
    pub storage_path: Option<PathBuf>,
    /// Database connection string for database backends
    pub connection_string: Option<String>,
    /// Enable automatic backups
    pub enable_backups: bool,
    /// Backup interval in hours
    pub backup_interval_hours: u64,
    /// Backup retention days
    pub backup_retention_days: u64,
    /// Enable data compression
    pub enable_compression: bool,
    /// Enable data encryption
    pub enable_encryption: bool,
    /// Encryption key (base64 encoded)
    pub encryption_key: Option<String>,
    /// Maximum data size in bytes
    pub max_data_size_bytes: u64,
    /// Enable data validation
    pub enable_validation: bool,
    /// Enable automatic cleanup
    pub enable_cleanup: bool,
    /// Cleanup interval in hours
    pub cleanup_interval_hours: u64,
    /// Data retention days
    pub data_retention_days: u64,
}

/// Storage backend types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageBackend {
    /// File-based storage (JSON/YAML)
    File,
    /// SQLite database
    Sqlite,
    /// PostgreSQL database
    Postgres,
    /// Redis database
    Redis,
    /// In-memory storage (for testing)
    Memory,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::File,
            storage_path: Some(PathBuf::from("./data")),
            connection_string: None,
            enable_backups: true,
            backup_interval_hours: 24,
            backup_retention_days: 30,
            enable_compression: true,
            enable_encryption: false,
            encryption_key: None,
            max_data_size_bytes: 100 * 1024 * 1024, // 100MB
            enable_validation: true,
            enable_cleanup: true,
            cleanup_interval_hours: 168, // 1 week
            data_retention_days: 90,
        }
    }
}

/// Persistence manager for coordinating all storage operations
pub struct PersistenceManager {
    config: PersistenceConfig,
    session_store: SessionStore,
    consensus_store: ConsensusStore,
    state_manager: StateManager,
}

impl PersistenceManager {
    /// Create a new persistence manager
    pub async fn new(config: PersistenceConfig) -> RhemaResult<Self> {
        let session_store = SessionStore::new(config.clone()).await?;
        let consensus_store = ConsensusStore::new(config.clone()).await?;
        let state_manager = StateManager::new(config.clone()).await?;

        Ok(Self {
            config,
            session_store,
            consensus_store,
            state_manager,
        })
    }

    /// Get session store reference
    pub fn session_store(&self) -> &SessionStore {
        &self.session_store
    }

    /// Get consensus store reference
    pub fn consensus_store(&self) -> &ConsensusStore {
        &self.consensus_store
    }

    /// Get state manager reference
    pub fn state_manager(&self) -> &StateManager {
        &self.state_manager
    }

    /// Perform backup of all data
    pub async fn backup(&self) -> RhemaResult<()> {
        self.session_store.backup().await?;
        self.consensus_store.backup().await?;
        self.state_manager.backup().await?;
        Ok(())
    }

    /// Perform cleanup of old data
    pub async fn cleanup(&self) -> RhemaResult<()> {
        self.session_store.cleanup().await?;
        self.consensus_store.cleanup().await?;
        self.state_manager.cleanup().await?;
        Ok(())
    }

    /// Validate all stored data
    pub async fn validate(&self) -> RhemaResult<()> {
        self.session_store.validate().await?;
        self.consensus_store.validate().await?;
        self.state_manager.validate().await?;
        Ok(())
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> RhemaResult<StorageStats> {
        let session_stats = self.session_store.get_stats().await?;
        let consensus_stats = self.consensus_store.get_stats().await?;
        let state_stats = self.state_manager.get_stats().await?;

        Ok(StorageStats {
            session_stats: session_stats.clone(),
            consensus_stats: consensus_stats.clone(),
            state_stats: state_stats.clone(),
            total_size_bytes: session_stats.size_bytes
                + consensus_stats.size_bytes
                + state_stats.size_bytes,
        })
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub session_stats: StoreStats,
    pub consensus_stats: StoreStats,
    pub state_stats: StoreStats,
    pub total_size_bytes: u64,
}

/// Individual store statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreStats {
    pub total_entries: usize,
    pub size_bytes: u64,
    pub last_backup: Option<chrono::DateTime<chrono::Utc>>,
    pub last_cleanup: Option<chrono::DateTime<chrono::Utc>>,
    pub validation_errors: usize,
}
