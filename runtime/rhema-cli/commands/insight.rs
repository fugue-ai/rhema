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

use clap::Subcommand;
use rhema_api::RhemaResult;
use crate::CliContext;

#[derive(Subcommand)]
pub enum InsightSubcommands {
    /// Record a new insight
    Record {
        /// Insight title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Insight content
        #[arg(long, value_name = "CONTENT")]
        content: String,

        /// Confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,

        /// Category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// List insights
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tag
        #[arg(long, value_name = "TAG")]
        tag: Option<String>,

        /// Filter by confidence level (minimum)
        #[arg(long, value_name = "LEVEL")]
        min_confidence: Option<u8>,
    },

    /// Update an insight
    Update {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New content
        #[arg(long, value_name = "CONTENT")]
        content: Option<String>,

        /// New confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// Delete an insight
    Delete {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

pub fn handle_insight(
    context: &CliContext,
    scope: &rhema_core::Scope,
    subcommand: &InsightSubcommands,
) -> RhemaResult<()> {
    match subcommand {
        InsightSubcommands::Record {
            title,
            content,
            confidence,
            category,
            tags,
        } => {
            match rhema_core::file_ops::add_knowledge(
                &scope.path,
                title.to_string(),
                content.to_string(),
                *confidence,
                category.clone(),
                tags.clone(),
            ) {
                Ok(id) => {
                    println!("💡 Insight recorded successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    println!("📄 Content: {}", content);
                    if let Some(conf) = confidence {
                        println!("🎯 Confidence: {}/10", conf);
                    }
                    if let Some(cat) = category {
                        println!("📂 Category: {}", cat);
                    }
                    if let Some(tag_list) = tags {
                        println!("🏷️  Tags: {}", tag_list);
                    }
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        InsightSubcommands::List {
            category,
            tag,
            min_confidence,
        } => {
            match rhema_core::file_ops::list_knowledge(
                &scope.path,
                category.clone(),
                tag.clone(),
                *min_confidence,
            ) {
                Ok(insights) => {
                    if insights.is_empty() {
                        println!("📭 No insights found");
                    } else {
                        println!("💡 Found {} insights:", insights.len());
                        for insight in insights {
                            println!(
                                "  • {} - {} (confidence: {})",
                                insight.id, insight.title, insight.confidence.unwrap_or(0)
                            );
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
        InsightSubcommands::Update {
            id,
            title,
            content,
            confidence,
            category,
            tags,
        } => {
            match rhema_core::file_ops::update_knowledge(
                &scope.path,
                id,
                title.clone(),
                content.clone(),
                *confidence,
                category.clone(),
                tags.clone(),
            ) {
                Ok(()) => {
                    println!("✅ Insight {} updated successfully!", id);
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        InsightSubcommands::Delete { id } => {
            match rhema_core::file_ops::delete_knowledge(&scope.path, id) {
                Ok(()) => {
                    println!("🗑️  Insight {} deleted successfully!", id);
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
