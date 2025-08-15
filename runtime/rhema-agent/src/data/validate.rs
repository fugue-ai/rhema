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

use crate::{
    JsonSchema, Rhema, RhemaResult, SchemaMigratable, Validatable, CURRENT_SCHEMA_VERSION,
};
use colored::*;
use serde_yaml;
use std::path::Path;
use walkdir::WalkDir;
use rhema_core::schema::RhemaLock;
use sha2::{Digest, Sha256};
use chrono::Utc;

/// Find the scope file in the given directory, checking multiple possible locations
fn find_scope_file(scope_path: &Path) -> Option<std::path::PathBuf> {
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

pub fn run(
    rhema: &Rhema,
    recursive: bool,
    json_schema: bool,
    migrate: bool,
    lock_file: bool,
    lock_only: bool,
    strict: bool,
) -> RhemaResult<()> {
    println!("🔍 Validating Rhema context files...");
    println!("{}", "─".repeat(80));

    if json_schema {
        print_json_schemas();
        return Ok(());
    }

    // Handle lock-only validation
    if lock_only {
        return validate_lock_file_only(rhema, strict);
    }

    let mut total_files = 0;
    let mut valid_files = 0;
    let mut errors = Vec::new();
    let mut migrations_performed = 0;
    let mut lock_errors = Vec::new();

    if recursive {
        // Validate all scopes in the repository
        let scopes = rhema.discover_scopes()?;

        for scope in scopes {
            println!(
                "📁 Validating scope: {}",
                scope.definition.name.bright_blue()
            );
            let (scope_files, scope_valid, scope_errors, scope_migrations) =
                validate_scope(&scope.path, migrate)?;
            total_files += scope_files;
            valid_files += scope_valid;
            errors.extend(scope_errors);
            migrations_performed += scope_migrations;
        }
    } else {
        // Validate only the current scope
        let current_dir = std::env::current_dir().map_err(|e| crate::RhemaError::IoError(e))?;

        let scopes = rhema.discover_scopes()?;
        let scope = crate::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
            crate::RhemaError::ConfigError(
                "No Rhema scope found in current directory or parent directories".to_string(),
            )
        })?;

        println!(
            "📁 Validating scope: {}",
            scope.definition.name.bright_blue()
        );
        let (scope_files, scope_valid, scope_errors, scope_migrations) =
            validate_scope(&scope.path, migrate)?;
        total_files = scope_files;
        valid_files = scope_valid;
        errors = scope_errors;
        migrations_performed = scope_migrations;
    }

    // Validate lock file if requested
    if lock_file {
        println!("🔒 Validating lock file...");
        let lock_validation_result = validate_lock_file(rhema, strict)?;
        lock_errors.extend(lock_validation_result);
    }

    // Print summary
    println!("{}", "─".repeat(80));
    println!("📊 Validation Summary:");
    println!("  📄 Total files: {}", total_files);
    println!("  ✅ Valid files: {}", valid_files.to_string().green());
    println!("  ❌ Errors: {}", errors.len().to_string().red());
    if migrations_performed > 0 {
        println!(
            "  🔄 Migrations performed: {}",
            migrations_performed.to_string().yellow()
        );
    }
    if lock_file {
        println!("  🔒 Lock file errors: {}", lock_errors.len().to_string().red());
    }

    // Combine all errors
    let all_errors = [&errors[..], &lock_errors[..]].concat();

    if !all_errors.is_empty() {
        println!("\n❌ Validation Errors:");
        for (i, error) in all_errors.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().red(), error);
        }
        return Err(crate::RhemaError::SchemaValidation(format!(
            "Validation failed with {} errors",
            all_errors.len()
        )));
    } else {
        println!("🎉 All files are valid!");
        if migrations_performed > 0 {
            println!("🔄 Schema migrations completed successfully!");
        }
        if lock_file {
            println!("🔒 Lock file validation passed!");
        }
    }

    Ok(())
}

