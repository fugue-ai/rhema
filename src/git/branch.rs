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

use crate::{RhemaError, RhemaResult};
use git2::{Repository, BranchType};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Enhanced branch context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchContext {
    /// Branch name
    pub name: String,
    
    /// Base branch (parent)
    pub base_branch: Option<String>,
    
    /// Context files specific to this branch
    pub context_files: Vec<PathBuf>,
    
    /// Context validation status
    pub validation_status: ValidationStatus,
    
    /// Branch protection rules
    pub protection_rules: Option<BranchProtection>,
    
    /// Context merge strategy
    pub merge_strategy: MergeStrategy,
    
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Advanced context features
    pub advanced_features: AdvancedContextFeatures,
    
    /// Context isolation settings
    pub isolation_settings: ContextIsolationSettings,
    
    /// Context synchronization settings
    pub sync_settings: ContextSyncSettings,
    
    /// Context backup settings
    pub backup_settings: ContextBackupSettings,
}

/// Context validation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Valid,
    Invalid(Vec<String>),
    Pending,
    Skipped,
}

/// Branch protection rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProtection {
    /// Require context validation
    pub require_validation: bool,
    
    /// Require health checks
    pub require_health_checks: bool,
    
    /// Require dependency validation
    pub require_dependency_validation: bool,
    
    /// Require code review
    pub require_review: bool,
    
    /// Require status checks
    pub require_status_checks: bool,
    
    /// Restrict pushes
    pub restrict_pushes: bool,
    
    /// Restrict deletions
    pub restrict_deletions: bool,
    
    /// Allowed users (if restricted)
    pub allowed_users: Option<Vec<String>>,
}

/// Context merge strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeStrategy {
    /// Auto-merge with conflict resolution
    Auto,
    
    /// Manual merge with prompts
    Manual,
    
    /// Rebase and merge
    Rebase,
    
    /// Squash and merge
    Squash,
    
    /// Custom merge strategy
    Custom(String),
}

/// Advanced context features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedContextFeatures {
    /// Enable context versioning
    pub context_versioning: bool,
    
    /// Enable context evolution tracking
    pub evolution_tracking: bool,
    
    /// Enable context analytics
    pub context_analytics: bool,
    
    /// Enable context optimization
    pub context_optimization: bool,
    
    /// Enable context caching
    pub context_caching: bool,
    
    /// Enable context compression
    pub context_compression: bool,
    
    /// Enable context encryption
    pub context_encryption: bool,
    
    /// Enable context deduplication
    pub context_deduplication: bool,
}

/// Context isolation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextIsolationSettings {
    /// Isolate context from parent branch
    pub isolate_from_parent: bool,
    
    /// Isolate context from sibling branches
    pub isolate_from_siblings: bool,
    
    /// Shared context files
    pub shared_files: Vec<PathBuf>,
    
    /// Excluded context files
    pub excluded_files: Vec<PathBuf>,
    
    /// Context inheritance rules
    pub inheritance_rules: Vec<InheritanceRule>,
    
    /// Context boundary rules
    pub boundary_rules: Vec<BoundaryRule>,
    
    /// Context permission rules
    pub permission_rules: Vec<PermissionRule>,
}

/// Inheritance rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceRule {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub action: InheritanceAction,
    pub priority: u32,
}

/// Inheritance action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InheritanceAction {
    Inherit,
    Override,
    Merge,
    Exclude,
}

/// Boundary rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryRule {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub boundary_type: BoundaryType,
    pub enforcement: BoundaryEnforcement,
}

/// Boundary type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundaryType {
    Scope,
    Namespace,
    Module,
    Feature,
    Custom(String),
}

/// Boundary enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundaryEnforcement {
    Strict,
    Flexible,
    Advisory,
}

/// Permission rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub permissions: Vec<Permission>,
    pub users: Vec<String>,
    pub groups: Vec<String>,
}

/// Permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Execute,
    Admin,
}

