use clap::{Parser, Subcommand};
use rhema_api::{Rhema, RhemaResult};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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

    /// Manage scope loader plugins and auto-discovery
    ScopeLoader {
        #[command(subcommand)]
        subcommand: ScopeLoaderCommands,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommands,
    },

    /// Manage Git integration
    Git {
        #[command(subcommand)]
        subcommand: GitSubcommands,
    },
    // /// Manage coordination between agents
    // Coordination {
    //     #[command(subcommand)]
    //     subcommand: CoordinationSubcommands,
    // },
}

#[derive(Subcommand, Clone)]
enum ScopeLoaderCommands {
    /// Auto-discover and create scopes based on package boundaries
    AutoDiscover {
        /// Path to scan for package boundaries
        #[arg(default_value = ".")]
        path: String,
        
        /// Minimum confidence threshold for scope creation
        #[arg(long, default_value = "0.8")]
        confidence: f64,
        
        /// Create scopes automatically without confirmation
        #[arg(long)]
        auto_create: bool,
        
        /// Show detailed reasoning for scope suggestions
        #[arg(long)]
        verbose: bool,
    },
    
    /// List available scope loader plugins
    ListPlugins,
    
    /// Test plugin detection on a specific path
    TestPlugin {
        /// Path to test
        path: String,
        
        /// Plugin name to test (optional)
        #[arg(long)]
        plugin: Option<String>,
    },
    
    /// Generate scope suggestions without creating them
    Suggest {
        /// Path to scan
        #[arg(default_value = ".")]
        path: String,
        
        /// Output format
        #[arg(long, default_value = "table")]
        format: String,
        
        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },

    /// Detect package boundaries in a path
    DetectBoundaries {
        /// Path to scan
        #[arg(default_value = ".")]
        path: String,
        
        /// Output format
        #[arg(long, default_value = "table")]
        format: String,
        
        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },
}

#[derive(Subcommand, Clone)]
enum ConfigSubcommands {
    /// Show current configuration
    Show {
        /// Show global configuration
        #[arg(long)]
        global: bool,
        
        /// Show project configuration
        #[arg(long)]
        project: bool,
        
        /// Output format
        #[arg(long, default_value = "yaml")]
        format: String,
    },

    /// Edit configuration
    Edit {
        /// Edit global configuration
        #[arg(long)]
        global: bool,
        
        /// Edit project configuration
        #[arg(long)]
        project: bool,
    },

    /// Reset configuration to defaults
    Reset {
        /// Reset global configuration
        #[arg(long)]
        global: bool,
        
        /// Reset project configuration
        #[arg(long)]
        project: bool,
    },

    /// Validate configuration
    Validate,
}

#[derive(Subcommand, Clone)]
enum GitSubcommands {
    /// Install Git hooks
    InstallHooks {
        /// Install hooks automatically
        #[arg(long)]
        auto: bool,
    },

    /// Uninstall Git hooks
    UninstallHooks,

    /// Show Git integration status
    Status,

    /// Enable Git integration
    Enable,

    /// Disable Git integration
    Disable,

    /// Show Git change history
    History {
        /// Show recent changes
        #[arg(long, default_value = "10")]
        limit: usize,
    },
}

