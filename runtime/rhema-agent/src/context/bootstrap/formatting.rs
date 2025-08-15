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

use super::types::*;

/// Format bootstrap markdown
pub fn format_bootstrap_markdown(content: &BootstrapContent) -> String {
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
pub fn format_bootstrap_text(content: &BootstrapContent) -> String {
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