fn validate_scope(
    scope_path: &Path,
    migrate: bool,
) -> RhemaResult<(usize, usize, Vec<String>, usize)> {
    let mut total_files = 0;
    let mut valid_files = 0;
    let mut errors = Vec::new();
    let mut migrations_performed = 0;

    // Validate the scope definition itself
    // Integrate with lock file system for comprehensive validation
    if let Some(rhema_file) = find_scope_file(scope_path) {
        total_files += 1;
        let file_name = rhema_file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("scope file");
        match validate_rhema_file(&rhema_file, migrate) {
            Ok(migrations) => {
                valid_files += 1;
                migrations_performed += migrations;
                println!("  ✅ {}", file_name);
                if migrations > 0 {
                    println!(
                        "    🔄 Schema migrated to version {}",
                        CURRENT_SCHEMA_VERSION.yellow()
                    );
                }
            }
            Err(e) => {
                errors.push(format!("{}: {}", file_name, e));
                println!("  ❌ {}: {}", file_name, e);
            }
        }

        // Additional lock file integration validation
        if let Ok(lock_validation_errors) = validate_scope_against_lock_file(scope_path, &rhema_file) {
            for error in lock_validation_errors {
                errors.push(format!("{} (lock validation): {}", file_name, error));
                println!("  ⚠️  {} (lock validation): {}", file_name, error);
            }
        }
    }

    // Validate other YAML files in the scope
    for entry in WalkDir::new(scope_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Skip scope files as we already validated them
            if file_name == "rhema.yaml" || file_name == "scope.yaml" {
                continue;
            }

            total_files += 1;
            match validate_context_file(path) {
                Ok(()) => {
                    valid_files += 1;
                    println!("  ✅ {}", file_name);
                }
                Err(e) => {
                    errors.push(format!("{}: {}", file_name, e));
                    println!("  ❌ {}: {}", file_name, e);
                }
            }
        }
    }

    Ok((total_files, valid_files, errors, migrations_performed))
}

fn validate_rhema_file(file_path: &Path, migrate: bool) -> RhemaResult<usize> {
    let content = std::fs::read_to_string(file_path).map_err(|e| crate::RhemaError::IoError(e))?;

    let mut scope: crate::RhemaScope =
        serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
            file: file_path.display().to_string(),
            message: e.to_string(),
        })?;

    let mut migrations = 0;

    // Perform schema migration if requested
    if migrate {
        match scope.migrate_to_latest() {
            Ok(()) => {
                // Write the migrated content back to file
                let migrated_content =
                    serde_yaml::to_string(&scope).map_err(|e| crate::RhemaError::InvalidYaml {
                        file: file_path.display().to_string(),
                        message: format!("Failed to serialize migrated scope: {}", e),
                    })?;

                std::fs::write(file_path, migrated_content)
                    .map_err(|e| crate::RhemaError::IoError(e))?;

                migrations = 1;
            }
            Err(e) => {
                return Err(crate::RhemaError::SchemaValidation(format!(
                    "Schema migration failed: {}",
                    e
                )));
            }
        }
    }

    // Validate the scope definition with enhanced validation
    Validatable::validate(&scope)?;

    Ok(migrations)
}

fn validate_context_file(file_path: &Path) -> RhemaResult<()> {
    let content = std::fs::read_to_string(file_path).map_err(|e| crate::RhemaError::IoError(e))?;

    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Try to parse as different context types with enhanced validation
    match file_name {
        "todos.yaml" => {
            let todos: crate::Todos =
                serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&todos)?;
        }
        "knowledge.yaml" => {
            let knowledge: crate::Knowledge =
                serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&knowledge)?;
        }
        "patterns.yaml" => {
            let patterns: crate::Patterns =
                serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&patterns)?;
        }
        "decisions.yaml" => {
            let decisions: crate::Decisions =
                serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&decisions)?;
        }
        "conventions.yaml" => {
            let conventions: crate::Conventions =
                serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&conventions)?;
        }
        _ => {
            // For unknown files, just validate that they're valid YAML
            let _: serde_yaml::Value =
                serde_yaml::from_str(&content).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
        }
    }

    Ok(())
}

fn print_json_schemas() {
    println!("📋 JSON Schemas for Rhema Context Files");
    println!("{}", "─".repeat(80));

    // Read the comprehensive schema file
    match std::fs::read_to_string("schemas/rhema-v1.json") {
        Ok(schema_content) => match serde_json::from_str::<serde_json::Value>(&schema_content) {
            Ok(schema) => {
                println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            }
            Err(e) => {
                println!("❌ Error parsing schema file: {}", e);
                println!("📋 Using fallback schemas...");
                print_fallback_schemas();
            }
        },
        Err(e) => {
            println!("❌ Error reading schema file: {}", e);
            println!("📋 Using fallback schemas...");
            print_fallback_schemas();
        }
    }
}

