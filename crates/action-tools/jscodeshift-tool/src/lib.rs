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

use async_trait::async_trait;
use tracing::{info, warn};
use rhema_action_tool::{ActionIntent, ActionResult, ActionError, SafetyLevel, TransformationTool, ToolResult};

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
            return Err(ActionError::Validation("No files specified for transformation".to_string()));
        }
        
        // Create temporary jscodeshift script based on intent
        let script_content = self.generate_jscodeshift_script(intent).await?;
        let script_path = std::env::temp_dir().join("jscodeshift_script.js");
        tokio::fs::write(&script_path, script_content).await
            .map_err(|e| ActionError::ToolExecution { tool: "jscodeshift".to_string(), message: format!("Failed to write jscodeshift script: {}", e) })?;
        
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
            return Err(ActionError::Validation(format!("File not found: {}", file_path)));
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
            .map_err(|e| ActionError::ToolExecution { tool: "jscodeshift".to_string(), message: format!("Failed to execute jscodeshift: {}", e) })?;
        
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
            Err(ActionError::ToolExecution { tool: "jscodeshift".to_string(), message: format!("Jscodeshift failed for {}: {}", file_path, stderr) })
        }
    }
} 