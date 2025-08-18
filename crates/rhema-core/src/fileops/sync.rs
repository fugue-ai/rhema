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

use crate::schema::knowledge::{
    DecisionEntry, DecisionStatus, Decisions, Knowledge, KnowledgeEntry, PatternEntry,
    PatternUsage, PatternUsageStats, Patterns, Priority, TodoEntry, TodoStatus, Todos,
};
use crate::{log_file_operation, RhemaError, RhemaResult, ValidationRules};
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
    // Validate file path for security
    ValidationRules::validate_file_path_static(file_path)?;

    if !file_path.exists() {
        let error_msg = format!("File not found: {}", file_path.display());
        log_file_operation("read", file_path, false, Some(error_msg.clone()))?;
        return Err(RhemaError::FileNotFound(error_msg));
    }

    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            let error_msg = e.to_string();
            log_file_operation("read", file_path, false, Some(error_msg.clone()))?;
            return Err(RhemaError::IoError(e));
        }
    };

    let data: T = match serde_yaml::from_str(&content) {
        Ok(data) => data,
        Err(e) => {
            let error_msg = e.to_string();
            log_file_operation("read", file_path, false, Some(error_msg.clone()))?;
            return Err(RhemaError::InvalidYaml {
                file: file_path.display().to_string(),
                message: error_msg,
            });
        }
    };

    // Log successful read operation
    log_file_operation("read", file_path, true, None)?;

    Ok(data)
}

/// Write a YAML file with the specified data
pub fn write_yaml_file<T>(file_path: &Path, data: &T) -> RhemaResult<()>
where
    T: serde::Serialize,
{
    // Validate file path for security
    ValidationRules::validate_file_path_static(file_path)?;

    // Ensure the directory exists
    if let Some(parent) = file_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            let error_msg = e.to_string();
            log_file_operation("write", file_path, false, Some(error_msg.clone()))?;
            return Err(RhemaError::IoError(e));
        }
    }

    let content = match serde_yaml::to_string(data) {
        Ok(content) => content,
        Err(e) => {
            let error_msg = e.to_string();
            log_file_operation("write", file_path, false, Some(error_msg.clone()))?;
            return Err(RhemaError::InvalidYaml {
                file: file_path.display().to_string(),
                message: error_msg,
            });
        }
    };

    match std::fs::write(file_path, content) {
        Ok(_) => {
            // Log successful write operation
            log_file_operation("write", file_path, true, None)?;
            Ok(())
        }
        Err(e) => {
            let error_msg = e.to_string();
            log_file_operation("write", file_path, false, Some(error_msg.clone()))?;
            Err(RhemaError::IoError(e))
        }
    }
}

/// Get or create the knowledge file for a scope
pub fn get_or_create_knowledge_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let knowledge_file = scope_path.join("knowledge.yaml");

    if !knowledge_file.exists() {
        let knowledge = Knowledge {
            entries: Vec::new(),
            categories: Some(HashMap::new()),
            custom: HashMap::new(),
        };
        write_yaml_file(&knowledge_file, &knowledge)?;
    }

    Ok(knowledge_file)
}

/// Add a knowledge entry
pub fn add_knowledge_entry(
    scope_path: &Path,
    title: &str,
    content: &str,
    category: Option<&str>,
    tags: Option<Vec<String>>,
    confidence: Option<u8>,
    source: Option<&str>,
) -> RhemaResult<String> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let mut knowledge: Knowledge = read_yaml_file(&knowledge_file)?;

    let entry = KnowledgeEntry {
        id: Uuid::new_v4().to_string(),
        title: title.to_string(),
        content: content.to_string(),
        category: category.map(|s| s.to_string()),
        tags,
        confidence,
        created_at: Utc::now(),
        updated_at: None,
        source: source.map(|s| s.to_string()),
        custom: HashMap::new(),
    };

    knowledge.entries.push(entry.clone());
    write_yaml_file(&knowledge_file, &knowledge)?;

    Ok(entry.id)
}

