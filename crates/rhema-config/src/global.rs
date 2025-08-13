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

use crate::{
    Config, ConfigAuditLog, ConfigEnvironment, ConfigHealth, ConfigStats, CURRENT_CONFIG_VERSION,
};
use chrono::{DateTime, Utc};
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use validator::Validate;
/// Global configuration for Rhema CLI
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GlobalConfig {
    /// Configuration version
    #[validate(length(min = 1))]
    pub version: String,

    /// User information
    pub user: UserConfig,

    /// Application settings
    pub application: ApplicationConfig,

    /// Environment settings
    pub environment: EnvironmentConfig,

    /// Security settings
    pub security: SecurityConfig,

    /// Performance settings
    pub performance: PerformanceConfig,

    /// Integration settings
    pub integrations: IntegrationConfig,

    /// Custom settings
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,

    /// Audit log
    pub audit_log: ConfigAuditLog,

    /// Health status
    pub health: ConfigHealth,

    /// Statistics
    pub stats: ConfigStats,

    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// User configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserConfig {
    /// User ID
    #[validate(length(min = 1))]
    pub id: String,

    /// User name
    #[validate(length(min = 1))]
    pub name: String,

    /// User email
    #[validate(email)]
    pub email: String,

    /// User preferences
    pub preferences: UserPreferences,

    /// User roles
    pub roles: Vec<String>,

    /// User permissions
    pub permissions: HashMap<String, Vec<String>>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Default output format
    pub default_output_format: String,

    /// Default editor
    pub default_editor: Option<String>,

    /// Color scheme
    pub color_scheme: String,

    /// Language preference
    pub language: String,

    /// Timezone
    pub timezone: String,

    /// Date format
    pub date_format: String,

    /// Time format
    pub time_format: String,

    /// Notification preferences
    pub notifications: NotificationPreferences,

    /// UI preferences
    pub ui: UIPreferences,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Enable email notifications
    pub email_enabled: bool,

    /// Enable desktop notifications
    pub desktop_enabled: bool,

    /// Enable sound notifications
    pub sound_enabled: bool,

    /// Notification frequency
    pub frequency: NotificationFrequency,

    /// Notification types
    pub types: Vec<String>,
}

/// Notification frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationFrequency {
    Immediate,
    Hourly,
    Daily,
    Weekly,
    Never,
}

/// UI preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPreferences {
    /// Theme
    pub theme: String,

    /// Font size
    pub font_size: u32,

    /// Show line numbers
    pub show_line_numbers: bool,

    /// Show minimap
    pub show_minimap: bool,

    /// Word wrap
    pub word_wrap: bool,

    /// Auto save
    pub auto_save: bool,

    /// Auto save interval (seconds)
    pub auto_save_interval: u64,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ApplicationConfig {
    /// Application name
    #[validate(length(min = 1))]
    pub name: String,

    /// Application version
    #[validate(length(min = 1))]
    pub version: String,

    /// Application description
    pub description: Option<String>,

    /// Application settings
    pub settings: AppSettings,

    /// Feature flags
    pub features: FeatureFlags,

    /// Plugin settings
    pub plugins: PluginConfig,
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Enable debug mode
    pub debug_mode: bool,

    /// Enable verbose logging
    pub verbose_logging: bool,

    /// Log level
    pub log_level: String,

    /// Log file path
    pub log_file: Option<PathBuf>,

    /// Max log file size (MB)
    pub max_log_size: u64,

    /// Log rotation count
    pub log_rotation_count: u32,

    /// Enable telemetry
    pub telemetry_enabled: bool,

    /// Telemetry endpoint
    pub telemetry_endpoint: Option<String>,

    /// Auto update enabled
    pub auto_update_enabled: bool,

    /// Update check interval (hours)
    pub update_check_interval: u64,
}

/// Feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Enable experimental features
    pub experimental_features: bool,

    /// Enable beta features
    pub beta_features: bool,

    /// Enable advanced features
    pub advanced_features: bool,

    /// Enable AI features
    pub ai_features: bool,

    /// Enable cloud features
    pub cloud_features: bool,

    /// Enable collaboration features
    pub collaboration_features: bool,

    /// Enable analytics features
    pub analytics_features: bool,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Plugin directory
    pub plugin_directory: PathBuf,

    /// Auto load plugins
    pub auto_load_plugins: bool,

    /// Enabled plugins
    pub enabled_plugins: Vec<String>,

    /// Disabled plugins
    pub disabled_plugins: Vec<String>,

    /// Plugin settings
    pub plugin_settings: HashMap<String, serde_json::Value>,
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EnvironmentConfig {
    /// Current environment
    pub current: ConfigEnvironment,

    /// Environment-specific settings
    pub environments: HashMap<ConfigEnvironment, EnvironmentSettings>,

    /// Environment variables
    pub environment_variables: HashMap<String, String>,

    /// Path settings
    pub paths: PathConfig,
}

