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

use crate::{Rhema, RhemaResult, scope::build_dependency_graph};
use colored::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run(rhema: &Rhema) -> RhemaResult<()> {
    println!("{}", "🔗 Analyzing scope dependencies...".blue().bold());
    
    // Discover all scopes
    let scopes = rhema.discover_scopes()?;
    
    if scopes.is_empty() {
        println!("{}", "No scopes found in the repository.".yellow());
        return Ok(());
    }
    
    // Build dependency graph
    // TODO: Use lock file for deterministic dependency graph generation
    let dependency_graph = build_dependency_graph(&scopes)?;
    
    // Display dependency graph
    display_dependency_graph(rhema, &scopes, &dependency_graph)?;
    
    // Check for circular dependencies
    check_circular_dependencies(&dependency_graph)?;
    
    // Analyze dependency impact
    analyze_dependency_impact(rhema, &scopes, &dependency_graph)?;
    
    Ok(())
}

fn display_dependency_graph(
    rhema: &Rhema,
    scopes: &[crate::scope::Scope],
    dependency_graph: &HashMap<String, Vec<String>>,
) -> RhemaResult<()> {
    println!("\n{}", "📊 Scope Dependency Graph".green().bold());
    println!("{}", "=".repeat(50));
    
    for scope in scopes {
        let scope_path = scope.relative_path(rhema.repo_root())?;
        let empty_vec = Vec::new();
        let dependencies = dependency_graph.get(&scope_path).unwrap_or(&empty_vec);
        
        println!("{}", format!("📁 {}", scope_path).cyan().bold());
        
        if dependencies.is_empty() {
            println!("  └── {} (no dependencies)", "🟢 Independent".green());
        } else {
            for (i, dep) in dependencies.iter().enumerate() {
                let is_last = i == dependencies.len() - 1;
                let prefix = if is_last { "  └── " } else { "  ├── " };
                println!("{}{}", prefix, format!("📦 {}", dep).yellow());
            }
        }
        println!();
    }
    Ok(())
}

fn check_circular_dependencies(
    dependency_graph: &HashMap<String, Vec<String>>,
) -> RhemaResult<()> {
    println!("{}", "🔄 Checking for circular dependencies...".blue().bold());
    
    let mut circular_deps = Vec::new();
    
    for node in dependency_graph.keys() {
        if has_circular_dependency(dependency_graph, node) {
            circular_deps.push(node.clone());
        }
    }
    
    if circular_deps.is_empty() {
        println!("{}", "✅ No circular dependencies found!".green());
    } else {
        println!("{}", "⚠️  Circular dependencies detected:".red().bold());
        for dep in circular_deps {
            println!("  🔴 {}", dep);
        }
    }
    
    println!();
    Ok(())
}

fn has_circular_dependency(
    graph: &HashMap<String, Vec<String>>,
    start_node: &str,
) -> bool {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    
    fn dfs(
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true; // Back edge found - cycle detected
        }
        
        if visited.contains(node) {
            return false; // Already processed
        }
        
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if dfs(graph, neighbor, visited, rec_stack) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(node);
        false
    }
    
    dfs(graph, start_node, &mut visited, &mut rec_stack)
}

fn analyze_dependency_impact(
    rhema: &Rhema,
    scopes: &[crate::scope::Scope],
    dependency_graph: &HashMap<String, Vec<String>>,
) -> RhemaResult<()> {
    println!("{}", "📈 Dependency Impact Analysis".green().bold());
    println!("{}", "=".repeat(50));
    
    // Calculate dependency depth for each scope
    let mut dependency_depths = HashMap::new();
    
    for scope in scopes {
        let scope_path = scope.relative_path(rhema.repo_root())?;
        let depth = calculate_dependency_depth(dependency_graph, &scope_path);
        dependency_depths.insert(scope_path, depth);
    }
    
    // Find scopes with highest dependency impact
    let mut high_impact_scopes: Vec<_> = dependency_depths.iter().collect();
    high_impact_scopes.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("{}", "🏆 High Impact Scopes (most dependencies):".yellow().bold());
    for (scope_path, depth) in high_impact_scopes.iter().take(5) {
        let impact_level = if **depth > 5 {
            "🔴 Critical"
        } else if **depth > 3 {
            "🟡 High"
        } else if **depth > 1 {
            "🟢 Medium"
        } else {
            "🔵 Low"
        };
        
        println!("  {} {} ({} dependencies)", impact_level, scope_path, depth);
    }
    
    println!();
    
    // Find independent scopes
    let independent_scopes: Vec<_> = dependency_depths
        .iter()
        .filter(|(_, depth)| **depth == 0)
        .collect();
    
    if !independent_scopes.is_empty() {
        println!("{}", "🟢 Independent Scopes (no dependencies):".green().bold());
        for (scope_path, _) in independent_scopes {
            println!("  📦 {}", scope_path);
        }
        println!();
    }
    
    // Analyze dependency chains
    analyze_dependency_chains(dependency_graph)?;
    
    Ok(())
}

fn calculate_dependency_depth(
    graph: &HashMap<String, Vec<String>>,
    scope_path: &str,
) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut depths = HashMap::new();
    
    queue.push_back((scope_path.to_string(), 0));
    depths.insert(scope_path.to_string(), 0);
    
    while let Some((current, depth)) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    let new_depth = depth + 1;
                    depths.insert(neighbor.clone(), new_depth);
                    queue.push_back((neighbor.clone(), new_depth));
                }
            }
        }
    }
    
    depths.values().max().copied().unwrap_or(0)
}

fn analyze_dependency_chains(
    dependency_graph: &HashMap<String, Vec<String>>,
) -> RhemaResult<()> {
    println!("{}", "⛓️  Longest Dependency Chains:".blue().bold());
    
    let mut longest_chains = Vec::new();
    
    for start_node in dependency_graph.keys() {
        let chain = find_longest_chain(dependency_graph, start_node);
        if chain.len() > 1 {
            longest_chains.push(chain);
        }
    }
    
    longest_chains.sort_by(|a, b| b.len().cmp(&a.len()));
    
    for (i, chain) in longest_chains.iter().take(3).enumerate() {
        println!("  {}. Chain length {}: {}", i + 1, chain.len(), chain.join(" → "));
    }
    
    println!();
    Ok(())
}

fn find_longest_chain(
    graph: &HashMap<String, Vec<String>>,
    start_node: &str,
) -> Vec<String> {
    let _visited: HashSet<String> = HashSet::new();
    let mut longest_chain = Vec::new();
    
    fn dfs_longest(
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        current_chain: &mut Vec<String>,
        longest_chain: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(node) {
            return;
        }
        
        visited.insert(node.to_string());
        current_chain.push(node.to_string());
        
        if current_chain.len() > longest_chain.len() {
            *longest_chain = current_chain.clone();
        }
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                dfs_longest(graph, neighbor, current_chain, longest_chain, visited);
            }
        }
        
        current_chain.pop();
        visited.remove(node);
    }
    
    dfs_longest(graph, start_node, &mut Vec::new(), &mut longest_chain, &mut HashSet::new());
    longest_chain
} 
