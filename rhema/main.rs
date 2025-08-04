/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::{Parser, Subcommand};
use rhema_cli::commands::{
    DecisionSubcommands, InsightSubcommands, PatternSubcommands, TodoSubcommands,
};
use rhema_cli::git::GitSubcommands;
use rhema_cli::performance::PerformanceSubcommands;
use rhema_cli::{Rhema, RhemaResult};

// Import command modules
use rhema_cli::context_rules::ContextRulesSubcommands;
use rhema_cli::prompt::PromptSubcommands;
use rhema_cli::template::TemplateSubcommands;
use rhema_cli::workflow::WorkflowSubcommands;
use rhema_cli::lock::LockSubcommands;
use rhema_cli::coordination::CoordinationSubcommands;

#[derive(Parser)]
#[command(name = "rhema")]
#[command(about = "Rhema Protocol CLI")]
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
    /// Initialize a new Rhema scope
    Init {
        /// Scope type (service, app, library, etc.)
        #[arg(long, value_name = "TYPE")]
        scope_type: Option<String>,

        /// Scope name
        #[arg(long, value_name = "NAME")]
        scope_name: Option<String>,

        /// Auto-detect configuration from repository structure
        #[arg(long, default_value = "false")]
        auto_config: bool,
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

        /// Include query statistics
        #[arg(long)]
        stats: bool,

        /// Output format (yaml, json, table, count)
        #[arg(long, value_name = "FORMAT", default_value = "yaml")]
        format: String,

        /// Include provenance tracking
        #[arg(long)]
        provenance: bool,

        /// Include field-level provenance
        #[arg(long)]
        field_provenance: bool,
    },

    /// Search across context files
    Search {
        /// Search term
        #[arg(value_name = "TERM")]
        term: String,

        /// Search in specific file type
        #[arg(long, value_name = "FILE")]
        in_file: Option<String>,

        /// Use regex pattern instead of simple text search
        #[arg(long)]
        regex: bool,
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

    /// Manage agent coordination and communication
    Coordination {
        #[command(subcommand)]
        subcommand: CoordinationSubcommands,
    },

    /// Manage decisions
    Decision {
        #[command(subcommand)]
        subcommand: DecisionSubcommands,
    },

    /// Show scope dependencies
    Dependencies {
        /// Analyze dependency impact
        #[arg(long)]
        impact: bool,

        /// Analyze business impact
        #[arg(long)]
        business: bool,

        /// Validate dependencies
        #[arg(long)]
        validate: bool,

        /// Check dependency health
        #[arg(long)]
        health: bool,

        /// Generate dependency report
        #[arg(long)]
        report: bool,

        /// Show critical path
        #[arg(long)]
        critical_path: bool,

        /// Recursive analysis
        #[arg(long)]
        recursive: bool,

        /// Output format (json, yaml, table, graphviz)
        #[arg(long, value_name = "FORMAT", default_value = "table")]
        format: String,

        /// Specific scope to analyze
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,
    },

    /// Show impact of changes
    Impact {
        /// File to analyze
        #[arg(value_name = "FILE")]
        file: String,
    },

    /// Sync knowledge across scopes
    SyncKnowledge,

    /// Advanced Git integration
    Git {
        #[command(subcommand)]
        subcommand: GitSubcommands,
    },

    /// Export context data
    ExportContext {
        /// Output format (json, yaml, markdown, text)
        #[arg(long, value_name = "FORMAT", default_value = "json")]
        format: String,

        /// Output file (optional, prints to console if not specified)
        #[arg(long, value_name = "FILE")]
        output_file: Option<String>,

        /// Scope filter
        #[arg(long, value_name = "SCOPE")]
        scope_filter: Option<String>,

        /// Include protocol information
        #[arg(long)]
        include_protocol: bool,

        /// Include knowledge base
        #[arg(long)]
        include_knowledge: bool,

        /// Include todo items
        #[arg(long)]
        include_todos: bool,

        /// Include decisions
        #[arg(long)]
        include_decisions: bool,

        /// Include patterns
        #[arg(long)]
        include_patterns: bool,

        /// Include conventions
        #[arg(long)]
        include_conventions: bool,

        /// Summarize data
        #[arg(long)]
        summarize: bool,

        /// AI agent format
        #[arg(long)]
        ai_agent_format: bool,
    },

    /// Generate context primer files
    Primer {
        /// Scope name
        #[arg(long, value_name = "SCOPE")]
        scope_name: Option<String>,

        /// Output directory
        #[arg(long, value_name = "DIR")]
        output_dir: Option<String>,

        /// Template type
        #[arg(long, value_name = "TEMPLATE")]
        template_type: Option<String>,

        /// Include examples
        #[arg(long)]
        include_examples: bool,

        /// Validate primer
        #[arg(long)]
        validate: bool,
    },

    /// Generate README with context
    GenerateReadme {
        /// Scope name
        #[arg(long, value_name = "SCOPE")]
        scope_name: Option<String>,

        /// Output file
        #[arg(long, value_name = "FILE")]
        output_file: Option<String>,

        /// Template
        #[arg(long, value_name = "TEMPLATE")]
        template: Option<String>,

        /// Include context
        #[arg(long)]
        include_context: bool,

        /// SEO optimized
        #[arg(long)]
        seo_optimized: bool,

        /// Custom sections (comma-separated)
        #[arg(long, value_name = "SECTIONS")]
        custom_sections: Option<String>,
    },

    /// Bootstrap context for AI agents
    BootstrapContext {
        /// Use case (code_review, feature_development, debugging, documentation, onboarding)
        #[arg(long, value_name = "USE_CASE", default_value = "code_review")]
        use_case: String,

        /// Output format (json, yaml, markdown, text, all)
        #[arg(long, value_name = "FORMAT", default_value = "json")]
        output_format: String,

        /// Output directory
        #[arg(long, value_name = "DIR")]
        output_dir: Option<String>,

        /// Scope filter
        #[arg(long, value_name = "SCOPE")]
        scope_filter: Option<String>,

        /// Include all data
        #[arg(long)]
        include_all: bool,

        /// Optimize for AI
        #[arg(long)]
        optimize_for_ai: bool,

        /// Create primer
        #[arg(long)]
        create_primer: bool,

        /// Create README
        #[arg(long)]
        create_readme: bool,
    },

    /// Manage MCP daemon service
    Daemon {
        #[command(flatten)]
        args: rhema_cli::daemon::DaemonArgs,
    },

    /// Performance monitoring and analytics
    Performance {
        #[command(subcommand)]
        subcommand: PerformanceSubcommands,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        subcommand: rhema_cli::config::ConfigSubcommands,
    },

    /// Manage prompt patterns
    Prompt {
        #[command(subcommand)]
        subcommand: PromptSubcommands,
    },

    /// Manage context injection rules
    ContextRules {
        #[command(subcommand)]
        subcommand: ContextRulesSubcommands,
    },

    /// Manage prompt chain workflows
    Workflow {
        #[command(subcommand)]
        subcommand: WorkflowSubcommands,
    },

    /// Manage and share prompt templates
    Template {
        #[command(subcommand)]
        subcommand: TemplateSubcommands,
    },

    /// Manage lock files
    Lock {
        #[command(subcommand)]
        subcommand: LockSubcommands,
    },

    /// Start interactive mode
    Interactive {
        /// Configuration file for interactive mode
        #[arg(long, value_name = "CONFIG")]
        config: Option<String>,

        /// Disable auto-completion
        #[arg(long)]
        no_auto_complete: bool,

        /// Disable syntax highlighting
        #[arg(long)]
        no_syntax_highlighting: bool,

        /// Disable context-aware features
        #[arg(long)]
        no_context_aware: bool,
    },
}

