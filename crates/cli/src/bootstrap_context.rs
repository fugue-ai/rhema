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
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Bootstrap context for AI agents
pub fn run(
    rhema: &Rhema,
    use_case: &str,
    output_format: &str,
    output_dir: Option<&str>,
    scope_filter: Option<&str>,
    include_all: bool,
    optimize_for_ai: bool,
    create_primer: bool,
    create_readme: bool,
) -> RhemaResult<()> {
    let scopes = rhema.list_scopes()?;

    // Filter scopes if specified
    let filtered_scopes = if let Some(filter) = scope_filter {
        scopes
            .into_iter()
            .filter(|scope| {
                scope.definition.name.contains(filter)
                    || scope.definition.scope_type.contains(filter)
            })
            .collect()
    } else {
        scopes
    };

    if filtered_scopes.is_empty() {
        return Err(crate::RhemaError::ConfigError(
            "No scopes found matching the filter criteria".to_string(),
        ));
    }

    // Determine output directory
    let output_path = if let Some(dir) = output_dir {
        PathBuf::from(dir)
    } else {
        std::env::current_dir()?.join("rhema-bootstrap")
    };

    // Create output directory
    fs::create_dir_all(&output_path)?;

    // Generate bootstrap content based on use case
    // Extract RhemaScope from Scope objects
    let rhema_scopes: Vec<RhemaScope> = filtered_scopes
        .iter()
        .map(|scope| scope.definition.clone())
        .collect();

    let bootstrap_content =
        generate_bootstrap_content(rhema, &rhema_scopes, use_case, include_all, optimize_for_ai)?;

    // Write bootstrap files
    write_bootstrap_files(&bootstrap_content, &output_path, output_format)?;

    // Generate additional files if requested
    if create_primer {
        generate_bootstrap_primer(rhema, &rhema_scopes, &output_path, use_case)?;
    }

    if create_readme {
        generate_bootstrap_readme(&bootstrap_content, &output_path)?;
    }

    println!("{}", "✓ Context bootstrap completed successfully!".green());
    println!("  Use case: {}", use_case.yellow());
    println!(
        "  Output directory: {}",
        output_path.display().to_string().yellow()
    );
    println!("  Format: {}", output_format.yellow());

    Ok(())
}

/// Bootstrap content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BootstrapContent {
    /// Bootstrap metadata
    pub metadata: BootstrapMetadata,

    /// Use case specific content
    pub use_case: UseCaseContent,

    /// Scopes summary
    pub scopes: Vec<ScopeSummary>,

    /// Context summary
    pub context_summary: ContextSummary,

    /// AI agent instructions
    pub ai_instructions: Option<AiInstructions>,

    /// Quick reference
    pub quick_reference: QuickReference,
}

/// Bootstrap metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BootstrapMetadata {
    /// Bootstrap version
    pub version: String,

    /// Generation timestamp
    pub generated_at: String,

    /// Use case
    pub use_case: String,

    /// Output format
    pub output_format: String,

    /// Scope count
    pub scope_count: usize,

    /// Optimization level
    pub optimization: String,
}

/// Use case specific content
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UseCaseContent {
    /// Use case name
    pub name: String,

    /// Description
    pub description: String,

    /// Key objectives
    pub objectives: Vec<String>,

    /// Context requirements
    pub context_requirements: Vec<String>,

    /// Success criteria
    pub success_criteria: Vec<String>,
}

/// Scope summary
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScopeSummary {
    /// Scope name
    pub name: String,

    /// Scope type
    pub scope_type: String,

    /// Description
    pub description: Option<String>,

    /// Key responsibilities
    pub responsibilities: Vec<String>,

    /// Dependencies
    pub dependencies: Option<Vec<String>>,

    /// Context relevance
    pub context_relevance: String,
}

/// Context summary
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextSummary {
    /// Knowledge entries
    pub knowledge_entries: usize,

    /// Todo items
    pub todo_items: usize,

    /// Decisions
    pub decisions: usize,

    /// Patterns
    pub patterns: usize,

    /// Conventions
    pub conventions: usize,

    /// Key insights
    pub key_insights: Vec<String>,

    /// Context gaps
    pub context_gaps: Vec<String>,
}

