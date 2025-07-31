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

pub mod invariants;
pub mod validator;

pub use invariants::*;


use crate::RhemaResult;
use crate::agent::{AgentState, SyncStatus};
use std::collections::HashMap;
use thiserror::Error;

/// Safety violation types as defined in the TLA+ specification
#[derive(Debug, Error)]
pub enum SafetyViolation {
    #[error("Context consistency violation: {0}")]
    ContextConsistency(String),
    
    #[error("Dependency integrity violation: {0}")]
    DependencyIntegrity(String),
    
    #[error("Agent coordination violation: {0}")]
    AgentCoordination(String),
    
    #[error("Lock consistency violation: {0}")]
    LockConsistency(String),
    
    #[error("Sync status consistency violation: {0}")]
    SyncStatusConsistency(String),
    
    #[error("Resource bounds violation: {0}")]
    ResourceBounds(String),
    
    #[error("Circular dependency violation: {0}")]
    CircularDependency(String),
    
    #[error("Deadlock violation: {0}")]
    Deadlock(String),
}

/// Safety validator for enforcing all TLA+ safety invariants
pub struct SafetyValidator {
    context_validator: ContextValidator,
    dependency_validator: DependencyValidator,
    agent_validator: AgentValidator,
    lock_validator: LockValidator,
    sync_validator: SyncValidator,
}

impl SafetyValidator {
    /// Create a new safety validator
    pub fn new() -> Self {
        Self {
            context_validator: ContextValidator::new(),
            dependency_validator: DependencyValidator::new(),
            agent_validator: AgentValidator::new(),
            lock_validator: LockValidator::new(),
            sync_validator: SyncValidator::new(),
        }
    }

    /// Validate YAML content
    pub fn validate_yaml_content(&self, content: &str) -> RhemaResult<()> {
        self.context_validator.validate_yaml_content(content)
    }

    /// Validate scope references
    pub fn validate_scope_references(&self, scope: &str, all_scopes: &[String]) -> RhemaResult<()> {
        self.context_validator.validate_scope_references(scope, all_scopes)
    }

    /// Validate no circular dependencies
    pub fn validate_no_circular_dependencies(&self, dependencies: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
        self.dependency_validator.validate_no_circular_dependencies(dependencies)
    }

    /// Validate dependency graph
    pub fn validate_dependency_graph(&self, graph: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
        self.dependency_validator.validate_dependency_graph(graph)
    }

    /// Validate dependency bounds
    pub fn validate_dependency_bounds(&self, deps: &[String], max_deps: usize) -> RhemaResult<()> {
        self.dependency_validator.validate_dependency_bounds(deps, max_deps)
    }

    /// Validate no self dependencies
    pub fn validate_no_self_dependencies(&self, scope: &str, deps: &[String]) -> RhemaResult<()> {
        self.dependency_validator.validate_no_self_dependencies(scope, deps)
    }

    /// Validate agent states
    pub fn validate_agent_states(&self, agents: &HashMap<String, AgentState>) -> RhemaResult<()> {
        self.agent_validator.validate_agent_states(agents)
    }

    /// Validate concurrent agents
    pub fn validate_concurrent_agents(&self, locks: &HashMap<String, Option<String>>, max_concurrent: usize) -> RhemaResult<()> {
        self.agent_validator.validate_concurrent_agents(locks, max_concurrent)
    }

    /// Validate agent progress
    pub fn validate_agent_progress(&self, agent_id: &str, state: &AgentState, max_block_time: std::time::Duration) -> RhemaResult<()> {
        self.agent_validator.validate_agent_progress(agent_id, state, max_block_time)
    }

    /// Validate lock ownership
    pub fn validate_lock_ownership(&self, locks: &HashMap<String, Option<String>>, agents: &[String]) -> RhemaResult<()> {
        self.lock_validator.validate_lock_ownership(locks, agents)
    }

