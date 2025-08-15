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

/// Business Intelligence integration
pub struct BusinessIntelligenceIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl BusinessIntelligenceIntegration {
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
    
    /// Execute a query
    pub async fn execute_query(&self, query: &str, parameters: HashMap<String, serde_json::Value>) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Business Intelligence not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let query_data = serde_json::json!({
            "query": query,
            "parameters": parameters
        });
        
        let url = format!("{}/query", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &query_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        Ok(result)
    }
    
    /// Get report data
    pub async fn get_report(&self, report_id: &str, parameters: HashMap<String, serde_json::Value>) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Business Intelligence not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let report_data = serde_json::json!({
            "parameters": parameters
        });
        
        let url = format!("{}/reports/{}/execute", base_url, report_id);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &report_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        Ok(result)
    }
    
    /// Create a dashboard
    pub async fn create_dashboard(&self, name: &str, widgets: Vec<serde_json::Value>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Business Intelligence not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let dashboard_data = serde_json::json!({
            "name": name,
            "widgets": widgets
        });
        
        let url = format!("{}/dashboards", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &dashboard_data.to_string(), Some(headers)).await?;
        let dashboard: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(dashboard["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get available data sources
    pub async fn get_data_sources(&self) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Business Intelligence not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/datasources", base_url);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let data_sources = result["data_sources"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(data_sources)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for BusinessIntelligenceIntegration {
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
            name: "business_intelligence".to_string(),
            version: "1.0.0".to_string(),
            description: "Business Intelligence integration for data analysis".to_string(),
            integration_type: IntegrationType::BusinessIntelligence,
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
