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

pub mod project_management;
pub mod documentation;
pub mod communication;
pub mod development;
pub mod analytics;

pub use project_management::*;
pub use documentation::*;
pub use communication::*;
pub use development::*;
pub use analytics::*;

use crate::error::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// Common trait for all external tool integrations
#[async_trait]
pub trait ExternalIntegration: Send + Sync + std::any::Any {
    /// Initialize the integration with configuration
    async fn initialize(&mut self, config: IntegrationConfig) -> RhemaResult<()>;
    
    /// Test the connection to the external service
    async fn test_connection(&self) -> RhemaResult<bool>;
    
    /// Get integration metadata
    fn get_metadata(&self) -> IntegrationMetadata;
    
    /// Get integration status
    async fn get_status(&self) -> RhemaResult<IntegrationStatus>;
    
    /// Get a reference to the Any trait for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Configuration for external integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub name: String,
    pub integration_type: IntegrationType,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub webhook_url: Option<String>,
    pub custom_headers: HashMap<String, String>,
    pub timeout_seconds: Option<u64>,
    pub retry_attempts: Option<u32>,
    pub enabled: bool,
}

/// Types of integrations supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, clap::ValueEnum)]
pub enum IntegrationType {
    // Project Management
    Jira,
    Asana,
    Trello,
    GitHubIssues,
    GitLabIssues,
    
    // Documentation
    Confluence,
    Notion,
    ReadTheDocs,
    Wiki,
    
    // Communication
    Slack,
    Discord,
    MicrosoftTeams,
    Email,
    
    // Development
    IDE,
    CodeReview,
    Testing,
    Build,
    Deployment,
    
    // Analytics
    Analytics,
    Monitoring,
    Logging,
    Performance,
    BusinessIntelligence,
}

impl std::fmt::Display for IntegrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationType::Jira => write!(f, "Jira"),
            IntegrationType::Asana => write!(f, "Asana"),
            IntegrationType::Trello => write!(f, "Trello"),
            IntegrationType::GitHubIssues => write!(f, "GitHub Issues"),
            IntegrationType::GitLabIssues => write!(f, "GitLab Issues"),
            IntegrationType::Confluence => write!(f, "Confluence"),
            IntegrationType::Notion => write!(f, "Notion"),
            IntegrationType::ReadTheDocs => write!(f, "ReadTheDocs"),
            IntegrationType::Wiki => write!(f, "Wiki"),
            IntegrationType::Slack => write!(f, "Slack"),
            IntegrationType::Discord => write!(f, "Discord"),
            IntegrationType::MicrosoftTeams => write!(f, "Microsoft Teams"),
            IntegrationType::Email => write!(f, "Email"),
            IntegrationType::IDE => write!(f, "IDE"),
            IntegrationType::CodeReview => write!(f, "Code Review"),
            IntegrationType::Testing => write!(f, "Testing"),
            IntegrationType::Build => write!(f, "Build"),
            IntegrationType::Deployment => write!(f, "Deployment"),
            IntegrationType::Analytics => write!(f, "Analytics"),
            IntegrationType::Monitoring => write!(f, "Monitoring"),
            IntegrationType::Logging => write!(f, "Logging"),
            IntegrationType::Performance => write!(f, "Performance"),
            IntegrationType::BusinessIntelligence => write!(f, "Business Intelligence"),
        }
    }
}

/// Metadata about an integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub integration_type: IntegrationType,
    pub capabilities: Vec<String>,
    pub required_config: Vec<String>,
    pub optional_config: Vec<String>,
}

/// Status of an integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatus {
    pub connected: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error_message: Option<String>,
    pub response_time_ms: Option<u64>,
    pub rate_limit_remaining: Option<u32>,
    pub rate_limit_reset: Option<chrono::DateTime<chrono::Utc>>,
}

/// Manager for all external integrations
pub struct IntegrationManager {
    pub integrations: HashMap<String, Box<dyn ExternalIntegration>>,
    pub configs: HashMap<String, IntegrationConfig>,
}

impl IntegrationManager {
    pub fn new() -> Self {
        Self {
            integrations: HashMap::new(),
            configs: HashMap::new(),
        }
    }
    
    /// Register an integration
    pub fn register_integration(
        &mut self,
        name: String,
        integration: Box<dyn ExternalIntegration>,
        config: IntegrationConfig,
    ) -> RhemaResult<()> {
        self.integrations.insert(name.clone(), integration);
        self.configs.insert(name, config);
        Ok(())
    }
    
    /// Get an integration by name
    pub fn get_integration(&self, name: &str) -> Option<&Box<dyn ExternalIntegration>> {
        self.integrations.get(name)
    }
    
