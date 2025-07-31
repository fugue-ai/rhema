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
use crate::agent::{AgentState, SyncStatus};
use crate::safety::SafetyViolation;
use std::collections::HashMap;
use std::time::Duration;

/// Context validator for YAML content and scope references
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
    pub fn validate_yaml_content(&self, content: &str) -> RhemaResult<()> {
        // Basic YAML validation
        if content.trim().is_empty() {
            return Ok(()); // Empty content is valid
        }

        // Try to parse as YAML
        match serde_yaml::from_str::<serde_yaml::Value>(content) {
            Ok(_) => Ok(()),
            Err(e) => Err(SafetyViolation::ContextConsistency(
                format!("Invalid YAML content: {}", e)
            ).into()),
        }
    }

    /// Validate scope references
    pub fn validate_scope_references(&self, scope: &str, all_scopes: &[String]) -> RhemaResult<()> {
        if all_scopes.contains(&scope.to_string()) {
            Ok(())
        } else {
            Err(SafetyViolation::ContextConsistency(
                format!("Scope reference not found: {}", scope)
            ).into())
        }
    }
}

/// Dependency validator for dependency graph integrity
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

    /// Validate no circular dependencies
    pub fn validate_no_circular_dependencies(&self, dependencies: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
        
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        for node in dependencies.keys() {
            if !visited.contains(node) {
                if self.has_cycle(dependencies, node, &mut visited, &mut rec_stack) {
                    return Err(SafetyViolation::CircularDependency(
                        format!("Circular dependency detected involving {}", node)
                    ).into());
                }
            }
        }
        
        Ok(())
    }

    /// Validate dependency graph
    pub fn validate_dependency_graph(&self, graph: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
        
        // Check that all dependencies exist in the graph
        for (scope, deps) in graph {
            for dep in deps {
                if !graph.contains_key(dep) {
                    return Err(SafetyViolation::DependencyIntegrity(
                        format!("Dependency {} not found in graph for scope {}", dep, scope)
                    ).into());
                }
            }
        }
        
        Ok(())
    }

    /// Validate dependency bounds
    pub fn validate_dependency_bounds(&self, deps: &[String], max_deps: usize) -> RhemaResult<()> {
        
        if deps.len() > max_deps {
            return Err(SafetyViolation::ResourceBounds(
                format!("Too many dependencies: {} > {}", deps.len(), max_deps)
            ).into());
        }
        
        Ok(())
    }

    /// Validate no self dependencies
    pub fn validate_no_self_dependencies(&self, scope: &str, deps: &[String]) -> RhemaResult<()> {
        
        if deps.contains(&scope.to_string()) {
            return Err(SafetyViolation::DependencyIntegrity(
                format!("Self-dependency detected for scope {}", scope)
            ).into());
        }
        
        Ok(())
    }

    /// Check for cycles in dependency graph using DFS
    fn has_cycle(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true; // Back edge found - cycle detected
        }
        
        if visited.contains(node) {
            return false; // Already processed
        }
        
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if self.has_cycle(graph, neighbor, visited, rec_stack) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(node);
        false
    }
}

/// Agent validator for agent state and coordination
pub struct AgentValidator {
    validation_count: usize,
}

