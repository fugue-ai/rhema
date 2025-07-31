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

use crate::{RhemaError, RhemaScope, schema::Validatable};
use serde_yaml;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a Rhema scope with its metadata and files
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scope {
    /// Path to the scope directory
    pub path: PathBuf,
    
    /// Scope definition from rhema.yaml
    pub definition: RhemaScope,
    
    /// Available files in this scope
    pub files: HashMap<String, PathBuf>,
}

impl Scope {
    /// Find the scope file in the given directory, checking multiple possible locations
    pub fn find_scope_file(scope_path: &Path) -> Result<PathBuf, RhemaError> {
        // Define the possible locations in order of preference
        let mut all_locations = Vec::new();
        
        // First priority: files in the scope directory itself
        all_locations.push(scope_path.join("rhema.yaml"));
        all_locations.push(scope_path.join("scope.yaml"));
        
        // Second priority: files in parent directory (if we're in a .rhema directory)
        if scope_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
            let parent = scope_path.parent().unwrap_or(scope_path);
            all_locations.push(parent.join("rhema.yaml"));
            all_locations.push(parent.join("scope.yaml"));
        }
        
        // Find the first existing file
        for location in &all_locations {
            if location.exists() {
                return Ok(location.clone());
            }
        }
        
        // If no file found, return error with all checked locations
        let checked_locations = all_locations.iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        
        Err(RhemaError::FileNotFound(
            format!("No scope file found in {} (checked: {})", scope_path.display(), checked_locations)
        ))
    }

    /// Create a new scope from a directory path
    pub fn new(path: PathBuf) -> Result<Self, RhemaError> {
        let rhema_file = Self::find_scope_file(&path)?;
        
        let content = std::fs::read_to_string(&rhema_file)
            .map_err(|e| RhemaError::IoError(e))?;
        
        let definition: RhemaScope = serde_yaml::from_str(&content)
            .map_err(|e| RhemaError::InvalidYaml {
                file: rhema_file.display().to_string(),
                message: e.to_string(),
            })?;
        
        // Validate the scope definition
        definition.validate()?;
        
        // Discover available files
        let files = Self::discover_files(&path)?;
        
        Ok(Scope {
            path,
            definition,
            files,
        })
    }
    
    /// Discover all YAML files in the scope directory
    fn discover_files(scope_path: &Path) -> Result<HashMap<String, PathBuf>, RhemaError> {
        let mut files = HashMap::new();
        
        for entry in std::fs::read_dir(scope_path)
            .map_err(|e| RhemaError::IoError(e))?
        {
            let entry = entry.map_err(|e| RhemaError::IoError(e))?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    files.insert(file_name.to_string(), path);
                }
            }
        }
        
        Ok(files)
    }
    
    /// Get a specific file path
    pub fn get_file(&self, filename: &str) -> Option<&PathBuf> {
        self.files.get(filename)
    }
    
    /// Check if a file exists in this scope
    pub fn has_file(&self, filename: &str) -> bool {
        self.files.contains_key(filename)
    }
    
    /// Get the relative path from repository root
    pub fn relative_path(&self, repo_root: &Path) -> Result<String, RhemaError> {
        let relative = self.path.strip_prefix(repo_root)
            .map_err(|_| RhemaError::ConfigError("Scope path not in repository".to_string()))?;
        Ok(relative.to_string_lossy().to_string())
    }
    
    /// Get dependencies as scope paths
    pub fn get_dependency_paths(&self) -> Vec<String> {
        self.definition.dependencies
            .as_ref()
            .map(|deps| deps.iter().map(|d| d.path.clone()).collect())
            .unwrap_or_default()
    }
}

/// Discover all scopes in a repository
pub fn discover_scopes(repo_root: &Path) -> Result<Vec<Scope>, RhemaError> {
    let mut scopes = Vec::new();
    
    for entry in WalkDir::new(repo_root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.is_dir() && path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
            if let Ok(scope) = Scope::new(path.to_path_buf()) {
                scopes.push(scope);
            }
        }
    }
    
    Ok(scopes)
}

/// Get a specific scope by path
pub fn get_scope(repo_root: &Path, scope_path: &str) -> Result<Scope, RhemaError> {
    let full_path = if scope_path.starts_with('/') {
        PathBuf::from(scope_path)
    } else {
        repo_root.join(scope_path)
    };
    
    let rhema_path = if full_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
        full_path
    } else {
        full_path.join(".rhema")
    };
    
    if !rhema_path.exists() {
        return Err(RhemaError::ScopeNotFound(
            format!("Scope not found: {}", scope_path)
        ));
    }
    
    Scope::new(rhema_path)
}