fn print_fallback_schemas() {
    println!("🔧 Rhema Scope Schema:");
    println!(
        "{}",
        serde_json::to_string_pretty(&crate::RhemaScope::json_schema()).unwrap()
    );

    println!("\n📚 Knowledge Schema:");
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "type": "object",
            "required": ["entries"],
            "properties": {
                "entries": {
                    "type": "array",
                    "minItems": 1,
                    "items": {
                        "type": "object",
                        "required": ["id", "title", "content", "created_at"],
                        "properties": {
                            "id": {
                                "type": "string",
                                "minLength": 1,
                                "pattern": "^[a-zA-Z0-9_-]+$"
                            },
                            "title": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 200
                            },
                            "content": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 10000
                            },
                            "category": {
                                "type": "string"
                            },
                            "tags": {
                                "type": "array",
                                "items": {
                                    "type": "string",
                                    "minLength": 1,
                                    "maxLength": 50
                                }
                            },
                            "confidence": {
                                "type": "integer",
                                "minimum": 1,
                                "maximum": 10
                            },
                            "created_at": {
                                "type": "string",
                                "format": "date-time"
                            },
                            "updated_at": {
                                "type": "string",
                                "format": "date-time"
                            },
                            "source": {
                                "type": "string"
                            }
                        }
                    }
                },
                "categories": {
                    "type": "object",
                    "additionalProperties": {
                        "type": "string"
                    }
                }
            }
        }))
        .unwrap()
    );

    println!("\n✅ Todo Schema:");
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "type": "object",
        "properties": {
            "todos": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["id", "title", "status", "priority", "created_at"],
                    "properties": {
                        "id": {
                            "type": "string",
                            "minLength": 1,
                            "pattern": "^[a-zA-Z0-9_-]+$"
                        },
                        "title": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": 200
                        },
                        "description": {
                            "type": "string",
                            "maxLength": 2000
                        },
                        "status": {
                            "type": "string",
                            "enum": ["pending", "in_progress", "blocked", "completed", "cancelled"]
                        },
                        "priority": {
                            "type": "string",
                            "enum": ["low", "medium", "high", "critical"]
                        },
                        "assigned_to": {
                            "type": "string"
                        },
                        "due_date": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "created_at": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "completed_at": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "outcome": {
                            "type": "string",
                            "maxLength": 500
                        },
                        "related_knowledge": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "pattern": "^[a-zA-Z0-9_-]+$"
                            }
                        }
                    }
                }
            }
        }
    })).unwrap());

    println!("\n🎯 Decision Schema:");
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "type": "object",
        "properties": {
            "decisions": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["id", "title", "description", "status", "decided_at"],
                    "properties": {
                        "id": {
                            "type": "string",
                            "minLength": 1,
                            "pattern": "^[a-zA-Z0-9_-]+$"
                        },
                        "title": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": 200
                        },
                        "description": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": 2000
                        },
                        "status": {
                            "type": "string",
                            "enum": ["proposed", "under_review", "approved", "rejected", "implemented", "deprecated"]
                        },
                        "context": {
                            "type": "string",
                            "maxLength": 1000
                        },
                        "alternatives": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 200
                            }
                        },
                        "rationale": {
                            "type": "string",
                            "maxLength": 2000
                        },
                        "consequences": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 500
                            }
                        },
                        "decided_at": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "review_date": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "decision_makers": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 100
                            }
                        }
                    }
                }
            }
        }
    })).unwrap());

    println!("\n📐 Pattern Schema:");
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "type": "object",
        "properties": {
            "patterns": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["id", "name", "description", "pattern_type", "usage", "created_at"],
                    "properties": {
                        "id": {
                            "type": "string",
                            "minLength": 1,
                            "pattern": "^[a-zA-Z0-9_-]+$"
                        },
                        "name": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": 100
                        },
                        "description": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": 2000
                        },
                        "pattern_type": {
                            "type": "string",
                            "minLength": 1
                        },
                        "usage": {
                            "type": "string",
                            "enum": ["required", "recommended", "optional", "deprecated"]
                        },
                        "effectiveness": {
                            "type": "integer",
                            "minimum": 1,
                            "maximum": 10
                        },
                        "examples": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 1000
                            }
                        },
                        "anti_patterns": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 200
                            }
                        },
                        "related_patterns": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "pattern": "^[a-zA-Z0-9_-]+$"
                            }
                        },
                        "created_at": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "updated_at": {
                            "type": "string",
                            "format": "date-time"
                        }
                    }
                }
            }
        }
    })).unwrap());

    println!("\n📋 Convention Schema:");
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "type": "object",
        "properties": {
            "conventions": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["id", "name", "description", "convention_type", "enforcement", "created_at"],
                    "properties": {
                        "id": {
                            "type": "string",
                            "minLength": 1,
                            "pattern": "^[a-zA-Z0-9_-]+$"
                        },
                        "name": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": 100
                        },
                        "description": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": 2000
                        },
                        "convention_type": {
                            "type": "string",
                            "minLength": 1
                        },
                        "enforcement": {
                            "type": "string",
                            "enum": ["required", "recommended", "optional", "deprecated"]
                        },
                        "examples": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 1000
                            }
                        },
                        "tools": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 100
                            }
                        },
                        "created_at": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "updated_at": {
                            "type": "string",
                            "format": "date-time"
                        }
                    }
                }
            }
        }
    })).unwrap());
}

