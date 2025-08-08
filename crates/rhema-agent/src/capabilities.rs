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

use crate::agent::{AgentCapability, AgentId};
use crate::error::{AgentError, AgentResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Capability request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    /// Request ID
    pub request_id: String,
    /// Requesting agent ID
    pub requesting_agent: AgentId,
    /// Required capability
    pub capability: AgentCapability,
    /// Request parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Request priority
    pub priority: u8,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
}

impl CapabilityRequest {
    pub fn new(requesting_agent: AgentId, capability: AgentCapability) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            requesting_agent,
            capability,
            parameters: HashMap::new(),
            priority: 5, // Default priority
            timestamp: Utc::now(),
        }
    }

    pub fn with_parameter(mut self, key: String, value: serde_json::Value) -> Self {
        self.parameters.insert(key, value);
        self
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

/// Capability response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponse {
    /// Response ID
    pub response_id: String,
    /// Request ID this response is for
    pub request_id: String,
    /// Providing agent ID
    pub providing_agent: AgentId,
    /// Capability provided
    pub capability: AgentCapability,
    /// Response status
    pub status: CapabilityResponseStatus,
    /// Response data
    pub data: Option<serde_json::Value>,
    /// Error message if any
    pub error: Option<String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

/// Capability response status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityResponseStatus {
    /// Capability available
    Available,
    /// Capability not available
    NotAvailable,
    /// Capability busy
    Busy,
    /// Capability error
    Error,
}

impl std::fmt::Display for CapabilityResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapabilityResponseStatus::Available => write!(f, "Available"),
            CapabilityResponseStatus::NotAvailable => write!(f, "NotAvailable"),
            CapabilityResponseStatus::Busy => write!(f, "Busy"),
            CapabilityResponseStatus::Error => write!(f, "Error"),
        }
    }
}

