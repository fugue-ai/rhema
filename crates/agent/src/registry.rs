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

use crate::agent::{Agent, AgentId, AgentType, AgentCapability, AgentState, AgentConfig, AgentContext, AgentStatus, AgentMessage, AgentRequest, AgentResponse, AgentHeartbeat, CoordinationMessage, AgentErrorMessage, CustomMessage, ResourceUsage, HealthStatus};
use crate::error::{AgentError, AgentResult};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// Registry entry for an agent
#[derive(Debug, Clone)]
pub struct RegistryEntry {
    /// Agent ID
    pub agent_id: AgentId,
    /// Agent name
    pub name: String,
    /// Agent type
    pub agent_type: AgentType,
    /// Agent capabilities
    pub capabilities: Vec<AgentCapability>,
    /// Agent state
    pub state: AgentState,
    /// Registration time
    pub registered_at: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Agent metadata
    pub metadata: HashMap<String, String>,
}

impl RegistryEntry {
    pub fn new(agent_id: AgentId, name: String, agent_type: AgentType, capabilities: Vec<AgentCapability>) -> Self {
        let now = Utc::now();
        Self {
            agent_id,
            name,
            agent_type,
            capabilities,
            state: AgentState::Initializing,
            registered_at: now,
            last_activity: now,
            metadata: HashMap::new(),
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }

    pub fn update_state(&mut self, state: AgentState) {
        self.state = state;
        self.update_activity();
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Registry query for filtering agents
#[derive(Debug, Clone)]
pub struct RegistryQuery {
    /// Filter by agent type
    pub agent_type: Option<AgentType>,
    /// Filter by agent state
    pub state: Option<AgentState>,
    /// Filter by capability
    pub capability: Option<AgentCapability>,
    /// Filter by name pattern
    pub name_pattern: Option<String>,
    /// Filter by metadata
    pub metadata: HashMap<String, String>,
    /// Limit results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
}

impl Default for RegistryQuery {
    fn default() -> Self {
        Self {
            agent_type: None,
            state: None,
            capability: None,
            name_pattern: None,
            metadata: HashMap::new(),
            limit: None,
            offset: None,
        }
    }
}

impl RegistryQuery {
    pub fn with_agent_type(mut self, agent_type: AgentType) -> Self {
        self.agent_type = Some(agent_type);
        self
    }

    pub fn with_state(mut self, state: AgentState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_capability(mut self, capability: AgentCapability) -> Self {
        self.capability = Some(capability);
        self
    }

    pub fn with_name_pattern(mut self, pattern: String) -> Self {
        self.name_pattern = Some(pattern);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Agent registry for managing agent registration and lifecycle
#[derive(Clone)]
pub struct AgentRegistry {
    /// Registered agents
    agents: DashMap<AgentId, Arc<RwLock<Box<dyn Agent>>>>,
    /// Registry entries for metadata
    entries: DashMap<AgentId, RegistryEntry>,
    /// Agent type index
    type_index: DashMap<AgentType, Vec<AgentId>>,
    /// Agent capability index
    capability_index: DashMap<AgentCapability, Vec<AgentId>>,
    /// Agent state index
    state_index: DashMap<AgentState, Vec<AgentId>>,
    /// Registry statistics
    stats: Arc<RwLock<RegistryStats>>,
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStats {
    /// Total agents registered
    pub total_agents: usize,
    /// Active agents
    pub active_agents: usize,
    /// Agents by type
    pub agents_by_type: HashMap<String, usize>,
    /// Agents by state
    pub agents_by_state: HashMap<String, usize>,
    /// Agents by capability
    pub agents_by_capability: HashMap<String, usize>,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl Default for RegistryStats {
    fn default() -> Self {
        Self {
            total_agents: 0,
            active_agents: 0,
            agents_by_type: HashMap::new(),
            agents_by_state: HashMap::new(),
            agents_by_capability: HashMap::new(),
            last_update: Utc::now(),
        }
    }
}

impl AgentRegistry {
    /// Create a new agent registry
    pub fn new() -> Self {
        Self {
            agents: DashMap::new(),
            entries: DashMap::new(),
            type_index: DashMap::new(),
            capability_index: DashMap::new(),
            state_index: DashMap::new(),
            stats: Arc::new(RwLock::new(RegistryStats::default())),
        }
    }

    /// Initialize the registry
    pub async fn initialize(&self) -> AgentResult<()> {
        // Clear any existing data
        self.agents.clear();
        self.entries.clear();
        self.type_index.clear();
        self.capability_index.clear();
        self.state_index.clear();
        
        // Reset statistics
        let mut stats = self.stats.write().await;
        *stats = RegistryStats::default();
        
        Ok(())
    }

    /// Register an agent
    pub async fn register(&self, agent: Box<dyn Agent>) -> AgentResult<()> {
        let agent_id = agent.id().clone();
        let agent_type = agent.agent_type().clone();
        let capabilities = agent.capabilities().to_vec();
        let state = agent.context().state.clone();

        // Create registry entry
        let entry = RegistryEntry {
            agent_id: agent_id.clone(),
            name: agent.config().name.clone(),
            agent_type: agent_type.clone(),
            capabilities: capabilities.clone(),
            state: state.clone(),
            registered_at: Utc::now(),
            last_activity: Utc::now(),
            metadata: HashMap::new(),
        };

        // Store agent and entry
        self.agents.insert(agent_id.clone(), Arc::new(RwLock::new(agent)));
        self.entries.insert(agent_id.clone(), entry);

        // Update indices
        self.update_type_index(&agent_id, &agent_type).await;
        for capability in &capabilities {
            self.update_capability_index(&agent_id, capability).await;
        }
        self.update_state_index(&agent_id, &state).await;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_agents += 1;
        stats.active_agents += 1;

        Ok(())
    }

    /// Unregister an agent
    pub async fn unregister(&self, agent_id: &AgentId) -> AgentResult<()> {
        // Remove from agents map
        if let Some((_, agent)) = self.agents.remove(agent_id) {
            // Stop the agent
            let mut agent_guard = agent.write().await;
            let _ = agent_guard.stop().await;
        }

        // Remove from entries map
        if let Some((_, entry)) = self.entries.remove(agent_id) {
            // Remove from indices
            self.remove_from_indices(agent_id, &entry).await;
        }

        // Update statistics
        self.update_stats().await;
        
        Ok(())
    }

    /// Get an agent by ID
    pub async fn get_agent(&self, agent_id: &AgentId) -> AgentResult<Arc<RwLock<Box<dyn Agent>>>> {
        self.agents
            .get(agent_id)
            .map(|entry| entry.clone())
            .ok_or_else(|| AgentError::AgentNotFound {
                agent_id: agent_id.clone(),
            })
    }

    /// Get registry entry by ID
    pub async fn get_entry(&self, agent_id: &AgentId) -> AgentResult<RegistryEntry> {
        self.entries
            .get(agent_id)
            .map(|entry| entry.clone())
            .ok_or_else(|| AgentError::AgentNotFound {
                agent_id: agent_id.clone(),
            })
    }

    /// Start an agent
    pub async fn start_agent(&self, agent_id: &AgentId) -> AgentResult<()> {
        let agent = self.get_agent(agent_id).await?;
        let mut agent_guard = agent.write().await;
        
        // Start the agent
        agent_guard.start().await?;
        
        // Update entry state
        if let Some(mut entry) = self.entries.get_mut(agent_id) {
            entry.update_state(AgentState::Ready);
        }
        
        // Update state index
        self.update_state_index(agent_id, &AgentState::Ready).await;
        
        Ok(())
    }

    /// Stop an agent
    pub async fn stop_agent(&self, agent_id: &AgentId) -> AgentResult<()> {
        let agent = self.get_agent(agent_id).await?;
        let mut agent_guard = agent.write().await;
        
        // Stop the agent
        agent_guard.stop().await?;
        
        // Update entry state
        if let Some(mut entry) = self.entries.get_mut(agent_id) {
            entry.update_state(AgentState::Stopped);
        }
        
        // Update state index
        self.update_state_index(agent_id, &AgentState::Stopped).await;
        
        Ok(())
    }

    /// Query agents
    pub async fn query(&self, query: RegistryQuery) -> AgentResult<Vec<RegistryEntry>> {
        let mut results = Vec::new();
        
        for entry in self.entries.iter() {
            let entry = entry.clone();
            
            // Apply filters
            if let Some(ref agent_type) = query.agent_type {
                if entry.agent_type != *agent_type {
                    continue;
                }
            }
            
            if let Some(ref state) = query.state {
                if entry.state != *state {
                    continue;
                }
            }
            
            if let Some(ref capability) = query.capability {
                if !entry.capabilities.contains(capability) {
                    continue;
                }
            }
            
            if let Some(ref pattern) = query.name_pattern {
                if !entry.name.contains(pattern) {
                    continue;
                }
            }
            
            // Check metadata filters
            let mut matches_metadata = true;
            for (key, value) in &query.metadata {
                if entry.get_metadata(key) != Some(value) {
                    matches_metadata = false;
                    break;
                }
            }
            
            if !matches_metadata {
                continue;
            }
            
            results.push(entry);
        }
        
        // Apply pagination
        if let Some(offset) = query.offset {
            if offset >= results.len() {
                return Ok(vec![]);
            }
            results = results.into_iter().skip(offset).collect();
        }
        
        if let Some(limit) = query.limit {
            if limit < results.len() {
                results.truncate(limit);
            }
        }
        
        Ok(results)
    }

    /// Get all agent IDs
    pub async fn get_all_agent_ids(&self) -> AgentResult<Vec<AgentId>> {
        Ok(self.agents.iter().map(|entry| entry.key().clone()).collect())
    }

    /// Get agents by type
    pub async fn get_agents_by_type(&self, agent_type: &AgentType) -> AgentResult<Vec<AgentId>> {
        Ok(self.type_index
            .get(agent_type)
            .map(|entry| entry.clone())
            .unwrap_or_default())
    }

    /// Get agents by capability
    pub async fn get_agents_by_capability(&self, capability: &AgentCapability) -> AgentResult<Vec<AgentId>> {
        Ok(self.capability_index
            .get(capability)
            .map(|entry| entry.clone())
            .unwrap_or_default())
    }

    /// Get agents by state
    pub async fn get_agents_by_state(&self, state: &AgentState) -> AgentResult<Vec<AgentId>> {
        Ok(self.state_index
            .get(state)
            .map(|entry| entry.clone())
            .unwrap_or_default())
    }

    /// Count total agents
    pub async fn count_agents(&self) -> usize {
        self.agents.len()
    }

    /// Count active agents
    pub async fn count_active_agents(&self) -> usize {
        self.state_index
            .get(&AgentState::Ready)
            .map(|entry| entry.len())
            .unwrap_or(0)
    }

    /// Get registry statistics
    pub async fn get_stats(&self) -> RegistryStats {
        self.stats.read().await.clone()
    }

    /// Update agent state
    pub async fn update_agent_state(&self, agent_id: &AgentId, state: AgentState) -> AgentResult<()> {
        if let Some(mut entry) = self.entries.get_mut(agent_id) {
            entry.update_state(state.clone());
            self.update_state_index(agent_id, &state).await;
        }
        
        Ok(())
    }

    /// Update agent activity
    pub async fn update_agent_activity(&self, agent_id: &AgentId) -> AgentResult<()> {
        if let Some(mut entry) = self.entries.get_mut(agent_id) {
            entry.update_activity();
        }
        
        Ok(())
    }

    /// Shutdown the registry
    pub async fn shutdown(&self) -> AgentResult<()> {
        // Stop all agents
        let agent_ids: Vec<AgentId> = self.agents.iter().map(|entry| entry.key().clone()).collect();
        
        for agent_id in agent_ids {
            let _ = self.stop_agent(&agent_id).await;
        }
        
        // Clear all data
        self.agents.clear();
        self.entries.clear();
        self.type_index.clear();
        self.capability_index.clear();
        self.state_index.clear();
        
        // Reset statistics
        let mut stats = self.stats.write().await;
        *stats = RegistryStats::default();
        
        Ok(())
    }

    /// Update type index
    async fn update_type_index(&self, agent_id: &AgentId, agent_type: &AgentType) {
        let mut type_index = self.type_index.get_mut(agent_type).unwrap_or_else(|| {
            self.type_index.insert(agent_type.clone(), Vec::new());
            self.type_index.get_mut(agent_type).unwrap()
        });
        if !type_index.contains(agent_id) {
            type_index.push(agent_id.clone());
        }
    }

    /// Update capability index
    async fn update_capability_index(&self, agent_id: &AgentId, capability: &AgentCapability) {
        let mut capability_index = self.capability_index.get_mut(capability).unwrap_or_else(|| {
            self.capability_index.insert(capability.clone(), Vec::new());
            self.capability_index.get_mut(capability).unwrap()
        });
        if !capability_index.contains(agent_id) {
            capability_index.push(agent_id.clone());
        }
    }

    /// Update state index
    async fn update_state_index(&self, agent_id: &AgentId, new_state: &AgentState) {
        // Remove from old state index
        if let Some(mut entry) = self.entries.get_mut(agent_id) {
            let old_state = entry.state.clone();
            if old_state != *new_state {
                if let Some(mut state_list) = self.state_index.get_mut(&old_state) {
                    state_list.retain(|id| id != agent_id);
                }
            }
        }

        // Add to new state index
        let mut state_index = self.state_index.get_mut(new_state).unwrap_or_else(|| {
            self.state_index.insert(new_state.clone(), Vec::new());
            self.state_index.get_mut(new_state).unwrap()
        });
        if !state_index.contains(agent_id) {
            state_index.push(agent_id.clone());
        }
    }

    /// Remove from indices when unregistering an agent
    async fn remove_from_indices(&self, agent_id: &AgentId, entry: &RegistryEntry) {
        // Remove from type index
        if let Some(mut type_list) = self.type_index.get_mut(&entry.agent_type) {
            type_list.retain(|id| id != agent_id);
        }
        
        // Remove from capability index
        for capability in &entry.capabilities {
            if let Some(mut cap_list) = self.capability_index.get_mut(capability) {
                cap_list.retain(|id| id != agent_id);
            }
        }
        
        // Remove from state index
        if let Some(mut state_list) = self.state_index.get_mut(&entry.state) {
            state_list.retain(|id| id != agent_id);
        }
    }

    /// Update registry statistics
    async fn update_stats(&self) {
        let mut stats = self.stats.write().await;
        
        stats.total_agents = self.agents.len();
        stats.active_agents = self.count_active_agents().await;
        stats.last_update = Utc::now();
        
        // Update agents by type
        stats.agents_by_type.clear();
        for entry in self.type_index.iter() {
            stats.agents_by_type.insert(entry.key().to_string(), entry.len());
        }
        
        // Update agents by state
        stats.agents_by_state.clear();
        for entry in self.state_index.iter() {
            stats.agents_by_state.insert(entry.key().to_string(), entry.len());
        }
        
        // Update agents by capability
        stats.agents_by_capability.clear();
        for entry in self.capability_index.iter() {
            stats.agents_by_capability.insert(entry.key().to_string(), entry.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{BaseAgent, AgentConfig, AgentType, AgentCapability};

    #[tokio::test]
    async fn test_registry_creation() {
        let registry = AgentRegistry::new();
        assert_eq!(registry.count_agents().await, 0);
    }

    #[tokio::test]
    async fn test_agent_registration() {
        let registry = AgentRegistry::new();
        registry.initialize().await.unwrap();
        
        let config = AgentConfig {
            name: "Test Agent".to_string(),
            agent_type: AgentType::Development,
            capabilities: vec![AgentCapability::CodeExecution],
            ..Default::default()
        };
        
        let agent = BaseAgent::new("test-agent".to_string(), config);
        assert!(registry.register(Box::new(agent)).await.is_ok());
        
        assert_eq!(registry.count_agents().await, 1);
    }

    #[tokio::test]
    async fn test_agent_query() {
        let registry = AgentRegistry::new();
        registry.initialize().await.unwrap();
        
        let config = AgentConfig {
            name: "Test Agent".to_string(),
            agent_type: AgentType::Development,
            capabilities: vec![AgentCapability::CodeExecution],
            ..Default::default()
        };
        
        let agent = BaseAgent::new("test-agent".to_string(), config);
        registry.register(Box::new(agent)).await.unwrap();
        
        let query = RegistryQuery::default()
            .with_agent_type(AgentType::Development);
        
        let results = registry.query(query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test Agent");
    }

    #[tokio::test]
    async fn test_agent_unregistration() {
        let registry = AgentRegistry::new();
        registry.initialize().await.unwrap();
        
        let config = AgentConfig {
            name: "Test Agent".to_string(),
            agent_type: AgentType::Development,
            capabilities: vec![AgentCapability::CodeExecution],
            ..Default::default()
        };
        
        let agent = BaseAgent::new("test-agent".to_string(), config);
        registry.register(Box::new(agent)).await.unwrap();
        
        assert_eq!(registry.count_agents().await, 1);
        
        registry.unregister(&"test-agent".to_string()).await.unwrap();
        assert_eq!(registry.count_agents().await, 0);
    }
} 