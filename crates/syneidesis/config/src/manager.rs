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

//! Configuration manager for loading and managing configuration

use crate::error::ConfigError;
use crate::loader::{ConfigLoader, EnvConfigLoader, FileConfigLoader};
use crate::types::SyneidesisConfig;
use crate::validation::ConfigValidator;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Configuration manager for loading and managing configuration
pub struct ConfigManager {
    /// Current configuration
    config: Arc<RwLock<Option<SyneidesisConfig>>>,

    /// Default configuration
    default_config: Arc<RwLock<Option<SyneidesisConfig>>>,

    /// Configuration loaders
    loaders: Vec<Box<dyn ConfigLoader + Send + Sync>>,

    /// Configuration validator
    validator: Option<ConfigValidator>,

    /// Environment variable prefix
    env_prefix: String,

    /// Custom configuration values
    custom_values: Arc<RwLock<HashMap<String, Value>>>,

    /// Configuration files
    config_files: Arc<RwLock<Vec<PathBuf>>>,

    /// Search paths for configuration files
    search_paths: Arc<RwLock<Vec<PathBuf>>>,

    /// Configuration file formats to try
    formats: Arc<RwLock<Vec<String>>>,

    /// Template variables
    template_vars: Arc<RwLock<HashMap<String, String>>>,

    /// Configuration settings
    settings: Arc<RwLock<ConfigSettings>>,

    /// Statistics
    statistics: Arc<RwLock<ConfigStatistics>>,
}

/// Configuration settings
#[derive(Debug, Clone)]
struct ConfigSettings {
    /// Enable validation
    validation_enabled: bool,

    /// Enable hot reloading
    hot_reload_enabled: bool,

    /// Configuration directory
    config_dir: Option<PathBuf>,

    /// Enable environment variable substitution
    env_substitution_enabled: bool,

    /// Enable template processing
    template_processing_enabled: bool,

    /// Enable configuration encryption
    encryption_enabled: bool,

    /// Encryption key
    encryption_key: Option<String>,

    /// Enable configuration backup
    backup_enabled: bool,

    /// Backup directory
    backup_dir: Option<PathBuf>,

    /// Enable configuration migration
    migration_enabled: bool,

    /// Migration scripts directory
    migration_dir: Option<PathBuf>,

    /// Enable configuration caching
    caching_enabled: bool,

    /// Cache directory
    cache_dir: Option<PathBuf>,

    /// Cache TTL in seconds
    cache_ttl: u64,

    /// Enable configuration monitoring
    monitoring_enabled: bool,

    /// Monitoring interval in seconds
    monitoring_interval: u64,

    /// Enable configuration statistics
    statistics_enabled: bool,

    /// Statistics collection interval in seconds
    statistics_interval: u64,
}

/// Configuration statistics
#[derive(Debug, Clone, Default)]
struct ConfigStatistics {
    /// Number of configuration loads
    load_count: u64,

    /// Number of configuration reloads
    reload_count: u64,

    /// Number of validation errors
    validation_errors: u64,

    /// Number of file read errors
    file_read_errors: u64,

    /// Number of parse errors
    parse_errors: u64,

    /// Last load time
    last_load_time: Option<chrono::DateTime<chrono::Utc>>,

    /// Last reload time
    last_reload_time: Option<chrono::DateTime<chrono::Utc>>,

    /// Total load time in milliseconds
    total_load_time_ms: u64,

