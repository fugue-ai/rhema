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
use std::collections::HashMap;
use std::fs;

use super::data_structures::*;
use super::formatting::format_context_data;

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
