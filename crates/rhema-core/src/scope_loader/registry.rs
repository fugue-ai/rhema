use std::collections::HashMap;
use std::path::Path;

use super::plugin::ScopeLoaderPlugin;
use super::types::*;

/// Registry for managing scope loader plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn ScopeLoaderPlugin>>,
    plugin_configs: HashMap<String, PluginConfig>,
    plugin_factories: HashMap<String, Box<dyn super::plugin::PluginFactory>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_configs: HashMap::new(),
            plugin_factories: HashMap::new(),
        }
    }

    /// Register a plugin with the registry
    pub fn register_plugin(&mut self, plugin: Box<dyn ScopeLoaderPlugin>) -> Result<(), RegistryError> {
        let metadata = plugin.metadata();
        let plugin_name = metadata.name.clone();

        if self.plugins.contains_key(&plugin_name) {
            return Err(RegistryError::PluginAlreadyRegistered(plugin_name));
        }

        self.plugins.insert(plugin_name.clone(), plugin);
        
        // Set default configuration if not already set
        if !self.plugin_configs.contains_key(&plugin_name) {
            self.plugin_configs.insert(plugin_name, PluginConfig {
                enabled: true,
                priority: metadata.priority,
                settings: HashMap::new(),
            });
        }

        Ok(())
    }

    /// Register a plugin factory
    pub fn register_factory(&mut self, factory: Box<dyn super::plugin::PluginFactory>) -> Result<(), RegistryError> {
        let factory_name = factory.plugin_name().to_string();
        
        if self.plugin_factories.contains_key(&factory_name) {
            return Err(RegistryError::PluginAlreadyRegistered(factory_name));
        }

        self.plugin_factories.insert(factory_name, factory);
        Ok(())
    }

    /// Get all plugins that can handle the given path
    pub fn get_plugins_for_path(&self, path: &Path) -> Vec<&dyn ScopeLoaderPlugin> {
        let mut applicable_plugins = Vec::new();

        for (name, plugin) in &self.plugins {
            // Check if plugin is enabled
            if let Some(config) = self.plugin_configs.get(name) {
                if !config.enabled {
                    continue;
                }
            }

            // Check if plugin can handle the path
            if plugin.can_handle(path) {
                applicable_plugins.push(plugin.as_ref());
            }
        }

        // Sort by priority (higher priority first)
        applicable_plugins.sort_by(|a, b| {
            let a_priority = self.plugin_configs.get(&a.metadata().name)
                .map(|c| c.priority)
                .unwrap_or(0);
            let b_priority = self.plugin_configs.get(&b.metadata().name)
                .map(|c| c.priority)
                .unwrap_or(0);
            b_priority.cmp(&a_priority)
        });

        applicable_plugins
    }

    /// Detect package boundaries using all applicable plugins
    pub fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError> {
        let plugins = self.get_plugins_for_path(path);
        
        if plugins.is_empty() {
            return Err(PluginError::PluginNotFound(format!(
                "No plugins found for path: {}",
                path.display()
            )));
        }

        let mut all_boundaries = Vec::new();

        for plugin in plugins {
            match plugin.detect_boundaries(path) {
                Ok(boundaries) => {
                    all_boundaries.extend(boundaries);
                }
                Err(e) => {
                    // Log error but continue with other plugins
                    eprintln!("Plugin {} failed to detect boundaries: {}", 
                             plugin.metadata().name, e);
                }
            }
        }

        Ok(all_boundaries)
    }

    /// Generate scope suggestions using all applicable plugins
    pub fn suggest_scopes(&self, path: &Path) -> Result<Vec<ScopeSuggestion>, PluginError> {
        let boundaries = self.detect_boundaries(path)?;
        let plugins = self.get_plugins_for_path(path);
        
        let mut all_suggestions = Vec::new();

        for plugin in plugins {
            match plugin.suggest_scopes(&boundaries) {
                Ok(suggestions) => {
                    all_suggestions.extend(suggestions);
                }
                Err(e) => {
                    // Log error but continue with other plugins
                    eprintln!("Plugin {} failed to generate suggestions: {}", 
                             plugin.metadata().name, e);
                }
            }
        }

        // Sort suggestions by confidence (higher confidence first)
        all_suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        Ok(all_suggestions)
    }

    /// Create scopes from suggestions using the best plugin
    pub fn create_scopes(&self, path: &Path, suggestions: &[ScopeSuggestion]) -> Result<Vec<crate::scope::Scope>, PluginError> {
        let plugins = self.get_plugins_for_path(path);
        
        if plugins.is_empty() {
            return Err(PluginError::PluginNotFound(format!(
                "No plugins found for path: {}",
                path.display()
            )));
        }

        // Use the highest priority plugin to create scopes
        let best_plugin = plugins.first().unwrap();
        
        best_plugin.create_scopes(suggestions)
    }

    /// Get plugin configuration
    pub fn get_plugin_config(&self, plugin_name: &str) -> Option<&PluginConfig> {
        self.plugin_configs.get(plugin_name)
    }

    /// Update plugin configuration
    pub fn update_plugin_config(&mut self, plugin_name: &str, config: PluginConfig) -> Result<(), RegistryError> {
        if !self.plugins.contains_key(plugin_name) {
            return Err(RegistryError::PluginNotFound(plugin_name.to_string()));
        }

        self.plugin_configs.insert(plugin_name.to_string(), config);
        Ok(())
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, plugin_name: &str) -> Result<(), RegistryError> {
        if let Some(config) = self.plugin_configs.get_mut(plugin_name) {
            config.enabled = true;
            Ok(())
        } else {
            Err(RegistryError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, plugin_name: &str) -> Result<(), RegistryError> {
        if let Some(config) = self.plugin_configs.get_mut(plugin_name) {
            config.enabled = false;
            Ok(())
        } else {
            Err(RegistryError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins
            .iter()
            .map(|(name, plugin)| {
                let config = self.plugin_configs.get(name).cloned();
                PluginInfo {
                    name: name.clone(),
                    metadata: plugin.metadata(),
                    config,
                }
            })
            .collect()
    }

    /// Get plugin by name
    pub fn get_plugin(&self, plugin_name: &str) -> Option<&dyn ScopeLoaderPlugin> {
        self.plugins.get(plugin_name).map(|p| p.as_ref())
    }

    /// Remove a plugin from the registry
    pub fn remove_plugin(&mut self, plugin_name: &str) -> Result<(), RegistryError> {
        if self.plugins.remove(plugin_name).is_some() {
            self.plugin_configs.remove(plugin_name);
            Ok(())
        } else {
            Err(RegistryError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Clear all plugins
    pub fn clear(&mut self) {
        self.plugins.clear();
        self.plugin_configs.clear();
        self.plugin_factories.clear();
    }

    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a registered plugin
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub metadata: PluginMetadata,
    pub config: Option<PluginConfig>,
}

/// Plugin registry builder for fluent configuration
pub struct PluginRegistryBuilder {
    registry: PluginRegistry,
}

impl PluginRegistryBuilder {
    /// Create a new registry builder
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
        }
    }

    /// Add a plugin to the registry
    pub fn with_plugin(mut self, plugin: Box<dyn ScopeLoaderPlugin>) -> Result<Self, RegistryError> {
        self.registry.register_plugin(plugin)?;
        Ok(self)
    }

    /// Add a plugin factory to the registry
    pub fn with_factory(mut self, factory: Box<dyn super::plugin::PluginFactory>) -> Result<Self, RegistryError> {
        self.registry.register_factory(factory)?;
        Ok(self)
    }

    /// Configure a plugin
    pub fn with_plugin_config(mut self, plugin_name: &str, config: PluginConfig) -> Result<Self, RegistryError> {
        self.registry.update_plugin_config(plugin_name, config)?;
        Ok(self)
    }

    /// Build the registry
    pub fn build(self) -> PluginRegistry {
        self.registry
    }
}

impl Default for PluginRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
