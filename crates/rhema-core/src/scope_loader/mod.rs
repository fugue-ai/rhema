pub mod advanced_analytics;
pub mod analytics;
pub mod background;
pub mod cicd_integration;
pub mod config;
pub mod distributed;
pub mod git_integration;
pub mod historical_analysis;
pub mod incremental;
pub mod integration;
pub mod integration_apis;
pub mod ml_confidence;
pub mod pattern_recognition;
pub mod plugin;
pub mod plugin_enhancements;
pub mod plugins;
pub mod registry;
pub mod resource_optimization;
pub mod service;
pub mod types;

#[cfg(test)]
pub mod test_example;

pub use advanced_analytics::*;
pub use analytics::*;
pub use background::*;
pub use cicd_integration::*;
pub use distributed::*;
pub use git_integration::*;
pub use historical_analysis::*;
pub use incremental::*;
pub use integration::*;
pub use integration_apis::*;
pub use plugin::ScopeLoaderPlugin;
pub use plugin_enhancements::*;
pub use plugins::*;
pub use registry::PluginRegistry;
pub use resource_optimization::*;
pub use service::ScopeLoaderService;

// Re-export specific types to avoid ambiguity
pub use config::{
    GlobalScopeLoaderConfig, PluginConfig as ConfigPluginConfig, ScopeLoaderConfigManager,
};
pub use types::{
    PackageBoundary, PackageManager, PluginConfig as TypesPluginConfig, PluginError,
    PluginMetadata, RegistryError, ScopeContext, ScopeLoaderError, ScopeSuggestion, ScopeType,
};
