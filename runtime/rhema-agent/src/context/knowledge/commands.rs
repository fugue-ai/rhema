use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum KnowledgeSubcommands {
    /// Initialize the unified knowledge system
    Init {
        /// Configuration file path
        #[arg(long, value_name = "CONFIG_FILE")]
        config_file: Option<PathBuf>,
        
        /// Cache directory path
        #[arg(long, value_name = "CACHE_DIR")]
        cache_dir: Option<PathBuf>,
    },
    
    /// Search knowledge using semantic and hybrid search
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,
        
        /// Use semantic search only
        #[arg(long)]
        semantic: bool,
        
        /// Use hybrid search (semantic + keyword)
        #[arg(long)]
        hybrid: bool,
        
        /// Search only cached content
        #[arg(long)]
        cache_only: bool,
        
        /// Search only indexed content
        #[arg(long)]
        index_only: bool,
        
        /// Maximum number of results
        #[arg(long, value_name = "LIMIT", default_value = "10")]
        limit: usize,
        
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// Cache management operations
    Cache {
        #[command(subcommand)]
        subcommand: CacheSubcommands,
    },
    
    /// Semantic indexing operations
    Index {
        /// Scope path to index
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
        
        /// Index all scopes
        #[arg(long)]
        all_scopes: bool,
        
        /// Specific file to index
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,
        
        /// Force reindex all data
        #[arg(long)]
        force: bool,
    },
    
    /// Proactive context suggestions
    Suggest {
        /// File to get suggestions for
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,
        
        /// Workflow to get suggestions for
        #[arg(long, value_name = "WORKFLOW")]
        workflow: Option<String>,
        
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// Cache warming operations
    Warm {
        /// Patterns file for warming
        #[arg(long, value_name = "PATTERNS")]
        patterns: Option<PathBuf>,
        
        /// Workflow to warm cache for
        #[arg(long, value_name = "WORKFLOW")]
        workflow: Option<String>,
        
        /// Agent ID to warm cache for
        #[arg(long, value_name = "AGENT_ID")]
        agent: Option<String>,
    },
    
    /// Share context between agents
    Share {
        /// Source agent ID
        #[arg(long, value_name = "FROM")]
        from: String,
        
        /// Target agent ID
        #[arg(long, value_name = "TO")]
        to: String,
        
        /// Context key to share
        #[arg(long, value_name = "CONTEXT")]
        context: String,
    },
    
    /// Knowledge synthesis operations
    Synthesize {
        /// Topic to synthesize knowledge for
        #[arg(long, value_name = "TOPIC")]
        topic: String,
        
        /// Scope path for synthesis
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
        
        /// Enable cross-scope synthesis
        #[arg(long)]
        cross_scope: bool,
        
        /// Output format (json, yaml, markdown)
        #[arg(long, value_name = "FORMAT", default_value = "markdown")]
        format: String,
    },
    
    /// System status and metrics
    Status {
        /// Show detailed metrics
        #[arg(long)]
        detailed: bool,
        
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// System optimization
    Optimize {
        /// Optimize cache only
        #[arg(long)]
        cache_only: bool,
        
        /// Optimize index only
        #[arg(long)]
        index_only: bool,
        
        /// Show optimization plan without executing
        #[arg(long)]
        dry_run: bool,
    },
    
    /// System metrics
    Metrics {
        /// Show cache metrics
        #[arg(long)]
        cache: bool,
        
        /// Show search metrics
        #[arg(long)]
        search: bool,
        
        /// Show synthesis metrics
        #[arg(long)]
        synthesis: bool,
        
        /// Show proactive metrics
        #[arg(long)]
        proactive: bool,
        
        /// Show performance metrics
        #[arg(long)]
        performance: bool,
        
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// System cleanup
    Cleanup {
        /// Clean up expired entries only
        #[arg(long)]
        expired_only: bool,
        
        /// Show cleanup plan without executing
        #[arg(long)]
        dry_run: bool,
        
        /// Force cleanup without confirmation
        #[arg(long)]
        force: bool,
    },
    
    /// File watching operations
    Watch {
        #[command(subcommand)]
        subcommand: WatchSubcommands,
    },
}

#[derive(Subcommand)]
pub enum CacheSubcommands {
    /// Get cached data
    Get {
        /// Cache key
        #[arg(value_name = "KEY")]
        key: String,
        
        /// Use semantic search
        #[arg(long)]
        semantic_search: bool,
        
        /// Semantic query
        #[arg(long, value_name = "QUERY")]
        query: Option<String>,
    },
    
    /// Set cached data
    Set {
        /// Cache key
        #[arg(value_name = "KEY")]
        key: String,
        
        /// Data value
        #[arg(value_name = "VALUE")]
        value: String,
        
        /// Enable semantic indexing
        #[arg(long)]
        index_semantic: bool,
        
        /// Time to live in seconds
        #[arg(long, value_name = "TTL")]
        ttl: Option<u64>,
    },
    
    /// Delete cached data
    Delete {
        /// Cache key
        #[arg(value_name = "KEY")]
        key: String,
        
        /// Remove from semantic index
        #[arg(long)]
        remove_from_index: bool,
    },
    
    /// List cache entries
    List {
        /// Filter by pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: Option<String>,
        
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
        
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// Clear all cache
    Clear {
        /// Clear memory cache only
        #[arg(long)]
        memory_only: bool,
        
        /// Clear disk cache only
        #[arg(long)]
        disk_only: bool,
        
        /// Force clear without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum WatchSubcommands {
    /// Watch a specific file
    File {
        /// File path to watch
        #[arg(value_name = "FILE_PATH")]
        file_path: PathBuf,
        
        /// Enable semantic indexing for watched file
        #[arg(long)]
        index: bool,
    },
    
    /// Watch a directory recursively
    Directory {
        /// Directory path to watch
        #[arg(value_name = "DIR_PATH")]
        dir_path: PathBuf,
        
        /// File patterns to watch (e.g., "*.rs", "*.md")
        #[arg(long, value_name = "PATTERNS")]
        patterns: Vec<String>,
        
        /// File patterns to ignore (e.g., "target/", ".git/")
        #[arg(long, value_name = "IGNORE_PATTERNS")]
        ignore_patterns: Vec<String>,
    },
    
    /// List watched files
    List {
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
        
        /// Filter by pattern
        #[arg(long, value_name = "PATTERN")]
        pattern: Option<String>,
        
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// Get file watch statistics
    Stats {
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// Get changed files
    Changed {
        /// Output format (json, yaml, table)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,
    },
    
    /// Stop watching a file
    Unwatch {
        /// File path to stop watching
        #[arg(value_name = "FILE_PATH")]
        file_path: PathBuf,
    },
    
    /// Stop watching a directory
    UnwatchDir {
        /// Directory path to stop watching
        #[arg(value_name = "DIR_PATH")]
        dir_path: PathBuf,
    },
}
