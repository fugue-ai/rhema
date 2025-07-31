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

use crate::file_ops;
use crate::{export_context, health, interactive_builder, migrate, query, search, stats, validate};
use crate::{Rhema, RhemaResult};
use rhema_core::schema::{Conventions, Decisions, Knowledge, Patterns, RhemaScope, Todos};
// use crate::scope::find_nearest_scope;
// use rhema_core::scope::Scope;
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
// use chrono::Utc;
// use uuid::Uuid;
use clap::Subcommand;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

/// Batch subcommands for different types of bulk operations
#[derive(Subcommand)]
pub enum BatchSubcommands {
    /// Bulk context file operations and management
    Context {
        /// Operation type (validate, migrate, export, import, health-check)
        #[arg(value_name = "OPERATION")]
        operation: String,

        /// Input file with operation parameters
        #[arg(long, value_name = "FILE")]
        input_file: String,

        /// Scope filter (optional)
        #[arg(long, value_name = "SCOPE")]
        scope_filter: Option<String>,

        /// Dry run mode (don't modify files)
        #[arg(long)]
        dry_run: bool,
    },

    /// Batch command execution and processing
    Commands {
        /// Command file with batch commands
        #[arg(long, value_name = "FILE")]
        command_file: String,

        /// Scope filter (optional)
        #[arg(long, value_name = "SCOPE")]
        scope_filter: Option<String>,

        /// Execute commands in parallel
        #[arg(long)]
        parallel: bool,

        /// Maximum number of parallel workers
        #[arg(long, value_name = "WORKERS", default_value = "4")]
        max_workers: usize,
    },

    /// Mass data import and export capabilities
    Data {
        /// Operation type (export, import)
        #[arg(value_name = "OPERATION")]
        operation: String,

        /// Input path for import or base path for export
        #[arg(long, value_name = "INPUT")]
        input_path: String,

        /// Output path for export or target path for import
        #[arg(long, value_name = "OUTPUT")]
        output_path: String,

        /// Data format (json, yaml, csv)
        #[arg(long, value_name = "FORMAT", default_value = "json")]
        format: String,

        /// Scope filter (optional)
        #[arg(long, value_name = "SCOPE")]
        scope_filter: Option<String>,
    },

    /// Bulk validation and health checking
    Validate {
        /// Validation type (validate, health-check, schema-check, dependency-check)
        #[arg(value_name = "TYPE")]
        validation_type: String,

        /// Scope filter (optional)
        #[arg(long, value_name = "SCOPE")]
        scope_filter: Option<String>,

        /// Output file for detailed report
        #[arg(long, value_name = "FILE")]
        output_file: Option<String>,

        /// Include detailed information
        #[arg(long)]
        detailed: bool,
    },

    /// Batch reporting and analytics
    Report {
        /// Report type (summary, analytics, health, dependencies, todos, knowledge)
        #[arg(value_name = "TYPE")]
        report_type: String,

        /// Scope filter (optional)
        #[arg(long, value_name = "SCOPE")]
        scope_filter: Option<String>,

        /// Output file for the report
        #[arg(long, value_name = "FILE")]
        output_file: String,

        /// Report format (json, yaml, markdown, html, csv)
        #[arg(long, value_name = "FORMAT", default_value = "json")]
        format: String,

        /// Include detailed information
        #[arg(long)]
        include_details: bool,
    },
}

/// Batch operation result with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// Operation type
    pub operation_type: String,
    /// Total items processed
    pub total_items: usize,
    /// Successfully processed items
    pub successful: usize,
    /// Failed items
    pub failed: usize,
    /// Warnings
    pub warnings: Vec<String>,
    /// Errors
    pub errors: Vec<String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Detailed results for each item
    pub details: Vec<BatchItemResult>,
}

/// Individual batch item result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemResult {
    /// Item identifier (file path, scope name, etc.)
    pub item_id: String,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Warning messages
    pub warnings: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Main entry point for batch operations
pub fn run(rhema: &Rhema, subcommand: &BatchSubcommands) -> RhemaResult<()> {
    match subcommand {
        BatchSubcommands::Context {
            operation,
            input_file,
            scope_filter,
            dry_run,
        } => run_context_operations(
            rhema,
            operation,
            input_file,
            scope_filter.as_deref(),
            *dry_run,
        ),
        BatchSubcommands::Commands {
            command_file,
            scope_filter,
            parallel,
            max_workers,
        } => run_command_execution(
            rhema,
            command_file,
            scope_filter.as_deref(),
            *parallel,
            *max_workers,
        ),
        BatchSubcommands::Data {
            operation,
            input_path,
            output_path,
            format,
            scope_filter,
        } => run_data_operations(
            rhema,
            operation,
            input_path,
            output_path,
            format,
            scope_filter.as_deref(),
        ),
        BatchSubcommands::Validate {
            validation_type,
            scope_filter,
            output_file,
            detailed,
        } => run_validation_operations(
            rhema,
            validation_type,
            scope_filter.as_deref(),
            output_file.as_deref(),
            *detailed,
        ),
        BatchSubcommands::Report {
            report_type,
            scope_filter,
            output_file,
            format,
            include_details,
        } => run_reporting_operations(
            rhema,
            report_type,
            scope_filter.as_deref(),
            output_file,
            format,
            *include_details,
        ),
    }
}

