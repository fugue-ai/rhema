pub mod analytics;
pub mod config;
pub mod git_integration;
pub mod plugin;
pub mod plugins;
pub mod registry;
pub mod service;
pub mod types;

#[cfg(test)]
pub mod test_example;

pub use analytics::*;
pub use git_integration::*;
pub use plugin::ScopeLoaderPlugin;
pub use plugins::*;
pub use registry::PluginRegistry;
pub use service::ScopeLoaderService;

// Re-export specific types to avoid ambiguity
pub use config::{GlobalScopeLoaderConfig, ScopeLoaderConfigManager, PluginConfig as ConfigPluginConfig};
pub use types::{PackageBoundary, PackageManager, ScopeSuggestion, ScopeType, PluginMetadata, PluginConfig as TypesPluginConfig, ScopeContext, PluginError, RegistryError, ScopeLoaderError};
