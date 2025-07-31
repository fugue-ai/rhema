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

use crate::{RhemaError, RhemaResult, Todos, TodoEntry, Knowledge, KnowledgeEntry, Patterns, PatternEntry, Decisions, DecisionEntry, TodoStatus, Priority, DecisionStatus, PatternUsage};
use chrono::Utc;
use serde_yaml;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Read a YAML file and deserialize it into the specified type
pub fn read_yaml_file<T>(file_path: &Path) -> RhemaResult<T>
where
    T: serde::de::DeserializeOwned,
{
    if !file_path.exists() {
        return Err(RhemaError::FileNotFound(
            format!("File not found: {}", file_path.display())
        ));
    }
    
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| RhemaError::IoError(e))?;
    
    let data: T = serde_yaml::from_str(&content)
        .map_err(|e| RhemaError::InvalidYaml {
            file: file_path.display().to_string(),
            message: e.to_string(),
        })?;
    
    Ok(data)
}

/// Write a YAML file with the specified data
pub fn write_yaml_file<T>(file_path: &Path, data: &T) -> RhemaResult<()>
where
    T: serde::Serialize,
{
    // Ensure the directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| RhemaError::IoError(e))?;
    }
    
    let content = serde_yaml::to_string(data)
        .map_err(|e| RhemaError::InvalidYaml {
            file: file_path.display().to_string(),
            message: e.to_string(),
        })?;
    
    std::fs::write(file_path, content)
        .map_err(|e| RhemaError::IoError(e))?;
    
    Ok(())
}

/// Get or create a todos file
pub fn get_or_create_todos_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let todos_file = scope_path.join("todos.yaml");
    
    if !todos_file.exists() {
        let empty_todos = Todos {
            todos: Vec::new(),
            custom: HashMap::new(),
        };
        write_yaml_file(&todos_file, &empty_todos)?;
    }
    
    Ok(todos_file)
}

/// Get or create a knowledge file
pub fn get_or_create_knowledge_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let knowledge_file = scope_path.join("knowledge.yaml");
    
    if !knowledge_file.exists() {
        let empty_knowledge = Knowledge {
            entries: Vec::new(),
            categories: Some(HashMap::new()),
            custom: HashMap::new(),
        };
        write_yaml_file(&knowledge_file, &empty_knowledge)?;
    }
    
    Ok(knowledge_file)
}

/// Get or create a patterns file
pub fn get_or_create_patterns_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let patterns_file = scope_path.join("patterns.yaml");
    
    if !patterns_file.exists() {
        let empty_patterns = Patterns {
            patterns: Vec::new(),
            custom: HashMap::new(),
        };
        write_yaml_file(&patterns_file, &empty_patterns)?;
    }
    
    Ok(patterns_file)
}

/// Get or create a decisions file
pub fn get_or_create_decisions_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let decisions_file = scope_path.join("decisions.yaml");
    
    if !decisions_file.exists() {
        let empty_decisions = Decisions {
            decisions: Vec::new(),
            custom: HashMap::new(),
        };
        write_yaml_file(&decisions_file, &empty_decisions)?;
    }
    
    Ok(decisions_file)
}

/// Add a new todo entry
pub fn add_todo(
    scope_path: &Path,
    title: String,
    description: Option<String>,
    priority: Priority,
    assignee: Option<String>,
    due_date: Option<String>,
) -> RhemaResult<String> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let mut todos: Todos = read_yaml_file(&todos_file)?;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    let due_date_parsed = if let Some(date_str) = due_date {
        Some(chrono::DateTime::parse_from_rfc3339(&date_str)
            .map_err(|_| RhemaError::ConfigError("Invalid date format. Use ISO 8601 format (e.g., 2023-12-01T10:00:00Z)".to_string()))?
            .with_timezone(&Utc))
    } else {
        None
    };
    
    let todo_entry = TodoEntry {
        id: id.clone(),
        title,
        description,
        status: TodoStatus::Pending,
        priority,
        assigned_to: assignee,
        due_date: due_date_parsed,
        created_at: now,
        completed_at: None,
        outcome: None,
        related_knowledge: None,
        custom: HashMap::new(),
    };
    
    todos.todos.push(todo_entry);
    write_yaml_file(&todos_file, &todos)?;
    
    Ok(id)
}

/// List todo entries with optional filtering
pub fn list_todos(
    scope_path: &Path,
    status_filter: Option<TodoStatus>,
    priority_filter: Option<Priority>,
    assignee_filter: Option<String>,
) -> RhemaResult<Vec<TodoEntry>> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let todos: Todos = read_yaml_file(&todos_file)?;
    
    let mut filtered_todos = todos.todos;
    
    if let Some(status) = status_filter {
        filtered_todos.retain(|todo| todo.status == status);
    }
    
    if let Some(priority) = priority_filter {
        filtered_todos.retain(|todo| todo.priority == priority);
    }
    
    if let Some(assignee) = assignee_filter {
        filtered_todos.retain(|todo| {
            todo.assigned_to.as_ref().map_or(false, |a| a == &assignee)
        });
    }
    
    Ok(filtered_todos)
}

