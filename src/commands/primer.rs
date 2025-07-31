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

use crate::{Rhema, RhemaResult, RhemaScope, TroubleshootingItem, IntegrationGuide};
use colored::*;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Generate context primer files
pub fn run(
    rhema: &Rhema,
    scope_name: Option<&str>,
    output_dir: Option<&str>,
    template_type: Option<&str>,
    include_examples: bool,
    validate: bool,
) -> RhemaResult<()> {
    let scopes = if let Some(name) = scope_name {
        vec![rhema.load_scope(name)?]
    } else {
        rhema.list_scopes()?
    };
    
    let output_path = if let Some(dir) = output_dir {
        PathBuf::from(dir)
    } else {
        std::env::current_dir()?.join("rhema-primer")
    };
    
    // Create output directory
    fs::create_dir_all(&output_path)?;
    
    for scope in scopes {
        generate_scope_primer(rhema, &scope.definition, &output_path, template_type, include_examples, validate)?;
    }
    
    println!("{}", "✓ Context primers generated successfully!".green());
    println!("  Output directory: {}", output_path.display().to_string().yellow());
    
    Ok(())
}

/// Primer structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextPrimer {
    /// Primer metadata
    pub metadata: PrimerMetadata,
    
    /// Scope information
    pub scope: ScopePrimer,
    
    /// Protocol information
    pub protocol: Option<ProtocolPrimer>,
    
    /// Quick start guide
    pub quick_start: QuickStartGuide,
    
    /// Usage examples
    pub examples: Option<Vec<UsageExample>>,
    
    /// Common patterns
    pub patterns: Option<Vec<CommonPattern>>,
    
    /// Troubleshooting
    pub troubleshooting: Option<Vec<TroubleshootingItem>>,
    
    /// Integration guides
    pub integrations: Option<Vec<IntegrationGuide>>,
}

/// Primer metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrimerMetadata {
    /// Primer version
    pub version: String,
    
    /// Generation timestamp
    pub generated_at: String,
    
    /// Scope name
    pub scope_name: String,
    
    /// Template type
    pub template_type: String,
}

/// Scope primer information
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScopePrimer {
    /// Scope name
    pub name: String,
    
    /// Scope type
    pub scope_type: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Version
    pub version: String,
    
    /// Key responsibilities
    pub responsibilities: Vec<String>,
    
    /// Dependencies
    pub dependencies: Option<Vec<String>>,
}

/// Protocol primer information
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProtocolPrimer {
    /// Protocol version
    pub version: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Key concepts
    pub concepts: Vec<ConceptPrimer>,
    
    /// CQL examples
    pub cql_examples: Vec<CqlExamplePrimer>,
}

/// Concept primer
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConceptPrimer {
    /// Concept name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Usage context
    pub usage_context: Option<String>,
}

/// CQL example primer
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CqlExamplePrimer {
    /// Example name
    pub name: String,
    
    /// Query
    pub query: String,
    
    /// Description
    pub description: String,
    
    /// Expected output
    pub expected_output: Option<String>,
}

/// Quick start guide
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuickStartGuide {
    /// Setup steps
    pub setup_steps: Vec<String>,
    
    /// Basic commands
    pub basic_commands: Vec<CommandExample>,
    
    /// Next steps
    pub next_steps: Vec<String>,
}

/// Command example
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommandExample {
    /// Command description
    pub description: String,
    
    /// Command
    pub command: String,
    
    /// Expected output
    pub expected_output: Option<String>,
}

/// Usage example
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsageExample {
    /// Example name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Steps
    pub steps: Vec<String>,
    
    /// Expected outcome
    pub expected_outcome: String,
}

/// Common pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommonPattern {
    /// Pattern name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// When to use
    pub when_to_use: String,
    
    /// Implementation
    pub implementation: Vec<String>,
}

