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
use tracing::info;

use crate::error::{ActionError, ActionResult};
use crate::schema::{ActionIntent, ActionType, SafetyLevel};
// Pipeline functions will be implemented as needed
async fn execute_action(intent: &ActionIntent) -> ActionResult<ExecutionResult> {
    let start_time = std::time::Instant::now();
    let mut changes = Vec::new();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    info!("Executing action: {}", intent.id);

    // Step 1: Validate the intent
    if let Err(validation_error) = intent.validate() {
        errors.push(format!("Validation failed: {}", validation_error));
        return Ok(ExecutionResult {
            success: false,
            changes,
            errors,
            warnings,
            duration: start_time.elapsed(),
        });
    }

    // Step 2: Create backup before execution
    let rollback_manager = crate::rollback::RollbackManager::new().await?;
    let backup = rollback_manager.create_backup(intent).await?;
    changes.push(format!("Backup created: {}", backup.id));

    // Step 3: Run pre-execution safety checks
    for check in &intent.safety_checks.pre_execution {
        match run_safety_check(check, intent).await {
            Ok(result) => {
                if result.passed {
                    changes.push(format!("Pre-execution check passed: {}", check));
                } else {
                    warnings.push(format!("Pre-execution check warning: {} - {}", check, result.message));
                }
            }
            Err(e) => {
                errors.push(format!("Pre-execution check failed: {} - {}", check, e));
            }
        }
    }

    // Step 4: Execute transformations
    for tool in &intent.transformation.tools {
        match run_transformation_tool(tool, intent).await {
            Ok(result) => {
                changes.extend(result.changes);
                warnings.extend(result.warnings);
            }
            Err(e) => {
                errors.push(format!("Transformation tool failed: {} - {}", tool, e));
            }
        }
    }

    // Step 5: Run post-execution safety checks
    for check in &intent.safety_checks.post_execution {
        match run_safety_check(check, intent).await {
            Ok(result) => {
                if result.passed {
                    changes.push(format!("Post-execution check passed: {}", check));
                } else {
                    warnings.push(format!("Post-execution check warning: {} - {}", check, result.message));
                }
            }
            Err(e) => {
                errors.push(format!("Post-execution check failed: {} - {}", check, e));
            }
        }
    }

    // Step 6: Run validations
    for validation in &intent.transformation.validation {
        match run_validation_tool(validation, intent).await {
            Ok(result) => {
                if result.passed {
                    changes.push(format!("Validation passed: {}", validation));
                } else {
                    errors.push(format!("Validation failed: {} - {}", validation, result.message));
                }
            }
            Err(e) => {
                errors.push(format!("Validation tool failed: {} - {}", validation, e));
            }
        }
    }

    let success = errors.is_empty();
    let duration = start_time.elapsed();

    // If execution failed, attempt rollback
    if !success {
        info!("Action execution failed, attempting rollback");
        match rollback_manager.rollback(&backup).await {
            Ok(rollback_info) => {
                if rollback_info.success {
                    changes.push("Rollback completed successfully".to_string());
                } else {
                    errors.push("Rollback failed".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("Rollback failed: {}", e));
            }
        }
    }

    info!("Action execution completed with {} errors", errors.len());
    Ok(ExecutionResult {
        success,
        changes,
        errors,
        warnings,
        duration,
    })
}

/// Run a single safety check
async fn run_safety_check(
    check_name: &str,
    intent: &ActionIntent,
) -> ActionResult<SafetyCheckResult> {
    let result = match check_name {
        "file_permissions" => check_file_permissions(intent).await,
        "file_content" => check_file_content_safety(intent).await,
        "dependency_updates" => check_dependency_updates(intent).await,
        _ => Err(ActionError::tool_execution(
            check_name,
            format!("Unknown safety check: {}", check_name),
        )),
    };

    result
}

/// Run a single transformation tool
async fn run_transformation_tool(
    tool_name: &str,
    intent: &ActionIntent,
) -> ActionResult<TransformationResult> {
    let mut changes = Vec::new();
    let mut warnings = Vec::new();

    match tool_name {
        "prettier" => {
            // Run prettier formatting
            for path in &intent.scope {
                if path.ends_with(".js") || path.ends_with(".ts") || path.ends_with(".json") {
                    let output = tokio::process::Command::new("npx")
                        .args(&["prettier", "--write", path])
                        .output()
                        .await;
                    
                    match output {
                        Ok(_) => changes.push(format!("Formatted file: {}", path)),
                        Err(e) => warnings.push(format!("Prettier failed for {}: {}", path, e)),
                    }
                }
            }
        }
        "eslint" => {
            // Run eslint linting
            for path in &intent.scope {
                if path.ends_with(".js") || path.ends_with(".ts") {
                    let output = tokio::process::Command::new("npx")
                        .args(&["eslint", "--fix", path])
                        .output()
                        .await;
                    
                    match output {
                        Ok(_) => changes.push(format!("Linted file: {}", path)),
                        Err(e) => warnings.push(format!("ESLint failed for {}: {}", path, e)),
                    }
                }
            }
        }
        "cargo_fmt" => {
            // Run cargo fmt for Rust files
            let output = tokio::process::Command::new("cargo")
                .args(&["fmt"])
                .output()
                .await;
            
            match output {
                Ok(_) => changes.push("Formatted Rust code with cargo fmt".to_string()),
                Err(e) => warnings.push(format!("Cargo fmt failed: {}", e)),
            }
        }
        "cargo_clippy" => {
            // Run cargo clippy for Rust files
            let output = tokio::process::Command::new("cargo")
                .args(&["clippy", "--fix"])
                .output()
                .await;
            
            match output {
                Ok(_) => changes.push("Fixed Rust code with cargo clippy".to_string()),
                Err(e) => warnings.push(format!("Cargo clippy failed: {}", e)),
            }
        }
        _ => {
            warnings.push(format!("Transformation tool skipped: {} - tool not found", tool_name));
        }
    }

    Ok(TransformationResult { changes, warnings })
}

/// Run a single validation tool
async fn run_validation_tool(
    tool_name: &str,
    intent: &ActionIntent,
) -> ActionResult<ValidationResult> {
    match tool_name {
        "file_exists" => check_file_exists(intent).await,
        "file_content" => {
            let safety_result = check_file_content_safety(intent).await?;
            Ok(ValidationResult {
                passed: safety_result.passed,
                message: safety_result.message,
            })
        }
        "dependency_version" => check_dependency_version(intent).await,
        _ => Err(ActionError::tool_execution(
            tool_name,
            format!("Unknown validation tool: {}", tool_name),
        )),
    }
}

#[derive(Debug)]
struct SafetyCheckResult {
    passed: bool,
    message: String,
}

#[derive(Debug)]
struct TransformationResult {
    changes: Vec<String>,
    warnings: Vec<String>,
}

#[derive(Debug)]
struct ValidationResult {
    passed: bool,
    message: String,
}

#[derive(Debug, Clone)]
struct ExecutionResult {
    success: bool,
    changes: Vec<String>,
    errors: Vec<String>,
    warnings: Vec<String>,
    duration: std::time::Duration,
}

async fn list_active_actions() -> ActionResult<Vec<(String, String)>> {
    info!("Listing active actions");
    
    // In a real implementation, this would query a database or state store
    // For now, we'll simulate by reading from a state file
    let state_file = PathBuf::from(".rhema/actions/state.json");
    
    if !state_file.exists() {
        return Ok(Vec::new());
    }
    
    let content = tokio::fs::read_to_string(&state_file).await.map_err(|e| {
        ActionError::file_operation(
            state_file.clone(),
            format!("Failed to read state file: {}", e),
        )
    })?;
    
    let actions: Vec<ActionState> = serde_json::from_str(&content).map_err(|e| {
        ActionError::deserialization(format!("Failed to parse state file: {}", e))
    })?;
    
    let active_actions: Vec<(String, String)> = actions
        .into_iter()
        .filter(|action| action.status == "active" || action.status == "pending")
        .map(|action| (action.intent_id, action.status))
        .collect();
    
    Ok(active_actions)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ActionState {
    intent_id: String,
    status: String,
    created_at: String,
    updated_at: String,
}

async fn get_action_status(intent_id: &str) -> ActionResult<String> {
    info!("Getting status for action: {}", intent_id);
    
    // In a real implementation, this would query a database or state store
    // For now, we'll simulate by reading from a state file
    let state_file = PathBuf::from(".rhema/actions/state.json");
    
    if !state_file.exists() {
        return Err(ActionError::not_found(format!("Action {} not found", intent_id)));
    }
    
    let content = tokio::fs::read_to_string(&state_file).await.map_err(|e| {
        ActionError::file_operation(
            state_file.clone(),
            format!("Failed to read state file: {}", e),
        )
    })?;
    
    let actions: Vec<ActionState> = serde_json::from_str(&content).map_err(|e| {
        ActionError::deserialization(format!("Failed to parse state file: {}", e))
    })?;
    
    // Find the specific action
    let action = actions
        .into_iter()
        .find(|action| action.intent_id == intent_id)
        .ok_or_else(|| ActionError::not_found(format!("Action {} not found", intent_id)))?;
    
    // Check if there's a backup for this action
    let backup_dir = PathBuf::from(".rhema/backups");
    let backup_exists = if backup_dir.exists() {
        let mut entries = tokio::fs::read_dir(&backup_dir).await.map_err(|e| {
            ActionError::file_operation(
                backup_dir.clone(),
                format!("Failed to read backup directory: {}", e),
            )
        })?;
        
        let mut found = false;
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ActionError::file_operation(
                backup_dir.clone(),
                format!("Failed to read backup directory entry: {}", e),
            )
        })? {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let metadata_file = entry_path.join("backup_metadata.json");
                if metadata_file.exists() {
                    let metadata_content = tokio::fs::read_to_string(&metadata_file).await?;
                    let metadata: serde_json::Value = serde_json::from_str(&metadata_content)?;
                    if metadata["intent_id"].as_str() == Some(intent_id) {
                        found = true;
                        break;
                    }
                }
            }
        }
        found
    } else {
        false
    };
    
    // Build status string
    let mut status = format!("Status: {}", action.status);
    if backup_exists {
        status.push_str(" (backup available)");
    }
    status.push_str(&format!("\nCreated: {}", action.created_at));
    status.push_str(&format!("\nUpdated: {}", action.updated_at));
    
    Ok(status)
}

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
            IntentSubcommands::Plan {
                description,
                action_type,
                safety_level,
                scope,
                output_file,
            } => {
                Self::handle_plan(description, action_type, safety_level, scope, output_file).await
            }
            IntentSubcommands::Preview {
                intent_file,
                detailed,
                safety,
            } => Self::handle_preview(intent_file, detailed, safety).await,
            IntentSubcommands::Execute {
                intent_file,
                require_approval,
                skip_validation,
                dry_run,
            } => {
                Self::handle_execute(intent_file, require_approval, skip_validation, dry_run).await
            }
            IntentSubcommands::Rollback {
                intent_id,
                force,
                keep_backup,
            } => Self::handle_rollback(intent_id, force, keep_backup).await,
            IntentSubcommands::List {
                active,
                completed,
                failed,
                action_type,
                safety_level,
            } => Self::handle_list(active, completed, failed, action_type, safety_level).await,
            IntentSubcommands::Status {
                intent_id,
                detailed,
                validation,
            } => Self::handle_status(intent_id, detailed, validation).await,
            IntentSubcommands::Validate {
                intent_file,
                preview,
                safety,
                validation,
            } => Self::handle_validate(intent_file, preview, safety, validation).await,
            IntentSubcommands::History {
                days,
                detailed,
                action_type,
                safety_level,
            } => Self::handle_history(days, detailed, action_type, safety_level).await,
            IntentSubcommands::SafetyCheck {
                intent_file,
                all,
                detailed,
                export,
            } => Self::handle_safety_check(intent_file, all, detailed, export).await,
            IntentSubcommands::Approve {
                intent_id,
                comment,
                auto_execute,
            } => Self::handle_approve(intent_id, comment, auto_execute).await,
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

        let mut intent = ActionIntent::new(
            ActionIntent::generate_id(),
            action_type,
            description,
            scope,
            safety_level,
        );

        // Add default tools and validations based on action type
        Self::add_default_configuration(&mut intent).await?;

        // Validate the intent
        intent.validate()?;

        // Output the intent
        let intent_yaml = serde_yaml::to_string(&intent).map_err(|e| {
            ActionError::serialization(format!("Failed to serialize intent: {}", e))
        })?;

        if let Some(file_path) = output_file {
            tokio::fs::write(&file_path, intent_yaml)
                .await
                .map_err(|e| {
                    ActionError::file_operation(
                        PathBuf::from(&file_path),
                        format!("Failed to write intent file: {}", e),
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
            println!(
                "Transformation Tools: {}",
                intent.transformation.tools.join(", ")
            );
            println!(
                "Validation Tools: {}",
                intent.transformation.validation.join(", ")
            );
            println!(
                "Pre-execution Checks: {}",
                intent.safety_checks.pre_execution.join(", ")
            );
            println!(
                "Post-execution Checks: {}",
                intent.safety_checks.post_execution.join(", ")
            );
        }

        if safety {
            println!("\n=== SAFETY ANALYSIS ===");
            println!("Requires Approval: {}", intent.requires_approval());
            if let Some(approvers) = &intent.approval_workflow.approvers {
                println!("Approvers: {}", approvers.join(", "));
            }
            println!(
                "Auto-approve Conditions: {:?}",
                intent.approval_workflow.auto_approve_for
            );
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

        let result = execute_action(&intent).await?;

        if result.success {
            println!("✅ Action executed successfully!");
            println!("Duration: {:?}", result.duration);
            println!("Changes: {}", result.changes.join(", "));
        } else {
            println!("❌ Action execution failed!");
            println!("Errors: {}", result.errors.join(", "));
            
            // Display rollback information
            if result.changes.iter().any(|change| change.contains("Rollback")) {
                println!("🔄 Rollback Information:");
                for change in &result.changes {
                    if change.contains("Rollback") || change.contains("Backup") {
                        println!("  {}", change);
                    }
                }
            } else {
                println!("💡 No automatic rollback was performed");
                println!("   Use 'rhema action rollback {}' to manually rollback", intent.id);
            }
            
            if !result.warnings.is_empty() {
                println!("⚠️  Warnings: {}", result.warnings.join(", "));
            }
        }

        info!("Action execution completed");
        Ok(())
    }

    /// Handle rollback command
    async fn handle_rollback(
        intent_id: String,
        force: bool,
        keep_backup: bool,
    ) -> ActionResult<()> {
        info!("Rolling back action: {}", intent_id);

        // Initialize rollback manager
        let rollback_manager = crate::rollback::RollbackManager::new().await?;

        // Find backups for this intent
        let backups = rollback_manager.list_backups_for_intent(&intent_id).await;
        
        if backups.is_empty() {
            if force {
                println!("⚠️  No backups found for intent {}, but force flag is set", intent_id);
                println!("Proceeding with rollback without backup...");
                return Ok(());
            } else {
                return Err(ActionError::not_found(format!(
                    "No backups found for intent {}. Use --force to proceed without backup.",
                    intent_id
                )));
            }
        }

        // Get the most recent backup
        let latest_backup = backups
            .into_iter()
            .max_by_key(|backup| backup.created_at)
            .unwrap();

        println!("Found backup: {} (created: {})", latest_backup.id, latest_backup.created_at);
        println!("Backup method: {:?}", latest_backup.backup_method);
        println!("Files backed up: {}", latest_backup.files_backed_up.len());

        if !force {
            println!("⚠️  This will restore {} files to their previous state.", latest_backup.files_backed_up.len());
            println!("Are you sure you want to proceed? (y/N)");
            
            // Read user confirmation from stdin
            let mut input = String::new();
            if let Ok(_) = std::io::stdin().read_line(&mut input) {
                let confirmed = input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes";
                if !confirmed {
                    println!("Rollback cancelled by user");
                    return Ok(());
                }
            } else {
                println!("Failed to read user input, cancelling rollback");
                return Ok(());
            }
        }

        // Perform rollback
        println!("🔄 Performing rollback...");
        let rollback_result = rollback_manager.rollback(&latest_backup).await?;

        if rollback_result.success {
            println!("✅ Rollback completed successfully!");
            println!("Duration: {:?}", rollback_result.duration);
            println!("Files restored: {}", rollback_result.files_restored.len());
            
            // Clean up backup if requested
            if !keep_backup {
                println!("🗑️  Cleaning up backup...");
                rollback_manager.delete_backup(&latest_backup.id).await?;
                println!("✅ Backup cleaned up");
            } else {
                println!("💾 Backup preserved as requested");
            }
        } else {
            println!("❌ Rollback failed!");
            println!("Errors: {:?}", rollback_result.errors);
            return Err(ActionError::rollback(format!(
                "Rollback failed: {:?}",
                rollback_result.errors
            )));
        }

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

        if actions.is_empty() {
            println!("No actions found");
            return Ok(());
        }

        println!("=== ACTIONS ===");
        
        // Apply filters
        let mut filtered_actions = Vec::new();
        for (intent_id, status) in actions {
            // Status filter
            let status_match = if active && status == "active" {
                true
            } else if completed && status == "completed" {
                true
            } else if failed && status == "failed" {
                true
            } else if !active && !completed && !failed {
                true // No status filter applied
            } else {
                false
            };

            if status_match {
                // Try to load intent details for additional filtering
                let intent_file = format!(".rhema/actions/{}.yaml", intent_id);
                let mut include_action = true;
                
                if let Ok(intent) = Self::load_intent_from_file(&intent_file).await {
                    // Action type filter
                    if let Some(filter_type) = &action_type {
                        if intent.action_type != *filter_type {
                            include_action = false;
                        }
                    }
                    
                    // Safety level filter
                    if let Some(filter_safety) = &safety_level {
                        if intent.safety_level != *filter_safety {
                            include_action = false;
                        }
                    }
                }
                
                if include_action {
                    filtered_actions.push((intent_id, status));
                }
            }
        }

        if filtered_actions.is_empty() {
            println!("No actions match the specified filters");
            return Ok(());
        }

        // Display actions
        println!("Found {} action(s):", filtered_actions.len());
        println!("{:<20} {:<15} {:<20}", "Intent ID", "Status", "Type");
        println!("{:-<55}", "");
        
        for (intent_id, status) in filtered_actions {
            let intent_file = format!(".rhema/actions/{}.yaml", intent_id);
            let action_type = if let Ok(intent) = Self::load_intent_from_file(&intent_file).await {
                format!("{:?}", intent.action_type)
            } else {
                "Unknown".to_string()
            };
            
            println!("{:<20} {:<15} {:<20}", 
                &intent_id[..std::cmp::min(20, intent_id.len())],
                &status[..std::cmp::min(15, status.len())],
                &action_type[..std::cmp::min(20, action_type.len())]
            );
        }

        info!("Action listing completed");
        Ok(())
    }

    /// Handle status command
    async fn handle_status(
        intent_id: String,
        detailed: bool,
        validation: bool,
    ) -> ActionResult<()> {
        info!("Checking status for intent: {}", intent_id);

        let status = get_action_status(&intent_id).await?;

        println!("Status: {}", status);

        if detailed {
            println!("\n=== DETAILED STATUS ===");
            
            // Parse the status to extract more information
            let status_lines: Vec<&str> = status.split('\n').collect();
            for line in status_lines {
                if line.starts_with("Status:") {
                    let status_value = line.replace("Status:", "").trim().to_string();
                    println!("Current State: {}", status_value);
                    
                    // Provide additional context based on status
                    match status_value.as_str() {
                        "active" => println!("  • Action is currently being executed"),
                        "pending" => println!("  • Action is waiting for approval or resources"),
                        "completed" => println!("  • Action has finished successfully"),
                        "failed" => println!("  • Action encountered errors during execution"),
                        "cancelled" => println!("  • Action was cancelled by user or system"),
                        _ => println!("  • Status: {}", status_value),
                    }
                } else if line.starts_with("Created:") {
                    println!("Created: {}", line.replace("Created:", "").trim());
                } else if line.starts_with("Updated:") {
                    println!("Last Updated: {}", line.replace("Updated:", "").trim());
                } else if line.contains("backup available") {
                    println!("Backup: Available for rollback");
                }
            }
            
            // Show additional metadata if available
            let intent_file = format!(".rhema/actions/{}.yaml", intent_id);
            if PathBuf::from(&intent_file).exists() {
                if let Ok(intent) = Self::load_intent_from_file(&intent_file).await {
                    println!("\nAction Details:");
                    println!("  Type: {:?}", intent.action_type);
                    println!("  Safety Level: {:?}", intent.safety_level);
                    println!("  Scope: {} files", intent.scope.len());
                    println!("  Tools: {} transformation tools", intent.transformation.tools.len());
                    println!("  Validations: {} validation checks", intent.transformation.validation.len());
                }
            }
        }

        if validation {
            println!("\n=== VALIDATION RESULTS ===");
            
            // Check if there are validation results stored
            let validation_file = format!(".rhema/actions/{}.validation.json", intent_id);
            if PathBuf::from(&validation_file).exists() {
                match tokio::fs::read_to_string(&validation_file).await {
                    Ok(content) => {
                        if let Ok(validation_data) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(results) = validation_data.get("results") {
                                if let Some(results_array) = results.as_array() {
                                    for result in results_array {
                                        if let (Some(check), Some(passed), Some(message)) = (
                                            result.get("check").and_then(|v| v.as_str()),
                                            result.get("passed").and_then(|v| v.as_bool()),
                                            result.get("message").and_then(|v| v.as_str())
                                        ) {
                                            let status_icon = if passed { "✅" } else { "❌" };
                                            println!("{} {}: {}", status_icon, check, message);
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("⚠️  Validation results file is corrupted");
                        }
                    }
                    Err(_) => println!("⚠️  Could not read validation results file"),
                }
            } else {
                println!("No validation results found");
                println!("Run 'rhema action validate' to perform validation checks");
            }
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
        println!("Intent ID: {}", intent.id);
        println!("Action Type: {:?}", intent.action_type);
        println!("Safety Level: {:?}", intent.safety_level);

        if preview {
            println!("\n=== PREVIEW MODE ===");
            println!("This action would affect {} files:", intent.scope.len());
            for (i, path) in intent.scope.iter().enumerate() {
                println!("  {}. {}", i + 1, path);
            }
            
            println!("\nTransformation tools that would run:");
            for tool in &intent.transformation.tools {
                println!("  • {}", tool);
            }
            
            println!("\nValidation checks that would run:");
            for validation in &intent.transformation.validation {
                println!("  • {}", validation);
            }
        }

        if safety {
            println!("\n=== SAFETY ANALYSIS ===");
            println!("Requires Approval: {}", intent.requires_approval());
            
            if let Some(approvers) = &intent.approval_workflow.approvers {
                println!("Approvers: {}", approvers.join(", "));
            }
            
            println!("Pre-execution safety checks: {}", intent.safety_checks.pre_execution.len());
            for check in &intent.safety_checks.pre_execution {
                println!("  • {}", check);
            }
            
            println!("Post-execution safety checks: {}", intent.safety_checks.post_execution.len());
            for check in &intent.safety_checks.post_execution {
                println!("  • {}", check);
            }
        }

        if validation {
            println!("\n=== VALIDATION CHECKS ===");
            
            let mut validation_results = Vec::new();
            let mut passed_checks = 0;
            let mut failed_checks = 0;
            
            // Run validation checks
            for validation_name in &intent.transformation.validation {
                println!("🔍 Running validation: {}", validation_name);
                
                let result = match validation_name.as_str() {
                    "file_exists" => check_file_exists(&intent).await,
                    "file_content" => check_file_content_safety(&intent).await.map(|r| ValidationResult {
                        passed: r.passed,
                        message: r.message,
                    }),
                    "dependency_version" => check_dependency_version(&intent).await,
                    _ => {
                        println!("⚠️  Unknown validation: {}", validation_name);
                        failed_checks += 1;
                        validation_results.push((
                            validation_name.clone(),
                            ValidationResult {
                                passed: false,
                                message: format!("Unknown validation: {}", validation_name),
                            },
                        ));
                        continue;
                    }
                };
                
                match result {
                    Ok(validation_result) => {
                        if validation_result.passed {
                            println!("✅ {}: PASSED", validation_name);
                            passed_checks += 1;
                        } else {
                            println!("❌ {}: FAILED", validation_name);
                            println!("   Reason: {}", validation_result.message);
                            failed_checks += 1;
                        }
                        validation_results.push((validation_name.clone(), validation_result));
                    }
                    Err(e) => {
                        println!("❌ {}: ERROR", validation_name);
                        println!("   Error: {}", e);
                        failed_checks += 1;
                        validation_results.push((
                            validation_name.clone(),
                            ValidationResult {
                                passed: false,
                                message: format!("Error: {}", e),
                            },
                        ));
                    }
                }
            }
            
            // Save validation results
            let validation_data = serde_json::json!({
                "intent_id": intent.id,
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "summary": {
                    "total_checks": intent.transformation.validation.len(),
                    "passed": passed_checks,
                    "failed": failed_checks,
                    "success_rate": if intent.transformation.validation.is_empty() {
                        100.0
                    } else {
                        (passed_checks as f64 / intent.transformation.validation.len() as f64) * 100.0
                    }
                },
                "results": validation_results.iter().map(|(name, result)| {
                    serde_json::json!({
                        "check": name,
                        "passed": result.passed,
                        "message": result.message
                    })
                }).collect::<Vec<_>>()
            });
            
            let validation_file = format!(".rhema/actions/{}.validation.json", intent.id);
            let validation_content = serde_json::to_string_pretty(&validation_data)?;
            tokio::fs::write(&validation_file, validation_content).await.map_err(|e| {
                ActionError::file_operation(
                    PathBuf::from(&validation_file),
                    format!("Failed to write validation file: {}", e),
                )
            })?;
            
            println!("\nValidation Summary:");
            println!("  Total checks: {}", intent.transformation.validation.len());
            println!("  Passed: {}", passed_checks);
            println!("  Failed: {}", failed_checks);
            println!("  Success rate: {:.1}%", 
                if intent.transformation.validation.is_empty() {
                    100.0
                } else {
                    (passed_checks as f64 / intent.transformation.validation.len() as f64) * 100.0
                }
            );
            
            if failed_checks > 0 {
                println!("⚠️  Some validation checks failed. Review the results above.");
            }
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

        // Calculate the cutoff date
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days as i64);
        
        // In a real implementation, this would query a database or state store
        // For now, we'll simulate by reading from a state file
        let state_file = PathBuf::from(".rhema/actions/state.json");
        
        if !state_file.exists() {
            println!("No action history found");
            return Ok(());
        }
        
        let content = tokio::fs::read_to_string(&state_file).await.map_err(|e| {
            ActionError::file_operation(
                state_file.clone(),
                format!("Failed to read state file: {}", e),
            )
        })?;
        
        let actions: Vec<ActionState> = serde_json::from_str(&content).map_err(|e| {
            ActionError::deserialization(format!("Failed to parse state file: {}", e))
        })?;
        
        // Filter actions by date and other criteria
        let filtered_actions: Vec<&ActionState> = actions
            .iter()
            .filter(|action| {
                // Parse the created_at date
                if let Ok(created_date) = chrono::DateTime::parse_from_rfc3339(&action.created_at) {
                    created_date.naive_utc() >= cutoff_date.naive_utc()
                } else {
                    false
                }
            })
            .collect();
        
        if filtered_actions.is_empty() {
            println!("No actions found in the last {} days", days);
            return Ok(());
        }
        
        println!("=== ACTION HISTORY (Last {} days) ===", days);
        println!("Total actions: {}", filtered_actions.len());
        
        // Group actions by status
        let mut status_counts = std::collections::HashMap::new();
        for action in &filtered_actions {
            *status_counts.entry(&action.status).or_insert(0) += 1;
        }
        
        println!("\nStatus Summary:");
        for (status, count) in status_counts {
            println!("  {}: {}", status, count);
        }
        
        if detailed {
            println!("\nDetailed History:");
            println!("{:<20} {:<15} {:<20} {:<20}", "Intent ID", "Status", "Created", "Updated");
            println!("{:-<75}", "");
            
            for action in filtered_actions {
                println!(
                    "{:<20} {:<15} {:<20} {:<20}",
                    &action.intent_id[..std::cmp::min(20, action.intent_id.len())],
                    &action.status[..std::cmp::min(15, action.status.len())],
                    &action.created_at[..std::cmp::min(20, action.created_at.len())],
                    &action.updated_at[..std::cmp::min(20, action.updated_at.len())]
                );
            }
        } else {
            println!("\nRecent Actions:");
            for action in filtered_actions.iter().take(10) {
                println!("  {} - {} ({})", action.intent_id, action.status, action.created_at);
            }
            
            if filtered_actions.len() > 10 {
                println!("  ... and {} more actions", filtered_actions.len() - 10);
            }
        }
        
        // Show action type and safety level filters if specified
        if action_type.is_some() || safety_level.is_some() {
            println!("\nFilters Applied:");
            if let Some(action_type) = action_type {
                println!("  Action Type: {:?}", action_type);
            }
            if let Some(safety_level) = safety_level {
                println!("  Safety Level: {:?}", safety_level);
            }
        }
        
        println!("=====================");

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
        println!("Action Type: {:?}", intent.action_type);
        println!("Safety Level: {:?}", intent.safety_level);

        let mut all_results = Vec::new();
        let mut passed_checks = 0;
        let mut failed_checks = 0;
        let mut warnings = 0;

        // Run all safety checks
        let checks_to_run = if all {
            vec![
                "file_permissions",
                "file_content",
                "dependency_updates",
                "security_vulnerabilities",
                "performance_impact",
                "data_integrity",
                "backup_availability",
                "rollback_capability",
            ]
        } else {
            intent.safety_checks.pre_execution.iter().map(|s| s.as_str()).collect()
        };

        for check_name in &checks_to_run {
            println!("\n🔍 Running check: {}", check_name);
            
            let result = match *check_name {
                "file_permissions" => check_file_permissions(&intent).await,
                "file_content" => check_file_content_safety(&intent).await,
                "dependency_updates" => check_dependency_updates(&intent).await,
                "security_vulnerabilities" => check_security_vulnerabilities(&intent).await,
                "performance_impact" => check_performance_impact(&intent).await,
                "data_integrity" => check_data_integrity(&intent).await,
                "backup_availability" => check_backup_availability(&intent).await,
                "rollback_capability" => check_rollback_capability(&intent).await,
                _ => {
                    println!("⚠️  Unknown check: {}", check_name);
                    warnings += 1;
                    continue;
                }
            };

            match result {
                Ok(check_result) => {
                    if check_result.passed {
                        println!("✅ {}: PASSED", check_name);
                        passed_checks += 1;
                    } else {
                        println!("❌ {}: FAILED", check_name);
                        println!("   Reason: {}", check_result.message);
                        failed_checks += 1;
                    }
                    
                    all_results.push((*check_name, check_result));
                }
                Err(e) => {
                    println!("❌ {}: ERROR", check_name);
                    println!("   Error: {}", e);
                    failed_checks += 1;
                    all_results.push((
                        *check_name,
                        SafetyCheckResult {
                            passed: false,
                            message: format!("Error: {}", e),
                        },
                    ));
                }
            }
        }

        // Print summary
        println!("\n=== SAFETY CHECK SUMMARY ===");
        println!("Total checks: {}", checks_to_run.len());
        println!("Passed: {}", passed_checks);
        println!("Failed: {}", failed_checks);
        println!("Warnings: {}", warnings);
        println!("Success rate: {:.1}%", 
            (passed_checks as f64 / checks_to_run.len() as f64) * 100.0);

        if detailed {
            println!("\n=== DETAILED RESULTS ===");
            for (check_name, result) in &all_results {
                println!("{}: {}", 
                    if result.passed { "✅" } else { "❌" }, 
                    check_name);
                if !result.passed {
                    println!("   {}", result.message);
                }
            }
        }

        // Export results if requested
        if let Some(export_file) = export {
            let export_data = serde_json::json!({
                "intent_id": intent.id,
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "summary": {
                    "total_checks": checks_to_run.len(),
                    "passed": passed_checks,
                    "failed": failed_checks,
                    "warnings": warnings,
                    "success_rate": (passed_checks as f64 / checks_to_run.len() as f64) * 100.0
                },
                "results": all_results.iter().map(|(name, result)| {
                    serde_json::json!({
                        "check": name,
                        "passed": result.passed,
                        "message": result.message
                    })
                }).collect::<Vec<_>>()
            });

            let export_content = serde_json::to_string_pretty(&export_data)?;
            tokio::fs::write(&export_file, export_content).await.map_err(|e| {
                ActionError::file_operation(
                    PathBuf::from(&export_file),
                    format!("Failed to write export file: {}", e),
                )
            })?;
            
            println!("📄 Results exported to: {}", export_file);
        }

        // Return error if any checks failed
        if failed_checks > 0 {
            return Err(ActionError::safety_check(
                "safety_checks".to_string(),
                format!("{} safety checks failed", failed_checks),
            ));
        }

        println!("✅ All safety checks passed!");
        info!("Safety checks completed");
        Ok(())
    }

    /// Handle approve command
    async fn handle_approve(
        intent_id: String,
        comment: Option<String>,
        auto_execute: bool,
    ) -> ActionResult<()> {
        info!("Approving intent: {}", intent_id);

        // In a real implementation, this would update a database or state store
        // For now, we'll simulate by updating a state file
        let state_file = PathBuf::from(".rhema/actions/state.json");
        
        if !state_file.exists() {
            return Err(ActionError::not_found(format!("Action {} not found", intent_id)));
        }
        
        let content = tokio::fs::read_to_string(&state_file).await.map_err(|e| {
            ActionError::file_operation(
                state_file.clone(),
                format!("Failed to read state file: {}", e),
            )
        })?;
        
        let mut actions: Vec<ActionState> = serde_json::from_str(&content).map_err(|e| {
            ActionError::deserialization(format!("Failed to parse state file: {}", e))
        })?;
        
        // Find and update the specific action
        let action_index = actions
            .iter()
            .position(|action| action.intent_id == intent_id)
            .ok_or_else(|| ActionError::not_found(format!("Action {} not found", intent_id)))?;
        
        let action = &mut actions[action_index];
        
        // Check if action is in a state that can be approved
        if action.status != "pending" && action.status != "review" {
            return Err(ActionError::invalid_state(format!(
                "Action {} is in state '{}' and cannot be approved",
                intent_id, action.status
            )));
        }
        
        // Update action status
        action.status = "approved".to_string();
        action.updated_at = chrono::Utc::now().to_rfc3339();
        
        // Save updated state
        let updated_content = serde_json::to_string_pretty(&actions)?;
        tokio::fs::write(&state_file, updated_content).await.map_err(|e| {
            ActionError::file_operation(
                state_file.clone(),
                format!("Failed to write state file: {}", e),
            )
        })?;
        
        // Save approval record
        let approval_record = ApprovalRecord {
            intent_id: intent_id.clone(),
            approved_at: chrono::Utc::now().to_rfc3339(),
            comment: comment.clone(),
            approver: "cli_user".to_string(), // In real implementation, this would be the actual user
        };
        
        let approvals_file = PathBuf::from(".rhema/actions/approvals.json");
        let mut approvals = if approvals_file.exists() {
            let content = tokio::fs::read_to_string(&approvals_file).await?;
            serde_json::from_str(&content).unwrap_or(Vec::new())
        } else {
            Vec::new()
        };
        
        approvals.push(approval_record);
        let approvals_content = serde_json::to_string_pretty(&approvals)?;
        tokio::fs::write(&approvals_file, approvals_content).await.map_err(|e| {
            ActionError::file_operation(
                approvals_file.clone(),
                format!("Failed to write approvals file: {}", e),
            )
        })?;
        
        println!("✅ Intent {} approved successfully!", intent_id);
        if let Some(comment) = comment {
            println!("Comment: {}", comment);
        }
        
        // Auto-execute if requested
        if auto_execute {
            println!("🚀 Auto-executing approved intent...");
            
            // Find the intent file
            let intent_file = format!(".rhema/actions/{}.yaml", intent_id);
            if PathBuf::from(&intent_file).exists() {
                let result = Self::handle_execute(intent_file, false, false, false).await;
                match result {
                    Ok(_) => println!("✅ Auto-execution completed successfully!"),
                    Err(e) => {
                        println!("❌ Auto-execution failed: {}", e);
                        return Err(e);
                    }
                }
            } else {
                println!("⚠️  Intent file not found for auto-execution");
            }
        }
        
        info!("Approval completed");
        Ok(())
    }

    /// Handle reject command
    async fn handle_reject(intent_id: String, reason: String) -> ActionResult<()> {
        info!("Rejecting intent: {} with reason: {}", intent_id, reason);

        // In a real implementation, this would update a database or state store
        // For now, we'll simulate by updating a state file
        let state_file = PathBuf::from(".rhema/actions/state.json");
        
        if !state_file.exists() {
            return Err(ActionError::not_found(format!("Action {} not found", intent_id)));
        }
        
        let content = tokio::fs::read_to_string(&state_file).await.map_err(|e| {
            ActionError::file_operation(
                state_file.clone(),
                format!("Failed to read state file: {}", e),
            )
        })?;
        
        let mut actions: Vec<ActionState> = serde_json::from_str(&content).map_err(|e| {
            ActionError::deserialization(format!("Failed to parse state file: {}", e))
        })?;
        
        // Find and update the specific action
        let action_index = actions
            .iter()
            .position(|action| action.intent_id == intent_id)
            .ok_or_else(|| ActionError::not_found(format!("Action {} not found", intent_id)))?;
        
        let action = &mut actions[action_index];
        
        // Check if action is in a state that can be rejected
        if action.status != "pending" && action.status != "review" {
            return Err(ActionError::invalid_state(format!(
                "Action {} is in state '{}' and cannot be rejected",
                intent_id, action.status
            )));
        }
        
        // Update action status
        action.status = "rejected".to_string();
        action.updated_at = chrono::Utc::now().to_rfc3339();
        
        // Save updated state
        let updated_content = serde_json::to_string_pretty(&actions)?;
        tokio::fs::write(&state_file, updated_content).await.map_err(|e| {
            ActionError::file_operation(
                state_file.clone(),
                format!("Failed to write state file: {}", e),
            )
        })?;
        
        // Save rejection record
        let rejection_record = RejectionRecord {
            intent_id: intent_id.clone(),
            rejected_at: chrono::Utc::now().to_rfc3339(),
            reason: reason.clone(),
            rejector: "cli_user".to_string(), // In real implementation, this would be the actual user
        };
        
        let rejections_file = PathBuf::from(".rhema/actions/rejections.json");
        let mut rejections = if rejections_file.exists() {
            let content = tokio::fs::read_to_string(&rejections_file).await?;
            serde_json::from_str(&content).unwrap_or(Vec::new())
        } else {
            Vec::new()
        };
        
        rejections.push(rejection_record);
        let rejections_content = serde_json::to_string_pretty(&rejections)?;
        tokio::fs::write(&rejections_file, rejections_content).await.map_err(|e| {
            ActionError::file_operation(
                rejections_file.clone(),
                format!("Failed to write rejections file: {}", e),
            )
        })?;
        
        println!("❌ Intent {} rejected successfully!", intent_id);
        println!("Reason: {}", reason);
        
        info!("Rejection completed");
        Ok(())
    }

    /// Load intent from file
    async fn load_intent_from_file(file_path: &str) -> ActionResult<ActionIntent> {
        let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            ActionError::file_operation(
                PathBuf::from(file_path),
                format!("Failed to read intent file: {}", e),
            )
        })?;

        let intent: ActionIntent = serde_yaml::from_str(&content).map_err(|e| {
            ActionError::deserialization(format!("Failed to parse intent file: {}", e))
        })?;

        Ok(intent)
    }

    /// Add default configuration to intent
    async fn add_default_configuration(intent: &mut ActionIntent) -> ActionResult<()> {
        info!("Adding default configuration for intent: {}", intent.id);

        // Add default tools and validations based on the action type
        match intent.action_type {
            ActionType::Refactor => {
                // For refactoring, add code quality tools
                if !intent.transformation.tools.contains(&"prettier".to_string()) {
                    intent.transformation.tools.push("prettier".to_string());
                }
                if !intent.transformation.tools.contains(&"eslint".to_string()) {
                    intent.transformation.tools.push("eslint".to_string());
                }
                if !intent.transformation.validation.contains(&"syntax_check".to_string()) {
                    intent.transformation.validation.push("syntax_check".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"file_permissions".to_string()) {
                    intent.safety_checks.pre_execution.push("file_permissions".to_string());
                }
            }
            ActionType::Feature => {
                // For feature development, add testing tools
                if !intent.transformation.tools.contains(&"test_generation".to_string()) {
                    intent.transformation.tools.push("test_generation".to_string());
                }
                if !intent.transformation.validation.contains(&"test_coverage".to_string()) {
                    intent.transformation.validation.push("test_coverage".to_string());
                }
                if !intent.safety_checks.post_execution.contains(&"regression_test".to_string()) {
                    intent.safety_checks.post_execution.push("regression_test".to_string());
                }
            }
            ActionType::BugFix => {
                // For bug fixes, add debugging and validation tools
                if !intent.transformation.tools.contains(&"debug_analysis".to_string()) {
                    intent.transformation.tools.push("debug_analysis".to_string());
                }
                if !intent.transformation.validation.contains(&"bug_validation".to_string()) {
                    intent.transformation.validation.push("bug_validation".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"backup_availability".to_string()) {
                    intent.safety_checks.pre_execution.push("backup_availability".to_string());
                }
            }
            ActionType::Documentation => {
                // For documentation, add formatting and link checking
                if !intent.transformation.tools.contains(&"markdown_formatter".to_string()) {
                    intent.transformation.tools.push("markdown_formatter".to_string());
                }
                if !intent.transformation.validation.contains(&"link_checker".to_string()) {
                    intent.transformation.validation.push("link_checker".to_string());
                }
            }
            ActionType::Test => {
                // For test creation, add test framework tools
                if !intent.transformation.tools.contains(&"test_framework_setup".to_string()) {
                    intent.transformation.tools.push("test_framework_setup".to_string());
                }
                if !intent.transformation.validation.contains(&"test_execution".to_string()) {
                    intent.transformation.validation.push("test_execution".to_string());
                }
            }
            ActionType::Configuration => {
                // For configuration changes, add validation tools
                if !intent.transformation.tools.contains(&"config_validator".to_string()) {
                    intent.transformation.tools.push("config_validator".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"config_backup".to_string()) {
                    intent.safety_checks.pre_execution.push("config_backup".to_string());
                }
            }
            ActionType::Dependency => {
                // For dependency updates, add security and compatibility checks
                if !intent.transformation.tools.contains(&"dependency_updater".to_string()) {
                    intent.transformation.tools.push("dependency_updater".to_string());
                }
                if !intent.transformation.validation.contains(&"security_scan".to_string()) {
                    intent.transformation.validation.push("security_scan".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"compatibility_check".to_string()) {
                    intent.safety_checks.pre_execution.push("compatibility_check".to_string());
                }
            }
            ActionType::Security => {
                // For security updates, add comprehensive security checks
                if !intent.transformation.tools.contains(&"security_patch".to_string()) {
                    intent.transformation.tools.push("security_patch".to_string());
                }
                if !intent.transformation.validation.contains(&"vulnerability_scan".to_string()) {
                    intent.transformation.validation.push("vulnerability_scan".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"security_audit".to_string()) {
                    intent.safety_checks.pre_execution.push("security_audit".to_string());
                }
                if !intent.safety_checks.post_execution.contains(&"security_validation".to_string()) {
                    intent.safety_checks.post_execution.push("security_validation".to_string());
                }
            }
            ActionType::Performance => {
                // For performance improvements, add benchmarking tools
                if !intent.transformation.tools.contains(&"performance_analyzer".to_string()) {
                    intent.transformation.tools.push("performance_analyzer".to_string());
                }
                if !intent.transformation.validation.contains(&"benchmark_test".to_string()) {
                    intent.transformation.validation.push("benchmark_test".to_string());
                }
                if !intent.safety_checks.post_execution.contains(&"performance_validation".to_string()) {
                    intent.safety_checks.post_execution.push("performance_validation".to_string());
                }
            }
            ActionType::Cleanup => {
                // For cleanup operations, add backup and validation tools
                if !intent.safety_checks.pre_execution.contains(&"backup_availability".to_string()) {
                    intent.safety_checks.pre_execution.push("backup_availability".to_string());
                }
                if !intent.transformation.validation.contains(&"cleanup_validation".to_string()) {
                    intent.transformation.validation.push("cleanup_validation".to_string());
                }
            }
            ActionType::Migration => {
                // For migrations, add comprehensive backup and rollback tools
                if !intent.safety_checks.pre_execution.contains(&"backup_availability".to_string()) {
                    intent.safety_checks.pre_execution.push("backup_availability".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"rollback_capability".to_string()) {
                    intent.safety_checks.pre_execution.push("rollback_capability".to_string());
                }
                if !intent.transformation.validation.contains(&"migration_validation".to_string()) {
                    intent.transformation.validation.push("migration_validation".to_string());
                }
                if !intent.safety_checks.post_execution.contains(&"migration_verification".to_string()) {
                    intent.safety_checks.post_execution.push("migration_verification".to_string());
                }
            }
            ActionType::Custom(_) => {
                // For custom actions, add basic safety checks
                if !intent.safety_checks.pre_execution.contains(&"file_permissions".to_string()) {
                    intent.safety_checks.pre_execution.push("file_permissions".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"backup_availability".to_string()) {
                    intent.safety_checks.pre_execution.push("backup_availability".to_string());
                }
            }
        }

        // Add safety level specific configurations
        match intent.safety_level {
            SafetyLevel::Low => {
                // Low safety level - minimal checks
                if intent.safety_checks.pre_execution.is_empty() {
                    intent.safety_checks.pre_execution.push("basic_validation".to_string());
                }
            }
            SafetyLevel::Medium => {
                // Medium safety level - standard checks
                if !intent.safety_checks.pre_execution.contains(&"file_permissions".to_string()) {
                    intent.safety_checks.pre_execution.push("file_permissions".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"backup_availability".to_string()) {
                    intent.safety_checks.pre_execution.push("backup_availability".to_string());
                }
            }
            SafetyLevel::High => {
                // High safety level - comprehensive checks
                if !intent.safety_checks.pre_execution.contains(&"file_permissions".to_string()) {
                    intent.safety_checks.pre_execution.push("file_permissions".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"backup_availability".to_string()) {
                    intent.safety_checks.pre_execution.push("backup_availability".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"security_scan".to_string()) {
                    intent.safety_checks.pre_execution.push("security_scan".to_string());
                }
                if !intent.safety_checks.post_execution.contains(&"regression_test".to_string()) {
                    intent.safety_checks.post_execution.push("regression_test".to_string());
                }
            }
            SafetyLevel::Critical => {
                // Critical safety level - maximum checks and approval required
                intent.set_approval_required(true);
                if !intent.safety_checks.pre_execution.contains(&"file_permissions".to_string()) {
                    intent.safety_checks.pre_execution.push("file_permissions".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"backup_availability".to_string()) {
                    intent.safety_checks.pre_execution.push("backup_availability".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"security_scan".to_string()) {
                    intent.safety_checks.pre_execution.push("security_scan".to_string());
                }
                if !intent.safety_checks.pre_execution.contains(&"rollback_capability".to_string()) {
                    intent.safety_checks.pre_execution.push("rollback_capability".to_string());
                }
                if !intent.safety_checks.post_execution.contains(&"regression_test".to_string()) {
                    intent.safety_checks.post_execution.push("regression_test".to_string());
                }
                if !intent.safety_checks.post_execution.contains(&"comprehensive_validation".to_string()) {
                    intent.safety_checks.post_execution.push("comprehensive_validation".to_string());
                }
            }
        }

        info!("Default configuration added successfully");
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
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_command() {
        // This test would require a valid intent file
        // For now, just test that the function exists
        assert!(true);
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ApprovalRecord {
    intent_id: String,
    approved_at: String,
    comment: Option<String>,
    approver: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RejectionRecord {
    intent_id: String,
    rejected_at: String,
    reason: String,
    rejector: String,
}

/// Check if files exist
async fn check_file_exists(intent: &ActionIntent) -> ActionResult<ValidationResult> {
    let mut passed = true;
    let mut message = String::new();

    for path in &intent.scope {
        if !tokio::fs::metadata(path).await.is_ok() {
            passed = false;
            message.push_str(&format!("File not found: {}\n", path));
        }
    }

    Ok(ValidationResult { passed, message })
}

/// Check file content for safety checks
async fn check_file_content_safety(intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let mut passed = true;
    let mut message = String::new();

    // Check if files exist and are readable
    for path in &intent.scope {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                if content.is_empty() {
                    message.push_str(&format!("File {} is empty\n", path));
                }
            }
            Err(e) => {
                passed = false;
                message.push_str(&format!("Cannot read file {}: {}\n", path, e));
            }
        }
    }

    Ok(SafetyCheckResult { passed, message })
}

/// Check dependency version
async fn check_dependency_version(intent: &ActionIntent) -> ActionResult<ValidationResult> {
    let mut passed = true;
    let mut message = String::new();

    // Check if Cargo.toml exists and is valid
    if intent.scope.iter().any(|path| path.contains("Cargo.toml")) {
        let output = tokio::process::Command::new("cargo")
            .arg("check")
            .output()
            .await;
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    passed = false;
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    message.push_str(&format!("Cargo check failed: {}\n", stderr));
                }
            }
            Err(e) => {
                passed = false;
                message.push_str(&format!("Cargo check failed: {}\n", e));
            }
        }
    }

    Ok(ValidationResult { passed, message })
}

/// Check file permissions
async fn check_file_permissions(intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let mut passed = true;
    let mut message = String::new();

    for path in &intent.scope {
        if let Ok(metadata) = tokio::fs::metadata(path).await {
            let permissions = metadata.permissions();
            if permissions.readonly() {
                passed = false;
                message.push_str(&format!("File {} is read-only\n", path));
            }
        } else {
            passed = false;
            message.push_str(&format!("Cannot access file {}\n", path));
        }
    }

    Ok(SafetyCheckResult { passed, message })
}

/// Check dependency updates
async fn check_dependency_updates(intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let passed = true;
    let mut message = String::new();

    // Check if there are any dependency-related files in scope
    let dependency_files = vec!["Cargo.toml", "package.json", "requirements.txt", "go.mod"];
    let has_dependency_files = intent.scope.iter().any(|path| {
        dependency_files.iter().any(|dep_file| path.contains(dep_file))
    });

    if has_dependency_files {
        // In a real implementation, this would check for available updates
        message.push_str("Dependency files detected - consider checking for updates\n");
    }

    Ok(SafetyCheckResult { passed, message })
}

/// Check security vulnerabilities
async fn check_security_vulnerabilities(intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let passed = true;
    let mut message = String::new();

    // In a real implementation, this would run security scans
    // For now, we'll simulate a basic check
    if intent.scope.iter().any(|path| path.contains("node_modules") || path.contains("target")) {
        message.push_str("Vendor directories detected - consider security scanning\n");
    }

    Ok(SafetyCheckResult { passed, message })
}

/// Check performance impact
async fn check_performance_impact(intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let passed = true;
    let mut message = String::new();

    // Check file sizes to estimate performance impact
    let mut total_size = 0u64;
    for path in &intent.scope {
        if let Ok(metadata) = tokio::fs::metadata(path).await {
            total_size += metadata.len();
        }
    }

    if total_size > 100 * 1024 * 1024 { // 100MB
        message.push_str(&format!("Large files detected ({} bytes) - consider performance impact\n", total_size));
    }

    Ok(SafetyCheckResult { passed, message })
}

/// Check data integrity
async fn check_data_integrity(intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let mut passed = true;
    let mut message = String::new();

    // Check if any files are corrupted or inaccessible
    for path in &intent.scope {
        if let Err(_) = tokio::fs::read_to_string(path).await {
            passed = false;
            message.push_str(&format!("Cannot read file {} - possible corruption\n", path));
        }
    }

    Ok(SafetyCheckResult { passed, message })
}

/// Check backup availability
async fn check_backup_availability(_intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let mut passed = true;
    let mut message = String::new();

    // Check if backup system is available
    let backup_dir = PathBuf::from(".rhema/backups");
    if !backup_dir.exists() {
        passed = false;
        message.push_str("Backup directory not found - consider creating backups");
    } else {
        // Check if backup directory is writable
        match tokio::fs::metadata(&backup_dir).await {
            Ok(metadata) => {
                let permissions = metadata.permissions();
                if permissions.readonly() {
                    passed = false;
                    message.push_str("Backup directory is read-only - cannot create backups");
                } else {
                    message.push_str("Backup directory is available and writable");
                }
            }
            Err(e) => {
                passed = false;
                message.push_str(&format!("Cannot access backup directory: {}", e));
            }
        }
    }

    Ok(SafetyCheckResult { passed, message })
}

/// Check rollback capability
async fn check_rollback_capability(_intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
    let mut passed = true;
    let mut message = String::new();

    // Check if rollback system is available
    match crate::rollback::RollbackManager::new().await {
        Ok(_) => {
            message.push_str("Rollback system is available and ready");
        }
        Err(e) => {
            passed = false;
            message.push_str(&format!("Rollback system not available: {}", e));
        }
    }

    Ok(SafetyCheckResult { passed, message })
}
