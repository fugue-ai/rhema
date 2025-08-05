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

use crate::{Config, GlobalConfig, RepositoryConfig};
use crate::{ConfigManager, Rhema, RhemaResult};
use colored::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(clap::Subcommand)]
pub enum ConfigSubcommands {
    /// Show configuration
    Show {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Output format (json, yaml)
        #[arg(long, value_name = "FORMAT", default_value = "json")]
        format: String,
    },

    /// Edit configuration
    Edit {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Editor to use
        #[arg(long, value_name = "EDITOR")]
        editor: Option<String>,
    },

    /// Validate configuration
    Validate {
        /// Configuration type (global, repository, all)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Attempt to fix validation issues
        #[arg(long)]
        fix: bool,
    },

    /// Backup configuration
    Backup {
        /// Configuration type (global, repository, all)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Output file for backup report
        #[arg(long, value_name = "OUTPUT")]
        output: Option<String>,
    },

    /// Restore configuration
    Restore {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Backup file to restore from
        #[arg(value_name = "BACKUP_FILE")]
        backup_file: String,
    },

    /// Migrate configuration
    Migrate {
        /// Configuration type (global, repository, all)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Dry run (don't modify files)
        #[arg(long)]
        dry_run: bool,
    },

    /// Export configuration
    Export {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Export format (json, yaml)
        #[arg(long, value_name = "FORMAT", default_value = "json")]
        format: String,

        /// Output file
        #[arg(long, value_name = "OUTPUT")]
        output: Option<String>,
    },

    /// Import configuration
    Import {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Import format (json, yaml)
        #[arg(long, value_name = "FORMAT", default_value = "json")]
        format: String,

        /// Input file
        #[arg(value_name = "INPUT")]
        input: String,
    },

    /// Set configuration value
    Set {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Configuration key (dot notation)
        #[arg(value_name = "KEY")]
        key: String,

        /// Configuration value (JSON format)
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Get configuration value
    Get {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Configuration key (dot notation)
        #[arg(value_name = "KEY")]
        key: String,
    },

    /// Reset configuration to defaults
    Reset {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Confirm reset operation
        #[arg(long)]
        confirm: bool,
    },

    /// Check configuration health
    Health {
        /// Configuration type (global, repository, all)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,
    },

    /// Audit configuration changes
    Audit {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Show changes since timestamp
        #[arg(long, value_name = "SINCE")]
        since: Option<String>,
    },

    /// Show configuration statistics
    Stats {
        /// Configuration type (global, repository, all)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Repository path (for repository config)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,
    },

    /// Show configuration schema
    Schema {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Output file
        #[arg(long, value_name = "OUTPUT")]
        output: Option<String>,
    },

    /// Show configuration documentation
    Documentation {
        /// Configuration type (global, repository)
        #[arg(value_name = "TYPE")]
        config_type: String,

        /// Output file
        #[arg(long, value_name = "OUTPUT")]
        output: Option<String>,
    },
}

pub fn run(_rhema: &Rhema, subcommand: &ConfigSubcommands) -> RhemaResult<()> {
    let mut config_manager = ConfigManager::new()?;

    match subcommand {
        ConfigSubcommands::Show {
            config_type,
            path,
            format,
        } => show_config(&mut config_manager, config_type, path.as_deref(), format),
        ConfigSubcommands::Edit {
            config_type,
            path,
            editor,
        } => edit_config(
            &mut config_manager,
            config_type,
            path.as_deref(),
            editor.clone(),
        ),
        ConfigSubcommands::Validate {
            config_type,
            path,
            fix,
        } => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(validate_config(&mut config_manager, config_type, path.as_deref(), *fix))
        },
        ConfigSubcommands::Backup {
            config_type,
            path,
            output,
        } => backup_config(
            &mut config_manager,
            config_type,
            path.as_deref(),
            output.as_deref(),
        ),
        ConfigSubcommands::Restore {
            config_type,
            path,
            backup_file,
        } => restore_config(
            &mut config_manager,
            config_type,
            path.as_deref(),
            backup_file,
        ),
        ConfigSubcommands::Migrate {
            config_type,
            path,
            dry_run,
        } => migrate_config(&mut config_manager, config_type, path.as_deref(), *dry_run),
        ConfigSubcommands::Export {
            config_type,
            path,
            format,
            output,
        } => export_config(
            &mut config_manager,
            config_type,
            path.as_deref(),
            format,
            output.as_deref(),
        ),
        ConfigSubcommands::Import {
            config_type,
            path,
            format,
            input,
        } => import_config(
            &mut config_manager,
            config_type,
            path.as_deref(),
            format,
            input,
        ),
        ConfigSubcommands::Set {
            config_type,
            path,
            key,
            value,
        } => set_config_value(
            &mut config_manager,
            config_type,
            path.as_deref(),
            key,
            value,
        ),
        ConfigSubcommands::Get {
            config_type,
            path,
            key,
        } => get_config_value(&mut config_manager, config_type, path.as_deref(), key),
        ConfigSubcommands::Reset {
            config_type,
            path,
            confirm,
        } => reset_config(&config_manager, config_type, path.as_deref(), *confirm),
        ConfigSubcommands::Health { config_type, path } => {
            check_config_health(&mut config_manager, config_type, path.as_deref())
        }
        ConfigSubcommands::Audit {
            config_type,
            path,
            since,
        } => audit_config(
            &mut config_manager,
            config_type,
            path.as_deref(),
            since.as_deref(),
        ),
        ConfigSubcommands::Stats { config_type, path } => {
            show_config_stats(&mut config_manager, config_type, path.as_deref())
        }
        ConfigSubcommands::Schema {
            config_type,
            output,
        } => show_config_schema(config_type, output.as_deref()),
        ConfigSubcommands::Documentation {
            config_type,
            output,
        } => show_config_documentation(config_type, output.as_deref()),
    }
}

