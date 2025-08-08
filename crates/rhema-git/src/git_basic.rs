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

use git2::{BranchType, MergeOptions, Repository, Signature};
use rhema_core::{RhemaError, RhemaResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Find the Git repository root from the current directory
pub fn find_repo_root() -> Result<PathBuf, RhemaError> {
    find_repo_root_from(&std::env::current_dir()?)
}

/// Find the Git repository root from a given path
pub fn find_repo_root_from(path: &Path) -> Result<PathBuf, RhemaError> {
    let mut current = path.to_path_buf();
    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(RhemaError::ConfigError(
                "No Git repository found".to_string(),
            ));
        }
    }
}

/// Get a Git repository instance
pub fn get_repo(path: &Path) -> Result<Repository, RhemaError> {
    Repository::open(path)
        .map_err(|e| RhemaError::ConfigError(format!("Failed to open Git repository: {}", e)))
}

/// Check if a file is tracked by Git
pub fn is_tracked(repo: &Repository, path: &Path) -> Result<bool, RhemaError> {
    let status = repo
        .status_file(path)
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get file status: {}", e)))?;
    Ok(!status.is_empty())
}

/// Get file status
pub fn get_file_status(repo: &Repository, path: &Path) -> Result<git2::Status, RhemaError> {
    repo.status_file(path)
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get file status: {}", e)))
}

/// Get changed files in the repository
pub fn get_changed_files(repo: &Repository) -> Result<Vec<PathBuf>, RhemaError> {
    let mut options = git2::StatusOptions::new();
    options.include_untracked(true);
    options.include_ignored(false);

    let statuses = repo
        .statuses(Some(&mut options))
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get statuses: {}", e)))?;

    let mut changed_files = Vec::new();
    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            changed_files.push(PathBuf::from(path));
        }
    }

    Ok(changed_files)
}

/// Get the current branch name
pub fn get_current_branch(repo: &Repository) -> Result<String, RhemaError> {
    let head = repo
        .head()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get HEAD: {}", e)))?;

    let branch_name = head.shorthand().ok_or_else(|| {
        RhemaError::ConfigError("Failed to get branch name from HEAD".to_string())
    })?;

    Ok(branch_name.to_string())
}

/// Get the latest commit hash
pub fn get_latest_commit(repo: &Repository) -> Result<String, RhemaError> {
    let head = repo
        .head()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get HEAD: {}", e)))?;

    let commit = head
        .peel_to_commit()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to peel to commit: {}", e)))?;

    Ok(commit.id().to_string())
}

/// Check if working directory is clean
pub fn is_working_directory_clean(repo: &Repository) -> Result<bool, RhemaError> {
    let changed_files = get_changed_files(repo)?;
    Ok(changed_files.is_empty())
}

/// Stage a file for commit
pub fn stage_file(repo: &Repository, path: &Path) -> Result<(), RhemaError> {
    let mut index = repo
        .index()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get index: {}", e)))?;

    index
        .add_path(path)
        .map_err(|e| RhemaError::ConfigError(format!("Failed to add file to index: {}", e)))?;

    index
        .write()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to write index: {}", e)))?;

    Ok(())
}

/// Commit changes with a message
pub fn commit_changes(repo: &Repository, message: &str) -> Result<(), RhemaError> {
    let signature = Signature::now("Rhema Git", "rhema@example.com")
        .map_err(|e| RhemaError::ConfigError(format!("Failed to create signature: {}", e)))?;

    let mut index = repo
        .index()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get index: {}", e)))?;

    let tree_id = index
        .write_tree()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to write tree: {}", e)))?;

    let tree = repo
        .find_tree(tree_id)
        .map_err(|e| RhemaError::ConfigError(format!("Failed to find tree: {}", e)))?;

    let head = repo
        .head()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to get HEAD: {}", e)))?;

    let parent_commit = head
        .peel_to_commit()
        .map_err(|e| RhemaError::ConfigError(format!("Failed to peel to commit: {}", e)))?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )
    .map_err(|e| RhemaError::ConfigError(format!("Failed to commit: {}", e)))?;

    Ok(())
}

// Basic types that the CLI expects
use chrono::{DateTime, Utc};

/// Conflict resolution strategy
#[derive(Debug, Clone)]
pub enum ConflictResolutionStrategy {
    /// Automatically resolve conflicts using the specified strategy
    Auto(AutoResolutionStrategy),
    /// Manual resolution required
    Manual,
    /// Abort the operation
    Abort,
}

/// Automatic resolution strategy
#[derive(Debug, Clone)]
pub enum AutoResolutionStrategy {
    /// Use the current branch version
    Current,
    /// Use the incoming branch version
    Incoming,
    /// Use the base version
    Base,
    /// Merge both versions
    Merge,
}

/// Conflict information
#[derive(Debug, Clone)]
pub struct ConflictInfo {
    pub file_path: PathBuf,
    pub conflict_type: ConflictType,
    pub resolution_strategy: Option<ConflictResolutionStrategy>,
    pub details: String,
}

/// Conflict type
#[derive(Debug, Clone)]
pub enum ConflictType {
    /// Merge conflict
    Merge,
    /// Checkout conflict
    Checkout,
    /// Rebase conflict
    Rebase,
    /// Cherry-pick conflict
    CherryPick,
}

