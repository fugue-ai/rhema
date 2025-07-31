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

use crate::{Rhema, RhemaResult, RhemaScope};
use colored::*;
use std::fs;
// use std::path::PathBuf;

/// Find the scope file in the given directory, checking multiple possible locations
fn find_scope_file(scope_path: &std::path::Path) -> Option<std::path::PathBuf> {
    // Define the possible locations in order of preference
    let possible_locations = [
        scope_path.join("rhema.yaml"),
        scope_path.join("scope.yaml"),
    ];
    
    // Check if we're in a .rhema directory, then also check parent directory
    let parent_locations = if scope_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
        let parent = scope_path.parent().unwrap_or(scope_path);
        vec![
            parent.join("rhema.yaml"),
            parent.join("scope.yaml"),
        ]
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

pub fn run(rhema: &Rhema, _recursive: bool, dry_run: bool) -> RhemaResult<()> {
    let scopes = rhema.list_scopes()?;
    
    if scopes.is_empty() {
        println!("{}", "No scopes found to migrate".yellow());
        return Ok(());
    }
    
    println!("{}", "🔄 Starting Rhema schema migration...".blue());
    if dry_run {
        println!("{}", "  DRY RUN MODE - No files will be modified".yellow());
    }
    println!();
    
    let mut migrated_count = 0;
    let mut error_count = 0;
    
    for scope in &scopes {
        match migrate_scope(&rhema, &scope.definition, dry_run) {
            Ok(migrated) => {
                if migrated {
                    migrated_count += 1;
                    println!("  ✓ Migrated scope: {}", scope.definition.name.green());
                } else {
                    println!("  - No migration needed: {}", scope.definition.name.yellow());
                }
            }
            Err(e) => {
                error_count += 1;
                println!("  ✗ Error migrating scope {}: {}", scope.definition.name.red(), e.to_string().red());
            }
        }
    }
    
    println!();
    println!("{}", "Migration Summary:".blue());
    println!("  Scopes processed: {}", scopes.len());
    println!("  Successfully migrated: {}", migrated_count.to_string().green());
    println!("  No migration needed: {}", (scopes.len() - migrated_count - error_count).to_string().yellow());
    if error_count > 0 {
        println!("  Errors: {}", error_count.to_string().red());
    }
    
    if migrated_count > 0 && !dry_run {
        println!();
        println!("{}", "✅ Migration completed successfully!".green());
        println!("  Next steps:");
        println!("    1. Review migrated files");
        println!("    2. Run 'rhema validate' to ensure everything is correct");
        println!("    3. Update protocol information as needed");
    }
    
    Ok(())
}

/// Migrate a single scope
fn migrate_scope(rhema: &Rhema, scope: &RhemaScope, dry_run: bool) -> RhemaResult<bool> {
    let scope_path = rhema.scope_path(&scope.name)?;
    
    let rhema_file = match find_scope_file(&scope_path) {
        Some(file) => file,
        None => return Ok(false),
    };
    
    // Read current rhema.yaml
    let content = fs::read_to_string(&rhema_file)?;
    let mut rhema_scope: RhemaScope = serde_yaml::from_str(&content)?;
    
    let mut migrated = false;
    
    // Check if protocol info needs to be added
    if rhema_scope.protocol_info.is_none() {
        println!("    Adding protocol information...");
        if !dry_run {
            rhema_scope.protocol_info = Some(create_default_protocol_info(&rhema_scope.scope_type));
            migrated = true;
        }
    }
    
    // Check if schema version needs to be updated
    if rhema_scope.schema_version.is_none() || rhema_scope.schema_version.as_ref().unwrap() != crate::CURRENT_SCHEMA_VERSION {
        println!("    Updating schema version...");
        if !dry_run {
            rhema_scope.schema_version = Some(crate::CURRENT_SCHEMA_VERSION.to_string());
            migrated = true;
        }
    }
    
    // Write back if migrated
    if migrated && !dry_run {
        let updated_content = serde_yaml::to_string(&rhema_scope)?;
        fs::write(&rhema_file, updated_content)?;
    }
    
    Ok(migrated)
}

/// Create default protocol information for migration
fn create_default_protocol_info(scope_type: &str) -> rhema_core::schema::ProtocolInfo {
    let concepts = vec![
        rhema_core::schema::ConceptDefinition {
            name: "Scope".to_string(),
            description: "A Rhema scope represents a logical unit of the codebase with its own context and responsibilities.".to_string(),
            related: Some(vec!["Dependencies".to_string(), "Patterns".to_string()]),
            examples: Some(vec![
                "A microservice with its own API and business logic".to_string(),
                "A frontend application with UI components".to_string(),
                "A shared library with utility functions".to_string(),
            ]),
        },
        rhema_core::schema::ConceptDefinition {
            name: "Knowledge".to_string(),
            description: "Structured information about the codebase, including insights, patterns, and best practices.".to_string(),
            related: Some(vec!["Patterns".to_string(), "Decisions".to_string()]),
            examples: Some(vec![
                "API documentation and usage examples".to_string(),
                "Architecture patterns and implementation details".to_string(),
                "Performance optimization techniques".to_string(),
            ]),
        },
        rhema_core::schema::ConceptDefinition {
            name: "CQL".to_string(),
            description: "Context Query Language for querying Rhema data structures.".to_string(),
            related: Some(vec!["Knowledge".to_string(), "Patterns".to_string()]),
            examples: Some(vec![
                "SELECT * FROM knowledge WHERE category = 'api'".to_string(),
                "SELECT * FROM patterns WHERE pattern_type = 'security'".to_string(),
                "SELECT * FROM decisions WHERE status = 'approved'".to_string(),
            ]),
        },
    ];
    
    let cql_examples = vec![
        rhema_core::schema::CqlExample {
            name: "Find API Knowledge".to_string(),
            query: "SELECT * FROM knowledge WHERE category = 'api'".to_string(),
            description: "Retrieve all knowledge entries related to API documentation and usage".to_string(),
            output_format: Some("JSON array of knowledge entries".to_string()),
            use_case: Some("Code review and development".to_string()),
        },
        rhema_core::schema::CqlExample {
            name: "Find Security Patterns".to_string(),
            query: "SELECT * FROM patterns WHERE pattern_type = 'security'".to_string(),
            description: "Retrieve all security-related patterns and best practices".to_string(),
            output_format: Some("JSON array of pattern entries".to_string()),
            use_case: Some("Security review and implementation".to_string()),
        },
        rhema_core::schema::CqlExample {
            name: "Find Approved Decisions".to_string(),
            query: "SELECT * FROM decisions WHERE status = 'approved'".to_string(),
            description: "Retrieve all approved architectural and design decisions".to_string(),
            output_format: Some("JSON array of decision entries".to_string()),
            use_case: Some("Architecture review and planning".to_string()),
        },
    ];
    
    let patterns = vec![
        rhema_core::schema::PatternDefinition {
            name: "Error Handling".to_string(),
            description: "Standardized approach to error handling across the scope".to_string(),
            when_to_use: Some("When implementing functions that may fail or need to report errors".to_string()),
            examples: Some(vec![
                "Use Result<T, E> for functions that can fail".to_string(),
                "Log errors with appropriate context and severity".to_string(),
                "Return meaningful error messages to users".to_string(),
            ]),
        },
        rhema_core::schema::PatternDefinition {
            name: "Configuration Management".to_string(),
            description: "Environment-based configuration management".to_string(),
            when_to_use: Some("When the scope needs different settings for different environments".to_string()),
            examples: Some(vec![
                "Use environment variables for sensitive configuration".to_string(),
                "Provide sensible defaults for all configuration options".to_string(),
                "Validate configuration on startup".to_string(),
            ]),
        },
    ];
    
    let integrations = vec![
        rhema_core::schema::IntegrationGuide {
            name: "IDE Integration".to_string(),
            description: "Integrate Rhema with your development environment".to_string(),
            setup: Some(vec![
                "Install Rhema CLI".to_string(),
                "Configure IDE extensions".to_string(),
                "Set up workspace settings".to_string(),
            ]),
            configuration: Some(vec![
                "Add Rhema commands to IDE command palette".to_string(),
                "Configure file watching for auto-sync".to_string(),
            ]),
            best_practices: Some(vec![
                "Use Rhema commands from IDE for consistency".to_string(),
                "Enable auto-validation on save".to_string(),
            ]),
        },
    ];
    
    let troubleshooting = vec![
        rhema_core::schema::TroubleshootingItem {
            issue: "Configuration validation fails".to_string(),
            description: "Rhema configuration files have validation errors".to_string(),
            solution: vec![
                "Run `rhema validate` to identify issues".to_string(),
                "Check YAML syntax and required fields".to_string(),
                "Review schema documentation".to_string(),
            ],
            prevention: Some(vec![
                "Use `rhema validate` before committing changes".to_string(),
                "Follow schema documentation".to_string(),
            ]),
        },
    ];
    
    rhema_core::schema::ProtocolInfo {
        version: "1.0.0".to_string(),
        description: Some(format!("Protocol information for {} scope (migrated)", scope_type)),
        concepts: Some(concepts),
        cql_examples: Some(cql_examples),
        patterns: Some(patterns),
        integrations: Some(integrations),
        troubleshooting: Some(troubleshooting),
        custom: std::collections::HashMap::new(),
    }
} 