/// AI agent instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AiInstructions {
    /// Context understanding
    pub context_understanding: String,

    /// Key concepts
    pub key_concepts: Vec<String>,

    /// Query patterns
    pub query_patterns: Vec<String>,

    /// Decision making guidelines
    pub decision_guidelines: Vec<String>,

    /// Context limitations
    pub context_limitations: Vec<String>,
}

/// Quick reference
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuickReference {
    /// Essential commands
    pub essential_commands: Vec<CommandReference>,

    /// Common queries
    pub common_queries: Vec<QueryReference>,

    /// Context patterns
    pub context_patterns: Vec<PatternReference>,

    /// Troubleshooting
    pub troubleshooting: Vec<TroubleshootingReference>,
}

/// Command reference
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommandReference {
    /// Command
    pub command: String,

    /// Description
    pub description: String,

    /// Use case
    pub use_case: String,
}

/// Query reference
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QueryReference {
    /// Query
    pub query: String,

    /// Description
    pub description: String,

    /// Expected output
    pub expected_output: String,
}

/// Pattern reference
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PatternReference {
    /// Pattern name
    pub name: String,

    /// Description
    pub description: String,

    /// Implementation
    pub implementation: String,
}

/// Troubleshooting reference
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TroubleshootingReference {
    /// Issue
    pub issue: String,

    /// Solution
    pub solution: String,

    /// Prevention
    pub prevention: Option<String>,
}

/// Generate bootstrap content
fn generate_bootstrap_content(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    use_case: &str,
    include_all: bool,
    optimize_for_ai: bool,
) -> RhemaResult<BootstrapContent> {
    let metadata = BootstrapMetadata {
        version: "1.0.0".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        use_case: use_case.to_string(),
        output_format: "bootstrap".to_string(),
        scope_count: scopes.len(),
        optimization: if optimize_for_ai {
            "ai_optimized".to_string()
        } else {
            "standard".to_string()
        },
    };

    let use_case_content = create_use_case_content(use_case);
    let scope_summaries = create_scope_summaries(rhema, scopes, use_case)?;
    let context_summary = create_context_summary(rhema, scopes, include_all)?;

    let ai_instructions = if optimize_for_ai {
        Some(create_ai_instructions(use_case, scopes))
    } else {
        None
    };

    let quick_reference = create_quick_reference(use_case, optimize_for_ai);

    Ok(BootstrapContent {
        metadata,
        use_case: use_case_content,
        scopes: scope_summaries,
        context_summary,
        ai_instructions,
        quick_reference,
    })
}