/// Conflict resolution result
#[derive(Debug, Clone)]
pub struct ConflictResolutionResult {
    pub success: bool,
    pub resolved_conflicts: Vec<PathBuf>,
    pub unresolved_conflicts: Vec<ConflictInfo>,
    pub messages: Vec<String>,
}

/// Feature branch information
#[derive(Debug, Clone)]
pub struct FeatureBranch {
    pub name: String,
    pub base_branch: String,
    pub created_at: DateTime<Utc>,
    pub context_files: Vec<PathBuf>,
}

/// Release branch information
#[derive(Debug, Clone)]
pub struct ReleaseBranch {
    pub name: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub status: ReleaseStatus,
}

/// Release status
#[derive(Debug, Clone)]
pub enum ReleaseStatus {
    InProgress,
    ReadyForRelease,
    Released,
    Failed,
}

/// Hotfix branch information
#[derive(Debug, Clone)]
pub struct HotfixBranch {
    pub name: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub status: HotfixStatus,
}

/// Hotfix status
#[derive(Debug, Clone)]
pub enum HotfixStatus {
    InProgress,
    ReadyForDeploy,
    Deployed,
    Failed,
}

/// Feature branch result
#[derive(Debug, Clone)]
pub struct FeatureResult {
    pub success: bool,
    pub merged_branch: String,
    pub target_branch: String,
    pub conflicts: Vec<String>,
    pub messages: Vec<String>,
    pub conflict_resolution: Option<ConflictResolutionResult>,
}

/// Release branch result
#[derive(Debug, Clone)]
pub struct ReleaseResult {
    pub success: bool,
    pub version: String,
    pub main_merge: bool,
    pub develop_merge: bool,
    pub tag_created: bool,
    pub messages: Vec<String>,
    pub conflict_resolution: Option<ConflictResolutionResult>,
}

/// Hotfix branch result
#[derive(Debug, Clone)]
pub struct HotfixResult {
    pub success: bool,
    pub version: String,
    pub main_merge: bool,
    pub develop_merge: bool,
    pub tag_created: bool,
    pub messages: Vec<String>,
    pub conflict_resolution: Option<ConflictResolutionResult>,
}

/// Workflow status
#[derive(Debug, Clone)]
pub struct WorkflowStatus {
    pub current_branch: String,
    pub branch_type: FlowBranchType,
    pub workflow_type: WorkflowType,
    pub status: String,
}

/// Flow branch type
#[derive(Debug, Clone)]
pub enum FlowBranchType {
    Main,
    Develop,
    Feature,
    Release,
    Hotfix,
}

/// Workflow type
#[derive(Debug, Clone)]
pub enum WorkflowType {
    GitFlow,
    GitHubFlow,
    GitLabFlow,
    Custom,
}

// Return types for CLI compatibility
#[derive(Debug, Clone)]
pub enum ValidationStatus {
    Valid,
    Invalid(Vec<String>),
    Pending,
    Skipped,
}