    /// Average load time in milliseconds
    avg_load_time_ms: u64,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(None)),
            default_config: Arc::new(RwLock::new(None)),
            loaders: Vec::new(),
            validator: None,
            env_prefix: "SYNEIDESIS".to_string(),
            custom_values: Arc::new(RwLock::new(HashMap::new())),
            config_files: Arc::new(RwLock::new(Vec::new())),
            search_paths: Arc::new(RwLock::new(Vec::new())),
            formats: Arc::new(RwLock::new(vec![
                "yaml".to_string(),
                "yml".to_string(),
                "json".to_string(),
                "toml".to_string(),
            ])),
            template_vars: Arc::new(RwLock::new(HashMap::new())),
            settings: Arc::new(RwLock::new(ConfigSettings {
                validation_enabled: true,
                hot_reload_enabled: false,
                config_dir: None,
                env_substitution_enabled: true,
                template_processing_enabled: false,
                encryption_enabled: false,
                encryption_key: None,
                backup_enabled: false,
                backup_dir: None,
                migration_enabled: false,
                migration_dir: None,
                caching_enabled: true,
                cache_dir: None,
                cache_ttl: 3600,
                monitoring_enabled: false,
                monitoring_interval: 60,
                statistics_enabled: false,
                statistics_interval: 300,
            })),
            statistics: Arc::new(RwLock::new(ConfigStatistics::default())),
        }
    }

    /// Set the environment variable prefix
    pub fn set_env_prefix(&mut self, prefix: &str) {
        self.env_prefix = prefix.to_string();
    }

    /// Get the environment variable prefix
    pub fn get_env_prefix(&self) -> &str {
        &self.env_prefix
    }

    /// Set the default configuration
    pub async fn set_default_config(&self, config: SyneidesisConfig) {
        let mut default_config = self.default_config.write().await;
        *default_config = Some(config);
    }

    /// Get the default configuration
    pub async fn get_default_config(&self) -> Option<SyneidesisConfig> {
        let default_config = self.default_config.read().await;
        default_config.clone()
    }

    /// Add a custom configuration value
    pub async fn add_custom_value(&self, key: &str, value: Value) {
        let mut custom_values = self.custom_values.write().await;
        custom_values.insert(key.to_string(), value);
    }

    /// Get a custom configuration value
    pub async fn get_custom_value(&self, key: &str) -> Option<Value> {
        let custom_values = self.custom_values.read().await;
        custom_values.get(key).cloned()
    }

    /// Set validation enabled
    pub async fn set_validation_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.validation_enabled = enabled;
    }

    /// Check if validation is enabled
    pub async fn is_validation_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.validation_enabled
    }

    /// Set hot reload enabled
    pub async fn set_hot_reload_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.hot_reload_enabled = enabled;
    }

    /// Check if hot reload is enabled
    pub async fn is_hot_reload_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.hot_reload_enabled
    }

    /// Set configuration directory
    pub async fn set_config_dir(&self, dir: &PathBuf) {
        let mut settings = self.settings.write().await;
        settings.config_dir = Some(dir.clone());
    }

    /// Get configuration directory
    pub async fn get_config_dir(&self) -> Option<PathBuf> {
        let settings = self.settings.read().await;
        settings.config_dir.clone()
    }

    /// Add a search path
    pub async fn add_search_path(&self, path: &PathBuf) {
        let mut search_paths = self.search_paths.write().await;
        search_paths.push(path.clone());
    }

    /// Get search paths
    pub async fn get_search_paths(&self) -> Vec<PathBuf> {
        let search_paths = self.search_paths.read().await;
        search_paths.clone()
    }

    /// Set file formats
    pub async fn set_formats(&self, formats: &[String]) {
        let mut formats_guard = self.formats.write().await;
        *formats_guard = formats.to_vec();
    }

    /// Get file formats
    pub async fn get_formats(&self) -> Vec<String> {
        let formats = self.formats.read().await;
        formats.clone()
    }

    /// Set environment variable substitution enabled
    pub async fn set_env_substitution_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.env_substitution_enabled = enabled;
    }

    /// Check if environment variable substitution is enabled
    pub async fn is_env_substitution_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.env_substitution_enabled
    }

    /// Set template processing enabled
    pub async fn set_template_processing_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.template_processing_enabled = enabled;
    }

    /// Check if template processing is enabled
    pub async fn is_template_processing_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.template_processing_enabled
    }

    /// Add a template variable
    pub async fn add_template_var(&self, key: &str, value: &str) {
        let mut template_vars = self.template_vars.write().await;
        template_vars.insert(key.to_string(), value.to_string());
    }

    /// Get template variables
    pub async fn get_template_vars(&self) -> HashMap<String, String> {
        let template_vars = self.template_vars.read().await;
        template_vars.clone()
    }

    /// Set encryption enabled
    pub async fn set_encryption_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.encryption_enabled = enabled;
    }

    /// Check if encryption is enabled
    pub async fn is_encryption_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.encryption_enabled
    }

    /// Set encryption key
    pub async fn set_encryption_key(&self, key: &str) {
        let mut settings = self.settings.write().await;
        settings.encryption_key = Some(key.to_string());
    }

    /// Get encryption key
    pub async fn get_encryption_key(&self) -> Option<String> {
        let settings = self.settings.read().await;
        settings.encryption_key.clone()
    }

    /// Set backup enabled
    pub async fn set_backup_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.backup_enabled = enabled;
    }

    /// Check if backup is enabled
    pub async fn is_backup_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.backup_enabled
    }

    /// Set backup directory
    pub async fn set_backup_dir(&self, dir: &PathBuf) {
        let mut settings = self.settings.write().await;
        settings.backup_dir = Some(dir.clone());
    }

    /// Get backup directory
    pub async fn get_backup_dir(&self) -> Option<PathBuf> {
        let settings = self.settings.read().await;
        settings.backup_dir.clone()
    }

    /// Set migration enabled
    pub async fn set_migration_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.migration_enabled = enabled;
    }

    /// Check if migration is enabled
    pub async fn is_migration_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.migration_enabled
    }

    /// Set migration directory
    pub async fn set_migration_dir(&self, dir: &PathBuf) {
        let mut settings = self.settings.write().await;
        settings.migration_dir = Some(dir.clone());
    }

    /// Get migration directory
    pub async fn get_migration_dir(&self) -> Option<PathBuf> {
        let settings = self.settings.read().await;
        settings.migration_dir.clone()
    }

    /// Set caching enabled
    pub async fn set_caching_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.caching_enabled = enabled;
    }

    /// Check if caching is enabled
    pub async fn is_caching_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.caching_enabled
    }

    /// Set cache directory
    pub async fn set_cache_dir(&self, dir: &PathBuf) {
        let mut settings = self.settings.write().await;
        settings.cache_dir = Some(dir.clone());
    }

    /// Get cache directory
    pub async fn get_cache_dir(&self) -> Option<PathBuf> {
        let settings = self.settings.read().await;
        settings.cache_dir.clone()
    }

    /// Set cache TTL
    pub async fn set_cache_ttl(&self, ttl: u64) {
        let mut settings = self.settings.write().await;
        settings.cache_ttl = ttl;
    }

    /// Get cache TTL
    pub async fn get_cache_ttl(&self) -> u64 {
        let settings = self.settings.read().await;
        settings.cache_ttl
    }

    /// Set monitoring enabled
    pub async fn set_monitoring_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.monitoring_enabled = enabled;
    }

    /// Check if monitoring is enabled
    pub async fn is_monitoring_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.monitoring_enabled
    }

    /// Set monitoring interval
    pub async fn set_monitoring_interval(&self, interval: u64) {
        let mut settings = self.settings.write().await;
        settings.monitoring_interval = interval;
    }

    /// Get monitoring interval
    pub async fn get_monitoring_interval(&self) -> u64 {
        let settings = self.settings.read().await;
        settings.monitoring_interval
    }

    /// Set statistics enabled
    pub async fn set_statistics_enabled(&self, enabled: bool) {
        let mut settings = self.settings.write().await;
        settings.statistics_enabled = enabled;
    }

    /// Check if statistics is enabled
    pub async fn is_statistics_enabled(&self) -> bool {
        let settings = self.settings.read().await;
        settings.statistics_enabled
    }

    /// Set statistics interval
    pub async fn set_statistics_interval(&self, interval: u64) {
        let mut settings = self.settings.write().await;
        settings.statistics_interval = interval;
    }

    /// Get statistics interval
    pub async fn get_statistics_interval(&self) -> u64 {
        let settings = self.settings.read().await;
        settings.statistics_interval
    }

    /// Add a configuration file
    pub async fn add_config_file(&mut self, file_path: &PathBuf) -> Result<(), ConfigError> {
        let mut config_files = self.config_files.write().await;
        config_files.push(file_path.clone());

        // Create file loader
        let file_loader = FileConfigLoader::new(file_path.clone());
        self.loaders.push(Box::new(file_loader));

        Ok(())
    }

    /// Add a configuration loader
    pub async fn add_loader(&mut self, loader: Box<dyn ConfigLoader + Send + Sync>) {
        self.loaders.push(loader);
    }

    /// Get configuration files
    pub async fn get_config_files(&self) -> Vec<PathBuf> {
        let config_files = self.config_files.read().await;
        config_files.clone()
    }

    /// Initialize the configuration manager
    pub async fn initialize(&mut self) -> Result<(), ConfigError> {
        info!("Initializing configuration manager");

        // Create environment loader
        let env_loader = EnvConfigLoader::new(&self.env_prefix);
        self.loaders.push(Box::new(env_loader));

        // Create validator if validation is enabled
        if self.is_validation_enabled().await {
            self.validator = Some(ConfigValidator::new());
        }

        // Load configuration
        self.load().await?;

        info!("Configuration manager initialized successfully");
        Ok(())
    }

    /// Load configuration from all sources
    pub async fn load(&self) -> Result<SyneidesisConfig, ConfigError> {
        let start_time = std::time::Instant::now();

        debug!("Loading configuration from {} sources", self.loaders.len());

        // Start with default configuration
        let mut config = self
            .get_default_config()
            .await
            .unwrap_or_else(SyneidesisConfig::default);

        // Load from all sources
        for loader in &self.loaders {
            match loader.load().await {
                Ok(loaded_config) => {
                    config = self.merge_configs(config, loaded_config);
                }
                Err(e) => {
                    warn!("Failed to load configuration from {}: {}", loader.name(), e);
                    self.update_statistics(|stats| stats.file_read_errors += 1)
                        .await;
                }
            }
        }

        // Apply custom values
        config = self.apply_custom_values(config).await;

        // Validate configuration if enabled
        if self.is_validation_enabled().await {
            if let Some(validator) = &self.validator {
                if let Err(e) = validator.validate(&config) {
                    error!("Configuration validation failed: {}", e);
                    self.update_statistics(|stats| stats.validation_errors += 1)
                        .await;
                    return Err(ConfigError::ValidationError {
                        message: e.to_string(),
                    });
                }
            }
        }

        // Store the configuration
        {
            let mut config_guard = self.config.write().await;
            *config_guard = Some(config.clone());
        }

        // Update statistics
        let load_time = start_time.elapsed();
        self.update_statistics(|stats| {
            stats.load_count += 1;
            stats.last_load_time = Some(chrono::Utc::now());
            stats.total_load_time_ms += load_time.as_millis() as u64;
            stats.avg_load_time_ms = stats.total_load_time_ms / stats.load_count;
        })
        .await;

        info!("Configuration loaded successfully in {:?}", load_time);
        Ok(config)
    }

    /// Reload configuration
    pub async fn reload(&self) -> Result<SyneidesisConfig, ConfigError> {
        let start_time = std::time::Instant::now();

        info!("Reloading configuration");

        // Clear current configuration
        {
            let mut config_guard = self.config.write().await;
            *config_guard = None;
        }

        // Load configuration
        let config = self.load().await?;

        // Update statistics
        let reload_time = start_time.elapsed();
        self.update_statistics(|stats| {
            stats.reload_count += 1;
            stats.last_reload_time = Some(chrono::Utc::now());
        })
        .await;

        info!("Configuration reloaded successfully in {:?}", reload_time);
        Ok(config)
    }

    /// Get current configuration
    pub async fn get_config(&self) -> Option<SyneidesisConfig> {
        let config = self.config.read().await;
        config.clone()
    }

    /// Get configuration statistics
    pub async fn get_statistics(&self) -> ConfigStatistics {
        let statistics = self.statistics.read().await;
        statistics.clone()
    }

    /// Update statistics
    async fn update_statistics<F>(&self, f: F)
    where
        F: FnOnce(&mut ConfigStatistics),
    {
        if self.is_statistics_enabled().await {
            let mut statistics = self.statistics.write().await;
            f(&mut statistics);
        }
    }

    /// Merge two configurations
    fn merge_configs(&self, base: SyneidesisConfig, overlay: SyneidesisConfig) -> SyneidesisConfig {
        // For now, use a simple merge strategy
        // In a real implementation, this would be more sophisticated
        let mut merged = base;

        // Merge system config
        if overlay.system.is_some() {
            merged.system = overlay.system;
        }

        // Merge agent config
        if overlay.agent.is_some() {
            merged.agent = overlay.agent;
        }

        // Merge coordination config
        if overlay.coordination.is_some() {
            merged.coordination = overlay.coordination;
        }

        // Merge gRPC config
        if overlay.grpc.is_some() {
            merged.grpc = overlay.grpc;
        }

        // Merge HTTP config
        if overlay.http.is_some() {
            merged.http = overlay.http;
        }

        // Merge network config
        if overlay.network.is_some() {
            merged.network = overlay.network;
        }

        // Merge security config
        if overlay.security.is_some() {
            merged.security = overlay.security;
        }

        // Merge logging config
        if overlay.logging.is_some() {
            merged.logging = overlay.logging;
        }

        // Merge validation config
        if overlay.validation.is_some() {
            merged.validation = overlay.validation;
        }

        // Merge custom values
        merged.custom.extend(overlay.custom);

        merged
    }

    /// Apply custom values to configuration
    async fn apply_custom_values(&self, mut config: SyneidesisConfig) -> SyneidesisConfig {
        let custom_values = self.custom_values.read().await;

        for (key, value) in custom_values.iter() {
            // Parse the key path (e.g., "system.name" -> ["system", "name"])
            let path_parts: Vec<&str> = key.split('.').collect();

            // Apply the value based on the path
            match path_parts.as_slice() {
                ["system", field] => {
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
                            _ => {
                                warn!("Unknown system field: {}", field);
                            }
                        }
                    }
                }
                ["agent", field] => {
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
                            _ => {
                                warn!("Unknown agent field: {}", field);
                            }
                        }
                    }
                }
                ["grpc", field] => {
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
                            _ => {
                                warn!("Unknown gRPC field: {}", field);
                            }
                        }
                    }
                }
                ["http", field] => {
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
                            _ => {
                                warn!("Unknown HTTP field: {}", field);
                            }
                        }
                    }
                }
                _ => {
                    warn!("Unknown configuration path: {}", key);
                }
            }
        }

        config
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SyneidesisConfig;

    #[tokio::test]
    async fn test_config_manager_new() {
        let manager = ConfigManager::new();
        assert!(manager.get_config().await.is_none());
    }

    #[tokio::test]
    async fn test_config_manager_with_defaults() {
        let manager = ConfigManager::new();
        let default_config = SyneidesisConfig::default();
        manager.set_default_config(default_config.clone()).await;

        let loaded_config = manager.get_default_config().await;
        assert!(loaded_config.is_some());
        assert_eq!(
            loaded_config.unwrap().system.unwrap().name,
            default_config.system.unwrap().name
        );
    }

    #[tokio::test]
    async fn test_config_manager_load() {
        let manager = ConfigManager::new();
        manager
            .set_default_config(SyneidesisConfig::default())
            .await;

        let config = manager.load().await.unwrap();
        assert!(config.system.is_some());
    }

    #[tokio::test]
    async fn test_config_manager_custom_values() {
        let manager = ConfigManager::new();
        manager
            .add_custom_value(
                "system.name",
                serde_json::Value::String("custom-name".to_string()),
            )
            .await;

        let value = manager.get_custom_value("system.name").await;
        assert_eq!(
            value,
            Some(serde_json::Value::String("custom-name".to_string()))
        );
    }

    #[tokio::test]
    async fn test_config_manager_validation() {
        let manager = ConfigManager::new();
        assert!(manager.is_validation_enabled().await);

        manager.set_validation_enabled(false).await;
        assert!(!manager.is_validation_enabled().await);
    }

    #[tokio::test]
    async fn test_config_manager_hot_reload() {
        let manager = ConfigManager::new();
        assert!(!manager.is_hot_reload_enabled().await);

        manager.set_hot_reload_enabled(true).await;
        assert!(manager.is_hot_reload_enabled().await);
    }

    #[tokio::test]
    async fn test_config_manager_search_paths() {
        let manager = ConfigManager::new();
        manager.add_search_path(&PathBuf::from("/test/path")).await;

        let search_paths = manager.get_search_paths().await;
        assert_eq!(search_paths.len(), 1);
        assert_eq!(search_paths[0], PathBuf::from("/test/path"));
    }

    #[tokio::test]
    async fn test_config_manager_formats() {
        let manager = ConfigManager::new();
        let formats = vec!["yaml".to_string(), "json".to_string()];
        manager.set_formats(&formats).await;

        let loaded_formats = manager.get_formats().await;
        assert_eq!(loaded_formats, formats);
    }

    #[tokio::test]
    async fn test_config_manager_template_vars() {
        let manager = ConfigManager::new();
        manager.add_template_var("ENV", "production").await;

        let template_vars = manager.get_template_vars().await;
        assert_eq!(template_vars.get("ENV"), Some(&"production".to_string()));
    }

    #[tokio::test]
    async fn test_config_manager_encryption() {
        let manager = ConfigManager::new();
        assert!(!manager.is_encryption_enabled().await);

        manager.set_encryption_enabled(true).await;
        manager.set_encryption_key("test-key").await;
        assert!(manager.is_encryption_enabled().await);
        assert_eq!(
            manager.get_encryption_key().await,
            Some("test-key".to_string())
        );
    }

    #[tokio::test]
    async fn test_config_manager_backup() {
        let manager = ConfigManager::new();
        assert!(!manager.is_backup_enabled().await);

        manager.set_backup_enabled(true).await;
        manager.set_backup_dir(&PathBuf::from("/backup")).await;
        assert!(manager.is_backup_enabled().await);
        assert_eq!(
            manager.get_backup_dir().await,
            Some(PathBuf::from("/backup"))
        );
    }

    #[tokio::test]
    async fn test_config_manager_caching() {
        let manager = ConfigManager::new();
        assert!(manager.is_caching_enabled().await);

        manager.set_caching_enabled(false).await;
        manager.set_cache_dir(&PathBuf::from("/cache")).await;
        manager.set_cache_ttl(1800).await;
        assert!(!manager.is_caching_enabled().await);
        assert_eq!(manager.get_cache_dir().await, Some(PathBuf::from("/cache")));
        assert_eq!(manager.get_cache_ttl().await, 1800);
    }

    #[tokio::test]
    async fn test_config_manager_monitoring() {
        let manager = ConfigManager::new();
        assert!(!manager.is_monitoring_enabled().await);

        manager.set_monitoring_enabled(true).await;
        manager.set_monitoring_interval(120).await;
        assert!(manager.is_monitoring_enabled().await);
        assert_eq!(manager.get_monitoring_interval().await, 120);
    }

    #[tokio::test]
    async fn test_config_manager_statistics() {
        let manager = ConfigManager::new();
        assert!(!manager.is_statistics_enabled().await);

        manager.set_statistics_enabled(true).await;
        manager.set_statistics_interval(600).await;
        assert!(manager.is_statistics_enabled().await);
        assert_eq!(manager.get_statistics_interval().await, 600);
    }

    #[tokio::test]
    async fn test_config_manager_config_files() {
        let mut manager = ConfigManager::new();
        let file_path = PathBuf::from("test.yaml");
        manager.add_config_file(&file_path).await.unwrap();

        let config_files = manager.get_config_files().await;
        assert_eq!(config_files.len(), 1);
        assert_eq!(config_files[0], file_path);
    }
}