/// Create use case content
fn create_use_case_content(use_case: &str) -> UseCaseContent {
    match use_case {
        "code_review" => UseCaseContent {
            name: "Code Review".to_string(),
            description: "AI-assisted code review with context awareness".to_string(),
            objectives: vec![
                "Identify potential issues and improvements".to_string(),
                "Ensure code follows project patterns and conventions".to_string(),
                "Validate architectural decisions".to_string(),
                "Check for security and performance concerns".to_string(),
            ],
            context_requirements: vec![
                "Knowledge of project patterns and conventions".to_string(),
                "Understanding of architectural decisions".to_string(),
                "Awareness of security and performance requirements".to_string(),
                "Familiarity with codebase structure".to_string(),
            ],
            success_criteria: vec![
                "Comprehensive issue identification".to_string(),
                "Actionable improvement suggestions".to_string(),
                "Consistency with project standards".to_string(),
                "Clear explanation of recommendations".to_string(),
            ],
        },
        "feature_development" => UseCaseContent {
            name: "Feature Development".to_string(),
            description: "AI-assisted feature development with full context".to_string(),
            objectives: vec![
                "Understand feature requirements and context".to_string(),
                "Identify implementation patterns and approaches".to_string(),
                "Ensure consistency with existing codebase".to_string(),
                "Plan integration points and dependencies".to_string(),
            ],
            context_requirements: vec![
                "Deep understanding of codebase architecture".to_string(),
                "Knowledge of existing patterns and conventions".to_string(),
                "Awareness of integration points and APIs".to_string(),
                "Understanding of data models and flows".to_string(),
            ],
            success_criteria: vec![
                "Accurate requirement understanding".to_string(),
                "Appropriate pattern selection".to_string(),
                "Consistent implementation approach".to_string(),
                "Proper integration planning".to_string(),
            ],
        },
        "debugging" => UseCaseContent {
            name: "Debugging".to_string(),
            description: "AI-assisted debugging with context awareness".to_string(),
            objectives: vec![
                "Identify root cause of issues".to_string(),
                "Understand system behavior and data flow".to_string(),
                "Suggest effective debugging strategies".to_string(),
                "Recommend fixes and improvements".to_string(),
            ],
            context_requirements: vec![
                "Understanding of system architecture".to_string(),
                "Knowledge of data flows and dependencies".to_string(),
                "Awareness of common failure patterns".to_string(),
                "Familiarity with debugging tools and techniques".to_string(),
            ],
            success_criteria: vec![
                "Accurate root cause identification".to_string(),
                "Effective debugging strategy".to_string(),
                "Appropriate fix recommendations".to_string(),
                "Prevention of similar issues".to_string(),
            ],
        },
        "documentation" => UseCaseContent {
            name: "Documentation".to_string(),
            description: "AI-assisted documentation generation and maintenance".to_string(),
            objectives: vec![
                "Generate comprehensive documentation".to_string(),
                "Maintain documentation accuracy".to_string(),
                "Ensure documentation completeness".to_string(),
                "Improve documentation usability".to_string(),
            ],
            context_requirements: vec![
                "Complete understanding of system functionality".to_string(),
                "Knowledge of user needs and use cases".to_string(),
                "Awareness of documentation standards".to_string(),
                "Understanding of system architecture".to_string(),
            ],
            success_criteria: vec![
                "Comprehensive and accurate documentation".to_string(),
                "User-friendly and accessible content".to_string(),
                "Consistent documentation structure".to_string(),
                "Up-to-date information".to_string(),
            ],
        },
        "onboarding" => UseCaseContent {
            name: "Onboarding".to_string(),
            description: "AI-assisted developer onboarding and knowledge transfer".to_string(),
            objectives: vec![
                "Provide comprehensive project overview".to_string(),
                "Explain key concepts and patterns".to_string(),
                "Guide through development setup".to_string(),
                "Share important decisions and context".to_string(),
            ],
            context_requirements: vec![
                "Complete project context and history".to_string(),
                "Key architectural decisions and rationale".to_string(),
                "Development environment setup".to_string(),
                "Important patterns and conventions".to_string(),
            ],
            success_criteria: vec![
                "Clear project understanding".to_string(),
                "Effective development setup".to_string(),
                "Familiarity with key concepts".to_string(),
                "Confidence in contributing".to_string(),
            ],
        },
        _ => UseCaseContent {
            name: use_case.to_string(),
            description: format!("AI-assisted {} with context awareness", use_case),
            objectives: vec![
                "Understand project context".to_string(),
                "Apply appropriate patterns and conventions".to_string(),
                "Ensure consistency and quality".to_string(),
                "Provide valuable insights and recommendations".to_string(),
            ],
            context_requirements: vec![
                "Project knowledge and context".to_string(),
                "Pattern and convention awareness".to_string(),
                "Architectural understanding".to_string(),
                "Quality and consistency standards".to_string(),
            ],
            success_criteria: vec![
                "Accurate context understanding".to_string(),
                "Appropriate recommendations".to_string(),
                "Consistent approach".to_string(),
                "Valuable insights".to_string(),
            ],
        },
    }
}

/// Create scope summaries
fn create_scope_summaries(
    _rhema: &Rhema,
    scopes: &[RhemaScope],
    use_case: &str,
) -> RhemaResult<Vec<ScopeSummary>> {
    let mut summaries = Vec::new();

    for scope in scopes {
        let responsibilities = infer_responsibilities(scope);
        let dependencies = scope
            .dependencies
            .as_ref()
            .map(|deps| deps.iter().map(|d| d.path.clone()).collect());

        let context_relevance = determine_context_relevance(scope, use_case);

        summaries.push(ScopeSummary {
            name: scope.name.clone(),
            scope_type: scope.scope_type.clone(),
            description: scope.description.clone(),
            responsibilities,
            dependencies,
            context_relevance,
        });
    }

    Ok(summaries)
}

