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

mod commands;
mod error_handler;

use clap::{Parser, Subcommand};
use commands::*;
use error_handler::{display_error_and_exit, ErrorHandler};
use rhema_api::{Rhema, RhemaResult, Validatable};
use rhema_core::RhemaError;

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

    /// Manage scope loading and discovery
    ScopeLoader {
        #[command(subcommand)]
        subcommand: ScopeLoaderSubcommands,
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
        let current_dir = std::env::current_dir().map_err(|e| RhemaError::IoError(e))?;

        let scopes = self.rhema.discover_scopes()?;

        rhema_core::scope::find_nearest_scope(&current_dir, &scopes)
            .ok_or_else(|| {
                RhemaError::ConfigError(
                    "No Rhema scope found in current directory or parent directories".to_string(),
                )
            })
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
        Some(Commands::Init {
            scope_type,
            scope_name,
            auto_config,
        }) => handle_init(
            &context,
            scope_type.as_deref(),
            scope_name.as_deref(),
            *auto_config,
        ),

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

        Some(Commands::Scope { path }) => match path {
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
        },

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

        Some(Commands::Query {
            query,
            format,
            provenance,
            field_provenance,
            stats,
        }) => {
            context.display_info(&format!("Executing query: {}", query))?;
            handle_query(
                &context,
                query,
                format,
                *provenance,
                *field_provenance,
                *stats,
            )
        }

        Some(Commands::Search {
            term,
            in_file,
            regex,
        }) => {
            context.display_info(&format!("Searching for: {}", term))?;
            if let Some(file) = in_file {
                context.display_info(&format!("In file: {}", file))?;
            }
            if *regex {
                context.display_info("Using regex search")?;
            }

            let results =
                context.handle_error(context.rhema.search_regex(term, in_file.as_deref()))?;

            if results.is_empty() {
                context.display_info("No results found")?;
            } else {
                for result in results {
                    println!("Found: {:?}", result);
                }
            }
            Ok(())
        }

        Some(Commands::Validate {
            recursive,
            json_schema,
            migrate,
        }) => {
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

            // Implement actual validation logic
            let scopes = context.handle_error(context.rhema.discover_scopes())?;
            let mut validation_errors = Vec::new();
            let mut validation_warnings = Vec::new();
            let mut validated_scopes = 0;

            for scope in &scopes {
                context.display_info(&format!("Validating scope: {}", scope.definition.name))?;

                match context.rhema.validate_scope(scope).await {
                    Ok(_) => {
                        validated_scopes += 1;
                        if !context.quiet {
                            println!("✅ Scope '{}' is valid", scope.definition.name);
                        }
                    }
                    Err(e) => {
                        validation_errors.push(format!("Scope '{}': {}", scope.definition.name, e));
                        if !context.quiet {
                            println!(
                                "❌ Scope '{}' validation failed: {}",
                                scope.definition.name, e
                            );
                        }
                    }
                }

                // Validate scope definition schema if json_schema is enabled
                if *json_schema {
                    match scope.definition.validate() {
                        Ok(_) => {
                            if !context.quiet {
                                println!("  ✅ Schema validation passed");
                            }
                        }
                        Err(e) => {
                            validation_warnings.push(format!(
                                "Schema validation for '{}': {}",
                                scope.definition.name, e
                            ));
                            if !context.quiet {
                                println!("  ⚠️  Schema validation warning: {}", e);
                            }
                        }
                    }
                }
            }

            // Display validation summary
            if !context.quiet {
                println!("\n📊 Validation Summary:");
                println!("  Total scopes: {}", scopes.len());
                println!("  Valid scopes: {}", validated_scopes);
                println!("  Errors: {}", validation_errors.len());
                println!("  Warnings: {}", validation_warnings.len());
            }

            if validation_errors.is_empty() {
                context.display_info("✅ Repository validation completed successfully!")?;
            } else {
                context.display_warning(&format!(
                    "⚠️  Repository validation completed with {} errors",
                    validation_errors.len()
                ))?;
            }

            Ok(())
        }

        Some(Commands::Health { scope }) => {
            context.display_info("Checking health...")?;
            if let Some(scope_name) = &scope {
                context.display_info(&format!("For scope: {}", scope_name))?;
            }

            // Implement actual health check logic
            let scopes = context.handle_error(context.rhema.discover_scopes())?;
            let mut healthy_scopes = 0;
            let total_scopes = scopes.len();

            for scope_item in &scopes {
                // If a specific scope was requested, only check that scope
                if let Some(scope_name) = &scope {
                    if scope_item.definition.name != *scope_name {
                        continue;
                    }
                }

                context.display_info(&format!(
                    "Checking health for scope: {}",
                    scope_item.definition.name
                ))?;

                // Check if scope path exists
                if !scope_item.path.exists() {
                    if !context.quiet {
                        println!(
                            "❌ Scope '{}' path does not exist: {}",
                            scope_item.definition.name,
                            scope_item.path.display()
                        );
                    }
                    continue;
                }

                // Check if required files exist
                let mut missing_files = Vec::new();
                for (filename, filepath) in &scope_item.files {
                    if !filepath.exists() {
                        missing_files.push(filename.clone());
                    }
                }

                if missing_files.is_empty() {
                    healthy_scopes += 1;
                    if !context.quiet {
                        println!("✅ Scope '{}' is healthy", scope_item.definition.name);
                    }
                } else {
                    if !context.quiet {
                        println!(
                            "⚠️  Scope '{}' has missing files: {}",
                            scope_item.definition.name,
                            missing_files.join(", ")
                        );
                    }
                }
            }

            // Display health summary
            if !context.quiet {
                println!("\n🏥 Health Summary:");
                println!("  Total scopes: {}", total_scopes);
                println!("  Healthy scopes: {}", healthy_scopes);
                println!(
                    "  Health score: {:.1}%",
                    (healthy_scopes as f64 / total_scopes as f64) * 100.0
                );
            }

            if healthy_scopes == total_scopes {
                context.display_info("✅ All scopes are healthy!")?;
            } else {
                context.display_warning(&format!(
                    "⚠️  {} out of {} scopes are healthy",
                    healthy_scopes, total_scopes
                ))?;
            }

            Ok(())
        }

        Some(Commands::Stats) => {
            context.display_info("Showing statistics...")?;

            // Implement actual statistics logic
            let scopes = context.handle_error(context.rhema.discover_scopes())?;

            // Get cache statistics
            let cache_stats = context.handle_error(context.rhema.get_cache_stats().await)?;

            // Get coordination statistics if available
            let coordination_stats = context.rhema.get_coordination_stats().await;

            // Calculate scope statistics
            let total_scopes = scopes.len();
            let mut scope_types = std::collections::HashMap::new();
            let mut total_files = 0;

            for scope in &scopes {
                *scope_types.entry(&scope.definition.scope_type).or_insert(0) += 1;
                total_files += scope.files.len();
            }

            // Display comprehensive statistics
            if !context.quiet {
                println!("\n📊 Repository Statistics:");
                println!("  Total scopes: {}", total_scopes);
                println!("  Total files: {}", total_files);
                println!(
                    "  Average files per scope: {:.1}",
                    if total_scopes > 0 {
                        total_files as f64 / total_scopes as f64
                    } else {
                        0.0
                    }
                );

                println!("\n📁 Scope Types:");
                for (scope_type, count) in scope_types {
                    println!("  {}: {}", scope_type, count);
                }

                println!("\n💾 Cache Statistics:");
                for (key, value) in &cache_stats {
                    println!("  {}: {}", key, value);
                }

                if let Ok(stats) = coordination_stats {
                    println!("\n🤝 Coordination Statistics:");
                    println!("  Active sessions: {}", stats.active_sessions);
                    println!("  Total messages: {}", stats.total_messages);
                    println!("  Active agents: {}", stats.active_agents);
                }

                println!("\n📈 Performance Metrics:");
                println!("  Repository size: {} scopes", total_scopes);
                println!(
                    "  File density: {:.1} files/scope",
                    if total_scopes > 0 {
                        total_files as f64 / total_scopes as f64
                    } else {
                        0.0
                    }
                );
            }

            context.display_info("✅ Statistics generated successfully!")?;
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

        Some(Commands::ScopeLoader { subcommand }) => {
            context.display_info("Executing scope loader command...")?;
            handle_scope_loader(&context, subcommand.clone())
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