impl CapabilityResponse {
    pub fn available(
        response_id: String,
        request_id: String,
        providing_agent: AgentId,
        capability: AgentCapability,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            response_id,
            request_id,
            providing_agent,
            capability,
            status: CapabilityResponseStatus::Available,
            data,
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn not_available(
        response_id: String,
        request_id: String,
        providing_agent: AgentId,
        capability: AgentCapability,
        error: String,
    ) -> Self {
        Self {
            response_id,
            request_id,
            providing_agent,
            capability,
            status: CapabilityResponseStatus::NotAvailable,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }

    pub fn busy(
        response_id: String,
        request_id: String,
        providing_agent: AgentId,
        capability: AgentCapability,
    ) -> Self {
        Self {
            response_id,
            request_id,
            providing_agent,
            capability,
            status: CapabilityResponseStatus::Busy,
            data: None,
            error: Some("Capability is currently busy".to_string()),
            timestamp: Utc::now(),
        }
    }
}

/// Capability manager for managing agent capabilities
pub struct CapabilityManager {
    /// Registered capabilities by agent
    agent_capabilities: Arc<RwLock<HashMap<AgentId, Vec<AgentCapability>>>>,
    /// Capability providers
    capability_providers: Arc<RwLock<HashMap<AgentCapability, Vec<AgentId>>>>,
    /// Active capability requests
    active_requests: Arc<RwLock<HashMap<String, CapabilityRequest>>>,
    /// Capability statistics
    stats: Arc<RwLock<CapabilityStats>>,
}

/// Capability statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStats {
    /// Total capabilities registered
    pub total_capabilities: usize,
    /// Capabilities by type
    pub capabilities_by_type: HashMap<String, usize>,
    /// Active requests
    pub active_requests: usize,
    /// Successful requests
    pub successful_requests: usize,
    /// Failed requests
    pub failed_requests: usize,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl Default for CapabilityStats {
    fn default() -> Self {
        Self {
            total_capabilities: 0,
            capabilities_by_type: HashMap::new(),
            active_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            last_update: Utc::now(),
        }
    }
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            agent_capabilities: Arc::new(RwLock::new(HashMap::new())),
            capability_providers: Arc::new(RwLock::new(HashMap::new())),
            active_requests: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CapabilityStats::default())),
        }
    }

    /// Initialize the capability manager
    pub async fn initialize(&self) -> AgentResult<()> {
        // Clear any existing data
        self.agent_capabilities.write().await.clear();
        self.capability_providers.write().await.clear();
        self.active_requests.write().await.clear();

        // Reset statistics
        let mut stats = self.stats.write().await;
        *stats = CapabilityStats::default();

        Ok(())
    }

    /// Register capabilities for an agent
    pub async fn register_capabilities(
        &self,
        agent_id: &AgentId,
        capabilities: Vec<AgentCapability>,
    ) -> AgentResult<()> {
        // Register agent capabilities
        {
            let mut agent_caps = self.agent_capabilities.write().await;
            agent_caps.insert(agent_id.clone(), capabilities.clone());
        }

        // Register capability providers
        {
            let mut providers = self.capability_providers.write().await;
            for capability in &capabilities {
                providers
                    .entry(capability.clone())
                    .or_insert_with(Vec::new)
                    .push(agent_id.clone());
            }
        }

        // Update statistics
        self.update_stats().await;

        Ok(())
    }

    /// Unregister capabilities for an agent
    pub async fn unregister_capabilities(&self, agent_id: &AgentId) -> AgentResult<()> {
        // Get agent capabilities
        let capabilities = {
            let agent_caps = self.agent_capabilities.read().await;
            agent_caps.get(agent_id).cloned().unwrap_or_default()
        };

        // Remove from agent capabilities
        {
            let mut agent_caps = self.agent_capabilities.write().await;
            agent_caps.remove(agent_id);
        }

        // Remove from capability providers
        {
            let mut providers = self.capability_providers.write().await;
            for capability in &capabilities {
                if let Some(agent_list) = providers.get_mut(capability) {
                    agent_list.retain(|id| id != agent_id);

                    // Remove capability if no providers left
                    if agent_list.is_empty() {
                        providers.remove(capability);
                    }
                }
            }
        }

        // Update statistics
        self.update_stats().await;

        Ok(())
    }

    /// Get capabilities for an agent
    pub async fn get_agent_capabilities(&self, agent_id: &AgentId) -> Vec<AgentCapability> {
        let agent_caps = self.agent_capabilities.read().await;
        agent_caps.get(agent_id).cloned().unwrap_or_default()
    }

    /// Get agents with a specific capability
    pub async fn get_capability_providers(&self, capability: &AgentCapability) -> Vec<AgentId> {
        let providers = self.capability_providers.read().await;
        providers.get(capability).cloned().unwrap_or_default()
    }

    /// Check if an agent has a capability
    pub async fn has_capability(&self, agent_id: &AgentId, capability: &AgentCapability) -> bool {
        let agent_caps = self.agent_capabilities.read().await;
        agent_caps
            .get(agent_id)
            .map(|caps| caps.contains(capability))
            .unwrap_or(false)
    }

    /// Request a capability
    pub async fn request_capability(
        &self,
        request: CapabilityRequest,
    ) -> AgentResult<CapabilityResponse> {
        // Get capability providers
        let providers = self.get_capability_providers(&request.capability).await;

        if providers.is_empty() {
            return Err(AgentError::CapabilityNotAvailable {
                capability: request.capability.to_string(),
            });
        }

        // Store active request
        {
            let mut active_requests = self.active_requests.write().await;
            active_requests.insert(request.request_id.clone(), request.clone());
        }

        // Find available provider (simple round-robin for now)
        let provider = providers.first().unwrap();

        // Create response
        let response = CapabilityResponse::available(
            uuid::Uuid::new_v4().to_string(),
            request.request_id.clone(),
            provider.clone(),
            request.capability.clone(),
            None,
        );

        // Remove from active requests
        {
            let mut active_requests = self.active_requests.write().await;
            active_requests.remove(&request.request_id);
        }

        // Update statistics
        self.update_stats().await;

        Ok(response)
    }

    /// Get active capability requests
    pub async fn get_active_requests(&self) -> Vec<CapabilityRequest> {
        let active_requests = self.active_requests.read().await;
        active_requests.values().cloned().collect()
    }

    /// Get capability statistics
    pub async fn get_stats(&self) -> CapabilityStats {
        self.stats.read().await.clone()
    }

    /// Update capability statistics
    async fn update_stats(&self) {
        let agent_caps = self.agent_capabilities.read().await;
        let active_requests = self.active_requests.read().await;
        let mut stats = self.stats.write().await;

        // Count total capabilities
        stats.total_capabilities = agent_caps.values().map(|caps| caps.len()).sum();

        // Count capabilities by type
        stats.capabilities_by_type.clear();
        for capabilities in agent_caps.values() {
            for capability in capabilities {
                *stats
                    .capabilities_by_type
                    .entry(capability.to_string())
                    .or_insert(0) += 1;
            }
        }

        // Update request counts
        stats.active_requests = active_requests.len();
        stats.last_update = Utc::now();
    }

    /// Shutdown the capability manager
    pub async fn shutdown(&self) -> AgentResult<()> {
        // Clear all data
        self.agent_capabilities.write().await.clear();
        self.capability_providers.write().await.clear();
        self.active_requests.write().await.clear();

        // Reset statistics
        let mut stats = self.stats.write().await;
        *stats = CapabilityStats::default();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentCapability;

    #[tokio::test]
    async fn test_capability_manager_creation() {
        let manager = CapabilityManager::new();
        assert!(manager.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_capability_registration() {
        let manager = CapabilityManager::new();
        manager.initialize().await.unwrap();

        let agent_id = "test-agent".to_string();
        let capabilities = vec![AgentCapability::CodeExecution, AgentCapability::FileRead];

        assert!(manager
            .register_capabilities(&agent_id, capabilities.clone())
            .await
            .is_ok());

        let registered_caps = manager.get_agent_capabilities(&agent_id).await;
        assert_eq!(registered_caps.len(), 2);
        assert!(registered_caps.contains(&AgentCapability::CodeExecution));
    }

    #[tokio::test]
    async fn test_capability_providers() {
        let manager = CapabilityManager::new();
        manager.initialize().await.unwrap();

        let agent_id = "test-agent".to_string();
        let capabilities = vec![AgentCapability::CodeExecution];

        manager
            .register_capabilities(&agent_id, capabilities)
            .await
            .unwrap();

        let providers = manager
            .get_capability_providers(&AgentCapability::CodeExecution)
            .await;
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0], agent_id);
    }

    #[tokio::test]
    async fn test_capability_request() {
        let manager = CapabilityManager::new();
        manager.initialize().await.unwrap();

        let agent_id = "test-agent".to_string();
        let capabilities = vec![AgentCapability::CodeExecution];

        manager
            .register_capabilities(&agent_id, capabilities)
            .await
            .unwrap();

        let request = CapabilityRequest::new(
            "requesting-agent".to_string(),
            AgentCapability::CodeExecution,
        );

        let response = manager.request_capability(request).await.unwrap();
        assert_eq!(response.status, CapabilityResponseStatus::Available);
        assert_eq!(response.providing_agent, agent_id);
    }

    #[test]
    fn test_capability_request_creation() {
        let agent_id = "test-agent".to_string();
        let request = CapabilityRequest::new(agent_id.clone(), AgentCapability::CodeExecution)
            .with_parameter("param1".to_string(), serde_json::json!("value1"))
            .with_priority(10);

        assert_eq!(request.requesting_agent, agent_id);
        assert_eq!(request.capability, AgentCapability::CodeExecution);
        assert_eq!(request.priority, 10);
        assert!(request.parameters.contains_key("param1"));
    }

    #[test]
    fn test_capability_response_creation() {
        let response = CapabilityResponse::available(
            "response-id".to_string(),
            "request-id".to_string(),
            "provider-agent".to_string(),
            AgentCapability::CodeExecution,
            Some(serde_json::json!({"result": "success"})),
        );

        assert_eq!(response.status, CapabilityResponseStatus::Available);
        assert_eq!(response.providing_agent, "provider-agent");
        assert!(response.data.is_some());
    }
}