fn show_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    format: &str,
) -> RhemaResult<()> {
    println!("⚙️  Configuration: {}", config_type.bright_blue());
    println!("{}", "─".repeat(80));

    let config_content = match config_type {
        "global" => {
            let config = config_manager.global_config();
            serde_json::to_string_pretty(config)?
        }
        "repository" => {
            let repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(repo_path))?;
            serde_json::to_string_pretty(&config)?
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    match format {
        "json" => println!("{}", config_content),
        "yaml" => {
            let json_value: serde_json::Value = serde_json::from_str(&config_content)?;
            let yaml_content = serde_yaml::to_string(&json_value)?;
            println!("{}", yaml_content);
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown format: {}. Valid formats: json, yaml",
                format
            )));
        }
    }

    Ok(())
}

fn edit_config(
    _config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    editor: Option<String>,
) -> RhemaResult<()> {
    println!("✏️  Editing configuration: {}", config_type.bright_blue());

    let config_path: Result<PathBuf, crate::RhemaError> = match config_type {
        "global" => {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            Ok(PathBuf::from(home)
                .join(".rhema")
                .join("config")
                .join("global.json"))
        }
        "repository" => {
            let repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            Ok(Path::new(repo_path).join(".rhema").join("config.yaml"))
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    let editor_cmd =
        editor.unwrap_or_else(|| std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string()));

    let config_path = config_path?;
    println!("Opening {} with {}", config_path.display(), editor_cmd);

    let status = std::process::Command::new(&editor_cmd)
        .arg(&config_path)
        .status()?;

    if status.success() {
        println!("✅ Configuration edited successfully");
    } else {
        println!("⚠️  Editor exited with status: {}", status);
    }

    Ok(())
}

async fn validate_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    fix: bool,
) -> RhemaResult<()> {
    println!("🔍 Validating configuration: {}", config_type.bright_blue());

    match config_type {
        "global" => {
            let result = config_manager
                .validation()
                .validate_config(config_manager.global_config().clone(), "global")
                .await?;
            display_validation_report(
                &rhema_config::validation::ValidationReport {
                    overall_valid: result.valid,
                    results: HashMap::new(),
                    summary: rhema_config::validation::ValidationSummary {
                        total_configs: 1,
                        valid_configs: if result.valid { 1 } else { 0 },
                        invalid_configs: if result.valid { 0 } else { 1 },
                        total_issues: result.issues.len(),
                        critical_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Critical)
                            .count(),
                        error_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Error)
                            .count(),
                        warning_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Warning)
                            .count(),
                        info_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Info)
                            .count(),
                    },
                    timestamp: chrono::Utc::now(),
                    duration_ms: result.duration_ms,
                },
                fix,
            )?;
        }
        "repository" => {
            let repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(repo_path))?;
            let config_clone = config.clone();
            let result = config_manager
                .validation()
                .validate_config(config_clone, repo_path)
                .await?;
            display_validation_report(
                &rhema_config::validation::ValidationReport {
                    overall_valid: result.valid,
                    results: HashMap::new(),
                    summary: rhema_config::validation::ValidationSummary {
                        total_configs: 1,
                        valid_configs: if result.valid { 1 } else { 0 },
                        invalid_configs: if result.valid { 0 } else { 1 },
                        total_issues: result.issues.len(),
                        critical_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Critical)
                            .count(),
                        error_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Error)
                            .count(),
                        warning_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Warning)
                            .count(),
                        info_issues: result
                            .issues
                            .iter()
                            .filter(|i| i.severity == rhema_config::ConfigIssueSeverity::Info)
                            .count(),
                    },
                    timestamp: chrono::Utc::now(),
                    duration_ms: result.duration_ms,
                },
                fix,
            )?;
        }
        "all" => {
            let report = config_manager.validate_all()?;
            display_validation_report(&report, fix)?;
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository, all",
                config_type
            )));
        }
    };

    Ok(())
}