    /// Validate one lock per agent
    pub fn validate_one_lock_per_agent(&self, locks: &HashMap<String, Option<String>>) -> RhemaResult<()> {
        self.lock_validator.validate_one_lock_per_agent(locks)
    }

    /// Validate lock timeouts
    pub fn validate_lock_timeouts(&self, locks: &HashMap<String, Option<String>>, timeouts: &HashMap<String, std::time::Instant>) -> RhemaResult<()> {
        self.lock_validator.validate_lock_timeouts(locks, timeouts)
    }

    /// Validate sync status consistency
    pub fn validate_sync_status_consistency(&self, sync_status: &HashMap<String, SyncStatus>, sync_dependencies: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
        self.sync_validator.validate_sync_status_consistency(sync_status, sync_dependencies)
    }

    /// Validate all safety invariants for the entire system
    pub fn validate_all_safety_invariants(
        &self,
        agents: &HashMap<String, AgentState>,
        locks: &HashMap<String, Option<String>>,
        sync_status: &HashMap<String, SyncStatus>,
        sync_dependencies: &HashMap<String, Vec<String>>,
        dependencies: &HashMap<String, Vec<String>>,
        max_concurrent_agents: usize,
        _max_block_time: std::time::Duration,
    ) -> RhemaResult<()> {
        // Validate context consistency
        self.validate_context_consistency(agents, dependencies)?;

        // Validate dependency integrity
        self.validate_dependency_integrity(dependencies)?;

        // Validate agent coordination
        self.validate_agent_coordination(agents, locks, max_concurrent_agents)?;

        // Validate lock consistency
        self.validate_lock_consistency(locks, &agents.keys().cloned().collect::<Vec<_>>())?;

        // Validate sync status consistency
        self.validate_sync_status_consistency(sync_status, sync_dependencies)?;

        Ok(())
    }

    /// Validate context consistency
    pub fn validate_context_consistency(
        &self,
        agents: &HashMap<String, AgentState>,
        dependencies: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        // Validate that all agents have valid states
        for (agent_id, state) in agents {
            if !matches!(state, AgentState::Idle | AgentState::Working | AgentState::Blocked | AgentState::Completed) {
                return Err(SafetyViolation::ContextConsistency(
                    format!("Invalid agent state for {}: {:?}", agent_id, state)
                ).into());
            }
        }

        // Validate no circular dependencies
        self.validate_no_circular_dependencies(dependencies)?;

        Ok(())
    }

    /// Validate dependency integrity
    pub fn validate_dependency_integrity(&self, dependencies: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
        // Validate dependency graph
        self.validate_dependency_graph(dependencies)?;

        // Validate no circular dependencies
        self.validate_no_circular_dependencies(dependencies)?;

        Ok(())
    }

    /// Validate agent coordination
    pub fn validate_agent_coordination(
        &self,
        agents: &HashMap<String, AgentState>,
        locks: &HashMap<String, Option<String>>,
        max_concurrent_agents: usize,
    ) -> RhemaResult<()> {
        // Validate agent states
        self.validate_agent_states(agents)?;

        // Validate concurrent agents
        self.validate_concurrent_agents(locks, max_concurrent_agents)?;

        Ok(())
    }

    /// Validate lock consistency
    pub fn validate_lock_consistency(
        &self,
        locks: &HashMap<String, Option<String>>,
        agents: &[String],
    ) -> RhemaResult<()> {
        // Validate lock ownership
        self.validate_lock_ownership(locks, agents)?;

        // Validate one lock per agent
        self.validate_one_lock_per_agent(locks)?;

        Ok(())
    }

    /// Get validation statistics
    pub fn get_validation_statistics(&self) -> ValidationStatistics {
        ValidationStatistics {
            context_validations: self.context_validator.validation_count(),
            dependency_validations: self.dependency_validator.validation_count(),
            agent_validations: self.agent_validator.validation_count(),
            lock_validations: self.lock_validator.validation_count(),
            sync_validations: self.sync_validator.validation_count(),
        }
    }
}

