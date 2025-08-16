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

use crate::scope_loader::{
    types::PackageManager, GlobalScopeLoaderConfig, PluginRegistry, ScopeContext,
    ScopeLoaderConfigManager, ScopeLoaderService, ScopeSuggestion, ScopeType,
};
use crate::{
    audit::{log_audit_event, AuditEvent, AuditEventType, AuditSeverity},
    validation::ValidationRules,
    RhemaError, RhemaResult,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use uuid::Uuid;

/// Integration status for scope loader
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntegrationStatus {
    /// Not integrated
    NotIntegrated,
    /// Integration in progress
    Integrating,
    /// Successfully integrated
    Integrated,
    /// Integration failed
    Failed(String),
}

/// Scope synchronization status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    /// Not synchronized
    NotSynced,
    /// Synchronization in progress
    Syncing,
    /// Successfully synchronized
    Synced,
    /// Synchronization failed
    Failed(String),
    /// Conflicts detected
    Conflicts(Vec<ScopeConflict>),
}

/// Scope conflict information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeConflict {
    /// Conflict ID
    pub id: String,
    /// Scope that has conflicts
    pub scope_path: PathBuf,
    /// Conflicting suggestions
    pub conflicting_suggestions: Vec<ScopeSuggestion>,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Timestamp when conflict was detected
    pub detected_at: DateTime<Utc>,
    /// Resolution status
    pub resolution_status: ConflictResolutionStatus,
}

/// Types of scope conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    /// Overlapping scope boundaries
    OverlappingBoundaries,
    /// Conflicting scope types
    ConflictingTypes,
    /// Duplicate scope names
    DuplicateNames,
    /// Conflicting dependencies
    ConflictingDependencies,
    /// Custom conflict
    Custom(String),
}

/// Conflict resolution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStatus {
    /// Not resolved
    NotResolved,
    /// Auto-resolved
    AutoResolved,
    /// Manually resolved
    ManuallyResolved,
    /// Resolution in progress
    Resolving,
}

/// Repository integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryIntegrationConfig {
    /// Repository path
    pub repository_path: PathBuf,
    /// Auto-create scopes on initialization
    pub auto_create_scopes: bool,
    /// Auto-sync scopes across team members
    pub auto_sync_scopes: bool,
    /// Conflict resolution strategy
    pub conflict_resolution: ConflictResolutionStrategy,
    /// Integration hooks
    pub hooks: IntegrationHooks,
    /// Team member configurations
    pub team_members: Vec<TeamMemberConfig>,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStrategy {
    /// Auto-resolve conflicts
    Auto,
    /// Require manual resolution
    Manual,
    /// Use team consensus
    TeamConsensus,
    /// Use priority-based resolution
    PriorityBased,
}

/// Integration hooks configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationHooks {
    /// Pre-scope creation hooks
    pub pre_scope_creation: Vec<String>,
    /// Post-scope creation hooks
    pub post_scope_creation: Vec<String>,
    /// Pre-sync hooks
    pub pre_sync: Vec<String>,
    /// Post-sync hooks
    pub post_sync: Vec<String>,
    /// Conflict detection hooks
    pub conflict_detection: Vec<String>,
}

/// Team member configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberConfig {
    /// Team member ID
    pub id: String,
    /// Team member name
    pub name: String,
    /// Team member email
    pub email: String,
    /// Role in scope management
    pub role: TeamRole,
    /// Permissions
    pub permissions: Vec<ScopePermission>,
    /// Last sync timestamp
    pub last_sync: Option<DateTime<Utc>>,
}

/// Team roles for scope management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TeamRole {
    /// Administrator - full access
    Administrator,
    /// Maintainer - can create and modify scopes
    Maintainer,
    /// Contributor - can suggest scopes
    Contributor,
    /// Viewer - read-only access
    Viewer,
}

