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
use rhema_core::schema::{
    ConceptDefinition, CqlExample, IntegrationGuide, PatternDefinition, ProtocolInfo,
    TroubleshootingItem,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
// use std::path::PathBuf;

/// Export context data in various formats
pub fn run(
    rhema: &Rhema,
    format: &str,
    output_file: Option<&str>,
    scope_filter: Option<&str>,
    include_protocol: bool,
    include_knowledge: bool,
    include_todos: bool,
    include_decisions: bool,
    include_patterns: bool,
    include_conventions: bool,
    summarize: bool,
    ai_agent_format: bool,
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

    // Extract RhemaScope from Scope objects
    let rhema_scopes: Vec<RhemaScope> = filtered_scopes
        .iter()
        .map(|scope| scope.definition.clone())
        .collect();

    // Collect context data
    let context_data = collect_context_data(
        rhema,
        &rhema_scopes,
        include_protocol,
        include_knowledge,
        include_todos,
        include_decisions,
        include_patterns,
        include_conventions,
        summarize,
    )?;

    // Format and output the data
    let output = format_context_data(&context_data, format, ai_agent_format)?;

    // Write to file or print to console
    if let Some(file_path) = output_file {
        fs::write(file_path, output)?;
        println!("{}", "✓ Context exported successfully!".green());
        println!("  Output: {}", file_path.yellow());
    } else {
        println!("{}", output);
    }

    Ok(())
}

/// Context data structure for export
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextExport {
    /// Export metadata
    pub metadata: ExportMetadata,

    /// Scopes data
    pub scopes: Vec<ScopeExport>,

    /// Protocol information
    pub protocol_info: Option<ProtocolExport>,

    /// Knowledge base
    pub knowledge: Option<KnowledgeExport>,

    /// Todo items
    pub todos: Option<TodosExport>,

    /// Decisions
    pub decisions: Option<DecisionsExport>,

    /// Patterns
    pub patterns: Option<PatternsExport>,

    /// Conventions
    pub conventions: Option<ConventionsExport>,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExportMetadata {
    /// Export timestamp
    pub exported_at: String,

    /// Export format
    pub format: String,

    /// Scope count
    pub scope_count: usize,

    /// Export options
    pub options: HashMap<String, bool>,
}

/// Scope export data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScopeExport {
    /// Scope name
    pub name: String,

    /// Scope type
    pub scope_type: String,

    /// Description
    pub description: Option<String>,

    /// Version
    pub version: String,

    /// Dependencies
    pub dependencies: Option<Vec<String>>,

    /// Protocol info
    pub protocol_info: Option<ProtocolInfo>,
}

/// Protocol export data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProtocolExport {
    /// Protocol version
    pub version: String,

    /// Description
    pub description: Option<String>,

    /// Concepts
    pub concepts: Option<Vec<ConceptDefinition>>,

    /// CQL examples
    pub cql_examples: Option<Vec<CqlExample>>,

    /// Patterns
    pub patterns: Option<Vec<PatternDefinition>>,

    /// Integrations
    pub integrations: Option<Vec<IntegrationGuide>>,

    /// Troubleshooting
    pub troubleshooting: Option<Vec<TroubleshootingItem>>,
}

/// Knowledge export data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct KnowledgeExport {
    /// Entry count
    pub entry_count: usize,

    /// Categories
    pub categories: Option<Vec<String>>,

    /// Recent entries (summarized)
    pub recent_entries: Option<Vec<KnowledgeEntrySummary>>,
}

/// Knowledge entry summary
#[derive(Debug, Clone, Serialize, Deserialize)]
struct KnowledgeEntrySummary {
    /// Entry ID
    pub id: String,

    /// Title
    pub title: String,

    /// Category
    pub category: Option<String>,

    /// Tags
    pub tags: Option<Vec<String>>,

    /// Confidence
    pub confidence: Option<u8>,
}

/// Todos export data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodosExport {
    /// Todo count
    pub todo_count: usize,

    /// By status
    pub by_status: HashMap<String, usize>,

    /// By priority
    pub by_priority: HashMap<String, usize>,

    /// Recent todos
    pub recent_todos: Option<Vec<TodoSummary>>,
}