/// Environment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSettings {
    /// Environment name
    pub name: String,

    /// Environment description
    pub description: Option<String>,

    /// Environment variables
    pub variables: HashMap<String, String>,

    /// Environment-specific paths
    pub paths: HashMap<String, PathBuf>,

    /// Environment-specific settings
    pub settings: HashMap<String, serde_json::Value>,
}

/// Path configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    /// Home directory
    pub home: PathBuf,

    /// Config directory
    pub config: PathBuf,

    /// Data directory
    pub data: PathBuf,

    /// Cache directory
    pub cache: PathBuf,

    /// Log directory
    pub log: PathBuf,

    /// Temp directory
    pub temp: PathBuf,

    /// Workspace directory
    pub workspace: Option<PathBuf>,

    /// Custom paths
    pub custom: HashMap<String, PathBuf>,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SecurityConfig {
    /// Encryption settings
    pub encryption: EncryptionConfig,

    /// Authentication settings
    pub authentication: AuthenticationConfig,

    /// Authorization settings
    pub authorization: AuthorizationConfig,

    /// Audit settings
    pub audit: AuditConfig,

    /// Compliance settings
    pub compliance: ComplianceConfig,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Enable encryption
    pub enabled: bool,

    /// Encryption algorithm
    pub algorithm: String,

    /// Key size
    pub key_size: u32,

    /// Key derivation function
    pub kdf: String,

    /// Salt size
    pub salt_size: u32,

    /// Iteration count
    pub iteration_count: u32,

    /// Key file path
    pub key_file: Option<PathBuf>,

    /// Master password required
    pub master_password_required: bool,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Authentication method
    pub method: String,

    /// Session timeout (minutes)
    pub session_timeout: u64,

    /// Max failed attempts
    pub max_failed_attempts: u32,

    /// Lockout duration (minutes)
    pub lockout_duration: u64,

    /// Require MFA
    pub require_mfa: bool,

    /// MFA method
    pub mfa_method: Option<String>,

    /// SSO enabled
    pub sso_enabled: bool,

    /// SSO provider
    pub sso_provider: Option<String>,
}

/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    /// Role-based access control
    pub rbac_enabled: bool,

    /// Default role
    pub default_role: String,

    /// Admin role
    pub admin_role: String,

    /// User role
    pub user_role: String,

    /// Guest role
    pub guest_role: String,

    /// Permission inheritance
    pub permission_inheritance: bool,

    /// Permission cache enabled
    pub permission_cache_enabled: bool,

    /// Permission cache timeout (minutes)
    pub permission_cache_timeout: u64,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Audit logging enabled
    pub enabled: bool,

    /// Audit log level
    pub log_level: String,

    /// Audit log file
    pub log_file: Option<PathBuf>,

    /// Audit retention days
    pub retention_days: u32,

    /// Audit events
    pub events: Vec<String>,

    /// Audit filters
    pub filters: HashMap<String, String>,
}

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Compliance framework
    pub framework: String,

    /// Compliance level
    pub level: String,

    /// Compliance reporting
    pub reporting_enabled: bool,

    /// Compliance checks
    pub checks: Vec<String>,

    /// Compliance rules
    pub rules: HashMap<String, serde_json::Value>,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PerformanceConfig {
    /// Cache settings
    pub cache: CacheConfig,

    /// Threading settings
    pub threading: ThreadingConfig,

    /// Memory settings
    pub memory: MemoryConfig,

    /// Network settings
    pub network: NetworkConfig,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Cache enabled
    pub enabled: bool,

    /// Cache type
    pub cache_type: String,

    /// Cache size (MB)
    pub cache_size: u64,

    /// Cache TTL (seconds)
    pub cache_ttl: u64,

    /// Cache directory
    pub cache_directory: PathBuf,

    /// Cache compression
    pub compression_enabled: bool,

    /// Cache encryption
    pub encryption_enabled: bool,
}

