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
use std::fmt;

/// Agent framework error types
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Agent not found: {agent_id}")]
    AgentNotFound { agent_id: String },

    #[error("Agent already exists: {agent_id}")]
    AgentAlreadyExists { agent_id: String },

    #[error("Agent is not active: {agent_id}")]
    AgentNotActive { agent_id: String },

    #[error("Agent is already active: {agent_id}")]
    AgentAlreadyActive { agent_id: String },

    #[error("Invalid agent configuration: {reason}")]
    InvalidConfiguration { reason: String },

    #[error("Agent execution failed: {reason}")]
    ExecutionFailed { reason: String },

    #[error("Agent communication failed: {reason}")]
    CommunicationFailed { reason: String },

    #[error("Agent coordination failed: {reason}")]
    CoordinationFailed { reason: String },

    #[error("Agent capability not available: {capability}")]
    CapabilityNotAvailable { capability: String },

    #[error("Policy violation: {violation}")]
    PolicyViolation { violation: String },

    #[error("Agent lifecycle error: {reason}")]
    LifecycleError { reason: String },

    #[error("Agent registry error: {reason}")]
    RegistryError { reason: String },

    #[error("Message broker error: {reason}")]
    MessageBrokerError { reason: String },

    #[error("Metrics collection error: {reason}")]
    MetricsError { reason: String },

    #[error("Agent timeout: {agent_id}")]
    AgentTimeout { agent_id: String },

    #[error("Agent deadlock detected: {agent_id}")]
    AgentDeadlock { agent_id: String },

    #[error("Agent resource exhaustion: {resource}")]
    ResourceExhaustion { resource: String },

    #[error("Agent initialization failed: {reason}")]
    InitializationFailed { reason: String },

    #[error("Agent shutdown failed: {reason}")]
    ShutdownFailed { reason: String },

    #[error("Agent serialization error: {reason}")]
    SerializationError { reason: String },

    #[error("Agent deserialization error: {reason}")]
    DeserializationError { reason: String },

    #[error("Agent validation error: {reason}")]
    ValidationError { reason: String },

    #[error("Agent permission denied: {operation}")]
    PermissionDenied { operation: String },

    #[error("Agent quota exceeded: {quota}")]
    QuotaExceeded { quota: String },

    #[error("Agent network error: {reason}")]
    NetworkError { reason: String },

    #[error("Agent storage error: {reason}")]
    StorageError { reason: String },

    #[error("Agent workflow error: {reason}")]
    WorkflowError { reason: String },

    #[error("Agent internal error: {reason}")]
    InternalError { reason: String },

    #[error("Agent unknown error: {reason}")]
    Unknown { reason: String },
}

impl AgentError {
    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AgentError::AgentTimeout { .. }
                | AgentError::CommunicationFailed { .. }
                | AgentError::NetworkError { .. }
                | AgentError::ResourceExhaustion { .. }
        )
    }

    /// Check if the error is fatal
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            AgentError::AgentDeadlock { .. }
                | AgentError::InternalError { .. }
                | AgentError::InitializationFailed { .. }
        )
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AgentError::AgentNotFound { .. }
            | AgentError::AgentAlreadyExists { .. }
            | AgentError::AgentNotActive { .. }
            | AgentError::AgentAlreadyActive { .. }
            | AgentError::InvalidConfiguration { .. }
            | AgentError::ValidationError { .. }
            | AgentError::PermissionDenied { .. } => ErrorSeverity::Warning,

            AgentError::ExecutionFailed { .. }
            | AgentError::CommunicationFailed { .. }
            | AgentError::CoordinationFailed { .. }
            | AgentError::CapabilityNotAvailable { .. }
            | AgentError::PolicyViolation { .. }
            | AgentError::LifecycleError { .. }
            | AgentError::RegistryError { .. }
            | AgentError::MessageBrokerError { .. }
            | AgentError::MetricsError { .. }
            | AgentError::AgentTimeout { .. }
            | AgentError::ResourceExhaustion { .. }
            | AgentError::QuotaExceeded { .. }
            | AgentError::NetworkError { .. }
            | AgentError::StorageError { .. }
            | AgentError::SerializationError { .. }
            | AgentError::DeserializationError { .. } => ErrorSeverity::Error,

            AgentError::AgentDeadlock { .. }
            | AgentError::InternalError { .. }
            | AgentError::InitializationFailed { .. }
            | AgentError::ShutdownFailed { .. }
            | AgentError::Unknown { .. } => ErrorSeverity::Critical,
            | AgentError::WorkflowError { .. } => ErrorSeverity::Error,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Warning => write!(f, "Warning"),
            ErrorSeverity::Error => write!(f, "Error"),
            ErrorSeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Result type for agent operations
