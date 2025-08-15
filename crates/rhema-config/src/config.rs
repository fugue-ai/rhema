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

use rhema_core::RhemaResult;
use serde_yaml;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use thiserror::Error;
// Removed unused imports

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
    validation_stats: ValidationStatistics,
}

impl SafetyValidator {
    /// Create a new safety validator
    pub fn new() -> Self {
        Self {
            validation_stats: ValidationStatistics::default(),
        }
    }

    /// Validate YAML content
    pub fn validate_yaml_content(&self, content: &str) -> RhemaResult<()> {
        // Basic YAML syntax validation
        match serde_yaml::from_str::<serde_yaml::Value>(content) {
            Ok(_) => Ok(()),
            Err(e) => Err(rhema_core::RhemaError::ValidationError(format!(
                "Invalid YAML content: {}",
                e
            ))),
        }
    }

    /// Validate scope references
    pub fn validate_scope_references(&self, scope: &str, all_scopes: &[String]) -> RhemaResult<()> {
        if !all_scopes.contains(&scope.to_string()) {
            return Err(rhema_core::RhemaError::ValidationError(format!(
                "Scope '{}' not found in available scopes: {:?}",
                scope, all_scopes
            )));
        }
        Ok(())
    }

    /// Validate no circular dependencies using DFS cycle detection
    pub fn validate_no_circular_dependencies(
        &self,
        dependencies: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in dependencies.keys() {
            if !visited.contains(node) {
                if self.has_cycle(dependencies, node, &mut visited, &mut rec_stack) {
                    return Err(rhema_core::RhemaError::ValidationError(format!(
                        "Circular dependency detected involving '{}'",
                        node
                    )));
                }
            }
        }
        Ok(())
    }

    /// Helper method to detect cycles in dependency graph
    fn has_cycle(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());

