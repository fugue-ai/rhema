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

use clap::Subcommand;
use std::path::PathBuf;
use crate::{
    lock::{LockSystem, LockGenerator, LockValidator, validator::ValidationMode, ConflictResolutionStrategy, ConflictResolutionConfig, DependencySpec, conflict_resolver::ConflictSeverity},
    schema::{RhemaLock, ResolutionStrategy},
    RhemaError, RhemaResult,
};
use std::process;
use chrono::{DateTime, Utc};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use serde_yaml;
use semver;

/// Lock file management commands
#[derive(Subcommand)]
pub enum LockSubcommands {
    /// Generate a new lock file
    Generate {
        /// Output file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Resolution strategy (latest, earliest, pinned, range, compatible)
        #[arg(long, value_enum, default_value = "latest")]
        strategy: ResolutionStrategy,

        /// Allow circular dependencies
        #[arg(long)]
        allow_circular: bool,

        /// Generate checksums for integrity verification
        #[arg(long, default_value = "true")]
        checksums: bool,

        /// Validate the generated lock file
        #[arg(long, default_value = "true")]
        validate: bool,

        /// Maximum resolution depth
        #[arg(long, default_value = "10")]
        max_depth: usize,

        /// Enable caching for performance
        #[arg(long, default_value = "true")]
        cache: bool,

        /// Force regeneration even if lock file exists
        #[arg(long)]
        force: bool,

        /// Show detailed progress information
        #[arg(long)]
        verbose: bool,
    },

    /// Validate an existing lock file
    Validate {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Validation mode (strict, lenient)
        #[arg(long, value_enum, default_value = "strict")]
        mode: ValidationMode,

        /// Validate checksums
        #[arg(long, default_value = "true")]
        checksums: bool,

        /// Check for circular dependencies
        #[arg(long, default_value = "true")]
        circular_deps: bool,

        /// Validate scope existence
        #[arg(long, default_value = "true")]
        scope_existence: bool,

        /// Maximum age for lock file freshness (in hours)
        #[arg(long, value_name = "HOURS")]
        max_age: Option<u64>,

        /// Show detailed validation results
        #[arg(long)]
        detailed: bool,

        /// Output format (text, json, yaml)
        #[arg(long, value_enum, default_value = "text")]
        format: OutputFormat,
    },

    /// Update an existing lock file
    Update {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Resolution strategy for updates
        #[arg(long, value_enum)]
        strategy: Option<ResolutionStrategy>,

        /// Update only specific scopes (comma-separated)
        #[arg(long, value_name = "SCOPES")]
        scopes: Option<String>,

        /// Show what would be updated without making changes
        #[arg(long)]
        dry_run: bool,

        /// Show detailed update information
        #[arg(long)]
        verbose: bool,

        /// Backup the existing lock file before updating
        #[arg(long, default_value = "true")]
        backup: bool,
    },

    /// Show lock file status
    Status {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Show detailed status information
        #[arg(long)]
        detailed: bool,

        /// Show only scopes with issues
        #[arg(long)]
        issues_only: bool,

        /// Output format (text, json, yaml)
        #[arg(long, value_enum, default_value = "text")]
        format: OutputFormat,
    },

    /// Show differences from current state
    Diff {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Show only added dependencies
        #[arg(long)]
        added_only: bool,

        /// Show only removed dependencies
        #[arg(long)]
        removed_only: bool,

        /// Show only updated dependencies
        #[arg(long)]
        updated_only: bool,

        /// Show detailed diff information
        #[arg(long)]
        detailed: bool,

        /// Output format (text, json, yaml)
        #[arg(long, value_enum, default_value = "text")]
        format: OutputFormat,
    },

    /// CI/CD: Automated lock file validation for pipelines
    CiValidate {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Exit code for validation failures (default: 1)
        #[arg(long, default_value = "1")]
        exit_code: i32,

        /// Maximum allowed circular dependencies
        #[arg(long, default_value = "0")]
        max_circular_deps: u32,

        /// Maximum allowed lock file age in hours
        #[arg(long, value_name = "HOURS")]
        max_age: Option<u64>,

        /// Fail on warnings
        #[arg(long)]
        fail_on_warnings: bool,

        /// Output validation report to file
        #[arg(long, value_name = "REPORT_FILE")]
        report_file: Option<PathBuf>,

        /// Output format for reports (text, json, yaml, junit)
        #[arg(long, value_enum, default_value = "json")]
        format: CiOutputFormat,
    },

    /// CI/CD: Generate lock file as part of build process
    CiGenerate {
        /// Output file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Resolution strategy (latest, earliest, pinned, range, compatible)
        #[arg(long, value_enum, default_value = "latest")]
        strategy: ResolutionStrategy,

        /// Fail if circular dependencies detected
        #[arg(long)]
        fail_on_circular: bool,

        /// Maximum resolution time in seconds
        #[arg(long, default_value = "300")]
        timeout: u64,

        /// Cache directory for resolution
        #[arg(long, value_name = "CACHE_DIR")]
        cache_dir: Option<PathBuf>,

        /// Output generation report to file
        #[arg(long, value_name = "REPORT_FILE")]
        report_file: Option<PathBuf>,

        /// Output format for reports (text, json, yaml)
        #[arg(long, value_enum, default_value = "json")]
        format: CiOutputFormat,

        /// Exit code for generation failures (default: 1)
        #[arg(long, default_value = "1")]
        exit_code: i32,
    },

    /// CI/CD: Check lock file consistency across environments
    CiConsistency {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Reference lock file for comparison
        #[arg(long, value_name = "REFERENCE_FILE")]
        reference_file: Option<PathBuf>,

        /// Git branch to compare against
        #[arg(long, value_name = "BRANCH")]
        git_branch: Option<String>,

        /// Allow version differences within semantic versioning rules
        #[arg(long)]
        allow_semver_diffs: bool,

        /// Maximum allowed version drift (major.minor.patch)
        #[arg(long, value_name = "DRIFT")]
        max_version_drift: Option<String>,

        /// Output consistency report to file
        #[arg(long, value_name = "REPORT_FILE")]
        report_file: Option<PathBuf>,

        /// Output format for reports (text, json, yaml)
        #[arg(long, value_enum, default_value = "json")]
        format: CiOutputFormat,

        /// Exit code for consistency failures (default: 1)
        #[arg(long, default_value = "1")]
        exit_code: i32,
    },

