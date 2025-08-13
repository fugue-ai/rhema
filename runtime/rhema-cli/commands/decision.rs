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
use rhema_core::DecisionStatus;
use crate::CliContext;

#[derive(Subcommand)]
pub enum DecisionSubcommands {
    /// Record a new decision
    Record {
        /// Decision title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Decision description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Decision status
        #[arg(long, value_enum, default_value = "proposed")]
        status: DecisionStatus,

        /// Decision context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// Decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,

        /// Alternatives considered (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,

        /// Rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,

        /// Consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },

    /// List decisions
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<DecisionStatus>,

        /// Filter by decision maker
        #[arg(long, value_name = "MAKER")]
        maker: Option<String>,
    },

    /// Update a decision
    Update {
        /// Decision ID
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
        status: Option<DecisionStatus>,

        /// New context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// New decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,

        /// New alternatives (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,

        /// New rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,

        /// New consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },

    /// Delete a decision
    Delete {
        /// Decision ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

pub fn handle_decision(
    context: &CliContext,
    scope: &rhema_core::Scope,
    subcommand: &DecisionSubcommands,
) -> RhemaResult<()> {
    match subcommand {
        DecisionSubcommands::Record {
            title,
            description,
            status,
            context: decision_context,
            makers,
            alternatives,
            rationale,
            consequences,
        } => {
            match rhema_core::file_ops::add_decision(
                &scope.path,
                title.to_string(),
                description.to_string(),
                status.clone(),
                decision_context.clone(),
                makers.clone(),
                alternatives.clone(),
                rationale.clone(),
                consequences.clone(),
            ) {
                Ok(id) => {
                    println!("🎯 Decision recorded successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    println!("📄 Description: {}", description);
                    println!("📊 Status: {:?}", status);
                    if let Some(ctx) = decision_context {
                        println!("🌍 Context: {}", ctx);
                    }
                    if let Some(makers) = makers {
                        println!("👥 Makers: {}", makers);
                    }
                    if let Some(alt) = alternatives {
                        println!("🔄 Alternatives: {}", alt);
                    }
                    if let Some(rat) = rationale {
                        println!("🧠 Rationale: {}", rat);
                    }
                    if let Some(cons) = consequences {
                        println!("📈 Consequences: {}", cons);
                    }
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        DecisionSubcommands::List { status, maker } => {
            match rhema_core::file_ops::list_decisions(
                &scope.path,
                status.clone(),
                maker.clone(),
            ) {
                Ok(decisions) => {
                    if decisions.is_empty() {
                        println!("📭 No decisions found");
                    } else {
                        println!("🎯 Found {} decisions:", decisions.len());
                        for decision in decisions {
                            println!(
                                "  • {} - {} ({:?})",
                                decision.id, decision.title, decision.status
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
        DecisionSubcommands::Update {
            id,
            title,
            description,
            status,
            context: decision_context,
            makers,
            alternatives,
            rationale,
            consequences,
        } => {
            match rhema_core::file_ops::update_decision(
                &scope.path,
                id,
                title.clone(),
                description.clone(),
                status.clone(),
                decision_context.clone(),
                makers.clone(),
                alternatives.clone(),
                rationale.clone(),
                consequences.clone(),
            ) {
                Ok(()) => {
                    println!("✅ Decision {} updated successfully!", id);
                    Ok(())
                }
                Err(e) => {
                    context.error_handler.display_error(&e)?;
                    Err(e)
                }
            }
        }
        DecisionSubcommands::Delete { id } => {
            match rhema_core::file_ops::delete_decision(&scope.path, id) {
                Ok(()) => {
                    println!("🗑️  Decision {} deleted successfully!", id);
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
