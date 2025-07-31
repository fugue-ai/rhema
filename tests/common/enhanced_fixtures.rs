//! Enhanced test fixtures for comprehensive testing
//! Provides realistic test data for unit, integration, performance, and security tests

use std::collections::HashMap;
use std::path::PathBuf;
use serde_yaml::Value;
use tempfile::TempDir;
use rhema::{Rhema, RhemaResult};

use super::{TestEnv, TestFixtures};

/// Enhanced test fixtures with comprehensive data
pub struct EnhancedFixtures {
    pub temp_dir: TempDir,
    pub rhema: Rhema,
    pub repo_path: PathBuf,
    pub test_data: TestData,
}

/// Comprehensive test data structure
pub struct TestData {
    pub scopes: Vec<ScopeData>,
    pub todos: Vec<TodoData>,
    pub insights: Vec<InsightData>,
    pub decisions: Vec<DecisionData>,
    pub patterns: Vec<PatternData>,
    pub dependencies: Vec<DependencyData>,
    pub large_datasets: Vec<LargeDatasetData>,
    pub security_test_cases: Vec<SecurityTestCase>,
    pub performance_test_cases: Vec<PerformanceTestCase>,
}

/// Scope test data
#[derive(Debug, Clone)]
pub struct ScopeData {
    pub name: String,
    pub scope_type: String,
    pub description: String,
    pub version: String,
    pub files: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Todo test data
#[derive(Debug, Clone, serde::Serialize)]
pub struct TodoData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub assignee: String,
    pub created_at: String,
    pub due_date: Option<String>,
    pub tags: Vec<String>,
}

/// Insight test data
#[derive(Debug, Clone, serde::Serialize)]
pub struct InsightData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub confidence: u8,
    pub category: String,
    pub created_at: String,
    pub tags: Vec<String>,
}

/// Decision test data
#[derive(Debug, Clone)]
pub struct DecisionData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub impact: String,
    pub rationale: String,
    pub created_at: String,
    pub stakeholders: Vec<String>,
}

/// Pattern test data
#[derive(Debug, Clone)]
pub struct PatternData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub examples: Vec<String>,
    pub created_at: String,
}

/// Dependency test data
#[derive(Debug, Clone)]
pub struct DependencyData {
    pub id: String,
    pub name: String,
    pub version: String,
    pub type_: String,
    pub status: String,
    pub description: String,
}

/// Large dataset test data
#[derive(Debug, Clone)]
pub struct LargeDatasetData {
    pub name: String,
    pub size: usize,
    pub items: Vec<Value>,
    pub complexity: String,
}

/// Security test case
#[derive(Debug, Clone)]
pub struct SecurityTestCase {
    pub name: String,
    pub category: String,
    pub malicious_input: String,
    pub expected_behavior: String,
    pub severity: String,
}

/// Performance test case
#[derive(Debug, Clone)]
pub struct PerformanceTestCase {
    pub name: String,
    pub operation: String,
    pub dataset_size: usize,
    pub expected_duration: std::time::Duration,
    pub memory_limit: usize,
}

impl EnhancedFixtures {
    /// Create new enhanced fixtures with comprehensive test data
    pub fn new() -> RhemaResult<Self> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path().to_path_buf();
        
        // Initialize git repository
        let _repo = git2::Repository::init(&repo_path)?;
        
        // Create Rhema instance
        let rhema = Rhema::new_from_path(repo_path.clone())?;
        
        // Generate comprehensive test data
        let test_data = Self::generate_test_data();
        