/// Get knowledge entries with optional filtering
pub fn get_knowledge_entries(
    scope_path: &Path,
    category_filter: Option<&str>,
    tag_filter: Option<&str>,
) -> RhemaResult<Vec<KnowledgeEntry>> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let knowledge: Knowledge = read_yaml_file(&knowledge_file)?;

    let mut filtered_entries = knowledge.entries;

    if let Some(category) = category_filter {
        filtered_entries.retain(|entry| entry.category.as_ref().map_or(false, |c| c == category));
    }

    if let Some(tag) = tag_filter {
        filtered_entries.retain(|entry| {
            entry
                .tags
                .as_ref()
                .map_or(false, |tags| tags.contains(&tag.to_string()))
        });
    }

    Ok(filtered_entries)
}

/// Update a knowledge entry
pub fn update_knowledge_entry(
    scope_path: &Path,
    id: &str,
    title: Option<String>,
    content: Option<String>,
    category: Option<String>,
    tags: Option<Vec<String>>,
    confidence: Option<u8>,
) -> RhemaResult<()> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let mut knowledge: Knowledge = read_yaml_file(&knowledge_file)?;

    let entry = knowledge
        .entries
        .iter_mut()
        .find(|e| e.id == id)
        .ok_or_else(|| {
            RhemaError::ConfigError(format!("Knowledge entry with ID {} not found", id))
        })?;

    if let Some(title) = title {
        entry.title = title;
    }
    if let Some(content) = content {
        entry.content = content;
    }
    if let Some(category) = category {
        entry.category = Some(category);
    }
    if let Some(tags) = tags {
        entry.tags = Some(tags);
    }
    if let Some(confidence) = confidence {
        entry.confidence = Some(confidence);
    }

    entry.updated_at = Some(Utc::now());

    write_yaml_file(&knowledge_file, &knowledge)?;
    Ok(())
}

/// Delete a knowledge entry
pub fn delete_knowledge_entry(scope_path: &Path, id: &str) -> RhemaResult<()> {
    let knowledge_file = get_or_create_knowledge_file(scope_path)?;
    let mut knowledge: Knowledge = read_yaml_file(&knowledge_file)?;

    let initial_len = knowledge.entries.len();
    knowledge.entries.retain(|e| e.id != id);

    if knowledge.entries.len() == initial_len {
        return Err(RhemaError::ConfigError(format!(
            "Knowledge entry with ID {} not found",
            id
        )));
    }

    write_yaml_file(&knowledge_file, &knowledge)?;
    Ok(())
}

/// Get or create the todos file for a scope
pub fn get_or_create_todos_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let todos_file = scope_path.join("todos.yaml");

    if !todos_file.exists() {
        let todos = Todos {
            todos: Vec::new(),
            custom: HashMap::new(),
        };
        write_yaml_file(&todos_file, &todos)?;
    }

    Ok(todos_file)
}

/// Add a todo entry
pub fn add_todo_entry(
    scope_path: &Path,
    title: &str,
    description: Option<&str>,
    priority: Priority,
    assigned_to: Option<&str>,
    due_date: Option<chrono::DateTime<Utc>>,
) -> RhemaResult<String> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let mut todos: Todos = read_yaml_file(&todos_file)?;

    let entry = TodoEntry {
        id: Uuid::new_v4().to_string(),
        title: title.to_string(),
        description: description.map(|s| s.to_string()),
        status: TodoStatus::Pending,
        priority,
        assigned_to: assigned_to.map(|s| s.to_string()),
        due_date,
        created_at: Utc::now(),
        completed_at: None,
        outcome: None,
        related_knowledge: None,
        custom: HashMap::new(),
    };

    todos.todos.push(entry.clone());
    write_yaml_file(&todos_file, &todos)?;

    Ok(entry.id)
}

/// Get todo entries with optional filtering
pub fn get_todo_entries(
    scope_path: &Path,
    status_filter: Option<TodoStatus>,
    priority_filter: Option<Priority>,
    assigned_to_filter: Option<&str>,
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

    if let Some(assigned_to) = assigned_to_filter {
        filtered_todos.retain(|todo| {
            todo.assigned_to
                .as_ref()
                .map_or(false, |a| a == assigned_to)
        });
    }

    Ok(filtered_todos)
}

