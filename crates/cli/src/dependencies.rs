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

use crate::{scope::build_dependency_graph, Rhema, RhemaResult};
use colored::*;
use std::collections::{HashMap, HashSet, VecDeque};
use rhema_core::schema::RhemaLock;
use serde_json;
use serde_yaml;

pub fn run(
    rhema: &Rhema,
    lock_file: bool,
    compare: bool,
    visualize: bool,
    conflicts: bool,
    impact: bool,
    format: &str,
) -> RhemaResult<()> {
    println!("{}", "🔗 Analyzing scope dependencies...".blue().bold());

    // Discover all scopes
    let scopes = rhema.discover_scopes()?;

    if scopes.is_empty() {
        println!("{}", "No scopes found in the repository.".yellow());
        return Ok(());
    }

    // Load lock file if requested
    let lock_data = if lock_file || compare {
        load_lock_file(rhema)?
    } else {
        None
    };

    // Build dependency graphs
    let current_graph = build_dependency_graph(&scopes)?;
    let lock_graph = if let Some(ref lock) = lock_data {
        build_lock_dependency_graph(lock)?
    } else {
        HashMap::new()
    };

    // Determine which graph to use for analysis
    let analysis_graph = if lock_file {
        &lock_graph
    } else {
        &current_graph
    };

    // Generate analysis results
    let mut results = DependencyAnalysisResults {
        scopes: scopes.clone(),
        current_graph,
        lock_graph,
        lock_data,
        dependency_depths: HashMap::new(),
        circular_dependencies: Vec::new(),
        version_conflicts: Vec::new(),
        longest_chains: Vec::new(),
        high_impact_scopes: Vec::new(),
        independent_scopes: Vec::new(),
        differences: Vec::new(),
    };

    // Perform analysis based on options
    if compare {
        compare_dependency_states(&mut results)?;
    }

    if conflicts {
        detect_version_conflicts(&mut results)?;
    }

    if impact {
        analyze_dependency_impact_enhanced(&mut results)?;
    }

    // Display results based on format
    match format {
        "json" => output_json(&results)?,
        "yaml" => output_yaml(&results)?,
        _ => output_text(&results, visualize)?,
    }

    Ok(())
}

fn display_dependency_graph(
    rhema: &Rhema,
    scopes: &[rhema_core::scope::Scope],
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
                let prefix = if is_last {
                    "  └── "
                } else {
                    "  ├── "
                };
                println!("{}{}", prefix, format!("📦 {}", dep).yellow());
            }
        }
        println!();
    }
    Ok(())
}

fn check_circular_dependencies(dependency_graph: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
    println!(
        "{}",
        "🔄 Checking for circular dependencies...".blue().bold()
    );

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

fn has_circular_dependency(graph: &HashMap<String, Vec<String>>, start_node: &str) -> bool {
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
    scopes: &[rhema_core::scope::Scope],
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

    println!(
        "{}",
        "🏆 High Impact Scopes (most dependencies):".yellow().bold()
    );
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
        println!(
            "{}",
            "🟢 Independent Scopes (no dependencies):".green().bold()
        );
        for (scope_path, _) in independent_scopes {
            println!("  📦 {}", scope_path);
        }
        println!();
    }

    // Analyze dependency chains
    analyze_dependency_chains(dependency_graph)?;

    Ok(())
}

fn calculate_dependency_depth(graph: &HashMap<String, Vec<String>>, scope_path: &str) -> usize {
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

fn analyze_dependency_chains(dependency_graph: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
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
        println!(
            "  {}. Chain length {}: {}",
            i + 1,
            chain.len(),
            chain.join(" → ")
        );
    }

    println!();
    Ok(())
}

fn find_longest_chain(graph: &HashMap<String, Vec<String>>, start_node: &str) -> Vec<String> {
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

    dfs_longest(
        graph,
        start_node,
        &mut Vec::new(),
        &mut longest_chain,
        &mut HashSet::new(),
    );
    longest_chain
}

