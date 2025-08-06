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
use tracing::info;
use rhema_action_tool::{ActionIntent, ActionResult, ActionError};
use rhema_action_tool::{ValidationTool, ToolResult};

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
            .ok_or_else(|| ActionError::Validation("Invalid Cargo.toml path".to_string()))?;
        
        let output = tokio::process::Command::new("cargo")
            .args(&["check"])
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution { 
                tool: "cargo".to_string(), 
                message: format!("Failed to run cargo check: {}", e) 
            })?;
        
        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution { 
                tool: "cargo".to_string(), 
                message: format!("Cargo check failed: {}", error) 
            })
        }
    }
} 