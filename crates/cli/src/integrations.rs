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

use crate::{RhemaError, RhemaResult};
use rhema_integrations::*;
use clap::Subcommand;
use colored::*;
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;

// Stub types for missing integrations
pub type SlackIntegration = IntegrationWrapper;
pub type JiraIntegration = IntegrationWrapper;
pub type LoggingConfig = IntegrationConfig;

#[derive(Subcommand)]
pub enum IntegrationSubcommands {
    /// List all available integrations
    List {
        /// Show only enabled integrations
        #[arg(long)]
        enabled: bool,
        
        /// Show only integrations of a specific type
        #[arg(long, value_enum)]
        integration_type: Option<IntegrationType>,
    },
    
    /// Configure an integration
    Config {
        /// Integration name
        #[arg(value_name = "NAME")]
        name: String,
        
        /// Integration type
        #[arg(long, value_enum)]
        integration_type: IntegrationType,
        
        /// Base URL for the integration
        #[arg(long, value_name = "URL")]
        base_url: Option<String>,
        
        /// API key for authentication
        #[arg(long, value_name = "KEY")]
        api_key: Option<String>,
        
        /// Username for authentication
        #[arg(long, value_name = "USERNAME")]
        username: Option<String>,
        
        /// Password for authentication
        #[arg(long, value_name = "PASSWORD")]
        password: Option<String>,
        
        /// Token for authentication
        #[arg(long, value_name = "TOKEN")]
        token: Option<String>,
        
        /// Webhook URL
        #[arg(long, value_name = "WEBHOOK")]
        webhook_url: Option<String>,
        
        /// Custom headers (key=value format)
        #[arg(long, value_name = "HEADERS")]
        headers: Vec<String>,
        
        /// Timeout in seconds
        #[arg(long, value_name = "SECONDS")]
        timeout: Option<u64>,
        
        /// Number of retry attempts
        #[arg(long, value_name = "ATTEMPTS")]
        retries: Option<u32>,
        
        /// Enable the integration
        #[arg(long)]
        enable: bool,
        
        /// Disable the integration
        #[arg(long)]
        disable: bool,
    },
    
    /// Test integration connectivity
    Test {
        /// Integration name to test
        #[arg(value_name = "NAME")]
        name: Option<String>,
        
        /// Test all integrations
        #[arg(long)]
        all: bool,
    },
    
    /// Show integration status
    Status {
        /// Integration name
        #[arg(value_name = "NAME")]
        name: Option<String>,
        
        /// Show detailed status
        #[arg(long)]
        detailed: bool,
    },
    
    /// Remove an integration
    Remove {
        /// Integration name to remove
        #[arg(value_name = "NAME")]
        name: String,
        
        /// Force removal without confirmation
        #[arg(long)]
        force: bool,
    },
    
    /// Execute integration action
    Execute {
        /// Integration name
        #[arg(value_name = "NAME")]
        name: String,
        
        /// Action to execute
        #[arg(value_name = "ACTION")]
        action: String,
        
        /// Action parameters (JSON format)
        #[arg(long, value_name = "PARAMS")]
        params: Option<String>,
    },
}

pub async fn handle_integrations(cmd: IntegrationSubcommands) -> RhemaResult<()> {
    match cmd {
        IntegrationSubcommands::List { enabled, integration_type } => {
            list_integrations(enabled, integration_type).await
        }
        IntegrationSubcommands::Config {
            name,
            integration_type,
            base_url,
            api_key,
            username,
            password,
            token,
            webhook_url,
            headers,
            timeout,
            retries,
            enable,
            disable,
        } => {
            config_integration(
                name,
                integration_type,
                base_url,
                api_key,
                username,
                password,
                token,
                webhook_url,
                headers,
                timeout,
                retries,
                enable,
                disable,
            ).await
        }
        IntegrationSubcommands::Test { name, all } => {
            test_integrations(name, all).await
        }
        IntegrationSubcommands::Status { name, detailed } => {
            show_integration_status(name, detailed).await
        }
        IntegrationSubcommands::Remove { name, force } => {
            remove_integration(name, force).await
        }
        IntegrationSubcommands::Execute { name, action, params } => {
            execute_integration_action(name, action, params).await
        }
    }
}

async fn list_integrations(enabled_only: bool, integration_type: Option<IntegrationType>) -> RhemaResult<()> {
    let mut manager = IntegrationManager::new();
    
    // Load existing configurations
    if let Ok(configs) = load_integration_configs().await {
        manager.initialize_from_config(configs).await?;
    }
    
    println!("{}", "Available Integrations".bold().blue());
    println!("{}", "===================".blue());
    
    let integrations = if let Some(integration_type) = integration_type {
        manager.get_integrations_by_type(integration_type)
    } else {
        manager.integrations.values().collect::<Vec<_>>()
    };
    
    if integrations.is_empty() {
        println!("{}", "No integrations found.".yellow());
        return Ok(());
    }
    
    for integration in integrations {
        let metadata = integration.get_metadata();
        let status = integration.get_status().await.unwrap_or_else(|_| IntegrationStatus {
            connected: false,
            last_check: chrono::Utc::now(),
            error_message: Some("Failed to get status".to_string()),
            response_time_ms: None,
            rate_limit_remaining: None,
            rate_limit_reset: None,
        });
        
        let status_icon = if status.connected { "✅" } else { "❌" };
        let status_text = if status.connected { "Connected" } else { "Disconnected" };
        
        if !enabled_only || status.connected {
            println!("{} {} ({}) - {}", 
                status_icon, 
                metadata.name.bold(), 
                metadata.integration_type.to_string().cyan(),
                status_text
            );
            println!("  Description: {}", metadata.description);
            println!("  Version: {}", metadata.version);
            println!("  Capabilities: {}", metadata.capabilities.join(", "));
            println!();
        }
    }
    
    Ok(())
}

