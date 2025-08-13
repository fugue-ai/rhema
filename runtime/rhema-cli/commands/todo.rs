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

use crate::CliContext;
use clap::Subcommand;
use rhema_api::RhemaResult;
use rhema_core::{Priority, TodoStatus};

#[derive(Subcommand)]
pub enum TodoSubcommands {
    /// Add a new todo
    Add {
        /// Todo title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Todo description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// Priority level
        #[arg(long, value_enum, default_value = "medium")]
        priority: Priority,

        /// Assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,

        /// Due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },

    /// List todos
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,

        /// Filter by priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,

        /// Filter by assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,
    },

    /// Complete a todo
    Complete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,

        /// Completion outcome
        #[arg(long, value_name = "OUTCOME")]
        outcome: Option<String>,
    },

    /// Update a todo
    Update {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,

        /// New priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,

        /// New assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,

        /// New due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },

    /// Delete a todo
    Delete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

pub fn handle_todo(
    context: &CliContext,
    scope: &rhema_core::Scope,
    subcommand: &TodoSubcommands,
) -> RhemaResult<()> {
    match subcommand {
        TodoSubcommands::Add {
            title,
            description,
            priority,
            assignee,
            due_date,
        } => {
            match rhema_core::file_ops::add_todo(
                &scope.path,
                title.to_string(),
                description.clone(),
                priority.clone(),
                assignee.clone(),
                due_date.clone(),
            ) {
                Ok(id) => {
                    println!("✅ Todo added successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    if let Some(desc) = description {
                        println!("📄 Description: {}", desc);
                    }
                    println!("🎯 Priority: {:?}", priority);
                    if let Some(assign) = assignee {
                        println!("👤 Assignee: {}", assign);
                    }
                    if let Some(date) = due_date {
                        println!("📅 Due date: {}", date);
                    }
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        TodoSubcommands::List {
            status,
            priority,
            assignee,
        } => {
            match rhema_core::file_ops::list_todos(
                &scope.path,
                status.clone(),
                priority.clone(),
                assignee.clone(),
            ) {
                Ok(todos) => {
                    if todos.is_empty() {
                        println!("📭 No todos found");
                    } else {
                        println!("📋 Found {} todos:", todos.len());
                        for todo in todos {
                            println!("  • {} - {} ({:?})", todo.id, todo.title, todo.status);
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        TodoSubcommands::Complete { id, outcome } => {
            match rhema_core::file_ops::complete_todo(&scope.path, id, outcome.clone()) {
                Ok(()) => {
                    println!("✅ Todo {} completed successfully!", id);
                    if let Some(out) = outcome {
                        println!("📊 Outcome: {}", out);
                    }
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        TodoSubcommands::Update {
            id,
            title,
            description,
            status,
            priority,
            assignee,
            due_date,
        } => {
            match rhema_core::file_ops::update_todo(
                &scope.path,
                id,
                title.clone(),
                description.clone(),
                status.clone(),
                priority.clone(),
                assignee.clone(),
                due_date.clone(),
            ) {
                Ok(()) => {
                    println!("✅ Todo {} updated successfully!", id);
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        TodoSubcommands::Delete { id } => {
            match rhema_core::file_ops::delete_todo(&scope.path, id) {
                Ok(()) => {
                    println!("🗑️  Todo {} deleted successfully!", id);
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
    }
}
