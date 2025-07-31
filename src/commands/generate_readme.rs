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
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Generate README files with Rhema context
pub fn run(
    rhema: &Rhema,
    scope_name: Option<&str>,
    output_file: Option<&str>,
    template: Option<&str>,
    include_context: bool,
    seo_optimized: bool,
    custom_sections: Option<Vec<String>>,
) -> RhemaResult<()> {
    let scopes = if let Some(name) = scope_name {
        vec![rhema.load_scope(name)?]
    } else {
        rhema.list_scopes()?
    };
    
    for scope in scopes {
        generate_scope_readme(
            rhema,
            &scope.definition,
            output_file,
            template,
            include_context,
            seo_optimized,
            &custom_sections,
        )?;
    }
    
    println!("{}", "✓ README files generated successfully!".green());
    
    Ok(())
}

/// README content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReadmeContent {
    /// Project title
    pub title: String,
    
    /// Project description
    pub description: String,
    
    /// Badges
    pub badges: Vec<Badge>,
    
    /// Table of contents
    pub table_of_contents: bool,
    
    /// Installation section
    pub installation: InstallationSection,
    
    /// Usage section
    pub usage: UsageSection,
    
    /// Features section
    pub features: Vec<String>,
    
    /// API documentation
    pub api_docs: Option<ApiDocsSection>,
    
    /// Configuration
    pub configuration: Option<ConfigurationSection>,
    
    /// Development
    pub development: DevelopmentSection,
    
    /// Contributing
    pub contributing: ContributingSection,
    
    /// License
    pub license: LicenseSection,
    
    /// Context information
    pub context: Option<ContextSection>,
    
    /// Custom sections
    pub custom_sections: HashMap<String, String>,
}

/// Badge information
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Badge {
    /// Badge text
    pub text: String,
    
    /// Badge URL
    pub url: String,
    
    /// Badge image URL
    pub image_url: String,
}

/// Installation section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstallationSection {
    /// Prerequisites
    pub prerequisites: Vec<String>,
    
    /// Installation steps
    pub steps: Vec<String>,
    
    /// Quick start
    pub quick_start: Option<String>,
}

/// Usage section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsageSection {
    /// Basic usage
    pub basic_usage: String,
    
    /// Examples
    pub examples: Vec<UsageExample>,
    
    /// Command reference
    pub commands: Option<Vec<CommandReference>>,
}

/// Usage example
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsageExample {
    /// Example title
    pub title: String,
    
    /// Example description
    pub description: String,
    
    /// Example code
    pub code: String,
    
    /// Expected output
    pub output: Option<String>,
}

/// Command reference
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommandReference {
    /// Command name
    pub command: String,
    
    /// Description
    pub description: String,
    
    /// Options
    pub options: Vec<String>,
}

/// API documentation section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiDocsSection {
    /// API overview
    pub overview: String,
    
    /// Endpoints
    pub endpoints: Vec<ApiEndpoint>,
    
    /// Authentication
    pub authentication: Option<String>,
}

/// API endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiEndpoint {
    /// HTTP method
    pub method: String,
    
    /// Path
    pub path: String,
    
    /// Description
    pub description: String,
    
    /// Parameters
    pub parameters: Vec<String>,
    
    /// Response
    pub response: Option<String>,
}

/// Configuration section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigurationSection {
    /// Configuration overview
    pub overview: String,
    
    /// Environment variables
    pub environment_variables: Vec<EnvVar>,
    
    /// Configuration files
    pub config_files: Vec<ConfigFile>,
}

/// Environment variable
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EnvVar {
    /// Variable name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Default value
    pub default: Option<String>,
    
    /// Required
    pub required: bool,
}

/// Configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigFile {
    /// File name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Example content
    pub example: Option<String>,
}

/// Development section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DevelopmentSection {
    /// Setup instructions
    pub setup: Vec<String>,
    
    /// Testing
    pub testing: String,
    
    /// Building
    pub building: String,
    
    /// Code style
    pub code_style: Option<String>,
}

/// Contributing section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContributingSection {
    /// Contributing guidelines
    pub guidelines: Vec<String>,
    
    /// Development workflow
    pub workflow: Vec<String>,
    
    /// Code of conduct
    pub code_of_conduct: Option<String>,
}

