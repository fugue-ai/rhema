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

//! Error types for the agent crate

use thiserror::Error;

/// Agent-related errors
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Agent not found: {agent_id}")]
    NotFound { agent_id: String },

    #[error("Agent already exists: {agent_id}")]
    AlreadyExists { agent_id: String },

    #[error("Agent registration failed: {message}")]
    RegistrationFailed { message: String },

    #[error("Task assignment failed: {message}")]
    TaskAssignmentFailed { message: String },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

/// Coordination-related errors
#[derive(Error, Debug)]
pub enum CoordinationError {
    #[error("Communication error: {message}")]
    Communication { message: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

/// Conflict resolution errors
#[derive(Error, Debug)]
pub enum ConflictError {
    #[error("Conflict detection failed: {message}")]
    DetectionFailed { message: String },

    #[error("Conflict resolution failed: {message}")]
    ResolutionFailed { message: String },

    #[error("Manual resolution required for conflict: {conflict_id}")]
    ManualResolutionRequired { conflict_id: String },

    #[error("Handler not found: {handler_name}")]
    HandlerNotFound { handler_name: String },

    #[error("Unsupported strategy: {strategy}")]
    UnsupportedStrategy { strategy: String },

    #[error("History error: {message}")]
    HistoryError { message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unknown error: {message}")]
    Unknown { message: String },
}
