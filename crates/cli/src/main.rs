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

use rhema_cli::git::GitSubcommands;
use rhema_cli::lock::LockSubcommands;
use rhema_cli::performance::PerformanceSubcommands;
use rhema_cli::commands::{
    ContextRulesSubcommands, DecisionSubcommands, InsightSubcommands, PatternSubcommands,
    PromptSubcommands, TemplateSubcommands, TodoSubcommands, WorkflowSubcommands,
};
use rhema_cli::{Rhema, RhemaResult};
use clap::{Parser, Subcommand};

// Import all the modules we need
use rhema_cli::{
    batch, bootstrap_context, config, context_rules, daemon, decision, dependencies,
    export_context, generate_readme, git, health, impact, init, insight, lock, migrate,
    pattern, performance, primer, prompt, query, schema, scopes, search, show, stats,
    sync, template, todo, validate, workflow, interactive, commands,
};

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

        /// Validate against lock file
        #[arg(long)]
        lock_file: bool,

        /// Validate lock file only (skip other validations)
        #[arg(long)]
        lock_only: bool,

        /// Strict lock file validation (treat warnings as errors)
        #[arg(long)]
        strict: bool,
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
    Dependencies {
        /// Analyze from lock file instead of current state
        #[arg(long)]
        lock_file: bool,

        /// Compare lock file with current state
        #[arg(long)]
        compare: bool,

        /// Show dependency chain visualization
        #[arg(long)]
        visualize: bool,

        /// Detect version conflicts
        #[arg(long)]
        conflicts: bool,

        /// Show detailed impact analysis
        #[arg(long)]
        impact: bool,

        /// Output format (text, json, yaml)
        #[arg(long, value_name = "FORMAT", default_value = "text")]
        format: String,
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

    /// Manage lock files
    Lock {
        #[command(subcommand)]
        subcommand: LockSubcommands,
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
        args: daemon::DaemonArgs,
    },

    /// Performance monitoring and analytics
    Performance {
        #[command(subcommand)]
        subcommand: PerformanceSubcommands,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        subcommand: config::ConfigSubcommands,
    },

    /// Manage prompt patterns
    Prompt {
        #[command(subcommand)]
        subcommand: prompt::PromptSubcommands,
    },

    /// Manage context injection rules
    ContextRules {
        #[command(subcommand)]
        subcommand: context_rules::ContextRulesSubcommands,
    },

    /// Manage prompt chain workflows
    Workflow {
        #[command(subcommand)]
        subcommand: workflow::WorkflowSubcommands,
    },

    /// Manage and share prompt templates
    Template {
        #[command(subcommand)]
        subcommand: template::TemplateSubcommands,
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
        } => init::run(
            &rhema,
            scope_type.as_deref(),
            scope_name.as_deref(),
            auto_config,
        ),
        Commands::Scopes => scopes::run(&rhema),
        Commands::Scope { path } => scopes::show_scope(&rhema, path.as_deref()),
        Commands::Tree => scopes::show_tree(&rhema),
        Commands::Show { file, scope } => show::run(&rhema, &file, scope.as_deref()),
        Commands::Query {
            query,
            stats,
            format,
            provenance,
            field_provenance,
        } => {
            if field_provenance {
                query::run_with_field_provenance(&rhema, &query)
            } else if provenance {
                query::run_with_provenance(&rhema, &query)
            } else if stats {
                query::run_with_stats(&rhema, &query)
            } else if format != "yaml" {
                query::run_formatted(&rhema, &query, format.as_str())
            } else {
                query::run(&rhema, &query)
            }
        }
        Commands::Search {
            term,
            in_file,
            regex,
        } => search::run(&rhema, &term, in_file.as_deref(), regex),
        Commands::Validate {
            recursive,
            json_schema,
            migrate,
            lock_file,
            lock_only,
            strict,
        } => validate::run(&rhema, recursive, json_schema, migrate, lock_file, lock_only, strict),
        Commands::Migrate { recursive, dry_run } => migrate::run(&rhema, recursive, dry_run),
        Commands::Schema {
            template_type,
            output_file,
        } => schema::run(&rhema, &template_type, output_file.as_deref()),
        Commands::Health { scope } => health::run(&rhema, scope.as_deref()),
        Commands::Stats => stats::run(&rhema),
        Commands::Todo { subcommand } => todo::run(&rhema, &subcommand),
        Commands::Insight { subcommand } => insight::run(&rhema, &subcommand),
        Commands::Pattern { subcommand } => pattern::run(&rhema, &subcommand),
        Commands::Decision { subcommand } => decision::run(&rhema, &subcommand),
        Commands::Dependencies {
            lock_file,
            compare,
            visualize,
            conflicts,
            impact,
            format,
        } => dependencies::run(&rhema, lock_file, compare, visualize, conflicts, impact, &format),
        Commands::Impact { file } => impact::run(&rhema, &file),
        Commands::SyncKnowledge => sync::run(&rhema),
        Commands::Git { subcommand } => git::run(&rhema, &subcommand),
        Commands::Lock { subcommand } => subcommand.execute(&rhema),
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
        } => export_context::run(
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
        } => primer::run(
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
            generate_readme::run(
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
        } => bootstrap_context::run(
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
            tokio::runtime::Runtime::new()?.block_on(daemon::execute_daemon(args))
        }
        Commands::Performance { subcommand } => tokio::runtime::Runtime::new()?.block_on(
            performance::run_performance_command(&rhema, &subcommand),
        ),
        Commands::Config { subcommand } => config::run(&rhema, &subcommand),
        Commands::Prompt { subcommand } => prompt::run(&rhema, &subcommand),
        Commands::ContextRules { subcommand } => context_rules::run(&rhema, &subcommand),
        Commands::Workflow { subcommand } => workflow::run(&rhema, &subcommand),
        Commands::Template { subcommand } => template::run(&rhema, &subcommand),
        Commands::Interactive {
            config,
            no_auto_complete,
            no_syntax_highlighting,
            no_context_aware,
        } => interactive::run_interactive_with_config(
            rhema,
            config.as_deref(),
            no_auto_complete,
            no_syntax_highlighting,
            no_context_aware,
        ),
    }
}
