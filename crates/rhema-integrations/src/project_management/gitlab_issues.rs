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

/// GitLab Issues integration for project management
pub struct GitLabIssuesIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl GitLabIssuesIntegration {
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

    /// Create a GitLab issue
    pub async fn create_issue(
        &self,
        project_id: &str,
        title: &str,
        description: Option<&str>,
        labels: Option<Vec<&str>>,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let mut issue_data = serde_json::json!({
            "title": title
        });

        if let Some(description) = description {
            issue_data["description"] = serde_json::Value::String(description.to_string());
        }

        if let Some(labels) = labels {
            issue_data["labels"] = serde_json::Value::String(labels.join(","));
        }

        let url = format!("{}/api/v4/projects/{}/issues", base_url, project_id);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("PRIVATE-TOKEN".to_string(), token.clone());

        let response = self
            .http_client
            .post(&url, &issue_data.to_string(), Some(headers))
            .await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;

        Ok(issue["iid"].as_u64().unwrap_or(0).to_string())
    }

    /// Get GitLab issue details
    pub async fn get_issue(
        &self,
        project_id: &str,
        issue_iid: &str,
    ) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let url = format!(
            "{}/api/v4/projects/{}/issues/{}",
            base_url, project_id, issue_iid
        );
        let mut headers = HashMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token.clone());

        let response = self.http_client.get(&url, Some(headers)).await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;
        Ok(issue)
    }

    /// Update GitLab issue
    pub async fn update_issue(
        &self,
        project_id: &str,
        issue_iid: &str,
        fields: serde_json::Value,
    ) -> RhemaResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let url = format!(
            "{}/api/v4/projects/{}/issues/{}",
            base_url, project_id, issue_iid
        );
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("PRIVATE-TOKEN".to_string(), token.clone());

        self.http_client
            .put(&url, &fields.to_string(), Some(headers))
            .await?;
        Ok(())
    }

    /// Get issues from a project
    pub async fn get_project_issues(
        &self,
        project_id: &str,
        state: Option<&str>,
    ) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let mut url = format!("{}/api/v4/projects/{}/issues", base_url, project_id);
        if let Some(state) = state {
            url.push_str(&format!("?state={}", state));
        }

        let mut headers = HashMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token.clone());

        let response = self.http_client.get(&url, Some(headers)).await?;
        let issues: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(issues)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for GitLabIssuesIntegration {
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
            name: "gitlab_issues".to_string(),
            version: "1.0.0".to_string(),
            description: "GitLab Issues integration for project management".to_string(),
            integration_type: IntegrationType::GitLabIssues,
            capabilities: vec![
                "issue_management".to_string(),
                "repository_tracking".to_string(),
            ],
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
