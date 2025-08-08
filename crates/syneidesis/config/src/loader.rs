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

//! Configuration loaders for different sources

use crate::error::ConfigError;
use crate::types::SyneidesisConfig;
use async_trait::async_trait;
use std::path::PathBuf;
use tracing::{debug, warn};

/// Trait for configuration loaders
#[async_trait]
pub trait ConfigLoader: Send + Sync {
    /// Load configuration from the source
    async fn load(&self) -> Result<SyneidesisConfig, ConfigError>;

    /// Get the name of the loader
    fn name(&self) -> &str;
}

/// File-based configuration loader
pub struct FileConfigLoader {
    /// File path
    file_path: PathBuf,
}

impl FileConfigLoader {
    /// Create a new file configuration loader
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Detect file format from extension
    fn detect_format(&self) -> Option<&str> {
        // First try to detect from file extension
        if let Some(format) = self
            .file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext.to_lowercase().as_str() {
                "yaml" | "yml" => Some("yaml"),
                "json" => Some("json"),
                "toml" => Some("toml"),
                _ => None,
            })
        {
            return Some(format);
        }

        // If no extension, try to detect from content
        if let Ok(content) = std::fs::read_to_string(&self.file_path) {
            let content = content.trim();
            if content.starts_with('{') || (content.starts_with('[') && content.contains('"')) {
                return Some("json");
            } else if content.starts_with("---") || content.contains(": ") {
                return Some("yaml");
            } else if content.starts_with('[') && content.contains(" = ") {
                return Some("toml");
            }
        }

        None
    }

    /// Load configuration from YAML file
    fn load_yaml(&self, content: &str) -> Result<SyneidesisConfig, ConfigError> {
        serde_yaml::from_str(content).map_err(|e| ConfigError::ParseError {
            format: "YAML".to_string(),
            path: self.file_path.clone(),
            source: Box::new(e),
        })
    }

    /// Load configuration from JSON file
    fn load_json(&self, content: &str) -> Result<SyneidesisConfig, ConfigError> {
        serde_json::from_str(content).map_err(|e| ConfigError::ParseError {
            format: "JSON".to_string(),
            path: self.file_path.clone(),
            source: Box::new(e),
        })
    }

    /// Load configuration from TOML file
    fn load_toml(&self, content: &str) -> Result<SyneidesisConfig, ConfigError> {
        toml::from_str(content).map_err(|e| ConfigError::ParseError {
            format: "TOML".to_string(),
            path: self.file_path.clone(),
            source: Box::new(e),
        })
    }
}

#[async_trait]
impl ConfigLoader for FileConfigLoader {
    async fn load(&self) -> Result<SyneidesisConfig, ConfigError> {
        debug!("Loading configuration from file: {:?}", self.file_path);

        // Check if file exists
        if !self.file_path.exists() {
            return Err(ConfigError::FileNotFound {
                path: self.file_path.clone(),
            });
        }

        // Read file content
        let content = tokio::fs::read_to_string(&self.file_path)
            .await
            .map_err(|e| ConfigError::FileReadError {
                path: self.file_path.clone(),
                source: e,
            })?;

        // Detect format and parse
        let format = self
            .detect_format()
            .ok_or_else(|| ConfigError::InvalidFormat {
                message: format!("Unknown file format for {:?}", self.file_path),
            })?;

        let config = match format {
            "yaml" => self.load_yaml(&content)?,
            "json" => self.load_json(&content)?,
            "toml" => self.load_toml(&content)?,
            _ => {
                return Err(ConfigError::InvalidFormat {
                    message: format!("Unsupported format: {format}"),
                });
            }
        };

        debug!(
            "Successfully loaded configuration from {:?}",
            self.file_path
        );
        Ok(config)
    }

    fn name(&self) -> &str {
        "FileConfigLoader"
    }
}

/// Environment variable configuration loader
pub struct EnvConfigLoader {
    /// Environment variable prefix
    prefix: String,
}

