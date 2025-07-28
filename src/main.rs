use clap::{Parser, Subcommand};
use gacp_cli::{Gacp, GacpResult, TodoSubcommands, InsightSubcommands, PatternSubcommands, DecisionSubcommands};

// Import command modules
use gacp_cli::commands;

#[derive(Parser)]
#[command(name = "gacp")]
#[command(about = "Git-Based Agent Context Protocol CLI")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Suppress output
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new GACP scope
    Init {
        /// Scope type (service, app, library, etc.)
        #[arg(long, value_name = "TYPE")]
        scope_type: Option<String>,
        
        /// Scope name
        #[arg(long, value_name = "NAME")]
        scope_name: Option<String>,
    },
    
    /// List all scopes in the repository
    Scopes,
    
    /// Show scope details
    Scope {
        /// Scope path
        #[arg(value_name = "PATH")]
        path: Option<String>,
    },
    
    /// Show scope hierarchy tree
    Tree,
    
    /// Display YAML file content
    Show {
        /// File name (without .yaml extension)
        #[arg(value_name = "FILE")]
        file: String,
        
        /// Scope path
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },
    
    /// Execute a CQL query
    Query {
        /// CQL query string
        #[arg(value_name = "QUERY")]
        query: String,
    },
    
    /// Search across context files
    Search {
        /// Search term
        #[arg(value_name = "TERM")]
        term: String,
        
        /// Search in specific file type
        #[arg(long, value_name = "FILE")]
        in_file: Option<String>,
    },
    
    /// Validate YAML files
    Validate {
        /// Validate recursively
        #[arg(long)]
        recursive: bool,
        
        /// Show JSON schemas
        #[arg(long)]
        json_schema: bool,
        
        /// Migrate schemas to latest version
        #[arg(long)]
        migrate: bool,
    },
    
    /// Migrate schema files to latest version
    Migrate {
        /// Migrate recursively
        #[arg(long)]
        recursive: bool,
        
        /// Dry run (don't modify files)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Generate schema templates
    Schema {
        /// Template type (scope, knowledge, todos, decisions, patterns, conventions, all)
        #[arg(value_name = "TYPE")]
        template_type: String,
        
        /// Output file (optional, prints to console if not specified)
        #[arg(long, value_name = "FILE")]
        output_file: Option<String>,
    },
    
    /// Check scope health
    Health {
        /// Specific scope to check
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },
    
    /// Show context statistics
    Stats,
    
    /// Manage todo items
    Todo {
        #[command(subcommand)]
        subcommand: TodoSubcommands,
    },
    
    /// Manage knowledge insights
    Insight {
        #[command(subcommand)]
        subcommand: InsightSubcommands,
    },
    
    /// Manage patterns
    Pattern {
        #[command(subcommand)]
        subcommand: PatternSubcommands,
    },
    
    /// Manage decisions
    Decision {
        #[command(subcommand)]
        subcommand: DecisionSubcommands,
    },
    
    /// Show scope dependencies
    Dependencies,
    
    /// Show impact of changes
    Impact {
        /// File to analyze
        #[arg(value_name = "FILE")]
        file: String,
    },
    
    /// Sync knowledge across scopes
    SyncKnowledge,
}



fn main() -> GacpResult<()> {
    let cli = Cli::parse();
    
    // Initialize GACP
    let gacp = Gacp::new()?;
    
    match &cli.command {
        Commands::Init { scope_type, scope_name } => {
            commands::init::run(&gacp, scope_type.as_deref(), scope_name.as_deref())
        }
        Commands::Scopes => {
            commands::scopes::run(&gacp)
        }
        Commands::Scope { path } => {
            commands::scopes::show_scope(&gacp, path.as_deref())
        }
        Commands::Tree => {
            commands::scopes::show_tree(&gacp)
        }
        Commands::Show { file, scope } => {
            commands::show::run(&gacp, file, scope.as_deref())
        }
        Commands::Query { query } => {
            commands::query::run(&gacp, query)
        }
        Commands::Search { term, in_file } => {
            commands::search::run(&gacp, term, in_file.as_deref())
        }
        Commands::Validate { recursive, json_schema, migrate } => {
            commands::validate::run(&gacp, *recursive, *json_schema, *migrate)
        }
        Commands::Migrate { recursive, dry_run } => {
            commands::migrate::run(&gacp, *recursive, *dry_run)
        }
        Commands::Schema { template_type, output_file } => {
            commands::schema::run(&gacp, template_type, output_file.as_deref())
        }
        Commands::Health { scope } => {
            commands::health::run(&gacp, scope.as_deref())
        }
        Commands::Stats => {
            commands::stats::run(&gacp)
        }
        Commands::Todo { subcommand } => {
            commands::todo::run(&gacp, subcommand)
        }
        Commands::Insight { subcommand } => {
            commands::insight::run(&gacp, subcommand)
        }
        Commands::Pattern { subcommand } => {
            commands::pattern::run(&gacp, subcommand)
        }
        Commands::Decision { subcommand } => {
            commands::decision::run(&gacp, subcommand)
        }
        Commands::Dependencies => {
            commands::dependencies::run(&gacp)
        }
        Commands::Impact { file } => {
            commands::impact::run(&gacp, file)
        }
        Commands::SyncKnowledge => {
            commands::sync::run(&gacp)
        }
    }
} 