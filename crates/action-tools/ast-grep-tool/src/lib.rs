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
            return Err(ActionError::Validation(
                "No files specified for ast-grep transformation".to_string(),
            ));
        }

        // Generate AST pattern based on intent
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
        matches!(
            language,
            "javascript" | "typescript" | "python" | "rust" | "go"
        )
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
    async fn execute_ast_grep_on_file(
        &self,
        pattern: &str,
        file_path: &str,
    ) -> ActionResult<String> {
        info!("Executing ast-grep on file: {}", file_path);

        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::Validation(format!(
                "File not found: {}",
                file_path
            )));
        }

        // Execute ast-grep
        let output = tokio::process::Command::new("sg")
            .args(&[pattern, file_path, "--json"])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "ast-grep".to_string(),
                message: format!("Failed to execute ast-grep: {}", e),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            info!("Ast-grep stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Ast-grep stderr: {}", stderr);
            }

            Ok(format!(
                "Successfully analyzed {}: found {} matches",
                file_path,
                stdout.lines().count()
            ))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "ast-grep".to_string(),
                message: format!("Ast-grep failed for {}: {}", file_path, stderr),
            })
        }
    }
}