/// Generate primer for a specific scope
fn generate_scope_primer(
    rhema: &Rhema,
    scope: &RhemaScope,
    output_path: &PathBuf,
    template_type: Option<&str>,
    include_examples: bool,
    validate: bool,
) -> RhemaResult<()> {
    let template = template_type.unwrap_or("default");
    
    // Create scope-specific output directory
    let scope_output = output_path.join(&scope.name);
    fs::create_dir_all(&scope_output)?;
    
    // Generate primer content
    let primer = create_primer_content(rhema, scope, template, include_examples)?;
    
    // Write primer files
    write_primer_files(&primer, &scope_output)?;
    
    // Validate if requested
    if validate {
        validate_primer(&primer)?;
    }
    
    println!("  Generated primer for scope: {}", scope.name.yellow());
    
    Ok(())
}

/// Create primer content
fn create_primer_content(
    _rhema: &Rhema,
    scope: &RhemaScope,
    template: &str,
    include_examples: bool,
) -> RhemaResult<ContextPrimer> {
    let metadata = PrimerMetadata {
        version: "1.0.0".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        scope_name: scope.name.clone(),
        template_type: template.to_string(),
    };
    
    let scope_primer = ScopePrimer {
        name: scope.name.clone(),
        scope_type: scope.scope_type.clone(),
        description: scope.description.clone(),
        version: scope.version.clone(),
        responsibilities: infer_responsibilities(scope),
        dependencies: scope.dependencies.as_ref().map(|deps| {
            deps.iter().map(|d| d.path.clone()).collect()
        }),
    };
    
    let protocol = if let Some(ref proto) = scope.protocol_info {
        Some(ProtocolPrimer {
            version: proto.version.clone(),
            description: proto.description.clone(),
            concepts: proto.concepts.as_ref().map(|c| {
                c.iter().map(|concept| ConceptPrimer {
                    name: concept.name.clone(),
                    description: concept.description.clone(),
                    usage_context: None,
                }).collect()
            }).unwrap_or_default(),
            cql_examples: proto.cql_examples.as_ref().map(|e| {
                e.iter().map(|ex| CqlExamplePrimer {
                    name: ex.name.clone(),
                    query: ex.query.clone(),
                    description: ex.description.clone(),
                    expected_output: ex.output_format.clone(),
                }).collect()
            }).unwrap_or_default(),
        })
    } else {
        None
    };
    
    let quick_start = create_quick_start_guide(scope, template);
    let examples = if include_examples {
        create_usage_examples(scope, template)
    } else {
        None
    };
    
    let patterns = create_common_patterns(scope, template);
    let troubleshooting = create_troubleshooting(scope, template);
    let integrations = create_integration_guides(scope, template);
    
    Ok(ContextPrimer {
        metadata,
        scope: scope_primer,
        protocol,
        quick_start,
        examples,
        patterns,
        troubleshooting,
        integrations,
    })
}

/// Infer scope responsibilities
fn infer_responsibilities(scope: &RhemaScope) -> Vec<String> {
    match scope.scope_type.as_str() {
        "service" => vec![
            "API endpoint management".to_string(),
            "Business logic implementation".to_string(),
            "Data processing and validation".to_string(),
            "Integration with external services".to_string(),
        ],
        "app" => vec![
            "User interface management".to_string(),
            "User interaction handling".to_string(),
            "Data presentation and formatting".to_string(),
            "Client-side state management".to_string(),
        ],
        "library" => vec![
            "Reusable functionality provision".to_string(),
            "API abstraction and simplification".to_string(),
            "Utility function implementation".to_string(),
            "Cross-platform compatibility".to_string(),
        ],
        "tool" => vec![
            "Command-line interface".to_string(),
            "Automation and scripting".to_string(),
            "Data transformation".to_string(),
            "System integration".to_string(),
        ],
        _ => vec![
            "Core functionality implementation".to_string(),
            "Data management and persistence".to_string(),
            "Configuration and customization".to_string(),
            "Error handling and logging".to_string(),
        ],
    }
}

