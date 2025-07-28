use crate::{Gacp, GacpResult};
use colored::*;
use std::collections::HashMap;

pub fn run(gacp: &Gacp) -> GacpResult<()> {
    let scopes = gacp.discover_scopes()?;
    
    if scopes.is_empty() {
        println!("{}", "No GACP scopes found in this repository.".yellow());
        println!("  Run 'gacp init' to create your first scope.");
        return Ok(());
    }
    
    println!("{}", "GACP Scopes:".bold());
    println!();
    
    for scope in &scopes {
        let relative_path = scope.relative_path(gacp.repo_root())?;
        println!("  {} ({})", 
            scope.definition.name.yellow(), 
            scope.definition.scope_type.cyan()
        );
        println!("    Path: {}", relative_path);
        if let Some(desc) = &scope.definition.description {
            println!("    Description: {}", desc);
        }
        println!("    Files: {}", scope.files.keys().cloned().collect::<Vec<_>>().join(", "));
        println!();
    }
    
    Ok(())
}

pub fn show_scope(gacp: &Gacp, path: Option<&str>) -> GacpResult<()> {
    let scopes = gacp.discover_scopes()?;
    
    if let Some(path) = path {
        // Show specific scope
        let scope = gacp.get_scope(path)?;
        display_scope_details(&scope, gacp.repo_root())?;
    } else {
        // Show current scope or all scopes
        let current_dir = std::env::current_dir()?;
        let current_scope = crate::scope::find_nearest_scope(&current_dir, &scopes);
        
        if let Some(scope) = current_scope {
            display_scope_details(scope, gacp.repo_root())?;
        } else {
            println!("{}", "No GACP scope found in current directory.".yellow());
            println!("  Available scopes:");
            for scope in &scopes {
                let relative_path = scope.relative_path(gacp.repo_root())?;
                println!("    • {}", relative_path);
            }
        }
    }
    
    Ok(())
}

pub fn show_tree(gacp: &Gacp) -> GacpResult<()> {
    let scopes = gacp.discover_scopes()?;
    let hierarchy = crate::scope::get_scope_hierarchy(&scopes, gacp.repo_root())?;
    
    if scopes.is_empty() {
        println!("{}", "No GACP scopes found in this repository.".yellow());
        return Ok(());
    }
    
    println!("{}", "GACP Scope Hierarchy:".bold());
    println!();
    
    // Find root scopes (those with no parent)
    let mut root_scopes = Vec::new();
    for scope in &scopes {
        let scope_rel_path = scope.relative_path(gacp.repo_root())?;
        let scope_dir = scope.path.parent().unwrap();
        
        let mut has_parent = false;
        for other_scope in &scopes {
            if other_scope.path != scope.path {
                let other_dir = other_scope.path.parent().unwrap();
                if scope_dir.starts_with(other_dir) && scope_dir != other_dir {
                    has_parent = true;
                    break;
                }
            }
        }
        
        if !has_parent {
            root_scopes.push(scope_rel_path);
        }
    }
    
    // Display tree starting from root scopes
    for root_scope in root_scopes {
        display_scope_tree(&root_scope, &hierarchy, 0)?;
    }
    
    Ok(())
}

fn display_scope_details(scope: &crate::Scope, repo_root: &std::path::Path) -> GacpResult<()> {
    let relative_path = scope.relative_path(repo_root)?;
    
    println!("{}", "Scope Details:".bold());
    println!("  Name: {}", scope.definition.name.yellow());
    println!("  Type: {}", scope.definition.scope_type.cyan());
    println!("  Path: {}", relative_path);
    println!("  Version: {}", scope.definition.version);
    
    if let Some(desc) = &scope.definition.description {
        println!("  Description: {}", desc);
    }
    
    if let Some(deps) = &scope.definition.dependencies {
        println!("  Dependencies:");
        for dep in deps {
            println!("    • {} ({})", dep.path, dep.dependency_type);
        }
    }
    
    println!("  Files:");
    for (filename, file_path) in &scope.files {
        println!("    • {} ({})", filename, file_path.display());
    }
    
    Ok(())
}

fn display_scope_tree(
    scope_path: &str, 
    hierarchy: &HashMap<String, Vec<String>>, 
    depth: usize
) -> GacpResult<()> {
    let indent = "  ".repeat(depth);
    let prefix = if depth == 0 { "└── " } else { "├── " };
    
    println!("{}{}{}", indent, prefix, scope_path.yellow());
    
    if let Some(children) = hierarchy.get(scope_path) {
        for (i, child) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            let child_prefix = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", indent, child_prefix, child.cyan());
            
            // Recursively display children
            display_scope_tree(child, hierarchy, depth + 1)?;
        }
    }
    
    Ok(())
} 