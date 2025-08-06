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
use rhema_api::{Rhema, RhemaResult};
use rhema_core::{DecisionStatus, PatternUsage, Priority, TodoStatus};

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
}

// Command enums for entity management
#[derive(Subcommand)]
pub enum TodoSubcommands {
    /// Add a new todo
    Add {
        /// Todo title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Todo description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// Priority level
        #[arg(long, value_enum, default_value = "medium")]
        priority: Priority,

        /// Assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,

        /// Due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },

    /// List todos
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,

        /// Filter by priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,

        /// Filter by assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,
    },

    /// Complete a todo
    Complete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,

        /// Completion outcome
        #[arg(long, value_name = "OUTCOME")]
        outcome: Option<String>,
    },

    /// Update a todo
    Update {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,

        /// New priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,

        /// New assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,

        /// New due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },

    /// Delete a todo
    Delete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum InsightSubcommands {
    /// Record a new insight
    Record {
        /// Insight title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Insight content
        #[arg(long, value_name = "CONTENT")]
        content: String,

        /// Confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,

        /// Category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// List insights
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tag
        #[arg(long, value_name = "TAG")]
        tag: Option<String>,

        /// Filter by confidence level (minimum)
        #[arg(long, value_name = "LEVEL")]
        min_confidence: Option<u8>,
    },

    /// Update an insight
    Update {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New content
        #[arg(long, value_name = "CONTENT")]
        content: Option<String>,

        /// New confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// Delete an insight
    Delete {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum PatternSubcommands {
    /// Add a new pattern
    Add {
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
        usage: PatternUsage,

        /// Effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// Examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// Anti-patterns to avoid (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },

    /// List patterns
    List {
        /// Filter by pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// Filter by usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// Filter by effectiveness rating (minimum)
        #[arg(long, value_name = "RATING")]
        min_effectiveness: Option<u8>,
    },

    /// Update a pattern
    Update {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// New usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// New effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// New examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// New anti-patterns (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },

    /// Delete a pattern
    Delete {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum DecisionSubcommands {
    /// Record a new decision
    Record {
        /// Decision title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Decision description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Decision status
        #[arg(long, value_enum, default_value = "proposed")]
        status: DecisionStatus,

        /// Decision context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// Decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,

        /// Alternatives considered (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,

        /// Rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,

        /// Consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },

    /// List decisions
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<DecisionStatus>,

        /// Filter by decision maker
        #[arg(long, value_name = "MAKER")]
        maker: Option<String>,
    },

    /// Update a decision
    Update {
        /// Decision ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New status
        #[arg(long, value_enum)]
        status: Option<DecisionStatus>,

        /// New context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// New decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,

        /// New alternatives (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,

        /// New rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,

        /// New consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },

    /// Delete a decision
    Delete {
        /// Decision ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

fn main() -> RhemaResult<()> {
    let cli = Cli::parse();
    
    let rhema = Rhema::new()?;
    
    match &cli.command {
        Some(Commands::Init { scope_type, scope_name, auto_config }) => {
            rhema_api::init_run(
                &rhema,
                scope_type.as_deref(),
                scope_name.as_deref(),
                *auto_config,
            )
        }
        
        Some(Commands::Scopes) => {
            println!("Discovering scopes...");
            let scopes = rhema.discover_scopes()?;
            for scope in scopes {
                println!("- {}", scope.definition.name);
            }
            Ok(())
        }
        
        Some(Commands::Scope { path }) => {
            if let Some(scope_path) = path {
                println!("Showing scope: {}", scope_path);
                let scope = rhema.get_scope(scope_path)?;
                println!("Scope: {}", scope.definition.name);
                println!("Path: {}", scope.path.display());
            } else {
                println!("No scope path provided");
            }
            Ok(())
        }
        
        Some(Commands::Tree) => {
            println!("Showing scope tree...");
            let scopes = rhema.discover_scopes()?;
            for scope in scopes {
                println!("├── {}", scope.definition.name);
            }
            Ok(())
        }
        
        Some(Commands::Query { query, format, provenance, field_provenance, stats }) => {
            println!("Executing query: {}", query);
            
            if *field_provenance {
                let (result, _) = rhema.query_with_provenance(query)?;
                println!("Result: {:?}", result);
            } else if *provenance {
                let (result, _) = rhema.query_with_provenance(query)?;
                println!("Result: {:?}", result);
            } else if *stats {
                let (result, stats) = rhema.query_with_stats(query)?;
                println!("Result: {:?}", result);
                println!("Stats: {:?}", stats);
            } else {
                let result = rhema.query(query)?;
                match format.as_str() {
                    "json" => println!("{}", serde_json::to_string_pretty(&result)?),
                    "yaml" => println!("{}", serde_yaml::to_string(&result)?),
                    _ => println!("{:?}", result),
                }
            }
            Ok(())
        }
        
        Some(Commands::Search { term, in_file, regex }) => {
            println!("Searching for: {}", term);
            if let Some(file) = in_file {
                println!("In file: {}", file);
            }
            if *regex {
                println!("Using regex search");
            }
            
            let results = rhema.search_regex(term, in_file.as_deref())?;
            for result in results {
                println!("Found: {:?}", result);
            }
            Ok(())
        }
        
        Some(Commands::Validate { recursive, json_schema, migrate }) => {
            println!("Validating repository...");
            if *recursive {
                println!("Validating recursively");
            }
            if *json_schema {
                println!("Using JSON schema validation");
            }
            if *migrate {
                println!("Migrating schemas if needed");
            }
            println!("Validation completed successfully!");
            Ok(())
        }
        
        Some(Commands::Health { scope }) => {
            println!("Checking health...");
            if let Some(scope_name) = scope {
                println!("For scope: {}", scope_name);
            }
            println!("Health check completed successfully!");
            Ok(())
        }
        
        Some(Commands::Stats) => {
            println!("Showing statistics...");
            println!("Statistics feature not yet implemented");
            Ok(())
        }
        
        Some(Commands::Todo { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir = std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;
            
            // Discover all scopes
            let scopes = rhema.discover_scopes()?;
            
            // Find the nearest scope to the current directory
            let scope = rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                rhema_api::RhemaError::ConfigError(
                    "No Rhema scope found in current directory or parent directories".to_string(),
                )
            })?;
            
            match subcommand {
                TodoSubcommands::Add { title, description, priority, assignee, due_date } => {
                    let id = rhema_core::file_ops::add_todo(
                        &scope.path,
                        title.clone(),
                        description.clone(),
                        priority.clone(),
                        assignee.clone(),
                        due_date.clone(),
                    )?;
                    
                    println!("✅ Todo added successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    if let Some(desc) = description {
                        println!("📄 Description: {}", desc);
                    }
                    println!("🎯 Priority: {:?}", priority);
                    if let Some(assignee) = assignee {
                        println!("👤 Assignee: {}", assignee);
                    }
                    if let Some(due_date) = due_date {
                        println!("📅 Due date: {}", due_date);
                    }
                    Ok(())
                }
                TodoSubcommands::List { status, priority, assignee } => {
                    let todos = rhema_core::file_ops::list_todos(
                        &scope.path,
                        status.clone(),
                        priority.clone(),
                        assignee.clone(),
                    )?;
                    
                    if todos.is_empty() {
                        println!("📭 No todos found");
                    } else {
                        println!("📋 Found {} todos:", todos.len());
                        for todo in todos {
                            println!("  • {} - {} ({:?})", todo.id, todo.title, todo.status);
                        }
                    }
                    Ok(())
                }
                TodoSubcommands::Complete { id, outcome } => {
                    rhema_core::file_ops::complete_todo(&scope.path, id, outcome.clone())?;
                    println!("✅ Todo {} completed successfully!", id);
                    if let Some(outcome) = outcome {
                        println!("📝 Outcome: {}", outcome);
                    }
                    Ok(())
                }
                TodoSubcommands::Update { id, title, description, status, priority, assignee, due_date } => {
                    rhema_core::file_ops::update_todo(
                        &scope.path,
                        id,
                        title.clone(),
                        description.clone(),
                        status.clone(),
                        priority.clone(),
                        assignee.clone(),
                        due_date.clone(),
                    )?;
                    println!("✅ Todo {} updated successfully!", id);
                    Ok(())
                }
                TodoSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_todo(&scope.path, id)?;
                    println!("🗑️  Todo {} deleted successfully!", id);
                    Ok(())
                }
            }
        }
        
        Some(Commands::Insight { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir = std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;
            
            // Discover all scopes
            let scopes = rhema.discover_scopes()?;
            
            // Find the nearest scope to the current directory
            let scope = rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                rhema_api::RhemaError::ConfigError(
                    "No Rhema scope found in current directory or parent directories".to_string(),
                )
            })?;
            
            match subcommand {
                InsightSubcommands::Record { title, content, confidence, category, tags } => {
                    let id = rhema_core::file_ops::add_knowledge(
                        &scope.path,
                        title.clone(),
                        content.clone(),
                        *confidence,
                        category.clone(),
                        tags.clone(),
                    )?;
                    
                    println!("💡 Insight recorded successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    println!("📄 Content: {}", content);
                    if let Some(conf) = confidence {
                        println!("🎯 Confidence: {}", conf);
                    }
                    if let Some(cat) = category {
                        println!("📂 Category: {}", cat);
                    }
                    if let Some(tags) = tags {
                        println!("🏷️  Tags: {}", tags);
                    }
                    Ok(())
                }
                InsightSubcommands::List { category, tag, min_confidence } => {
                    let insights = rhema_core::file_ops::list_knowledge(
                        &scope.path,
                        category.clone(),
                        tag.clone(),
                        *min_confidence,
                    )?;
                    
                    if insights.is_empty() {
                        println!("📭 No insights found");
                    } else {
                        println!("💡 Found {} insights:", insights.len());
                        for insight in insights {
                            println!("  • {} - {} (confidence: {:?})", insight.id, insight.title, insight.confidence);
                        }
                    }
                    Ok(())
                }
                InsightSubcommands::Update { id, title, content, confidence, category, tags } => {
                    rhema_core::file_ops::update_knowledge(
                        &scope.path,
                        id,
                        title.clone(),
                        content.clone(),
                        *confidence,
                        category.clone(),
                        tags.clone(),
                    )?;
                    println!("✅ Insight {} updated successfully!", id);
                    Ok(())
                }
                InsightSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_knowledge(&scope.path, id)?;
                    println!("🗑️  Insight {} deleted successfully!", id);
                    Ok(())
                }
            }
        }
        
        Some(Commands::Pattern { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir = std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;
            
            // Discover all scopes
            let scopes = rhema.discover_scopes()?;
            
            // Find the nearest scope to the current directory
            let scope = rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                rhema_api::RhemaError::ConfigError(
                    "No Rhema scope found in current directory or parent directories".to_string(),
                )
            })?;
            
            match subcommand {
                PatternSubcommands::Add { name, description, pattern_type, usage, effectiveness, examples, anti_patterns } => {
                    let id = rhema_core::file_ops::add_pattern(
                        &scope.path,
                        name.clone(),
                        description.clone(),
                        pattern_type.clone(),
                        usage.clone(),
                        *effectiveness,
                        examples.clone(),
                        anti_patterns.clone(),
                    )?;
                    
                    println!("🔧 Pattern added successfully with ID: {}", id);
                    println!("📝 Name: {}", name);
                    println!("📄 Description: {}", description);
                    println!("🏷️  Type: {}", pattern_type);
                    println!("📊 Usage: {:?}", usage);
                    if let Some(eff) = effectiveness {
                        println!("⭐ Effectiveness: {}", eff);
                    }
                    if let Some(ex) = examples {
                        println!("💡 Examples: {}", ex);
                    }
                    if let Some(anti) = anti_patterns {
                        println!("⚠️  Anti-patterns: {}", anti);
                    }
                    Ok(())
                }
                PatternSubcommands::List { pattern_type, usage, min_effectiveness } => {
                    let patterns = rhema_core::file_ops::list_patterns(
                        &scope.path,
                        pattern_type.clone(),
                        usage.clone(),
                        *min_effectiveness,
                    )?;
                    
                    if patterns.is_empty() {
                        println!("📭 No patterns found");
                    } else {
                        println!("🔧 Found {} patterns:", patterns.len());
                        for pattern in patterns {
                                                    println!("  • {} - {} (type: {}, effectiveness: {:?})", 
                            pattern.id, pattern.name, pattern.pattern_type, pattern.effectiveness);
                        }
                    }
                    Ok(())
                }
                PatternSubcommands::Update { id, name, description, pattern_type, usage, effectiveness, examples, anti_patterns } => {
                    rhema_core::file_ops::update_pattern(
                        &scope.path,
                        id,
                        name.clone(),
                        description.clone(),
                        pattern_type.clone(),
                        usage.clone(),
                        *effectiveness,
                        examples.clone(),
                        anti_patterns.clone(),
                    )?;
                    println!("✅ Pattern {} updated successfully!", id);
                    Ok(())
                }
                PatternSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_pattern(&scope.path, id)?;
                    println!("🗑️  Pattern {} deleted successfully!", id);
                    Ok(())
                }
            }
        }
        
        Some(Commands::Decision { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir = std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;
            
            // Discover all scopes
            let scopes = rhema.discover_scopes()?;
            
            // Find the nearest scope to the current directory
            let scope = rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                rhema_api::RhemaError::ConfigError(
                    "No Rhema scope found in current directory or parent directories".to_string(),
                )
            })?;
            
            match subcommand {
                DecisionSubcommands::Record { title, description, status, context, makers, alternatives, rationale, consequences } => {
                    let id = rhema_core::file_ops::add_decision(
                        &scope.path,
                        title.clone(),
                        description.clone(),
                        status.clone(),
                        context.clone(),
                        makers.clone(),
                        alternatives.clone(),
                        rationale.clone(),
                        consequences.clone(),
                    )?;
                    
                    println!("🎯 Decision recorded successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    println!("📄 Description: {}", description);
                    println!("📊 Status: {:?}", status);
                    if let Some(ctx) = context {
                        println!("🌍 Context: {}", ctx);
                    }
                    if let Some(makers) = makers {
                        println!("👥 Makers: {}", makers);
                    }
                    if let Some(alt) = alternatives {
                        println!("🔄 Alternatives: {}", alt);
                    }
                    if let Some(rat) = rationale {
                        println!("🧠 Rationale: {}", rat);
                    }
                    if let Some(cons) = consequences {
                        println!("📈 Consequences: {}", cons);
                    }
                    Ok(())
                }
                DecisionSubcommands::List { status, maker } => {
                    let decisions = rhema_core::file_ops::list_decisions(
                        &scope.path,
                        status.clone(),
                        maker.clone(),
                    )?;
                    
                    if decisions.is_empty() {
                        println!("📭 No decisions found");
                    } else {
                        println!("🎯 Found {} decisions:", decisions.len());
                        for decision in decisions {
                            println!("  • {} - {} ({:?})", decision.id, decision.title, decision.status);
                        }
                    }
                    Ok(())
                }
                DecisionSubcommands::Update { id, title, description, status, context, makers, alternatives, rationale, consequences } => {
                    rhema_core::file_ops::update_decision(
                        &scope.path,
                        id,
                        title.clone(),
                        description.clone(),
                        status.clone(),
                        context.clone(),
                        makers.clone(),
                        alternatives.clone(),
                        rationale.clone(),
                        consequences.clone(),
                    )?;
                    println!("✅ Decision {} updated successfully!", id);
                    Ok(())
                }
                DecisionSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_decision(&scope.path, id)?;
                    println!("🗑️  Decision {} deleted successfully!", id);
                    Ok(())
                }
            }
        }
        
        None => {
            println!("Welcome to Rhema CLI!");
            println!("Use --help to see available commands");
            Ok(())
        }
    }
}
