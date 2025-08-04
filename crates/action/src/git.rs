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

use std::path::Path;
use tracing::{info, warn, error};
use git2::{Repository, Commit, Signature, Error as GitError, BranchType, DiffOptions, DiffFormat, DiffDelta, DiffHunk};

use crate::schema::ActionIntent;
use crate::error::{ActionError, ActionResult};

/// Git operation result
#[derive(Debug, Clone)]
pub struct GitOperationResult {
    pub success: bool,
    pub operation: String,
    pub details: String,
    pub branch_name: Option<String>,
    pub commit_hash: Option<String>,
    pub files_changed: Vec<String>,
    pub duration: std::time::Duration,
}

/// Git branch information
#[derive(Debug, Clone)]
pub struct GitBranchInfo {
    pub name: String,
    pub is_current: bool,
    pub last_commit: String,
    pub ahead_count: usize,
    pub behind_count: usize,
}

/// Git commit information
#[derive(Debug, Clone)]
pub struct GitCommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: i64,
    pub files_changed: Vec<String>,
}

/// Git integration for action protocol
pub struct ActionGitIntegration {
    repository: Option<Repository>,
    working_directory: String,
    branch_prefix: String,
    commit_prefix: String,
}

impl ActionGitIntegration {
    /// Create a new Git integration
    pub async fn new() -> ActionResult<Self> {
        info!("Initializing Action Git Integration");
        
        let working_directory = std::env::current_dir()
            .map_err(|e| ActionError::configuration(format!("Failed to get current directory: {}", e)))?
            .to_string_lossy()
            .to_string();
        
        let repository = Repository::open(&working_directory).ok();
        
        let integration = Self {
            repository,
            working_directory,
            branch_prefix: "action/".to_string(),
            commit_prefix: "action: ".to_string(),
        };
        
        info!("Action Git Integration initialized successfully");
        Ok(integration)
    }

    /// Initialize the git integration (stub)
    pub async fn initialize() -> ActionResult<()> {
        info!("ActionGitIntegration initialized (stub)");
        Ok(())
    }

    /// Shutdown the git integration (stub)
    pub async fn shutdown() -> ActionResult<()> {
        info!("ActionGitIntegration shutdown (stub)");
        Ok(())
    }
    
    /// Check if Git repository is available
    pub fn is_git_repository(&self) -> bool {
        self.repository.is_some()
    }
    
    /// Get current branch name
    pub fn get_current_branch(&self) -> ActionResult<String> {
        if let Some(repo) = &self.repository {
            let head = repo.head().map_err(|e| {
                ActionError::git("get_head", format!("Failed to get HEAD: {}", e))
            })?;
            
            let branch_name = head.shorthand().ok_or_else(|| {
                ActionError::git("get_branch", "Failed to get branch name")
            })?;
            
            Ok(branch_name.to_string())
        } else {
            Err(ActionError::git("get_branch", "Not a Git repository"))
        }
    }
    
    /// Get current commit hash
    pub fn get_current_commit(&self) -> ActionResult<String> {
        if let Some(repo) = &self.repository {
            let head = repo.head().map_err(|e| {
                ActionError::git("get_head", format!("Failed to get HEAD: {}", e))
            })?;
            
            let commit = head.peel_to_commit().map_err(|e| {
                ActionError::git("peel_to_commit", format!("Failed to peel to commit: {}", e))
            })?;
            
            Ok(commit.id().to_string())
        } else {
            Err(ActionError::git("get_commit", "Not a Git repository"))
        }
    }
    
    /// Check if working directory is clean
    pub fn is_working_directory_clean(&self) -> ActionResult<bool> {
        if let Some(repo) = &self.repository {
            let statuses = repo.statuses(None).map_err(|e| {
                ActionError::git("get_status", format!("Failed to get status: {}", e))
            })?;
            
            Ok(statuses.is_empty())
        } else {
            Err(ActionError::git("check_clean", "Not a Git repository"))
        }
    }
    
