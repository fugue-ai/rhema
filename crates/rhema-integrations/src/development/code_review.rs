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

/// Code Review integration
pub struct CodeReviewIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl CodeReviewIntegration {
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

    /// Create a pull request/merge request
    pub async fn create_pull_request(
        &self,
        title: &str,
        description: &str,
        source_branch: &str,
        target_branch: &str,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let pr_data = serde_json::json!({
            "title": title,
            "body": description,
            "head": source_branch,
            "base": target_branch
        });

        let url = format!(
            "{}/repos/{}/pulls",
            base_url,
            config.custom_headers.get("repo").unwrap_or(&"".to_string())
        );
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }

        let response = self
            .http_client
            .post(&url, &pr_data.to_string(), Some(headers))
            .await?;
        let pr: serde_json::Value = serde_json::from_str(&response)?;

        Ok(pr["number"].as_u64().unwrap_or(0).to_string())
    }

    /// Get pull request details
    pub async fn get_pull_request(&self, pr_number: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let url = format!(
            "{}/repos/{}/pulls/{}",
            base_url,
            config.custom_headers.get("repo").unwrap_or(&"".to_string()),
            pr_number
        );
        let mut headers = HashMap::new();

        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }

        let response = self.http_client.get(&url, Some(headers)).await?;
        let pr: serde_json::Value = serde_json::from_str(&response)?;
        Ok(pr)
    }

    /// Add a comment to a pull request
    pub async fn add_comment(
        &self,
        pr_number: &str,
        comment: &str,
        path: Option<&str>,
        line: Option<u32>,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let mut comment_data = serde_json::json!({
            "body": comment
        });

        if let Some(path) = path {
            comment_data["path"] = serde_json::Value::String(path.to_string());
            if let Some(line) = line {
                comment_data["line"] = serde_json::Value::Number(serde_json::Number::from(line));
            }
        }

        let url = format!(
            "{}/repos/{}/pulls/{}/comments",
            base_url,
            config.custom_headers.get("repo").unwrap_or(&"".to_string()),
            pr_number
        );
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }

        let response = self
            .http_client
            .post(&url, &comment_data.to_string(), Some(headers))
            .await?;
        let comment_result: serde_json::Value = serde_json::from_str(&response)?;

        Ok(comment_result["id"].as_u64().unwrap_or(0).to_string())
    }

    /// Get pull request comments
    pub async fn get_comments(&self, pr_number: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config
            .base_url
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;

        let url = format!(
            "{}/repos/{}/pulls/{}/comments",
            base_url,
            config.custom_headers.get("repo").unwrap_or(&"".to_string()),
            pr_number
        );
        let mut headers = HashMap::new();

        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }

        let response = self.http_client.get(&url, Some(headers)).await?;
        let comments: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(comments)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for CodeReviewIntegration {
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
            name: "code_review".to_string(),
            version: "1.0.0".to_string(),
            description: "Code Review integration for development".to_string(),
            integration_type: IntegrationType::CodeReview,
            capabilities: vec![
                "review_management".to_string(),
                "comment_system".to_string(),
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
