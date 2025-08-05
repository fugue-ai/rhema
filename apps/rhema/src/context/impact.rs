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

use crate::{scope::find_nearest_scope, Rhema, RhemaResult};
use colored::*;

use std::path::Path;

pub fn run(rhema: &Rhema, file: &str) -> RhemaResult<()> {
    println!("{}", "📊 Analyzing impact of changes...".blue().bold());

    // Discover all scopes
    let scopes = rhema.discover_scopes()?;

    if scopes.is_empty() {
        println!("{}", "No scopes found in the repository.".yellow());
        return Ok(());
    }

    // Convert file path to absolute path
    let file_path = if Path::new(file).is_absolute() {
        Path::new(file).to_path_buf()
    } else {
        rhema.repo_root().join(file)
    };

    if !file_path.exists() {
        println!("{}", format!("File not found: {}", file).red());
        return Ok(());
    }

    // Find the scope that contains this file
    let affected_scope = find_nearest_scope(&file_path, &scopes);

    if let Some(scope) = affected_scope {
        let scope_path = scope.relative_path(rhema.repo_root())?;
        println!(
            "{}",
            format!("🎯 File '{}' is in scope: {}", file, scope_path)
                .green()
                .bold()
        );

        // Analyze direct impact
        analyze_direct_impact(rhema, scope, &file_path)?;

        // Analyze indirect impact through dependencies
        analyze_indirect_impact(rhema, &scopes, scope)?;

        // Generate impact report
        generate_impact_report(rhema, &scopes, scope, &file_path)?;
    } else {
        println!(
            "{}",
            format!("⚠️  File '{}' is not within any Rhema scope", file).yellow()
        );
    }

    Ok(())
}

fn analyze_direct_impact(
    rhema: &Rhema,
    scope: &rhema_core::scope::Scope,
    file_path: &Path,
) -> RhemaResult<()> {
    println!("\n{}", "📋 Direct Impact Analysis".green().bold());
    println!("{}", "=".repeat(50));

    let scope_path = scope.relative_path(rhema.repo_root())?;
    let file_name = file_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Determine file type and potential impact
    let file_extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let impact_level = match file_extension {
        "rs" | "py" | "js" | "ts" | "java" | "go" => "🔴 High",
        "yaml" | "yml" | "json" | "toml" => "🟡 Medium",
        "md" | "txt" | "rst" => "🟢 Low",
        _ => "🔵 Unknown",
    };

    println!("  📁 File: {}", file_name);
    println!("  📂 Scope: {}", scope_path);
    println!("  🎯 Impact Level: {}", impact_level);

    // Check if file is a context file
    if scope.has_file(file_name) {
        println!("  📝 Context File: {}", "Yes".green());
        analyze_context_file_impact(rhema, scope, file_name)?;
    } else {
        println!("  📝 Context File: {}", "No".yellow());
        analyze_source_file_impact(rhema, scope, file_path)?;
    }

    Ok(())
}

fn analyze_context_file_impact(
    _rhema: &Rhema,
    _scope: &rhema_core::scope::Scope,
    file_name: &str,
) -> RhemaResult<()> {
    println!("\n  {} Context File Impact:", "📊".blue());

    match file_name {
        "todos.yaml" => {
            println!("    • Todo items may be affected");
            println!("    • Task priorities and assignments could change");
            println!("    • Project timelines might be impacted");
        }
        "knowledge.yaml" => {
            println!("    • Knowledge base entries may be updated");
            println!("    • Documentation could be affected");
            println!("    • Team knowledge sharing might be impacted");
        }
        "patterns.yaml" => {
            println!("    • Design patterns may be modified");
            println!("    • Code architecture could be affected");
            println!("    • Development practices might change");
        }
        "decisions.yaml" => {
            println!("    • Decision records may be updated");
            println!("    • Project direction could be affected");
            println!("    • Stakeholder alignment might be impacted");
        }
        "rhema.yaml" => {
            println!("    • Scope definition may be modified");
            println!("    • Dependencies could be affected");
            println!("    • Scope relationships might change");
        }
        _ => {
            println!("    • Custom context file - impact depends on content");
        }
    }

    Ok(())
}

