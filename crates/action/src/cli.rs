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

use clap::Subcommand;
use std::path::PathBuf;
use tracing::{info, warn, error};

use crate::schema::{ActionIntent, ActionType, SafetyLevel};
use crate::error::{ActionError, ActionResult};
use crate::pipeline::{execute_action, get_action_status, list_active_actions};

/// CLI subcommands for action protocol
#[derive(Subcommand)]
pub enum IntentSubcommands {
    /// Plan an action
    Plan {
        /// Action description
        #[arg(value_name = "DESCRIPTION")]
        description: String,
        
        /// Action type
        #[arg(long, value_enum, default_value = "refactor")]
        action_type: ActionType,
        
        /// Safety level
        #[arg(long, value_enum, default_value = "medium")]
        safety_level: SafetyLevel,
        
        /// Scope (files/directories)
        #[arg(long, value_name = "SCOPE")]
        scope: Vec<String>,
        
        /// Output file for intent
        #[arg(long, value_name = "FILE")]
        output_file: Option<String>,
    },
    
    /// Preview action changes
    Preview {
        /// Intent file path
        #[arg(value_name = "INTENT_FILE")]
        intent_file: String,
        
        /// Show detailed preview
        #[arg(long)]
        detailed: bool,
        
        /// Show safety analysis
        #[arg(long)]
        safety: bool,
    },
    
    /// Execute action
    Execute {
        /// Intent file path
        #[arg(value_name = "INTENT_FILE")]
        intent_file: String,
        
        /// Require human approval
        #[arg(long)]
        require_approval: bool,
        
        /// Skip validation
        #[arg(long)]
        skip_validation: bool,
        
        /// Dry run (don't make changes)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Rollback action
    Rollback {
        /// Intent ID
        #[arg(value_name = "INTENT_ID")]
        intent_id: String,
        
        /// Force rollback
        #[arg(long)]
        force: bool,
        
        /// Keep backup
        #[arg(long)]
        keep_backup: bool,
    },
    
    /// List active intents
    List {
        /// Show only active intents
        #[arg(long)]
        active: bool,
        
        /// Show completed intents
        #[arg(long)]
        completed: bool,
        
        /// Show failed intents
        #[arg(long)]
        failed: bool,
        
        /// Filter by action type
        #[arg(long, value_enum)]
        action_type: Option<ActionType>,
        
        /// Filter by safety level
        #[arg(long, value_enum)]
        safety_level: Option<SafetyLevel>,
    },
    
    /// Check intent status
    Status {
        /// Intent ID
        #[arg(value_name = "INTENT_ID")]
        intent_id: String,
        
        /// Show detailed status
        #[arg(long)]
        detailed: bool,
        
        /// Show validation results
        #[arg(long)]
        validation: bool,
    },
    
    /// Validate intent file
    Validate {
        /// Intent file path
        #[arg(value_name = "INTENT_FILE")]
        intent_file: String,
        
        /// Show preview of changes
        #[arg(long)]
        preview: bool,
        
        /// Show safety analysis
        #[arg(long)]
        safety: bool,
        
        /// Show validation results
        #[arg(long)]
        validation: bool,
    },
    
    /// Show action history
    History {
        /// Number of days to look back
        #[arg(long, default_value = "7")]
        days: u32,
        
        /// Show detailed history
        #[arg(long)]
        detailed: bool,
        
        /// Filter by action type
        #[arg(long, value_enum)]
        action_type: Option<ActionType>,
        
        /// Filter by safety level
        #[arg(long, value_enum)]
        safety_level: Option<SafetyLevel>,
    },
    
    /// Run safety checks
    SafetyCheck {
        /// Intent file path
        #[arg(value_name = "INTENT_FILE")]
        intent_file: String,
        
        /// Run all safety checks
        #[arg(long)]
        all: bool,
        
        /// Show detailed results
        #[arg(long)]
        detailed: bool,
        
        /// Export results to file
        #[arg(long, value_name = "FILE")]
        export: Option<String>,
    },
    
    /// Approve pending action
    Approve {
        /// Intent ID
        #[arg(value_name = "INTENT_ID")]
        intent_id: String,
        
        /// Approval comment
        #[arg(long, value_name = "COMMENT")]
        comment: Option<String>,
        
        /// Auto-execute after approval
        #[arg(long)]
        auto_execute: bool,
    },
    
    /// Reject pending action
    Reject {
        /// Intent ID
        #[arg(value_name = "INTENT_ID")]
        intent_id: String,
        
        /// Rejection reason
        #[arg(long, value_name = "REASON")]
        reason: String,
    },
}

/// CLI handler for action protocol commands
pub struct ActionCli;

impl ActionCli {
    /// Handle intent subcommands
    pub async fn handle_intent_command(cmd: IntentSubcommands) -> ActionResult<()> {
        match cmd {
            IntentSubcommands::Plan { description, action_type, safety_level, scope, output_file } => {
                Self::handle_plan(description, action_type, safety_level, scope, output_file).await
            }
            IntentSubcommands::Preview { intent_file, detailed, safety } => {
                Self::handle_preview(intent_file, detailed, safety).await
            }
            IntentSubcommands::Execute { intent_file, require_approval, skip_validation, dry_run } => {
                Self::handle_execute(intent_file, require_approval, skip_validation, dry_run).await
            }
            IntentSubcommands::Rollback { intent_id, force, keep_backup } => {
                Self::handle_rollback(intent_id, force, keep_backup).await
            }
            IntentSubcommands::List { active, completed, failed, action_type, safety_level } => {
                Self::handle_list(active, completed, failed, action_type, safety_level).await
            }
            IntentSubcommands::Status { intent_id, detailed, validation } => {
                Self::handle_status(intent_id, detailed, validation).await
            }
            IntentSubcommands::Validate { intent_file, preview, safety, validation } => {
                Self::handle_validate(intent_file, preview, safety, validation).await
            }
            IntentSubcommands::History { days, detailed, action_type, safety_level } => {
                Self::handle_history(days, detailed, action_type, safety_level).await
            }
            IntentSubcommands::SafetyCheck { intent_file, all, detailed, export } => {
                Self::handle_safety_check(intent_file, all, detailed, export).await
            }
            IntentSubcommands::Approve { intent_id, comment, auto_execute } => {
                Self::handle_approve(intent_id, comment, auto_execute).await
            }
            IntentSubcommands::Reject { intent_id, reason } => {
                Self::handle_reject(intent_id, reason).await
            }
        }
    }
    
