use rhema_core::{RhemaError, RhemaResult};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git hook types
#[derive(Debug, Clone)]
pub enum HookType {
    PreCommit,
    PostCommit,
    PrePush,
    PostPush,
    PreMerge,
    PostMerge,
    PreRebase,
    PostRebase,
}

impl HookType {
    /// Get the filename for this hook type
    pub fn filename(&self) -> &'static str {
        match self {
            HookType::PreCommit => "pre-commit",
            HookType::PostCommit => "post-commit",
            HookType::PrePush => "pre-push",
            HookType::PostPush => "post-push",
            HookType::PreMerge => "pre-merge-commit",
            HookType::PostMerge => "post-merge",
            HookType::PreRebase => "pre-rebase",
            HookType::PostRebase => "post-rebase",
        }
    }

    /// Get a description for this hook type
    pub fn description(&self) -> &'static str {
        match self {
            HookType::PreCommit => "Runs before commit is created",
            HookType::PostCommit => "Runs after commit is created",
            HookType::PrePush => "Runs before push to remote",
            HookType::PostPush => "Runs after push to remote",
            HookType::PreMerge => "Runs before merge commit",
            HookType::PostMerge => "Runs after merge commit",
            HookType::PreRebase => "Runs before rebase",
            HookType::PostRebase => "Runs after rebase",
        }
    }
}

impl std::fmt::Display for HookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filename())
    }
}

/// Hook execution result
#[derive(Debug, Clone)]
pub struct HookResult {
    pub success: bool,
    pub hook_type: HookType,
    pub messages: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Git hooks manager
pub struct GitHooksManager {
    repo_path: PathBuf,
    hooks_dir: PathBuf,
}

impl GitHooksManager {
    /// Create a new Git hooks manager
    pub fn new(repo_path: &Path) -> RhemaResult<Self> {
        let hooks_dir = repo_path.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(RhemaError::ConfigError(
                "Git hooks directory not found".to_string(),
            ));
        }

        Ok(Self {
            repo_path: repo_path.to_path_buf(),
            hooks_dir,
        })
    }

    /// Install a hook script
    pub fn install_hook(&self, hook_type: &HookType, script_content: &str) -> RhemaResult<()> {
        let hook_name = self.get_hook_filename(hook_type);
        let hook_path = self.hooks_dir.join(hook_name);

        // Write the hook script
        fs::write(&hook_path, script_content)
            .map_err(|e| RhemaError::ConfigError(format!("Failed to write hook script: {}", e)))?;

        // Make the hook executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms).map_err(|e| {
                RhemaError::ConfigError(format!("Failed to set hook permissions: {}", e))
            })?;
        }

        Ok(())
    }

    /// Execute a hook
    pub fn execute_hook(&self, hook_type: &HookType) -> RhemaResult<HookResult> {
        let hook_name = self.get_hook_filename(hook_type);
        let hook_path = self.hooks_dir.join(hook_name);

        if !hook_path.exists() {
            return Ok(HookResult {
                success: true,
                hook_type: hook_type.clone(),
                messages: vec!["Hook not installed, skipping".to_string()],
                errors: vec![],
                warnings: vec![],
            });
        }

        // Execute the hook script
        let output = Command::new(&hook_path)
            .current_dir(&self.repo_path)
            .output()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to execute hook: {}", e)))?;

        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let messages: Vec<String> = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.to_string())
            .collect();

        let errors: Vec<String> = stderr
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.to_string())
            .collect();

        Ok(HookResult {
            success,
            hook_type: hook_type.clone(),
            messages,
            errors,
            warnings: vec![],
        })
    }

    /// Get the filename for a hook type
    fn get_hook_filename(&self, hook_type: &HookType) -> String {
        match hook_type {
            HookType::PreCommit => "pre-commit".to_string(),
            HookType::PostCommit => "post-commit".to_string(),
            HookType::PrePush => "pre-push".to_string(),
            HookType::PostPush => "post-push".to_string(),
            HookType::PreMerge => "pre-merge-commit".to_string(),
            HookType::PostMerge => "post-merge".to_string(),
            HookType::PreRebase => "pre-rebase".to_string(),
            HookType::PostRebase => "post-rebase".to_string(),
        }
    }

    /// Install default Rhema hooks
    pub fn install_default_hooks(&self) -> RhemaResult<()> {
        // Pre-commit hook for validation
        let pre_commit_script = r#"#!/bin/sh
# Rhema pre-commit hook
echo "Running Rhema pre-commit validation..."

# Check for TODO comments in staged files
if git diff --cached --name-only | xargs grep -l "TODO\|FIXME" 2>/dev/null; then
    echo "Warning: Found TODO/FIXME comments in staged files"
fi

# Check for large files
git diff --cached --name-only | while read file; do
    if [ -f "$file" ]; then
        size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo 0)
        if [ "$size" -gt 10485760 ]; then
            echo "Warning: Large file detected: $file ($size bytes)"
        fi
    fi
done

echo "Pre-commit validation completed"
"#;

        // Post-commit hook for notifications
        let post_commit_script = r#"#!/bin/sh
# Rhema post-commit hook
echo "Running Rhema post-commit actions..."

# Get commit information
commit_hash=$(git rev-parse HEAD)
commit_message=$(git log -1 --pretty=format:%s)
branch_name=$(git branch --show-current)

echo "Commit $commit_hash on branch $branch_name: $commit_message"

# Here you could add notifications, logging, etc.
echo "Post-commit actions completed"
"#;

        // Pre-push hook for validation
        let pre_push_script = r#"#!/bin/sh
# Rhema pre-push hook
echo "Running Rhema pre-push validation..."

# Check if tests pass (if there's a test command)
if command -v cargo >/dev/null 2>&1; then
    echo "Running tests..."
    cargo test
    if [ $? -ne 0 ]; then
        echo "Tests failed, push aborted"
        exit 1
    fi
fi

echo "Pre-push validation completed"
"#;

        self.install_hook(&HookType::PreCommit, pre_commit_script)?;
        self.install_hook(&HookType::PostCommit, post_commit_script)?;
        self.install_hook(&HookType::PrePush, pre_push_script)?;

        Ok(())
    }

    /// List installed hooks
    pub fn list_hooks(&self) -> RhemaResult<Vec<String>> {
        let mut hooks = Vec::new();

        for entry in fs::read_dir(&self.hooks_dir).map_err(|e| {
            RhemaError::ConfigError(format!("Failed to read hooks directory: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                RhemaError::ConfigError(format!("Failed to read hook entry: {}", e))
            })?;

            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        hooks.push(name_str.to_string());
                    }
                }
            }
        }

        Ok(hooks)
    }

    /// Remove a hook
    pub fn remove_hook(&self, hook_type: &HookType) -> RhemaResult<()> {
        let hook_name = self.get_hook_filename(hook_type);
        let hook_path = self.hooks_dir.join(hook_name);

        if hook_path.exists() {
            fs::remove_file(&hook_path)
                .map_err(|e| RhemaError::ConfigError(format!("Failed to remove hook: {}", e)))?;
        }

        Ok(())
    }
}
