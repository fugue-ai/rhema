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

use crate::{RhemaResult, TodoSubcommands, InsightSubcommands, PatternSubcommands, DecisionSubcommands, GitSubcommands, Priority, TodoStatus, DecisionStatus, PatternUsage};
use colored::*;
// use std::collections::HashMap;

/// Enhanced command parser for interactive mode
#[derive(Debug)]
pub struct InteractiveCommandParser {
    _input: String,
    parts: Vec<String>,
    current_pos: usize,
}

impl InteractiveCommandParser {
    pub fn new(input: &str) -> Self {
        let parts = Self::parse_input(input);
        Self {
            _input: input.to_string(),
            parts,
            current_pos: 0,
        }
    }

    /// Parse input string into parts, handling quoted strings
    fn parse_input(input: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut quote_char = '\0';
        let mut escaped = false;

        for (_i, ch) in input.chars().enumerate() {
            if escaped {
                current.push(ch);
                escaped = false;
                continue;
            }

            if ch == '\\' {
                escaped = true;
                continue;
            }

            if !in_quotes && (ch == '"' || ch == '\'') {
                in_quotes = true;
                quote_char = ch;
                continue;
            }

            if in_quotes && ch == quote_char {
                in_quotes = false;
                continue;
            }

            if !in_quotes && ch.is_whitespace() {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(ch);
            }
        }

        if !current.is_empty() {
            parts.push(current);
        }

        parts
    }

    /// Get the next argument
    pub fn next(&mut self) -> Option<&str> {
        if self.current_pos < self.parts.len() {
            let arg = &self.parts[self.current_pos];
            self.current_pos += 1;
            Some(arg)
        } else {
            None
        }
    }

    /// Peek at the next argument without consuming it
    pub fn peek(&self) -> Option<&str> {
        if self.current_pos < self.parts.len() {
            Some(&self.parts[self.current_pos])
        } else {
            None
        }
    }

    /// Get remaining arguments as a slice
    pub fn remaining(&self) -> &[String] {
        &self.parts[self.current_pos..]
    }

    /// Check if there are more arguments
    pub fn has_more(&self) -> bool {
        self.current_pos < self.parts.len()
    }

    /// Reset position to beginning
    pub fn reset(&mut self) {
        self.current_pos = 0;
    }

    /// Get the command (first argument)
    pub fn command(&self) -> Option<&str> {
        self.parts.first().map(|s| s.as_str())
    }

    /// Get all arguments as strings
    pub fn args(&self) -> Vec<String> {
        self.parts[1..].to_vec()
    }

    /// Parse todo subcommands
    pub fn parse_todo_subcommand(&mut self) -> RhemaResult<TodoSubcommands> {
        let subcommand = self.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("todo requires a subcommand".to_string()))?;