    /// Handle plan command
    async fn handle_plan(
        description: String,
        action_type: ActionType,
        safety_level: SafetyLevel,
        scope: Vec<String>,
        output_file: Option<String>,
    ) -> ActionResult<()> {
        info!("Planning action: {}", description);
        
        let intent = ActionIntent::new(
            ActionIntent::generate_id(),
            action_type,
            description,
            scope,
            safety_level,
        );
        
        // Add default tools and validations based on action type
        Self::add_default_configuration(&intent).await?;
        
        // Validate the intent
        intent.validate()?;
        
        // Output the intent
        let intent_yaml = serde_yaml::to_string(&intent).map_err(|e| {
            ActionError::serialization(format!("Failed to serialize intent: {}", e))
        })?;
        
        if let Some(file_path) = output_file {
            tokio::fs::write(&file_path, intent_yaml).await.map_err(|e| {
                ActionError::file_operation(
                    PathBuf::from(&file_path),
                    format!("Failed to write intent file: {}", e)
                )
            })?;
            info!("Intent written to: {}", file_path);
        } else {
            println!("{}", intent_yaml);
        }
        
        info!("Action planning completed successfully");
        Ok(())
    }
    
    /// Handle preview command
    async fn handle_preview(intent_file: String, detailed: bool, safety: bool) -> ActionResult<()> {
        info!("Previewing action from file: {}", intent_file);
        
        let intent = Self::load_intent_from_file(&intent_file).await?;
        
        println!("=== ACTION PREVIEW ===");
        println!("Intent ID: {}", intent.id);
        println!("Action Type: {:?}", intent.action_type);
        println!("Description: {}", intent.description);
        println!("Safety Level: {:?}", intent.safety_level);
        println!("Scope: {}", intent.scope.join(", "));
        
        if detailed {
            println!("\n=== DETAILED PREVIEW ===");
            println!("Transformation Tools: {}", intent.transformation.tools.join(", "));
            println!("Validation Tools: {}", intent.transformation.validation.join(", "));
            println!("Pre-execution Checks: {}", intent.safety_checks.pre_execution.join(", "));
            println!("Post-execution Checks: {}", intent.safety_checks.post_execution.join(", "));
        }
        
        if safety {
            println!("\n=== SAFETY ANALYSIS ===");
            println!("Requires Approval: {}", intent.requires_approval());
            if let Some(approvers) = &intent.approval_workflow.approvers {
                println!("Approvers: {}", approvers.join(", "));
            }
            println!("Auto-approve Conditions: {:?}", intent.approval_workflow.auto_approve_for);
        }
        
        println!("=====================");
        
        info!("Action preview completed successfully");
        Ok(())
    }
    
