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

use anyhow::Result;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::schema::{ActionIntent as SchemaActionIntent, ActionType, SafetyLevel};
use crate::tools::ToolRegistry;
use rhema_action_tool::{ActionError, ActionIntent, ActionResult, ToolResult};

/// Action safety pipeline for executing actions with safety checks
pub struct ActionSafetyPipeline {
    tool_registry: Arc<ToolRegistry>,
}

impl ActionSafetyPipeline {
    /// Create a new action safety pipeline
    pub async fn new() -> Result<Self> {
        info!("Initializing Action Safety Pipeline");

        let tool_registry = Arc::new(
            ToolRegistry::new()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to initialize tool registry: {:?}", e))?,
        );

        info!("Action Safety Pipeline initialized successfully");
        Ok(Self { tool_registry })
    }

    /// Execute an action with safety checks
    pub async fn execute_action(&self, intent: &SchemaActionIntent) -> Result<ExecutionResult> {
        info!("Executing action: {}", intent.id);

        let start = std::time::Instant::now();

        // Convert schema intent to shared intent
        let shared_intent = self.convert_to_shared_intent(intent);

        // Execute based on action type
        let result = match intent.action_type {
            ActionType::Refactor => self.execute_refactor_action(&shared_intent).await?,
            ActionType::BugFix => self.execute_bugfix_action(&shared_intent).await?,
            ActionType::Feature => self.execute_feature_action(&shared_intent).await?,
            ActionType::Security => self.execute_security_action(&shared_intent).await?,
            ActionType::Performance => self.execute_performance_action(&shared_intent).await?,
            ActionType::Documentation => {
                // Documentation actions typically don't need execution
                ToolResult {
                    success: true,
                    changes: vec!["Documentation action completed".to_string()],
                    output: "Documentation action completed".to_string(),
                    errors: vec![],
                    warnings: vec![],
                    duration: std::time::Duration::from_secs(1),
                }
            }
            ActionType::Test => self.execute_test_action(&shared_intent).await?,
            ActionType::Configuration => self.execute_configuration_action(&shared_intent).await?,
            ActionType::Dependency => self.execute_dependency_action(&shared_intent).await?,
            ActionType::Cleanup => self.execute_cleanup_action(&shared_intent).await?,
            ActionType::Migration => self.execute_migration_action(&shared_intent).await?,
            ActionType::Custom(_) => self.execute_default_action(&shared_intent).await?,
        };

        let duration = start.elapsed();

        let execution_result = ExecutionResult {
            success: result.success,
            changes: result.changes,
            errors: result.errors,
            warnings: result.warnings,
            duration,
        };

        if execution_result.success {
            info!("Action execution completed successfully in {:?}", duration);
        } else {
            warn!(
                "Action execution completed with {} errors in {:?}",
                execution_result.errors.len(),
                duration
            );
        }

        Ok(execution_result)
    }

    /// Execute refactor action
    async fn execute_refactor_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing refactor action");

        // Run transformation tools
        let transformations = vec![
            ("prettier", "Code formatting"),
            ("eslint", "Code linting"),
            ("ast-grep", "AST-based transformations"),
        ];

        let mut all_changes = Vec::new();
        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();

        for (tool_name, description) in transformations {
            match self.tool_registry.execute_tool(tool_name, intent).await {
                Ok(result) => {
                    all_changes.extend(result.changes);
                    all_errors.extend(result.errors);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    error!("{} failed: {:?}", description, e);
                    all_errors.push(format!("{} failed: {:?}", description, e));
                }
            }
        }

