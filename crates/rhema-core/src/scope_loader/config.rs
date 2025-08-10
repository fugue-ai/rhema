use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::RhemaResult;

/// Global scope loader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalScopeLoaderConfig {
    /// Plugin configurations
    pub plugins: HashMap<String, PluginConfig>,
    
    /// Auto-discovery settings
    pub auto_discovery: AutoDiscoveryConfig,
    
    /// Monorepo settings
    pub monorepo: MonorepoConfig,
    
    /// Caching settings
    pub caching: CachingConfig,
    
    /// Performance settings
    pub performance: PerformanceConfig,
    
    /// Analytics settings
    pub analytics: AnalyticsConfig,
    
    /// Plugin management settings
    pub plugin_management: PluginManagementConfig,
}

/// Plugin-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Whether the plugin is enabled
    pub enabled: bool,
    
    /// Plugin priority (higher numbers = higher priority)
    pub priority: u32,
    
    /// Plugin-specific settings
    pub settings: HashMap<String, serde_json::Value>,
    
    /// Custom confidence threshold for this plugin
    pub confidence_threshold: Option<f64>,
    
    /// Plugin timeout in seconds
    pub timeout: Option<u64>,
    
    /// Whether to include development dependencies
    pub include_dev_dependencies: bool,
    
    /// Minimum package size to consider
    pub min_package_size: Option<usize>,
}

/// Auto-discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDiscoveryConfig {
    /// Whether auto-discovery is enabled
    pub enabled: bool,
    
    /// Minimum confidence threshold for scope creation
    pub min_confidence: f64,
    
    /// Whether to create scopes automatically without confirmation
    pub auto_create: bool,
    
    /// Whether to show confirmation prompts
    pub confirm_prompt: bool,
    
    /// Whether to show detailed reasoning
    pub verbose: bool,
    
    /// Maximum depth for boundary detection
    pub max_depth: usize,
    
    /// Whether to include hidden directories
    pub include_hidden: bool,
    
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
    
    /// File patterns to include
    pub include_patterns: Vec<String>,
}

/// Monorepo configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonorepoConfig {
    /// Whether monorepo support is enabled
    pub enabled: bool,
    
    /// Maximum depth for monorepo detection
    pub max_depth: usize,
    
    /// Whether to perform cross-package analysis
    pub cross_package_analysis: bool,
    
    /// Whether to create hierarchical scopes
    pub hierarchical_scopes: bool,
    
    /// Whether to detect workspace configurations
    pub detect_workspaces: bool,
    
    /// Whether to analyze dependencies across packages
    pub analyze_dependencies: bool,
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    
    /// Cache duration in seconds
    pub cache_duration: u64,
    
    /// Cache path
    pub cache_path: Option<String>,
    
    /// Maximum cache size in MB
    pub max_cache_size: Option<usize>,
    
    /// Whether to enable cache compression
    pub enable_compression: bool,
    
    /// Cache cleanup interval in seconds
    pub cleanup_interval: Option<u64>,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum number of concurrent plugin executions
    pub max_concurrent_plugins: usize,
    
    /// Plugin execution timeout in seconds
    pub plugin_timeout: u64,
    
    /// Whether to enable parallel processing
    pub enable_parallel_processing: bool,
    
    /// Maximum memory usage in MB
    pub max_memory_usage: Option<usize>,
    
    /// Whether to enable performance monitoring
    pub enable_monitoring: bool,
}

/// Analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Whether analytics are enabled
    pub enabled: bool,
    
    /// Analytics data path
    pub data_path: Option<String>,
    
    /// Whether to track plugin performance
    pub track_plugin_performance: bool,
    
    /// Whether to track scope creation success rates
    pub track_success_rates: bool,
    
    /// Whether to track usage statistics
    pub track_usage_stats: bool,
    
    /// Analytics retention period in days
    pub retention_period: Option<u32>,
}

/// Plugin management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManagementConfig {
    /// Path to custom plugins
    pub plugins_path: Option<String>,
    
    /// Whether to auto-update plugins
    pub auto_update_plugins: bool,
    
    /// Plugin update interval in hours
    pub update_interval: Option<u32>,
    
    /// Whether to enable plugin marketplace
    pub enable_marketplace: bool,
    
    /// Marketplace URL
    pub marketplace_url: Option<String>,
    
    /// Whether to enable plugin versioning
    pub enable_versioning: bool,
}

