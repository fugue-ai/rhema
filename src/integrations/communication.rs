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

use super::*;

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

#[async_trait]
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

#[async_trait]
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

/// Microsoft Teams integration for enterprise collaboration
pub struct MicrosoftTeamsIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl MicrosoftTeamsIntegration {
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
    
    /// Send a message to a Teams channel
    pub async fn send_message(&self, webhook_url: &str, text: &str, title: Option<&str>) -> RhemaResult<()> {
        let mut message_data = serde_json::json!({
            "text": text
        });
        
        if let Some(title) = title {
            message_data["title"] = serde_json::Value::String(title.to_string());
        }
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        self.http_client.post(webhook_url, &message_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Send a card message
    pub async fn send_card(&self, webhook_url: &str, card: serde_json::Value) -> RhemaResult<()> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        self.http_client.post(webhook_url, &card.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Send an adaptive card
    pub async fn send_adaptive_card(&self, webhook_url: &str, card: serde_json::Value) -> RhemaResult<()> {
        let message_data = serde_json::json!({
            "type": "message",
            "attachments": [
                {
                    "contentType": "application/vnd.microsoft.card.adaptive",
                    "content": card
                }
            ]
        });
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        self.http_client.post(webhook_url, &message_data.to_string(), Some(headers)).await?;
        Ok(())
    }
}

#[async_trait]
impl ExternalIntegration for MicrosoftTeamsIntegration {
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
            name: "microsoft_teams".to_string(),
            version: "1.0.0".to_string(),
            description: "Microsoft Teams integration for communication".to_string(),
            integration_type: IntegrationType::MicrosoftTeams,
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

/// Email integration for notifications and reporting
pub struct EmailIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl EmailIntegration {
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
    
    /// Send an email using a service like SendGrid or Mailgun
    pub async fn send_email(&self, to: &str, subject: &str, body: &str, from: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Email not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let email_data = serde_json::json!({
            "personalizations": [
                {
                    "to": [
                        {
                            "email": to
                        }
                    ]
                }
            ],
            "from": {
                "email": from.unwrap_or("noreply@rhema.dev")
            },
            "subject": subject,
            "content": [
                {
                    "type": "text/plain",
                    "value": body
                }
            ]
        });
        
        let url = format!("{}/v3/mail/send", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &email_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(result["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Send an HTML email
    pub async fn send_html_email(&self, to: &str, subject: &str, html_body: &str, text_body: Option<&str>, from: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Email not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let mut content = vec![
            serde_json::json!({
                "type": "text/html",
                "value": html_body
            })
        ];
        
        if let Some(text_body) = text_body {
            content.push(serde_json::json!({
                "type": "text/plain",
                "value": text_body
            }));
        }
        
        let email_data = serde_json::json!({
            "personalizations": [
                {
                    "to": [
                        {
                            "email": to
                        }
                    ]
                }
            ],
            "from": {
                "email": from.unwrap_or("noreply@rhema.dev")
            },
            "subject": subject,
            "content": content
        });
        
        let url = format!("{}/v3/mail/send", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &email_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(result["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get email delivery status
    pub async fn get_delivery_status(&self, message_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Email not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/v3/messages/{}", base_url, message_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let status: serde_json::Value = serde_json::from_str(&response)?;
        Ok(status)
    }
}

#[async_trait]
impl ExternalIntegration for EmailIntegration {
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
            name: "email".to_string(),
            version: "1.0.0".to_string(),
            description: "Email integration for communication".to_string(),
            integration_type: IntegrationType::Email,
            capabilities: vec!["email_sending".to_string(), "email_receiving".to_string()],
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