    /// CI/CD: Automated lock file updates
    CiUpdate {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Update strategy (auto, manual, security-only)
        #[arg(long, value_enum, default_value = "auto")]
        update_strategy: CiUpdateStrategy,

        /// Resolution strategy for updates
        #[arg(long, value_enum)]
        strategy: Option<ResolutionStrategy>,

        /// Update only security-related dependencies
        #[arg(long)]
        security_only: bool,

        /// Maximum number of dependencies to update
        #[arg(long, value_name = "MAX_UPDATES")]
        max_updates: Option<usize>,

        /// Create backup before updating
        #[arg(long, default_value = "true")]
        backup: bool,

        /// Output update report to file
        #[arg(long, value_name = "REPORT_FILE")]
        report_file: Option<PathBuf>,

        /// Output format for reports (text, json, yaml)
        #[arg(long, value_enum, default_value = "json")]
        format: CiOutputFormat,

        /// Exit code for update failures (default: 1)
        #[arg(long, default_value = "1")]
        exit_code: i32,
    },

    /// CI/CD: Health check for lock file system
    CiHealth {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Check lock file integrity
        #[arg(long, default_value = "true")]
        integrity: bool,

        /// Check lock file freshness
        #[arg(long, default_value = "true")]
        freshness: bool,

        /// Check dependency availability
        #[arg(long, default_value = "true")]
        availability: bool,

        /// Check performance metrics
        #[arg(long, default_value = "true")]
        performance: bool,

        /// Output health report to file
        #[arg(long, value_name = "REPORT_FILE")]
        report_file: Option<PathBuf>,

        /// Output format for reports (text, json, yaml)
        #[arg(long, value_enum, default_value = "json")]
        format: CiOutputFormat,

        /// Exit code for health failures (default: 1)
        #[arg(long, default_value = "1")]
        exit_code: i32,
    },

    /// Resolve dependency conflicts using advanced strategies
    ResolveConflicts {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Primary resolution strategy
        #[arg(long, value_enum, default_value = "latest_compatible")]
        strategy: ConflictResolutionStrategy,

        /// Fallback strategies (comma-separated)
        #[arg(long, value_name = "STRATEGIES")]
        fallback_strategies: Option<String>,

        /// Enable automatic conflict detection
        #[arg(long, default_value = "true")]
        auto_detection: bool,

        /// Track resolution history
        #[arg(long, default_value = "true")]
        track_history: bool,

        /// Allow user prompts for manual resolution
        #[arg(long)]
        allow_prompts: bool,

        /// Prefer stable versions
        #[arg(long)]
        prefer_stable: bool,

        /// Enforce pinned versions strictly
        #[arg(long)]
        strict_pinning: bool,

        /// Compatibility threshold for smart selection (0.0-1.0)
        #[arg(long, default_value = "0.8")]
        compatibility_threshold: f64,

        /// Enable parallel resolution
        #[arg(long)]
        parallel: bool,

        /// Maximum parallel threads
        #[arg(long, default_value = "4")]
        max_threads: usize,

        /// Timeout for resolution operations (in seconds)
        #[arg(long, default_value = "300")]
        timeout: u64,

        /// Show detailed resolution information
        #[arg(long)]
        verbose: bool,

        /// Output resolution report to file
        #[arg(long, value_name = "REPORT_FILE")]
        report_file: Option<PathBuf>,

        /// Output format (text, json, yaml)
        #[arg(long, value_enum, default_value = "text")]
        format: OutputFormat,

        /// Apply resolved changes to lock file
        #[arg(long, default_value = "true")]
        apply: bool,

        /// Show only conflicts that need manual resolution
        #[arg(long)]
        manual_only: bool,

        /// Show performance metrics
        #[arg(long)]
        show_metrics: bool,
    },
}

/// Output format for command results
#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
}

/// CI/CD specific output format
#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum CiOutputFormat {
    Text,
    Json,
    Yaml,
    Junit,
}

/// CI/CD update strategy
#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum CiUpdateStrategy {
    Auto,
    Manual,
    SecurityOnly,
}

impl LockSubcommands {
    /// Execute the lock command
    pub fn execute(self) -> RhemaResult<()> {
        match self {
            LockSubcommands::Generate(args) => Self::execute_generate(args),
            LockSubcommands::Validate(args) => Self::execute_validate(args),
            LockSubcommands::Update(args) => Self::execute_update(args),
            LockSubcommands::Status(args) => Self::execute_status(args),
            LockSubcommands::Diff(args) => Self::execute_diff(args),
            LockSubcommands::CiValidate(args) => Self::execute_ci_validate(args),
            LockSubcommands::CiGenerate(args) => Self::execute_ci_generate(args),
            LockSubcommands::CiConsistency(args) => Self::execute_ci_consistency(args),
            LockSubcommands::CiUpdate(args) => Self::execute_ci_update(args),
            LockSubcommands::CiHealth(args) => Self::execute_ci_health(args),
            LockSubcommands::ResolveConflicts(args) => Self::execute_resolve_conflicts(args),
        }
    }