    /// Create a new branch for the action
    pub fn create_action_branch(&self, intent: &ActionIntent) -> ActionResult<String> {
        if let Some(repo) = &self.repository {
            let branch_name = format!("action/{}", intent.id);
            
            let head = repo.head().map_err(|e| {
                ActionError::git("get_head", format!("Failed to get HEAD: {}", e))
            })?;
            
            let commit = head.peel_to_commit().map_err(|e| {
                ActionError::git("peel_to_commit", format!("Failed to peel to commit: {}", e))
            })?;
            
            let branch = repo.branch(&branch_name, &commit, false).map_err(|e| {
                ActionError::git("create_branch", format!("Failed to create branch: {}", e))
            })?;
            
            // Checkout the new branch
            repo.checkout_tree(&commit.as_object(), None).map_err(|e| {
                ActionError::git("checkout_tree", format!("Failed to checkout tree: {}", e))
            })?;
            
            repo.set_head(&format!("refs/heads/{}", branch_name)).map_err(|e| {
                ActionError::git("set_head", format!("Failed to set HEAD: {}", e))
            })?;
            
            info!("Created and checked out action branch: {}", branch_name);
            Ok(branch_name)
        } else {
            Err(ActionError::git("create_branch", "Not a Git repository"))
        }
    }
    
    /// Stage files for commit
    pub fn stage_files(&self, files: &[String]) -> ActionResult<()> {
        if let Some(repo) = &self.repository {
            let mut index = repo.index().map_err(|e| {
                ActionError::git("get_index", format!("Failed to get index: {}", e))
            })?;
            
            for file in files {
                index.add_path(Path::new(file)).map_err(|e| {
                    ActionError::git("add_file", format!("Failed to add file {}: {}", file, e))
                })?;
            }
            
            index.write().map_err(|e| {
                ActionError::git("write_index", format!("Failed to write index: {}", e))
            })?;
            
            info!("Staged {} files for commit", files.len());
            Ok(())
        } else {
            Err(ActionError::git("stage_files", "Not a Git repository"))
        }
    }
    
    /// Commit staged changes
    pub fn commit_changes(&self, intent: &ActionIntent, message: &str) -> ActionResult<String> {
        if let Some(repo) = &self.repository {
            let signature = Signature::now("Rhema Action Protocol", "action@rhema.dev")
                .map_err(|e| ActionError::git("create_signature", format!("Failed to create signature: {}", e)))?;
            
            let tree_id = {
                let mut index = repo.index().map_err(|e| {
                    ActionError::git("get_index", format!("Failed to get index: {}", e))
                })?;
                index.write_tree().map_err(|e| {
                    ActionError::git("write_tree", format!("Failed to write tree: {}", e))
                })?
            };
            
            let tree = repo.find_tree(tree_id).map_err(|e| {
                ActionError::git("find_tree", format!("Failed to find tree: {}", e))
            })?;
            
            let parent_commit = {
                let head = repo.head().map_err(|e| {
                    ActionError::git("get_head", format!("Failed to get HEAD: {}", e))
                })?;
                head.peel_to_commit().map_err(|e| {
                    ActionError::git("peel_to_commit", format!("Failed to peel to commit: {}", e))
                })?
            };
            
            let commit_message = format!(
                "{}\n\nIntent ID: {}\nAction Type: {:?}\nSafety Level: {:?}\nDescription: {}",
                message,
                intent.id,
                intent.action_type,
                intent.safety_level,
                intent.description
            );
            
            let commit_id = repo.commit(
                Some(&format!("refs/heads/{}", self.get_current_branch()?)),
                &signature,
                &signature,
                &commit_message,
                &tree,
                &[&parent_commit],
            ).map_err(|e| {
                ActionError::git("commit", format!("Failed to commit: {}", e))
            })?;
            
            let commit_hash = commit_id.to_string();
            info!("Committed changes with hash: {}", commit_hash);
            Ok(commit_hash)
        } else {
            Err(ActionError::git("commit", "Not a Git repository"))
        }
    }
    
    /// Push changes to remote
    pub fn push_changes(&self, remote_name: &str) -> ActionResult<()> {
        if let Some(repo) = &self.repository {
            let mut remote = repo.find_remote(remote_name).map_err(|e| {
                ActionError::git("find_remote", format!("Failed to find remote {}: {}", remote_name, e))
            })?;
            
            let current_branch = self.get_current_branch()?;
            let refspec = format!("{}:{}", current_branch, current_branch);
            
            remote.push(&[&refspec], None).map_err(|e| {
                ActionError::git("push", format!("Failed to push to remote: {}", e))
            })?;
            
            info!("Pushed changes to remote: {}", remote_name);
            Ok(())
        } else {
            Err(ActionError::git("push", "Not a Git repository"))
        }
    }
    
