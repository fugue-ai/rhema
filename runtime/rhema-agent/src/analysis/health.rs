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

use crate::scope::{get_scope, validate_scope_relationships};
use crate::{Rhema, RhemaResult};
use colored::*;
use rhema_core::lock::LockFileOps;
use rhema_core::schema::RhemaLock;
use std::path::{Path, PathBuf};
use chrono::{Utc, Duration};
use sha2::{Digest, Sha256};

pub fn run(rhema: &Rhema, scope: Option<&str>) -> RhemaResult<()> {
    println!("🏥 Checking Rhema scope health...");
    println!("{}", "─".repeat(80));

    let scopes = if let Some(scope_path) = scope {
        // Check specific scope
        let scope_obj = get_scope(rhema.repo_root(), scope_path)?;
        vec![scope_obj]
    } else {
        // Check all scopes
        rhema.discover_scopes()?
    };

    let mut total_issues = 0;
    let mut healthy_scopes = 0;

    for scope in &scopes {
        println!("📁 Checking scope: {}", scope.definition.name.bright_blue());
        let issues = check_scope_health(scope, rhema.repo_root())?;

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
    match validate_scope_relationships(&scopes, rhema.repo_root()) {
        Ok(()) => {
            println!("  ✅ All scope relationships are valid");
        }
        Err(e) => {
            println!("  ❌ Scope relationship issues: {}", e.to_string().red());
            total_issues += 1;
        }
    }

    // Check lock file health
    println!("🔒 Checking lock file health...");
    let lock_issues = check_lock_file_health(rhema.repo_root(), &scopes)?;
    
    if lock_issues.is_empty() {
        println!("  ✅ Lock file is healthy");
    } else {
        println!("  ⚠️  Found {} lock file issue(s):", lock_issues.len());
        for issue in &lock_issues {
            println!("    • {}", issue.red());
        }
        total_issues += lock_issues.len();
    }

    // Print summary
    println!("{}", "─".repeat(80));
    println!("📊 Health Summary:");
    println!("  📁 Total scopes: {}", scopes.len());
    println!(
        "  ✅ Healthy scopes: {}",
        healthy_scopes.to_string().green()
    );
    println!("  ⚠️  Total issues: {}", total_issues.to_string().red());

    if total_issues == 0 {
        println!("🎉 All scopes and lock files are healthy!");
    } else {
        println!("🔧 Consider running 'rhema validate' for detailed validation");
    }

    Ok(())
}

/// Check lock file health and consistency
fn check_lock_file_health(
    repo_root: &Path,
    scopes: &[rhema_core::scope::Scope],
) -> RhemaResult<Vec<String>> {
    let mut issues = Vec::new();
    let lock_file_path = repo_root.join("rhema.lock");

    // 1. Check if lock file exists and is valid
    if !lock_file_path.exists() {
        issues.push("Lock file does not exist (rhema.lock)".to_string());
        return Ok(issues);
    }

    // Read and validate lock file
    let lock_data = match LockFileOps::read_lock_file(&lock_file_path) {
        Ok(data) => data,
        Err(e) => {
            issues.push(format!("Lock file is invalid: {}", e));
            return Ok(issues);
        }
    };

    // 2. Validate lock file checksums
    let expected_checksum = lock_data.calculate_checksum();
    if lock_data.checksum != expected_checksum {
        issues.push("Lock file checksum is invalid - file may be corrupted".to_string());
    }

    // 3. Check if lock file is consistent with current scope state
    let scope_consistency_issues = check_scope_consistency(&lock_data, scopes, repo_root)?;
    issues.extend(scope_consistency_issues);

    // 4. Check for dependency version mismatches
    let dependency_issues = check_dependency_versions(&lock_data, scopes, repo_root)?;
    issues.extend(dependency_issues);

    // 5. Check if lock file is stale
    let staleness_issues = check_lock_file_staleness(&lock_data, &lock_file_path)?;
    issues.extend(staleness_issues);

    // 6. Validate individual scope and dependency checksums
    let checksum_issues = validate_checksums(&lock_data, repo_root)?;
    issues.extend(checksum_issues);

    Ok(issues)
}

/// Check if lock file scopes are consistent with current scope state
fn check_scope_consistency(
    lock_data: &RhemaLock,
    scopes: &[rhema_core::scope::Scope],
    repo_root: &Path,
) -> RhemaResult<Vec<String>> {
    let mut issues = Vec::new();

    // Check for scopes in lock file that don't exist anymore
    for (scope_path, _) in &lock_data.scopes {
        let scope_dir = if scope_path.starts_with('/') {
            PathBuf::from(scope_path)
        } else {
            repo_root.join(scope_path)
        };

        if !scope_dir.exists() {
            issues.push(format!("Locked scope no longer exists: {}", scope_path));
        }
    }

    // Check for current scopes that aren't in lock file
    for scope in scopes {
        let scope_path = scope.path.strip_prefix(repo_root)
            .unwrap_or(&scope.path)
            .to_string_lossy()
            .to_string();
        
        if !lock_data.scopes.contains_key(&scope_path) {
            issues.push(format!("Scope not locked: {}", scope_path));
        }
    }

    Ok(issues)
}

/// Check for dependency version mismatches
fn check_dependency_versions(
    lock_data: &RhemaLock,
    scopes: &[rhema_core::scope::Scope],
    repo_root: &Path,
) -> RhemaResult<Vec<String>> {
    let mut issues = Vec::new();

    for scope in scopes {
        let scope_path = scope.path.strip_prefix(repo_root)
            .unwrap_or(&scope.path)
            .to_string_lossy()
            .to_string();

        if let Some(locked_scope) = lock_data.scopes.get(&scope_path) {
            // Check if scope version matches
            if locked_scope.version != scope.definition.version {
                issues.push(format!(
                    "Scope version mismatch for {}: locked={}, current={}",
                    scope_path, locked_scope.version, scope.definition.version
                ));
            }

            // Check dependencies
            if let Some(dependencies) = &scope.definition.dependencies {
                for dep in dependencies {
                    let dep_path = dep.path.clone();
                    
                    if let Some(locked_dep) = locked_scope.dependencies.get(&dep_path) {
                        // Check if dependency path still exists
                        let actual_dep_path = if dep_path.starts_with('/') {
                            PathBuf::from(&dep_path)
                        } else {
                            repo_root.join(&dep_path)
                        };

                        if !actual_dep_path.exists() {
                            issues.push(format!(
                                "Locked dependency no longer exists: {} in scope {}",
                                dep_path, scope_path
                            ));
                        }

                        // Check dependency type consistency
                        if format!("{:?}", locked_dep.dependency_type) != dep.dependency_type {
                            issues.push(format!(
                                "Dependency type mismatch for {} in scope {}: locked={:?}, current={}",
                                dep_path, scope_path, locked_dep.dependency_type, dep.dependency_type
                            ));
                        }
                    } else {
                        issues.push(format!(
                            "Dependency not locked: {} in scope {}",
                            dep_path, scope_path
                        ));
                    }
                }
            }

            // Check for locked dependencies that are no longer declared
            for (dep_path, _) in &locked_scope.dependencies {
                let mut found = false;
                if let Some(dependencies) = &scope.definition.dependencies {
                    for dep in dependencies {
                        if dep.path == *dep_path {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    issues.push(format!(
                        "Locked dependency no longer declared: {} in scope {}",
                        dep_path, scope_path
                    ));
                }
            }
        }
    }

    Ok(issues)
}

/// Check if lock file is stale (too old)
fn check_lock_file_staleness(
    lock_data: &RhemaLock,
    lock_file_path: &Path,
) -> RhemaResult<Vec<String>> {
    let mut issues = Vec::new();

    // Check lock file age (warn if older than 30 days)
    let lock_age = Utc::now() - lock_data.generated_at;
    let max_age = Duration::days(30);
    
    if lock_age > max_age {
        issues.push(format!(
            "Lock file is stale ({} days old) - consider regenerating",
            lock_age.num_days()
        ));
    }

    // Check if lock file is older than any source files
    if let Ok(metadata) = std::fs::metadata(lock_file_path) {
        if let Ok(modified_time) = metadata.modified() {
            let lock_modified: chrono::DateTime<Utc> = chrono::DateTime::from(modified_time);
            
            // Check if any scope files are newer than lock file
            for (scope_path, locked_scope) in &lock_data.scopes {
                let scope_dir = PathBuf::from(scope_path);
                if scope_dir.exists() {
                    if let Ok(scope_metadata) = std::fs::metadata(&scope_dir) {
                        if let Ok(scope_modified) = scope_metadata.modified() {
                            let scope_modified_dt: chrono::DateTime<Utc> = chrono::DateTime::from(scope_modified);
                            if scope_modified_dt > lock_modified {
                                issues.push(format!(
                                    "Scope {} has been modified since lock file was generated",
                                    scope_path
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(issues)
}

/// Validate checksums for scopes and dependencies
fn validate_checksums(
    lock_data: &RhemaLock,
    repo_root: &Path,
) -> RhemaResult<Vec<String>> {
    let mut issues = Vec::new();

    for (scope_path, locked_scope) in &lock_data.scopes {
        // Validate scope source checksum if present
        if let Some(expected_checksum) = &locked_scope.source_checksum {
            let scope_dir = if scope_path.starts_with('/') {
                PathBuf::from(scope_path)
            } else {
                repo_root.join(scope_path)
            };

            if scope_dir.exists() {
                // Calculate current checksum for scope files
                let current_checksum = calculate_scope_checksum(&scope_dir)?;
                if current_checksum != *expected_checksum {
                    issues.push(format!(
                        "Scope checksum mismatch for {}: expected={}, current={}",
                        scope_path, expected_checksum, current_checksum
                    ));
                }
            }
        }

        // Validate dependency checksums
        for (dep_path, locked_dep) in &locked_scope.dependencies {
            let dep_dir = if dep_path.starts_with('/') {
                PathBuf::from(dep_path)
            } else {
                repo_root.join(dep_path)
            };

            if dep_dir.exists() {
                let current_checksum = calculate_scope_checksum(&dep_dir)?;
                if current_checksum != locked_dep.checksum {
                    issues.push(format!(
                        "Dependency checksum mismatch for {} in scope {}: expected={}, current={}",
                        dep_path, scope_path, locked_dep.checksum, current_checksum
                    ));
                }
            }
        }
    }

    Ok(issues)
}

/// Calculate checksum for a scope directory
pub fn calculate_scope_checksum(scope_dir: &Path) -> RhemaResult<String> {
    let mut hasher = Sha256::new();
    
    // Walk through all files in the scope directory
    for entry in walkdir::WalkDir::new(scope_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip .git and other hidden directories
        if path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
        {
            continue;
        }

        if path.is_file() {
            // Read file content and update hash
            if let Ok(content) = std::fs::read(path) {
                hasher.update(&content);
            }
        }
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn check_scope_health(
    scope: &rhema_core::scope::Scope,
    repo_root: &std::path::Path,
) -> RhemaResult<Vec<String>> {
    let mut issues = Vec::new();

    // Check required files
    let required_files = [
        "rhema.yaml",
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
    // TODO: Integrate with lock file system for deterministic dependency validation
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

            let rhema_path = if dep_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
                dep_path
            } else {
                dep_path.join(".rhema")
            };

            if !rhema_path.exists() {
                issues.push(format!("Dependency scope not found: {}", dep.path));
            }
        }
    }

    // Check file permissions
    for entry in std::fs::read_dir(&scope.path).map_err(|e| crate::RhemaError::IoError(e))? {
        let entry = entry.map_err(|e| crate::RhemaError::IoError(e))?;
        let path = entry.path();

        if path.is_file() {
            let metadata = std::fs::metadata(&path).map_err(|e| crate::RhemaError::IoError(e))?;

            if metadata.permissions().readonly() {
                issues.push(format!(
                    "File is read-only: {}",
                    path.file_name().unwrap().to_string_lossy()
                ));
            }
        }
    }

    // Check for empty files
    for entry in std::fs::read_dir(&scope.path).map_err(|e| crate::RhemaError::IoError(e))? {
        let entry = entry.map_err(|e| crate::RhemaError::IoError(e))?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content =
                std::fs::read_to_string(&path).map_err(|e| crate::RhemaError::IoError(e))?;

            if content.trim().is_empty() {
                issues.push(format!(
                    "File is empty: {}",
                    path.file_name().unwrap().to_string_lossy()
                ));
            }
        }
    }

    // Check for malformed YAML files
    for entry in std::fs::read_dir(&scope.path).map_err(|e| crate::RhemaError::IoError(e))? {
        let entry = entry.map_err(|e| crate::RhemaError::IoError(e))?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content =
                std::fs::read_to_string(&path).map_err(|e| crate::RhemaError::IoError(e))?;

            if serde_yaml::from_str::<serde_yaml::Value>(&content).is_err() {
                issues.push(format!(
                    "Malformed YAML: {}",
                    path.file_name().unwrap().to_string_lossy()
                ));
            }
        }
    }

    Ok(issues)
}