impl Default for GlobalScopeLoaderConfig {
    fn default() -> Self {
        let mut plugins = HashMap::new();
        
        // Default Cargo plugin config
        plugins.insert("cargo".to_string(), PluginConfig {
            enabled: true,
            priority: 100,
            settings: HashMap::new(),
            confidence_threshold: None,
            timeout: None,
            include_dev_dependencies: true,
            min_package_size: Some(100),
        });
        
        // Default Node.js plugin config
        plugins.insert("node".to_string(), PluginConfig {
            enabled: true,
            priority: 90,
            settings: HashMap::new(),
            confidence_threshold: None,
            timeout: None,
            include_dev_dependencies: false,
            min_package_size: Some(1000),
        });
        
        // Default Nx plugin config
        plugins.insert("nx".to_string(), PluginConfig {
            enabled: true,
            priority: 80,
            settings: HashMap::new(),
            confidence_threshold: None,
            timeout: None,
            include_dev_dependencies: false,
            min_package_size: Some(500),
        });

        Self {
            plugins,
            auto_discovery: AutoDiscoveryConfig::default(),
            monorepo: MonorepoConfig::default(),
            caching: CachingConfig::default(),
            performance: PerformanceConfig::default(),
            analytics: AnalyticsConfig::default(),
            plugin_management: PluginManagementConfig::default(),
        }
    }
}

impl Default for AutoDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_confidence: 0.8,
            auto_create: false,
            confirm_prompt: true,
            verbose: false,
            max_depth: 5,
            include_hidden: false,
            exclude_patterns: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            include_patterns: vec![],
        }
    }
}

impl Default for MonorepoConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_depth: 5,
            cross_package_analysis: true,
            hierarchical_scopes: true,
            detect_workspaces: true,
            analyze_dependencies: true,
        }
    }
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_duration: 3600,
            cache_path: Some(".rhema/cache".to_string()),
            max_cache_size: Some(100),
            enable_compression: true,
            cleanup_interval: Some(86400), // 24 hours
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_plugins: 4,
            plugin_timeout: 30,
            enable_parallel_processing: true,
            max_memory_usage: Some(512),
            enable_monitoring: true,
        }
    }
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            data_path: Some(".rhema/analytics".to_string()),
            track_plugin_performance: true,
            track_success_rates: true,
            track_usage_stats: true,
            retention_period: Some(30),
        }
    }
}

impl Default for PluginManagementConfig {
    fn default() -> Self {
        Self {
            plugins_path: Some("~/.rhema/plugins".to_string()),
            auto_update_plugins: true,
            update_interval: Some(24),
            enable_marketplace: false,
            marketplace_url: Some("https://plugins.rhema.ai".to_string()),
            enable_versioning: true,
        }
    }
}