fn main() -> RhemaResult<()> {
    let cli = Cli::parse();

    // Initialize Rhema
    let rhema = Rhema::new()?;

    match cli.command {
        Commands::Init {
            scope_type,
            scope_name,
            auto_config,
        } => rhema_cli::init::run(
            &rhema,
            scope_type.as_deref(),
            scope_name.as_deref(),
            auto_config,
        ),
        Commands::Scopes => rhema_cli::scopes::run(&rhema),
        Commands::Scope { path } => rhema_cli::scopes::show_scope(&rhema, path.as_deref()),
        Commands::Tree => rhema_cli::scopes::show_tree(&rhema),
        Commands::Show { file, scope } => rhema_cli::show::run(&rhema, &file, scope.as_deref()),
        Commands::Query {
            query,
            stats,
            format,
            provenance,
            field_provenance,
        } => {
            if field_provenance {
                rhema_cli::query::run_with_field_provenance(&rhema, &query)
            } else if provenance {
                rhema_cli::query::run_with_provenance(&rhema, &query)
            } else if stats {
                rhema_cli::query::run_with_stats(&rhema, &query)
            } else if format != "yaml" {
                rhema_cli::query::run_formatted(&rhema, &query, format.as_str())
            } else {
                rhema_cli::query::run(&rhema, &query)
            }
        }
        Commands::Search {
            term,
            in_file,
            regex,
        } => rhema_cli::search::run(&rhema, &term, in_file.as_deref(), regex),
        Commands::Validate {
            recursive,
            json_schema,
            migrate,
        } => rhema_cli::validate::run(&rhema, recursive, json_schema, migrate, false, false, false),
        Commands::Migrate { recursive, dry_run } => {
            rhema_cli::migrate::run(&rhema, recursive, dry_run)
        }
        Commands::Schema {
            template_type,
            output_file,
        } => rhema_cli::schema::run(&rhema, &template_type, output_file.as_deref()),
        Commands::Health { scope } => rhema_cli::health::run(&rhema, scope.as_deref()),
        Commands::Stats => rhema_cli::stats::run(&rhema),
        Commands::Todo { subcommand } => rhema_cli::todo::run(&rhema, &subcommand),
        Commands::Insight { subcommand } => rhema_cli::insight::run(&rhema, &subcommand),
        Commands::Pattern { subcommand } => rhema_cli::pattern::run(&rhema, &subcommand),
        Commands::Coordination { subcommand } => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(rhema_cli::coordination::run(&rhema, &subcommand))
        },
        Commands::Decision { subcommand } => rhema_cli::decision::run(&rhema, &subcommand),
        Commands::Dependencies {
            impact,
            business,
            validate,
            health,
            report,
            critical_path,
            recursive,
            format,
            scope,
        } => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(rhema_cli::dependencies::run(
                &rhema,
                impact,
                business,
                validate,
                health,
                report,
                critical_path,
                recursive,
                format.as_str(),
                scope.as_deref(),
            ))
        },
        Commands::Impact { file } => rhema_cli::impact::run(&rhema, &file),
        Commands::SyncKnowledge => rhema_cli::sync::run(&rhema),
        Commands::Git { subcommand } => rhema_cli::git::run(&rhema, &subcommand),
        Commands::ExportContext {
            format,
            output_file,
            scope_filter,
            include_protocol,
            include_knowledge,
            include_todos,
            include_decisions,
            include_patterns,
            include_conventions,
            summarize,
            ai_agent_format,
        } => rhema_cli::export_context::run(
            &rhema,
            &format,
            output_file.as_deref(),
            scope_filter.as_deref(),
            include_protocol,
            include_knowledge,
            include_todos,
            include_decisions,
            include_patterns,
            include_conventions,
            summarize,
            ai_agent_format,
        ),
        Commands::Primer {
            scope_name,
            output_dir,
            template_type,
            include_examples,
            validate,
        } => rhema_cli::primer::run(
            &rhema,
            scope_name.as_deref(),
            output_dir.as_deref(),
            template_type.as_deref(),
            include_examples,
            validate,
        ),
        Commands::GenerateReadme {
            scope_name,
            output_file,
            template,
            include_context,
            seo_optimized,
            custom_sections,
        } => {
            let custom_sections_vec = custom_sections
                .as_ref()
                .map(|s| s.split(',').map(|s| s.trim().to_string()).collect());
            rhema_cli::generate_readme::run(
                &rhema,
                scope_name.as_deref(),
                output_file.as_deref(),
                template.as_deref(),
                include_context,
                seo_optimized,
                custom_sections_vec,
            )
        }
        Commands::BootstrapContext {
            use_case,
            output_format,
            output_dir,
            scope_filter,
            include_all,
            optimize_for_ai,
            create_primer,
            create_readme,
        } => rhema_cli::bootstrap_context::run(
            &rhema,
            &use_case,
            &output_format,
            output_dir.as_deref(),
            scope_filter.as_deref(),
            include_all,
            optimize_for_ai,
            create_primer,
            create_readme,
        ),
        Commands::Daemon { args } => {
            tokio::runtime::Runtime::new()?.block_on(rhema_cli::daemon::execute_daemon(args))
        }
        Commands::Performance { subcommand } => tokio::runtime::Runtime::new()?.block_on(
            rhema_cli::performance::run_performance_command(&rhema, &subcommand),
        ),
        Commands::Config { subcommand } => rhema_cli::config::run(&rhema, &subcommand),
        Commands::Prompt { subcommand } => rhema_cli::prompt::run(&rhema, &subcommand),
        Commands::ContextRules { subcommand } => rhema_cli::context_rules::run(&rhema, &subcommand),
        Commands::Workflow { subcommand } => rhema_cli::workflow::run(&rhema, &subcommand),
        Commands::Template { subcommand } => rhema_cli::template::run(&rhema, &subcommand),
        Commands::Lock { subcommand } => subcommand.execute(&rhema),
        Commands::Interactive {
            config,
            no_auto_complete,
            no_syntax_highlighting,
            no_context_aware,
        } => rhema_cli::interactive::run_interactive_with_config(
            rhema,
            config.as_deref(),
            no_auto_complete,
            no_syntax_highlighting,
            no_context_aware,
        ),
    }
}
