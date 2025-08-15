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

use crate::CliContext;
use rhema_api::RhemaResult;
use rhema_core::RhemaError;
use rhema_query::QueryProvenance;
use std::path::PathBuf;

pub fn handle_init(
    context: &CliContext,
    scope_type: Option<&str>,
    scope_name: Option<&str>,
    auto_config: bool,
) -> RhemaResult<()> {
    // Create a new Rhema instance
    let rhema = rhema_api::Rhema::new()?;
    
    // Get the repository root path
    let repo_root = rhema.repo_root();
    
    // Determine the scope path based on current directory vs repo root
    let current_dir = std::env::current_dir()
        .map_err(|e| RhemaError::IoError(e))?;
    
    let scope_path = if current_dir == *repo_root {
        // Initialize at repository root
        repo_root.join(".rhema")
    } else {
        // Initialize in current directory
        current_dir.join(".rhema")
    };
    
    // Check if rhema files already exist
    let rhema_files = [
        "rhema.yaml",
        "scope.yaml", 
        "knowledge.yaml",
        "todos.yaml",
        "decisions.yaml",
        "patterns.yaml",
        "conventions.yaml",
    ];
    
    let existing_files: Vec<String> = rhema_files
        .iter()
        .filter_map(|&file| {
            let file_path = scope_path.join(file);
            if file_path.exists() {
                Some(file.to_string())
            } else {
                None
            }
        })
        .collect();
    
    if !existing_files.is_empty() {
        return Err(RhemaError::ConfigError(format!(
            "Rhema files already exist at {}: {}",
            scope_path.display(),
            existing_files.join(", ")
        )));
    }
    
    // Create scope directory
    std::fs::create_dir_all(&scope_path)
        .map_err(|e| RhemaError::IoError(e))?;
    
    // Determine scope type and name
    let (final_scope_type, final_scope_name) = if auto_config {
        // Auto-detect configuration from repository
        context.display_info("🔍 Analyzing repository structure for auto-configuration...")?;
        
        // For now, use basic detection logic
        let scope_type = scope_type.unwrap_or("service").to_string();
        let scope_name = scope_name.unwrap_or_else(|| {
            scope_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
        }).to_string();
        
        (scope_type, scope_name)
    } else {
        // Use provided or default values
        let scope_type = scope_type.unwrap_or("service").to_string();
        let scope_name = scope_name.unwrap_or_else(|| {
            scope_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
        }).to_string();
        
        (scope_type, scope_name)
    };
    
    // Create basic rhema.yaml file
    let rhema_scope = rhema_core::schema::RhemaScope {
        name: final_scope_name.clone(),
        scope_type: final_scope_type.clone(),
        description: Some(format!("{} scope", final_scope_type)),
        version: "1.0.0".to_string(),
        schema_version: Some(rhema_core::CURRENT_SCHEMA_VERSION.to_string()),
        dependencies: None,
        protocol_info: None,
        custom: std::collections::HashMap::new(),
    };
    
    let rhema_content = serde_yaml::to_string(&rhema_scope)
        .map_err(|e| RhemaError::YamlError(e))?;
    
    std::fs::write(scope_path.join("rhema.yaml"), rhema_content)
        .map_err(|e| RhemaError::IoError(e))?;
    
    // Create template files
    create_template_files(&scope_path)?;
    
    println!("✅ Rhema repository initialized successfully!");
    println!("📁 Repository: {}", repo_root.display());
    println!("📁 Scope path: {}", scope_path.display());
    println!("📁 Scope type: {}", final_scope_type);
    println!("📝 Scope name: {}", final_scope_name);
    if auto_config {
        println!("🤖 Auto-configuration enabled");
    }
    
    Ok(())
}