/// Todo summary
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoSummary {
    /// Todo ID
    pub id: String,

    /// Title
    pub title: String,

    /// Status
    pub status: String,

    /// Priority
    pub priority: String,

    /// Assigned to
    pub assigned_to: Option<String>,
}

/// Decisions export data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DecisionsExport {
    /// Decision count
    pub decision_count: usize,

    /// By status
    pub by_status: HashMap<String, usize>,

    /// Recent decisions
    pub recent_decisions: Option<Vec<DecisionSummary>>,
}

/// Decision summary
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DecisionSummary {
    /// Decision ID
    pub id: String,

    /// Title
    pub title: String,

    /// Status
    pub status: String,

    /// Decision date
    pub decided_at: String,
}

/// Patterns export data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PatternsExport {
    /// Pattern count
    pub pattern_count: usize,

    /// By type
    pub by_type: HashMap<String, usize>,

    /// Recent patterns
    pub recent_patterns: Option<Vec<PatternSummary>>,
}

/// Pattern summary
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PatternSummary {
    /// Pattern ID
    pub id: String,

    /// Name
    pub name: String,

    /// Pattern type
    pub pattern_type: String,

    /// Usage
    pub usage: String,
}

/// Conventions export data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConventionsExport {
    /// Convention count
    pub convention_count: usize,

    /// By type
    pub by_type: HashMap<String, usize>,

    /// By enforcement level
    pub by_enforcement: HashMap<String, usize>,

    /// Recent conventions
    pub recent_conventions: Option<Vec<ConventionSummary>>,
}

/// Convention summary
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConventionSummary {
    /// Convention ID
    pub id: String,

    /// Name
    pub name: String,

    /// Convention type
    pub convention_type: String,

    /// Enforcement level
    pub enforcement: String,
}

/// Collect context data from scopes
fn collect_context_data(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    include_protocol: bool,
    include_knowledge: bool,
    include_todos: bool,
    include_decisions: bool,
    include_patterns: bool,
    include_conventions: bool,
    summarize: bool,
) -> RhemaResult<ContextExport> {
    let mut scope_exports = Vec::new();
    let mut protocol_info = None;

    for scope in scopes {
        let scope_export = ScopeExport {
            name: scope.name.clone(),
            scope_type: scope.scope_type.clone(),
            description: scope.description.clone(),
            version: scope.version.clone(),
            dependencies: scope
                .dependencies
                .as_ref()
                .map(|deps| deps.iter().map(|d| d.path.clone()).collect()),
            protocol_info: scope.protocol_info.clone(),
        };

        scope_exports.push(scope_export);

        // Collect protocol info from first scope that has it
        if protocol_info.is_none() && include_protocol {
            if let Some(ref proto) = scope.protocol_info {
                protocol_info = Some(ProtocolExport {
                    version: proto.version.clone(),
                    description: proto.description.clone(),
                    concepts: proto.concepts.clone(),
                    cql_examples: proto.cql_examples.clone(),
                    patterns: proto.patterns.clone(),
                    integrations: proto.integrations.clone(),
                    troubleshooting: proto.troubleshooting.clone(),
                });
            }
        }
    }

    // Collect other data types if requested
    let knowledge = if include_knowledge {
        collect_knowledge_data(rhema, scopes, summarize)?
    } else {
        None
    };

    let todos = if include_todos {
        collect_todos_data(rhema, scopes, summarize)?
    } else {
        None
    };

    let decisions = if include_decisions {
        collect_decisions_data(rhema, scopes, summarize)?
    } else {
        None
    };

    let patterns = if include_patterns {
        collect_patterns_data(rhema, scopes, summarize)?
    } else {
        None
    };

    let conventions = if include_conventions {
        collect_conventions_data(rhema, scopes, summarize)?
    } else {
        None
    };

    let metadata = ExportMetadata {
        exported_at: chrono::Utc::now().to_rfc3339(),
        format: "context_export".to_string(),
        scope_count: scopes.len(),
        options: {
            let mut opts = HashMap::new();
            opts.insert("include_protocol".to_string(), include_protocol);
            opts.insert("include_knowledge".to_string(), include_knowledge);
            opts.insert("include_todos".to_string(), include_todos);
            opts.insert("include_decisions".to_string(), include_decisions);
            opts.insert("include_patterns".to_string(), include_patterns);
            opts.insert("include_conventions".to_string(), include_conventions);
            opts.insert("summarize".to_string(), summarize);
            opts
        },
    };

    Ok(ContextExport {
        metadata,
        scopes: scope_exports,
        protocol_info,
        knowledge,
        todos,
        decisions,
        patterns,
        conventions,
    })
}