/// Complete a todo entry
pub fn complete_todo(
    scope_path: &Path,
    id: &str,
    outcome: Option<String>,
) -> RhemaResult<()> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let mut todos: Todos = read_yaml_file(&todos_file)?;
    
    let todo = todos.todos.iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Todo with ID {} not found", id)))?;
    
    todo.status = TodoStatus::Completed;
    todo.completed_at = Some(Utc::now());
    todo.outcome = outcome;
    
    write_yaml_file(&todos_file, &todos)?;
    Ok(())
}

/// Update a todo entry
pub fn update_todo(
    scope_path: &Path,
    id: &str,
    title: Option<String>,
    description: Option<String>,
    status: Option<TodoStatus>,
    priority: Option<Priority>,
    assignee: Option<String>,
    due_date: Option<String>,
) -> RhemaResult<()> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let mut todos: Todos = read_yaml_file(&todos_file)?;
    
    let todo = todos.todos.iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Todo with ID {} not found", id)))?;
    
    if let Some(title) = title {
        todo.title = title;
    }
    if let Some(description) = description {
        todo.description = Some(description);
    }
    if let Some(status) = status {
        todo.status = status;
    }
    if let Some(priority) = priority {
        todo.priority = priority;
    }
    if let Some(assignee) = assignee {
        todo.assigned_to = Some(assignee);
    }
    if let Some(date_str) = due_date {
        let due_date_parsed = chrono::DateTime::parse_from_rfc3339(&date_str)
            .map_err(|_| RhemaError::ConfigError("Invalid date format. Use ISO 8601 format (e.g., 2023-12-01T10:00:00Z)".to_string()))?
            .with_timezone(&Utc);
        todo.due_date = Some(due_date_parsed);
    }
    
    write_yaml_file(&todos_file, &todos)?;
    Ok(())
}

/// Delete a todo entry
pub fn delete_todo(scope_path: &Path, id: &str) -> RhemaResult<()> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let mut todos: Todos = read_yaml_file(&todos_file)?;
    
    let initial_len = todos.todos.len();
    todos.todos.retain(|t| t.id != id);
    
    if todos.todos.len() == initial_len {
        return Err(RhemaError::ConfigError(format!("Todo with ID {} not found", id)));
    }
    
    write_yaml_file(&todos_file, &todos)?;
    Ok(())
}

/// Add a new knowledge entry
pub fn add_knowledge(
    scope_path: &Path,
    title: String,
    content: String,
    confidence: Option<u8>,
    category: Option<String>,
    tags: Option<String>,
) -> RhemaResult<String> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let mut knowledge: Knowledge = read_yaml_file(&knowledge_file)?;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    let tags_vec = tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());
    
    let knowledge_entry = KnowledgeEntry {
        id: id.clone(),
        title,
        content,
        category,
        tags: tags_vec,
        confidence,
        created_at: now,
        updated_at: None,
        source: None,
        custom: HashMap::new(),
    };
    
    knowledge.entries.push(knowledge_entry);
    write_yaml_file(&knowledge_file, &knowledge)?;
    
    Ok(id)
}

/// List knowledge entries with optional filtering
pub fn list_knowledge(
    scope_path: &Path,
    category_filter: Option<String>,
    tag_filter: Option<String>,
    min_confidence: Option<u8>,
) -> RhemaResult<Vec<KnowledgeEntry>> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let knowledge: Knowledge = read_yaml_file(&knowledge_file)?;
    
    let mut filtered_entries = knowledge.entries;
    
    if let Some(category) = category_filter {
        filtered_entries.retain(|entry| {
            entry.category.as_ref().map_or(false, |c| c == &category)
        });
    }
    
    if let Some(tag) = tag_filter {
        filtered_entries.retain(|entry| {
            entry.tags.as_ref().map_or(false, |tags| tags.contains(&tag))
        });
    }
    
    if let Some(min_conf) = min_confidence {
        filtered_entries.retain(|entry| {
            entry.confidence.map_or(false, |conf| conf >= min_conf)
        });
    }
    
    Ok(filtered_entries)
}

