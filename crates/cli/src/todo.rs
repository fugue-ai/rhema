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
use rhema_core::{Priority, TodoStatus};
// TodoSubcommands will be defined in this module
use crate::file_ops;
use crate::scope::find_nearest_scope;

// Use the TodoSubcommands from commands module
use crate::commands::TodoSubcommands;
use colored::*;

pub fn run(rhema: &Rhema, subcommand: &TodoSubcommands) -> RhemaResult<()> {
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
        TodoSubcommands::Add {
            title,
            description,
            priority,
            assignee,
            due_date,
        } => add_todo(scope, title, description, priority, assignee, due_date),
        TodoSubcommands::List {
            status,
            priority,
            assignee,
        } => list_todos(scope, status, priority, assignee),
        TodoSubcommands::Complete { id, outcome } => complete_todo(scope, id, outcome),
        TodoSubcommands::Update {
            id,
            title,
            description,
            status,
            priority,
            assignee,
            due_date,
        } => update_todo(
            scope,
            id,
            title,
            description,
            status,
            priority,
            assignee,
            due_date,
        ),
        TodoSubcommands::Delete { id } => delete_todo(scope, id),
    }
}

fn add_todo(
    scope: &rhema_core::scope::Scope,
    title: &str,
    description: &Option<String>,
    priority: &Priority,
    assignee: &Option<String>,
    due_date: &Option<String>,
) -> RhemaResult<()> {
    let id = file_ops::add_todo(
        &scope.path,
        title.to_string(),
        description.clone(),
        priority.clone(),
        assignee.clone(),
        due_date.clone(),
    )?;

    println!("✅ Todo added successfully with ID: {}", id.green());
    println!("📝 Title: {}", title);
    if let Some(desc) = description {
        println!("📄 Description: {}", desc);
    }
    println!("🎯 Priority: {:?}", priority);
    if let Some(assignee) = assignee {
        println!("👤 Assignee: {}", assignee);
    }
    if let Some(date) = due_date {
        println!("📅 Due date: {}", date);
    }

    Ok(())
}

fn list_todos(
    scope: &rhema_core::scope::Scope,
    status: &Option<TodoStatus>,
    priority: &Option<Priority>,
    assignee: &Option<String>,
) -> RhemaResult<()> {
    let todos = file_ops::list_todos(
        &scope.path,
        status.clone(),
        priority.clone(),
        assignee.clone(),
    )?;

    if todos.is_empty() {
        println!("📭 No todos found");
        return Ok(());
    }

    println!("📋 Todos in scope: {}", scope.definition.name);
    println!("{}", "─".repeat(80));

    for todo in todos {
        let status_color = match todo.status {
            TodoStatus::Pending => "yellow",
            TodoStatus::InProgress => "blue",
            TodoStatus::Blocked => "red",
            TodoStatus::Completed => "green",
            TodoStatus::Cancelled => "dimmed",
        };

        let priority_color = match todo.priority {
            Priority::Low => "green",
            Priority::Medium => "yellow",
            Priority::High => "red",
            Priority::Critical => "red",
        };

        println!("🆔 ID: {}", todo.id);
        println!("📝 Title: {}", todo.title);
        if let Some(desc) = &todo.description {
            println!("📄 Description: {}", desc);
        }
        println!(
            "📊 Status: {}",
            format!("{:?}", todo.status).color(status_color)
        );
        println!(
            "🎯 Priority: {}",
            format!("{:?}", todo.priority).color(priority_color)
        );
        if let Some(assignee) = &todo.assigned_to {
            println!("👤 Assignee: {}", assignee);
        }
        if let Some(due_date) = &todo.due_date {
            println!("📅 Due date: {}", due_date.format("%Y-%m-%d %H:%M"));
        }
        println!("📅 Created: {}", todo.created_at.format("%Y-%m-%d %H:%M"));
        if let Some(completed_at) = &todo.completed_at {
            println!("✅ Completed: {}", completed_at.format("%Y-%m-%d %H:%M"));
        }
        if let Some(outcome) = &todo.outcome {
            println!("📈 Outcome: {}", outcome);
        }
        println!("{}", "─".repeat(80));
    }

    Ok(())
}

fn complete_todo(
    scope: &rhema_core::scope::Scope,
    id: &str,
    outcome: &Option<String>,
) -> RhemaResult<()> {
    file_ops::complete_todo(&scope.path, id, outcome.clone())?;

    println!("✅ Todo {} completed successfully", id.green());
    if let Some(outcome) = outcome {
        println!("📈 Outcome: {}", outcome);
    }

    Ok(())
}

fn update_todo(
    scope: &rhema_core::scope::Scope,
    id: &str,
    title: &Option<String>,
    description: &Option<String>,
    status: &Option<TodoStatus>,
    priority: &Option<Priority>,
    assignee: &Option<String>,
    due_date: &Option<String>,
) -> RhemaResult<()> {
    file_ops::update_todo(
        &scope.path,
        id,
        title.clone(),
        description.clone(),
        status.clone(),
        priority.clone(),
        assignee.clone(),
        due_date.clone(),
    )?;

    println!("✅ Todo {} updated successfully", id.green());

    if title.is_some()
        || description.is_some()
        || status.is_some()
        || priority.is_some()
        || assignee.is_some()
        || due_date.is_some()
    {
        println!("📝 Updated fields:");
        if title.is_some() {
            println!("  - Title: {}", title.as_ref().unwrap());
        }
        if description.is_some() {
            println!("  - Description: {}", description.as_ref().unwrap());
        }
        if status.is_some() {
            println!("  - Status: {:?}", status.as_ref().unwrap());
        }
        if priority.is_some() {
            println!("  - Priority: {:?}", priority.as_ref().unwrap());
        }
        if assignee.is_some() {
            println!("  - Assignee: {}", assignee.as_ref().unwrap());
        }
        if due_date.is_some() {
            println!("  - Due date: {}", due_date.as_ref().unwrap());
        }
    }

    Ok(())
}

fn delete_todo(scope: &rhema_core::scope::Scope, id: &str) -> RhemaResult<()> {
    file_ops::delete_todo(&scope.path, id)?;

    println!("🗑️  Todo {} deleted successfully", id.green());

    Ok(())
}
