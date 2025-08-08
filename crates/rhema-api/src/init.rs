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
use colored::*;
use std::fs;
use std::path::PathBuf;

pub fn run(
    rhema: &Rhema,
    scope_type: Option<&str>,
    scope_name: Option<&str>,
    auto_config: bool,
) -> RhemaResult<()> {
    let current_dir = std::env::current_dir()?;
    let repo_root = rhema.repo_root();

    // Determine scope path
    let scope_path = if current_dir == *repo_root {
        // Initialize at repository root
        repo_root.join(".rhema")
    } else {
        // Initialize in current directory
        current_dir.join(".rhema")
    };

    // Check if rhema-defined files already exist
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
        return Err(crate::RhemaError::ConfigError(format!(
            "Rhema files already exist at {}: {}",
            scope_path.display(),
            existing_files.join(", ")
        )));
    }

    // Create scope directory
    fs::create_dir_all(&scope_path)?;

    // Determine scope type and name
    let (scope_type, scope_name, description, custom_fields) = if auto_config {
        // Auto-detect configuration from repository
        println!("🔍 Analyzing repository structure for auto-configuration...");
        let analysis = rhema_query::repo_analysis::RepoAnalysis::analyze(&current_dir)?;

        // Display analysis results
        display_analysis_results(&analysis)?;

        (
            analysis.suggested_scope_type,
            analysis.suggested_scope_name,
            analysis.suggested_description,
            analysis.custom_fields,
        )
    } else {
        // Use provided or default values
        let scope_type = scope_type.unwrap_or("service").to_string();
        let scope_name = scope_name
            .unwrap_or_else(|| {
                scope_path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
            })
            .to_string();

        (
            scope_type.clone(),
            scope_name.clone(),
            format!("{} scope", scope_type),
            std::collections::HashMap::new(),
        )
    };

    // Create protocol info
    let protocol_info = create_default_protocol_info(&scope_type);

    // Create rhema.yaml
    let rhema_scope = rhema_core::schema::RhemaScope {
        name: scope_name.clone(),
        scope_type: scope_type.to_string(),
        description: Some(description),
        version: "1.0.0".to_string(),
        schema_version: Some(rhema_core::CURRENT_SCHEMA_VERSION.to_string()),
        dependencies: None,
        protocol_info: Some(protocol_info),
        custom: custom_fields,
    };

    let rhema_content = serde_yaml::to_string(&rhema_scope)?;
    fs::write(scope_path.join("rhema.yaml"), rhema_content)?;

    // Create template files
    create_template_files(&scope_path)?;

    println!("{}", "✓ Rhema scope initialized successfully!".green());
    if auto_config {
        println!("  🎯 Auto-configured based on repository analysis");
    }
    println!("  Scope: {}", scope_name.yellow());
    println!("  Type: {}", scope_type.yellow());
    println!("  Path: {}", scope_path.display().to_string().yellow());
    println!();
    println!("  Next steps:");
    println!("    • Edit .rhema/rhema.yaml to customize scope settings");
    println!("    • Add knowledge entries: rhema insight record \"your insight\"");
    println!("    • Add todo items: rhema todo add \"your todo\"");

    Ok(())
}