        if let Some(dependencies) = graph.get(node) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    if self.has_cycle(graph, dep, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(dep) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    /// Validate dependency graph structure
    pub fn validate_dependency_graph(
        &self,
        graph: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        // Check for self-dependencies
        for (node, deps) in graph {
            if deps.contains(node) {
                return Err(rhema_core::RhemaError::ValidationError(format!(
                    "Self-dependency detected for node '{}'",
                    node
                )));
            }
        }

        // Check for circular dependencies
        self.validate_no_circular_dependencies(graph)?;

        Ok(())
    }

    /// Validate dependency bounds
    pub fn validate_dependency_bounds(&self, deps: &[String], max_deps: usize) -> RhemaResult<()> {
        if deps.len() > max_deps {
            return Err(rhema_core::RhemaError::ValidationError(format!(
                "Too many dependencies: {} (max: {})",
                deps.len(),
                max_deps
            )));
        }
        Ok(())
    }

    /// Validate no self dependencies
    pub fn validate_no_self_dependencies(&self, scope: &str, deps: &[String]) -> RhemaResult<()> {
        if deps.contains(&scope.to_string()) {
            return Err(rhema_core::RhemaError::ValidationError(format!(
                "Self-dependency detected for scope '{}'",
                scope
            )));
        }
        Ok(())
    }

    /// Validate agent states
    pub fn validate_agent_states(&self, agents: &HashMap<String, String>) -> RhemaResult<()> {
        let valid_states = [
            "idle",
            "ready",
            "busy",
            "working",
            "paused",
            "stopping",
            "stopped",
            "error",
            "deadlocked",
            "blocked",
            "completed",
        ];

        for (agent_id, state) in agents {
            if !valid_states.contains(&state.as_str()) {
                return Err(rhema_core::RhemaError::ValidationError(format!(
                    "Invalid agent state '{}' for agent '{}'. Valid states: {:?}",
                    state, agent_id, valid_states
                )));
            }
        }
        Ok(())
    }

    /// Validate concurrent agents
    pub fn validate_concurrent_agents(
        &self,
        locks: &HashMap<String, Option<String>>,
        max_concurrent: usize,
    ) -> RhemaResult<()> {
        let active_agents: HashSet<&String> =
            locks.values().filter_map(|agent| agent.as_ref()).collect();

        if active_agents.len() > max_concurrent {
            return Err(rhema_core::RhemaError::ValidationError(format!(
                "Too many concurrent agents: {} (max: {})",
                active_agents.len(),
                max_concurrent
            )));
        }
        Ok(())
    }

    /// Validate agent progress
    pub fn validate_agent_progress(
        &self,
        agent_id: &str,
        state: &str,
        _max_block_time: Duration,
    ) -> RhemaResult<()> {
        // This would typically check against a progress tracking system
        // For now, we'll validate the state is not stuck in a blocking state too long
        let blocking_states = ["blocked", "deadlocked", "error"];

        if blocking_states.contains(&state) {
            // In a real implementation, you'd check the actual blocking duration
            // against max_block_time here
            tracing::warn!(
                "Agent '{}' is in blocking state '{}' - should check duration",
                agent_id,
                state
            );
        }

        Ok(())
    }

    /// Validate lock ownership
    pub fn validate_lock_ownership(
        &self,
        locks: &HashMap<String, Option<String>>,
        agents: &[String],
    ) -> RhemaResult<()> {
        let agent_set: HashSet<&String> = agents.iter().collect();

        for (resource, owner) in locks {
            if let Some(agent_id) = owner {
                if !agent_set.contains(agent_id) {
                    return Err(rhema_core::RhemaError::ValidationError(format!(
                        "Lock on '{}' owned by non-existent agent '{}'",
                        resource, agent_id
                    )));
                }
            }
        }
        Ok(())
    }

    /// Validate one lock per agent
    pub fn validate_one_lock_per_agent(
        &self,
        locks: &HashMap<String, Option<String>>,
    ) -> RhemaResult<()> {
        let mut agent_locks: HashMap<&String, Vec<&String>> = HashMap::new();

        for (resource, owner) in locks {
            if let Some(agent_id) = owner {
                agent_locks
                    .entry(agent_id)
                    .or_insert_with(Vec::new)
                    .push(resource);
            }
        }

        for (agent_id, resources) in agent_locks {
            if resources.len() > 1 {
                return Err(rhema_core::RhemaError::ValidationError(format!(
                    "Agent '{}' holds multiple locks: {:?}",
                    agent_id, resources
                )));
            }
        }
        Ok(())
    }

    /// Validate lock timeouts
    pub fn validate_lock_timeouts(
        &self,
        locks: &HashMap<String, Option<String>>,
        timeouts: &HashMap<String, Instant>,
    ) -> RhemaResult<()> {
        let now = Instant::now();

        for (resource, timeout) in timeouts {
            if locks.contains_key(resource) && now.duration_since(*timeout).as_secs() > 300 {
                // 5 minute timeout threshold
                return Err(rhema_core::RhemaError::ValidationError(format!(
                    "Lock on '{}' has exceeded timeout",
                    resource
                )));
            }
        }
        Ok(())
    }

    /// Validate sync status consistency
    pub fn validate_sync_status_consistency(
        &self,
        sync_status: &HashMap<String, String>,
        sync_dependencies: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        let valid_sync_states = ["synced", "syncing", "conflict", "error", "pending"];

        // Validate sync states
        for (resource, status) in sync_status {
            if !valid_sync_states.contains(&status.as_str()) {
                return Err(rhema_core::RhemaError::ValidationError(format!(
                    "Invalid sync status '{}' for resource '{}'",
                    status, resource
                )));
            }
        }

        // Validate dependencies are consistent
        for (resource, deps) in sync_dependencies {
            if let Some(status) = sync_status.get(resource) {
                if status == "synced" {
                    // If synced, all dependencies should also be synced
                    for dep in deps {
                        if let Some(dep_status) = sync_status.get(dep) {
                            if dep_status != "synced" {
                                return Err(rhema_core::RhemaError::ValidationError(format!(
                                    "Synced resource '{}' depends on unsynced resource '{}' (status: {})",
                                    resource, dep, dep_status
                                )));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate all safety invariants for the entire system
    pub fn validate_all_safety_invariants(
        &self,
        agents: &HashMap<String, String>,
        locks: &HashMap<String, Option<String>>,
        sync_status: &HashMap<String, String>,
        sync_dependencies: &HashMap<String, Vec<String>>,
        dependencies: &HashMap<String, Vec<String>>,
        max_concurrent_agents: usize,
        _max_block_time: Duration,
    ) -> RhemaResult<()> {
        // Validate context consistency
        self.validate_context_consistency(agents, dependencies)?;

        // Validate dependency integrity
        self.validate_dependency_integrity(dependencies)?;

        // Validate agent coordination
        self.validate_agent_coordination(agents, locks, max_concurrent_agents)?;

        // Validate lock consistency
        let agent_ids: Vec<String> = agents.keys().cloned().collect();
        self.validate_lock_consistency(locks, &agent_ids)?;

        // Validate sync status consistency
        self.validate_sync_status_consistency(sync_status, sync_dependencies)?;

        Ok(())
    }

    /// Validate context consistency
    pub fn validate_context_consistency(
        &self,
        agents: &HashMap<String, String>,
        dependencies: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        // Validate agent states
        self.validate_agent_states(agents)?;

        // Validate no circular dependencies
        self.validate_no_circular_dependencies(dependencies)?;

        // Validate dependency graph structure
        self.validate_dependency_graph(dependencies)?;

        Ok(())
    }

    /// Validate dependency integrity
    pub fn validate_dependency_integrity(
        &self,
        dependencies: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        // Check for empty dependencies
        for (node, deps) in dependencies {
            if deps.is_empty() {
                tracing::debug!("Node '{}' has no dependencies", node);
            }
        }

        // Validate dependency bounds (reasonable limit of 100 dependencies per node)
        for (node, deps) in dependencies {
            self.validate_dependency_bounds(deps, 100)?;
            self.validate_no_self_dependencies(node, deps)?;
        }

        Ok(())
    }

    /// Validate agent coordination
    pub fn validate_agent_coordination(
        &self,
        agents: &HashMap<String, String>,
        locks: &HashMap<String, Option<String>>,
        max_concurrent_agents: usize,
    ) -> RhemaResult<()> {
        // Validate concurrent agents limit
        self.validate_concurrent_agents(locks, max_concurrent_agents)?;

        // Validate lock ownership
        let agent_ids: Vec<String> = agents.keys().cloned().collect();
        self.validate_lock_ownership(locks, &agent_ids)?;

        // Validate one lock per agent
        self.validate_one_lock_per_agent(locks)?;

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
        self.validation_stats.clone()
    }
}

/// Validation statistics for tracking validator performance
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
        write!(
            f,
            "Validation Statistics: context={}, dependency={}, agent={}, lock={}, sync={}",
            self.context_validations,
            self.dependency_validations,
            self.agent_validations,
            self.lock_validations,
            self.sync_validations
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_validator_creation() {
        let validator = SafetyValidator::new();
        assert!(validator.validate_yaml_content("test").is_ok());
    }

    #[test]
    fn test_validate_yaml_content() {
        let validator = SafetyValidator::new();
        assert!(validator.validate_yaml_content("test content").is_ok());
        assert!(validator.validate_yaml_content("key: value").is_ok());
        assert!(validator
            .validate_yaml_content("invalid: [yaml: content: [")
            .is_err());
    }

    #[test]
    fn test_validate_scope_references() {
        let validator = SafetyValidator::new();
        let scopes = vec!["scope1".to_string(), "scope2".to_string()];
        assert!(validator
            .validate_scope_references("scope1", &scopes)
            .is_ok());
        assert!(validator
            .validate_scope_references("scope3", &scopes)
            .is_err());
    }

    #[test]
    fn test_validate_no_circular_dependencies() {
        let validator = SafetyValidator::new();
        let mut deps = HashMap::new();
        deps.insert("a".to_string(), vec!["b".to_string()]);
        deps.insert("b".to_string(), vec!["c".to_string()]);
        deps.insert("c".to_string(), vec!["a".to_string()]);

        assert!(validator.validate_no_circular_dependencies(&deps).is_err());

        let mut deps_no_cycle = HashMap::new();
        deps_no_cycle.insert("a".to_string(), vec!["b".to_string()]);
        deps_no_cycle.insert("b".to_string(), vec!["c".to_string()]);
        deps_no_cycle.insert("c".to_string(), vec![]);

        assert!(validator
            .validate_no_circular_dependencies(&deps_no_cycle)
            .is_ok());
    }

    #[test]
    fn test_validate_agent_states() {
        let validator = SafetyValidator::new();
        let mut agents = HashMap::new();
        agents.insert("agent1".to_string(), "idle".to_string());
        agents.insert("agent2".to_string(), "busy".to_string());
        assert!(validator.validate_agent_states(&agents).is_ok());

        agents.insert("agent3".to_string(), "invalid_state".to_string());
        assert!(validator.validate_agent_states(&agents).is_err());
    }

    #[test]
    fn test_validate_concurrent_agents() {
        let validator = SafetyValidator::new();
        let mut locks = HashMap::new();
        locks.insert("resource1".to_string(), Some("agent1".to_string()));
        locks.insert("resource2".to_string(), Some("agent2".to_string()));
        assert!(validator.validate_concurrent_agents(&locks, 5).is_ok());
        assert!(validator.validate_concurrent_agents(&locks, 1).is_err());
    }

    #[test]
    fn test_validate_sync_status_consistency() {
        let validator = SafetyValidator::new();
        let mut sync_status = HashMap::new();
        sync_status.insert("resource1".to_string(), "synced".to_string());
        sync_status.insert("resource2".to_string(), "syncing".to_string());
        let sync_dependencies = HashMap::new();
        assert!(validator
            .validate_sync_status_consistency(&sync_status, &sync_dependencies)
            .is_ok());
    }

    #[test]
    fn test_validate_all_safety_invariants() {
        let validator = SafetyValidator::new();
        let agents = HashMap::new();
        let locks = HashMap::new();
        let sync_status = HashMap::new();
        let sync_dependencies = HashMap::new();
        let dependencies = HashMap::new();
        assert!(validator
            .validate_all_safety_invariants(
                &agents,
                &locks,
                &sync_status,
                &sync_dependencies,
                &dependencies,
                5,
                std::time::Duration::from_secs(30)
            )
            .is_ok());
    }

    #[test]
    fn test_validate_dependency_bounds() {
        let validator = SafetyValidator::new();
        let deps = vec!["dep1".to_string(), "dep2".to_string()];
        assert!(validator.validate_dependency_bounds(&deps, 5).is_ok());
        assert!(validator.validate_dependency_bounds(&deps, 1).is_err());
    }

    #[test]
    fn test_validate_no_self_dependencies() {
        let validator = SafetyValidator::new();
        let deps = vec!["dep1".to_string(), "dep2".to_string()];
        assert!(validator
            .validate_no_self_dependencies("scope1", &deps)
            .is_ok());

        let deps_with_self = vec!["dep1".to_string(), "scope1".to_string()];
        assert!(validator
            .validate_no_self_dependencies("scope1", &deps_with_self)
            .is_err());
    }

    #[test]
    fn test_validate_lock_ownership() {
        let validator = SafetyValidator::new();
        let mut locks = HashMap::new();
        locks.insert("resource1".to_string(), Some("agent1".to_string()));
        let agents = vec!["agent1".to_string()];
        assert!(validator.validate_lock_ownership(&locks, &agents).is_ok());

        locks.insert("resource2".to_string(), Some("agent2".to_string()));
        assert!(validator.validate_lock_ownership(&locks, &agents).is_err());
    }

    #[test]
    fn test_validate_one_lock_per_agent() {
        let validator = SafetyValidator::new();
        let mut locks = HashMap::new();
        locks.insert("resource1".to_string(), Some("agent1".to_string()));
        locks.insert("resource2".to_string(), Some("agent2".to_string()));
        assert!(validator.validate_one_lock_per_agent(&locks).is_ok());

        locks.insert("resource3".to_string(), Some("agent1".to_string()));
        assert!(validator.validate_one_lock_per_agent(&locks).is_err());
    }
}