/// Infer scope responsibilities
fn infer_responsibilities(scope: &RhemaScope) -> Vec<String> {
    match scope.scope_type.as_str() {
        "service" => vec![
            "API endpoint management".to_string(),
            "Business logic implementation".to_string(),
            "Data processing and validation".to_string(),
        ],
        "app" => vec![
            "User interface management".to_string(),
            "User interaction handling".to_string(),
            "Data presentation".to_string(),
        ],
        "library" => vec![
            "Reusable functionality".to_string(),
            "API abstraction".to_string(),
            "Utility functions".to_string(),
        ],
        _ => vec![
            "Core functionality".to_string(),
            "Data management".to_string(),
            "Configuration".to_string(),
        ],
    }
}

/// Determine context relevance
fn determine_context_relevance(_scope: &RhemaScope, use_case: &str) -> String {
    match use_case {
        "code_review" => {
            "High - Contains patterns, conventions, and architectural decisions".to_string()
        }
        "feature_development" => {
            "High - Provides implementation patterns and integration context".to_string()
        }
        "debugging" => "Medium - Contains error patterns and system behavior".to_string(),
        "documentation" => "High - Contains comprehensive project knowledge".to_string(),
        "onboarding" => {
            "High - Essential for understanding project structure and decisions".to_string()
        }
        _ => "Medium - Provides general project context".to_string(),
    }
}

/// Create context summary
fn create_context_summary(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    include_all: bool,
) -> RhemaResult<ContextSummary> {
    let mut knowledge_entries = 0;
    let mut todo_items = 0;
    let mut decisions = 0;
    let mut patterns = 0;
    let mut conventions = 0;

    for scope in scopes {
        if let Ok(knowledge) = rhema.load_knowledge(&scope.name) {
            knowledge_entries += knowledge.entries.len();
        }
        if let Ok(todos) = rhema.load_todos(&scope.name) {
            todo_items += todos.todos.len();
        }
        if let Ok(decisions_data) = rhema.load_decisions(&scope.name) {
            decisions += decisions_data.decisions.len();
        }
        if let Ok(patterns_data) = rhema.load_patterns(&scope.name) {
            patterns += patterns_data.patterns.len();
        }
        if let Ok(conventions_data) = rhema.load_conventions(&scope.name) {
            conventions += conventions_data.conventions.len();
        }
    }

    let key_insights = vec![
        format!("{} knowledge entries available", knowledge_entries),
        format!("{} todo items tracked", todo_items),
        format!("{} decisions documented", decisions),
        format!("{} patterns defined", patterns),
        format!("{} conventions established", conventions),
    ];

    let context_gaps = if include_all {
        vec![
            "Consider adding more specific error handling patterns".to_string(),
            "Document more integration scenarios".to_string(),
            "Add performance optimization guidelines".to_string(),
        ]
    } else {
        vec![]
    };

    Ok(ContextSummary {
        knowledge_entries,
        todo_items,
        decisions,
        patterns,
        conventions,
        key_insights,
        context_gaps,
    })
}

