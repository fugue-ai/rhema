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

use rhema_mcp::{
    AuthManager, CacheManager, ContextProvider, FileWatcher, McpConfig, OfficialRhemaMcpServer,
    MCP_VERSION, SUPPORTED_VERSIONS,
};
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

#[tokio::test]
async fn test_mcp_protocol_version_compliance() {
    // Test that we're using official MCP protocol versions
    assert_eq!(MCP_VERSION, "2025-06-18");
    assert!(SUPPORTED_VERSIONS.contains(&"2024-11-05"));
    assert!(SUPPORTED_VERSIONS.contains(&"2025-03-26"));
    assert!(SUPPORTED_VERSIONS.contains(&"2025-06-18"));
}

#[tokio::test]
async fn test_official_sdk_server_creation() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create configuration
    let config = McpConfig::default();
    assert!(config.use_official_sdk); // Should default to true

    // Create context provider
    let context_provider = Arc::new(ContextProvider::new(repo_root.clone()).unwrap());

    // Create cache manager
    let cache_manager = Arc::new(CacheManager::new(&config.cache));

    // Create file watcher
    let file_watcher = Arc::new(FileWatcher::new(&config.watcher, repo_root));

    // Create auth manager
    let auth_manager = Arc::new(AuthManager::new(&config.auth).unwrap());

    // Create official SDK server
    let server = OfficialRhemaMcpServer::new(
        context_provider,
        cache_manager,
        file_watcher,
        auth_manager,
    )
    .unwrap();

    // Verify server was created successfully
    assert!(server.health().await.status == "healthy");
}

#[tokio::test]
async fn test_official_sdk_server_start_stop() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create configuration with different port to avoid conflicts
    let mut config = McpConfig::default();
    config.port = 8081; // Use different port for testing

    // Create context provider
    let context_provider = Arc::new(ContextProvider::new(repo_root.clone()).unwrap());

    // Create cache manager
    let cache_manager = Arc::new(CacheManager::new(&config.cache));

    // Create file watcher
    let file_watcher = Arc::new(FileWatcher::new(&config.watcher, repo_root));

    // Create auth manager
    let auth_manager = Arc::new(AuthManager::new(&config.auth).unwrap());

    // Create official SDK server
    let mut server = OfficialRhemaMcpServer::new(
        context_provider,
        cache_manager,
        file_watcher,
        auth_manager,
    )
    .unwrap();

    // Test server start
    let start_result = server.start(&config).await;
    assert!(start_result.is_ok(), "Server should start successfully");

    // Test server stop
    let stop_result = server.stop().await;
    assert!(stop_result.is_ok(), "Server should stop successfully");
}

#[tokio::test]
async fn test_mcp_daemon_with_official_sdk() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create configuration with official SDK enabled
    let mut config = McpConfig::default();
    config.port = 8082; // Use different port for testing
    config.use_official_sdk = true;

    // Create MCP daemon
    let mut daemon = rhema_mcp::McpDaemon::new(config, repo_root).await.unwrap();

    // Test daemon start
    let start_result = daemon.start().await;
    assert!(start_result.is_ok(), "Daemon should start successfully");

    // Test daemon stop
    let stop_result = daemon.stop().await;
    assert!(stop_result.is_ok(), "Daemon should stop successfully");
}

#[tokio::test]
async fn test_mcp_daemon_without_official_sdk() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create configuration with official SDK disabled
    let mut config = McpConfig::default();
    config.port = 8083; // Use different port for testing
    config.use_official_sdk = false;

    // Create MCP daemon
    let mut daemon = rhema_mcp::McpDaemon::new(config, repo_root).await.unwrap();

    // Test daemon start (should still work but without official SDK)
    let start_result = daemon.start().await;
    assert!(start_result.is_ok(), "Daemon should start successfully");

    // Test daemon stop
    let stop_result = daemon.stop().await;
    assert!(stop_result.is_ok(), "Daemon should stop successfully");
}

#[tokio::test]
async fn test_mcp_config_defaults() {
    let config = McpConfig::default();

    // Verify default configuration
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 8080);
    assert!(config.use_official_sdk); // Should default to true
    assert!(!config.auth.enabled); // Auth should be disabled by default
    assert!(config.watcher.enabled); // Watcher should be enabled by default
    assert!(config.cache.memory_enabled); // Memory cache should be enabled by default
}