impl EnvConfigLoader {
    /// Create a new environment variable configuration loader
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    /// Convert environment variable name to configuration path
    fn env_to_path(&self, env_var: &str) -> String {
        env_var
            .strip_prefix(&self.prefix)
            .unwrap_or(env_var)
            .trim_start_matches('_') // Remove leading underscore if present
            .to_lowercase()
            .replace('_', ".")
            .replace(".max.agents", ".max_agents")
            .replace(".heartbeat.interval", ".heartbeat_interval")
            .replace(".agent.timeout", ".agent_timeout")
            .replace(".auto.discovery", ".auto_discovery")
            .replace(".load.balancing", ".load_balancing")
            .replace(".max.message.size", ".max_message_size")
            .replace(".connection.timeout", ".connection_timeout")
            .replace(".enable.metrics", ".enable_metrics")
            .replace(".enable.conflict.resolution", ".enable_conflict_resolution")
            .replace(".enable.state.sync", ".enable_state_sync")
    }

    /// Parse environment variable value
    fn parse_env_value(&self, value: &str) -> serde_json::Value {
        // Try to parse as different types
        if let Ok(int_val) = value.parse::<i64>() {
            return serde_json::Value::Number(int_val.into());
        }

        if let Ok(float_val) = value.parse::<f64>() {
            return serde_json::Value::Number(
                serde_json::Number::from_f64(float_val).unwrap_or(serde_json::Number::from(0)),
            );
        }

        if let Ok(bool_val) = value.parse::<bool>() {
            return serde_json::Value::Bool(bool_val);
        }

        // Default to string
        serde_json::Value::String(value.to_string())
    }

    /// Build configuration from environment variables
    fn build_config_from_env(&self) -> SyneidesisConfig {
        let mut config = SyneidesisConfig::default();

        for (key, value) in std::env::vars() {
            if key.starts_with(&self.prefix) {
                let path = self.env_to_path(&key);
                let parsed_value = self.parse_env_value(&value);

                // Apply the value to the configuration
                self.apply_env_value(&mut config, &path, parsed_value);
            }
        }

        config
    }

