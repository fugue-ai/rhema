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
use base64::{engine::general_purpose, Engine as _};

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

#[async_trait::async_trait]
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
