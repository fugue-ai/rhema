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
use crate::scope::find_nearest_scope;
use colored::*;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(rhema: &Rhema, term: &str, in_file: Option<&str>, regex: bool) -> RhemaResult<()> {
    // Get the current working directory to find the nearest scope
    let current_dir = std::env::current_dir()
        .map_err(|e| crate::RhemaError::IoError(e))?;
    
    // Discover all scopes
    let scopes = rhema.discover_scopes()?;
    
    // Find the nearest scope to the current directory
    let scope = find_nearest_scope(&current_dir, &scopes)
        .ok_or_else(|| crate::RhemaError::ConfigError("No Rhema scope found in current directory or parent directories".to_string()))?;
    
    let search_type = if regex { "regex pattern" } else { "term" };
    println!("🔍 Searching for '{}' ({}) in scope: {}", term.bright_blue(), search_type, scope.definition.name);
    println!("{}", "─".repeat(80));
    
    let mut found_matches = false;
    
    // Search through all YAML files in the scope
    for entry in WalkDir::new(&scope.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            // Skip if we're filtering by specific file and this doesn't match
            if let Some(target_file) = in_file {
                if !file_name.contains(target_file) {
                    continue;
                }
            }
            
            // Search in this file
            let matches = if regex {
                search_in_file_regex(path, term)?
            } else {
                search_in_file(path, term)?
            };
            
            if !matches.is_empty() {
                found_matches = true;
                display_search_results(file_name, path, &matches);
            }
        }
    }
    
    if !found_matches {
        println!("❌ No matches found for '{}'", term);
    }
    
    Ok(())
}

/// Search for text in a file
fn search_in_file(file_path: &Path, term: &str) -> RhemaResult<Vec<String>> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::RhemaError::IoError(e))?;
    
    let mut matches = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for (line_num, line) in lines.iter().enumerate() {
        if line.to_lowercase().contains(&term.to_lowercase()) {
            matches.push(format!("Line {}: {}", line_num + 1, line.trim()));
        }
    }
    
    Ok(matches)
}

/// Search for regex pattern in a file
fn search_in_file_regex(file_path: &Path, pattern: &str) -> RhemaResult<Vec<String>> {
    let regex = regex::Regex::new(pattern)
        .map_err(|_| crate::RhemaError::InvalidQuery(format!("Invalid regex pattern: {}", pattern)))?;
    
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::RhemaError::IoError(e))?;
    
    let mut matches = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for (line_num, line) in lines.iter().enumerate() {
        if regex.is_match(line) {
            matches.push(format!("Line {}: {}", line_num + 1, line.trim()));
        }
    }
    
    Ok(matches)
}

/// Display search results
fn display_search_results(file_name: &str, file_path: &Path, matches: &[String]) {
    println!("📄 {} ({})", file_name.bright_green(), file_path.display());
    for match_line in matches {
        println!("  {}", match_line);
    }
    println!();
} 