/// Batch context file operations
pub fn run_context_operations(
    rhema: &Rhema,
    operation: &str,
    input_file: &str,
    scope_filter: Option<&str>,
    dry_run: bool,
) -> RhemaResult<()> {
    println!("🔄 Executing batch context operations...");
    println!("{}", "─".repeat(80));

    let start_time = std::time::Instant::now();
    let mut result = BatchOperationResult {
        operation_type: operation.to_string(),
        total_items: 0,
        successful: 0,
        failed: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        processing_time_ms: 0,
        details: Vec::new(),
    };

    // Discover scopes
    let scopes = rhema.discover_scopes()?;
    let filtered_scopes = if let Some(filter) = scope_filter {
        scopes
            .into_iter()
            .filter(|scope| scope.path.to_string_lossy().contains(filter))
            .collect()
    } else {
        scopes
    };

    if filtered_scopes.is_empty() {
        println!("{}", "No scopes found matching the filter.".yellow());
        return Ok(());
    }

    // Read input file
    let input_data = read_batch_input_file(input_file)?;

    // Create progress bar
    let progress_bar = ProgressBar::new(filtered_scopes.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Process each scope
    for scope in &filtered_scopes {
        progress_bar.set_message(format!("Processing scope: {}", scope.path.display()));

        match operation {
            "validate" => {
                let item_result = batch_validate_scope(rhema, scope, &input_data)?;
                result.details.push(item_result);
            }
            "migrate" => {
                let item_result = if dry_run {
                    batch_migrate_scope_dry_run(rhema, scope, &input_data)?
                } else {
                    batch_migrate_scope(rhema, scope, &input_data)?
                };
                result.details.push(item_result);
            }
            "export" => {
                let item_result = batch_export_scope(rhema, scope, &input_data)?;
                result.details.push(item_result);
            }
            "import" => {
                let item_result = if dry_run {
                    batch_import_scope_dry_run(rhema, scope, &input_data)?
                } else {
                    batch_import_scope(rhema, scope, &input_data)?
                };
                result.details.push(item_result);
            }
            "health-check" => {
                let item_result = batch_health_check_scope(rhema, scope, &input_data)?;
                result.details.push(item_result);
            }
            _ => {
                return Err(crate::RhemaError::ConfigError(format!(
                    "Unknown batch operation: {}",
                    operation
                )));
            }
        }

        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Batch operation completed");

    // Calculate statistics
    result.total_items = filtered_scopes.len();
    result.successful = result.details.iter().filter(|d| d.success).count();
    result.failed = result.total_items - result.successful;
    result.processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Print results
    print_batch_results(&result);

    Ok(())
}

/// Batch command execution
pub fn run_command_execution(
    rhema: &Rhema,
    command_file: &str,
    scope_filter: Option<&str>,
    parallel: bool,
    _max_workers: usize,
) -> RhemaResult<()> {
    println!("⚡ Executing batch commands...");
    println!("{}", "─".repeat(80));

    let start_time = std::time::Instant::now();
    let mut result = BatchOperationResult {
        operation_type: "command_execution".to_string(),
        total_items: 0,
        successful: 0,
        failed: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        processing_time_ms: 0,
        details: Vec::new(),
    };

    // Read command file
    let commands = read_batch_commands_file(command_file)?;

    // Discover scopes
    let scopes = rhema.discover_scopes()?;
    let filtered_scopes = if let Some(filter) = scope_filter {
        scopes
            .into_iter()
            .filter(|scope| scope.path.to_string_lossy().contains(filter))
            .collect()
    } else {
        scopes
    };

    if filtered_scopes.is_empty() {
        println!("{}", "No scopes found matching the filter.".yellow());
        return Ok(());
    }

    // Create progress bar
    let total_operations = commands.len() * filtered_scopes.len();
    let progress_bar = ProgressBar::new(total_operations as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    if parallel {
        // Parallel execution using rayon
        use rayon::prelude::*;

        let results: Vec<BatchItemResult> = filtered_scopes
            .par_iter()
            .flat_map(|scope| {
                commands
                    .iter()
                    .map(|cmd| {
                        let item_result = execute_command_on_scope(rhema, scope, cmd);
                        progress_bar.inc(1);
                        item_result
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        result.details = results;
    } else {
        // Sequential execution
        for scope in &filtered_scopes {
            for cmd in &commands {
                progress_bar.set_message(format!(
                    "Executing '{}' on scope: {}",
                    cmd.command,
                    scope.path.display()
                ));

                let item_result = execute_command_on_scope(rhema, scope, cmd);
                result.details.push(item_result);

                progress_bar.inc(1);
            }
        }
    }

    progress_bar.finish_with_message("Batch command execution completed");

    // Calculate statistics
    result.total_items = total_operations;
    result.successful = result.details.iter().filter(|d| d.success).count();
    result.failed = result.total_items - result.successful;
    result.processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Print results
    print_batch_results(&result);

    Ok(())
}

/// Batch data import/export
pub fn run_data_operations(
    rhema: &Rhema,
    operation: &str,
    input_path: &str,
    output_path: &str,
    format: &str,
    scope_filter: Option<&str>,
) -> RhemaResult<()> {
    println!("📊 Executing batch data operations...");
    println!("{}", "─".repeat(80));

    let start_time = std::time::Instant::now();
    let mut result = BatchOperationResult {
        operation_type: format!("data_{}", operation),
        total_items: 0,
        successful: 0,
        failed: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        processing_time_ms: 0,
        details: Vec::new(),
    };

    // Discover scopes
    let scopes = rhema.discover_scopes()?;
    let filtered_scopes = if let Some(filter) = scope_filter {
        scopes
            .into_iter()
            .filter(|scope| scope.path.to_string_lossy().contains(filter))
            .collect()
    } else {
        scopes
    };

    if filtered_scopes.is_empty() {
        println!("{}", "No scopes found matching the filter.".yellow());
        return Ok(());
    }

    // Create progress bar
    let progress_bar = ProgressBar::new(filtered_scopes.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    match operation {
        "export" => {
            let mut all_data = HashMap::new();

            for scope in &filtered_scopes {
                progress_bar.set_message(format!(
                    "Exporting data from scope: {}",
                    scope.path.display()
                ));

                let item_result = export_scope_data(rhema, scope, &mut all_data)?;
                result.details.push(item_result);

                progress_bar.inc(1);
            }

            // Write combined data to output file
            write_export_data(&all_data, output_path, format)?;
        }
        "import" => {
            // Read import data
            let import_data = read_import_data(input_path, format)?;

            for scope in &filtered_scopes {
                progress_bar
                    .set_message(format!("Importing data to scope: {}", scope.path.display()));

                let item_result = import_scope_data(rhema, scope, &import_data)?;
                result.details.push(item_result);

                progress_bar.inc(1);
            }
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unknown data operation: {}",
                operation
            )));
        }
    }

    progress_bar.finish_with_message("Batch data operation completed");

    // Calculate statistics
    result.total_items = filtered_scopes.len();
    result.successful = result.details.iter().filter(|d| d.success).count();
    result.failed = result.total_items - result.successful;
    result.processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Print results
    print_batch_results(&result);

    Ok(())
}

/// Batch validation and health checking
pub fn run_validation_operations(
    rhema: &Rhema,
    operation: &str,
    scope_filter: Option<&str>,
    output_file: Option<&str>,
    detailed: bool,
) -> RhemaResult<()> {
    println!("🔍 Executing batch validation operations...");
    println!("{}", "─".repeat(80));

    let start_time = std::time::Instant::now();
    let mut result = BatchOperationResult {
        operation_type: format!("validation_{}", operation),
        total_items: 0,
        successful: 0,
        failed: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        processing_time_ms: 0,
        details: Vec::new(),
    };

    // Discover scopes
    let scopes = rhema.discover_scopes()?;
    let filtered_scopes = if let Some(filter) = scope_filter {
        scopes
            .into_iter()
            .filter(|scope| scope.path.to_string_lossy().contains(filter))
            .collect()
    } else {
        scopes
    };

    if filtered_scopes.is_empty() {
        println!("{}", "No scopes found matching the filter.".yellow());
        return Ok(());
    }

    // Create progress bar
    let progress_bar = ProgressBar::new(filtered_scopes.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Process each scope
    for scope in &filtered_scopes {
        progress_bar.set_message(format!("Validating scope: {}", scope.path.display()));

        let item_result = match operation {
            "validate" => batch_validate_scope_comprehensive(rhema, scope, detailed)?,
            "health-check" => batch_health_check_scope_comprehensive(rhema, scope, detailed)?,
            "schema-check" => batch_schema_check_scope(rhema, scope, detailed)?,
            "dependency-check" => batch_dependency_check_scope(rhema, scope, detailed)?,
            _ => {
                return Err(crate::RhemaError::ConfigError(format!(
                    "Unknown validation operation: {}",
                    operation
                )));
            }
        };

        result.details.push(item_result);
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Batch validation completed");

    // Calculate statistics
    result.total_items = filtered_scopes.len();
    result.successful = result.details.iter().filter(|d| d.success).count();
    result.failed = result.total_items - result.successful;
    result.processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Print results
    print_batch_results(&result);

    // Write detailed report if requested
    if let Some(output_path) = output_file {
        write_validation_report(&result, output_path)?;
        println!("📄 Detailed report written to: {}", output_path.green());
    }

    Ok(())
}

/// Batch reporting and analytics
pub fn run_reporting_operations(
    rhema: &Rhema,
    report_type: &str,
    scope_filter: Option<&str>,
    output_file: &str,
    format: &str,
    include_details: bool,
) -> RhemaResult<()> {
    println!("📈 Generating batch reports...");
    println!("{}", "─".repeat(80));

    let start_time = std::time::Instant::now();
    let mut result = BatchOperationResult {
        operation_type: format!("reporting_{}", report_type),
        total_items: 0,
        successful: 0,
        failed: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        processing_time_ms: 0,
        details: Vec::new(),
    };

    // Discover scopes
    let scopes = rhema.discover_scopes()?;
    let filtered_scopes = if let Some(filter) = scope_filter {
        scopes
            .into_iter()
            .filter(|scope| scope.path.to_string_lossy().contains(filter))
            .collect()
    } else {
        scopes
    };

    if filtered_scopes.is_empty() {
        println!("{}", "No scopes found matching the filter.".yellow());
        return Ok(());
    }

    // Create progress bar
    let progress_bar = ProgressBar::new(filtered_scopes.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Collect data for reporting
    let mut report_data = HashMap::new();

    for scope in &filtered_scopes {
        progress_bar.set_message(format!(
            "Collecting data from scope: {}",
            scope.path.display()
        ));

        let item_result = collect_scope_report_data(rhema, scope, report_type, &mut report_data)?;
        result.details.push(item_result);

        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Data collection completed");

    // Generate report
    println!("📊 Generating {} report...", report_type);
    let report = generate_batch_report(report_type, &report_data, include_details)?;

    // Write report
    write_batch_report(&report, output_file, format)?;

    // Calculate statistics
    result.total_items = filtered_scopes.len();
    result.successful = result.details.iter().filter(|d| d.success).count();
    result.failed = result.total_items - result.successful;
    result.processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Print results
    print_batch_results(&result);

    println!("📄 Report written to: {}", output_file.green());

    Ok(())
}

// Helper functions

fn read_batch_input_file(input_file: &str) -> RhemaResult<HashMap<String, serde_yaml::Value>> {
    let path = Path::new(input_file);
    if !path.exists() {
        return Err(crate::RhemaError::FileNotFound(format!(
            "Batch input file not found: {}",
            input_file
        )));
    }

    let content = fs::read_to_string(path).map_err(|e| crate::RhemaError::IoError(e))?;

    let data: HashMap<String, serde_yaml::Value> =
        serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
            file: input_file.to_string(),
            message: e.to_string(),
        })?;

    Ok(data)
}

fn read_batch_commands_file(command_file: &str) -> RhemaResult<Vec<BatchCommand>> {
    let path = Path::new(command_file);
    if !path.exists() {
        return Err(crate::RhemaError::FileNotFound(format!(
            "Batch command file not found: {}",
            command_file
        )));
    }

    let content = fs::read_to_string(path).map_err(|e| crate::RhemaError::IoError(e))?;

    let commands: Vec<BatchCommand> =
        serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
            file: command_file.to_string(),
            message: e.to_string(),
        })?;

    Ok(commands)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BatchCommand {
    command: String,
    args: Option<HashMap<String, serde_yaml::Value>>,
    description: Option<String>,
}

fn batch_validate_scope(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    _input_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    // Validate scope files
    let scope_path = Path::new(&scope.path);

    let _rhema_file = match find_scope_file(scope_path) {
        Some(file) => file,
        None => {
            return Ok(BatchItemResult {
                item_id: scope.path.to_string_lossy().to_string(),
                success: false,
                error: Some("No scope file (rhema.yaml or scope.yaml) found".to_string()),
                warnings,
                metadata,
            });
        }
    };

    // Validate YAML files
    let yaml_files = [
        "knowledge.yaml",
        "todos.yaml",
        "decisions.yaml",
        "patterns.yaml",
        "conventions.yaml",
    ];
    let mut valid_files = 0;

    for file_name in &yaml_files {
        let file_path = scope_path.join(file_name);
        if file_path.exists() {
            match file_ops::read_yaml_file::<serde_yaml::Value>(&file_path) {
                Ok(_) => valid_files += 1,
                Err(e) => warnings.push(format!("Invalid {}: {}", file_name, e)),
            }
        }
    }

    metadata.insert("valid_files".to_string(), valid_files.to_string());
    metadata.insert("total_files".to_string(), yaml_files.len().to_string());

    let success = warnings.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_migrate_scope(
    rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    input_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Get migration parameters from input data
    let target_version = input_data
        .get("target_version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0.0");

    let backup_files = input_data
        .get("backup_files")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let backup_directory = input_data
        .get("backup_directory")
        .and_then(|v| v.as_str())
        .unwrap_or("./backups");

    // Create backup if requested
    if backup_files {
        let backup_path = Path::new(backup_directory).join(&scope.path);
        if let Err(e) = create_backup(scope_path, &backup_path) {
            warnings.push(format!("Failed to create backup: {}", e));
        } else {
            metadata.insert("backup_created".to_string(), "true".to_string());
            metadata.insert(
                "backup_path".to_string(),
                backup_path.to_string_lossy().to_string(),
            );
        }
    }

    // Perform migration
    match crate::migrate::run(rhema, false, false) {
        Ok(_) => {
            metadata.insert("migration_result".to_string(), "success".to_string());
            metadata.insert("target_version".to_string(), target_version.to_string());
        }
        Err(e) => {
            warnings.push(format!("Migration failed: {}", e));
        }
    }

    let success = warnings.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_migrate_scope_dry_run(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    _input_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let metadata = HashMap::new();

    // Dry run implementation
    warnings.push("Dry run: Migration would be performed".to_string());

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success: true,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_export_scope(
    rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    input_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Get export parameters from input data
    let format = input_data
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("json");

    let output_directory = input_data
        .get("output_directory")
        .and_then(|v| v.as_str())
        .unwrap_or("./exports");

    let include_protocol = input_data
        .get("include_protocol")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let include_knowledge = input_data
        .get("include_knowledge")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let include_todos = input_data
        .get("include_todos")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let include_decisions = input_data
        .get("include_decisions")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let include_patterns = input_data
        .get("include_patterns")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let include_conventions = input_data
        .get("include_conventions")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let summarize = input_data
        .get("summarize")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let ai_agent_format = input_data
        .get("ai_agent_format")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let _compress_output = input_data
        .get("compress_output")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Create output directory
    let output_path = Path::new(output_directory);
    fs::create_dir_all(output_path).map_err(|e| crate::RhemaError::IoError(e))?;

    // Generate output filename
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let scope_name = scope_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    let output_file = output_path.join(format!("{}_{}.{}", scope_name, timestamp, format));

    // Perform export
    match crate::export_context::run(
        rhema,
        format,
        Some(output_file.to_string_lossy().as_ref()),
        None,
        include_protocol,
        include_knowledge,
        include_todos,
        include_decisions,
        include_patterns,
        include_conventions,
        summarize,
        ai_agent_format,
    ) {
        Ok(_) => {
            metadata.insert("export_result".to_string(), "success".to_string());
            metadata.insert(
                "output_file".to_string(),
                output_file.to_string_lossy().to_string(),
            );
            metadata.insert("format".to_string(), format.to_string());
        }
        Err(e) => {
            warnings.push(format!("Export failed: {}", e));
        }
    }

    let success = warnings.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_import_scope(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    _input_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let metadata = HashMap::new();

    // Implementation for batch import
    warnings.push("Import not yet implemented".to_string());

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success: true,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_import_scope_dry_run(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    _input_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let metadata = HashMap::new();

    // Dry run implementation
    warnings.push("Dry run: Import would be performed".to_string());

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success: true,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_health_check_scope(
    rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    input_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Get health check parameters from input data
    let check_dependencies = input_data
        .get("check_dependencies")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let check_file_permissions = input_data
        .get("check_file_permissions")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let check_schema_compliance = input_data
        .get("check_schema_compliance")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let check_data_integrity = input_data
        .get("check_data_integrity")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let _generate_report = input_data
        .get("generate_report")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let _report_format = input_data
        .get("report_format")
        .and_then(|v| v.as_str())
        .unwrap_or("json");

    // Perform health check
    match crate::health::run(rhema, Some(scope.path.to_str().unwrap_or(""))) {
        Ok(_) => {
            metadata.insert("health_check_result".to_string(), "success".to_string());

            // Additional checks based on parameters
            if check_dependencies {
                let dep_result = batch_dependency_check_scope(rhema, scope, false)?;
                metadata.insert(
                    "dependencies_healthy".to_string(),
                    dep_result.success.to_string(),
                );
                if !dep_result.warnings.is_empty() {
                    warnings.extend(dep_result.warnings);
                }
            }

            if check_schema_compliance {
                let schema_result = batch_schema_check_scope(rhema, scope, false)?;
                metadata.insert(
                    "schema_compliant".to_string(),
                    schema_result.success.to_string(),
                );
                if !schema_result.warnings.is_empty() {
                    warnings.extend(schema_result.warnings);
                }
            }

            if check_file_permissions {
                // Check file permissions
                let yaml_files = [
                    "knowledge.yaml",
                    "todos.yaml",
                    "decisions.yaml",
                    "patterns.yaml",
                    "conventions.yaml",
                ];
                for file_name in &yaml_files {
                    let file_path = scope_path.join(file_name);
                    if file_path.exists() {
                        if let Ok(metadata_info) = fs::metadata(&file_path) {
                            if metadata_info.permissions().readonly() {
                                warnings.push(format!("{} is read-only", file_name));
                            }
                        }
                    }
                }
            }

            if check_data_integrity {
                // Check data integrity by validating all YAML files
                let yaml_files = [
                    "knowledge.yaml",
                    "todos.yaml",
                    "decisions.yaml",
                    "patterns.yaml",
                    "conventions.yaml",
                ];
                for file_name in &yaml_files {
                    let file_path = scope_path.join(file_name);
                    if file_path.exists() {
                        if let Err(e) =
                            validate_yaml_file(&file_path, file_name.trim_end_matches(".yaml"))
                        {
                            warnings.push(format!("Data integrity issue in {}: {}", file_name, e));
                        }
                    }
                }
            }
        }
        Err(e) => {
            warnings.push(format!("Health check failed: {}", e));
        }
    }

    let success = warnings.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn execute_command_on_scope(
    rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    cmd: &BatchCommand,
) -> BatchItemResult {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Change to scope directory for command execution
    let original_dir = std::env::current_dir().unwrap_or_default();
    let _ = std::env::set_current_dir(scope_path);

    let result = match cmd.command.as_str() {
        "validate" => {
            // Execute validation command
            let empty_args = HashMap::new();
            let args = cmd.args.as_ref().unwrap_or(&empty_args);
            let recursive = args
                .get("recursive")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            match crate::validate::run(rhema, recursive, false, false) {
                Ok(_) => {
                    metadata.insert("validation_result".to_string(), "success".to_string());
                    true
                }
                Err(e) => {
                    warnings.push(format!("Validation failed: {}", e));
                    false
                }
            }
        }
        "health" => {
            // Execute health check command
            match crate::health::run(rhema, Some(scope.path.to_str().unwrap_or(""))) {
                Ok(_) => {
                    metadata.insert("health_result".to_string(), "success".to_string());
                    true
                }
                Err(e) => {
                    warnings.push(format!("Health check failed: {}", e));
                    false
                }
            }
        }
        "stats" => {
            // Execute stats command
            match crate::stats::run(rhema) {
                Ok(_) => {
                    metadata.insert("stats_result".to_string(), "success".to_string());
                    true
                }
                Err(e) => {
                    warnings.push(format!("Stats generation failed: {}", e));
                    false
                }
            }
        }
        "query" => {
            // Execute query command
            let empty_args = HashMap::new();
            let args = cmd.args.as_ref().unwrap_or(&empty_args);
            let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");

            if query.is_empty() {
                warnings.push("No query specified".to_string());
                false
            } else {
                match crate::query::run_formatted(rhema, query, "yaml") {
                    Ok(_) => {
                        metadata.insert("query_result".to_string(), "success".to_string());
                        true
                    }
                    Err(e) => {
                        warnings.push(format!("Query failed: {}", e));
                        false
                    }
                }
            }
        }
        "search" => {
            // Execute search command
            let empty_args = HashMap::new();
            let args = cmd.args.as_ref().unwrap_or(&empty_args);
            let term = args.get("term").and_then(|v| v.as_str()).unwrap_or("");
            let in_file = args.get("in_file").and_then(|v| v.as_str());
            let regex = args.get("regex").and_then(|v| v.as_bool()).unwrap_or(false);

            if term.is_empty() {
                warnings.push("No search term specified".to_string());
                false
            } else {
                match crate::search::run(rhema, term, in_file, regex) {
                    Ok(_) => {
                        metadata.insert("search_result".to_string(), "success".to_string());
                        true
                    }
                    Err(e) => {
                        warnings.push(format!("Search failed: {}", e));
                        false
                    }
                }
            }
        }
        _ => {
            warnings.push(format!("Unknown command: {}", cmd.command));
            false
        }
    };

    // Restore original directory
    let _ = std::env::set_current_dir(original_dir);

    BatchItemResult {
        item_id: format!("{}:{}", scope.path.display(), cmd.command),
        success: result,
        error: None,
        warnings,
        metadata,
    }
}

fn export_scope_data(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    all_data: &mut HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);
    let mut scope_data = HashMap::new();

    // Export scope file
    if let Some(rhema_file) = find_scope_file(scope_path) {
        let file_name = rhema_file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("scope file");
        match file_ops::read_yaml_file::<serde_yaml::Value>(&rhema_file) {
            Ok(data) => {
                scope_data.insert(file_name.to_string(), data);
            }
            Err(e) => {
                warnings.push(format!("Failed to read {}: {}", file_name, e));
            }
        }
    }

    // Export YAML files
    let yaml_files = [
        "knowledge.yaml",
        "todos.yaml",
        "decisions.yaml",
        "patterns.yaml",
        "conventions.yaml",
    ];
    let mut exported_files = 0;

    for file_name in &yaml_files {
        let file_path = scope_path.join(file_name);
        if file_path.exists() {
            match file_ops::read_yaml_file::<serde_yaml::Value>(&file_path) {
                Ok(data) => {
                    scope_data.insert(file_name.to_string(), data);
                    exported_files += 1;
                }
                Err(e) => {
                    warnings.push(format!("Failed to read {}: {}", file_name, e));
                }
            }
        }
    }

    metadata.insert("exported_files".to_string(), exported_files.to_string());
    metadata.insert("total_files".to_string(), yaml_files.len().to_string());

    // Add scope data to all_data
    all_data.insert(
        scope.path.to_string_lossy().to_string(),
        serde_yaml::Value::Mapping(
            scope_data
                .into_iter()
                .map(|(k, v)| (serde_yaml::Value::String(k), v))
                .collect(),
        ),
    );

    let success = warnings.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn read_import_data(
    input_path: &str,
    format: &str,
) -> RhemaResult<HashMap<String, serde_yaml::Value>> {
    let path = Path::new(input_path);
    if !path.exists() {
        return Err(crate::RhemaError::FileNotFound(format!(
            "Import data file not found: {}",
            input_path
        )));
    }

    let content = fs::read_to_string(path).map_err(|e| crate::RhemaError::IoError(e))?;

    match format {
        "json" => {
            let data: HashMap<String, serde_yaml::Value> = serde_json::from_str(&content)
                .map_err(|e| crate::RhemaError::ConfigError(format!("Invalid JSON: {}", e)))?;
            Ok(data)
        }
        "yaml" | "yml" => {
            let data: HashMap<String, serde_yaml::Value> =
                serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: input_path.to_string(),
                    message: e.to_string(),
                })?;
            Ok(data)
        }
        _ => Err(crate::RhemaError::ConfigError(format!(
            "Unsupported import format: {}",
            format
        ))),
    }
}

fn import_scope_data(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    import_data: &HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Get scope data from import_data
    let scope_data = match import_data.get(scope.path.to_string_lossy().as_ref()) {
        Some(data) => data,
        None => {
            return Ok(BatchItemResult {
                item_id: scope.path.to_string_lossy().to_string(),
                success: false,
                error: Some("No data found for this scope in import file".to_string()),
                warnings,
                metadata,
            });
        }
    };

    // Ensure scope directory exists
    if !scope_path.exists() {
        fs::create_dir_all(scope_path).map_err(|e| crate::RhemaError::IoError(e))?;
    }

    let mut imported_files = 0;

    // Import files from scope data
    if let serde_yaml::Value::Mapping(files) = scope_data {
        for (file_name, file_data) in files {
            if let serde_yaml::Value::String(name) = file_name {
                let file_path = scope_path.join(name);

                // Convert YAML value back to string
                let content = serde_yaml::to_string(file_data).map_err(|e| {
                    crate::RhemaError::ConfigError(format!("Failed to serialize {}: {}", name, e))
                })?;

                // Write file
                fs::write(&file_path, content).map_err(|e| crate::RhemaError::IoError(e))?;

                imported_files += 1;
            }
        }
    }

    metadata.insert("imported_files".to_string(), imported_files.to_string());

    let success = imported_files > 0;

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn write_export_data(
    all_data: &HashMap<String, serde_yaml::Value>,
    output_path: &str,
    format: &str,
) -> RhemaResult<()> {
    let path = Path::new(output_path);

    // Ensure output directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| crate::RhemaError::IoError(e))?;
    }

    let content = match format {
        "json" => serde_json::to_string_pretty(all_data).map_err(|e| {
            crate::RhemaError::ConfigError(format!("Failed to serialize to JSON: {}", e))
        })?,
        "yaml" | "yml" => serde_yaml::to_string(all_data).map_err(|e| {
            crate::RhemaError::ConfigError(format!("Failed to serialize to YAML: {}", e))
        })?,
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unsupported export format: {}",
                format
            )));
        }
    };

    fs::write(path, content).map_err(|e| crate::RhemaError::IoError(e))?;

    Ok(())
}

fn batch_validate_scope_comprehensive(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    detailed: bool,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Validate scope file
    let rhema_file = match find_scope_file(scope_path) {
        Some(file) => file,
        None => {
            return Ok(BatchItemResult {
                item_id: scope.path.to_string_lossy().to_string(),
                success: false,
                error: Some("No scope file (rhema.yaml or scope.yaml) found".to_string()),
                warnings,
                metadata,
            });
        }
    };

    // Validate rhema.yaml content
    match file_ops::read_yaml_file::<rhema_core::schema::RhemaScope>(&rhema_file) {
        Ok(scope_def) => {
            metadata.insert("scope_name".to_string(), scope_def.name);
            metadata.insert("scope_version".to_string(), scope_def.version);
        }
        Err(e) => {
            warnings.push(format!("Invalid rhema.yaml: {}", e));
        }
    }

    // Validate all YAML files
    let yaml_files = [
        ("knowledge.yaml", "knowledge"),
        ("todos.yaml", "todos"),
        ("decisions.yaml", "decisions"),
        ("patterns.yaml", "patterns"),
        ("conventions.yaml", "conventions"),
    ];

    let mut valid_files = 0;
    let mut total_files = 0;

    for (file_name, schema_type) in &yaml_files {
        let file_path = scope_path.join(file_name);
        if file_path.exists() {
            total_files += 1;
            match validate_yaml_file(&file_path, schema_type) {
                Ok(_) => {
                    valid_files += 1;
                    if detailed {
                        metadata.insert(format!("{}_valid", schema_type), "true".to_string());
                    }
                }
                Err(e) => {
                    warnings.push(format!("Invalid {}: {}", file_name, e));
                    if detailed {
                        metadata.insert(format!("{}_valid", schema_type), "false".to_string());
                        metadata.insert(format!("{}_error", schema_type), e.to_string());
                    }
                }
            }
        }
    }

    metadata.insert("valid_files".to_string(), valid_files.to_string());
    metadata.insert("total_files".to_string(), total_files.to_string());

    // Check file permissions
    if detailed {
        for (file_name, _) in &yaml_files {
            let file_path = scope_path.join(file_name);
            if file_path.exists() {
                match fs::metadata(&file_path) {
                    Ok(metadata_info) => {
                        if metadata_info.permissions().readonly() {
                            warnings.push(format!("{} is read-only", file_name));
                        }
                    }
                    Err(_) => {
                        warnings.push(format!("Cannot access {}", file_name));
                    }
                }
            }
        }
    }

    let success = warnings.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_health_check_scope_comprehensive(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    detailed: bool,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Check if scope directory exists and is accessible
    if !scope_path.exists() {
        return Ok(BatchItemResult {
            item_id: scope.path.to_string_lossy().to_string(),
            success: false,
            error: Some("Scope directory does not exist".to_string()),
            warnings,
            metadata,
        });
    }

    // Check scope file
    let rhema_file = match find_scope_file(scope_path) {
        Some(file) => file,
        None => {
            return Ok(BatchItemResult {
                item_id: scope.path.to_string_lossy().to_string(),
                success: false,
                error: Some("No scope file (rhema.yaml or scope.yaml) found".to_string()),
                warnings,
                metadata,
            });
        }
    };

    // Validate rhema.yaml content
    let scope_def = match file_ops::read_yaml_file::<rhema_core::schema::RhemaScope>(&rhema_file) {
        Ok(def) => def,
        Err(e) => {
            return Ok(BatchItemResult {
                item_id: scope.path.to_string_lossy().to_string(),
                success: false,
                error: Some(format!("Invalid rhema.yaml: {}", e)),
                warnings,
                metadata,
            });
        }
    };

    metadata.insert("scope_name".to_string(), scope_def.name.clone());
    metadata.insert("scope_version".to_string(), scope_def.version.clone());

    // Check required YAML files
    let required_files = [
        "knowledge.yaml",
        "todos.yaml",
        "decisions.yaml",
        "patterns.yaml",
    ];
    let mut missing_files = Vec::new();
    let mut file_count = 0;

    for file_name in &required_files {
        let file_path = scope_path.join(file_name);
        if file_path.exists() {
            file_count += 1;

            // Check file permissions
            if let Ok(metadata_info) = fs::metadata(&file_path) {
                if metadata_info.permissions().readonly() {
                    warnings.push(format!("{} is read-only", file_name));
                }
            }

            // Validate file content
            if let Err(e) = validate_yaml_file(&file_path, file_name.trim_end_matches(".yaml")) {
                warnings.push(format!("Invalid {}: {}", file_name, e));
            }
        } else {
            missing_files.push(file_name.to_string());
        }
    }

    metadata.insert("file_count".to_string(), file_count.to_string());
    metadata.insert("missing_files".to_string(), missing_files.len().to_string());

    if !missing_files.is_empty() {
        warnings.push(format!("Missing files: {}", missing_files.join(", ")));
    }

    // Check dependencies if specified
    if let Some(dependencies) = &scope_def.dependencies {
        let mut valid_deps = 0;
        let mut invalid_deps = Vec::new();

        for dep in dependencies {
            let dep_path = Path::new(&dep.path);
            if dep_path.exists() {
                valid_deps += 1;
            } else {
                invalid_deps.push(dep.path.clone());
            }
        }

        metadata.insert("valid_dependencies".to_string(), valid_deps.to_string());
        metadata.insert(
            "total_dependencies".to_string(),
            dependencies.len().to_string(),
        );

        if !invalid_deps.is_empty() {
            warnings.push(format!("Invalid dependencies: {}", invalid_deps.join(", ")));
        }
    }

    // Check for orphaned files
    if detailed {
        let mut orphaned_files = Vec::new();
        for entry in WalkDir::new(scope_path)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_string_lossy();
                if !file_name.ends_with(".yaml") && file_name != "rhema.yaml" {
                    orphaned_files.push(file_name.to_string());
                }
            }
        }

        if !orphaned_files.is_empty() {
            warnings.push(format!("Orphaned files: {}", orphaned_files.join(", ")));
        }
    }

    let success = warnings.is_empty() && missing_files.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_schema_check_scope(
    _rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    detailed: bool,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Check scope file schema
    if let Some(rhema_file) = find_scope_file(scope_path) {
        match file_ops::read_yaml_file::<rhema_core::schema::RhemaScope>(&rhema_file) {
            Ok(scope_def) => {
                metadata.insert(
                    "scope_schema_version".to_string(),
                    scope_def.version.clone(),
                );

                // Check for required fields
                if scope_def.name.is_empty() {
                    warnings.push("Scope name is empty".to_string());
                }
                if scope_def
                    .description
                    .as_ref()
                    .map_or(true, |d| d.is_empty())
                {
                    warnings.push("Scope description is empty".to_string());
                }
            }
            Err(e) => {
                return Ok(BatchItemResult {
                    item_id: scope.path.to_string_lossy().to_string(),
                    success: false,
                    error: Some(format!("Invalid rhema.yaml schema: {}", e)),
                    warnings,
                    metadata,
                });
            }
        }
    }

    // Check schema for each YAML file
    let schema_checks = [
        ("knowledge.yaml", "knowledge"),
        ("todos.yaml", "todos"),
        ("decisions.yaml", "decisions"),
        ("patterns.yaml", "patterns"),
        ("conventions.yaml", "conventions"),
    ];

    let mut schema_errors = 0;
    let mut schema_warnings = 0;

    for (file_name, schema_type) in &schema_checks {
        let file_path = scope_path.join(file_name);
        if file_path.exists() {
            match validate_schema_compliance(&file_path, schema_type) {
                Ok(validation_result) => {
                    if detailed {
                        metadata
                            .insert(format!("{}_schema_valid", schema_type), "true".to_string());
                    }
                    schema_warnings += validation_result.warnings.len();
                }
                Err(e) => {
                    schema_errors += 1;
                    warnings.push(format!("{} schema error: {}", file_name, e));
                    if detailed {
                        metadata
                            .insert(format!("{}_schema_valid", schema_type), "false".to_string());
                        metadata.insert(format!("{}_schema_error", schema_type), e.to_string());
                    }
                }
            }
        }
    }

    metadata.insert("schema_errors".to_string(), schema_errors.to_string());
    metadata.insert("schema_warnings".to_string(), schema_warnings.to_string());

    let success = schema_errors == 0;

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn batch_dependency_check_scope(
    rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    detailed: bool,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let mut metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);

    // Read scope definition
    let rhema_file = match find_scope_file(scope_path) {
        Some(file) => file,
        None => {
            return Ok(BatchItemResult {
                item_id: scope.path.to_string_lossy().to_string(),
                success: false,
                error: Some("No scope file (rhema.yaml or scope.yaml) found".to_string()),
                warnings,
                metadata,
            });
        }
    };

    let scope_def = match file_ops::read_yaml_file::<rhema_core::schema::RhemaScope>(&rhema_file) {
        Ok(def) => def,
        Err(e) => {
            return Ok(BatchItemResult {
                item_id: scope.path.to_string_lossy().to_string(),
                success: false,
                error: Some(format!("Invalid rhema.yaml: {}", e)),
                warnings,
                metadata,
            });
        }
    };

    // Check dependencies
    let mut valid_deps = 0;
    let mut invalid_deps = Vec::new();
    let mut missing_deps = Vec::new();
    let mut circular_deps = Vec::new();

    if let Some(dependencies) = &scope_def.dependencies {
        for dep in dependencies {
            let dep_path = Path::new(&dep.path);

            if !dep_path.exists() {
                missing_deps.push(dep.path.clone());
                continue;
            }

            // Check if dependency has valid rhema.yaml
            let dep_rhema_file = dep_path.join("rhema.yaml");
            if !dep_rhema_file.exists() {
                invalid_deps.push(format!("{}: missing rhema.yaml", dep.path));
                continue;
            }

            // Validate dependency scope
            match file_ops::read_yaml_file::<rhema_core::schema::RhemaScope>(&dep_rhema_file) {
                Ok(dep_def) => {
                    valid_deps += 1;

                    // Check for circular dependencies
                    if let Some(dep_deps) = &dep_def.dependencies {
                        for dep_dep in dep_deps {
                            if dep_dep.path == scope.path.to_string_lossy() {
                                circular_deps.push(format!(
                                    "Circular dependency: {} -> {}",
                                    scope.path.display(),
                                    dep.path
                                ));
                            }
                        }
                    }

                    if detailed {
                        metadata.insert(format!("dep_{}_name", valid_deps), dep_def.name);
                        metadata.insert(format!("dep_{}_version", valid_deps), dep_def.version);
                    }
                }
                Err(e) => {
                    invalid_deps.push(format!("{}: invalid rhema.yaml - {}", dep.path, e));
                }
            }
        }

        metadata.insert("valid_dependencies".to_string(), valid_deps.to_string());
        metadata.insert(
            "total_dependencies".to_string(),
            dependencies.len().to_string(),
        );
        metadata.insert(
            "invalid_dependencies".to_string(),
            invalid_deps.len().to_string(),
        );
        metadata.insert(
            "missing_dependencies".to_string(),
            missing_deps.len().to_string(),
        );
        metadata.insert(
            "circular_dependencies".to_string(),
            circular_deps.len().to_string(),
        );
    }

    // Add warnings for issues
    if !missing_deps.is_empty() {
        warnings.push(format!("Missing dependencies: {}", missing_deps.join(", ")));
    }

    if !invalid_deps.is_empty() {
        warnings.push(format!("Invalid dependencies: {}", invalid_deps.join(", ")));
    }

    if !circular_deps.is_empty() {
        warnings.push(format!(
            "Circular dependencies: {}",
            circular_deps.join(", ")
        ));
    }

    // Check for orphaned scopes (scopes that depend on this one but aren't listed as dependencies)
    if detailed {
        let all_scopes = rhema.discover_scopes()?;
        let mut orphaned_refs = Vec::new();

        for other_scope in &all_scopes {
            if other_scope.path == scope.path {
                continue;
            }

            let other_rhema_file = Path::new(&other_scope.path).join("rhema.yaml");
            if let Ok(other_def) =
                file_ops::read_yaml_file::<rhema_core::schema::RhemaScope>(&other_rhema_file)
            {
                if let Some(other_deps) = &other_def.dependencies {
                    for other_dep in other_deps {
                        if other_dep.path == scope.path.to_string_lossy() {
                            // Check if this scope lists the other scope as a dependency
                            let is_listed = scope_def
                                .dependencies
                                .as_ref()
                                .map(|deps| {
                                    deps.iter()
                                        .any(|d| d.path == other_scope.path.to_string_lossy())
                                })
                                .unwrap_or(false);

                            if !is_listed {
                                orphaned_refs.push(other_scope.path.clone());
                            }
                        }
                    }
                }
            }
        }

        if !orphaned_refs.is_empty() {
            warnings.push(format!(
                "Orphaned references: {}",
                orphaned_refs
                    .iter()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        metadata.insert(
            "orphaned_references".to_string(),
            orphaned_refs.len().to_string(),
        );
    }

    let success = missing_deps.is_empty() && invalid_deps.is_empty() && circular_deps.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn write_validation_report(result: &BatchOperationResult, output_file: &str) -> RhemaResult<()> {
    let path = Path::new(output_file);

    // Ensure output directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| crate::RhemaError::IoError(e))?;
    }

    // Create detailed validation report
    let mut report = String::new();
    report.push_str(&format!("# Rhema Validation Report\n\n"));
    report.push_str(&format!(
        "Generated: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    report.push_str(&format!("Operation Type: {}\n", result.operation_type));
    report.push_str(&format!("Total Items: {}\n", result.total_items));
    report.push_str(&format!("Successful: {}\n", result.successful));
    report.push_str(&format!("Failed: {}\n", result.failed));
    report.push_str(&format!(
        "Processing Time: {}ms\n\n",
        result.processing_time_ms
    ));

    if !result.warnings.is_empty() {
        report.push_str("## Global Warnings\n\n");
        for warning in &result.warnings {
            report.push_str(&format!("- {}\n", warning));
        }
        report.push_str("\n");
    }

    if !result.errors.is_empty() {
        report.push_str("## Global Errors\n\n");
        for error in &result.errors {
            report.push_str(&format!("- {}\n", error));
        }
        report.push_str("\n");
    }

    report.push_str("## Detailed Results\n\n");

    for detail in &result.details {
        report.push_str(&format!("### {}\n", detail.item_id));
        report.push_str(&format!(
            "**Status:** {}\n",
            if detail.success {
                "✅ Success"
            } else {
                "❌ Failed"
            }
        ));

        if let Some(error) = &detail.error {
            report.push_str(&format!("**Error:** {}\n", error));
        }

        if !detail.warnings.is_empty() {
            report.push_str("**Warnings:**\n");
            for warning in &detail.warnings {
                report.push_str(&format!("- {}\n", warning));
            }
        }

        if !detail.metadata.is_empty() {
            report.push_str("**Metadata:**\n");
            for (key, value) in &detail.metadata {
                report.push_str(&format!("- {}: {}\n", key, value));
            }
        }

        report.push_str("\n");
    }

    // Write report
    fs::write(path, report).map_err(|e| crate::RhemaError::IoError(e))?;

    Ok(())
}

fn collect_scope_report_data(
    rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    report_type: &str,
    report_data: &mut HashMap<String, serde_yaml::Value>,
) -> RhemaResult<BatchItemResult> {
    let mut warnings = Vec::new();
    let metadata = HashMap::new();

    let scope_path = Path::new(&scope.path);
    let mut scope_report = HashMap::new();

    // Read scope definition
    if let Some(rhema_file) = find_scope_file(scope_path) {
        if let Ok(scope_def) =
            file_ops::read_yaml_file::<rhema_core::schema::RhemaScope>(&rhema_file)
        {
            scope_report.insert(
                "name".to_string(),
                serde_yaml::Value::String(scope_def.name),
            );
            scope_report.insert(
                "version".to_string(),
                serde_yaml::Value::String(scope_def.version),
            );
            scope_report.insert(
                "description".to_string(),
                serde_yaml::Value::String(scope_def.description.unwrap_or_default()),
            );
        }
    }

    match report_type {
        "summary" => {
            // Collect basic summary data
            let mut file_counts = HashMap::new();
            let yaml_files = [
                "knowledge.yaml",
                "todos.yaml",
                "decisions.yaml",
                "patterns.yaml",
                "conventions.yaml",
            ];

            for file_name in &yaml_files {
                let file_path = scope_path.join(file_name);
                if file_path.exists() {
                    file_counts.insert(file_name.to_string(), true);
                } else {
                    file_counts.insert(file_name.to_string(), false);
                }
            }

            scope_report.insert("files".to_string(), serde_yaml::to_value(file_counts)?);
        }
        "analytics" => {
            // Collect analytics data
            let mut analytics = HashMap::new();

            // Count knowledge entries
            let knowledge_file = scope_path.join("knowledge.yaml");
            if knowledge_file.exists() {
                if let Ok(knowledge) =
                    file_ops::read_yaml_file::<rhema_core::schema::Knowledge>(&knowledge_file)
                {
                    analytics.insert(
                        "knowledge_count".to_string(),
                        serde_yaml::Value::Number(serde_yaml::Number::from(
                            knowledge.entries.len(),
                        )),
                    );
                }
            }

            // Count todos
            let todos_file = scope_path.join("todos.yaml");
            if todos_file.exists() {
                if let Ok(todos) =
                    file_ops::read_yaml_file::<rhema_core::schema::Todos>(&todos_file)
                {
                    analytics.insert(
                        "todos_count".to_string(),
                        serde_yaml::Value::Number(serde_yaml::Number::from(todos.todos.len())),
                    );

                    // Count by status
                    let mut status_counts = HashMap::new();
                    for todo in &todos.todos {
                        let status = format!("{:?}", todo.status);
                        *status_counts.entry(status).or_insert(0) += 1;
                    }
                    analytics.insert(
                        "todo_status_counts".to_string(),
                        serde_yaml::to_value(status_counts)?,
                    );
                }
            }

            // Count decisions
            let decisions_file = scope_path.join("decisions.yaml");
            if decisions_file.exists() {
                if let Ok(decisions) =
                    file_ops::read_yaml_file::<rhema_core::schema::Decisions>(&decisions_file)
                {
                    analytics.insert(
                        "decisions_count".to_string(),
                        serde_yaml::Value::Number(serde_yaml::Number::from(
                            decisions.decisions.len(),
                        )),
                    );
                }
            }

            // Count patterns
            let patterns_file = scope_path.join("patterns.yaml");
            if patterns_file.exists() {
                if let Ok(patterns) =
                    file_ops::read_yaml_file::<rhema_core::schema::Patterns>(&patterns_file)
                {
                    analytics.insert(
                        "patterns_count".to_string(),
                        serde_yaml::Value::Number(serde_yaml::Number::from(
                            patterns.patterns.len(),
                        )),
                    );
                }
            }

            scope_report.insert("analytics".to_string(), serde_yaml::to_value(analytics)?);
        }
        "health" => {
            // Collect health data
            let health_result = batch_health_check_scope_comprehensive(rhema, scope, true)?;
            scope_report.insert(
                "health_status".to_string(),
                serde_yaml::Value::Bool(health_result.success),
            );
            scope_report.insert(
                "health_warnings".to_string(),
                serde_yaml::to_value(health_result.warnings)?,
            );
            scope_report.insert(
                "health_metadata".to_string(),
                serde_yaml::to_value(health_result.metadata)?,
            );
        }
        "dependencies" => {
            // Collect dependency data
            let dep_result = batch_dependency_check_scope(rhema, scope, true)?;
            scope_report.insert(
                "dependency_status".to_string(),
                serde_yaml::Value::Bool(dep_result.success),
            );
            scope_report.insert(
                "dependency_warnings".to_string(),
                serde_yaml::to_value(dep_result.warnings)?,
            );
            scope_report.insert(
                "dependency_metadata".to_string(),
                serde_yaml::to_value(dep_result.metadata)?,
            );
        }
        "todos" => {
            // Collect todos data
            let todos_file = scope_path.join("todos.yaml");
            if todos_file.exists() {
                if let Ok(todos) =
                    file_ops::read_yaml_file::<rhema_core::schema::Todos>(&todos_file)
                {
                    scope_report.insert("todos".to_string(), serde_yaml::to_value(todos)?);
                }
            }
        }
        "knowledge" => {
            // Collect knowledge data
            let knowledge_file = scope_path.join("knowledge.yaml");
            if knowledge_file.exists() {
                if let Ok(knowledge) =
                    file_ops::read_yaml_file::<rhema_core::schema::Knowledge>(&knowledge_file)
                {
                    scope_report.insert("knowledge".to_string(), serde_yaml::to_value(knowledge)?);
                }
            }
        }
        _ => {
            warnings.push(format!("Unknown report type: {}", report_type));
        }
    }

    // Add scope report to report_data
    report_data.insert(
        scope.path.to_string_lossy().to_string(),
        serde_yaml::Value::Mapping(
            scope_report
                .into_iter()
                .map(|(k, v)| (serde_yaml::Value::String(k), v))
                .collect(),
        ),
    );

    let success = warnings.is_empty();

    Ok(BatchItemResult {
        item_id: scope.path.to_string_lossy().to_string(),
        success,
        error: None,
        warnings,
        metadata,
    })
}

fn generate_batch_report(
    report_type: &str,
    report_data: &HashMap<String, serde_yaml::Value>,
    include_details: bool,
) -> RhemaResult<String> {
    let mut report = String::new();

    // Add header
    report.push_str(&format!(
        "# Rhema Batch Report - {}\n\n",
        report_type.to_uppercase()
    ));
    report.push_str(&format!(
        "Generated: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    report.push_str(&format!("Scopes analyzed: {}\n\n", report_data.len()));

    match report_type {
        "summary" => {
            report.push_str("## Summary\n\n");

            let mut total_files = 0;
            let mut scopes_with_files = 0;

            for (scope_path, scope_data) in report_data {
                report.push_str(&format!("### {}\n", scope_path));

                if let serde_yaml::Value::Mapping(data) = scope_data {
                    if let Some(name) = data.get(&serde_yaml::Value::String("name".to_string())) {
                        if let serde_yaml::Value::String(name_str) = name {
                            report.push_str(&format!("**Name:** {}\n", name_str));
                        }
                    }

                    if let Some(version) =
                        data.get(&serde_yaml::Value::String("version".to_string()))
                    {
                        if let serde_yaml::Value::String(version_str) = version {
                            report.push_str(&format!("**Version:** {}\n", version_str));
                        }
                    }

                    if let Some(files) = data.get(&serde_yaml::Value::String("files".to_string())) {
                        if let serde_yaml::Value::Mapping(files_map) = files {
                            let mut file_count = 0;
                            for (file_name, exists) in files_map {
                                if let (
                                    serde_yaml::Value::String(name),
                                    serde_yaml::Value::Bool(exists_bool),
                                ) = (file_name, exists)
                                {
                                    if *exists_bool {
                                        file_count += 1;
                                        report.push_str(&format!("- ✅ {}\n", name));
                                    } else {
                                        report.push_str(&format!("- ❌ {}\n", name));
                                    }
                                }
                            }
                            total_files += file_count;
                            if file_count > 0 {
                                scopes_with_files += 1;
                            }
                        }
                    }
                }
                report.push_str("\n");
            }

            report.push_str(&format!("## Overall Statistics\n\n"));
            report.push_str(&format!("- Total scopes: {}\n", report_data.len()));
            report.push_str(&format!("- Scopes with files: {}\n", scopes_with_files));
            report.push_str(&format!("- Total files: {}\n", total_files));
        }
        "analytics" => {
            report.push_str("## Analytics Report\n\n");

            let mut total_knowledge = 0;
            let mut total_todos = 0;
            let mut total_decisions = 0;
            let mut total_patterns = 0;

            for (scope_path, scope_data) in report_data {
                report.push_str(&format!("### {}\n", scope_path));

                if let serde_yaml::Value::Mapping(data) = scope_data {
                    if let Some(analytics) =
                        data.get(&serde_yaml::Value::String("analytics".to_string()))
                    {
                        if let serde_yaml::Value::Mapping(analytics_map) = analytics {
                            for (key, value) in analytics_map {
                                if let serde_yaml::Value::String(key_str) = key {
                                    match key_str.as_str() {
                                        "knowledge_count" => {
                                            if let serde_yaml::Value::Number(n) = value {
                                                if let Some(count) = n.as_u64() {
                                                    total_knowledge += count;
                                                    report.push_str(&format!(
                                                        "- Knowledge entries: {}\n",
                                                        count
                                                    ));
                                                }
                                            }
                                        }
                                        "todos_count" => {
                                            if let serde_yaml::Value::Number(n) = value {
                                                if let Some(count) = n.as_u64() {
                                                    total_todos += count;
                                                    report
                                                        .push_str(&format!("- Todos: {}\n", count));
                                                }
                                            }
                                        }
                                        "decisions_count" => {
                                            if let serde_yaml::Value::Number(n) = value {
                                                if let Some(count) = n.as_u64() {
                                                    total_decisions += count;
                                                    report.push_str(&format!(
                                                        "- Decisions: {}\n",
                                                        count
                                                    ));
                                                }
                                            }
                                        }
                                        "patterns_count" => {
                                            if let serde_yaml::Value::Number(n) = value {
                                                if let Some(count) = n.as_u64() {
                                                    total_patterns += count;
                                                    report.push_str(&format!(
                                                        "- Patterns: {}\n",
                                                        count
                                                    ));
                                                }
                                            }
                                        }
                                        _ => {
                                            if include_details {
                                                report.push_str(&format!(
                                                    "- {}: {:?}\n",
                                                    key_str, value
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                report.push_str("\n");
            }

            report.push_str(&format!("## Overall Analytics\n\n"));
            report.push_str(&format!("- Total knowledge entries: {}\n", total_knowledge));
            report.push_str(&format!("- Total todos: {}\n", total_todos));
            report.push_str(&format!("- Total decisions: {}\n", total_decisions));
            report.push_str(&format!("- Total patterns: {}\n", total_patterns));
        }
        "health" => {
            report.push_str("## Health Report\n\n");

            let mut healthy_scopes = 0;
            let mut unhealthy_scopes = 0;

            for (scope_path, scope_data) in report_data {
                report.push_str(&format!("### {}\n", scope_path));

                if let serde_yaml::Value::Mapping(data) = scope_data {
                    if let Some(health_status) =
                        data.get(&serde_yaml::Value::String("health_status".to_string()))
                    {
                        if let serde_yaml::Value::Bool(status) = health_status {
                            if *status {
                                healthy_scopes += 1;
                                report.push_str("**Status:** ✅ Healthy\n");
                            } else {
                                unhealthy_scopes += 1;
                                report.push_str("**Status:** ❌ Unhealthy\n");
                            }
                        }
                    }

                    if let Some(warnings) =
                        data.get(&serde_yaml::Value::String("health_warnings".to_string()))
                    {
                        if let serde_yaml::Value::Sequence(warnings_list) = warnings {
                            if !warnings_list.is_empty() {
                                report.push_str("**Warnings:**\n");
                                for warning in warnings_list {
                                    if let serde_yaml::Value::String(warning_str) = warning {
                                        report.push_str(&format!("- {}\n", warning_str));
                                    }
                                }
                            }
                        }
                    }
                }
                report.push_str("\n");
            }

            report.push_str(&format!("## Overall Health\n\n"));
            report.push_str(&format!("- Healthy scopes: {}\n", healthy_scopes));
            report.push_str(&format!("- Unhealthy scopes: {}\n", unhealthy_scopes));
            report.push_str(&format!(
                "- Health rate: {:.1}%\n",
                if report_data.is_empty() {
                    0.0
                } else {
                    (healthy_scopes as f64 / report_data.len() as f64) * 100.0
                }
            ));
        }
        _ => {
            report.push_str(&format!("## {} Report\n\n", report_type.to_uppercase()));
            report.push_str("Detailed data available in the output file.\n");
        }
    }

    Ok(report)
}

fn write_batch_report(report: &str, output_file: &str, format: &str) -> RhemaResult<()> {
    let path = Path::new(output_file);

    // Ensure output directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| crate::RhemaError::IoError(e))?;
    }

    match format {
        "markdown" | "md" => {
            // Write as markdown
            fs::write(path, report).map_err(|e| crate::RhemaError::IoError(e))?;
        }
        "html" => {
            // Convert markdown to HTML (simple conversion)
            let html_content = convert_markdown_to_html(report);
            fs::write(path, html_content).map_err(|e| crate::RhemaError::IoError(e))?;
        }
        "json" => {
            // Convert report to JSON structure
            let json_content = convert_report_to_json(report);
            fs::write(path, json_content).map_err(|e| crate::RhemaError::IoError(e))?;
        }
        "yaml" | "yml" => {
            // Convert report to YAML structure
            let yaml_content = convert_report_to_yaml(report);
            fs::write(path, yaml_content).map_err(|e| crate::RhemaError::IoError(e))?;
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unsupported report format: {}",
                format
            )));
        }
    }

    Ok(())
}

fn convert_markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<title>Rhema Batch Report</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: Arial, sans-serif; margin: 40px; }\n");
    html.push_str("h1, h2, h3 { color: #333; }\n");
    html.push_str("code { background-color: #f4f4f4; padding: 2px 4px; border-radius: 3px; }\n");
    html.push_str(
        "pre { background-color: #f4f4f4; padding: 10px; border-radius: 5px; overflow-x: auto; }\n",
    );
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    // Simple markdown to HTML conversion
    let mut in_code_block = false;
    for line in markdown.lines() {
        if line.starts_with("```") {
            if in_code_block {
                html.push_str("</pre>\n");
                in_code_block = false;
            } else {
                html.push_str("<pre><code>\n");
                in_code_block = true;
            }
        } else if in_code_block {
            html.push_str(&format!("{}\n", line));
        } else if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
        } else if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", &line[4..]));
        } else if line.starts_with("- ") {
            html.push_str(&format!("<li>{}</li>\n", &line[2..]));
        } else if line.starts_with("**") && line.ends_with("**") {
            html.push_str(&format!(
                "<strong>{}</strong><br>\n",
                &line[2..line.len() - 2]
            ));
        } else if !line.is_empty() {
            html.push_str(&format!("<p>{}</p>\n", line));
        } else {
            html.push_str("<br>\n");
        }
    }

    html.push_str("</body>\n</html>");
    html
}

fn convert_report_to_json(_report: &str) -> String {
    // Simple JSON conversion - in a real implementation, you'd parse the markdown
    // and create a proper JSON structure
    r#"{
  "report_type": "batch_report",
  "generated_at": "2024-01-01T00:00:00Z",
  "content": "Report content would be parsed and structured here"
}"#
    .to_string()
}

fn convert_report_to_yaml(_report: &str) -> String {
    // Simple YAML conversion - in a real implementation, you'd parse the markdown
    // and create a proper YAML structure
    r#"report_type: batch_report
generated_at: "2024-01-01T00:00:00Z"
content: "Report content would be parsed and structured here"
"#
    .to_string()
}

fn create_backup(source_path: &Path, backup_path: &Path) -> RhemaResult<()> {
    // Ensure backup directory exists
    if let Some(parent) = backup_path.parent() {
        fs::create_dir_all(parent).map_err(|e| crate::RhemaError::IoError(e))?;
    }

    // Copy directory recursively
    if source_path.is_dir() {
        copy_dir_recursive(source_path, backup_path)?;
    } else {
        fs::copy(source_path, backup_path).map_err(|e| crate::RhemaError::IoError(e))?;
    }

    Ok(())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> RhemaResult<()> {
    if !source.is_dir() {
        return Err(crate::RhemaError::ConfigError(
            "Source is not a directory".to_string(),
        ));
    }

    // Create destination directory
    fs::create_dir_all(destination).map_err(|e| crate::RhemaError::IoError(e))?;

    // Copy all files and subdirectories
    for entry in WalkDir::new(source)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let source_file = entry.path();
        let relative_path = source_file.strip_prefix(source).map_err(|e| {
            crate::RhemaError::ConfigError(format!("Failed to get relative path: {}", e))
        })?;
        let dest_file = destination.join(relative_path);

        if source_file.is_dir() {
            fs::create_dir_all(&dest_file).map_err(|e| crate::RhemaError::IoError(e))?;
        } else {
            fs::copy(source_file, &dest_file).map_err(|e| crate::RhemaError::IoError(e))?;
        }
    }

    Ok(())
}

fn validate_yaml_file(file_path: &Path, schema_type: &str) -> RhemaResult<()> {
    match schema_type {
        "knowledge" => {
            let _: rhema_core::schema::Knowledge = file_ops::read_yaml_file(file_path)?;
        }
        "todos" => {
            let _: rhema_core::schema::Todos = file_ops::read_yaml_file(file_path)?;
        }
        "decisions" => {
            let _: rhema_core::schema::Decisions = file_ops::read_yaml_file(file_path)?;
        }
        "patterns" => {
            let _: rhema_core::schema::Patterns = file_ops::read_yaml_file(file_path)?;
        }
        "conventions" => {
            let _: rhema_core::schema::Conventions = file_ops::read_yaml_file(file_path)?;
        }
        _ => {
            // Generic YAML validation
            let _: serde_yaml::Value = file_ops::read_yaml_file(file_path)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct SchemaValidationResult {
    warnings: Vec<String>,
}

fn validate_schema_compliance(
    file_path: &Path,
    schema_type: &str,
) -> RhemaResult<SchemaValidationResult> {
    let mut warnings = Vec::new();

    match schema_type {
        "knowledge" => {
            let knowledge: rhema_core::schema::Knowledge = file_ops::read_yaml_file(file_path)?;

            // Check for empty entries
            if knowledge.entries.is_empty() {
                warnings.push("Knowledge file has no entries".to_string());
            }

            // Check entry validity
            for (i, entry) in knowledge.entries.iter().enumerate() {
                if entry.title.is_empty() {
                    warnings.push(format!("Knowledge entry {} has empty title", i));
                }
                if entry.content.is_empty() {
                    warnings.push(format!("Knowledge entry {} has empty content", i));
                }
                if let Some(confidence) = entry.confidence {
                    if confidence > 100 {
                        warnings.push(format!(
                            "Knowledge entry {} has invalid confidence value: {}",
                            i, confidence
                        ));
                    }
                }
            }
        }
        "todos" => {
            let todos: rhema_core::schema::Todos = file_ops::read_yaml_file(file_path)?;

            // Check for empty todos
            if todos.todos.is_empty() {
                warnings.push("Todos file has no entries".to_string());
            }

            // Check todo validity
            for (i, todo) in todos.todos.iter().enumerate() {
                if todo.title.is_empty() {
                    warnings.push(format!("Todo {} has empty title", i));
                }
                if todo.id.is_empty() {
                    warnings.push(format!("Todo {} has empty ID", i));
                }
            }
        }
        "decisions" => {
            let decisions: rhema_core::schema::Decisions = file_ops::read_yaml_file(file_path)?;

            // Check for empty decisions
            if decisions.decisions.is_empty() {
                warnings.push("Decisions file has no entries".to_string());
            }

            // Check decision validity
            for (i, decision) in decisions.decisions.iter().enumerate() {
                if decision.title.is_empty() {
                    warnings.push(format!("Decision {} has empty title", i));
                }
                if decision.description.is_empty() {
                    warnings.push(format!("Decision {} has empty description", i));
                }
                if decision.id.is_empty() {
                    warnings.push(format!("Decision {} has empty ID", i));
                }
            }
        }
        "patterns" => {
            let patterns: rhema_core::schema::Patterns = file_ops::read_yaml_file(file_path)?;

            // Check for empty patterns
            if patterns.patterns.is_empty() {
                warnings.push("Patterns file has no entries".to_string());
            }

            // Check pattern validity
            for (i, pattern) in patterns.patterns.iter().enumerate() {
                if pattern.name.is_empty() {
                    warnings.push(format!("Pattern {} has empty name", i));
                }
                if pattern.description.is_empty() {
                    warnings.push(format!("Pattern {} has empty description", i));
                }
                if pattern.id.is_empty() {
                    warnings.push(format!("Pattern {} has empty ID", i));
                }
            }
        }
        _ => {
            // Generic validation
            let _: serde_yaml::Value = file_ops::read_yaml_file(file_path)?;
        }
    }

    Ok(SchemaValidationResult { warnings })
}

fn print_batch_results(result: &BatchOperationResult) {
    println!("\n📊 Batch Operation Results:");
    println!("{}", "─".repeat(80));
    println!("Operation Type: {}", result.operation_type.blue());
    println!("Total Items: {}", result.total_items);
    println!("Successful: {}", result.successful.to_string().green());
    println!("Failed: {}", result.failed.to_string().red());
    println!("Processing Time: {}ms", result.processing_time_ms);

    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("  - {}", warning.yellow());
        }
    }

    if !result.errors.is_empty() {
        println!("\n❌ Errors:");
        for error in &result.errors {
            println!("  - {}", error.red());
        }
    }

    if result.failed > 0 {
        println!("\n🔍 Failed Items:");
        for detail in &result.details {
            if !detail.success {
                println!(
                    "  - {}: {}",
                    detail.item_id,
                    detail
                        .error
                        .as_ref()
                        .unwrap_or(&"Unknown error".to_string())
                );
            }
        }
    }

    println!("{}", "─".repeat(80));
}

/// Find the scope file in the given directory, checking multiple possible locations
fn find_scope_file(scope_path: &std::path::Path) -> Option<std::path::PathBuf> {
    // Define the possible locations in order of preference
    let possible_locations = [scope_path.join("rhema.yaml"), scope_path.join("scope.yaml")];

    // Check if we're in a .rhema directory, then also check parent directory
    let parent_locations = if scope_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
        let parent = scope_path.parent().unwrap_or(scope_path);
        vec![parent.join("rhema.yaml"), parent.join("scope.yaml")]
    } else {
        vec![]
    };

    // Combine all possible locations
    let all_locations = [&possible_locations[..], &parent_locations[..]].concat();

    // Find the first existing file
    for location in all_locations {
        if location.exists() {
            return Some(location);
        }
    }

    None
}
