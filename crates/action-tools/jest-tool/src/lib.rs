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
            return Err(ActionError::Validation("No files specified for Jest testing".to_string()));
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
            .map_err(|e| ActionError::ToolExecution { 
                tool: "jest".to_string(), 
                message: format!("Failed to execute Jest: {}", e) 
            })?;
        
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
            Err(ActionError::ToolExecution { 
                tool: "jest".to_string(), 
                message: format!("Jest tests failed: {}", stderr) 
            })
        }
    }
} 