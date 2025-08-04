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

use rhema::{
    schema::{
        ConceptDefinition, CqlExample, IntegrationGuide, PatternDefinition, ProtocolInfo,
        TroubleshootingItem,
    },
    Rhema, RhemaResult,
};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_protocol_info_creation() -> RhemaResult<()> {
    let protocol_info = create_test_protocol_info();

    // Test basic fields
    assert_eq!(protocol_info.version, "1.0.0");
    assert!(protocol_info.description.is_some());

    // Test concepts
    assert!(protocol_info.concepts.is_some());
    let concepts = protocol_info.concepts.unwrap();
    assert_eq!(concepts.len(), 3);
    assert_eq!(concepts[0].name, "Scope");
    assert_eq!(concepts[1].name, "Knowledge");
    assert_eq!(concepts[2].name, "CQL");

    // Test CQL examples
    assert!(protocol_info.cql_examples.is_some());
    let examples = protocol_info.cql_examples.unwrap();
    assert_eq!(examples.len(), 3);
    assert_eq!(examples[0].name, "Find API Knowledge");
    assert_eq!(examples[1].name, "Find Security Patterns");
    assert_eq!(examples[2].name, "Find Approved Decisions");

    // Test patterns
    assert!(protocol_info.patterns.is_some());
    let patterns = protocol_info.patterns.unwrap();
    assert_eq!(patterns.len(), 2);
    assert_eq!(patterns[0].name, "Error Handling");
    assert_eq!(patterns[1].name, "Configuration Management");

    // Test integrations
    assert!(protocol_info.integrations.is_some());
    let integrations = protocol_info.integrations.unwrap();
    assert_eq!(integrations.len(), 1);
    assert_eq!(integrations[0].name, "IDE Integration");

    // Test troubleshooting
    assert!(protocol_info.troubleshooting.is_some());
    let troubleshooting = protocol_info.troubleshooting.unwrap();
    assert_eq!(troubleshooting.len(), 1);
    assert_eq!(troubleshooting[0].issue, "Configuration validation fails");

    Ok(())
}

#[test]
fn test_protocol_info_validation() -> RhemaResult<()> {
    let protocol_info = create_test_protocol_info();

    // Test validation
    protocol_info.validate()?;

    Ok(())
}

#[test]
fn test_protocol_info_serialization() -> RhemaResult<()> {
    let protocol_info = create_test_protocol_info();

    // Test YAML serialization
    let yaml = serde_yaml::to_string(&protocol_info)?;
    assert!(!yaml.is_empty());
    assert!(yaml.contains("version: 1.0.0"));
    assert!(yaml.contains("Scope"));
    assert!(yaml.contains("Knowledge"));
    assert!(yaml.contains("CQL"));

    // Test JSON serialization
    let json = serde_json::to_string(&protocol_info)?;
    assert!(!json.is_empty());
    assert!(json.contains("\"version\":\"1.0.0\""));
    assert!(json.contains("\"Scope\""));
    assert!(json.contains("\"Knowledge\""));
    assert!(json.contains("\"CQL\""));

    // Test deserialization
    let deserialized: ProtocolInfo = serde_yaml::from_str(&yaml)?;
    assert_eq!(deserialized.version, protocol_info.version);
    assert_eq!(deserialized.description, protocol_info.description);

    Ok(())
}

#[test]
fn test_export_context_functionality() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = setup_test_rhema(&temp_dir)?;

    // Create test scope with protocol info
    let scope_name = "test-service";
    create_test_scope(&rhema, scope_name)?;

    // Test export context
    let output_file = temp_dir.path().join("export.json");
    rhema::commands::export_context::run(
        &rhema,
        "json",
        Some(output_file.to_str().unwrap()),
        None,
        true,  // include_protocol
        true,  // include_knowledge
        true,  // include_todos
        true,  // include_decisions
        true,  // include_patterns
        true,  // include_conventions
        false, // summarize
        false, // ai_agent_format
    )?;

    // Verify export file was created
    assert!(output_file.exists());

    // Read and verify export content
    let content = fs::read_to_string(&output_file)?;
    let export_data: serde_json::Value = serde_json::from_str(&content)?;

    // Check basic structure
    assert!(export_data.get("metadata").is_some());
    assert!(export_data.get("scopes").is_some());
    assert!(export_data.get("protocol_info").is_some());

    // Check scopes
    let scopes = export_data.get("scopes").unwrap().as_array().unwrap();
    assert_eq!(scopes.len(), 1);
    assert_eq!(scopes[0]["name"], scope_name);

    // Check protocol info
    let protocol_info = export_data.get("protocol_info").unwrap();
    assert_eq!(protocol_info["version"], "1.0.0");

    Ok(())
}