/// Results structure for dependency analysis
#[derive(Debug, Clone)]
struct DependencyAnalysisResults {
    scopes: Vec<rhema_core::scope::Scope>,
    current_graph: HashMap<String, Vec<String>>,
    lock_graph: HashMap<String, Vec<String>>,
    lock_data: Option<RhemaLock>,
    dependency_depths: HashMap<String, usize>,
    circular_dependencies: Vec<Vec<String>>,
    version_conflicts: Vec<VersionConflict>,
    longest_chains: Vec<Vec<String>>,
    high_impact_scopes: Vec<(String, usize)>,
    independent_scopes: Vec<String>,
    differences: Vec<DependencyDifference>,
}

/// Version conflict information
#[derive(Debug, Clone, serde::Serialize)]
struct VersionConflict {
    scope: String,
    dependency: String,
    expected_version: String,
    actual_version: String,
    conflict_type: ConflictType,
}

#[derive(Debug, Clone, serde::Serialize)]
enum ConflictType {
    VersionMismatch,
    MissingDependency,
    ExtraDependency,
    TypeMismatch,
}

/// Dependency difference between lock file and current state
#[derive(Debug, Clone, serde::Serialize)]
struct DependencyDifference {
    scope: String,
    difference_type: DifferenceType,
    details: String,
}

#[derive(Debug, Clone, serde::Serialize)]
enum DifferenceType {
    Added,
    Removed,
    Modified,
    VersionChanged,
}

/// Load lock file from repository
fn load_lock_file(rhema: &Rhema) -> RhemaResult<Option<RhemaLock>> {
    let lock_file_path = rhema.repo_root.join("rhema.lock");
    
    if !lock_file_path.exists() {
        println!("{}", "⚠️  No lock file found. Using current state only.".yellow());
        return Ok(None);
    }

    let lock_content = std::fs::read_to_string(&lock_file_path)
        .map_err(|e| crate::RhemaError::IoError(e))?;

    let lock_file: RhemaLock = serde_yaml::from_str(&lock_content)
        .map_err(|e| crate::RhemaError::InvalidYaml {
            file: lock_file_path.display().to_string(),
            message: e.to_string(),
        })?;

    println!("{}", "🔒 Loaded lock file for analysis".green());
    Ok(Some(lock_file))
}

/// Build dependency graph from lock file data
fn build_lock_dependency_graph(lock_file: &RhemaLock) -> RhemaResult<HashMap<String, Vec<String>>> {
    let mut graph = HashMap::new();

    for (scope_path, locked_scope) in &lock_file.scopes {
        let mut dependencies = Vec::new();
        for (dep_path, _) in &locked_scope.dependencies {
            dependencies.push(dep_path.clone());
        }
        graph.insert(scope_path.clone(), dependencies);
    }

    Ok(graph)
}

/// Compare dependency states between lock file and current state
fn compare_dependency_states(results: &mut DependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "🔄 Comparing lock file with current state...".blue().bold());

    if results.lock_data.is_none() {
        println!("{}", "⚠️  No lock file available for comparison".yellow());
        return Ok(());
    }

    let mut differences = Vec::new();

    // Compare scopes
    let current_scopes: HashSet<String> = results.current_graph.keys().cloned().collect();
    let lock_scopes: HashSet<String> = results.lock_graph.keys().cloned().collect();

    // Find added scopes
    for scope in &current_scopes {
        if !lock_scopes.contains(scope) {
            differences.push(DependencyDifference {
                scope: scope.clone(),
                difference_type: DifferenceType::Added,
                details: "Scope added to current state".to_string(),
            });
        }
    }

    // Find removed scopes
    for scope in &lock_scopes {
        if !current_scopes.contains(scope) {
            differences.push(DependencyDifference {
                scope: scope.clone(),
                difference_type: DifferenceType::Removed,
                details: "Scope removed from current state".to_string(),
            });
        }
    }

    // Compare dependencies for common scopes
    for scope in &current_scopes {
        if lock_scopes.contains(scope) {
            let current_deps: HashSet<String> = results.current_graph
                .get(scope)
                .unwrap_or(&Vec::new())
                .iter()
                .cloned()
                .collect();
            
            let lock_deps: HashSet<String> = results.lock_graph
                .get(scope)
                .unwrap_or(&Vec::new())
                .iter()
                .cloned()
                .collect();

            // Find added dependencies
            for dep in &current_deps {
                if !lock_deps.contains(dep) {
                    differences.push(DependencyDifference {
                        scope: scope.clone(),
                        difference_type: DifferenceType::Added,
                        details: format!("Dependency '{}' added", dep),
                    });
                }
            }

            // Find removed dependencies
            for dep in &lock_deps {
                if !current_deps.contains(dep) {
                    differences.push(DependencyDifference {
                        scope: scope.clone(),
                        difference_type: DifferenceType::Removed,
                        details: format!("Dependency '{}' removed", dep),
                    });
                }
            }
        }
    }

    results.differences = differences;

    if results.differences.is_empty() {
        println!("{}", "✅ Lock file and current state are identical".green());
    } else {
        println!("{}", format!("⚠️  Found {} differences", results.differences.len()).yellow());
    }

    Ok(())
}

