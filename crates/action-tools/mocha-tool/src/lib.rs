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
use tracing::{info, warn};

/// Mocha validation tool
pub struct MochaTool;

#[async_trait]
impl ValidationTool for MochaTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running Mocha tests for intent: {}", intent.id);

        let start = std::time::Instant::now();

        // Extract file paths from intent
        let files = &intent.scope;
        if files.is_empty() {
            return Err(ActionError::Validation(
                "No files specified for Mocha testing".to_string(),
            ));
        }

        // Find test files
        let test_files: Vec<&String> = files
            .iter()
            .filter(|f| {
                f.contains("test")
                    || f.contains("spec")
                    || f.ends_with(".test.js")
                    || f.ends_with(".test.ts")
                    || f.ends_with(".spec.js")
                    || f.ends_with(".spec.ts")
            })
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

        // Run Mocha tests
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();

        match self.run_mocha_tests(&test_files).await {
            Ok(output) => {
                changes.push("Mocha tests completed successfully".to_string());
                if !output.is_empty() {
                    changes.push(format!("Mocha output: {}", output));
                }
            }
            Err(e) => errors.push(format!("Mocha tests failed: {}", e)),
        }

        let success = errors.is_empty();

        Ok(ToolResult {
            success,
            changes,
            output: format!("Ran Mocha tests on {} files", test_files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    fn name(&self) -> &str {
        "mocha"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    async fn is_available(&self) -> bool {
        // Check if Mocha is installed
        tokio::process::Command::new("npx")
            .args(&["mocha", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl MochaTool {
    /// Run Mocha tests on specified files
    async fn run_mocha_tests(&self, test_files: &[&String]) -> ActionResult<String> {
        info!("Running Mocha tests on {} files", test_files.len());

        // Execute Mocha
        let output = tokio::process::Command::new("npx")
            .args(&["mocha", "--reporter", "spec", "--timeout", "5000"])
            .args(test_files.iter().map(|f| f.as_str()))
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "mocha".to_string(),
                message: format!("Failed to execute Mocha: {}", e),
            })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            info!("Mocha stdout: {}", stdout);
            if !stderr.is_empty() {
                warn!("Mocha stderr: {}", stderr);
            }

            Ok(stdout.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "mocha".to_string(),
                message: format!("Mocha tests failed: {}", stderr),
            })
        }
    }
}
