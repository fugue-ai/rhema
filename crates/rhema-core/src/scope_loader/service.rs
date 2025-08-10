use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use super::registry::PluginRegistry;
use super::types::*;
use crate::scope::Scope;

/// Configuration for the scope loader service
#[derive(Debug, Clone)]
pub struct ScopeLoaderConfig {
    /// Minimum confidence threshold for scope creation
    pub min_confidence_threshold: f64,
    /// Whether to auto-create scopes without confirmation
    pub auto_create: bool,
    /// Whether to show confirmation prompts
    pub confirm_prompt: bool,
    /// Maximum depth for boundary detection
    pub max_depth: usize,
    /// Whether to enable caching
    pub enable_caching: bool,
    /// Cache duration in seconds
    pub cache_duration: u64,
    /// Cache path
    pub cache_path: Option<String>,
}

impl Default for ScopeLoaderConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.8,
            auto_create: false,
            confirm_prompt: true,
            max_depth: 5,
            enable_caching: true,
            cache_duration: 3600,
            cache_path: None,
        }
    }
}

/// Cache for scope loader results
#[derive(Debug, Clone)]
pub struct ScopeCache {
    boundaries: HashMap<String, (Vec<PackageBoundary>, chrono::DateTime<chrono::Utc>)>,
    suggestions: HashMap<String, (Vec<ScopeSuggestion>, chrono::DateTime<chrono::Utc>)>,
    scopes: HashMap<String, (Vec<Scope>, chrono::DateTime<chrono::Utc>)>,
}

impl ScopeCache {
    pub fn new() -> Self {
        Self {
            boundaries: HashMap::new(),
            suggestions: HashMap::new(),
            scopes: HashMap::new(),
        }
    }

    pub fn get_boundaries(&self, key: &str) -> Option<&Vec<PackageBoundary>> {
        self.boundaries.get(key).and_then(|(boundaries, timestamp)| {
            if chrono::Utc::now().signed_duration_since(*timestamp).num_seconds() < 3600 {
                Some(boundaries)
            } else {
                None
            }
        })
    }

    pub fn set_boundaries(&mut self, key: String, boundaries: Vec<PackageBoundary>) {
        self.boundaries.insert(key, (boundaries, chrono::Utc::now()));
    }

    pub fn get_suggestions(&self, key: &str) -> Option<&Vec<ScopeSuggestion>> {
        self.suggestions.get(key).and_then(|(suggestions, timestamp)| {
            if chrono::Utc::now().signed_duration_since(*timestamp).num_seconds() < 3600 {
                Some(suggestions)
            } else {
                None
            }
        })
    }

    pub fn set_suggestions(&mut self, key: String, suggestions: Vec<ScopeSuggestion>) {
        self.suggestions.insert(key, (suggestions, chrono::Utc::now()));
    }

    pub fn get_scopes(&self, key: &str) -> Option<&Vec<Scope>> {
        self.scopes.get(key).and_then(|(scopes, timestamp)| {
            if chrono::Utc::now().signed_duration_since(*timestamp).num_seconds() < 3600 {
                Some(scopes)
            } else {
                None
            }
        })
    }

    pub fn set_scopes(&mut self, key: String, scopes: Vec<Scope>) {
        self.scopes.insert(key, (scopes, chrono::Utc::now()));
    }

    pub fn clear(&mut self) {
        self.boundaries.clear();
        self.suggestions.clear();
        self.scopes.clear();
    }
}

impl Default for ScopeCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Main service for scope loading and discovery
pub struct ScopeLoaderService {
    registry: PluginRegistry,
    cache: Arc<tokio::sync::RwLock<ScopeCache>>,
    config: ScopeLoaderConfig,
}

impl ScopeLoaderService {
    /// Create a new scope loader service
    pub fn new(registry: PluginRegistry, config: ScopeLoaderConfig) -> Self {
        Self {
            registry,
            cache: Arc::new(tokio::sync::RwLock::new(ScopeCache::new())),
            config,
        }
    }

    /// Create a new scope loader service with default configuration
    pub fn new_with_default_config(registry: PluginRegistry) -> Self {
        Self::new(registry, ScopeLoaderConfig::default())
    }