pub type AgentResult<T> = Result<T, AgentError>;

/// Error context for additional information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub agent_id: Option<String>,
    pub operation: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            agent_id: None,
            operation: None,
            timestamp: chrono::Utc::now(),
            additional_info: std::collections::HashMap::new(),
        }
    }

    pub fn with_agent_id(mut self, agent_id: String) -> Self {
        self.agent_id = Some(agent_id);
        self
    }

    pub fn with_operation(mut self, operation: String) -> Self {
        self.operation = Some(operation);
        self
    }

    pub fn with_info(mut self, key: String, value: String) -> Self {
        self.additional_info.insert(key, value);
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced error with context
#[derive(Debug)]
pub struct ContextualAgentError {
    pub error: AgentError,
    pub context: ErrorContext,
}

impl ContextualAgentError {
    pub fn new(error: AgentError, context: ErrorContext) -> Self {
        Self { error, context }
    }

    pub fn with_agent_id(mut self, agent_id: String) -> Self {
        self.context = self.context.with_agent_id(agent_id);
        self
    }

    pub fn with_operation(mut self, operation: String) -> Self {
        self.context = self.context.with_operation(operation);
        self
    }
}

impl fmt::Display for ContextualAgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        
        if let Some(ref agent_id) = self.context.agent_id {
            write!(f, " (Agent: {})", agent_id)?;
        }
        
        if let Some(ref operation) = self.context.operation {
            write!(f, " (Operation: {})", operation)?;
        }
        
        write!(f, " at {}", self.context.timestamp)
    }
}

impl std::error::Error for ContextualAgentError {}

/// Result type for contextual agent operations
pub type ContextualAgentResult<T> = Result<T, ContextualAgentError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity() {
        let error = AgentError::AgentNotFound {
            agent_id: "test".to_string(),
        };
        assert_eq!(error.severity(), ErrorSeverity::Warning);
        assert!(error.is_recoverable() == false);
        assert!(error.is_fatal() == false);
    }

    #[test]
    fn test_fatal_error() {
        let error = AgentError::InternalError {
            reason: "test".to_string(),
        };
        assert_eq!(error.severity(), ErrorSeverity::Critical);
        assert!(error.is_fatal());
    }

    #[test]
    fn test_recoverable_error() {
        let error = AgentError::AgentTimeout {
            agent_id: "test".to_string(),
        };
        assert!(error.is_recoverable());
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new()
            .with_agent_id("test-agent".to_string())
            .with_operation("test-operation".to_string())
            .with_info("key".to_string(), "value".to_string());

        assert_eq!(context.agent_id, Some("test-agent".to_string()));
        assert_eq!(context.operation, Some("test-operation".to_string()));
        assert_eq!(context.additional_info.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_contextual_error() {
        let error = AgentError::AgentNotFound {
            agent_id: "test".to_string(),
        };
        let context = ErrorContext::new().with_agent_id("test-agent".to_string());
        let contextual_error = ContextualAgentError::new(error, context);

        assert!(contextual_error.to_string().contains("test-agent"));
    }
} 