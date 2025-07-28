use crate::{Gacp, GacpResult, schema::{Validatable, SchemaMigratable, CURRENT_SCHEMA_VERSION}};
use colored::*;
use serde_yaml;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(gacp: &Gacp, recursive: bool, dry_run: bool) -> GacpResult<()> {
    println!("🔄 Migrating GACP schema files...");
    println!("{}", "─".repeat(80));
    
    if dry_run {
        println!("🔍 DRY RUN MODE - No files will be modified");
        println!("{}", "─".repeat(80));
    }
    
    let mut total_files = 0;
    let mut migrated_files = 0;
    let mut errors = Vec::new();
    
    if recursive {
        // Migrate all scopes in the repository
        let scopes = gacp.discover_scopes()?;
        
        for scope in scopes {
            println!("📁 Migrating scope: {}", scope.definition.name.bright_blue());
            let (scope_files, scope_migrated, scope_errors) = migrate_scope(&scope.path, dry_run)?;
            total_files += scope_files;
            migrated_files += scope_migrated;
            errors.extend(scope_errors);
        }
    } else {
        // Migrate only the current scope
        let current_dir = std::env::current_dir()
            .map_err(|e| crate::GacpError::IoError(e))?;
        
        let scopes = gacp.discover_scopes()?;
        let scope = crate::scope::find_nearest_scope(&current_dir, &scopes)
            .ok_or_else(|| crate::GacpError::ConfigError("No GACP scope found in current directory or parent directories".to_string()))?;
        
        println!("📁 Migrating scope: {}", scope.definition.name.bright_blue());
        let (scope_files, scope_migrated, scope_errors) = migrate_scope(&scope.path, dry_run)?;
        total_files = scope_files;
        migrated_files = scope_migrated;
        errors = scope_errors;
    }
    
    // Print summary
    println!("{}", "─".repeat(80));
    println!("📊 Migration Summary:");
    println!("  📄 Total files checked: {}", total_files);
    println!("  🔄 Files migrated: {}", migrated_files.to_string().yellow());
    println!("  ❌ Errors: {}", errors.len().to_string().red());
    
    if !errors.is_empty() {
        println!("\n❌ Migration Errors:");
        for (i, error) in errors.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().red(), error);
        }
        return Err(crate::GacpError::SchemaValidation(
            format!("Migration failed with {} errors", errors.len())
        ));
    } else {
        if dry_run {
            println!("🎉 All files are ready for migration!");
        } else {
            println!("🎉 Schema migration completed successfully!");
        }
    }
    
    Ok(())
}

fn migrate_scope(scope_path: &Path, dry_run: bool) -> GacpResult<(usize, usize, Vec<String>)> {
    let mut total_files = 0;
    let mut migrated_files = 0;
    let mut errors = Vec::new();
    
    // Migrate the scope definition itself
    let gacp_file = scope_path.join("gacp.yaml");
    if gacp_file.exists() {
        total_files += 1;
        match migrate_gacp_file(&gacp_file, dry_run) {
            Ok(migrated) => {
                if migrated {
                    migrated_files += 1;
                    if dry_run {
                        println!("  🔍 gacp.yaml (would be migrated)");
                    } else {
                        println!("  🔄 gacp.yaml (migrated to version {})", CURRENT_SCHEMA_VERSION.yellow());
                    }
                } else {
                    println!("  ✅ gacp.yaml (already up to date)");
                }
            }
            Err(e) => {
                errors.push(format!("gacp.yaml: {}", e));
                println!("  ❌ gacp.yaml: {}", e);
            }
        }
    }
    
    // Check other YAML files for potential schema issues
    for entry in WalkDir::new(scope_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            // Skip gacp.yaml as we already handled it
            if file_name == "gacp.yaml" {
                continue;
            }
            
            total_files += 1;
            // For now, just validate other files to check for schema issues
            match validate_context_file(path) {
                Ok(()) => {
                    println!("  ✅ {}", file_name);
                }
                Err(e) => {
                    errors.push(format!("{}: {}", file_name, e));
                    println!("  ❌ {}: {}", file_name, e);
                }
            }
        }
    }
    
    Ok((total_files, migrated_files, errors))
}

fn migrate_gacp_file(file_path: &Path, dry_run: bool) -> GacpResult<bool> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::GacpError::IoError(e))?;
    
    let mut scope: crate::GacpScope = serde_yaml::from_str(&content)
        .map_err(|e| crate::GacpError::InvalidYaml {
            file: file_path.display().to_string(),
            message: e.to_string(),
        })?;
    
    let current_version = scope.get_schema_version().unwrap_or_else(|| "0.1.0".to_string());
    
    if current_version == CURRENT_SCHEMA_VERSION {
        return Ok(false); // Already up to date
    }
    
    // Perform schema migration
    match scope.migrate_to_latest() {
        Ok(()) => {
            if !dry_run {
                // Write the migrated content back to file
                let migrated_content = serde_yaml::to_string(&scope)
                    .map_err(|e| crate::GacpError::InvalidYaml {
                        file: file_path.display().to_string(),
                        message: format!("Failed to serialize migrated scope: {}", e),
                    })?;
                
                std::fs::write(file_path, migrated_content)
                    .map_err(|e| crate::GacpError::IoError(e))?;
            }
            Ok(true)
        }
        Err(e) => {
            Err(crate::GacpError::SchemaValidation(
                format!("Schema migration failed: {}", e)
            ))
        }
    }
}

fn validate_context_file(file_path: &Path) -> GacpResult<()> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::GacpError::IoError(e))?;
    
    let file_name = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    // Validate context files to check for schema issues
    match file_name {
        "todos.yaml" => {
            let todos: crate::Todos = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&todos)?;
        }
        "knowledge.yaml" => {
            let knowledge: crate::Knowledge = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&knowledge)?;
        }
        "patterns.yaml" => {
            let patterns: crate::Patterns = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&patterns)?;
        }
        "decisions.yaml" => {
            let decisions: crate::Decisions = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&decisions)?;
        }
        "conventions.yaml" => {
            let conventions: crate::Conventions = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            Validatable::validate(&conventions)?;
        }
        _ => {
            // For unknown files, just validate that they're valid YAML
            let _: serde_yaml::Value = serde_yaml::from_str(&content)
                .map_err(|e| crate::GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
        }
    }
    
    Ok(())
} 