        match subcommand {
            "add" => {
                let title = self.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("todo add requires a title".to_string()))?
                    .to_string();
                
                let mut priority = Priority::Medium;
                let mut assignee = None;
                let mut due_date = None;
                let mut description = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--priority" | "-p" => {
                            let priority_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--priority requires a value".to_string()))?;
                            priority = match priority_str {
                                "low" | "l" => Priority::Low,
                                "medium" | "m" => Priority::Medium,
                                "high" | "h" => Priority::High,
                                "critical" | "c" => Priority::Critical,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid priority: {}", priority_str)
                                )),
                            };
                        }
                        "--assignee" | "-a" => {
                            assignee = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--assignee requires a value".to_string()))?
                                .to_string());
                        }
                        "--due-date" | "-d" => {
                            due_date = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--due-date requires a value".to_string()))?
                                .to_string());
                        }
                        "--description" | "-desc" => {
                            description = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--description requires a value".to_string()))?
                                .to_string());
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(TodoSubcommands::Add {
                    title,
                    description,
                    priority,
                    assignee,
                    due_date,
                })
            }
            "list" | "ls" => {
                let mut status = None;
                let mut priority = None;
                let mut assignee = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--status" | "-s" => {
                            let status_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--status requires a value".to_string()))?;
                            status = Some(match status_str {
                                "todo" | "t" => TodoStatus::Pending,
                                "in_progress" | "ip" => TodoStatus::InProgress,
                                "done" | "d" => TodoStatus::Completed,
                                "cancelled" | "c" => TodoStatus::Cancelled,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid status: {}", status_str)
                                )),
                            });
                        }
                        "--priority" | "-p" => {
                            let priority_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--priority requires a value".to_string()))?;
                            priority = Some(match priority_str {
                                "low" | "l" => Priority::Low,
                                "medium" | "m" => Priority::Medium,
                                "high" | "h" => Priority::High,
                                "critical" | "c" => Priority::Critical,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid priority: {}", priority_str)
                                )),
                            });
                        }
                        "--assignee" | "-a" => {
                            assignee = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--assignee requires a value".to_string()))?
                                .to_string());
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(TodoSubcommands::List {
                    status,
                    priority,
                    assignee,
                })
            }
            "update" | "edit" => {
                let id = self.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("todo update requires an ID".to_string()))?
                    .to_string();
                
                let mut title = None;
                let mut description = None;
                let mut status = None;
                let mut priority = None;
                let mut assignee = None;
                let mut due_date = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--title" | "-t" => {
                            title = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--title requires a value".to_string()))?
                                .to_string());
                        }
                        "--description" | "-desc" => {
                            description = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--description requires a value".to_string()))?
                                .to_string());
                        }
                        "--status" | "-s" => {
                            let status_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--status requires a value".to_string()))?;
                            status = Some(match status_str {
                                "todo" | "t" => TodoStatus::Pending,
                                "in_progress" | "ip" => TodoStatus::InProgress,
                                "done" | "d" => TodoStatus::Completed,
                                "cancelled" | "c" => TodoStatus::Cancelled,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid status: {}", status_str)
                                )),
                            });
                        }
                        "--priority" | "-p" => {
                            let priority_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--priority requires a value".to_string()))?;
                            priority = Some(match priority_str {
                                "low" | "l" => Priority::Low,
                                "medium" | "m" => Priority::Medium,
                                "high" | "h" => Priority::High,
                                "critical" | "c" => Priority::Critical,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid priority: {}", priority_str)
                                )),
                            });
                        }
                        "--assignee" | "-a" => {
                            assignee = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--assignee requires a value".to_string()))?
                                .to_string());
                        }
                        "--due-date" | "-d" => {
                            due_date = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--due-date requires a value".to_string()))?
                                .to_string());
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(TodoSubcommands::Update {
                    id,
                    title,
                    description,
                    status,
                    priority,
                    assignee,
                    due_date,
                })
            }
            "delete" | "rm" => {
                let id = self.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("todo delete requires an ID".to_string()))?
                    .to_string();
                
                Ok(TodoSubcommands::Delete { id })
            }
            "complete" | "done" => {
                let id = self.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("todo complete requires an ID".to_string()))?
                    .to_string();
                
                let outcome = self.next().map(|s| s.to_string());
                
                Ok(TodoSubcommands::Complete { id, outcome })
            }
            _ => {
                Err(crate::error::RhemaError::InvalidCommand(
                    format!("Unknown todo subcommand: {}. Available: add, list, update, delete, complete", subcommand)
                ))
            }
        }
    }

    /// Parse insight subcommands
    pub fn parse_insight_subcommand(&mut self) -> RhemaResult<InsightSubcommands> {
        let subcommand = self.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("insight requires a subcommand".to_string()))?;

        match subcommand {
            "record" | "add" => {
                let title = self.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("insight record requires a title".to_string()))?
                    .to_string();
                
                let mut content = None;
                let mut confidence = None;
                let mut category = None;
                let mut tags = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--content" | "-c" => {
                            content = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--content requires a value".to_string()))?
                                .to_string());
                        }
                        "--confidence" | "-conf" => {
                            let conf_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--confidence requires a value".to_string()))?;
                            confidence = Some(conf_str.parse::<u8>()
                                .map_err(|_| crate::error::RhemaError::InvalidCommand("Confidence must be a number 1-10".to_string()))?);
                        }
                        "--category" | "-cat" => {
                            category = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--category requires a value".to_string()))?
                                .to_string());
                        }
                        "--tags" | "-t" => {
                            tags = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--tags requires a value".to_string()))?
                                .to_string());
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(InsightSubcommands::Record {
                    title,
                    content: content.expect("Content is required"),
                    confidence,
                    category,
                    tags,
                })
            }
            "list" | "ls" => {
                let mut category = None;
                let mut tag = None;
                let mut min_confidence = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--category" | "-cat" => {
                            category = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--category requires a value".to_string()))?
                                .to_string());
                        }
                        "--tag" | "-t" => {
                            tag = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--tag requires a value".to_string()))?
                                .to_string());
                        }
                        "--min-confidence" | "-mc" => {
                            let conf_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--min-confidence requires a value".to_string()))?;
                            min_confidence = Some(conf_str.parse::<u8>()
                                .map_err(|_| crate::error::RhemaError::InvalidCommand("Min confidence must be a number 1-10".to_string()))?);
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(InsightSubcommands::List {
                    category,
                    tag,
                    min_confidence,
                })
            }
            _ => {
                Err(crate::error::RhemaError::InvalidCommand(
                    format!("Unknown insight subcommand: {}. Available: record, list", subcommand)
                ))
            }
        }
    }

    /// Parse pattern subcommands
    pub fn parse_pattern_subcommand(&mut self) -> RhemaResult<PatternSubcommands> {
        let subcommand = self.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("pattern requires a subcommand".to_string()))?;

        match subcommand {
            "add" => {
                let name = self.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("pattern add requires a name".to_string()))?
                    .to_string();
                
                let mut description = None;
                let mut pattern_type = None;
                let mut usage = None;
                let mut effectiveness = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--description" | "-desc" => {
                            description = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--description requires a value".to_string()))?
                                .to_string());
                        }
                        "--type" | "-t" => {
                            pattern_type = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--type requires a value".to_string()))?
                                .to_string());
                        }
                        "--usage" | "-u" => {
                            let usage_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--usage requires a value".to_string()))?;
                            usage = Some(match usage_str {
                                "recommended" | "rec" => PatternUsage::Recommended,
                                "optional" | "opt" => PatternUsage::Optional,
                                "deprecated" | "dep" => PatternUsage::Deprecated,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid usage: {}", usage_str)
                                )),
                            });
                        }
                        "--effectiveness" | "-e" => {
                            let eff_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--effectiveness requires a value".to_string()))?;
                            effectiveness = Some(eff_str.parse::<u8>()
                                .map_err(|_| crate::error::RhemaError::InvalidCommand("Effectiveness must be a number 1-10".to_string()))?);
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(PatternSubcommands::Add {
                    name,
                    description: description.expect("Description is required"),
                    pattern_type: pattern_type.expect("Pattern type is required"),
                    usage: usage.expect("Usage is required"),
                    effectiveness,
                    examples: None,
                    anti_patterns: None,
                })
            }
            "list" | "ls" => {
                let mut pattern_type = None;
                let mut usage = None;
                let mut min_effectiveness = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--type" | "-t" => {
                            pattern_type = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--type requires a value".to_string()))?
                                .to_string());
                        }
                        "--usage" | "-u" => {
                            let usage_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--usage requires a value".to_string()))?;
                            usage = Some(match usage_str {
                                "recommended" | "rec" => PatternUsage::Recommended,
                                "optional" | "opt" => PatternUsage::Optional,
                                "deprecated" | "dep" => PatternUsage::Deprecated,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid usage: {}", usage_str)
                                )),
                            });
                        }
                        "--min-effectiveness" | "-me" => {
                            let eff_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--min-effectiveness requires a value".to_string()))?;
                            min_effectiveness = Some(eff_str.parse::<u8>()
                                .map_err(|_| crate::error::RhemaError::InvalidCommand("Min effectiveness must be a number 1-10".to_string()))?);
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(PatternSubcommands::List {
                    pattern_type,
                    usage,
                    min_effectiveness,
                })
            }
            _ => {
                Err(crate::error::RhemaError::InvalidCommand(
                    format!("Unknown pattern subcommand: {}. Available: add, list", subcommand)
                ))
            }
        }
    }

    /// Parse decision subcommands
    pub fn parse_decision_subcommand(&mut self) -> RhemaResult<DecisionSubcommands> {
        let subcommand = self.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("decision requires a subcommand".to_string()))?;

        match subcommand {
            "record" | "add" => {
                let title = self.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("decision record requires a title".to_string()))?
                    .to_string();
                
                let mut description = None;
                let mut status = None;
                let mut maker = None;
                let mut rationale = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--description" | "-desc" => {
                            description = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--description requires a value".to_string()))?
                                .to_string());
                        }
                        "--status" | "-s" => {
                            let status_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--status requires a value".to_string()))?;
                            status = Some(match status_str {
                                "proposed" | "prop" => DecisionStatus::Proposed,
                                "approved" | "app" => DecisionStatus::Approved,
                                "rejected" | "rej" => DecisionStatus::Rejected,
                                "superseded" | "sup" => DecisionStatus::Deprecated,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid status: {}", status_str)
                                )),
                            });
                        }
                        "--maker" | "-m" => {
                            maker = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--maker requires a value".to_string()))?
                                .to_string());
                        }
                        "--rationale" | "-r" => {
                            rationale = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--rationale requires a value".to_string()))?
                                .to_string());
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(DecisionSubcommands::Record {
                    title,
                    description: description.expect("Description is required"),
                    status: status.expect("Status is required"),
                    makers: maker,
                    rationale,
                    context: None,
                    alternatives: None,
                    consequences: None,
                })
            }
            "list" | "ls" => {
                let mut status = None;
                let mut maker = None;

                while let Some(arg) = self.next() {
                    match arg {
                        "--status" | "-s" => {
                            let status_str = self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--status requires a value".to_string()))?;
                            status = Some(match status_str {
                                "proposed" | "prop" => DecisionStatus::Proposed,
                                "approved" | "app" => DecisionStatus::Approved,
                                "rejected" | "rej" => DecisionStatus::Rejected,
                                "superseded" | "sup" => DecisionStatus::Deprecated,
                                _ => return Err(crate::error::RhemaError::InvalidCommand(
                                    format!("Invalid status: {}", status_str)
                                )),
                            });
                        }
                        "--maker" | "-m" => {
                            maker = Some(self.next()
                                .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--maker requires a value".to_string()))?
                                .to_string());
                        }
                        _ => {
                            return Err(crate::error::RhemaError::InvalidCommand(
                                format!("Unknown argument: {}", arg)
                            ));
                        }
                    }
                }

                Ok(DecisionSubcommands::List {
                    status,
                    maker,
                })
            }
            _ => {
                Err(crate::error::RhemaError::InvalidCommand(
                    format!("Unknown decision subcommand: {}. Available: record, list", subcommand)
                ))
            }
        }
    }

    /// Parse git subcommands
    pub fn parse_git_subcommand(&mut self) -> RhemaResult<GitSubcommands> {
        let subcommand = self.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("git requires a subcommand".to_string()))?;

        match subcommand {
            "status" => Ok(GitSubcommands::Status),
            _ => {
                Err(crate::error::RhemaError::InvalidCommand(
                    format!("Unknown git subcommand: {}. Available: status", subcommand)
                ))
            }
        }
    }

    /// Show helpful error message with suggestions
    pub fn show_error_with_suggestions(&self, error: &str) {
        println!("{}", error.red().bold());
        
        if let Some(cmd) = self.command() {
            match cmd {
                "todo" => {
                    println!("{}", "Available todo subcommands:".yellow());
                    println!("  add <title> [--priority <level>] [--assignee <name>] [--due-date <date>]");
                    println!("  list [--status <status>] [--priority <level>] [--assignee <name>]");
                    println!("  update <id> [--title <title>] [--status <status>] [--priority <level>]");
                    println!("  delete <id>");
                    println!("  complete <id> [--outcome <description>]");
                }
                "insight" => {
                    println!("{}", "Available insight subcommands:".yellow());
                    println!("  record <title> [--content <content>] [--confidence <1-10>] [--category <category>] [--tags <tags>]");
                    println!("  list [--category <category>] [--tag <tag>] [--min-confidence <1-10>]");
                }
                "pattern" => {
                    println!("{}", "Available pattern subcommands:".yellow());
                    println!("  add <name> [--description <desc>] [--type <type>] [--usage <usage>] [--effectiveness <1-10>]");
                    println!("  list [--type <type>] [--usage <usage>] [--min-effectiveness <1-10>]");
                }
                "decision" => {
                    println!("{}", "Available decision subcommands:".yellow());
                    println!("  record <title> [--description <desc>] [--status <status>] [--maker <name>] [--rationale <rationale>]");
                    println!("  list [--status <status>] [--maker <name>]");
                }
                "git" => {
                    println!("{}", "Available git subcommands:".yellow());
                    println!("  status");
                    println!("  commit <message>");
                    println!("  push");
                    println!("  pull");
                    println!("  branch <name>");
                }
                _ => {
                    println!("{}", "Type 'help' for a list of all available commands".yellow());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_basic() {
        let parser = InteractiveCommandParser::new("todo add \"Implement user auth\" --priority high");
        assert_eq!(parser.parts, vec!["todo", "add", "Implement user auth", "--priority", "high"]);
    }

    #[test]
    fn test_parse_input_with_quotes() {
        let parser = InteractiveCommandParser::new("todo add \"Implement user authentication\" --description \"Add OAuth2 support\"");
        assert_eq!(parser.parts, vec!["todo", "add", "Implement user authentication", "--description", "Add OAuth2 support"]);
    }

    #[test]
    fn test_parse_input_with_escaped_chars() {
        let parser = InteractiveCommandParser::new("todo add \"Fix \\\"quoted\\\" text\"");
        assert_eq!(parser.parts, vec!["todo", "add", "Fix \"quoted\" text"]);
    }

    #[test]
    fn test_todo_add_parsing() {
        let mut parser = InteractiveCommandParser::new("add \"Test todo\" --priority high --assignee john");
        let result = parser.parse_todo_subcommand().unwrap();
        
        if let TodoSubcommands::Add { title, priority, assignee, .. } = result {
            assert_eq!(title, "Test todo");
            assert_eq!(priority, Priority::High);
            assert_eq!(assignee, Some("john".to_string()));
        } else {
            panic!("Expected Add subcommand");
        }
    }

    #[test]
    fn test_insight_record_parsing() {
        let mut parser = InteractiveCommandParser::new("record \"Database optimization\" --content \"Optimized queries for better performance\" --confidence 8 --category performance");
        let result = parser.parse_insight_subcommand().unwrap();
        
        if let InsightSubcommands::Record { title, content, confidence, category, .. } = result {
            assert_eq!(title, "Database optimization");
            assert_eq!(content, "Optimized queries for better performance");
            assert_eq!(confidence, Some(8));
            assert_eq!(category, Some("performance".to_string()));
        } else {
            panic!("Expected Record subcommand");
        }
    }
} 