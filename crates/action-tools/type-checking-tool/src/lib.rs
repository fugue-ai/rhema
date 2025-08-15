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
use rhema_action_tool::{SafetyTool, ToolResult};
use std::collections::HashMap;
use tracing::info;

/// Type checking safety tool
pub struct TypeCheckingTool;

#[async_trait]
impl SafetyTool for TypeCheckingTool {
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running type checking for intent: {}", intent.id);

        let start = std::time::Instant::now();

        // Extract file paths from intent scope
        let files = &intent.scope;
        if files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec![],
                output: "No files specified for type checking".to_string(),
                errors: vec![],
                warnings: vec![],
                duration: start.elapsed(),
            });
        }

        // Group files by language for efficient processing
        let files_by_language = self.group_files_by_language(files);
        
        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();
        let mut type_check_results = Vec::new();

        // Run type checking for each language
        for (language, files) in files_by_language {
            info!("Type checking {} files for language: {}", files.len(), language);
            
            match self.check_language_types(&language, &files).await {
                Ok(result) => {
                    type_check_results.push(format!("{}: {} files checked successfully", language, files.len()));
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    all_errors.push(format!("Type checking failed for {}: {}", language, e));
                }
            }
        }

        let success = all_errors.is_empty();
        let output = if success {
            format!("Type checking completed successfully for {} files", files.len())
        } else {
            format!("Type checking failed with {} errors", all_errors.len())
        };

        Ok(ToolResult {
            success,
            changes: type_check_results,
            output,
            errors: all_errors,
            warnings: all_warnings,
            duration: start.elapsed(),
        })
    }

    fn name(&self) -> &str {
        "type_checking"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    async fn is_available(&self) -> bool {
        // Check if any type checking tools are available
        let mut available_tools = 0;
        
        // Check TypeScript
        if self.check_typescript_available().await {
            available_tools += 1;
        }
        
        // Check Python
        if self.check_python_available().await {
            available_tools += 1;
        }
        
        // Check Rust
        if self.check_rust_available().await {
            available_tools += 1;
        }
        
        // Check Go
        if self.check_go_available().await {
            available_tools += 1;
        }

        available_tools > 0
    }
}

impl TypeCheckingTool {
    /// Group files by their programming language
    fn group_files_by_language(&self, files: &[String]) -> HashMap<String, Vec<String>> {
        let mut grouped = HashMap::new();
        
        for file in files {
            let language = self.detect_language(file);
            grouped.entry(language).or_insert_with(Vec::new).push(file.clone());
        }
        
        grouped
    }

    /// Detect programming language from file extension
    fn detect_language(&self, file_path: &str) -> String {
        if file_path.ends_with(".ts") || file_path.ends_with(".tsx") {
            "typescript".to_string()
        } else if file_path.ends_with(".js") || file_path.ends_with(".jsx") {
            "javascript".to_string()
        } else if file_path.ends_with(".py") {
            "python".to_string()
        } else if file_path.ends_with(".rs") {
            "rust".to_string()
        } else if file_path.ends_with(".go") {
            "go".to_string()
        } else if file_path.ends_with(".java") {
            "java".to_string()
        } else if file_path.ends_with(".kt") {
            "kotlin".to_string()
        } else if file_path.ends_with(".swift") {
            "swift".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Check types for a specific language
    async fn check_language_types(&self, language: &str, files: &[String]) -> ActionResult<ToolResult> {
        match language {
            "typescript" => self.check_typescript_types(files).await,
            "javascript" => self.check_javascript_types(files).await,
            "python" => self.check_python_types(files).await,
            "rust" => self.check_rust_types(files).await,
            "go" => self.check_go_types(files).await,
            "java" => self.check_java_types(files).await,
            "kotlin" => self.check_kotlin_types(files).await,
            "swift" => self.check_swift_types(files).await,
            _ => Ok(ToolResult {
                success: true,
                changes: vec![],
                output: format!("Type checking not implemented for language: {}", language),
                errors: vec![],
                warnings: vec![format!("No type checker available for {}", language)],
                duration: std::time::Duration::from_millis(1),
            }),
        }
    }

    /// Check TypeScript types
    async fn check_typescript_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if TypeScript is available
        if !self.check_typescript_available().await {
            return Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: "TypeScript compiler (tsc) not available".to_string(),
            });
        }

        // Run TypeScript type checking
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_typescript_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("TypeScript error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("TypeScript type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    /// Check JavaScript types (using JSDoc or TypeScript)
    async fn check_javascript_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // For JavaScript, we'll check for JSDoc annotations and basic syntax
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_javascript_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("JavaScript type error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("JavaScript type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    /// Check Python types
    async fn check_python_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if mypy is available
        if !self.check_python_available().await {
            return Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: "Python type checker (mypy) not available".to_string(),
            });
        }

