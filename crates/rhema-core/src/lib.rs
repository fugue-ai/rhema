pub mod audit;
pub mod cache;
pub mod coordination;
pub mod error;
pub mod fileops;
pub mod lock;
pub mod schema;
pub mod scope;
pub mod scope_loader;
pub mod utils;
pub mod validation;

pub use audit::*;
pub use cache::*;

// Re-export coordination types (excluding conflicts)
pub use coordination::{
    AgentInfo, AgentMessage, AgentPerformanceMetrics, AgentStatus, ConnectionStats,
    CoordinationConfig, CoordinationManager, GrpcCoordinationClient, HealthCheckConfig,
    MessagePriority, MessageType, MockCoordinationClient, RetryConfig, TlsConfig,
};

pub use error::{RhemaError, RhemaResult};
pub use lock::*;
pub use schema::*;
pub use scope::*;
pub use scope_loader::{
    ConfigPluginConfig, PackageBoundary, PackageManager, PluginError, PluginMetadata,
    PluginRegistry, RegistryError, ScopeContext, ScopeLoaderError, ScopeLoaderPlugin,
    ScopeLoaderService, ScopeSuggestion, ScopeType,
};
pub use validation::*;