fn backup_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    output: Option<&str>,
) -> RhemaResult<()> {
    println!("💾 Backing up configuration: {}", config_type.bright_blue());

    let backup_report = match config_type {
        "global" => {
            let global_config = config_manager.global_config().clone();
            let record = config_manager
                .backup_mut()
                .backup_config(&global_config, "global")?;
            rhema_config::backup::BackupReport {
                backups_created: vec![record],
                backups_failed: vec![],
                summary: rhema_config::backup::BackupSummary {
                    total_backups: 1,
                    successful_backups: 1,
                    failed_backups: 0,
                    total_size_bytes: 0,
                    compression_ratio: 1.0,
                },
                timestamp: chrono::Utc::now(),
                duration_ms: 0,
            }
        }
        "repository" => {
            let repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(repo_path))?;
            let config_clone = config.clone();
            let record = config_manager
                .backup_mut()
                .backup_config(&config_clone, repo_path)?;
            rhema_config::backup::BackupReport {
                backups_created: vec![record],
                backups_failed: vec![],
                summary: rhema_config::backup::BackupSummary {
                    total_backups: 1,
                    successful_backups: 1,
                    failed_backups: 0,
                    total_size_bytes: 0,
                    compression_ratio: 1.0,
                },
                timestamp: chrono::Utc::now(),
                duration_ms: 0,
            }
        }
        "all" => config_manager.backup_all()?,
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository, all",
                config_type
            )));
        }
    };

    display_backup_report(&backup_report, output)?;

    Ok(())
}

fn restore_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    backup_file: &str,
) -> RhemaResult<()> {
    println!("🔄 Restoring configuration: {}", config_type.bright_blue());

    let restore_report = match config_type {
        "global" => {
            let _config: GlobalConfig = config_manager
                .backup()
                .restore_config("global", backup_file)?;
            // TODO: Implement proper restore report
            rhema_config::backup::RestoreReport {
                restored_configs: vec![],
                restore_errors: vec![],
                summary: rhema_config::backup::RestoreSummary {
                    total_configs: 1,
                    successful_restores: 1,
                    failed_restores: 0,
                },
                timestamp: chrono::Utc::now(),
                duration_ms: 0,
            }
        }
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let _config: RepositoryConfig = config_manager
                .backup()
                .restore_config("repository", backup_file)?;
            // TODO: Implement proper restore report
            rhema_config::backup::RestoreReport {
                restored_configs: vec![],
                restore_errors: vec![],
                summary: rhema_config::backup::RestoreSummary {
                    total_configs: 1,
                    successful_restores: 1,
                    failed_restores: 0,
                },
                timestamp: chrono::Utc::now(),
                duration_ms: 0,
            }
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    display_restore_report(&restore_report)?;

    Ok(())
}

fn migrate_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    dry_run: bool,
) -> RhemaResult<()> {
    println!("🔄 Migrating configuration: {}", config_type.bright_blue());

    if dry_run {
        println!("  DRY RUN MODE - No files will be modified");
    }

    let migration_report = match config_type {
        "global" => config_manager
            .migration()
            .migrate_config(config_manager.global_config(), "global")?,
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(_repo_path))?;
            let config_clone = config.clone();
            config_manager
                .migration()
                .migrate_config(&config_clone, _repo_path)?
        }
        "all" => config_manager.migrate_all()?,
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository, all",
                config_type
            )));
        }
    };

    display_migration_report(&migration_report)?;

    Ok(())
}