        Ok(Self {
            temp_dir,
            rhema,
            repo_path,
            test_data,
        })
    }

    /// Create enhanced fixtures with specific test scenarios
    pub fn with_scenario(scenario: &str) -> RhemaResult<Self> {
        let mut fixtures = Self::new()?;
        
        match scenario {
            "unit_tests" => fixtures.setup_unit_test_data()?,
            "integration_tests" => fixtures.setup_integration_test_data()?,
            "performance_tests" => fixtures.setup_performance_test_data()?,
            "security_tests" => fixtures.setup_security_test_data()?,
            "stress_tests" => fixtures.setup_stress_test_data()?,
            "load_tests" => fixtures.setup_load_test_data()?,
            "property_tests" => fixtures.setup_property_test_data()?,
            _ => fixtures.setup_basic_test_data()?,
        }
        
        Ok(fixtures)
    }

    /// Generate comprehensive test data
    fn generate_test_data() -> TestData {
        TestData {
            scopes: Self::generate_scope_data(),
            todos: Self::generate_todo_data(),
            insights: Self::generate_insight_data(),
            decisions: Self::generate_decision_data(),
            patterns: Self::generate_pattern_data(),
            dependencies: Self::generate_dependency_data(),
            large_datasets: Self::generate_large_dataset_data(),
            security_test_cases: Self::generate_security_test_cases(),
            performance_test_cases: Self::generate_performance_test_cases(),
        }
    }

    /// Generate scope test data
    fn generate_scope_data() -> Vec<ScopeData> {
        vec![
            ScopeData {
                name: "simple".to_string(),
                scope_type: "service".to_string(),
                description: "Simple test scope".to_string(),
                version: "1.0.0".to_string(),
                files: vec!["simple.yaml".to_string()],
                dependencies: vec![],
            },
            ScopeData {
                name: "complex".to_string(),
                scope_type: "application".to_string(),
                description: "Complex test scope with multiple files".to_string(),
                version: "2.0.0".to_string(),
                files: vec![
                    "complex.yaml".to_string(),
                    "todos.yaml".to_string(),
                    "insights.yaml".to_string(),
                    "decisions.yaml".to_string(),
                ],
                dependencies: vec!["simple".to_string()],
            },
            ScopeData {
                name: "microservice".to_string(),
                scope_type: "microservice".to_string(),
                description: "Microservice scope for testing".to_string(),
                version: "1.5.0".to_string(),
                files: vec![
                    "microservice.yaml".to_string(),
                    "api.yaml".to_string(),
                    "database.yaml".to_string(),
                ],
                dependencies: vec!["complex".to_string()],
            },
        ]
    }

    /// Generate todo test data
    fn generate_todo_data() -> Vec<TodoData> {
        vec![
            TodoData {
                id: "todo-001".to_string(),
                title: "Implement user authentication".to_string(),
                description: "Add JWT-based authentication system".to_string(),
                status: "pending".to_string(),
                priority: "high".to_string(),
                assignee: "alice".to_string(),
                created_at: "2024-01-15T10:00:00Z".to_string(),
                due_date: Some("2024-02-01T17:00:00Z".to_string()),
                tags: vec!["auth".to_string(), "security".to_string()],
            },
            TodoData {
                id: "todo-002".to_string(),
                title: "Fix database connection issue".to_string(),
                description: "Resolve connection pooling problems".to_string(),
                status: "in_progress".to_string(),
                priority: "medium".to_string(),
                assignee: "bob".to_string(),
                created_at: "2024-01-16T14:30:00Z".to_string(),
                due_date: None,
                tags: vec!["database".to_string(), "bug".to_string()],
            },
            TodoData {
                id: "todo-003".to_string(),
                title: "Update API documentation".to_string(),
                description: "Refresh OpenAPI documentation".to_string(),
                status: "completed".to_string(),
                priority: "low".to_string(),
                assignee: "charlie".to_string(),
                created_at: "2024-01-14T09:15:00Z".to_string(),
                due_date: Some("2024-01-20T17:00:00Z".to_string()),
                tags: vec!["documentation".to_string(), "api".to_string()],
            },
        ]
    }

    /// Generate insight test data
    fn generate_insight_data() -> Vec<InsightData> {
        vec![
            InsightData {
                id: "insight-001".to_string(),
                title: "Performance bottleneck identified".to_string(),
                content: "Database queries are taking too long due to missing indexes".to_string(),
                confidence: 9,
                category: "performance".to_string(),
                created_at: "2024-01-15T11:00:00Z".to_string(),
                tags: vec!["performance".to_string(), "database".to_string()],
            },
            InsightData {
                id: "insight-002".to_string(),
                title: "Security vulnerability found".to_string(),
                content: "SQL injection vulnerability in user input validation".to_string(),
                confidence: 8,
                category: "security".to_string(),
                created_at: "2024-01-16T15:45:00Z".to_string(),
                tags: vec!["security".to_string(), "vulnerability".to_string()],
            },
        ]
    }

    /// Generate decision test data
    fn generate_decision_data() -> Vec<DecisionData> {
        vec![
            DecisionData {
                id: "decision-001".to_string(),
                title: "Choose database technology".to_string(),
                description: "Decision on primary database for the application".to_string(),
                status: "approved".to_string(),
                impact: "high".to_string(),
                rationale: "PostgreSQL provides better performance and features".to_string(),
                created_at: "2024-01-10T10:00:00Z".to_string(),
                stakeholders: vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()],
            },
        ]
    }

    /// Generate pattern test data
    fn generate_pattern_data() -> Vec<PatternData> {
        vec![
            PatternData {
                id: "pattern-001".to_string(),
                name: "Repository Pattern".to_string(),
                description: "Data access abstraction pattern".to_string(),
                category: "architecture".to_string(),
                examples: vec![
                    "UserRepository".to_string(),
                    "OrderRepository".to_string(),
                ],
                created_at: "2024-01-12T14:00:00Z".to_string(),
            },
        ]
    }

    /// Generate dependency test data
    fn generate_dependency_data() -> Vec<DependencyData> {
        vec![
            DependencyData {
                id: "dep-001".to_string(),
                name: "serde".to_string(),
                version: "1.0".to_string(),
                type_: "library".to_string(),
                status: "active".to_string(),
                description: "Serialization framework".to_string(),
            },
            DependencyData {
                id: "dep-002".to_string(),
                name: "tokio".to_string(),
                version: "1.0".to_string(),
                type_: "runtime".to_string(),
                status: "active".to_string(),
                description: "Async runtime".to_string(),
            },
        ]
    }

    /// Generate large dataset test data
    fn generate_large_dataset_data() -> Vec<LargeDatasetData> {
        vec![
            LargeDatasetData {
                name: "large_todos".to_string(),
                size: 10000,
                items: Self::generate_large_todo_items(10000),
                complexity: "high".to_string(),
            },
            LargeDatasetData {
                name: "large_insights".to_string(),
                size: 5000,
                items: Self::generate_large_insight_items(5000),
                complexity: "medium".to_string(),
            },
        ]
    }

    /// Generate security test cases
    fn generate_security_test_cases() -> Vec<SecurityTestCase> {
        vec![
            SecurityTestCase {
                name: "path_traversal".to_string(),
                category: "input_validation".to_string(),
                malicious_input: "../../../etc/passwd".to_string(),
                expected_behavior: "rejected".to_string(),
                severity: "high".to_string(),
            },
            SecurityTestCase {
                name: "sql_injection".to_string(),
                category: "query_injection".to_string(),
                malicious_input: "'; DROP TABLE todos; --".to_string(),
                expected_behavior: "sanitized".to_string(),
                severity: "critical".to_string(),
            },
            SecurityTestCase {
                name: "yaml_injection".to_string(),
                category: "yaml_parsing".to_string(),
                malicious_input: "!!python/object/apply:os.system ['rm -rf /']".to_string(),
                expected_behavior: "rejected".to_string(),
                severity: "critical".to_string(),
            },
        ]
    }

    /// Generate performance test cases
    fn generate_performance_test_cases() -> Vec<PerformanceTestCase> {
        vec![
            PerformanceTestCase {
                name: "query_small_dataset".to_string(),
                operation: "query".to_string(),
                dataset_size: 100,
                expected_duration: std::time::Duration::from_millis(50),
                memory_limit: 1024 * 1024, // 1MB
            },
            PerformanceTestCase {
                name: "query_large_dataset".to_string(),
                operation: "query".to_string(),
                dataset_size: 10000,
                expected_duration: std::time::Duration::from_millis(500),
                memory_limit: 10 * 1024 * 1024, // 10MB
            },
        ]
    }

    /// Generate large todo items
    fn generate_large_todo_items(count: usize) -> Vec<Value> {
        (0..count)
            .map(|i| {
                serde_yaml::from_str(&format!(
                    r#"
id: "todo-{:06}"
title: "Large todo item {}"
description: "This is a large todo item for performance testing"
status: "pending"
priority: "medium"
assignee: "user{}"
created_at: "2024-01-15T10:00:00Z"
tags: ["performance", "test"]
"#,
                    i, i, i % 10
                ))
                .unwrap()
            })
            .collect()
    }

    /// Generate large insight items
    fn generate_large_insight_items(count: usize) -> Vec<Value> {
        (0..count)
            .map(|i| {
                serde_yaml::from_str(&format!(
                    r#"
id: "insight-{:06}"
title: "Large insight item {}"
content: "This is a large insight item for performance testing"
confidence: {}
category: "performance"
created_at: "2024-01-15T10:00:00Z"
tags: ["performance", "test"]
"#,
                    i, i, (i % 10) + 1
                ))
                .unwrap()
            })
            .collect()
    }

    /// Setup unit test data
    fn setup_unit_test_data(&mut self) -> RhemaResult<()> {
        // Create basic scope structure
        let rhema_dir = self.repo_path.join(".rhema");
        std::fs::create_dir_all(&rhema_dir)?;
        
        // Create rhema.yaml
        let rhema_yaml = r#"
name: unit-test-scope
scope_type: service
description: Unit test scope
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#;
        std::fs::write(rhema_dir.join("rhema.yaml"), rhema_yaml)?;
        
        // Create simple.yaml
        let simple_yaml = r#"
items:
  - id: "item-001"
    name: "Test Item 1"
    active: true
  - id: "item-002"
    name: "Test Item 2"
    active: false
"#;
        std::fs::write(rhema_dir.join("simple.yaml"), simple_yaml)?;
        
        Ok(())
    }

    /// Setup integration test data
    fn setup_integration_test_data(&mut self) -> RhemaResult<()> {
        self.setup_unit_test_data()?;
        
        let rhema_dir = self.repo_path.join(".rhema");
        
        // Create todos.yaml
        let todos_yaml = serde_yaml::to_string(&self.test_data.todos)?;
        std::fs::write(rhema_dir.join("todos.yaml"), todos_yaml)?;
        
        // Create insights.yaml
        let insights_yaml = serde_yaml::to_string(&self.test_data.insights)?;
        std::fs::write(rhema_dir.join("insights.yaml"), insights_yaml)?;
        
        Ok(())
    }

    /// Setup performance test data
    fn setup_performance_test_data(&mut self) -> RhemaResult<()> {
        self.setup_integration_test_data()?;
        
        let rhema_dir = self.repo_path.join(".rhema");
        
        // Create large datasets
        for dataset in &self.test_data.large_datasets {
            let dataset_yaml = serde_yaml::to_string(&dataset.items)?;
            std::fs::write(rhema_dir.join(&format!("{}.yaml", dataset.name)), dataset_yaml)?;
        }
        
        Ok(())
    }

    /// Setup security test data
    fn setup_security_test_data(&mut self) -> RhemaResult<()> {
        self.setup_unit_test_data()?;
        
        let rhema_dir = self.repo_path.join(".rhema");
        
        // Create security test files
        for test_case in &self.test_data.security_test_cases {
            let test_file = format!("security_test_{}.yaml", test_case.name);
            let test_content = format!(
                r#"
test_case:
  name: "{}"
  category: "{}"
  malicious_input: "{}"
  expected_behavior: "{}"
  severity: "{}"
"#,
                test_case.name, test_case.category, test_case.malicious_input,
                test_case.expected_behavior, test_case.severity
            );
            std::fs::write(rhema_dir.join(test_file), test_content)?;
        }
        
        Ok(())
    }

    /// Setup stress test data
    fn setup_stress_test_data(&mut self) -> RhemaResult<()> {
        self.setup_performance_test_data()?;
        
        // Add additional stress test data
        let rhema_dir = self.repo_path.join(".rhema");
        
        // Create very large dataset for stress testing
        let stress_dataset = Self::generate_large_todo_items(100000);
        let stress_yaml = serde_yaml::to_string(&stress_dataset)?;
        std::fs::write(rhema_dir.join("stress_dataset.yaml"), stress_yaml)?;
        
        Ok(())
    }

    /// Setup load test data
    fn setup_load_test_data(&mut self) -> RhemaResult<()> {
        self.setup_stress_test_data()?;
        
        // Add load test specific data
        let rhema_dir = self.repo_path.join(".rhema");
        
        // Create multiple large files for load testing
        for i in 0..10 {
            let load_dataset = Self::generate_large_todo_items(5000);
            let load_yaml = serde_yaml::to_string(&load_dataset)?;
            std::fs::write(rhema_dir.join(&format!("load_dataset_{}.yaml", i)), load_yaml)?;
        }
        
        Ok(())
    }

    /// Setup property test data
    fn setup_property_test_data(&mut self) -> RhemaResult<()> {
        self.setup_unit_test_data()?;
        
        let rhema_dir = self.repo_path.join(".rhema");
        
        // Create property test data with various edge cases
        let property_test_data = r#"
property_tests:
  - name: "empty_dataset"
    data: []
  - name: "single_item"
    data:
      - id: "single"
        value: "test"
  - name: "nested_structure"
    data:
      - id: "nested"
        nested:
          level1:
            level2: "value"
  - name: "special_characters"
    data:
      - id: "special"
        value: "!@#$%^&*()_+-=[]{}|;':\",./<>?"
"#;
        std::fs::write(rhema_dir.join("property_tests.yaml"), property_test_data)?;
        
        Ok(())
    }

    /// Setup basic test data
    fn setup_basic_test_data(&mut self) -> RhemaResult<()> {
        self.setup_unit_test_data()
    }

    /// Get test data by type
    pub fn get_todos(&self) -> &Vec<TodoData> {
        &self.test_data.todos
    }

    /// Get test data by type
    pub fn get_insights(&self) -> &Vec<InsightData> {
        &self.test_data.insights
    }

    /// Get test data by type
    pub fn get_decisions(&self) -> &Vec<DecisionData> {
        &self.test_data.decisions
    }

    /// Get test data by type
    pub fn get_patterns(&self) -> &Vec<PatternData> {
        &self.test_data.patterns
    }

    /// Get test data by type
    pub fn get_dependencies(&self) -> &Vec<DependencyData> {
        &self.test_data.dependencies
    }

    /// Get security test cases
    pub fn get_security_test_cases(&self) -> &Vec<SecurityTestCase> {
        &self.test_data.security_test_cases
    }

    /// Get performance test cases
    pub fn get_performance_test_cases(&self) -> &Vec<PerformanceTestCase> {
        &self.test_data.performance_test_cases
    }

    /// Get large dataset by name
    pub fn get_large_dataset(&self, name: &str) -> Option<&LargeDatasetData> {
        self.test_data.large_datasets.iter().find(|d| d.name == name)
    }
} 