/// Collect knowledge data
fn collect_knowledge_data(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    summarize: bool,
) -> RhemaResult<Option<KnowledgeExport>> {
    let mut all_entries = Vec::new();
    let mut all_categories = std::collections::HashSet::new();

    for scope in scopes {
        if let Ok(knowledge) = rhema.load_knowledge(&scope.name) {
            for entry in &knowledge.entries {
                all_entries.push(KnowledgeEntrySummary {
                    id: entry.id.clone(),
                    title: entry.title.clone(),
                    category: entry.category.clone(),
                    tags: entry.tags.clone(),
                    confidence: entry.confidence,
                });

                if let Some(ref category) = entry.category {
                    all_categories.insert(category.clone());
                }
            }
        }
    }

    if all_entries.is_empty() {
        return Ok(None);
    }

    // Sort by creation date (most recent first)
    all_entries.sort_by(|a, b| {
        // For now, just use ID for sorting since we don't have timestamps in summary
        a.id.cmp(&b.id)
    });

    let entry_count = all_entries.len();

    let recent_entries = if summarize && entry_count > 10 {
        Some(all_entries.into_iter().take(10).collect())
    } else if summarize {
        Some(all_entries)
    } else {
        None
    };

    Ok(Some(KnowledgeExport {
        entry_count,
        categories: Some(all_categories.into_iter().collect()),
        recent_entries,
    }))
}

/// Collect todos data
fn collect_todos_data(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    summarize: bool,
) -> RhemaResult<Option<TodosExport>> {
    let mut all_todos = Vec::new();
    let mut by_status = HashMap::new();
    let mut by_priority = HashMap::new();

    for scope in scopes {
        if let Ok(todos) = rhema.load_todos(&scope.name) {
            for todo in &todos.todos {
                let status = format!("{:?}", todo.status);
                let priority = format!("{:?}", todo.priority);

                *by_status.entry(status.clone()).or_insert(0) += 1;
                *by_priority.entry(priority.clone()).or_insert(0) += 1;

                all_todos.push(TodoSummary {
                    id: todo.id.clone(),
                    title: todo.title.clone(),
                    status,
                    priority,
                    assigned_to: todo.assigned_to.clone(),
                });
            }
        }
    }

    if all_todos.is_empty() {
        return Ok(None);
    }

    // Sort by creation date (most recent first)
    all_todos.sort_by(|a, b| a.id.cmp(&b.id));

    let todo_count = all_todos.len();

    let recent_todos = if summarize && todo_count > 10 {
        Some(all_todos.into_iter().take(10).collect())
    } else if summarize {
        Some(all_todos)
    } else {
        None
    };

    Ok(Some(TodosExport {
        todo_count,
        by_status,
        by_priority,
        recent_todos,
    }))
}

/// Collect decisions data
fn collect_decisions_data(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    summarize: bool,
) -> RhemaResult<Option<DecisionsExport>> {
    let mut all_decisions = Vec::new();
    let mut by_status = HashMap::new();

    for scope in scopes {
        if let Ok(decisions) = rhema.load_decisions(&scope.name) {
            for decision in &decisions.decisions {
                let status = format!("{:?}", decision.status);
                *by_status.entry(status.clone()).or_insert(0) += 1;

                all_decisions.push(DecisionSummary {
                    id: decision.id.clone(),
                    title: decision.title.clone(),
                    status,
                    decided_at: decision.decided_at.to_rfc3339(),
                });
            }
        }
    }

    if all_decisions.is_empty() {
        return Ok(None);
    }

    // Sort by decision date (most recent first)
    all_decisions.sort_by(|a, b| b.decided_at.cmp(&a.decided_at));

    let decision_count = all_decisions.len();

    let recent_decisions = if summarize && decision_count > 10 {
        Some(all_decisions.into_iter().take(10).collect())
    } else if summarize {
        Some(all_decisions)
    } else {
        None
    };

    Ok(Some(DecisionsExport {
        decision_count,
        by_status,
        recent_decisions,
    }))
}

