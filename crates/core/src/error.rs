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

/// Custom error type for Rhema operations
#[derive(Error, Debug)]
pub enum RhemaError {
    #[error("Git repository not found: {0}")]
    GitRepoNotFound(String),

    #[error("Invalid YAML file {file}: {message}")]
    InvalidYaml { file: String, message: String },

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Scope not found: {0}")]
    ScopeNotFound(String),

    #[error("Invalid query syntax: {0}")]
    InvalidQuery(String),

    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Hook execution failed: {0}")]
    HookError(String),

    #[error("Branch context error: {0}")]
    BranchContextError(String),

    #[error("Workflow error: {0}")]
    WorkflowError(String),

    #[error("Automation error: {0}")]
    AutomationError(String),

    #[error("Context conflict detected: {0}")]
    ContextConflict(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Notification error: {0}")]
    NotificationError(String),

    #[error("Backup error: {0}")]
    BackupError(String),

    #[error("Sync error: {0}")]
    SyncError(String),

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Integration error: {0}")]
    IntegrationError(String),

    #[error("Performance error: {0}")]
    PerformanceError(String),

    #[error("Monitoring error: {0}")]
    MonitoringError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("MCP error: {0}")]
    McpError(String),

    #[error("Daemon error: {0}")]
    DaemonError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Client error: {0}")]
    ClientError(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Watcher error: {0}")]
    WatcherError(String),

    #[error("Auth error: {0}")]
    AuthError(String),

    #[error("Context error: {0}")]
    ContextError(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Agent error: {0}")]
    AgentError(String),

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Sync error: {0}")]
    CoordinationError(String),

    #[error("Safety violation: {0}")]
    SafetyViolation(String),
}

/// Result type for Rhema operations
pub type RhemaResult<T> = Result<T, RhemaError>;

impl From<anyhow::Error> for RhemaError {
    fn from(err: anyhow::Error) -> Self {
        RhemaError::ConfigError(err.to_string())
    }
}

impl From<reqwest::Error> for RhemaError {
    fn from(err: reqwest::Error) -> Self {
        RhemaError::NetworkError(err.to_string())
    }
}

impl From<redis::RedisError> for RhemaError {
    fn from(err: redis::RedisError) -> Self {
        RhemaError::CacheError(err.to_string())
    }
}

impl From<notify::Error> for RhemaError {
    fn from(err: notify::Error) -> Self {
        RhemaError::WatcherError(err.to_string())
    }
}

impl From<toml::de::Error> for RhemaError {
    fn from(err: toml::de::Error) -> Self {
        RhemaError::ConfigError(err.to_string())
    }
}

impl From<toml::ser::Error> for RhemaError {
    fn from(err: toml::ser::Error) -> Self {
        RhemaError::ConfigError(err.to_string())
    }
}

impl From<Box<bincode::ErrorKind>> for RhemaError {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        RhemaError::CacheError(err.to_string())
    }
}

impl From<prometheus::Error> for RhemaError {
    fn from(err: prometheus::Error) -> Self {
        RhemaError::MonitoringError(err.to_string())
    }
}

impl From<rustyline::error::ReadlineError> for RhemaError {
    fn from(err: rustyline::error::ReadlineError) -> Self {
        RhemaError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        ))
    }
}