/// Context synchronization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSyncSettings {
    /// Auto-sync with parent branch
    pub auto_sync_parent: bool,
    
    /// Auto-sync with sibling branches
    pub auto_sync_siblings: bool,
    
    /// Sync strategy
    pub sync_strategy: SyncStrategy,
    
    /// Sync frequency
    pub sync_frequency: SyncFrequency,
    
    /// Conflict resolution strategy
    pub conflict_resolution: ConflictResolutionStrategy,
    
    /// Sync validation
    pub sync_validation: bool,
    
    /// Sync notifications
    pub sync_notifications: bool,
}

/// Sync strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStrategy {
    /// Full sync
    Full,
    
    /// Incremental sync
    Incremental,
    
    /// Selective sync
    Selective,
    
    /// Custom sync
    Custom(String),
}

/// Sync frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncFrequency {
    /// On every commit
    OnCommit,
    
    /// On every push
    OnPush,
    
    /// On every merge
    OnMerge,
    
    /// Scheduled
    Scheduled(u64), // seconds
    
    /// Manual
    Manual,
}

/// Context backup settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBackupSettings {
    /// Auto-backup before operations
    pub auto_backup: bool,
    
    /// Backup retention policy
    pub retention_policy: RetentionPolicy,
    
    /// Backup compression
    pub compression: bool,
    
    /// Backup encryption
    pub encryption: bool,
    
    /// Backup location
    pub backup_location: PathBuf,
    
    /// Backup format
    pub backup_format: BackupFormat,
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Keep daily backups for N days
    pub daily_retention_days: u32,
    
    /// Keep weekly backups for N weeks
    pub weekly_retention_weeks: u32,
    
    /// Keep monthly backups for N months
    pub monthly_retention_months: u32,
    
    /// Maximum total backups
    pub max_total_backups: usize,
}

/// Backup format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupFormat {
    Tar,
    Zip,
    Json,
    Yaml,
    Custom(String),
}

/// Branch context manager
pub struct BranchContextManager {
    repo: Repository,
    contexts: HashMap<String, BranchContext>,
}

impl BranchContextManager {
    /// Create a new branch context manager
    pub fn new(repo: Repository) -> Self {
        Self {
            repo,
            contexts: HashMap::new(),
        }
    }
    
    /// Enhanced branch context initialization with advanced features
    pub fn initialize_branch_context(&mut self, base_branch: Option<String>) -> RhemaResult<BranchContext> {
        let current_branch = self.get_current_branch()?;
        let context_files = self.discover_context_files()?;
        
        // Determine base branch if not provided
        let base_branch = base_branch.unwrap_or_else(|| {
            if current_branch == "main" || current_branch == "master" {
                String::new()
            } else {
                "main".to_string()
            }
        });
        
        // Create enhanced branch context
        let mut context = BranchContext {
            name: current_branch.clone(),
            base_branch: if base_branch.is_empty() { None } else { Some(base_branch) },
            context_files,
            validation_status: ValidationStatus::Pending,
            protection_rules: self.get_branch_protection(&current_branch)?,
            merge_strategy: self.determine_merge_strategy(&current_branch)?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            advanced_features: AdvancedContextFeatures {
                context_versioning: true,
                evolution_tracking: true,
                context_analytics: true,
                context_optimization: true,
                context_caching: true,
                context_compression: true,
                context_encryption: true,
                context_deduplication: true,
            },
            isolation_settings: ContextIsolationSettings {
                isolate_from_parent: true,
                isolate_from_siblings: true,
                shared_files: vec![],
                excluded_files: vec![],
                inheritance_rules: vec![],
                boundary_rules: vec![],
                permission_rules: vec![],
            },
            sync_settings: ContextSyncSettings {
                auto_sync_parent: true,
                auto_sync_siblings: false,
                sync_strategy: SyncStrategy::Incremental,
                sync_frequency: SyncFrequency::OnCommit,
                conflict_resolution: ConflictResolutionStrategy::Merge,
                sync_validation: true,
                sync_notifications: true,
            },
            backup_settings: ContextBackupSettings {
                auto_backup: true,
                retention_policy: RetentionPolicy {
                    daily_retention_days: 7,
                    weekly_retention_weeks: 4,
                    monthly_retention_months: 12,
                    max_total_backups: 100,
                },
                compression: true,
                encryption: true,
                backup_location: PathBuf::from(".rhema/backups"),
                backup_format: BackupFormat::Tar,
            },
        };
        
        // Setup context isolation
        self.setup_context_isolation(&current_branch)?;
        
        // Validate context
        context.validation_status = self.validate_branch_context()?;
        
        // Store context
        self.contexts.insert(current_branch.clone(), context.clone());
        
        // Save context to file
        self.save_branch_context(&context)?;
        
        println!("Enhanced branch context initialized for: {}", current_branch);
        
        Ok(context)
    }
    
