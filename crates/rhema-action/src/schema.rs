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

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

use crate::error::{ActionError, ActionResult};

/// Action types supported by the protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    /// Code refactoring actions
    Refactor,
    /// Feature development actions
    Feature,
    /// Bug fix actions
    BugFix,
    /// Documentation updates
    Documentation,
    /// Test creation or updates
    Test,
    /// Configuration changes
    Configuration,
    /// Dependency updates
    Dependency,
    /// Security updates
    Security,
    /// Performance improvements
    Performance,
    /// Code cleanup
    Cleanup,
    /// Migration actions
    Migration,
    /// Custom action type
    Custom(String),
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Refactor => write!(f, "refactor"),
            ActionType::Feature => write!(f, "feature"),
            ActionType::BugFix => write!(f, "bugfix"),
            ActionType::Documentation => write!(f, "documentation"),
            ActionType::Test => write!(f, "test"),
            ActionType::Configuration => write!(f, "configuration"),
            ActionType::Dependency => write!(f, "dependency"),
            ActionType::Security => write!(f, "security"),
            ActionType::Performance => write!(f, "performance"),
            ActionType::Cleanup => write!(f, "cleanup"),
            ActionType::Migration => write!(f, "migration"),
            ActionType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

impl FromStr for ActionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "refactor" => Ok(ActionType::Refactor),
            "feature" => Ok(ActionType::Feature),
            "bugfix" => Ok(ActionType::BugFix),
            "documentation" => Ok(ActionType::Documentation),
            "test" => Ok(ActionType::Test),
            "configuration" => Ok(ActionType::Configuration),
            "dependency" => Ok(ActionType::Dependency),
            "security" => Ok(ActionType::Security),
            "performance" => Ok(ActionType::Performance),
            "cleanup" => Ok(ActionType::Cleanup),
            "migration" => Ok(ActionType::Migration),
            s if s.starts_with("custom:") => {
                let custom = s.strip_prefix("custom:").unwrap_or(s);
                Ok(ActionType::Custom(custom.to_string()))
            }
            _ => Err(format!("Unknown action type: {}", s)),
        }
    }
}

/// Safety levels for actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum SafetyLevel {
    /// Low risk actions (auto-approved)
    Low,
    /// Medium risk actions (basic approval)
    Medium,
    /// High risk actions (senior approval)
    High,
    /// Critical risk actions (team approval)
    Critical,
}

impl fmt::Display for SafetyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafetyLevel::Low => write!(f, "low"),
            SafetyLevel::Medium => write!(f, "medium"),
            SafetyLevel::High => write!(f, "high"),
            SafetyLevel::Critical => write!(f, "critical"),
        }
    }
}

impl FromStr for SafetyLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(SafetyLevel::Low),
            "medium" => Ok(SafetyLevel::Medium),
            "high" => Ok(SafetyLevel::High),
            "critical" => Ok(SafetyLevel::Critical),
            _ => Err(format!("Unknown safety level: {}", s)),
        }
    }
}

/// Context reference for action intent
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ContextReference {
    /// File containing the context
    #[validate(length(min = 1))]
    pub file: String,

    /// Section within the file
    pub section: Option<String>,

    /// Specific line numbers
    pub lines: Option<(usize, usize)>,

    /// Context tags
    pub tags: Option<Vec<String>>,
}

/// Transformation configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TransformationConfig {
    /// Tools to use for transformation
    #[validate(length(min = 1))]
    pub tools: Vec<String>,

    /// Validation tools to run
    pub validation: Vec<String>,

    /// Rollback strategy
    #[serde(default = "default_rollback_strategy")]
    pub rollback_strategy: String,

    /// Tool-specific configuration
    pub tool_config: Option<HashMap<String, serde_json::Value>>,

    /// Transformation timeout (seconds)
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_rollback_strategy() -> String {
    "git_revert".to_string()
}

fn default_timeout() -> u64 {
    300 // 5 minutes
}

/// Safety checks configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SafetyChecks {
    /// Pre-execution checks
    pub pre_execution: Vec<String>,

    /// Post-execution checks
    pub post_execution: Vec<String>,

    /// Custom safety rules
    pub custom_rules: Option<Vec<String>>,

    /// Safety check timeout (seconds)
    #[serde(default = "default_safety_timeout")]
    pub timeout: u64,
}

fn default_safety_timeout() -> u64 {
    600 // 10 minutes
}

/// Approval workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ApprovalWorkflow {
    /// Whether approval is required
    #[serde(default = "default_approval_required")]
    pub required: bool,

    /// Required approvers
    pub approvers: Option<Vec<String>>,

    /// Auto-approve conditions
    pub auto_approve_for: Option<Vec<String>>,

    /// Approval timeout (seconds)
    #[serde(default = "default_approval_timeout")]
    pub timeout: u64,

    /// Approval notification channels
    pub notification_channels: Option<Vec<String>>,
}

