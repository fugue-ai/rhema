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

/// Analytics platform integration
pub struct AnalyticsIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl AnalyticsIntegration {
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

    /// Track an event
    pub async fn track_event(
        &self,
        event_name: &str,
        properties: HashMap<String, serde_json::Value>,
    ) -> RhemaResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let event_data = serde_json::json!({
            "event": event_name,
            "properties": properties,
            "timestamp": chrono::Utc::now().timestamp_millis()
        });

        let url = format!("{}/track", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        self.http_client
            .post(&url, &event_data.to_string(), Some(headers))
            .await?;
        Ok(())
    }

    /// Identify a user
    pub async fn identify_user(
        &self,
        user_id: &str,
        traits: HashMap<String, serde_json::Value>,
    ) -> RhemaResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let identify_data = serde_json::json!({
            "userId": user_id,
            "traits": traits
        });

        let url = format!("{}/identify", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        self.http_client
            .post(&url, &identify_data.to_string(), Some(headers))
            .await?;
        Ok(())
    }

    /// Get analytics data
    pub async fn get_analytics_data(
        &self,
        query: &str,
        start_date: &str,
        end_date: &str,
    ) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let query_data = serde_json::json!({
            "query": query,
            "start_date": start_date,
            "end_date": end_date
        });

        let url = format!("{}/query", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        let response = self
            .http_client
            .post(&url, &query_data.to_string(), Some(headers))
            .await?;
        let data: serde_json::Value = serde_json::from_str(&response)?;
        Ok(data)
    }

    /// Get user analytics
    pub async fn get_user_analytics(&self, user_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let url = format!("{}/users/{}", base_url, user_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        let response = self.http_client.get(&url, Some(headers)).await?;
        let data: serde_json::Value = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for AnalyticsIntegration {
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
            name: "analytics".to_string(),
            version: "1.0.0".to_string(),
            description: "Analytics integration for data analysis".to_string(),
            integration_type: IntegrationType::Analytics,
            capabilities: vec!["data_analysis".to_string(), "reporting".to_string()],
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
