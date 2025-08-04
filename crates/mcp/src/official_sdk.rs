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

use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use super::{AuthManager, CacheManager, ContextProvider, FileWatcher};
use crate::mcp::McpConfig;

/// Official MCP Protocol versions supported by Rhema
pub const MCP_VERSION: &str = "2025-06-18";
pub const SUPPORTED_VERSIONS: &[&str] = &["2024-11-05", "2025-03-26", "2025-06-18"];

/// MCP Tool Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolResult {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { data: Vec<u8>, mime_type: String },
}

/// MCP Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub content: Value,
    pub size: Option<u64>,
    pub title: Option<String>,
}

/// MCP Tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Value,
    pub output_schema: Option<Value>,
    pub title: Option<String>,
}

/// MCP Prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Value,
    pub title: Option<String>,
}

/// Rhema MCP Server using official protocol
#[derive(Clone)]
pub struct OfficialRhemaMcpServer {
    context_provider: Arc<ContextProvider>,
    cache_manager: Arc<CacheManager>,
    file_watcher: Arc<FileWatcher>,
    auth_manager: Arc<AuthManager>,
    resources: Arc<RwLock<HashMap<String, Resource>>>,
    tools: Arc<RwLock<HashMap<String, Tool>>>,
    prompts: Arc<RwLock<HashMap<String, Prompt>>>,
    start_time: std::time::Instant,
}

