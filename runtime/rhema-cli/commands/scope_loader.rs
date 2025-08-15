/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::CliContext;
use clap::{Subcommand, ValueEnum};
use rhema_api::RhemaResult;
use rhema_core::scope_loader::service::ScopeLoaderConfig;
use rhema_core::scope_loader::*;
use std::path::PathBuf;

#[derive(Debug, Subcommand, Clone)]
pub enum ScopeLoaderSubcommands {
    /// Discover package boundaries in the current directory
    Discover {
        /// Path to analyze (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Show detailed information about each boundary
        #[arg(short, long)]
        verbose: bool,

        /// Output format
        #[arg(short, long, default_value = "table")]
        format: OutputFormat,
    },

    /// Generate scope suggestions based on discovered boundaries
    Suggest {
        /// Path to analyze (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Minimum confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.7")]
        confidence: f64,

        /// Show detailed reasoning for each suggestion
        #[arg(short, long)]
        verbose: bool,

        /// Output format
        #[arg(short, long, default_value = "table")]
        format: OutputFormat,
    },

    /// Create scopes from suggestions
    Create {
        /// Path to analyze (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Minimum confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.8")]
        confidence: f64,

        /// Auto-create without confirmation
        #[arg(short, long)]
        auto: bool,

        /// Dry run - show what would be created without actually creating
        #[arg(long)]
        dry_run: bool,
    },

    /// List available plugins
    Plugins {
        /// Show detailed plugin information
        #[arg(short, long)]
        verbose: bool,

        /// Output format
        #[arg(short, long, default_value = "table")]
        format: OutputFormat,
    },

    /// Show scope loader statistics and cache information
    Stats {
        /// Clear cache after showing stats
        #[arg(short, long)]
        clear_cache: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
    Csv,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Csv => write!(f, "csv"),
        }
    }
}

pub fn handle_scope_loader(
    context: &CliContext,
    subcommand: ScopeLoaderSubcommands,
) -> RhemaResult<()> {
    match subcommand {
        ScopeLoaderSubcommands::Discover {
            path,
            verbose,
            format,
        } => handle_discover(context, path, verbose, format),
        ScopeLoaderSubcommands::Suggest {
            path,
            confidence,
            verbose,
            format,
        } => handle_suggest(context, path, confidence, verbose, format),
        ScopeLoaderSubcommands::Create {
            path,
            confidence,
            auto,
            dry_run,
        } => handle_create(context, path, confidence, auto, dry_run),
        ScopeLoaderSubcommands::Plugins { verbose, format } => {
            handle_plugins(context, verbose, format)
        }
        ScopeLoaderSubcommands::Stats { clear_cache } => handle_stats(context, clear_cache),
    }
}

fn handle_discover(
    context: &CliContext,
    path: Option<PathBuf>,
    verbose: bool,
    format: OutputFormat,
) -> RhemaResult<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    println!(
        "🔍 Discovering package boundaries in: {}",
        target_path.display()
    );

    // Create scope loader service
    let registry = create_plugin_registry()?;
    let config = ScopeLoaderConfig {
        min_confidence_threshold: 0.0, // We want to see all boundaries
        auto_create: false,
        confirm_prompt: false,
        max_depth: 5,
        enable_caching: true,
        cache_duration: 3600,
        cache_path: None,
    };

    let service = ScopeLoaderService::new(registry, config);

    // Detect boundaries
    match tokio::runtime::Runtime::new()?.block_on(service.detect_boundaries(&target_path)) {
        Ok(boundaries) => {
            println!("✅ Discovered {} package boundaries", boundaries.len());

            match format {
                OutputFormat::Table => {
                    if verbose {
                        println!("\n📦 Package Boundaries:");
                        println!(
                            "{:<30} {:<15} {:<20} {:<10}",
                            "Name", "Manager", "Path", "Confidence"
                        );
                        println!("{}", "-".repeat(75));

                        for boundary in &boundaries {
                            println!(
                                "{:<30} {:<15} {:<20}",
                                boundary.package_info.name,
                                boundary.package_manager.as_str(),
                                boundary
                                    .path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy(),
                            );
                        }
                    } else {
                        println!("\n📦 Package Boundaries:");
                        for boundary in &boundaries {
                            println!(
                                "  • {} ({}) at {}",
                                boundary.package_info.name,
                                boundary.package_manager.as_str(),
                                boundary.path.display()
                            );
                        }
                    }
                }
                OutputFormat::Json => {
                    let json = serde_json::to_string_pretty(&boundaries)
                        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
                    println!("{}", json);
                }
                OutputFormat::Yaml => {
                    let yaml = serde_yaml::to_string(&boundaries)
                        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
                    println!("{}", yaml);
                }
                OutputFormat::Csv => {
                    println!("name,manager,path");
                    for boundary in &boundaries {
                        println!(
                            "{},{},{}",
                            boundary.package_info.name,
                            boundary.package_manager.as_str(),
                            boundary.path.display()
                        );
                    }
                }
            }
        }
        Err(e) => {
            context
                .error_handler
                .display_error(&rhema_core::RhemaError::ConfigError(e.to_string()))?;
            return Err(rhema_core::RhemaError::ConfigError(e.to_string()));
        }
    }

    Ok(())
}

