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

/// ReadTheDocs integration for documentation
pub struct ReadTheDocsIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl ReadTheDocsIntegration {
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

    /// Get project documentation
    pub async fn get_project_docs(&self, project_slug: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
        let default_url = "https://readthedocs.org".to_string();
        let base_url = config.base_url.as_ref().unwrap_or(&default_url);

        let url = format!("{}/api/v3/projects/{}/", base_url, project_slug);
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());

        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Token {}", token));
        }

        let response = self.http_client.get(&url, Some(headers)).await?;
        let project: serde_json::Value = serde_json::from_str(&response)?;
        Ok(project)
    }

    /// Get project builds
    pub async fn get_project_builds(
        &self,
        project_slug: &str,
    ) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
        let default_url = "https://readthedocs.org".to_string();
        let base_url = config.base_url.as_ref().unwrap_or(&default_url);

        let url = format!("{}/api/v3/projects/{}/builds/", base_url, project_slug);
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());

        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Token {}", token));
        }

        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;

        let builds = result["results"].as_array().unwrap_or(&Vec::new()).clone();

        Ok(builds)
    }

    /// Trigger a documentation build
    pub async fn trigger_build(
        &self,
        project_slug: &str,
        version: Option<&str>,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
        let default_url = "https://readthedocs.org".to_string();
        let base_url = config.base_url.as_ref().unwrap_or(&default_url);
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let mut build_data = serde_json::json!({});
        if let Some(version) = version {
            build_data["version"] = serde_json::Value::String(version.to_string());
        }

        let url = format!("{}/api/v3/projects/{}/builds/", base_url, project_slug);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Token {}", token));

        let response = self
            .http_client
            .post(&url, &build_data.to_string(), Some(headers))
            .await?;
        let build: serde_json::Value = serde_json::from_str(&response)?;

        Ok(build["id"].as_u64().unwrap_or(0).to_string())
    }

    /// Search documentation
    pub async fn search_docs(
        &self,
        project_slug: &str,
        query: &str,
    ) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
        let default_url = "https://readthedocs.org".to_string();
        let base_url = config.base_url.as_ref().unwrap_or(&default_url);

        let url = format!(
            "{}/api/v3/projects/{}/search/?q={}",
            base_url, project_slug, query
        );
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());

        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Token {}", token));
        }

        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;

        let results = result["results"].as_array().unwrap_or(&Vec::new()).clone();

        Ok(results)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for ReadTheDocsIntegration {
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
            name: "readthedocs".to_string(),
            version: "1.0.0".to_string(),
            description: "ReadTheDocs integration for documentation".to_string(),
            integration_type: IntegrationType::ReadTheDocs,
            capabilities: vec![
                "project_management".to_string(),
                "build_management".to_string(),
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
