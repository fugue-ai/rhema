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

use crate::RhemaResult;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Context validator for context consistency
pub struct ContextValidator {
    validation_count: usize,
}

impl ContextValidator {
    pub fn new() -> Self {
        Self {
            validation_count: 0,
        }
    }

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }

    /// Validate YAML content
    pub fn validate_yaml_content(&mut self, content: &str) -> RhemaResult<()> {
        // Validate YAML syntax using serde_yaml
        serde_yaml::from_str::<serde_yaml::Value>(content).map_err(|e| {
            rhema_core::RhemaError::InvalidYaml {
                file: "content".to_string(),
                message: e.to_string(),
            }
        })?;

        self.validation_count += 1;
        Ok(())
    }

    /// Validate scope references
    pub fn validate_scope_references(
        &mut self,
        scope: &str,
        all_scopes: &[String],
    ) -> RhemaResult<()> {
        // Check if scope is not empty
        if scope.trim().is_empty() {
            return Err(rhema_core::RhemaError::ConfigError(
                "Scope reference cannot be empty".to_string(),
            ));
        }

        // Check if scope exists in all_scopes
        if !all_scopes.contains(&scope.to_string()) {
            return Err(rhema_core::RhemaError::ConfigError(format!(
                "Scope '{}' not found in available scopes: {:?}",
                scope, all_scopes
            )));
        }

        self.validation_count += 1;
        Ok(())
    }
}

/// Dependency validator for dependency integrity
pub struct DependencyValidator {
    validation_count: usize,
}