fn handle_suggest(
    context: &CliContext,
    path: Option<PathBuf>,
    confidence: f64,
    verbose: bool,
    format: OutputFormat,
) -> RhemaResult<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    println!(
        "💡 Generating scope suggestions for: {}",
        target_path.display()
    );
    println!("🎯 Confidence threshold: {:.2}", confidence);

    // Create scope loader service
    let registry = create_plugin_registry()?;
    let config = ScopeLoaderConfig {
        min_confidence_threshold: confidence,
        auto_create: false,
        confirm_prompt: false,
        max_depth: 5,
        enable_caching: true,
        cache_duration: 3600,
        cache_path: None,
    };

    let service = ScopeLoaderService::new(registry, config);

    // Generate suggestions
    match tokio::runtime::Runtime::new()?.block_on(service.suggest_scopes(&target_path)) {
        Ok(suggestions) => {
            println!("✅ Generated {} scope suggestions", suggestions.len());

            match format {
                OutputFormat::Table => {
                    if verbose {
                        println!("\n🎯 Scope Suggestions:");
                        println!(
                            "{:<30} {:<15} {:<10} {:<50}",
                            "Name", "Type", "Confidence", "Reasoning"
                        );
                        println!("{}", "-".repeat(105));

                        for suggestion in &suggestions {
                            println!(
                                "{:<30} {:<15} {:.2} {:<50}",
                                suggestion.name,
                                suggestion.scope_type.as_str(),
                                suggestion.confidence,
                                suggestion.reasoning.chars().take(47).collect::<String>() + "..."
                            );
                        }
                    } else {
                        println!("\n🎯 Scope Suggestions:");
                        for suggestion in &suggestions {
                            println!(
                                "  • {} ({}) - Confidence: {:.2}",
                                suggestion.name,
                                suggestion.scope_type.as_str(),
                                suggestion.confidence
                            );
                        }
                    }
                }
                OutputFormat::Json => {
                    let json = serde_json::to_string_pretty(&suggestions)
                        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
                    println!("{}", json);
                }
                OutputFormat::Yaml => {
                    let yaml = serde_yaml::to_string(&suggestions)
                        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
                    println!("{}", yaml);
                }
                OutputFormat::Csv => {
                    println!("name,type,confidence,reasoning");
                    for suggestion in &suggestions {
                        println!(
                            "{},{},{:.2},\"{}\"",
                            suggestion.name,
                            suggestion.scope_type.as_str(),
                            suggestion.confidence,
                            suggestion.reasoning.replace("\"", "\"\"")
                        );
                    }
                }
            }
        }
        Err(e) => {
            context
                .error_handler
                .display_error(&rhema_core::RhemaError::ConfigError(e.to_string()))?;
            return Err(rhema_core::RhemaError::ConfigError(e.to_string()));
        }
    }

    Ok(())
}

