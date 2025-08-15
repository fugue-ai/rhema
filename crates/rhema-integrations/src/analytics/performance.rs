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

/// Performance monitoring integration
pub struct PerformanceIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl PerformanceIntegration {
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
    
    /// Record a performance metric
    pub async fn record_metric(&self, metric_name: &str, value: f64, unit: &str, tags: HashMap<String, String>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Performance not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let metric_data = serde_json::json!({
            "metric": metric_name,
            "value": value,
            "unit": unit,
            "tags": tags,
            "timestamp": chrono::Utc::now().timestamp()
        });
        
        let url = format!("{}/metrics", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        self.http_client.post(&url, &metric_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Record a trace
    pub async fn record_trace(&self, trace_id: &str, spans: Vec<serde_json::Value>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Performance not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let trace_data = serde_json::json!({
            "trace_id": trace_id,
            "spans": spans,
            "timestamp": chrono::Utc::now().timestamp()
        });
        
        let url = format!("{}/traces", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        self.http_client.post(&url, &trace_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Get performance dashboard
    pub async fn get_performance_dashboard(&self, dashboard_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Performance not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/dashboards/{}", base_url, dashboard_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let dashboard: serde_json::Value = serde_json::from_str(&response)?;
        Ok(dashboard)
    }
    
    /// Get performance alerts
    pub async fn get_performance_alerts(&self) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Performance not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/alerts", base_url);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let alerts = result["alerts"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(alerts)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for PerformanceIntegration {
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
            name: "performance".to_string(),
            version: "1.0.0".to_string(),
            description: "Performance integration for performance monitoring".to_string(),
            integration_type: IntegrationType::Performance,
            capabilities: vec!["performance_monitoring".to_string(), "profiling".to_string()],
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