/// Scope permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScopePermission {
    /// Create scopes
    Create,
    /// Modify scopes
    Modify,
    /// Delete scopes
    Delete,
    /// View scopes
    View,
    /// Sync scopes
    Sync,
    /// Resolve conflicts
    ResolveConflicts,
}

/// Scope integration manager
pub struct ScopeIntegrationManager {
    /// Integration configuration
    pub config: RepositoryIntegrationConfig,
    /// Scope loader service
    pub scope_loader: Arc<ScopeLoaderService>,
    /// Plugin registry
    pub plugin_registry: Arc<PluginRegistry>,
    /// Integration status
    pub integration_status: Arc<Mutex<IntegrationStatus>>,
    /// Sync status
    pub sync_status: Arc<Mutex<SyncStatus>>,
    /// Active conflicts
    pub active_conflicts: Arc<Mutex<Vec<ScopeConflict>>>,
    /// Event channel for real-time updates
    pub event_tx: mpsc::UnboundedSender<IntegrationEvent>,
    /// Team member cache
    pub team_members: Arc<Mutex<HashMap<String, TeamMemberConfig>>>,
}

/// Integration events for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationEvent {
    /// Scope created
    ScopeCreated {
        scope_path: PathBuf,
        suggestion: ScopeSuggestion,
        created_by: String,
        timestamp: DateTime<Utc>,
    },
    /// Scope updated
    ScopeUpdated {
        scope_path: PathBuf,
        old_suggestion: ScopeSuggestion,
        new_suggestion: ScopeSuggestion,
        updated_by: String,
        timestamp: DateTime<Utc>,
    },
    /// Scope deleted
    ScopeDeleted {
        scope_path: PathBuf,
        deleted_by: String,
        timestamp: DateTime<Utc>,
    },
    /// Sync started
    SyncStarted {
        initiator: String,
        timestamp: DateTime<Utc>,
    },
    /// Sync completed
    SyncCompleted {
        initiator: String,
        scopes_synced: usize,
        conflicts_resolved: usize,
        timestamp: DateTime<Utc>,
    },
    /// Conflict detected
    ConflictDetected {
        conflict: ScopeConflict,
        timestamp: DateTime<Utc>,
    },
    /// Conflict resolved
    ConflictResolved {
        conflict_id: String,
        resolution_method: ConflictResolutionStatus,
        resolved_by: String,
        timestamp: DateTime<Utc>,
    },
}

impl ScopeIntegrationManager {
    /// Create a new scope integration manager
    pub fn new(
        config: RepositoryIntegrationConfig,
        scope_loader: Arc<ScopeLoaderService>,
        plugin_registry: Arc<PluginRegistry>,
    ) -> RhemaResult<(Self, mpsc::UnboundedReceiver<IntegrationEvent>)> {
        // Validate repository path
        ValidationRules::validate_scope_path_static(&config.repository_path)?;

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let manager = Self {
            config,
            scope_loader,
            plugin_registry,
            integration_status: Arc::new(Mutex::new(IntegrationStatus::NotIntegrated)),
            sync_status: Arc::new(Mutex::new(SyncStatus::NotSynced)),
            active_conflicts: Arc::new(Mutex::new(Vec::new())),
            event_tx,
            team_members: Arc::new(Mutex::new(HashMap::new())),
        };

        Ok((manager, event_rx))
    }

