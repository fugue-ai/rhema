use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::RhemaResult;

/// Git integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitIntegrationConfig {
    /// Whether Git integration is enabled
    pub enabled: bool,
    
    /// Whether to monitor file changes
    pub monitor_file_changes: bool,
    
    /// Whether to monitor branch changes
    pub monitor_branch_changes: bool,
    
    /// Whether to monitor commit changes
    pub monitor_commit_changes: bool,
    
    /// File patterns to monitor
    pub monitored_patterns: Vec<String>,
    
    /// File patterns to ignore
    pub ignored_patterns: Vec<String>,
    
    /// Whether to auto-discover scopes on changes
    pub auto_discover_on_changes: bool,
    
    /// Whether to show notifications
    pub show_notifications: bool,
    
    /// Hook installation directory
    pub hooks_directory: Option<String>,
    
    /// Whether to install Git hooks automatically
    pub install_hooks_automatically: bool,
}

/// Git repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRepoInfo {
    /// Repository root path
    pub root_path: PathBuf,
    
    /// Current branch
    pub current_branch: String,
    
    /// Last commit hash
    pub last_commit: String,
    
    /// Repository URL
    pub remote_url: Option<String>,
    
    /// Whether it's a Git repository
    pub is_git_repo: bool,
    
    /// Git hooks directory
    pub hooks_directory: Option<PathBuf>,
}

/// Git change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GitChangeEvent {
    /// File was modified
    FileModified {
        path: PathBuf,
        timestamp: DateTime<Utc>,
    },
    
    /// File was added
    FileAdded {
        path: PathBuf,
        timestamp: DateTime<Utc>,
    },
    
    /// File was deleted
    FileDeleted {
        path: PathBuf,
        timestamp: DateTime<Utc>,
    },
    
    /// Branch was changed
    BranchChanged {
        old_branch: String,
        new_branch: String,
        timestamp: DateTime<Utc>,
    },
    
    /// New commit was made
    CommitMade {
        commit_hash: String,
        message: String,
        timestamp: DateTime<Utc>,
    },
}

/// Git hook types
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum GitHookType {
    PreCommit,
    PostCommit,
    PrePush,
    PostPush,
    PreReceive,
    PostReceive,
    PreMergeCommit,
    PostMergeCommit,
    PreRebase,
    PostRebase,
}

/// Git integration manager
pub struct GitIntegrationManager {
    config: GitIntegrationConfig,
    repo_info: Arc<RwLock<GitRepoInfo>>,
    change_history: Arc<RwLock<Vec<GitChangeEvent>>>,
    hooks_installed: Arc<RwLock<HashMap<GitHookType, bool>>>,
}

impl Default for GitIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            monitor_file_changes: true,
            monitor_branch_changes: true,
            monitor_commit_changes: true,
            monitored_patterns: vec![
                "package.json".to_string(),
                "Cargo.toml".to_string(),
                "pyproject.toml".to_string(),
                "go.mod".to_string(),
                "pom.xml".to_string(),
                "build.gradle".to_string(),
                "project.json".to_string(),
                "nx.json".to_string(),
            ],
            ignored_patterns: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
                "*.log".to_string(),
                "*.tmp".to_string(),
            ],
            auto_discover_on_changes: true,
            show_notifications: true,
            hooks_directory: None,
            install_hooks_automatically: true,
        }
    }
}