/// Create AI instructions
fn create_ai_instructions(use_case: &str, _scopes: &[RhemaScope]) -> AiInstructions {
    let context_understanding = match use_case {
        "code_review" => "Focus on code quality, patterns, and architectural consistency. Consider security, performance, and maintainability aspects.".to_string(),
        "feature_development" => "Understand the existing architecture and patterns. Ensure new features integrate well and follow established conventions.".to_string(),
        "debugging" => "Analyze system behavior and data flow. Identify root causes and suggest effective debugging strategies.".to_string(),
        "documentation" => "Generate comprehensive and user-friendly documentation. Ensure accuracy and completeness.".to_string(),
        "onboarding" => "Provide clear explanations of project structure, key concepts, and development workflow.".to_string(),
        _ => "Understand the project context and provide appropriate guidance based on the specific use case.".to_string(),
    };

    let key_concepts = vec![
        "Rhema (Git-Based Agent Context Protocol)".to_string(),
        "Context Query Language (CQL)".to_string(),
        "Knowledge management".to_string(),
        "Pattern documentation".to_string(),
        "Decision tracking".to_string(),
    ];

    let query_patterns = vec![
        "Use CQL to query knowledge: `SELECT * FROM knowledge WHERE category = 'api'`".to_string(),
        "Search for patterns: `SELECT * FROM patterns WHERE pattern_type = 'security'`".to_string(),
        "Find decisions: `SELECT * FROM decisions WHERE status = 'approved'`".to_string(),
        "Look for todos: `SELECT * FROM todos WHERE priority = 'high'`".to_string(),
    ];

    let decision_guidelines = vec![
        "Consider existing architectural decisions".to_string(),
        "Follow established patterns and conventions".to_string(),
        "Evaluate impact on other components".to_string(),
        "Document rationale for new decisions".to_string(),
    ];

    let context_limitations = vec![
        "Context may not include all recent changes".to_string(),
        "Some decisions may lack detailed rationale".to_string(),
        "Patterns may need validation in current context".to_string(),
        "Consider checking for updates to context".to_string(),
    ];

    AiInstructions {
        context_understanding,
        key_concepts,
        query_patterns,
        decision_guidelines,
        context_limitations,
    }
}

/// Create quick reference
fn create_quick_reference(_use_case: &str, _optimize_for_ai: bool) -> QuickReference {
    let essential_commands = vec![
        CommandReference {
            command: "rhema scope".to_string(),
            description: "Show scope information".to_string(),
            use_case: "Understanding project structure".to_string(),
        },
        CommandReference {
            command: "rhema query".to_string(),
            description: "Execute CQL query".to_string(),
            use_case: "Finding specific information".to_string(),
        },
        CommandReference {
            command: "rhema search".to_string(),
            description: "Search across context".to_string(),
            use_case: "Finding relevant content".to_string(),
        },
        CommandReference {
            command: "rhema validate".to_string(),
            description: "Validate configuration".to_string(),
            use_case: "Ensuring data integrity".to_string(),
        },
    ];

    let common_queries = vec![
        QueryReference {
            query: "SELECT * FROM knowledge WHERE category = 'api'".to_string(),
            description: "Find API-related knowledge".to_string(),
            expected_output: "API documentation and examples".to_string(),
        },
        QueryReference {
            query: "SELECT * FROM patterns WHERE pattern_type = 'security'".to_string(),
            description: "Find security patterns".to_string(),
            expected_output: "Security implementation patterns".to_string(),
        },
        QueryReference {
            query: "SELECT * FROM decisions WHERE status = 'approved'".to_string(),
            description: "Find approved decisions".to_string(),
            expected_output: "Architectural and design decisions".to_string(),
        },
    ];

    let context_patterns = vec![
        PatternReference {
            name: "Error Handling".to_string(),
            description: "Standardized error handling approach".to_string(),
            implementation: "Use consistent error response format and logging".to_string(),
        },
        PatternReference {
            name: "Configuration Management".to_string(),
            description: "Environment-based configuration".to_string(),
            implementation: "Use environment variables and config files".to_string(),
        },
        PatternReference {
            name: "API Design".to_string(),
            description: "RESTful API design principles".to_string(),
            implementation: "Follow REST conventions and use proper HTTP methods".to_string(),
        },
    ];

    let troubleshooting = vec![
        TroubleshootingReference {
            issue: "Configuration validation fails".to_string(),
            solution: "Run `rhema validate` and fix YAML syntax issues".to_string(),
            prevention: Some("Use `rhema validate` before committing changes".to_string()),
        },
        TroubleshootingReference {
            issue: "Scope not found".to_string(),
            solution: "Check scope path and ensure rhema.yaml exists".to_string(),
            prevention: Some("Use `rhema scopes` to list available scopes".to_string()),
        },
    ];

    QuickReference {
        essential_commands,
        common_queries,
        context_patterns,
        troubleshooting,
    }
}

