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

pub mod state;
pub mod locks;
pub mod coordination;

pub use state::*;
pub use locks::*;
pub use coordination::*;

use crate::RhemaResult;
use crate::safety::SafetyValidator;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Agent coordination system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCoordinationConfig {
    /// Maximum number of concurrent agents per scope
    pub max_concurrent_agents: usize,
    
    /// Maximum time an agent can be blocked
    pub max_block_time: Duration,
    
    /// Lock timeout duration
    pub lock_timeout: Duration,
    
    /// Whether safety validation is enabled
    pub safety_validation_enabled: bool,
    
    /// Whether to use strict mode for safety validation
    pub strict_mode: bool,
    
    /// Whether to auto-recover from safety violations
    pub auto_recovery: bool,
}

impl Default for AgentCoordinationConfig {
    fn default() -> Self {
        Self {
            max_concurrent_agents: 3,
            max_block_time: Duration::from_secs(300), // 5 minutes
            lock_timeout: Duration::from_secs(60),    // 1 minute
            safety_validation_enabled: true,
            strict_mode: false,
            auto_recovery: true,
        }
    }
}

/// Main agent coordination service
pub struct AgentCoordinationService {
    config: AgentCoordinationConfig,
    agent_manager: AgentManager,
    lock_manager: LockManager,
    sync_coordinator: SyncCoordinator,
    safety_validator: SafetyValidator,
}

impl AgentCoordinationService {
    /// Create a new agent coordination service
    pub fn new(config: AgentCoordinationConfig) -> RhemaResult<Self> {
        let agent_manager = AgentManager::new(config.max_concurrent_agents, config.max_block_time);
        let lock_manager = LockManager::new(config.lock_timeout);
        let sync_coordinator = SyncCoordinator::new();
        let safety_validator = SafetyValidator::new();

        Ok(Self {
            config,
            agent_manager,
            lock_manager,
            sync_coordinator,
            safety_validator,
        })
    }

    /// Get agent manager reference
    pub fn agent_manager(&self) -> &AgentManager {
        &self.agent_manager
    }

    /// Get mutable agent manager reference
    pub fn agent_manager_mut(&mut self) -> &mut AgentManager {
        &mut self.agent_manager
    }

    /// Get lock manager reference
    pub fn lock_manager(&self) -> &LockManager {
        &self.lock_manager
    }

    /// Get mutable lock manager reference
    pub fn lock_manager_mut(&mut self) -> &mut LockManager {
        &mut self.lock_manager
    }

    /// Get sync coordinator reference
    pub fn sync_coordinator(&self) -> &SyncCoordinator {
        &self.sync_coordinator
    }

    /// Get mutable sync coordinator reference
    pub fn sync_coordinator_mut(&mut self) -> &mut SyncCoordinator {
        &mut self.sync_coordinator
    }

    /// Get safety validator reference
    pub fn safety_validator(&self) -> &SafetyValidator {
        &self.safety_validator
    }

    /// Get configuration reference
    pub fn config(&self) -> &AgentCoordinationConfig {
        &self.config
    }

    /// Run safety validation on the entire system
    pub async fn validate_safety(&self) -> RhemaResult<()> {
        if !self.config.safety_validation_enabled {
            return Ok(());
        }

        // Validate agent coordination
        self.safety_validator.validate_agent_coordination(
            self.agent_manager.agents(),
            self.lock_manager.locks(),
            self.config.max_concurrent_agents,
        )?;

        // Validate lock consistency
        self.safety_validator.validate_lock_consistency(
            self.lock_manager.locks(),
            &self.agent_manager.agent_ids(),
        )?;

        // Validate sync status consistency
        self.safety_validator.validate_sync_status_consistency(
            self.sync_coordinator.sync_status(),
            self.sync_coordinator.sync_dependencies(),
        )?;

        Ok(())
    }

    /// Handle agent join request
    pub async fn handle_agent_join(&mut self, agent_id: String) -> RhemaResult<()> {
        self.agent_manager.agent_join(agent_id).await?;
        self.validate_safety().await?;
        Ok(())
    }

    /// Handle agent leave request
    pub async fn handle_agent_leave(&mut self, agent_id: String) -> RhemaResult<()> {
        // Release all locks held by the agent
        self.lock_manager.release_agent_locks(&agent_id).await?;
        
        self.agent_manager.agent_leave(agent_id).await?;
        self.validate_safety().await?;
        Ok(())
    }

