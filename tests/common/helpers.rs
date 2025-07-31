//! Helper utilities for testing

use rhema::RhemaResult;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper functions for test setup and teardown
#[allow(dead_code)]
pub struct TestHelpers;

#[allow(dead_code)]
impl TestHelpers {
    /// Create a temporary directory with git repository initialized
    pub fn create_temp_git_repo() -> RhemaResult<(TempDir, PathBuf)> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repository
        let _repo = git2::Repository::init(&repo_path)?;

        Ok((temp_dir, repo_path))
    }

    /// Create a basic Rhema scope structure
    pub fn create_basic_scope(repo_path: &PathBuf) -> RhemaResult<()> {
        let rhema_dir = repo_path.join(".rhema");
        fs::create_dir_all(&rhema_dir)?;

        // Create rhema.yaml
        let rhema_yaml = r#"
name: simple
scope_type: service
description: Test scope for unit testing
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
        fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;

        // Create simple.yaml
        let simple_yaml = r#"
items:
  - id: "item-001"
    name: "Test Item 1"
    active: true
    created_at: "2024-01-15T10:00:00Z"
  - id: "item-002"
    name: "Test Item 2"
    active: false
    created_at: "2024-01-16T10:00:00Z"
"#;
        fs::write(rhema_dir.join("simple.yaml"), simple_yaml)?;

        Ok(())
    }

    /// Create a complex Rhema scope structure with multiple files
    pub fn create_complex_scope(repo_path: &PathBuf) -> RhemaResult<()> {
        let rhema_dir = repo_path.join(".rhema");
        fs::create_dir_all(&rhema_dir)?;

        // Create rhema.yaml
        let rhema_yaml = r#"
name: complex-scope
scope_type: service
description: Complex test scope with multiple data files
version: "2.0.0"
schema_version: "1.0.0"
dependencies:
  - name: dependency-1
    version: "1.0.0"
    type: service
"#;
        fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;

        // Create todos.yaml
        let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Test todo 1"
    status: pending
    priority: high
    created_at: "2024-01-15T10:00:00Z"
  - id: "todo-002"
    title: "Test todo 2"
    status: completed
    priority: medium
    created_at: "2024-01-16T10:00:00Z"
"#;
        fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;

        // Create insights.yaml
        let insights_yaml = r#"
insights:
  - id: "insight-001"
    title: "Test insight"
    content: "This is a test insight"
    confidence: 8
    category: "testing"
    created_at: "2024-01-15T10:00:00Z"
"#;
        fs::write(rhema_dir.join("insights.yaml"), insights_yaml)?;

        Ok(())
    }

    /// Create a nested Rhema scope structure
    pub fn create_nested_scope(repo_path: &PathBuf) -> RhemaResult<()> {
        let rhema_dir = repo_path.join(".rhema");
        fs::create_dir_all(&rhema_dir)?;

        // Create subdirectories
        let data_dir = rhema_dir.join("data");
        let schemas_dir = rhema_dir.join("schemas");
        fs::create_dir_all(&data_dir)?;
        fs::create_dir_all(&schemas_dir)?;

        // Create rhema.yaml
        let rhema_yaml = r#"
name: nested-scope
scope_type: service
description: Nested test scope structure
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
        fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;

        // Create data files in subdirectories
        let todos_yaml = r#"
todos:
  - id: "todo-001"
    title: "Nested todo"
    status: pending
    priority: high
    created_at: "2024-01-15T10:00:00Z"
"#;
        fs::write(data_dir.join("todos.yaml"), todos_yaml)?;

        // Create schema files
        let todo_schema = r#"
type: object
properties:
  id:
    type: string
  title:
    type: string
  status:
    type: string
    enum: [pending, completed]
required: [id, title, status]
"#;
        fs::write(schemas_dir.join("todo.yaml"), todo_schema)?;

        Ok(())
    }

    /// Create a large dataset for performance testing
    pub fn create_large_dataset(repo_path: &PathBuf, size: usize) -> RhemaResult<()> {
        let rhema_dir = repo_path.join(".rhema");
        fs::create_dir_all(&rhema_dir)?;

        // Create rhema.yaml
        let rhema_yaml = r#"
name: large-scope
scope_type: service
description: Large dataset for performance testing
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
        fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;

        // Generate large todos dataset
        let mut todos = Vec::new();
        for i in 0..size {
            let todo = format!(
                r#"  - id: "todo-{:06}"
    title: "Large dataset todo {}"
    status: {}
    priority: {}
    created_at: "2024-01-{:02}T10:00:00Z""#,
                i,
                i,
                if i % 3 == 0 {
                    "pending"
                } else if i % 3 == 1 {
                    "in_progress"
                } else {
                    "completed"
                },
                if i % 4 == 0 {
                    "low"
                } else if i % 4 == 1 {
                    "medium"
                } else if i % 4 == 2 {
                    "high"
                } else {
                    "critical"
                },
                (i % 30) + 1
            );
            todos.push(todo);
        }

        let todos_yaml = format!("todos:\n{}", todos.join("\n"));
        fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;

        Ok(())
    }

    /// Create a git repository with commit history
    pub fn create_git_history(repo_path: &PathBuf, commits: &[(&str, &str)]) -> RhemaResult<()> {
        let repo = git2::Repository::open(repo_path)?;

        for (message, content) in commits {
            // Create a test file
            let file_path = repo_path.join("test.txt");
            fs::write(&file_path, content)?;

            // Stage and commit
            let mut index = repo.index()?;
            index.add_path(&std::path::Path::new("test.txt"))?;
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;

            let signature = git2::Signature::now("Test User", "test@example.com")?;
            let parent_commit = repo.head().ok().and_then(|head| head.peel_to_commit().ok());

            let _commit_id = if let Some(parent) = parent_commit {
                repo.commit(
                    Some(&repo.head()?.name().unwrap()),
                    &signature,
                    &signature,
                    message,
                    &tree,
                    &[&parent],
                )?
            } else {
                repo.commit(
                    Some("refs/heads/main"),
                    &signature,
                    &signature,
                    message,
                    &tree,
                    &[],
                )?
            };

            // Update HEAD
            repo.set_head(&format!("refs/heads/main"))?;
        }

        Ok(())
    }

    /// Create a file with specific permissions
    pub fn create_file_with_permissions(
        path: &PathBuf,
        content: &str,
        permissions: u32,
    ) -> RhemaResult<()> {
        fs::write(path, content)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(permissions);
            fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// Create a directory with specific permissions
    pub fn create_dir_with_permissions(path: &PathBuf, permissions: u32) -> RhemaResult<()> {
        fs::create_dir_all(path)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(permissions);
            fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// Generate random test data
    pub fn generate_random_data(size: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut data = Vec::new();
        for i in 0..size {
            let id = rng.gen_range(1000..9999);
            let status = if rng.gen_bool(0.5) {
                "pending"
            } else {
                "completed"
            };
            let priority = match rng.gen_range(0..4) {
                0 => "low",
                1 => "medium",
                2 => "high",
                _ => "critical",
            };

            data.push(format!(
                r#"  - id: "todo-{}"
    title: "Random todo {}"
    status: {}
    priority: {}
    created_at: "2024-01-{:02}T10:00:00Z""#,
                id,
                i,
                status,
                priority,
                (i % 30) + 1
            ));
        }

        format!("todos:\n{}", data.join("\n"))
    }

    /// Create a corrupted YAML file
    pub fn create_corrupted_yaml(path: &PathBuf) -> RhemaResult<()> {
        let corrupted_content = r#"
todos:
  - id: "todo-001"
    title: "Test todo"
    status: pending
    priority: high
    created_at: "2024-01-15T10:00:00Z"
  - id: "todo-002"
    title: "Corrupted todo
    status: pending
    priority: high
    created_at: "2024-01-16T10:00:00Z"
"#;
        fs::write(path, corrupted_content)?;
        Ok(())
    }

    /// Create a malicious YAML file for security testing
    pub fn create_malicious_yaml(path: &PathBuf) -> RhemaResult<()> {
        let malicious_content = r#"
!!python/object/apply:os.system ['echo "malicious code executed"']
todos:
  - id: "todo-001"
    title: "Malicious todo"
    status: pending
    priority: high
"#;
        fs::write(path, malicious_content)?;
        Ok(())
    }

    /// Create a file with path traversal attempt
    pub fn create_path_traversal_file(path: &PathBuf) -> RhemaResult<()> {
        let traversal_content = r#"
todos:
  - id: "../../../etc/passwd"
    title: "Path traversal attempt"
    status: pending
    priority: high
"#;
        fs::write(path, traversal_content)?;
        Ok(())
    }

    /// Create a large file for performance testing
    pub fn create_large_file(path: &PathBuf, size_mb: usize) -> RhemaResult<()> {
        let mut content = String::new();
        let chunk = "This is a test chunk for large file generation. ".repeat(100);

        for i in 0..(size_mb * 1024 * 1024 / chunk.len()) {
            content.push_str(&format!("Chunk {}: {}", i, chunk));
        }

        fs::write(path, content)?;
        Ok(())
    }

    /// Create a symbolic link
    pub fn create_symlink(target: &PathBuf, link: &PathBuf) -> RhemaResult<()> {
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(target, link)?;
        }

        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_file(target, link)?;
        }

        Ok(())
    }

    /// Create a hard link
    pub fn create_hardlink(target: &PathBuf, link: &PathBuf) -> RhemaResult<()> {
        fs::hard_link(target, link)?;
        Ok(())
    }

    /// Create a file with specific encoding
    pub fn create_file_with_encoding(
        path: &PathBuf,
        content: &str,
        encoding: &str,
    ) -> RhemaResult<()> {
        match encoding {
            "utf8" => fs::write(path, content)?,
            "utf16" => {
                let utf16_bytes: Vec<u8> = content
                    .encode_utf16()
                    .flat_map(|c| c.to_le_bytes())
                    .collect();
                fs::write(path, utf16_bytes)?;
            }
            "ascii" => {
                let ascii_bytes: Vec<u8> = content.bytes().filter(|&b| b < 128).collect();
                fs::write(path, ascii_bytes)?;
            }
            _ => {
                return Err(rhema::RhemaError::ConfigError(format!(
                    "Unsupported encoding: {}",
                    encoding
                )))
            }
        }
        Ok(())
    }

    /// Create a file with specific line endings
    pub fn create_file_with_line_endings(
        path: &PathBuf,
        content: &str,
        line_ending: &str,
    ) -> RhemaResult<()> {
        let processed_content = content.replace("\n", line_ending);
        fs::write(path, processed_content)?;
        Ok(())
    }

    /// Create a file with specific ownership (Unix only)
    #[cfg(unix)]
    pub fn create_file_with_ownership(path: &PathBuf, uid: u32, gid: u32) -> RhemaResult<()> {
        use std::os::unix::fs::chown;
        fs::write(path, "test content")?;
        chown(path, Some(uid), Some(gid))?;
        Ok(())
    }

    /// Create a file with specific ownership (Windows only)
    #[cfg(windows)]
    pub fn create_file_with_ownership(path: &PathBuf, _uid: u32, _gid: u32) -> RhemaResult<()> {
        fs::write(path, "test content")?;
        Ok(())
    }

    /// Create a file with specific timestamps
    pub fn create_file_with_timestamps(
        path: &PathBuf,
        _access_time: std::time::SystemTime,
        _modify_time: std::time::SystemTime,
    ) -> RhemaResult<()> {
        fs::write(path, "test content")?;

        #[cfg(unix)]
        {
            // Note: FileTimesExt is not available in all Rust versions
            // This is a simplified version that just creates the file
            // In a real implementation, you would handle timestamps differently
        }

        Ok(())
    }

    /// Create a file with specific attributes
    pub fn create_file_with_attributes(path: &PathBuf, attributes: &[&str]) -> RhemaResult<()> {
        fs::write(path, "test content")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)?.permissions();

            for attr in attributes {
                match *attr {
                    "readonly" => perms.set_readonly(true),
                    "executable" => perms.set_mode(0o755),
                    "hidden" => {
                        // Unix doesn't have hidden attribute, but we can set it to 0o600
                        perms.set_mode(0o600);
                    }
                    _ => {}
                }
            }

            fs::set_permissions(path, perms)?;
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            let metadata = fs::metadata(path)?;
            let mut attrs = metadata.file_attributes();

            for attr in attributes {
                match *attr {
                    "readonly" => attrs |= 0x1, // FILE_ATTRIBUTE_READONLY
                    "hidden" => attrs |= 0x2,   // FILE_ATTRIBUTE_HIDDEN
                    "system" => attrs |= 0x4,   // FILE_ATTRIBUTE_SYSTEM
                    _ => {}
                }
            }

            // Note: Setting file attributes on Windows requires additional work
            // This is a simplified version
        }

        Ok(())
    }
}
