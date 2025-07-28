use crate::{Gacp, GacpResult};
use crate::scope::{get_scope, validate_scope_relationships};
use colored::*;

pub fn run(gacp: &Gacp, scope: Option<&str>) -> GacpResult<()> {
    println!("🏥 Checking GACP scope health...");
    println!("{}", "─".repeat(80));
    
    let scopes = if let Some(scope_path) = scope {
        // Check specific scope
        let scope_obj = get_scope(gacp.repo_root(), scope_path)?;
        vec![scope_obj]
    } else {
        // Check all scopes
        gacp.discover_scopes()?
    };
    
    let mut total_issues = 0;
    let mut healthy_scopes = 0;
    
    for scope in &scopes {
        println!("📁 Checking scope: {}", scope.definition.name.bright_blue());
        let issues = check_scope_health(scope, gacp.repo_root())?;
        
        if issues.is_empty() {
            println!("  ✅ Scope is healthy");
            healthy_scopes += 1;
        } else {
            println!("  ⚠️  Found {} issue(s):", issues.len());
            for issue in &issues {
                println!("    • {}", issue.red());
            }
            total_issues += issues.len();
        }
        println!();
    }
    
    // Check scope relationships
    println!("🔗 Checking scope relationships...");
    match validate_scope_relationships(&scopes, gacp.repo_root()) {
        Ok(()) => {
            println!("  ✅ All scope relationships are valid");
        }
        Err(e) => {
            println!("  ❌ Scope relationship issues: {}", e.to_string().red());
            total_issues += 1;
        }
    }
    
    // Print summary
    println!("{}", "─".repeat(80));
    println!("📊 Health Summary:");
    println!("  📁 Total scopes: {}", scopes.len());
    println!("  ✅ Healthy scopes: {}", healthy_scopes.to_string().green());
    println!("  ⚠️  Total issues: {}", total_issues.to_string().red());
    
    if total_issues == 0 {
        println!("🎉 All scopes are healthy!");
    } else {
        println!("🔧 Consider running 'gacp validate' for detailed validation");
    }
    
    Ok(())
}

fn check_scope_health(scope: &crate::Scope, repo_root: &std::path::Path) -> GacpResult<Vec<String>> {
    let mut issues = Vec::new();
    
    // Check required files
    let required_files = [
        "gacp.yaml",
        "todos.yaml",
        "knowledge.yaml",
        "patterns.yaml",
        "decisions.yaml",
    ];
    
    for file in &required_files {
        let file_path = scope.path.join(file);
        if !file_path.exists() {
            issues.push(format!("Missing required file: {}", file));
        }
    }
    
    // Check scope definition
    if scope.definition.name.is_empty() {
        issues.push("Scope name is empty".to_string());
    }
    
    if scope.definition.scope_type.is_empty() {
        issues.push("Scope type is empty".to_string());
    }
    
    if scope.definition.version.is_empty() {
        issues.push("Scope version is empty".to_string());
    }
    
    // Check dependencies
    if let Some(dependencies) = &scope.definition.dependencies {
        for dep in dependencies {
            if dep.path.is_empty() {
                issues.push("Dependency path is empty".to_string());
            }
            
            if dep.dependency_type.is_empty() {
                issues.push("Dependency type is empty".to_string());
            }
            
            // Check if dependency scope exists
            let dep_path = if dep.path.starts_with('/') {
                std::path::PathBuf::from(&dep.path)
            } else {
                repo_root.join(&dep.path)
            };
            
            let gacp_path = if dep_path.file_name().and_then(|s| s.to_str()) == Some(".gacp") {
                dep_path
            } else {
                dep_path.join(".gacp")
            };
            
            if !gacp_path.exists() {
                issues.push(format!("Dependency scope not found: {}", dep.path));
            }
        }
    }
    
    // Check file permissions
    for entry in std::fs::read_dir(&scope.path)
        .map_err(|e| crate::GacpError::IoError(e))?
    {
        let entry = entry.map_err(|e| crate::GacpError::IoError(e))?;
        let path = entry.path();
        
        if path.is_file() {
            let metadata = std::fs::metadata(&path)
                .map_err(|e| crate::GacpError::IoError(e))?;
            
            if metadata.permissions().readonly() {
                issues.push(format!("File is read-only: {}", path.file_name().unwrap().to_string_lossy()));
            }
        }
    }
    
    // Check for empty files
    for entry in std::fs::read_dir(&scope.path)
        .map_err(|e| crate::GacpError::IoError(e))?
    {
        let entry = entry.map_err(|e| crate::GacpError::IoError(e))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| crate::GacpError::IoError(e))?;
            
            if content.trim().is_empty() {
                issues.push(format!("File is empty: {}", path.file_name().unwrap().to_string_lossy()));
            }
        }
    }
    
    // Check for malformed YAML files
    for entry in std::fs::read_dir(&scope.path)
        .map_err(|e| crate::GacpError::IoError(e))?
    {
        let entry = entry.map_err(|e| crate::GacpError::IoError(e))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| crate::GacpError::IoError(e))?;
            
            if serde_yaml::from_str::<serde_yaml::Value>(&content).is_err() {
                issues.push(format!("Malformed YAML: {}", path.file_name().unwrap().to_string_lossy()));
            }
        }
    }
    
    Ok(issues)
} 