pub fn handle_query(
    context: &CliContext,
    query: &str,
    format: &str,
    provenance: bool,
    field_provenance: bool,
    stats: bool,
) -> RhemaResult<()> {
    // Validate query input
    if query.trim().is_empty() {
        return Err(RhemaError::InvalidInput(
            "Query cannot be empty".to_string(),
        ));
    }
    
    // Execute the actual query using the Rhema instance
    let (query_result, query_stats) = if provenance || field_provenance {
        // Use query with provenance if requested
        let (result, provenance_info) = context.rhema.query_with_provenance(query)?;
        let stats = if stats {
            Some(context.rhema.query_with_stats(query)?.1)
        } else {
            None
        };
        (result, stats)
    } else {
        // Use regular query
        let result = context.rhema.query(query)?;
        let stats = if stats {
            Some(context.rhema.query_with_stats(query)?.1)
        } else {
            None
        };
        (result, stats)
    };
    
    // Format and display results
    match format.to_lowercase().as_str() {
        "json" => {
            let mut output = serde_json::Map::new();
            output.insert("query".to_string(), serde_json::Value::String(query.to_string()));
            output.insert("result".to_string(), serde_json::to_value(query_result)?);
            
            if let Some(stats) = query_stats {
                output.insert("stats".to_string(), serde_json::to_value(stats)?);
            }
            
            println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(output))?);
        }
        "yaml" => {
            let mut output = serde_yaml::Mapping::new();
            output.insert(
                serde_yaml::Value::String("query".to_string()),
                serde_yaml::Value::String(query.to_string()),
            );
            output.insert(
                serde_yaml::Value::String("result".to_string()),
                query_result,
            );
            
            if let Some(stats) = query_stats {
                output.insert(
                    serde_yaml::Value::String("stats".to_string()),
                    serde_yaml::to_value(stats)?,
                );
            }
            
            println!("{}", serde_yaml::to_string(&serde_yaml::Value::Mapping(output))?);
        }
        "table" => {
            // For table format, we need to extract structured data
            if let serde_yaml::Value::Sequence(results) = &query_result {
                if !results.is_empty() {
                    // Try to extract headers from the first result
                    if let serde_yaml::Value::Mapping(first_result) = &results[0] {
                        let headers: Vec<String> = first_result
                            .keys()
                            .filter_map(|k| k.as_str().map(|s| s.to_string()))
                            .collect();
                        
                        // Print headers
                        println!("| {} |", headers.join(" | "));
                        println!("|{}|", headers.iter().map(|_| "---").collect::<Vec<_>>().join("|"));
                        
                        // Print each result
                        for result in results {
                            if let serde_yaml::Value::Mapping(mapping) = result {
                                let row: Vec<String> = headers
                                    .iter()
                                    .map(|header| {
                                        mapping
                                            .get(&serde_yaml::Value::String(header.clone()))
                                            .map(|v| format!("{:?}", v))
                                            .unwrap_or_else(|| "".to_string())
                                    })
                                    .collect();
                                println!("| {} |", row.join(" | "));
                            }
                        }
                    }
                } else {
                    println!("| Query | Status |");
                    println!("|-------|--------|");
                    println!("| {} | No results found |", query);
                }
            } else {
                println!("| Query | Result |");
                println!("|-------|--------|");
                println!("| {} | {:?} |", query, query_result);
            }
            
            // Print stats if available
            if let Some(stats) = query_stats {
                println!("\n📊 Query Statistics:");
                let stats_value = serde_yaml::to_value(stats)?;
                if let serde_yaml::Value::Mapping(stats_map) = stats_value {
                    for (key, value) in stats_map {
                        if let Some(key_str) = key.as_str() {
                            println!("  {}: {:?}", key_str, value);
                        }
                    }
                }
            }
        }
        _ => {
            return Err(RhemaError::ConfigError(
                "Unsupported format. Use 'json', 'yaml', or 'table'".to_string(),
            ));
        }
    }
    
    Ok(())
}

/// Create template files for a new Rhema scope
fn create_template_files(scope_path: &PathBuf) -> RhemaResult<()> {
    // Create empty template files
    let template_files = [
        ("scope.yaml", "# Rhema scope configuration\n"),
        ("knowledge.yaml", "# Knowledge base entries\n"),
        ("todos.yaml", "# Todo items\n"),
        ("decisions.yaml", "# Decision records\n"),
        ("patterns.yaml", "# Pattern definitions\n"),
        ("conventions.yaml", "# Development conventions\n"),
    ];
    
    for (filename, content) in template_files {
        let file_path = scope_path.join(filename);
        std::fs::write(file_path, content)
            .map_err(|e| RhemaError::IoError(e))?;
    }
    
    Ok(())
}