    /// Handle context modification request
    pub async fn handle_context_modification(
        &mut self,
        agent_id: &str,
        scope_path: &str,
        _file: &str,
        content: &str,
    ) -> RhemaResult<()> {
        // Validate agent exists and can work
        let agent_state = self.agent_manager.get_agent_state(agent_id)
            .ok_or_else(|| AgentError::AgentNotFound(agent_id.to_string()))?;

        if agent_state != AgentState::Working && agent_state != AgentState::Idle {
            return Err(AgentError::InvalidAgentState(agent_id.to_string(), agent_state).into());
        }

        // Try to acquire lock
        let lock_acquired = self.lock_manager.acquire_lock(scope_path, agent_id).await?;
        if !lock_acquired {
            return Err(AgentError::AgentBlockedTooLong(scope_path.to_string()).into());
        }

        // Set agent to working state
        self.agent_manager.set_agent_state(agent_id, AgentState::Working).await?;

        // Validate content
        self.safety_validator.validate_yaml_content(content)?;

        // Perform the modification (this would be handled by the caller)
        // For now, we just validate that the operation is safe

        self.validate_safety().await?;
        Ok(())
    }

    /// Handle sync request
    pub async fn handle_sync_request(&mut self, scope_path: &str) -> RhemaResult<()> {
        // Check if dependencies are ready
        let deps_ready = self.sync_coordinator.check_sync_dependencies(scope_path).await?;
        if !deps_ready {
            return Err(SyncError::DependenciesNotReady(scope_path.to_string()).into());
        }

        // Start sync
        self.sync_coordinator.start_sync(scope_path).await?;
        self.validate_safety().await?;
        Ok(())
    }

    /// Complete sync for a scope
    pub async fn complete_sync(&mut self, scope_path: &str) -> RhemaResult<()> {
        self.sync_coordinator.complete_sync(scope_path).await?;
        self.validate_safety().await?;
        Ok(())
    }

    /// Fail sync for a scope
    pub async fn fail_sync(&mut self, scope_path: &str, error: String) -> RhemaResult<()> {
        self.sync_coordinator.fail_sync(scope_path, error).await?;
        self.validate_safety().await?;
        Ok(())
    }

    /// Release lock for a scope
    pub async fn release_lock(&mut self, scope_path: &str, agent_id: &str) -> RhemaResult<()> {
        self.lock_manager.release_lock(scope_path, agent_id).await?;
        
        // Set agent back to idle if it's not working on other scopes
        if !self.lock_manager.has_agent_locks(agent_id) {
            self.agent_manager.set_agent_state(agent_id, AgentState::Idle).await?;
        }

        self.validate_safety().await?;
        Ok(())
    }

    /// Get system status
    pub fn get_system_status(&self) -> SystemStatus {
        SystemStatus {
            total_agents: self.agent_manager.agents().len(),
            active_agents: self.agent_manager.active_agents().len(),
            total_locks: self.lock_manager.active_locks().len(),
            syncing_scopes: self.sync_coordinator.syncing_scopes().len(),
            completed_syncs: self.sync_coordinator.completed_syncs().len(),
            failed_syncs: self.sync_coordinator.failed_syncs().len(),
        }
    }

    /// Cleanup expired resources
    pub async fn cleanup(&mut self) -> RhemaResult<()> {
        // Cleanup expired locks
        self.lock_manager.cleanup_expired_locks().await?;
        
        // Check agent progress
        self.agent_manager.check_agent_progress().await?;
        
        // Validate safety after cleanup
        self.validate_safety().await?;
        Ok(())
    }
}

/// System status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_locks: usize,
    pub syncing_scopes: usize,
    pub completed_syncs: usize,
    pub failed_syncs: usize,
}

/// Agent request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentRequest {
    Join { agent_id: String },
    Leave { agent_id: String },
    ModifyContext {
        agent_id: String,
        scope_path: String,
        file: String,
        content: String,
    },
    Sync { scope_path: String },
    CompleteSync { scope_path: String },
    FailSync { scope_path: String, error: String },
    ReleaseLock { scope_path: String, agent_id: String },
    GetStatus,
}

/// Agent response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentResponse {
    Success,
    Error(String),
    Status(SystemStatus),
} 