/// License section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LicenseSection {
    /// License type
    pub license_type: String,
    
    /// License text
    pub license_text: Option<String>,
    
    /// Copyright
    pub copyright: Option<String>,
}

/// Context section
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextSection {
    /// Context overview
    pub overview: String,
    
    /// Key concepts
    pub concepts: Vec<String>,
    
    /// Rhema integration
    pub rhema_integration: String,
    
    /// Context queries
    pub context_queries: Vec<String>,
}

/// Generate README for a specific scope
fn generate_scope_readme(
    rhema: &Rhema,
    scope: &RhemaScope,
    output_file: Option<&str>,
    template: Option<&str>,
    include_context: bool,
    seo_optimized: bool,
    custom_sections: &Option<Vec<String>>,
) -> RhemaResult<()> {
    let template_type = template.unwrap_or("default");
    
    // Create README content
    let content = create_readme_content(rhema, scope, template_type, include_context, seo_optimized, custom_sections)?;
    
    // Determine output file
    let output_path = if let Some(file) = output_file {
        PathBuf::from(file)
    } else {
        PathBuf::from("README.md")
    };
    
    // Generate README
    let readme_content = format_readme(&content, seo_optimized)?;
    
    // Write to file
    fs::write(&output_path, readme_content)?;
    
    println!("  Generated README for scope: {}", scope.name.yellow());
    println!("  Output: {}", output_path.display().to_string().yellow());
    
    Ok(())
}

/// Create README content
fn create_readme_content(
    rhema: &Rhema,
    scope: &RhemaScope,
    template: &str,
    include_context: bool,
    seo_optimized: bool,
    custom_sections: &Option<Vec<String>>,
) -> RhemaResult<ReadmeContent> {
    let title = format!("{}", scope.name);
    let description = scope.description.clone().unwrap_or_else(|| {
        format!("A {} implementation", scope.scope_type)
    });
    
    let badges = create_badges(scope, seo_optimized);
    
    let installation = create_installation_section(scope, template);
    let usage = create_usage_section(scope, template);
    let features = create_features(scope, template);
    
    let api_docs = if scope.scope_type == "service" {
        create_api_docs_section(scope)
    } else {
        None
    };
    
    let configuration = create_configuration_section(scope, template);
    let development = create_development_section(scope, template);
    let contributing = create_contributing_section();
    let license = create_license_section();
    
    let context = if include_context {
        create_context_section(rhema, scope)
    } else {
        None
    };
    
    let custom_sections_map = if let Some(ref sections) = custom_sections {
        let mut map = HashMap::new();
        for section in sections {
            map.insert(section.clone(), format!("Custom section: {}", section));
        }
        map
    } else {
        HashMap::new()
    };
    
    Ok(ReadmeContent {
        title,
        description,
        badges,
        table_of_contents: true,
        installation,
        usage,
        features,
        api_docs,
        configuration,
        development,
        contributing,
        license,
        context,
        custom_sections: custom_sections_map,
    })
}

/// Create badges
fn create_badges(scope: &RhemaScope, seo_optimized: bool) -> Vec<Badge> {
    let mut badges = vec![
        Badge {
            text: "License".to_string(),
            url: "https://opensource.org/licenses/Apache-2.0".to_string(),
            image_url: "https://img.shields.io/badge/License-Apache%202.0-blue.svg".to_string(),
        },
        Badge {
            text: "Rhema".to_string(),
            url: "https://github.com/fugue-ai/rhema".to_string(),
            image_url: "https://img.shields.io/badge/Rhema-Enabled-green.svg".to_string(),
        },
    ];
    
    if seo_optimized {
        badges.push(Badge {
            text: "Documentation".to_string(),
            url: "#".to_string(),
            image_url: "https://img.shields.io/badge/docs-complete-brightgreen.svg".to_string(),
        });
    }
    
    match scope.scope_type.as_str() {
        "service" => {
            badges.push(Badge {
                text: "API".to_string(),
                url: "#".to_string(),
                image_url: "https://img.shields.io/badge/API-REST-blue.svg".to_string(),
            });
        }
        "app" => {
            badges.push(Badge {
                text: "UI".to_string(),
                url: "#".to_string(),
                image_url: "https://img.shields.io/badge/UI-React-blue.svg".to_string(),
            });
        }
        "library" => {
            badges.push(Badge {
                text: "Library".to_string(),
                url: "#".to_string(),
                image_url: "https://img.shields.io/badge/Library-Rust-orange.svg".to_string(),
            });
        }
        _ => {}
    }
    
    badges
}

