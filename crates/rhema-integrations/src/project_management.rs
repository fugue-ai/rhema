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

#[async_trait]
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

#[async_trait]
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

/// Trello integration for board management
pub struct TrelloIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl TrelloIntegration {
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
    
    /// Create a Trello card
    pub async fn create_card(&self, list_id: &str, name: &str, description: Option<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let mut params = format!("idList={}&key={}&token={}&name={}", list_id, api_key, token, name);
        
        if let Some(description) = description {
            params.push_str(&format!("&desc={}", description));
        }
        
        let url = format!("https://api.trello.com/1/cards?{}", params);
        
        let response = self.http_client.post(&url, "", None).await?;
        let card: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(card["id"].as_str().unwrap_or("").to_string())
    }
    
    /// Get Trello card details
    pub async fn get_card(&self, card_id: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://api.trello.com/1/cards/{}?key={}&token={}", card_id, api_key, token);
        
        let response = self.http_client.get(&url, None).await?;
        let card: serde_json::Value = serde_json::from_str(&response)?;
        Ok(card)
    }
    
    /// Update Trello card
    pub async fn update_card(&self, card_id: &str, fields: serde_json::Value) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://api.trello.com/1/cards/{}?key={}&token={}", card_id, api_key, token);
        
        self.http_client.put(&url, &fields.to_string(), None).await?;
        Ok(())
    }
    
    /// Get cards from a board
    pub async fn get_board_cards(&self, board_id: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Trello not configured".to_string()))?;
        let api_key = config.api_key.as_ref().ok_or_else(|| RhemaError::ConfigError("API key not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("https://api.trello.com/1/boards/{}/cards?key={}&token={}", board_id, api_key, token);
        
        let response = self.http_client.get(&url, None).await?;
        let cards: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(cards)
    }
}

#[async_trait]
impl ExternalIntegration for TrelloIntegration {
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
            name: "trello".to_string(),
            version: "1.0.0".to_string(),
            description: "Trello integration for project management".to_string(),
            integration_type: IntegrationType::Trello,
            capabilities: vec!["board_management".to_string(), "card_tracking".to_string()],
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

#[async_trait]
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
    pub async fn create_issue(&self, project_id: &str, title: &str, description: Option<&str>, labels: Option<Vec<&str>>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
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
        
        let response = self.http_client.post(&url, &issue_data.to_string(), Some(headers)).await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(issue["iid"].as_u64().unwrap_or(0).to_string())
    }
    
    /// Get GitLab issue details
    pub async fn get_issue(&self, project_id: &str, issue_iid: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("{}/api/v4/projects/{}/issues/{}", base_url, project_id, issue_iid);
        let mut headers = HashMap::new();
        headers.insert("PRIVATE-TOKEN".to_string(), token.clone());
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let issue: serde_json::Value = serde_json::from_str(&response)?;
        Ok(issue)
    }
    
    /// Update GitLab issue
    pub async fn update_issue(&self, project_id: &str, issue_iid: &str, fields: serde_json::Value) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
        let url = format!("{}/api/v4/projects/{}/issues/{}", base_url, project_id, issue_iid);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("PRIVATE-TOKEN".to_string(), token.clone());
        
        self.http_client.put(&url, &fields.to_string(), Some(headers)).await?;
        Ok(())
    }
    
    /// Get issues from a project
    pub async fn get_project_issues(&self, project_id: &str, state: Option<&str>) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("GitLab not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        let token = config.token.as_ref().ok_or_else(|| RhemaError::ConfigError("Token not configured".to_string()))?;
        
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

#[async_trait]
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