    /// Discover scopes using the plugin system
    pub async fn discover_scopes(&self, path: &Path) -> Result<Vec<Scope>, ScopeLoaderError> {
        // Check cache first
        let cache_key = path.to_string_lossy().to_string();
        if self.config.enable_caching {
            if let Some(cached_scopes) = self.cache.read().await.get_scopes(&cache_key) {
                return Ok(cached_scopes.clone());
            }
        }

        // Get applicable plugins
        let plugins = self.registry.get_plugins_for_path(path);
        
        if plugins.is_empty() {
            return Err(ScopeLoaderError::NoPluginsFound(path.display().to_string()));
        }

        // Detect package boundaries
        let mut all_boundaries = Vec::new();
        for plugin in plugins {
            match plugin.detect_boundaries(path) {
                Ok(boundaries) => {
                    all_boundaries.extend(boundaries);
                }
                Err(e) => {
                    eprintln!("Plugin {} failed to detect boundaries: {}", 
                             plugin.metadata().name, e);
                }
            }
        }

        // Generate scope suggestions
        let suggestions = self.generate_suggestions(&all_boundaries)?;
        
        // Create scopes from suggestions
        let scopes = self.create_scopes_from_suggestions(&suggestions)?;

        // Cache the results
        if self.config.enable_caching {
            self.cache.write().await.set_scopes(cache_key, scopes.clone());
        }

        Ok(scopes)
    }

    /// Auto-create scopes based on detected boundaries
    pub async fn auto_create_scopes(&self, path: &Path) -> Result<Vec<Scope>, ScopeLoaderError> {
        let suggestions = self.suggest_scopes(path).await?;
        
        // Filter high-confidence suggestions
        let high_confidence: Vec<_> = suggestions
            .into_iter()
            .filter(|s| s.confidence >= self.config.min_confidence_threshold)
            .collect();

        if high_confidence.is_empty() {
            return Ok(Vec::new());
        }

        // Create scopes from high-confidence suggestions
        self.create_scopes_from_suggestions(&high_confidence)
    }

    /// Generate scope suggestions for a path
    pub async fn suggest_scopes(&self, path: &Path) -> Result<Vec<ScopeSuggestion>, ScopeLoaderError> {
        // Check cache first
        let cache_key = format!("{}_suggestions", path.to_string_lossy());
        if self.config.enable_caching {
            if let Some(cached_suggestions) = self.cache.read().await.get_suggestions(&cache_key) {
                return Ok(cached_suggestions.clone());
            }
        }

        let suggestions = self.registry.suggest_scopes(path)?;

        // Cache the results
        if self.config.enable_caching {
            self.cache.write().await.set_suggestions(cache_key, suggestions.clone());
        }

        Ok(suggestions)
    }

    /// Detect package boundaries for a path
    pub async fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, ScopeLoaderError> {
        // Check cache first
        let cache_key = format!("{}_boundaries", path.to_string_lossy());
        if self.config.enable_caching {
            if let Some(cached_boundaries) = self.cache.read().await.get_boundaries(&cache_key) {
                return Ok(cached_boundaries.clone());
            }
        }

        let boundaries = self.registry.detect_boundaries(path)?;

        // Cache the results
        if self.config.enable_caching {
            self.cache.write().await.set_boundaries(cache_key, boundaries.clone());
        }

        Ok(boundaries)
    }

    /// Generate scope suggestions from package boundaries
    fn generate_suggestions(&self, boundaries: &[PackageBoundary]) -> Result<Vec<ScopeSuggestion>, ScopeLoaderError> {
        let plugins = self.registry.list_plugins();
        let mut all_suggestions = Vec::new();

        for plugin_info in plugins {
            if let Some(plugin) = self.registry.get_plugin(&plugin_info.name) {
                match plugin.suggest_scopes(boundaries) {
                    Ok(suggestions) => {
                        all_suggestions.extend(suggestions);
                    }
                    Err(e) => {
                        eprintln!("Plugin {} failed to generate suggestions: {}", 
                                 plugin_info.name, e);
                    }
                }
            }
        }

        // Sort by confidence (higher confidence first)
        all_suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        Ok(all_suggestions)
    }

