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

use crate::{Rhema, RhemaResult};
// InsightSubcommands will be defined in this module
use crate::file_ops;
use crate::scope::find_nearest_scope;

// Use the InsightSubcommands from commands module
use crate::commands::InsightSubcommands;
use colored::*;

pub fn run(rhema: &Rhema, subcommand: &InsightSubcommands) -> RhemaResult<()> {
    // Get the current working directory to find the nearest scope
    let current_dir = std::env::current_dir().map_err(|e| crate::RhemaError::IoError(e))?;

    // Discover all scopes
    let scopes = rhema.discover_scopes()?;

    // Find the nearest scope to the current directory
    let scope = find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
        crate::RhemaError::ConfigError(
            "No Rhema scope found in current directory or parent directories".to_string(),
        )
    })?;

    match subcommand {
        InsightSubcommands::Record {
            title,
            content,
            confidence,
            category,
            tags,
        } => record_insight(scope, title, content, confidence, category, tags),
        InsightSubcommands::List {
            category,
            tag,
            min_confidence,
        } => list_insights(scope, category, tag, min_confidence),
        InsightSubcommands::Update {
            id,
            title,
            content,
            confidence,
            category,
            tags,
        } => update_insight(scope, id, title, content, confidence, category, tags),
        InsightSubcommands::Delete { id } => delete_insight(scope, id),
    }
}

fn record_insight(
    scope: &rhema_core::scope::Scope,
    title: &str,
    content: &str,
    confidence: &Option<u8>,
    category: &Option<String>,
    tags: &Option<String>,
) -> RhemaResult<()> {
    let id = file_ops::add_knowledge(
        &scope.path,
        title.to_string(),
        content.to_string(),
        confidence.clone(),
        category.clone(),
        tags.clone(),
    )?;

    println!("💡 Insight recorded successfully with ID: {}", id.green());
    println!("📝 Title: {}", title);
    println!("📄 Content: {}", content);
    if let Some(conf) = confidence {
        println!("🎯 Confidence: {}/10", conf);
    }
    if let Some(cat) = category {
        println!("📂 Category: {}", cat);
    }
    if let Some(tags) = tags {
        println!("🏷️  Tags: {}", tags);
    }

    Ok(())
}

fn list_insights(
    scope: &rhema_core::scope::Scope,
    category: &Option<String>,
    tag: &Option<String>,
    min_confidence: &Option<u8>,
) -> RhemaResult<()> {
    let insights = file_ops::list_knowledge(
        &scope.path,
        category.clone(),
        tag.clone(),
        min_confidence.clone(),
    )?;

    if insights.is_empty() {
        println!("📭 No insights found");
        return Ok(());
    }

    println!("💡 Insights in scope: {}", scope.definition.name);
    println!("{}", "─".repeat(80));

    for insight in insights {
        println!("🆔 ID: {}", insight.id);
        println!("📝 Title: {}", insight.title);
        println!("📄 Content: {}", insight.content);
        if let Some(cat) = &insight.category {
            println!("📂 Category: {}", cat);
        }
        if let Some(tags) = &insight.tags {
            println!("🏷️  Tags: {}", tags.join(", "));
        }
        if let Some(conf) = &insight.confidence {
            let confidence_color = if *conf >= 8 {
                "green"
            } else if *conf >= 5 {
                "yellow"
            } else {
                "red"
            };
            println!(
                "🎯 Confidence: {}",
                format!("{}/10", conf).color(confidence_color)
            );
        }
        println!(
            "📅 Created: {}",
            insight.created_at.format("%Y-%m-%d %H:%M")
        );
        if let Some(updated_at) = &insight.updated_at {
            println!("📝 Updated: {}", updated_at.format("%Y-%m-%d %H:%M"));
        }
        if let Some(source) = &insight.source {
            println!("🔗 Source: {}", source);
        }
        println!("{}", "─".repeat(80));
    }

    Ok(())
}

fn update_insight(
    scope: &rhema_core::scope::Scope,
    id: &str,
    title: &Option<String>,
    content: &Option<String>,
    confidence: &Option<u8>,
    category: &Option<String>,
    tags: &Option<String>,
) -> RhemaResult<()> {
    file_ops::update_knowledge(
        &scope.path,
        id,
        title.clone(),
        content.clone(),
        confidence.clone(),
        category.clone(),
        tags.clone(),
    )?;

    println!("✅ Insight {} updated successfully", id.green());

    if title.is_some()
        || content.is_some()
        || confidence.is_some()
        || category.is_some()
        || tags.is_some()
    {
        println!("📝 Updated fields:");
        if title.is_some() {
            println!("  - Title: {}", title.as_ref().unwrap());
        }
        if content.is_some() {
            println!("  - Content: {}", content.as_ref().unwrap());
        }
        if confidence.is_some() {
            println!("  - Confidence: {}/10", confidence.as_ref().unwrap());
        }
        if category.is_some() {
            println!("  - Category: {}", category.as_ref().unwrap());
        }
        if tags.is_some() {
            println!("  - Tags: {}", tags.as_ref().unwrap());
        }
    }

    Ok(())
}

fn delete_insight(scope: &rhema_core::scope::Scope, id: &str) -> RhemaResult<()> {
    file_ops::delete_knowledge(&scope.path, id)?;

    println!("🗑️  Insight {} deleted successfully", id.green());

    Ok(())
}