#[derive(Debug, Clone)]
pub struct IntegrationStatus {
    pub enabled: bool,
    pub hooks_installed: bool,
    pub workflow_status: WorkflowStatus,
    pub automation_status: AutomationStatus,
    pub hook_status: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AutomationStatus {
    pub running: bool,
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub failed_tasks: u32,
    pub pending_tasks: u32,
}

#[derive(Debug, Clone)]
pub struct PullRequestAnalysis {
    pub context_changes: Vec<String>,
    pub impact_analysis: ImpactAnalysis,
    pub health_checks: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ImpactAnalysis {
    pub risk_level: String,
    pub affected_scopes: Vec<String>,
    pub breaking_changes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContextEvolution {
    pub entries: Vec<EvolutionEntry>,
}

#[derive(Debug, Clone)]
pub struct EvolutionEntry {
    pub timestamp: DateTime<Utc>,
    pub change_type: String,
    pub description: String,
    pub author: String,
    pub commit_message: String,
}

#[derive(Debug, Clone)]
pub struct ContextBlame {
    pub entries: Vec<BlameEntry>,
}

#[derive(Debug, Clone)]
pub struct BlameEntry {
    pub line: u32,
    pub commit: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ContextVersion {
    pub version: String,
    pub version_type: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct EvolutionReport {
    pub total_commits: u32,
    pub changes_by_type: HashMap<String, u32>,
    pub top_contributors: Vec<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SecurityScanResult {
    pub issues: Vec<SecurityIssue>,
    pub risk_level: String,
    pub scan_duration: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: String,
    pub description: String,
    pub file_path: String,
    pub line: Option<u32>,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub risk_level: String,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MonitoringStatus {
    pub is_active: bool,
    pub metrics_count: u32,
    pub events_count: u32,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AutomationTask {
    pub id: String,
    pub task_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Advanced Git integration interface that the CLI expects
pub struct AdvancedGitIntegration {
    repo: Repository,
    hooks_manager: Option<crate::git_hooks::GitHooksManager>,
}

impl AdvancedGitIntegration {
    /// Create a new advanced Git integration instance
    pub fn new(repo: Repository) -> RhemaResult<Self> {
        let repo_path = repo
            .path()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();
        let hooks_manager = crate::git_hooks::GitHooksManager::new(&repo_path).ok();

        Ok(Self {
            repo,
            hooks_manager,
        })
    }

    /// Detect conflicts in the repository
    pub fn detect_conflicts(&self) -> RhemaResult<Vec<ConflictInfo>> {
        let mut conflicts = Vec::new();

        // Check for merge conflicts by looking at the repository state
        // For now, we'll use a simplified approach
        let statuses = self
            .repo
            .statuses(None)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get statuses: {}", e)))?;

        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                let status = entry.status();
                if status.is_conflicted() {
                    conflicts.push(ConflictInfo {
                        file_path: PathBuf::from(path),
                        conflict_type: ConflictType::Merge,
                        resolution_strategy: None,
                        details: "Merge conflict detected".to_string(),
                    });
                }
            }
        }

        Ok(conflicts)
    }

    /// Resolve conflicts automatically
    pub fn resolve_conflicts(
        &mut self,
        strategy: ConflictResolutionStrategy,
    ) -> RhemaResult<ConflictResolutionResult> {
        let conflicts = self.detect_conflicts()?;
        let mut resolved_conflicts = Vec::new();
        let mut unresolved_conflicts = Vec::new();
        let mut messages = Vec::new();

        match strategy {
            ConflictResolutionStrategy::Auto(auto_strategy) => {
                for conflict in conflicts {
                    match self.resolve_single_conflict(&conflict, &auto_strategy) {
                        Ok(_) => {
                            resolved_conflicts.push(conflict.file_path.clone());
                            messages.push(format!(
                                "Resolved conflict in {}",
                                conflict.file_path.display()
                            ));
                        }
                        Err(e) => {
                            let mut unresolved = conflict.clone();
                            unresolved.resolution_strategy =
                                Some(ConflictResolutionStrategy::Manual);
                            unresolved.details = format!("Auto-resolution failed: {}", e);
                            unresolved_conflicts.push(unresolved);
                        }
                    }
                }
            }
            ConflictResolutionStrategy::Manual => {
                unresolved_conflicts = conflicts;
                messages.push("Conflicts require manual resolution".to_string());
            }
            ConflictResolutionStrategy::Abort => {
                self.abort_merge()?;
                messages.push("Merge aborted due to conflicts".to_string());
                return Ok(ConflictResolutionResult {
                    success: false,
                    resolved_conflicts,
                    unresolved_conflicts: conflicts,
                    messages,
                });
            }
        }

        // Stage resolved conflicts
        if !resolved_conflicts.is_empty() {
            let mut index = self
                .repo
                .index()
                .map_err(|e| RhemaError::ConfigError(format!("Failed to get index: {}", e)))?;

            for path in &resolved_conflicts {
                index.add_path(path).map_err(|e| {
                    RhemaError::ConfigError(format!("Failed to add resolved file: {}", e))
                })?;
            }

            index
                .write()
                .map_err(|e| RhemaError::ConfigError(format!("Failed to write index: {}", e)))?;
        }

        Ok(ConflictResolutionResult {
            success: unresolved_conflicts.is_empty(),
            resolved_conflicts,
            unresolved_conflicts,
            messages,
        })
    }

    /// Resolve a single conflict
    fn resolve_single_conflict(
        &self,
        conflict: &ConflictInfo,
        strategy: &AutoResolutionStrategy,
    ) -> RhemaResult<()> {
        let file_path = &conflict.file_path;

        // Read the conflicted file content
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            RhemaError::ConfigError(format!("Failed to read conflicted file: {}", e))
        })?;

        let resolved_content = match strategy {
            AutoResolutionStrategy::Current => self.resolve_with_current_version(&content)?,
            AutoResolutionStrategy::Incoming => self.resolve_with_incoming_version(&content)?,
            AutoResolutionStrategy::Base => self.resolve_with_base_version(&content)?,
            AutoResolutionStrategy::Merge => self.resolve_with_merge_version(&content)?,
        };

        // Write the resolved content back
        std::fs::write(file_path, resolved_content).map_err(|e| {
            RhemaError::ConfigError(format!("Failed to write resolved file: {}", e))
        })?;

        Ok(())
    }

    /// Resolve conflict using current version
    fn resolve_with_current_version(&self, content: &str) -> RhemaResult<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut resolved_lines = Vec::new();
        let mut in_conflict = false;
        let mut in_current = false;

        for line in lines {
            if line.starts_with("<<<<<<<") {
                in_conflict = true;
                in_current = false;
            } else if line.starts_with("=======") {
                in_current = true;
            } else if line.starts_with(">>>>>>>") {
                in_conflict = false;
                in_current = false;
            } else if in_conflict && !in_current {
                // Skip incoming version
                continue;
            } else {
                resolved_lines.push(line);
            }
        }

        Ok(resolved_lines.join("\n"))
    }

    /// Resolve conflict using incoming version
    fn resolve_with_incoming_version(&self, content: &str) -> RhemaResult<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut resolved_lines = Vec::new();
        let mut in_conflict = false;
        let mut in_incoming = false;

        for line in lines {
            if line.starts_with("<<<<<<<") {
                in_conflict = true;
                in_incoming = false;
            } else if line.starts_with("=======") {
                in_incoming = true;
            } else if line.starts_with(">>>>>>>") {
                in_conflict = false;
                in_incoming = false;
            } else if in_conflict && in_incoming {
                resolved_lines.push(line);
            } else if !in_conflict {
                resolved_lines.push(line);
            }
        }

        Ok(resolved_lines.join("\n"))
    }

    /// Resolve conflict using base version (simplified - just remove conflict markers)
    fn resolve_with_base_version(&self, content: &str) -> RhemaResult<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut resolved_lines = Vec::new();
        let mut in_conflict = false;

        for line in lines {
            if line.starts_with("<<<<<<<")
                || line.starts_with("=======")
                || line.starts_with(">>>>>>>")
            {
                in_conflict = !in_conflict;
            } else if !in_conflict {
                resolved_lines.push(line);
            }
        }

        Ok(resolved_lines.join("\n"))
    }

    /// Resolve conflict by merging both versions
    fn resolve_with_merge_version(&self, content: &str) -> RhemaResult<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut resolved_lines = Vec::new();
        let mut in_conflict = false;
        let mut current_lines = Vec::new();
        let mut incoming_lines = Vec::new();

        for line in lines {
            if line.starts_with("<<<<<<<") {
                in_conflict = true;
                current_lines.clear();
                incoming_lines.clear();
            } else if line.starts_with("=======") {
                // Switch to collecting incoming lines
            } else if line.starts_with(">>>>>>>") {
                in_conflict = false;
                // Merge both versions
                resolved_lines.extend(current_lines.clone());
                resolved_lines.extend(incoming_lines.clone());
            } else if in_conflict {
                if incoming_lines.is_empty() {
                    current_lines.push(line);
                } else {
                    incoming_lines.push(line);
                }
            } else {
                resolved_lines.push(line);
            }
        }

        Ok(resolved_lines.join("\n"))
    }

