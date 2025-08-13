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
use rhema_core::PatternUsage;

#[derive(Subcommand)]
pub enum PatternSubcommands {
    /// Add a new pattern
    Add {
        /// Pattern name
        #[arg(value_name = "NAME")]
        name: String,

        /// Pattern description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: String,

        /// Usage context
        #[arg(long, value_enum, default_value = "recommended")]
        usage: PatternUsage,

        /// Effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// Examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// Anti-patterns to avoid (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },

    /// List patterns
    List {
        /// Filter by pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// Filter by usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// Filter by effectiveness rating (minimum)
        #[arg(long, value_name = "RATING")]
        min_effectiveness: Option<u8>,
    },

    /// Update a pattern
    Update {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// New usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// New effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// New examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// New anti-patterns (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },

    /// Delete a pattern
    Delete {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

pub fn handle_pattern(
    context: &CliContext,
    scope: &rhema_core::Scope,
    subcommand: &PatternSubcommands,
) -> RhemaResult<()> {
    match subcommand {
        PatternSubcommands::Add {
            name,
            description,
            pattern_type,
            usage,
            effectiveness,
            examples,
            anti_patterns,
        } => {
            match rhema_core::file_ops::add_pattern(
                &scope.path,
                name.to_string(),
                description.to_string(),
                pattern_type.to_string(),
                usage.clone(),
                *effectiveness,
                examples.clone(),
                anti_patterns.clone(),
            ) {
                Ok(id) => {
                    println!("🔧 Pattern added successfully with ID: {}", id);
                    println!("📝 Name: {}", name);
                    println!("📄 Description: {}", description);
                    println!("🏷️  Type: {}", pattern_type);
                    println!("🎯 Usage: {:?}", usage);
                    if let Some(eff) = effectiveness {
                        println!("⭐ Effectiveness: {}/10", eff);
                    }
                    if let Some(ex) = examples {
                        println!("📚 Examples: {}", ex);
                    }
                    if let Some(anti) = anti_patterns {
                        println!("⚠️  Anti-patterns: {}", anti);
                    }
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        PatternSubcommands::List {
            pattern_type,
            usage,
            min_effectiveness,
        } => {
            match rhema_core::file_ops::list_patterns(
                &scope.path,
                pattern_type.clone(),
                usage.clone(),
                *min_effectiveness,
            ) {
                Ok(patterns) => {
                    if patterns.is_empty() {
                        println!("📭 No patterns found");
                    } else {
                        println!("🔧 Found {} patterns:", patterns.len());
                        for pattern in patterns {
                            println!(
                                "  • {} - {} ({:?})",
                                pattern.id, pattern.name, pattern.usage
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
        PatternSubcommands::Update {
            id,
            name,
            description,
            pattern_type,
            usage,
            effectiveness,
            examples,
            anti_patterns,
        } => {
            match rhema_core::file_ops::update_pattern(
                &scope.path,
                id,
                name.clone(),
                description.clone(),
                pattern_type.clone(),
                usage.clone(),
                *effectiveness,
                examples.clone(),
                anti_patterns.clone(),
            ) {
                Ok(()) => {
                    println!("✅ Pattern {} updated successfully!", id);
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        PatternSubcommands::Delete { id } => {
            match rhema_core::file_ops::delete_pattern(&scope.path, id) {
                Ok(()) => {
                    println!("🗑️  Pattern {} deleted successfully!", id);
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
