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

/// Logging platform integration
pub struct LoggingIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl LoggingIntegration {
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

    /// Send a log entry
    pub async fn log(
        &self,
        level: &str,
        message: &str,
        fields: HashMap<String, serde_json::Value>,
    ) -> RhemaResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let log_data = serde_json::json!({
            "level": level,
            "message": message,
            "fields": fields,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let url = format!("{}/logs", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        self.http_client
            .post(&url, &log_data.to_string(), Some(headers))
            .await?;
        Ok(())
    }

    /// Search logs
    pub async fn search_logs(
        &self,
        query: &str,
        start_time: &str,
        end_time: &str,
        limit: Option<u32>,
    ) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let search_data = serde_json::json!({
            "query": query,
            "start_time": start_time,
            "end_time": end_time,
            "limit": limit.unwrap_or(100)
        });

        let url = format!("{}/logs/search", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        let response = self
            .http_client
            .post(&url, &search_data.to_string(), Some(headers))
            .await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;

        let logs = result["logs"].as_array().unwrap_or(&Vec::new()).clone();

        Ok(logs)
    }

    /// Get log statistics
    pub async fn get_log_stats(&self, time_range: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let url = format!("{}/logs/stats?time_range={}", base_url, time_range);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        let response = self.http_client.get(&url, Some(headers)).await?;
        let stats: serde_json::Value = serde_json::from_str(&response)?;
        Ok(stats)
    }

    /// Create a log alert
    pub async fn create_log_alert(
        &self,
        alert_name: &str,
        query: &str,
        threshold: u32,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let alert_data = serde_json::json!({
            "name": alert_name,
            "query": query,
            "threshold": threshold
        });

        let url = format!("{}/alerts", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        let response = self
            .http_client
            .post(&url, &alert_data.to_string(), Some(headers))
            .await?;
        let alert: serde_json::Value = serde_json::from_str(&response)?;

        Ok(alert["id"].as_str().unwrap_or("").to_string())
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for LoggingIntegration {
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
            name: "logging".to_string(),
            version: "1.0.0".to_string(),
            description: "Logging integration for log management".to_string(),
            integration_type: IntegrationType::Logging,
            capabilities: vec!["log_management".to_string(), "log_analysis".to_string()],
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