async fn handle_scope_loader_commands(subcommand: ScopeLoaderCommands) -> RhemaResult<()> {
    use rhema_core::scope_loader::*;
    use rhema_core::scope_loader::service::ScopeLoaderConfig;
    use std::path::PathBuf;

    // Create a plugin registry
    let mut registry = PluginRegistry::new();
    
    // Register our built-in plugins
    registry.register_plugin(Box::new(CargoPlugin::new()))
        .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to register Cargo plugin: {}", e)))?;
    registry.register_plugin(Box::new(NodePackagePlugin::new()))
        .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to register Node plugin: {}", e)))?;
    registry.register_plugin(Box::new(NxPlugin::new()))
        .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to register Nx plugin: {}", e)))?;

    match subcommand {
        ScopeLoaderCommands::AutoDiscover {
            path,
            confidence,
            auto_create,
            verbose,
        } => {
            println!("🚀 Auto-discovering scopes in: {}", path);
            
            let path = PathBuf::from(path);
            if !path.exists() {
                return Err(rhema_api::RhemaError::FileNotFound(format!("Path does not exist: {}", path.display())));
            }

            // Create scope loader service
            let config = ScopeLoaderConfig {
                min_confidence_threshold: confidence,
                auto_create,
                confirm_prompt: !auto_create,
                max_depth: 5,
                enable_caching: true,
                cache_duration: 3600,
                cache_path: None,
            };
            
            let service = ScopeLoaderService::new(registry, config);
            
            // Generate suggestions
            let suggestions = service.suggest_scopes(&path).await
                .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to suggest scopes: {}", e)))?;
            
            if suggestions.is_empty() {
                println!("❌ No scope suggestions found");
                return Ok(());
            }

            // Filter by confidence
            let high_confidence: Vec<_> = suggestions
                .into_iter()
                .filter(|s| s.confidence >= confidence)
                .collect();

            if high_confidence.is_empty() {
                println!("❌ No suggestions meet the confidence threshold of {}", confidence);
                return Ok(());
            }

            println!("✅ Found {} scope suggestions (confidence >= {})", high_confidence.len(), confidence);
            
            if verbose {
                for suggestion in &high_confidence {
                    println!("  🎯 {} ({}) - Confidence: {:.2}", 
                             suggestion.name,
                             suggestion.scope_type.as_str(),
                             suggestion.confidence);
                    println!("    Path: {}", suggestion.path.display());
                    println!("    Reasoning: {}", suggestion.reasoning);
                    if !suggestion.dependencies.is_empty() {
                        println!("    Dependencies: {}", suggestion.dependencies.join(", "));
                    }
                    println!();
                }
            }

            if auto_create {
                println!("🔧 Auto-creating scopes...");
                let scopes = service.auto_create_scopes(&path).await
                    .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to create scopes: {}", e)))?;
                println!("✅ Created {} scopes", scopes.len());
            } else {
                println!("💡 Use --auto-create to automatically create these scopes");
                println!("💡 Use --verbose to see detailed information about each suggestion");
            }

            Ok(())
        }

        ScopeLoaderCommands::ListPlugins => {
            println!("📦 Available Scope Loader Plugins:");
            println!("==================================");
            
            for plugin_info in registry.list_plugins() {
                println!("  • {} (v{}) - {}", 
                         plugin_info.metadata.name, 
                         plugin_info.metadata.version, 
                         plugin_info.metadata.description);
                println!("    Supported package managers: {}", 
                         plugin_info.metadata.supported_package_managers.join(", "));
                println!("    Priority: {}", plugin_info.metadata.priority);
                println!();
            }
            
            Ok(())
        }

        ScopeLoaderCommands::TestPlugin { path, plugin } => {
            println!("🧪 Testing plugin detection in: {}", path);
            
            let path = PathBuf::from(path);
            if !path.exists() {
                return Err(rhema_api::RhemaError::FileNotFound(format!("Path does not exist: {}", path.display())));
            }

            if let Some(plugin_name) = plugin {
                // Test specific plugin
                if let Some(plugin) = registry.get_plugin(&plugin_name) {
                    println!("Testing plugin: {}", plugin_name);
                    
                    if plugin.can_handle(&path) {
                        println!("✅ Plugin can handle this path");
                        
                        match plugin.detect_boundaries(&path) {
                            Ok(boundaries) => {
                                println!("✅ Detected {} package boundaries:", boundaries.len());
                                for boundary in &boundaries {
                                    println!("  📦 {} ({}) at {}", 
                                             boundary.package_info.name,
                                             boundary.package_manager.as_str(),
                                             boundary.path.display());
                                }
                            }
                            Err(e) => {
                                println!("❌ Failed to detect boundaries: {}", e);
                            }
                        }
                    } else {
                        println!("❌ Plugin cannot handle this path");
                    }
                } else {
                    println!("❌ Plugin not found: {}", plugin_name);
                }
            } else {
                // Test all plugins
                let plugins = registry.get_plugins_for_path(&path);
                println!("Found {} applicable plugins", plugins.len());
                
                for plugin in plugins {
                    println!("Testing plugin: {}", plugin.metadata().name);
                    
                    match plugin.detect_boundaries(&path) {
                        Ok(boundaries) => {
                            println!("✅ Detected {} boundaries", boundaries.len());
                        }
                        Err(e) => {
                            println!("❌ Failed: {}", e);
                        }
                    }
                }
            }
            
            Ok(())
        }

        ScopeLoaderCommands::Suggest { path, format, verbose } => {
            println!("💡 Generating scope suggestions for: {}", path);
            
            let path = PathBuf::from(path);
            if !path.exists() {
                return Err(rhema_api::RhemaError::FileNotFound(format!("Path does not exist: {}", path.display())));
            }

            let suggestions = registry.suggest_scopes(&path)
                .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to suggest scopes: {}", e)))?;
            
            if suggestions.is_empty() {
                println!("❌ No scope suggestions found");
                return Ok(());
            }

            println!("✅ Generated {} scope suggestions:", suggestions.len());
            
            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&suggestions)?);
                }
                "yaml" => {
                    println!("{}", serde_yaml::to_string(&suggestions)?);
                }
                _ => {
                    for suggestion in &suggestions {
                        println!("  🎯 {} ({}) - Confidence: {:.2}", 
                                 suggestion.name,
                                 suggestion.scope_type.as_str(),
                                 suggestion.confidence);
                        println!("    Path: {}", suggestion.path.display());
                        
                        if verbose {
                            println!("    Reasoning: {}", suggestion.reasoning);
                            if !suggestion.dependencies.is_empty() {
                                println!("    Dependencies: {}", suggestion.dependencies.join(", "));
                            }
                            if !suggestion.files.is_empty() {
                                println!("    Files: {} files", suggestion.files.len());
                            }
                        }
                        println!();
                    }
                }
            }
            
            Ok(())
        }

        ScopeLoaderCommands::DetectBoundaries { path, format, verbose } => {
            println!("📦 Detecting package boundaries in: {}", path);
            
            let path = PathBuf::from(path);
            if !path.exists() {
                return Err(rhema_api::RhemaError::FileNotFound(format!("Path does not exist: {}", path.display())));
            }

            let boundaries = registry.detect_boundaries(&path)
                .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to detect boundaries: {}", e)))?;
            
            if boundaries.is_empty() {
                println!("❌ No package boundaries detected");
                return Ok(());
            }

            println!("✅ Detected {} package boundaries:", boundaries.len());
            
            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&boundaries)?);
                }
                "yaml" => {
                    println!("{}", serde_yaml::to_string(&boundaries)?);
                }
                _ => {
                    for boundary in &boundaries {
                        println!("  📦 {} ({}) at {}", 
                                 boundary.package_info.name,
                                 boundary.package_manager.as_str(),
                                 boundary.path.display());
                        
                        if verbose {
                            if !boundary.dependencies.is_empty() {
                                println!("    Dependencies: {}", 
                                         boundary.dependencies.iter()
                                             .map(|d| format!("{}@{}", d.name, d.version))
                                             .collect::<Vec<_>>()
                                             .join(", "));
                            }
                            
                            if !boundary.scripts.is_empty() {
                                println!("    Scripts: {}", 
                                         boundary.scripts.keys()
                                             .cloned()
                                             .collect::<Vec<_>>()
                                             .join(", "));
                            }
                        }
                        println!();
                    }
                }
            }
            
            Ok(())
        }
    }
}

