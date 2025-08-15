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

/// Notion integration for knowledge management
pub struct NotionIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl NotionIntegration {
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

    /// Create a Notion page
    pub async fn create_page(
        &self,
        parent_id: &str,
        title: &str,
        content: Option<&str>,
    ) -> RhemaResult<String> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let mut page_data = serde_json::json!({
            "parent": {
                "database_id": parent_id
            },
            "properties": {
                "Name": {
                    "title": [
                        {
                            "text": {
                                "content": title
                            }
                        }
                    ]
                }
            }
        });

        if let Some(content) = content {
            page_data["children"] = serde_json::json!([
                {
                    "object": "block",
                    "type": "paragraph",
                    "paragraph": {
                        "rich_text": [
                            {
                                "type": "text",
                                "text": {
                                    "content": content
                                }
                            }
                        ]
                    }
                }
            ]);
        }

        let url = "https://api.notion.com/v1/pages";
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("Notion-Version".to_string(), "2022-06-28".to_string());

        let response = self
            .http_client
            .post(url, &page_data.to_string(), Some(headers))
            .await?;
        let page: serde_json::Value = serde_json::from_str(&response)?;

        Ok(page["id"].as_str().unwrap_or("").to_string())
    }

    /// Get Notion page content
    pub async fn get_page(&self, page_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let url = format!("https://api.notion.com/v1/pages/{}", page_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("Notion-Version".to_string(), "2022-06-28".to_string());

        let response = self.http_client.get(&url, Some(headers)).await?;
        let page: serde_json::Value = serde_json::from_str(&response)?;
        Ok(page)
    }

    /// Get Notion page blocks
    pub async fn get_page_blocks(&self, page_id: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let url = format!("https://api.notion.com/v1/blocks/{}/children", page_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("Notion-Version".to_string(), "2022-06-28".to_string());

        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;

        let blocks = result["results"].as_array().unwrap_or(&Vec::new()).clone();

        Ok(blocks)
    }

    /// Search Notion pages
    pub async fn search_pages(&self, query: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config
            .token
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;

        let search_data = serde_json::json!({
            "query": query,
            "filter": {
                "property": "object",
                "value": "page"
            }
        });

        let url = "https://api.notion.com/v1/search";
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("Notion-Version".to_string(), "2022-06-28".to_string());

        let response = self
            .http_client
            .post(url, &search_data.to_string(), Some(headers))
            .await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;

        let pages = result["results"].as_array().unwrap_or(&Vec::new()).clone();

        Ok(pages)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for NotionIntegration {
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
            name: "notion".to_string(),
            version: "1.0.0".to_string(),
            description: "Notion integration for documentation".to_string(),
            integration_type: IntegrationType::Notion,
            capabilities: vec![
                "page_management".to_string(),
                "database_management".to_string(),
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