/// Update a todo entry
pub fn update_todo_entry(
    scope_path: &Path,
    id: &str,
    title: Option<String>,
    description: Option<String>,
    status: Option<TodoStatus>,
    priority: Option<Priority>,
    assigned_to: Option<String>,
    due_date: Option<chrono::DateTime<Utc>>,
    outcome: Option<String>,
) -> RhemaResult<()> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let mut todos: Todos = read_yaml_file(&todos_file)?;

    let todo = todos
        .todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| RhemaError::ConfigError(format!("Todo with ID {} not found", id)))?;

    if let Some(title) = title {
        todo.title = title;
    }
    if let Some(description) = description {
        todo.description = Some(description);
    }
    if let Some(status) = status {
        todo.status = status.clone();
        if status == TodoStatus::Completed && todo.completed_at.is_none() {
            todo.completed_at = Some(Utc::now());
        }
    }
    if let Some(priority) = priority {
        todo.priority = priority;
    }
    if let Some(assigned_to) = assigned_to {
        todo.assigned_to = Some(assigned_to);
    }
    if let Some(due_date) = due_date {
        todo.due_date = Some(due_date);
    }
    if let Some(outcome) = outcome {
        todo.outcome = Some(outcome);
    }

    write_yaml_file(&todos_file, &todos)?;
    Ok(())
}

/// Delete a todo entry
pub fn delete_todo_entry(scope_path: &Path, id: &str) -> RhemaResult<()> {
    let todos_file = get_or_create_todos_file(scope_path)?;
    let mut todos: Todos = read_yaml_file(&todos_file)?;

    let initial_len = todos.todos.len();
    todos.todos.retain(|t| t.id != id);

    if todos.todos.len() == initial_len {
        return Err(RhemaError::ConfigError(format!(
            "Todo with ID {} not found",
            id
        )));
    }

    write_yaml_file(&todos_file, &todos)?;
    Ok(())
}

/// Get or create the decisions file for a scope
pub fn get_or_create_decisions_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let decisions_file = scope_path.join("decisions.yaml");

    if !decisions_file.exists() {
        let decisions = Decisions {
            decisions: Vec::new(),
            custom: HashMap::new(),
        };
        write_yaml_file(&decisions_file, &decisions)?;
    }

    Ok(decisions_file)
}

/// Add a decision entry
pub fn add_decision_entry(
    scope_path: &Path,
    title: &str,
    description: &str,
    context: Option<&str>,
    makers: Option<&str>,
    alternatives: Option<&str>,
    rationale: Option<&str>,
    consequences: Option<&str>,
) -> RhemaResult<String> {
    let decisions_file = get_or_create_decisions_file(scope_path)?;
    let mut decisions: Decisions = read_yaml_file(&decisions_file)?;

    let entry = DecisionEntry {
        id: Uuid::new_v4().to_string(),
        title: title.to_string(),
        description: description.to_string(),
        status: DecisionStatus::Proposed,
        context: context.map(|s| s.to_string()),
        alternatives: alternatives.map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        rationale: rationale.map(|s| s.to_string()),
        consequences: consequences.map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        decided_at: Utc::now(),
        review_date: None,
        decision_makers: makers.map(|s| s.split(',').map(|s| s.trim().to_string()).collect()),
        custom: HashMap::new(),
    };

    decisions.decisions.push(entry.clone());
    write_yaml_file(&decisions_file, &decisions)?;

    Ok(entry.id)
}