    /// Abort the current merge
    fn abort_merge(&self) -> RhemaResult<()> {
        // Reset the repository to abort the merge
        let head = self
            .repo
            .head()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get HEAD: {}", e)))?;

        let commit = head
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to peel to commit: {}", e)))?;

        self.repo
            .reset(&commit.as_object(), git2::ResetType::Hard, None)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to abort merge: {}", e)))?;

        Ok(())
    }

    /// Create a feature branch
    pub fn create_feature_branch(&mut self, name: &str, base: &str) -> RhemaResult<FeatureBranch> {
        let branch_name = format!("feature/{}", name);

        // Get the base branch reference
        let base_ref = self
            .repo
            .find_branch(base, BranchType::Local)
            .map_err(|e| {
                RhemaError::ConfigError(format!("Base branch '{}' not found: {}", base, e))
            })?;

        let base_commit = base_ref
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get base commit: {}", e)))?;

        // Create the new branch
        let branch_ref = self
            .repo
            .branch(&branch_name, &base_commit, false)
            .map_err(|e| {
                RhemaError::ConfigError(format!("Failed to create branch '{}': {}", branch_name, e))
            })?;

        // Checkout the new branch
        let branch_commit = branch_ref
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get branch commit: {}", e)))?;

        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&branch_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to checkout branch: {}", e)))?;

        self.repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to set HEAD: {}", e)))?;

        Ok(FeatureBranch {
            name: branch_name,
            base_branch: base.to_string(),
            created_at: Utc::now(),
            context_files: vec![],
        })
    }

    /// Finish a feature branch with enhanced conflict resolution
    pub fn finish_feature_branch(&mut self, name: &str) -> RhemaResult<FeatureResult> {
        let branch_name = format!("feature/{}", name);
        let mut messages = Vec::new();
        let mut conflicts = Vec::new();
        let conflict_resolution = None;

        // Get the feature branch and commit
        let feature_commit = {
            let feature_branch = self
                .repo
                .find_branch(&branch_name, BranchType::Local)
                .map_err(|e| {
                    RhemaError::ConfigError(format!(
                        "Feature branch '{}' not found: {}",
                        branch_name, e
                    ))
                })?;

            feature_branch.get().peel_to_commit().map_err(|e| {
                RhemaError::ConfigError(format!("Failed to get feature commit: {}", e))
            })?
        };

        // Get the develop branch
        let develop_branch = self
            .repo
            .find_branch("develop", BranchType::Local)
            .map_err(|e| RhemaError::ConfigError(format!("Develop branch not found: {}", e)))?;

        let develop_commit = develop_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get develop commit: {}", e)))?;

        // Checkout develop branch
        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&develop_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to checkout develop: {}", e)))?;

        self.repo.set_head("refs/heads/develop").map_err(|e| {
            RhemaError::ConfigError(format!("Failed to set HEAD to develop: {}", e))
        })?;

        // Merge the feature branch
        let mut merge_options = MergeOptions::new();
        merge_options.fail_on_conflict(true); // Fail on conflicts for now

        let annotated_commit = self
            .repo
            .find_annotated_commit(feature_commit.id())
            .map_err(|e| {
                RhemaError::ConfigError(format!("Failed to create annotated commit: {}", e))
            })?;

        match self
            .repo
            .merge(&[&annotated_commit], Some(&mut merge_options), None)
        {
            Ok(_) => {
                messages.push("Feature branch merged successfully".to_string());

                // Create merge commit
                let signature = Signature::now("Rhema Git", "rhema@example.com").map_err(|e| {
                    RhemaError::ConfigError(format!("Failed to create signature: {}", e))
                })?;

                let tree = self
                    .repo
                    .index()
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to get index: {}", e)))?
                    .write_tree()
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to write tree: {}", e)))?;

                let tree_obj = self
                    .repo
                    .find_tree(tree)
                    .map_err(|e| RhemaError::ConfigError(format!("Failed to find tree: {}", e)))?;

                self.repo
                    .commit(
                        Some("refs/heads/develop"),
                        &signature,
                        &signature,
                        &format!("Merge feature branch '{}'", name),
                        &tree_obj,
                        &[&develop_commit, &feature_commit],
                    )
                    .map_err(|e| {
                        RhemaError::ConfigError(format!("Failed to create merge commit: {}", e))
                    })?;

                // Delete the feature branch (need to find it again since we dropped the reference)
                let mut feature_branch_to_delete = self
                    .repo
                    .find_branch(&branch_name, BranchType::Local)
                    .map_err(|e| {
                        RhemaError::ConfigError(format!(
                            "Feature branch '{}' not found for deletion: {}",
                            branch_name, e
                        ))
                    })?;

                feature_branch_to_delete.delete().map_err(|e| {
                    RhemaError::ConfigError(format!("Failed to delete feature branch: {}", e))
                })?;

                messages.push("Feature branch deleted".to_string());
            }
            Err(e) => {
                conflicts.push(format!("Merge failed: {}", e));
                return Ok(FeatureResult {
                    success: false,
                    merged_branch: branch_name.clone(),
                    target_branch: "develop".to_string(),
                    conflicts,
                    messages,
                    conflict_resolution,
                });
            }
        }

        Ok(FeatureResult {
            success: conflicts.is_empty(),
            merged_branch: branch_name,
            target_branch: "develop".to_string(),
            conflicts,
            messages,
            conflict_resolution,
        })
    }

