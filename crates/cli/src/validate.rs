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

pub fn run(rhema: &Rhema, recursive: bool, json_schema: bool, migrate: bool) -> RhemaResult<()> {
    println!("🔍 Validating Rhema context files...");
    println!("{}", "─".repeat(80));

    if json_schema {
        print_json_schemas();
        return Ok(());
    }

    let mut total_files = 0;
    let mut valid_files = 0;
    let mut errors = Vec::new();
    let mut migrations_performed = 0;

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

    if !errors.is_empty() {
        println!("\n❌ Validation Errors:");
        for (i, error) in errors.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().red(), error);
        }
        return Err(crate::RhemaError::SchemaValidation(format!(
            "Validation failed with {} errors",
            errors.len()
        )));
    } else {
        println!("🎉 All files are valid!");
        if migrations_performed > 0 {
            println!("🔄 Schema migrations completed successfully!");
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
    // TODO: Integrate with lock file system for comprehensive validation
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
