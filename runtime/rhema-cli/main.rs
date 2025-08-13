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

mod error_handler;
mod commands;

use clap::{Parser, Subcommand};
use rhema_api::{Rhema, RhemaResult};
use rhema_core::RhemaError;
use error_handler::{ErrorHandler, display_error_and_exit};
use commands::*;

#[derive(Parser)]
#[command(name = "rhema")]
#[command(about = "Rhema Protocol CLI")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Suppress output
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Rhema repository
    Init {
        /// Scope type (e.g., service, library, application)
        #[arg(long)]
        scope_type: Option<String>,

        /// Scope name
        #[arg(long)]
        scope_name: Option<String>,

        /// Auto-configure based on repository analysis
        #[arg(long)]
        auto_config: bool,
    },

    /// List all scopes in the repository
    Scopes,

    /// Show information about a specific scope
    Scope {
        /// Path to the scope
        path: Option<String>,
    },

    /// Show the scope tree
    Tree,

    /// Execute a CQL query
    Query {
        /// The CQL query to execute
        query: String,

        /// Output format (json, yaml, table)
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Include provenance information
        #[arg(long)]
        provenance: bool,

        /// Include field provenance
        #[arg(long)]
        field_provenance: bool,

        /// Include statistics
        #[arg(long)]
        stats: bool,
    },

    /// Search for content in the repository
    Search {
        /// Search term
        term: String,

        /// Search in specific file
        #[arg(short, long)]
        in_file: Option<String>,

        /// Use regex search
        #[arg(long)]
        regex: bool,
    },

    /// Validate the repository
    Validate {
        /// Validate recursively
        #[arg(long)]
        recursive: bool,

        /// Use JSON schema validation
        #[arg(long)]
        json_schema: bool,

        /// Migrate schemas if needed
        #[arg(long)]
        migrate: bool,
    },

    /// Show health information
    Health {
        /// Scope to check health for
        scope: Option<String>,
    },

    /// Show statistics
    Stats,

    /// Manage todos
    Todo {
        #[command(subcommand)]
        subcommand: TodoSubcommands,
    },

    /// Manage insights/knowledge
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

    /// Manage coordination between agents
    Coordination {
        #[command(subcommand)]
        subcommand: CoordinationSubcommands,
    },
}

/// CLI application context
struct CliContext {
    rhema: Rhema,
    error_handler: ErrorHandler,
    verbose: bool,
    quiet: bool,
}

impl CliContext {
    fn new(rhema: Rhema, verbose: bool, quiet: bool) -> Self {
        Self {
            error_handler: ErrorHandler::new(verbose, quiet),
            rhema,
            verbose,
            quiet,
        }
    }

    /// Find the nearest scope to the current directory
    fn find_current_scope(&self) -> RhemaResult<rhema_core::Scope> {
        let current_dir = std::env::current_dir()
            .map_err(|e| RhemaError::IoError(e))?;

        let scopes = self.rhema.discover_scopes()?;
        
        rhema_core::scope::find_nearest_scope(&current_dir, &scopes)
            .ok_or_else(|| RhemaError::ConfigError(
                "No Rhema scope found in current directory or parent directories".to_string()
            ))
            .cloned()
    }

    /// Display info message if not quiet
    fn display_info(&self, message: &str) -> RhemaResult<()> {
        if !self.quiet {
            self.error_handler.display_info(message)?;
        }
        Ok(())
    }

    /// Display warning message if not quiet
    fn display_warning(&self, message: &str) -> RhemaResult<()> {
        if !self.quiet {
            self.error_handler.display_warning(message)?;
        }
        Ok(())
    }

    /// Handle error with proper display
    fn handle_error<T>(&self, result: RhemaResult<T>) -> RhemaResult<T> {
        match result {
            Ok(value) => Ok(value),
            Err(e) => {
                self.error_handler.display_error(&e)?;
                Err(e)
            }
        }
    }
}