/// Validate lock file only (skip other validations)
fn validate_lock_file_only(rhema: &Rhema, strict: bool) -> RhemaResult<()> {
    println!("🔒 Validating lock file only...");
    println!("{}", "─".repeat(80));

    let lock_errors = validate_lock_file(rhema, strict)?;

    println!("{}", "─".repeat(80));
    println!("📊 Lock File Validation Summary:");
    println!("  🔒 Lock file errors: {}", lock_errors.len().to_string().red());

    if !lock_errors.is_empty() {
        println!("\n❌ Lock File Validation Errors:");
        for (i, error) in lock_errors.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().red(), error);
        }
        return Err(crate::RhemaError::SchemaValidation(format!(
            "Lock file validation failed with {} errors",
            lock_errors.len()
        )));
    } else {
        println!("🎉 Lock file validation passed!");
    }

    Ok(())
}

/// Comprehensive lock file validation
fn validate_lock_file(rhema: &Rhema, strict: bool) -> RhemaResult<Vec<String>> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 1. Check if lock file exists
    let lock_file_path = rhema.repo_root.join("rhema.lock");
    if !lock_file_path.exists() {
        errors.push("Lock file (rhema.lock) does not exist".to_string());
        return Ok(errors);
    }

    // 2. Parse and validate lock file structure
    let lock_content = std::fs::read_to_string(&lock_file_path)
        .map_err(|e| crate::RhemaError::IoError(e))?;

    let lock_file: RhemaLock = serde_yaml::from_str(&lock_content)
        .map_err(|e| crate::RhemaError::InvalidYaml {
            file: lock_file_path.display().to_string(),
            message: e.to_string(),
        })?;

    // 3. Validate all scopes in lock file exist
    println!("  🔍 Checking scope existence...");
    for (scope_path, locked_scope) in &lock_file.scopes {
        let scope_dir = rhema.repo_root.join(scope_path);
        if !scope_dir.exists() {
            errors.push(format!(
                "Scope '{}' in lock file does not exist in filesystem",
                scope_path
            ));
            continue;
        }

        // Check if scope file exists
        if let Some(scope_file) = find_scope_file(&scope_dir) {
            let scope_content = std::fs::read_to_string(&scope_file)
                .map_err(|e| crate::RhemaError::IoError(e))?;

            let current_scope: crate::RhemaScope = serde_yaml::from_str(&scope_content)
                .map_err(|e| crate::RhemaError::InvalidYaml {
                    file: scope_file.display().to_string(),
                    message: e.to_string(),
                })?;

            // Validate scope checksum
            let current_checksum = calculate_scope_checksum(&scope_dir)?;
            if let Some(source_checksum) = &locked_scope.source_checksum {
                if current_checksum != *source_checksum {
                    errors.push(format!(
                        "Scope '{}' checksum mismatch: expected {}, got {}",
                        scope_path, source_checksum, current_checksum
                    ));
                }
            }

            // 4. Validate all dependencies in lock file are valid
            println!("    🔍 Validating dependencies for scope '{}'...", scope_path);
            for (dep_path, locked_dep) in &locked_scope.dependencies {
                // Check if dependency exists
                let dep_dir = rhema.repo_root.join(dep_path);
                if !dep_dir.exists() {
                    errors.push(format!(
                        "Dependency '{}' in scope '{}' does not exist",
                        dep_path, scope_path
                    ));
                    continue;
                }

                // Validate dependency checksum
                let dep_checksum = calculate_scope_checksum(&dep_dir)?;
                if dep_checksum != locked_dep.checksum {
                    errors.push(format!(
                        "Dependency '{}' in scope '{}' checksum mismatch: expected {}, got {}",
                        dep_path, scope_path, locked_dep.checksum, dep_checksum
                    ));
                }

                // Check dependency type consistency
                if let Some(dep_scope_file) = find_scope_file(&dep_dir) {
                    let dep_content = std::fs::read_to_string(&dep_scope_file)
                        .map_err(|e| crate::RhemaError::IoError(e))?;

                    let dep_scope: crate::RhemaScope = serde_yaml::from_str(&dep_content)
                        .map_err(|e| crate::RhemaError::InvalidYaml {
                            file: dep_scope_file.display().to_string(),
                            message: e.to_string(),
                        })?;

                    if format!("{:?}", locked_dep.dependency_type) != dep_scope.scope_type {
                        errors.push(format!(
                            "Dependency type mismatch for '{}' in scope '{}': locked={:?}, current={}",
                            dep_path, scope_path, locked_dep.dependency_type, dep_scope.scope_type
                        ));
                    }
                }
            }
        } else {
            errors.push(format!(
                "Scope file not found for scope '{}' in lock file",
                scope_path
            ));
        }
    }

    // 5. Check for circular dependencies
    println!("  🔍 Checking for circular dependencies...");
    let circular_deps = detect_circular_dependencies(&lock_file)?;
    for cycle in circular_deps {
        errors.push(format!("Circular dependency detected: {}", cycle.join(" -> ")));
    }

    // 6. Validate version constraints
    println!("  🔍 Validating version constraints...");
    for (scope_path, locked_scope) in &lock_file.scopes {
        for (dep_path, locked_dep) in &locked_scope.dependencies {
            if let Some(version_constraint) = &locked_dep.original_constraint {
                // Check if version constraint is satisfied
                if !is_version_constraint_satisfied(dep_path, version_constraint, rhema)? {
                    errors.push(format!(
                        "Version constraint '{}' not satisfied for dependency '{}' in scope '{}'",
                        version_constraint, dep_path, scope_path
                    ));
                }
            }
        }
    }

    // 7. Check lock file age (warnings only)
    println!("  🔍 Checking lock file age...");
    let lock_metadata = std::fs::metadata(&lock_file_path)
        .map_err(|e| crate::RhemaError::IoError(e))?;
    let modified_time = lock_metadata.modified()
        .map_err(|e| crate::RhemaError::IoError(e))?;
    let lock_modified: chrono::DateTime<Utc> = chrono::DateTime::from(modified_time);
    let now = Utc::now();
    let age = now.signed_duration_since(lock_modified);

    if age > chrono::Duration::days(30) {
        let warning = format!(
            "Lock file is {} days old (last modified: {})",
            age.num_days(),
            lock_modified.format("%Y-%m-%d %H:%M:%S")
        );
        if strict {
            errors.push(warning);
        } else {
            warnings.push(warning);
        }
    }

    // Print warnings if not in strict mode
    if !warnings.is_empty() && !strict {
        println!("  ⚠️  Warnings:");
        for warning in &warnings {
            println!("    {}", warning.yellow());
        }
    }

    // Combine errors and warnings if in strict mode
    if strict {
        errors.extend(warnings);
    }

    Ok(errors)
}

