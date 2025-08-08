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
            return Err(ActionError::Validation(
                "No files specified for transformation".to_string(),
            ));
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
    async fn generate_comby_pattern(
        &self,
        intent: &ActionIntent,
    ) -> ActionResult<(String, String)> {
        let description = &intent.description.to_lowercase();

        // Simple pattern generation based on intent description
        if description.contains("rename") || description.contains("refactor") {
            Ok((
                "function :[name]() { :[body] }".to_string(),
                "function :[name]() { :[body] }".to_string(),
            ))
        } else if description.contains("add") || description.contains("insert") {
            Ok((
                ":[before]".to_string(),
                ":[before]\n// TODO: Add implementation".to_string(),
            ))
        } else if description.contains("remove") || description.contains("delete") {
            Ok((
                "// TODO: Remove this\n:[code]".to_string(),
                ":[code]".to_string(),
            ))
        } else {
            // Generic pattern for other transformations
            Ok((":[pattern]".to_string(), ":[replacement]".to_string()))
        }
    }

    /// Execute comby on a specific file
    async fn execute_comby_on_file(
        &self,
        pattern: &str,
        rewrite: &str,
        file_path: &str,
    ) -> ActionResult<String> {
        info!("Executing comby on file: {}", file_path);

        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::Validation(format!(
                "File not found: {}",
                file_path
            )));
        }

        // Execute comby
        let output = tokio::process::Command::new("comby")
            .args(&[pattern, rewrite, file_path, "--in-place", "--timeout", "30"])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "comby".to_string(),
                message: format!("Failed to execute comby: {}", e),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            info!("Comby stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Comby stderr: {}", stderr);
            }

            Ok(format!(
                "Successfully transformed {}: {}",
                file_path,
                stdout.trim()
            ))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "comby".to_string(),
                message: format!("Comby failed for {}: {}", file_path, stderr),
            })
        }
    }
}
