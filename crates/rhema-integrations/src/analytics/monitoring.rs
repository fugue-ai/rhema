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

/// Monitoring tool integration
pub struct MonitoringIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl MonitoringIntegration {
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

    /// Send a metric
    pub async fn send_metric(
        &self,
        metric_name: &str,
        value: f64,
        tags: HashMap<String, String>,
    ) -> RhemaResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let metric_data = serde_json::json!({
            "metric": metric_name,
            "value": value,
            "tags": tags,
            "timestamp": chrono::Utc::now().timestamp()
        });

        let url = format!("{}/metrics", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        self.http_client
            .post(&url, &metric_data.to_string(), Some(headers))
            .await?;
        Ok(())
    }

    /// Send a log entry
    pub async fn send_log(
        &self,
        level: &str,
        message: &str,
        context: HashMap<String, serde_json::Value>,
    ) -> RhemaResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
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
            "context": context,
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

    /// Create an alert
    pub async fn create_alert(
        &self,
        alert_name: &str,
        condition: &str,
        severity: &str,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
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
            "condition": condition,
            "severity": severity
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

    /// Get monitoring dashboard
    pub async fn get_dashboard(&self, dashboard_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let url = format!("{}/dashboards/{}", base_url, dashboard_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        let response = self.http_client.get(&url, Some(headers)).await?;
        let dashboard: serde_json::Value = serde_json::from_str(&response)?;
        Ok(dashboard)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for MonitoringIntegration {
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
            name: "monitoring".to_string(),
            version: "1.0.0".to_string(),
            description: "Monitoring integration for system monitoring".to_string(),
            integration_type: IntegrationType::Monitoring,
            capabilities: vec!["system_monitoring".to_string(), "alerting".to_string()],
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