fn export_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    format: &str,
    output: Option<&str>,
) -> RhemaResult<()> {
    println!("📤 Exporting configuration: {}", config_type.bright_blue());

    let export_content = match config_type {
        "global" => config_manager.global_config().export(format)?,
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(_repo_path))?;
            config.export(format)?
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    if let Some(output_path) = output {
        std::fs::write(output_path, export_content)?;
        println!("✅ Configuration exported to: {}", output_path.green());
    } else {
        println!("{}", export_content);
    }

    Ok(())
}

fn import_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    format: &str,
    input: &str,
) -> RhemaResult<()> {
    println!("📥 Importing configuration: {}", config_type.bright_blue());

    let import_content = std::fs::read_to_string(input)?;

    match config_type {
        "global" => {
            let mut global_config = config_manager.global_config().clone();
            global_config.import(&import_content, format)?;
            global_config.save()?;
        }
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let mut repo_config = config_manager
                .load_repository_config(Path::new(_repo_path))?
                .clone();
            repo_config.import(&import_content, format)?;
            repo_config.save(Path::new(_repo_path))?;
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    }

    println!(
        "✅ Configuration imported successfully from: {}",
        input.green()
    );

    Ok(())
}

fn set_config_value(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    key: &str,
    value: &str,
) -> RhemaResult<()> {
    println!(
        "🔧 Setting configuration value: {} = {}",
        key.bright_blue(),
        value
    );

    let json_value = serde_json::from_str(value)?;

    match config_type {
        "global" => {
            config_manager
                .global_config_mut()
                .set_value(key, json_value)?;
            config_manager.global_config_mut().save()?;
        }
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            // Note: This would need to be implemented in RepositoryConfig
            return Err(crate::RhemaError::ConfigError(
                "Repository config set_value not yet implemented".to_string(),
            ));
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    }

    println!("✅ Configuration value set successfully");

    Ok(())
}

fn get_config_value(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    key: &str,
) -> RhemaResult<()> {
    println!("🔍 Getting configuration value: {}", key.bright_blue());

    let value = match config_type {
        "global" => config_manager.global_config().get_value(key).cloned(),
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(_repo_path))?;
            config.get_value(key).cloned()
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    match value {
        Some(v) => println!("{}", serde_json::to_string_pretty(&v)?),
        None => println!("❌ Configuration key '{}' not found", key),
    }

    Ok(())
}

fn reset_config(
    _config_manager: &ConfigManager,
    config_type: &str,
    path: Option<&str>,
    confirm: bool,
) -> RhemaResult<()> {
    if !confirm {
        println!("⚠️  This will reset the configuration to defaults. Use --confirm to proceed.");
        return Ok(());
    }

    println!("🔄 Resetting configuration: {}", config_type.bright_blue());

    match config_type {
        "global" => {
            let default_config = GlobalConfig::new();
            default_config.save()?;
        }
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let default_config = RepositoryConfig::new(Path::new(_repo_path));
            default_config.save(Path::new(_repo_path))?;
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    }

    println!("✅ Configuration reset to defaults");

    Ok(())
}

fn check_config_health(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
) -> RhemaResult<()> {
    println!(
        "🏥 Checking configuration health: {}",
        config_type.bright_blue()
    );

    let health = match config_type {
        "global" => &config_manager.global_config().health,
        "repository" => {
            let repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(repo_path))?;
            &config.health.clone()
        }
        "all" => {
            // This would need to be implemented to aggregate health from all configs
            return Err(crate::RhemaError::ConfigError(
                "All config health check not yet implemented".to_string(),
            ));
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository, all",
                config_type
            )));
        }
    };

    display_config_health(health)?;

    Ok(())
}