/// Detect version conflicts in dependencies
fn detect_version_conflicts(results: &mut DependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "🔍 Detecting version conflicts...".blue().bold());

    if results.lock_data.is_none() {
        println!("{}", "⚠️  No lock file available for version conflict detection".yellow());
        return Ok(());
    }

    let lock_file = results.lock_data.as_ref().unwrap();
    let mut conflicts = Vec::new();

    for (scope_path, locked_scope) in &lock_file.scopes {
        for (dep_path, locked_dep) in &locked_scope.dependencies {
            // Check if dependency exists in current state
            if let Some(current_deps) = results.current_graph.get(scope_path) {
                if current_deps.contains(dep_path) {
                    // Check version consistency
                    if let Some(current_scope) = results.scopes.iter().find(|s| {
                        s.relative_path(&std::env::current_dir().unwrap_or_default()).unwrap_or_default() == *scope_path
                    }) {
                        // This is a simplified version check - in a real implementation,
                        // you would compare actual version constraints
                        if locked_dep.version != "current" {
                            conflicts.push(VersionConflict {
                                scope: scope_path.clone(),
                                dependency: dep_path.clone(),
                                expected_version: locked_dep.version.clone(),
                                actual_version: "current".to_string(),
                                conflict_type: ConflictType::VersionMismatch,
                            });
                        }
                    }
                } else {
                    conflicts.push(VersionConflict {
                        scope: scope_path.clone(),
                        dependency: dep_path.clone(),
                        expected_version: locked_dep.version.clone(),
                        actual_version: "missing".to_string(),
                        conflict_type: ConflictType::MissingDependency,
                    });
                }
            } else {
                conflicts.push(VersionConflict {
                    scope: scope_path.clone(),
                    dependency: dep_path.clone(),
                    expected_version: locked_dep.version.clone(),
                    actual_version: "scope_missing".to_string(),
                    conflict_type: ConflictType::MissingDependency,
                });
            }
        }
    }

    results.version_conflicts = conflicts;

    if results.version_conflicts.is_empty() {
        println!("{}", "✅ No version conflicts detected".green());
    } else {
        println!("{}", format!("⚠️  Found {} version conflicts", results.version_conflicts.len()).red());
    }

    Ok(())
}

