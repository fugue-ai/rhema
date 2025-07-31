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

use crate::{
    schema::{
        DependencyType, LockMetadata, LockPerformanceMetrics, LockedDependency, LockedScope,
        RhemaLock, ValidationStatus, Validatable,
    },
    scope::Scope,
    RhemaError, RhemaResult,
};
use chrono::Utc;
use log::{debug, info, warn};
use semver::{Version, VersionReq};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

use super::resolver::DependencyResolver;
use crate::schema::ResolutionStrategy;
use super::validator::LockValidator;
use super::cache::{LockFileCache, CacheConfig, CacheKey, get_global_cache};

/// Configuration for lock file generation
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Strategy for dependency resolution
    pub resolution_strategy: ResolutionStrategy,
    /// Whether to allow circular dependencies
    pub allow_circular_dependencies: bool,
    /// Whether to generate checksums for integrity verification
    pub generate_checksums: bool,
    /// Whether to validate the generated lock file
    pub validate_output: bool,
    /// Maximum depth for dependency resolution
    pub max_resolution_depth: usize,
    /// Whether to use caching for performance
    pub enable_caching: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            resolution_strategy: ResolutionStrategy::Latest,
            allow_circular_dependencies: false,
            generate_checksums: true,
            validate_output: true,
            max_resolution_depth: 10,
            enable_caching: true,
        }
    }
}

/// Lock file generation engine
pub struct LockGenerator {
    config: GeneratorConfig,
    resolver: DependencyResolver,
    validator: LockValidator,
    cache: HashMap<String, LockedScope>,
    performance_metrics: LockPerformanceMetrics,
}

impl LockGenerator {
    /// Create a new lock generator with default configuration
    pub fn new() -> Self {
        Self::with_config(GeneratorConfig::default())
    }