#[test]
fn test_primer_generation() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = setup_test_rhema(&temp_dir)?;

    // Create test scope
    let scope_name = "test-app";
    create_test_scope(&rhema, scope_name)?;

    // Test primer generation
    let primer_dir = temp_dir.path().join("primer");
    rhema::commands::primer::run(
        &rhema,
        Some(scope_name),
        Some(primer_dir.to_str().unwrap()),
        Some("app"),
        true, // include_examples
        true, // validate
    )?;

    // Verify primer files were created
    let scope_primer_dir = primer_dir.join(scope_name);
    assert!(scope_primer_dir.exists());

    let primer_yaml = scope_primer_dir.join("primer.yaml");
    let primer_json = scope_primer_dir.join("primer.json");
    let primer_md = scope_primer_dir.join("primer.md");
    let primer_txt = scope_primer_dir.join("primer.txt");

    assert!(primer_yaml.exists());
    assert!(primer_json.exists());
    assert!(primer_md.exists());
    assert!(primer_txt.exists());

    // Verify primer content
    let yaml_content = fs::read_to_string(&primer_yaml)?;
    let primer_data: serde_yaml::Value = serde_yaml::from_str(&yaml_content)?;

    assert_eq!(primer_data["metadata"]["scope_name"], scope_name);
    assert_eq!(primer_data["scope"]["name"], scope_name);
    assert_eq!(primer_data["scope"]["scope_type"], "app");

    Ok(())
}

#[test]
fn test_readme_generation() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = setup_test_rhema(&temp_dir)?;

    // Create test scope
    let scope_name = "test-library";
    create_test_scope(&rhema, scope_name)?;

    // Test README generation
    let readme_file = temp_dir.path().join("README.md");
    rhema::commands::generate_readme::run(
        &rhema,
        Some(scope_name),
        Some(readme_file.to_str().unwrap()),
        Some("library"),
        true, // include_context
        true, // seo_optimized
        None, // custom_sections
    )?;

    // Verify README was created
    assert!(readme_file.exists());

    // Verify README content
    let content = fs::read_to_string(&readme_file)?;
    assert!(content.contains("# test-library"));
    assert!(content.contains("## Installation"));
    assert!(content.contains("## Usage"));
    assert!(content.contains("## Features"));
    assert!(content.contains("## Context Management"));

    Ok(())
}

#[test]
fn test_bootstrap_context() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = setup_test_rhema(&temp_dir)?;

    // Create test scope
    let scope_name = "test-service";
    create_test_scope(&rhema, scope_name)?;

    // Test bootstrap context
    let bootstrap_dir = temp_dir.path().join("bootstrap");
    rhema::commands::bootstrap_context::run(
        &rhema,
        "code_review",
        "json",
        Some(bootstrap_dir.to_str().unwrap()),
        None, // scope_filter
        true, // include_all
        true, // optimize_for_ai
        true, // create_primer
        true, // create_readme
    )?;

    // Verify bootstrap files were created
    assert!(bootstrap_dir.exists());

    let bootstrap_json = bootstrap_dir.join("bootstrap.json");
    let bootstrap_yaml = bootstrap_dir.join("bootstrap.yaml");
    let bootstrap_md = bootstrap_dir.join("bootstrap.md");
    let bootstrap_txt = bootstrap_dir.join("bootstrap.txt");
    let primer_md = bootstrap_dir.join("primer.md");
    let readme_md = bootstrap_dir.join("README.md");

    assert!(bootstrap_json.exists());
    assert!(bootstrap_yaml.exists());
    assert!(bootstrap_md.exists());
    assert!(bootstrap_txt.exists());
    assert!(primer_md.exists());
    assert!(readme_md.exists());

    // Verify bootstrap content
    let json_content = fs::read_to_string(&bootstrap_json)?;
    let bootstrap_data: serde_json::Value = serde_json::from_str(&json_content)?;

    assert_eq!(bootstrap_data["metadata"]["use_case"], "code_review");
    assert_eq!(bootstrap_data["metadata"]["scope_count"], 1);
    assert_eq!(bootstrap_data["use_case"]["name"], "Code Review");

    // Check scopes
    let scopes = bootstrap_data["scopes"].as_array().unwrap();
    assert_eq!(scopes.len(), 1);
    assert_eq!(scopes[0]["name"], scope_name);

    // Check AI instructions
    assert!(bootstrap_data.get("ai_instructions").is_some());
    let ai_instructions = bootstrap_data.get("ai_instructions").unwrap();
    assert!(ai_instructions.get("context_understanding").is_some());
    assert!(ai_instructions.get("key_concepts").is_some());

    Ok(())
}