/// Collect patterns data
fn collect_patterns_data(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    summarize: bool,
) -> RhemaResult<Option<PatternsExport>> {
    let mut all_patterns = Vec::new();
    let mut by_type = HashMap::new();

    for scope in scopes {
        if let Ok(patterns) = rhema.load_patterns(&scope.name) {
            for pattern in &patterns.patterns {
                *by_type.entry(pattern.pattern_type.clone()).or_insert(0) += 1;

                all_patterns.push(PatternSummary {
                    id: pattern.id.clone(),
                    name: pattern.name.clone(),
                    pattern_type: pattern.pattern_type.clone(),
                    usage: format!("{:?}", pattern.usage),
                });
            }
        }
    }

    if all_patterns.is_empty() {
        return Ok(None);
    }

    // Sort by creation date (most recent first)
    all_patterns.sort_by(|a, b| a.id.cmp(&b.id));

    let pattern_count = all_patterns.len();

    let recent_patterns = if summarize && pattern_count > 10 {
        Some(all_patterns.into_iter().take(10).collect())
    } else if summarize {
        Some(all_patterns)
    } else {
        None
    };

    Ok(Some(PatternsExport {
        pattern_count,
        by_type,
        recent_patterns,
    }))
}

/// Collect conventions data
fn collect_conventions_data(
    rhema: &Rhema,
    scopes: &[RhemaScope],
    summarize: bool,
) -> RhemaResult<Option<ConventionsExport>> {
    let mut all_conventions = Vec::new();
    let mut by_type = HashMap::new();
    let mut by_enforcement = HashMap::new();

    for scope in scopes {
        if let Ok(conventions) = rhema.load_conventions(&scope.name) {
            for convention in &conventions.conventions {
                *by_type
                    .entry(convention.convention_type.clone())
                    .or_insert(0) += 1;
                *by_enforcement
                    .entry(format!("{:?}", convention.enforcement))
                    .or_insert(0) += 1;

                all_conventions.push(ConventionSummary {
                    id: convention.id.clone(),
                    name: convention.name.clone(),
                    convention_type: convention.convention_type.clone(),
                    enforcement: format!("{:?}", convention.enforcement),
                });
            }
        }
    }

    if all_conventions.is_empty() {
        return Ok(None);
    }

    // Sort by creation date (most recent first)
    all_conventions.sort_by(|a, b| a.id.cmp(&b.id));

    let convention_count = all_conventions.len();

    let recent_conventions = if summarize && convention_count > 10 {
        Some(all_conventions.into_iter().take(10).collect())
    } else if summarize {
        Some(all_conventions)
    } else {
        None
    };

    Ok(Some(ConventionsExport {
        convention_count,
        by_type,
        by_enforcement,
        recent_conventions,
    }))
}

/// Format context data for output
fn format_context_data(
    data: &ContextExport,
    format: &str,
    ai_agent_format: bool,
) -> RhemaResult<String> {
    match format.to_lowercase().as_str() {
        "json" => {
            if ai_agent_format {
                format_ai_agent_json(data)
            } else {
                serde_json::to_string_pretty(data).map_err(|e| crate::RhemaError::JsonError(e))
            }
        }
        "yaml" => {
            if ai_agent_format {
                format_ai_agent_yaml(data)
            } else {
                serde_yaml::to_string(data).map_err(|e| crate::RhemaError::YamlError(e))
            }
        }
        "markdown" => format_markdown(data, ai_agent_format),
        "text" => format_text(data, ai_agent_format),
        _ => Err(crate::RhemaError::ConfigError(format!(
            "Unsupported format: {}",
            format
        ))),
    }
}