/// Threading configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadingConfig {
    /// Max threads
    pub max_threads: u32,

    /// Thread pool size
    pub thread_pool_size: u32,

    /// Async runtime threads
    pub async_runtime_threads: u32,

    /// Blocking thread pool size
    pub blocking_thread_pool_size: u32,
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Max memory usage (MB)
    pub max_memory_usage: u64,

    /// Memory limit (MB)
    pub memory_limit: u64,

    /// Garbage collection enabled
    pub gc_enabled: bool,

    /// GC interval (seconds)
    pub gc_interval: u64,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Connection timeout (seconds)
    pub connection_timeout: u64,

    /// Request timeout (seconds)
    pub request_timeout: u64,

    /// Max connections
    pub max_connections: u32,

    /// Keep alive enabled
    pub keep_alive_enabled: bool,

    /// Keep alive timeout (seconds)
    pub keep_alive_timeout: u64,

    /// Proxy settings
    pub proxy: Option<ProxyConfig>,
}

/// Proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy enabled
    pub enabled: bool,

    /// Proxy URL
    pub url: String,

    /// Proxy username
    pub username: Option<String>,

    /// Proxy password
    pub password: Option<String>,

    /// Proxy bypass
    pub bypass: Vec<String>,
}

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct IntegrationConfig {
    /// Git integration
    pub git: GitIntegrationConfig,

    /// IDE integration
    pub ide: IDEIntegrationConfig,

    /// CI/CD integration
    pub cicd: CICDIntegrationConfig,

    /// Cloud integration
    pub cloud: CloudIntegrationConfig,

    /// External services
    pub external_services: HashMap<String, ExternalServiceConfig>,
}

/// Git integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitIntegrationConfig {
    /// Git enabled
    pub enabled: bool,

    /// Git provider
    pub provider: String,

    /// Git credentials
    pub credentials: GitCredentials,

    /// Git hooks enabled
    pub hooks_enabled: bool,

    /// Git hooks directory
    pub hooks_directory: Option<PathBuf>,

    /// Git workflow
    pub workflow: String,

    /// Git branch naming
    pub branch_naming: HashMap<String, String>,
}

/// Git credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCredentials {
    /// Username
    pub username: Option<String>,

    /// Email
    pub email: Option<String>,

    /// SSH key path
    pub ssh_key_path: Option<PathBuf>,

    /// Personal access token
    pub personal_access_token: Option<String>,

    /// OAuth token
    pub oauth_token: Option<String>,
}

/// IDE integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEIntegrationConfig {
    /// IDE enabled
    pub enabled: bool,

    /// Supported IDEs
    pub supported_ides: Vec<String>,

    /// IDE settings
    pub ide_settings: HashMap<String, serde_json::Value>,

    /// Auto-sync enabled
    pub auto_sync_enabled: bool,

    /// Sync interval (seconds)
    pub sync_interval: u64,
}

/// CI/CD integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CICDIntegrationConfig {
    /// CI/CD enabled
    pub enabled: bool,

    /// CI/CD provider
    pub provider: String,

    /// CI/CD settings
    pub settings: HashMap<String, serde_json::Value>,

    /// Auto-deploy enabled
    pub auto_deploy_enabled: bool,

    /// Deployment environments
    pub deployment_environments: Vec<String>,
}

/// Cloud integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudIntegrationConfig {
    /// Cloud enabled
    pub enabled: bool,

    /// Cloud provider
    pub provider: String,

    /// Cloud credentials
    pub credentials: CloudCredentials,

    /// Cloud regions
    pub regions: Vec<String>,

    /// Cloud services
    pub services: Vec<String>,
}

/// Cloud credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCredentials {
    /// Access key ID
    pub access_key_id: Option<String>,

    /// Secret access key
    pub secret_access_key: Option<String>,

    /// Session token
    pub session_token: Option<String>,

    /// Credentials file
    pub credentials_file: Option<PathBuf>,

    /// Profile name
    pub profile_name: Option<String>,
}

/// External service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServiceConfig {
    /// Service enabled
    pub enabled: bool,

    /// Service URL
    pub url: String,

    /// Service API key
    pub api_key: Option<String>,

    /// Service timeout (seconds)
    pub timeout: u64,

    /// Service retry attempts
    pub retry_attempts: u32,

    /// Service settings
    pub settings: HashMap<String, serde_json::Value>,
}