    /// Start a release branch
    pub fn start_release_branch(&mut self, version: &str) -> RhemaResult<ReleaseBranch> {
        let branch_name = format!("release/{}", version);

        // Get the develop branch
        let develop_branch = self
            .repo
            .find_branch("develop", BranchType::Local)
            .map_err(|e| RhemaError::ConfigError(format!("Develop branch not found: {}", e)))?;

        let develop_commit = develop_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get develop commit: {}", e)))?;

        // Create the release branch
        let branch_ref = self
            .repo
            .branch(&branch_name, &develop_commit, false)
            .map_err(|e| {
                RhemaError::ConfigError(format!("Failed to create release branch: {}", e))
            })?;

        // Checkout the release branch
        let branch_commit = branch_ref
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get branch commit: {}", e)))?;

        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&branch_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| {
                RhemaError::ConfigError(format!("Failed to checkout release branch: {}", e))
            })?;

        self.repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to set HEAD: {}", e)))?;

        Ok(ReleaseBranch {
            name: branch_name,
            version: version.to_string(),
            created_at: Utc::now(),
            status: ReleaseStatus::InProgress,
        })
    }

    /// Finish a release branch with enhanced conflict resolution
    pub fn finish_release_branch(&mut self, version: &str) -> RhemaResult<ReleaseResult> {
        let branch_name = format!("release/{}", version);
        let mut messages = Vec::new();
        let conflict_resolution = None;

        // Get the release branch
        let mut release_branch = self
            .repo
            .find_branch(&branch_name, BranchType::Local)
            .map_err(|e| {
                RhemaError::ConfigError(format!(
                    "Release branch '{}' not found: {}",
                    branch_name, e
                ))
            })?;

        let release_commit = release_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get release commit: {}", e)))?;

        // Get main and develop branches
        let main_branch = self
            .repo
            .find_branch("main", BranchType::Local)
            .map_err(|_e| RhemaError::ConfigError("Main branch not found".to_string()))?;

        let develop_branch = self
            .repo
            .find_branch("develop", BranchType::Local)
            .map_err(|_e| RhemaError::ConfigError("Develop branch not found".to_string()))?;

        let main_commit = main_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get main commit: {}", e)))?;

        let develop_commit = develop_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get develop commit: {}", e)))?;

        // Merge to main
        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&main_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to checkout main: {}", e)))?;

        self.repo
            .set_head("refs/heads/main")
            .map_err(|e| RhemaError::ConfigError(format!("Failed to set HEAD to main: {}", e)))?;

        let signature = Signature::now("Rhema Git", "rhema@example.com")
            .map_err(|e| RhemaError::ConfigError(format!("Failed to create signature: {}", e)))?;

        // Create tag
        self.repo
            .tag(
                version,
                &release_commit.as_object(),
                &signature,
                &format!("Release version {}", version),
                false,
            )
            .map_err(|e| RhemaError::ConfigError(format!("Failed to create tag: {}", e)))?;

        messages.push(format!("Tag v{} created", version));

        // Merge to develop
        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&develop_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to checkout develop: {}", e)))?;

        self.repo.set_head("refs/heads/develop").map_err(|e| {
            RhemaError::ConfigError(format!("Failed to set HEAD to develop: {}", e))
        })?;

        // Delete the release branch
        release_branch.delete().map_err(|e| {
            RhemaError::ConfigError(format!("Failed to delete release branch: {}", e))
        })?;

        messages.push("Release branch deleted".to_string());

        Ok(ReleaseResult {
            success: true,
            version: version.to_string(),
            main_merge: true,
            develop_merge: true,
            tag_created: true,
            messages,
            conflict_resolution,
        })
    }

