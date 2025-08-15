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

/// Asana integration for task management
pub struct AsanaIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl AsanaIntegration {
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
    
    /// Create an Asana task
    pub async fn create_task(&self, workspace_id: &str, project_id: &str, name: &str, notes: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Asana not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let mut task_data = serde_json::json!({
            "data": {
                "workspace": workspace_id,
                "projects": [project_id],
                "name": name,
            }
        });
        
        if let Some(notes) = notes {
            task_data["data"]["notes"] = serde_json::Value::String(notes.to_string());
        }
        
        let url = "https://app.asana.com/api/1.0/tasks";
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        let response = self.http_client.post(url, &task_data.to_string(), Some(headers)).await?;
        let task: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(task["data"]["gid"].as_str().unwrap_or("").to_string())
    }
    
    /// Get Asana task details
    pub async fn get_task(&self, task_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Asana not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://app.asana.com/api/1.0/tasks/{}", task_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let task: serde_json::Value = serde_json::from_str(&response)?;
        Ok(task)
    }
    
    /// Update Asana task
    pub async fn update_task(&self, task_id: &str, fields: serde_json::Value) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Asana not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let update_data = serde_json::json!({
            "data": fields
        });
        
        let url = format!("https://app.asana.com/api/1.0/tasks/{}", task_id);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        self.http_client.put(&url, &update_data.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Get tasks from a project
    pub async fn get_project_tasks(&self, project_id: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Asana not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://app.asana.com/api/1.0/projects/{}/tasks", project_id);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let result: serde_json::Value = serde_json::from_str(&response)?;
        
        let tasks = result["data"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();
        
        Ok(tasks)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for AsanaIntegration {
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
            name: "asana".to_string(),
            version: "1.0.0".to_string(),
            description: "Asana integration for project management".to_string(),
            integration_type: IntegrationType::Asana,
            capabilities: vec!["task_management".to_string(), "project_tracking".to_string()],
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