impl OfficialRhemaMcpServer {
    /// Create a new Rhema MCP server with official protocol
    pub async fn new(
        context_provider: Arc<ContextProvider>,
        cache_manager: Arc<CacheManager>,
        file_watcher: Arc<FileWatcher>,
        auth_manager: Arc<AuthManager>,
        _config: &McpConfig,
    ) -> RhemaResult<Self> {
        let resources = Arc::new(RwLock::new(HashMap::new()));
        let tools = Arc::new(RwLock::new(HashMap::new()));
        let prompts = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            context_provider,
            cache_manager,
            file_watcher,
            auth_manager,
            resources,
            tools,
            prompts,
            start_time: std::time::Instant::now(),
        })
    }

    /// Start the MCP server
    pub async fn start(&mut self, _config: &McpConfig) -> RhemaResult<()> {
        info!("Starting Rhema MCP server with official protocol");

        // Initialize resources, tools, and prompts
        self.initialize_resources().await?;
        self.initialize_tools().await?;
        self.initialize_prompts().await?;

        info!("Rhema MCP server started successfully");
        Ok(())
    }

    /// Stop the MCP server
    pub async fn stop(&mut self) -> RhemaResult<()> {
        info!("Rhema MCP server stopped");
        Ok(())
    }

    /// Handle tool calls
    pub async fn handle_tool_call(
        &self,
        name: String,
        arguments: Value,
    ) -> RhemaResult<ToolResult> {
        info!("Executing tool: {}", name);

        match name.as_str() {
            "rhema_query" => {
                let query = arguments["query"]
                    .as_str()
                    .ok_or_else(|| rhema_core::RhemaError::InvalidInput("Missing query parameter".to_string()))?;
                
                // Execute the actual query
                let result = self.context_provider.execute_query(query).await?;
                
                Ok(ToolResult::Text { 
                    text: serde_json::to_string(&result)?
                })
            }
            "rhema_search" => {
                let pattern = arguments["pattern"]
                    .as_str()
                    .ok_or_else(|| rhema_core::RhemaError::InvalidInput("Missing pattern parameter".to_string()))?;
                
                let file_filter = arguments["file_filter"].as_str();
                
                // Execute the actual search
                let results = self.context_provider.search_regex(pattern, file_filter).await?;
                
                Ok(ToolResult::Text { 
                    text: serde_json::to_string(&results)?
                })
            }
            "rhema_scope" => {
                let scope_name = arguments["name"]
                    .as_str()
                    .ok_or_else(|| rhema_core::RhemaError::InvalidInput("Missing name parameter".to_string()))?;
                
                // Get scope information
                let scope = self.context_provider.get_scope(scope_name).await?;
                
                Ok(ToolResult::Text { 
                    text: serde_json::to_string(&scope)?
                })
            }
            "rhema_scopes" => {
                // Get all scopes
                let scopes = self.context_provider.get_scopes().await?;
                
                Ok(ToolResult::Text { 
                    text: serde_json::to_string(&scopes)?
                })
            }
            "rhema_knowledge" => {
                let scope_name = arguments["scope"]
                    .as_str()
                    .ok_or_else(|| rhema_core::RhemaError::InvalidInput("Missing scope parameter".to_string()))?;
                
                // Load knowledge for scope
                let knowledge = self.context_provider.get_knowledge_for_mcp(scope_name).await?;
                
                Ok(ToolResult::Text { 
                    text: serde_json::to_string(&knowledge)?
                })
            }
            _ => {
                warn!("Unknown tool: {}", name);
                Err(rhema_core::RhemaError::InvalidInput(format!("Unknown tool: {}", name)))
            }
        }
    }

    /// Get all tools
    pub async fn get_tools(&self) -> Vec<Tool> {
        let tools_guard = self.tools.read().await;
        tools_guard.values().cloned().collect()
    }

    /// Get all resources
    pub async fn get_resources(&self) -> Vec<Resource> {
        let resources_guard = self.resources.read().await;
        resources_guard.values().cloned().collect()
    }

    /// Get all prompts
    pub async fn get_prompts(&self) -> Vec<Prompt> {
        let prompts_guard = self.prompts.read().await;
        prompts_guard.values().cloned().collect()
    }

    /// Initialize resources
    async fn initialize_resources(&self) -> RhemaResult<()> {
        let mut resources_guard = self.resources.write().await;
        
        // Add default Rhema resources
        resources_guard.insert(
            "rhema://context/schema".to_string(),
            Resource {
                uri: "rhema://context/schema".to_string(),
                name: "Rhema Context Schema".to_string(),
                description: Some("Schema definition for Rhema context files".to_string()),
                mime_type: Some("application/json".to_string()),
                content: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "version": { "type": "string" },
                        "scopes": { "type": "array" }
                    }
                }),
                size: None,
                title: None,
            },
        );

        resources_guard.insert(
            "rhema://docs/architecture".to_string(),
            Resource {
                uri: "rhema://docs/architecture".to_string(),
                name: "Rhema Architecture".to_string(),
                description: Some("Rhema system architecture documentation".to_string()),
                mime_type: Some("text/markdown".to_string()),
                content: serde_json::json!("# Rhema Architecture\n\nRhema is a Git-native toolkit for capturing and organizing project knowledge."),
                size: None,
                title: None,
            },
        );

        info!("Initialized {} resources", resources_guard.len());
        Ok(())
    }

    /// Initialize tools
    async fn initialize_tools(&self) -> RhemaResult<()> {
        let mut tools_guard = self.tools.write().await;
        
        // Add Rhema query tool
        tools_guard.insert(
            "rhema_query".to_string(),
            Tool {
                name: "rhema_query".to_string(),
                description: Some("Execute CQL queries against Rhema context".to_string()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "CQL query to execute"
                        }
                    },
                    "required": ["query"]
                }),
                output_schema: None,
                title: None,
            },
        );

        // Add Rhema search tool
        tools_guard.insert(
            "rhema_search".to_string(),
            Tool {
                name: "rhema_search".to_string(),
                description: Some("Search Rhema context using regex patterns".to_string()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Regex pattern to search for"
                        },
                        "file_filter": {
                            "type": "string",
                            "description": "Optional file filter pattern"
                        }
                    },
                    "required": ["pattern"]
                }),
                output_schema: None,
                title: None,
            },
        );

        // Add Rhema scope tool
        tools_guard.insert(
            "rhema_scope".to_string(),
            Tool {
                name: "rhema_scope".to_string(),
                description: Some("Get information about a specific Rhema scope".to_string()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the scope to retrieve"
                        }
                    },
                    "required": ["name"]
                }),
                output_schema: None,
                title: None,
            },
        );

        // Add Rhema scopes tool
        tools_guard.insert(
            "rhema_scopes".to_string(),
            Tool {
                name: "rhema_scopes".to_string(),
                description: Some("List all available Rhema scopes".to_string()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {}
                }),
                output_schema: None,
                title: None,
            },
        );

        // Add Rhema knowledge tool
        tools_guard.insert(
            "rhema_knowledge".to_string(),
            Tool {
                name: "rhema_knowledge".to_string(),
                description: Some("Load knowledge for a specific Rhema scope".to_string()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "scope": {
                            "type": "string",
                            "description": "Name of the scope to load knowledge for"
                        }
                    },
                    "required": ["scope"]
                }),
                output_schema: None,
                title: None,
            },
        );

        info!("Initialized {} tools", tools_guard.len());
        Ok(())
    }

    /// Initialize prompts
    async fn initialize_prompts(&self) -> RhemaResult<()> {
        let mut prompts_guard = self.prompts.write().await;
        
        // Add default Rhema prompts
        prompts_guard.insert(
            "rhema_context_analysis".to_string(),
            Prompt {
                name: "rhema_context_analysis".to_string(),
                description: Some("Analyze Rhema context and provide insights".to_string()),
                arguments: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "context_uri": {
                            "type": "string",
                            "description": "URI of the context to analyze"
                        }
                    },
                    "required": ["context_uri"]
                }),
                title: None,
            },
        );

        prompts_guard.insert(
            "rhema_scope_overview".to_string(),
            Prompt {
                name: "rhema_scope_overview".to_string(),
                description: Some("Provide an overview of Rhema scopes".to_string()),
                arguments: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "scope_name": {
                            "type": "string",
                            "description": "Name of the scope to overview"
                        }
                    },
                    "required": ["scope_name"]
                }),
                title: None,
            },
        );

        info!("Initialized {} prompts", prompts_guard.len());
        Ok(())
    }

    /// Get server health status
    pub async fn health(&self) -> crate::mcp::HealthStatus {
        // Get actual cache statistics
        let cache_stats = self.cache_manager.get_statistics().await;
        
        crate::mcp::HealthStatus {
            status: "healthy".to_string(),
            uptime: self.get_uptime().await.as_secs(),
            connections: self.get_connection_count().await,
            cache_hit_rate: cache_stats.hit_rate,
            memory_usage: self.get_memory_usage().await,
            request_count: 0, // TODO: Track request count
            error_count: 0,   // TODO: Track error count
            error_rate: 0.0,
            restart_count: 0,
        }
    }

    /// Get server uptime
    async fn get_uptime(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Get connection count
    async fn get_connection_count(&self) -> usize {
        // TODO: Get actual connection count
        1 // Placeholder
    }

    /// Get memory usage
    async fn get_memory_usage(&self) -> crate::mcp::MemoryUsage {
        // TODO: Get actual system memory usage
        crate::mcp::MemoryUsage {
            used: 0,
            total: 0,
            cache_size: 0,
            used_mb: 0,
        }
    }
} 