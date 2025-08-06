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
use rhema_action_tool::{ActionIntent, ActionResult, ActionError, SafetyLevel};
use rhema_action_tool::{TransformationTool, ToolResult};

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
            return Err(ActionError::Validation("No files specified for formatting".to_string()));
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
            return Err(ActionError::Validation(format!("File not found: {}", file_path)));
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
            .map_err(|e| ActionError::ToolExecution { 
                tool: "prettier".to_string(), 
                message: format!("Failed to execute prettier: {}", e) 
            })?;
        
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
            Err(ActionError::ToolExecution { 
                tool: "prettier".to_string(), 
                message: format!("Prettier failed for {}: {}", file_path, stderr) 
            })
        }
    }
} 