/// Create quick start guide
fn create_quick_start_guide(scope: &RhemaScope, template: &str) -> QuickStartGuide {
    let setup_steps = match template {
        "service" => vec![
            "Install Rhema CLI".to_string(),
            "Navigate to service directory".to_string(),
            "Run `rhema init --scope-type service`".to_string(),
            "Configure service-specific settings".to_string(),
            "Validate configuration with `rhema validate`".to_string(),
        ],
        "app" => vec![
            "Install Rhema CLI".to_string(),
            "Navigate to application directory".to_string(),
            "Run `rhema init --scope-type app`".to_string(),
            "Configure UI framework settings".to_string(),
            "Set up build and deployment scripts".to_string(),
        ],
        "library" => vec![
            "Install Rhema CLI".to_string(),
            "Navigate to library directory".to_string(),
            "Run `rhema init --scope-type library`".to_string(),
            "Configure API documentation settings".to_string(),
            "Set up testing framework".to_string(),
        ],
        _ => vec![
            "Install Rhema CLI".to_string(),
            "Navigate to project directory".to_string(),
            "Run `rhema init`".to_string(),
            "Configure project settings".to_string(),
            "Validate setup".to_string(),
        ],
    };
    
    let basic_commands = vec![
        CommandExample {
            description: "Show scope information".to_string(),
            command: format!("rhema scope {}", scope.name),
            expected_output: Some("Displays scope details and configuration".to_string()),
        },
        CommandExample {
            description: "Query context data".to_string(),
            command: "rhema query 'SELECT * FROM knowledge'".to_string(),
            expected_output: Some("Returns knowledge entries".to_string()),
        },
        CommandExample {
            description: "Search for specific content".to_string(),
            command: "rhema search 'authentication'".to_string(),
            expected_output: Some("Finds content containing 'authentication'".to_string()),
        },
        CommandExample {
            description: "Validate configuration".to_string(),
            command: "rhema validate".to_string(),
            expected_output: Some("Validates all Rhema files".to_string()),
        },
    ];
    
    let next_steps = vec![
        "Explore available commands with `rhema --help`".to_string(),
        "Add knowledge entries with `rhema insight add`".to_string(),
        "Create todo items with `rhema todo add`".to_string(),
        "Document decisions with `rhema decision add`".to_string(),
        "Define patterns with `rhema pattern add`".to_string(),
    ];
    
    QuickStartGuide {
        setup_steps,
        basic_commands,
        next_steps,
    }
}

/// Create usage examples
fn create_usage_examples(_scope: &RhemaScope, template: &str) -> Option<Vec<UsageExample>> {
    let examples = match template {
        "service" => vec![
            UsageExample {
                name: "API Documentation".to_string(),
                description: "Document API endpoints and their usage".to_string(),
                steps: vec![
                    "Use `rhema insight add` to document endpoints".to_string(),
                    "Include request/response examples".to_string(),
                    "Add authentication requirements".to_string(),
                ],
                expected_outcome: "Comprehensive API documentation in knowledge base".to_string(),
            },
            UsageExample {
                name: "Service Dependencies".to_string(),
                description: "Track service dependencies and integration points".to_string(),
                steps: vec![
                    "Use `rhema dependencies` to view dependencies".to_string(),
                    "Document integration patterns".to_string(),
                    "Track external service requirements".to_string(),
                ],
                expected_outcome: "Clear dependency mapping and integration documentation".to_string(),
            },
        ],
        "app" => vec![
            UsageExample {
                name: "UI Component Documentation".to_string(),
                description: "Document reusable UI components and their usage".to_string(),
                steps: vec![
                    "Use `rhema insight add` to document components".to_string(),
                    "Include props and usage examples".to_string(),
                    "Add accessibility considerations".to_string(),
                ],
                expected_outcome: "Component library documentation".to_string(),
            },
        ],
        _ => vec![
            UsageExample {
                name: "Project Setup".to_string(),
                description: "Document project setup and configuration".to_string(),
                steps: vec![
                    "Use `rhema insight add` to document setup steps".to_string(),
                    "Include environment requirements".to_string(),
                    "Add troubleshooting tips".to_string(),
                ],
                expected_outcome: "Complete setup documentation".to_string(),
            },
        ],
    };
    
    Some(examples)
}