/// Calculate checksum for a scope directory
fn calculate_scope_checksum(scope_dir: &Path) -> RhemaResult<String> {
    let mut hasher = Sha256::new();
    
    // Walk through all files in the scope directory
    for entry in WalkDir::new(scope_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.is_file() {
            // Add file path to hash
            hasher.update(path.to_string_lossy().as_bytes());
            
            // Add file content to hash
            if let Ok(content) = std::fs::read(path) {
                hasher.update(&content);
            }
        }
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

/// Detect circular dependencies in the lock file
fn detect_circular_dependencies(lock_file: &RhemaLock) -> RhemaResult<Vec<Vec<String>>> {
    let mut cycles = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut rec_stack = std::collections::HashSet::new();

    for scope_path in lock_file.scopes.keys() {
        if !visited.contains(scope_path) {
            let mut path = Vec::new();
            if has_cycle_dfs(
                scope_path,
                lock_file,
                &mut visited,
                &mut rec_stack,
                &mut path,
                &mut cycles,
            ) {
                // Cycle detected, but we continue to find all cycles
            }
        }
    }

    Ok(cycles)
}

/// DFS to detect cycles in dependency graph
fn has_cycle_dfs(
    scope_path: &str,
    lock_file: &RhemaLock,
    visited: &mut std::collections::HashSet<String>,
    rec_stack: &mut std::collections::HashSet<String>,
    path: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) -> bool {
    visited.insert(scope_path.to_string());
    rec_stack.insert(scope_path.to_string());
    path.push(scope_path.to_string());

    if let Some(locked_scope) = lock_file.scopes.get(scope_path) {
        for dep_path in locked_scope.dependencies.keys() {
            if !visited.contains(dep_path) {
                if has_cycle_dfs(dep_path, lock_file, visited, rec_stack, path, cycles) {
                    return true;
                }
            } else if rec_stack.contains(dep_path) {
                // Found a cycle
                let cycle_start = path.iter().position(|p| p == dep_path).unwrap_or(0);
                let cycle = path[cycle_start..].to_vec();
                cycles.push(cycle);
            }
        }
    }

    rec_stack.remove(scope_path);
    path.pop();
    false
}

/// Check if version constraint is satisfied
fn is_version_constraint_satisfied(
    dep_path: &str,
    version_constraint: &str,
    rhema: &Rhema,
) -> RhemaResult<bool> {
    // For now, we'll implement a simple version constraint checker
    // This can be enhanced to support semantic versioning
    
    let dep_dir = rhema.repo_root.join(dep_path);
    if let Some(scope_file) = find_scope_file(&dep_dir) {
        let content = std::fs::read_to_string(&scope_file)
            .map_err(|e| crate::RhemaError::IoError(e))?;

        let scope: crate::RhemaScope = serde_yaml::from_str(&content)
            .map_err(|e| crate::RhemaError::InvalidYaml {
                file: scope_file.display().to_string(),
                message: e.to_string(),
            })?;

        // Implement proper semantic versioning constraint parsing
        return parse_and_check_version_constraint(&scope.version, version_constraint);
    }

    // If no version found, assume constraint is not satisfied
    Ok(false)
}

/// Validate a scope against the lock file for comprehensive validation
fn validate_scope_against_lock_file(
    scope_path: &Path,
    scope_file: &Path,
) -> RhemaResult<Vec<String>> {
    let mut errors = Vec::new();
    
    // Try to find the lock file in the repository root
    let lock_file_path = find_lock_file_path(scope_path)?;
    if !lock_file_path.exists() {
        // No lock file found, this is not an error but a warning
        return Ok(vec!["No lock file found for comprehensive validation".to_string()]);
    }

    // Load the lock file
    let lock_content = std::fs::read_to_string(&lock_file_path)
        .map_err(|e| crate::RhemaError::IoError(e))?;

    let lock_file: RhemaLock = serde_yaml::from_str(&lock_content)
        .map_err(|e| crate::RhemaError::InvalidYaml {
            file: lock_file_path.display().to_string(),
            message: e.to_string(),
        })?;

    // Get the relative path of this scope from the repository root
    let repo_root = find_repository_root(scope_path)?;
    let relative_scope_path = scope_path
        .strip_prefix(&repo_root)
        .map_err(|_| crate::RhemaError::ConfigError(
            "Failed to determine relative scope path".to_string()
        ))?;

    let scope_path_str = relative_scope_path.to_string_lossy();

    // Check if this scope is in the lock file
    if let Some(locked_scope) = lock_file.scopes.get(&scope_path_str) {
        // Validate scope checksum
        let current_checksum = calculate_scope_checksum(scope_path)?;
        if let Some(source_checksum) = &locked_scope.source_checksum {
            if current_checksum != *source_checksum {
                errors.push(format!(
                    "Scope checksum mismatch: expected {}, got {}",
                    source_checksum, current_checksum
                ));
            }
        }

        // Validate scope file content
        let scope_content = std::fs::read_to_string(scope_file)
            .map_err(|e| crate::RhemaError::IoError(e))?;

        let current_scope: crate::RhemaScope = serde_yaml::from_str(&scope_content)
            .map_err(|e| crate::RhemaError::InvalidYaml {
                file: scope_file.display().to_string(),
                message: e.to_string(),
            })?;

        // Validate dependencies
        for (dep_path, locked_dep) in &locked_scope.dependencies {
            let dep_dir = repo_root.join(dep_path);
            if !dep_dir.exists() {
                errors.push(format!(
                    "Locked dependency '{}' does not exist",
                    dep_path
                ));
                continue;
            }

            // Check dependency checksum
            let dep_checksum = calculate_scope_checksum(&dep_dir)?;
            if dep_checksum != locked_dep.checksum {
                errors.push(format!(
                    "Dependency '{}' checksum mismatch: expected {}, got {}",
                    dep_path, locked_dep.checksum, dep_checksum
                ));
            }

            // Validate dependency type consistency
            if let Some(dep_scope_file) = find_scope_file(&dep_dir) {
                let dep_content = std::fs::read_to_string(&dep_scope_file)
                    .map_err(|e| crate::RhemaError::IoError(e))?;

                let dep_scope: crate::RhemaScope = serde_yaml::from_str(&dep_content)
                    .map_err(|e| crate::RhemaError::InvalidYaml {
                        file: dep_scope_file.display().to_string(),
                        message: e.to_string(),
                    })?;

                if format!("{:?}", locked_dep.dependency_type) != dep_scope.scope_type {
                    errors.push(format!(
                        "Dependency type mismatch for '{}': locked={:?}, current={}",
                        dep_path, locked_dep.dependency_type, dep_scope.scope_type
                    ));
                }
            }
        }
    } else {
        errors.push("Scope not found in lock file".to_string());
    }

    Ok(errors)
}

/// Find the lock file path by walking up the directory tree
fn find_lock_file_path(scope_path: &Path) -> RhemaResult<std::path::PathBuf> {
    let mut current = scope_path;
    
    loop {
        let lock_file = current.join("rhema.lock");
        if lock_file.exists() {
            return Ok(lock_file);
        }
        
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            return Err(crate::RhemaError::ConfigError(
                "No lock file found in directory tree".to_string()
            ));
        }
    }
}

/// Find the repository root by looking for rhema.lock file
fn find_repository_root(scope_path: &Path) -> RhemaResult<std::path::PathBuf> {
    let mut current = scope_path;
    
    loop {
        let lock_file = current.join("rhema.lock");
        if lock_file.exists() {
            return Ok(current.to_path_buf());
        }
        
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            return Err(crate::RhemaError::ConfigError(
                "No repository root found (no rhema.lock file)".to_string()
            ));
        }
    }
}