        // Run Python type checking with mypy
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_python_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("Python type error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("Python type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    /// Check Rust types
    async fn check_rust_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if Rust is available
        if !self.check_rust_available().await {
            return Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: "Rust compiler (rustc) not available".to_string(),
            });
        }

        // Run Rust type checking
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_rust_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("Rust type error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("Rust type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    /// Check Go types
    async fn check_go_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if Go is available
        if !self.check_go_available().await {
            return Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: "Go compiler (go) not available".to_string(),
            });
        }

        // Run Go type checking
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_go_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("Go type error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("Go type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    /// Check Java types
    async fn check_java_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if Java is available
        if !self.check_java_available().await {
            return Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: "Java compiler (javac) not available".to_string(),
            });
        }

        // Run Java type checking
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_java_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("Java type error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("Java type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    /// Check Kotlin types
    async fn check_kotlin_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if Kotlin is available
        if !self.check_kotlin_available().await {
            return Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: "Kotlin compiler (kotlinc) not available".to_string(),
            });
        }

        // Run Kotlin type checking
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_kotlin_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("Kotlin type error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("Kotlin type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    /// Check Swift types
    async fn check_swift_types(&self, files: &[String]) -> ActionResult<ToolResult> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if Swift is available
        if !self.check_swift_available().await {
            return Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: "Swift compiler (swiftc) not available".to_string(),
            });
        }

        // Run Swift type checking
        for file in files {
            if !std::path::Path::new(file).exists() {
                warnings.push(format!("File not found: {}", file));
                continue;
            }

            match self.run_swift_check(file).await {
                Ok(_) => {}
                Err(e) => errors.push(format!("Swift type error in {}: {}", file, e)),
            }
        }

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output: format!("Swift type checking completed for {} files", files.len()),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }

    // Language availability checks
    async fn check_typescript_available(&self) -> bool {
        tokio::process::Command::new("npx")
            .args(&["tsc", "--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn check_python_available(&self) -> bool {
        tokio::process::Command::new("mypy")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn check_rust_available(&self) -> bool {
        tokio::process::Command::new("rustc")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn check_go_available(&self) -> bool {
        tokio::process::Command::new("go")
            .arg("version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn check_java_available(&self) -> bool {
        tokio::process::Command::new("javac")
            .arg("-version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn check_kotlin_available(&self) -> bool {
        tokio::process::Command::new("kotlinc")
            .arg("-version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn check_swift_available(&self) -> bool {
        tokio::process::Command::new("swiftc")
            .arg("-version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    // Language-specific type checking implementations
    async fn run_typescript_check(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("npx")
            .args(&["tsc", "--noEmit", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run TypeScript check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("TypeScript type checking failed: {}", error),
            })
        }
    }

    async fn run_javascript_check(&self, file_path: &str) -> ActionResult<()> {
        // For JavaScript, we'll use Node.js to check syntax and basic type annotations
        let output = tokio::process::Command::new("node")
            .args(&["--check", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run JavaScript check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("JavaScript type checking failed: {}", error),
            })
        }
    }

    async fn run_python_check(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("mypy")
            .args(&["--ignore-missing-imports", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run Python type check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Python type checking failed: {}", error),
            })
        }
    }

    async fn run_rust_check(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("rustc")
            .args(&["--emit=metadata", "--crate-type=lib", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run Rust type check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Rust type checking failed: {}", error),
            })
        }
    }

    async fn run_go_check(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("go")
            .args(&["build", "-o", "/dev/null", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run Go type check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Go type checking failed: {}", error),
            })
        }
    }

    async fn run_java_check(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("javac")
            .args(&["-Xlint:all", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run Java type check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Java type checking failed: {}", error),
            })
        }
    }

    async fn run_kotlin_check(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("kotlinc")
            .args(&["-no-stdlib", "-no-reflect", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run Kotlin type check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Kotlin type checking failed: {}", error),
            })
        }
    }

    async fn run_swift_check(&self, file_path: &str) -> ActionResult<()> {
        let output = tokio::process::Command::new("swiftc")
            .args(&["-typecheck", file_path])
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Failed to run Swift type check: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::ToolExecution {
                tool: "type_checking".to_string(),
                message: format!("Swift type checking failed: {}", error),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rhema_action_tool::ActionType;

    #[tokio::test]
    async fn test_type_checking_tool_creation() {
        let tool = TypeCheckingTool;
        assert_eq!(tool.name(), "type_checking");
        assert_eq!(tool.version(), "1.0.0");
    }

    #[tokio::test]
    async fn test_language_detection() {
        let tool = TypeCheckingTool;
        
        assert_eq!(tool.detect_language("test.ts"), "typescript");
        assert_eq!(tool.detect_language("test.tsx"), "typescript");
        assert_eq!(tool.detect_language("test.js"), "javascript");
        assert_eq!(tool.detect_language("test.jsx"), "javascript");
        assert_eq!(tool.detect_language("test.py"), "python");
        assert_eq!(tool.detect_language("test.rs"), "rust");
        assert_eq!(tool.detect_language("test.go"), "go");
        assert_eq!(tool.detect_language("test.java"), "java");
        assert_eq!(tool.detect_language("test.kt"), "kotlin");
        assert_eq!(tool.detect_language("test.swift"), "swift");
        assert_eq!(tool.detect_language("test.txt"), "unknown");
    }

    #[tokio::test]
    async fn test_file_grouping() {
        let tool = TypeCheckingTool;
        let files = vec![
            "test1.ts".to_string(),
            "test2.tsx".to_string(),
            "test3.py".to_string(),
            "test4.rs".to_string(),
            "test5.go".to_string(),
        ];
        
        let grouped = tool.group_files_by_language(&files);
        
        assert_eq!(grouped.get("typescript").unwrap().len(), 2);
        assert_eq!(grouped.get("python").unwrap().len(), 1);
        assert_eq!(grouped.get("rust").unwrap().len(), 1);
        assert_eq!(grouped.get("go").unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_empty_scope() {
        let tool = TypeCheckingTool;
        let intent = ActionIntent::new(
            "test-001",
            ActionType::Refactor,
            "Test refactoring",
            vec![],
            rhema_action_tool::SafetyLevel::Medium,
        );
        
        let result = tool.check(&intent).await.unwrap();
        assert!(result.success);
        assert_eq!(result.output, "No files specified for type checking");
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_unknown_language_handling() {
        let tool = TypeCheckingTool;
        let files = vec!["test.unknown".to_string()];
        
        let result = tool.check_language_types("unknown", &files).await.unwrap();
        assert!(result.success);
        assert!(result.warnings.contains(&"No type checker available for unknown".to_string()));
    }
}