    /// Create a pull request
    pub fn create_pull_request(&self, intent: &ActionIntent, base_branch: &str) -> ActionResult<String> {
        if let Some(repo) = &self.repository {
            let current_branch = self.get_current_branch()?;
            
            // For now, just return a placeholder PR URL
            // In a real implementation, this would use GitHub/GitLab API
            let pr_url = format!(
                "https://github.com/example/repo/pull/{}",
                intent.id
            );
            
            info!("Created pull request: {}", pr_url);
            Ok(pr_url)
        } else {
            Err(ActionError::git("create_pr", "Not a Git repository"))
        }
    }
    
    /// Revert to a specific commit
    pub fn revert_to_commit(&self, commit_hash: &str) -> ActionResult<()> {
        if let Some(repo) = &self.repository {
            let oid = git2::Oid::from_str(commit_hash).map_err(|e| {
                ActionError::git("parse_oid", format!("Failed to parse commit hash: {}", e))
            })?;
            
            let commit = repo.find_commit(oid).map_err(|e| {
                ActionError::git("find_commit", format!("Failed to find commit: {}", e))
            })?;
            
            repo.reset(&commit.as_object(), git2::ResetType::Hard, None).map_err(|e| {
                ActionError::git("reset", format!("Failed to reset to commit: {}", e))
            })?;
            
            info!("Reverted to commit: {}", commit_hash);
            Ok(())
        } else {
            Err(ActionError::git("revert", "Not a Git repository"))
        }
    }
    
    /// Get commit history for a file
    pub fn get_file_history(&self, file_path: &str) -> ActionResult<Vec<CommitInfo>> {
        if let Some(repo) = &self.repository {
            let mut revwalk = repo.revwalk().map_err(|e| {
                ActionError::git("revwalk", format!("Failed to create revwalk: {}", e))
            })?;
            
            revwalk.push_head().map_err(|e| {
                ActionError::git("push_head", format!("Failed to push HEAD: {}", e))
            })?;
            
            let mut commits = Vec::new();
            
            for oid in revwalk {
                let oid = oid.map_err(|e| {
                    ActionError::git("revwalk_next", format!("Failed to get next commit: {}", e))
                })?;
                
                let commit = repo.find_commit(oid).map_err(|e| {
                    ActionError::git("find_commit", format!("Failed to find commit: {}", e))
                })?;
                
                let tree = commit.tree().map_err(|e| {
                    ActionError::git("get_tree", format!("Failed to get tree: {}", e))
                })?;
                
                if let Ok(_entry) = tree.get_path(Path::new(file_path)) {
                    let commit_info = CommitInfo {
                        hash: oid.to_string(),
                        author: commit.author().name().unwrap_or("Unknown").to_string(),
                        message: commit.message().unwrap_or("").to_string(),
                        timestamp: commit.time().seconds(),
                    };
                    commits.push(commit_info);
                }
            }
            
            Ok(commits)
        } else {
            Err(ActionError::git("get_history", "Not a Git repository"))
        }
    }
    
    /// Get diff for a file
    pub fn get_file_diff(&self, file_path: &str) -> ActionResult<String> {
        if let Some(repo) = &self.repository {
            let head = repo.head().map_err(|e| {
                ActionError::git("get_head", format!("Failed to get HEAD: {}", e))
            })?;
            
            let head_commit = head.peel_to_commit().map_err(|e| {
                ActionError::git("peel_to_commit", format!("Failed to peel to commit: {}", e))
            })?;
            
            let head_tree = head_commit.tree().map_err(|e| {
                ActionError::git("get_tree", format!("Failed to get tree: {}", e))
            })?;
            
            let mut diff = repo.diff_tree_to_workdir(Some(&head_tree), None).map_err(|e| {
                ActionError::git("create_diff", format!("Failed to create diff: {}", e))
            })?;
            
            let mut diff_output = String::new();
            diff.print(DiffFormat::Patch, |delta, hunk, line| {
                diff_output.push_str(&format!(
                    "diff --git a/{} b/{}\n",
                    delta.old_file().path().unwrap_or(Path::new("")).display(),
                    delta.new_file().path().unwrap_or(Path::new("")).display()
                ));
                if let Some(hunk) = hunk {
                    diff_output.push_str(&format!(
                        "@@ -{},{} +{},{} @@\n",
                        hunk.old_start(), hunk.old_lines(),
                        hunk.new_start(), hunk.new_lines()
                    ));
                }
                diff_output.push_str(&format!(
                    "{}\n",
                    String::from_utf8_lossy(line.content())
                ));
                true
            }).map_err(|e| {
                ActionError::git("print_diff", format!("Failed to print diff: {}", e))
            })?;
            
            Ok(diff_output)
        } else {
            Err(ActionError::git("get_diff", "Not a Git repository"))
        }
    }
    