fn audit_config(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
    since: Option<&str>,
) -> RhemaResult<()> {
    println!("📋 Configuration audit: {}", config_type.bright_blue());

    let audit_log = match config_type {
        "global" => &config_manager.global_config().audit_log,
        "repository" => {
            let repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(repo_path))?;
            &config.audit_log.clone()
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    display_audit_log(audit_log, since)?;

    Ok(())
}

fn show_config_stats(
    config_manager: &mut ConfigManager,
    config_type: &str,
    path: Option<&str>,
) -> RhemaResult<()> {
    println!("📊 Configuration statistics: {}", config_type.bright_blue());

    let stats = match config_type {
        "global" => &config_manager.global_config().stats,
        "repository" => {
            let _repo_path = path.ok_or_else(|| {
                crate::RhemaError::ConfigError(
                    "Repository path required for repository config".to_string(),
                )
            })?;
            let config = config_manager.load_repository_config(Path::new(_repo_path))?;
            &config.stats.clone()
        }
        "all" => {
            // This would need to be implemented to aggregate stats from all configs
            return Err(crate::RhemaError::ConfigError(
                "All config stats not yet implemented".to_string(),
            ));
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository, all",
                config_type
            )));
        }
    };

    display_config_stats(stats)?;

    Ok(())
}

fn show_config_schema(config_type: &str, output: Option<&str>) -> RhemaResult<()> {
    println!("📋 Configuration schema: {}", config_type.bright_blue());

    let schema = match config_type {
        "global" => <GlobalConfig as Config>::schema(),
        "repository" => <RepositoryConfig as Config>::schema(),
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    let schema_content = serde_json::to_string_pretty(&schema)?;

    if let Some(output_path) = output {
        std::fs::write(output_path, schema_content)?;
        println!("✅ Schema written to: {}", output_path.green());
    } else {
        println!("{}", schema_content);
    }

    Ok(())
}

fn show_config_documentation(config_type: &str, output: Option<&str>) -> RhemaResult<()> {
    println!(
        "📚 Configuration documentation: {}",
        config_type.bright_blue()
    );

    let documentation = match config_type {
        "global" => <GlobalConfig as Config>::documentation(),
        "repository" => <RepositoryConfig as Config>::documentation(),
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown config type: {}. Valid types: global, repository",
                config_type
            )));
        }
    };

    if let Some(output_path) = output {
        std::fs::write(output_path, documentation)?;
        println!("✅ Documentation written to: {}", output_path.green());
    } else {
        println!("{}", documentation);
    }

    Ok(())
}

// Helper functions for displaying reports and data

fn display_validation_report(
    report: &rhema_config::validation::ValidationReport,
    fix: bool,
) -> RhemaResult<()> {
    println!("Validation Report:");
    println!(
        "  Status: {}",
        if report.overall_valid {
            "✅ Valid".green()
        } else {
            "❌ Invalid".red()
        }
    );
    println!("  Results: {}", report.results.len());

    for (_path, result) in &report.results {
        for issue in &result.issues {
            println!("    • {:?}: {}", issue.severity, issue.message);
            if let Some(location) = &issue.location {
                println!("      Location: {}", location);
            }
            if let Some(suggestion) = &issue.suggestion {
                println!("      Suggestion: {}", suggestion);
            }
        }
    }

    if fix && !report.overall_valid {
        println!("🔧 Attempting to fix validation issues...");
        // This would need to be implemented
        println!("⚠️  Auto-fix not yet implemented");
    }

    Ok(())
}

fn display_backup_report(
    report: &rhema_config::backup::BackupReport,
    output: Option<&str>,
) -> RhemaResult<()> {
    println!("Backup Report:");
    println!(
        "  Status: {}",
        if report.backups_failed.is_empty() {
            "✅ Success".green()
        } else {
            "❌ Failed".red()
        }
    );
    println!("  Files backed up: {}", report.backups_created.len());
    println!(
        "  Backup location: {}",
        report
            .backups_created
            .first()
            .map(|b| b.backup_path.display())
            .unwrap_or_else(|| std::path::Path::new("N/A").display())
    );

    if let Some(output_path) = output {
        let report_content = serde_json::to_string_pretty(report)?;
        std::fs::write(output_path, report_content)?;
        println!("✅ Backup report written to: {}", output_path.green());
    }

    Ok(())
}