/// Create installation section
fn create_installation_section(_scope: &RhemaScope, template: &str) -> InstallationSection {
    let prerequisites = match template {
        "service" => vec![
            "Rust 1.70+".to_string(),
            "PostgreSQL 13+".to_string(),
            "Redis 6+".to_string(),
        ],
        "app" => vec![
            "Node.js 18+".to_string(),
            "npm or yarn".to_string(),
        ],
        "library" => vec![
            "Rust 1.70+".to_string(),
            "Cargo".to_string(),
        ],
        _ => vec![
            "Rust 1.70+".to_string(),
        ],
    };
    
    let steps = match template {
        "service" => vec![
            "Clone the repository".to_string(),
            "Install dependencies: `cargo build`".to_string(),
            "Set up environment variables".to_string(),
            "Run database migrations".to_string(),
            "Start the service: `cargo run`".to_string(),
        ],
        "app" => vec![
            "Clone the repository".to_string(),
            "Install dependencies: `npm install`".to_string(),
            "Set up environment variables".to_string(),
            "Start development server: `npm start`".to_string(),
        ],
        "library" => vec![
            "Clone the repository".to_string(),
            "Install dependencies: `cargo build`".to_string(),
            "Run tests: `cargo test`".to_string(),
        ],
        _ => vec![
            "Clone the repository".to_string(),
            "Install dependencies".to_string(),
            "Configure the project".to_string(),
            "Run the application".to_string(),
        ],
    };
    
    let quick_start = Some(match template {
        "service" => "```bash\ncargo run --bin service\n```".to_string(),
        "app" => "```bash\nnpm start\n```".to_string(),
        "library" => "```bash\ncargo test\n```".to_string(),
        _ => "```bash\ncargo run\n```".to_string(),
    });
    
    InstallationSection {
        prerequisites,
        steps,
        quick_start,
    }
}

/// Create usage section
fn create_usage_section(_scope: &RhemaScope, template: &str) -> UsageSection {
    let basic_usage = match template {
        "service" => "The service provides REST API endpoints for managing resources.".to_string(),
        "app" => "The application provides a user interface for interacting with the system.".to_string(),
        "library" => "The library provides reusable functions and utilities.".to_string(),
        _ => "This project provides core functionality for the system.".to_string(),
    };
    
    let examples = vec![
        UsageExample {
            title: "Basic Example".to_string(),
            description: "A simple example of using the main functionality.".to_string(),
            code: "```rust\n// Example code here\n```".to_string(),
            output: Some("Expected output here".to_string()),
        },
    ];
    
    let commands = Some(vec![
        CommandReference {
            command: "rhema scope".to_string(),
            description: "Show scope information".to_string(),
            options: vec!["--verbose".to_string(), "--format json".to_string()],
        },
        CommandReference {
            command: "rhema query".to_string(),
            description: "Execute CQL query".to_string(),
            options: vec!["--stats".to_string(), "--format yaml".to_string()],
        },
    ]);
    
    UsageSection {
        basic_usage,
        examples,
        commands,
    }
}

/// Create features
fn create_features(_scope: &RhemaScope, template: &str) -> Vec<String> {
    match template {
        "service" => vec![
            "RESTful API endpoints".to_string(),
            "Database integration".to_string(),
            "Authentication and authorization".to_string(),
            "Error handling and logging".to_string(),
            "Configuration management".to_string(),
        ],
        "app" => vec![
            "Modern user interface".to_string(),
            "Responsive design".to_string(),
            "State management".to_string(),
            "API integration".to_string(),
            "Error handling".to_string(),
        ],
        "library" => vec![
            "Reusable components".to_string(),
            "Type-safe interfaces".to_string(),
            "Comprehensive documentation".to_string(),
            "Unit tests".to_string(),
            "Performance optimized".to_string(),
        ],
        _ => vec![
            "Core functionality".to_string(),
            "Configuration support".to_string(),
            "Error handling".to_string(),
            "Documentation".to_string(),
        ],
    }
}