    /// Apply environment variable value to configuration
    fn apply_env_value(&self, config: &mut SyneidesisConfig, path: &str, value: serde_json::Value) {
        let path_parts: Vec<&str> = path.split('.').collect();

        match path_parts.as_slice() {
            ["system", field] => {
                if config.system.is_none() {
                    config.system = Some(Default::default());
                }
                if let Some(system_config) = &mut config.system {
                    match *field {
                        "name" => {
                            if let Some(name) = value.as_str() {
                                system_config.name = name.to_string();
                            }
                        }
                        "version" => {
                            if let Some(version) = value.as_str() {
                                system_config.version = version.to_string();
                            }
                        }
                        "environment" => {
                            if let Some(env) = value.as_str() {
                                system_config.environment = env.to_string();
                            }
                        }
                        "debug" => {
                            if let Some(debug) = value.as_bool() {
                                system_config.debug = debug;
                            }
                        }
                        "profiling" => {
                            if let Some(profiling) = value.as_bool() {
                                system_config.profiling = profiling;
                            }
                        }
                        "metrics_enabled" => {
                            if let Some(enabled) = value.as_bool() {
                                system_config.metrics_enabled = enabled;
                            }
                        }
                        _ => {
                            warn!("Unknown system environment variable: {}", field);
                        }
                    }
                }
            }
            ["agent", field] => {
                if config.agent.is_none() {
                    config.agent = Some(Default::default());
                }
                if let Some(agent_config) = &mut config.agent {
                    match *field {
                        "max_agents" => {
                            if let Some(max_agents) = value.as_u64() {
                                agent_config.max_agents = max_agents as usize;
                            }
                        }
                        "heartbeat_interval" => {
                            if let Some(interval) = value.as_u64() {
                                agent_config.heartbeat_interval =
                                    std::time::Duration::from_secs(interval);
                            }
                        }
                        "agent_timeout" => {
                            if let Some(timeout) = value.as_u64() {
                                agent_config.agent_timeout =
                                    std::time::Duration::from_secs(timeout);
                            }
                        }
                        "auto_discovery" => {
                            if let Some(discovery) = value.as_bool() {
                                agent_config.auto_discovery = discovery;
                            }
                        }
                        "load_balancing" => {
                            if let Some(lb) = value.as_bool() {
                                agent_config.load_balancing = lb;
                            }
                        }
                        "failover" => {
                            if let Some(failover) = value.as_bool() {
                                agent_config.failover = failover;
                            }
                        }
                        _ => {
                            warn!("Unknown agent environment variable: {}", field);
                        }
                    }
                }
            }
            ["coordination", field] => {
                if config.coordination.is_none() {
                    config.coordination = Some(Default::default());
                }
                if let Some(coord_config) = &mut config.coordination {
                    match *field {
                        "max_agents" => {
                            if let Some(max_agents) = value.as_u64() {
                                coord_config.max_agents = max_agents as usize;
                            }
                        }
                        "heartbeat_interval" => {
                            if let Some(interval) = value.as_u64() {
                                coord_config.heartbeat_interval = interval;
                            }
                        }
                        "agent_timeout" => {
                            if let Some(timeout) = value.as_u64() {
                                coord_config.agent_timeout = timeout;
                            }
                        }
                        "enable_metrics" => {
                            if let Some(enabled) = value.as_bool() {
                                coord_config.enable_metrics = enabled;
                            }
                        }
                        "enable_conflict_resolution" => {
                            if let Some(enabled) = value.as_bool() {
                                coord_config.enable_conflict_resolution = enabled;
                            }
                        }
                        "enable_state_sync" => {
                            if let Some(enabled) = value.as_bool() {
                                coord_config.enable_state_sync = enabled;
                            }
                        }
                        _ => {
                            warn!("Unknown coordination environment variable: {}", field);
                        }
                    }
                }
            }
            ["grpc", field] => {
                if config.grpc.is_none() {
                    config.grpc = Some(Default::default());
                }
                if let Some(grpc_config) = &mut config.grpc {
                    match *field {
                        "addr" => {
                            if let Some(addr) = value.as_str() {
                                grpc_config.addr = addr.to_string();
                            }
                        }
                        "max_message_size" => {
                            if let Some(size) = value.as_u64() {
                                grpc_config.max_message_size = size as usize;
                            }
                        }
                        "connection_timeout" => {
                            if let Some(timeout) = value.as_u64() {
                                grpc_config.connection_timeout = timeout;
                            }
                        }
                        "enable_reflection" => {
                            if let Some(enabled) = value.as_bool() {
                                grpc_config.enable_reflection = enabled;
                            }
                        }
                        "enable_health_checks" => {
                            if let Some(enabled) = value.as_bool() {
                                grpc_config.enable_health_checks = enabled;
                            }
                        }
                        "enable_metrics" => {
                            if let Some(enabled) = value.as_bool() {
                                grpc_config.enable_metrics = enabled;
                            }
                        }
                        "enable_tracing" => {
                            if let Some(enabled) = value.as_bool() {
                                grpc_config.enable_tracing = enabled;
                            }
                        }
                        _ => {
                            warn!("Unknown gRPC environment variable: {}", field);
                        }
                    }
                }
            }
            ["http", field] => {
                if config.http.is_none() {
                    config.http = Some(Default::default());
                }
                if let Some(http_config) = &mut config.http {
                    match *field {
                        "addr" => {
                            if let Some(addr) = value.as_str() {
                                http_config.addr = addr.to_string();
                            }
                        }
                        "port" => {
                            if let Some(port) = value.as_u64() {
                                http_config.port = port as u16;
                            }
                        }
                        "max_request_size" => {
                            if let Some(size) = value.as_u64() {
                                http_config.max_request_size = size as usize;
                            }
                        }
                        "request_timeout" => {
                            if let Some(timeout) = value.as_u64() {
                                http_config.request_timeout = timeout;
                            }
                        }
                        "enable_cors" => {
                            if let Some(enabled) = value.as_bool() {
                                http_config.enable_cors = enabled;
                            }
                        }
                        "enable_rate_limiting" => {
                            if let Some(enabled) = value.as_bool() {
                                http_config.enable_rate_limiting = enabled;
                            }
                        }
                        "enable_compression" => {
                            if let Some(enabled) = value.as_bool() {
                                http_config.enable_compression = enabled;
                            }
                        }
                        "enable_websocket" => {
                            if let Some(enabled) = value.as_bool() {
                                http_config.enable_websocket = enabled;
                            }
                        }
                        _ => {
                            warn!("Unknown HTTP environment variable: {}", field);
                        }
                    }
                }
            }
            ["logging", field] => {
                if config.logging.is_none() {
                    config.logging = Some(Default::default());
                }
                if let Some(logging_config) = &mut config.logging {
                    match *field {
                        "level" => {
                            if let Some(level) = value.as_str() {
                                logging_config.level = level.to_string();
                            }
                        }
                        "format" => {
                            if let Some(format) = value.as_str() {
                                logging_config.format = format.to_string();
                            }
                        }
                        "enable_console" => {
                            if let Some(enabled) = value.as_bool() {
                                logging_config.enable_console = enabled;
                            }
                        }
                        "enable_file" => {
                            if let Some(enabled) = value.as_bool() {
                                logging_config.enable_file = enabled;
                            }
                        }
                        "enable_json" => {
                            if let Some(enabled) = value.as_bool() {
                                logging_config.enable_json = enabled;
                            }
                        }
                        _ => {
                            warn!("Unknown logging environment variable: {}", field);
                        }
                    }
                }
            }
            _ => {
                // Store unknown environment variables in custom section
                config.custom.insert(path.to_string(), value);
            }
        }
    }
}

