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
use base64::Engine;

/// Jira integration for issue and project tracking
pub struct JiraIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl JiraIntegration {
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
    
    /// Create a Jira issue
    pub async fn create_issue(&self, project_key: &str, summary: &str, description: &str, issue_type: &str) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Jira not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let issue_data = serde_json::json!({
            "fields": {
                "project": {
                    "key": project_key
                },
                "summary": summary,
                "description": description,
                "issuetype": {
                    "name": issue_type
                }
            }
        });
        
        let url = format!("{}/rest/api/2/issue", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.post(&url, &issue_data.to_string(), Some(headers)).await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(issue["key"].as_str().unwrap_or("").to_string())
    }
    
    /// Get Jira issue details
    pub async fn get_issue(&self, issue_key: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Jira not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/rest/api/2/issue/{}", base_url, issue_key);
        let mut headers = HashMap::new();
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;
        Ok(issue)
    }
    
    /// Update Jira issue
    pub async fn update_issue(&self, issue_key: &str, fields: serde_json::Value) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Jira not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let update_data = serde_json::json!({
            "fields": fields
        });
        
        let url = format!("{}/rest/api/2/issue/{}", base_url, issue_key);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        self.http_client.put(&url, &update_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Search Jira issues
    pub async fn search_issues(&self, jql: &str, max_results: Option<u32>) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Jira not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let search_data = serde_json::json!({
            "jql": jql,
            "maxResults": max_results.unwrap_or(50),
            "fields": ["summary", "description", "status", "assignee", "created", "updated"]
        });
        
        let url = format!("{}/rest/api/2/search", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.post(&url, &search_data.to_string(), Some(headers)).await?;
        let search_result: serde_json::Value = serde_json::from_str(&response)?;
        
        let issues = search_result["issues"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(issues)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for JiraIntegration {
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
            name: "jira".to_string(),
            version: "1.0.0".to_string(),
            description: "Jira integration for project management".to_string(),
            integration_type: IntegrationType::Jira,
            capabilities: vec!["issue_management".to_string(), "project_tracking".to_string()],
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
