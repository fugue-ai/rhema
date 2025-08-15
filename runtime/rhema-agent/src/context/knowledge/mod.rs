// Knowledge system submodule
// This module provides unified knowledge management capabilities including
// semantic search, caching, indexing, and proactive context suggestions.

pub mod commands;
pub mod cache;
pub mod watch;
pub mod operations;

pub use commands::{KnowledgeSubcommands, CacheSubcommands, WatchSubcommands};
pub use operations::KnowledgeCommand;