/// Write bootstrap files
fn write_bootstrap_files(
    content: &BootstrapContent,
    output_path: &PathBuf,
    format: &str,
) -> RhemaResult<()> {
    match format.to_lowercase().as_str() {
        "json" => {
            let json_content = serde_json::to_string_pretty(content)?;
            fs::write(output_path.join("bootstrap.json"), json_content)?;
        }
        "yaml" => {
            let yaml_content = serde_yaml::to_string(content)?;
            fs::write(output_path.join("bootstrap.yaml"), yaml_content)?;
        }
        "markdown" => {
            let md_content = format_bootstrap_markdown(content);
            fs::write(output_path.join("bootstrap.md"), md_content)?;
        }
        "text" => {
            let text_content = format_bootstrap_text(content);
            fs::write(output_path.join("bootstrap.txt"), text_content)?;
        }
        "all" => {
            // Write all formats
            let json_content = serde_json::to_string_pretty(content)?;
            fs::write(output_path.join("bootstrap.json"), json_content)?;

            let yaml_content = serde_yaml::to_string(content)?;
            fs::write(output_path.join("bootstrap.yaml"), yaml_content)?;

            let md_content = format_bootstrap_markdown(content);
            fs::write(output_path.join("bootstrap.md"), md_content)?;

            let text_content = format_bootstrap_text(content);
            fs::write(output_path.join("bootstrap.txt"), text_content)?;
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unsupported format: {}",
                format
            )));
        }
    }

    Ok(())
}

