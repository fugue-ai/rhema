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
use rhema_action_tool::{ActionIntent, ActionResult, ActionError};
use rhema_action_tool::{ValidationTool, ToolResult};

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
            return Err(ActionError::Validation("No files specified for PyTest".to_string()));
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
            .map_err(|e| ActionError::ToolExecution { 
                tool: "pytest".to_string(), 
                message: format!("Failed to execute PyTest: {}", e) 
            })?;
        
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
            Err(ActionError::ToolExecution { 
                tool: "pytest".to_string(), 
                message: format!("PyTest failed: {}", stderr) 
            })
        }
    }
} 