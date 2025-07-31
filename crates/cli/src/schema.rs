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

use crate::{RhemaResult};
pub use rhema_core::schema::CURRENT_SCHEMA_VERSION;
use colored::*;
use std::path::Path;

pub fn run(_rhema: &crate::Rhema, template_type: &str, output_file: Option<&str>) -> RhemaResult<()> {
    println!("📋 Generating Rhema schema template...");
    println!("{}", "─".repeat(80));
    
    let template = match template_type {
        "scope" => generate_scope_template(),
        "knowledge" => generate_knowledge_template(),
        "todos" => generate_todos_template(),
        "decisions" => generate_decisions_template(),
        "patterns" => generate_patterns_template(),
        "conventions" => generate_conventions_template(),
        "all" => generate_all_templates(),
        _ => {
            return Err(crate::RhemaError::ConfigError(
                format!("Unknown template type: {}. Valid types: scope, knowledge, todos, decisions, patterns, conventions, all", template_type)
            ));
        }
    };
    
    if let Some(output_path) = output_file {
        // Write to file
        let output_path = Path::new(output_path);
        std::fs::write(output_path, template)
            .map_err(|e| crate::RhemaError::IoError(e))?;
        println!("✅ Template written to: {}", output_path.display().to_string().green());
    } else {
        // Print to console
        println!("{}", template);
    }
    
    Ok(())
}

fn generate_scope_template() -> String {
    format!(
        r#"# Rhema Scope Definition Template
# Schema Version: {}

name: "example-scope"
scope_type: "service"  # service, app, library, etc.
description: "Example scope for demonstration"
version: "1.0.0"
schema_version: "{}"

# Optional dependencies on other scopes
dependencies:
  - path: "../shared-lib"
    dependency_type: "required"  # required, optional, peer
    version: ">=1.0.0"

# Custom fields for extensibility
custom_field: "custom_value"
"#,
        CURRENT_SCHEMA_VERSION,
        CURRENT_SCHEMA_VERSION
    )
}

fn generate_knowledge_template() -> String {
    format!(
        r#"# Rhema Knowledge Base Template
# Schema Version: {}

# Knowledge entries
entries:
  - id: "example-knowledge-1"
    title: "Example Knowledge Entry"
    content: "This is an example knowledge entry with detailed information about a specific topic or insight."
    category: "architecture"
    tags:
      - "example"
      - "template"
    confidence: 8  # 1-10 scale
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"
    source: "Documentation"

# Optional categories for organization
categories:
  architecture: "System architecture and design decisions"
  patterns: "Design patterns and best practices"
  troubleshooting: "Common issues and solutions"

# Custom fields for extensibility
custom_field: "custom_value"
"#,
        CURRENT_SCHEMA_VERSION
    )
}

fn generate_todos_template() -> String {
    format!(
        r#"# Rhema Todo Items Template
# Schema Version: {}

# Todo entries
todos:
  - id: "example-todo-1"
    title: "Example Todo Item"
    description: "This is an example todo item with detailed description of what needs to be done."
    status: "pending"  # pending, in_progress, blocked, completed, cancelled
    priority: "medium"  # low, medium, high, critical
    assigned_to: "developer@example.com"
    due_date: "2024-12-31T23:59:59Z"
    created_at: "2024-01-01T00:00:00Z"
    completed_at: null  # Set when status is completed
    outcome: null  # Description of completion outcome
    related_knowledge:
      - "example-knowledge-1"

# Custom fields for extensibility
custom_field: "custom_value"
"#,
        CURRENT_SCHEMA_VERSION
    )
}

fn generate_decisions_template() -> String {
    format!(
        r#"# Rhema Decision Records Template
# Schema Version: {}

# Decision entries
decisions:
  - id: "example-decision-1"
    title: "Example Decision Record"
    description: "This is an example decision record documenting an important architectural or design decision."
    status: "approved"  # proposed, under_review, approved, rejected, implemented, deprecated
    context: "Context and background information about the decision"
    alternatives:
      - "Alternative approach 1"
      - "Alternative approach 2"
    rationale: "Detailed explanation of why this decision was made"
    consequences:
      - "Positive consequence 1"
      - "Potential risk 1"
    decided_at: "2024-01-01T00:00:00Z"
    review_date: "2024-12-31T23:59:59Z"  # Optional review date
    decision_makers:
      - "architect@example.com"
      - "tech-lead@example.com"

# Custom fields for extensibility
custom_field: "custom_value"
"#,
        CURRENT_SCHEMA_VERSION
    )
}