impl AgentValidator {
    pub fn new() -> Self {
        Self {
            validation_count: 0,
        }
    }

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }

    /// Validate agent states
    pub fn validate_agent_states(&self, agents: &HashMap<String, AgentState>) -> RhemaResult<()> {
        
        for (agent_id, state) in agents {
            if !matches!(state, AgentState::Idle | AgentState::Working | AgentState::Blocked | AgentState::Completed) {
                return Err(SafetyViolation::AgentCoordination(
                    format!("Invalid agent state for {}: {:?}", agent_id, state)
                ).into());
            }
        }
        
        Ok(())
    }

    /// Validate concurrent agents
    pub fn validate_concurrent_agents(&self, locks: &HashMap<String, Option<String>>, max_concurrent: usize) -> RhemaResult<()> {
        
        let active_locks = locks.values().filter(|agent_id| agent_id.is_some()).count();
        
        if active_locks > max_concurrent {
            return Err(SafetyViolation::AgentCoordination(
                format!("Too many concurrent agents: {} > {}", active_locks, max_concurrent)
            ).into());
        }
        
        Ok(())
    }

    /// Validate agent progress
    pub fn validate_agent_progress(&self, agent_id: &str, state: &AgentState, _max_block_time: Duration) -> RhemaResult<()> {
        
        // This would typically check if an agent has been blocked for too long
        // For now, we just validate the state is valid
        if !matches!(state, AgentState::Idle | AgentState::Working | AgentState::Blocked | AgentState::Completed) {
            return Err(SafetyViolation::AgentCoordination(
                format!("Invalid agent state for {}: {:?}", agent_id, state)
            ).into());
        }
        
        Ok(())
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

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }

    /// Validate lock ownership
    pub fn validate_lock_ownership(&self, locks: &HashMap<String, Option<String>>, agents: &[String]) -> RhemaResult<()> {
        
        for (scope, agent_id) in locks {
            if let Some(agent_id) = agent_id {
                if !agents.contains(agent_id) {
                    return Err(SafetyViolation::LockConsistency(
                        format!("Lock held by non-existent agent {} for scope {}", agent_id, scope)
                    ).into());
                }
            }
        }
        
        Ok(())
    }

    /// Validate one lock per agent
    pub fn validate_one_lock_per_agent(&self, locks: &HashMap<String, Option<String>>) -> RhemaResult<()> {
        
        let mut agent_lock_counts = HashMap::new();
        
        for agent_id in locks.values().filter_map(|id| id.as_ref()) {
            *agent_lock_counts.entry(agent_id.clone()).or_insert(0) += 1;
        }
        
        for (agent_id, count) in agent_lock_counts {
            if count > 1 {
                return Err(SafetyViolation::LockConsistency(
                    format!("Agent {} holds {} locks (max 1)", agent_id, count)
                ).into());
            }
        }
        
        Ok(())
    }

    /// Validate lock timeouts
    pub fn validate_lock_timeouts(&self, locks: &HashMap<String, Option<String>>, timeouts: &HashMap<String, std::time::Instant>) -> RhemaResult<()> {
        
        let now = std::time::Instant::now();
        
        for (scope, agent_id) in locks {
            if let Some(agent_id) = agent_id {
                if let Some(timeout) = timeouts.get(agent_id) {
                    if *timeout <= now {
                        return Err(SafetyViolation::LockConsistency(
                            format!("Lock timeout for agent {} on scope {}", agent_id, scope)
                        ).into());
                    }
                }
            }
        }
        
        Ok(())
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

    pub fn validation_count(&self) -> usize {
        self.validation_count
    }

    /// Validate sync status consistency
    pub fn validate_sync_status_consistency(&self, sync_status: &HashMap<String, SyncStatus>, sync_dependencies: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
        
        for (scope, status) in sync_status {
            // Validate status is valid
            if !matches!(status, SyncStatus::Idle | SyncStatus::Syncing | SyncStatus::Completed | SyncStatus::Failed) {
                return Err(SafetyViolation::SyncStatusConsistency(
                    format!("Invalid sync status for scope {}: {:?}", scope, status)
                ).into());
            }
            
            // Check dependencies if syncing
            if *status == SyncStatus::Syncing {
                if let Some(dependencies) = sync_dependencies.get(scope) {
                    for dep in dependencies {
                        if let Some(dep_status) = sync_status.get(dep) {
                            if *dep_status != SyncStatus::Completed {
                                return Err(SafetyViolation::SyncStatusConsistency(
                                    format!("Scope {} is syncing but dependency {} is not completed", scope, dep)
                                ).into());
                            }
                        } else {
                            return Err(SafetyViolation::SyncStatusConsistency(
                                format!("Scope {} is syncing but dependency {} not found", scope, dep)
                            ).into());
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentState, SyncStatus};

    #[test]
    fn test_context_validator() {
        let validator = ContextValidator::new();
        
        // Valid YAML
        assert!(validator.validate_yaml_content("key: value").is_ok());
        assert!(validator.validate_yaml_content("").is_ok());
        
        // Invalid YAML
        assert!(validator.validate_yaml_content("invalid: yaml: content:").is_err());
        
        // Scope references
        let all_scopes = vec!["scope1".to_string(), "scope2".to_string()];
        assert!(validator.validate_scope_references("scope1", &all_scopes).is_ok());
        assert!(validator.validate_scope_references("nonexistent", &all_scopes).is_err());
    }

    #[test]
    fn test_dependency_validator() {
        let validator = DependencyValidator::new();
        
        // No circular dependencies
        let mut deps = HashMap::new();
        deps.insert("scope1".to_string(), vec![]);
        deps.insert("scope2".to_string(), vec!["scope1".to_string()]);
        assert!(validator.validate_no_circular_dependencies(&deps).is_ok());
        
        // Circular dependency
        deps.insert("scope1".to_string(), vec!["scope2".to_string()]);
        assert!(validator.validate_no_circular_dependencies(&deps).is_err());
        
        // Dependency bounds
        assert!(validator.validate_dependency_bounds(&["dep1".to_string(), "dep2".to_string()], 3).is_ok());
        assert!(validator.validate_dependency_bounds(&["dep1".to_string(), "dep2".to_string()], 1).is_err());
        
        // No self dependencies
        assert!(validator.validate_no_self_dependencies("scope1", &["dep1".to_string()]).is_ok());
        assert!(validator.validate_no_self_dependencies("scope1", &["scope1".to_string()]).is_err());
    }

    #[test]
    fn test_agent_validator() {
        let validator = AgentValidator::new();
        
        // Valid agent states
        let mut agents = HashMap::new();
        agents.insert("agent1".to_string(), AgentState::Idle);
        agents.insert("agent2".to_string(), AgentState::Working);
        assert!(validator.validate_agent_states(&agents).is_ok());
        
        // Concurrent agents
        let mut locks = HashMap::new();
        locks.insert("scope1".to_string(), Some("agent1".to_string()));
        locks.insert("scope2".to_string(), Some("agent2".to_string()));
        assert!(validator.validate_concurrent_agents(&locks, 3).is_ok());
        assert!(validator.validate_concurrent_agents(&locks, 1).is_err());
    }

    #[test]
    fn test_lock_validator() {
        let validator = LockValidator::new();
        
        // Lock ownership
        let mut locks = HashMap::new();
        locks.insert("scope1".to_string(), Some("agent1".to_string()));
        let agents = vec!["agent1".to_string()];
        assert!(validator.validate_lock_ownership(&locks, &agents).is_ok());
        
        // Invalid agent
        locks.insert("scope2".to_string(), Some("nonexistent".to_string()));
        assert!(validator.validate_lock_ownership(&locks, &agents).is_err());
        
        // One lock per agent
        assert!(validator.validate_one_lock_per_agent(&locks).is_ok());
        
        // Multiple locks per agent
        locks.insert("scope3".to_string(), Some("agent1".to_string()));
        assert!(validator.validate_one_lock_per_agent(&locks).is_err());
    }

    #[test]
    fn test_sync_validator() {
        let validator = SyncValidator::new();
        
        // Valid sync status
        let mut sync_status = HashMap::new();
        sync_status.insert("scope1".to_string(), SyncStatus::Completed);
        sync_status.insert("scope2".to_string(), SyncStatus::Idle);
        
        let mut sync_dependencies = HashMap::new();
        sync_dependencies.insert("scope2".to_string(), vec!["scope1".to_string()]);
        
        assert!(validator.validate_sync_status_consistency(&sync_status, &sync_dependencies).is_ok());
        
        // Invalid dependency state
        sync_status.insert("scope2".to_string(), SyncStatus::Syncing);
        assert!(validator.validate_sync_status_consistency(&sync_status, &sync_dependencies).is_ok());
        
        // Missing dependency
        sync_dependencies.insert("scope3".to_string(), vec!["nonexistent".to_string()]);
        sync_status.insert("scope3".to_string(), SyncStatus::Syncing);
        assert!(validator.validate_sync_status_consistency(&sync_status, &sync_dependencies).is_err());
    }
} 