impl Config for GlobalConfig {
    fn version(&self) -> &str {
        &self.version
    }

    fn validate_config(&self) -> rhema_core::RhemaResult<()> {
        // Basic validation
        if self.version.is_empty() {
            return Err(rhema_core::RhemaError::ConfigError(
                "Version cannot be empty".to_string(),
            ));
        }

        // Validate user configuration
        if self.user.name.is_empty() {
            return Err(rhema_core::RhemaError::ConfigError(
                "User name cannot be empty".to_string(),
            ));
        }

        // Validate application configuration
        if self.application.name.is_empty() {
            return Err(rhema_core::RhemaError::ConfigError(
                "Application name cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn load_from_file(_path: &Path) -> rhema_core::RhemaResult<Self> {
        Self::load()
    }

    fn save_to_file(&self, _path: &Path) -> rhema_core::RhemaResult<()> {
        // Implementation would save to the specified path
        Ok(())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "version": {"type": "string", "minLength": 1},
                "user": {"type": "object"},
                "application": {"type": "object"},
                "environment": {"type": "object"},
                "security": {"type": "object"},
                "performance": {"type": "object"},
                "integrations": {"type": "object"},
                "custom": {"type": "object"},
                "audit_log": {"type": "object"},
                "health": {"type": "object"},
                "stats": {"type": "object"},
                "updated_at": {"type": "string", "format": "date-time"}
            },
            "required": ["version", "user", "application", "environment", "security", "performance", "integrations"]
        })
    }

    fn documentation() -> &'static str {
        "Global configuration for Rhema application"
    }
}

