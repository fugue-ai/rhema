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

use crate::{ExternalIntegration, IntegrationConfig, IntegrationStatus, IntegrationMetadata, IntegrationHttpClient, IntegrationType};
use rhema_core::{RhemaResult, RhemaError};

use std::collections::HashMap;

/// Slack integration for notifications and collaboration
pub struct SlackIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl SlackIntegration {
    pub fn new() -> Self {
        Self {
            config: None,
            http_client: IntegrationHttpClient::new(),
            status: IntegrationStatus {
                connected: false,
                last_check: chrono::Utc::now(),
                error_message: None,
                response_time_ms: None,
                rate_limit_remaining: None,
                rate_limit_reset: None,
            },
        }
    }
    
    /// Send a message to a Slack channel
    pub async fn send_message(&self, channel: &str, text: &str, attachments: Option<Vec<serde_json::Value>>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Slack not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let mut message_data = serde_json::json!({
            "channel": channel,
            "text": text
        });
        
        if let Some(attachments) = attachments {
            message_data["attachments"] = serde_json::Value::Array(attachments);
        }
        
        let url = "https://slack.com/api/chat.postMessage";
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        let response = self.http_client.post(url, &message_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        if result["ok"].as_bool().unwrap_or(false) {
            Ok(result["ts"].as_str().unwrap_or("").to_string())
        } else {
            Err(RhemaError::ExternalServiceError(format!("Slack API error: {}", result["error"].as_str().unwrap_or("Unknown error"))))
        }
    }
    
    /// Send a message with blocks (rich formatting)
    pub async fn send_blocks(&self, channel: &str, blocks: Vec<serde_json::Value>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Slack not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let message_data = serde_json::json!({
            "channel": channel,
            "blocks": blocks
        });
        
        let url = "https://slack.com/api/chat.postMessage";
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        let response = self.http_client.post(url, &message_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        if result["ok"].as_bool().unwrap_or(false) {
            Ok(result["ts"].as_str().unwrap_or("").to_string())
        } else {
            Err(RhemaError::ExternalServiceError(format!("Slack API error: {}", result["error"].as_str().unwrap_or("Unknown error"))))
        }
    }
    
    /// Get channel information
    pub async fn get_channel_info(&self, channel: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Slack not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://slack.com/api/conversations.info?channel={}", channel);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        if result["ok"].as_bool().unwrap_or(false) {
            Ok(result["channel"].clone())
        } else {
            Err(RhemaError::ExternalServiceError(format!("Slack API error: {}", result["error"].as_str().unwrap_or("Unknown error"))))
        }
    }
    
    /// List channels
    pub async fn list_channels(&self) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Slack not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = "https://slack.com/api/conversations.list";
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        let response = self.http_client.get(url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        if result["ok"].as_bool().unwrap_or(false) {
            let channels = result["channels"]
                .as_array()
                .unwrap_or(&Vec::new())
                .clone();
            Ok(channels)
        } else {
            Err(RhemaError::ExternalServiceError(format!("Slack API error: {}", result["error"].as_str().unwrap_or("Unknown error"))))
        }
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for SlackIntegration {
    async fn initialize(&mut self, config: IntegrationConfig) -> RhemaResult<()> {
        self.config = Some(config);
        self.status.connected = true;
        self.status.last_check = chrono::Utc::now();
        Ok(())
    }

    async fn test_connection(&self) -> RhemaResult<bool> {
        Ok(self.config.is_some())
    }

    fn get_metadata(&self) -> IntegrationMetadata {
        IntegrationMetadata {
            name: "slack".to_string(),
            version: "1.0.0".to_string(),
            description: "Slack integration for communication".to_string(),
            integration_type: IntegrationType::Slack,
            capabilities: vec!["messaging".to_string(), "channel_management".to_string()],
            required_config: vec!["api_key".to_string()],
            optional_config: vec!["base_url".to_string()],
        }
    }

    async fn get_status(&self) -> RhemaResult<IntegrationStatus> {
        Ok(self.status.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
