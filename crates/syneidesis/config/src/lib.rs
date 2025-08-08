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

//! # Syneidesis Configuration Management
//!
//! A centralized configuration management system for the Syneidesis ecosystem.
//! This crate provides comprehensive configuration loading, validation, and
//! management capabilities across all Syneidesis components.
//!
//! ## Features
//!
//! - **Multi-format Support**: YAML, JSON, TOML, and environment variables
//! - **Validation**: Schema-based configuration validation
//! - **Environment Integration**: Seamless environment variable support
//! - **Hot Reloading**: Dynamic configuration updates
//! - **Type Safety**: Strongly typed configuration structures
//! - **Default Values**: Comprehensive default configurations
//!
//! ## Quick Start
//!
//! ```rust
//! use syneidesis_config::{ConfigManager, ConfigBuilder};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create configuration manager
//!     let config_manager = ConfigBuilder::new()
//!         .with_file("config.yaml")
//!         .with_env_prefix("SYNEIDESIS")
//!         .with_defaults()
//!         .build()
//!         .await?;
//!
//!     // Load configuration
//!     let config = config_manager.load().await?;
//!
//!     println!("Configuration loaded: {:?}", config);
//!     Ok(())
//! }
//! ```

pub mod builder;
pub mod error;
pub mod loader;
pub mod manager;
pub mod types;
pub mod validation;

// Re-export main types for easy access
pub use builder::ConfigBuilder;
pub use error::{ConfigError, ValidationError};
pub use loader::{ConfigLoader, EnvConfigLoader, FileConfigLoader};
pub use manager::ConfigManager;
pub use types::{
    AgentConfig, CoordinationConfig, GrpcClientConfig, GrpcConfig, HttpConfig, LoggingConfig,
    NetworkConfig, SecurityConfig, SystemConfig, ValidationConfig, WebSocketConfig,
};
pub use validation::{ConfigValidator, ValidationRule};

/// Default configuration file paths
pub const DEFAULT_CONFIG_PATHS: &[&str] = &[
    "config.yaml",
    "config.yml",
    "config.json",
    "config.toml",
    "syneidesis.yaml",
    "syneidesis.yml",
    "syneidesis.json",
    "syneidesis.toml",
];

/// Default environment variable prefix
pub const DEFAULT_ENV_PREFIX: &str = "SYNEIDESIS";

/// Default configuration directory
pub const DEFAULT_CONFIG_DIR: &str = "config";

/// Initialize the configuration system with default settings
pub async fn init() -> Result<ConfigManager, ConfigError> {
    ConfigBuilder::new().with_defaults().build().await
}

/// Initialize the configuration system with a specific config file
pub async fn init_with_file(config_path: &str) -> Result<ConfigManager, ConfigError> {
    ConfigBuilder::new()
        .with_file(config_path)
        .with_defaults()
        .build()
        .await
}

/// Initialize the configuration system with environment variables
pub async fn init_with_env(env_prefix: &str) -> Result<ConfigManager, ConfigError> {
    ConfigBuilder::new()
        .with_env_prefix(env_prefix)
        .with_defaults()
        .build()
        .await
}

/// Create a configuration builder for custom configuration
pub fn builder() -> ConfigBuilder {
    ConfigBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_init_with_defaults() {
        let config_manager = init().await.unwrap();
        let config = config_manager.load().await.unwrap();
        assert!(config.system.is_some());
    }

    #[tokio::test]
    async fn test_init_with_file() {
        let temp_file = NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .with_extension("yaml");
        let config_content = r#"
system:
  name: "test-system"
  version: "1.0.0"
"#;
        fs::write(&temp_file, config_content).unwrap();

        let config_manager = init_with_file(temp_file.to_str().unwrap()).await.unwrap();
        let config = config_manager.load().await.unwrap();
        assert_eq!(config.system.unwrap().name, "test-system");
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let config_manager = builder()
            .with_file("nonexistent.yaml")
            .with_defaults()
            .build()
            .await
            .unwrap();

        let config = config_manager.load().await.unwrap();
        assert!(config.system.is_some());
    }
}