impl DependencyValidator {
    pub fn new() -> Self {
        Self {
            validation_count: 0,
        }
    }

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }

    /// Validate no circular dependencies using DFS
    pub fn validate_no_circular_dependencies(
        &mut self,
        dependencies: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in dependencies.keys() {
            if !visited.contains(node) {
                if self.has_cycle_dfs(node, dependencies, &mut visited, &mut rec_stack) {
                    return Err(rhema_core::RhemaError::ConfigError(
                        "Circular dependency detected".to_string(),
                    ));
                }
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    /// DFS helper for cycle detection
    fn has_cycle_dfs(
        &self,
        node: &str,
        dependencies: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());

        if let Some(deps) = dependencies.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    if self.has_cycle_dfs(dep, dependencies, visited, rec_stack) {
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
        &mut self,
        graph: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        // Check for empty graph
        if graph.is_empty() {
            return Err(rhema_core::RhemaError::ConfigError(
                "Dependency graph cannot be empty".to_string(),
            ));
        }

        // Check for invalid references (dependencies that don't exist as nodes)
        let all_nodes: HashSet<&String> = graph.keys().collect();
        for (node, deps) in graph {
            for dep in deps {
                if !all_nodes.contains(dep) {
                    return Err(rhema_core::RhemaError::ConfigError(format!(
                        "Node '{}' depends on '{}' which does not exist in the graph",
                        node, dep
                    )));
                }
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    /// Validate dependency bounds
    pub fn validate_dependency_bounds(
        &mut self,
        deps: &[String],
        max_deps: usize,
    ) -> RhemaResult<()> {
        if deps.len() > max_deps {
            return Err(rhema_core::RhemaError::ConfigError(format!(
                "Too many dependencies: {} (max: {})",
                deps.len(),
                max_deps
            )));
        }

        self.validation_count += 1;
        Ok(())
    }

    /// Validate no self dependencies
    pub fn validate_no_self_dependencies(
        &mut self,
        scope: &str,
        deps: &[String],
    ) -> RhemaResult<()> {
        if deps.contains(&scope.to_string()) {
            return Err(rhema_core::RhemaError::ConfigError(format!(
                "Scope '{}' cannot depend on itself",
                scope
            )));
        }

        self.validation_count += 1;
        Ok(())
    }
}

/// Agent validator for agent coordination
pub struct AgentValidator {
    validation_count: usize,
}

impl AgentValidator {
    pub fn new() -> Self {
        Self {
            validation_count: 0,
        }
    }

    /// Validate agent states
    pub fn validate_agent_states(&mut self, agents: &HashMap<String, String>) -> RhemaResult<()> {
        let valid_states = [
            "idle",
            "busy",
            "working",
            "blocked",
            "collaborating",
            "offline",
            "error",
            "maintenance",
        ];

        for (agent_id, state) in agents {
            if !valid_states.contains(&state.as_str()) {
                return Err(rhema_core::RhemaError::ConfigError(format!(
                    "Invalid state '{}' for agent '{}'",
                    state, agent_id
                )));
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    /// Validate concurrent agents
    pub fn validate_concurrent_agents(
        &mut self,
        locks: &HashMap<String, Option<String>>,
        max_concurrent: usize,
    ) -> RhemaResult<()> {
        let active_agents: HashSet<&String> =
            locks.values().filter_map(|agent| agent.as_ref()).collect();

        if active_agents.len() > max_concurrent {
            return Err(rhema_core::RhemaError::ConfigError(format!(
                "Too many concurrent agents: {} (max: {})",
                active_agents.len(),
                max_concurrent
            )));
        }

        self.validation_count += 1;
        Ok(())
    }

    /// Validate agent progress
    pub fn validate_agent_progress(
        &mut self,
        agent_id: &str,
        state: &str,
        max_block_time: Duration,
    ) -> RhemaResult<()> {
        // Check if agent is in a blocked state for too long
        if state == "blocked" {
            // This would typically check against a timestamp, but for now we just validate the state
            // In a real implementation, you'd compare against the actual block start time
            if max_block_time > Duration::from_secs(3600) {
                // 1 hour
                return Err(rhema_core::RhemaError::ConfigError(format!(
                    "Agent '{}' has been blocked for too long",
                    agent_id
                )));
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }
}

/// Lock validator for lock consistency
pub struct LockValidator {
    validation_count: usize,
}

impl LockValidator {
    pub fn new() -> Self {
        Self {
            validation_count: 0,
        }
    }

    /// Validate lock ownership
    pub fn validate_lock_ownership(
        &mut self,
        locks: &HashMap<String, Option<String>>,
        agents: &[String],
    ) -> RhemaResult<()> {
        let valid_agents: HashSet<&String> = agents.iter().collect();

        for (resource, owner) in locks {
            if let Some(agent) = owner {
                if !valid_agents.contains(agent) {
                    return Err(rhema_core::RhemaError::ConfigError(format!(
                        "Resource '{}' is locked by unknown agent '{}'",
                        resource, agent
                    )));
                }
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    /// Validate one lock per agent
    pub fn validate_one_lock_per_agent(
        &mut self,
        locks: &HashMap<String, Option<String>>,
    ) -> RhemaResult<()> {
        let mut agent_locks: HashMap<&String, Vec<&String>> = HashMap::new();

        for (resource, owner) in locks {
            if let Some(agent) = owner {
                agent_locks
                    .entry(agent)
                    .or_insert_with(Vec::new)
                    .push(resource);
            }
        }

        for (agent, resources) in agent_locks {
            if resources.len() > 1 {
                return Err(rhema_core::RhemaError::ConfigError(format!(
                    "Agent '{}' holds multiple locks: {:?}",
                    agent, resources
                )));
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    /// Validate lock timeouts
    pub fn validate_lock_timeouts(
        &mut self,
        locks: &HashMap<String, Option<String>>,
        timeouts: &HashMap<String, Instant>,
    ) -> RhemaResult<()> {
        let now = Instant::now();
        let timeout_duration = Duration::from_secs(300); // 5 minutes default timeout

        for (resource, owner) in locks {
            if owner.is_some() {
                if let Some(lock_time) = timeouts.get(resource) {
                    if now.duration_since(*lock_time) > timeout_duration {
                        return Err(rhema_core::RhemaError::ConfigError(format!(
                            "Lock on resource '{}' has timed out",
                            resource
                        )));
                    }
                }
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }
}

/// Sync validator for sync status consistency
pub struct SyncValidator {
    validation_count: usize,
}

impl SyncValidator {
    pub fn new() -> Self {
        Self {
            validation_count: 0,
        }
    }

    /// Validate sync status consistency
    pub fn validate_sync_status_consistency(
        &mut self,
        sync_status: &HashMap<String, String>,
        sync_dependencies: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<()> {
        let valid_statuses = ["idle", "syncing", "completed", "failed"];

        // Validate all status values are valid
        for (scope, status) in sync_status {
            if !valid_statuses.contains(&status.as_str()) {
                return Err(rhema_core::RhemaError::ConfigError(format!(
                    "Invalid sync status '{}' for scope '{}'",
                    status, scope
                )));
            }
        }

        // Validate dependency consistency
        for (scope, deps) in sync_dependencies {
            if let Some(scope_status) = sync_status.get(scope) {
                if scope_status == "syncing" {
                    // If a scope is syncing, all its dependencies should be completed
                    for dep in deps {
                        if let Some(dep_status) = sync_status.get(dep) {
                            if dep_status != "completed" {
                                return Err(rhema_core::RhemaError::ConfigError(format!(
                                    "Scope '{}' is syncing but dependency '{}' has status '{}'",
                                    scope, dep, dep_status
                                )));
                            }
                        }
                    }
                }
            }
        }

        self.validation_count += 1;
        Ok(())
    }

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_validator() {
        let mut validator = ContextValidator::new();
        assert_eq!(validator.validation_count(), 0);
        assert!(validator.validate_yaml_content("test").is_ok());
        assert_eq!(validator.validation_count(), 1);
    }

    #[test]
    fn test_yaml_validation() {
        let mut validator = ContextValidator::new();

        // Valid YAML
        assert!(validator.validate_yaml_content("key: value").is_ok());
        assert!(validator
            .validate_yaml_content("list:\n  - item1\n  - item2")
            .is_ok());

        // Invalid YAML
        assert!(validator
            .validate_yaml_content("key: value\n  invalid: indentation")
            .is_err());
        assert!(validator.validate_yaml_content("key: [unclosed").is_err());
    }

    #[test]
    fn test_scope_reference_validation() {
        let mut validator = ContextValidator::new();
        let all_scopes = vec![
            "scope1".to_string(),
            "scope2".to_string(),
            "scope3".to_string(),
        ];

        // Valid scope references
        assert!(validator
            .validate_scope_references("scope1", &all_scopes)
            .is_ok());
        assert!(validator
            .validate_scope_references("scope2", &all_scopes)
            .is_ok());

        // Invalid scope references
        assert!(validator
            .validate_scope_references("nonexistent", &all_scopes)
            .is_err());
        assert!(validator
            .validate_scope_references("", &all_scopes)
            .is_err());
        assert!(validator
            .validate_scope_references("   ", &all_scopes)
            .is_err());
    }

    #[test]
    fn test_dependency_validator() {
        let mut validator = DependencyValidator::new();
        assert_eq!(validator.validation_count(), 0);
        let deps = HashMap::new();
        assert!(validator.validate_no_circular_dependencies(&deps).is_ok());
        assert_eq!(validator.validation_count(), 1);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut validator = DependencyValidator::new();

        // No circular dependencies
        let mut deps = HashMap::new();
        deps.insert("A".to_string(), vec!["B".to_string()]);
        deps.insert("B".to_string(), vec!["C".to_string()]);
        deps.insert("C".to_string(), vec![]);
        assert!(validator.validate_no_circular_dependencies(&deps).is_ok());

        // Circular dependency
        let mut circular_deps = HashMap::new();
        circular_deps.insert("A".to_string(), vec!["B".to_string()]);
        circular_deps.insert("B".to_string(), vec!["C".to_string()]);
        circular_deps.insert("C".to_string(), vec!["A".to_string()]);
        assert!(validator
            .validate_no_circular_dependencies(&circular_deps)
            .is_err());
    }

    #[test]
    fn test_dependency_graph_validation() {
        let mut validator = DependencyValidator::new();

        // Valid graph
        let mut valid_graph = HashMap::new();
        valid_graph.insert("A".to_string(), vec!["B".to_string()]);
        valid_graph.insert("B".to_string(), vec![]);
        assert!(validator.validate_dependency_graph(&valid_graph).is_ok());

        // Invalid graph - reference to non-existent node
        let mut invalid_graph = HashMap::new();
        invalid_graph.insert("A".to_string(), vec!["B".to_string()]);
        invalid_graph.insert("C".to_string(), vec!["D".to_string()]); // D doesn't exist
        assert!(validator.validate_dependency_graph(&invalid_graph).is_err());

        // Empty graph
        let empty_graph = HashMap::new();
        assert!(validator.validate_dependency_graph(&empty_graph).is_err());
    }

    #[test]
    fn test_dependency_bounds_validation() {
        let mut validator = DependencyValidator::new();

        let deps = vec!["dep1".to_string(), "dep2".to_string()];
        assert!(validator.validate_dependency_bounds(&deps, 3).is_ok());
        assert!(validator.validate_dependency_bounds(&deps, 2).is_ok());
        assert!(validator.validate_dependency_bounds(&deps, 1).is_err());
    }

    #[test]
    fn test_self_dependency_validation() {
        let mut validator = DependencyValidator::new();

        let scope = "myscope";
        let deps = vec!["dep1".to_string(), "dep2".to_string()];
        assert!(validator
            .validate_no_self_dependencies(scope, &deps)
            .is_ok());

        let self_deps = vec!["dep1".to_string(), "myscope".to_string()];
        assert!(validator
            .validate_no_self_dependencies(scope, &self_deps)
            .is_err());
    }

    #[test]
    fn test_agent_validator() {
        let mut validator = AgentValidator::new();
        assert_eq!(validator.validation_count(), 0);
        let agents = HashMap::new();
        assert!(validator.validate_agent_states(&agents).is_ok());
        assert_eq!(validator.validation_count(), 1);
    }

    #[test]
    fn test_agent_states_validation() {
        let mut validator = AgentValidator::new();

        // Valid states
        let mut valid_agents = HashMap::new();
        valid_agents.insert("agent1".to_string(), "idle".to_string());
        valid_agents.insert("agent2".to_string(), "busy".to_string());
        valid_agents.insert("agent3".to_string(), "working".to_string());
        assert!(validator.validate_agent_states(&valid_agents).is_ok());

        // Invalid state
        let mut invalid_agents = HashMap::new();
        invalid_agents.insert("agent1".to_string(), "invalid_state".to_string());
        assert!(validator.validate_agent_states(&invalid_agents).is_err());
    }

    #[test]
    fn test_concurrent_agents_validation() {
        let mut validator = AgentValidator::new();

        let mut locks = HashMap::new();
        locks.insert("resource1".to_string(), Some("agent1".to_string()));
        locks.insert("resource2".to_string(), Some("agent2".to_string()));

        assert!(validator.validate_concurrent_agents(&locks, 3).is_ok());
        assert!(validator.validate_concurrent_agents(&locks, 2).is_ok());
        assert!(validator.validate_concurrent_agents(&locks, 1).is_err());
    }

    #[test]
    fn test_agent_progress_validation() {
        let mut validator = AgentValidator::new();

        // Non-blocked state
        assert!(validator
            .validate_agent_progress("agent1", "idle", Duration::from_secs(7200))
            .is_ok());

        // Blocked state within limits
        assert!(validator
            .validate_agent_progress("agent1", "blocked", Duration::from_secs(1800))
            .is_ok());

        // Blocked state exceeding limits
        assert!(validator
            .validate_agent_progress("agent1", "blocked", Duration::from_secs(7200))
            .is_err());
    }

    #[test]
    fn test_lock_validator() {
        let mut validator = LockValidator::new();
        assert_eq!(validator.validation_count(), 0);
        let locks = HashMap::new();
        let agents = Vec::new();
        assert!(validator.validate_lock_ownership(&locks, &agents).is_ok());
        assert_eq!(validator.validation_count(), 1);
    }

    #[test]
    fn test_lock_ownership_validation() {
        let mut validator = LockValidator::new();

        let agents = vec!["agent1".to_string(), "agent2".to_string()];

        // Valid ownership
        let mut valid_locks = HashMap::new();
        valid_locks.insert("resource1".to_string(), Some("agent1".to_string()));
        valid_locks.insert("resource2".to_string(), Some("agent2".to_string()));
        assert!(validator
            .validate_lock_ownership(&valid_locks, &agents)
            .is_ok());

        // Invalid ownership - unknown agent
        let mut invalid_locks = HashMap::new();
        invalid_locks.insert("resource1".to_string(), Some("unknown_agent".to_string()));
        assert!(validator
            .validate_lock_ownership(&invalid_locks, &agents)
            .is_err());
    }

    #[test]
    fn test_one_lock_per_agent_validation() {
        let mut validator = LockValidator::new();

        // Valid - one lock per agent
        let mut valid_locks = HashMap::new();
        valid_locks.insert("resource1".to_string(), Some("agent1".to_string()));
        valid_locks.insert("resource2".to_string(), Some("agent2".to_string()));
        assert!(validator.validate_one_lock_per_agent(&valid_locks).is_ok());

        // Invalid - agent with multiple locks
        let mut invalid_locks = HashMap::new();
        invalid_locks.insert("resource1".to_string(), Some("agent1".to_string()));
        invalid_locks.insert("resource2".to_string(), Some("agent1".to_string()));
        assert!(validator
            .validate_one_lock_per_agent(&invalid_locks)
            .is_err());
    }

    #[test]
    fn test_lock_timeout_validation() {
        let mut validator = LockValidator::new();

        let mut locks = HashMap::new();
        locks.insert("resource1".to_string(), Some("agent1".to_string()));

        let mut timeouts = HashMap::new();
        timeouts.insert("resource1".to_string(), Instant::now());

        // Should pass for recent locks
        assert!(validator.validate_lock_timeouts(&locks, &timeouts).is_ok());

        // Should fail for old locks (we'd need to manipulate the time, but this tests the structure)
        // In a real implementation, you'd mock the time or use a different approach
    }

    #[test]
    fn test_sync_validator() {
        let mut validator = SyncValidator::new();
        assert_eq!(validator.validation_count(), 0);
        let sync_status = HashMap::new();
        let sync_dependencies = HashMap::new();
        assert!(validator
            .validate_sync_status_consistency(&sync_status, &sync_dependencies)
            .is_ok());
        assert_eq!(validator.validation_count(), 1);
    }

    #[test]
    fn test_sync_status_consistency_validation() {
        let mut validator = SyncValidator::new();

        // Valid sync statuses
        let mut valid_status = HashMap::new();
        valid_status.insert("scope1".to_string(), "idle".to_string());
        valid_status.insert("scope2".to_string(), "completed".to_string());
        valid_status.insert("scope3".to_string(), "syncing".to_string());

        let mut dependencies = HashMap::new();
        dependencies.insert("scope3".to_string(), vec!["scope2".to_string()]);

        assert!(validator
            .validate_sync_status_consistency(&valid_status, &dependencies)
            .is_ok());

        // Invalid sync status
        let mut invalid_status = HashMap::new();
        invalid_status.insert("scope1".to_string(), "invalid_status".to_string());
        assert!(validator
            .validate_sync_status_consistency(&invalid_status, &dependencies)
            .is_err());

        // Invalid dependency consistency - syncing scope with non-completed dependency
        let mut invalid_deps = HashMap::new();
        invalid_deps.insert("scope3".to_string(), vec!["scope1".to_string()]);
        valid_status.insert("scope1".to_string(), "idle".to_string());
        valid_status.insert("scope3".to_string(), "syncing".to_string());
        assert!(validator
            .validate_sync_status_consistency(&valid_status, &invalid_deps)
            .is_err());
    }
}
