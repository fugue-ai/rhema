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

use thiserror::Error;
use std::path::PathBuf;

/// Action Protocol specific errors
#[derive(Error, Debug)]
pub enum ActionError {
    /// Schema validation errors
    #[error("Schema validation failed: {message}")]
    SchemaValidation { message: String },

    /// Intent parsing errors
    #[error("Failed to parse action intent: {message}")]
    IntentParsing { message: String },

    /// File operation errors
    #[error("File operation failed for {path}: {message}")]
    FileOperation { path: PathBuf, message: String },

    /// Tool execution errors
    #[error("Tool execution failed for {tool}: {message}")]
    ToolExecution { tool: String, message: String },

    /// Validation errors
    #[error("Validation failed: {message}")]
    Validation { message: String },

    /// Safety check errors
    #[error("Safety check failed: {check}: {message}")]
    SafetyCheck { check: String, message: String },

    /// Rollback errors
    #[error("Rollback failed: {message}")]
    Rollback { message: String },

    /// Approval workflow errors
    #[error("Approval workflow error: {message}")]
    Approval { message: String },

    /// Git integration errors
    #[error("Git operation failed: {operation}: {message}")]
    Git { operation: String, message: String },

    /// Pipeline execution errors
    #[error("Pipeline execution failed: {stage}: {message}")]
    Pipeline { stage: String, message: String },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Configuration { message: String },

    /// Permission errors
    #[error("Permission denied: {message}")]
    Permission { message: String },

    /// Timeout errors
    #[error("Operation timed out: {operation}")]
    Timeout { operation: String },

    /// Resource not found errors
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    /// State errors
    #[error("Invalid state: {message}")]
    InvalidState { message: String },

    /// External tool errors
    #[error("External tool error for {tool}: {message}")]
    ExternalTool { tool: String, message: String },

    /// Network errors
    #[error("Network error: {message}")]
    Network { message: String },

    /// Serialization errors
    #[error("Serialization error: {message}")]
    Serialization { message: String },

    /// Deserialization errors
    #[error("Deserialization error: {message}")]
    Deserialization { message: String },

    /// Internal errors
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl ActionError {
    /// Create a schema validation error
    pub fn schema_validation(message: impl Into<String>) -> Self {
        Self::SchemaValidation { message: message.into() }
    }

    /// Create an intent parsing error
    pub fn intent_parsing(message: impl Into<String>) -> Self {
        Self::IntentParsing { message: message.into() }
    }

    /// Create a file operation error
    pub fn file_operation(path: PathBuf, message: impl Into<String>) -> Self {
        Self::FileOperation { path, message: message.into() }
    }

    /// Create a tool execution error
    pub fn tool_execution(tool: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ToolExecution { tool: tool.into(), message: message.into() }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation { message: message.into() }
    }

    /// Create a safety check error
    pub fn safety_check(check: impl Into<String>, message: impl Into<String>) -> Self {
        Self::SafetyCheck { check: check.into(), message: message.into() }
    }

    /// Create a rollback error
    pub fn rollback(message: impl Into<String>) -> Self {
        Self::Rollback { message: message.into() }
    }

    /// Create an approval error
    pub fn approval(message: impl Into<String>) -> Self {
        Self::Approval { message: message.into() }
    }

    /// Create a git error
    pub fn git(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Git { operation: operation.into(), message: message.into() }
    }

    /// Create a pipeline error
    pub fn pipeline(stage: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Pipeline { stage: stage.into(), message: message.into() }
    }

    /// Create a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration { message: message.into() }
    }

    /// Create a permission error
    pub fn permission(message: impl Into<String>) -> Self {
        Self::Permission { message: message.into() }
    }

    /// Create a timeout error
    pub fn timeout(operation: impl Into<String>) -> Self {
        Self::Timeout { operation: operation.into() }
    }

    /// Create a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound { resource: resource.into() }
    }

    /// Create an invalid state error
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState { message: message.into() }
    }

    /// Create an external tool error
    pub fn external_tool(tool: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalTool { tool: tool.into(), message: message.into() }
    }

    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network { message: message.into() }
    }

    /// Create a serialization error
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::Serialization { message: message.into() }
    }

    /// Create a deserialization error
    pub fn deserialization(message: impl Into<String>) -> Self {
        Self::Deserialization { message: message.into() }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal { message: message.into() }
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::Validation { .. } |
            Self::SafetyCheck { .. } |
            Self::Timeout { .. } |
            Self::Network { .. } |
            Self::ExternalTool { .. }
        )
    }

    /// Check if this is a fatal error
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::SchemaValidation { .. } |
            Self::IntentParsing { .. } |
            Self::Configuration { .. } |
            Self::Permission { .. } |
            Self::InvalidState { .. } |
            Self::Internal { .. }
        )
    }

    /// Get error context for logging
    pub fn context(&self) -> String {
        match self {
            Self::SchemaValidation { message } => format!("Schema validation: {}", message),
            Self::IntentParsing { message } => format!("Intent parsing: {}", message),
            Self::FileOperation { path, message } => format!("File operation on {:?}: {}", path, message),
            Self::ToolExecution { tool, message } => format!("Tool execution for {}: {}", tool, message),
            Self::Validation { message } => format!("Validation: {}", message),
            Self::SafetyCheck { check, message } => format!("Safety check {}: {}", check, message),
            Self::Rollback { message } => format!("Rollback: {}", message),
            Self::Approval { message } => format!("Approval: {}", message),
            Self::Git { operation, message } => format!("Git {}: {}", operation, message),
            Self::Pipeline { stage, message } => format!("Pipeline {}: {}", stage, message),
            Self::Configuration { message } => format!("Configuration: {}", message),
            Self::Permission { message } => format!("Permission: {}", message),
            Self::Timeout { operation } => format!("Timeout: {}", operation),
            Self::NotFound { resource } => format!("Not found: {}", resource),
            Self::InvalidState { message } => format!("Invalid state: {}", message),
            Self::ExternalTool { tool, message } => format!("External tool {}: {}", tool, message),
            Self::Network { message } => format!("Network: {}", message),
            Self::Serialization { message } => format!("Serialization: {}", message),
            Self::Deserialization { message } => format!("Deserialization: {}", message),
            Self::Internal { message } => format!("Internal: {}", message),
        }
    }
}

/// Result type for Action Protocol operations
pub type ActionResult<T> = Result<T, ActionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ActionError::validation("Test validation error");
        assert!(matches!(error, ActionError::Validation { .. }));
        assert!(error.is_recoverable());
    }

    #[test]
    fn test_error_context() {
        let error = ActionError::tool_execution("jscodeshift", "Failed to execute");
        let context = error.context();
        assert!(context.contains("jscodeshift"));
        assert!(context.contains("Failed to execute"));
    }

    #[test]
    fn test_error_recoverability() {
        let recoverable = ActionError::validation("Test");
        assert!(recoverable.is_recoverable());
        assert!(!recoverable.is_fatal());

        let fatal = ActionError::configuration("Test");
        assert!(!fatal.is_recoverable());
        assert!(fatal.is_fatal());
    }
} 