use crate::{Gacp, GacpResult, schema::{Validatable, SchemaMigratable, JsonSchema, CURRENT_SCHEMA_VERSION}};
use colored::*;
use serde_yaml;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(gacp: &Gacp, recursive: bool, json_schema: bool, migrate: bool) -> GacpResult<()> {
    println!("🔍 Validating GACP context files...");
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
        let scopes = gacp.discover_scopes()?;
        
        for scope in scopes {
            println!("📁 Validating scope: {}", scope.definition.name.bright_blue());
            let (scope_files, scope_valid, scope_errors, scope_migrations) = validate_scope(&scope.path, migrate)?;
            total_files += scope_files;
            valid_files += scope_valid;
            errors.extend(scope_errors);
            migrations_performed += scope_migrations;
        }
    } else {
        // Validate only the current scope
        let current_dir = std::env::current_dir()
            .map_err(|e| crate::GacpError::IoError(e))?;
        
        let scopes = gacp.discover_scopes()?;
        let scope = crate::scope::find_nearest_scope(&current_dir, &scopes)
            .ok_or_else(|| crate::GacpError::ConfigError("No GACP scope found in current directory or parent directories".to_string()))?;
        
        println!("📁 Validating scope: {}", scope.definition.name.bright_blue());
        let (scope_files, scope_valid, scope_errors, scope_migrations) = validate_scope(&scope.path, migrate)?;
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
        println!("  🔄 Migrations performed: {}", migrations_performed.to_string().yellow());
    }
    
    if !errors.is_empty() {
        println!("\n❌ Validation Errors:");
        for (i, error) in errors.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().red(), error);
        }
        return Err(crate::GacpError::SchemaValidation(
            format!("Validation failed with {} errors", errors.len())
        ));
    } else {
        println!("🎉 All files are valid!");
        if migrations_performed > 0 {
            println!("🔄 Schema migrations completed successfully!");
        }
    }
    
    Ok(())
}

fn validate_scope(scope_path: &Path, migrate: bool) -> GacpResult<(usize, usize, Vec<String>, usize)> {
    let mut total_files = 0;
    let mut valid_files = 0;
    let mut errors = Vec::new();
    let mut migrations_performed = 0;
    
    // Validate the scope definition itself
    let gacp_file = scope_path.join("gacp.yaml");
    if gacp_file.exists() {
        total_files += 1;
        match validate_gacp_file(&gacp_file, migrate) {
            Ok(migrations) => {
                valid_files += 1;
                migrations_performed += migrations;
                println!("  ✅ gacp.yaml");
                if migrations > 0 {
                    println!("    🔄 Schema migrated to version {}", CURRENT_SCHEMA_VERSION.yellow());
                }
            }
            Err(e) => {
                errors.push(format!("gacp.yaml: {}", e));
                println!("  ❌ gacp.yaml: {}", e);
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
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            // Skip gacp.yaml as we already validated it
            if file_name == "gacp.yaml" {
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

fn validate_gacp_file(file_path: &Path, migrate: bool) -> GacpResult<usize> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::GacpError::IoError(e))?;
    
    let mut scope: crate::GacpScope = serde_yaml::from_str(&content)
        .map_err(|e| crate::GacpError::InvalidYaml {
            file: file_path.display().to_string(),
            message: e.to_string(),
        })?;
    
    let mut migrations = 0;
    
    // Perform schema migration if requested
    if migrate {
        match scope.migrate_to_latest() {
            Ok(()) => {
                // Write the migrated content back to file
                let migrated_content = serde_yaml::to_string(&scope)
                    .map_err(|e| crate::GacpError::InvalidYaml {
                        file: file_path.display().to_string(),
                        message: format!("Failed to serialize migrated scope: {}", e),
                    })?;
                
                std::fs::write(file_path, migrated_content)
                    .map_err(|e| crate::GacpError::IoError(e))?;
                
                migrations = 1;
            }
            Err(e) => {
                return Err(crate::GacpError::SchemaValidation(
                    format!("Schema migration failed: {}", e)
                ));
            }
        }
    }
    
    // Validate the scope definition with enhanced validation
    Validatable::validate(&scope)?;
    
    Ok(migrations)
}

fn validate_context_file(file_path: &Path) -> GacpResult<()> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::GacpError::IoError(e))?;
    
    let file_name = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    // Try to parse as different context types with enhanced validation
    match file_name {
        "todos.yaml" => {
            let todos: crate::Todos = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&todos)?;
        }
        "knowledge.yaml" => {
            let knowledge: crate::Knowledge = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&knowledge)?;
        }
        "patterns.yaml" => {
            let patterns: crate::Patterns = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&patterns)?;
        }
        "decisions.yaml" => {
            let decisions: crate::Decisions = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&decisions)?;
        }
        "conventions.yaml" => {
            let conventions: crate::Conventions = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&conventions)?;
        }
        _ => {
            // For unknown files, just validate that they're valid YAML
            let _: serde_yaml::Value = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
        }
    }
    
    Ok(())
}

fn print_json_schemas() {
    println!("📋 JSON Schemas for GACP Context Files");
    println!("{}", "─".repeat(80));
    
    println!("🔧 GACP Scope Schema:");
    println!("{}", serde_json::to_string_pretty(&crate::GacpScope::json_schema()).unwrap());
    
    println!("\n📚 Knowledge Schema:");
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
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
    })).unwrap());
    
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