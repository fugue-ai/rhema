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

use super::{PersistenceConfig, StoreStats, StorageBackend};
use crate::agent::real_time_coordination::{CoordinationSession, AdvancedSession, SessionStatus};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc};

/// Session store for persisting coordination sessions
pub struct SessionStore {
    config: PersistenceConfig,
    sessions: Arc<RwLock<HashMap<String, StoredSession>>>,
    advanced_sessions: Arc<RwLock<HashMap<String, StoredAdvancedSession>>>,
    file_path: Option<PathBuf>,
}

/// Stored session with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSession {
    pub session: CoordinationSession,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub size_bytes: u64,
}

/// Stored advanced session with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAdvancedSession {
    pub session: AdvancedSession,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub size_bytes: u64,
}

impl SessionStore {
    /// Create a new session store
    pub async fn new(config: PersistenceConfig) -> RhemaResult<Self> {
        let file_path = match &config.backend {
            StorageBackend::File => {
                let path = config.storage_path
                    .as_ref()
                    .map(|p| p.join("sessions"))
                    .unwrap_or_else(|| PathBuf::from("./data/sessions"));
                
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
            sessions: Arc::new(RwLock::new(HashMap::new())),
            advanced_sessions: Arc::new(RwLock::new(HashMap::new())),
            file_path,
        };

        // Load existing data
        store.load().await?;

        Ok(store)
    }

    /// Store a coordination session
    pub async fn store_session(&self, session: CoordinationSession) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&session)?.len() as u64;
        let now = Utc::now();
        
