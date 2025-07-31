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
use base64::{engine::general_purpose, Engine as _};

/// Confluence integration for documentation
pub struct ConfluenceIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl ConfluenceIntegration {
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
    
    /// Create a Confluence page
    pub async fn create_page(&self, space_key: &str, title: &str, content: &str, parent_id: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Confluence not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let mut page_data = serde_json::json!({
            "type": "page",
            "title": title,
            "space": {
                "key": space_key
            },
            "body": {
                "storage": {
                    "value": content,
                    "representation": "storage"
                }
            }
        });
        
        if let Some(parent_id) = parent_id {
            page_data["ancestors"] = serde_json::json!([
                {
                    "id": parent_id
                }
            ]);
        }
        
        let url = format!("{}/rest/api/content", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.post(&url, &page_data.to_string(), Some(headers)).await?;
        let page: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(page["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get Confluence page content
    pub async fn get_page(&self, page_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Confluence not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/rest/api/content/{}?expand=body.storage", base_url, page_id);
        let mut headers = HashMap::new();
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let page: serde_json::Value = serde_json::from_str(&response)?;
        Ok(page)
    }
    
    /// Update Confluence page
    pub async fn update_page(&self, page_id: &str, title: &str, content: &str, version: u32) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Confluence not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let page_data = serde_json::json!({
            "version": {
                "number": version + 1
            },
            "title": title,
            "type": "page",
            "body": {
                "storage": {
                    "value": content,
                    "representation": "storage"
                }
            }
        });
        
        let url = format!("{}/rest/api/content/{}", base_url, page_id);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        self.http_client.put(&url, &page_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Search Confluence pages
    pub async fn search_pages(&self, query: &str, space_key: Option<&str>) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Confluence not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let mut url = format!("{}/rest/api/content/search?cql=text~\"{}\"", base_url, query);
        if let Some(space_key) = space_key {
            url.push_str(&format!(" AND space=\"{}\"", space_key));
        }
        
        let mut headers = HashMap::new();
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let pages = result["results"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(pages)
    }
}

#[async_trait]
impl ExternalIntegration for ConfluenceIntegration {
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
            name: "confluence".to_string(),
            version: "1.0.0".to_string(),
            description: "Confluence integration for documentation".to_string(),
            integration_type: IntegrationType::Confluence,
            capabilities: vec!["page_management".to_string(), "space_management".to_string()],
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
    pub async fn create_page(&self, parent_id: &str, title: &str, content: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
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
        
        let response = self.http_client.post(url, &page_data.to_string(), Some(headers)).await?;
        let page: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(page["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get Notion page content
    pub async fn get_page(&self, page_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://api.notion.com/v1/blocks/{}/children", page_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        headers.insert("Notion-Version".to_string(), "2022-06-28".to_string());
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let blocks = result["results"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(blocks)
    }
    
    /// Search Notion pages
    pub async fn search_pages(&self, query: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Notion not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
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
        
        let response = self.http_client.post(url, &search_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let pages = result["results"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(pages)
    }
}

#[async_trait]
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
            capabilities: vec!["page_management".to_string(), "database_management".to_string()],
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
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
    pub async fn get_project_builds(&self, project_slug: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
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
        
        let builds = result["results"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(builds)
    }
    
    /// Trigger a documentation build
    pub async fn trigger_build(&self, project_slug: &str, version: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
        let default_url = "https://readthedocs.org".to_string();
        let base_url = config.base_url.as_ref().unwrap_or(&default_url);
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let mut build_data = serde_json::json!({});
        if let Some(version) = version {
            build_data["version"] = serde_json::Value::String(version.to_string());
        }
        
        let url = format!("{}/api/v3/projects/{}/builds/", base_url, project_slug);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Token {}", token));
        
        let response = self.http_client.post(&url, &build_data.to_string(), Some(headers)).await?;
        let build: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(build["id"].as_u64().unwrap_or(0).to_string())
    }
    
    /// Search documentation
    pub async fn search_docs(&self, project_slug: &str, query: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("ReadTheDocs not configured".to_string()))?;
        let default_url = "https://readthedocs.org".to_string();
        let base_url = config.base_url.as_ref().unwrap_or(&default_url);
        
        let url = format!("{}/api/v3/projects/{}/search/?q={}", base_url, project_slug, query);
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Token {}", token));
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let results = result["results"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(results)
    }
}

#[async_trait]
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
            capabilities: vec!["project_management".to_string(), "build_management".to_string()],
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

/// Wiki integration for knowledge sharing
pub struct WikiIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl WikiIntegration {
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
    
    /// Create a wiki page
    pub async fn create_page(&self, title: &str, content: &str, namespace: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Wiki not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let mut page_data = serde_json::json!({
            "title": title,
            "content": content
        });
        
        if let Some(namespace) = namespace {
            page_data["namespace"] = serde_json::Value::String(namespace.to_string());
        }
        
        let url = format!("{}/api/pages", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.post(&url, &page_data.to_string(), Some(headers)).await?;
        let page: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(page["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get wiki page content
    pub async fn get_page(&self, page_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Wiki not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/api/pages/{}", base_url, page_id);
        let mut headers = HashMap::new();
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let page: serde_json::Value = serde_json::from_str(&response)?;
        Ok(page)
    }
    
    /// Update wiki page
    pub async fn update_page(&self, page_id: &str, title: &str, content: &str) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Wiki not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let page_data = serde_json::json!({
            "title": title,
            "content": content
        });
        
        let url = format!("{}/api/pages/{}", base_url, page_id);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        self.http_client.put(&url, &page_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Search wiki pages
    pub async fn search_pages(&self, query: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Wiki not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/api/search?q={}", base_url, query);
        let mut headers = HashMap::new();
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        } else if let Some(username) = &config.username {
            if let Some(password) = &config.password {
                let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));
                headers.insert("Authorization".to_string(), format!("Basic {}", auth));
            }
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let pages = result["results"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(pages)
    }
}

#[async_trait]
impl ExternalIntegration for WikiIntegration {
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
            name: "wiki".to_string(),
            version: "1.0.0".to_string(),
            description: "Wiki integration for documentation".to_string(),
            integration_type: IntegrationType::Wiki,
            capabilities: vec!["page_management".to_string(), "content_management".to_string()],
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