#[async_trait]
impl ConfigLoader for EnvConfigLoader {
    async fn load(&self) -> Result<SyneidesisConfig, ConfigError> {
        debug!(
            "Loading configuration from environment variables with prefix: {}",
            self.prefix
        );

        let config = self.build_config_from_env();

        debug!("Successfully loaded configuration from environment variables");
        Ok(config)
    }

    fn name(&self) -> &str {
        "EnvConfigLoader"
    }
}

/// Memory-based configuration loader
pub struct MemoryConfigLoader {
    /// Configuration data
    config: SyneidesisConfig,
}

impl MemoryConfigLoader {
    /// Create a new memory-based configuration loader
    pub fn new(config: SyneidesisConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ConfigLoader for MemoryConfigLoader {
    async fn load(&self) -> Result<SyneidesisConfig, ConfigError> {
        debug!("Loading configuration from memory");
        Ok(self.config.clone())
    }

    fn name(&self) -> &str {
        "MemoryConfigLoader"
    }
}

/// URL-based configuration loader
pub struct UrlConfigLoader {
    /// URL to load configuration from
    url: String,
}

impl UrlConfigLoader {
    /// Create a new URL-based configuration loader
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[async_trait]
impl ConfigLoader for UrlConfigLoader {
    async fn load(&self) -> Result<SyneidesisConfig, ConfigError> {
        debug!("Loading configuration from URL: {}", self.url);

        // This is a placeholder implementation
        // In a real implementation, this would make HTTP requests
        // and parse the response based on content type

        Err(ConfigError::InitError {
            message: "URL-based configuration loading not yet implemented".to_string(),
        })
    }

    fn name(&self) -> &str {
        "UrlConfigLoader"
    }
}

/// Database-based configuration loader
pub struct DatabaseConfigLoader {
    /// Database connection string
    connection_string: String,

    /// Configuration table name
    table_name: String,
}

impl DatabaseConfigLoader {
    /// Create a new database-based configuration loader
    pub fn new(connection_string: String, table_name: String) -> Self {
        Self {
            connection_string,
            table_name,
        }
    }
}

#[async_trait]
impl ConfigLoader for DatabaseConfigLoader {
    async fn load(&self) -> Result<SyneidesisConfig, ConfigError> {
        debug!(
            "Loading configuration from database: {}",
            self.connection_string
        );

        // This is a placeholder implementation
        // In a real implementation, this would connect to the database
        // and load configuration from the specified table

        Err(ConfigError::InitError {
            message: "Database-based configuration loading not yet implemented".to_string(),
        })
    }

    fn name(&self) -> &str {
        "DatabaseConfigLoader"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_file_config_loader_yaml() {
        let temp_file = NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .with_extension("yaml");
        let config_content = r#"
system:
  name: "test-system"
  version: "1.0.0"
  environment: "test"
agent:
  max_agents: 100
  heartbeat_interval:
    secs: 30
    nanos: 0
"#;
        fs::write(&temp_file, config_content).unwrap();

        let loader = FileConfigLoader::new(temp_file.to_path_buf());
        let config = loader.load().await.unwrap();

        let system = config.system.as_ref().unwrap();
        assert_eq!(system.name, "test-system");
        assert_eq!(system.version, "1.0.0");
        assert_eq!(system.environment, "test");
        let agent = config.agent.as_ref().unwrap();
        assert_eq!(agent.max_agents, 100);
    }

    #[tokio::test]
    async fn test_file_config_loader_json() {
        let temp_file = NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .with_extension("json");
        let config_content = r#"{
  "system": {
    "name": "test-system",
    "version": "1.0.0",
    "environment": "test"
  },
  "agent": {
    "max_agents": 100,
    "heartbeat_interval": {
      "secs": 30,
      "nanos": 0
    }
  }
}"#;
        fs::write(&temp_file, config_content).unwrap();

        let loader = FileConfigLoader::new(temp_file.to_path_buf());
        let config = loader.load().await.unwrap();

        let system = config.system.as_ref().unwrap();
        assert_eq!(system.name, "test-system");
        let agent = config.agent.as_ref().unwrap();
        assert_eq!(agent.max_agents, 100);
    }

    #[tokio::test]
    async fn test_file_config_loader_toml() {
        let temp_file = NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .with_extension("toml");
        let config_content = r#"
[system]
name = "test-system"
version = "1.0.0"
environment = "test"

[agent]
max_agents = 100
heartbeat_interval = { secs = 30, nanos = 0 }
"#;
        fs::write(&temp_file, config_content).unwrap();

        let loader = FileConfigLoader::new(temp_file.to_path_buf());
        let config = loader.load().await.unwrap();

        let system = config.system.as_ref().unwrap();
        assert_eq!(system.name, "test-system");
        let agent = config.agent.as_ref().unwrap();
        assert_eq!(agent.max_agents, 100);
    }

    #[tokio::test]
    async fn test_file_config_loader_not_found() {
        let loader = FileConfigLoader::new(PathBuf::from("nonexistent.yaml"));
        let result = loader.load().await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::FileNotFound { .. }
        ));
    }

    #[tokio::test]
    async fn test_env_config_loader() {
        // Set environment variables
        std::env::set_var("SYNEIDESIS_SYSTEM_NAME", "env-system");
        std::env::set_var("SYNEIDESIS_SYSTEM_VERSION", "2.0.0");
        std::env::set_var("SYNEIDESIS_AGENT_MAX_AGENTS", "200");
        std::env::set_var("SYNEIDESIS_AGENT_HEARTBEAT_INTERVAL", "60");
        std::env::set_var("SYNEIDESIS_GRPC_ADDR", "0.0.0.0:50051");

        let loader = EnvConfigLoader::new("SYNEIDESIS");
        let config = loader.load().await.unwrap();

        let system = config.system.as_ref().unwrap();
        assert_eq!(system.name, "env-system");
        assert_eq!(system.version, "2.0.0");
        let agent = config.agent.as_ref().unwrap();
        assert_eq!(agent.max_agents, 200);
        assert_eq!(agent.heartbeat_interval.as_secs(), 60);
        assert_eq!(config.grpc.unwrap().addr, "0.0.0.0:50051");

        // Clean up environment variables
        std::env::remove_var("SYNEIDESIS_SYSTEM_NAME");
        std::env::remove_var("SYNEIDESIS_SYSTEM_VERSION");
        std::env::remove_var("SYNEIDESIS_AGENT_MAX_AGENTS");
        std::env::remove_var("SYNEIDESIS_AGENT_HEARTBEAT_INTERVAL");
        std::env::remove_var("SYNEIDESIS_GRPC_ADDR");
    }

    #[tokio::test]
    async fn test_memory_config_loader() {
        let test_config = SyneidesisConfig::default();
        let loader = MemoryConfigLoader::new(test_config.clone());
        let config = loader.load().await.unwrap();

        let system = config.system.as_ref().unwrap();
        let test_system = test_config.system.as_ref().unwrap();
        assert_eq!(system.name, test_system.name);
    }

    #[tokio::test]
    async fn test_loader_names() {
        let file_loader = FileConfigLoader::new(PathBuf::from("test.yaml"));
        assert_eq!(file_loader.name(), "FileConfigLoader");

        let env_loader = EnvConfigLoader::new("TEST");
        assert_eq!(env_loader.name(), "EnvConfigLoader");

        let memory_loader = MemoryConfigLoader::new(SyneidesisConfig::default());
        assert_eq!(memory_loader.name(), "MemoryConfigLoader");

        let url_loader = UrlConfigLoader::new("http://example.com/config".to_string());
        assert_eq!(url_loader.name(), "UrlConfigLoader");

        let db_loader =
            DatabaseConfigLoader::new("sqlite://test.db".to_string(), "config".to_string());
        assert_eq!(db_loader.name(), "DatabaseConfigLoader");
    }
}