/// Validation statistics
#[derive(Debug, Clone, Default)]
pub struct ValidationStatistics {
    pub context_validations: usize,
    pub dependency_validations: usize,
    pub agent_validations: usize,
    pub lock_validations: usize,
    pub sync_validations: usize,
}

impl std::fmt::Display for ValidationStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context: {}, Dependencies: {}, Agents: {}, Locks: {}, Sync: {}", 
            self.context_validations, self.dependency_validations, self.agent_validations, self.lock_validations, self.sync_validations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentState, SyncStatus};

    #[test]
    fn test_safety_validator_creation() {
        let validator = SafetyValidator::new();
        assert!(validator.get_validation_statistics().context_validations == 0);
    }

    #[test]
    fn test_validate_yaml_content() {
        let validator = SafetyValidator::new();
        
        // Valid YAML
        assert!(validator.validate_yaml_content("key: value").is_ok());
        
        // Invalid YAML
        assert!(validator.validate_yaml_content("invalid: yaml: content:").is_err());
    }

    #[test]
    fn test_validate_scope_references() {
        let validator = SafetyValidator::new();
        let all_scopes = vec!["scope1".to_string(), "scope2".to_string()];
        
        // Valid scope reference
        assert!(validator.validate_scope_references("scope1", &all_scopes).is_ok());
        
        // Invalid scope reference
        assert!(validator.validate_scope_references("nonexistent", &all_scopes).is_err());
    }

    #[test]
    fn test_validate_agent_states() {
        let validator = SafetyValidator::new();
        let mut agents = HashMap::new();
        
        // Valid states
        agents.insert("agent1".to_string(), AgentState::Idle);
        agents.insert("agent2".to_string(), AgentState::Working);
        assert!(validator.validate_agent_states(&agents).is_ok());
    }

    #[test]
    fn test_validate_concurrent_agents() {
        let validator = SafetyValidator::new();
        let mut locks = HashMap::new();
        
        // Valid concurrent agents
        locks.insert("scope1".to_string(), Some("agent1".to_string()));
        locks.insert("scope2".to_string(), Some("agent2".to_string()));
        assert!(validator.validate_concurrent_agents(&locks, 3).is_ok());
        
        // Too many concurrent agents
        locks.insert("scope3".to_string(), Some("agent3".to_string()));
        locks.insert("scope4".to_string(), Some("agent4".to_string()));
        assert!(validator.validate_concurrent_agents(&locks, 3).is_err());
    }

    #[test]
    fn test_validate_sync_status_consistency() {
        let validator = SafetyValidator::new();
        let mut sync_status = HashMap::new();
        let mut sync_dependencies = HashMap::new();
        
        // Valid sync status
        sync_status.insert("scope1".to_string(), SyncStatus::Completed);
        sync_status.insert("scope2".to_string(), SyncStatus::Idle);
        sync_dependencies.insert("scope2".to_string(), vec!["scope1".to_string()]);
        
        assert!(validator.validate_sync_status_consistency(&sync_status, &sync_dependencies).is_ok());
    }

    #[test]
    fn test_validate_all_safety_invariants() {
        let validator = SafetyValidator::new();
        let mut agents = HashMap::new();
        let mut locks = HashMap::new();
        let mut sync_status = HashMap::new();
        let mut sync_dependencies = HashMap::new();
        let mut dependencies = HashMap::new();
        
        // Set up valid state
        agents.insert("agent1".to_string(), AgentState::Idle);
        locks.insert("scope1".to_string(), Some("agent1".to_string()));
        sync_status.insert("scope1".to_string(), SyncStatus::Idle);
        dependencies.insert("scope1".to_string(), vec![]);
        
        assert!(validator.validate_all_safety_invariants(
            &agents,
            &locks,
            &sync_status,
            &sync_dependencies,
            &dependencies,
            3,
            std::time::Duration::from_secs(300),
        ).is_ok());
    }
} 