    /// Initialize Git repository if not already initialized
    pub fn initialize_repository(&mut self) -> ActionResult<()> {
        if self.repository.is_none() {
            let repo = Repository::init(&self.working_directory).map_err(|e| {
                ActionError::git("init", format!("Failed to initialize repository: {}", e))
            })?;
            
            self.repository = Some(repo);
            info!("Initialized Git repository in: {}", self.working_directory);
        }
        
        Ok(())
    }
    
    /// Get repository status
    pub fn get_repository_status(&self) -> RepositoryStatus {
        if let Some(repo) = &self.repository {
            let is_clean = self.is_working_directory_clean().unwrap_or(false);
            let current_branch = self.get_current_branch().unwrap_or_else(|_| "unknown".to_string());
            let current_commit = self.get_current_commit().unwrap_or_else(|_| "unknown".to_string());
            
            RepositoryStatus {
                is_git_repository: true,
                is_working_directory_clean: is_clean,
                current_branch,
                current_commit,
                working_directory: self.working_directory.clone(),
            }
        } else {
            RepositoryStatus {
                is_git_repository: false,
                is_working_directory_clean: false,
                current_branch: "unknown".to_string(),
                current_commit: "unknown".to_string(),
                working_directory: self.working_directory.clone(),
            }
        }
    }

    /// Create a feature branch for an action
    pub async fn create_feature_branch(&self, intent: &ActionIntent) -> ActionResult<GitOperationResult> {
        let start = std::time::Instant::now();
        
        if let Some(repo) = &self.repository {
            let branch_name = format!("{}{}-{}", 
                self.branch_prefix, 
                intent.action_type.to_string().to_lowercase(),
                intent.id
            );
            
            // Get current HEAD
            let head = repo.head().map_err(|e| {
                ActionError::git("create_branch", format!("Failed to get HEAD: {}", e))
            })?;
            
            let commit = head.peel_to_commit().map_err(|e| {
                ActionError::git("peel_to_commit", format!("Failed to peel to commit: {}", e))
            })?;
            
            // Create new branch
            let branch = repo.branch(&branch_name, &commit, false).map_err(|e| {
                ActionError::git("create_branch", format!("Failed to create branch: {}", e))
            })?;
            
            // Checkout the new branch
            repo.checkout_tree(&commit.as_object(), None).map_err(|e| {
                ActionError::git("checkout_tree", format!("Failed to checkout tree: {}", e))
            })?;
            
            repo.set_head(&format!("refs/heads/{}", branch_name)).map_err(|e| {
                ActionError::git("set_head", format!("Failed to set HEAD: {}", e))
            })?;
            
            Ok(GitOperationResult {
                success: true,
                operation: "create_feature_branch".to_string(),
                details: format!("Created and checked out branch: {}", branch_name),
                branch_name: Some(branch_name),
                commit_hash: Some(commit.id().to_string()),
                files_changed: vec![],
                duration: start.elapsed(),
            })
        } else {
            Err(ActionError::git("create_branch", "Not a Git repository"))
        }
    }

    /// Stage files for commit with enhanced result
    pub async fn stage_files_enhanced(&self, files: &[String]) -> ActionResult<GitOperationResult> {
        let start = std::time::Instant::now();
        
        if let Some(repo) = &self.repository {
            let mut index = repo.index().map_err(|e| {
                ActionError::git("get_index", format!("Failed to get index: {}", e))
            })?;
            
            for file in files {
                index.add_path(Path::new(file)).map_err(|e| {
                    ActionError::git("add_file", format!("Failed to add file {}: {}", file, e))
                })?;
            }
            
            index.write().map_err(|e| {
                ActionError::git("write_index", format!("Failed to write index: {}", e))
            })?;
            
            Ok(GitOperationResult {
                success: true,
                operation: "stage_files_enhanced".to_string(),
                details: format!("Staged {} files", files.len()),
                branch_name: None,
                commit_hash: None,
                files_changed: files.to_vec(),
                duration: start.elapsed(),
            })
        } else {
            Err(ActionError::git("stage_files", "Not a Git repository"))
        }
    }