    /// Create a new lock generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Self {
        Self {
            resolver: DependencyResolver::new(config.resolution_strategy.clone()),
            validator: LockValidator::new(),
            cache: HashMap::new(),
            performance_metrics: LockPerformanceMetrics::new(0),
            config,
        }
    }

    /// Generate a complete lock file for the repository
    pub fn generate(&mut self, repo_path: &Path) -> RhemaResult<RhemaLock> {
        let start_time = Instant::now();
        info!("Starting lock file generation for repository: {}", repo_path.display());

        // Check cache first if caching is enabled
        if self.config.enable_caching {
            if let Ok(cache) = get_global_cache() {
                if let Ok(Some(cached_lock)) = cache.get_lock_file(repo_path) {
                    info!("Using cached lock file for repository: {}", repo_path.display());
                    return Ok(cached_lock);
                }
            }
        }

        // Discover all scopes in the repository
        let scopes = self.discover_scopes(repo_path)?;
        info!("Discovered {} scopes in repository", scopes.len());

        // Build dependency graph and detect cycles
        let dependency_graph = self.build_dependency_graph(&scopes)?;
        let circular_dependencies = self.detect_circular_dependencies(&dependency_graph)?;

        if !circular_dependencies.is_empty() && !self.config.allow_circular_dependencies {
            return Err(RhemaError::CircularDependency(
                format!("Circular dependencies detected: {:?}", circular_dependencies)
            ));
        }

        // Resolve dependencies for each scope
        let locked_scopes = self.resolve_all_dependencies(&scopes, &dependency_graph)?;

        // Create the lock file
        let mut lock_file = RhemaLock::new("rhema-lock-generator");
        lock_file.scopes = locked_scopes;

        // Update performance metrics
        self.performance_metrics.generation_time_ms = start_time.elapsed().as_millis() as u64;
        lock_file.metadata.performance_metrics = Some(self.performance_metrics.clone());

        // Update metadata
        self.update_lock_metadata(&mut lock_file, &circular_dependencies);

        // Generate checksum for integrity verification
        if self.config.generate_checksums {
            lock_file.update_checksum();
        }

        // Validate the generated lock file
        if self.config.validate_output {
            self.validator.validate(&lock_file)?;
        }

        // Cache the generated lock file if caching is enabled
        if self.config.enable_caching {
            if let Ok(cache) = get_global_cache() {
                if let Err(e) = cache.set_lock_file(repo_path, &lock_file, Some(3600)) {
                    warn!("Failed to cache lock file: {}", e);
                } else {
                    debug!("Cached lock file for repository: {}", repo_path.display());
                }
            }
        }

        info!("Lock file generation completed successfully");
        debug!("Generated lock file with {} scopes and {} dependencies", 
               lock_file.metadata.total_scopes, lock_file.metadata.total_dependencies);

        Ok(lock_file)
    }

    /// Update an existing lock file
    pub fn update(&mut self, existing_lock: &mut RhemaLock, repo_path: &Path) -> RhemaResult<()> {
        let start_time = Instant::now();
        info!("Updating existing lock file for repository: {}", repo_path.display());

        // Discover current scopes
        let current_scopes = self.discover_scopes(repo_path)?;
        
        // Check for changes in existing scopes
        let mut updated_scopes = HashMap::new();
        
        for scope in &current_scopes {
            let scope_path = scope.relative_path(repo_path)?;
            
            if let Some(existing_scope) = existing_lock.get_scope(&scope_path) {
                // Check if scope has changed
                if self.has_scope_changed(&scope, existing_scope, repo_path)? {
                    info!("Scope {} has changed, updating dependencies", scope_path);
                    let locked_scope = self.resolve_scope_dependencies(&scope, repo_path)?;
                    updated_scopes.insert(scope_path, locked_scope);
                } else {
                    // Keep existing scope
                    updated_scopes.insert(scope_path, existing_scope.clone());
                }
            } else {
                // New scope
                info!("New scope discovered: {}", scope_path);
                let locked_scope = self.resolve_scope_dependencies(&scope, repo_path)?;
                updated_scopes.insert(scope_path, locked_scope);
            }
        }

        // Remove scopes that no longer exist
        let current_scope_paths: HashSet<_> = current_scopes
            .iter()
            .map(|s| s.relative_path(repo_path).unwrap())
            .collect();
        
        let existing_scope_paths: HashSet<_> = existing_lock.scopes.keys().cloned().collect();
        let removed_scopes: Vec<_> = existing_scope_paths.difference(&current_scope_paths).collect();
        
        for removed_scope in removed_scopes {
            info!("Removing scope that no longer exists: {}", removed_scope);
        }

        // Update the lock file
        existing_lock.scopes = updated_scopes;
        existing_lock.generated_at = Utc::now();
        
        // Update performance metrics
        self.performance_metrics.generation_time_ms = start_time.elapsed().as_millis() as u64;
        existing_lock.metadata.performance_metrics = Some(self.performance_metrics.clone());

        // Update metadata
        self.update_lock_metadata(existing_lock, &[]);

        // Regenerate checksum
        if self.config.generate_checksums {
            existing_lock.update_checksum();
        }

        info!("Lock file update completed successfully");
        Ok(())
    }

    /// Discover all scopes in the repository
    fn discover_scopes(&self, repo_path: &Path) -> RhemaResult<Vec<Scope>> {
        debug!("Discovering scopes in repository: {}", repo_path.display());
        
        let scopes = crate::scope::discover_scopes(repo_path)?;
        
        // Validate each scope
        for scope in &scopes {
            scope.definition.validate()?;
        }
        
        info!("Discovered {} scopes", scopes.len());
        Ok(scopes)
    }

    /// Build dependency graph from scopes
    fn build_dependency_graph(&self, scopes: &[Scope]) -> RhemaResult<HashMap<String, Vec<String>>> {
        debug!("Building dependency graph for {} scopes", scopes.len());
        
        let mut graph = HashMap::new();
        
        for scope in scopes {
            let scope_path = scope.relative_path(&scope.path.parent().unwrap_or(&scope.path))?;
            let mut dependencies = Vec::new();
            
            if let Some(deps) = &scope.definition.dependencies {
                for dep in deps {
                    dependencies.push(dep.path.clone());
                }
            }
            
            graph.insert(scope_path, dependencies);
        }
        
        Ok(graph)
    }

    /// Detect circular dependencies in the dependency graph
    fn detect_circular_dependencies(&self, graph: &HashMap<String, Vec<String>>) -> RhemaResult<Vec<Vec<String>>> {
        debug!("Detecting circular dependencies");
        
        let mut circular_deps = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node in graph.keys() {
            if !visited.contains(node) {
                let mut path = Vec::new();
                if self.has_cycle_dfs(graph, node, &mut visited, &mut rec_stack, &mut path) {
                    circular_deps.push(path);
                }
            }
        }
        
        if !circular_deps.is_empty() {
            warn!("Detected {} circular dependency chains", circular_deps.len());
            for (i, cycle) in circular_deps.iter().enumerate() {
                warn!("Circular dependency {}: {}", i + 1, cycle.join(" -> "));
            }
        }
        
        Ok(circular_deps)
    }

    /// DFS to detect cycles in dependency graph
    fn has_cycle_dfs(
        &self,
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        
        if let Some(dependencies) = graph.get(node) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    if self.has_cycle_dfs(graph, dep, visited, rec_stack, path) {
                        return true;
                    }
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    return true;
                }
            }
        }
        
        rec_stack.remove(node);
        path.pop();
        false
    }

    /// Resolve dependencies for all scopes
    fn resolve_all_dependencies(
        &mut self,
        scopes: &[Scope],
        dependency_graph: &HashMap<String, Vec<String>>,
    ) -> RhemaResult<HashMap<String, LockedScope>> {
        debug!("Resolving dependencies for {} scopes", scopes.len());
        
        let mut locked_scopes = HashMap::new();
        
        for scope in scopes {
            let scope_path = scope.relative_path(&scope.path.parent().unwrap_or(&scope.path))?;
            
            // Check cache first
            if self.config.enable_caching {
                if let Some(cached_scope) = self.cache.get(&scope_path) {
                    debug!("Using cached scope for: {}", scope_path);
                    locked_scopes.insert(scope_path, cached_scope.clone());
                    continue;
                }
            }
            
            let locked_scope = self.resolve_scope_dependencies(scope, &scope.path.parent().unwrap_or(&scope.path))?;
            
            // Cache the result
            if self.config.enable_caching {
                self.cache.insert(scope_path.clone(), locked_scope.clone());
            }
            
            locked_scopes.insert(scope_path, locked_scope);
        }
        
        Ok(locked_scopes)
    }

    /// Resolve dependencies for a single scope
    fn resolve_scope_dependencies(&mut self, scope: &Scope, repo_path: &Path) -> RhemaResult<LockedScope> {
        let scope_path = scope.relative_path(repo_path)?;
        let scope_path_str = scope_path.to_string();
        
        // Check cache first if caching is enabled
        if self.config.enable_caching {
            // Try global cache first
            if let Ok(cache) = get_global_cache() {
                if let Ok(Some(cached_scope)) = cache.get_scope(&Path::new(&scope_path)) {
                    debug!("Using cached scope from global cache for: {}", scope_path_str);
                    return Ok(cached_scope);
                }
            }
            
            // Fall back to local cache
            if let Some(cached_scope) = self.cache.get(&scope_path_str) {
                debug!("Using cached scope from local cache for: {}", scope_path_str);
                return Ok(cached_scope.clone());
            }
        }
        
        debug!("Resolving dependencies for scope: {}", scope_path);
        
        let mut locked_scope = LockedScope::new(&scope.definition.version, &scope_path);
        
        // Generate source checksum if enabled
        if self.config.generate_checksums {
            locked_scope.source_checksum = Some(self.generate_scope_checksum(scope)?);
        }
        
        // Resolve dependencies
        if let Some(dependencies) = &scope.definition.dependencies {
            for dep in dependencies {
                let locked_dep = self.resolve_dependency(dep, repo_path)?;
                locked_scope.add_dependency(dep.path.clone(), locked_dep);
            }
        }
        
        // Check for circular dependencies
        locked_scope.has_circular_dependencies = self.check_scope_circular_dependencies(scope, repo_path)?;
        
        // Cache the result if caching is enabled
        if self.config.enable_caching {
            // Cache in global cache
            if let Ok(cache) = get_global_cache() {
                if let Err(e) = cache.set_scope(&Path::new(&scope_path), &locked_scope, Some(1800)) {
                    warn!("Failed to cache scope in global cache: {}", e);
                }
            }
            
            // Cache in local cache
            self.cache.insert(scope_path_str, locked_scope.clone());
        }
        
        Ok(locked_scope)
    }

    /// Resolve a single dependency
    fn resolve_dependency(&mut self, dep: &crate::schema::ScopeDependency, repo_path: &Path) -> RhemaResult<LockedDependency> {
        self.performance_metrics.increment_resolution_attempts();
        
        // Check cache first if caching is enabled
        if self.config.enable_caching {
            let constraint = dep.version.as_deref().unwrap_or("latest");
            if let Ok(cache) = get_global_cache() {
                if let Ok(Some(cached_dep)) = cache.get_dependency(&dep.path, constraint) {
                    debug!("Using cached dependency: {} with constraint: {}", dep.path, constraint);
                    return Ok(cached_dep);
                }
            }
        }
        
        debug!("Resolving dependency: {} at path: {}", dep.path, dep.path);
        
        // Find the dependency scope
        let dep_path = repo_path.join(&dep.path);
        if !dep_path.exists() {
            return Err(RhemaError::FileNotFound(format!(
                "Dependency scope not found: {}",
                dep_path.display()
            )));
        }
        
        let dep_scope = Scope::new(dep_path)?;
        
        // Resolve version constraint
        let resolved_version = if let Some(constraint) = &dep.version {
            self.resolve_version_constraint(&dep_scope.definition.version, constraint)?
        } else {
            dep_scope.definition.version.clone()
        };
        
        // Determine dependency type
        let dependency_type = match dep.dependency_type.as_str() {
            "required" => DependencyType::Required,
            "optional" => DependencyType::Optional,
            "peer" => DependencyType::Peer,
            "development" => DependencyType::Development,
            "build" => DependencyType::Build,
            _ => {
                warn!("Unknown dependency type: {}, defaulting to required", dep.dependency_type);
                DependencyType::Required
            }
        };
        
        // Create locked dependency
        let mut locked_dep = LockedDependency::new(&resolved_version, &dep.path, dependency_type);
        
        // Set original constraint if provided
        if let Some(constraint) = &dep.version {
            locked_dep.set_original_constraint(constraint);
        }
        
        // Generate checksum if enabled
        if self.config.generate_checksums {
            locked_dep.update_checksum();
        }
        
        // Resolve transitive dependencies
        if let Some(transitive_deps) = &dep_scope.definition.dependencies {
            for transitive_dep in transitive_deps {
                locked_dep.add_dependency(&transitive_dep.path);
            }
        }
        
        // Cache the result if caching is enabled
        if self.config.enable_caching {
            let constraint = dep.version.as_deref().unwrap_or("latest");
            if let Ok(cache) = get_global_cache() {
                if let Err(e) = cache.set_dependency(&dep.path, constraint, &locked_dep, Some(900)) {
                    warn!("Failed to cache dependency: {}", e);
                }
            }
        }
        
        Ok(locked_dep)
    }

    /// Resolve version constraint using semantic versioning
    fn resolve_version_constraint(&self, current_version: &str, constraint: &str) -> RhemaResult<String> {
        debug!("Resolving version constraint: {} for current version: {}", constraint, current_version);
        
        let current_ver = Version::parse(current_version)
            .map_err(|e| RhemaError::InvalidVersion(format!("Invalid current version: {}", e)))?;
        
        let version_req = VersionReq::parse(constraint)
            .map_err(|e| RhemaError::InvalidVersion(format!("Invalid version constraint: {}", e)))?;
        
        // Check if current version satisfies the constraint
        if version_req.matches(&current_ver) {
            return Ok(current_version.to_string());
        }
        
        // If not, try to find a compatible version
        match self.config.resolution_strategy {
            ResolutionStrategy::Latest => {
                // For now, return current version and log a warning
                warn!("Version constraint '{}' not satisfied by current version '{}', using current version", 
                      constraint, current_version);
                Ok(current_version.to_string())
            }
            ResolutionStrategy::Earliest => {
                // Return the earliest compatible version (current for now)
                Ok(current_version.to_string())
            }
            ResolutionStrategy::Pinned => {
                // Return the exact version specified in constraint
                Ok(constraint.to_string())
            }
            ResolutionStrategy::Range => {
                // Return a version within the range
                Ok(current_version.to_string())
            }
            ResolutionStrategy::Compatible => {
                // Return a compatible version
                Ok(current_version.to_string())
            }
        }
    }

    /// Generate checksum for a scope
    fn generate_scope_checksum(&self, scope: &Scope) -> RhemaResult<String> {
        let mut hasher = Sha256::new();
        
        // Hash the scope definition
        let scope_yaml = serde_yaml::to_string(&scope.definition)
            .map_err(|e| RhemaError::SerializationError(e.to_string()))?;
        hasher.update(scope_yaml.as_bytes());
        
        // Hash all files in the scope
        for (_, file_path) in &scope.files {
            if file_path.exists() {
                let content = std::fs::read(file_path)
                    .map_err(|e| RhemaError::IoError(e))?;
                hasher.update(&content);
            }
        }
        
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Check if a scope has circular dependencies
    fn check_scope_circular_dependencies(&self, scope: &Scope, repo_path: &Path) -> RhemaResult<bool> {
        if let Some(dependencies) = &scope.definition.dependencies {
            for dep in dependencies {
                let dep_path = repo_path.join(&dep.path);
                if dep_path.exists() {
                    // Simple check: if dependency path contains current scope path, it's circular
                    let scope_path_str = scope.relative_path(repo_path)?;
                    if dep.path.contains(&scope_path_str) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    /// Check if a scope has changed since last lock
    fn has_scope_changed(&self, scope: &Scope, locked_scope: &LockedScope, repo_path: &Path) -> RhemaResult<bool> {
        // Check if version has changed
        if scope.definition.version != locked_scope.version {
            return Ok(true);
        }
        
        // Check if source checksum has changed
        if let Some(expected_checksum) = &locked_scope.source_checksum {
            let current_checksum = self.generate_scope_checksum(scope)?;
            if current_checksum != *expected_checksum {
                return Ok(true);
            }
        }
        
        // Check if dependencies have changed
        let current_deps: HashSet<_> = scope.definition.dependencies
            .as_ref()
            .map(|deps| deps.iter().map(|d| d.path.clone()).collect())
            .unwrap_or_default();
        
        let locked_deps: HashSet<_> = locked_scope.dependencies.keys().cloned().collect();
        
        if current_deps != locked_deps {
            return Ok(true);
        }
        
        Ok(false)
    }

    /// Update lock file metadata
    fn update_lock_metadata(&self, lock_file: &mut RhemaLock, circular_dependencies: &[Vec<String>]) {
        let total_scopes = lock_file.scopes.len() as u32;
        let total_dependencies: u32 = lock_file.scopes
            .values()
            .map(|scope| scope.dependencies.len() as u32)
            .sum();
        
        let circular_count = circular_dependencies.len() as u32;
        
        lock_file.metadata.total_scopes = total_scopes;
        lock_file.metadata.total_dependencies = total_dependencies;
        lock_file.metadata.circular_dependencies = circular_count;
        lock_file.metadata.resolution_strategy = self.config.resolution_strategy.clone();
        
        // Set validation status based on circular dependencies
        if circular_count > 0 {
            lock_file.metadata.validation_status = ValidationStatus::Warning;
            lock_file.metadata.add_validation_message(
                &format!("{} circular dependency chains detected", circular_count)
            );
        } else {
            lock_file.metadata.validation_status = ValidationStatus::Valid;
        }
        
        lock_file.metadata.last_validated = Some(Utc::now());
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &LockPerformanceMetrics {
        &self.performance_metrics
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        
        // Also clear global cache if available
        if let Ok(cache) = get_global_cache() {
            if let Err(e) = cache.clear() {
                warn!("Failed to clear global cache: {}", e);
            } else {
                info!("Global cache cleared");
            }
        }
        
        info!("Lock generator cache cleared");
    }
}

impl Default for LockGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{RhemaScope, ScopeDependency};
    use std::path::PathBuf;
    use std::collections::HashMap;

    #[test]
    fn test_generator_creation() {
        let generator = LockGenerator::new();
        assert_eq!(generator.config.resolution_strategy, ResolutionStrategy::Latest);
        assert!(!generator.config.allow_circular_dependencies);
        assert!(generator.config.generate_checksums);
    }

    #[test]
    fn test_custom_config() {
        let config = GeneratorConfig {
            resolution_strategy: ResolutionStrategy::Pinned,
            allow_circular_dependencies: true,
            generate_checksums: false,
            validate_output: false,
            max_resolution_depth: 5,
            enable_caching: false,
        };
        
        let generator = LockGenerator::with_config(config);
        assert_eq!(generator.config.resolution_strategy, ResolutionStrategy::Pinned);
        assert!(generator.config.allow_circular_dependencies);
        assert!(!generator.config.generate_checksums);
    }

    #[test]
    fn test_version_constraint_resolution() {
        let generator = LockGenerator::new();
        
        // Test exact version match
        let result = generator.resolve_version_constraint("1.2.3", "1.2.3").unwrap();
        assert_eq!(result, "1.2.3");
        
        // Test compatible version
        let result = generator.resolve_version_constraint("1.2.3", "^1.2.0").unwrap();
        assert_eq!(result, "1.2.3");
        
        // Test incompatible version
        let result = generator.resolve_version_constraint("1.2.3", "^2.0.0").unwrap();
        assert_eq!(result, "1.2.3"); // Should return current version with warning
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = HashMap::new();
        graph.insert("scope1".to_string(), vec!["scope2".to_string()]);
        graph.insert("scope2".to_string(), vec!["scope3".to_string()]);
        graph.insert("scope3".to_string(), vec!["scope1".to_string()]);
        
        let generator = LockGenerator::new();
        let circular_deps = generator.detect_circular_dependencies(&graph).unwrap();
        
        assert!(!circular_deps.is_empty());
        assert!(circular_deps[0].contains(&"scope1".to_string()));
    }

    #[test]
    fn test_scope_checksum_generation() {
        let scope_def = RhemaScope {
            name: "test-scope".to_string(),
            scope_type: "service".to_string(),
            description: Some("Test scope".to_string()),
            version: "1.0.0".to_string(),
            schema_version: Some("1.0.0".to_string()),
            dependencies: None,
            protocol_info: None,
            custom: HashMap::new(),
        };
        
        let scope = Scope {
            path: PathBuf::from("/tmp/test"),
            definition: scope_def,
            files: HashMap::new(),
        };
        
        let generator = LockGenerator::new();
        let checksum = generator.generate_scope_checksum(&scope).unwrap();
        
        assert_eq!(checksum.len(), 64); // SHA-256 hex string length
    }
} 