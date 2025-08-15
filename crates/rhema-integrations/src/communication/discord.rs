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

/// Discord integration for community management
pub struct DiscordIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl DiscordIntegration {
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
    
    /// Send a message to a Discord channel
    pub async fn send_message(&self, channel_id: &str, content: &str, embeds: Option<Vec<serde_json::Value>>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Discord not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let mut message_data = serde_json::json!({
            "content": content
        });
        
        if let Some(embeds) = embeds {
            message_data["embeds"] = serde_json::Value::Array(embeds);
        }
        
        let url = format!("https://discord.com/api/v10/channels/{}/messages", channel_id);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bot {}", token));
        
        let response = self.http_client.post(&url, &message_data.to_string(), Some(headers)).await?;
        let message: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(message["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get channel information
    pub async fn get_channel(&self, channel_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Discord not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://discord.com/api/v10/channels/{}", channel_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bot {}", token));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let channel: serde_json::Value = serde_json::from_str(&response)?;
        Ok(channel)
    }
    
    /// Get guild (server) channels
    pub async fn get_guild_channels(&self, guild_id: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Discord not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://discord.com/api/v10/guilds/{}/channels", guild_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bot {}", token));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let channels: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(channels)
    }
    
    /// Create a webhook message
    pub async fn send_webhook(&self, webhook_url: &str, content: &str, embeds: Option<Vec<serde_json::Value>>) -> RhemaResult<()> {
        let mut message_data = serde_json::json!({
            "content": content
        });
        
        if let Some(embeds) = embeds {
            message_data["embeds"] = serde_json::Value::Array(embeds);
        }
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        self.http_client.post(webhook_url, &message_data.to_string(), Some(headers)).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for DiscordIntegration {
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
            name: "discord".to_string(),
            version: "1.0.0".to_string(),
            description: "Discord integration for communication".to_string(),
            integration_type: IntegrationType::Discord,
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