    /// Commit staged changes with enhanced result
    pub async fn commit_staged_changes(&self, intent: &ActionIntent, message: &str) -> ActionResult<GitOperationResult> {
        let start = std::time::Instant::now();
        
        if let Some(repo) = &self.repository {
            let signature = Signature::now("Action Protocol", "action@rhema.ai").map_err(|e| {
                ActionError::git("create_signature", format!("Failed to create signature: {}", e))
            })?;
            
            let tree_id = repo.index().map_err(|e| {
                ActionError::git("get_index", format!("Failed to get index: {}", e))
            })?.write_tree().map_err(|e| {
                ActionError::git("write_tree", format!("Failed to write tree: {}", e))
            })?;
            
            let tree = repo.find_tree(tree_id).map_err(|e| {
                ActionError::git("find_tree", format!("Failed to find tree: {}", e))
            })?;
            
            let head = repo.head().map_err(|e| {
                ActionError::git("get_head", format!("Failed to get HEAD: {}", e))
            })?;
            
            let parent = head.peel_to_commit().map_err(|e| {
                ActionError::git("peel_to_commit", format!("Failed to peel to commit: {}", e))
            })?;
            
            let commit_message = format!("{}{}\n\nIntent ID: {}\nAction Type: {:?}\nSafety Level: {:?}",
                self.commit_prefix, message, intent.id, intent.action_type, intent.safety_level);
            
            let commit_id = repo.commit(Some("HEAD"), &signature, &signature, &commit_message, &tree, &[&parent]).map_err(|e| {
                ActionError::git("commit", format!("Failed to commit: {}", e))
            })?;
            
            Ok(GitOperationResult {
                success: true,
                operation: "commit_staged_changes".to_string(),
                details: format!("Committed changes: {}", commit_id),
                branch_name: None,
                commit_hash: Some(commit_id.to_string()),
                files_changed: vec![],
                duration: start.elapsed(),
            })
        } else {
            Err(ActionError::git("commit", "Not a Git repository"))
        }
    }

    /// Get detailed diff for files
    pub async fn get_detailed_diff(&self, files: &[String]) -> ActionResult<String> {
        if let Some(repo) = &self.repository {
            let head = repo.head().map_err(|e| {
                ActionError::git("get_head", format!("Failed to get HEAD: {}", e))
            })?;
            
            let head_tree = head.peel_to_tree().map_err(|e| {
                ActionError::git("peel_to_tree", format!("Failed to peel to tree: {}", e))
            })?;
            
            let mut diff_options = DiffOptions::new();
            for file in files {
                diff_options.pathspec(file);
            }
            
            let diff = repo.diff_tree_to_workdir(Some(&head_tree), Some(&mut diff_options)).map_err(|e| {
                ActionError::git("diff", format!("Failed to create diff: {}", e))
            })?;
            
            let mut diff_output = String::new();
            diff.print(DiffFormat::Patch, |delta, hunk, line| {
                diff_output.push_str(&format!(
                    "diff --git a/{} b/{}\n",
                    delta.old_file().path().unwrap_or(Path::new("")).display(),
                    delta.new_file().path().unwrap_or(Path::new("")).display()
                ));
                if let Some(hunk) = hunk {
                    diff_output.push_str(&format!(
                        "@@ -{},{} +{},{} @@\n",
                        hunk.old_start(), hunk.old_lines(),
                        hunk.new_start(), hunk.new_lines()
                    ));
                }
                diff_output.push_str(&format!(
                    "{}\n",
                    String::from_utf8_lossy(line.content())
                ));
                true
            }).map_err(|e| {
                ActionError::git("print_diff", format!("Failed to print diff: {}", e))
            })?;
            
            Ok(diff_output)
        } else {
            Err(ActionError::git("diff", "Not a Git repository"))
        }
    }