/// Format for AI agent JSON
fn format_ai_agent_json(data: &ContextExport) -> RhemaResult<String> {
    let ai_format = serde_json::json!({
        "context_type": "rhema_export",
        "export_metadata": {
            "timestamp": data.metadata.exported_at,
            "scope_count": data.metadata.scope_count,
            "format_version": "1.0"
        },
        "scopes": data.scopes.iter().map(|s| serde_json::json!({
            "name": s.name,
            "type": s.scope_type,
            "description": s.description,
            "version": s.version
        })).collect::<Vec<_>>(),
        "protocol_info": data.protocol_info.as_ref().map(|p| serde_json::json!({
            "version": p.version,
            "description": p.description,
            "concepts": p.concepts.as_ref().map(|c| c.iter().map(|concept| serde_json::json!({
                "name": concept.name,
                "description": concept.description
            })).collect::<Vec<_>>()),
            "cql_examples": p.cql_examples.as_ref().map(|e| e.iter().map(|ex| serde_json::json!({
                "name": ex.name,
                "query": ex.query,
                "description": ex.description
            })).collect::<Vec<_>>())
        })),
        "summary": {
            "knowledge_entries": data.knowledge.as_ref().map(|k| k.entry_count),
            "todos": data.todos.as_ref().map(|t| t.todo_count),
            "decisions": data.decisions.as_ref().map(|d| d.decision_count),
            "patterns": data.patterns.as_ref().map(|p| p.pattern_count),
            "conventions": data.conventions.as_ref().map(|c| c.convention_count)
        }
    });

    serde_json::to_string_pretty(&ai_format).map_err(|e| crate::RhemaError::JsonError(e))
}

/// Format for AI agent YAML
fn format_ai_agent_yaml(data: &ContextExport) -> RhemaResult<String> {
    let ai_format = serde_yaml::to_value(serde_json::json!({
        "context_type": "rhema_export",
        "export_metadata": {
            "timestamp": data.metadata.exported_at,
            "scope_count": data.metadata.scope_count,
            "format_version": "1.0"
        },
        "scopes": data.scopes.iter().map(|s| serde_json::json!({
            "name": s.name,
            "type": s.scope_type,
            "description": s.description,
            "version": s.version
        })).collect::<Vec<_>>(),
        "protocol_info": data.protocol_info.as_ref().map(|p| serde_json::json!({
            "version": p.version,
            "description": p.description,
            "concepts": p.concepts.as_ref().map(|c| c.iter().map(|concept| serde_json::json!({
                "name": concept.name,
                "description": concept.description
            })).collect::<Vec<_>>()),
            "cql_examples": p.cql_examples.as_ref().map(|e| e.iter().map(|ex| serde_json::json!({
                "name": ex.name,
                "query": ex.query,
                "description": ex.description
            })).collect::<Vec<_>>())
        })),
        "summary": {
            "knowledge_entries": data.knowledge.as_ref().map(|k| k.entry_count),
            "todos": data.todos.as_ref().map(|t| t.todo_count),
            "decisions": data.decisions.as_ref().map(|d| d.decision_count),
            "patterns": data.patterns.as_ref().map(|p| p.pattern_count),
            "conventions": data.conventions.as_ref().map(|c| c.convention_count)
        }
    }))?;

    serde_yaml::to_string(&ai_format).map_err(|e| crate::RhemaError::YamlError(e))
}

