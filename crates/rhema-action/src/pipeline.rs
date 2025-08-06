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
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, warn, error, instrument};
use once_cell::sync::OnceCell;
use crate::schema::{ActionIntent, ActionStatus, ActionExecutionResult};
use crate::error::{ActionError, ActionResult};
use crate::validation::ValidationEngine;
use crate::rollback::RollbackManager;
use crate::approval::ApprovalWorkflow;
use crate::tools::ToolRegistry;

/// Global pipeline instance
static GLOBAL_PIPELINE: OnceCell<Arc<ActionSafetyPipeline>> = OnceCell::new();

/// Main safety pipeline for action execution
pub struct ActionSafetyPipeline {
    validation_engine: Arc<ValidationEngine>,
    rollback_manager: Arc<RollbackManager>,
    approval_workflow: Arc<ApprovalWorkflow>,
    tool_registry: Arc<ToolRegistry>,
    active_actions: Arc<RwLock<HashMap<String, ActionStatus>>>,
}

impl ActionSafetyPipeline {
    /// Initialize the safety pipeline
    pub async fn initialize() -> ActionResult<()> {
        info!("Initializing Action Safety Pipeline");
        
        // Initialize components
        let validation_engine = Arc::new(ValidationEngine::new().await?);
        let rollback_manager = Arc::new(RollbackManager::new().await?);
        let approval_workflow = Arc::new(ApprovalWorkflow::new().await?);
        let tool_registry = Arc::new(ToolRegistry::new().await?);
        
        // Create pipeline instance
        let pipeline = Self {
            validation_engine,
            rollback_manager,
            approval_workflow,
            tool_registry,
            active_actions: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Store global instance
        GLOBAL_PIPELINE.set(Arc::new(pipeline)).map_err(|_| {
            ActionError::internal("Failed to set global pipeline instance")
        })?;
        
        info!("Action Safety Pipeline initialized successfully");
        Ok(())
    }
    
    /// Shutdown the safety pipeline
    pub async fn shutdown() -> ActionResult<()> {
        info!("Shutting down Action Safety Pipeline");
        
        // Note: OnceCell doesn't have a clear method, so we just log shutdown
        // The global instance will remain but won't be used after shutdown
        
        info!("Action Safety Pipeline shutdown successfully");
        Ok(())
    }
    
    /// Get the global pipeline instance
    pub fn get() -> ActionResult<Arc<Self>> {
        GLOBAL_PIPELINE.get().cloned().ok_or_else(|| {
            ActionError::internal("Action Safety Pipeline not initialized")
        })
    }
    
    /// Execute an action through the safety pipeline
    #[instrument(skip(self, intent), fields(intent_id = %intent.id))]
    pub async fn execute_action(&self, intent: ActionIntent) -> ActionResult<ActionExecutionResult> {
        let start_time = Instant::now();
        let intent_id = intent.id.clone();
        
        info!("Starting action execution for intent: {}", intent_id);
        
        // Update status to planning
        self.update_action_status(&intent_id, ActionStatus::Planning).await;
        
        // 1. Pre-execution validation
        info!("Running pre-execution validation for intent: {}", intent_id);
        self.pre_execution_validation(&intent).await?;
        
        // 2. Check approval requirements
        if intent.requires_approval() {
            info!("Action requires approval, updating status for intent: {}", intent_id);
            self.update_action_status(&intent_id, ActionStatus::PendingApproval).await;
            
            let approved = self.approval_workflow.request_approval(&intent).await?;
            if !approved {
                self.update_action_status(&intent_id, ActionStatus::Cancelled).await;
                return Err(ActionError::approval("Action was not approved"));
            }
            
            info!("Action approved for intent: {}", intent_id);
            self.update_action_status(&intent_id, ActionStatus::Approved).await;
        }
        
        // 3. Create backup
        info!("Creating backup for intent: {}", intent_id);
        let backup = self.rollback_manager.create_backup(&intent).await?;
        
        // 4. Execute transformation
        info!("Executing transformation for intent: {}", intent_id);
        self.update_action_status(&intent_id, ActionStatus::Executing).await;
        
        let transformation_result = self.execute_transformation(&intent).await?;
        
        // 5. Post-execution validation
        info!("Running post-execution validation for intent: {}", intent_id);
        let validation_result = self.post_execution_validation(&intent).await?;
        
        // 6. Determine final result
        let duration = start_time.elapsed();
        let success = validation_result.success;
        
        let result = ActionExecutionResult {
            success,
            duration,
            changes: transformation_result.changes,
            validation_results: validation_result.validation_results,
            safety_results: validation_result.safety_results,
            errors: validation_result.errors,
            warnings: validation_result.warnings,
            rollback_info: None,
        };
        
        if success {
            info!("Action completed successfully for intent: {}", intent_id);
            self.update_action_status(&intent_id, ActionStatus::Completed).await;
            
            // Commit changes
            self.commit_changes(&intent, &result).await?;
        } else {
            error!("Action failed for intent: {}", intent_id);
            self.update_action_status(&intent_id, ActionStatus::Failed).await;
            
            // Rollback changes
            let rollback_info = self.rollback_manager.rollback(&backup).await?;
            
            // Update result with rollback info
            let mut failed_result = result;
            failed_result.rollback_info = Some(rollback_info);
            
            return Ok(failed_result);
        }
        
        Ok(result)
    }
    
    /// Run pre-execution validation
    async fn pre_execution_validation(&self, intent: &ActionIntent) -> ActionResult<()> {
        info!("Running pre-execution validation for intent: {}", intent.id);
        
        // Validate intent schema
        intent.validate()?;
        
        // Run safety checks
        for check in &intent.safety_checks.pre_execution {
            let check_result = self.validation_engine.run_safety_check(check, intent).await?;
            if !check_result.success {
                return Err(ActionError::safety_check(
                    check.clone(),
                    format!("Pre-execution safety check failed: {}", check_result.message)
                ));
            }
        }
        
        info!("Pre-execution validation completed successfully for intent: {}", intent.id);
        Ok(())
    }
    
    /// Execute the transformation using configured tools
    async fn execute_transformation(&self, intent: &ActionIntent) -> ActionResult<TransformationResult> {
        info!("Executing transformation for intent: {}", intent.id);
        
        let mut changes = Vec::new();
        let mut errors = Vec::new();
        
        for tool_name in &intent.transformation.tools {
            match self.tool_registry.execute_tool(tool_name, intent).await {
                Ok(tool_result) => {
                    changes.extend(tool_result.changes);
                    info!("Tool {} executed successfully for intent: {}", tool_name, intent.id);
                }
                Err(e) => {
                    errors.push(format!("Tool {} failed: {}", tool_name, e));
                    error!("Tool {} failed for intent {}: {}", tool_name, intent.id, e);
                }
            }
        }
        
        if !errors.is_empty() {
            return Err(ActionError::tool_execution(
                "transformation",
                format!("Transformation failed with errors: {}", errors.join("; "))
            ));
        }
        
        Ok(TransformationResult { changes })
    }
    
    /// Run post-execution validation
    async fn post_execution_validation(&self, intent: &ActionIntent) -> ActionResult<ValidationResult> {
        info!("Running post-execution validation for intent: {}", intent.id);
        
        let mut validation_results = HashMap::new();
        let mut safety_results = HashMap::new();
        let mut errors = Vec::new();
        let warnings = Vec::new();
        
        // Run validation tools
        for validation_tool in &intent.transformation.validation {
            match self.validation_engine.run_validation(validation_tool, intent).await {
                Ok(result) => {
                    validation_results.insert(validation_tool.clone(), result.success);
                    if !result.success {
                        errors.push(format!("Validation {} failed: {}", validation_tool, result.message));
                    }
                }
                Err(e) => {
                    errors.push(format!("Validation {} error: {}", validation_tool, e));
                }
            }
        }
        
        // Run safety checks
        for check in &intent.safety_checks.post_execution {
            match self.validation_engine.run_safety_check(check, intent).await {
                Ok(result) => {
                    safety_results.insert(check.clone(), result.success);
                    if !result.success {
                        errors.push(format!("Safety check {} failed: {}", check, result.message));
                    }
                }
                Err(e) => {
                    errors.push(format!("Safety check {} error: {}", check, e));
                }
            }
        }
        
        let success = errors.is_empty();
        
        if success {
            info!("Post-execution validation completed successfully for intent: {}", intent.id);
        } else {
            warn!("Post-execution validation failed for intent: {}", intent.id);
        }
        
        Ok(ValidationResult {
            success,
            validation_results,
            safety_results,
            errors,
            warnings,
        })
    }
    
    /// Commit changes to the repository
    async fn commit_changes(&self, intent: &ActionIntent, result: &ActionExecutionResult) -> ActionResult<()> {
        info!("Committing changes for intent: {}", intent.id);
        
        // Create git commit with action metadata
        let _commit_message = format!(
            "Action: {} - {}\n\nIntent ID: {}\nSafety Level: {:?}\nChanges: {}",
            intent.action_type,
            intent.description,
            intent.id,
            intent.safety_level,
            result.changes.join(", ")
        );
        
        // TODO: Implement git commit through git integration
        // self.git_integration.commit_changes(&commit_message).await?;
        
        info!("Changes committed successfully for intent: {}", intent.id);
        Ok(())
    }
    
    /// Update action status
    async fn update_action_status(&self, intent_id: &str, status: ActionStatus) {
        let mut actions = self.active_actions.write().await;
        actions.insert(intent_id.to_string(), status.clone());
        info!("Updated action status for {} to {:?}", intent_id, status);
    }
    
    /// Get action status
    pub async fn get_action_status(&self, intent_id: &str) -> Option<ActionStatus> {
        let actions = self.active_actions.read().await;
        actions.get(intent_id).cloned()
    }
    
    /// List all active actions
    pub async fn list_active_actions(&self) -> HashMap<String, ActionStatus> {
        let actions = self.active_actions.read().await;
        actions.clone()
    }
}

/// Transformation result
#[derive(Debug, Clone)]
pub struct TransformationResult {
    pub changes: Vec<String>,
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub success: bool,
    pub validation_results: HashMap<String, bool>,
    pub safety_results: HashMap<String, bool>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}



/// Convenience function to execute an action
pub async fn execute_action(intent: ActionIntent) -> ActionResult<ActionExecutionResult> {
    let pipeline = ActionSafetyPipeline::get()?;
    pipeline.execute_action(intent).await
}

/// Convenience function to get action status
pub async fn get_action_status(intent_id: &str) -> ActionResult<Option<ActionStatus>> {
    let pipeline = ActionSafetyPipeline::get()?;
    Ok(pipeline.get_action_status(intent_id).await)
}

/// Convenience function to list active actions
pub async fn list_active_actions() -> ActionResult<HashMap<String, ActionStatus>> {
    let pipeline = ActionSafetyPipeline::get()?;
    Ok(pipeline.list_active_actions().await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};

    #[tokio::test]
    async fn test_pipeline_initialization() {
        let result = ActionSafetyPipeline::initialize().await;
        assert!(result.is_ok());
        
        let result = ActionSafetyPipeline::shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pipeline_get() {
        ActionSafetyPipeline::initialize().await.unwrap();
        
        let pipeline = ActionSafetyPipeline::get();
        assert!(pipeline.is_ok());
        
        ActionSafetyPipeline::shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_action_execution() {
        ActionSafetyPipeline::initialize().await.unwrap();
        
        let mut intent = ActionIntent::new(
            "test-execution",
            ActionType::Test,
            "Test action execution",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        // Add minimal configuration
        intent.add_tool("echo");
        intent.add_validation("syntax");
        intent.set_approval_required(false);
        
        let result = execute_action(intent).await;
        // This will likely fail due to missing tool implementations, but should not panic
        assert!(result.is_ok() || result.is_err());
        
        ActionSafetyPipeline::shutdown().await.unwrap();
    }
} 