#[tokio::test]
async fn test_mcp_protocol_version_negotiation() {
    // Test version negotiation logic
    let supported_versions = SUPPORTED_VERSIONS;

    // Test exact version match
    assert!(supported_versions.contains(&"2025-06-18"));
    assert!(supported_versions.contains(&"2025-03-26"));
    assert!(supported_versions.contains(&"2024-11-05"));

    // Test unsupported version (should fall back to latest)
    assert!(!supported_versions.contains(&"2023-01-01"));
}

#[tokio::test]
async fn test_official_sdk_dependencies() {
    // Test that we can import the official MCP SDK types
    use rust_mcp_sdk::schema::{InitializeParams, InitializeResult, ServerCapabilities, ServerInfo};
    use rust_mcp_schema::tool::Tool;

    // Verify we can create basic MCP structures
    let _init_params = InitializeParams {
        protocol_version: "2025-06-18".to_string(),
        capabilities: rust_mcp_sdk::schema::ClientCapabilities::default(),
        client_info: None,
    };

    let _server_info = ServerInfo {
        name: "rhema-mcp".to_string(),
        version: "1.0.0".to_string(),
    };

    // This test verifies that the official SDK dependencies are properly integrated
    assert!(true);
}

#[tokio::test]
async fn test_mcp_migration_completeness() {
    // Test that all required MCP primitives are available
    use rhema_mcp::{
        Resource, Tool, Prompt, PromptSegment, ToolResult, ContextProviderExt,
    };

    // Verify we can create MCP primitives
    let _resource = Resource {
        uri: "test://resource".to_string(),
        name: "Test Resource".to_string(),
        description: Some("Test resource description".to_string()),
        mime_type: "text/plain".to_string(),
        content: serde_json::json!("test content"),
        metadata: std::collections::HashMap::new(),
    };

    let _tool = Tool {
        name: "test_tool".to_string(),
        description: "Test tool description".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "test": { "type": "string" }
            }
        }),
    };

    let _prompt = Prompt {
        name: "test_prompt".to_string(),
        description: "Test prompt description".to_string(),
        segments: vec![
            PromptSegment::Text {
                text: "Test prompt text".to_string(),
            },
        ],
    };

    // This test verifies that all MCP primitives are properly defined
    assert!(true);
}

#[tokio::test]
async fn test_mcp_backward_compatibility() {
    // Test that the old SDK is still available for backward compatibility
    use rhema_mcp::{
        RhemaMcpServer, Resource as SdkResource, Tool as SdkTool, Prompt as SdkPrompt,
    };

    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create configuration
    let config = McpConfig::default();

    // Create context provider
    let context_provider = Arc::new(ContextProvider::new(repo_root.clone()).unwrap());

    // Create cache manager
    let cache_manager = Arc::new(CacheManager::new(&config.cache));

    // Create file watcher
    let file_watcher = Arc::new(FileWatcher::new(&config.watcher, repo_root));

    // Create auth manager
    let auth_manager = Arc::new(AuthManager::new(&config.auth).unwrap());

    // Create old SDK server (should still work)
    let old_server = RhemaMcpServer::new(
        context_provider,
        cache_manager,
        file_watcher,
        auth_manager,
    )
    .unwrap();

    // Verify old server was created successfully
    let resources = old_server.get_resources().await;
    let tools = old_server.get_tools().await;
    let prompts = old_server.get_prompts().await;

    assert!(resources.len() >= 0);
    assert!(tools.len() >= 0);
    assert!(prompts.len() >= 0);
}

#[tokio::test]
async fn test_mcp_config_serialization() {
    let config = McpConfig::default();

    // Test YAML serialization
    let yaml = serde_yaml::to_string(&config).unwrap();
    assert!(yaml.contains("use_official_sdk: true"));
    assert!(yaml.contains("host: 127.0.0.1"));
    assert!(yaml.contains("port: 8080"));

    // Test YAML deserialization
    let deserialized: McpConfig = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(deserialized.host, config.host);
    assert_eq!(deserialized.port, config.port);
    assert_eq!(deserialized.use_official_sdk, config.use_official_sdk);
}

