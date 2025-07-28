use crate::GacpError;
use git2::{Repository, StatusOptions};
use std::path::{Path, PathBuf};

/// Find the Git repository root from the current directory
pub fn find_repo_root() -> Result<PathBuf, GacpError> {
    let current_dir = std::env::current_dir()
        .map_err(|e| GacpError::IoError(e))?;
    
    find_repo_root_from(&current_dir)
}

/// Find the Git repository root from a specific path
pub fn find_repo_root_from(path: &Path) -> Result<PathBuf, GacpError> {
    let mut current = path;
    
    loop {
        if current.join(".git").exists() {
            return Ok(current.to_path_buf());
        }
        
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            return Err(GacpError::GitRepoNotFound(
                format!("No Git repository found from {}", path.display())
            ));
        }
    }
}

/// Get the Git repository instance
pub fn get_repo(path: &Path) -> Result<Repository, GacpError> {
    Repository::open(path).map_err(|e| GacpError::GitError(e))
}

/// Check if a file is tracked by Git
pub fn is_tracked(repo: &Repository, path: &Path) -> Result<bool, GacpError> {
    let repo_path = repo.path().parent().ok_or_else(|| {
        GacpError::GitError(git2::Error::from_str("Invalid repository path"))
    })?;
    
    let relative_path = path.strip_prefix(repo_path).map_err(|_| {
        GacpError::GitError(git2::Error::from_str("Path not in repository"))
    })?;
    
    let status = repo.status_file(relative_path).map_err(|e| GacpError::GitError(e))?;
    Ok(!status.is_empty())
}

/// Get Git status for a specific file
pub fn get_file_status(repo: &Repository, path: &Path) -> Result<git2::Status, GacpError> {
    let repo_path = repo.path().parent().ok_or_else(|| {
        GacpError::GitError(git2::Error::from_str("Invalid repository path"))
    })?;
    
    let relative_path = path.strip_prefix(repo_path).map_err(|_| {
        GacpError::GitError(git2::Error::from_str("Path not in repository"))
    })?;
    
    repo.status_file(relative_path).map_err(|e| GacpError::GitError(e))
}

/// Get all changed files in the repository
pub fn get_changed_files(repo: &Repository) -> Result<Vec<PathBuf>, GacpError> {
    let mut options = StatusOptions::new();
    options.include_untracked(true);
    options.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut options)).map_err(|e| GacpError::GitError(e))?;
    let repo_path = repo.path().parent().ok_or_else(|| {
        GacpError::GitError(git2::Error::from_str("Invalid repository path"))
    })?;
    
    let mut changed_files = Vec::new();
    
    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            let full_path = repo_path.join(path);
            changed_files.push(full_path);
        }
    }
    
    Ok(changed_files)
}

/// Get the current branch name
pub fn get_current_branch(repo: &Repository) -> Result<String, GacpError> {
    let head = repo.head().map_err(|e| GacpError::GitError(e))?;
    
    if head.is_branch() {
        let branch_name = head.name().ok_or_else(|| {
            GacpError::GitError(git2::Error::from_str("Invalid branch name"))
        })?;
        
        // Remove "refs/heads/" prefix
        Ok(branch_name.replace("refs/heads/", ""))
    } else {
        Err(GacpError::GitError(git2::Error::from_str("Not on a branch")))
    }
}

/// Get the latest commit hash
pub fn get_latest_commit(repo: &Repository) -> Result<String, GacpError> {
    let head = repo.head().map_err(|e| GacpError::GitError(e))?;
    let commit = head.peel_to_commit().map_err(|e| GacpError::GitError(e))?;
    Ok(commit.id().to_string())
}

/// Check if the working directory is clean
pub fn is_working_directory_clean(repo: &Repository) -> Result<bool, GacpError> {
    let mut options = StatusOptions::new();
    options.include_untracked(false);
    options.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut options)).map_err(|e| GacpError::GitError(e))?;
    Ok(statuses.is_empty())
}

/// Stage a file for commit
pub fn stage_file(repo: &Repository, path: &Path) -> Result<(), GacpError> {
    let repo_path = repo.path().parent().ok_or_else(|| {
        GacpError::GitError(git2::Error::from_str("Invalid repository path"))
    })?;
    
    let relative_path = path.strip_prefix(repo_path).map_err(|_| {
        GacpError::GitError(git2::Error::from_str("Path not in repository"))
    })?;
    
    let mut index = repo.index().map_err(|e| GacpError::GitError(e))?;
    index.add_path(relative_path).map_err(|e| GacpError::GitError(e))?;
    index.write().map_err(|e| GacpError::GitError(e))?;
    
    Ok(())
}

/// Commit staged changes
pub fn commit_changes(repo: &Repository, message: &str) -> Result<(), GacpError> {
    let signature = repo.signature().map_err(|e| GacpError::GitError(e))?;
    let tree_id = repo.index().map_err(|e| GacpError::GitError(e))?.write_tree().map_err(|e| GacpError::GitError(e))?;
    let tree = repo.find_tree(tree_id).map_err(|e| GacpError::GitError(e))?;
    
    let head = repo.head().map_err(|e| GacpError::GitError(e))?;
    let parent = head.peel_to_commit().map_err(|e| GacpError::GitError(e))?;
    
    repo.commit(
        Some(&head.name().unwrap()),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent],
    ).map_err(|e| GacpError::GitError(e))?;
    
    Ok(())
} 