impl GlobalConfig {
    /// Create a new global configuration
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            version: CURRENT_CONFIG_VERSION.to_string(),
            user: UserConfig::default(),
            application: ApplicationConfig::default(),
            environment: EnvironmentConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            integrations: IntegrationConfig::default(),
            custom: HashMap::new(),
            audit_log: ConfigAuditLog::new(),
            health: ConfigHealth::default(),
            stats: ConfigStats::new(),
            updated_at: now,
        }
    }

    /// Load global configuration from file
    pub fn load() -> RhemaResult<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| rhema_core::RhemaError::IoError(e))?;

            let config: Self = serde_yaml::from_str(&content).map_err(|e| {
                rhema_core::RhemaError::InvalidYaml {
                    file: config_path.display().to_string(),
                    message: e.to_string(),
                }
            })?;

            config.validate_config()?;
            Ok(config)
        } else {
            // Create default configuration
            let config = Self::new();
            config.save()?;
            Ok(config)
        }
    }

    /// Load global configuration from JSON string
    pub fn load_from_json(json: &str) -> RhemaResult<Self> {
        let config: Self =
            serde_json::from_str(json).map_err(|e| rhema_core::RhemaError::InvalidJson {
                message: e.to_string(),
            })?;

        config.validate_config()?;
        Ok(config)
    }

    /// Save global configuration to file
    pub fn save(&self) -> RhemaResult<()> {
        let config_path = Self::get_config_path()?;

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| rhema_core::RhemaError::IoError(e))?;
        }

        let content =
            serde_yaml::to_string(self).map_err(|e| rhema_core::RhemaError::InvalidYaml {
                file: config_path.display().to_string(),
                message: e.to_string(),
            })?;

        std::fs::write(&config_path, content).map_err(|e| rhema_core::RhemaError::IoError(e))?;

        Ok(())
    }

    /// Get configuration file path
    fn get_config_path() -> RhemaResult<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| {
                rhema_core::RhemaError::ConfigError(
                    "Could not determine config directory".to_string(),
                )
            })?
            .join("rhema");

        Ok(config_dir.join("global.yaml"))
    }

    /// Update configuration
    pub fn update(&mut self) -> RhemaResult<()> {
        self.updated_at = Utc::now();
        self.save()
    }

    pub fn get_value(&self, path: &str) -> Option<&serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = serde_json::to_value(self).ok()?;

        for part in parts {
            current = current.get(part)?.clone();
        }

        // Convert back to a reference - this is a simplified approach
        // In a real implementation, you'd want to return a proper reference
        None
    }

    pub fn set_value(&mut self, path: &str, value: serde_json::Value) -> RhemaResult<()> {
        let parts: Vec<&str> = path.split('.').collect();

        // This is a simplified implementation
        // In a real implementation, you'd want to properly traverse and set nested values
        match parts.as_slice() {
            ["user", "name"] => {
                if let Some(name) = value.as_str() {
                    self.user.name = name.to_string();
                }
            }
            ["user", "email"] => {
                if let Some(email) = value.as_str() {
                    self.user.email = email.to_string();
                }
            }
            ["application", "name"] => {
                if let Some(name) = value.as_str() {
                    self.application.name = name.to_string();
                }
            }
            ["application", "version"] => {
                if let Some(version) = value.as_str() {
                    self.application.version = version.to_string();
                }
            }
            ["environment", "current"] => {
                if let Some(env_str) = value.as_str() {
                    self.environment.current = match env_str {
                        "development" => ConfigEnvironment::Development,
                        "testing" => ConfigEnvironment::Testing,
                        "staging" => ConfigEnvironment::Staging,
                        "production" => ConfigEnvironment::Production,
                        _ => ConfigEnvironment::Custom(env_str.to_string()),
                    };
                }
            }
            _ => {
                // For custom fields, store in the custom HashMap
                self.custom.insert(path.to_string(), value);
            }
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn export(&self, format: &str) -> RhemaResult<String> {
        match format {
            "json" => {
                let json = serde_json::to_string_pretty(self)?;
                Ok(json)
            }
            "yaml" => {
                let yaml = serde_yaml::to_string(self)?;
                Ok(yaml)
            }
            _ => Err(rhema_core::RhemaError::ConfigError(format!(
                "Unsupported export format: {}. Supported formats: json, yaml",
                format
            ))),
        }
    }

    pub fn import(&mut self, content: &str, format: &str) -> RhemaResult<()> {
        let imported_config: GlobalConfig = match format {
            "json" => serde_json::from_str(content)?,
            "yaml" => serde_yaml::from_str(content)?,
            _ => {
                return Err(rhema_core::RhemaError::ConfigError(format!(
                    "Unsupported import format: {}. Supported formats: json, yaml",
                    format
                )));
            }
        };

        // Merge the imported configuration with the current one
        *self = imported_config;
        self.updated_at = Utc::now();

        Ok(())
    }
}

// Default implementations
impl Default for UserConfig {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default User".to_string(),
            email: "user@example.com".to_string(),
            preferences: UserPreferences::default(),
            roles: vec!["user".to_string()],
            permissions: HashMap::new(),
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            default_output_format: "yaml".to_string(),
            default_editor: None,
            color_scheme: "auto".to_string(),
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            date_format: "%Y-%m-%d".to_string(),
            time_format: "%H:%M:%S".to_string(),
            notifications: NotificationPreferences::default(),
            ui: UIPreferences::default(),
        }
    }
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            email_enabled: false,
            desktop_enabled: true,
            sound_enabled: false,
            frequency: NotificationFrequency::Immediate,
            types: vec!["error".to_string(), "warning".to_string()],
        }
    }
}

