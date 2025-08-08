//! Test fixtures for common testing scenarios

use git2::Repository;
use rhema_api::Rhema;
use rhema_core::RhemaResult;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test fixtures for different testing scenarios
pub struct TestFixtures;

impl TestFixtures {
    /// Create a basic test fixture with minimal setup
    pub fn basic() -> RhemaResult<(TempDir, Rhema)> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repository
        let _repo = Repository::init(&repo_path)?;

        // Create Rhema instance
        let rhema = Rhema::new_from_path(repo_path)?;

        Ok((temp_dir, rhema))
    }

    /// Create a test fixture with a basic scope setup
    pub fn with_scope() -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, rhema) = Self::basic()?;
        let repo_path = temp_dir.path().to_path_buf();

        // Create .rhema directory
        let rhema_dir = repo_path.join(".rhema");
        std::fs::create_dir_all(&rhema_dir)?;

        // Create basic rhema.yaml
        let rhema_yaml = r#"
name: test-scope
scope_type: service
description: Test scope for unit testing
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
        std::fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;

        Ok((temp_dir, rhema))
    }

    /// Create a test fixture with sample data files
    pub fn with_sample_data() -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, rhema) = Self::with_scope()?;
        let repo_path = temp_dir.path().to_path_buf();
        let rhema_dir = repo_path.join(".rhema");

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
        std::fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;

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
        std::fs::write(rhema_dir.join("insights.yaml"), insights_yaml)?;

        Ok((temp_dir, rhema))
    }

    /// Create a test fixture with complex data structure
    pub fn with_complex_data() -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, rhema) = Self::with_scope()?;
        let repo_path = temp_dir.path().to_path_buf();
        let rhema_dir = repo_path.join(".rhema");

        // Create subdirectories
        let data_dir = rhema_dir.join("data");
        let schemas_dir = rhema_dir.join("schemas");
        std::fs::create_dir_all(&data_dir)?;
        std::fs::create_dir_all(&schemas_dir)?;

        // Create complex data files
        let complex_data = r#"
items:
  - id: "item-001"
    name: "Complex Item 1"
    type: "service"
    metadata:
      version: "1.0.0"
      tags: ["test", "complex"]
    dependencies:
      - name: "dep-1"
        version: "1.0.0"
  - id: "item-002"
    name: "Complex Item 2"
    type: "library"
    metadata:
      version: "2.0.0"
      tags: ["production", "stable"]
"#;
        std::fs::write(data_dir.join("complex.yaml"), complex_data)?;

        // Create schema files
        let item_schema = r#"
type: object
properties:
  id:
    type: string
  name:
    type: string
  type:
    type: string
    enum: [service, library, tool]
  metadata:
    type: object
    properties:
      version:
        type: string
      tags:
        type: array
        items:
          type: string
required: [id, name, type]
"#;
        std::fs::write(schemas_dir.join("item.yaml"), item_schema)?;

        Ok((temp_dir, rhema))
    }

    /// Create a test fixture for performance testing
    pub fn for_performance_testing(size: usize) -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, rhema) = Self::with_scope()?;
        let repo_path = temp_dir.path().to_path_buf();
        let rhema_dir = repo_path.join(".rhema");

        // Generate large dataset
        let mut items = Vec::new();
        for i in 0..size {
            let item = format!(
                r#"  - id: "item-{:06}"
    name: "Performance Item {}"
    type: "service"
    metadata:
      version: "1.0.0"
      created_at: "2024-01-{:02}T10:00:00Z"
    status: "{}"
    priority: "{}""#,
                i,
                i,
                (i % 30) + 1,
                if i % 3 == 0 {
                    "active"
                } else if i % 3 == 1 {
                    "inactive"
                } else {
                    "pending"
                },
                if i % 4 == 0 {
                    "low"
                } else if i % 4 == 1 {
                    "medium"
                } else if i % 4 == 2 {
                    "high"
                } else {
                    "critical"
                }
            );
            items.push(item);
        }

        let large_data = format!("items:\n{}", items.join("\n"));
        std::fs::write(rhema_dir.join("performance.yaml"), large_data)?;

        Ok((temp_dir, rhema))
    }

    /// Create a test fixture for security testing
    pub fn for_security_testing() -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, rhema) = Self::with_scope()?;
        let repo_path = temp_dir.path().to_path_buf();
        let rhema_dir = repo_path.join(".rhema");

        // Create files with potential security issues
        let malicious_content = r#"
items:
  - id: "../../../etc/passwd"
    name: "Path traversal attempt"
    type: "malicious"
  - id: "normal-item"
    name: "Normal Item"
    type: "service"
    content: "!!python/object/apply:os.system ['echo malicious']"
"#;
        std::fs::write(rhema_dir.join("security_test.yaml"), malicious_content)?;

        Ok((temp_dir, rhema))
    }

    /// Create a test fixture with git history
    pub fn with_git_history(commits: &[(&str, &str)]) -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, rhema) = Self::basic()?;
        let repo_path = temp_dir.path().to_path_buf();
        let repo = Repository::open(&repo_path)?;

        for (message, content) in commits {
            // Create a test file
            let file_path = repo_path.join("test.txt");
            std::fs::write(&file_path, content)?;

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
            repo.set_head("refs/heads/main")?;
        }

        Ok((temp_dir, rhema))
    }
}
