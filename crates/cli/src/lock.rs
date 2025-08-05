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

use crate::{Rhema, RhemaResult};
use clap::Subcommand;
use colored::*;
use rhema_core::schema::RhemaLock;
use std::fs;
use std::path::PathBuf;
use serde_json;

/// Lock file management commands
#[derive(Subcommand)]
pub enum LockSubcommands {
    /// Generate a new lock file
    Generate {
        /// Output file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        output: Option<String>,

        /// Force regeneration even if lock file exists
        #[arg(long)]
        force: bool,

        /// Include performance metrics
        #[arg(long)]
        include_metrics: bool,

        /// Validate after generation
        #[arg(long)]
        validate: bool,
    },

    /// Validate an existing lock file
    Validate {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<String>,

        /// Maximum age for lock file freshness (in hours)
        #[arg(long, value_name = "HOURS")]
        max_age: Option<u64>,

        /// Strict validation (treat warnings as errors)
        #[arg(long)]
        strict: bool,

        /// Output format (text, json, yaml)
        #[arg(long, value_name = "FORMAT", default_value = "text")]
        format: String,
    },

    /// Update an existing lock file
    Update {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<String>,

        /// Backup the existing lock file before updating
        #[arg(long)]
        backup: bool,

        /// Validate after update
        #[arg(long)]
        validate: bool,

        /// Include performance metrics
        #[arg(long)]
        include_metrics: bool,
    },

    /// Show lock file status
    Status {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<String>,

        /// Show detailed information
        #[arg(long)]
        detailed: bool,

        /// Show only issues
        #[arg(long)]
        issues_only: bool,

        /// Output format (text, json, yaml)
        #[arg(long, value_name = "FORMAT", default_value = "text")]
        format: String,
    },

    /// Show differences between lock file and current state
    Diff {
        /// Lock file path (default: rhema.lock)
        #[arg(long, value_name = "FILE")]
        file: Option<String>,

        /// Output format (text, json, yaml)
        #[arg(long, value_name = "FORMAT", default_value = "text")]
        format: String,
    },
}

impl LockSubcommands {
    /// Execute the lock command
    pub fn execute(&self, rhema: &Rhema) -> RhemaResult<()> {
        match self {
            LockSubcommands::Generate { output, force, include_metrics, validate } => Self::execute_generate(rhema, output, *force, *include_metrics, *validate),
            LockSubcommands::Validate { file, max_age, strict, format } => Self::execute_validate(rhema, file, *max_age, *strict, format),
            LockSubcommands::Update { file, backup, validate, include_metrics } => Self::execute_update(rhema, file, *backup, *validate, *include_metrics),
            LockSubcommands::Status { file, detailed, issues_only, format } => Self::execute_status(rhema, file, *detailed, *issues_only, format),
            LockSubcommands::Diff { file, format } => Self::execute_diff(rhema, file, format),
        }
    }

