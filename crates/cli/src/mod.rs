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

use clap::Args;
use rhema_core::schema::{Prompts, Workflows, TemplateLibrary, TemplateExport};
use std::fs;
use std::path::Path;

pub mod init;
pub mod scopes;
pub mod show;
pub mod query;
pub mod search;
pub mod validate;
pub mod migrate;
pub mod schema;
pub mod health;
pub mod stats;
pub mod todo;
pub mod insight;
pub mod pattern;
pub mod decision;
pub mod dependencies;
pub mod impact;
pub mod sync;
pub mod git;
pub mod integrations;
pub mod export_context;
pub mod primer;
pub mod generate_readme;
pub mod bootstrap_context;
pub mod daemon;
pub mod interactive;
pub mod interactive_advanced;
pub mod interactive_enhanced;
pub mod interactive_parser;
pub mod interactive_builder;
pub mod batch;
pub mod performance;
pub mod config;
pub mod prompt;
pub mod context_rules;
pub mod workflow;
pub mod template;
pub mod commands;
pub mod locomo;

/// Common arguments for commands that need a scope
#[derive(Args)]
pub struct ScopeArgs {
    /// Scope path (relative to repository root)
    #[arg(value_name = "SCOPE")]
    scope: Option<String>,
}

/// Common arguments for commands that need a file
#[derive(Args)]
pub struct FileArgs {
    /// File name (without .yaml extension)
    #[arg(value_name = "FILE")]
    file: String,
}

/// Common arguments for commands that need a query
#[derive(Args)]
pub struct QueryArgs {
    /// CQL query string
    #[arg(value_name = "QUERY")]
    query: String,
}

/// Common arguments for search commands
#[derive(Args)]
pub struct SearchArgs {
    /// Search term
    #[arg(value_name = "TERM")]
    term: String,
    
    /// Search in specific file type
    #[arg(long, value_name = "FILE")]
    in_file: Option<String>,
}

/// Common arguments for todo commands
#[derive(Args)]
pub struct TodoArgs {
    /// Todo title
    #[arg(value_name = "TITLE")]
    title: String,
    
    /// Priority level
    #[arg(long, value_enum, default_value = "medium")]
    priority: rhema_core::schema::Priority,
    
    /// Assignee
    #[arg(long, value_name = "ASSIGNEE")]
    assignee: Option<String>,
    
    /// Due date (ISO format)
    #[arg(long, value_name = "DATE")]
    due_date: Option<String>,
}

/// Common arguments for insight commands
#[derive(Args)]
pub struct InsightArgs {
    /// Insight content
    #[arg(value_name = "INSIGHT")]
    insight: String,
    
    /// Confidence level (1-10)
    #[arg(long, value_name = "LEVEL")]
    confidence: Option<u8>,
    
    /// Category
    #[arg(long, value_name = "CATEGORY")]
    category: Option<String>,
    
    /// Tags (comma-separated)
    #[arg(long, value_name = "TAGS")]
    tags: Option<String>,
}

/// Common arguments for export context commands
#[derive(Args)]
pub struct ExportContextArgs {
    /// Output format (json, yaml, markdown, text)
    #[arg(long, value_name = "FORMAT", default_value = "json")]
    format: String,
    
    /// Output file (optional, prints to console if not specified)
    #[arg(long, value_name = "FILE")]
    output_file: Option<String>,
    
    /// Scope filter
    #[arg(long, value_name = "SCOPE")]
    scope_filter: Option<String>,
    
    /// Include protocol information
    #[arg(long)]
    include_protocol: bool,
    
    /// Include knowledge base
    #[arg(long)]
    include_knowledge: bool,
    
    /// Include todo items
    #[arg(long)]
    include_todos: bool,
    
    /// Include decisions
    #[arg(long)]
    include_decisions: bool,
    
    /// Include patterns
    #[arg(long)]
    include_patterns: bool,
    
    /// Include conventions
    #[arg(long)]
    include_conventions: bool,
    
    /// Summarize data
    #[arg(long)]
    summarize: bool,
    
    /// AI agent format
    #[arg(long)]
    ai_agent_format: bool,
}

/// Common arguments for primer commands
#[derive(Args)]
pub struct PrimerArgs {
    /// Scope name
    #[arg(long, value_name = "SCOPE")]
    scope_name: Option<String>,
    
    /// Output directory
    #[arg(long, value_name = "DIR")]
    output_dir: Option<String>,
    
    /// Template type
    #[arg(long, value_name = "TEMPLATE")]
    template_type: Option<String>,
    
    /// Include examples
    #[arg(long)]
    include_examples: bool,
    
    /// Validate primer
    #[arg(long)]
    validate: bool,
}

/// Common arguments for generate readme commands
#[derive(Args)]
pub struct GenerateReadmeArgs {
    /// Scope name
    #[arg(long, value_name = "SCOPE")]
    scope_name: Option<String>,
    
    /// Output file
    #[arg(long, value_name = "FILE")]
    output_file: Option<String>,
    
