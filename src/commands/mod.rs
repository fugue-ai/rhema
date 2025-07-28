pub mod init;
pub mod scopes;
pub mod show;
pub mod query;
pub mod search;
pub mod validate;
pub mod migrate;
pub mod schema;
pub mod health;
pub mod stats;
pub mod todo;
pub mod insight;
pub mod pattern;
pub mod decision;
pub mod dependencies;
pub mod impact;
pub mod sync;

use clap::Args;

/// Common arguments for commands that need a scope
#[derive(Args)]
pub struct ScopeArgs {
    /// Scope path (relative to repository root)
    #[arg(value_name = "SCOPE")]
    scope: Option<String>,
}

/// Common arguments for commands that need a file
#[derive(Args)]
pub struct FileArgs {
    /// File name (without .yaml extension)
    #[arg(value_name = "FILE")]
    file: String,
}

/// Common arguments for commands that need a query
#[derive(Args)]
pub struct QueryArgs {
    /// CQL query string
    #[arg(value_name = "QUERY")]
    query: String,
}

/// Common arguments for search commands
#[derive(Args)]
pub struct SearchArgs {
    /// Search term
    #[arg(value_name = "TERM")]
    term: String,
    
    /// Search in specific file type
    #[arg(long, value_name = "FILE")]
    in_file: Option<String>,
}

/// Common arguments for todo commands
#[derive(Args)]
pub struct TodoArgs {
    /// Todo title
    #[arg(value_name = "TITLE")]
    title: String,
    
    /// Priority level
    #[arg(long, value_enum, default_value = "medium")]
    priority: crate::schema::Priority,
    
    /// Assignee
    #[arg(long, value_name = "ASSIGNEE")]
    assignee: Option<String>,
    
    /// Due date (ISO format)
    #[arg(long, value_name = "DATE")]
    due_date: Option<String>,
}

/// Common arguments for insight commands
#[derive(Args)]
pub struct InsightArgs {
    /// Insight content
    #[arg(value_name = "INSIGHT")]
    insight: String,
    
    /// Confidence level (1-10)
    #[arg(long, value_name = "LEVEL")]
    confidence: Option<u8>,
    
    /// Category
    #[arg(long, value_name = "CATEGORY")]
    category: Option<String>,
    
    /// Tags (comma-separated)
    #[arg(long, value_name = "TAGS")]
    tags: Option<String>,
}

/// Common arguments for pattern commands
#[derive(Args)]
pub struct PatternArgs {
    /// Pattern name
    #[arg(value_name = "NAME")]
    name: String,
    
    /// Pattern description
    #[arg(long, value_name = "DESCRIPTION")]
    description: String,
    
    /// Pattern type
    #[arg(long, value_name = "TYPE")]
    pattern_type: String,
    
    /// Usage context
    #[arg(long, value_enum, default_value = "recommended")]
    usage: crate::schema::PatternUsage,
    
    /// Effectiveness rating (1-10)
    #[arg(long, value_name = "RATING")]
    effectiveness: Option<u8>,
}

/// Common arguments for decision commands
#[derive(Args)]
pub struct DecisionArgs {
    /// Decision title
    #[arg(value_name = "TITLE")]
    title: String,
    
    /// Decision description
    #[arg(long, value_name = "DESCRIPTION")]
    description: String,
    
    /// Decision status
    #[arg(long, value_enum, default_value = "proposed")]
    status: crate::schema::DecisionStatus,
    
    /// Decision context
    #[arg(long, value_name = "CONTEXT")]
    context: Option<String>,
    
    /// Decision makers (comma-separated)
    #[arg(long, value_name = "MAKERS")]
    makers: Option<String>,
} 