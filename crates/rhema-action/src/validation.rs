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

use std::sync::Arc;
use tracing::{info, warn, error};
use anyhow::Result;

use crate::tools::ToolRegistry;
use crate::schema::{ActionIntent as SchemaActionIntent, ActionType, SafetyLevel};
use rhema_action_tool::{ActionIntent, ActionResult, ActionError, ToolResult};

/// Action validation manager
pub struct ActionValidator {
    tool_registry: Arc<ToolRegistry>,
}

impl ActionValidator {
    /// Create a new action validator
    pub async fn new() -> Result<Self> {
        info!("Initializing Action Validator");
        
        let tool_registry = Arc::new(ToolRegistry::new().await.map_err(|e| {
            anyhow::anyhow!("Failed to initialize tool registry: {:?}", e)
        })?);
        
        info!("Action Validator initialized successfully");
        Ok(Self { tool_registry })
    }
    
    /// Validate an action intent
    pub async fn validate_action(&self, intent: &SchemaActionIntent) -> Result<ValidationResult> {
        info!("Validating action intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        let mut validation_errors = Vec::new();
        let mut validation_warnings = Vec::new();
        
        // Convert schema intent to shared intent
        let shared_intent = self.convert_to_shared_intent(intent);
        
        // Run validation based on action type
        match intent.action_type {
            ActionType::Refactor => {
                self.validate_refactor_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::BugFix => {
                self.validate_bugfix_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Feature => {
                self.validate_feature_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Security => {
                self.validate_security_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Performance => {
                self.validate_performance_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Documentation => {
                // Documentation actions typically don't need extensive validation
                info!("Documentation action validation skipped");
            },
            ActionType::Test => {
                self.validate_test_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Configuration => {
                self.validate_configuration_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Dependency => {
                self.validate_dependency_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Cleanup => {
                self.validate_cleanup_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Migration => {
                self.validate_migration_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
            ActionType::Custom(_) => {
                // Custom actions use default validation
                self.validate_default_action(&shared_intent, &mut validation_errors, &mut validation_warnings).await?;
            },
        }
        
        let success = validation_errors.is_empty();
        let duration = start.elapsed();
        
        let errors_count = validation_errors.len();
        let result = ValidationResult {
            success,
            errors: validation_errors,
            warnings: validation_warnings,
            duration,
        };
        
        if success {
            info!("Action validation completed successfully in {:?}", duration);
        } else {
            warn!("Action validation completed with {} errors in {:?}", errors_count, duration);
        }
        
        Ok(result)
    }
    
    /// Validate refactor action
    async fn validate_refactor_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating refactor action");
        
        // Run TypeScript validation
        let tool_result = self.tool_registry.execute_validation("typescript", intent).await.map_err(|e| {
            anyhow::anyhow!("TypeScript validation failed: {:?}", e)
        })?;
        
        if !tool_result.success {
            errors.extend(tool_result.errors);
        }
        warnings.extend(tool_result.warnings);
        
        // Run syntax validation
        let syntax_result = self.tool_registry.execute_safety_check("syntax_validation", intent).await.map_err(|e| {
            anyhow::anyhow!("Syntax validation failed: {:?}", e)
        })?;
        
        if !syntax_result.success {
            errors.extend(syntax_result.errors);
        }
        warnings.extend(syntax_result.warnings);
        
        Ok(())
    }
    
    /// Validate bugfix action
    async fn validate_bugfix_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating bugfix action");
        
        // Run Jest tests
        let jest_result = self.tool_registry.execute_validation("jest", intent).await.map_err(|e| {
            anyhow::anyhow!("Jest validation failed: {:?}", e)
        })?;
        
        if !jest_result.success {
            errors.extend(jest_result.errors);
        }
        warnings.extend(jest_result.warnings);
        
        // Run TypeScript validation
        let ts_result = self.tool_registry.execute_validation("typescript", intent).await.map_err(|e| {
            anyhow::anyhow!("TypeScript validation failed: {:?}", e)
        })?;
        
        if !ts_result.success {
            errors.extend(ts_result.errors);
        }
        warnings.extend(ts_result.warnings);
        
        Ok(())
    }
    
    /// Validate feature action
    async fn validate_feature_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating feature action");
        
        // Run comprehensive validation
        let validations = vec![
            ("typescript", "TypeScript validation"),
            ("jest", "Jest tests"),
            ("syntax_validation", "Syntax validation"),
        ];
        
        for (tool_name, description) in validations {
            match self.tool_registry.execute_validation(tool_name, intent).await {
                Ok(result) => {
                    if !result.success {
                        errors.extend(result.errors);
                    }
                    warnings.extend(result.warnings);
                },
                Err(e) => {
                    error!("{} failed: {:?}", description, e);
                    errors.push(format!("{} failed: {:?}", description, e));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate security action
    async fn validate_security_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating security action");
        
        // Run security scanning
        let security_result = self.tool_registry.execute_safety_check("security_scanning", intent).await.map_err(|e| {
            anyhow::anyhow!("Security scanning failed: {:?}", e)
        })?;
        
        if !security_result.success {
            errors.extend(security_result.errors);
        }
        warnings.extend(security_result.warnings);
        
        // Run syntax validation
        let syntax_result = self.tool_registry.execute_safety_check("syntax_validation", intent).await.map_err(|e| {
            anyhow::anyhow!("Syntax validation failed: {:?}", e)
        })?;
        
        if !syntax_result.success {
            errors.extend(syntax_result.errors);
        }
        warnings.extend(syntax_result.warnings);
        
        Ok(())
    }
    
    /// Validate performance action
    async fn validate_performance_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating performance action");
        
        // Run type checking
        let type_result = self.tool_registry.execute_safety_check("type_checking", intent).await.map_err(|e| {
            anyhow::anyhow!("Type checking failed: {:?}", e)
        })?;
        
        if !type_result.success {
            errors.extend(type_result.errors);
        }
        warnings.extend(type_result.warnings);
        
        // Run test coverage
        let coverage_result = self.tool_registry.execute_safety_check("test_coverage", intent).await.map_err(|e| {
            anyhow::anyhow!("Test coverage check failed: {:?}", e)
        })?;
        
        if !coverage_result.success {
            errors.extend(coverage_result.errors);
        }
        warnings.extend(coverage_result.warnings);
        
        Ok(())
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
                ActionType::Documentation => rhema_action_tool::ActionType::Custom("documentation".to_string()),
                ActionType::Test => rhema_action_tool::ActionType::Test,
                ActionType::Configuration => rhema_action_tool::ActionType::Custom("configuration".to_string()),
                ActionType::Dependency => rhema_action_tool::ActionType::Custom("dependency".to_string()),
                ActionType::Cleanup => rhema_action_tool::ActionType::Custom("cleanup".to_string()),
                ActionType::Migration => rhema_action_tool::ActionType::Custom("migration".to_string()),
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
            metadata: serde_json::to_value(intent.metadata.as_ref()).unwrap_or(serde_json::Value::Null),
            context_refs: intent.context_refs.as_ref().map(|refs| {
                refs.iter().map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)).collect()
            }),
            transformation: serde_json::to_value(&intent.transformation).unwrap_or(serde_json::Value::Null),
            safety_checks: serde_json::to_value(&intent.safety_checks).unwrap_or(serde_json::Value::Null),
            approval_workflow: serde_json::to_value(&intent.approval_workflow).unwrap_or(serde_json::Value::Null),
            created_by: intent.created_by.clone(),
            tags: intent.tags.clone(),
            priority: intent.priority.clone(),
            estimated_effort: intent.estimated_effort.clone(),
            dependencies: intent.dependencies.clone(),
        }
    }

    /// Validate test action
    async fn validate_test_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating test action");
        
        // Run Jest tests
        let jest_result = self.tool_registry.execute_validation("jest", intent).await.map_err(|e| {
            anyhow::anyhow!("Jest validation failed: {:?}", e)
        })?;
        
        if !jest_result.success {
            errors.extend(jest_result.errors);
        }
        warnings.extend(jest_result.warnings);
        
        Ok(())
    }

    /// Validate configuration action
    async fn validate_configuration_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating configuration action");
        
        // Run syntax validation
        let syntax_result = self.tool_registry.execute_safety_check("syntax_validation", intent).await.map_err(|e| {
            anyhow::anyhow!("Syntax validation failed: {:?}", e)
        })?;
        
        if !syntax_result.success {
            errors.extend(syntax_result.errors);
        }
        warnings.extend(syntax_result.warnings);
        
        Ok(())
    }

    /// Validate dependency action
    async fn validate_dependency_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating dependency action");
        
        // Run cargo validation
        let cargo_result = self.tool_registry.execute_validation("cargo", intent).await.map_err(|e| {
            anyhow::anyhow!("Cargo validation failed: {:?}", e)
        })?;
        
        if !cargo_result.success {
            errors.extend(cargo_result.errors);
        }
        warnings.extend(cargo_result.warnings);
        
        Ok(())
    }

    /// Validate cleanup action
    async fn validate_cleanup_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating cleanup action");
        
        // Run syntax validation
        let syntax_result = self.tool_registry.execute_safety_check("syntax_validation", intent).await.map_err(|e| {
            anyhow::anyhow!("Syntax validation failed: {:?}", e)
        })?;
        
        if !syntax_result.success {
            errors.extend(syntax_result.errors);
        }
        warnings.extend(syntax_result.warnings);
        
        Ok(())
    }

    /// Validate migration action
    async fn validate_migration_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating migration action");
        
        // Run comprehensive validation
        let type_result = self.tool_registry.execute_safety_check("type_checking", intent).await.map_err(|e| {
            anyhow::anyhow!("Type checking failed: {:?}", e)
        })?;
        
        if !type_result.success {
            errors.extend(type_result.errors);
        }
        warnings.extend(type_result.warnings);
        
        Ok(())
    }

    /// Validate default action (for custom types)
    async fn validate_default_action(
        &self,
        intent: &ActionIntent,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>
    ) -> Result<()> {
        info!("Validating default action");
        
        // Run basic syntax validation
        let syntax_result = self.tool_registry.execute_safety_check("syntax_validation", intent).await.map_err(|e| {
            anyhow::anyhow!("Syntax validation failed: {:?}", e)
        })?;
        
        if !syntax_result.success {
            errors.extend(syntax_result.errors);
        }
        warnings.extend(syntax_result.warnings);
        
        Ok(())
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration: std::time::Duration,
} 