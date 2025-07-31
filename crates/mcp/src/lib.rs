pub mod auth;
pub mod cache;
pub mod context;
pub mod mcp;
pub mod sdk;
pub mod watcher;

// Re-export specific types to avoid ambiguity
pub use auth::AuthManager;
pub use cache::{CacheConfig as CacheManagerConfig, CacheManager};
pub use context::ContextProvider;
pub use mcp::{AuthConfig, CacheConfig, McpConfig, McpDaemon, WatcherConfig};
pub use sdk::{ContextProviderExt, RhemaMcpServer};
pub use watcher::{FileWatcher, WatcherConfig as FileWatcherConfig};