        let stored_session = StoredSession {
            session,
            created_at: now,
            updated_at: now,
            access_count: 0,
            last_accessed: now,
            size_bytes,
        };

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(stored_session.session.id.clone(), stored_session);
        }

        self.save().await?;
        Ok(())
    }

    /// Store an advanced session
    pub async fn store_advanced_session(&self, session: AdvancedSession) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&session)?.len() as u64;
        let now = Utc::now();
        
        let stored_session = StoredAdvancedSession {
            session,
            created_at: now,
            updated_at: now,
            access_count: 0,
            last_accessed: now,
            size_bytes,
        };

        {
            let mut sessions = self.advanced_sessions.write().await;
            sessions.insert(stored_session.session.id.clone(), stored_session);
        }

        self.save().await?;
        Ok(())
    }

    /// Retrieve a coordination session
    pub async fn get_session(&self, session_id: &str) -> Option<CoordinationSession> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(stored_session) = sessions.get_mut(session_id) {
            stored_session.access_count += 1;
            stored_session.last_accessed = Utc::now();
            Some(stored_session.session.clone())
        } else {
            None
        }
    }

    /// Retrieve an advanced session
    pub async fn get_advanced_session(&self, session_id: &str) -> Option<AdvancedSession> {
        let mut sessions = self.advanced_sessions.write().await;
        
        if let Some(stored_session) = sessions.get_mut(session_id) {
            stored_session.access_count += 1;
            stored_session.last_accessed = Utc::now();
            Some(stored_session.session.clone())
        } else {
            None
        }
    }

    /// Update a coordination session
    pub async fn update_session(&self, session: CoordinationSession) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&session)?.len() as u64;
        let now = Utc::now();
        
        let mut sessions = self.sessions.write().await;
        
        if let Some(stored_session) = sessions.get_mut(&session.id) {
            stored_session.session = session;
            stored_session.updated_at = now;
            stored_session.size_bytes = size_bytes;
        } else {
            return Err(rhema_core::RhemaError::NotFound(format!("Session {} not found", session.id)));
        }

        self.save().await?;
        Ok(())
    }

    /// Update an advanced session
    pub async fn update_advanced_session(&self, session: AdvancedSession) -> RhemaResult<()> {
        let size_bytes = serde_json::to_string(&session)?.len() as u64;
        let now = Utc::now();
        
        let mut sessions = self.advanced_sessions.write().await;
        
        if let Some(stored_session) = sessions.get_mut(&session.id) {
            stored_session.session = session;
            stored_session.updated_at = now;
            stored_session.size_bytes = size_bytes;
        } else {
            return Err(rhema_core::RhemaError::NotFound(format!("Advanced session {} not found", session.id)));
        }

        self.save().await?;
        Ok(())
    }

    /// Delete a session
    pub async fn delete_session(&self, session_id: &str) -> RhemaResult<()> {
        {
            let mut sessions = self.sessions.write().await;
            sessions.remove(session_id);
        }
        
        {
            let mut advanced_sessions = self.advanced_sessions.write().await;
            advanced_sessions.remove(session_id);
        }

        self.save().await?;
        Ok(())
    }

    /// List all sessions
    pub async fn list_sessions(&self) -> Vec<CoordinationSession> {
        let sessions = self.sessions.read().await;
        sessions.values().map(|s| s.session.clone()).collect()
    }

    /// List all advanced sessions
    pub async fn list_advanced_sessions(&self) -> Vec<AdvancedSession> {
        let sessions = self.advanced_sessions.read().await;
        sessions.values().map(|s| s.session.clone()).collect()
    }

    /// Get sessions by status
    pub async fn get_sessions_by_status(&self, status: SessionStatus) -> Vec<CoordinationSession> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| s.session.status == status)
            .map(|s| s.session.clone())
            .collect()
    }

    /// Get sessions by participant
    pub async fn get_sessions_by_participant(&self, agent_id: &str) -> Vec<CoordinationSession> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| s.session.participants.contains(&agent_id.to_string()))
            .map(|s| s.session.clone())
            .collect()
    }

    /// Load data from storage
    async fn load(&mut self) -> RhemaResult<()> {
        match &self.config.backend {
            StorageBackend::File => {
                if let Some(path) = &self.file_path {
                    if path.exists() {
                        let data = tokio::fs::read_to_string(path).await?;
                        let stored_data: StoredSessionData = serde_json::from_str(&data)?;
                        
                        let session_count = stored_data.sessions.len();
                        let advanced_session_count = stored_data.advanced_sessions.len();
                        
                        *self.sessions.write().await = stored_data.sessions;
                        *self.advanced_sessions.write().await = stored_data.advanced_sessions;
                        
                        info!("Loaded {} sessions and {} advanced sessions from storage", 
                              session_count, advanced_session_count);
                    }
                }
            }
            _ => {
                // For other backends, start with empty storage
                info!("Using in-memory session storage");
            }
        }
        Ok(())
    }

    /// Save data to storage
    async fn save(&self) -> RhemaResult<()> {
        match &self.config.backend {
            StorageBackend::File => {
                if let Some(path) = &self.file_path {
                    let sessions = self.sessions.read().await;
                    let advanced_sessions = self.advanced_sessions.read().await;
                    
                    let stored_data = StoredSessionData {
                        sessions: sessions.clone(),
                        advanced_sessions: advanced_sessions.clone(),
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
            let backup_path = self.file_path
                .as_ref()
                .map(|p| p.with_extension("backup"))
                .unwrap_or_else(|| PathBuf::from("./data/sessions.backup"));
            
            let sessions = self.sessions.read().await;
            let advanced_sessions = self.advanced_sessions.read().await;
            
            let stored_data = StoredSessionData {
                sessions: sessions.clone(),
                advanced_sessions: advanced_sessions.clone(),
            };
            
            let data = serde_json::to_string_pretty(&stored_data)?;
            tokio::fs::write(backup_path, data).await?;
            
            info!("Session backup completed");
        }
        Ok(())
    }

    /// Perform cleanup
    pub async fn cleanup(&self) -> RhemaResult<()> {
        if self.config.enable_cleanup {
            let cutoff_date = Utc::now() - chrono::Duration::days(self.config.data_retention_days as i64);
            
            {
                let mut sessions = self.sessions.write().await;
                sessions.retain(|_, stored_session| stored_session.updated_at > cutoff_date);
            }
            
            {
                let mut advanced_sessions = self.advanced_sessions.write().await;
                advanced_sessions.retain(|_, stored_session| stored_session.updated_at > cutoff_date);
            }
            
            self.save().await?;
            info!("Session cleanup completed");
        }
        Ok(())
    }

    /// Validate stored data
    pub async fn validate(&self) -> RhemaResult<()> {
        if self.config.enable_validation {
            let sessions = self.sessions.read().await;
            let advanced_sessions = self.advanced_sessions.read().await;
            
            for (id, stored_session) in sessions.iter() {
                if stored_session.session.id != *id {
                    return Err(rhema_core::RhemaError::Validation(format!("Session ID mismatch: {}", id)));
                }
            }
            
            for (id, stored_session) in advanced_sessions.iter() {
                if stored_session.session.id != *id {
                    return Err(rhema_core::RhemaError::Validation(format!("Advanced session ID mismatch: {}", id)));
                }
            }
            
            info!("Session validation completed successfully");
        }
        Ok(())
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> RhemaResult<StoreStats> {
        let sessions = self.sessions.read().await;
        let advanced_sessions = self.advanced_sessions.read().await;
        
        let total_entries = sessions.len() + advanced_sessions.len();
        let size_bytes = sessions.values().map(|s| s.size_bytes).sum::<u64>() +
                        advanced_sessions.values().map(|s| s.size_bytes).sum::<u64>();
        
        Ok(StoreStats {
            total_entries,
            size_bytes,
            last_backup: None, // TODO: Track backup timestamps
            last_cleanup: None, // TODO: Track cleanup timestamps
            validation_errors: 0, // TODO: Track validation errors
        })
    }
}

/// Stored session data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredSessionData {
    sessions: HashMap<String, StoredSession>,
    advanced_sessions: HashMap<String, StoredAdvancedSession>,
} 