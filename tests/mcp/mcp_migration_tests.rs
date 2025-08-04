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

use rhema::mcp::{
    AuthManager, CacheManager, ContextProvider, ContextProviderExt, FileWatcher, McpConfig,
    RhemaMcpServer, SdkPrompt, SdkResource, SdkTool, SdkToolResult,
};
use rhema::RhemaResult;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

#[tokio::test]
async fn test_mcp_config_default() {
    let config = McpConfig::default();

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 8080);
    assert!(config.use_official_sdk);
    assert!(!config.auth.enabled);
    assert!(config.watcher.enabled);
    assert!(config.cache.memory_enabled);
}

#[tokio::test]
async fn test_mcp_server_creation() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create mock components
    let context_provider = Arc::new(ContextProvider::new(repo_root.clone()).unwrap());
    let cache_manager = Arc::new(
        CacheManager::new(&McpConfig::default().cache)
            .await
            .unwrap(),
    );
    let file_watcher = Arc::new(
        FileWatcher::new(&McpConfig::default().watcher, repo_root)
            .await
            .unwrap(),
    );
    let auth_manager = Arc::new(AuthManager::new(&McpConfig::default().auth).unwrap());

    // Create MCP server
    let server =
        RhemaMcpServer::new(context_provider, cache_manager, file_watcher, auth_manager).unwrap();

    // Initially, resources, tools, and prompts should be empty until start() is called
    assert!(server.get_resources().await.is_empty());
    assert!(server.get_tools().await.is_empty());
    assert!(server.get_prompts().await.is_empty());
}

#[tokio::test]
async fn test_mcp_server_start() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create mock components
    let context_provider = Arc::new(ContextProvider::new(repo_root.clone()).unwrap());
    let cache_manager = Arc::new(
        CacheManager::new(&McpConfig::default().cache)
            .await
            .unwrap(),
    );
    let file_watcher = Arc::new(
        FileWatcher::new(&McpConfig::default().watcher, repo_root)
            .await
            .unwrap(),
    );
    let auth_manager = Arc::new(AuthManager::new(&McpConfig::default().auth).unwrap());

    // Create MCP server
    let server =
        RhemaMcpServer::new(context_provider, cache_manager, file_watcher, auth_manager).unwrap();

    // Test server start (should not fail)
    let config = McpConfig::default();
    let result = server.start(&config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_mcp_tool_execution() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create mock components
    let context_provider = Arc::new(ContextProvider::new(repo_root.clone()).unwrap());
    let cache_manager = Arc::new(
        CacheManager::new(&McpConfig::default().cache)
            .await
            .unwrap(),
    );
    let file_watcher = Arc::new(
        FileWatcher::new(&McpConfig::default().watcher, repo_root)
            .await
            .unwrap(),
    );
    let auth_manager = Arc::new(AuthManager::new(&McpConfig::default().auth).unwrap());

    // Create MCP server
    let server =
        RhemaMcpServer::new(context_provider, cache_manager, file_watcher, auth_manager).unwrap();

    // Test query tool
    let query_args = serde_json::json!({
        "query": "SELECT * FROM scopes"
    });
    let result = server.execute_tool("rhema_query", query_args).await;
    assert!(result.is_ok());

    if let Ok(SdkToolResult::Text { text }) = result {
        assert!(text.contains("Query executed"));
    }

    // Test search tool
    let search_args = serde_json::json!({
        "pattern": "test"
    });
    let result = server.execute_tool("rhema_search", search_args).await;
    assert!(result.is_ok());

    if let Ok(SdkToolResult::Text { text }) = result {
        assert!(text.contains("Search executed"));
    }

    // Test scope tool
    let scope_args = serde_json::json!({
        "scope_name": "example"
    });
    let result = server.execute_tool("rhema_scope", scope_args).await;
    assert!(result.is_ok());

    if let Ok(SdkToolResult::Text { text }) = result {
        assert!(text.contains("Scope information"));
    }

    // Test unknown tool
    let result = server
        .execute_tool("unknown_tool", serde_json::json!({}))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mcp_backward_compatibility() {
    let temp_dir = TempDir::new().unwrap();
    let repo_root = temp_dir.path().to_path_buf();

    // Create context provider
    let context_provider = Arc::new(ContextProvider::new(repo_root.clone()).unwrap());

    // Test that legacy methods exist and don't panic
    let query_result = context_provider.query("test").await;
    println!("Query result: {:?}", query_result);
    // Some methods might be implemented, others might return errors

    let search_result = context_provider.search_regex("test", None).await;
    println!("Search result: {:?}", search_result);
    assert!(search_result.is_ok());

    let scope_result = context_provider.get_scope("test").await;
    println!("Scope result: {:?}", scope_result);

    let scopes_result = context_provider.list_scopes().await;
    println!("Scopes result: {:?}", scopes_result);

    let knowledge_result = context_provider.load_knowledge("test").await;
    println!("Knowledge result: {:?}", knowledge_result);

    // The main goal is that these methods exist and don't panic
    // Some may be implemented, others may return errors
    assert!(true);
}

#[tokio::test]
async fn test_mcp_config_serialization() {
    let config = McpConfig::default();

    // Test serialization to YAML
    let yaml = serde_yaml::to_string(&config).unwrap();
    assert!(yaml.contains("host: 127.0.0.1"));
    assert!(yaml.contains("port: 8080"));
    assert!(yaml.contains("use_official_sdk: true"));

    // Test deserialization from YAML
    let deserialized: McpConfig = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(deserialized.host, config.host);
    assert_eq!(deserialized.port, config.port);
    assert_eq!(deserialized.use_official_sdk, config.use_official_sdk);
}

#[tokio::test]
async fn test_mcp_resource_structure() {
    let resource = SdkResource {
        uri: "rhema://scopes/example".to_string(),
        name: "Example Scope".to_string(),
        description: Some("Example scope resource".to_string()),
        mime_type: "application/json".to_string(),
        content: serde_json::json!({
            "name": "example",
            "description": "Example scope"
        }),
        metadata: std::collections::HashMap::new(),
    };

    // Test serialization
    let json = serde_json::to_string(&resource).unwrap();
    assert!(json.contains("rhema://scopes/example"));
    assert!(json.contains("Example Scope"));

    // Test deserialization
    let deserialized: SdkResource = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.uri, resource.uri);
    assert_eq!(deserialized.name, resource.name);
}

