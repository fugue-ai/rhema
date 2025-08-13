//! Test helper utilities for common testing operations

use git2::Repository;
use rhema_api::Rhema;
use rhema_core::RhemaResult;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test helper utilities for common operations
pub struct TestHelpers;

impl TestHelpers {
    /// Create a temporary directory for testing
    pub fn create_temp_dir() -> RhemaResult<TempDir> {
        Ok(TempDir::new()?)
    }

    /// Create a git repository in a temporary directory
    pub fn create_test_repo() -> RhemaResult<(TempDir, Repository)> {
        let temp_dir = Self::create_temp_dir()?;
        let repo_path = temp_dir.path().to_path_buf();
        let repo = Repository::init(&repo_path)?;
        Ok((temp_dir, repo))
    }

    /// Create a Rhema instance in a temporary directory
    pub fn create_test_rhema() -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, _repo) = Self::create_test_repo()?;
        let repo_path = temp_dir.path().to_path_buf();
        let rhema = Rhema::new_from_path(repo_path)?;
        Ok((temp_dir, rhema))
    }

    /// Create test files in a directory
    pub fn create_test_files(dir: &PathBuf, files: &[(&str, &str)]) -> RhemaResult<()> {
        for (filename, content) in files {
            let file_path = dir.join(filename);
            std::fs::write(file_path, content)?;
        }
        Ok(())
    }

    /// Clean up test files
    pub fn cleanup_test_files(dir: &PathBuf, files: &[&str]) -> RhemaResult<()> {
        for filename in files {
            let file_path = dir.join(filename);
            if file_path.exists() {
                std::fs::remove_file(file_path)?;
            }
        }
        Ok(())
    }

    /// Wait for a condition to be true with timeout
    pub async fn wait_for_condition<F>(condition: F, timeout_ms: u64) -> bool
    where
        F: Fn() -> bool,
    {
        use std::time::Instant;
        use tokio::time::{sleep, Duration};

        let start = Instant::now();
        let timeout = Duration::from_millis(timeout_ms);

        while start.elapsed() < timeout {
            if condition() {
                return true;
            }
            sleep(Duration::from_millis(10)).await;
        }

        false
    }

    /// Generate random test data
    pub fn generate_test_data(size: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        (0..size)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    /// Create a mock configuration for testing
    pub fn create_mock_config() -> String {
        r#"
name: test-config
version: "1.0.0"
description: "Test configuration for unit testing"
settings:
  debug: true
  timeout: 30
  retries: 3
"#
        .to_string()
    }

    /// Validate file contents
    pub fn validate_file_contents(path: &PathBuf, expected_content: &str) -> RhemaResult<bool> {
        if !path.exists() {
            return Ok(false);
        }

        let content = std::fs::read_to_string(path)?;
        Ok(content.contains(expected_content))
    }

    /// Create a test environment with specific setup
    pub fn create_test_env_with_setup<F>(setup_fn: F) -> RhemaResult<(TempDir, Rhema)>
    where
        F: FnOnce(&PathBuf) -> RhemaResult<()>,
    {
        let (temp_dir, rhema) = Self::create_test_rhema()?;
        let repo_path = temp_dir.path().to_path_buf();
        setup_fn(&repo_path)?;
        Ok((temp_dir, rhema))
    }

    /// Create a basic scope for testing
    pub fn create_basic_scope(path: &PathBuf) -> RhemaResult<()> {
        let scope_dir = path.join(".rhema");
        std::fs::create_dir_all(&scope_dir)?;

        let rhema_yaml = r#"
name: test-scope
scope_type: service
description: Test scope for unit testing
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
        std::fs::write(scope_dir.join("rhema.yaml"), rhema_yaml)?;

        // Create a simple.yaml file for testing
        let simple_yaml = r#"
items:
  - name: item1
    value: 10
    active: true
  - name: item2
    value: 20
    active: false
  - name: item3
    value: 15
    active: true
"#;
        std::fs::write(scope_dir.join("simple.yaml"), simple_yaml)?;

        Ok(())
    }

    /// Create a minimal valid Rust project structure
    pub fn create_minimal_rust_project(path: &PathBuf) -> RhemaResult<()> {
        // Create src directory
        let src_dir = path.join("src");
        std::fs::create_dir_all(&src_dir)?;

        // Create Cargo.toml
        let cargo_toml = r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
        std::fs::write(path.join("Cargo.toml"), cargo_toml)?;

        // Create lib.rs
        let lib_rs = r#"
pub fn main() {
    println!("Hello, World!");
}
"#;
        std::fs::write(src_dir.join("lib.rs"), lib_rs)?;

        // Create main.rs
        let main_rs = r#"
fn main() {
    println!("Hello, World!");
}
"#;
        std::fs::write(src_dir.join("main.rs"), main_rs)?;

        Ok(())
    }

    /// Create a test environment with a minimal Rust project
    pub fn create_test_env_with_rust_project() -> RhemaResult<(TempDir, Rhema)> {
        let (temp_dir, rhema) = Self::create_test_rhema()?;
        let repo_path = temp_dir.path().to_path_buf();
        
        // Create minimal Rust project
        Self::create_minimal_rust_project(&repo_path)?;
        
        Ok((temp_dir, rhema))
    }

    /// Create a complex scope for testing
    pub fn create_complex_scope(path: &PathBuf) -> RhemaResult<()> {
        let scope_dir = path.join(".rhema");
        std::fs::create_dir_all(&scope_dir)?;

        let scope_config = r#"
name: "complex"
version: "2.0.0"
scope_type: "library"
description: "A complex test scope with dependencies"
dependencies:
  - name: "dep1"
    version: "1.0.0"
  - name: "dep2"
    version: "2.0.0"
settings:
  debug: true
  timeout: 60
"#;

        std::fs::write(scope_dir.join("rhema.yaml"), scope_config)?;

        // Create subdirectories
        let sub_dir = scope_dir.join("src");
        std::fs::create_dir_all(&sub_dir)?;
        std::fs::write(
            sub_dir.join("main.rs"),
            "fn main() { println!(\"Hello, world!\"); }",
        )?;

        Ok(())
    }
}