/// Format as markdown
fn format_markdown(data: &ContextExport, ai_agent_format: bool) -> RhemaResult<String> {
    let mut md = String::new();

    if ai_agent_format {
        md.push_str("# Rhema Context Export (AI Agent Format)\n\n");
        md.push_str(&format!("**Export Date:** {}\n", data.metadata.exported_at));
        md.push_str(&format!(
            "**Scope Count:** {}\n\n",
            data.metadata.scope_count
        ));

        md.push_str("## Scopes\n\n");
        for scope in &data.scopes {
            md.push_str(&format!("### {}\n", scope.name));
            md.push_str(&format!("- **Type:** {}\n", scope.scope_type));
            md.push_str(&format!("- **Version:** {}\n", scope.version));
            if let Some(ref desc) = scope.description {
                md.push_str(&format!("- **Description:** {}\n", desc));
            }
            md.push_str("\n");
        }

        if let Some(ref protocol) = data.protocol_info {
            md.push_str("## Protocol Information\n\n");
            md.push_str(&format!("**Version:** {}\n\n", protocol.version));

            if let Some(ref desc) = protocol.description {
                md.push_str(&format!("**Description:** {}\n\n", desc));
            }

            if let Some(ref concepts) = protocol.concepts {
                md.push_str("### Key Concepts\n\n");
                for concept in concepts {
                    md.push_str(&format!("#### {}\n", concept.name));
                    md.push_str(&format!("{}\n\n", concept.description));
                }
            }

            if let Some(ref examples) = protocol.cql_examples {
                md.push_str("### CQL Examples\n\n");
                for example in examples {
                    md.push_str(&format!("#### {}\n", example.name));
                    md.push_str(&format!("**Query:** `{}`\n", example.query));
                    md.push_str(&format!("**Description:** {}\n\n", example.description));
                }
            }
        }
    } else {
        md.push_str("# Rhema Context Export\n\n");
        md.push_str(&format!("**Export Date:** {}\n", data.metadata.exported_at));
        md.push_str(&format!("**Scope Count:** {}\n", data.metadata.scope_count));
        md.push_str(&format!("**Format:** {}\n\n", data.metadata.format));

        md.push_str("## Export Options\n\n");
        for (key, value) in &data.metadata.options {
            md.push_str(&format!("- **{}:** {}\n", key, value));
        }
        md.push_str("\n");

        md.push_str("## Scopes\n\n");
        for scope in &data.scopes {
            md.push_str(&format!("### {}\n", scope.name));
            md.push_str(&format!("- **Type:** {}\n", scope.scope_type));
            md.push_str(&format!("- **Version:** {}\n", scope.version));
            if let Some(ref desc) = scope.description {
                md.push_str(&format!("- **Description:** {}\n", desc));
            }
            if let Some(ref deps) = scope.dependencies {
                md.push_str("- **Dependencies:**\n");
                for dep in deps {
                    md.push_str(&format!("  - {}\n", dep));
                }
            }
            md.push_str("\n");
        }

        // Add other sections based on what's included
        if let Some(ref knowledge) = data.knowledge {
            md.push_str("## Knowledge Base\n\n");
            md.push_str(&format!("**Total Entries:** {}\n", knowledge.entry_count));
            if let Some(ref categories) = knowledge.categories {
                md.push_str(&format!("**Categories:** {}\n", categories.join(", ")));
            }
            md.push_str("\n");
        }

        if let Some(ref todos) = data.todos {
            md.push_str("## Todo Items\n\n");
            md.push_str(&format!("**Total Todos:** {}\n", todos.todo_count));
            md.push_str("**By Status:**\n");
            for (status, count) in &todos.by_status {
                md.push_str(&format!("- {}: {}\n", status, count));
            }
            md.push_str("**By Priority:**\n");
            for (priority, count) in &todos.by_priority {
                md.push_str(&format!("- {}: {}\n", priority, count));
            }
            md.push_str("\n");
        }

        if let Some(ref decisions) = data.decisions {
            md.push_str("## Decisions\n\n");
            md.push_str(&format!(
                "**Total Decisions:** {}\n",
                decisions.decision_count
            ));
            md.push_str("**By Status:**\n");
            for (status, count) in &decisions.by_status {
                md.push_str(&format!("- {}: {}\n", status, count));
            }
            md.push_str("\n");
        }

        if let Some(ref patterns) = data.patterns {
            md.push_str("## Patterns\n\n");
            md.push_str(&format!("**Total Patterns:** {}\n", patterns.pattern_count));
            md.push_str("**By Type:**\n");
            for (pattern_type, count) in &patterns.by_type {
                md.push_str(&format!("- {}: {}\n", pattern_type, count));
            }
            md.push_str("\n");
        }

        if let Some(ref conventions) = data.conventions {
            md.push_str("## Conventions\n\n");
            md.push_str(&format!(
                "**Total Conventions:** {}\n",
                conventions.convention_count
            ));
            md.push_str("**By Type:**\n");
            for (convention_type, count) in &conventions.by_type {
                md.push_str(&format!("- {}: {}\n", convention_type, count));
            }
            md.push_str("**By Enforcement Level:**\n");
            for (enforcement, count) in &conventions.by_enforcement {
                md.push_str(&format!("- {}: {}\n", enforcement, count));
            }
            md.push_str("\n");
        }
    }

    Ok(md)
}