    /// Get all integrations of a specific type
    pub fn get_integrations_by_type(&self, integration_type: IntegrationType) -> Vec<&Box<dyn ExternalIntegration>> {
        self.integrations
            .iter()
            .filter_map(|(name, integration)| {
                if self.configs.get(name)?.integration_type == integration_type {
                    Some(integration)
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Test all integrations
    pub async fn test_all_integrations(&self) -> HashMap<String, IntegrationStatus> {
        let mut results = HashMap::new();
        
        for (name, integration) in &self.integrations {
            let status = integration.get_status().await.unwrap_or_else(|_| IntegrationStatus {
                connected: false,
                last_check: chrono::Utc::now(),
                error_message: Some("Failed to get status".to_string()),
                response_time_ms: None,
                rate_limit_remaining: None,
                rate_limit_reset: None,
            });
            results.insert(name.clone(), status);
        }
        
        results
    }
    
    /// Initialize all integrations from configuration
    pub async fn initialize_from_config(&mut self, configs: Vec<IntegrationConfig>) -> RhemaResult<()> {
        for config in configs {
            if !config.enabled {
                continue;
            }
            
            let mut integration = self.create_integration(&config.integration_type)?;
            integration.initialize(config.clone()).await?;
            self.register_integration(config.name.clone(), integration, config)?;
        }
        Ok(())
    }
    
    /// Create an integration instance based on type
    fn create_integration(&self, integration_type: &IntegrationType) -> RhemaResult<Box<dyn ExternalIntegration>> {
        match integration_type {
            IntegrationType::Jira => Ok(Box::new(project_management::JiraIntegration::new())),
            IntegrationType::Asana => Ok(Box::new(project_management::AsanaIntegration::new())),
            IntegrationType::Trello => Ok(Box::new(project_management::TrelloIntegration::new())),
            IntegrationType::GitHubIssues => Ok(Box::new(project_management::GitHubIssuesIntegration::new())),
            IntegrationType::GitLabIssues => Ok(Box::new(project_management::GitLabIssuesIntegration::new())),
            IntegrationType::Confluence => Ok(Box::new(documentation::ConfluenceIntegration::new())),
            IntegrationType::Notion => Ok(Box::new(documentation::NotionIntegration::new())),
            IntegrationType::ReadTheDocs => Ok(Box::new(documentation::ReadTheDocsIntegration::new())),
            IntegrationType::Wiki => Ok(Box::new(documentation::WikiIntegration::new())),
            IntegrationType::Slack => Ok(Box::new(communication::SlackIntegration::new())),
            IntegrationType::Discord => Ok(Box::new(communication::DiscordIntegration::new())),
            IntegrationType::MicrosoftTeams => Ok(Box::new(communication::MicrosoftTeamsIntegration::new())),
            IntegrationType::Email => Ok(Box::new(communication::EmailIntegration::new())),
            IntegrationType::IDE => Ok(Box::new(development::IDEIntegration::new())),
            IntegrationType::CodeReview => Ok(Box::new(development::CodeReviewIntegration::new())),
            IntegrationType::Testing => Ok(Box::new(development::TestingIntegration::new())),
            IntegrationType::Build => Ok(Box::new(development::BuildIntegration::new())),
            IntegrationType::Deployment => Ok(Box::new(development::DeploymentIntegration::new())),
            IntegrationType::Analytics => Ok(Box::new(analytics::AnalyticsIntegration::new())),
            IntegrationType::Monitoring => Ok(Box::new(analytics::MonitoringIntegration::new())),
            IntegrationType::Logging => Ok(Box::new(analytics::LoggingIntegration::new())),
            IntegrationType::Performance => Ok(Box::new(analytics::PerformanceIntegration::new())),
            IntegrationType::BusinessIntelligence => Ok(Box::new(analytics::BusinessIntelligenceIntegration::new())),
        }
    }
}

/// Common HTTP client for integrations
pub struct IntegrationHttpClient {
    client: reqwest::Client,
}

impl IntegrationHttpClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client }
    }
    
    pub async fn get(&self, url: &str, headers: Option<HashMap<String, String>>) -> RhemaResult<String> {
        let mut request = self.client.get(url);
        
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        
        let response = request.send().await?;
        let body = response.text().await?;
        Ok(body)
    }
    
    pub async fn post(&self, url: &str, body: &str, headers: Option<HashMap<String, String>>) -> RhemaResult<String> {
        let mut request = self.client.post(url).body(body.to_string());
        
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        
        let response = request.send().await?;
        let response_body = response.text().await?;
        Ok(response_body)
    }
    
    pub async fn put(&self, url: &str, body: &str, headers: Option<HashMap<String, String>>) -> RhemaResult<String> {
        let mut request = self.client.put(url).body(body.to_string());
        
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        
        let response = request.send().await?;
        let response_body = response.text().await?;
        Ok(response_body)
    }
    
    pub async fn delete(&self, url: &str, headers: Option<HashMap<String, String>>) -> RhemaResult<String> {
        let mut request = self.client.delete(url);
        
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        
        let response = request.send().await?;
        let response_body = response.text().await?;
        Ok(response_body)
    }
    
    pub async fn patch(&self, url: &str, body: &str, headers: Option<HashMap<String, String>>) -> RhemaResult<String> {
        let mut request = self.client.patch(url).body(body.to_string());
        
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        
        let response = request.send().await?;
        let response_body = response.text().await?;
        Ok(response_body)
    }
} 