/// Parse and check version constraint using semantic versioning
fn parse_and_check_version_constraint(
    current_version: &str,
    version_constraint: &str,
) -> RhemaResult<bool> {
    // Parse the current version
    let current_semver = parse_semver(current_version)?;
    
    // Parse the constraint
    let constraint = parse_version_constraint(version_constraint)?;
    
    // Check if the current version satisfies the constraint
    Ok(constraint.matches(&current_semver))
}

/// Simple semantic version structure
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
    build: Option<String>,
}

/// Version constraint types
#[derive(Debug, Clone)]
enum VersionConstraint {
    Exact(String),
    GreaterThan(String),
    GreaterThanOrEqual(String),
    LessThan(String),
    LessThanOrEqual(String),
    Range(String, String), // inclusive
    Compatible(String), // ^1.2.3 means >=1.2.3,<2.0.0
}

impl VersionConstraint {
    fn matches(&self, version: &SemVer) -> bool {
        match self {
            VersionConstraint::Exact(constraint) => {
                if let Ok(constraint_semver) = parse_semver(constraint) {
                    version == &constraint_semver
                } else {
                    false
                }
            }
            VersionConstraint::GreaterThan(constraint) => {
                if let Ok(constraint_semver) = parse_semver(constraint) {
                    version > &constraint_semver
                } else {
                    false
                }
            }
            VersionConstraint::GreaterThanOrEqual(constraint) => {
                if let Ok(constraint_semver) = parse_semver(constraint) {
                    version >= &constraint_semver
                } else {
                    false
                }
            }
            VersionConstraint::LessThan(constraint) => {
                if let Ok(constraint_semver) = parse_semver(constraint) {
                    version < &constraint_semver
                } else {
                    false
                }
            }
            VersionConstraint::LessThanOrEqual(constraint) => {
                if let Ok(constraint_semver) = parse_semver(constraint) {
                    version <= &constraint_semver
                } else {
                    false
                }
            }
            VersionConstraint::Range(min, max) => {
                if let (Ok(min_semver), Ok(max_semver)) = (parse_semver(min), parse_semver(max)) {
                    version >= &min_semver && version <= &max_semver
                } else {
                    false
                }
            }
            VersionConstraint::Compatible(constraint) => {
                if let Ok(constraint_semver) = parse_semver(constraint) {
                    // Compatible means >= current version and < next major version
                    let next_major = SemVer {
                        major: constraint_semver.major + 1,
                        minor: 0,
                        patch: 0,
                        pre_release: None,
                        build: None,
                    };
                    version >= &constraint_semver && version < &next_major
                } else {
                    false
                }
            }
        }
    }
}