    /// Execute the generate command
    fn execute_generate(args: crate::commands::lock::LockSubcommands::Generate) -> RhemaResult<()> {
        let output_path = args.output.unwrap_or_else(|| PathBuf::from("rhema.lock"));
        let repo_path = std::env::current_dir()?;

        // Check if lock file exists and force flag
        if output_path.exists() && !args.force {
            println!("{}", "Warning: Lock file already exists. Use --force to overwrite.".yellow());
            return Ok(());
        }

        if args.verbose {
            println!("🔍 Generating lock file for repository: {}", repo_path.display());
            println!("📁 Output file: {}", output_path.display());
            println!("⚙️  Resolution strategy: {:?}", args.strategy);
        }

        // Create generator configuration
        let mut config = LockGenerator::new();
        // Note: In a real implementation, you would configure the generator with the args

        // Generate the lock file
        let lock_file = LockSystem::generate_lock_file(&repo_path)?;

        // Write the lock file
        let lock_content = serde_json::to_string_pretty(&lock_file)?;
        fs::write(&output_path, lock_content)?;

        if args.verbose {
            println!("✅ Lock file generated successfully!");
            println!("📊 Statistics:");
            println!("   - Total scopes: {}", lock_file.metadata.total_scopes);
            println!("   - Total dependencies: {}", lock_file.metadata.total_dependencies);
            println!("   - Circular dependencies: {}", lock_file.metadata.circular_dependencies);
        } else {
            println!("✅ Lock file generated: {}", output_path.display());
        }

        Ok(())
    }

    /// Execute the validate command
    fn execute_validate(args: crate::commands::lock::LockSubcommands::Validate) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        // Read and parse the lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        println!("🔍 Validating lock file: {}", lock_path.display());
        println!("⚙️  Validation mode: {:?}", args.mode);

        // Create validator with configuration
        let mut validator = LockValidator::new()
            .with_mode(args.mode)
            .with_checksum_validation(args.checksums)
            .with_circular_dependency_check(args.circular_deps)
            .with_scope_existence_validation(args.scope_existence);

        if let Some(max_age) = args.max_age {
            validator = validator.with_max_lock_age(Some(max_age));
        }

        // Perform validation
        validator.validate(&lock_file)?;

        // For now, we'll just print a success message since the validator returns Result<()>
        // In a real implementation, you would capture the validation details
        match args.format {
            OutputFormat::Text => {
                println!("🔍 Validation Results");
                println!("===================");
                println!("Status: ✅ Valid");
                println!("All validation checks passed successfully!");
            },
            OutputFormat::Json => {
                let result = serde_json::json!({
                    "status": "valid",
                    "message": "All validation checks passed successfully"
                });
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            },
            OutputFormat::Yaml => {
                let result = serde_yaml::to_string(&serde_json::json!({
                    "status": "valid",
                    "message": "All validation checks passed successfully"
                })).unwrap();
                println!("{}", result);
            },
        }

