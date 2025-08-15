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

#[async_trait::async_trait]
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