/// Display repository analysis results
fn display_analysis_results(
    analysis: &rhema_query::repo_analysis::RepoAnalysis,
) -> RhemaResult<()> {
    println!("\n📊 Repository Analysis Results:");
    println!("{}", "─".repeat(50));

    // Project type
    let project_type_str = match analysis.project_type {
        rhema_query::repo_analysis::ProjectType::Monorepo => "Monorepo",
        rhema_query::repo_analysis::ProjectType::Microservice => "Microservice",
        rhema_query::repo_analysis::ProjectType::Monolithic => "Monolithic",
        rhema_query::repo_analysis::ProjectType::Library => "Library",
        rhema_query::repo_analysis::ProjectType::Application => "Application",
        rhema_query::repo_analysis::ProjectType::Service => "Service",
        rhema_query::repo_analysis::ProjectType::Unknown => "Unknown",
    };
    println!("🏗️  Project Type: {}", project_type_str.green());

    // Languages
    if !analysis.languages.is_empty() {
        println!("💻 Languages: {}", analysis.languages.join(", ").yellow());
    }

    // Frameworks
    if !analysis.frameworks.is_empty() {
        println!("🔧 Frameworks: {}", analysis.frameworks.join(", ").yellow());
    }

    // Databases
    if !analysis.databases.is_empty() {
        println!("🗄️  Databases: {}", analysis.databases.join(", ").yellow());
    }

    // Infrastructure
    if !analysis.infrastructure.is_empty() {
        println!(
            "🏗️  Infrastructure: {}",
            analysis.infrastructure.join(", ").yellow()
        );
    }

    // Recommendations
    println!("\n🎯 Generated Recommendations:");
    println!("  Scope Type: {}", analysis.suggested_scope_type.green());
    println!("  Scope Name: {}", analysis.suggested_scope_name.green());
    println!("  Description: {}", analysis.suggested_description.cyan());

    println!("\n✅ Auto-configuration complete!");
    Ok(())
}

fn create_template_files(scope_path: &PathBuf) -> RhemaResult<()> {
    // Create knowledge.yaml template
    let knowledge_template = r#"# Knowledge Base
# This file contains insights, learnings, and domain knowledge for this scope

entries: []
categories:
  architecture: "System architecture and design decisions"
  patterns: "Design patterns and best practices"
  gotchas: "Common pitfalls and how to avoid them"
  performance: "Performance considerations and optimizations"
"#;
    fs::write(scope_path.join("knowledge.yaml"), knowledge_template)?;

    // Create todos.yaml template
    let todos_template = r#"# Todo Items
# This file tracks work items, tasks, and improvements for this scope

todos: []
"#;
    fs::write(scope_path.join("todos.yaml"), todos_template)?;

    // Create decisions.yaml template
    let decisions_template = r#"# Decisions
# This file records important decisions made for this scope

decisions: []
"#;
    fs::write(scope_path.join("decisions.yaml"), decisions_template)?;

    // Create patterns.yaml template
    let patterns_template = r#"# Patterns
# This file documents design patterns and architectural patterns used in this scope

patterns: []
"#;
    fs::write(scope_path.join("patterns.yaml"), patterns_template)?;

    // Create conventions.yaml template
    let conventions_template = r#"# Conventions
# This file documents coding conventions, naming conventions, and standards

conventions: []
"#;
    fs::write(scope_path.join("conventions.yaml"), conventions_template)?;

    Ok(())
}

/// Create default protocol information
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
            description: "Retrieve all knowledge entries related to API documentation and usage"
                .to_string(),
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
            when_to_use: Some(
                "When implementing functions that may fail or need to report errors".to_string(),
            ),
            examples: Some(vec![
                "Use Result<T, E> for functions that can fail".to_string(),
                "Log errors with appropriate context and severity".to_string(),
                "Return meaningful error messages to users".to_string(),
            ]),
        },
        rhema_core::schema::PatternDefinition {
            name: "Configuration Management".to_string(),
            description: "Environment-based configuration management".to_string(),
            when_to_use: Some(
                "When the scope needs different settings for different environments".to_string(),
            ),
            examples: Some(vec![
                "Use environment variables for sensitive configuration".to_string(),
                "Provide sensible defaults for all configuration options".to_string(),
                "Validate configuration on startup".to_string(),
            ]),
        },
    ];

    let integrations = vec![rhema_core::schema::IntegrationGuide {
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
    }];

    let troubleshooting = vec![rhema_core::schema::TroubleshootingItem {
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
    }];

    rhema_core::schema::ProtocolInfo {
        version: "1.0.0".to_string(),
        description: Some(format!("Protocol information for {} scope", scope_type)),
        concepts: Some(concepts),
        cql_examples: Some(cql_examples),
        patterns: Some(patterns),
        integrations: Some(integrations),
        troubleshooting: Some(troubleshooting),
        custom: std::collections::HashMap::new(),
    }
}
