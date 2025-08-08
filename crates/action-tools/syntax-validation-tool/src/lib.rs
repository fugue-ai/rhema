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
use rhema_action_tool::{ActionError, ActionIntent, ActionResult, SafetyLevel};
use rhema_action_tool::{SafetyTool, ToolResult};
use tracing::info;

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
            return Err(ActionError::Validation(
                "No files specified for syntax validation".to_string(),
            ));
        }

        // Validate syntax for each file
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();

        for file in files {
            match self.validate_file_syntax(file).await {
                Ok(result) => changes.push(result),
                Err(e) => errors.push(format!("Syntax validation failed for {}: {}", file, e)),
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
            return Err(ActionError::Validation(format!(
                "File not found: {}",
                file_path
            )));
        }

        // Determine language and run appropriate syntax checker
        if file_path.ends_with(".js")
            || file_path.ends_with(".ts")
            || file_path.ends_with(".jsx")
            || file_path.ends_with(".tsx")
        {
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
            .map_err(|e| ActionError::ToolExecution {
                tool: "syntax_validation".to_string(),
                message: format!("Failed to check JavaScript syntax: {}", e),
            })?;

        if output.status.success() {
            Ok("JavaScript syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "syntax_validation".to_string(),
                message: format!("JavaScript syntax error: {}", error),
            })
        }
    }

    /// Validate Python syntax
    async fn validate_python_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("python3")
            .args(&["-m", "py_compile", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "syntax_validation".to_string(),
                message: format!("Failed to check Python syntax: {}", e),
            })?;

        if output.status.success() {
            Ok("Python syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "syntax_validation".to_string(),
                message: format!("Python syntax error: {}", error),
            })
        }
    }

    /// Validate Rust syntax
    async fn validate_rust_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("rustc")
            .args(&["--emit=metadata", "--crate-type=lib", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "syntax_validation".to_string(),
                message: format!("Failed to check Rust syntax: {}", e),
            })?;

        if output.status.success() {
            Ok("Rust syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "syntax_validation".to_string(),
                message: format!("Rust syntax error: {}", error),
            })
        }
    }
}