/// Create API docs section
fn create_api_docs_section(_scope: &RhemaScope) -> Option<ApiDocsSection> {
    Some(ApiDocsSection {
        overview: "The API provides RESTful endpoints for managing resources.".to_string(),
        endpoints: vec![
            ApiEndpoint {
                method: "GET".to_string(),
                path: "/api/v1/resources".to_string(),
                description: "List all resources".to_string(),
                parameters: vec!["page (optional)".to_string(), "limit (optional)".to_string()],
                response: Some("JSON array of resources".to_string()),
            },
            ApiEndpoint {
                method: "POST".to_string(),
                path: "/api/v1/resources".to_string(),
                description: "Create a new resource".to_string(),
                parameters: vec!["resource data (JSON)".to_string()],
                response: Some("Created resource (JSON)".to_string()),
            },
        ],
        authentication: Some("Bearer token authentication is required for all endpoints.".to_string()),
    })
}

/// Create configuration section
fn create_configuration_section(_scope: &RhemaScope, _template: &str) -> Option<ConfigurationSection> {
    Some(ConfigurationSection {
        overview: "Configuration is handled through environment variables and configuration files.".to_string(),
        environment_variables: vec![
            EnvVar {
                name: "DATABASE_URL".to_string(),
                description: "Database connection string".to_string(),
                default: Some("postgresql://localhost/db".to_string()),
                required: true,
            },
            EnvVar {
                name: "LOG_LEVEL".to_string(),
                description: "Logging level".to_string(),
                default: Some("info".to_string()),
                required: false,
            },
        ],
        config_files: vec![
            ConfigFile {
                name: "config.yaml".to_string(),
                description: "Main configuration file".to_string(),
                example: Some("```yaml\n# Configuration example\n```".to_string()),
            },
        ],
    })
}

/// Create development section
fn create_development_section(_scope: &RhemaScope, template: &str) -> DevelopmentSection {
    let setup = match template {
        "service" => vec![
            "Clone the repository".to_string(),
            "Install Rust toolchain".to_string(),
            "Set up development database".to_string(),
            "Install development dependencies".to_string(),
        ],
        "app" => vec![
            "Clone the repository".to_string(),
            "Install Node.js".to_string(),
            "Install dependencies".to_string(),
            "Set up development environment".to_string(),
        ],
        _ => vec![
            "Clone the repository".to_string(),
            "Install dependencies".to_string(),
            "Set up development environment".to_string(),
        ],
    };
    
    let testing = match template {
        "service" => "Run tests with `cargo test`".to_string(),
        "app" => "Run tests with `npm test`".to_string(),
        _ => "Run tests with `cargo test`".to_string(),
    };
    
    let building = match template {
        "service" => "Build with `cargo build --release`".to_string(),
        "app" => "Build with `npm run build`".to_string(),
        _ => "Build with `cargo build`".to_string(),
    };
    
    let code_style = Some("Follow the project's coding standards and run linters.".to_string());
    
    DevelopmentSection {
        setup,
        testing,
        building,
        code_style,
    }
}

/// Create contributing section
fn create_contributing_section() -> ContributingSection {
    ContributingSection {
        guidelines: vec![
            "Fork the repository".to_string(),
            "Create a feature branch".to_string(),
            "Make your changes".to_string(),
            "Add tests for new functionality".to_string(),
            "Submit a pull request".to_string(),
        ],
        workflow: vec![
            "Ensure all tests pass".to_string(),
            "Update documentation".to_string(),
            "Follow coding standards".to_string(),
            "Provide clear commit messages".to_string(),
        ],
        code_of_conduct: Some("Please read our Code of Conduct before contributing.".to_string()),
    }
}

/// Create license section
fn create_license_section() -> LicenseSection {
    LicenseSection {
        license_type: "Apache 2.0".to_string(),
        license_text: None,
        copyright: Some("Copyright 2025 Cory Parent".to_string()),
    }
}