/// Update a knowledge entry
pub fn update_knowledge(
    scope_path: &Path,
    id: &str,
    title: Option<String>,
    content: Option<String>,
    confidence: Option<u8>,
    category: Option<String>,
    tags: Option<String>,
) -> RhemaResult<()> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let mut knowledge: Knowledge = read_yaml_file(&knowledge_file)?;
    
    let entry = knowledge.entries.iter_mut()
        .find(|e| e.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Knowledge entry with ID {} not found", id)))?;
    
    if let Some(title) = title {
        entry.title = title;
    }
    if let Some(content) = content {
        entry.content = content;
    }
    if let Some(confidence) = confidence {
        entry.confidence = Some(confidence);
    }
    if let Some(category) = category {
        entry.category = Some(category);
    }
    if let Some(tags) = tags {
        entry.tags = Some(tags.split(',').map(|s| s.trim().to_string()).collect());
    }
    
    entry.updated_at = Some(Utc::now());
    
    write_yaml_file(&knowledge_file, &knowledge)?;
    Ok(())
}

/// Delete a knowledge entry
pub fn delete_knowledge(scope_path: &Path, id: &str) -> RhemaResult<()> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let mut knowledge: Knowledge = read_yaml_file(&knowledge_file)?;
    
    let initial_len = knowledge.entries.len();
    knowledge.entries.retain(|e| e.id != id);
    
    if knowledge.entries.len() == initial_len {
        return Err(RhemaError::ConfigError(format!("Knowledge entry with ID {} not found", id)));
    }
    
    write_yaml_file(&knowledge_file, &knowledge)?;
    Ok(())
}

/// Add a new pattern entry
pub fn add_pattern(
    scope_path: &Path,
    name: String,
    description: String,
    pattern_type: String,
    usage: PatternUsage,
    effectiveness: Option<u8>,
    examples: Option<String>,
    anti_patterns: Option<String>,
) -> RhemaResult<String> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let mut patterns: Patterns = read_yaml_file(&patterns_file)?;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    let examples_vec = examples.map(|e| e.split(',').map(|s| s.trim().to_string()).collect());
    let anti_patterns_vec = anti_patterns.map(|a| a.split(',').map(|s| s.trim().to_string()).collect());
    
    let pattern_entry = PatternEntry {
        id: id.clone(),
        name,
        description,
        pattern_type,
        usage,
        effectiveness,
        examples: examples_vec,
        anti_patterns: anti_patterns_vec,
        related_patterns: None,
        created_at: now,
        updated_at: None,
        custom: HashMap::new(),
    };
    
    patterns.patterns.push(pattern_entry);
    write_yaml_file(&patterns_file, &patterns)?;
    
    Ok(id)
}

/// List pattern entries with optional filtering
pub fn list_patterns(
    scope_path: &Path,
    pattern_type_filter: Option<String>,
    usage_filter: Option<PatternUsage>,
    min_effectiveness: Option<u8>,
) -> RhemaResult<Vec<PatternEntry>> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let patterns: Patterns = read_yaml_file(&patterns_file)?;
    
    let mut filtered_patterns = patterns.patterns;
    
    if let Some(pattern_type) = pattern_type_filter {
        filtered_patterns.retain(|pattern| pattern.pattern_type == pattern_type);
    }
    
    if let Some(usage) = usage_filter {
        filtered_patterns.retain(|pattern| pattern.usage == usage);
    }
    
    if let Some(min_eff) = min_effectiveness {
        filtered_patterns.retain(|pattern| {
            pattern.effectiveness.map_or(false, |eff| eff >= min_eff)
        });
    }
    
    Ok(filtered_patterns)
}

/// Update a pattern entry
pub fn update_pattern(
    scope_path: &Path,
    id: &str,
    name: Option<String>,
    description: Option<String>,
    pattern_type: Option<String>,
    usage: Option<PatternUsage>,
    effectiveness: Option<u8>,
    examples: Option<String>,
    anti_patterns: Option<String>,
) -> RhemaResult<()> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let mut patterns: Patterns = read_yaml_file(&patterns_file)?;
    
    let pattern = patterns.patterns.iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Pattern with ID {} not found", id)))?;
    
    if let Some(name) = name {
        pattern.name = name;
    }
    if let Some(description) = description {
        pattern.description = description;
    }
    if let Some(pattern_type) = pattern_type {
        pattern.pattern_type = pattern_type;
    }
    if let Some(usage) = usage {
        pattern.usage = usage;
    }
    if let Some(effectiveness) = effectiveness {
        pattern.effectiveness = Some(effectiveness);
    }
    if let Some(examples) = examples {
        pattern.examples = Some(examples.split(',').map(|s| s.trim().to_string()).collect());
    }
    if let Some(anti_patterns) = anti_patterns {
        pattern.anti_patterns = Some(anti_patterns.split(',').map(|s| s.trim().to_string()).collect());
    }
    
    pattern.updated_at = Some(Utc::now());
    
    write_yaml_file(&patterns_file, &patterns)?;
    Ok(())
}