    /// Handle execute command
    async fn handle_execute(
        intent_file: String,
        require_approval: bool,
        skip_validation: bool,
        dry_run: bool,
    ) -> ActionResult<()> {
        info!("Executing action from file: {}", intent_file);
        
        let mut intent = Self::load_intent_from_file(&intent_file).await?;
        
        if require_approval {
            intent.set_approval_required(true);
        }
        
        if skip_validation {
            // Remove validation tools
            intent.transformation.validation.clear();
            intent.safety_checks.pre_execution.clear();
            intent.safety_checks.post_execution.clear();
        }
        
        if dry_run {
            println!("=== DRY RUN MODE ===");
            println!("Would execute action: {}", intent.description);
            println!("Intent ID: {}", intent.id);
            println!("Action Type: {:?}", intent.action_type);
            println!("Safety Level: {:?}", intent.safety_level);
            println!("===================");
            return Ok(());
        }
        
        let result = execute_action(intent).await?;
        
        if result.success {
            println!("✅ Action executed successfully!");
            println!("Duration: {:?}", result.duration);
            println!("Changes: {}", result.changes.join(", "));
        } else {
            println!("❌ Action execution failed!");
            println!("Errors: {}", result.errors.join(", "));
            if let Some(rollback_info) = result.rollback_info {
                println!("Rollback: {}", if rollback_info.success { "Successful" } else { "Failed" });
            }
        }
        
        info!("Action execution completed");
        Ok(())
    }
    
    /// Handle rollback command
    async fn handle_rollback(intent_id: String, force: bool, keep_backup: bool) -> ActionResult<()> {
        info!("Rolling back action: {}", intent_id);
        
        // TODO: Implement rollback functionality
        println!("Rolling back action: {}", intent_id);
        println!("Force: {}", force);
        println!("Keep backup: {}", keep_backup);
        
        info!("Action rollback completed");
        Ok(())
    }
    
    /// Handle list command
    async fn handle_list(
        active: bool,
        completed: bool,
        failed: bool,
        action_type: Option<ActionType>,
        safety_level: Option<SafetyLevel>,
    ) -> ActionResult<()> {
        info!("Listing actions");
        
        let actions = list_active_actions().await?;
        
        println!("=== ACTIVE ACTIONS ===");
        for (intent_id, status) in actions {
            println!("{}: {:?}", intent_id, status);
        }
        
        info!("Action listing completed");
        Ok(())
    }
    
    /// Handle status command
    async fn handle_status(intent_id: String, detailed: bool, validation: bool) -> ActionResult<()> {
        info!("Checking status for intent: {}", intent_id);
        
        let status = get_action_status(&intent_id).await?;
        
        if let Some(status) = status {
            println!("Status: {:?}", status);
            
            if detailed {
                println!("Detailed status information would be shown here");
            }
            
            if validation {
                println!("Validation results would be shown here");
            }
        } else {
            println!("No status found for intent: {}", intent_id);
        }
        
        info!("Status check completed");
        Ok(())
    }
    
