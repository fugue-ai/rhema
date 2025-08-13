use crate::scope::Scope;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::types::*;

/// Core trait that all scope loader plugins must implement
pub trait ScopeLoaderPlugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Check if this plugin can handle the given directory
    fn can_handle(&self, path: &Path) -> bool;

    /// Detect package boundaries in the given directory
    fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError>;

    /// Generate scope suggestions based on detected boundaries
    fn suggest_scopes(
        &self,
        boundaries: &[PackageBoundary],
    ) -> Result<Vec<ScopeSuggestion>, PluginError>;

    /// Create scopes from suggestions
    fn create_scopes(&self, suggestions: &[ScopeSuggestion]) -> Result<Vec<Scope>, PluginError>;

    /// Load context for a specific scope
    fn load_context(&self, scope: &Scope) -> Result<ScopeContext, PluginError>;
}

/// Plugin configuration trait for plugins that need configuration
pub trait ConfigurablePlugin: ScopeLoaderPlugin {
    /// Get plugin configuration
    fn get_config(&self) -> &PluginConfig;

    /// Update plugin configuration
    fn update_config(&mut self, config: PluginConfig) -> Result<(), PluginError>;
}

/// Plugin that can be enabled/disabled
pub trait ToggleablePlugin: ScopeLoaderPlugin {
    /// Check if plugin is enabled
    fn is_enabled(&self) -> bool;

    /// Enable the plugin
    fn enable(&mut self);

    /// Disable the plugin
    fn disable(&mut self);
}

/// Plugin that supports priority-based execution
pub trait PrioritizedPlugin: ScopeLoaderPlugin {
    /// Get plugin priority (higher numbers = higher priority)
    fn get_priority(&self) -> u32;

    /// Set plugin priority
    fn set_priority(&mut self, priority: u32);
}

/// Plugin that can be cached
pub trait CacheablePlugin: ScopeLoaderPlugin {
    /// Get cache key for the plugin
    fn get_cache_key(&self, path: &Path) -> String;

    /// Check if cached data is still valid
    fn is_cache_valid(&self, path: &Path, cache_data: &serde_json::Value) -> bool;

    /// Serialize plugin data for caching
    fn serialize_for_cache(
        &self,
        data: &[PackageBoundary],
    ) -> Result<serde_json::Value, PluginError>;

    /// Deserialize plugin data from cache
    fn deserialize_from_cache(
        &self,
        cache_data: &serde_json::Value,
    ) -> Result<Vec<PackageBoundary>, PluginError>;
}

/// Plugin that supports async operations
#[async_trait::async_trait]
pub trait AsyncScopeLoaderPlugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Check if this plugin can handle the given directory
    fn can_handle(&self, path: &Path) -> bool;

    /// Detect package boundaries in the given directory (async)
    async fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError>;

    /// Generate scope suggestions based on detected boundaries (async)
    async fn suggest_scopes(
        &self,
        boundaries: &[PackageBoundary],
    ) -> Result<Vec<ScopeSuggestion>, PluginError>;

    /// Create scopes from suggestions (async)
    async fn create_scopes(
        &self,
        suggestions: &[ScopeSuggestion],
    ) -> Result<Vec<Scope>, PluginError>;

    /// Load context for a specific scope (async)
    async fn load_context(&self, scope: &Scope) -> Result<ScopeContext, PluginError>;
}

/// Plugin factory trait for creating plugin instances
pub trait PluginFactory: Send + Sync {
    /// Create a new plugin instance
    fn create_plugin(
        &self,
        config: Option<PluginConfig>,
    ) -> Result<Box<dyn ScopeLoaderPlugin>, PluginError>;

    /// Get plugin name
    fn plugin_name(&self) -> &str;

    /// Get plugin version
    fn plugin_version(&self) -> &str;

    /// Get supported package managers
    fn supported_package_managers(&self) -> Vec<String>;
}

/// Plugin lifecycle management
pub trait PluginLifecycle: ScopeLoaderPlugin {
    /// Initialize the plugin
    fn initialize(&mut self) -> Result<(), PluginError>;

    /// Shutdown the plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Check if plugin is initialized
    fn is_initialized(&self) -> bool;

    /// Get plugin health status
    fn health_check(&self) -> Result<PluginHealth, PluginError>;
}

/// Plugin health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginHealth {
    pub status: PluginHealthStatus,
    pub message: String,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub metrics: HashMap<String, serde_json::Value>,
}

/// Plugin health status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginHealthStatus {
    Healthy,
    Warning,
    Error,
    Unknown,
}