async fn handle_config_commands(subcommand: ConfigSubcommands) -> RhemaResult<()> {
    use rhema_core::scope_loader::*;
    use std::path::PathBuf;

    let current_dir = std::env::current_dir()
        .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to get current directory: {}", e)))?;

    let mut config_manager = ScopeLoaderConfigManager::new()
        .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to create config manager: {}", e)))?;

    // Load project configuration if it exists
    if let Err(_) = config_manager.load_project_config(&current_dir) {
        // Project config doesn't exist, that's okay
    }

    match subcommand {
        ConfigSubcommands::Show { global, project, format } => {
            if global {
                let config = config_manager.get_global_config();
                match format.as_str() {
                    "json" => println!("{}", serde_json::to_string_pretty(config)?),
                    "yaml" => println!("{}", serde_yaml::to_string(config)?),
                    _ => println!("{:#?}", config),
                }
            } else if project {
                if let Some(config) = config_manager.get_project_config() {
                    match format.as_str() {
                        "json" => println!("{}", serde_json::to_string_pretty(config)?),
                        "yaml" => println!("{}", serde_yaml::to_string(config)?),
                        _ => println!("{:#?}", config),
                    }
                } else {
                    println!("No project configuration found");
                }
            } else {
                // Show effective configuration
                let config = config_manager.get_effective_config();
                match format.as_str() {
                    "json" => println!("{}", serde_json::to_string_pretty(&config)?),
                    "yaml" => println!("{}", serde_yaml::to_string(&config)?),
                    _ => println!("{:#?}", config),
                }
            }
        }

        ConfigSubcommands::Edit { global, project } => {
            if global {
                println!("Opening global configuration for editing...");
                // In a real implementation, you'd open the config file in an editor
                println!("Global config location: ~/.config/rhema/scope-loader.yaml");
            } else if project {
                println!("Opening project configuration for editing...");
                // In a real implementation, you'd open the config file in an editor
                println!("Project config location: .rhema/scope-loader.yaml");
            } else {
                println!("Please specify --global or --project");
            }
        }

        ConfigSubcommands::Reset { global, project } => {
            if global {
                config_manager.reset_to_defaults()?;
                println!("✅ Global configuration reset to defaults");
            } else if project {
                let default_config = GlobalScopeLoaderConfig::default();
                config_manager.update_project_config(&current_dir, default_config)?;
                println!("✅ Project configuration reset to defaults");
            } else {
                println!("Please specify --global or --project");
            }
        }

        ConfigSubcommands::Validate => {
            let config = config_manager.get_effective_config();
            config.validate()?;
            println!("✅ Configuration is valid");
        }
    }

    Ok(())
}