async fn config_integration(
    name: String,
    integration_type: IntegrationType,
    base_url: Option<String>,
    api_key: Option<String>,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
    webhook_url: Option<String>,
    headers: Vec<String>,
    timeout: Option<u64>,
    retries: Option<u32>,
    enable: bool,
    disable: bool,
) -> RhemaResult<()> {
    let mut custom_headers = HashMap::new();
    
    // Parse custom headers
    for header in headers {
        if let Some((key, value)) = header.split_once('=') {
            custom_headers.insert(key.to_string(), value.to_string());
        }
    }
    
    let mut config = IntegrationConfig {
        name: name.clone(),
        integration_type,
        base_url,
        api_key,
        username,
        password,
        token,
        webhook_url,
        custom_headers,
        timeout_seconds: timeout,
        retry_attempts: retries,
        enabled: !disable,
    };
    
    if enable {
        config.enabled = true;
    }
    
    // Save configuration
    save_integration_config(&config).await?;
    
    println!("{}", "Integration configured successfully!".green().bold());
    println!("Name: {}", name.bold());
    println!("Type: {}", config.integration_type.to_string().cyan());
    println!("Enabled: {}", if config.enabled { "Yes".green() } else { "No".red() });
    
    if let Some(base_url) = &config.base_url {
        println!("Base URL: {}", base_url);
    }
    
    if config.api_key.is_some() || config.token.is_some() || config.username.is_some() {
        println!("Authentication: {}", "Configured".green());
    }
    
    Ok(())
}

async fn test_integrations(name: Option<String>, all: bool) -> RhemaResult<()> {
    let mut manager = IntegrationManager::new();
    
    // Load existing configurations
    if let Ok(configs) = load_integration_configs().await {
        manager.initialize_from_config(configs).await?;
    }
    
    if all {
        println!("{}", "Testing all integrations...".blue().bold());
        let results = manager.test_all_integrations().await;
        
        for (name, status) in results {
            let status_icon = if status.connected { "✅" } else { "❌" };
            let status_text = if status.connected { "Connected" } else { "Failed" };
            
            println!("{} {} - {}", status_icon, name.bold(), status_text);
            
            if let Some(error) = status.error_message {
                println!("  Error: {}", error.red());
            }
        }
    } else if let Some(name) = name {
        if let Some(integration) = manager.get_integration(&name) {
            println!("{}", format!("Testing integration: {}", name).blue().bold());
            
            match integration.test_connection().await {
                Ok(connected) => {
                    let status_icon = if connected { "✅" } else { "❌" };
                    let status_text = if connected { "Connected" } else { "Failed" };
                    println!("{} {}", status_icon, status_text);
                }
                Err(e) => {
                    println!("{} Error: {}", "❌".red(), e.to_string().red());
                }
            }
        } else {
            return Err(RhemaError::ConfigError(format!("Integration '{}' not found", name)));
        }
    } else {
        return Err(RhemaError::ConfigError("Please specify an integration name or use --all".to_string()));
    }
    
    Ok(())
}

async fn show_integration_status(name: Option<String>, detailed: bool) -> RhemaResult<()> {
    let mut manager = IntegrationManager::new();
    
    // Load existing configurations
    if let Ok(configs) = load_integration_configs().await {
        manager.initialize_from_config(configs).await?;
    }
    
    if let Some(name) = name {
        if let Some(integration) = manager.get_integration(&name) {
            let status = integration.get_status().await?;
            let metadata = integration.get_metadata();
            
            println!("{}", format!("Status for: {}", name).blue().bold());
            println!("{}", "=".repeat(30).blue());
            println!("Connected: {}", if status.connected { "Yes".green() } else { "No".red() });
            println!("Last Check: {}", status.last_check.format("%Y-%m-%d %H:%M:%S UTC"));
            
            if let Some(error) = status.error_message {
                println!("Error: {}", error.red());
            }
            
            if detailed {
                println!("Type: {}", metadata.integration_type.to_string().cyan());
                println!("Version: {}", metadata.version);
                println!("Description: {}", metadata.description);
                println!("Capabilities: {}", metadata.capabilities.join(", "));
                
                if let Some(response_time) = status.response_time_ms {
                    println!("Response Time: {}ms", response_time);
                }
                
                if let Some(rate_limit) = status.rate_limit_remaining {
                    println!("Rate Limit Remaining: {}", rate_limit);
                }
            }
        } else {
            return Err(RhemaError::ConfigError(format!("Integration '{}' not found", name)));
        }
    } else {
        println!("{}", "Integration Status Overview".blue().bold());
        println!("{}", "=========================".blue());
        
        let results = manager.test_all_integrations().await;
        
        for (name, status) in results {
            let status_icon = if status.connected { "✅" } else { "❌" };
            println!("{} {} - {}", status_icon, name.bold(), 
                if status.connected { "Connected".green() } else { "Disconnected".red() });
        }
    }
    
    Ok(())
}