fn default_approval_required() -> bool {
    true
}

fn default_approval_timeout() -> u64 {
    3600 // 1 hour
}

/// Action intent - the core schema for action protocol
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ActionIntent {
    /// Unique identifier for the intent
    #[validate(length(min = 1))]
    pub id: String,

    /// Action type
    pub action_type: ActionType,

    /// Human-readable description
    #[validate(length(min = 1))]
    pub description: String,

    /// Scope of the action (files/directories)
    #[validate(length(min = 1))]
    pub scope: Vec<String>,

    /// Safety level
    pub safety_level: SafetyLevel,

    /// Context references
    pub context_refs: Option<Vec<ContextReference>>,

    /// Transformation configuration
    pub transformation: TransformationConfig,

    /// Safety checks configuration
    pub safety_checks: SafetyChecks,

    /// Approval workflow configuration
    pub approval_workflow: ApprovalWorkflow,

    /// Metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Created timestamp
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,

    /// Created by
    pub created_by: Option<String>,

    /// Tags for categorization
    pub tags: Option<Vec<String>>,

    /// Priority level
    pub priority: Option<String>,

    /// Estimated effort
    pub estimated_effort: Option<String>,

    /// Dependencies (other intents)
    pub dependencies: Option<Vec<String>>,
}

impl ActionIntent {
    /// Create a new action intent
    pub fn new(
        id: impl Into<String>,
        action_type: ActionType,
        description: impl Into<String>,
        scope: Vec<String>,
        safety_level: SafetyLevel,
    ) -> Self {
        Self {
            id: id.into(),
            action_type,
            description: description.into(),
            scope,
            safety_level,
            context_refs: None,
            transformation: TransformationConfig {
                tools: vec![],
                validation: vec![],
                rollback_strategy: default_rollback_strategy(),
                tool_config: None,
                timeout: default_timeout(),
            },
            safety_checks: SafetyChecks {
                pre_execution: vec![],
                post_execution: vec![],
                custom_rules: None,
                timeout: default_safety_timeout(),
            },
            approval_workflow: ApprovalWorkflow {
                required: default_approval_required(),
                approvers: None,
                auto_approve_for: None,
                timeout: default_approval_timeout(),
                notification_channels: None,
            },
            metadata: None,
            created_at: Utc::now(),
            created_by: None,
            tags: None,
            priority: None,
            estimated_effort: None,
            dependencies: None,
        }
    }

    /// Validate the action intent
    pub fn validate(&self) -> ActionResult<()> {
        // Custom validation
        self.validate_scope()?;
        self.validate_tools()?;
        self.validate_safety_checks()?;

        Ok(())
    }

    /// Validate scope paths
    fn validate_scope(&self) -> ActionResult<()> {
        for path in &self.scope {
            if path.is_empty() {
                return Err(ActionError::schema_validation("Scope path cannot be empty"));
            }
            if path.contains("..") {
                return Err(ActionError::schema_validation(
                    "Scope path cannot contain '..'",
                ));
            }
        }
        Ok(())
    }

    /// Validate transformation tools
    fn validate_tools(&self) -> ActionResult<()> {
        if self.transformation.tools.is_empty() {
            return Err(ActionError::schema_validation(
                "At least one transformation tool must be specified",
            ));
        }

        // Validate tool names
        let valid_tools = [
            "jscodeshift",
            "comby",
            "ast-grep",
            "prettier",
            "eslint",
            "typescript",
            "jest",
            "mocha",
            "pytest",
            "cargo",
            "npm",
        ];

        for tool in &self.transformation.tools {
            if !valid_tools.contains(&tool.as_str()) {
                return Err(ActionError::schema_validation(format!(
                    "Unknown transformation tool: {}",
                    tool
                )));
            }
        }

        Ok(())
    }

    /// Validate safety checks
    fn validate_safety_checks(&self) -> ActionResult<()> {
        let valid_checks = [
            "syntax_validation",
            "type_checking",
            "test_coverage",
            "build_validation",
            "test_execution",
            "lint_checking",
            "security_scanning",
            "performance_check",
            "dependency_check",
        ];

        for check in &self.safety_checks.pre_execution {
            if !valid_checks.contains(&check.as_str()) {
                return Err(ActionError::schema_validation(format!(
                    "Unknown pre-execution safety check: {}",
                    check
                )));
            }
        }

        for check in &self.safety_checks.post_execution {
            if !valid_checks.contains(&check.as_str()) {
                return Err(ActionError::schema_validation(format!(
                    "Unknown post-execution safety check: {}",
                    check
                )));
            }
        }

        Ok(())
    }