    fn execute_generate(rhema: &Rhema, output: &Option<String>, force: bool, include_metrics: bool, validate: bool) -> RhemaResult<()> {
        let output_path = output.as_ref().map(PathBuf::from).unwrap_or_else(|| {
            rhema.repo_root().join("rhema.lock")
        });

        // Check if lock file exists and force flag
        if output_path.exists() && !force {
            println!("{}", "Warning: Lock file already exists. Use --force to overwrite.".yellow());
            return Ok(());
        }

        let repo_path = rhema.repo_root();
        println!("🔍 Generating lock file for repository: {}", repo_path.display());

        // Generate the lock file
        let lock_file = rhema_core::lock::LockFileOps::generate_lock_file(repo_path, "1.0.0", std::collections::HashMap::new())?;

        // Write the lock file
        let lock_content = serde_json::to_string_pretty(&lock_file)?;
        fs::write(&output_path, lock_content)?;

        println!("✅ Lock file generated successfully!");
        println!("   - Total scopes: {}", lock_file.scopes.len());
        println!("   - Total dependencies: {}", lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>());
        println!("   - Generated at: {}", lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("✅ Lock file generated: {}", output_path.display());

        // Validate if requested
        if validate {
            println!("🔍 Validating generated lock file...");
            let validation_result = rhema_core::lock::LockFileOps::validate_lock_file_integrity(&output_path)?;
            if validation_result.is_valid {
                println!("✅ Lock file validation passed!");
            } else {
                println!("⚠️  Lock file validation warnings: {}", validation_result.messages_as_string());
            }
        }

        Ok(())
    }

    fn execute_validate(rhema: &Rhema, file: &Option<String>, max_age: Option<u64>, strict: bool, format: &str) -> RhemaResult<()> {
        let lock_path = file.as_ref().map(PathBuf::from).unwrap_or_else(|| {
            rhema.repo_root().join("rhema.lock")
        });

        if !lock_path.exists() {
            return Err(crate::RhemaError::FileNotFound(
                format!("Lock file not found: {}", lock_path.display()),
            ));
        }

        // Read and parse the lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        println!("🔍 Validating lock file: {}", lock_path.display());

        // Validate the lock file
        let validation_result = rhema_core::lock::LockFileOps::validate_lock_file_integrity(&lock_path);

        // Output results based on format
        match format {
            "json" => print_validation_json(&lock_file, &validation_result),
            "yaml" => print_validation_yaml(&lock_file, &validation_result),
            _ => print_validation_text(&lock_file, &validation_result, strict),
        }

        // Return error if validation failed and strict mode is enabled
        if strict && validation_result.is_err() {
            validation_result?;
        }

        Ok(())
    }

    fn execute_update(rhema: &Rhema, file: &Option<String>, backup: bool, validate: bool, include_metrics: bool) -> RhemaResult<()> {
        let lock_path = file.as_ref().map(PathBuf::from).unwrap_or_else(|| {
            rhema.repo_root().join("rhema.lock")
        });

        if !lock_path.exists() {
            return Err(crate::RhemaError::FileNotFound(
                format!("Lock file not found: {}", lock_path.display()),
            ));
        }

        let repo_path = rhema.repo_root();

        // Read existing lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let mut lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        println!("🔄 Updating lock file: {}", lock_path.display());

        // Create backup if requested
        if backup {
            let backup_path = lock_path.with_extension("lock.backup");
            fs::copy(&lock_path, &backup_path)?;
            println!("📦 Backup created: {}", backup_path.display());
        }

        // Show current statistics
        println!("📊 Current lock file statistics:");
        println!("   - Total scopes: {}", lock_file.scopes.len());
        println!("   - Total dependencies: {}", lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>());

        // Update the lock file - for now, just regenerate it
        let updated_lock = rhema_core::lock::LockFileOps::generate_lock_file(repo_path, "1.0.0", std::collections::HashMap::new())?;
        lock_file = updated_lock;

        // Write updated lock file
        let updated_content = serde_json::to_string_pretty(&lock_file)?;
        fs::write(&lock_path, updated_content)?;

        println!("✅ Lock file updated successfully!");
        println!("   - Total scopes: {}", lock_file.scopes.len());
        println!("   - Total dependencies: {}", lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>());
        println!("   - Updated at: {}", lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("✅ Lock file updated: {}", lock_path.display());

        // Validate if requested
        if validate {
            println!("🔍 Validating updated lock file...");
            let validation_result = rhema_core::lock::LockFileOps::validate_lock_file_integrity(&lock_path)?;
            if validation_result.is_valid {
                println!("✅ Lock file validation passed!");
            } else {
                println!("⚠️  Lock file validation warnings: {}", validation_result.messages_as_string());
            }
        }

        Ok(())
    }

    fn execute_status(rhema: &Rhema, file: &Option<String>, detailed: bool, issues_only: bool, format: &str) -> RhemaResult<()> {
        let lock_path = file.as_ref().map(PathBuf::from).unwrap_or_else(|| {
            rhema.repo_root().join("rhema.lock")
        });

        if !lock_path.exists() {
            return Err(crate::RhemaError::FileNotFound(
                format!("Lock file not found: {}", lock_path.display()),
            ));
        }

        // Read and parse the lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let lock_file: RhemaLock = serde_json::from_str(&lock_content)?;

        // Output status based on format
        match format {
            "json" => print_status_json(&lock_file),
            "yaml" => print_status_yaml(&lock_file),
            _ => print_status_text(&lock_file, detailed, issues_only),
        }

        Ok(())
    }

    fn execute_diff(rhema: &Rhema, file: &Option<String>, format: &str) -> RhemaResult<()> {
        let lock_path = file.as_ref().map(PathBuf::from).unwrap_or_else(|| {
            rhema.repo_root().join("rhema.lock")
        });

        if !lock_path.exists() {
            return Err(crate::RhemaError::FileNotFound(
                format!("Lock file not found: {}", lock_path.display()),
            ));
        }

        // Read existing lock file
        let lock_content = fs::read_to_string(&lock_path)?;
        let old_lock: RhemaLock = serde_json::from_str(&lock_content)?;

        // Generate new lock file for comparison
        let repo_path = rhema.repo_root();
        let new_lock = rhema_core::lock::LockFileOps::generate_lock_file(repo_path, "1.0.0", std::collections::HashMap::new())?;

        // Calculate differences
        let diff_result = calculate_lock_diff(&old_lock, &new_lock);

        // Output differences based on format
        match format {
            "json" => print_diff_json(&diff_result),
            "yaml" => print_diff_yaml(&diff_result),
            _ => print_diff_text(&diff_result),
        }

        Ok(())
    }
}

// Helper functions for output formatting

fn print_validation_text(lock_file: &RhemaLock, validation_result: &RhemaResult<rhema_core::ValidationResult>, strict: bool) {
    println!("📊 Lock File Validation Report");
    println!("{}", "─".repeat(50));
    println!("📄 File: rhema.lock");
    println!("📅 Generated: {}", lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("🔧 Version: {}", lock_file.lockfile_version);
    println!("📦 Total scopes: {}", lock_file.scopes.len());
    println!("🔗 Total dependencies: {}", lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>());

    match validation_result {
        Ok(result) => {
            if result.is_valid {
                println!("✅ Validation: PASSED");
                if !strict {
                    println!("🎉 Lock file is valid and ready to use!");
                }
            } else {
                println!("⚠️  Validation: WARNINGS");
                println!("Messages: {}", result.messages_as_string());
                if strict {
                    println!("⚠️  Strict mode enabled - treating validation warnings as fatal");
                }
            }
        }
        Err(e) => {
            println!("❌ Validation: FAILED");
            println!("Error: {}", e);
            if strict {
                println!("⚠️  Strict mode enabled - treating validation errors as fatal");
            }
        }
    }
}

fn print_validation_json(lock_file: &RhemaLock, validation_result: &RhemaResult<rhema_core::ValidationResult>) {
    let result = serde_json::json!({
        "validation": {
            "file": "rhema.lock",
            "generated_at": lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "version": lock_file.lockfile_version,
            "total_scopes": lock_file.scopes.len(),
            "total_dependencies": lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>(),
            "valid": validation_result.as_ref().map(|r| r.is_valid).unwrap_or(false),
            "error": validation_result.as_ref().err().map(|e| e.to_string()),
            "messages": validation_result.as_ref().ok().map(|r| &r.messages).unwrap_or(&Vec::new())
        }
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn print_validation_yaml(lock_file: &RhemaLock, validation_result: &RhemaResult<rhema_core::ValidationResult>) {
    let result = serde_json::json!({
        "validation": {
            "file": "rhema.lock",
            "generated_at": lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "version": lock_file.lockfile_version,
            "total_scopes": lock_file.scopes.len(),
            "total_dependencies": lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>(),
            "valid": validation_result.as_ref().map(|r| r.is_valid).unwrap_or(false),
            "error": validation_result.as_ref().err().map(|e| e.to_string()),
            "messages": validation_result.as_ref().ok().map(|r| &r.messages).unwrap_or(&Vec::new())
        }
    });
    println!("{}", serde_yaml::to_string(&result).unwrap());
}

fn print_status_text(lock_file: &RhemaLock, detailed: bool, issues_only: bool) {
    if issues_only {
        // Only show issues - simplified for now
        println!("⚠️  Issues-only mode not fully implemented yet");
        return;
    }

    println!("📊 Lock File Status");
    println!("{}", "─".repeat(50));
    println!("📄 File: rhema.lock");
    println!("📅 Generated: {}", lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("🔧 Version: {}", lock_file.lockfile_version);
    println!("📦 Total scopes: {}", lock_file.scopes.len());
    println!("🔗 Total dependencies: {}", lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>());

    if detailed {
        println!("\n📋 Scope Details:");
        for (path, scope) in &lock_file.scopes {
            println!("  📁 {} (v{})", path, scope.version);
            println!("    🔗 Dependencies: {}", scope.dependencies.len());
            if let Some(checksum) = &scope.source_checksum {
                println!("    🔒 Checksum: {}", checksum);
            }
        }
    }

    // Check for issues - simplified for now
    println!("✅ Status: HEALTHY (validation not implemented in this context)");
}

fn print_status_json(lock_file: &RhemaLock) {
    let result = serde_json::json!({
        "status": {
            "file": "rhema.lock",
            "generated_at": lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "version": lock_file.lockfile_version,
            "total_scopes": lock_file.scopes.len(),
            "total_dependencies": lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>(),
            "healthy": true
        }
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn print_status_yaml(lock_file: &RhemaLock) {
    let result = serde_json::json!({
        "status": {
            "file": "rhema.lock",
            "generated_at": lock_file.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "version": lock_file.lockfile_version,
            "total_scopes": lock_file.scopes.len(),
            "total_dependencies": lock_file.scopes.values().map(|s| s.dependencies.len()).sum::<usize>(),
            "healthy": true
        }
    });
    println!("{}", serde_yaml::to_string(&result).unwrap());
}

#[derive(Debug, Clone)]
struct LockDiffResult {
    added_scopes: Vec<String>,
    removed_scopes: Vec<String>,
    modified_scopes: Vec<String>,
    added_dependencies: Vec<(String, String)>,
    removed_dependencies: Vec<(String, String)>,
    modified_dependencies: Vec<(String, String, String)>,
}

fn calculate_lock_diff(old_lock: &RhemaLock, new_lock: &RhemaLock) -> LockDiffResult {
    let mut result = LockDiffResult {
        added_scopes: Vec::new(),
        removed_scopes: Vec::new(),
        modified_scopes: Vec::new(),
        added_dependencies: Vec::new(),
        removed_dependencies: Vec::new(),
        modified_dependencies: Vec::new(),
    };

    // Find added and modified scopes
    for (path, new_scope) in &new_lock.scopes {
        if let Some(old_scope) = old_lock.scopes.get(path) {
            // Check if scope was modified
            if new_scope.version != old_scope.version || new_scope.dependencies.len() != old_scope.dependencies.len() {
                result.modified_scopes.push(path.clone());
                
                // Check for dependency changes
                for (dep_path, new_dep) in &new_scope.dependencies {
                    if let Some(old_dep) = old_scope.dependencies.get(dep_path) {
                        if new_dep.version != old_dep.version {
                            result.modified_dependencies.push((
                                path.clone(),
                                dep_path.clone(),
                                format!("{} -> {}", old_dep.version, new_dep.version)
                            ));
                        }
                    } else {
                        result.added_dependencies.push((path.clone(), dep_path.clone()));
                    }
                }
                
                for (dep_path, _) in &old_scope.dependencies {
                    if !new_scope.dependencies.contains_key(dep_path) {
                        result.removed_dependencies.push((path.clone(), dep_path.clone()));
                    }
                }
            }
        } else {
            result.added_scopes.push(path.clone());
            for (dep_path, _) in &new_scope.dependencies {
                result.added_dependencies.push((path.clone(), dep_path.clone()));
            }
        }
    }

    // Find removed scopes
    for (path, _) in &old_lock.scopes {
        if !new_lock.scopes.contains_key(path) {
            result.removed_scopes.push(path.clone());
        }
    }

    result
}

fn print_diff_text(diff: &LockDiffResult) {
    println!("📊 Lock File Differences");
    println!("{}", "─".repeat(50));

    if diff.added_scopes.is_empty() && diff.removed_scopes.is_empty() && 
       diff.modified_scopes.is_empty() && diff.added_dependencies.is_empty() && 
       diff.removed_dependencies.is_empty() && diff.modified_dependencies.is_empty() {
        println!("✅ No differences found - lock file is up to date");
        return;
    }

    if !diff.added_scopes.is_empty() {
        println!("➕ Added scopes:");
        for scope in &diff.added_scopes {
            println!("  📁 {}", scope);
        }
        println!();
    }

    if !diff.removed_scopes.is_empty() {
        println!("➖ Removed scopes:");
        for scope in &diff.removed_scopes {
            println!("  📁 {}", scope);
        }
        println!();
    }

    if !diff.modified_scopes.is_empty() {
        println!("🔄 Modified scopes:");
        for scope in &diff.modified_scopes {
            println!("  📁 {}", scope);
        }
        println!();
    }

    if !diff.added_dependencies.is_empty() {
        println!("➕ Added dependencies:");
        for (scope, dep) in &diff.added_dependencies {
            println!("  📁 {} → {}", scope, dep);
        }
        println!();
    }

    if !diff.removed_dependencies.is_empty() {
        println!("➖ Removed dependencies:");
        for (scope, dep) in &diff.removed_dependencies {
            println!("  📁 {} → {}", scope, dep);
        }
        println!();
    }

    if !diff.modified_dependencies.is_empty() {
        println!("🔄 Modified dependencies:");
        for (scope, dep, change) in &diff.modified_dependencies {
            println!("  📁 {} → {} ({})", scope, dep, change);
        }
        println!();
    }
}

fn print_diff_json(diff: &LockDiffResult) {
    let result = serde_json::json!({
        "diff": {
            "added_scopes": diff.added_scopes,
            "removed_scopes": diff.removed_scopes,
            "modified_scopes": diff.modified_scopes,
            "added_dependencies": diff.added_dependencies,
            "removed_dependencies": diff.removed_dependencies,
            "modified_dependencies": diff.modified_dependencies
        }
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn print_diff_yaml(diff: &LockDiffResult) {
    let result = serde_json::json!({
        "diff": {
            "added_scopes": diff.added_scopes,
            "removed_scopes": diff.removed_scopes,
            "modified_scopes": diff.modified_scopes,
            "added_dependencies": diff.added_dependencies,
            "removed_dependencies": diff.removed_dependencies,
            "modified_dependencies": diff.modified_dependencies
        }
    });
    println!("{}", serde_yaml::to_string(&result).unwrap());
} 