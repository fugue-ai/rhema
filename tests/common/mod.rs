//! Common testing utilities and infrastructure for Rhema CLI tests

pub mod fixtures;
pub mod mocks;
pub mod helpers;
pub mod generators;
pub mod enhanced_fixtures;
pub mod enhanced_mocks;
pub mod coordination_fixtures;

use std::path::PathBuf;
use tempfile::TempDir;
use rhema::{Rhema, RhemaResult};

/// Test environment setup and teardown utilities
pub struct TestEnv {
    pub temp_dir: TempDir,
    pub rhema: Rhema,
    pub repo_path: PathBuf,
}

impl TestEnv {
    /// Create a new test environment with a git repository and Rhema instance
    pub fn new() -> RhemaResult<Self> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_path_buf();
        
        // Initialize git repository
        let _repo = git2::Repository::init(&repo_path)?;
        
        // Create Rhema instance
        let rhema = Rhema::new_from_path(repo_path.clone())?;
        
        Ok(Self {
            temp_dir,
            rhema,
            repo_path,
        })
    }
    
    /// Create a test environment with a basic scope setup
    pub fn with_scope() -> RhemaResult<Self> {
        let mut env = Self::new()?;
        
        // Create .rhema directory
        let rhema_dir = env.repo_path.join(".rhema");
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
        
        Ok(env)
    }
    
    /// Create a test environment with sample data files
    pub fn with_sample_data() -> RhemaResult<Self> {
        let mut env = Self::with_scope()?;
        
        let rhema_dir = env.repo_path.join(".rhema");
        
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
        
        Ok(env)
    }
}

/// Test result assertion utilities
pub mod assertions {
    use super::*;
    use serde_yaml::Value;
    
    /// Assert that a query result contains expected data
    pub fn assert_query_contains(result: &Value, expected: &str) {
        let result_str = serde_yaml::to_string(result).unwrap();
        assert!(
            result_str.contains(expected),
            "Query result should contain '{}', but got: {}",
            expected,
            result_str
        );
    }
    
    /// Assert that a query result does not contain unexpected data
    pub fn assert_query_not_contains(result: &Value, unexpected: &str) {
        let result_str = serde_yaml::to_string(result).unwrap();
        assert!(
            !result_str.contains(unexpected),
            "Query result should not contain '{}', but got: {}",
            unexpected,
            result_str
        );
    }
    
    /// Assert that a file exists at the given path
    pub fn assert_file_exists(path: &PathBuf) {
        assert!(
            path.exists(),
            "File should exist at: {}",
            path.display()
        );
    }
    
    /// Assert that a directory exists at the given path
    pub fn assert_dir_exists(path: &PathBuf) {
        assert!(
            path.exists() && path.is_dir(),
            "Directory should exist at: {}",
            path.display()
        );
    }
}

/// Performance testing utilities
pub mod performance {
    use std::time::{Duration, Instant};
    
    /// Measure execution time of a function
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Assert that execution time is within acceptable bounds
    pub fn assert_execution_time<F>(f: F, max_duration: Duration)
    where
        F: FnOnce(),
    {
        let (_, duration) = measure_time(f);
        assert!(
            duration <= max_duration,
            "Execution took {:?}, expected <= {:?}",
            duration,
            max_duration
        );
    }
}

/// Security testing utilities
pub mod security {
    use std::path::PathBuf;
    
    /// Test for path traversal vulnerabilities
    pub fn test_path_traversal(base_path: &PathBuf) -> Vec<String> {
        let malicious_paths = vec![
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32\\config\\sam",
            "....//....//....//etc/passwd",
            "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd",
        ];
        
        let mut vulnerabilities = Vec::new();
        
        for path in malicious_paths {
            let test_path = base_path.join(path);
            if test_path.exists() {
                vulnerabilities.push(format!("Path traversal vulnerability: {}", path));
            }
        }
        
        vulnerabilities
    }
    
    /// Test for YAML injection vulnerabilities
    pub fn test_yaml_injection() -> Vec<String> {
        let malicious_yaml = vec![
            "!!python/object/apply:os.system ['rm -rf /']",
            "!!python/object/apply:subprocess.check_output [['cat', '/etc/passwd']]",
            "!!binary |\n  Q2F0Y2ggbWUgaWYgeW91IGNhbiE=",
        ];
        
        let mut vulnerabilities = Vec::new();
        
        for yaml in malicious_yaml {
            if let Ok(_) = serde_yaml::from_str::<serde_yaml::Value>(yaml) {
                vulnerabilities.push(format!("YAML injection vulnerability: {}", yaml));
            }
        }
        
        vulnerabilities
    }
} 