    /// Check if approval is required
    pub fn requires_approval(&self) -> bool {
        if !self.approval_workflow.required {
            return false;
        }

        // Check auto-approve conditions
        if let Some(auto_approve) = &self.approval_workflow.auto_approve_for {
            if auto_approve.contains(&"low_risk".to_string())
                && self.safety_level == SafetyLevel::Low
            {
                return false;
            }
            if auto_approve.contains(&"test_only".to_string())
                && self.action_type == ActionType::Test
            {
                return false;
            }
        }

        true
    }

    /// Generate a unique ID if not provided
    pub fn generate_id() -> String {
        format!("intent-{}", Uuid::new_v4().simple())
    }

    /// Add a context reference
    pub fn add_context_ref(&mut self, context_ref: ContextReference) {
        if self.context_refs.is_none() {
            self.context_refs = Some(vec![]);
        }
        self.context_refs.as_mut().unwrap().push(context_ref);
    }

    /// Add a transformation tool
    pub fn add_tool(&mut self, tool: impl Into<String>) {
        self.transformation.tools.push(tool.into());
    }

    /// Add a validation tool
    pub fn add_validation(&mut self, validation: impl Into<String>) {
        self.transformation.validation.push(validation.into());
    }

    /// Add a pre-execution safety check
    pub fn add_pre_execution_check(&mut self, check: impl Into<String>) {
        self.safety_checks.pre_execution.push(check.into());
    }

    /// Add a post-execution safety check
    pub fn add_post_execution_check(&mut self, check: impl Into<String>) {
        self.safety_checks.post_execution.push(check.into());
    }

    /// Set approval requirements
    pub fn set_approval_required(&mut self, required: bool) {
        self.approval_workflow.required = required;
    }

    /// Add an approver
    pub fn add_approver(&mut self, approver: impl Into<String>) {
        if self.approval_workflow.approvers.is_none() {
            self.approval_workflow.approvers = Some(vec![]);
        }
        self.approval_workflow
            .approvers
            .as_mut()
            .unwrap()
            .push(approver.into());
    }
}

/// Action status tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    /// Intent created but not yet executed
    Created,
    /// Intent is being planned
    Planning,
    /// Intent is pending approval
    PendingApproval,
    /// Intent is approved and ready for execution
    Approved,
    /// Intent is being executed
    Executing,
    /// Intent execution completed successfully
    Completed,
    /// Intent execution failed
    Failed,
    /// Intent was rolled back
    RolledBack,
    /// Intent was cancelled
    Cancelled,
}

/// Action execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionExecutionResult {
    /// Success status
    pub success: bool,

    /// Execution duration
    pub duration: std::time::Duration,

    /// Changes made
    pub changes: Vec<String>,

    /// Validation results
    pub validation_results: HashMap<String, bool>,

    /// Safety check results
    pub safety_results: HashMap<String, bool>,

    /// Error messages (if any)
    pub errors: Vec<String>,

    /// Warnings
    pub warnings: Vec<String>,

    /// Rollback information (if applicable)
    pub rollback_info: Option<RollbackInfo>,
}

/// Rollback information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackInfo {
    /// Rollback method used
    pub method: String,

    /// Rollback duration
    pub duration: std::time::Duration,

    /// Files restored
    pub files_restored: Vec<String>,

    /// Rollback success
    pub success: bool,

    /// Rollback errors (if any)
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_intent_creation() {
        let intent = ActionIntent::new(
            "test-001",
            ActionType::Refactor,
            "Test refactoring",
            vec!["src/".to_string()],
            SafetyLevel::Medium,
        );

        assert_eq!(intent.id, "test-001");
        assert_eq!(intent.action_type, ActionType::Refactor);
        assert_eq!(intent.description, "Test refactoring");
        assert_eq!(intent.safety_level, SafetyLevel::Medium);
    }

    #[test]
    fn test_action_intent_validation() {
        let mut intent = ActionIntent::new(
            "test-002",
            ActionType::Test,
            "Test action",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );

        intent.add_tool("jest");
        intent.add_validation("typescript");

        let result = intent.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_approval_requirements() {
        let mut intent = ActionIntent::new(
            "test-003",
            ActionType::Refactor,
            "Test refactoring",
            vec!["src/".to_string()],
            SafetyLevel::High,
        );

        // High safety level should require approval by default
        assert!(intent.requires_approval());

        // Set to not required
        intent.set_approval_required(false);
        assert!(!intent.requires_approval());
    }

    #[test]
    fn test_auto_approve_conditions() {
        let mut intent = ActionIntent::new(
            "test-004",
            ActionType::Test,
            "Test action",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );

        intent.approval_workflow.auto_approve_for = Some(vec!["low_risk".to_string()]);

        // Low risk should auto-approve
        assert!(!intent.requires_approval());
    }
}
