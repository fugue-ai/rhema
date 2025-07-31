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

use crate::{Rhema, RhemaResult, PatternSubcommands, PatternUsage};
use crate::file_ops;
use crate::scope::find_nearest_scope;
use colored::*;

pub fn run(rhema: &Rhema, subcommand: &PatternSubcommands) -> RhemaResult<()> {
    // Get the current working directory to find the nearest scope
    let current_dir = std::env::current_dir()
        .map_err(|e| crate::RhemaError::IoError(e))?;
    
    // Discover all scopes
    let scopes = rhema.discover_scopes()?;
    
    // Find the nearest scope to the current directory
    let scope = find_nearest_scope(&current_dir, &scopes)
        .ok_or_else(|| crate::RhemaError::ConfigError("No Rhema scope found in current directory or parent directories".to_string()))?;
    
    match subcommand {
        PatternSubcommands::Add { name, description, pattern_type, usage, effectiveness, examples, anti_patterns } => {
            add_pattern(scope, name, description, pattern_type, usage, effectiveness, examples, anti_patterns)
        }
        PatternSubcommands::List { pattern_type, usage, min_effectiveness } => {
            list_patterns(scope, pattern_type, usage, min_effectiveness)
        }
        PatternSubcommands::Update { id, name, description, pattern_type, usage, effectiveness, examples, anti_patterns } => {
            update_pattern(scope, id, name, description, pattern_type, usage, effectiveness, examples, anti_patterns)
        }
        PatternSubcommands::Delete { id } => {
            delete_pattern(scope, id)
        }
    }
}

fn add_pattern(
    scope: &crate::Scope,
    name: &str,
    description: &str,
    pattern_type: &str,
    usage: &PatternUsage,
    effectiveness: &Option<u8>,
    examples: &Option<String>,
    anti_patterns: &Option<String>,
) -> RhemaResult<()> {
    let id = file_ops::add_pattern(
        &scope.path,
        name.to_string(),
        description.to_string(),
        pattern_type.to_string(),
        usage.clone(),
        effectiveness.clone(),
        examples.clone(),
        anti_patterns.clone(),
    )?;
    
    println!("🔄 Pattern added successfully with ID: {}", id.green());
    println!("📝 Name: {}", name);
    println!("📄 Description: {}", description);
    println!("🏷️  Type: {}", pattern_type);
    println!("📊 Usage: {:?}", usage);
    if let Some(eff) = effectiveness {
        println!("⭐ Effectiveness: {}/10", eff);
    }
    if let Some(examples) = examples {
        println!("💡 Examples: {}", examples);
    }
    if let Some(anti_patterns) = anti_patterns {
        println!("⚠️  Anti-patterns: {}", anti_patterns);
    }
    
    Ok(())
}

fn list_patterns(
    scope: &crate::Scope,
    pattern_type: &Option<String>,
    usage: &Option<PatternUsage>,
    min_effectiveness: &Option<u8>,
) -> RhemaResult<()> {
    let patterns = file_ops::list_patterns(
        &scope.path,
        pattern_type.clone(),
        usage.clone(),
        min_effectiveness.clone(),
    )?;
    
    if patterns.is_empty() {
        println!("📭 No patterns found");
        return Ok(());
    }
    
    println!("🔄 Patterns in scope: {}", scope.definition.name);
    println!("{}", "─".repeat(80));
    
    for pattern in patterns {
        let usage_color = match pattern.usage {
            PatternUsage::Required => "red",
            PatternUsage::Recommended => "green",
            PatternUsage::Optional => "yellow",
            PatternUsage::Deprecated => "dimmed",
        };
        
        println!("🆔 ID: {}", pattern.id);
        println!("📝 Name: {}", pattern.name);
        println!("📄 Description: {}", pattern.description);
        println!("🏷️  Type: {}", pattern.pattern_type);
        println!("📊 Usage: {}", format!("{:?}", pattern.usage).color(usage_color));
        if let Some(eff) = &pattern.effectiveness {
            let effectiveness_color = if *eff >= 8 { "green" } else if *eff >= 5 { "yellow" } else { "red" };
            println!("⭐ Effectiveness: {}", format!("{}/10", eff).color(effectiveness_color));
        }
        if let Some(examples) = &pattern.examples {
            println!("💡 Examples: {}", examples.join(", "));
        }
        if let Some(anti_patterns) = &pattern.anti_patterns {
            println!("⚠️  Anti-patterns: {}", anti_patterns.join(", "));
        }
        if let Some(related) = &pattern.related_patterns {
            println!("🔗 Related patterns: {}", related.join(", "));
        }
        println!("📅 Created: {}", pattern.created_at.format("%Y-%m-%d %H:%M"));
        if let Some(updated_at) = &pattern.updated_at {
            println!("📝 Updated: {}", updated_at.format("%Y-%m-%d %H:%M"));
        }
        println!("{}", "─".repeat(80));
    }
    
    Ok(())
}

fn update_pattern(
    scope: &crate::Scope,
    id: &str,
    name: &Option<String>,
    description: &Option<String>,
    pattern_type: &Option<String>,
    usage: &Option<PatternUsage>,
    effectiveness: &Option<u8>,
    examples: &Option<String>,
    anti_patterns: &Option<String>,
) -> RhemaResult<()> {
    file_ops::update_pattern(
        &scope.path,
        id,
        name.clone(),
        description.clone(),
        pattern_type.clone(),
        usage.clone(),
        effectiveness.clone(),
        examples.clone(),
        anti_patterns.clone(),
    )?;
    
    println!("✅ Pattern {} updated successfully", id.green());
    
    if name.is_some() || description.is_some() || pattern_type.is_some() || 
       usage.is_some() || effectiveness.is_some() || examples.is_some() || anti_patterns.is_some() {
        println!("📝 Updated fields:");
        if name.is_some() {
            println!("  - Name: {}", name.as_ref().unwrap());
        }
        if description.is_some() {
            println!("  - Description: {}", description.as_ref().unwrap());
        }
        if pattern_type.is_some() {
            println!("  - Type: {}", pattern_type.as_ref().unwrap());
        }
        if usage.is_some() {
            println!("  - Usage: {:?}", usage.as_ref().unwrap());
        }
        if effectiveness.is_some() {
            println!("  - Effectiveness: {}/10", effectiveness.as_ref().unwrap());
        }
        if examples.is_some() {
            println!("  - Examples: {}", examples.as_ref().unwrap());
        }
        if anti_patterns.is_some() {
            println!("  - Anti-patterns: {}", anti_patterns.as_ref().unwrap());
        }
    }
    
    Ok(())
}

fn delete_pattern(
    scope: &crate::Scope,
    id: &str,
) -> RhemaResult<()> {
    file_ops::delete_pattern(&scope.path, id)?;
    
    println!("🗑️  Pattern {} deleted successfully", id.green());
    
    Ok(())
} 