/// Parse semantic version string
fn parse_semver(version: &str) -> RhemaResult<SemVer> {
    // Remove leading 'v' if present
    let version = version.trim_start_matches('v');
    
    // Split on dots and handle pre-release/build metadata
    let parts: Vec<&str> = version.split('-').collect();
    let version_part = parts[0];
    let pre_release = if parts.len() > 1 {
        let pre_parts: Vec<&str> = parts[1].split('+').collect();
        if pre_parts.len() > 1 {
            // Has both pre-release and build metadata
            Some(pre_parts[0].to_string())
        } else {
            // Only pre-release
            Some(parts[1].to_string())
        }
    } else {
        None
    };
    
    let build = if parts.len() > 1 {
        let pre_parts: Vec<&str> = parts[1].split('+').collect();
        if pre_parts.len() > 1 {
            Some(pre_parts[1].to_string())
        } else {
            None
        }
    } else {
        None
    };
    
    let version_numbers: Vec<&str> = version_part.split('.').collect();
    if version_numbers.len() < 3 {
        return Err(crate::RhemaError::SchemaValidation(
            format!("Invalid semantic version format: {}", version)
        ));
    }
    
    let major = version_numbers[0].parse::<u32>()
        .map_err(|_| crate::RhemaError::SchemaValidation(
            format!("Invalid major version: {}", version_numbers[0])
        ))?;
    let minor = version_numbers[1].parse::<u32>()
        .map_err(|_| crate::RhemaError::SchemaValidation(
            format!("Invalid minor version: {}", version_numbers[1])
        ))?;
    let patch = version_numbers[2].parse::<u32>()
        .map_err(|_| crate::RhemaError::SchemaValidation(
            format!("Invalid patch version: {}", version_numbers[2])
        ))?;
    
    Ok(SemVer {
        major,
        minor,
        patch,
        pre_release,
        build,
    })
}

