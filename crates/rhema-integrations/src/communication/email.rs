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

/// Email integration for notifications and reporting
pub struct EmailIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl EmailIntegration {
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
    
    /// Send an email using a service like SendGrid or Mailgun
    pub async fn send_email(&self, to: &str, subject: &str, body: &str, from: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Email not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let email_data = serde_json::json!({
            "personalizations": [
                {
                    "to": [
                        {
                            "email": to
                        }
                    ]
                }
            ],
            "from": {
                "email": from.unwrap_or("noreply@rhema.dev")
            },
            "subject": subject,
            "content": [
                {
                    "type": "text/plain",
                    "value": body
                }
            ]
        });
        
        let url = format!("{}/v3/mail/send", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &email_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(result["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Send an HTML email
    pub async fn send_html_email(&self, to: &str, subject: &str, html_body: &str, text_body: Option<&str>, from: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Email not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let mut content = vec![
            serde_json::json!({
                "type": "text/html",
                "value": html_body
            })
        ];
        
        if let Some(text_body) = text_body {
            content.push(serde_json::json!({
                "type": "text/plain",
                "value": text_body
            }));
        }
        
        let email_data = serde_json::json!({
            "personalizations": [
                {
                    "to": [
                        {
                            "email": to
                        }
                    ]
                }
            ],
            "from": {
                "email": from.unwrap_or("noreply@rhema.dev")
            },
            "subject": subject,
            "content": content
        });
        
        let url = format!("{}/v3/mail/send", base_url);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.post(&url, &email_data.to_string(), Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(result["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get email delivery status
    pub async fn get_delivery_status(&self, message_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Email not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/v3/messages/{}", base_url, message_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let status: serde_json::Value = serde_json::from_str(&response)?;
        Ok(status)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for EmailIntegration {
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
            name: "email".to_string(),
            version: "1.0.0".to_string(),
            description: "Email integration for communication".to_string(),
            integration_type: IntegrationType::Email,
            capabilities: vec!["email_sending".to_string(), "email_receiving".to_string()],
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