    /// Create scopes from suggestions
    fn create_scopes_from_suggestions(&self, suggestions: &[ScopeSuggestion]) -> Result<Vec<Scope>, ScopeLoaderError> {
        let mut scopes = Vec::new();

        for suggestion in suggestions {
            match self.create_scope_from_suggestion(suggestion) {
                Ok(scope) => {
                    scopes.push(scope);
                }
                Err(e) => {
                    eprintln!("Failed to create scope from suggestion {}: {}", 
                             suggestion.name, e);
                }
            }
        }

        Ok(scopes)
    }

    /// Create a single scope from a suggestion
    fn create_scope_from_suggestion(&self, suggestion: &ScopeSuggestion) -> Result<Scope, ScopeLoaderError> {
        // Create the scope directory structure
        let scope_path = &suggestion.path;
        
        // Create .rhema directory if it doesn't exist
        let rhema_dir = scope_path.join(".rhema");
        if !rhema_dir.exists() {
            std::fs::create_dir_all(&rhema_dir)
                .map_err(|e| ScopeLoaderError::ScopeCreationFailed(format!("Failed to create .rhema directory: {}", e)))?;
        }

        // Create rhema.yaml file
        let rhema_content = self.generate_rhema_yaml(suggestion)?;
        let rhema_file = rhema_dir.join("rhema.yaml");
        std::fs::write(&rhema_file, rhema_content)
            .map_err(|e| ScopeLoaderError::ScopeCreationFailed(format!("Failed to write rhema.yaml: {}", e)))?;

        // Create the scope
        Scope::new(scope_path.clone())
            .map_err(|e| ScopeLoaderError::ScopeCreationFailed(format!("Failed to create scope: {}", e)))
    }

    /// Generate rhema.yaml content for a scope suggestion
    fn generate_rhema_yaml(&self, suggestion: &ScopeSuggestion) -> Result<String, ScopeLoaderError> {
        let _yaml = serde_yaml::to_string(&serde_yaml::Mapping::new())
            .map_err(|e| ScopeLoaderError::ScopeCreationFailed(format!("Failed to create YAML: {}", e)))?;

        // Create a basic rhema.yaml structure
        let mut mapping = serde_yaml::Mapping::new();
        
        // Add scope name
        mapping.insert(
            serde_yaml::Value::String("name".to_string()),
            serde_yaml::Value::String(suggestion.name.clone()),
        );

        // Add scope type
        mapping.insert(
            serde_yaml::Value::String("type".to_string()),
            serde_yaml::Value::String(suggestion.scope_type.as_str().to_string()),
        );

        // Add description
        mapping.insert(
            serde_yaml::Value::String("description".to_string()),
            serde_yaml::Value::String(suggestion.reasoning.clone()),
        );

        // Add confidence
        mapping.insert(
            serde_yaml::Value::String("confidence".to_string()),
            serde_yaml::Value::Number(serde_yaml::Number::from(suggestion.confidence as i64)),
        );

        // Add dependencies if any
        if !suggestion.dependencies.is_empty() {
            let deps: Vec<serde_yaml::Value> = suggestion.dependencies
                .iter()
                .map(|d| serde_yaml::Value::String(d.clone()))
                .collect();
            mapping.insert(
                serde_yaml::Value::String("dependencies".to_string()),
                serde_yaml::Value::Sequence(deps),
            );
        }

        // Add metadata if any
        if !suggestion.metadata.is_empty() {
            let metadata_value = serde_yaml::to_value(&suggestion.metadata)
                .map_err(|e| ScopeLoaderError::ScopeCreationFailed(format!("Failed to serialize metadata: {}", e)))?;
            mapping.insert(
                serde_yaml::Value::String("metadata".to_string()),
                metadata_value,
            );
        }

        serde_yaml::to_string(&mapping)
            .map_err(|e| ScopeLoaderError::ScopeCreationFailed(format!("Failed to serialize YAML: {}", e)))
    }

    /// Get the plugin registry
    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    /// Get mutable access to the plugin registry
    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    /// Get the service configuration
    pub fn config(&self) -> &ScopeLoaderConfig {
        &self.config
    }

    /// Update the service configuration
    pub fn update_config(&mut self, config: ScopeLoaderConfig) {
        self.config = config;
    }

    /// Clear the cache
    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        CacheStats {
            boundaries_count: cache.boundaries.len(),
            suggestions_count: cache.suggestions.len(),
            scopes_count: cache.scopes.len(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub boundaries_count: usize,
    pub suggestions_count: usize,
    pub scopes_count: usize,
}