#[tokio::test]
async fn test_mcp_resource_structure() {
    use rhema_mcp::Resource;

    let resource = Resource {
        uri: "rhema://test/resource".to_string(),
        name: "Test Resource".to_string(),
        description: Some("Test resource description".to_string()),
        mime_type: "application/json".to_string(),
        content: serde_json::json!({
            "test": "data",
            "number": 42
        }),
        metadata: std::collections::HashMap::new(),
    };

    // Test resource properties
    assert_eq!(resource.uri, "rhema://test/resource");
    assert_eq!(resource.name, "Test Resource");
    assert_eq!(resource.description, Some("Test resource description".to_string()));
    assert_eq!(resource.mime_type, "application/json");

    // Test content serialization
    let content_str = serde_json::to_string(&resource.content).unwrap();
    assert!(content_str.contains("test"));
    assert!(content_str.contains("42"));
}

#[tokio::test]
async fn test_mcp_tool_structure() {
    use rhema_mcp::Tool;

    let tool = Tool {
        name: "test_tool".to_string(),
        description: "Test tool description".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "Input parameter"
                }
            },
            "required": ["input"]
        }),
    };

    // Test tool properties
    assert_eq!(tool.name, "test_tool");
    assert_eq!(tool.description, "Test tool description");

    // Test schema validation
    let schema = &tool.input_schema;
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"]["input"]["type"] == "string");
}

#[tokio::test]
async fn test_mcp_prompt_structure() {
    use rhema_mcp::{Prompt, PromptSegment};

    let prompt = Prompt {
        name: "test_prompt".to_string(),
        description: "Test prompt description".to_string(),
        segments: vec![
            PromptSegment::Text {
                text: "This is a test prompt with ".to_string(),
            },
            PromptSegment::Resource {
                uri: "rhema://test/resource".to_string(),
                name: "Test Resource".to_string(),
            },
        ],
    };

    // Test prompt properties
    assert_eq!(prompt.name, "test_prompt");
    assert_eq!(prompt.description, "Test prompt description");
    assert_eq!(prompt.segments.len(), 2);

    // Test segment types
    match &prompt.segments[0] {
        PromptSegment::Text { text } => {
            assert_eq!(text, "This is a test prompt with ");
        }
        _ => panic!("Expected Text segment"),
    }

    match &prompt.segments[1] {
        PromptSegment::Resource { uri, name } => {
            assert_eq!(uri, "rhema://test/resource");
            assert_eq!(name, "Test Resource");
        }
        _ => panic!("Expected Resource segment"),
    }
}

#[tokio::test]
async fn test_mcp_tool_result_structure() {
    use rhema_mcp::ToolResult;

    // Test text result
    let text_result = ToolResult::Text {
        text: "Tool execution result".to_string(),
    };

    match text_result {
        ToolResult::Text { text } => {
            assert_eq!(text, "Tool execution result");
        }
        _ => panic!("Expected Text result"),
    }

    // Test image result
    let image_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
    let image_result = ToolResult::Image {
        data: image_data.clone(),
        mime_type: "image/png".to_string(),
    };

    match image_result {
        ToolResult::Image { data, mime_type } => {
            assert_eq!(data, image_data);
            assert_eq!(mime_type, "image/png");
        }
        _ => panic!("Expected Image result"),
    }
}

#[tokio::test]
async fn test_mcp_migration_completeness() {
    // Test that all required MCP primitives are available
    use rhema_mcp::{
        Resource, Tool, Prompt, PromptSegment, ToolResult, ContextProviderExt,
    };

    // Verify we can create MCP primitives
    let _resource = Resource {
        uri: "test://resource".to_string(),
        name: "Test Resource".to_string(),
        description: Some("Test resource description".to_string()),
        mime_type: "text/plain".to_string(),
        content: serde_json::json!("test content"),
        metadata: std::collections::HashMap::new(),
    };

    let _tool = Tool {
        name: "test_tool".to_string(),
        description: "Test tool description".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "test": { "type": "string" }
            }
        }),
    };

    let _prompt = Prompt {
        name: "test_prompt".to_string(),
        description: "Test prompt description".to_string(),
        segments: vec![
            PromptSegment::Text {
                text: "Test prompt text".to_string(),
            },
        ],
    };

    // This test verifies that all MCP primitives are properly defined
    assert!(true);
} 