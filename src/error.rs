use thiserror::Error;

/// Custom error type for GACP operations
#[derive(Error, Debug)]
pub enum GacpError {
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

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Result type for GACP operations
pub type GacpResult<T> = Result<T, GacpError>;

impl From<anyhow::Error> for GacpError {
    fn from(err: anyhow::Error) -> Self {
        GacpError::ConfigError(err.to_string())
    }
} 