impl GitIntegrationManager {
    /// Create a new Git integration manager
    pub fn new(repo_path: &Path, config: GitIntegrationConfig) -> RhemaResult<Self> {
        let repo_info = Self::discover_git_repo(repo_path)?;
        
        Ok(Self {
            config,
            repo_info: Arc::new(RwLock::new(repo_info)),
            change_history: Arc::new(RwLock::new(Vec::new())),
            hooks_installed: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Discover Git repository information
    fn discover_git_repo(repo_path: &Path) -> RhemaResult<GitRepoInfo> {
        let git_dir = repo_path.join(".git");
        let is_git_repo = git_dir.exists() && git_dir.is_dir();
        
        if !is_git_repo {
            return Ok(GitRepoInfo {
                root_path: repo_path.to_path_buf(),
                current_branch: String::new(),
                last_commit: String::new(),
                remote_url: None,
                is_git_repo: false,
                hooks_directory: None,
            });
        }

        // Get current branch
        let head_file = git_dir.join("HEAD");
        let current_branch = if head_file.exists() {
            let content = fs::read_to_string(&head_file)
                .map_err(|e| crate::RhemaError::IoError(e))?;
            
            if content.starts_with("ref: refs/heads/") {
                content.trim_start_matches("ref: refs/heads/").trim().to_string()
            } else {
                "detached".to_string()
            }
        } else {
            "unknown".to_string()
        };

        // Get last commit
        let last_commit = Self::get_last_commit(&git_dir)?;

        // Get remote URL
        let config_file = git_dir.join("config");
        let remote_url = if config_file.exists() {
            Self::extract_remote_url(&config_file)?
        } else {
            None
        };

        // Get hooks directory
        let hooks_directory = Some(git_dir.join("hooks"));

        Ok(GitRepoInfo {
            root_path: repo_path.to_path_buf(),
            current_branch,
            last_commit,
            remote_url,
            is_git_repo: true,
            hooks_directory,
        })
    }

    /// Get the last commit hash
    fn get_last_commit(git_dir: &Path) -> RhemaResult<String> {
        let head_file = git_dir.join("HEAD");
        if !head_file.exists() {
            return Ok(String::new());
        }

        let content = fs::read_to_string(&head_file)
            .map_err(|e| crate::RhemaError::IoError(e))?;

        if content.starts_with("ref: refs/heads/") {
            let ref_path = content.trim_start_matches("ref: ").trim();
            let ref_file = git_dir.join(ref_path);
            
            if ref_file.exists() {
                let commit_hash = fs::read_to_string(&ref_file)
                    .map_err(|e| crate::RhemaError::IoError(e))?;
                Ok(commit_hash.trim().to_string())
            } else {
                Ok(String::new())
            }
        } else {
            Ok(content.trim().to_string())
        }
    }

    /// Extract remote URL from Git config
    fn extract_remote_url(config_file: &Path) -> RhemaResult<Option<String>> {
        let content = fs::read_to_string(config_file)
            .map_err(|e| crate::RhemaError::IoError(e))?;

        for line in content.lines() {
            if line.trim().starts_with("url = ") {
                let url = line.trim_start_matches("url = ").trim();
                return Ok(Some(url.to_string()));
            }
        }

        Ok(None)
    }

    /// Check if a file should be monitored
    pub fn should_monitor_file(&self, file_path: &Path) -> bool {
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Check ignored patterns first
        for pattern in &self.config.ignored_patterns {
            if Self::matches_pattern(file_name, pattern) {
                return false;
            }
        }

        // Check monitored patterns
        for pattern in &self.config.monitored_patterns {
            if Self::matches_pattern(file_name, pattern) {
                return true;
            }
        }

        false
    }

    /// Check if a filename matches a pattern
    fn matches_pattern(file_name: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            // Simple wildcard matching
            let pattern_parts: Vec<&str> = pattern.split('*').collect();
            if pattern_parts.len() == 2 {
                file_name.starts_with(pattern_parts[0]) && file_name.ends_with(pattern_parts[1])
            } else {
                file_name.contains(pattern_parts[0])
            }
        } else {
            file_name == pattern
        }
    }

    /// Record a file change event
    pub async fn record_file_change(
        &self,
        file_path: PathBuf,
        event_type: &str,
    ) -> RhemaResult<()> {
        if !self.config.enabled || !self.config.monitor_file_changes {
            return Ok(());
        }

        if !self.should_monitor_file(&file_path) {
            return Ok(());
        }

        let event = match event_type {
            "modified" => GitChangeEvent::FileModified {
                path: file_path.clone(),
                timestamp: Utc::now(),
            },
            "added" => GitChangeEvent::FileAdded {
                path: file_path.clone(),
                timestamp: Utc::now(),
            },
            "deleted" => GitChangeEvent::FileDeleted {
                path: file_path.clone(),
                timestamp: Utc::now(),
            },
            _ => return Ok(()),
        };

        let mut history = self.change_history.write().await;
        history.push(event);

        // Keep only last 100 events
        if history.len() > 100 {
            history.remove(0);
        }

        if self.config.show_notifications {
            println!("📁 Git change detected: {} {}", event_type, file_path.display());
        }

        Ok(())
    }

    /// Record a branch change event
    pub async fn record_branch_change(
        &self,
        old_branch: String,
        new_branch: String,
    ) -> RhemaResult<()> {
        if !self.config.enabled || !self.config.monitor_branch_changes {
            return Ok(());
        }

        let event = GitChangeEvent::BranchChanged {
            old_branch: old_branch.clone(),
            new_branch: new_branch.clone(),
            timestamp: Utc::now(),
        };

        let mut history = self.change_history.write().await;
        history.push(event);

        // Keep only last 100 events
        if history.len() > 100 {
            history.remove(0);
        }

        if self.config.show_notifications {
            println!("🌿 Branch changed: {} → {}", old_branch, new_branch);
        }

        Ok(())
    }

    /// Record a commit event
    pub async fn record_commit(
        &self,
        commit_hash: String,
        message: String,
    ) -> RhemaResult<()> {
        if !self.config.enabled || !self.config.monitor_commit_changes {
            return Ok(());
        }

        let event = GitChangeEvent::CommitMade {
            commit_hash: commit_hash.clone(),
            message: message.clone(),
            timestamp: Utc::now(),
        };

        let mut history = self.change_history.write().await;
        history.push(event);

        // Keep only last 100 events
        if history.len() > 100 {
            history.remove(0);
        }

        if self.config.show_notifications {
            println!("💾 Commit made: {}", message);
        }

        Ok(())
    }

    /// Install Git hooks
    pub async fn install_hooks(&self) -> RhemaResult<()> {
        if !self.config.enabled || !self.config.install_hooks_automatically {
            return Ok(());
        }

        let repo_info = self.repo_info.read().await;
        if !repo_info.is_git_repo {
            return Ok(());
        }

        let hooks_dir = match &repo_info.hooks_directory {
            Some(dir) => dir,
            None => return Ok(()),
        };

        // Create hooks directory if it doesn't exist
        if !hooks_dir.exists() {
            fs::create_dir_all(hooks_dir)
                .map_err(|e| crate::RhemaError::IoError(e))?;
        }

        // Install pre-commit hook
        self.install_hook(GitHookType::PreCommit, hooks_dir).await?;

        // Install post-commit hook
        self.install_hook(GitHookType::PostCommit, hooks_dir).await?;

        println!("✅ Git hooks installed successfully");

        Ok(())
    }

    /// Install a specific Git hook
    async fn install_hook(&self, hook_type: GitHookType, hooks_dir: &Path) -> RhemaResult<()> {
        let hook_name = match hook_type {
            GitHookType::PreCommit => "pre-commit",
            GitHookType::PostCommit => "post-commit",
            GitHookType::PrePush => "pre-push",
            GitHookType::PostPush => "post-push",
            GitHookType::PreReceive => "pre-receive",
            GitHookType::PostReceive => "post-receive",
            GitHookType::PreMergeCommit => "pre-merge-commit",
            GitHookType::PostMergeCommit => "post-merge-commit",
            GitHookType::PreRebase => "pre-rebase",
            GitHookType::PostRebase => "post-rebase",
        };

        let hook_path = hooks_dir.join(hook_name);
        
        // Create hook script
        let hook_script = self.generate_hook_script(&hook_type)?;
        
        fs::write(&hook_path, hook_script)
            .map_err(|e| crate::RhemaError::IoError(e))?;

        // Make hook executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)
                .map_err(|e| crate::RhemaError::IoError(e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)
                .map_err(|e| crate::RhemaError::IoError(e))?;
        }

        let mut hooks_installed = self.hooks_installed.write().await;
        hooks_installed.insert(hook_type, true);

        Ok(())
    }

    /// Generate hook script content
    fn generate_hook_script(&self, hook_type: &GitHookType) -> RhemaResult<String> {
        let script = match hook_type {
            GitHookType::PreCommit => {
                r#"#!/bin/sh
# Rhema scope loader pre-commit hook
echo "🔍 Rhema: Checking for scope changes..."

# Run scope discovery
rhema scope-loader auto-discover --auto-create --confidence 0.8

# Continue with commit
exit 0
"#
            },
            GitHookType::PostCommit => {
                r#"#!/bin/sh
# Rhema scope loader post-commit hook
echo "✅ Rhema: Commit completed, updating scope information..."

# Update scope information
rhema scope-loader update-scopes

echo "✅ Rhema: Scope information updated"
"#
            },
            _ => {
                r#"#!/bin/sh
# Rhema scope loader hook
echo "🔍 Rhema: Processing Git event..."
exit 0
"#
            },
        };

        Ok(script.to_string())
    }

    /// Uninstall Git hooks
    pub async fn uninstall_hooks(&self) -> RhemaResult<()> {
        let repo_info = self.repo_info.read().await;
        if !repo_info.is_git_repo {
            return Ok(());
        }

        let hooks_dir = match &repo_info.hooks_directory {
            Some(dir) => dir,
            None => return Ok(()),
        };

        let hook_types = [
            GitHookType::PreCommit,
            GitHookType::PostCommit,
            GitHookType::PrePush,
            GitHookType::PostPush,
        ];

        for hook_type in &hook_types {
            let hook_name = match hook_type {
                GitHookType::PreCommit => "pre-commit",
                GitHookType::PostCommit => "post-commit",
                GitHookType::PrePush => "pre-push",
                GitHookType::PostPush => "post-push",
                _ => continue,
            };

            let hook_path = hooks_dir.join(hook_name);
            if hook_path.exists() {
                fs::remove_file(&hook_path)
                    .map_err(|e| crate::RhemaError::IoError(e))?;
            }
        }

        let mut hooks_installed = self.hooks_installed.write().await;
        hooks_installed.clear();

        println!("🗑️ Git hooks uninstalled successfully");

        Ok(())
    }

    /// Get change history
    pub async fn get_change_history(&self) -> Vec<GitChangeEvent> {
        self.change_history.read().await.clone()
    }

    /// Get repository information
    pub async fn get_repo_info(&self) -> GitRepoInfo {
        self.repo_info.read().await.clone()
    }

    /// Update repository information
    pub async fn update_repo_info(&self) -> RhemaResult<()> {
        let repo_info = self.repo_info.read().await;
        let new_repo_info = Self::discover_git_repo(&repo_info.root_path)?;
        
        drop(repo_info);
        
        let mut repo_info = self.repo_info.write().await;
        *repo_info = new_repo_info;
        
        Ok(())
    }

    /// Check if hooks are installed
    pub async fn are_hooks_installed(&self) -> HashMap<GitHookType, bool> {
        self.hooks_installed.read().await.clone()
    }

    /// Get configuration
    pub fn get_config(&self) -> &GitIntegrationConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: GitIntegrationConfig) {
        self.config = config;
    }

    /// Check if Git integration is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Enable Git integration
    pub fn enable(&mut self) {
        self.config.enabled = true;
    }

    /// Disable Git integration
    pub fn disable(&mut self) {
        self.config.enabled = false;
    }
}

/// Git hook handler trait
pub trait GitHookHandler: Send + Sync {
    /// Handle pre-commit hook
    fn handle_pre_commit(&self, repo_path: &Path) -> RhemaResult<()>;
    
    /// Handle post-commit hook
    fn handle_post_commit(&self, repo_path: &Path) -> RhemaResult<()>;
    
    /// Handle pre-push hook
    fn handle_pre_push(&self, repo_path: &Path) -> RhemaResult<()>;
    
    /// Handle post-push hook
    fn handle_post_push(&self, repo_path: &Path) -> RhemaResult<()>;
}

/// Default Git hook handler
pub struct DefaultGitHookHandler;

impl GitHookHandler for DefaultGitHookHandler {
    fn handle_pre_commit(&self, _repo_path: &Path) -> RhemaResult<()> {
        // Default implementation does nothing
        Ok(())
    }

    fn handle_post_commit(&self, _repo_path: &Path) -> RhemaResult<()> {
        // Default implementation does nothing
        Ok(())
    }

    fn handle_pre_push(&self, _repo_path: &Path) -> RhemaResult<()> {
        // Default implementation does nothing
        Ok(())
    }

    fn handle_post_push(&self, _repo_path: &Path) -> RhemaResult<()> {
        // Default implementation does nothing
        Ok(())
    }
}