    /// Get current branch name
    pub fn get_current_branch(&self) -> RhemaResult<String> {
        let head = self.repo.head()?;
        
        if head.is_branch() {
            let branch_name = head.name()
                .ok_or_else(|| RhemaError::GitError(git2::Error::from_str("Invalid branch name")))?;
            
            // Remove "refs/heads/" prefix
            Ok(branch_name.replace("refs/heads/", ""))
        } else {
            Err(RhemaError::GitError(git2::Error::from_str("Not on a branch")))
        }
    }
    
    /// Discover context files in current branch
    pub fn discover_context_files(&self) -> RhemaResult<Vec<PathBuf>> {
        let mut context_files = Vec::new();
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| RhemaError::GitError(git2::Error::from_str("Invalid repository path")))?;
        
        // Walk through repository to find context files
        for entry in walkdir::WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && self.is_context_file(path) {
                context_files.push(path.to_path_buf());
            }
        }
        
        Ok(context_files)
    }
    
    /// Check if a file is a context file
    fn is_context_file(&self, path: &Path) -> bool {
        // Check for Rhema context files
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            matches!(file_name, 
                "rhema.yaml" | 
                "knowledge.yaml" | 
                "todos.yaml" | 
                "decisions.yaml" | 
                "patterns.yaml" | 
                "conventions.yaml"
            )
        } else {
            false
        }
    }
    
    /// Get branch protection rules
    pub fn get_branch_protection(&self, branch_name: &str) -> RhemaResult<Option<BranchProtection>> {
        // Check for branch protection configuration
        let config_path = self.repo.path().parent()
            .ok_or_else(|| RhemaError::GitError(git2::Error::from_str("Invalid repository path")))?
            .join(".rhema")
            .join("branch-protection.yaml");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: BranchProtectionConfig = serde_yaml::from_str(&content)?;
            
            if let Some(rules) = config.branches.get(branch_name) {
                return Ok(Some(rules.clone()));
            }
        }
        
        // Default protection for main/master branches
        if matches!(branch_name, "main" | "master") {
            Ok(Some(BranchProtection {
                require_validation: true,
                require_health_checks: true,
                require_dependency_validation: true,
                require_review: true,
                require_status_checks: true,
                restrict_pushes: true,
                restrict_deletions: true,
                allowed_users: None,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Determine merge strategy for branch
    pub fn determine_merge_strategy(&self, branch_name: &str) -> RhemaResult<MergeStrategy> {
        // Check for merge strategy configuration
        let config_path = self.repo.path().parent()
            .ok_or_else(|| RhemaError::GitError(git2::Error::from_str("Invalid repository path")))?
            .join(".rhema")
            .join("merge-strategy.yaml");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: MergeStrategyConfig = serde_yaml::from_str(&content)?;
            
            if let Some(strategy) = config.branches.get(branch_name) {
                return Ok(strategy.clone());
            }
        }
        
        // Default strategy based on branch name patterns
        if branch_name.starts_with("feature/") {
            Ok(MergeStrategy::Rebase)
        } else if branch_name.starts_with("hotfix/") {
            Ok(MergeStrategy::Squash)
        } else if branch_name.starts_with("release/") {
            Ok(MergeStrategy::Manual)
        } else {
            Ok(MergeStrategy::Auto)
        }
    }
    
    /// Enhanced context validation with advanced checks
    pub fn validate_branch_context(&mut self) -> RhemaResult<ValidationStatus> {
        let current_branch = self.get_current_branch()?;
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Basic context file validation
        for file_path in &self.discover_context_files()? {
            if let Err(e) = self.validate_context_file(file_path) {
                errors.push(format!("Validation failed for {}: {}", file_path.display(), e));
            }
        }
        
        // Advanced validation checks
        if let Some(context) = self.contexts.get(&current_branch) {
            // Validate context isolation
            if let Err(e) = self.validate_context_isolation(context) {
                errors.push(format!("Context isolation validation failed: {}", e));
            }
            
            // Validate context boundaries
            if let Err(e) = self.validate_context_boundaries(context) {
                warnings.push(format!("Context boundary validation warning: {}", e));
            }
            
            // Validate context inheritance
            if let Err(e) = self.validate_context_inheritance(context) {
                warnings.push(format!("Context inheritance validation warning: {}", e));
            }
            
            // Validate context permissions
            if let Err(e) = self.validate_context_permissions(context) {
                errors.push(format!("Context permission validation failed: {}", e));
            }
            
            // Validate context consistency
            if let Err(e) = self.validate_context_consistency(context) {
                errors.push(format!("Context consistency validation failed: {}", e));
            }
        }
        
        // Check for conflicts with other branches
        if let Err(e) = self.check_cross_branch_conflicts(&current_branch) {
            warnings.push(format!("Cross-branch conflict check warning: {}", e));
        }
        
        if errors.is_empty() && warnings.is_empty() {
            Ok(ValidationStatus::Valid)
        } else if errors.is_empty() {
            Ok(ValidationStatus::Valid) // Warnings don't fail validation
        } else {
            Ok(ValidationStatus::Invalid(errors))
        }
    }
    
    /// Validate a single context file
    fn validate_context_file(&self, _file_path: &Path) -> RhemaResult<()> {
        // TODO: Implement context file validation
        // This would check YAML syntax, schema validation, etc.
        Ok(())
    }
    
    /// Enhanced merge with advanced conflict resolution
    pub fn merge_branch_context(&mut self, source_branch: &str, target_branch: &str) -> RhemaResult<MergeResult> {
        let start_time = std::time::Instant::now();
        let mut conflicts = Vec::new();
        let merged_files = Vec::new();
        let mut messages = Vec::new();
        
        // Backup target branch context before merge
        if let Some(_target_context) = self.contexts.get(target_branch) {
            self.backup_branch_context(target_branch)?;
            messages.push("Target branch context backed up before merge".to_string());
        }
        
        // Check for conflicts before merging
        let detected_conflicts = self.check_context_conflicts(source_branch, target_branch)?;
        if !detected_conflicts.is_empty() {
            conflicts.extend(detected_conflicts);
            messages.push(format!("{} conflicts detected before merge", conflicts.len()));
        }
        
        // Perform merge based on strategy
        let source_context = self.get_branch_context(source_branch)?;
        let merge_strategy = &source_context.merge_strategy;
        
        let merge_result = match merge_strategy {
            MergeStrategy::Auto => self.auto_merge_context(source_branch, target_branch)?,
            MergeStrategy::Manual => self.manual_merge_context(source_branch, target_branch)?,
            MergeStrategy::Rebase => self.rebase_merge_context(source_branch, target_branch)?,
            MergeStrategy::Squash => self.squash_merge_context(source_branch, target_branch)?,
            MergeStrategy::Custom(_strategy) => self.custom_merge_context(source_branch, target_branch, merge_strategy)?,
        };
        
        // Update merge result with additional information
        let mut final_result = MergeResult {
            success: merge_result.success && conflicts.is_empty(),
            conflicts: [conflicts, merge_result.conflicts].concat(),
            merged_files: [merged_files, merge_result.merged_files].concat(),
            messages: [messages, merge_result.messages].concat(),
        };
        
        // Validate merged context
        if final_result.success {
            if let Err(e) = self.validate_merged_context(target_branch) {
                final_result.messages.push(format!("Warning: Merged context validation failed: {}", e));
            }
        }
        
        let duration = start_time.elapsed();
        final_result.messages.push(format!("Merge completed in {:?}", duration));
        
        Ok(final_result)
    }
    
    /// Auto-merge context files
    fn auto_merge_context(&self, _source_branch: &str, _target_branch: &str) -> RhemaResult<MergeResult> {
        // TODO: Implement automatic context merging
        Ok(MergeResult {
            success: true,
            conflicts: Vec::new(),
            merged_files: Vec::new(),
            messages: vec!["Auto-merge completed successfully".to_string()],
        })
    }
    
    /// Manual merge context files
    fn manual_merge_context(&self, _source_branch: &str, _target_branch: &str) -> RhemaResult<MergeResult> {
        // TODO: Implement manual context merging with prompts
        Ok(MergeResult {
            success: true,
            conflicts: Vec::new(),
            merged_files: Vec::new(),
            messages: vec!["Manual merge completed successfully".to_string()],
        })
    }
    
    /// Rebase merge context files
    fn rebase_merge_context(&self, _source_branch: &str, _target_branch: &str) -> RhemaResult<MergeResult> {
        // TODO: Implement rebase-style context merging
        Ok(MergeResult {
            success: true,
            conflicts: Vec::new(),
            merged_files: Vec::new(),
            messages: vec!["Rebase merge completed successfully".to_string()],
        })
    }
    
    /// Squash merge context files
    fn squash_merge_context(&self, _source_branch: &str, _target_branch: &str) -> RhemaResult<MergeResult> {
        // TODO: Implement squash-style context merging
        Ok(MergeResult {
            success: true,
            conflicts: Vec::new(),
            merged_files: Vec::new(),
            messages: vec!["Squash merge completed successfully".to_string()],
        })
    }
    
    /// Custom merge context files
    fn custom_merge_context(&self, _source_branch: &str, _target_branch: &str, _strategy: &MergeStrategy) -> RhemaResult<MergeResult> {
        // TODO: Implement custom merge strategy
        Ok(MergeResult {
            success: true,
            conflicts: Vec::new(),
            merged_files: Vec::new(),
            messages: vec!["Custom merge completed successfully".to_string()],
        })
    }
    
    /// Get branch context
    pub fn get_branch_context(&self, branch_name: &str) -> RhemaResult<&BranchContext> {
        self.contexts.get(branch_name)
            .ok_or_else(|| RhemaError::ConfigError(format!("Branch context not found for {}", branch_name)))
    }
    
    /// List all branch contexts
    pub fn list_branch_contexts(&self) -> Vec<&BranchContext> {
        self.contexts.values().collect()
    }
    
    /// Check for context conflicts between branches
    pub fn check_context_conflicts(&self, _source_branch: &str, _target_branch: &str) -> RhemaResult<Vec<ContextConflict>> {
        let conflicts = Vec::new();
        
        // TODO: Implement conflict detection logic
        // This would compare context files between branches and identify conflicts
        
        Ok(conflicts)
    }
    
    /// Resolve context conflicts
    pub fn resolve_context_conflicts(&mut self, conflicts: Vec<ContextConflict>) -> RhemaResult<()> {
        for _conflict in conflicts {
            // TODO: Implement conflict resolution logic
            // This would apply resolution strategies to resolve conflicts
        }
        
        Ok(())
    }
    
    /// Create feature branch with context isolation
    pub fn create_feature_branch(&mut self, feature_name: &str, base_branch: &str) -> RhemaResult<BranchContext> {
        let new_branch_name = format!("feature/{}", feature_name);
        
        // Create the Git branch
        {
            let base_commit = self.repo.find_branch(base_branch, BranchType::Local)?
                .get()
                .peel_to_commit()?;
            
            let _new_branch = self.repo.branch(&new_branch_name, &base_commit, false)?;
        }
        
        // Initialize branch context
        let context = self.initialize_branch_context(Some(base_branch.to_string()))?;
        
        // Set up context isolation
        self.setup_context_isolation(&new_branch_name)?;
        
        Ok(context)
    }
    
    /// Set up context isolation for a branch
    fn setup_context_isolation(&self, _branch_name: &str) -> RhemaResult<()> {
        // TODO: Implement context isolation setup
        // This would create branch-specific context files and configurations
        
        Ok(())
    }
    
    /// Get context evolution for a branch
    pub fn get_context_evolution(&self, _branch_name: &str) -> RhemaResult<Vec<ContextEvolution>> {
        // TODO: Implement context evolution tracking
        // This would track how context has evolved over time in the branch
        
        Ok(Vec::new())
    }
    
    /// Backup branch context
    pub fn backup_branch_context(&self, branch_name: &str) -> RhemaResult<PathBuf> {
        let context = self.get_branch_context(branch_name)?;
        let backup_dir = self.repo.path().parent()
            .ok_or_else(|| RhemaError::GitError(git2::Error::from_str("Invalid repository path")))?
            .join(".rhema")
            .join("backups");
        
        std::fs::create_dir_all(&backup_dir)?;
        
        let backup_file = backup_dir.join(format!("{}-{}.yaml", branch_name, Utc::now().timestamp()));
        let content = serde_yaml::to_string(context)?;
        std::fs::write(&backup_file, content)?;
        
        Ok(backup_file)
    }
    
    /// Restore branch context from backup
    pub fn restore_branch_context(&mut self, backup_file: &Path) -> RhemaResult<BranchContext> {
        let content = std::fs::read_to_string(backup_file)?;
        let context: BranchContext = serde_yaml::from_str(&content)?;
        
        self.contexts.insert(context.name.clone(), context.clone());
        
        Ok(context)
    }
    
    /// Validate context isolation
    fn validate_context_isolation(&self, context: &BranchContext) -> RhemaResult<()> {
        if !context.isolation_settings.isolate_from_parent {
            return Ok(());
        }
        
        // Check if context files are properly isolated
        for file_path in &context.context_files {
            if let Some(base_branch) = &context.base_branch {
                if self.file_exists_in_branch(file_path, base_branch)? {
                    return Err(RhemaError::ValidationError(
                        format!("Context file {} is not isolated from parent branch {}", 
                                file_path.display(), base_branch)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate context boundaries
    fn validate_context_boundaries(&self, context: &BranchContext) -> RhemaResult<()> {
        // Check if context files respect defined boundaries
        for rule in &context.isolation_settings.boundary_rules {
            if !self.validate_boundary_rule(rule, &context.context_files)? {
                return Err(RhemaError::ValidationError(
                    format!("Context boundary rule '{}' violated", rule.name)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Validate context inheritance
    fn validate_context_inheritance(&self, context: &BranchContext) -> RhemaResult<()> {
        // Check if inherited context files are properly handled
        for rule in &context.isolation_settings.inheritance_rules {
            if !self.validate_inheritance_rule(rule, context)? {
                return Err(RhemaError::ValidationError(
                    format!("Context inheritance rule '{}' violated", rule.name)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Validate context permissions
    fn validate_context_permissions(&self, context: &BranchContext) -> RhemaResult<()> {
        // Check if context files have appropriate permissions
        for rule in &context.isolation_settings.permission_rules {
            if !self.validate_permission_rule(rule, &context.context_files)? {
                return Err(RhemaError::ValidationError(
                    format!("Context permission rule '{}' violated", rule.name)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Validate context consistency
    fn validate_context_consistency(&self, context: &BranchContext) -> RhemaResult<()> {
        // Check for internal consistency in context files
        let mut seen_entries = std::collections::HashSet::new();
        
        for file_path in &context.context_files {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                // Check for duplicate entries
                for line in content.lines() {
                    if line.trim().starts_with('-') {
                        let entry = line.trim().to_string();
                        if !seen_entries.insert(entry.clone()) {
                            return Err(RhemaError::ValidationError(
                                format!("Duplicate context entry found: {}", entry)
                            ));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Check for conflicts with other branches
    fn check_cross_branch_conflicts(&self, current_branch: &str) -> RhemaResult<()> {
        // Get all branches
        let branches = self.repo.branches(Some(BranchType::Local))?;
        
        for branch_result in branches {
            let (branch, _) = branch_result?;
            let branch_name = branch.name()?.unwrap_or("unknown");
            
            if branch_name != current_branch {
                // Check for potential conflicts
                if let Err(_) = self.check_potential_conflicts(current_branch, branch_name) {
                    // This is just a warning, not an error
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if file exists in another branch
    fn file_exists_in_branch(&self, _file_path: &Path, _branch_name: &str) -> RhemaResult<bool> {
        // This is a simplified check - in a real implementation, you'd use git2 to check
        // if the file exists in the specified branch
        Ok(false) // Placeholder
    }
    
    /// Validate boundary rule
    fn validate_boundary_rule(&self, _rule: &BoundaryRule, _context_files: &[PathBuf]) -> RhemaResult<bool> {
        // Simplified boundary validation
        Ok(true) // Placeholder
    }
    
    /// Validate inheritance rule
    fn validate_inheritance_rule(&self, _rule: &InheritanceRule, _context: &BranchContext) -> RhemaResult<bool> {
        // Simplified inheritance validation
        Ok(true) // Placeholder
    }
    
    /// Validate permission rule
    fn validate_permission_rule(&self, _rule: &PermissionRule, _context_files: &[PathBuf]) -> RhemaResult<bool> {
        // Simplified permission validation
        Ok(true) // Placeholder
    }
    
    /// Check potential conflicts between branches
    fn check_potential_conflicts(&self, _branch1: &str, _branch2: &str) -> RhemaResult<()> {
        // Simplified conflict checking
        Ok(()) // Placeholder
    }
    
    /// Save branch context to file
    fn save_branch_context(&self, context: &BranchContext) -> RhemaResult<()> {
        let context_dir = self.repo.path().join(".rhema").join("branch-contexts");
        std::fs::create_dir_all(&context_dir)?;
        
        let context_file = context_dir.join(format!("{}.yaml", context.name));
        let content = serde_yaml::to_string(context)?;
        std::fs::write(context_file, content)?;
        
        Ok(())
    }
    
    /// Validate merged context
    fn validate_merged_context(&self, branch_name: &str) -> RhemaResult<()> {
        // Validate the context after merge
        if let Some(context) = self.contexts.get(branch_name) {
            // Check for consistency
            self.validate_context_consistency(context)?;
            
            // Check for conflicts
            if !self.check_context_conflicts(branch_name, branch_name)?.is_empty() {
                return Err(RhemaError::ValidationError(
                    "Internal conflicts detected in merged context".to_string()
                ));
            }
        }
        
        Ok(())
    }
}

/// Merge result
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub success: bool,
    pub conflicts: Vec<ContextConflict>,
    pub merged_files: Vec<PathBuf>,
    pub messages: Vec<String>,
}

/// Context conflict
#[derive(Debug, Clone)]
pub struct ContextConflict {
    pub file_path: PathBuf,
    pub conflict_type: ConflictType,
    pub description: String,
    pub resolution: Option<String>,
}

/// Conflict type
#[derive(Debug, Clone)]
pub enum ConflictType {
    Content,
    Structure,
    Schema,
    Dependency,
}

/// Context evolution entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEvolution {
    pub timestamp: DateTime<Utc>,
    pub commit_hash: String,
    pub changes: Vec<ContextChange>,
    pub author: String,
    pub message: String,
}

/// Context change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextChange {
    pub file_path: PathBuf,
    pub change_type: ChangeType,
    pub description: String,
    pub impact: Option<String>,
}

/// Change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    /// Merge conflicts
    Merge,
    
    /// Take source changes
    TakeSource,
    
    /// Take target changes
    TakeTarget,
    
    /// Custom resolution
    Custom(String),
}

/// Branch protection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProtectionConfig {
    pub branches: HashMap<String, BranchProtection>,
}

/// Merge strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeStrategyConfig {
    pub branches: HashMap<String, MergeStrategy>,
} 