    /// Initialize integration with repository
    pub async fn initialize_integration(&self) -> RhemaResult<()> {
        let mut status = self.integration_status.lock().unwrap();
        *status = IntegrationStatus::Integrating;

        // Log integration start
        let event = AuditEvent::new(
            AuditEventType::SystemEvent,
            AuditSeverity::Info,
            "scope_integration_started".to_string(),
            true,
        )
        .with_detail(
            "repository_path".to_string(),
            self.config.repository_path.to_string_lossy().to_string(),
        );

        log_audit_event(event)?;

        // Initialize team members
        self.initialize_team_members().await?;

        // Run pre-integration hooks
        self.run_hooks(&self.config.hooks.pre_scope_creation)
            .await?;

        // Auto-create scopes if enabled
        if self.config.auto_create_scopes {
            self.auto_create_scopes().await?;
        }

        // Run post-integration hooks
        self.run_hooks(&self.config.hooks.post_scope_creation)
            .await?;

        *status = IntegrationStatus::Integrated;

        // Log integration completion
        let event = AuditEvent::new(
            AuditEventType::SystemEvent,
            AuditSeverity::Info,
            "scope_integration_completed".to_string(),
            true,
        )
        .with_detail(
            "repository_path".to_string(),
            self.config.repository_path.to_string_lossy().to_string(),
        );

        log_audit_event(event)?;

        Ok(())
    }

    /// Auto-create scopes based on repository analysis
    pub async fn auto_create_scopes(&self) -> RhemaResult<Vec<ScopeSuggestion>> {
        let suggestions = Vec::new();

        // Analyze repository structure
        let context = ScopeContext {
            scope_name: "repository".to_string(),
            package_manager: PackageManager::Cargo, // Default to Cargo for Rust projects
            dependencies: Vec::new(),
            scripts: HashMap::new(),
            metadata: HashMap::new(),
        };

        // Get scope suggestions from all plugins
        for plugin in self.plugin_registry.get_all_plugins() {
            // For now, we'll use a placeholder implementation since the plugin trait doesn't have an analyze method
            // In a real implementation, you would call the appropriate plugin methods
            tracing::info!("Plugin {} would analyze context", plugin.metadata().name);
        }

        Ok(suggestions)
    }

    /// Synchronize scopes across team members
    pub async fn sync_scopes(&self, initiator: &str) -> RhemaResult<SyncResult> {
        let mut sync_status = self.sync_status.lock().unwrap();
        *sync_status = SyncStatus::Syncing;

        // Log sync start
        let event = AuditEvent::new(
            AuditEventType::SystemEvent,
            AuditSeverity::Info,
            "scope_sync_started".to_string(),
            true,
        )
        .with_detail("initiator".to_string(), initiator.to_string());

        log_audit_event(event)?;

        // Run pre-sync hooks
        self.run_hooks(&self.config.hooks.pre_sync).await?;

        let mut conflicts = Vec::new();
        let mut scopes_synced = 0;

        // Get all team member scopes
        for team_member in self.config.team_members.iter() {
            match self.sync_team_member_scopes(team_member).await {
                Ok(synced_count) => {
                    scopes_synced += synced_count;
                }
                Err(e) => {
                    tracing::warn!("Failed to sync scopes for {}: {}", team_member.name, e);
                }
            }
        }

        // Detect and resolve conflicts
        conflicts = self.detect_conflicts().await?;

        if !conflicts.is_empty() {
            match self.config.conflict_resolution {
                ConflictResolutionStrategy::Auto => {
                    self.auto_resolve_conflicts(&conflicts).await?;
                }
                ConflictResolutionStrategy::Manual => {
                    // Store conflicts for manual resolution
                    let mut active_conflicts = self.active_conflicts.lock().unwrap();
                    active_conflicts.extend(conflicts.clone());
                }
                ConflictResolutionStrategy::TeamConsensus => {
                    self.resolve_conflicts_by_consensus(&conflicts).await?;
                }
                ConflictResolutionStrategy::PriorityBased => {
                    self.resolve_conflicts_by_priority(&conflicts).await?;
                }
            }
        }

        // Run post-sync hooks
        self.run_hooks(&self.config.hooks.post_sync).await?;

        let conflicts_resolved = conflicts.len();

        // Update sync status
        if conflicts.is_empty() {
            *sync_status = SyncStatus::Synced;
        } else {
            *sync_status = SyncStatus::Conflicts(conflicts.clone());
        }

        // Log sync completion
        let event = AuditEvent::new(
            AuditEventType::SystemEvent,
            AuditSeverity::Info,
            "scope_sync_completed".to_string(),
            true,
        )
        .with_detail("initiator".to_string(), initiator.to_string())
        .with_detail("scopes_synced".to_string(), scopes_synced.to_string())
        .with_detail(
            "conflicts_resolved".to_string(),
            conflicts_resolved.to_string(),
        );

        log_audit_event(event)?;

        // Send sync completion event
        let _ = self.event_tx.send(IntegrationEvent::SyncCompleted {
            initiator: initiator.to_string(),
            scopes_synced,
            conflicts_resolved,
            timestamp: Utc::now(),
        });

        Ok(SyncResult {
            scopes_synced,
            conflicts_detected: conflicts.len(),
            conflicts_resolved,
        })
    }

