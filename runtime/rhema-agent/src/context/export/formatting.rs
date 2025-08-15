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

use serde_json;
use super::data_structures::*;

/// Format context data for output
pub fn format_context_data(
    data: &ContextExport,
    format: &str,
    ai_agent_format: bool,
) -> crate::RhemaResult<String> {
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
fn format_ai_agent_json(data: &ContextExport) -> crate::RhemaResult<String> {
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
fn format_ai_agent_yaml(data: &ContextExport) -> crate::RhemaResult<String> {
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
fn format_markdown(data: &ContextExport, ai_agent_format: bool) -> crate::RhemaResult<String> {
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
fn format_text(data: &ContextExport, ai_agent_format: bool) -> crate::RhemaResult<String> {
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