/// Delete a pattern entry
pub fn delete_pattern(scope_path: &Path, id: &str) -> RhemaResult<()> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let mut patterns: Patterns = read_yaml_file(&patterns_file)?;
    
    let initial_len = patterns.patterns.len();
    patterns.patterns.retain(|p| p.id != id);
    
    if patterns.patterns.len() == initial_len {
        return Err(RhemaError::ConfigError(format!("Pattern with ID {} not found", id)));
    }
    
    write_yaml_file(&patterns_file, &patterns)?;
    Ok(())
}

/// Add a new decision entry
pub fn add_decision(
    scope_path: &Path,
    title: String,
    description: String,
    status: DecisionStatus,
    context: Option<String>,
    makers: Option<String>,
    alternatives: Option<String>,
    rationale: Option<String>,
    consequences: Option<String>,
) -> RhemaResult<String> {
    let decisions_file = get_or_create_decisions_file(scope_path)?;
    let mut decisions: Decisions = read_yaml_file(&decisions_file)?;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    let makers_vec = makers.map(|m| m.split(',').map(|s| s.trim().to_string()).collect());
    let alternatives_vec = alternatives.map(|a| a.split(',').map(|s| s.trim().to_string()).collect());
    let consequences_vec = consequences.map(|c| c.split(',').map(|s| s.trim().to_string()).collect());
    
    let decision_entry = DecisionEntry {
        id: id.clone(),
        title,
        description,
        status,
        context,
        alternatives: alternatives_vec,
        rationale,
        consequences: consequences_vec,
        decided_at: now,
        review_date: None,
        decision_makers: makers_vec,
        custom: HashMap::new(),
    };
    
    decisions.decisions.push(decision_entry);
    write_yaml_file(&decisions_file, &decisions)?;
    
    Ok(id)
}

/// List decision entries with optional filtering
pub fn list_decisions(
    scope_path: &Path,
    status_filter: Option<DecisionStatus>,
    maker_filter: Option<String>,
) -> RhemaResult<Vec<DecisionEntry>> {
    let decisions_file = get_or_create_decisions_file(scope_path)?;
    let decisions: Decisions = read_yaml_file(&decisions_file)?;
    
    let mut filtered_decisions = decisions.decisions;
    
    if let Some(status) = status_filter {
        filtered_decisions.retain(|decision| decision.status == status);
    }
    
    if let Some(maker) = maker_filter {
        filtered_decisions.retain(|decision| {
            decision.decision_makers.as_ref().map_or(false, |makers| makers.contains(&maker))
        });
    }
    
    Ok(filtered_decisions)
}

/// Update a decision entry
pub fn update_decision(
    scope_path: &Path,
    id: &str,
    title: Option<String>,
    description: Option<String>,
    status: Option<DecisionStatus>,
    context: Option<String>,
    makers: Option<String>,
    alternatives: Option<String>,
    rationale: Option<String>,
    consequences: Option<String>,
) -> RhemaResult<()> {
    let decisions_file = get_or_create_decisions_file(scope_path)?;
    let mut decisions: Decisions = read_yaml_file(&decisions_file)?;
    
    let decision = decisions.decisions.iter_mut()
        .find(|d| d.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Decision with ID {} not found", id)))?;
    
    if let Some(title) = title {
        decision.title = title;
    }
    if let Some(description) = description {
        decision.description = description;
    }
    if let Some(status) = status {
        decision.status = status;
    }
    if let Some(context) = context {
        decision.context = Some(context);
    }
    if let Some(makers) = makers {
        decision.decision_makers = Some(makers.split(',').map(|s| s.trim().to_string()).collect());
    }
    if let Some(alternatives) = alternatives {
        decision.alternatives = Some(alternatives.split(',').map(|s| s.trim().to_string()).collect());
    }
    if let Some(rationale) = rationale {
        decision.rationale = Some(rationale);
    }
    if let Some(consequences) = consequences {
        decision.consequences = Some(consequences.split(',').map(|s| s.trim().to_string()).collect());
    }
    
    write_yaml_file(&decisions_file, &decisions)?;
    Ok(())
}

/// Delete a decision entry
pub fn delete_decision(scope_path: &Path, id: &str) -> RhemaResult<()> {
    let decisions_file = get_or_create_decisions_file(scope_path)?;
    let mut decisions: Decisions = read_yaml_file(&decisions_file)?;
    
    let initial_len = decisions.decisions.len();
    decisions.decisions.retain(|d| d.id != id);
    
    if decisions.decisions.len() == initial_len {
        return Err(RhemaError::ConfigError(format!("Decision with ID {} not found", id)));
    }
    
    write_yaml_file(&decisions_file, &decisions)?;
    Ok(())
} 
