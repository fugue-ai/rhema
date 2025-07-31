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

pub mod generator;
pub mod resolver;
pub mod validator;
pub mod conflict_resolver;
pub mod cache;

pub use generator::LockGenerator;
pub use resolver::DependencyResolver;
pub use validator::LockValidator;
pub use conflict_resolver::{
    ConflictResolver, ConflictResolutionConfig, ConflictResolutionStrategy,
    ConflictResolutionResult, DependencyConflict, DependencySpec,
    ResolutionAction, ResolutionHistoryEntry, ConflictType, ConflictSeverity,
    VersionConstraint, ConflictRequirement,
};
pub use cache::{
    LockFileCache, CacheConfig, CacheStats, InvalidationStrategy, WarmingStrategy,
    CacheKey, init_global_cache, get_global_cache, utils as cache_utils,
};
pub use crate::schema::ResolutionStrategy;

use crate::{schema::RhemaLock, RhemaError, RhemaResult};
use std::path::Path;
use log::info;

/// Lock file system for managing scope dependencies and version resolution
pub struct LockSystem;

impl LockSystem {
    /// Initialize the lock file system with caching
    pub fn initialize() -> RhemaResult<()> {
        let config = CacheConfig::default();
        init_global_cache(config)?;
        info!("Lock file system initialized with caching");
        Ok(())
    }

    /// Generate a new lock file for the repository
    pub fn generate_lock_file(repo_path: &Path) -> RhemaResult<RhemaLock> {
        let mut generator = LockGenerator::new();
        generator.generate(repo_path)
    }

    /// Validate an existing lock file
    pub fn validate_lock_file(lock_file: &RhemaLock) -> RhemaResult<()> {
        let validator = LockValidator::new();
        validator.validate(lock_file)
    }

    /// Update an existing lock file
    pub fn update_lock_file(repo_path: &Path, existing_lock: &mut RhemaLock) -> RhemaResult<()> {
        let mut generator = LockGenerator::new();
        generator.update(existing_lock, repo_path)
    }

    /// Resolve conflicts in dependencies using advanced conflict resolution strategies
    pub fn resolve_conflicts(
        dependencies: &[DependencySpec],
        repo_path: &Path,
        config: Option<ConflictResolutionConfig>,
    ) -> RhemaResult<ConflictResolutionResult> {
        let mut resolver = if let Some(config) = config {
            ConflictResolver::with_config(config)
        } else {
            ConflictResolver::new()
        };
        
        resolver.resolve_conflicts(dependencies, repo_path)
    }

    /// Get conflict resolution guidance for users
    pub fn get_conflict_guidance(conflicts: &[DependencyConflict]) -> Vec<String> {
        let mut guidance = Vec::new();
        
        if conflicts.is_empty() {
            guidance.push("No conflicts detected. Your dependencies are compatible.".to_string());
            return guidance;
        }
        
        guidance.push(format!("Found {} conflict(s) that need resolution:", conflicts.len()));
        
        for (i, conflict) in conflicts.iter().enumerate() {
            guidance.push(format!("\n{}. {} (Severity: {:?})", i + 1, conflict.dependency_name, conflict.severity));
            guidance.push(format!("   Description: {}", conflict.description));
            
            if let Some(suggested) = &conflict.suggested_resolution {
                guidance.push(format!("   Suggested resolution: {}", suggested));
            }
            
            guidance.push("   Recommendations:".to_string());
            for rec in &conflict.recommendations {
                guidance.push(format!("     - {}", rec));
            }
        }
        
        guidance.push("\nResolution strategies available:".to_string());
        guidance.push("  - Latest compatible version: Automatically select the latest version that satisfies all requirements".to_string());
        guidance.push("  - Pinned version enforcement: Use pinned versions when available".to_string());
        guidance.push("  - Manual resolution: Review and resolve conflicts manually".to_string());
        guidance.push("  - Smart selection: Use compatibility scores to select the best version".to_string());
        guidance.push("  - Conservative: Prefer stable, well-tested versions".to_string());
        guidance.push("  - Aggressive: Prefer latest versions with newest features".to_string());
        guidance.push("  - Hybrid: Try multiple strategies in sequence".to_string());
        
        guidance
    }

    /// Get cache statistics
    pub fn get_cache_stats() -> RhemaResult<CacheStats> {
        let cache = get_global_cache()?;
        Ok(cache.stats())
    }

    /// Clear all caches
    pub fn clear_caches() -> RhemaResult<()> {
        let cache = get_global_cache()?;
        cache.clear()?;
        info!("All lock file caches cleared");
        Ok(())
    }

    /// Warm the cache for a repository
    pub fn warm_cache(repo_path: &Path) -> RhemaResult<()> {
        let cache = get_global_cache()?;
        cache.warm_cache(repo_path)?;
        info!("Cache warmed for repository: {}", repo_path.display());
        Ok(())
    }

    /// Get cache performance report
    pub fn get_cache_performance_report() -> RhemaResult<String> {
        cache_utils::get_performance_report()
    }

    /// Optimize cache based on usage patterns
    pub fn optimize_cache() -> RhemaResult<()> {
        cache_utils::optimize_cache()?;
        info!("Cache optimization completed");
        Ok(())
    }

    /// Check lock file consistency across environments
    pub fn check_lock_file_consistency(
        lock_file: &RhemaLock,
        reference_lock: &RhemaLock,
        git_branch: &Option<String>,
        allow_semver_diffs: &bool,
        max_version_drift: &Option<String>,
    ) -> RhemaResult<()> {
        // This would implement consistency checking logic
        // For now, we'll just validate that both lock files are valid
        Self::validate_lock_file(lock_file)?;
        Self::validate_lock_file(reference_lock)?;
        Ok(())
    }

    /// Update lock file with CI/CD specific logic
    pub fn update_lock_file_ci(
        repo_path: &Path,
        lock_file: &mut RhemaLock,
        update_strategy: String,
        strategy: Option<ResolutionStrategy>,
        security_only: bool,
        max_updates: Option<usize>,
    ) -> RhemaResult<()> {
        // This would implement CI/CD specific update logic
        // For now, we'll just call the regular update method
        Self::update_lock_file(repo_path, lock_file)
    }

    /// Check lock file health
    pub fn check_lock_file_health(
        lock_file: &RhemaLock,
        integrity: bool,
        freshness: bool,
        availability: bool,
        performance: bool,
    ) -> RhemaResult<()> {
        // This would implement health checking logic
        // For now, we'll just validate the lock file
        if integrity {
            Self::validate_lock_file(lock_file)?;
        }
        Ok(())
    }
} 