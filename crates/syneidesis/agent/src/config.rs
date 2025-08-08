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

//! Configuration types for the agent crate

// Re-export configuration types from syneidesis-config
pub use syneidesis_config::types::{AgentConfig, CoordinationConfig, HttpConfig, WebSocketConfig};

// Note: The following local config types have been removed and replaced with
// centralized types from syneidesis-config:
// - CoordinationConfig -> syneidesis_config::types::CoordinationConfig
// - AgentConfig -> syneidesis_config::types::AgentConfig
// - HttpConfig -> syneidesis_config::types::HttpConfig
// - WebSocketConfig -> syneidesis_config::types::WebSocketConfig

// Local configuration types that are specific to the agent crate
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Task configuration specific to agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    /// Maximum number of tasks
    pub max_tasks: usize,
    /// Task timeout
    pub task_timeout: Duration,
    /// Retry attempts
    pub retry_attempts: u32,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            max_tasks: 1000,
            task_timeout: Duration::from_secs(3600),
            retry_attempts: 3,
        }
    }
}

/// Communication configuration specific to agents
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommunicationConfig {
    /// WebSocket configuration
    pub websocket: WebSocketConfig,
    /// HTTP configuration
    pub http: HttpConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_default_config() {
        let agent_config = AgentConfig::default();
        let coordination_config = CoordinationConfig::default();

        assert!(agent_config.validate().is_ok());
        assert!(coordination_config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let agent_config = AgentConfig::default();
        let json = serde_json::to_string(&agent_config).unwrap();
        let deserialized: AgentConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(agent_config.max_agents, deserialized.max_agents);
    }
}