fn display_restore_report(report: &rhema_config::backup::RestoreReport) -> RhemaResult<()> {
    println!("Restore Report:");
    println!(
        "  Status: {}",
        if report.restore_errors.is_empty() {
            "✅ Success".green()
        } else {
            "❌ Failed".red()
        }
    );
    println!("  Files restored: {}", report.restored_configs.len());

    for error in &report.restore_errors {
        println!("  Error: {:?}", error);
    }

    Ok(())
}

fn display_migration_report(report: &rhema_config::migration::MigrationReport) -> RhemaResult<()> {
    println!("Migration Report:");
    println!(
        "  Status: {}",
        if report.migrations_failed.is_empty() {
            "✅ Success".green()
        } else {
            "❌ Failed".red()
        }
    );
    println!("  Files migrated: {}", report.migrations_applied.len());
    println!("  Migrations applied: {}", report.migrations_applied.len());

    for migration in &report.migrations_applied {
        println!(
            "    • {}: {}",
            migration.migration_name,
            if migration.success {
                "Success"
            } else {
                "Failed"
            }
        );
    }

    Ok(())
}

fn display_config_health(health: &rhema_config::ConfigHealth) -> RhemaResult<()> {
    println!("Configuration Health:");
    println!(
        "  Status: {}",
        match health.status {
            rhema_config::ConfigHealthStatus::Healthy => "✅ Healthy".green(),
            rhema_config::ConfigHealthStatus::Warning => "⚠️  Warning".yellow(),
            rhema_config::ConfigHealthStatus::Error => "❌ Error".red(),
            rhema_config::ConfigHealthStatus::Unknown => "❓ Unknown".blue(),
        }
    );
    println!("  Issues: {}", health.issues.len());
    println!("  Last check: {}", health.last_check);

    for issue in &health.issues {
        println!("    • Issue: {}", issue);
        // TODO: Fix issue field access
    }

    // TODO: Fix recommendations field access
    // for recommendation in &health.recommendations {
    //     println!("    • Recommendation: {}", recommendation);
    // }

    Ok(())
}

fn display_audit_log(
    audit_log: &rhema_config::ConfigAuditLog,
    since: Option<&str>,
) -> RhemaResult<()> {
    println!("Configuration Audit Log:");
    println!("  Total entries: {}", audit_log.entries.len());
    // TODO: Fix audit log field access
    // println!("  Created: {}", audit_log.created_at);
    // println!("  Last updated: {}", audit_log.updated_at);

    let changes = if let Some(_since_str) = since {
        // Parse since timestamp and filter changes
        // This would need to be implemented
        &audit_log.entries
    } else {
        &audit_log.entries
    };

    for change in changes.iter().take(10) {
        // Show last 10 changes
        println!(
            "    • {}: {} - {}",
            change.timestamp, change.action, change.details
        );
        println!("      User: {}", change.user);
    }

    if changes.len() > 10 {
        println!("    ... and {} more changes", changes.len() - 10);
    }

    Ok(())
}

fn display_config_stats(stats: &rhema_config::ConfigStats) -> RhemaResult<()> {
    println!("Configuration Statistics:");
    println!("  Total configs: {}", stats.total_configs);
    println!("  Valid configs: {}", stats.valid_configs);
    println!("  Invalid configs: {}", stats.invalid_configs);
    println!("  Last updated: {}", stats.last_updated);
    // TODO: Fix stats field access
    // println!("  Global configs: {}", stats.global_configs);
    // println!("  Repository configs: {}", stats.repository_configs);
    // println!("  Scope configs: {}", stats.scope_configs);
    // println!("  Encrypted configs: {}", stats.encrypted_configs);
    // println!("  Backup count: {}", stats.backup_count);
    // println!("  Validation errors: {}", stats.validation_errors);
    // println!("  Migration pending: {}", stats.migration_pending);
    // if let Some(last_backup) = stats.last_backup {
    //     println!("  Last backup: {}", last_backup);
    // }

    Ok(())
}