/// Format as plain text
fn format_text(data: &ContextExport, ai_agent_format: bool) -> RhemaResult<String> {
    let mut text = String::new();

    if ai_agent_format {
        text.push_str("Rhema CONTEXT EXPORT (AI AGENT FORMAT)\n");
        text.push_str("=====================================\n\n");
        text.push_str(&format!("Export Date: {}\n", data.metadata.exported_at));
        text.push_str(&format!("Scope Count: {}\n\n", data.metadata.scope_count));

        text.push_str("SCOPES:\n");
        text.push_str("-------\n");
        for scope in &data.scopes {
            text.push_str(&format!("Name: {}\n", scope.name));
            text.push_str(&format!("Type: {}\n", scope.scope_type));
            text.push_str(&format!("Version: {}\n", scope.version));
            if let Some(ref desc) = scope.description {
                text.push_str(&format!("Description: {}\n", desc));
            }
            text.push_str("\n");
        }

        if let Some(ref protocol) = data.protocol_info {
            text.push_str("PROTOCOL INFORMATION:\n");
            text.push_str("---------------------\n");
            text.push_str(&format!("Version: {}\n", protocol.version));
            if let Some(ref desc) = protocol.description {
                text.push_str(&format!("Description: {}\n", desc));
            }
            text.push_str("\n");

            if let Some(ref concepts) = protocol.concepts {
                text.push_str("KEY CONCEPTS:\n");
                for concept in concepts {
                    text.push_str(&format!("- {}: {}\n", concept.name, concept.description));
                }
                text.push_str("\n");
            }

            if let Some(ref examples) = protocol.cql_examples {
                text.push_str("CQL EXAMPLES:\n");
                for example in examples {
                    text.push_str(&format!("- {}: {}\n", example.name, example.description));
                    text.push_str(&format!("  Query: {}\n", example.query));
                }
                text.push_str("\n");
            }
        }
    } else {
        text.push_str("Rhema CONTEXT EXPORT\n");
        text.push_str("===================\n\n");
        text.push_str(&format!("Export Date: {}\n", data.metadata.exported_at));
        text.push_str(&format!("Scope Count: {}\n", data.metadata.scope_count));
        text.push_str(&format!("Format: {}\n\n", data.metadata.format));

        text.push_str("EXPORT OPTIONS:\n");
        for (key, value) in &data.metadata.options {
            text.push_str(&format!("- {}: {}\n", key, value));
        }
        text.push_str("\n");

        text.push_str("SCOPES:\n");
        text.push_str("-------\n");
        for scope in &data.scopes {
            text.push_str(&format!("Name: {}\n", scope.name));
            text.push_str(&format!("Type: {}\n", scope.scope_type));
            text.push_str(&format!("Version: {}\n", scope.version));
            if let Some(ref desc) = scope.description {
                text.push_str(&format!("Description: {}\n", desc));
            }
            if let Some(ref deps) = scope.dependencies {
                text.push_str("Dependencies:\n");
                for dep in deps {
                    text.push_str(&format!("  - {}\n", dep));
                }
            }
            text.push_str("\n");
        }

        // Add summary information
        if let Some(ref knowledge) = data.knowledge {
            text.push_str(&format!("Knowledge Entries: {}\n", knowledge.entry_count));
        }
        if let Some(ref todos) = data.todos {
            text.push_str(&format!("Todo Items: {}\n", todos.todo_count));
        }
        if let Some(ref decisions) = data.decisions {
            text.push_str(&format!("Decisions: {}\n", decisions.decision_count));
        }
        if let Some(ref patterns) = data.patterns {
            text.push_str(&format!("Patterns: {}\n", patterns.pattern_count));
        }
        if let Some(ref conventions) = data.conventions {
            text.push_str(&format!("Conventions: {}\n", conventions.convention_count));
        }
    }

    Ok(text)
}
