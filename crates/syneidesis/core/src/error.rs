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

//! Core error types for the Syneidesis coordination ecosystem

use thiserror::Error;

/// Main error type for core operations
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid identifier: {id}")]
    InvalidIdentifier { id: String },

    #[error("Invalid state: {message}")]
    InvalidState { message: String },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

impl CoreError {
    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create an invalid identifier error
    pub fn invalid_identifier(id: impl Into<String>) -> Self {
        Self::InvalidIdentifier { id: id.into() }
    }

    /// Create an invalid state error
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState {
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// Create a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create an unknown error
    pub fn unknown(message: impl Into<String>) -> Self {
        Self::Unknown {
            message: message.into(),
        }
    }
}

/// Result type for core operations
pub type CoreResult<T> = Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let error = CoreError::validation("Invalid input");
        assert!(matches!(error, CoreError::Validation { message } if message == "Invalid input"));
    }

    #[test]
    fn test_invalid_identifier_error() {
        let error = CoreError::invalid_identifier("invalid-id");
        assert!(matches!(error, CoreError::InvalidIdentifier { id } if id == "invalid-id"));
    }

    #[test]
    fn test_invalid_state_error() {
        let error = CoreError::invalid_state("Agent is offline");
        assert!(
            matches!(error, CoreError::InvalidState { message } if message == "Agent is offline")
        );
    }

    #[test]
    fn test_not_found_error() {
        let error = CoreError::not_found("agent-123");
        assert!(matches!(error, CoreError::NotFound { resource } if resource == "agent-123"));
    }

    #[test]
    fn test_configuration_error() {
        let error = CoreError::configuration("Missing required field");
        assert!(
            matches!(error, CoreError::Configuration { message } if message == "Missing required field")
        );
    }

    #[test]
    fn test_unknown_error() {
        let error = CoreError::unknown("Something went wrong");
        assert!(
            matches!(error, CoreError::Unknown { message } if message == "Something went wrong")
        );
    }
}