/// Create context section
fn create_context_section(_rhema: &Rhema, _scope: &RhemaScope) -> Option<ContextSection> {
    let overview = "This project uses Rhema (Git-Based Agent Context Protocol) for comprehensive context management and documentation.".to_string();
    
    let concepts = vec![
        "Knowledge Management".to_string(),
        "Decision Tracking".to_string(),
        "Pattern Documentation".to_string(),
        "Context Query Language (CQL)".to_string(),
    ];
    
    let rhema_integration = "The project integrates with Rhema to provide AI agents with comprehensive context about the codebase, decisions, and patterns.".to_string();
    
    let context_queries = vec![
        "`rhema query 'SELECT * FROM knowledge WHERE category = \"api\"'`".to_string(),
        "`rhema query 'SELECT * FROM decisions WHERE status = \"approved\"'`".to_string(),
        "`rhema query 'SELECT * FROM patterns WHERE pattern_type = \"security\"'`".to_string(),
    ];
    
    Some(ContextSection {
        overview,
        concepts,
        rhema_integration,
        context_queries,
    })
}

/// Format README content
fn format_readme(content: &ReadmeContent, _seo_optimized: bool) -> RhemaResult<String> {
    let mut md = String::new();
    
    // Title and badges
    md.push_str(&format!("# {}\n\n", content.title));
    md.push_str(&content.description);
    md.push_str("\n\n");
    
    // Badges
    for badge in &content.badges {
        md.push_str(&format!("[![{}]({})]({}) ", badge.text, badge.image_url, badge.url));
    }
    md.push_str("\n\n");
    
    // Table of contents
    if content.table_of_contents {
        md.push_str("## Table of Contents\n\n");
        md.push_str("- [Installation](#installation)\n");
        md.push_str("- [Usage](#usage)\n");
        md.push_str("- [Features](#features)\n");
        if content.api_docs.is_some() {
            md.push_str("- [API Documentation](#api-documentation)\n");
        }
        if content.configuration.is_some() {
            md.push_str("- [Configuration](#configuration)\n");
        }
        md.push_str("- [Development](#development)\n");
        md.push_str("- [Contributing](#contributing)\n");
        if content.context.is_some() {
            md.push_str("- [Context Management](#context-management)\n");
        }
        md.push_str("- [License](#license)\n\n");
    }
    
    // Installation
    md.push_str("## Installation\n\n");
    md.push_str("### Prerequisites\n\n");
    for prereq in &content.installation.prerequisites {
        md.push_str(&format!("- {}\n", prereq));
    }
    md.push_str("\n");
    
    md.push_str("### Steps\n\n");
    for (i, step) in content.installation.steps.iter().enumerate() {
        md.push_str(&format!("{}. {}\n", i + 1, step));
    }
    md.push_str("\n");
    
    if let Some(ref quick_start) = content.installation.quick_start {
        md.push_str("### Quick Start\n\n");
        md.push_str(quick_start);
        md.push_str("\n\n");
    }
    
    // Usage
    md.push_str("## Usage\n\n");
    md.push_str(&content.usage.basic_usage);
    md.push_str("\n\n");
    
    if !content.usage.examples.is_empty() {
        md.push_str("### Examples\n\n");
        for example in &content.usage.examples {
            md.push_str(&format!("#### {}\n", example.title));
            md.push_str(&format!("{}\n\n", example.description));
            md.push_str(&example.code);
            md.push_str("\n");
            if let Some(ref output) = example.output {
                md.push_str(&format!("**Output:**\n```\n{}\n```\n", output));
            }
            md.push_str("\n");
        }
    }
    
    if let Some(ref commands) = content.usage.commands {
        md.push_str("### Commands\n\n");
        for cmd in commands {
            md.push_str(&format!("#### `{}`\n", cmd.command));
            md.push_str(&format!("{}\n\n", cmd.description));
            if !cmd.options.is_empty() {
                md.push_str("**Options:**\n");
                for option in &cmd.options {
                    md.push_str(&format!("- `{}`\n", option));
                }
                md.push_str("\n");
            }
        }
    }
    
    // Features
    md.push_str("## Features\n\n");
    for feature in &content.features {
        md.push_str(&format!("- {}\n", feature));
    }
    md.push_str("\n");
    
    // API Documentation
    if let Some(ref api_docs) = content.api_docs {
        md.push_str("## API Documentation\n\n");
        md.push_str(&api_docs.overview);
        md.push_str("\n\n");
        
        md.push_str("### Endpoints\n\n");
        for endpoint in &api_docs.endpoints {
            md.push_str(&format!("#### `{} {}`\n", endpoint.method, endpoint.path));
            md.push_str(&format!("{}\n\n", endpoint.description));
            if !endpoint.parameters.is_empty() {
                md.push_str("**Parameters:**\n");
                for param in &endpoint.parameters {
                    md.push_str(&format!("- {}\n", param));
                }
                md.push_str("\n");
            }
            if let Some(ref response) = endpoint.response {
                md.push_str(&format!("**Response:** {}\n\n", response));
            }
        }
        
        if let Some(ref auth) = api_docs.authentication {
            md.push_str("### Authentication\n\n");
            md.push_str(auth);
            md.push_str("\n\n");
        }
    }
    
    // Configuration
    if let Some(ref config) = content.configuration {
        md.push_str("## Configuration\n\n");
        md.push_str(&config.overview);
        md.push_str("\n\n");
        
        if !config.environment_variables.is_empty() {
            md.push_str("### Environment Variables\n\n");
            for env_var in &config.environment_variables {
                md.push_str(&format!("#### `{}`\n", env_var.name));
                md.push_str(&format!("{}\n", env_var.description));
                if let Some(ref default) = env_var.default {
                    md.push_str(&format!("**Default:** `{}`\n", default));
                }
                md.push_str(&format!("**Required:** {}\n\n", env_var.required));
            }
        }
        
        if !config.config_files.is_empty() {
            md.push_str("### Configuration Files\n\n");
            for config_file in &config.config_files {
                md.push_str(&format!("#### `{}`\n", config_file.name));
                md.push_str(&format!("{}\n", config_file.description));
                if let Some(ref example) = config_file.example {
                    md.push_str(example);
                    md.push_str("\n");
                }
                md.push_str("\n");
            }
        }
    }
    
    // Development
    md.push_str("## Development\n\n");
    md.push_str("### Setup\n\n");
    for step in &content.development.setup {
        md.push_str(&format!("- {}\n", step));
    }
    md.push_str("\n");
    
    md.push_str("### Testing\n\n");
    md.push_str(&content.development.testing);
    md.push_str("\n\n");
    
    md.push_str("### Building\n\n");
    md.push_str(&content.development.building);
    md.push_str("\n\n");
    
    if let Some(ref code_style) = content.development.code_style {
        md.push_str("### Code Style\n\n");
        md.push_str(code_style);
        md.push_str("\n\n");
    }
    
    // Contributing
    md.push_str("## Contributing\n\n");
    md.push_str("### Guidelines\n\n");
    for guideline in &content.contributing.guidelines {
        md.push_str(&format!("- {}\n", guideline));
    }
    md.push_str("\n");
    
    md.push_str("### Workflow\n\n");
    for step in &content.contributing.workflow {
        md.push_str(&format!("- {}\n", step));
    }
    md.push_str("\n");
    
    if let Some(ref coc) = content.contributing.code_of_conduct {
        md.push_str(&format!("### Code of Conduct\n\n{}\n\n", coc));
    }
    
    // Context Management
    if let Some(ref context) = content.context {
        md.push_str("## Context Management\n\n");
        md.push_str(&context.overview);
        md.push_str("\n\n");
        
        md.push_str("### Key Concepts\n\n");
        for concept in &context.concepts {
            md.push_str(&format!("- {}\n", concept));
        }
        md.push_str("\n");
        
        md.push_str("### Rhema Integration\n\n");
        md.push_str(&context.rhema_integration);
        md.push_str("\n\n");
        
        md.push_str("### Context Queries\n\n");
        for query in &context.context_queries {
            md.push_str(&format!("- {}\n", query));
        }
        md.push_str("\n");
    }
    
    // Custom sections
    for (section_name, section_content) in &content.custom_sections {
        md.push_str(&format!("## {}\n\n", section_name));
        md.push_str(section_content);
        md.push_str("\n\n");
    }
    
    // License
    md.push_str("## License\n\n");
    md.push_str(&format!("This project is licensed under the {} License", content.license.license_type));
    if let Some(ref copyright) = content.license.copyright {
        md.push_str(&format!(" - {}", copyright));
    }
    md.push_str(".\n");
    
    Ok(md)
} 