fn generate_patterns_template() -> String {
    format!(
        r#"# Rhema Design Patterns Template
# Schema Version: {}

# Pattern entries
patterns:
  - id: "example-pattern-1"
    name: "Example Design Pattern"
    description: "This is an example design pattern with detailed description of when and how to use it."
    pattern_type: "architectural"
    usage: "recommended"  # required, recommended, optional, deprecated
    effectiveness: 9  # 1-10 scale
    examples:
      - "Example implementation 1"
      - "Example implementation 2"
    anti_patterns:
      - "Anti-pattern to avoid 1"
      - "Anti-pattern to avoid 2"
    related_patterns:
      - "related-pattern-1"
      - "related-pattern-2"
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"

# Custom fields for extensibility
custom_field: "custom_value"
"#,
        CURRENT_SCHEMA_VERSION
    )
}

fn generate_conventions_template() -> String {
    format!(
        r#"# Rhema Coding Conventions Template
# Schema Version: {}

# Convention entries
conventions:
  - id: "example-convention-1"
    name: "Example Coding Convention"
    description: "This is an example coding convention with detailed description of the rule or standard."
    convention_type: "naming"
    enforcement: "required"  # required, recommended, optional, deprecated
    examples:
      - "Good example: camelCase for variables"
      - "Bad example: snake_case for variables"
    tools:
      - "eslint"
      - "prettier"
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"

# Custom fields for extensibility
custom_field: "custom_value"
"#,
        CURRENT_SCHEMA_VERSION
    )
}

fn generate_all_templates() -> String {
    format!(
        r#"# Rhema Complete Template Set
# Schema Version: {}

# =============================================================================
# rhema.yaml - Scope Definition
# =============================================================================

name: "example-scope"
scope_type: "service"
description: "Example scope for demonstration"
version: "1.0.0"
schema_version: "{}"

dependencies:
  - path: "../shared-lib"
    dependency_type: "required"
    version: ">=1.0.0"

# =============================================================================
# knowledge.yaml - Knowledge Base
# =============================================================================

entries:
  - id: "example-knowledge-1"
    title: "Example Knowledge Entry"
    content: "This is an example knowledge entry with detailed information about a specific topic or insight."
    category: "architecture"
    tags:
      - "example"
      - "template"
    confidence: 8
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"
    source: "Documentation"

categories:
  architecture: "System architecture and design decisions"
  patterns: "Design patterns and best practices"
  troubleshooting: "Common issues and solutions"

# =============================================================================
# todos.yaml - Todo Items
# =============================================================================

todos:
  - id: "example-todo-1"
    title: "Example Todo Item"
    description: "This is an example todo item with detailed description of what needs to be done."
    status: "pending"
    priority: "medium"
    assigned_to: "developer@example.com"
    due_date: "2024-12-31T23:59:59Z"
    created_at: "2024-01-01T00:00:00Z"
    completed_at: null
    outcome: null
    related_knowledge:
      - "example-knowledge-1"

# =============================================================================
# decisions.yaml - Decision Records
# =============================================================================

decisions:
  - id: "example-decision-1"
    title: "Example Decision Record"
    description: "This is an example decision record documenting an important architectural or design decision."
    status: "approved"
    context: "Context and background information about the decision"
    alternatives:
      - "Alternative approach 1"
      - "Alternative approach 2"
    rationale: "Detailed explanation of why this decision was made"
    consequences:
      - "Positive consequence 1"
      - "Potential risk 1"
    decided_at: "2024-01-01T00:00:00Z"
    review_date: "2024-12-31T23:59:59Z"
    decision_makers:
      - "architect@example.com"
      - "tech-lead@example.com"

# =============================================================================
# patterns.yaml - Design Patterns
# =============================================================================

patterns:
  - id: "example-pattern-1"
    name: "Example Design Pattern"
    description: "This is an example design pattern with detailed description of when and how to use it."
    pattern_type: "architectural"
    usage: "recommended"
    effectiveness: 9
    examples:
      - "Example implementation 1"
      - "Example implementation 2"
    anti_patterns:
      - "Anti-pattern to avoid 1"
      - "Anti-pattern to avoid 2"
    related_patterns:
      - "related-pattern-1"
      - "related-pattern-2"
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"

# =============================================================================
# conventions.yaml - Coding Conventions
# =============================================================================

conventions:
  - id: "example-convention-1"
    name: "Example Coding Convention"
    description: "This is an example coding convention with detailed description of the rule or standard."
    convention_type: "naming"
    enforcement: "required"
    examples:
      - "Good example: camelCase for variables"
      - "Bad example: snake_case for variables"
    tools:
      - "eslint"
      - "prettier"
    created_at: "2024-01-01T00:00:00Z"
    updated_at: "2024-01-01T00:00:00Z"
"#,
        CURRENT_SCHEMA_VERSION,
        CURRENT_SCHEMA_VERSION
    )
} 