/// Get decision entries with optional filtering
pub fn get_decision_entries(
    scope_path: &Path,
    status_filter: Option<DecisionStatus>,
    maker_filter: Option<&str>,
) -> RhemaResult<Vec<DecisionEntry>> {
    let decisions_file = get_or_create_decisions_file(scope_path)?;
    let decisions: Decisions = read_yaml_file(&decisions_file)?;

    let mut filtered_decisions = decisions.decisions;

    if let Some(status) = status_filter {
        filtered_decisions.retain(|decision| decision.status == status);
    }

    if let Some(maker) = maker_filter {
        filtered_decisions.retain(|decision| {
            decision
                .decision_makers
                .as_ref()
                .map_or(false, |makers| makers.contains(&maker.to_string()))
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

    let decision = decisions
        .decisions
        .iter_mut()
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
        decision.alternatives = Some(
            alternatives
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    if let Some(rationale) = rationale {
        decision.rationale = Some(rationale);
    }
    if let Some(consequences) = consequences {
        decision.consequences = Some(
            consequences
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
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
        return Err(RhemaError::ConfigError(format!(
            "Decision with ID {} not found",
            id
        )));
    }

    write_yaml_file(&decisions_file, &decisions)?;
    Ok(())
}

// ===== PATTERN MANAGEMENT FUNCTIONS =====

/// Get or create the patterns file for a scope
pub fn get_or_create_patterns_file(scope_path: &Path) -> RhemaResult<PathBuf> {
    let patterns_file = scope_path.join("patterns.yaml");

    if !patterns_file.exists() {
        let patterns = Patterns {
            patterns: Vec::new(),
            custom: HashMap::new(),
        };
        write_yaml_file(&patterns_file, &patterns)?;
    }

    Ok(patterns_file)
}

/// Add a new pattern entry
pub fn add_pattern_entry(
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

    let pattern_entry = PatternEntry {
        id: id.clone(),
        name,
        description,
        pattern_type,
        usage,
        effectiveness,
        examples: examples.map(|e| e.split(',').map(|s| s.trim().to_string()).collect()),
        anti_patterns: anti_patterns.map(|a| a.split(',').map(|s| s.trim().to_string()).collect()),
        related_patterns: None,
        category: None,
        maturity: None,
        complexity: None,
        maintenance_effort: None,
        performance_impact: None,
        security_implications: None,
        testing_requirements: None,
        documentation_requirements: None,
        dependencies: None,
        applicable_contexts: None,
        version: None,
        author: None,
        review_status: None,
        last_reviewed: None,
        reviewers: None,
        usage_stats: Some(PatternUsageStats {
            usage_count: 0,
            success_count: 0,
            success_rate: 0.0,
            avg_implementation_time: None,
            last_used: None,
            common_use_cases: None,
            reported_issues: None,
            satisfaction_rating: None,
        }),
        created_at: now,
        updated_at: None,
        custom: HashMap::new(),
    };

    patterns.patterns.push(pattern_entry);
    write_yaml_file(&patterns_file, &patterns)?;

    Ok(id)
}

/// Get pattern entries with optional filtering
pub fn get_pattern_entries(
    scope_path: &Path,
    pattern_type: Option<String>,
    usage: Option<PatternUsage>,
    min_effectiveness: Option<u8>,
) -> RhemaResult<Vec<PatternEntry>> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let patterns: Patterns = read_yaml_file(&patterns_file)?;

    let mut filtered_patterns = patterns.patterns;

    if let Some(pattern_type) = pattern_type {
        filtered_patterns.retain(|pattern| pattern.pattern_type == pattern_type);
    }

    if let Some(usage) = usage {
        filtered_patterns.retain(|pattern| pattern.usage == usage);
    }

    if let Some(min_effectiveness) = min_effectiveness {
        filtered_patterns.retain(|pattern| {
            pattern
                .effectiveness
                .map_or(false, |eff| eff >= min_effectiveness)
        });
    }

    Ok(filtered_patterns)
}

/// Update a pattern entry
pub fn update_pattern_entry(
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

    let pattern = patterns
        .patterns
        .iter_mut()
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
        pattern.anti_patterns = Some(
            anti_patterns
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }

    pattern.updated_at = Some(Utc::now());

    write_yaml_file(&patterns_file, &patterns)?;
    Ok(())
}

/// Delete a pattern entry
pub fn delete_pattern_entry(scope_path: &Path, id: &str) -> RhemaResult<()> {
    let patterns_file = get_or_create_patterns_file(scope_path)?;
    let mut patterns: Patterns = read_yaml_file(&patterns_file)?;

    let initial_len = patterns.patterns.len();
    patterns.patterns.retain(|p| p.id != id);

    if patterns.patterns.len() == initial_len {
        return Err(RhemaError::ConfigError(format!(
            "Pattern with ID {} not found",
            id
        )));
    }

    write_yaml_file(&patterns_file, &patterns)?;
    Ok(())
}