        let changes_count = all_changes.len();
        Ok(ToolResult {
            success: all_errors.is_empty(),
            changes: all_changes,
            output: format!("Refactor action completed with {} changes", changes_count),
            errors: all_errors,
            warnings: all_warnings,
            duration: std::time::Duration::from_secs(1), // Placeholder
        })
    }

    /// Execute bugfix action
    async fn execute_bugfix_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing bugfix action");

        // Run validation tools first
        let validations = vec![
            ("typescript", "TypeScript validation"),
            ("jest", "Jest tests"),
        ];

        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();

        for (tool_name, description) in validations {
            match self
                .tool_registry
                .execute_validation(tool_name, intent)
                .await
            {
                Ok(result) => {
                    all_errors.extend(result.errors);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    error!("{} failed: {:?}", description, e);
                    all_errors.push(format!("{} failed: {:?}", description, e));
                }
            }
        }

        Ok(ToolResult {
            success: all_errors.is_empty(),
            changes: vec!["Bugfix validation completed".to_string()],
            output: "Bugfix action completed".to_string(),
            errors: all_errors,
            warnings: all_warnings,
            duration: std::time::Duration::from_secs(1), // Placeholder
        })
    }

    /// Execute feature action
    async fn execute_feature_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing feature action");

        // Run comprehensive validation and transformation
        let mut all_changes = Vec::new();
        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();

        // Run transformations
        let transformations = vec![("prettier", "Code formatting"), ("eslint", "Code linting")];

        for (tool_name, description) in transformations {
            match self.tool_registry.execute_tool(tool_name, intent).await {
                Ok(result) => {
                    all_changes.extend(result.changes);
                    all_errors.extend(result.errors);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    error!("{} failed: {:?}", description, e);
                    all_errors.push(format!("{} failed: {:?}", description, e));
                }
            }
        }

        // Run validations
        let validations = vec![
            ("typescript", "TypeScript validation"),
            ("jest", "Jest tests"),
        ];

        for (tool_name, description) in validations {
            match self
                .tool_registry
                .execute_validation(tool_name, intent)
                .await
            {
                Ok(result) => {
                    all_errors.extend(result.errors);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    error!("{} failed: {:?}", description, e);
                    all_errors.push(format!("{} failed: {:?}", description, e));
                }
            }
        }

        let changes_count = all_changes.len();
        Ok(ToolResult {
            success: all_errors.is_empty(),
            changes: all_changes,
            output: format!("Feature action completed with {} changes", changes_count),
            errors: all_errors,
            warnings: all_warnings,
            duration: std::time::Duration::from_secs(1), // Placeholder
        })
    }

    /// Execute security action
    async fn execute_security_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing security action");

        // Run security checks
        let security_checks = vec![
            ("security_scanning", "Security scanning"),
            ("syntax_validation", "Syntax validation"),
        ];

        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();

        for (tool_name, description) in security_checks {
            match self
                .tool_registry
                .execute_safety_check(tool_name, intent)
                .await
            {
                Ok(result) => {
                    all_errors.extend(result.errors);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    error!("{} failed: {:?}", description, e);
                    all_errors.push(format!("{} failed: {:?}", description, e));
                }
            }
        }

        Ok(ToolResult {
            success: all_errors.is_empty(),
            changes: vec!["Security checks completed".to_string()],
            output: "Security action completed".to_string(),
            errors: all_errors,
            warnings: all_warnings,
            duration: std::time::Duration::from_secs(1), // Placeholder
        })
    }

    /// Execute performance action
    async fn execute_performance_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing performance action");

        // Run performance-related checks
        let performance_checks = vec![
            ("type_checking", "Type checking"),
            ("test_coverage", "Test coverage"),
        ];

        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();

        for (tool_name, description) in performance_checks {
            match self
                .tool_registry
                .execute_safety_check(tool_name, intent)
                .await
            {
                Ok(result) => {
                    all_errors.extend(result.errors);
                    all_warnings.extend(result.warnings);
                }
                Err(e) => {
                    error!("{} failed: {:?}", description, e);
                    all_errors.push(format!("{} failed: {:?}", description, e));
                }
            }
        }

        Ok(ToolResult {
            success: all_errors.is_empty(),
            changes: vec!["Performance checks completed".to_string()],
            output: "Performance action completed".to_string(),
            errors: all_errors,
            warnings: all_warnings,
            duration: std::time::Duration::from_secs(1), // Placeholder
        })
    }

    /// Convert schema intent to shared intent
    fn convert_to_shared_intent(&self, intent: &SchemaActionIntent) -> ActionIntent {
        ActionIntent {
            id: intent.id.clone(),
            action_type: match &intent.action_type {
                ActionType::Refactor => rhema_action_tool::ActionType::Refactor,
                ActionType::BugFix => rhema_action_tool::ActionType::BugFix,
                ActionType::Feature => rhema_action_tool::ActionType::Feature,
                ActionType::Security => rhema_action_tool::ActionType::Security,
                ActionType::Performance => rhema_action_tool::ActionType::Performance,
                ActionType::Documentation => {
                    rhema_action_tool::ActionType::Custom("documentation".to_string())
                }
                ActionType::Test => rhema_action_tool::ActionType::Test,
                ActionType::Configuration => {
                    rhema_action_tool::ActionType::Custom("configuration".to_string())
                }
                ActionType::Dependency => {
                    rhema_action_tool::ActionType::Custom("dependency".to_string())
                }
                ActionType::Cleanup => rhema_action_tool::ActionType::Custom("cleanup".to_string()),
                ActionType::Migration => {
                    rhema_action_tool::ActionType::Custom("migration".to_string())
                }
                ActionType::Custom(s) => rhema_action_tool::ActionType::Custom(s.clone()),
            },
            description: intent.description.clone(),
            scope: intent.scope.clone(),
            safety_level: match intent.safety_level {
                SafetyLevel::Low => rhema_action_tool::SafetyLevel::Low,
                SafetyLevel::Medium => rhema_action_tool::SafetyLevel::Medium,
                SafetyLevel::High => rhema_action_tool::SafetyLevel::High,
                SafetyLevel::Critical => rhema_action_tool::SafetyLevel::Critical,
            },
            created_at: intent.created_at,
            metadata: serde_json::to_value(intent.metadata.as_ref())
                .unwrap_or(serde_json::Value::Null),
            context_refs: intent.context_refs.as_ref().map(|refs| {
                refs.iter()
                    .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                    .collect()
            }),
            transformation: serde_json::to_value(&intent.transformation)
                .unwrap_or(serde_json::Value::Null),
            safety_checks: serde_json::to_value(&intent.safety_checks)
                .unwrap_or(serde_json::Value::Null),
            approval_workflow: serde_json::to_value(&intent.approval_workflow)
                .unwrap_or(serde_json::Value::Null),
            created_by: intent.created_by.clone(),
            tags: intent.tags.clone(),
            priority: intent.priority.clone(),
            estimated_effort: intent.estimated_effort.clone(),
            dependencies: intent.dependencies.clone(),
        }
    }

    /// Execute test action
    async fn execute_test_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing test action");

        // Run Jest tests
        let jest_result = self
            .tool_registry
            .execute_validation("jest", intent)
            .await
            .map_err(|e| anyhow::anyhow!("Jest validation failed: {:?}", e))?;

        Ok(ToolResult {
            success: jest_result.success,
            changes: vec!["Test execution completed".to_string()],
            output: "Test action completed".to_string(),
            errors: jest_result.errors,
            warnings: jest_result.warnings,
            duration: std::time::Duration::from_secs(1),
        })
    }

    /// Execute configuration action
    async fn execute_configuration_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing configuration action");

        // Run syntax validation
        let syntax_result = self
            .tool_registry
            .execute_safety_check("syntax_validation", intent)
            .await
            .map_err(|e| anyhow::anyhow!("Syntax validation failed: {:?}", e))?;

        Ok(ToolResult {
            success: syntax_result.success,
            changes: vec!["Configuration action completed".to_string()],
            output: "Configuration action completed".to_string(),
            errors: syntax_result.errors,
            warnings: syntax_result.warnings,
            duration: std::time::Duration::from_secs(1),
        })
    }

    /// Execute dependency action
    async fn execute_dependency_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing dependency action");

        // Run cargo validation
        let cargo_result = self
            .tool_registry
            .execute_validation("cargo", intent)
            .await
            .map_err(|e| anyhow::anyhow!("Cargo validation failed: {:?}", e))?;

        Ok(ToolResult {
            success: cargo_result.success,
            changes: vec!["Dependency action completed".to_string()],
            output: "Dependency action completed".to_string(),
            errors: cargo_result.errors,
            warnings: cargo_result.warnings,
            duration: std::time::Duration::from_secs(1),
        })
    }

    /// Execute cleanup action
    async fn execute_cleanup_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing cleanup action");

        // Run syntax validation
        let syntax_result = self
            .tool_registry
            .execute_safety_check("syntax_validation", intent)
            .await
            .map_err(|e| anyhow::anyhow!("Syntax validation failed: {:?}", e))?;

        Ok(ToolResult {
            success: syntax_result.success,
            changes: vec!["Cleanup action completed".to_string()],
            output: "Cleanup action completed".to_string(),
            errors: syntax_result.errors,
            warnings: syntax_result.warnings,
            duration: std::time::Duration::from_secs(1),
        })
    }

    /// Execute migration action
    async fn execute_migration_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing migration action");

        // Run comprehensive validation
        let type_result = self
            .tool_registry
            .execute_safety_check("type_checking", intent)
            .await
            .map_err(|e| anyhow::anyhow!("Type checking failed: {:?}", e))?;

        Ok(ToolResult {
            success: type_result.success,
            changes: vec!["Migration action completed".to_string()],
            output: "Migration action completed".to_string(),
            errors: type_result.errors,
            warnings: type_result.warnings,
            duration: std::time::Duration::from_secs(1),
        })
    }

    /// Execute default action (for custom types)
    async fn execute_default_action(&self, intent: &ActionIntent) -> Result<ToolResult> {
        info!("Executing default action");

        // Run basic syntax validation
        let syntax_result = self
            .tool_registry
            .execute_safety_check("syntax_validation", intent)
            .await
            .map_err(|e| anyhow::anyhow!("Syntax validation failed: {:?}", e))?;

        Ok(ToolResult {
            success: syntax_result.success,
            changes: vec!["Default action completed".to_string()],
            output: "Default action completed".to_string(),
            errors: syntax_result.errors,
            warnings: syntax_result.warnings,
            duration: std::time::Duration::from_secs(1),
        })
    }
}

/// Execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub changes: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration: std::time::Duration,
}
