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

/// GitHub Issues integration for project tracking
pub struct GitHubIssuesIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl GitHubIssuesIntegration {
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
    
    /// Create a GitHub issue
    pub async fn create_issue(&self, owner: &str, repo: &str, title: &str, body: Option<&str>, labels: Option<Vec<&str>>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitHub not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let mut issue_data = serde_json::json!({
            "title": title
        });
        
        if let Some(body) = body {
            issue_data["body"] = serde_json::Value::String(body.to_string());
        }
        
        if let Some(labels) = labels {
            issue_data["labels"] = serde_json::Value::Array(
                labels.iter().map(|l| serde_json::Value::String(l.to_string())).collect()
            );
        }
        
        let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("User-Agent".to_string(), "RHEMA".to_string());
        
        let response = self.http_client.post(&url, &issue_data.to_string(), Some(headers)).await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(issue["number"].as_u64().unwrap_or(0).to_string())
    }
    
    /// Get GitHub issue details
    pub async fn get_issue(&self, owner: &str, repo: &str, issue_number: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitHub not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://api.github.com/repos/{}/{}/issues/{}", owner, repo, issue_number);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("User-Agent".to_string(), "RHEMA".to_string());
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;
        Ok(issue)
    }
    
    /// Update GitHub issue
    pub async fn update_issue(&self, owner: &str, repo: &str, issue_number: &str, fields: serde_json::Value) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitHub not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://api.github.com/repos/{}/{}/issues/{}", owner, repo, issue_number);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("User-Agent".to_string(), "RHEMA".to_string());
        
        self.http_client.patch(&url, &fields.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Get issues from a repository
    pub async fn get_repo_issues(&self, owner: &str, repo: &str, state: Option<&str>) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitHub not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let mut url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
        if let Some(state) = state {
            url.push_str(&format!("?state={}", state));
        }
        
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("User-Agent".to_string(), "RHEMA".to_string());
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let issues: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(issues)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for GitHubIssuesIntegration {
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
            name: "github_issues".to_string(),
            version: "1.0.0".to_string(),
            description: "GitHub Issues integration for project management".to_string(),
            integration_type: IntegrationType::GitHubIssues,
            capabilities: vec!["issue_management".to_string(), "repository_tracking".to_string()],
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