/// Create common patterns
fn create_common_patterns(_scope: &RhemaScope, template: &str) -> Option<Vec<CommonPattern>> {
    let patterns = match template {
        "service" => vec![
            CommonPattern {
                name: "Error Handling".to_string(),
                description: "Standardized error handling across the service".to_string(),
                when_to_use: "When implementing API endpoints or business logic".to_string(),
                implementation: vec![
                    "Use consistent error response format".to_string(),
                    "Include error codes and messages".to_string(),
                    "Log errors with appropriate context".to_string(),
                ],
            },
            CommonPattern {
                name: "Data Validation".to_string(),
                description: "Input validation and sanitization patterns".to_string(),
                when_to_use: "When processing user input or external data".to_string(),
                implementation: vec![
                    "Validate all input parameters".to_string(),
                    "Use schema validation where possible".to_string(),
                    "Sanitize data before processing".to_string(),
                ],
            },
        ],
        _ => vec![
            CommonPattern {
                name: "Configuration Management".to_string(),
                description: "Managing configuration across environments".to_string(),
                when_to_use: "When deploying to different environments".to_string(),
                implementation: vec![
                    "Use environment-specific config files".to_string(),
                    "Validate configuration on startup".to_string(),
                    "Provide sensible defaults".to_string(),
                ],
            },
        ],
    };
    
    Some(patterns)
}

