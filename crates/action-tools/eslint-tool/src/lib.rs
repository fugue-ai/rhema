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
use rhema_action_tool::{ToolResult, TransformationTool};
use tracing::{info, warn};

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
            return Err(ActionError::Validation(
                "No files specified for linting".to_string(),
            ));
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
            return Err(ActionError::Validation(format!(
                "File not found: {}",
                file_path
            )));
        }

        // Execute eslint with auto-fix
        let output = tokio::process::Command::new("npx")
            .args(&["eslint", "--fix", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "eslint".to_string(),
                message: format!("Failed to execute eslint: {}", e),
            })?;

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
            Err(ActionError::ToolExecution {
                tool: "eslint".to_string(),
                message: format!("ESLint failed for {}: {}", file_path, stderr),
            })
        }
    }
}