async fn remove_integration(name: String, force: bool) -> RhemaResult<()> {
    if !force {
        println!("Are you sure you want to remove integration '{}'? (y/N)", name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("{}", "Operation cancelled.".yellow());
            return Ok(());
        }
    }
    
    // Remove configuration file
    let config_path = get_integration_config_path(&name).await?;
    if config_path.exists() {
        std::fs::remove_file(config_path)?;
        println!("{}", format!("Integration '{}' removed successfully.", name).green().bold());
    } else {
        println!("{}", format!("Integration '{}' not found.", name).yellow());
    }
    
    Ok(())
}

async fn execute_integration_action(name: String, action: String, params: Option<String>) -> RhemaResult<()> {
    let mut manager = IntegrationManager::new();
    
    // Load existing configurations
    if let Ok(configs) = load_integration_configs().await {
        manager.initialize_from_config(configs).await?;
    }
    
    let integration = manager.get_integration(&name)
        .ok_or_else(|| RhemaError::ConfigError(format!("Integration '{}' not found", name)))?;
    
    let metadata = integration.get_metadata();
    
    if !metadata.capabilities.contains(&action) {
        return Err(RhemaError::ConfigError(
            format!("Action '{}' not supported by integration '{}'. Available actions: {}", 
                action, name, metadata.capabilities.join(", "))
        ));
    }
    
    println!("{}", format!("Executing '{}' on integration '{}'", action, name).blue().bold());
    
    // Parse parameters
    let parameters: HashMap<String, serde_json::Value> = if let Some(params) = params {
        serde_json::from_str(&params)?
    } else {
        HashMap::new()
    };
    
    // Execute the action based on integration type
    match metadata.integration_type {
        IntegrationType::Slack => {
            if let Some(slack_integration) = integration.as_ref().downcast_ref::<SlackIntegration>() {
                match action.as_str() {
                    "send_message" => {
                        let channel = parameters.get("channel").and_then(|v| v.as_str()).unwrap_or("#general");
                        let text = parameters.get("text").and_then(|v| v.as_str()).unwrap_or("Hello from Rhema!");
                        
                        let result = slack_integration.send_message(channel, text, None).await?;
                        println!("Message sent successfully! Timestamp: {}", result);
                    }
                    _ => return Err(RhemaError::ConfigError(format!("Action '{}' not implemented", action))),
                }
            }
        }
        IntegrationType::Jira => {
            if let Some(jira_integration) = integration.as_ref().downcast_ref::<JiraIntegration>() {
                match action.as_str() {
                    "create_issue" => {
                        let project_key = parameters.get("project_key").and_then(|v| v.as_str()).unwrap_or("PROJ");
                        let summary = parameters.get("summary").and_then(|v| v.as_str()).unwrap_or("New Issue");
                        let description = parameters.get("description").and_then(|v| v.as_str()).unwrap_or("Issue description");
                        let issue_type = parameters.get("issue_type").and_then(|v| v.as_str()).unwrap_or("Task");
                        
                        let result = jira_integration.create_issue(project_key, summary, description, issue_type).await?;
                        println!("Issue created successfully! Key: {}", result);
                    }
                    _ => return Err(RhemaError::ConfigError(format!("Action '{}' not implemented", action))),
                }
            }
        }
        _ => return Err(RhemaError::ConfigError(format!("Integration type '{}' not yet supported for execution", metadata.integration_type.to_string()))),
    }
    
    println!("{}", "Action executed successfully!".green().bold());
    Ok(())
}

// Helper functions for configuration management

async fn get_integration_config_path(name: &str) -> RhemaResult<PathBuf> {
    let config_dir = std::env::var("Rhema_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut home = std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."));
            home.push(".rhema");
            home.push("integrations");
            home
        });
    
    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join(format!("{}.json", name)))
}

async fn save_integration_config(config: &IntegrationConfig) -> RhemaResult<()> {
    let config_path = get_integration_config_path(&config.name).await?;
    let config_json = serde_json::to_string_pretty(config)?;
    std::fs::write(config_path, config_json)?;
    Ok(())
}

async fn load_integration_configs() -> RhemaResult<Vec<IntegrationConfig>> {
    let config_dir = std::env::var("Rhema_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut home = std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."));
            home.push(".rhema");
            home.push("integrations");
            home
        });
    
    if !config_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut configs = Vec::new();
    
    for entry in std::fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str::<IntegrationConfig>(&content) {
                    configs.push(config);
                }
            }
        }
    }
    
    Ok(configs)
} 