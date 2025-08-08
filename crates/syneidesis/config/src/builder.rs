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

//! Configuration builder for creating configuration managers

use crate::error::ConfigError;
use crate::loader::EnvConfigLoader;
use crate::manager::ConfigManager;
use crate::types::SyneidesisConfig;
use std::collections::HashMap;
use std::path::PathBuf;

/// Builder for creating configuration managers
pub struct ConfigBuilder {
    /// Configuration files to load
    config_files: Vec<PathBuf>,

    /// Environment variable prefix
    env_prefix: Option<String>,

    /// Default configuration
    default_config: Option<SyneidesisConfig>,

    /// Custom configuration values
    custom_values: HashMap<String, serde_json::Value>,

    /// Enable validation
    enable_validation: bool,

    /// Enable hot reloading
    enable_hot_reload: bool,

    /// Configuration directory
    config_dir: Option<PathBuf>,

    /// Search paths for configuration files
    search_paths: Vec<PathBuf>,

    /// Configuration file formats to try
    formats: Vec<String>,

    /// Enable environment variable substitution
    enable_env_substitution: bool,

    /// Enable template processing
    enable_template_processing: bool,

    /// Template variables
    template_vars: HashMap<String, String>,

    /// Enable configuration encryption
    enable_encryption: bool,

    /// Encryption key
    encryption_key: Option<String>,

    /// Enable configuration backup
    enable_backup: bool,

    /// Backup directory
    backup_dir: Option<PathBuf>,

    /// Enable configuration migration
    enable_migration: bool,

    /// Migration scripts directory
    migration_dir: Option<PathBuf>,

    /// Enable configuration caching
    enable_caching: bool,

    /// Cache directory
    cache_dir: Option<PathBuf>,

    /// Cache TTL in seconds
    cache_ttl: u64,

    /// Enable configuration monitoring
    enable_monitoring: bool,

    /// Monitoring interval in seconds
    monitoring_interval: u64,

    /// Enable configuration statistics
    enable_statistics: bool,