fn analyze_source_file_impact(
    _rhema: &Rhema,
    _scope: &rhema_core::scope::Scope,
    file_path: &Path,
) -> RhemaResult<()> {
    println!("\n  {} Source File Impact:", "💻".blue());

    let file_extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match file_extension {
        "rs" => {
            println!("    • Rust source code changes");
            println!("    • API interfaces might be affected");
            println!("    • Dependencies could be impacted");
            println!("    • Tests may need updates");
        }
        "py" => {
            println!("    • Python source code changes");
            println!("    • Function signatures might be affected");
            println!("    • Import statements could be impacted");
            println!("    • Unit tests may need updates");
        }
        "js" | "ts" => {
            println!("    • JavaScript/TypeScript changes");
            println!("    • Module exports might be affected");
            println!("    • Type definitions could be impacted");
            println!("    • Frontend components may need updates");
        }
        "java" => {
            println!("    • Java source code changes");
            println!("    • Class interfaces might be affected");
            println!("    • Package structure could be impacted");
            println!("    • JUnit tests may need updates");
        }
        "go" => {
            println!("    • Go source code changes");
            println!("    • Function signatures might be affected");
            println!("    • Package imports could be impacted");
            println!("    • Go tests may need updates");
        }
        _ => {
            println!("    • Source code changes in {}", file_extension);
            println!("    • Impact depends on file content and usage");
        }
    }

    Ok(())
}

fn analyze_indirect_impact(
    rhema: &Rhema,
    scopes: &[rhema_core::scope::Scope],
    affected_scope: &rhema_core::scope::Scope,
) -> RhemaResult<()> {
    println!("\n{}", "🔄 Indirect Impact Analysis".green().bold());
    println!("{}", "=".repeat(50));

    let affected_scope_path = affected_scope.relative_path(rhema.repo_root())?;

    // Find scopes that depend on the affected scope
    let mut dependent_scopes = Vec::new();

    for scope in scopes {
        let dependencies = scope.get_dependency_paths();
        if dependencies.contains(&affected_scope_path) {
            let scope_path = scope.relative_path(rhema.repo_root())?;
            dependent_scopes.push(scope_path);
        }
    }

    if dependent_scopes.is_empty() {
        println!("  ✅ No other scopes depend on '{}'", affected_scope_path);
    } else {
        println!(
            "  ⚠️  The following scopes depend on '{}':",
            affected_scope_path
        );
        for dep_scope in &dependent_scopes {
            println!("    📦 {}", dep_scope);
        }

        // Analyze potential cascading effects
        println!("\n  {} Potential Cascading Effects:", "🌊".blue());
        for dep_scope in &dependent_scopes {
            println!(
                "    • Changes in '{}' could affect '{}'",
                affected_scope_path, dep_scope
            );
        }
    }

    Ok(())
}

fn generate_impact_report(
    rhema: &Rhema,
    scopes: &[rhema_core::scope::Scope],
    affected_scope: &rhema_core::scope::Scope,
    file_path: &Path,
) -> RhemaResult<()> {
    println!("\n{}", "📈 Impact Report Summary".green().bold());
    println!("{}", "=".repeat(50));

    let scope_path = affected_scope.relative_path(rhema.repo_root())?;
    let file_name = file_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Count affected context entries
    let mut affected_entries = 0;
    let mut affected_files = Vec::new();

    // Check each context file type
    let context_files = [
        "todos.yaml",
        "knowledge.yaml",
        "patterns.yaml",
        "decisions.yaml",
    ];

    for context_file in &context_files {
        if affected_scope.has_file(context_file) {
            affected_files.push(context_file.to_string());
            // This is a simplified count - in a real implementation, you'd parse the files
            affected_entries += 1;
        }
    }

    println!("  📊 Scope: {}", scope_path);
    println!("  📁 File: {}", file_name);
    println!("  📝 Context Files: {}", affected_files.len());
    println!("  📋 Affected Entries: {}", affected_entries);

    // Recommendations
    println!("\n  {} Recommendations:", "💡".yellow());

    if file_path.extension().and_then(|s| s.to_str()) == Some("yaml") {
        println!("    • Review and validate YAML syntax");
        println!("    • Check for schema compliance");
        println!("    • Update related documentation");
    } else {
        println!("    • Run tests to ensure functionality");
        println!("    • Update related context files if needed");
        println!("    • Notify team members of changes");
    }

    // Check for dependent scopes
    let dependent_count = scopes
        .iter()
        .filter(|s| s.get_dependency_paths().contains(&scope_path))
        .count();

    if dependent_count > 0 {
        println!("    • Review {} dependent scope(s)", dependent_count);
        println!("    • Consider impact on dependent projects");
    }

    println!("\n  {} Next Steps:", "🚀".green());
    println!("    • Commit changes with descriptive message");
    println!("    • Update related context files if necessary");
    println!("    • Run validation: rhema validate");
    println!("    • Check health: rhema health");

    Ok(())
}
