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

//! Error types for the Syneidesis coordination library

use syneidesis_grpc::types::GrpcError;
use thiserror::Error;

/// Main error type for coordination operations
#[derive(Error, Debug)]
pub enum CoordinationError {
    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),

    #[error("Conflict error: {0}")]
    Conflict(#[from] ConflictError),

    #[error("State synchronization error: {0}")]
    StateSync(#[from] StateSyncError),

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Communication error: {message}")]
    Communication { message: String },

    #[error("Initialization error: {message}")]
    Initialization { message: String },

    #[error("Timeout error: {operation}")]
    Timeout { operation: String },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Invalid state: {message}")]
    InvalidState { message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // WebSocket and HTTP error variants removed; see syneidesis-http crate for implementation.
    #[error("Unknown error: {message}")]
    Unknown { message: String },

    #[error("gRPC error: {0}")]
    Grpc(#[from] GrpcError),
}

/// Agent-specific errors
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Agent not found: {agent_id}")]
    NotFound { agent_id: String },

    #[error("Agent already exists: {agent_id}")]
    AlreadyExists { agent_id: String },

    #[error("Agent is offline: {agent_id}")]
    Offline { agent_id: String },

    #[error("Agent is unhealthy: {agent_id}")]
    Unhealthy { agent_id: String },

    #[error("Agent validation failed: {message}")]
    ValidationFailed { message: String },

    #[error("Agent registration failed: {message}")]
    RegistrationFailed { message: String },

    #[error("Agent heartbeat timeout: {agent_id}")]
    HeartbeatTimeout { agent_id: String },

    #[error("Agent capability not supported: {capability}")]
    CapabilityNotSupported { capability: String },

    #[error("Agent task assignment failed: {message}")]
    TaskAssignmentFailed { message: String },

    #[error("Agent state update failed: {message}")]
    StateUpdateFailed { message: String },
}

/// Conflict resolution errors
#[derive(Error, Debug)]
pub enum ConflictError {
    #[error("Conflict detection failed: {message}")]
    DetectionFailed { message: String },

    #[error("Conflict resolution failed: {message}")]
    ResolutionFailed { message: String },

    #[error("Unsupported conflict strategy: {strategy}")]
    UnsupportedStrategy { strategy: String },

    #[error("Conflict handler not found: {handler_name}")]
    HandlerNotFound { handler_name: String },

    #[error("Conflict resolution timeout: {conflict_id}")]
    ResolutionTimeout { conflict_id: String },

    #[error("Manual resolution required: {conflict_id}")]
    ManualResolutionRequired { conflict_id: String },

    #[error("Conflict history error: {message}")]
    HistoryError { message: String },
}

/// State synchronization errors
#[derive(Error, Debug)]
pub enum StateSyncError {
    #[error("State synchronization failed: {message}")]
    SyncFailed { message: String },

    #[error("State validation failed: {message}")]
    ValidationFailed { message: String },

    #[error("State merge conflict: {conflict_id}")]
    MergeConflict { conflict_id: String },

    #[error("State version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u64, actual: u64 },

    #[error("State corruption detected: {message}")]
    CorruptionDetected { message: String },

    #[error("State backup failed: {message}")]
    BackupFailed { message: String },

    #[error("State restore failed: {message}")]
    RestoreFailed { message: String },
}

impl From<CoordinationError> for std::io::Error {
    fn from(err: CoordinationError) -> Self {
        std::io::Error::other(err)
    }
}

/// Result type for coordination operations
pub type CoordinationResult<T> = Result<T, CoordinationError>;

/// Result type for agent operations
pub type AgentResult<T> = Result<T, AgentError>;

/// Result type for conflict resolution operations
pub type ConflictResult<T> = Result<T, ConflictError>;

/// Result type for state synchronization operations
pub type StateSyncResult<T> = Result<T, StateSyncError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordination_error_display() {
        let error = CoordinationError::Configuration {
            message: "Invalid config".to_string(),
        };
        assert!(error.to_string().contains("Invalid config"));
    }

    #[test]
    fn test_agent_error_display() {
        let error = AgentError::NotFound {
            agent_id: "test-agent".to_string(),
        };
        assert!(error.to_string().contains("test-agent"));
    }

    #[test]
    fn test_conflict_error_display() {
        let error = ConflictError::DetectionFailed {
            message: "Test conflict".to_string(),
        };
        assert!(error.to_string().contains("Test conflict"));
    }

    #[test]
    fn test_state_sync_error_display() {
        let error = StateSyncError::SyncFailed {
            message: "Test sync".to_string(),
        };
        assert!(error.to_string().contains("Test sync"));
    }

    #[test]
    fn test_error_conversions() {
        let agent_error = AgentError::NotFound {
            agent_id: "test".to_string(),
        };
        let coordination_error: CoordinationError = agent_error.into();
        assert!(matches!(coordination_error, CoordinationError::Agent(_)));
    }
}