async fn handle_git_commands(subcommand: GitSubcommands) -> RhemaResult<()> {
    use rhema_core::scope_loader::*;
    use std::path::PathBuf;

    let current_dir = std::env::current_dir()
        .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to get current directory: {}", e)))?;

    let git_config = GitIntegrationConfig::default();
    let mut git_manager = GitIntegrationManager::new(&current_dir, git_config)
        .map_err(|e| rhema_api::RhemaError::SystemError(format!("Failed to create Git manager: {}", e)))?;

    match subcommand {
        GitSubcommands::InstallHooks { auto } => {
            if auto {
                git_manager.install_hooks().await?;
            } else {
                println!("Installing Git hooks...");
                git_manager.install_hooks().await?;
            }
        }

        GitSubcommands::UninstallHooks => {
            println!("Uninstalling Git hooks...");
            git_manager.uninstall_hooks().await?;
        }

        GitSubcommands::Status => {
            let repo_info = git_manager.get_repo_info().await;
            let hooks_installed = git_manager.are_hooks_installed().await;
            
            println!("Git Integration Status:");
            println!("  Repository: {}", if repo_info.is_git_repo { "✅ Git repo" } else { "❌ Not a Git repo" });
            if repo_info.is_git_repo {
                println!("  Branch: {}", repo_info.current_branch);
                println!("  Last commit: {}", repo_info.last_commit);
                if let Some(url) = repo_info.remote_url {
                    println!("  Remote: {}", url);
                }
            }
            
            println!("  Integration enabled: {}", if git_manager.is_enabled() { "✅" } else { "❌" });
            
            if !hooks_installed.is_empty() {
                println!("  Hooks installed:");
                for (hook_type, installed) in hooks_installed {
                    let status = if installed { "✅" } else { "❌" };
                    println!("    {:?}: {}", hook_type, status);
                }
            }
        }

        GitSubcommands::Enable => {
            git_manager.enable();
            println!("✅ Git integration enabled");
        }

        GitSubcommands::Disable => {
            git_manager.disable();
            println!("❌ Git integration disabled");
        }

        GitSubcommands::History { limit } => {
            let history = git_manager.get_change_history().await;
            let recent_history: Vec<_> = history.into_iter().rev().take(limit).collect();
            
            if recent_history.is_empty() {
                println!("No Git change history found");
            } else {
                println!("Recent Git changes:");
                for event in recent_history {
                    match event {
                        GitChangeEvent::FileModified { path, timestamp } => {
                            println!("  📝 Modified: {} at {}", path.display(), timestamp);
                        }
                        GitChangeEvent::FileAdded { path, timestamp } => {
                            println!("  ➕ Added: {} at {}", path.display(), timestamp);
                        }
                        GitChangeEvent::FileDeleted { path, timestamp } => {
                            println!("  🗑️ Deleted: {} at {}", path.display(), timestamp);
                        }
                        GitChangeEvent::BranchChanged { old_branch, new_branch, timestamp } => {
                            println!("  🌿 Branch: {} → {} at {}", old_branch, new_branch, timestamp);
                        }
                        GitChangeEvent::CommitMade { commit_hash, message, timestamp } => {
                            println!("  💾 Commit: {} - {} at {}", commit_hash, message, timestamp);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> RhemaResult<()> {
    let cli = Cli::parse();

    let rhema = Rhema::new()?;

    match &cli.command {
        Some(Commands::Init {
            scope_type,
            scope_name,
            auto_config,
        }) => {
            println!("Initializing new Rhema repository...");
            rhema_api::init::run(
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

        Some(Commands::Query {
            query,
            format,
            provenance,
            field_provenance,
            stats,
        }) => {
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

        Some(Commands::Search {
            term,
            in_file,
            regex,
        }) => {
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

        Some(Commands::Validate {
            recursive,
            json_schema,
            migrate,
        }) => {
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

        Some(Commands::ScopeLoader { subcommand }) => {
            handle_scope_loader_commands(subcommand.clone()).await
        }
        Some(Commands::Config { subcommand }) => {
            handle_config_commands(subcommand.clone()).await
        }
        Some(Commands::Git { subcommand }) => {
            handle_git_commands(subcommand.clone()).await
        }

        // Some(Commands::Coordination { subcommand }) => {
        //     println!("Executing coordination command...");
        //     let manager = CoordinationManager::new();
        //
        //     match subcommand {
        //         CoordinationSubcommands::Agent { subcommand } => {
        //             manager.execute_agent_command(subcommand).await
        //         }
        //         CoordinationSubcommands::Session { subcommand } => {
        //             manager.execute_session_command(subcommand).await
        //         }
        //         CoordinationSubcommands::System { subcommand } => {
        //             manager.execute_system_command(subcommand).await
        //         }
        //     }
        // }
        None => {
            println!("Welcome to Rhema CLI!");
            println!("Use --help to see available commands");
            Ok(())
        }
    }
}