/// Create troubleshooting guide
fn create_troubleshooting(_scope: &RhemaScope, _template: &str) -> Option<Vec<TroubleshootingItem>> {
    let items = vec![
        TroubleshootingItem {
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
        TroubleshootingItem {
            issue: "Scope not found".to_string(),
            description: "Rhema cannot locate the specified scope".to_string(),
            solution: vec![
                "Verify scope path is correct".to_string(),
                "Check if scope directory exists".to_string(),
                "Ensure rhema.yaml file is present".to_string(),
            ],
            prevention: Some(vec![
                "Use `rhema scopes` to list available scopes".to_string(),
                "Initialize scopes properly with `rhema init`".to_string(),
            ]),
        },
    ];
    
    Some(items)
}

/// Create integration guides
fn create_integration_guides(_scope: &RhemaScope, _template: &str) -> Option<Vec<IntegrationGuide>> {
    let guides = vec![
        IntegrationGuide {
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
        IntegrationGuide {
            name: "CI/CD Integration".to_string(),
            description: "Integrate Rhema validation into CI/CD pipelines".to_string(),
            setup: Some(vec![
                "Add Rhema CLI to CI environment".to_string(),
                "Configure validation steps".to_string(),
                "Set up reporting".to_string(),
            ]),
            configuration: Some(vec![
                "Run `rhema validate` in build pipeline".to_string(),
                "Generate context reports for deployment".to_string(),
            ]),
            best_practices: Some(vec![
                "Fail builds on validation errors".to_string(),
                "Include context in deployment artifacts".to_string(),
            ]),
        },
    ];
    
    Some(guides)
}

/// Write primer files
fn write_primer_files(primer: &ContextPrimer, output_path: &PathBuf) -> RhemaResult<()> {
    // Write YAML primer
    let yaml_content = serde_yaml::to_string(primer)?;
    fs::write(output_path.join("primer.yaml"), yaml_content)?;
    
    // Write JSON primer
    let json_content = serde_json::to_string_pretty(primer)?;
    fs::write(output_path.join("primer.json"), json_content)?;
    
    // Write markdown primer
    let md_content = format_markdown_primer(primer);
    fs::write(output_path.join("primer.md"), md_content)?;
    
    // Write text primer
    let text_content = format_text_primer(primer);
    fs::write(output_path.join("primer.txt"), text_content)?;
    
    Ok(())
}

/// Format markdown primer
fn format_markdown_primer(primer: &ContextPrimer) -> String {
    let mut md = String::new();
    
    md.push_str(&format!("# {} Context Primer\n\n", primer.scope.name));
    md.push_str(&format!("**Generated:** {}\n", primer.metadata.generated_at));
    md.push_str(&format!("**Template:** {}\n\n", primer.metadata.template_type));
    
    // Scope information
    md.push_str("## Scope Information\n\n");
    md.push_str(&format!("**Name:** {}\n", primer.scope.name));
    md.push_str(&format!("**Type:** {}\n", primer.scope.scope_type));
    md.push_str(&format!("**Version:** {}\n", primer.scope.version));
    if let Some(ref desc) = primer.scope.description {
        md.push_str(&format!("**Description:** {}\n", desc));
    }
    md.push_str("\n");
    
    // Responsibilities
    md.push_str("### Key Responsibilities\n\n");
    for responsibility in &primer.scope.responsibilities {
        md.push_str(&format!("- {}\n", responsibility));
    }
    md.push_str("\n");
    
    // Quick start
    md.push_str("## Quick Start Guide\n\n");
    md.push_str("### Setup Steps\n\n");
    for (i, step) in primer.quick_start.setup_steps.iter().enumerate() {
        md.push_str(&format!("{}. {}\n", i + 1, step));
    }
    md.push_str("\n");
    
    md.push_str("### Basic Commands\n\n");
    for cmd in &primer.quick_start.basic_commands {
        md.push_str(&format!("**{}**\n", cmd.description));
        md.push_str(&format!("```bash\n{}\n```\n", cmd.command));
        if let Some(ref output) = cmd.expected_output {
            md.push_str(&format!("*{}*\n", output));
        }
        md.push_str("\n");
    }
    
    // Protocol information
    if let Some(ref protocol) = primer.protocol {
        md.push_str("## Protocol Information\n\n");
        md.push_str(&format!("**Version:** {}\n", protocol.version));
        if let Some(ref desc) = protocol.description {
            md.push_str(&format!("**Description:** {}\n", desc));
        }
        md.push_str("\n");
        
        if !protocol.concepts.is_empty() {
            md.push_str("### Key Concepts\n\n");
            for concept in &protocol.concepts {
                md.push_str(&format!("#### {}\n", concept.name));
                md.push_str(&format!("{}\n\n", concept.description));
            }
        }
        
        if !protocol.cql_examples.is_empty() {
            md.push_str("### CQL Examples\n\n");
            for example in &protocol.cql_examples {
                md.push_str(&format!("#### {}\n", example.name));
                md.push_str(&format!("**Query:** `{}`\n", example.query));
                md.push_str(&format!("**Description:** {}\n\n", example.description));
            }
        }
    }
    
    // Usage examples
    if let Some(ref examples) = primer.examples {
        md.push_str("## Usage Examples\n\n");
        for example in examples {
            md.push_str(&format!("### {}\n", example.name));
            md.push_str(&format!("{}\n\n", example.description));
            md.push_str("**Steps:**\n");
            for step in &example.steps {
                md.push_str(&format!("- {}\n", step));
            }
            md.push_str(&format!("**Expected Outcome:** {}\n\n", example.expected_outcome));
        }
    }
    
    // Common patterns
    if let Some(ref patterns) = primer.patterns {
        md.push_str("## Common Patterns\n\n");
        for pattern in patterns {
            md.push_str(&format!("### {}\n", pattern.name));
            md.push_str(&format!("{}\n\n", pattern.description));
            md.push_str(&format!("**When to use:** {}\n\n", pattern.when_to_use));
            md.push_str("**Implementation:**\n");
            for step in &pattern.implementation {
                md.push_str(&format!("- {}\n", step));
            }
            md.push_str("\n");
        }
    }
    
    // Troubleshooting
    if let Some(ref troubleshooting) = primer.troubleshooting {
        md.push_str("## Troubleshooting\n\n");
        for item in troubleshooting {
            md.push_str(&format!("### {}\n", item.issue));
            md.push_str(&format!("{}\n\n", item.description));
            md.push_str("**Solution:**\n");
            for step in &item.solution {
                md.push_str(&format!("- {}\n", step));
            }
            if let Some(ref prevention) = item.prevention {
                md.push_str("\n**Prevention:**\n");
                for tip in prevention {
                    md.push_str(&format!("- {}\n", tip));
                }
            }
            md.push_str("\n");
        }
    }
    
    // Integration guides
    if let Some(ref integrations) = primer.integrations {
        md.push_str("## Integration Guides\n\n");
        for integration in integrations {
            md.push_str(&format!("### {}\n", integration.name));
            md.push_str(&format!("{}\n\n", integration.description));
            
            if let Some(ref setup) = integration.setup {
                md.push_str("**Setup:**\n");
                for step in setup {
                    md.push_str(&format!("- {}\n", step));
                }
                md.push_str("\n");
            }
            
            if let Some(ref config) = integration.configuration {
                md.push_str("**Configuration:**\n");
                for item in config {
                    md.push_str(&format!("- {}\n", item));
                }
                md.push_str("\n");
            }
            
            if let Some(ref practices) = integration.best_practices {
                md.push_str("**Best Practices:**\n");
                for practice in practices {
                    md.push_str(&format!("- {}\n", practice));
                }
                md.push_str("\n");
            }
        }
    }
    
    md
}

/// Format text primer
fn format_text_primer(primer: &ContextPrimer) -> String {
    let mut text = String::new();
    
    text.push_str(&format!("{} CONTEXT PRIMER\n", primer.scope.name.to_uppercase()));
    text.push_str(&"=".repeat(primer.scope.name.len() + 16));
    text.push_str("\n\n");
    
    text.push_str(&format!("Generated: {}\n", primer.metadata.generated_at));
    text.push_str(&format!("Template: {}\n\n", primer.metadata.template_type));
    
    text.push_str("SCOPE INFORMATION:\n");
    text.push_str("------------------\n");
    text.push_str(&format!("Name: {}\n", primer.scope.name));
    text.push_str(&format!("Type: {}\n", primer.scope.scope_type));
    text.push_str(&format!("Version: {}\n", primer.scope.version));
    if let Some(ref desc) = primer.scope.description {
        text.push_str(&format!("Description: {}\n", desc));
    }
    text.push_str("\n");
    
    text.push_str("KEY RESPONSIBILITIES:\n");
    for responsibility in &primer.scope.responsibilities {
        text.push_str(&format!("- {}\n", responsibility));
    }
    text.push_str("\n");
    
    text.push_str("QUICK START GUIDE:\n");
    text.push_str("-----------------\n");
    text.push_str("Setup Steps:\n");
    for (i, step) in primer.quick_start.setup_steps.iter().enumerate() {
        text.push_str(&format!("{}. {}\n", i + 1, step));
    }
    text.push_str("\n");
    
    text.push_str("Basic Commands:\n");
    for cmd in &primer.quick_start.basic_commands {
        text.push_str(&format!("- {}: {}\n", cmd.description, cmd.command));
    }
    text.push_str("\n");
    
    if let Some(ref protocol) = primer.protocol {
        text.push_str("PROTOCOL INFORMATION:\n");
        text.push_str("---------------------\n");
        text.push_str(&format!("Version: {}\n", protocol.version));
        if let Some(ref desc) = protocol.description {
            text.push_str(&format!("Description: {}\n", desc));
        }
        text.push_str("\n");
        
        if !protocol.concepts.is_empty() {
            text.push_str("Key Concepts:\n");
            for concept in &protocol.concepts {
                text.push_str(&format!("- {}: {}\n", concept.name, concept.description));
            }
            text.push_str("\n");
        }
    }
    
    text
}

/// Validate primer
fn validate_primer(primer: &ContextPrimer) -> RhemaResult<()> {
    // Basic validation
    if primer.scope.name.is_empty() {
        return Err(crate::RhemaError::ValidationError(
            "Scope name cannot be empty".to_string()
        ));
    }
    
    if primer.scope.responsibilities.is_empty() {
        return Err(crate::RhemaError::ValidationError(
            "Scope must have at least one responsibility".to_string()
        ));
    }
    
    if primer.quick_start.setup_steps.is_empty() {
        return Err(crate::RhemaError::ValidationError(
            "Quick start guide must have setup steps".to_string()
        ));
    }
    
    if primer.quick_start.basic_commands.is_empty() {
        return Err(crate::RhemaError::ValidationError(
            "Quick start guide must have basic commands".to_string()
        ));
    }
    
    println!("  ✓ Primer validation passed");
    Ok(())
} 