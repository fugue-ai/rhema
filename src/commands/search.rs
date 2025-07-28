use crate::{Gacp, GacpResult};
use crate::scope::find_nearest_scope;
use colored::*;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(gacp: &Gacp, term: &str, in_file: Option<&str>) -> GacpResult<()> {
    // Get the current working directory to find the nearest scope
    let current_dir = std::env::current_dir()
        .map_err(|e| crate::GacpError::IoError(e))?;
    
    // Discover all scopes
    let scopes = gacp.discover_scopes()?;
    
    // Find the nearest scope to the current directory
    let scope = find_nearest_scope(&current_dir, &scopes)
        .ok_or_else(|| crate::GacpError::ConfigError("No GACP scope found in current directory or parent directories".to_string()))?;
    
    println!("🔍 Searching for '{}' in scope: {}", term.bright_blue(), scope.definition.name);
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
            if let Ok(matches) = search_in_file(path, term) {
                if !matches.is_empty() {
                    found_matches = true;
                    display_search_results(file_name, path, &matches);
                }
            }
        }
    }
    
    if !found_matches {
        println!("❌ No matches found for '{}'", term);
    }
    
    Ok(())
}

fn search_in_file(file_path: &Path, term: &str) -> Result<Vec<SearchMatch>, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    let term_lower = term.to_lowercase();
    let mut matches = Vec::new();
    
    for (line_num, line) in content.lines().enumerate() {
        let line_lower = line.to_lowercase();
        if line_lower.contains(&term_lower) {
            let start_pos = line_lower.find(&term_lower).unwrap();
            let end_pos = start_pos + term.len();
            
            matches.push(SearchMatch {
                line_number: line_num + 1,
                line_content: line.to_string(),
                match_start: start_pos,
                match_end: end_pos,
            });
        }
    }
    
    Ok(matches)
}

fn display_search_results(file_name: &str, file_path: &Path, matches: &[SearchMatch]) {
    println!("📄 File: {} ({})", file_name.bright_blue(), file_path.display().to_string().dimmed());
    println!("🎯 Found {} match(es)", matches.len().to_string().green());
    
    for (i, m) in matches.iter().enumerate() {
        println!("  {}. Line {}: {}", 
            (i + 1).to_string().cyan(),
            m.line_number.to_string().yellow(),
            highlight_match(&m.line_content, m.match_start, m.match_end)
        );
    }
    println!("{}", "─".repeat(80));
}

fn highlight_match(line: &str, start: usize, end: usize) -> String {
    if start >= line.len() || end > line.len() || start >= end {
        return line.to_string();
    }
    
    let before = &line[..start];
    let matched = &line[start..end];
    let after = &line[end..];
    
    format!("{}{}{}", before, matched.bright_red().bold(), after)
}

#[derive(Debug)]
struct SearchMatch {
    line_number: usize,
    line_content: String,
    match_start: usize,
    match_end: usize,
} 