    /// Detect conflicts between scope suggestions
    async fn detect_conflicts(&self) -> RhemaResult<Vec<ScopeConflict>> {
        let mut conflicts = Vec::new();

        // Get all scope suggestions
        let all_suggestions = self.get_all_suggestions().await?;

        // Check for overlapping boundaries
        for i in 0..all_suggestions.len() {
            for j in (i + 1)..all_suggestions.len() {
                let suggestion1 = &all_suggestions[i];
                let suggestion2 = &all_suggestions[j];

                if self.suggestions_overlap(suggestion1, suggestion2) {
                    let conflict = ScopeConflict {
                        id: Uuid::new_v4().to_string(),
                        scope_path: suggestion1.path.clone(),
                        conflicting_suggestions: vec![suggestion1.clone(), suggestion2.clone()],
                        conflict_type: ConflictType::OverlappingBoundaries,
                        detected_at: Utc::now(),
                        resolution_status: ConflictResolutionStatus::NotResolved,
                    };

                    conflicts.push(conflict);
                }
            }
        }

        // Check for duplicate names
        let mut name_groups: HashMap<String, Vec<&ScopeSuggestion>> = HashMap::new();
        for suggestion in &all_suggestions {
            name_groups
                .entry(suggestion.name.clone())
                .or_default()
                .push(suggestion);
        }

        for (name, suggestions) in name_groups {
            if suggestions.len() > 1 {
                let conflict = ScopeConflict {
                    id: Uuid::new_v4().to_string(),
                    scope_path: suggestions[0].path.clone(),
                    conflicting_suggestions: suggestions.iter().map(|s| (*s).clone()).collect(),
                    conflict_type: ConflictType::DuplicateNames,
                    detected_at: Utc::now(),
                    resolution_status: ConflictResolutionStatus::NotResolved,
                };

                conflicts.push(conflict);
            }
        }

        Ok(conflicts)
    }