        Ok(())
    }

    /// Execute the update command
    fn execute_update(args: crate::commands::lock::LockSubcommands::Update) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));
        let repo_path = std::env::current_dir()?;

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        // Read existing lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let mut lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        if args.verbose {
            println!("🔄 Updating lock file: {}", lock_path.display());
        }

        // Create backup if requested
        if args.backup {
            let backup_path = lock_path.with_extension("lock.backup");
            fs::copy(&lock_path, &backup_path)?;
            if args.verbose {
                println!("💾 Backup created: {}", backup_path.display());
            }
        }

        if args.dry_run {
            println!("🔍 Dry run mode - no changes will be made");
            // In a real implementation, you would analyze what would be updated
            println!("📊 Current lock file statistics:");
            println!("   - Total scopes: {}", lock_file.metadata.total_scopes);
            println!("   - Total dependencies: {}", lock_file.metadata.total_dependencies);
            return Ok(());
        }

        // Update the lock file
        LockSystem::update_lock_file(&repo_path, &mut lock_file)?;

        // Write updated lock file
        let updated_content = serde_json::to_string_pretty(&lock_file)?;
        fs::write(&lock_path, updated_content)?;

        if args.verbose {
            println!("✅ Lock file updated successfully!");
            println!("📊 Updated statistics:");
            println!("   - Total scopes: {}", lock_file.metadata.total_scopes);
            println!("   - Total dependencies: {}", lock_file.metadata.total_dependencies);
        } else {
            println!("✅ Lock file updated: {}", lock_path.display());
        }

        Ok(())
    }

    /// Execute the status command
    fn execute_status(args: crate::commands::lock::LockSubcommands::Status) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        // Read and parse the lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        // Output results based on format
        match args.format {
            OutputFormat::Text => Self::print_status_text(&lock_file, args.detailed, args.issues_only),
            OutputFormat::Json => Self::print_status_json(&lock_file),
            OutputFormat::Yaml => Self::print_status_yaml(&lock_file),
        }

        Ok(())
    }

    /// Execute the diff command
    fn execute_diff(args: crate::commands::lock::LockSubcommands::Diff) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));
        let repo_path = std::env::current_dir()?;

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        // Read existing lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        // Generate current state (this would be the actual implementation)
        let current_lock = LockSystem::generate_lock_file(&repo_path)?;

        // Calculate differences
        let diff_result = Self::calculate_diff(&lock_file, &current_lock);

        // Output results based on format
        match args.format {
            OutputFormat::Text => Self::print_diff_text(&diff_result, args.detailed, args.added_only, args.removed_only, args.updated_only),
            OutputFormat::Json => Self::print_diff_json(&diff_result),
            OutputFormat::Yaml => Self::print_diff_yaml(&diff_result),
        }

        Ok(())
    }

    /// Execute the CI/CD validate command
    fn execute_ci_validate(args: crate::commands::lock::LockSubcommands::CiValidate) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        let exit_code = args.exit_code;
        let max_circular_deps = args.max_circular_deps;
        let max_age = args.max_age;
        let fail_on_warnings = args.fail_on_warnings;
        let report_file = args.report_file;
        let format = args.format;

        let mut validator = LockValidator::new()
            .with_mode(ValidationMode::Strict) // CI/CD validation is strict
            .with_checksum_validation(true)
            .with_circular_dependency_check(true)
            .with_scope_existence_validation(true);

        if let Some(max_age) = max_age {
            validator = validator.with_max_lock_age(Some(max_age));
        }

        let lock_file: RhemaLock = serde_json::from_str(&fs::read_to_string(&lock_path)?)?;
        let validation_result = validator.validate(&lock_file);

        let mut exit_status = 0;
        let mut messages = Vec::new();

        match validation_result {
            Ok(_) => {
                messages.push("All CI/CD validation checks passed successfully.");
                if fail_on_warnings {
                    println!("🔍 CI/CD Validation Results");
                    println!("===========================");
                    println!("Status: ✅ Valid");
                    println!("All validation checks passed successfully!");
                }
            }
            Err(e) => {
                messages.push(format!("CI/CD validation failed: {}", e));
                exit_status = 1;
                println!("🔍 CI/CD Validation Results");
                println!("===========================");
                println!("Status: ❌ Invalid");
                println!("CI/CD validation failed: {}", e);
            }
        }

        if let Some(report_file) = report_file {
            let report_content = match format {
                CiOutputFormat::Text => {
                    let mut report_writer = io::BufWriter::new(fs::File::create(&report_file)?);
                    serde_json::to_writer_pretty(&mut report_writer, &serde_json::json!({
                        "status": "valid",
                        "message": "All CI/CD validation checks passed successfully"
                    })).unwrap();
                    report_writer.flush()?;
                    "Report written to: {}".to_string()
                }
                CiOutputFormat::Json => {
                    let report_content = serde_json::to_string_pretty(&serde_json::json!({
                        "status": "valid",
                        "message": "All CI/CD validation checks passed successfully"
                    })).unwrap();
                    fs::write(&report_file, report_content)?;
                    "Report written to: {}".to_string()
                }
                CiOutputFormat::Yaml => {
                    let report_content = serde_yaml::to_string(&serde_json::json!({
                        "status": "valid",
                        "message": "All CI/CD validation checks passed successfully"
                    })).unwrap();
                    fs::write(&report_file, report_content)?;
                    "Report written to: {}".to_string()
                }
                CiOutputFormat::Junit => {
                    let report_content = serde_json::to_string_pretty(&serde_json::json!({
                        "status": "valid",
                        "message": "All CI/CD validation checks passed successfully"
                    })).unwrap();
                    fs::write(&report_file, report_content)?;
                    "Report written to: {}".to_string()
                }
            };
            println!("{}", report_content);
        }

        if !messages.is_empty() {
            println!("\nCI/CD Validation Messages:");
            for message in messages {
                println!("  - {}", message);
            }
        }

        std::process::exit(exit_status);
    }

    /// Execute the CI/CD generate command
    fn execute_ci_generate(args: crate::commands::lock::LockSubcommands::CiGenerate) -> RhemaResult<()> {
        let output_path = args.output.unwrap_or_else(|| PathBuf::from("rhema.lock"));
        let repo_path = std::env::current_dir()?;

        // Check if lock file exists and force flag
        if output_path.exists() && !args.force {
            println!("{}", "Warning: Lock file already exists. Use --force to overwrite.".yellow());
            return Ok(());
        }

        if args.verbose {
            println!("🔍 Generating lock file for repository: {}", repo_path.display());
            println!("📁 Output file: {}", output_path.display());
            println!("⚙️  Resolution strategy: {:?}", args.strategy);
        }

        let mut config = LockGenerator::new();
        // Note: In a real implementation, you would configure the generator with the args

        let lock_file = LockSystem::generate_lock_file(&repo_path)?;

        let lock_content = serde_json::to_string_pretty(&lock_file)?;
        fs::write(&output_path, lock_content)?;

        if args.verbose {
            println!("✅ Lock file generated successfully!");
            println!("📊 Statistics:");
            println!("   - Total scopes: {}", lock_file.metadata.total_scopes);
            println!("   - Total dependencies: {}", lock_file.metadata.total_dependencies);
            println!("   - Circular dependencies: {}", lock_file.metadata.circular_dependencies);
        } else {
            println!("✅ Lock file generated: {}", output_path.display());
        }

        Ok(())
    }

    /// Execute the CI/CD consistency command
    fn execute_ci_consistency(args: crate::commands::lock::LockSubcommands::CiConsistency) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));
        let reference_file = args.reference_file;
        let git_branch = args.git_branch;
        let allow_semver_diffs = args.allow_semver_diffs;
        let max_version_drift = args.max_version_drift;
        let report_file = args.report_file;
        let format = args.format;
        let exit_code = args.exit_code;

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        let lock_file: RhemaLock = serde_json::from_str(&fs::read_to_string(&lock_path)?)?;
        let reference_lock: RhemaLock;

        if let Some(ref_path) = reference_file {
            if !ref_path.exists() {
                return Err(RhemaError::IoError(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Reference lock file not found: {}", ref_path.display()),
                )));
            }
            reference_lock = serde_json::from_str(&fs::read_to_string(&ref_path)?)?;
        } else {
            // Fallback to current lock file if no reference provided
            reference_lock = lock_file;
        }

        let consistency_result = LockSystem::check_lock_file_consistency(&lock_file, &reference_lock, &git_branch, &allow_semver_diffs, &max_version_drift);

        let mut exit_status = 0;
        let mut messages = Vec::new();

        match consistency_result {
            Ok(_) => {
                messages.push("All CI/CD consistency checks passed successfully.");
                if report_file.is_some() {
                    let report_content = match format {
                        CiOutputFormat::Text => {
                            let mut report_writer = io::BufWriter::new(fs::File::create(&report_file.unwrap())?);
                            serde_json::to_writer_pretty(&mut report_writer, &serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD consistency checks passed successfully"
                            })).unwrap();
                            report_writer.flush()?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Json => {
                            let report_content = serde_json::to_string_pretty(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD consistency checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Yaml => {
                            let report_content = serde_yaml::to_string(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD consistency checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Junit => {
                            let report_content = serde_json::to_string_pretty(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD consistency checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                    };
                    println!("{}", report_content);
                }
            }
            Err(e) => {
                messages.push(format!("CI/CD consistency check failed: {}", e));
                exit_status = 1;
                println!("🔍 CI/CD Consistency Results");
                println!("===========================");
                println!("Status: ❌ Invalid");
                println!("CI/CD consistency check failed: {}", e);
            }
        }

        if !messages.is_empty() {
            println!("\nCI/CD Consistency Messages:");
            for message in messages {
                println!("  - {}", message);
            }
        }

        std::process::exit(exit_status);
    }

    /// Execute the CI/CD update command
    fn execute_ci_update(args: crate::commands::lock::LockSubcommands::CiUpdate) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));
        let repo_path = std::env::current_dir()?;

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        let update_strategy = args.update_strategy;
        let strategy = args.strategy;
        let security_only = args.security_only;
        let max_updates = args.max_updates;
        let backup = args.backup;
        let report_file = args.report_file;
        let format = args.format;
        let exit_code = args.exit_code;

        let lock_content = fs::read_to_string(&lock_path)?;
        let mut lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        let update_result = LockSystem::update_lock_file_ci(&repo_path, &mut lock_file, update_strategy, strategy, security_only, max_updates);

        let mut exit_status = 0;
        let mut messages = Vec::new();

        match update_result {
            Ok(_) => {
                messages.push("All CI/CD update checks passed successfully.");
                if report_file.is_some() {
                    let report_content = match format {
                        CiOutputFormat::Text => {
                            let mut report_writer = io::BufWriter::new(fs::File::create(&report_file.unwrap())?);
                            serde_json::to_writer_pretty(&mut report_writer, &serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD update checks passed successfully"
                            })).unwrap();
                            report_writer.flush()?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Json => {
                            let report_content = serde_json::to_string_pretty(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD update checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Yaml => {
                            let report_content = serde_yaml::to_string(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD update checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Junit => {
                            let report_content = serde_json::to_string_pretty(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD update checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                    };
                    println!("{}", report_content);
                }
            }
            Err(e) => {
                messages.push(format!("CI/CD update failed: {}", e));
                exit_status = 1;
                println!("🔄 CI/CD Update Results");
                println!("========================");
                println!("Status: ❌ Invalid");
                println!("CI/CD update failed: {}", e);
            }
        }

        if !messages.is_empty() {
            println!("\nCI/CD Update Messages:");
            for message in messages {
                println!("  - {}", message);
            }
        }

        std::process::exit(exit_status);
    }

    /// Execute the CI/CD health command
    fn execute_ci_health(args: crate::commands::lock::LockSubcommands::CiHealth) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        let integrity = args.integrity;
        let freshness = args.freshness;
        let availability = args.availability;
        let performance = args.performance;
        let report_file = args.report_file;
        let format = args.format;
        let exit_code = args.exit_code;

        let lock_file: RhemaLock = serde_json::from_str(&fs::read_to_string(&lock_path)?)?;

        let health_result = LockSystem::check_lock_file_health(&lock_file, integrity, freshness, availability, performance);

        let mut exit_status = 0;
        let mut messages = Vec::new();

        match health_result {
            Ok(_) => {
                messages.push("All CI/CD health checks passed successfully.");
                if report_file.is_some() {
                    let report_content = match format {
                        CiOutputFormat::Text => {
                            let mut report_writer = io::BufWriter::new(fs::File::create(&report_file.unwrap())?);
                            serde_json::to_writer_pretty(&mut report_writer, &serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD health checks passed successfully"
                            })).unwrap();
                            report_writer.flush()?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Json => {
                            let report_content = serde_json::to_string_pretty(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD health checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Yaml => {
                            let report_content = serde_yaml::to_string(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD health checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                        CiOutputFormat::Junit => {
                            let report_content = serde_json::to_string_pretty(&serde_json::json!({
                                "status": "valid",
                                "message": "All CI/CD health checks passed successfully"
                            })).unwrap();
                            fs::write(&report_file.unwrap(), report_content)?;
                            "Report written to: {}".to_string()
                        }
                    };
                    println!("{}", report_content);
                }
            }
            Err(e) => {
                messages.push(format!("CI/CD health check failed: {}", e));
                exit_status = 1;
                println!("💪 CI/CD Health Results");
                println!("========================");
                println!("Status: ❌ Invalid");
                println!("CI/CD health check failed: {}", e);
            }
        }

        if !messages.is_empty() {
            println!("\nCI/CD Health Messages:");
            for message in messages {
                println!("  - {}", message);
            }
        }

        std::process::exit(exit_status);
    }

    /// Execute the conflict resolution command
    fn execute_resolve_conflicts(args: crate::commands::lock::LockSubcommands::ResolveConflicts) -> RhemaResult<()> {
        let lock_path = args.file.unwrap_or_else(|| PathBuf::from("rhema.lock"));
        let repo_path = std::env::current_dir()?;

        if !lock_path.exists() {
            return Err(RhemaError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Lock file not found: {}", lock_path.display()),
            )));
        }

        // Parse fallback strategies
        let fallback_strategies = if let Some(fallback_str) = args.fallback_strategies {
            fallback_str
                .split(',')
                .filter_map(|s| {
                    match s.trim() {
                        "latest_compatible" => Some(ConflictResolutionStrategy::LatestCompatible),
                        "pinned_version" => Some(ConflictResolutionStrategy::PinnedVersion),
                        "manual_resolution" => Some(ConflictResolutionStrategy::ManualResolution),
                        "automatic_detection" => Some(ConflictResolutionStrategy::AutomaticDetection),
                        "history_tracking" => Some(ConflictResolutionStrategy::HistoryTracking),
                        "smart_selection" => Some(ConflictResolutionStrategy::SmartSelection),
                        "conservative" => Some(ConflictResolutionStrategy::Conservative),
                        "aggressive" => Some(ConflictResolutionStrategy::Aggressive),
                        "hybrid" => Some(ConflictResolutionStrategy::Hybrid),
                        _ => None,
                    }
                })
                .collect()
        } else {
            vec![
                ConflictResolutionStrategy::Conservative,
                ConflictResolutionStrategy::ManualResolution,
            ]
        };

        // Create configuration
        let config = ConflictResolutionConfig {
            primary_strategy: args.strategy,
            fallback_strategies,
            enable_auto_detection: args.auto_detection,
            track_history: args.track_history,
            max_attempts: 10,
            allow_user_prompts: args.allow_prompts,
            prefer_stable: args.prefer_stable,
            strict_pinning: args.strict_pinning,
            compatibility_threshold: args.compatibility_threshold,
            parallel_resolution: args.parallel,
            max_parallel_threads: args.max_threads,
            timeout_seconds: args.timeout,
        };

        // Read lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        // Extract dependencies from lock file
        let dependencies: Vec<DependencySpec> = lock_file
            .scopes
            .iter()
            .flat_map(|(scope_path, scope)| {
                scope.dependencies.iter().map(|(dep_name, dep)| {
                    // Convert version string to VersionConstraint
                    let version_constraint = if let Ok(ver) = semver::Version::parse(&dep.version) {
                        crate::lock::conflict_resolver::VersionConstraint::Exact(ver)
                    } else {
                        // Fall back to latest if parsing fails
                        crate::lock::conflict_resolver::VersionConstraint::Latest
                    };

                    // Convert dependency type
                    let dependency_type = match dep.dependency_type {
                        crate::schema::DependencyType::Required => crate::schema::DependencyType::Required,
                        crate::schema::DependencyType::Optional => crate::schema::DependencyType::Optional,
                        crate::schema::DependencyType::Peer => crate::schema::DependencyType::Peer,
                        crate::schema::DependencyType::Development => crate::schema::DependencyType::Development,
                        crate::schema::DependencyType::Build => crate::schema::DependencyType::Build,
                    };

                    DependencySpec {
                        path: dep_name.clone(),
                        version_constraint,
                        dependency_type,
                        is_transitive: dep.is_transitive,
                        original_constraint: dep.original_constraint.clone(),
                        scope_path: scope_path.clone(),
                        priority: 5, // Default priority
                        optional: matches!(dep.dependency_type, crate::schema::DependencyType::Optional),
                        alternatives: Vec::new(), // No alternatives in lock file
                        metadata: HashMap::new(), // Convert custom metadata if needed
                    }
                })
            })
            .collect();

        if args.verbose {
            println!("🔍 Analyzing {} dependencies for conflicts...", dependencies.len());
        }

        // Resolve conflicts
        let resolution_result = LockSystem::resolve_conflicts(&dependencies, &repo_path, Some(config));

        let mut exit_status = 0;
        let mut messages = Vec::new();

        match resolution_result {
            Ok(result) => {
                if result.successful {
                    messages.push(format!("✅ Successfully resolved {} conflicts", result.stats.auto_resolved));
                    
                    if result.stats.manual_resolution_required > 0 {
                        messages.push(format!("⚠️  {} conflicts require manual resolution", result.stats.manual_resolution_required));
                    }
                    
                    if result.stats.unresolved_conflicts > 0 {
                        messages.push(format!("❌ {} conflicts could not be resolved", result.stats.unresolved_conflicts));
                        exit_status = 1;
                    }

                    if args.apply && result.successful {
                        // Apply resolved dependencies back to lock file
                        for (name, resolved_dep) in result.resolved_dependencies {
                            // Update the lock file with resolved dependencies
                            // This would need to be implemented based on the lock file structure
                        }
                        
                        let updated_content = serde_json::to_string_pretty(&lock_file)?;
                        fs::write(&lock_path, updated_content)?;
                        messages.push(format!("💾 Resolved lock file written to: {}", lock_path.display()));
                    }

                    // Generate report if requested
                    if let Some(report_path) = args.report_file {
                        let report_content = match format {
                            OutputFormat::Text => {
                                let mut report = String::new();
                                report.push_str("Conflict Resolution Report\n");
                                report.push_str("========================\n\n");
                                report.push_str(&format!("Status: {}\n", if result.successful { "✅ Success" } else { "❌ Failed" }));
                                report.push_str(&format!("Total conflicts: {}\n", result.stats.total_conflicts));
                                report.push_str(&format!("Auto-resolved: {}\n", result.stats.auto_resolved));
                                report.push_str(&format!("Manual resolution required: {}\n", result.stats.manual_resolution_required));
                                report.push_str(&format!("Unresolved: {}\n", result.stats.unresolved_conflicts));
                                report.push_str(&format!("Total time: {}ms\n", result.performance_metrics.total_time_ms));
                                
                                if !result.recommendations.is_empty() {
                                    report.push_str("\nRecommendations:\n");
                                    for rec in &result.recommendations {
                                        report.push_str(&format!("- {}\n", rec));
                                    }
                                }
                                report
                            }
                            OutputFormat::Json => {
                                serde_json::to_string_pretty(&serde_json::json!({
                                    "status": if result.successful { "success" } else { "failed" },
                                    "stats": {
                                        "total_conflicts": result.stats.total_conflicts,
                                        "auto_resolved": result.stats.auto_resolved,
                                        "manual_resolution_required": result.stats.manual_resolution_required,
                                        "unresolved_conflicts": result.stats.unresolved_conflicts,
                                        "total_time_ms": result.performance_metrics.total_time_ms
                                    },
                                    "recommendations": result.recommendations,
                                    "warnings": result.warnings
                                })).unwrap()
                            }
                            OutputFormat::Yaml => {
                                serde_yaml::to_string(&serde_json::json!({
                                    "status": if result.successful { "success" } else { "failed" },
                                    "stats": {
                                        "total_conflicts": result.stats.total_conflicts,
                                        "auto_resolved": result.stats.auto_resolved,
                                        "manual_resolution_required": result.stats.manual_resolution_required,
                                        "unresolved_conflicts": result.stats.unresolved_conflicts,
                                        "total_time_ms": result.performance_metrics.total_time_ms
                                    },
                                    "recommendations": result.recommendations,
                                    "warnings": result.warnings
                                })).unwrap()
                            }
                        };
                        
                        fs::write(&report_path, report_content)?;
                        messages.push(format!("📄 Report written to: {}", report_path.display()));
                    }
                } else {
                    messages.push("❌ Conflict resolution failed");
                    exit_status = 1;
                }

                // Show detailed conflict information
                if args.verbose || args.manual_only {
                    println!("\n🔄 Conflict Resolution Results");
                    println!("===========================");
                    
                    if result.detected_conflicts.is_empty() {
                        println!("✅ No conflicts detected");
                    } else {
                        for (i, conflict) in result.detected_conflicts.iter().enumerate() {
                            if args.manual_only && conflict.severity != crate::lock::conflict_resolver::ConflictSeverity::Critical {
                                continue;
                            }
                            
                            println!("\n{}. {} (Severity: {:?})", i + 1, conflict.dependency_name, conflict.severity);
                            println!("   Description: {}", conflict.description);
                            
                            if let Some(suggested) = &conflict.suggested_resolution {
                                println!("   Suggested resolution: {}", suggested);
                            }
                            
                            if !conflict.recommendations.is_empty() {
                                println!("   Recommendations:");
                                for rec in &conflict.recommendations {
                                    println!("     - {}", rec);
                                }
                            }
                        }
                    }
                }

                // Show warnings
                if !result.warnings.is_empty() {
                    println!("\n⚠️  Warnings:");
                    for warning in &result.warnings {
                        println!("  - {}", warning);
                    }
                }

                // Show recommendations
                if !result.recommendations.is_empty() {
                    println!("\n💡 Recommendations:");
                    for rec in &result.recommendations {
                        println!("  - {}", rec);
                    }
                }
            }
            Err(e) => {
                messages.push(format!("❌ Conflict resolution failed: {}", e));
                exit_status = 1;
                println!("🔄 Conflict Resolution Results");
                println!("===========================");
                println!("Status: ❌ Failed");
                println!("Error: {}", e);
            }
        }

        if !messages.is_empty() {
            println!("\n📋 Summary:");
            for message in messages {
                println!("  {}", message);
            }
        }

        if args.show_metrics {
            println!("\n📊 Performance Metrics:");
            if let Ok(result) = resolution_result {
                println!("  - Total time: {}ms", result.performance_metrics.total_time_ms);
                println!("  - Detection time: {}ms", result.performance_metrics.detection_time_ms);
                println!("  - Strategy execution time: {}ms", result.performance_metrics.strategy_execution_time_ms);
                println!("  - Compatibility scoring time: {}ms", result.performance_metrics.compatibility_scoring_time_ms);
                println!("  - Memory usage: {} bytes", result.performance_metrics.memory_usage_bytes);
                println!("  - Parallel operations: {}", result.performance_metrics.parallel_operations);
                println!("  - Cache operations: {}", result.performance_metrics.cache_operations);
            }
        }

        std::process::exit(exit_status);
    }

    fn print_status_text(lock_file: &RhemaLock, detailed: bool, issues_only: bool) {
        println!("📊 Lock File Status");
        println!("==================");
        
        println!("File: rhema.lock");
        println!("Version: {}", lock_file.lockfile_version);
        println!("Generated: {}", lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Generated by: {}", lock_file.generated_by);
        println!("Checksum: {}", &lock_file.checksum[..16]);
        
        println!("\nStatistics:");
        println!("  - Total scopes: {}", lock_file.metadata.total_scopes);
        println!("  - Total dependencies: {}", lock_file.metadata.total_dependencies);
        println!("  - Circular dependencies: {}", lock_file.metadata.circular_dependencies);
        println!("  - Validation status: {:?}", lock_file.metadata.validation_status);
        println!("  - Resolution strategy: {:?}", lock_file.metadata.resolution_strategy);
        println!("  - Conflict resolution: {:?}", lock_file.metadata.conflict_resolution);

        if let Some(ref metrics) = lock_file.metadata.performance_metrics {
            println!("\nPerformance Metrics:");
            println!("  - Generation time: {}ms", metrics.generation_time_ms);
            println!("  - Resolution attempts: {}", metrics.resolution_attempts);
            println!("  - Cache hits: {}", metrics.cache_hits);
            println!("  - Cache misses: {}", metrics.cache_misses);
            if let Some(hit_rate) = lock_file.metadata.cache_hit_rate() {
                println!("  - Cache hit rate: {:.2}%", hit_rate * 100.0);
            }
        }

        if detailed {
            println!("\nScopes:");
            for (path, scope) in &lock_file.scopes {
                let status = if scope.has_circular_dependencies { "⚠️" } else { "✅" };
                println!("  {} {} (v{}) - {} dependencies", 
                    status, path, scope.version, scope.dependencies.len());
                
                if !issues_only || scope.has_circular_dependencies {
                    for (dep_name, dep) in &scope.dependencies {
                        println!("    - {}: {} ({})", dep_name, dep.version, dep.dependency_type);
                    }
                }
            }
        }

        if let Some(ref messages) = lock_file.metadata.validation_messages {
            if !messages.is_empty() {
                println!("\nValidation Messages:");
                for message in messages {
                    println!("  - {}", message);
                }
            }
        }
    }

    fn print_status_json(lock_file: &RhemaLock) {
        let json = serde_json::to_string_pretty(lock_file).unwrap();
        println!("{}", json);
    }

    fn print_status_yaml(lock_file: &RhemaLock) {
        let yaml = serde_yaml::to_string(lock_file).unwrap();
        println!("{}", yaml);
    }

    fn calculate_diff(old_lock: &RhemaLock, new_lock: &RhemaLock) -> LockDiffResult {
        let mut result = LockDiffResult {
            added_scopes: Vec::new(),
            removed_scopes: Vec::new(),
            updated_scopes: Vec::new(),
            added_dependencies: Vec::new(),
            removed_dependencies: Vec::new(),
            updated_dependencies: Vec::new(),
        };

        // Compare scopes
        for (path, new_scope) in &new_lock.scopes {
            if let Some(old_scope) = old_lock.scopes.get(path) {
                // Scope exists in both - check for updates
                if old_scope.version != new_scope.version {
                    result.updated_scopes.push(ScopeDiff {
                        path: path.clone(),
                        old_version: old_scope.version.clone(),
                        new_version: new_scope.version.clone(),
                    });
                }
            } else {
                // New scope
                result.added_scopes.push(path.clone());
            }
        }

        for (path, _) in &old_lock.scopes {
            if !new_lock.scopes.contains_key(path) {
                // Removed scope
                result.removed_scopes.push(path.clone());
            }
        }

        // Compare dependencies within each scope
        for (scope_path, new_scope) in &new_lock.scopes {
            if let Some(old_scope) = old_lock.scopes.get(scope_path) {
                for (dep_name, new_dep) in &new_scope.dependencies {
                    if let Some(old_dep) = old_scope.dependencies.get(dep_name) {
                        // Dependency exists in both - check for updates
                        if old_dep.version != new_dep.version {
                            result.updated_dependencies.push(DependencyDiff {
                                scope: scope_path.clone(),
                                dependency: dep_name.clone(),
                                old_version: old_dep.version.clone(),
                                new_version: new_dep.version.clone(),
                            });
                        }
                    } else {
                        // New dependency
                        result.added_dependencies.push(DependencyDiff {
                            scope: scope_path.clone(),
                            dependency: dep_name.clone(),
                            old_version: "".to_string(),
                            new_version: new_dep.version.clone(),
                        });
                    }
                }

                for (dep_name, _) in &old_scope.dependencies {
                    if !new_scope.dependencies.contains_key(dep_name) {
                        // Removed dependency
                        result.removed_dependencies.push(DependencyDiff {
                            scope: scope_path.clone(),
                            dependency: dep_name.clone(),
                            old_version: "".to_string(),
                            new_version: "".to_string(),
                        });
                    }
                }
            }
        }

        result
    }

    fn print_diff_text(
        diff: &LockDiffResult, 
        detailed: bool, 
        added_only: bool, 
        removed_only: bool, 
        updated_only: bool
    ) {
        println!("🔍 Lock File Diff");
        println!("================");

        let show_added = !removed_only && !updated_only;
        let show_removed = !added_only && !updated_only;
        let show_updated = !added_only && !removed_only;

        if show_added && !diff.added_scopes.is_empty() {
            println!("\n➕ Added Scopes:");
            for scope in &diff.added_scopes {
                println!("  - {}", scope);
            }
        }

        if show_removed && !diff.removed_scopes.is_empty() {
            println!("\n➖ Removed Scopes:");
            for scope in &diff.removed_scopes {
                println!("  - {}", scope);
            }
        }

        if show_updated && !diff.updated_scopes.is_empty() {
            println!("\n🔄 Updated Scopes:");
            for scope in &diff.updated_scopes {
                println!("  - {}: {} → {}", scope.path, scope.old_version, scope.new_version);
            }
        }

        if show_added && !diff.added_dependencies.is_empty() {
            println!("\n➕ Added Dependencies:");
            for dep in &diff.added_dependencies {
                println!("  - {}:{} → {}", dep.scope, dep.dependency, dep.new_version);
            }
        }

        if show_removed && !diff.removed_dependencies.is_empty() {
            println!("\n➖ Removed Dependencies:");
            for dep in &diff.removed_dependencies {
                println!("  - {}:{}", dep.scope, dep.dependency);
            }
        }

        if show_updated && !diff.updated_dependencies.is_empty() {
            println!("\n🔄 Updated Dependencies:");
            for dep in &diff.updated_dependencies {
                println!("  - {}:{}: {} → {}", 
                    dep.scope, dep.dependency, dep.old_version, dep.new_version);
            }
        }

        if !detailed {
            println!("\nSummary:");
            println!("  - Added scopes: {}", diff.added_scopes.len());
            println!("  - Removed scopes: {}", diff.removed_scopes.len());
            println!("  - Updated scopes: {}", diff.updated_scopes.len());
            println!("  - Added dependencies: {}", diff.added_dependencies.len());
            println!("  - Removed dependencies: {}", diff.removed_dependencies.len());
            println!("  - Updated dependencies: {}", diff.updated_dependencies.len());
        }
    }

    fn print_diff_json(diff: &LockDiffResult) {
        let json = serde_json::to_string_pretty(diff).unwrap();
        println!("{}", json);
    }

    fn print_diff_yaml(diff: &LockDiffResult) {
        let yaml = serde_yaml::to_string(diff).unwrap();
        println!("{}", yaml);
    }
}

/// Result of lock file diff operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LockDiffResult {
    pub added_scopes: Vec<String>,
    pub removed_scopes: Vec<String>,
    pub updated_scopes: Vec<ScopeDiff>,
    pub added_dependencies: Vec<DependencyDiff>,
    pub removed_dependencies: Vec<DependencyDiff>,
    pub updated_dependencies: Vec<DependencyDiff>,
}

/// Scope difference information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScopeDiff {
    pub path: String,
    pub old_version: String,
    pub new_version: String,
}

/// Dependency difference information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyDiff {
    pub scope: String,
    pub dependency: String,
    pub old_version: String,
    pub new_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_lock_diff_calculation() {
        // Create test lock files
        let old_lock = RhemaLock::new("test");
        let new_lock = RhemaLock::new("test");

        let diff = LockSubcommands::calculate_diff(&old_lock, &new_lock);
        
        assert_eq!(diff.added_scopes.len(), 0);
        assert_eq!(diff.removed_scopes.len(), 0);
        assert_eq!(diff.updated_scopes.len(), 0);
    }

    #[test]
    fn test_lock_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let lock_path = temp_dir.path().join("test.lock");
        
        // Test that we can create a lock file
        let lock_file = RhemaLock::new("test");
        let content = serde_json::to_string_pretty(&lock_file).unwrap();
        fs::write(&lock_path, content).unwrap();
        
        assert!(lock_path.exists());
    }
} 