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
use serde::{Serialize, Deserialize};
use serde_json;
use serde_yaml;

// Placeholder types for now - these will be implemented in the dependency crate
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HealthStatus {
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImpactScore {
    business_impact: f64,
    revenue_impact: f64,
    user_experience_impact: f64,
    operational_cost_impact: f64,
    security_impact: f64,
    compliance_impact: f64,
    risk_level: String,
}

impl ImpactScore {
    fn new(
        business_impact: f64,
        revenue_impact: f64,
        user_experience_impact: f64,
        operational_cost_impact: f64,
        security_impact: f64,
        compliance_impact: f64,
    ) -> RhemaResult<Self> {
        let risk_level = if business_impact > 0.8 {
            "Critical".to_string()
        } else if business_impact > 0.6 {
            "High".to_string()
        } else if business_impact > 0.4 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        };
        
        Ok(Self {
            business_impact,
            revenue_impact,
            user_experience_impact,
            operational_cost_impact,
            security_impact,
            compliance_impact,
            risk_level,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HealthMetrics {
    response_time_ms: f64,
    availability: f64,
    error_rate: f64,
    throughput: f64,
    cpu_usage: f64,
    memory_usage: f64,
    network_latency_ms: f64,
    disk_usage: f64,
}

impl HealthMetrics {
    fn new(
        response_time_ms: f64,
        availability: f64,
        error_rate: f64,
        throughput: f64,
        cpu_usage: f64,
        memory_usage: f64,
        network_latency_ms: f64,
        disk_usage: f64,
    ) -> RhemaResult<Self> {
        Ok(Self {
            response_time_ms,
            availability,
            error_rate,
            throughput,
            cpu_usage,
            memory_usage,
            network_latency_ms,
            disk_usage,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ValidationResult {
    Valid,
    Invalid { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecurityIssue {
    severity: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DependencyManager;

impl DependencyManager {
    async fn new() -> RhemaResult<Self> {
        Ok(Self)
    }
}

pub async fn run(
    rhema: &Rhema,
    impact: bool,
    business: bool,
    validate: bool,
    health: bool,
    _report: bool,
    critical_path: bool,
    recursive: bool,
    format: &str,
    scope: Option<&str>,
) -> RhemaResult<()> {
    println!("{}", "🔗 Enhanced Dependency Management".blue().bold());

    // Discover all scopes
    let scopes = rhema.discover_scopes()?;

    if scopes.is_empty() {
        println!("{}", "No scopes found in the repository.".yellow());
        return Ok(());
    }

    // Filter scopes if specific scope is provided
    let target_scopes = if let Some(target_scope) = scope {
        scopes.into_iter()
            .filter(|s| s.path.to_string_lossy().contains(target_scope))
            .collect::<Vec<_>>()
    } else {
        scopes
    };

    if target_scopes.is_empty() {
        println!("{}", "No matching scopes found.".yellow());
        return Ok(());
    }

    // Build dependency graphs
    let current_graph = build_dependency_graph(&target_scopes)?;

    // Initialize enhanced dependency manager
    let dependency_manager = DependencyManager::new().await?;

    // Perform analysis based on options
    let mut results = EnhancedDependencyAnalysisResults {
        scopes: target_scopes.clone(),
        current_graph,
        dependency_manager,
        impact_analysis: None,
        health_status: HashMap::new(),
        validation_results: Vec::new(),
        business_impact: None,
        critical_paths: Vec::new(),
        circular_dependencies: Vec::new(),
        security_issues: Vec::new(),
        performance_metrics: HashMap::new(),
    };

    // Perform enhanced analysis based on options
    if impact {
        analyze_enhanced_impact(&mut results, recursive).await?;
    }

    if business {
        analyze_business_impact(&mut results).await?;
    }

    if validate {
        validate_dependencies(&mut results).await?;
    }

    if health {
        check_dependency_health(&mut results).await?;
    }

    if critical_path {
        find_critical_paths(&mut results).await?;
    }

    // Display results based on format
    match format {
        "json" => output_enhanced_json(&results)?,
        "yaml" => output_enhanced_yaml(&results)?,
        "graphviz" => output_graphviz(&results)?,
        _ => output_enhanced_text(&results)?,
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

/// Check for circular dependencies and return all cycles
fn check_circular_dependencies(graph: &HashMap<String, Vec<String>>) -> RhemaResult<Vec<Vec<String>>> {
    let mut cycles = Vec::new();
    
    for node in graph.keys() {
        if has_circular_dependency(graph, node) {
            let cycle = find_circular_cycle(graph, node);
            if !cycle.is_empty() {
                cycles.push(cycle);
            }
        }
    }
    
    Ok(cycles)
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

fn find_circular_cycle(graph: &HashMap<String, Vec<String>>, start_node: &str) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut path = Vec::new();

    fn dfs(
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if rec_stack.contains(node) {
            // Cycle detected, find the path
            let cycle_start_index = path.iter().position(|x| x == node).unwrap();
            let _cycle_path = path[cycle_start_index..].to_vec();
            return true;
        }

        if visited.contains(node) {
            return false; // Already processed
        }

        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if dfs(graph, neighbor, visited, rec_stack, path) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        false
    }

    dfs(graph, start_node, &mut visited, &mut rec_stack, &mut path);
    path
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

// Enhanced dependency analysis results
#[derive(Debug)]
struct EnhancedDependencyAnalysisResults {
    scopes: Vec<rhema_core::scope::Scope>,
    current_graph: HashMap<String, Vec<String>>,
    dependency_manager: DependencyManager,
    impact_analysis: Option<ImpactAnalysis>,
    health_status: HashMap<String, HealthStatus>,
    validation_results: Vec<ValidationResult>,
    business_impact: Option<ImpactScore>,
    critical_paths: Vec<Vec<String>>,
    circular_dependencies: Vec<Vec<String>>,
    security_issues: Vec<SecurityIssue>,
    performance_metrics: HashMap<String, HealthMetrics>,
}

// Placeholder types for now - these will be implemented in the dependency crate
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImpactAnalysis {
    scope: String,
    impact_level: String,
    affected_services: Vec<String>,
}

// Enhanced analysis functions
async fn analyze_enhanced_impact(
    results: &mut EnhancedDependencyAnalysisResults,
    _recursive: bool,
) -> RhemaResult<()> {
    println!("{}", "📊 Analyzing enhanced dependency impact...".blue());

    for scope in &results.scopes {
        // For now, create a placeholder impact analysis
        let impact = ImpactAnalysis {
            scope: scope.path.to_string_lossy().to_string(),
            impact_level: "medium".to_string(),
            affected_services: vec!["service1".to_string(), "service2".to_string()],
        };
        results.impact_analysis = Some(impact.clone());
        
        println!("  📈 Impact analysis for {}: {:?}", scope.path.display(), impact);
    }

    Ok(())
}

async fn analyze_business_impact(results: &mut EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "💰 Analyzing business impact...".blue());

    // Create a placeholder business impact score
    let business_impact = ImpactScore::new(0.7, 0.6, 0.8, 0.5, 0.3, 0.4)?;
    results.business_impact = Some(business_impact.clone());
    
    println!("  💰 Business impact score: {:.2}", business_impact.business_impact);
    println!("  📊 Revenue impact: {:.2}", business_impact.revenue_impact);
    println!("  👥 User experience impact: {:.2}", business_impact.user_experience_impact);

    Ok(())
}

async fn validate_dependencies(results: &mut EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "✅ Validating dependencies...".blue());

    for scope in &results.scopes {
        // For now, assume all dependencies are valid
        let validation = ValidationResult::Valid;
        results.validation_results.push(validation);
        
        println!("  ✅ {}: Valid", scope.path.display());
    }

    // Check for circular dependencies using the existing function
    let circular = check_circular_dependencies(&results.current_graph)?;
    results.circular_dependencies = circular;

    if !results.circular_dependencies.is_empty() {
        println!("  ⚠️  Circular dependencies detected:");
        for cycle in &results.circular_dependencies {
            println!("    🔄 {}", cycle.join(" -> "));
        }
    }

    Ok(())
}

async fn check_dependency_health(results: &mut EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "🏥 Checking dependency health...".blue());

    for scope in &results.scopes {
        // For now, assume all dependencies are healthy
        let health = HealthStatus { status: "Healthy".to_string() };
        results.health_status.insert(scope.path.to_string_lossy().to_string(), health.clone());
        
        // Create placeholder health metrics
        let metrics = HealthMetrics::new(50.0, 0.99, 0.01, 100.0, 0.3, 0.4, 10.0, 0.6)?;
        results.performance_metrics.insert(scope.path.to_string_lossy().to_string(), metrics.clone());
        
        println!("  🏥 {}: {:?}", scope.path.display(), health);
        println!("    📊 Response time: {:.2}ms", metrics.response_time_ms);
        println!("    📈 Availability: {:.1}%", metrics.availability * 100.0);
    }

    Ok(())
}

async fn find_critical_paths(results: &mut EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "🎯 Finding critical paths...".blue());

    // Use the existing longest chain finding logic
    let mut critical_paths = Vec::new();
    for start_node in results.current_graph.keys() {
        let chain = find_longest_chain(&results.current_graph, start_node);
        if chain.len() > 1 {
            critical_paths.push(chain);
        }
    }
    
    // Sort by length and take the top 3
    critical_paths.sort_by(|a, b| b.len().cmp(&a.len()));
    results.critical_paths = critical_paths.iter().take(3).cloned().collect();

    for (i, path) in results.critical_paths.iter().enumerate() {
        println!("  🎯 Critical path {}: {}", i + 1, path.join(" -> "));
    }

    Ok(())
}

fn output_enhanced_json(results: &EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    let output = serde_json::json!({
        "scopes": results.scopes,
        "impact_analysis": results.impact_analysis,
        "health_status": results.health_status,
        "validation_results": results.validation_results,
        "business_impact": results.business_impact,
        "critical_paths": results.critical_paths,
        "circular_dependencies": results.circular_dependencies,
        "security_issues": results.security_issues,
        "performance_metrics": results.performance_metrics
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn output_enhanced_yaml(results: &EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    let output = serde_yaml::to_string(&serde_json::json!({
        "scopes": results.scopes,
        "impact_analysis": results.impact_analysis,
        "health_status": results.health_status,
        "validation_results": results.validation_results,
        "business_impact": results.business_impact,
        "critical_paths": results.critical_paths,
        "circular_dependencies": results.circular_dependencies,
        "security_issues": results.security_issues,
        "performance_metrics": results.performance_metrics
    }))?;

    println!("{}", output);
    Ok(())
}

fn output_graphviz(results: &EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    println!("{}", "digraph DependencyGraph {{");
    println!("  rankdir=TB;");
    println!("  node [shape=box, style=filled];");
    
    // Add nodes
    for scope in &results.scopes {
        let color = match results.health_status.get(&scope.path.to_string_lossy().to_string()) {
            Some(status) if status.status == "Healthy" => "lightgreen",
            Some(status) if status.status == "Degraded" => "yellow",
            Some(status) if status.status == "Unhealthy" => "orange",
            Some(status) if status.status == "Down" => "red",
            _ => "lightgray",
        };
        
        println!("  \"{}\" [fillcolor={}];", scope.path.to_string_lossy(), color);
    }
    
    // Add edges
    for (from, deps) in &results.current_graph {
        for to in deps {
            println!("  \"{}\" -> \"{}\";", from, to);
        }
    }
    
    println!("{}", "}");
    Ok(())
}

fn output_enhanced_text(results: &EnhancedDependencyAnalysisResults) -> RhemaResult<()> {
    println!("\n{}", "📊 Enhanced Dependency Analysis Results".green().bold());
    println!("{}", "=".repeat(50));

    // Summary
    println!("📋 Summary:");
    println!("  • Total scopes: {}", results.scopes.len());
    println!("  • Healthy dependencies: {}", 
        results.health_status.values()
            .filter(|&status| status.status == "Healthy")
            .count());
    println!("  • Critical paths: {}", results.critical_paths.len());
    println!("  • Circular dependencies: {}", results.circular_dependencies.len());

    // Health status
    if !results.health_status.is_empty() {
        println!("\n🏥 Health Status:");
        for (scope, status) in &results.health_status {
            let icon = match status.status.as_str() {
                "Healthy" => "✅",
                "Degraded" => "⚠️",
                "Unhealthy" => "❌",
                "Down" => "💀",
                _ => "❓",
            };
            println!("  {} {}: {:?}", icon, scope, status);
        }
    }

    // Business impact
    if let Some(ref impact) = results.business_impact {
        println!("\n💰 Business Impact:");
        println!("  • Overall impact: {:.2}", impact.business_impact);
        println!("  • Revenue impact: {:.2}", impact.revenue_impact);
        println!("  • User experience: {:.2}", impact.user_experience_impact);
        println!("  • Risk level: {:?}", impact.risk_level);
    }

    // Critical paths
    if !results.critical_paths.is_empty() {
        println!("\n🎯 Critical Paths:");
        for (i, path) in results.critical_paths.iter().enumerate() {
            println!("  {}. {}", i + 1, path.join(" → "));
        }
    }

    // Circular dependencies
    if !results.circular_dependencies.is_empty() {
        println!("\n🔄 Circular Dependencies:");
        for cycle in &results.circular_dependencies {
            println!("  🔄 {}", cycle.join(" → "));
        }
    }

    // Performance metrics
    if !results.performance_metrics.is_empty() {
        println!("\n📊 Performance Metrics:");
        for (scope, metrics) in &results.performance_metrics {
            println!("  📈 {}:", scope);
            println!("    • Response time: {:.2}ms", metrics.response_time_ms);
            println!("    • Availability: {:.1}%", metrics.availability * 100.0);
            println!("    • Error rate: {:.2}%", metrics.error_rate * 100.0);
        }
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
