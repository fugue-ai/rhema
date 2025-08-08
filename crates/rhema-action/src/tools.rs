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

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use rhema_action_tool::{ActionIntent, ActionResult, ActionError, ToolResult, TransformationTool, ValidationTool, SafetyTool};

// Import tool implementations from dedicated crates
use rhema_action_jscodeshift::JscodeshiftTool;
use rhema_action_comby::CombyTool;
use rhema_action_ast_grep::AstGrepTool;
use rhema_action_prettier::PrettierTool;
use rhema_action_eslint::ESLintTool;

use rhema_action_typescript::TypeScriptTool;
use rhema_action_jest::JestTool;
use rhema_action_mocha::MochaTool;
use rhema_action_pytest::PyTestTool;
use rhema_action_cargo::CargoTool;

use rhema_action_syntax_validation::SyntaxValidationTool;
use rhema_action_type_checking::TypeCheckingTool;
use rhema_action_test_coverage::TestCoverageTool;
use rhema_action_security_scanning::SecurityScanningTool;

/// Tool registry for managing all available tools
pub struct ToolRegistry {
    transformation_tools: Arc<RwLock<HashMap<String, Box<dyn TransformationTool>>>>,
    validation_tools: Arc<RwLock<HashMap<String, Box<dyn ValidationTool>>>>,
    safety_tools: Arc<RwLock<HashMap<String, Box<dyn SafetyTool>>>>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub async fn new() -> ActionResult<Self> {
        info!("Initializing Tool Registry");
        
        let registry = Self {
            transformation_tools: Arc::new(RwLock::new(HashMap::new())),
            validation_tools: Arc::new(RwLock::new(HashMap::new())),
            safety_tools: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Register built-in tools
        registry.register_builtin_tools().await?;
        
        info!("Tool Registry initialized successfully");
        Ok(registry)
    }

    /// Initialize the tool registry (stub)
    pub async fn initialize() -> ActionResult<()> {
        info!("ToolRegistry initialized (stub)");
        Ok(())
    }

    /// Shutdown the tool registry (stub)
    pub async fn shutdown() -> ActionResult<()> {
        info!("ToolRegistry shutdown (stub)");
        Ok(())
    }
    
    /// Register built-in tools
    async fn register_builtin_tools(&self) -> ActionResult<()> {
        info!("Registering built-in tools");
        
        // Register transformation tools
        self.register_transformation_tool("jscodeshift", Box::new(JscodeshiftTool)).await;
        self.register_transformation_tool("comby", Box::new(CombyTool)).await;
        self.register_transformation_tool("ast-grep", Box::new(AstGrepTool)).await;
        self.register_transformation_tool("prettier", Box::new(PrettierTool)).await;
        self.register_transformation_tool("eslint", Box::new(ESLintTool)).await;
        
        // Register validation tools
        self.register_validation_tool("typescript", Box::new(TypeScriptTool)).await;
        self.register_validation_tool("jest", Box::new(JestTool)).await;
        self.register_validation_tool("mocha", Box::new(MochaTool)).await;
        self.register_validation_tool("pytest", Box::new(PyTestTool)).await;
        self.register_validation_tool("cargo", Box::new(CargoTool)).await;
        
        // Register safety tools
        self.register_safety_tool("syntax_validation", Box::new(SyntaxValidationTool)).await;
        self.register_safety_tool("type_checking", Box::new(TypeCheckingTool)).await;
        self.register_safety_tool("test_coverage", Box::new(TestCoverageTool)).await;
        self.register_safety_tool("security_scanning", Box::new(SecurityScanningTool)).await;
        
        info!("Built-in tools registered successfully");
        Ok(())
    }
    
    /// Register a transformation tool
    pub async fn register_transformation_tool(&self, name: &str, tool: Box<dyn TransformationTool>) {
        let mut tools = self.transformation_tools.write().await;
        tools.insert(name.to_string(), tool);
        info!("Registered transformation tool: {}", name);
    }
    
    /// Register a validation tool
    pub async fn register_validation_tool(&self, name: &str, tool: Box<dyn ValidationTool>) {
        let mut tools = self.validation_tools.write().await;
        tools.insert(name.to_string(), tool);
        info!("Registered validation tool: {}", name);
    }
    
    /// Register a safety tool
    pub async fn register_safety_tool(&self, name: &str, tool: Box<dyn SafetyTool>) {
        let mut tools = self.safety_tools.write().await;
        tools.insert(name.to_string(), tool);
        info!("Registered safety tool: {}", name);
    }
    
    /// Execute a transformation tool
    pub async fn execute_tool(&self, tool_name: &str, intent: &ActionIntent) -> ActionResult<ToolResult> {
        let tools = self.transformation_tools.read().await;
        
        if let Some(tool) = tools.get(tool_name) {
            info!("Executing transformation tool: {}", tool_name);
            
            // Check if tool is available
            if !tool.is_available().await {
                return Err(ActionError::ToolExecution { 
                    tool: tool_name.to_string(), 
                    message: "Tool is not available".to_string() 
                });
            }
            
            // Execute the tool
            let result = tool.execute(intent).await?;
            
            if result.success {
                info!("Transformation tool {} executed successfully", tool_name);
            } else {
                warn!("Transformation tool {} completed with errors", tool_name);
            }
            
            Ok(result)
        } else {
            Err(ActionError::ToolExecution { 
                tool: tool_name.to_string(), 
                message: "Tool not found".to_string() 
            })
        }
    }
    
    /// Execute a validation tool
    pub async fn execute_validation(&self, tool_name: &str, intent: &ActionIntent) -> ActionResult<ToolResult> {
        let tools = self.validation_tools.read().await;
        
        if let Some(tool) = tools.get(tool_name) {
            info!("Executing validation tool: {}", tool_name);
            
            // Check if tool is available
            if !tool.is_available().await {
                return Err(ActionError::ToolExecution { 
                    tool: tool_name.to_string(), 
                    message: "Tool is not available".to_string() 
                });
            }
            
            // Execute the tool
            let result = tool.validate(intent).await?;
            
            if result.success {
                info!("Validation tool {} executed successfully", tool_name);
            } else {
                warn!("Validation tool {} completed with errors", tool_name);
            }
            
            Ok(result)
        } else {
            Err(ActionError::ToolExecution { 
                tool: tool_name.to_string(), 
                message: "Tool not found".to_string() 
            })
        }
    }
    
    /// Execute a safety tool
    pub async fn execute_safety_check(&self, tool_name: &str, intent: &ActionIntent) -> ActionResult<ToolResult> {
        let tools = self.safety_tools.read().await;
        
        if let Some(tool) = tools.get(tool_name) {
            info!("Executing safety tool: {}", tool_name);
            
            // Check if tool is available
            if !tool.is_available().await {
                return Err(ActionError::ToolExecution { 
                    tool: tool_name.to_string(), 
                    message: "Tool is not available".to_string() 
                });
            }
            
            // Execute the tool
            let result = tool.check(intent).await?;
            
            if result.success {
                info!("Safety tool {} executed successfully", tool_name);
            } else {
                warn!("Safety tool {} completed with errors", tool_name);
            }
            
            Ok(result)
        } else {
            Err(ActionError::ToolExecution { 
                tool: tool_name.to_string(), 
                message: "Tool not found".to_string() 
            })
        }
    }
    
    /// List all available transformation tools
    pub async fn list_transformation_tools(&self) -> Vec<String> {
        let tools = self.transformation_tools.read().await;
        tools.keys().cloned().collect()
    }
    
    /// List all available validation tools
    pub async fn list_validation_tools(&self) -> Vec<String> {
        let tools = self.validation_tools.read().await;
        tools.keys().cloned().collect()
    }
    
    /// List all available safety tools
    pub async fn list_safety_tools(&self) -> Vec<String> {
        let tools = self.safety_tools.read().await;
        tools.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rhema_action_tool::{ActionType, SafetyLevel};

    #[tokio::test]
    async fn test_tool_registry_creation() {
        let registry = ToolRegistry::new().await;
        assert!(registry.is_ok());
    }

    #[tokio::test]
    async fn test_tool_registry_listing() {
        let registry = ToolRegistry::new().await.unwrap();
        
        let transformation_tools = registry.list_transformation_tools().await;
        assert!(!transformation_tools.is_empty());
        
        let validation_tools = registry.list_validation_tools().await;
        assert!(!validation_tools.is_empty());
        
        let safety_tools = registry.list_safety_tools().await;
        assert!(!safety_tools.is_empty());
    }

    #[tokio::test]
    async fn test_jscodeshift_tool() {
        let tool = JscodeshiftTool;
        
        assert_eq!(tool.name(), "jscodeshift");
        assert_eq!(tool.version(), "1.0.0");
        assert_eq!(tool.safety_level(), SafetyLevel::Medium);
        assert!(tool.supports_language("javascript"));
        assert!(tool.supports_language("typescript"));
        assert!(!tool.supports_language("python"));
        
        let is_available = tool.is_available().await;
        assert!(is_available);
    }

    #[tokio::test]
    async fn test_tool_execution() {
        let registry = ToolRegistry::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-tool",
            ActionType::Refactor,
            "Test tool execution",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        let result = registry.execute_tool("jscodeshift", &intent).await;
        assert!(result.is_ok());
        
        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert!(!tool_result.changes.is_empty());
    }
} 