    /// Template
    #[arg(long, value_name = "TEMPLATE")]
    template: Option<String>,
    
    /// Include context
    #[arg(long)]
    include_context: bool,
    
    /// SEO optimized
    #[arg(long)]
    seo_optimized: bool,
    
    /// Custom sections (comma-separated)
    #[arg(long, value_name = "SECTIONS")]
    custom_sections: Option<String>,
}

/// Common arguments for bootstrap context commands
#[derive(Args)]
pub struct BootstrapContextArgs {
    /// Use case (code_review, feature_development, debugging, documentation, onboarding)
    #[arg(long, value_name = "USE_CASE", default_value = "code_review")]
    use_case: String,
    
    /// Output format (json, yaml, markdown, text, all)
    #[arg(long, value_name = "FORMAT", default_value = "json")]
    output_format: String,
    
    /// Output directory
    #[arg(long, value_name = "DIR")]
    output_dir: Option<String>,
    
    /// Scope filter
    #[arg(long, value_name = "SCOPE")]
    scope_filter: Option<String>,
    
    /// Include all data
    #[arg(long)]
    include_all: bool,
    
    /// Optimize for AI
    #[arg(long)]
    optimize_for_ai: bool,
    
    /// Create primer
    #[arg(long)]
    create_primer: bool,
    
    /// Create README
    #[arg(long)]
    create_readme: bool,
}

/// Common arguments for pattern commands
#[derive(Args)]
pub struct PatternArgs {
    /// Pattern name
    #[arg(value_name = "NAME")]
    name: String,
    
    /// Pattern description
    #[arg(long, value_name = "DESCRIPTION")]
    description: String,
    
    /// Pattern type
    #[arg(long, value_name = "TYPE")]
    pattern_type: String,
    
    /// Usage context
    #[arg(long, value_enum, default_value = "recommended")]
    usage: rhema_core::schema::PatternUsage,
    
    /// Effectiveness rating (1-10)
    #[arg(long, value_name = "RATING")]
    effectiveness: Option<u8>,
}

/// Common arguments for decision commands
#[derive(Args)]
pub struct DecisionArgs {
    /// Decision title
    #[arg(value_name = "TITLE")]
    title: String,
    
    /// Decision description
    #[arg(long, value_name = "DESCRIPTION")]
    description: String,
    
    /// Decision status
    #[arg(long, value_enum, default_value = "proposed")]
    status: rhema_core::schema::DecisionStatus,
    
    /// Decision context
    #[arg(long, value_name = "CONTEXT")]
    context: Option<String>,
    
    /// Decision makers (comma-separated)
    #[arg(long, value_name = "MAKERS")]
    makers: Option<String>,
} 

/// Load prompts.yaml from the given path
pub fn load_prompts<P: AsRef<Path>>(path: P) -> crate::RhemaResult<Prompts> {
    let content = fs::read_to_string(path)?;
    let prompts: Prompts = serde_yaml::from_str(&content)?;
    Ok(prompts)
}

/// Save prompts.yaml to the given path
pub fn save_prompts<P: AsRef<Path>>(path: P, prompts: &Prompts) -> crate::RhemaResult<()> {
    let content = serde_yaml::to_string(prompts)?;
    fs::write(path, content)?;
    Ok(())
}

/// Load workflows.yaml from the given path
pub fn load_workflows<P: AsRef<Path>>(path: P) -> crate::RhemaResult<Workflows> {
    let content = fs::read_to_string(path)?;
    let workflows: Workflows = serde_yaml::from_str(&content)?;
    Ok(workflows)
}

/// Save workflows.yaml to the given path
pub fn save_workflows<P: AsRef<Path>>(path: P, workflows: &Workflows) -> crate::RhemaResult<()> {
    let content = serde_yaml::to_string(workflows)?;
    fs::write(path, content)?;
    Ok(())
}

/// Load template library from the given path
pub fn load_template_library<P: AsRef<Path>>(path: P) -> crate::RhemaResult<TemplateLibrary> {
    let content = fs::read_to_string(path)?;
    let library: TemplateLibrary = serde_yaml::from_str(&content)?;
    Ok(library)
}

/// Save template library to the given path
pub fn save_template_library<P: AsRef<Path>>(path: P, library: &TemplateLibrary) -> crate::RhemaResult<()> {
    let content = serde_yaml::to_string(library)?;
    fs::write(path, content)?;
    Ok(())
}

/// Load template export from the given path
pub fn load_template_export<P: AsRef<Path>>(path: P) -> crate::RhemaResult<TemplateExport> {
    let content = fs::read_to_string(path)?;
    let export: TemplateExport = serde_yaml::from_str(&content)?;
    Ok(export)
}

/// Save template export to the given path
pub fn save_template_export<P: AsRef<Path>>(path: P, export: &TemplateExport) -> crate::RhemaResult<()> {
    let content = serde_yaml::to_string(export)?;
    fs::write(path, content)?;
    Ok(())
} 