impl GlobalScopeLoaderConfig {
    /// Load configuration from file
    pub fn load_from_file(path: &Path) -> RhemaResult<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| crate::RhemaError::IoError(e))?;

        let config: Self = serde_yaml::from_str(&content)
            .map_err(|e| crate::RhemaError::InvalidYaml {
                file: path.display().to_string(),
                message: e.to_string(),
            })?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &Path) -> RhemaResult<()> {
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| crate::RhemaError::IoError(e))?;
        }

        let content = serde_yaml::to_string(self)
            .map_err(|e| crate::RhemaError::InvalidYaml {
                file: path.display().to_string(),
                message: e.to_string(),
            })?;

        fs::write(path, content)
            .map_err(|e| crate::RhemaError::IoError(e))?;

        Ok(())
    }

    /// Load global configuration from default location
    pub fn load_global() -> RhemaResult<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::RhemaError::ConfigError("Could not determine config directory".to_string()))?
            .join("rhema");
        
        let config_path = config_dir.join("scope-loader.yaml");
        Self::load_from_file(&config_path)
    }

    /// Save global configuration to default location
    pub fn save_global(&self) -> RhemaResult<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::RhemaError::ConfigError("Could not determine config directory".to_string()))?
            .join("rhema");
        
        let config_path = config_dir.join("scope-loader.yaml");
        self.save_to_file(&config_path)
    }

    /// Load project-specific configuration
    pub fn load_project(project_root: &Path) -> RhemaResult<Self> {
        let config_path = project_root.join(".rhema").join("scope-loader.yaml");
        Self::load_from_file(&config_path)
    }

    /// Save project-specific configuration
    pub fn save_project(&self, project_root: &Path) -> RhemaResult<()> {
        let config_path = project_root.join(".rhema").join("scope-loader.yaml");
        self.save_to_file(&config_path)
    }

    /// Get plugin configuration by name
    pub fn get_plugin_config(&self, name: &str) -> Option<&PluginConfig> {
        self.plugins.get(name)
    }

    /// Get plugin configuration by name (mutable)
    pub fn get_plugin_config_mut(&mut self, name: &str) -> Option<&mut PluginConfig> {
        self.plugins.get_mut(name)
    }

    /// Check if a plugin is enabled
    pub fn is_plugin_enabled(&self, name: &str) -> bool {
        self.plugins.get(name)
            .map(|config| config.enabled)
            .unwrap_or(false)
    }

    /// Get plugin priority
    pub fn get_plugin_priority(&self, name: &str) -> u32 {
        self.plugins.get(name)
            .map(|config| config.priority)
            .unwrap_or(0)
    }

    /// Get plugin confidence threshold
    pub fn get_plugin_confidence_threshold(&self, name: &str) -> f64 {
        self.plugins.get(name)
            .and_then(|config| config.confidence_threshold)
            .unwrap_or(self.auto_discovery.min_confidence)
    }

    /// Merge with another configuration
    pub fn merge(&mut self, other: &Self) {
        // Merge plugins
        for (name, config) in &other.plugins {
            self.plugins.insert(name.clone(), config.clone());
        }

        // Merge auto-discovery settings
        if other.auto_discovery.enabled {
            self.auto_discovery = other.auto_discovery.clone();
        }

        // Merge monorepo settings
        if other.monorepo.enabled {
            self.monorepo = other.monorepo.clone();
        }

        // Merge caching settings
        if other.caching.enabled {
            self.caching = other.caching.clone();
        }

        // Merge performance settings
        self.performance = other.performance.clone();

        // Merge analytics settings
        if other.analytics.enabled {
            self.analytics = other.analytics.clone();
        }

        // Merge plugin management settings
        self.plugin_management = other.plugin_management.clone();
    }

    /// Validate configuration
    pub fn validate(&self) -> RhemaResult<()> {
        // Validate confidence thresholds
        if self.auto_discovery.min_confidence < 0.0 || self.auto_discovery.min_confidence > 1.0 {
            return Err(crate::RhemaError::ConfigError(
                "min_confidence must be between 0.0 and 1.0".to_string()
            ));
        }

        // Validate plugin configurations
        for (name, config) in &self.plugins {
            if let Some(threshold) = config.confidence_threshold {
                if threshold < 0.0 || threshold > 1.0 {
                    return Err(crate::RhemaError::ConfigError(
                        format!("Plugin {} confidence threshold must be between 0.0 and 1.0", name)
                    ));
                }
            }
        }

        // Validate performance settings
        if self.performance.max_concurrent_plugins == 0 {
            return Err(crate::RhemaError::ConfigError(
                "max_concurrent_plugins must be greater than 0".to_string()
            ));
        }

        Ok(())
    }
}

/// Configuration manager for scope loader
pub struct ScopeLoaderConfigManager {
    global_config: GlobalScopeLoaderConfig,
    project_config: Option<GlobalScopeLoaderConfig>,
}

impl ScopeLoaderConfigManager {
    /// Create a new configuration manager
    pub fn new() -> RhemaResult<Self> {
        let global_config = GlobalScopeLoaderConfig::load_global()?;
        Ok(Self {
            global_config,
            project_config: None,
        })
    }

    /// Load project configuration
    pub fn load_project_config(&mut self, project_root: &Path) -> RhemaResult<()> {
        self.project_config = Some(GlobalScopeLoaderConfig::load_project(project_root)?);
        Ok(())
    }

    /// Get effective configuration (global + project)
    pub fn get_effective_config(&self) -> GlobalScopeLoaderConfig {
        let mut config = self.global_config.clone();
        
        if let Some(project_config) = &self.project_config {
            config.merge(project_config);
        }

        config
    }

    /// Get global configuration
    pub fn get_global_config(&self) -> &GlobalScopeLoaderConfig {
        &self.global_config
    }

    /// Get project configuration
    pub fn get_project_config(&self) -> Option<&GlobalScopeLoaderConfig> {
        self.project_config.as_ref()
    }

    /// Update global configuration
    pub fn update_global_config(&mut self, config: GlobalScopeLoaderConfig) -> RhemaResult<()> {
        config.validate()?;
        self.global_config = config;
        self.global_config.save_global()?;
        Ok(())
    }

    /// Update project configuration
    pub fn update_project_config(&mut self, project_root: &Path, config: GlobalScopeLoaderConfig) -> RhemaResult<()> {
        config.validate()?;
        self.project_config = Some(config.clone());
        config.save_project(project_root)?;
        Ok(())
    }

    /// Reset to default configuration
    pub fn reset_to_defaults(&mut self) -> RhemaResult<()> {
        self.global_config = GlobalScopeLoaderConfig::default();
        self.global_config.save_global()?;
        Ok(())
    }
}