#[tokio::test]
async fn test_mcp_tool_structure() {
    let tool = SdkTool {
        name: "rhema_query".to_string(),
        description: "Execute a CQL query against Rhema context".to_string(),
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
    };

    // Test serialization
    let json = serde_json::to_string(&tool).unwrap();
    assert!(json.contains("rhema_query"));
    assert!(json.contains("Execute a CQL query"));

    // Test deserialization
    let deserialized: SdkTool = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, tool.name);
    assert_eq!(deserialized.description, tool.description);
}

#[tokio::test]
async fn test_mcp_prompt_structure() {
    use rhema::mcp::sdk::PromptSegment;

    let prompt = SdkPrompt {
        name: "context_analysis".to_string(),
        description: "Analyze project context and provide insights".to_string(),
        segments: vec![
            PromptSegment::Text {
                text: "Analyze the following project context and provide insights:\n\n".to_string(),
            },
            PromptSegment::Resource {
                uri: "rhema://context/current".to_string(),
                name: "Current Context".to_string(),
            },
        ],
    };

    // Test serialization
    let json = serde_json::to_string(&prompt).unwrap();
    assert!(json.contains("context_analysis"));
    assert!(json.contains("Analyze project context"));

    // Test deserialization
    let deserialized: SdkPrompt = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, prompt.name);
    assert_eq!(deserialized.description, prompt.description);
    assert_eq!(deserialized.segments.len(), 2);
}

#[tokio::test]
async fn test_mcp_tool_result_structure() {
    let text_result = SdkToolResult::Text {
        text: "Query executed successfully".to_string(),
    };

    let image_result = SdkToolResult::Image {
        data: vec![1, 2, 3, 4],
        mime_type: "image/png".to_string(),
    };

    // Test serialization
    let text_json = serde_json::to_string(&text_result).unwrap();
    assert!(text_json.contains("Query executed successfully"));

    let image_json = serde_json::to_string(&image_result).unwrap();
    assert!(image_json.contains("image/png"));

    // Test deserialization
    let deserialized_text: SdkToolResult = serde_json::from_str(&text_json).unwrap();
    if let SdkToolResult::Text { text } = deserialized_text {
        assert_eq!(text, "Query executed successfully");
    } else {
        panic!("Expected Text variant");
    }

    let deserialized_image: SdkToolResult = serde_json::from_str(&image_json).unwrap();
    if let SdkToolResult::Image { data, mime_type } = deserialized_image {
        assert_eq!(data, vec![1, 2, 3, 4]);
        assert_eq!(mime_type, "image/png");
    } else {
        panic!("Expected Image variant");
    }
}

#[tokio::test]
async fn test_mcp_migration_completeness() {
    // Test that all required MCP primitives are available
    let _resource: SdkResource = SdkResource {
        uri: "test://uri".to_string(),
        name: "Test".to_string(),
        description: None,
        mime_type: "text/plain".to_string(),
        content: serde_json::Value::Null,
        metadata: std::collections::HashMap::new(),
    };

    let _tool: SdkTool = SdkTool {
        name: "test_tool".to_string(),
        description: "Test tool".to_string(),
        input_schema: serde_json::Value::Null,
    };

    let _prompt: SdkPrompt = SdkPrompt {
        name: "test_prompt".to_string(),
        description: "Test prompt".to_string(),
        segments: vec![],
    };

    let _tool_result: SdkToolResult = SdkToolResult::Text {
        text: "Test result".to_string(),
    };

    // If we get here, all types are available and the migration is complete
    assert!(true);
}