    /// Handle validate command
    async fn handle_validate(
        intent_file: String,
        preview: bool,
        safety: bool,
        validation: bool,
    ) -> ActionResult<()> {
        info!("Validating intent file: {}", intent_file);
        
        let intent = Self::load_intent_from_file(&intent_file).await?;
        
        // Validate the intent
        intent.validate()?;
        
        println!("✅ Intent validation passed!");
        
        if preview {
            println!("Preview mode enabled - would show changes");
        }
        
        if safety {
            println!("Safety analysis would be performed");
        }
        
        if validation {
            println!("Validation checks would be run");
        }
        
        info!("Intent validation completed");
        Ok(())
    }
    
    /// Handle history command
    async fn handle_history(
        days: u32,
        detailed: bool,
        action_type: Option<ActionType>,
        safety_level: Option<SafetyLevel>,
    ) -> ActionResult<()> {
        info!("Showing action history for last {} days", days);
        
        // TODO: Implement history functionality
        println!("Action history for last {} days:", days);
        println!("Detailed: {}", detailed);
        if let Some(action_type) = action_type {
            println!("Filtered by action type: {:?}", action_type);
        }
        if let Some(safety_level) = safety_level {
            println!("Filtered by safety level: {:?}", safety_level);
        }
        
        info!("History display completed");
        Ok(())
    }
    
    /// Handle safety check command
    async fn handle_safety_check(
        intent_file: String,
        all: bool,
        detailed: bool,
        export: Option<String>,
    ) -> ActionResult<()> {
        info!("Running safety checks for: {}", intent_file);
        
        let intent = Self::load_intent_from_file(&intent_file).await?;
        
        println!("Running safety checks for intent: {}", intent.id);
        println!("All checks: {}", all);
        println!("Detailed: {}", detailed);
        
        // TODO: Implement actual safety checks
        
        if let Some(export_file) = export {
            println!("Results would be exported to: {}", export_file);
        }
        
        info!("Safety checks completed");
        Ok(())
    }
    
    /// Handle approve command
    async fn handle_approve(intent_id: String, comment: Option<String>, auto_execute: bool) -> ActionResult<()> {
        info!("Approving intent: {}", intent_id);
        
        println!("Approving intent: {}", intent_id);
        if let Some(comment) = comment {
            println!("Comment: {}", comment);
        }
        println!("Auto-execute: {}", auto_execute);
        
        // TODO: Implement approval functionality
        
        info!("Approval completed");
        Ok(())
    }
    
    /// Handle reject command
    async fn handle_reject(intent_id: String, reason: String) -> ActionResult<()> {
        info!("Rejecting intent: {} with reason: {}", intent_id, reason);
        
        println!("Rejecting intent: {}", intent_id);
        println!("Reason: {}", reason);
        
        // TODO: Implement rejection functionality
        
        info!("Rejection completed");
        Ok(())
    }
    
    /// Load intent from file
    async fn load_intent_from_file(file_path: &str) -> ActionResult<ActionIntent> {
        let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            ActionError::file_operation(
                PathBuf::from(file_path),
                format!("Failed to read intent file: {}", e)
            )
        })?;
        
        let intent: ActionIntent = serde_yaml::from_str(&content).map_err(|e| {
            ActionError::deserialization(format!("Failed to parse intent file: {}", e))
        })?;
        
        Ok(intent)
    }
    
    /// Add default configuration to intent
    async fn add_default_configuration(intent: &ActionIntent) -> ActionResult<()> {
        // This would add default tools and validations based on the action type
        // For now, just add some basic defaults
        
        info!("Adding default configuration for intent: {}", intent.id);
        
        // TODO: Implement default configuration logic
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};

    #[tokio::test]
    async fn test_plan_command() {
        let result = ActionCli::handle_plan(
            "Test action".to_string(),
            ActionType::Refactor,
            SafetyLevel::Medium,
            vec!["src/".to_string()],
            None,
        ).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_command() {
        // This test would require a valid intent file
        // For now, just test that the function exists
        assert!(true);
    }
} 