#[test]
fn test_migration_with_protocol_info() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let rhema = setup_test_rhema(&temp_dir)?;

    // Create test scope without protocol info
    let scope_name = "legacy-scope";
    create_legacy_scope(&rhema, scope_name)?;

    // Test migration
    rhema::commands::migrate::run(&rhema, false, false)?;

    // Verify protocol info was added
    let scope_path = rhema.scope_path(scope_name)?;
    let rhema_file = scope_path.join("rhema.yaml");

    let content = fs::read_to_string(&rhema_file)?;
    let scope: rhema::RhemaScope = serde_yaml::from_str(&content)?;

    assert!(scope.protocol_info.is_some());
    let protocol_info = scope.protocol_info.unwrap();
    assert_eq!(protocol_info.version, "1.0.0");
    assert!(protocol_info.concepts.is_some());
    assert!(protocol_info.cql_examples.is_some());

    Ok(())
}

// Helper functions

fn create_test_protocol_info() -> ProtocolInfo {
    let concepts = vec![
        ConceptDefinition {
            name: "Scope".to_string(),
            description: "A Rhema scope represents a logical unit of the codebase.".to_string(),
            related: Some(vec!["Dependencies".to_string(), "Patterns".to_string()]),
            examples: Some(vec![
                "A microservice with its own API".to_string(),
                "A frontend application".to_string(),
            ]),
        },
        ConceptDefinition {
            name: "Knowledge".to_string(),
            description: "Structured information about the codebase.".to_string(),
            related: Some(vec!["Patterns".to_string(), "Decisions".to_string()]),
            examples: Some(vec![
                "API documentation".to_string(),
                "Architecture patterns".to_string(),
            ]),
        },
        ConceptDefinition {
            name: "CQL".to_string(),
            description: "Context Query Language for querying Rhema data.".to_string(),
            related: Some(vec!["Knowledge".to_string(), "Patterns".to_string()]),
            examples: Some(vec![
                "SELECT * FROM knowledge WHERE category = 'api'".to_string(),
                "SELECT * FROM patterns WHERE pattern_type = 'security'".to_string(),
            ]),
        },
    ];

    let cql_examples = vec![
        CqlExample {
            name: "Find API Knowledge".to_string(),
            query: "SELECT * FROM knowledge WHERE category = 'api'".to_string(),
            description: "Retrieve API-related knowledge".to_string(),
            output_format: Some("JSON array".to_string()),
            use_case: Some("Code review".to_string()),
        },
        CqlExample {
            name: "Find Security Patterns".to_string(),
            query: "SELECT * FROM patterns WHERE pattern_type = 'security'".to_string(),
            description: "Retrieve security patterns".to_string(),
            output_format: Some("JSON array".to_string()),
            use_case: Some("Security review".to_string()),
        },
        CqlExample {
            name: "Find Approved Decisions".to_string(),
            query: "SELECT * FROM decisions WHERE status = 'approved'".to_string(),
            description: "Retrieve approved decisions".to_string(),
            output_format: Some("JSON array".to_string()),
            use_case: Some("Architecture review".to_string()),
        },
    ];

    let patterns = vec![
        PatternDefinition {
            name: "Error Handling".to_string(),
            description: "Standardized error handling approach".to_string(),
            when_to_use: Some("When implementing functions that may fail".to_string()),
            examples: Some(vec![
                "Use Result<T, E> for functions".to_string(),
                "Log errors with context".to_string(),
            ]),
        },
        PatternDefinition {
            name: "Configuration Management".to_string(),
            description: "Environment-based configuration".to_string(),
            when_to_use: Some("When deploying to different environments".to_string()),
            examples: Some(vec![
                "Use environment variables".to_string(),
                "Provide sensible defaults".to_string(),
            ]),
        },
    ];

    let integrations = vec![IntegrationGuide {
        name: "IDE Integration".to_string(),
        description: "Integrate Rhema with your IDE".to_string(),
        setup: Some(vec![
            "Install Rhema CLI".to_string(),
            "Configure IDE extensions".to_string(),
        ]),
        configuration: Some(vec![
            "Add Rhema commands to palette".to_string(),
            "Configure file watching".to_string(),
        ]),
        best_practices: Some(vec![
            "Use Rhema commands from IDE".to_string(),
            "Enable auto-validation".to_string(),
        ]),
    }];

    let troubleshooting = vec![TroubleshootingItem {
        issue: "Configuration validation fails".to_string(),
        description: "Rhema configuration has validation errors".to_string(),
        solution: vec![
            "Run `rhema validate`".to_string(),
            "Check YAML syntax".to_string(),
        ],
        prevention: Some(vec![
            "Use `rhema validate` before committing".to_string(),
            "Follow schema documentation".to_string(),
        ]),
    }];

    ProtocolInfo {
        version: "1.0.0".to_string(),
        description: Some("Test protocol information".to_string()),
        concepts: Some(concepts),
        cql_examples: Some(cql_examples),
        patterns: Some(patterns),
        integrations: Some(integrations),
        troubleshooting: Some(troubleshooting),
        custom: std::collections::HashMap::new(),
    }
}

