pub mod context;
pub mod cache;
pub mod watcher;
pub mod auth;
pub mod sdk;
pub mod mcp;

// Re-export specific types to avoid ambiguity
pub use context::ContextProvider;
pub use cache::{CacheManager, CacheConfig as CacheManagerConfig};
pub use watcher::{FileWatcher, WatcherConfig as FileWatcherConfig};
pub use auth::AuthManager;
pub use sdk::{RhemaMcpServer, ContextProviderExt};
pub use mcp::{McpConfig, CacheConfig, WatcherConfig, AuthConfig, McpDaemon};
