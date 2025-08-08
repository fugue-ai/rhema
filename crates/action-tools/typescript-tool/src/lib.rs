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
use rhema_action_tool::{ActionError, ActionIntent, ActionResult};
use rhema_action_tool::{ToolResult, ValidationTool};
use tracing::info;

/// TypeScript validation tool
pub struct TypeScriptTool;

#[async_trait]
impl ValidationTool for TypeScriptTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running TypeScript validation for intent: {}", intent.id);

        let start = std::time::Instant::now();

        // Extract TypeScript files from intent scope
        let ts_files: Vec<&str> = intent
            .scope
            .iter()
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
                Ok(_) => {}
                Err(e) => errors.push(format!("TypeScript error in {}: {}", file, e)),
            }
        }

        let success = errors.is_empty();

        Ok(ToolResult {
            success,
            changes: vec![],
            output: format!(
                "TypeScript validation completed for {} files",
                ts_files.len()
            ),
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
            .map_err(|e| ActionError::ToolExecution {
                tool: "typescript".to_string(),
                message: format!("Failed to run TypeScript check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "typescript".to_string(),
                message: format!("TypeScript validation failed: {}", error),
            })
        }
    }
}