/// Format bootstrap markdown
fn format_bootstrap_markdown(content: &BootstrapContent) -> String {
    let mut md = String::new();

    md.push_str(&format!(
        "# Rhema Context Bootstrap - {}\n\n",
        content.use_case.name
    ));
    md.push_str(&format!(
        "**Generated:** {}\n",
        content.metadata.generated_at
    ));
    md.push_str(&format!("**Use Case:** {}\n", content.use_case.name));
    md.push_str(&format!(
        "**Scope Count:** {}\n",
        content.metadata.scope_count
    ));
    md.push_str(&format!(
        "**Optimization:** {}\n\n",
        content.metadata.optimization
    ));

    // Use case information
    md.push_str("## Use Case Information\n\n");
    md.push_str(&content.use_case.description);
    md.push_str("\n\n");

    md.push_str("### Objectives\n\n");
    for objective in &content.use_case.objectives {
        md.push_str(&format!("- {}\n", objective));
    }
    md.push_str("\n");

    md.push_str("### Context Requirements\n\n");
    for requirement in &content.use_case.context_requirements {
        md.push_str(&format!("- {}\n", requirement));
    }
    md.push_str("\n");

    md.push_str("### Success Criteria\n\n");
    for criterion in &content.use_case.success_criteria {
        md.push_str(&format!("- {}\n", criterion));
    }
    md.push_str("\n");

    // Scopes
    md.push_str("## Scopes\n\n");
    for scope in &content.scopes {
        md.push_str(&format!("### {}\n", scope.name));
        md.push_str(&format!("**Type:** {}\n", scope.scope_type));
        if let Some(ref desc) = scope.description {
            md.push_str(&format!("**Description:** {}\n", desc));
        }
        md.push_str("**Responsibilities:**\n");
        for responsibility in &scope.responsibilities {
            md.push_str(&format!("- {}\n", responsibility));
        }
        md.push_str(&format!(
            "**Context Relevance:** {}\n",
            scope.context_relevance
        ));
        md.push_str("\n");
    }

    // Context summary
    md.push_str("## Context Summary\n\n");
    md.push_str(&format!(
        "- **Knowledge Entries:** {}\n",
        content.context_summary.knowledge_entries
    ));
    md.push_str(&format!(
        "- **Todo Items:** {}\n",
        content.context_summary.todo_items
    ));
    md.push_str(&format!(
        "- **Decisions:** {}\n",
        content.context_summary.decisions
    ));
    md.push_str(&format!(
        "- **Patterns:** {}\n",
        content.context_summary.patterns
    ));
    md.push_str(&format!(
        "- **Conventions:** {}\n",
        content.context_summary.conventions
    ));
    md.push_str("\n");

    md.push_str("### Key Insights\n\n");
    for insight in &content.context_summary.key_insights {
        md.push_str(&format!("- {}\n", insight));
    }
    md.push_str("\n");

    if !content.context_summary.context_gaps.is_empty() {
        md.push_str("### Context Gaps\n\n");
        for gap in &content.context_summary.context_gaps {
            md.push_str(&format!("- {}\n", gap));
        }
        md.push_str("\n");
    }

    // AI instructions
    if let Some(ref ai_instructions) = content.ai_instructions {
        md.push_str("## AI Agent Instructions\n\n");
        md.push_str(&format!(
            "**Context Understanding:** {}\n\n",
            ai_instructions.context_understanding
        ));

        md.push_str("### Key Concepts\n\n");
        for concept in &ai_instructions.key_concepts {
            md.push_str(&format!("- {}\n", concept));
        }
        md.push_str("\n");

        md.push_str("### Query Patterns\n\n");
        for pattern in &ai_instructions.query_patterns {
            md.push_str(&format!("- {}\n", pattern));
        }
        md.push_str("\n");

        md.push_str("### Decision Guidelines\n\n");
        for guideline in &ai_instructions.decision_guidelines {
            md.push_str(&format!("- {}\n", guideline));
        }
        md.push_str("\n");

        md.push_str("### Context Limitations\n\n");
        for limitation in &ai_instructions.context_limitations {
            md.push_str(&format!("- {}\n", limitation));
        }
        md.push_str("\n");
    }

    // Quick reference
    md.push_str("## Quick Reference\n\n");

    md.push_str("### Essential Commands\n\n");
    for cmd in &content.quick_reference.essential_commands {
        md.push_str(&format!("#### `{}`\n", cmd.command));
        md.push_str(&format!("{}\n", cmd.description));
        md.push_str(&format!("**Use Case:** {}\n\n", cmd.use_case));
    }

    md.push_str("### Common Queries\n\n");
    for query in &content.quick_reference.common_queries {
        md.push_str(&format!("#### `{}`\n", query.query));
        md.push_str(&format!("{}\n", query.description));
        md.push_str(&format!(
            "**Expected Output:** {}\n\n",
            query.expected_output
        ));
    }

    md.push_str("### Context Patterns\n\n");
    for pattern in &content.quick_reference.context_patterns {
        md.push_str(&format!("#### {}\n", pattern.name));
        md.push_str(&format!("{}\n", pattern.description));
        md.push_str(&format!(
            "**Implementation:** {}\n\n",
            pattern.implementation
        ));
    }

    md.push_str("### Troubleshooting\n\n");
    for item in &content.quick_reference.troubleshooting {
        md.push_str(&format!("#### {}\n", item.issue));
        md.push_str(&format!("**Solution:** {}\n", item.solution));
        if let Some(ref prevention) = item.prevention {
            md.push_str(&format!("**Prevention:** {}\n", prevention));
        }
        md.push_str("\n");
    }

    md
}

