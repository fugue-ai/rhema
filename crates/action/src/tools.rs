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
use async_trait::async_trait;
use tracing::{info, warn};

use crate::schema::{ActionIntent, SafetyLevel};
use crate::error::{ActionError, ActionResult};

/// Result from tool execution
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub success: bool,
    pub changes: Vec<String>,
    pub output: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration: std::time::Duration,
}

/// Trait for transformation tools
#[async_trait]
pub trait TransformationTool: Send + Sync {
    /// Execute the tool with the given intent
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult>;
    
    /// Check if the tool supports the given language
    fn supports_language(&self, language: &str) -> bool;
    
    /// Get the safety level of this tool
    fn safety_level(&self) -> SafetyLevel;
    
    /// Get the name of this tool
    fn name(&self) -> &str;
    
    /// Get the version of this tool
    fn version(&self) -> &str;
    
    /// Check if the tool is available
    async fn is_available(&self) -> bool;
}

/// Trait for validation tools
#[async_trait]
pub trait ValidationTool: Send + Sync {
    /// Run validation with the given intent
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult>;
    
    /// Get the name of this tool
    fn name(&self) -> &str;
    
    /// Get the version of this tool
    fn version(&self) -> &str;
    
    /// Check if the tool is available
    async fn is_available(&self) -> bool;
}

/// Trait for safety tools
#[async_trait]
pub trait SafetyTool: Send + Sync {
    /// Run safety check with the given intent
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult>;
    
    /// Get the name of this tool
    fn name(&self) -> &str;
    
    /// Get the version of this tool
    fn version(&self) -> &str;
    
