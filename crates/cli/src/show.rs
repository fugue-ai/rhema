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

use crate::scope::{find_nearest_scope, get_scope};
use crate::{Rhema, RhemaResult};
use colored::*;
use serde_yaml;
use std::path::Path;

pub fn run(rhema: &Rhema, file: &str, scope: Option<&str>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_path) = scope {
        // Use the specified scope
        let scope_obj = get_scope(rhema.repo_root(), scope_path)?;
        scope_obj.path
    } else {
        // Find the nearest scope to the current directory
        let current_dir = std::env::current_dir().map_err(|e| crate::RhemaError::IoError(e))?;

        let scopes = rhema.discover_scopes()?;
        let scope_obj = find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
            crate::RhemaError::ConfigError(
                "No Rhema scope found in current directory or parent directories".to_string(),
            )
        })?;
        scope_obj.path.clone()
    };

    // Construct the file path
    let file_path = if file.ends_with(".yaml") {
        scope_path.join(file)
    } else {
        scope_path.join(format!("{}.yaml", file))
    };

    if !file_path.exists() {
        return Err(crate::RhemaError::FileNotFound(format!(
            "File not found: {}",
            file_path.display()
        )));
    }

    // Read and display the file content
    let content = std::fs::read_to_string(&file_path).map_err(|e| crate::RhemaError::IoError(e))?;

    // Try to parse as YAML for pretty formatting
    match serde_yaml::from_str::<serde_yaml::Value>(&content) {
        Ok(yaml_value) => {
            // Pretty print the YAML
            let pretty_content =
                serde_yaml::to_string(&yaml_value).map_err(|e| crate::RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;

            display_file_content(&file_path, &pretty_content, true);
        }
        Err(_) => {
            // If it's not valid YAML, display as plain text
            display_file_content(&file_path, &content, false);
        }
    }

    Ok(())
}

fn display_file_content(file_path: &Path, content: &str, is_yaml: bool) {
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    println!("📄 File: {}", file_name.bright_blue());
    println!("📍 Path: {}", file_path.display().to_string().dimmed());
    if is_yaml {
        println!("📋 Type: {}", "YAML".green());
    } else {
        println!("📋 Type: {}", "Text".yellow());
    }
    println!("{}", "─".repeat(80));

    // Simple syntax highlighting for YAML
    if is_yaml {
        for line in content.lines() {
            let highlighted_line = highlight_yaml_line(line);
            println!("{}", highlighted_line);
        }
    } else {
        println!("{}", content);
    }

    println!("{}", "─".repeat(80));
}

fn highlight_yaml_line(line: &str) -> String {
    let mut highlighted = String::new();
    let mut in_quotes = false;
    let mut in_key = true;
    let mut current_word = String::new();

    for (i, ch) in line.chars().enumerate() {
        match ch {
            '"' | '\'' => {
                if !in_quotes {
                    in_quotes = true;
                    highlighted.push_str(&format!("{}", ch.to_string().yellow()));
                } else {
                    in_quotes = false;
                    highlighted.push_str(&format!("{}", ch.to_string().yellow()));
                }
            }
            ':' if !in_quotes && in_key => {
                highlighted.push_str(&format!("{}", current_word.green()));
                highlighted.push_str(&format!("{}", ch.to_string().white()));
                current_word.clear();
                in_key = false;
            }
            '#' if !in_quotes => {
                highlighted.push_str(&current_word);
                highlighted.push_str(&format!("{}", ch.to_string().bright_black()));
                highlighted.push_str(&line[i + 1..].bright_black());
                break;
            }
            ' ' | '\t' => {
                if !current_word.is_empty() {
                    if in_key {
                        highlighted.push_str(&current_word.green());
                    } else {
                        // Try to detect value types
                        let colored_word = if current_word == "true" || current_word == "false" {
                            current_word.blue()
                        } else if current_word.parse::<f64>().is_ok() {
                            current_word.cyan()
                        } else if current_word.starts_with('-') {
                            current_word.magenta()
                        } else {
                            current_word.white()
                        };
                        highlighted.push_str(&colored_word);
                    }
                    current_word.clear();
                }
                highlighted.push(ch);
            }
            _ => {
                if in_quotes {
                    highlighted.push_str(&ch.to_string().yellow());
                } else {
                    current_word.push(ch);
                }
            }
        }
    }

    // Handle any remaining word
    if !current_word.is_empty() {
        if in_key {
            highlighted.push_str(&current_word.green());
        } else {
            let colored_word = if current_word == "true" || current_word == "false" {
                current_word.blue()
            } else if current_word.parse::<f64>().is_ok() {
                current_word.cyan()
            } else if current_word.starts_with('-') {
                current_word.magenta()
            } else {
                current_word.white()
            };
            highlighted.push_str(&colored_word);
        }
    }

    highlighted
}