/// Format bootstrap text
fn format_bootstrap_text(content: &BootstrapContent) -> String {
    let mut text = String::new();

    text.push_str(&format!(
        "Rhema CONTEXT BOOTSTRAP - {}\n",
        content.use_case.name.to_uppercase()
    ));
    text.push_str(&"=".repeat(content.use_case.name.len() + 25));
    text.push_str("\n\n");

    text.push_str(&format!("Generated: {}\n", content.metadata.generated_at));
    text.push_str(&format!("Use Case: {}\n", content.use_case.name));
    text.push_str(&format!("Scope Count: {}\n", content.metadata.scope_count));
    text.push_str(&format!(
        "Optimization: {}\n\n",
        content.metadata.optimization
    ));

    text.push_str("USE CASE INFORMATION:\n");
    text.push_str("---------------------\n");
    text.push_str(&content.use_case.description);
    text.push_str("\n\n");

    text.push_str("Objectives:\n");
    for objective in &content.use_case.objectives {
        text.push_str(&format!("- {}\n", objective));
    }
    text.push_str("\n");

    text.push_str("Context Requirements:\n");
    for requirement in &content.use_case.context_requirements {
        text.push_str(&format!("- {}\n", requirement));
    }
    text.push_str("\n");

    text.push_str("SCOPES:\n");
    text.push_str("-------\n");
    for scope in &content.scopes {
        text.push_str(&format!("Name: {}\n", scope.name));
        text.push_str(&format!("Type: {}\n", scope.scope_type));
        if let Some(ref desc) = scope.description {
            text.push_str(&format!("Description: {}\n", desc));
        }
        text.push_str("Responsibilities:\n");
        for responsibility in &scope.responsibilities {
            text.push_str(&format!("  - {}\n", responsibility));
        }
        text.push_str(&format!("Context Relevance: {}\n", scope.context_relevance));
        text.push_str("\n");
    }

    text.push_str("CONTEXT SUMMARY:\n");
    text.push_str("----------------\n");
    text.push_str(&format!(
        "Knowledge Entries: {}\n",
        content.context_summary.knowledge_entries
    ));
    text.push_str(&format!(
        "Todo Items: {}\n",
        content.context_summary.todo_items
    ));
    text.push_str(&format!(
        "Decisions: {}\n",
        content.context_summary.decisions
    ));
    text.push_str(&format!("Patterns: {}\n", content.context_summary.patterns));
    text.push_str(&format!(
        "Conventions: {}\n",
        content.context_summary.conventions
    ));
    text.push_str("\n");

    text.push_str("Key Insights:\n");
    for insight in &content.context_summary.key_insights {
        text.push_str(&format!("- {}\n", insight));
    }
    text.push_str("\n");

    if let Some(ref ai_instructions) = content.ai_instructions {
        text.push_str("AI AGENT INSTRUCTIONS:\n");
        text.push_str("----------------------\n");
        text.push_str(&format!(
            "Context Understanding: {}\n\n",
            ai_instructions.context_understanding
        ));

        text.push_str("Key Concepts:\n");
        for concept in &ai_instructions.key_concepts {
            text.push_str(&format!("- {}\n", concept));
        }
        text.push_str("\n");

        text.push_str("Query Patterns:\n");
        for pattern in &ai_instructions.query_patterns {
            text.push_str(&format!("- {}\n", pattern));
        }
        text.push_str("\n");
    }

    text.push_str("QUICK REFERENCE:\n");
    text.push_str("----------------\n");
    text.push_str("Essential Commands:\n");
    for cmd in &content.quick_reference.essential_commands {
        text.push_str(&format!("- {}: {}\n", cmd.command, cmd.description));
    }
    text.push_str("\n");

    text.push_str("Common Queries:\n");
    for query in &content.quick_reference.common_queries {
        text.push_str(&format!("- {}: {}\n", query.query, query.description));
    }
    text.push_str("\n");

    text
}

/// Generate bootstrap primer
fn generate_bootstrap_primer(
    _rhema: &Rhema,
    _scopes: &[RhemaScope],
    output_path: &PathBuf,
    use_case: &str,
) -> RhemaResult<()> {
    // This would integrate with the primer command
    // For now, create a simple primer file
    let primer_content = format!("# Rhema Bootstrap Primer - {}\n\nThis primer provides context for AI agents working on {} tasks.\n", use_case, use_case);
    fs::write(output_path.join("primer.md"), primer_content)?;

    Ok(())
}

/// Generate bootstrap README
fn generate_bootstrap_readme(content: &BootstrapContent, output_path: &PathBuf) -> RhemaResult<()> {
    let readme_content = format!("# Rhema Context Bootstrap\n\nThis directory contains bootstrap files for {} use case.\n\n## Files\n\n- `bootstrap.json` - JSON format bootstrap data\n- `bootstrap.yaml` - YAML format bootstrap data\n- `bootstrap.md` - Markdown format bootstrap data\n- `bootstrap.txt` - Text format bootstrap data\n- `primer.md` - Context primer for AI agents\n\n## Usage\n\nUse these files to bootstrap AI agent context for {} tasks.\n", content.use_case.name, content.use_case.name);
    fs::write(output_path.join("README.md"), readme_content)?;

    Ok(())
}
