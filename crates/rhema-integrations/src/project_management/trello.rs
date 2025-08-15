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

use crate::{
    ExternalIntegration, IntegrationConfig, IntegrationHttpClient, IntegrationMetadata,
    IntegrationStatus, IntegrationType,
};
use rhema_core::{RhemaError, RhemaResult};

use std::collections::HashMap;

/// Trello integration for board management
pub struct TrelloIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl TrelloIntegration {
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

    /// Create a Trello card
    pub async fn create_card(
        &self,
        list_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let mut params = format!(
            "idList={}&key={}&token={}&name={}",
            list_id, api_key, token, name
        );

        if let Some(description) = description {
            params.push_str(&format!("&desc={}", description));
        }

        let url = format!("https://api.trello.com/1/cards?{}", params);

        let response = self.http_client.post(&url, "", None).await?;
        let card: serde_json::Value = serde_json::from_str(&response)?;

        Ok(card["id"].as_str().unwrap_or("").to_string())
    }

    /// Get Trello card details
    pub async fn get_card(&self, card_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let url = format!(
            "https://api.trello.com/1/cards/{}?key={}&token={}",
            card_id, api_key, token
        );

        let response = self.http_client.get(&url, None).await?;
        let card: serde_json::Value = serde_json::from_str(&response)?;
        Ok(card)
    }

    /// Update Trello card
    pub async fn update_card(&self, card_id: &str, fields: serde_json::Value) -> RhemaResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let url = format!(
            "https://api.trello.com/1/cards/{}?key={}&token={}",
            card_id, api_key, token
        );

        self.http_client
            .put(&url, &fields.to_string(), None)
            .await?;
        Ok(())
    }

    /// Get cards from a board
    pub async fn get_board_cards(&self, board_id: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let url = format!(
            "https://api.trello.com/1/boards/{}/cards?key={}&token={}",
            board_id, api_key, token
        );

        let response = self.http_client.get(&url, None).await?;
        let cards: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(cards)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for TrelloIntegration {
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
            name: "trello".to_string(),
            version: "1.0.0".to_string(),
            description: "Trello integration for project management".to_string(),
            integration_type: IntegrationType::Trello,
            capabilities: vec!["board_management".to_string(), "card_tracking".to_string()],
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