fn setup_test_rhema(temp_dir: &TempDir) -> RhemaResult<Rhema> {
    // Create a temporary repository structure
    let repo_root = temp_dir.path();

    // Create .rhema directory
    let rhema_dir = repo_root.join(".rhema");
    fs::create_dir_all(&rhema_dir)?;

    // Create a basic rhema.yaml for the repository
    let repo_rhema = r#"
name: "test-repo"
scope_type: "repository"
description: "Test repository for context bootstrapping"
version: "1.0.0"
schema_version: "1.0.0"
"#;
    fs::write(rhema_dir.join("rhema.yaml"), repo_rhema)?;

    // Initialize Rhema
    let rhema = Rhema::new()?;

    Ok(rhema)
}

fn create_test_scope(rhema: &Rhema, scope_name: &str) -> RhemaResult<()> {
    let scope_path = rhema.scope_path(scope_name)?;
    fs::create_dir_all(&scope_path)?;

    // Create rhema.yaml with protocol info
    let rhema_content = format!(
        r#"
name: "{}"
scope_type: "service"
description: "Test service scope"
version: "1.0.0"
schema_version: "1.0.0"
protocol_info:
  version: "1.0.0"
  description: "Protocol information for test service"
  concepts:
    - name: "API"
      description: "Application Programming Interface"
      related: ["REST", "GraphQL"]
      examples: ["HTTP endpoints", "WebSocket connections"]
  cql_examples:
    - name: "Find API Knowledge"
      query: "SELECT * FROM knowledge WHERE category = 'api'"
      description: "Find API-related knowledge"
      output_format: "JSON array"
      use_case: "Code review"
"#,
        scope_name
    );
    fs::write(scope_path.join("rhema.yaml"), rhema_content)?;

    // Create other context files
    let knowledge_content = r#"
entries:
  - id: "api-docs"
    title: "API Documentation"
    content: "Comprehensive API documentation"
    category: "api"
    tags: ["documentation", "api"]
    confidence: 8
    created_at: "2024-01-01T00:00:00Z"
"#;
    fs::write(scope_path.join("knowledge.yaml"), knowledge_content)?;

    let todos_content = r#"
todos:
  - id: "improve-docs"
    title: "Improve API Documentation"
    description: "Add more examples and error handling docs"
    status: "Pending"
    priority: "Medium"
    created_at: "2024-01-01T00:00:00Z"
"#;
    fs::write(scope_path.join("todos.yaml"), todos_content)?;

    let decisions_content = r#"
decisions:
  - id: "use-rest"
    title: "Use REST API Design"
    description: "Standardize on REST API design patterns"
    status: "Approved"
    decided_at: "2024-01-01T00:00:00Z"
"#;
    fs::write(scope_path.join("decisions.yaml"), decisions_content)?;

    let patterns_content = r#"
patterns:
  - id: "error-handling"
    name: "Error Handling"
    description: "Standardized error handling approach"
    pattern_type: "error-handling"
    usage: "Required"
    created_at: "2024-01-01T00:00:00Z"
"#;
    fs::write(scope_path.join("patterns.yaml"), patterns_content)?;

    let conventions_content = r#"
conventions:
  - id: "naming"
    name: "Naming Convention"
    description: "Use snake_case for variables and functions"
    convention_type: "naming"
    enforcement: "Required"
    created_at: "2024-01-01T00:00:00Z"
"#;
    fs::write(scope_path.join("conventions.yaml"), conventions_content)?;

    Ok(())
}

fn create_legacy_scope(rhema: &Rhema, scope_name: &str) -> RhemaResult<()> {
    let scope_path = rhema.scope_path(scope_name)?;
    fs::create_dir_all(&scope_path)?;

    // Create legacy rhema.yaml without protocol info
    let rhema_content = format!(
        r#"
name: "{}"
scope_type: "service"
description: "Legacy service scope"
version: "1.0.0"
"#,
        scope_name
    );
    fs::write(scope_path.join("rhema.yaml"), rhema_content)?;

    Ok(())
}