/// Enhanced dependency impact analysis using lock file data
fn analyze_dependency_impact_enhanced(results: &mut DependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "📈 Enhanced Dependency Impact Analysis".green().bold());

    // Calculate dependency depths for current graph
    for scope in &results.scopes {
        let scope_path = scope.relative_path(&std::env::current_dir().unwrap_or_default())
            .unwrap_or_else(|_| scope.path.to_string_lossy().to_string());
        let depth = calculate_dependency_depth(&results.current_graph, &scope_path);
        results.dependency_depths.insert(scope_path.clone(), depth);
    }

    // Find high impact scopes
    let mut high_impact: Vec<_> = results.dependency_depths.iter().collect();
    high_impact.sort_by(|a, b| b.1.cmp(a.1));
    results.high_impact_scopes = high_impact.iter().take(5)
        .map(|(scope, depth)| (scope.to_string(), **depth))
        .collect::<Vec<(String, usize)>>();

    // Find independent scopes
    results.independent_scopes = results.dependency_depths
        .iter()
        .filter(|(_, depth)| **depth == 0)
        .map(|(scope, _)| scope.clone())
        .collect();

    // Find longest dependency chains
    let mut longest_chains = Vec::new();
    for start_node in results.current_graph.keys() {
        let chain = find_longest_chain(&results.current_graph, start_node);
        if chain.len() > 1 {
            longest_chains.push(chain);
        }
    }
    longest_chains.sort_by(|a, b| b.len().cmp(&a.len()));
    results.longest_chains = longest_chains.iter().take(3).cloned().collect();

    // Detect circular dependencies
    let mut circular_deps = Vec::new();
    for node in results.current_graph.keys() {
        if has_circular_dependency(&results.current_graph, node) {
            // Find the actual cycle
            let cycle = find_circular_cycle(&results.current_graph, node);
            if !cycle.is_empty() {
                circular_deps.push(cycle);
            }
        }
    }
    results.circular_dependencies = circular_deps;

    println!("✅ Enhanced analysis completed");
    Ok(())
}

/// Find circular dependency cycle
fn find_circular_cycle(graph: &HashMap<String, Vec<String>>, start_node: &str) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut path = Vec::new();
    let mut cycle = Vec::new();

    fn dfs_cycle(
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycle: &mut Vec<String>,
    ) -> bool {
        if rec_stack.contains(node) {
            // Found a cycle
            if let Some(start_idx) = path.iter().position(|x| x == node) {
                *cycle = path[start_idx..].to_vec();
            }
            return true;
        }

        if visited.contains(node) {
            return false;
        }

        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if dfs_cycle(graph, neighbor, visited, rec_stack, path, cycle) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        false
    }

    dfs_cycle(graph, start_node, &mut visited, &mut rec_stack, &mut path, &mut cycle);
    cycle
}

/// Output results in JSON format
fn output_json(results: &DependencyAnalysisResults) -> RhemaResult<()> {
    let json_output = serde_json::json!({
        "analysis": {
            "total_scopes": results.scopes.len(),
            "dependency_depths": results.dependency_depths,
            "circular_dependencies": results.circular_dependencies,
            "version_conflicts": results.version_conflicts,
            "longest_chains": results.longest_chains,
            "high_impact_scopes": results.high_impact_scopes,
            "independent_scopes": results.independent_scopes,
            "differences": results.differences,
        }
    });

    println!("{}", serde_json::to_string_pretty(&json_output)?);
    Ok(())
}

/// Output results in YAML format
fn output_yaml(results: &DependencyAnalysisResults) -> RhemaResult<()> {
    let yaml_output = serde_yaml::to_string(&serde_json::json!({
        "analysis": {
            "total_scopes": results.scopes.len(),
            "dependency_depths": results.dependency_depths,
            "circular_dependencies": results.circular_dependencies,
            "version_conflicts": results.version_conflicts,
            "longest_chains": results.longest_chains,
            "high_impact_scopes": results.high_impact_scopes,
            "independent_scopes": results.independent_scopes,
            "differences": results.differences,
        }
    }))?;

    println!("{}", yaml_output);
    Ok(())
}