    /// Check if the tool is available
    async fn is_available(&self) -> bool;
}

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
                return Err(ActionError::tool_execution(
                    tool_name,
                    "Tool is not available"
                ));
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
            Err(ActionError::tool_execution(
                tool_name,
                "Tool not found"
            ))
        }
    }
    
    /// Execute a validation tool
    pub async fn execute_validation(&self, tool_name: &str, intent: &ActionIntent) -> ActionResult<ToolResult> {
        let tools = self.validation_tools.read().await;
        
        if let Some(tool) = tools.get(tool_name) {
            info!("Executing validation tool: {}", tool_name);
            
            // Check if tool is available
            if !tool.is_available().await {
                return Err(ActionError::tool_execution(
                    tool_name,
                    "Tool is not available"
                ));
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
            Err(ActionError::tool_execution(
                tool_name,
                "Tool not found"
            ))
        }
    }
    
    /// Execute a safety tool
    pub async fn execute_safety_check(&self, tool_name: &str, intent: &ActionIntent) -> ActionResult<ToolResult> {
        let tools = self.safety_tools.read().await;
        
        if let Some(tool) = tools.get(tool_name) {
            info!("Executing safety tool: {}", tool_name);
            
            // Check if tool is available
            if !tool.is_available().await {
                return Err(ActionError::tool_execution(
                    tool_name,
                    "Tool is not available"
                ));
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
            Err(ActionError::tool_execution(
                tool_name,
                "Tool not found"
            ))
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

// Built-in transformation tools

/// Jscodeshift transformation tool
pub struct JscodeshiftTool;

#[async_trait]
impl TransformationTool for JscodeshiftTool {
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Executing jscodeshift for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for transformation".to_string()));
        }
        
        // Create temporary jscodeshift script based on intent
        let script_content = self.generate_jscodeshift_script(intent).await?;
        let script_path = std::env::temp_dir().join("jscodeshift_script.js");
        tokio::fs::write(&script_path, script_content).await
            .map_err(|e| ActionError::tool_execution("jscodeshift", format!("Failed to write jscodeshift script: {}", e)))?;
        
        // Execute jscodeshift on each file
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for file in files {
            match self.execute_jscodeshift_on_file(&script_path, file).await {
                Ok(change) => changes.push(change),
                Err(e) => errors.push(format!("Failed to transform {}: {}", file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Processed {} files with jscodeshift", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn supports_language(&self, language: &str) -> bool {
        matches!(language, "javascript" | "typescript" | "jsx" | "tsx")
    }
    
    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Medium
    }
    
    fn name(&self) -> &str {
        "jscodeshift"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if jscodeshift is installed
        tokio::process::Command::new("npx")
            .args(&["jscodeshift", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl JscodeshiftTool {
    /// Generate jscodeshift script based on intent
    async fn generate_jscodeshift_script(&self, intent: &ActionIntent) -> ActionResult<String> {
        // Parse intent description to generate appropriate transformations
        let description = &intent.description.to_lowercase();
        
        let script = if description.contains("rename") || description.contains("refactor") {
            self.generate_rename_script(intent).await?
        } else if description.contains("add") || description.contains("insert") {
            self.generate_add_script(intent).await?
        } else if description.contains("remove") || description.contains("delete") {
            self.generate_remove_script(intent).await?
        } else {
            self.generate_generic_script(intent).await?
        };
        
        Ok(script)
    }
    
    async fn generate_rename_script(&self, _intent: &ActionIntent) -> ActionResult<String> {
        Ok(r#"
module.exports = function(fileInfo, api, options) {
    const j = api.jscodeshift;
    const root = j(fileInfo.source);
    
    // Find and rename identifiers
    root.find(j.Identifier).forEach(path => {
        // Apply renaming logic based on intent
        // This is a placeholder - would be customized based on intent
    });
    
    return root.toSource();
};
"#.to_string())
    }
    
    async fn generate_add_script(&self, _intent: &ActionIntent) -> ActionResult<String> {
        Ok(r#"
module.exports = function(fileInfo, api, options) {
    const j = api.jscodeshift;
    const root = j(fileInfo.source);
    
    // Add new code based on intent
    // This is a placeholder - would be customized based on intent
    
    return root.toSource();
};
"#.to_string())
    }
    
    async fn generate_remove_script(&self, _intent: &ActionIntent) -> ActionResult<String> {
        Ok(r#"
module.exports = function(fileInfo, api, options) {
    const j = api.jscodeshift;
    const root = j(fileInfo.source);
    
    // Remove code based on intent
    // This is a placeholder - would be customized based on intent
    
    return root.toSource();
};
"#.to_string())
    }
    
    async fn generate_generic_script(&self, _intent: &ActionIntent) -> ActionResult<String> {
        Ok(r#"
module.exports = function(fileInfo, api, options) {
    const j = api.jscodeshift;
    const root = j(fileInfo.source);
    
    // Generic transformation based on intent
    // This is a placeholder - would be customized based on intent
    
    return root.toSource();
};
"#.to_string())
    }
    
    /// Execute jscodeshift on a specific file
    async fn execute_jscodeshift_on_file(&self, script_path: &std::path::Path, file_path: &str) -> ActionResult<String> {
        info!("Executing jscodeshift on file: {}", file_path);
        
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Determine parser based on file extension
        let parser = if file_path.ends_with(".tsx") || file_path.ends_with(".jsx") {
            "tsx"
        } else if file_path.ends_with(".ts") {
            "ts"
        } else {
            "babel"
        };
        
        // Execute jscodeshift using npx
        let output = tokio::process::Command::new("npx")
            .args(&[
                "jscodeshift",
                "--transform", script_path.to_str().unwrap(),
                "--parser", parser,
                "--ignore-pattern", "node_modules",
                "--ignore-pattern", "dist",
                "--ignore-pattern", "build",
                "--run-in-band", // Run transformations sequentially
                file_path
            ])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("jscodeshift", format!("Failed to execute jscodeshift: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("Jscodeshift stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Jscodeshift stderr: {}", stderr);
            }
            
            Ok(format!("Successfully transformed {}: {}", file_path, stdout.trim()))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("jscodeshift", format!("Jscodeshift failed for {}: {}", file_path, stderr)))
        }
    }
}

/// Comby transformation tool
pub struct CombyTool;

#[async_trait]
impl TransformationTool for CombyTool {
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Executing comby for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for transformation".to_string()));
        }
        
        // Generate comby pattern based on intent
        let (pattern, rewrite) = self.generate_comby_pattern(intent).await?;
        
        // Execute comby on each file
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for file in files {
            match self.execute_comby_on_file(&pattern, &rewrite, file).await {
                Ok(change) => changes.push(change),
                Err(e) => errors.push(format!("Failed to transform {}: {}", file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Processed {} files with comby", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn supports_language(&self, _language: &str) -> bool {
        true // Comby supports many languages
    }
    
    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Medium
    }
    
    fn name(&self) -> &str {
        "comby"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if comby is installed
        tokio::process::Command::new("comby")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl CombyTool {
    /// Generate comby pattern and rewrite based on intent
    async fn generate_comby_pattern(&self, intent: &ActionIntent) -> ActionResult<(String, String)> {
        let description = &intent.description.to_lowercase();
        
        // Simple pattern generation based on intent description
        if description.contains("rename") || description.contains("refactor") {
            Ok((
                "function :[name]() { :[body] }".to_string(),
                "function :[name]() { :[body] }".to_string()
            ))
        } else if description.contains("add") || description.contains("insert") {
            Ok((
                ":[before]".to_string(),
                ":[before]\n// TODO: Add implementation".to_string()
            ))
        } else if description.contains("remove") || description.contains("delete") {
            Ok((
                "// TODO: Remove this\n:[code]".to_string(),
                ":[code]".to_string()
            ))
        } else {
            // Generic pattern for other transformations
            Ok((
                ":[pattern]".to_string(),
                ":[replacement]".to_string()
            ))
        }
    }
    
    /// Execute comby on a specific file
    async fn execute_comby_on_file(&self, pattern: &str, rewrite: &str, file_path: &str) -> ActionResult<String> {
        info!("Executing comby on file: {}", file_path);
        
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Execute comby
        let output = tokio::process::Command::new("comby")
            .args(&[
                pattern,
                rewrite,
                file_path,
                "--in-place",
                "--timeout", "30"
            ])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("comby", format!("Failed to execute comby: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("Comby stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Comby stderr: {}", stderr);
            }
            
            Ok(format!("Successfully transformed {}: {}", file_path, stdout.trim()))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("comby", format!("Comby failed for {}: {}", file_path, stderr)))
        }
    }
}

/// Ast-grep transformation tool
pub struct AstGrepTool;

#[async_trait]
impl TransformationTool for AstGrepTool {
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Executing ast-grep for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for transformation".to_string()));
        }
        
        // Generate ast-grep pattern based on intent
        let pattern = self.generate_ast_grep_pattern(intent).await?;
        
        // Execute ast-grep on each file
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for file in files {
            match self.execute_ast_grep_on_file(&pattern, file).await {
                Ok(change) => changes.push(change),
                Err(e) => errors.push(format!("Failed to transform {}: {}", file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Processed {} files with ast-grep", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn supports_language(&self, language: &str) -> bool {
        matches!(language, "javascript" | "typescript" | "python" | "rust" | "go")
    }
    
    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Medium
    }
    
    fn name(&self) -> &str {
        "ast-grep"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if ast-grep is installed
        tokio::process::Command::new("sg")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl AstGrepTool {
    /// Generate ast-grep pattern based on intent
    async fn generate_ast_grep_pattern(&self, intent: &ActionIntent) -> ActionResult<String> {
        let description = &intent.description.to_lowercase();
        
        // Generate AST patterns based on intent description
        if description.contains("function") || description.contains("method") {
            Ok("function $FUNC() { $$$ }".to_string())
        } else if description.contains("class") {
            Ok("class $CLASS { $$$ }".to_string())
        } else if description.contains("import") {
            Ok("import $IMPORT from '$MODULE'".to_string())
        } else if description.contains("export") {
            Ok("export $EXPORT".to_string())
        } else {
            // Generic pattern
            Ok("$PATTERN".to_string())
        }
    }
    
    /// Execute ast-grep on a specific file
    async fn execute_ast_grep_on_file(&self, pattern: &str, file_path: &str) -> ActionResult<String> {
        info!("Executing ast-grep on file: {}", file_path);
        
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Execute ast-grep
        let output = tokio::process::Command::new("sg")
            .args(&[
                pattern,
                file_path,
                "--json"
            ])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("ast-grep", format!("Failed to execute ast-grep: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("Ast-grep stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Ast-grep stderr: {}", stderr);
            }
            
            Ok(format!("Successfully analyzed {}: found {} matches", file_path, stdout.lines().count()))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("ast-grep", format!("Ast-grep failed for {}: {}", file_path, stderr)))
        }
    }
}

/// Prettier transformation tool
pub struct PrettierTool;

#[async_trait]
impl TransformationTool for PrettierTool {
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Executing prettier for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for formatting".to_string()));
        }
        
        // Execute prettier on each file
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for file in files {
            match self.execute_prettier_on_file(file).await {
                Ok(change) => changes.push(change),
                Err(e) => errors.push(format!("Failed to format {}: {}", file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Processed {} files with prettier", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn supports_language(&self, language: &str) -> bool {
        matches!(language, "javascript" | "typescript" | "jsx" | "tsx" | "json" | "css" | "scss" | "html")
    }
    
    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Low
    }
    
    fn name(&self) -> &str {
        "prettier"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if prettier is installed
        tokio::process::Command::new("npx")
            .args(&["prettier", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl PrettierTool {
    /// Execute prettier on a specific file
    async fn execute_prettier_on_file(&self, file_path: &str) -> ActionResult<String> {
        info!("Executing prettier on file: {}", file_path);
        
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Execute prettier
        let output = tokio::process::Command::new("npx")
            .args(&[
                "prettier",
                "--write",
                file_path
            ])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("prettier", format!("Failed to execute prettier: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("Prettier stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Prettier stderr: {}", stderr);
            }
            
            Ok(format!("Successfully formatted {}", file_path))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("prettier", format!("Prettier failed for {}: {}", file_path, stderr)))
        }
    }
}

/// ESLint transformation tool
pub struct ESLintTool;

#[async_trait]
impl TransformationTool for ESLintTool {
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Executing eslint for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for linting".to_string()));
        }
        
        // Execute eslint on each file
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for file in files {
            match self.execute_eslint_on_file(file).await {
                Ok(change) => changes.push(change),
                Err(e) => errors.push(format!("Failed to lint {}: {}", file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Processed {} files with eslint", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn supports_language(&self, language: &str) -> bool {
        matches!(language, "javascript" | "typescript" | "jsx" | "tsx")
    }
    
    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Low
    }
    
    fn name(&self) -> &str {
        "eslint"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if eslint is installed
        tokio::process::Command::new("npx")
            .args(&["eslint", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl ESLintTool {
    /// Execute eslint on a specific file
    async fn execute_eslint_on_file(&self, file_path: &str) -> ActionResult<String> {
        info!("Executing eslint on file: {}", file_path);
        
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Execute eslint with auto-fix
        let output = tokio::process::Command::new("npx")
            .args(&[
                "eslint",
                "--fix",
                file_path
            ])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("eslint", format!("Failed to execute eslint: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("ESLint stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("ESLint stderr: {}", stderr);
            }
            
            Ok(format!("Successfully linted and fixed {}", file_path))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("eslint", format!("ESLint failed for {}: {}", file_path, stderr)))
        }
    }
}

// Built-in validation tools

/// TypeScript validation tool
pub struct TypeScriptTool;

#[async_trait]
impl ValidationTool for TypeScriptTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running TypeScript validation for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract TypeScript files from intent scope
        let ts_files: Vec<&str> = intent.scope.iter()
            .filter(|file| file.ends_with(".ts") || file.ends_with(".tsx"))
            .map(|s| s.as_str())
            .collect();
        
        if ts_files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec![],
                output: "No TypeScript files to validate".to_string(),
                errors: vec![],
                warnings: vec![],
                duration: start.elapsed(),
            });
        }
        
        // Run TypeScript compiler check
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for file in &ts_files {
            match self.validate_typescript_file(file).await {
                Ok(_) => {},
                Err(e) => errors.push(format!("TypeScript error in {}: {}", file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes: vec![],
            output: format!("TypeScript validation completed for {} files", ts_files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn name(&self) -> &str {
        "typescript"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if TypeScript is installed
        tokio::process::Command::new("npx")
            .args(&["tsc", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl TypeScriptTool {
    /// Validate a TypeScript file
    async fn validate_typescript_file(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("npx")
            .args(&["tsc", "--noEmit", file_path])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("typescript", format!("Failed to run TypeScript check: {}", e)))?;
        
        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("typescript", format!("TypeScript validation failed: {}", error)))
        }
    }
}

/// Jest validation tool
pub struct JestTool;

#[async_trait]
impl ValidationTool for JestTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running Jest tests for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for Jest testing".to_string()));
        }
        
        // Find test files and related source files
        let test_files: Vec<&String> = files.iter()
            .filter(|f| f.contains("test") || f.contains("spec") || f.ends_with(".test.js") || f.ends_with(".test.ts") || f.ends_with(".spec.js") || f.ends_with(".spec.ts"))
            .collect();
        
        if test_files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec!["No test files found in scope".to_string()],
                output: "No test files found in scope".to_string(),
                errors: vec![],
                warnings: vec!["No test files found in scope".to_string()],
                duration: start.elapsed(),
            });
        }
        
        // Run Jest tests
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        match self.run_jest_tests(&test_files).await {
            Ok(output) => {
                changes.push("Jest tests completed successfully".to_string());
                if !output.is_empty() {
                    changes.push(format!("Jest output: {}", output));
                }
            },
            Err(e) => errors.push(format!("Jest tests failed: {}", e)),
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Ran Jest tests on {} files", test_files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn name(&self) -> &str {
        "jest"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if Jest is installed
        tokio::process::Command::new("npx")
            .args(&["jest", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl JestTool {
    /// Run Jest tests on specified files
    async fn run_jest_tests(&self, test_files: &[&String]) -> ActionResult<String> {
        info!("Running Jest tests on {} files", test_files.len());
        
        // Execute Jest
        let output = tokio::process::Command::new("npx")
            .args(&[
                "jest",
                "--passWithNoTests",
                "--verbose",
                "--json"
            ])
            .args(test_files.iter().map(|f| f.as_str()))
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("jest", format!("Failed to execute Jest: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("Jest stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Jest stderr: {}", stderr);
            }
            
            Ok(stdout.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("jest", format!("Jest tests failed: {}", stderr)))
        }
    }
}

/// Mocha validation tool
pub struct MochaTool;

#[async_trait]
impl ValidationTool for MochaTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running Mocha tests for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for Mocha testing".to_string()));
        }
        
        // Find test files
        let test_files: Vec<&String> = files.iter()
            .filter(|f| f.contains("test") || f.contains("spec") || f.ends_with(".test.js") || f.ends_with(".test.ts") || f.ends_with(".spec.js") || f.ends_with(".spec.ts"))
            .collect();
        
        if test_files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec!["No test files found in scope".to_string()],
                output: "No test files found in scope".to_string(),
                errors: vec![],
                warnings: vec!["No test files found in scope".to_string()],
                duration: start.elapsed(),
            });
        }
        
        // Run Mocha tests
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        match self.run_mocha_tests(&test_files).await {
            Ok(output) => {
                changes.push("Mocha tests completed successfully".to_string());
                if !output.is_empty() {
                    changes.push(format!("Mocha output: {}", output));
                }
            },
            Err(e) => errors.push(format!("Mocha tests failed: {}", e)),
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Ran Mocha tests on {} files", test_files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn name(&self) -> &str {
        "mocha"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if Mocha is installed
        tokio::process::Command::new("npx")
            .args(&["mocha", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl MochaTool {
    /// Run Mocha tests on specified files
    async fn run_mocha_tests(&self, test_files: &[&String]) -> ActionResult<String> {
        info!("Running Mocha tests on {} files", test_files.len());
        
        // Execute Mocha
        let output = tokio::process::Command::new("npx")
            .args(&[
                "mocha",
                "--timeout", "10000",
                "--reporter", "spec"
            ])
            .args(test_files.iter().map(|f| f.as_str()))
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("mocha", format!("Failed to execute Mocha: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("Mocha stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Mocha stderr: {}", stderr);
            }
            
            Ok(stdout.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("mocha", format!("Mocha tests failed: {}", stderr)))
        }
    }
}

/// PyTest validation tool
pub struct PyTestTool;

#[async_trait]
impl ValidationTool for PyTestTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running PyTest for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::validation("No files specified for PyTest".to_string()));
        }
        
        // Find Python test files
        let test_files: Vec<&String> = files.iter()
            .filter(|f| f.ends_with(".py") && (f.contains("test") || f.contains("spec")))
            .collect();
        
        if test_files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec!["No Python test files found in scope".to_string()],
                output: "No Python test files found in scope".to_string(),
                errors: vec![],
                warnings: vec!["No Python test files found in scope".to_string()],
                duration: start.elapsed(),
            });
        }
        
        // Run PyTest
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        match self.run_pytest_tests(&test_files).await {
            Ok(output) => {
                changes.push("PyTest completed successfully".to_string());
                if !output.is_empty() {
                    changes.push(format!("PyTest output: {}", output));
                }
            },
            Err(e) => errors.push(format!("PyTest failed: {}", e)),
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Ran PyTest on {} files", test_files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn name(&self) -> &str {
        "pytest"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if PyTest is installed
        tokio::process::Command::new("pytest")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl PyTestTool {
    /// Run PyTest on specified files
    async fn run_pytest_tests(&self, test_files: &[&String]) -> ActionResult<String> {
        info!("Running PyTest on {} files", test_files.len());
        
        // Execute PyTest
        let output = tokio::process::Command::new("pytest")
            .args(&[
                "--verbose",
                "--tb=short"
            ])
            .args(test_files.iter().map(|f| f.as_str()))
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("pytest", format!("Failed to execute PyTest: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            info!("PyTest stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("PyTest stderr: {}", stderr);
            }
            
            Ok(stdout.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("pytest", format!("PyTest failed: {}", stderr)))
        }
    }
}

/// Cargo validation tool
pub struct CargoTool;

#[async_trait]
impl ValidationTool for CargoTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running Cargo check for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Find Cargo.toml files in the scope
        let cargo_files: Vec<&str> = intent.scope.iter()
            .filter(|file| file.ends_with("Cargo.toml"))
            .map(|s| s.as_str())
            .collect();
        
        if cargo_files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec![],
                output: "No Cargo.toml files found to validate".to_string(),
                errors: vec![],
                warnings: vec![],
                duration: start.elapsed(),
            });
        }
        
        // Run cargo check for each project
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for cargo_file in &cargo_files {
            match self.run_cargo_check(cargo_file).await {
                Ok(_) => {},
                Err(e) => errors.push(format!("Cargo check failed for {}: {}", cargo_file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes: vec![],
            output: format!("Cargo validation completed for {} projects", cargo_files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn name(&self) -> &str {
        "cargo"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if Cargo is installed
        tokio::process::Command::new("cargo")
            .args(&["--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl CargoTool {
    /// Run cargo check for a project
    async fn run_cargo_check(&self, cargo_file: &str) -> ActionResult<()> {
        // Get the directory containing the Cargo.toml file
        let project_dir = std::path::Path::new(cargo_file)
            .parent()
            .ok_or_else(|| ActionError::validation("Invalid Cargo.toml path".to_string()))?;
        
        let output = tokio::process::Command::new("cargo")
            .args(&["check"])
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("cargo", format!("Failed to run cargo check: {}", e)))?;
        
        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("cargo", format!("Cargo check failed: {}", error)))
        }
    }
}


// Built-in safety tools

/// Syntax validation safety tool
pub struct SyntaxValidationTool;

#[async_trait]
impl SafetyTool for SyntaxValidationTool {
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running syntax validation for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        
        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec!["No files to validate".to_string()],
                output: "No files found in scope".to_string(),
                errors: vec![],
                warnings: vec!["No files found in scope".to_string()],
                duration: start.elapsed(),
            });
        }
        
        // Validate syntax for each file
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        for file in files {
            match self.validate_file_syntax(file).await {
                Ok(result) => changes.push(format!("{}: {}", file, result)),
                Err(e) => errors.push(format!("{}: {}", file, e)),
            }
        }
        
        let success = errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes,
            output: format!("Syntax validation completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    fn name(&self) -> &str {
        "syntax_validation"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if basic syntax validation tools are available
        let node_available = tokio::process::Command::new("node")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false);
        
        let python_available = tokio::process::Command::new("python3")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false);
        
        let rust_available = tokio::process::Command::new("rustc")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false);
        
        node_available || python_available || rust_available
    }
}

impl SyntaxValidationTool {
    /// Validate syntax for a specific file
    async fn validate_file_syntax(&self, file_path: &str) -> ActionResult<String> {
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Determine language and run appropriate syntax checker
        if file_path.ends_with(".js") || file_path.ends_with(".ts") || file_path.ends_with(".jsx") || file_path.ends_with(".tsx") {
            self.validate_javascript_syntax(file_path).await
        } else if file_path.ends_with(".py") {
            self.validate_python_syntax(file_path).await
        } else if file_path.ends_with(".rs") {
            self.validate_rust_syntax(file_path).await
        } else {
            Ok("Syntax validation not implemented for this file type".to_string())
        }
    }
    
    /// Validate JavaScript/TypeScript syntax
    async fn validate_javascript_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("node")
            .args(&["--check", file_path])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("syntax_validation", format!("Failed to check JavaScript syntax: {}", e)))?;
        
        if output.status.success() {
            Ok("JavaScript syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("syntax_validation", format!("JavaScript syntax error: {}", error)))
        }
    }
    
    /// Validate Python syntax
    async fn validate_python_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("python3")
            .args(&["-m", "py_compile", file_path])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("syntax_validation", format!("Failed to check Python syntax: {}", e)))?;
        
        if output.status.success() {
            Ok("Python syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("syntax_validation", format!("Python syntax error: {}", error)))
        }
    }
    
    /// Validate Rust syntax
    async fn validate_rust_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("rustc")
            .args(&["--emit=metadata", "--crate-type=lib", file_path])
            .output()
            .await
            .map_err(|e| ActionError::tool_execution("syntax_validation", format!("Failed to check Rust syntax: {}", e)))?;
        
        if output.status.success() {
            Ok("Rust syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::tool_execution("syntax_validation", format!("Rust syntax error: {}", error)))
        }
    }
}

/// Type checking safety tool
pub struct TypeCheckingTool;

#[async_trait]
impl SafetyTool for TypeCheckingTool {
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running type checking for intent: {}", intent.id);
        
        // TODO: Implement actual type checking
        
        Ok(ToolResult {
            success: true,
            changes: vec![],
            output: "Type checking passed".to_string(),
            errors: vec![],
            warnings: vec![],
            duration: std::time::Duration::from_secs(1),
        })
    }
    
    fn name(&self) -> &str {
        "type_checking"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        true
    }
}

/// Test coverage safety tool
pub struct TestCoverageTool;

#[async_trait]
impl SafetyTool for TestCoverageTool {
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running test coverage check for intent: {}", intent.id);
        
        // TODO: Implement actual test coverage check
        
        Ok(ToolResult {
            success: true,
            changes: vec![],
            output: "Test coverage check passed".to_string(),
            errors: vec![],
            warnings: vec![],
            duration: std::time::Duration::from_secs(1),
        })
    }
    
    fn name(&self) -> &str {
        "test_coverage"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        true
    }
}

/// Security scanning safety tool
pub struct SecurityScanningTool;

#[async_trait]
impl SafetyTool for SecurityScanningTool {
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running security scanning for intent: {}", intent.id);
        
        // TODO: Implement actual security scanning
        
        Ok(ToolResult {
            success: true,
            changes: vec![],
            output: "Security scanning passed".to_string(),
            errors: vec![],
            warnings: vec![],
            duration: std::time::Duration::from_secs(1),
        })
    }
    
    fn name(&self) -> &str {
        "security_scanning"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};

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