    /// Auto-resolve conflicts
    async fn auto_resolve_conflicts(&self, conflicts: &[ScopeConflict]) -> RhemaResult<()> {
        for conflict in conflicts {
            match conflict.conflict_type {
                ConflictType::OverlappingBoundaries => {
                    // Merge overlapping scopes
                    self.merge_overlapping_scopes(&conflict.conflicting_suggestions)
                        .await?;
                }
                ConflictType::DuplicateNames => {
                    // Rename duplicate scopes
                    self.rename_duplicate_scopes(&conflict.conflicting_suggestions)
                        .await?;
                }
                ConflictType::ConflictingTypes => {
                    // Use the most specific type
                    self.resolve_conflicting_types(&conflict.conflicting_suggestions)
                        .await?;
                }
                _ => {
                    // For other conflicts, use the first suggestion
                    if let Some(first_suggestion) = conflict.conflicting_suggestions.first() {
                        self.create_scope(first_suggestion).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Create a scope from suggestion
    async fn create_scope(&self, suggestion: &ScopeSuggestion) -> RhemaResult<()> {
        // Validate suggestion
        ValidationRules::validate_scope_path_static(&suggestion.path)?;
        ValidationRules::validate_title_static(&suggestion.name)?;

        // Create scope directory
        tokio::fs::create_dir_all(&suggestion.path)
            .await
            .map_err(|e| RhemaError::IoError(e))?;

        // Create scope configuration
        let scope_config =
            serde_yaml::to_string(&suggestion).map_err(|e| RhemaError::InvalidYaml {
                file: suggestion.path.to_string_lossy().to_string(),
                message: e.to_string(),
            })?;

        let config_path = suggestion.path.join("scope.yaml");
        tokio::fs::write(&config_path, scope_config)
            .await
            .map_err(|e| RhemaError::IoError(e))?;

        // Log scope creation
        let event = AuditEvent::new(
            AuditEventType::FileCreate,
            AuditSeverity::Info,
            "scope_created".to_string(),
            true,
        )
        .with_resource(suggestion.path.to_string_lossy().to_string())
        .with_detail("scope_name".to_string(), suggestion.name.clone())
        .with_detail(
            "scope_type".to_string(),
            format!("{:?}", suggestion.scope_type),
        );

        log_audit_event(event)?;

        // Send scope creation event
        let _ = self.event_tx.send(IntegrationEvent::ScopeCreated {
            scope_path: suggestion.path.clone(),
            suggestion: suggestion.clone(),
            created_by: "system".to_string(),
            timestamp: Utc::now(),
        });

        Ok(())
    }

    /// Validate a scope suggestion
    async fn validate_suggestion(&self, suggestion: &ScopeSuggestion) -> RhemaResult<bool> {
        // Check if scope already exists
        if suggestion.path.exists() {
            return Ok(false);
        }

        // Validate suggestion properties
        ValidationRules::validate_scope_path_static(&suggestion.path)?;
        ValidationRules::validate_title_static(&suggestion.name)?;

        // Check confidence score
        if suggestion.confidence < 0.5 {
            return Ok(false);
        }

        Ok(true)
    }

    /// Check if two suggestions overlap
    fn suggestions_overlap(
        &self,
        suggestion1: &ScopeSuggestion,
        suggestion2: &ScopeSuggestion,
    ) -> bool {
        suggestion1.path.starts_with(&suggestion2.path)
            || suggestion2.path.starts_with(&suggestion1.path)
    }

    /// Initialize team members
    async fn initialize_team_members(&self) -> RhemaResult<()> {
        let mut team_members = self.team_members.lock().unwrap();

        for member in &self.config.team_members {
            team_members.insert(member.id.clone(), member.clone());
        }

        Ok(())
    }

    /// Run integration hooks
    async fn run_hooks(&self, hooks: &[String]) -> RhemaResult<()> {
        for hook in hooks {
            // Execute hook (placeholder implementation)
            tracing::info!("Running hook: {}", hook);
        }

        Ok(())
    }

    /// Sync team member scopes
    async fn sync_team_member_scopes(&self, team_member: &TeamMemberConfig) -> RhemaResult<usize> {
        // Placeholder implementation
        Ok(0)
    }

    /// Get all scope suggestions
    async fn get_all_suggestions(&self) -> RhemaResult<Vec<ScopeSuggestion>> {
        // Placeholder implementation
        Ok(Vec::new())
    }

    /// Merge overlapping scopes
    async fn merge_overlapping_scopes(&self, suggestions: &[ScopeSuggestion]) -> RhemaResult<()> {
        // Placeholder implementation
        Ok(())
    }

    /// Rename duplicate scopes
    async fn rename_duplicate_scopes(&self, suggestions: &[ScopeSuggestion]) -> RhemaResult<()> {
        // Placeholder implementation
        Ok(())
    }

    /// Resolve conflicting types
    async fn resolve_conflicting_types(&self, suggestions: &[ScopeSuggestion]) -> RhemaResult<()> {
        // Placeholder implementation
        Ok(())
    }

    /// Resolve conflicts by team consensus
    async fn resolve_conflicts_by_consensus(&self, conflicts: &[ScopeConflict]) -> RhemaResult<()> {
        // Placeholder implementation
        Ok(())
    }

    /// Resolve conflicts by priority
    async fn resolve_conflicts_by_priority(&self, conflicts: &[ScopeConflict]) -> RhemaResult<()> {
        // Placeholder implementation
        Ok(())
    }

    /// Get integration status
    pub fn get_integration_status(&self) -> IntegrationStatus {
        self.integration_status.lock().unwrap().clone()
    }

    /// Get sync status
    pub fn get_sync_status(&self) -> SyncStatus {
        self.sync_status.lock().unwrap().clone()
    }

    /// Get active conflicts
    pub fn get_active_conflicts(&self) -> Vec<ScopeConflict> {
        self.active_conflicts.lock().unwrap().clone()
    }
}

/// Result of scope synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// Number of scopes synced
    pub scopes_synced: usize,
    /// Number of conflicts detected
    pub conflicts_detected: usize,
    /// Number of conflicts resolved
    pub conflicts_resolved: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_integration_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let config = RepositoryIntegrationConfig {
            repository_path: temp_dir.path().to_path_buf(),
            auto_create_scopes: true,
            auto_sync_scopes: true,
            conflict_resolution: ConflictResolutionStrategy::Auto,
            hooks: IntegrationHooks {
                pre_scope_creation: vec![],
                post_scope_creation: vec![],
                pre_sync: vec![],
                post_sync: vec![],
                conflict_detection: vec![],
            },
            team_members: vec![],
        };

        let scope_loader = Arc::new(ScopeLoaderService::new_with_default_config(
            PluginRegistry::new(),
        ));
        let plugin_registry = Arc::new(PluginRegistry::new());

        let (manager, _event_rx) =
            ScopeIntegrationManager::new(config, scope_loader, plugin_registry).unwrap();

        assert_eq!(
            manager.get_integration_status(),
            IntegrationStatus::NotIntegrated
        );
        assert_eq!(manager.get_sync_status(), SyncStatus::NotSynced);
    }

    #[test]
    fn test_conflict_detection() {
        let manager = ScopeIntegrationManager {
            config: RepositoryIntegrationConfig {
                repository_path: PathBuf::from("test"),
                auto_create_scopes: false,
                auto_sync_scopes: false,
                conflict_resolution: ConflictResolutionStrategy::Auto,
                hooks: IntegrationHooks {
                    pre_scope_creation: vec![],
                    post_scope_creation: vec![],
                    pre_sync: vec![],
                    post_sync: vec![],
                    conflict_detection: vec![],
                },
                team_members: vec![],
            },
            scope_loader: Arc::new(ScopeLoaderService::new_with_default_config(
                PluginRegistry::new(),
            )),
            plugin_registry: Arc::new(PluginRegistry::new()),
            integration_status: Arc::new(Mutex::new(IntegrationStatus::NotIntegrated)),
            sync_status: Arc::new(Mutex::new(SyncStatus::NotSynced)),
            active_conflicts: Arc::new(Mutex::new(Vec::new())),
            event_tx: mpsc::unbounded_channel().0,
            team_members: Arc::new(Mutex::new(HashMap::new())),
        };

        let suggestion1 = ScopeSuggestion {
            path: PathBuf::from("src"),
            name: "source".to_string(),
            scope_type: ScopeType::Library,
            confidence: 0.8,
            reasoning: "Source code".to_string(),
            files: Vec::new(),
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        };

        let suggestion2 = ScopeSuggestion {
            path: PathBuf::from("src/utils"),
            name: "utils".to_string(),
            scope_type: ScopeType::Library,
            confidence: 0.7,
            reasoning: "Utility functions".to_string(),
            files: Vec::new(),
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        };

        assert!(manager.suggestions_overlap(&suggestion1, &suggestion2));
        assert!(manager.suggestions_overlap(&suggestion2, &suggestion1));
    }
}