/// Output results in text format
fn output_text(results: &DependencyAnalysisResults, visualize: bool) -> RhemaResult<()> {
    println!("{}", "=".repeat(80));
    println!("{}", "📊 DEPENDENCY ANALYSIS RESULTS".green().bold());
    println!("{}", "=".repeat(80));

    // Basic statistics
    println!("📈 Statistics:");
    println!("  • Total scopes: {}", results.scopes.len());
    println!("  • Scopes with dependencies: {}", results.dependency_depths.values().filter(|&&d| d > 0).count());
    println!("  • Independent scopes: {}", results.independent_scopes.len());
    println!("  • Circular dependencies: {}", results.circular_dependencies.len());
    println!("  • Version conflicts: {}", results.version_conflicts.len());

    // Differences (if comparison was performed)
    if !results.differences.is_empty() {
        println!("\n🔄 Differences between lock file and current state:");
        for diff in &results.differences {
            let icon = match diff.difference_type {
                DifferenceType::Added => "➕",
                DifferenceType::Removed => "➖",
                DifferenceType::Modified => "🔄",
                DifferenceType::VersionChanged => "📦",
            };
            println!("  {} {}: {}", icon, diff.scope, diff.details);
        }
    }

    // Version conflicts
    if !results.version_conflicts.is_empty() {
        println!("\n⚠️  Version Conflicts:");
        for conflict in &results.version_conflicts {
            let conflict_icon = match conflict.conflict_type {
                ConflictType::VersionMismatch => "🔴",
                ConflictType::MissingDependency => "🟡",
                ConflictType::ExtraDependency => "🟢",
                ConflictType::TypeMismatch => "🟠",
            };
            println!("  {} {} → {}: expected {}, got {}", 
                conflict_icon, conflict.scope, conflict.dependency, 
                conflict.expected_version, conflict.actual_version);
        }
    }

    // High impact scopes
    if !results.high_impact_scopes.is_empty() {
        println!("\n🏆 High Impact Scopes:");
        for (scope, depth) in &results.high_impact_scopes {
            let impact_level = if *depth > 5 {
                "🔴 Critical"
            } else if *depth > 3 {
                "🟡 High"
            } else if *depth > 1 {
                "🟢 Medium"
            } else {
                "🔵 Low"
            };
            println!("  {} {} ({} dependencies)", impact_level, scope, depth);
        }
    }

    // Independent scopes
    if !results.independent_scopes.is_empty() {
        println!("\n🟢 Independent Scopes:");
        for scope in &results.independent_scopes {
            println!("  📦 {}", scope);
        }
    }

    // Circular dependencies
    if !results.circular_dependencies.is_empty() {
        println!("\n🔄 Circular Dependencies:");
        for (i, cycle) in results.circular_dependencies.iter().enumerate() {
            println!("  {}. {}", i + 1, cycle.join(" → "));
        }
    }

    // Longest chains
    if !results.longest_chains.is_empty() {
        println!("\n⛓️  Longest Dependency Chains:");
        for (i, chain) in results.longest_chains.iter().enumerate() {
            println!("  {}. Chain length {}: {}", i + 1, chain.len(), chain.join(" → "));
        }
    }

    // Visualization (if requested)
    if visualize {
        println!("\n🎨 Dependency Graph Visualization:");
        display_dependency_graph_enhanced(&results.current_graph)?;
    }

    Ok(())
}

/// Enhanced dependency graph display
fn display_dependency_graph_enhanced(dependency_graph: &HashMap<String, Vec<String>>) -> RhemaResult<()> {
    println!("{}", "📊 Enhanced Dependency Graph".green().bold());
    println!("{}", "=".repeat(50));

    for (scope_path, dependencies) in dependency_graph {
        println!("{}", format!("📁 {}", scope_path).cyan().bold());

        if dependencies.is_empty() {
            println!("  └── {} (no dependencies)", "🟢 Independent".green());
        } else {
            for (i, dep) in dependencies.iter().enumerate() {
                let is_last = i == dependencies.len() - 1;
                let prefix = if is_last { "  └── " } else { "  ├── " };
                
                // Check if this dependency has its own dependencies
                let has_subdeps = dependency_graph.get(dep).map(|deps| !deps.is_empty()).unwrap_or(false);
                let icon = if has_subdeps { "📦" } else { "📄" };
                
                println!("{}{} {}", prefix, icon, format!("{}", dep).yellow());
            }
        }
        println!();
    }
    Ok(())
}