    /// Statistics collection interval in seconds
    statistics_interval: u64,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config_files: Vec::new(),
            env_prefix: None,
            default_config: None,
            custom_values: HashMap::new(),
            enable_validation: true,
            enable_hot_reload: false,
            config_dir: None,
            search_paths: Vec::new(),
            formats: vec![
                "yaml".to_string(),
                "yml".to_string(),
                "json".to_string(),
                "toml".to_string(),
            ],
            enable_env_substitution: true,
            enable_template_processing: false,
            template_vars: HashMap::new(),
            enable_encryption: false,
            encryption_key: None,
            enable_backup: false,
            backup_dir: None,
            enable_migration: false,
            migration_dir: None,
            enable_caching: true,
            cache_dir: None,
            cache_ttl: 3600, // 1 hour
            enable_monitoring: false,
            monitoring_interval: 60, // 1 minute
            enable_statistics: false,
            statistics_interval: 300, // 5 minutes
        }
    }

    /// Add a configuration file to load
    pub fn with_file(mut self, file_path: &str) -> Self {
        self.config_files.push(PathBuf::from(file_path));
        self
    }

    /// Add multiple configuration files to load
    pub fn with_files(mut self, file_paths: &[&str]) -> Self {
        for path in file_paths {
            self.config_files.push(PathBuf::from(*path));
        }
        self
    }

    /// Set the environment variable prefix
    pub fn with_env_prefix(mut self, prefix: &str) -> Self {
        self.env_prefix = Some(prefix.to_string());
        self
    }

    /// Set the default configuration
    pub fn with_default_config(mut self, config: SyneidesisConfig) -> Self {
        self.default_config = Some(config);
        self
    }

    /// Add default configuration (uses Default implementation)
    pub fn with_defaults(mut self) -> Self {
        self.default_config = Some(SyneidesisConfig::default());
        self
    }

    /// Add a custom configuration value
    pub fn with_custom_value(mut self, key: &str, value: serde_json::Value) -> Self {
        self.custom_values.insert(key.to_string(), value);
        self
    }

    /// Add multiple custom configuration values
    pub fn with_custom_values(mut self, values: HashMap<String, serde_json::Value>) -> Self {
        self.custom_values.extend(values);
        self
    }

    /// Enable or disable validation
    pub fn with_validation(mut self, enable: bool) -> Self {
        self.enable_validation = enable;
        self
    }

    /// Enable or disable hot reloading
    pub fn with_hot_reload(mut self, enable: bool) -> Self {
        self.enable_hot_reload = enable;
        self
    }

    /// Set the configuration directory
    pub fn with_config_dir(mut self, dir: &str) -> Self {
        self.config_dir = Some(PathBuf::from(dir));
        self
    }

    /// Add a search path for configuration files
    pub fn with_search_path(mut self, path: &str) -> Self {
        self.search_paths.push(PathBuf::from(path));
        self
    }

    /// Add multiple search paths for configuration files
    pub fn with_search_paths(mut self, paths: &[&str]) -> Self {
        for path in paths {
            self.search_paths.push(PathBuf::from(*path));
        }
        self
    }

    /// Set the configuration file formats to try
    pub fn with_formats(mut self, formats: &[&str]) -> Self {
        self.formats = formats.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Enable or disable environment variable substitution
    pub fn with_env_substitution(mut self, enable: bool) -> Self {
        self.enable_env_substitution = enable;
        self
    }

    /// Enable or disable template processing
    pub fn with_template_processing(mut self, enable: bool) -> Self {
        self.enable_template_processing = enable;
        self
    }

    /// Add a template variable
    pub fn with_template_var(mut self, key: &str, value: &str) -> Self {
        self.template_vars
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Add multiple template variables
    pub fn with_template_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.template_vars.extend(vars);
        self
    }

    /// Enable or disable configuration encryption
    pub fn with_encryption(mut self, enable: bool) -> Self {
        self.enable_encryption = enable;
        self
    }

    /// Set the encryption key
    pub fn with_encryption_key(mut self, key: &str) -> Self {
        self.encryption_key = Some(key.to_string());
        self
    }

    /// Enable or disable configuration backup
    pub fn with_backup(mut self, enable: bool) -> Self {
        self.enable_backup = enable;
        self
    }

    /// Set the backup directory
    pub fn with_backup_dir(mut self, dir: &str) -> Self {
        self.backup_dir = Some(PathBuf::from(dir));
        self
    }

    /// Enable or disable configuration migration
    pub fn with_migration(mut self, enable: bool) -> Self {
        self.enable_migration = enable;
        self
    }

    /// Set the migration scripts directory
    pub fn with_migration_dir(mut self, dir: &str) -> Self {
        self.migration_dir = Some(PathBuf::from(dir));
        self
    }

    /// Enable or disable configuration caching
    pub fn with_caching(mut self, enable: bool) -> Self {
        self.enable_caching = enable;
        self
    }

    /// Set the cache directory
    pub fn with_cache_dir(mut self, dir: &str) -> Self {
        self.cache_dir = Some(PathBuf::from(dir));
        self
    }

    /// Set the cache TTL in seconds
    pub fn with_cache_ttl(mut self, ttl: u64) -> Self {
        self.cache_ttl = ttl;
        self
    }

    /// Enable or disable configuration monitoring
    pub fn with_monitoring(mut self, enable: bool) -> Self {
        self.enable_monitoring = enable;
        self
    }

    /// Set the monitoring interval in seconds
    pub fn with_monitoring_interval(mut self, interval: u64) -> Self {
        self.monitoring_interval = interval;
        self
    }

    /// Enable or disable configuration statistics
    pub fn with_statistics(mut self, enable: bool) -> Self {
        self.enable_statistics = enable;
        self
    }

    /// Set the statistics collection interval in seconds
    pub fn with_statistics_interval(mut self, interval: u64) -> Self {
        self.statistics_interval = interval;
        self
    }

    /// Build the configuration manager
    pub async fn build(self) -> Result<ConfigManager, ConfigError> {
        // Create the configuration manager with the builder settings
        let mut manager = ConfigManager::new();

        // Set the environment prefix and add environment loader
        if let Some(prefix) = self.env_prefix {
            manager.set_env_prefix(&prefix);
            manager
                .add_loader(Box::new(EnvConfigLoader::new(&prefix)))
                .await;
        }

        // Set the default configuration
        if let Some(config) = self.default_config {
            manager.set_default_config(config).await;
        }

        // Add custom values
        for (key, value) in self.custom_values {
            manager.add_custom_value(&key, value).await;
        }

        // Configure validation
        manager.set_validation_enabled(self.enable_validation).await;

        // Configure hot reloading
        manager.set_hot_reload_enabled(self.enable_hot_reload).await;

        // Set configuration directory
        if let Some(dir) = self.config_dir {
            manager.set_config_dir(&dir).await;
        }

        // Add search paths
        for path in self.search_paths {
            manager.add_search_path(&path).await;
        }

        // Set file formats
        manager.set_formats(&self.formats).await;

        // Configure environment variable substitution
        manager
            .set_env_substitution_enabled(self.enable_env_substitution)
            .await;

        // Configure template processing
        manager
            .set_template_processing_enabled(self.enable_template_processing)
            .await;

        // Add template variables
        for (key, value) in self.template_vars {
            manager.add_template_var(&key, &value).await;
        }

        // Configure encryption
        manager.set_encryption_enabled(self.enable_encryption).await;
        if let Some(key) = self.encryption_key {
            manager.set_encryption_key(&key).await;
        }

        // Configure backup
        manager.set_backup_enabled(self.enable_backup).await;
        if let Some(dir) = self.backup_dir {
            manager.set_backup_dir(&dir).await;
        }

        // Configure migration
        manager.set_migration_enabled(self.enable_migration).await;
        if let Some(dir) = self.migration_dir {
            manager.set_migration_dir(&dir).await;
        }

        // Configure caching
        manager.set_caching_enabled(self.enable_caching).await;
        if let Some(dir) = self.cache_dir {
            manager.set_cache_dir(&dir).await;
        }
        manager.set_cache_ttl(self.cache_ttl).await;

        // Configure monitoring
        manager.set_monitoring_enabled(self.enable_monitoring).await;
        manager
            .set_monitoring_interval(self.monitoring_interval)
            .await;

        // Configure statistics
        manager.set_statistics_enabled(self.enable_statistics).await;
        manager
            .set_statistics_interval(self.statistics_interval)
            .await;

        // Initialize the manager (adds environment loader)
        manager.initialize().await?;

        // Add configuration files
        for file_path in self.config_files {
            manager.add_config_file(&file_path).await?;
        }

        Ok(manager)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_builder_with_defaults() {
        let manager = ConfigBuilder::new().with_defaults().build().await.unwrap();

        let config = manager.load().await.unwrap();
        assert!(config.system.is_some());
    }

    #[tokio::test]
    async fn test_builder_with_file() {
        let temp_file = tempfile::NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .with_extension("yaml");
        let config_content = r#"
system:
  name: "test-system"
  version: "1.0.0"
"#;
        std::fs::write(&temp_file, config_content).unwrap();

        let manager = ConfigBuilder::new()
            .with_file(temp_file.to_str().unwrap())
            .with_defaults()
            .build()
            .await
            .unwrap();

        let config = manager.load().await.unwrap();
        assert_eq!(config.system.unwrap().name, "test-system");
    }

    #[tokio::test]
    async fn test_builder_with_env_prefix() {
        std::env::set_var("SYNEIDESIS_SYSTEM_NAME", "env-system");

        let manager = ConfigBuilder::new()
            .with_env_prefix("SYNEIDESIS")
            .with_defaults()
            .build()
            .await
            .unwrap();

        let config = manager.load().await.unwrap();
        assert_eq!(config.system.unwrap().name, "env-system");

        std::env::remove_var("SYNEIDESIS_SYSTEM_NAME");
    }

    #[tokio::test]
    async fn test_builder_with_custom_values() {
        let mut custom_values = HashMap::new();
        custom_values.insert(
            "system.name".to_string(),
            serde_json::Value::String("custom-system".to_string()),
        );

        let manager = ConfigBuilder::new()
            .with_custom_values(custom_values)
            .with_defaults()
            .build()
            .await
            .unwrap();

        let config = manager.load().await.unwrap();
        assert_eq!(config.system.unwrap().name, "custom-system");
    }

    #[tokio::test]
    async fn test_builder_with_validation_disabled() {
        let manager = ConfigBuilder::new()
            .with_validation(false)
            .with_defaults()
            .build()
            .await
            .unwrap();

        assert!(!manager.is_validation_enabled().await);
    }

    #[tokio::test]
    async fn test_builder_with_hot_reload() {
        let manager = ConfigBuilder::new()
            .with_hot_reload(true)
            .with_defaults()
            .build()
            .await
            .unwrap();

        assert!(manager.is_hot_reload_enabled().await);
    }

    #[tokio::test]
    async fn test_builder_with_search_paths() {
        let manager = ConfigBuilder::new()
            .with_search_paths(&["/etc/syneidesis", "./config"])
            .with_defaults()
            .build()
            .await
            .unwrap();

        let search_paths = manager.get_search_paths().await;
        assert_eq!(search_paths.len(), 2);
        assert!(search_paths.contains(&PathBuf::from("/etc/syneidesis")));
        assert!(search_paths.contains(&PathBuf::from("./config")));
    }

    #[tokio::test]
    async fn test_builder_with_formats() {
        let manager = ConfigBuilder::new()
            .with_formats(&["yaml", "json"])
            .with_defaults()
            .build()
            .await
            .unwrap();

        let formats = manager.get_formats().await;
        assert_eq!(formats, vec!["yaml", "json"]);
    }

    #[tokio::test]
    async fn test_builder_with_template_vars() {
        let mut template_vars = HashMap::new();
        template_vars.insert("ENV".to_string(), "production".to_string());

        let manager = ConfigBuilder::new()
            .with_template_vars(template_vars)
            .with_template_processing(true)
            .with_defaults()
            .build()
            .await
            .unwrap();

        let vars = manager.get_template_vars().await;
        assert_eq!(vars.get("ENV"), Some(&"production".to_string()));
    }

    #[tokio::test]
    async fn test_builder_with_encryption() {
        let manager = ConfigBuilder::new()
            .with_encryption(true)
            .with_encryption_key("test-key")
            .with_defaults()
            .build()
            .await
            .unwrap();

        assert!(manager.is_encryption_enabled().await);
    }

    #[tokio::test]
    async fn test_builder_with_backup() {
        let manager = ConfigBuilder::new()
            .with_backup(true)
            .with_backup_dir("/tmp/backup")
            .with_defaults()
            .build()
            .await
            .unwrap();

        assert!(manager.is_backup_enabled().await);
        assert_eq!(
            manager.get_backup_dir().await,
            Some(PathBuf::from("/tmp/backup"))
        );
    }

    #[tokio::test]
    async fn test_builder_with_caching() {
        let manager = ConfigBuilder::new()
            .with_caching(true)
            .with_cache_dir("/tmp/cache")
            .with_cache_ttl(1800)
            .with_defaults()
            .build()
            .await
            .unwrap();

        assert!(manager.is_caching_enabled().await);
        assert_eq!(
            manager.get_cache_dir().await,
            Some(PathBuf::from("/tmp/cache"))
        );
        assert_eq!(manager.get_cache_ttl().await, 1800);
    }

    #[tokio::test]
    async fn test_builder_with_monitoring() {
        let manager = ConfigBuilder::new()
            .with_monitoring(true)
            .with_monitoring_interval(120)
            .with_defaults()
            .build()
            .await
            .unwrap();

        assert!(manager.is_monitoring_enabled().await);
        assert_eq!(manager.get_monitoring_interval().await, 120);
    }

    #[tokio::test]
    async fn test_builder_with_statistics() {
        let manager = ConfigBuilder::new()
            .with_statistics(true)
            .with_statistics_interval(600)
            .with_defaults()
            .build()
            .await
            .unwrap();

        assert!(manager.is_statistics_enabled().await);
        assert_eq!(manager.get_statistics_interval().await, 600);
    }
}