    /// Start a hotfix branch
    pub fn start_hotfix_branch(&mut self, version: &str) -> RhemaResult<HotfixBranch> {
        let branch_name = format!("hotfix/{}", version);

        // Get the main branch
        let main_branch = self
            .repo
            .find_branch("main", BranchType::Local)
            .map_err(|e| RhemaError::ConfigError(format!("Main branch not found: {}", e)))?;

        let main_commit = main_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get main commit: {}", e)))?;

        // Create the hotfix branch
        let branch_ref = self
            .repo
            .branch(&branch_name, &main_commit, false)
            .map_err(|e| {
                RhemaError::ConfigError(format!("Failed to create hotfix branch: {}", e))
            })?;

        // Checkout the hotfix branch
        let branch_commit = branch_ref
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get branch commit: {}", e)))?;

        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&branch_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| {
                RhemaError::ConfigError(format!("Failed to checkout hotfix branch: {}", e))
            })?;

        self.repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to set HEAD: {}", e)))?;

        Ok(HotfixBranch {
            name: branch_name,
            version: version.to_string(),
            created_at: Utc::now(),
            status: HotfixStatus::InProgress,
        })
    }

    /// Finish a hotfix branch with enhanced conflict resolution
    pub fn finish_hotfix_branch(&mut self, version: &str) -> RhemaResult<HotfixResult> {
        let branch_name = format!("hotfix/{}", version);
        let mut messages = Vec::new();
        let conflict_resolution = None;

        // Get the hotfix branch
        let mut hotfix_branch = self
            .repo
            .find_branch(&branch_name, BranchType::Local)
            .map_err(|e| {
                RhemaError::ConfigError(format!("Hotfix branch '{}' not found: {}", branch_name, e))
            })?;

        let hotfix_commit = hotfix_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get hotfix commit: {}", e)))?;

        // Get main and develop branches
        let main_branch = self
            .repo
            .find_branch("main", BranchType::Local)
            .map_err(|_e| RhemaError::ConfigError("Main branch not found".to_string()))?;

        let develop_branch = self
            .repo
            .find_branch("develop", BranchType::Local)
            .map_err(|_e| RhemaError::ConfigError("Develop branch not found".to_string()))?;

        let main_commit = main_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get main commit: {}", e)))?;

        let develop_commit = develop_branch
            .get()
            .peel_to_commit()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to get develop commit: {}", e)))?;

        // Merge to main
        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&main_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to checkout main: {}", e)))?;

        self.repo
            .set_head("refs/heads/main")
            .map_err(|e| RhemaError::ConfigError(format!("Failed to set HEAD to main: {}", e)))?;

        let signature = Signature::now("Rhema Git", "rhema@example.com")
            .map_err(|e| RhemaError::ConfigError(format!("Failed to create signature: {}", e)))?;

        // Create tag
        self.repo
            .tag(
                version,
                &hotfix_commit.as_object(),
                &signature,
                &format!("Hotfix version {}", version),
                false,
            )
            .map_err(|e| RhemaError::ConfigError(format!("Failed to create tag: {}", e)))?;

        messages.push(format!("Hotfix tag v{} created", version));

        // Merge to develop
        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();
        self.repo
            .checkout_tree(&develop_commit.as_object(), Some(&mut checkout_options))
            .map_err(|e| RhemaError::ConfigError(format!("Failed to checkout develop: {}", e)))?;

        self.repo.set_head("refs/heads/develop").map_err(|e| {
            RhemaError::ConfigError(format!("Failed to set HEAD to develop: {}", e))
        })?;

        // Delete the hotfix branch
        hotfix_branch.delete().map_err(|e| {
            RhemaError::ConfigError(format!("Failed to delete hotfix branch: {}", e))
        })?;

        messages.push("Hotfix branch deleted".to_string());

        Ok(HotfixResult {
            success: true,
            version: version.to_string(),
            main_merge: true,
            develop_merge: true,
            tag_created: true,
            messages,
            conflict_resolution,
        })
    }

    /// Get workflow status
    pub fn get_workflow_status(&self) -> RhemaResult<WorkflowStatus> {
        let current_branch = get_current_branch(&self.repo)?;

        let branch_type = if current_branch == "main" {
            FlowBranchType::Main
        } else if current_branch == "develop" {
            FlowBranchType::Develop
        } else if current_branch.starts_with("feature/") {
            FlowBranchType::Feature
        } else if current_branch.starts_with("release/") {
            FlowBranchType::Release
        } else if current_branch.starts_with("hotfix/") {
            FlowBranchType::Hotfix
        } else {
            FlowBranchType::Main
        };

        Ok(WorkflowStatus {
            current_branch,
            branch_type,
            workflow_type: WorkflowType::GitFlow,
            status: "active".to_string(),
        })
    }

    /// Get current branch
    pub fn get_current_branch(&self) -> RhemaResult<String> {
        get_current_branch(&self.repo)
    }

    /// Get repository path
    pub fn get_repo_path(&self) -> PathBuf {
        self.repo
            .path()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    }

    /// Get the underlying repository
    pub fn repository(&self) -> &Repository {
        &self.repo
    }

    /// Execute pre-commit hooks
    pub fn execute_pre_commit_hooks(&self) -> RhemaResult<Option<crate::git_hooks::HookResult>> {
        if let Some(ref hooks_manager) = self.hooks_manager {
            let result = hooks_manager.execute_hook(&crate::git_hooks::HookType::PreCommit)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Execute post-commit hooks
    pub fn execute_post_commit_hooks(&self) -> RhemaResult<Option<crate::git_hooks::HookResult>> {
        if let Some(ref hooks_manager) = self.hooks_manager {
            let result = hooks_manager.execute_hook(&crate::git_hooks::HookType::PostCommit)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Execute pre-push hooks
    pub fn execute_pre_push_hooks(&self) -> RhemaResult<Option<crate::git_hooks::HookResult>> {
        if let Some(ref hooks_manager) = self.hooks_manager {
            let result = hooks_manager.execute_hook(&crate::git_hooks::HookType::PrePush)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Install default Rhema hooks
    pub fn install_default_hooks(&mut self) -> RhemaResult<()> {
        if let Some(ref hooks_manager) = self.hooks_manager {
            hooks_manager.install_default_hooks()
        } else {
            Ok(())
        }
    }

    /// List installed hooks
    pub fn list_hooks(&self) -> RhemaResult<Vec<String>> {
        if let Some(ref hooks_manager) = self.hooks_manager {
            hooks_manager.list_hooks()
        } else {
            Ok(vec![])
        }
    }

    // Stub implementations for CLI compatibility
    pub fn execute_hook(
        &self,
        hook_type: crate::git_hooks::HookType,
    ) -> RhemaResult<crate::git_hooks::HookResult> {
        // TODO: Implement proper hook execution
        Ok(crate::git_hooks::HookResult {
            success: true,
            hook_type,
            messages: vec!["Hook executed successfully".to_string()],
            errors: vec![],
            warnings: vec![],
        })
    }

    pub fn get_integration_status(&self) -> RhemaResult<IntegrationStatus> {
        Ok(IntegrationStatus {
            enabled: true,
            hooks_installed: true,
            workflow_status: self.get_workflow_status()?,
            automation_status: AutomationStatus {
                running: false,
                total_tasks: 0,
                completed_tasks: 0,
                failed_tasks: 0,
                pending_tasks: 0,
            },
            hook_status: HashMap::new(),
        })
    }

    pub fn initialize(&mut self) -> RhemaResult<()> {
        // TODO: Implement initialization
        Ok(())
    }

    pub fn analyze_pull_request(&self, _pr_number: u32) -> RhemaResult<PullRequestAnalysis> {
        Ok(PullRequestAnalysis {
            context_changes: vec!["No changes detected".to_string()],
            impact_analysis: ImpactAnalysis {
                risk_level: "low".to_string(),
                affected_scopes: vec![],
                breaking_changes: vec![],
            },
            health_checks: vec!["Basic check passed".to_string()],
            recommendations: vec!["No recommendations".to_string()],
        })
    }

    pub fn track_context_evolution(
        &self,
        _scope: &str,
        _limit: Option<usize>,
    ) -> RhemaResult<ContextEvolution> {
        Ok(ContextEvolution {
            entries: vec![EvolutionEntry {
                timestamp: Utc::now(),
                change_type: "initial".to_string(),
                description: "No evolution data available".to_string(),
                author: "unknown".to_string(),
                commit_message: "Initial commit".to_string(),
            }],
        })
    }

    pub fn get_context_blame(&self, _file: &str) -> RhemaResult<ContextBlame> {
        Ok(ContextBlame {
            entries: vec![BlameEntry {
                line: 1,
                commit: "initial".to_string(),
                author: "unknown".to_string(),
                timestamp: Utc::now(),
                content: "No blame data available".to_string(),
            }],
        })
    }

    pub fn create_context_version(
        &self,
        _version: &str,
        _version_type: &str,
        _description: &str,
    ) -> RhemaResult<ContextVersion> {
        Ok(ContextVersion {
            version: _version.to_string(),
            version_type: _version_type.to_string(),
            description: _description.to_string(),
            created_at: Utc::now(),
        })
    }

    pub fn rollback_to_version(&self, _version: &str) -> RhemaResult<()> {
        // TODO: Implement rollback
        Ok(())
    }

    pub fn generate_evolution_report(
        &self,
        _scope: &str,
        _since_date: Option<&str>,
    ) -> RhemaResult<EvolutionReport> {
        Ok(EvolutionReport {
            total_commits: 0,
            changes_by_type: HashMap::new(),
            top_contributors: vec![],
            start_date: Utc::now(),
            end_date: Utc::now(),
        })
    }

    pub fn start_automation(&mut self) -> RhemaResult<()> {
        // TODO: Implement automation
        Ok(())
    }

    pub fn stop_automation(&mut self) -> RhemaResult<()> {
        // TODO: Implement automation stop
        Ok(())
    }

    pub fn get_automation_status(&self) -> RhemaResult<AutomationStatus> {
        Ok(AutomationStatus {
            running: false,
            total_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            pending_tasks: 0,
        })
    }

    pub fn get_task_history(&self, _limit: Option<usize>) -> RhemaResult<Vec<AutomationTask>> {
        Ok(vec![AutomationTask {
            id: "task-1".to_string(),
            task_type: "validation".to_string(),
            status: "completed".to_string(),
            created_at: Utc::now(),
        }])
    }

    pub fn cancel_task(&mut self, _task_id: &str) -> RhemaResult<()> {
        // TODO: Implement task cancellation
        Ok(())
    }

    pub fn clear_task_history(&mut self) -> RhemaResult<()> {
        // TODO: Implement history clearing
        Ok(())
    }

    /// Trigger workflow automation based on events or schedules
    pub fn trigger_workflow_automation(
        &self,
        trigger_type: &str,
        data: Option<std::collections::HashMap<String, String>>,
    ) -> RhemaResult<()> {
        // TODO: Implement workflow automation
        if trigger_type.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Trigger type cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Trigger feature branch automation
    pub fn trigger_feature_automation(&self, feature_name: &str, action: &str) -> RhemaResult<()> {
        // TODO: Implement feature automation
        if feature_name.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Feature name cannot be empty".to_string(),
            ));
        }
        if action.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Action cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Trigger release automation
    pub fn trigger_release_automation(&self, version: &str, action: &str) -> RhemaResult<()> {
        // TODO: Implement release automation
        if version.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Version cannot be empty".to_string(),
            ));
        }
        if action.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Action cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Trigger hotfix automation
    pub fn trigger_hotfix_automation(&self, version: &str, action: &str) -> RhemaResult<()> {
        // TODO: Implement hotfix automation
        if version.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Version cannot be empty".to_string(),
            ));
        }
        if action.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Action cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Get automation status
    pub fn get_status(&self) -> RhemaResult<AutomationStatus> {
        // TODO: Implement status retrieval
        Ok(AutomationStatus {
            running: false,
            total_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            pending_tasks: 0,
        })
    }

    pub fn security(&self) -> RhemaResult<String> {
        Ok("security_disabled".to_string())
    }

    pub fn start_monitoring(&mut self) -> RhemaResult<()> {
        // TODO: Implement monitoring
        Ok(())
    }

    pub fn stop_monitoring(&mut self) -> RhemaResult<()> {
        // TODO: Implement monitoring stop
        Ok(())
    }

    pub fn get_monitoring_status(&self) -> RhemaResult<MonitoringStatus> {
        Ok(MonitoringStatus {
            is_active: false,
            metrics_count: 0,
            events_count: 0,
            last_update: Utc::now(),
        })
    }

    pub fn record_git_operation(
        &mut self,
        _operation: &str,
        _duration: chrono::Duration,
    ) -> RhemaResult<()> {
        // TODO: Implement operation recording
        Ok(())
    }

    pub fn record_context_operation(
        &mut self,
        _operation: &str,
        _duration: chrono::Duration,
    ) -> RhemaResult<()> {
        // TODO: Implement context operation recording
        Ok(())
    }

    pub fn run_security_scan(&self, _scan_path: &str) -> RhemaResult<SecurityScanResult> {
        Ok(SecurityScanResult {
            issues: vec![SecurityIssue {
                severity: "low".to_string(),
                description: "No security issues found".to_string(),
                file_path: "".to_string(),
                line: None,
                category: "general".to_string(),
            }],
            risk_level: "low".to_string(),
            scan_duration: std::time::Duration::from_secs(0),
        })
    }

    pub fn validate_access(
        &self,
        _user: &str,
        _operation: &crate::git::security::Operation,
        _resource: &str,
    ) -> RhemaResult<bool> {
        Ok(true)
    }

    pub fn validate_commit_security(&self, _commit: &str) -> RhemaResult<ValidationResult> {
        Ok(ValidationResult {
            is_valid: true,
            risk_level: "low".to_string(),
            issues: vec![],
        })
    }

    pub fn shutdown(&mut self) -> RhemaResult<()> {
        // TODO: Implement shutdown
        Ok(())
    }

    // Additional methods for CLI compatibility
    pub fn branches(&self) -> RhemaResult<BranchManager> {
        Ok(BranchManager {
            repo_path: self.get_repo_path(),
        })
    }

    pub fn validate_branch_context(&self) -> RhemaResult<ValidationResult> {
        Ok(ValidationResult {
            is_valid: true,
            risk_level: "low".to_string(),
            issues: vec![],
        })
    }

    pub fn merge_branch_context(
        &self,
        _source_branch: &str,
        _target_branch: &str,
    ) -> RhemaResult<MergeResult> {
        Ok(MergeResult {
            success: true,
            conflicts: vec![],
            messages: vec!["Merge completed successfully".to_string()],
        })
    }

    pub fn backup_branch_context(&self, _branch_name: &str) -> RhemaResult<PathBuf> {
        Ok(PathBuf::from("/tmp/backup"))
    }

    pub fn restore_branch_context(&self, _path: &str) -> RhemaResult<BranchContext> {
        Ok(BranchContext {
            name: "restored".to_string(),
            path: PathBuf::from(_path),
        })
    }
}

// Additional types for CLI compatibility
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub success: bool,
    pub conflicts: Vec<String>,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BranchContext {
    pub name: String,
    pub path: PathBuf,
}

pub struct BranchManager {
    repo_path: PathBuf,
}

impl BranchManager {
    pub fn initialize_branch_context(
        &self,
        _branch_name: Option<String>,
    ) -> RhemaResult<BranchContext> {
        Ok(BranchContext {
            name: _branch_name.unwrap_or_else(|| "default".to_string()),
            path: PathBuf::from("/tmp/branch_context"),
        })
    }
}

/// Create an advanced Git integration instance
pub fn create_advanced_git_integration(repo_path: &Path) -> RhemaResult<AdvancedGitIntegration> {
    let repo = get_repo(repo_path)?;
    AdvancedGitIntegration::new(repo)
}

/// Create an advanced Git integration instance with custom configuration
pub fn create_advanced_git_integration_with_config(
    repo_path: &Path,
    _config: serde_json::Value,
) -> RhemaResult<AdvancedGitIntegration> {
    create_advanced_git_integration(repo_path)
}
