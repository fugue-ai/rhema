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
use crate::file_ops;
use colored::*;
use serde_yaml;
use std::collections::HashMap;
use walkdir::WalkDir;

pub fn run(rhema: &Rhema) -> RhemaResult<()> {
    println!("📊 Rhema Statistics");
    println!("{}", "─".repeat(80));
    
    let scopes = rhema.discover_scopes()?;
    
    // Scope statistics
    println!("📁 Scope Statistics:");
    println!("  Total scopes: {}", scopes.len().to_string().bright_blue());
    
    let mut scope_types = HashMap::new();
    let mut total_files = 0;
    
    for scope in &scopes {
        *scope_types.entry(&scope.definition.scope_type).or_insert(0) += 1;
        
        // Count files in scope
        for entry in WalkDir::new(&scope.path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                total_files += 1;
            }
        }
    }
    
    println!("  Total YAML files: {}", total_files.to_string().bright_blue());
    println!("  Scope types:");
    for (scope_type, count) in scope_types {
        println!("    • {}: {}", scope_type, count.to_string().green());
    }
    
    // Context entry statistics
    println!("\n📋 Context Entry Statistics:");
    
    let mut total_todos = 0;
    let mut total_insights = 0;
    let mut total_patterns = 0;
    let mut total_decisions = 0;
    let mut total_conventions = 0;
    
    let mut todo_status_counts = HashMap::new();
    let mut todo_priority_counts = HashMap::new();
    let mut insight_confidence_counts = HashMap::new();
    let mut pattern_usage_counts = HashMap::new();
    let mut decision_status_counts = HashMap::new();
    
    for scope in &scopes {
        // Count todos
        if let Ok(todos) = file_ops::list_todos(&scope.path, None, None, None) {
            total_todos += todos.len();
            for todo in todos {
                *todo_status_counts.entry(format!("{:?}", todo.status)).or_insert(0) += 1;
                *todo_priority_counts.entry(format!("{:?}", todo.priority)).or_insert(0) += 1;
            }
        }
        
        // Count insights
        if let Ok(insights) = file_ops::list_knowledge(&scope.path, None, None, None) {
            total_insights += insights.len();
            for insight in insights {
                if let Some(confidence) = insight.confidence {
                    *insight_confidence_counts.entry(confidence).or_insert(0) += 1;
                }
            }
        }
        
        // Count patterns
        if let Ok(patterns) = file_ops::list_patterns(&scope.path, None, None, None) {
            total_patterns += patterns.len();
            for pattern in patterns {
                *pattern_usage_counts.entry(format!("{:?}", pattern.usage)).or_insert(0) += 1;
            }
        }
        
        // Count decisions
        if let Ok(decisions) = file_ops::list_decisions(&scope.path, None, None) {
            total_decisions += decisions.len();
            for decision in decisions {
                *decision_status_counts.entry(format!("{:?}", decision.status)).or_insert(0) += 1;
            }
        }
        
        // Count conventions
        let conventions_file = scope.path.join("conventions.yaml");
        if conventions_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&conventions_file) {
                if let Ok(conventions) = serde_yaml::from_str::<crate::Conventions>(&content) {
                    total_conventions += conventions.conventions.len();
                }
            }
        }
    }
    
    println!("  📝 Todos: {}", total_todos.to_string().bright_blue());
    if !todo_status_counts.is_empty() {
        println!("    Status breakdown:");
        for (status, count) in &todo_status_counts {
            println!("      • {}: {}", status, count.to_string().green());
        }
    }
    if !todo_priority_counts.is_empty() {
        println!("    Priority breakdown:");
        for (priority, count) in &todo_priority_counts {
            println!("      • {}: {}", priority, count.to_string().green());
        }
    }
    
    println!("  💡 Insights: {}", total_insights.to_string().bright_blue());
    if !insight_confidence_counts.is_empty() {
        println!("    Confidence breakdown:");
        for (confidence, count) in &insight_confidence_counts {
            println!("      • {}/10: {}", confidence, count.to_string().green());
        }
    }
    
    println!("  🔄 Patterns: {}", total_patterns.to_string().bright_blue());
    if !pattern_usage_counts.is_empty() {
        println!("    Usage breakdown:");
        for (usage, count) in &pattern_usage_counts {
            println!("      • {}: {}", usage, count.to_string().green());
        }
    }
    
    println!("  🎯 Decisions: {}", total_decisions.to_string().bright_blue());
    if !decision_status_counts.is_empty() {
        println!("    Status breakdown:");
        for (status, count) in &decision_status_counts {
            println!("      • {}: {}", status, count.to_string().green());
        }
    }
    
    println!("  📏 Conventions: {}", total_conventions.to_string().bright_blue());
    
    // Activity metrics
    println!("\n📈 Activity Metrics:");
    
    let mut recent_activity = 0;
    let mut oldest_entry = None;
    let mut newest_entry = None;
    
    for scope in &scopes {
        // Check todos
        if let Ok(todos) = file_ops::list_todos(&scope.path, None, None, None) {
            for todo in todos {
                if let Some(completed_at) = todo.completed_at {
                    if completed_at > chrono::Utc::now() - chrono::Duration::days(7) {
                        recent_activity += 1;
                    }
                }
                
                if oldest_entry.is_none() || todo.created_at < oldest_entry.unwrap() {
                    oldest_entry = Some(todo.created_at);
                }
                if newest_entry.is_none() || todo.created_at > newest_entry.unwrap() {
                    newest_entry = Some(todo.created_at);
                }
            }
        }
        
        // Check insights
        if let Ok(insights) = file_ops::list_knowledge(&scope.path, None, None, None) {
            for insight in insights {
                if oldest_entry.is_none() || insight.created_at < oldest_entry.unwrap() {
                    oldest_entry = Some(insight.created_at);
                }
                if newest_entry.is_none() || insight.created_at > newest_entry.unwrap() {
                    newest_entry = Some(insight.created_at);
                }
            }
        }
        
        // Check patterns
        if let Ok(patterns) = file_ops::list_patterns(&scope.path, None, None, None) {
            for pattern in patterns {
                if oldest_entry.is_none() || pattern.created_at < oldest_entry.unwrap() {
                    oldest_entry = Some(pattern.created_at);
                }
                if newest_entry.is_none() || pattern.created_at > newest_entry.unwrap() {
                    newest_entry = Some(pattern.created_at);
                }
            }
        }
        
        // Check decisions
        if let Ok(decisions) = file_ops::list_decisions(&scope.path, None, None) {
            for decision in decisions {
                if oldest_entry.is_none() || decision.decided_at < oldest_entry.unwrap() {
                    oldest_entry = Some(decision.decided_at);
                }
                if newest_entry.is_none() || decision.decided_at > newest_entry.unwrap() {
                    newest_entry = Some(decision.decided_at);
                }
            }
        }
    }
    
    println!("  Recent activity (last 7 days): {} completed todos", recent_activity.to_string().bright_blue());
    
    if let Some(oldest) = oldest_entry {
        println!("  Oldest entry: {}", oldest.format("%Y-%m-%d %H:%M").to_string().bright_blue());
    }
    
    if let Some(newest) = newest_entry {
        println!("  Newest entry: {}", newest.format("%Y-%m-%d %H:%M").to_string().bright_blue());
    }
    
    // Summary
    let total_entries = total_todos + total_insights + total_patterns + total_decisions + total_conventions;
    println!("\n📊 Summary:");
    println!("  Total context entries: {}", total_entries.to_string().bright_blue());
    println!("  Average entries per scope: {:.1}", (total_entries as f64 / scopes.len() as f64).to_string().bright_blue());
    
    Ok(())
} 