/// Build a dependency graph from scopes
pub fn build_dependency_graph(scopes: &[Scope]) -> Result<HashMap<String, Vec<String>>, RhemaError> {
    let mut graph = HashMap::new();
    
    for scope in scopes {
        let scope_path = scope.relative_path(&scope.path.parent().unwrap())?;
        let dependencies = scope.get_dependency_paths();
        graph.insert(scope_path, dependencies);
    }
    
    // Validate for circular dependencies
    validate_dependency_graph(&graph)?;
    
    Ok(graph)
}

/// Validate that the dependency graph has no cycles
fn validate_dependency_graph(graph: &HashMap<String, Vec<String>>) -> Result<(), RhemaError> {
    let mut visited = std::collections::HashSet::new();
    let mut rec_stack = std::collections::HashSet::new();
    
    for node in graph.keys() {
        if !visited.contains(node) {
            if has_cycle(graph, node, &mut visited, &mut rec_stack) {
                return Err(RhemaError::CircularDependency(
                    format!("Circular dependency detected involving {}", node)
                ));
            }
        }
    }
    
    Ok(())
}

/// Check for cycles in the dependency graph using DFS
fn has_cycle(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    visited: &mut std::collections::HashSet<String>,
    rec_stack: &mut std::collections::HashSet<String>,
) -> bool {
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());
    
    if let Some(dependencies) = graph.get(node) {
        for dep in dependencies {
            if !visited.contains(dep) {
                if has_cycle(graph, dep, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(dep) {
                return true;
            }
        }
    }
    
    rec_stack.remove(node);
    false
}

/// Get scope hierarchy (parent/child relationships)
pub fn get_scope_hierarchy(scopes: &[Scope], repo_root: &Path) -> Result<HashMap<String, Vec<String>>, RhemaError> {
    let mut hierarchy = HashMap::new();
    
    for scope in scopes {
        let scope_rel_path = scope.relative_path(repo_root)?;
        let scope_dir = scope.path.parent().unwrap();
        let _scope_dir_rel = scope_dir.strip_prefix(repo_root)
            .map_err(|_| RhemaError::ConfigError("Invalid scope path".to_string()))?;
        
        let mut children = Vec::new();
        
        for other_scope in scopes {
            if other_scope.path != scope.path {
                let other_dir = other_scope.path.parent().unwrap();
                if other_dir.starts_with(scope_dir) && other_dir != scope_dir {
                    let child_rel_path = other_scope.relative_path(repo_root)?;
                    children.push(child_rel_path);
                }
            }
        }
        
        hierarchy.insert(scope_rel_path, children);
    }
    
    Ok(hierarchy)
}

/// Find the nearest scope for a given file path
pub fn find_nearest_scope<'a>(file_path: &Path, scopes: &'a [Scope]) -> Option<&'a Scope> {
    let mut nearest_scope = None;
    let mut max_common_prefix = 0;
    
    for scope in scopes {
        let scope_dir = scope.path.parent().unwrap();
        
        if file_path.starts_with(scope_dir) {
            let common_components = scope_dir.components().count();
            if common_components > max_common_prefix {
                max_common_prefix = common_components;
                nearest_scope = Some(scope);
            }
        }
    }
    
    nearest_scope
}

/// Get all scopes that contain a specific file type
pub fn get_scopes_with_file<'a>(scopes: &'a [Scope], filename: &str) -> Vec<&'a Scope> {
    scopes.iter()
        .filter(|scope| scope.has_file(filename))
        .collect()
}

/// Validate scope relationships
pub fn validate_scope_relationships(scopes: &[Scope], repo_root: &Path) -> Result<(), RhemaError> {
    let graph = build_dependency_graph(scopes)?;
    
    // Check that all referenced dependencies exist
    for (scope_path, dependencies) in &graph {
        for dep in dependencies {
            let dep_path = if dep.starts_with('/') {
                PathBuf::from(dep)
            } else {
                repo_root.join(dep)
            };
            
            let rhema_path = if dep_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
                dep_path
            } else {
                dep_path.join(".rhema")
            };
            
            if !rhema_path.exists() {
                return Err(RhemaError::ScopeNotFound(
                    format!("Dependency not found: {} (referenced by {})", dep, scope_path)
                ));
            }
        }
    }
    
    Ok(())
} 