impl Default for UIPreferences {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            font_size: 12,
            show_line_numbers: true,
            show_minimap: true,
            word_wrap: false,
            auto_save: true,
            auto_save_interval: 300,
        }
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            name: "Rhema CLI".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: Some("Git-Based Agent Context Protocol CLI".to_string()),
            settings: AppSettings::default(),
            features: FeatureFlags::default(),
            plugins: PluginConfig::default(),
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            debug_mode: false,
            verbose_logging: false,
            log_level: "info".to_string(),
            log_file: None,
            max_log_size: 100,
            log_rotation_count: 5,
            telemetry_enabled: false,
            telemetry_endpoint: None,
            auto_update_enabled: true,
            update_check_interval: 24,
        }
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            experimental_features: false,
            beta_features: false,
            advanced_features: true,
            ai_features: true,
            cloud_features: false,
            collaboration_features: false,
            analytics_features: false,
        }
    }
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            plugin_directory: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("rhema/plugins"),
            auto_load_plugins: true,
            enabled_plugins: Vec::new(),
            disabled_plugins: Vec::new(),
            plugin_settings: HashMap::new(),
        }
    }
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            current: ConfigEnvironment::Development,
            environments: HashMap::new(),
            environment_variables: HashMap::new(),
            paths: PathConfig::default(),
        }
    }
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            home: dirs::home_dir().unwrap_or_else(|| PathBuf::from("~")),
            config: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("rhema"),
            data: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("~/.local/share"))
                .join("rhema"),
            cache: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("~/.cache"))
                .join("rhema"),
            log: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("~/.local/share"))
                .join("rhema/logs"),
            temp: std::env::temp_dir().join("rhema"),
            workspace: None,
            custom: HashMap::new(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            encryption: EncryptionConfig::default(),
            authentication: AuthenticationConfig::default(),
            authorization: AuthorizationConfig::default(),
            audit: AuditConfig::default(),
            compliance: ComplianceConfig::default(),
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "AES-256-GCM".to_string(),
            key_size: 256,
            kdf: "PBKDF2".to_string(),
            salt_size: 32,
            iteration_count: 100000,
            key_file: None,
            master_password_required: false,
        }
    }
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            method: "local".to_string(),
            session_timeout: 1440, // 24 hours
            max_failed_attempts: 5,
            lockout_duration: 30,
            require_mfa: false,
            mfa_method: None,
            sso_enabled: false,
            sso_provider: None,
        }
    }
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            rbac_enabled: true,
            default_role: "user".to_string(),
            admin_role: "admin".to_string(),
            user_role: "user".to_string(),
            guest_role: "guest".to_string(),
            permission_inheritance: true,
            permission_cache_enabled: true,
            permission_cache_timeout: 60,
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: "info".to_string(),
            log_file: None,
            retention_days: 90,
            events: vec!["config_change".to_string(), "security_event".to_string()],
            filters: HashMap::new(),
        }
    }
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            framework: "basic".to_string(),
            level: "standard".to_string(),
            reporting_enabled: false,
            checks: Vec::new(),
            rules: HashMap::new(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            cache: CacheConfig::default(),
            threading: ThreadingConfig::default(),
            memory: MemoryConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_type: "file".to_string(),
            cache_size: 100,
            cache_ttl: 3600,
            cache_directory: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("~/.cache"))
                .join("rhema"),
            compression_enabled: true,
            encryption_enabled: false,
        }
    }
}

impl Default for ThreadingConfig {
    fn default() -> Self {
        Self {
            max_threads: num_cpus::get() as u32,
            thread_pool_size: 4,
            async_runtime_threads: 4,
            blocking_thread_pool_size: 8,
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memory_usage: 1024,
            memory_limit: 2048,
            gc_enabled: true,
            gc_interval: 300,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            connection_timeout: 30,
            request_timeout: 60,
            max_connections: 100,
            keep_alive_enabled: true,
            keep_alive_timeout: 300,
            proxy: None,
        }
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            git: GitIntegrationConfig::default(),
            ide: IDEIntegrationConfig::default(),
            cicd: CICDIntegrationConfig::default(),
            cloud: CloudIntegrationConfig::default(),
            external_services: HashMap::new(),
        }
    }
}

impl Default for GitIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: "auto".to_string(),
            credentials: GitCredentials::default(),
            hooks_enabled: true,
            hooks_directory: None,
            workflow: "gitflow".to_string(),
            branch_naming: HashMap::new(),
        }
    }
}

impl Default for GitCredentials {
    fn default() -> Self {
        Self {
            username: None,
            email: None,
            ssh_key_path: None,
            personal_access_token: None,
            oauth_token: None,
        }
    }
}

impl Default for IDEIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            supported_ides: vec![
                "vscode".to_string(),
                "intellij".to_string(),
                "vim".to_string(),
            ],
            ide_settings: HashMap::new(),
            auto_sync_enabled: true,
            sync_interval: 300,
        }
    }
}

impl Default for CICDIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "github".to_string(),
            settings: HashMap::new(),
            auto_deploy_enabled: false,
            deployment_environments: vec!["staging".to_string(), "production".to_string()],
        }
    }
}

impl Default for CloudIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "aws".to_string(),
            credentials: CloudCredentials::default(),
            regions: vec!["us-east-1".to_string()],
            services: vec!["s3".to_string(), "ec2".to_string()],
        }
    }
}

impl Default for CloudCredentials {
    fn default() -> Self {
        Self {
            access_key_id: None,
            secret_access_key: None,
            session_token: None,
            credentials_file: None,
            profile_name: None,
        }
    }
}

impl Default for ConfigHealth {
    fn default() -> Self {
        Self {
            status: super::ConfigHealthStatus::Unknown,
            issues: Vec::new(),
            last_check: Utc::now(),
        }
    }
}
