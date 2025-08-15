pub mod error;
pub mod file_ops;
pub mod lock;
pub mod schema;
pub mod scope;
pub mod scope_loader;
pub mod utils;
pub mod coordination;

pub use error::{RhemaError, RhemaResult};
pub use lock::*;
pub use schema::*;
pub use scope::*;
pub use scope_loader::{
    ConfigPluginConfig, PackageBoundary, PackageManager, PluginError, PluginMetadata,
    PluginRegistry, RegistryError, ScopeContext, ScopeLoaderError, ScopeLoaderPlugin,
    ScopeLoaderService, ScopeSuggestion, ScopeType,
};
pub use coordination::*;
