use crate::{Gacp, GacpResult, GacpScope};
use colored::*;
use std::fs;
use std::path::PathBuf;

pub fn run(gacp: &Gacp, scope_type: Option<&str>, scope_name: Option<&str>) -> GacpResult<()> {
    let current_dir = std::env::current_dir()?;
    let repo_root = gacp.repo_root();
    
    // Determine scope path
    let scope_path = if current_dir == *repo_root {
        // Initialize at repository root
        repo_root.join(".gacp")
    } else {
        // Initialize in current directory
        current_dir.join(".gacp")
    };
    
    if scope_path.exists() {
        return Err(crate::GacpError::ConfigError(
            format!("GACP scope already exists at {}", scope_path.display())
        ));
    }
    
    // Create scope directory
    fs::create_dir_all(&scope_path)?;
    
    // Determine scope type and name
    let scope_type = scope_type.unwrap_or("service").to_string();
    let scope_name = scope_name.unwrap_or_else(|| {
        scope_path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
    }).to_string();
    
    // Create gacp.yaml
    let gacp_scope = GacpScope {
        name: scope_name.clone(),
        scope_type: scope_type.clone(),
        description: Some(format!("{} scope", scope_type)),
        version: "1.0.0".to_string(),
        schema_version: Some(crate::schema::CURRENT_SCHEMA_VERSION.to_string()),
        dependencies: None,
        custom: std::collections::HashMap::new(),
    };
    
    let gacp_content = serde_yaml::to_string(&gacp_scope)?;
    fs::write(scope_path.join("gacp.yaml"), gacp_content)?;
    
    // Create template files
    create_template_files(&scope_path)?;
    
    println!("{}", "✓ GACP scope initialized successfully!".green());
    println!("  Scope: {}", scope_name.yellow());
    println!("  Type: {}", scope_type.yellow());
    println!("  Path: {}", scope_path.display().to_string().yellow());
    println!();
    println!("  Next steps:");
    println!("    • Edit .gacp/gacp.yaml to customize scope settings");
    println!("    • Add knowledge entries: gacp insight record \"your insight\"");
    println!("    • Add todo items: gacp todo add \"your todo\"");
    
    Ok(())
}

fn create_template_files(scope_path: &PathBuf) -> GacpResult<()> {
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