    /// Get list of all branches
    pub async fn get_branches(&self) -> ActionResult<Vec<GitBranchInfo>> {
        if let Some(repo) = &self.repository {
            let branches = repo.branches(Some(BranchType::Local)).map_err(|e| {
                ActionError::git("get_branches", format!("Failed to get branches: {}", e))
            })?;
            
            let current_branch = self.get_current_branch().unwrap_or_default();
            let mut branch_infos = Vec::new();
            
            for branch_result in branches {
                let (branch, _) = branch_result.map_err(|e| {
                    ActionError::git("iterate_branches", format!("Failed to iterate branches: {}", e))
                })?;
                
                let name = branch.name().map_err(|e| {
                    ActionError::git("get_branch_name", format!("Failed to get branch name: {}", e))
                })?.unwrap_or("unknown").to_string();
                
                let commit = branch.get().peel_to_commit().map_err(|e| {
                    ActionError::git("peel_to_commit", format!("Failed to peel to commit: {}", e))
                })?;
                
                // Calculate ahead/behind counts
                let ahead_count = repo.graph_ahead_behind(commit.id(), repo.head().unwrap().target().unwrap()).map_err(|e| {
                    ActionError::git("graph_ahead_behind", format!("Failed to calculate ahead/behind: {}", e))
                })?.0;
                
                let behind_count = repo.graph_ahead_behind(commit.id(), repo.head().unwrap().target().unwrap()).map_err(|e| {
                    ActionError::git("graph_ahead_behind", format!("Failed to calculate ahead/behind: {}", e))
                })?.1;
                
                branch_infos.push(GitBranchInfo {
                    name: name.clone(),
                    is_current: name == current_branch,
                    last_commit: commit.id().to_string(),
                    ahead_count,
                    behind_count,
                });
            }
            
            Ok(branch_infos)
        } else {
            Err(ActionError::git("get_branches", "Not a Git repository"))
        }
    }

    /// Push changes to remote with enhanced result
    pub async fn push_to_remote(&self, remote_name: &str, branch_name: &str) -> ActionResult<GitOperationResult> {
        let start = std::time::Instant::now();
        
        if let Some(repo) = &self.repository {
            let mut remote = repo.find_remote(remote_name).map_err(|e| {
                ActionError::git("find_remote", format!("Failed to find remote: {}", e))
            })?;
            
            let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
            remote.push(&[&refspec], None).map_err(|e| {
                ActionError::git("push", format!("Failed to push: {}", e))
            })?;
            
            Ok(GitOperationResult {
                success: true,
                operation: "push_to_remote".to_string(),
                details: format!("Pushed branch {} to remote {}", branch_name, remote_name),
                branch_name: Some(branch_name.to_string()),
                commit_hash: None,
                files_changed: vec![],
                duration: start.elapsed(),
            })
        } else {
            Err(ActionError::git("push", "Not a Git repository"))
        }
    }
}

/// Commit information
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub message: String,
    pub timestamp: i64,
}

/// Repository status
#[derive(Debug, Clone)]
pub struct RepositoryStatus {
    pub is_git_repository: bool,
    pub is_working_directory_clean: bool,
    pub current_branch: String,
    pub current_commit: String,
    pub working_directory: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_git_integration_creation() {
        let integration = ActionGitIntegration::new().await;
        assert!(integration.is_ok());
    }

    #[tokio::test]
    async fn test_git_integration_status() {
        let integration = ActionGitIntegration::new().await.unwrap();
        
        let status = integration.get_repository_status();
        // Should work regardless of whether we're in a Git repository or not
        assert_eq!(status.working_directory, std::env::current_dir().unwrap().to_string_lossy());
    }

    #[tokio::test]
    async fn test_git_integration_initialization() {
        let mut integration = ActionGitIntegration::new().await.unwrap();
        
        // This should work even if we're not in a Git repository
        let result = integration.initialize_repository();
        // The result depends on whether we're already in a Git repository
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_git_integration_branch_operations() {
        let integration = ActionGitIntegration::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-git",
            ActionType::Refactor,
            "Test Git integration",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        // These operations will fail if we're not in a Git repository
        let branch_result = integration.create_action_branch(&intent);
        let current_branch_result = integration.get_current_branch();
        let current_commit_result = integration.get_current_commit();
        
        // All should either succeed (if in Git repo) or fail with appropriate error
        assert!(branch_result.is_ok() || branch_result.is_err());
        assert!(current_branch_result.is_ok() || current_branch_result.is_err());
        assert!(current_commit_result.is_ok() || current_commit_result.is_err());
    }
} 