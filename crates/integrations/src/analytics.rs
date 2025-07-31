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
    pub async fn track_event(&self, event_name: &str, properties: HashMap<String, serde_json::Value>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let event_data = serde_json::json!({
            "event": event_name,
            "properties": properties,
            "timestamp": chrono::Utc::now().timestamp_millis()
        });
        
        let url = format!("{}/track", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        self.http_client.post(&url, &event_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Identify a user
    pub async fn identify_user(&self, user_id: &str, traits: HashMap<String, serde_json::Value>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let identify_data = serde_json::json!({
            "userId": user_id,
            "traits": traits
        });
        
        let url = format!("{}/identify", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        self.http_client.post(&url, &identify_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Get analytics data
    pub async fn get_analytics_data(&self, query: &str, start_date: &str, end_date: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let query_data = serde_json::json!({
            "query": query,
            "start_date": start_date,
            "end_date": end_date
        });
        
        let url = format!("{}/query", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &query_data.to_string(), Some(headers)).await?;
        let data: serde_json::Value = serde_json::from_str(&response)?;
        Ok(data)
    }
    
    /// Get user analytics
    pub async fn get_user_analytics(&self, user_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Analytics not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/users/{}", base_url, user_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let data: serde_json::Value = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[async_trait]
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
    pub async fn send_metric(&self, metric_name: &str, value: f64, tags: HashMap<String, String>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
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
        
        self.http_client.post(&url, &metric_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Send a log entry
    pub async fn send_log(&self, level: &str, message: &str, context: HashMap<String, serde_json::Value>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
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
        
        self.http_client.post(&url, &log_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Create an alert
    pub async fn create_alert(&self, alert_name: &str, condition: &str, severity: &str) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let alert_data = serde_json::json!({
            "name": alert_name,
            "condition": condition,
            "severity": severity
        });
        
        let url = format!("{}/alerts", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &alert_data.to_string(), Some(headers)).await?;
        let alert: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(alert["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get monitoring dashboard
    pub async fn get_dashboard(&self, dashboard_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Monitoring not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/dashboards/{}", base_url, dashboard_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let dashboard: serde_json::Value = serde_json::from_str(&response)?;
        Ok(dashboard)
    }
}

#[async_trait]
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
    pub async fn log(&self, level: &str, message: &str, fields: HashMap<String, serde_json::Value>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
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
        
        self.http_client.post(&url, &log_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Search logs
    pub async fn search_logs(&self, query: &str, start_time: &str, end_time: &str, limit: Option<u32>) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
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
        
        let response = self.http_client.post(&url, &search_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let logs = result["logs"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(logs)
    }
    
    /// Get log statistics
    pub async fn get_log_stats(&self, time_range: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/logs/stats?time_range={}", base_url, time_range);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let stats: serde_json::Value = serde_json::from_str(&response)?;
        Ok(stats)
    }
    
    /// Create a log alert
    pub async fn create_log_alert(&self, alert_name: &str, query: &str, threshold: u32) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Logging not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let alert_data = serde_json::json!({
            "name": alert_name,
            "query": query,
            "threshold": threshold
        });
        
        let url = format!("{}/alerts", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &alert_data.to_string(), Some(headers)).await?;
        let alert: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(alert["id"].as_str().unwrap_or("").to_string())
    }
}

#[async_trait]
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

#[async_trait]
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

#[async_trait]
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