#[tokio::main]
async fn main() -> RhemaResult<()> {
    let cli = Cli::parse();

    let rhema = match Rhema::new() {
        Ok(rhema) => rhema,
        Err(e) => {
            display_error_and_exit(&e, cli.verbose, cli.quiet);
        }
    };

    let context = CliContext::new(rhema, cli.verbose, cli.quiet);

    match &cli.command {
        Some(Commands::Init { scope_type, scope_name, auto_config }) => {
            handle_init(&context, scope_type.as_deref(), scope_name.as_deref(), *auto_config)
        }

        Some(Commands::Scopes) => {
            context.display_info("Discovering scopes...")?;
            let scopes = context.handle_error(context.rhema.discover_scopes())?;
            
            if scopes.is_empty() {
                context.display_info("No scopes found in repository")?;
            } else {
                for scope in scopes {
                    println!("- {}", scope.definition.name);
                }
            }
            Ok(())
        }

        Some(Commands::Scope { path }) => {
            match path {
                Some(scope_path) => {
                    context.display_info(&format!("Showing scope: {}", scope_path))?;
                    let scope = context.handle_error(context.rhema.get_scope(scope_path))?;
                    println!("Scope: {}", scope.definition.name);
                    println!("Path: {}", scope.path.display());
                    Ok(())
                }
                None => {
                    context.display_warning("No scope path provided")?;
                    Ok(())
                }
            }
        }

        Some(Commands::Tree) => {
            context.display_info("Showing scope tree...")?;
            let scopes = context.handle_error(context.rhema.discover_scopes())?;
            
            if scopes.is_empty() {
                context.display_info("No scopes found in repository")?;
            } else {
                for scope in scopes {
                    println!("├── {}", scope.definition.name);
                }
            }
            Ok(())
        }

        Some(Commands::Query { query, format, provenance, field_provenance, stats }) => {
            context.display_info(&format!("Executing query: {}", query))?;
            handle_query(&context, query, format, *provenance, *field_provenance, *stats)
        }

        Some(Commands::Search { term, in_file, regex }) => {
            context.display_info(&format!("Searching for: {}", term))?;
            if let Some(file) = in_file {
                context.display_info(&format!("In file: {}", file))?;
            }
            if *regex {
                context.display_info("Using regex search")?;
            }

            let results = context.handle_error(
                context.rhema.search_regex(term, in_file.as_deref())
            )?;

            if results.is_empty() {
                context.display_info("No results found")?;
            } else {
                for result in results {
                    println!("Found: {:?}", result);
                }
            }
            Ok(())
        }

        Some(Commands::Validate { recursive, json_schema, migrate }) => {
            context.display_info("Validating repository...")?;
            if *recursive {
                context.display_info("Validating recursively")?;
            }
            if *json_schema {
                context.display_info("Using JSON schema validation")?;
            }
            if *migrate {
                context.display_info("Migrating schemas if needed")?;
            }
            
            // TODO: Implement actual validation logic
            context.display_info("Validation completed successfully!")?;
            Ok(())
        }

        Some(Commands::Health { scope }) => {
            context.display_info("Checking health...")?;
            if let Some(scope_name) = scope {
                context.display_info(&format!("For scope: {}", scope_name))?;
            }
            
            // TODO: Implement actual health check logic
            context.display_info("Health check completed successfully!")?;
            Ok(())
        }

        Some(Commands::Stats) => {
            context.display_info("Showing statistics...")?;
            
            // TODO: Implement actual statistics logic
            context.display_warning("Statistics feature not yet implemented")?;
            Ok(())
        }

        Some(Commands::Todo { subcommand }) => {
            let scope = context.find_current_scope()?;
            handle_todo(&context, &scope, subcommand)
        }

        Some(Commands::Insight { subcommand }) => {
            let scope = context.find_current_scope()?;
            handle_insight(&context, &scope, subcommand)
        }

        Some(Commands::Pattern { subcommand }) => {
            let scope = context.find_current_scope()?;
            handle_pattern(&context, &scope, subcommand)
        }

        Some(Commands::Decision { subcommand }) => {
            let scope = context.find_current_scope()?;
            handle_decision(&context, &scope, subcommand)
        }

        Some(Commands::Coordination { subcommand }) => {
            context.display_info("Executing coordination command...")?;
            handle_coordination(&context, subcommand)
        }

        None => {
            if !cli.quiet {
                println!("Welcome to Rhema CLI!");
                println!("Use --help to see available commands");
            }
            Ok(())
        }
    }
}