fn handle_create(
    context: &CliContext,
    path: Option<PathBuf>,
    confidence: f64,
    auto: bool,
    dry_run: bool,
) -> RhemaResult<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    if dry_run {
        println!(
            "🧪 DRY RUN: Would create scopes in: {}",
            target_path.display()
        );
    } else {
        println!("🚀 Creating scopes in: {}", target_path.display());
    }
    println!("🎯 Confidence threshold: {:.2}", confidence);

    // Create scope loader service
    let registry = create_plugin_registry()?;
    let config = ScopeLoaderConfig {
        min_confidence_threshold: confidence,
        auto_create: auto,
        confirm_prompt: !auto,
        max_depth: 5,
        enable_caching: true,
        cache_duration: 3600,
        cache_path: None,
    };

    let service = ScopeLoaderService::new(registry, config);

    // Generate suggestions first
    let suggestions =
        match tokio::runtime::Runtime::new()?.block_on(service.suggest_scopes(&target_path)) {
            Ok(s) => s,
            Err(e) => {
                context
                    .error_handler
                    .display_error(&rhema_core::RhemaError::ConfigError(e.to_string()))?;
                return Err(rhema_core::RhemaError::ConfigError(e.to_string()));
            }
        };

    if suggestions.is_empty() {
        println!(
            "ℹ️  No scope suggestions found with confidence >= {:.2}",
            confidence
        );
        return Ok(());
    }

    println!("📋 Found {} scope suggestions:", suggestions.len());
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!(
            "  {}. {} ({}) - Confidence: {:.2}",
            i + 1,
            suggestion.name,
            suggestion.scope_type.as_str(),
            suggestion.confidence
        );
    }

    if dry_run {
        println!("\n🧪 DRY RUN: Would create {} scopes", suggestions.len());
        return Ok(());
    }

    // Create scopes
    match tokio::runtime::Runtime::new()?.block_on(service.auto_create_scopes(&target_path)) {
        Ok(scopes) => {
            println!("✅ Successfully created {} scopes:", scopes.len());
            for scope in &scopes {
                println!("  • {} at {}", scope.definition.name, scope.path.display());
            }
        }
        Err(e) => {
            context
                .error_handler
                .display_error(&rhema_core::RhemaError::ConfigError(e.to_string()))?;
            return Err(rhema_core::RhemaError::ConfigError(e.to_string()));
        }
    }

    Ok(())
}

fn handle_plugins(_context: &CliContext, verbose: bool, format: OutputFormat) -> RhemaResult<()> {
    println!("🔌 Available Scope Loader Plugins");

    let registry = create_plugin_registry()?;
    let plugins = registry.list_plugins();

    match format {
        OutputFormat::Table => {
            if verbose {
                println!("\n📦 Plugin Details:");
                println!(
                    "{:<20} {:<10} {:<50} {:<20}",
                    "Name", "Version", "Description", "Package Managers"
                );
                println!("{}", "-".repeat(100));

                for plugin_info in &plugins {
                    let managers = plugin_info.metadata.supported_package_managers.join(", ");
                    println!(
                        "{:<20} {:<10} {:<50} {:<20}",
                        plugin_info.metadata.name,
                        plugin_info.metadata.version,
                        plugin_info
                            .metadata
                            .description
                            .chars()
                            .take(47)
                            .collect::<String>()
                            + "...",
                        managers
                    );
                }
            } else {
                println!("\n📦 Plugins:");
                for plugin_info in &plugins {
                    println!(
                        "  • {} v{} - {}",
                        plugin_info.metadata.name,
                        plugin_info.metadata.version,
                        plugin_info.metadata.description
                    );
                }
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&plugins)
                .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
            println!("{}", json);
        }
        OutputFormat::Yaml => {
            let yaml = serde_yaml::to_string(&plugins)
                .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
            println!("{}", yaml);
        }
        OutputFormat::Csv => {
            println!("name,version,description,package_managers");
            for plugin_info in &plugins {
                let managers = plugin_info.metadata.supported_package_managers.join(";");
                println!(
                    "{},{},\"{}\",{}",
                    plugin_info.metadata.name,
                    plugin_info.metadata.version,
                    plugin_info.metadata.description.replace("\"", "\"\""),
                    managers
                );
            }
        }
    }

    Ok(())
}

fn handle_stats(_context: &CliContext, clear_cache: bool) -> RhemaResult<()> {
    println!("📊 Scope Loader Statistics");

    // Create scope loader service
    let registry = create_plugin_registry()?;
    let config = ScopeLoaderConfig {
        min_confidence_threshold: 0.7,
        auto_create: false,
        confirm_prompt: false,
        max_depth: 5,
        enable_caching: true,
        cache_duration: 3600,
        cache_path: None,
    };

    let service = ScopeLoaderService::new(registry, config);

    // Get cache stats
    let stats = tokio::runtime::Runtime::new()?.block_on(service.cache_stats());

    println!("\n💾 Cache Statistics:");
    println!("  • Boundaries cached: {}", stats.boundaries_count);
    println!("  • Suggestions cached: {}", stats.suggestions_count);
    println!("  • Scopes cached: {}", stats.scopes_count);

    if clear_cache {
        println!("\n🧹 Clearing cache...");
        tokio::runtime::Runtime::new()?.block_on(service.clear_cache());
        println!("✅ Cache cleared");
    }

    Ok(())
}

fn create_plugin_registry() -> RhemaResult<PluginRegistry> {
    let mut registry = PluginRegistry::new();

    // Register built-in plugins
    registry
        .register_plugin(Box::new(CargoPlugin::new()))
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    registry
        .register_plugin(Box::new(NodePackagePlugin::new()))
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    registry
        .register_plugin(Box::new(NxPlugin::new()))
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;

    Ok(registry)
}