/// Parse version constraint string
fn parse_version_constraint(constraint: &str) -> RhemaResult<VersionConstraint> {
    let constraint = constraint.trim();
    
    if constraint.starts_with('=') {
        Ok(VersionConstraint::Exact(constraint[1..].to_string()))
    } else if constraint.starts_with('>') {
        if constraint.starts_with(">=") {
            Ok(VersionConstraint::GreaterThanOrEqual(constraint[2..].to_string()))
        } else {
            Ok(VersionConstraint::GreaterThan(constraint[1..].to_string()))
        }
    } else if constraint.starts_with('<') {
        if constraint.starts_with("<=") {
            Ok(VersionConstraint::LessThanOrEqual(constraint[2..].to_string()))
        } else {
            Ok(VersionConstraint::LessThan(constraint[1..].to_string()))
        }
    } else if constraint.starts_with('^') {
        Ok(VersionConstraint::Compatible(constraint[1..].to_string()))
    } else if constraint.contains(" - ") {
        let parts: Vec<&str> = constraint.split(" - ").collect();
        if parts.len() == 2 {
            Ok(VersionConstraint::Range(parts[0].to_string(), parts[1].to_string()))
        } else {
            Err(crate::RhemaError::SchemaValidation(
                format!("Invalid range constraint format: {}", constraint)
            ))
        }
    } else {
        // Default to exact match
        Ok(VersionConstraint::Exact(constraint.to_string()))
    }
}
