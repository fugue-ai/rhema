//! End-to-end workflow tests for Rhema
//! 
//! These tests cover complete user workflows from project initialization
//! through advanced features, ensuring the entire system works together.

use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;
use rhema::Rhema;
use rhema::config::Config;
use rhema::types::{Todo, Insight, Pattern, Decision};

/// Test fixture for creating temporary Rhema projects
struct TestProject {
    temp_dir: TempDir,
    rhema: Rhema,
    project_path: PathBuf,
}

impl TestProject {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let project_path = temp_dir.path().join("test-project");
        fs::create_dir(&project_path)?;
        
        // Initialize Rhema in the project
        let rhema = Rhema::new(&project_path)?;
        rhema.init()?;
        
        Ok(TestProject {
            temp_dir,
            rhema,
            project_path,
        })
    }
    
    fn create_source_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a basic Rust project structure
        let src_dir = self.project_path.join("src");
        fs::create_dir(&src_dir)?;
        
        // Create main.rs
        fs::write(
            src_dir.join("main.rs"),
            r#"
fn main() {
    println!("Hello, Rhema!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main() {
        assert_eq!(2 + 2, 4);
    }
}
"#,
        )?;
        
        // Create lib.rs
        fs::write(
            src_dir.join("lib.rs"),
            r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
"#,
        )?;
        
        // Create Cargo.toml
        fs::write(
            self.project_path.join("Cargo.toml"),
            r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
"#,
        )?;
        
        Ok(())
    }
}

#[tokio::test]
async fn test_complete_project_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    // Phase 1: Project Initialization
    let project = TestProject::new()?;
    project.create_source_files()?;
    
    // Verify project structure
    assert!(project.project_path.join(".rhema").exists());
    assert!(project.project_path.join("src").exists());
    assert!(project.project_path.join("Cargo.toml").exists());
    
    // Phase 2: Basic Configuration
    let config = Config::load(&project.project_path)?;
    assert_eq!(config.project.name, "test-project");
    
    // Phase 3: Todo Management Workflow
    let todo = Todo {
        id: "TODO-001".to_string(),
        title: "Implement core functionality".to_string(),
        description: Some("Add the main business logic".to_string()),
        status: "pending".to_string(),
        priority: "high".to_string(),
        assignee: Some("developer".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        due_date: None,
        tags: vec!["core".to_string(), "feature".to_string()],
        related: vec![],
    };
    
    project.rhema.todo_add(&todo).await?;
    
    // Verify todo was added
    let todos = project.rhema.todo_list().await?;
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "Implement core functionality");
    
    // Phase 4: Insight Recording Workflow
    let insight = Insight {
        id: "INSIGHT-001".to_string(),
        title: "Performance consideration".to_string(),
        content: "Consider using async/await for I/O operations".to_string(),
        confidence: 8,
        category: "performance".to_string(),
        tags: vec!["async".to_string(), "io".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        related: vec![],
    };
    
    project.rhema.insight_add(&insight).await?;
    
    // Verify insight was added
    let insights = project.rhema.insight_list().await?;
    assert_eq!(insights.len(), 1);
    assert_eq!(insights[0].title, "Performance consideration");
    
    // Phase 5: Pattern Recognition Workflow
    let pattern = Pattern {
        id: "PATTERN-001".to_string(),
        name: "Error handling pattern".to_string(),
        description: "Consistent error handling across modules".to_string(),
        pattern_type: "code".to_string(),
        examples: vec!["src/error.rs".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        tags: vec!["error-handling".to_string()],
        related: vec![],
    };
    
    project.rhema.pattern_add(&pattern).await?;
    
    // Verify pattern was added
    let patterns = project.rhema.pattern_list().await?;
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].name, "Error handling pattern");
    
    // Phase 6: Decision Tracking Workflow
    let decision = Decision {
        id: "DECISION-001".to_string(),
        title: "Use async/await for I/O".to_string(),
        description: "Decision to use async/await for better performance".to_string(),
        status: "approved".to_string(),
        rationale: "Improves performance and resource utilization".to_string(),
        alternatives: vec!["Synchronous I/O".to_string(), "Thread pools".to_string()],
        impact: "Better scalability".to_string(),
        date: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        tags: vec!["architecture".to_string()],
        related: vec![],
    };
    
    project.rhema.decision_add(&decision).await?;
    
    // Verify decision was added
    let decisions = project.rhema.decision_list().await?;
    assert_eq!(decisions.len(), 1);
    assert_eq!(decisions[0].title, "Use async/await for I/O");
    
    // Phase 7: Search and Query Workflow
    let search_results = project.rhema.search("performance").await?;
    assert!(!search_results.is_empty());
    
    // Phase 8: Validation Workflow
    let validation_result = project.rhema.validate().await?;
    assert!(validation_result.is_valid());
    
    // Phase 9: Health Check Workflow
    let health_result = project.rhema.health().await?;
    assert!(health_result.is_healthy());
    
    Ok(())
}

#[tokio::test]
async fn test_collaboration_workflow() -> Result<(), Box<dyn std::error::Error>> {
    // Create multiple projects to simulate collaboration
    let project1 = TestProject::new()?;
    let project2 = TestProject::new()?;
    
    // Phase 1: Individual Work
    project1.create_source_files()?;
    project2.create_source_files()?;
    
    // Add different todos to each project
    let todo1 = Todo {
        id: "TODO-001".to_string(),
        title: "Implement user authentication".to_string(),
        description: Some("Add user login and registration".to_string()),
        status: "in-progress".to_string(),
        priority: "high".to_string(),
        assignee: Some("alice".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        due_date: None,
        tags: vec!["auth".to_string()],
        related: vec![],
    };
    
    let todo2 = Todo {
        id: "TODO-002".to_string(),
        title: "Design database schema".to_string(),
        description: Some("Create database tables and relationships".to_string()),
        status: "pending".to_string(),
        priority: "medium".to_string(),
        assignee: Some("bob".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        due_date: None,
        tags: vec!["database".to_string()],
        related: vec![],
    };
    
    project1.rhema.todo_add(&todo1).await?;
    project2.rhema.todo_add(&todo2).await?;
    
    // Phase 2: Knowledge Sharing
    let shared_insight = Insight {
        id: "INSIGHT-001".to_string(),
        title: "Shared architecture insight".to_string(),
        content: "Use microservices for better scalability".to_string(),
        confidence: 9,
        category: "architecture".to_string(),
        tags: vec!["microservices".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        related: vec![],
    };
    
    project1.rhema.insight_add(&shared_insight).await?;
    
    // Phase 3: Conflict Resolution
    // Simulate a conflict by updating the same todo in both projects
    let mut conflicting_todo = todo1.clone();
    conflicting_todo.status = "completed".to_string();
    conflicting_todo.updated_at = chrono::Utc::now();
    
    project1.rhema.todo_update(&conflicting_todo).await?;
    
    // Phase 4: Synchronization
    // In a real scenario, this would involve Git operations
    let todos1 = project1.rhema.todo_list().await?;
    let todos2 = project2.rhema.todo_list().await?;
    
    assert_eq!(todos1.len(), 1);
    assert_eq!(todos2.len(), 1);
    
    // Phase 5: Cross-Project Queries
    // This would require a higher-level coordination system
    let search_results = project1.rhema.search("auth").await?;
    assert!(!search_results.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_large_scale_management() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProject::new()?;
    project.create_source_files()?;
    
    // Phase 1: Bulk Data Creation
    let mut todos = Vec::new();
    let mut insights = Vec::new();
    
    // Create 100 todos
    for i in 0..100 {
        let todo = Todo {
            id: format!("TODO-{:03}", i),
            title: format!("Task {}", i),
            description: Some(format!("Description for task {}", i)),
            status: if i % 3 == 0 { "completed".to_string() } else { "pending".to_string() },
            priority: if i % 4 == 0 { "high".to_string() } else { "medium".to_string() },
            assignee: Some(format!("developer-{}", i % 5)),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            due_date: None,
            tags: vec![format!("tag-{}", i % 10)],
            related: vec![],
        };
        todos.push(todo);
    }
    
    // Create 50 insights
    for i in 0..50 {
        let insight = Insight {
            id: format!("INSIGHT-{:03}", i),
            title: format!("Insight {}", i),
            content: format!("Content for insight {}", i),
            confidence: (i % 10) + 1,
            category: format!("category-{}", i % 5),
            tags: vec![format!("tag-{}", i % 10)],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            related: vec![],
        };
        insights.push(insight);
    }
    
    // Phase 2: Bulk Operations
    for todo in &todos {
        project.rhema.todo_add(todo).await?;
    }
    
    for insight in &insights {
        project.rhema.insight_add(insight).await?;
    }
    
    // Phase 3: Performance Testing
    let start = std::time::Instant::now();
    
    // Test bulk listing
    let all_todos = project.rhema.todo_list().await?;
    let all_insights = project.rhema.insight_list().await?;
    
    let duration = start.elapsed();
    
    // Verify all data was added
    assert_eq!(all_todos.len(), 100);
    assert_eq!(all_insights.len(), 50);
    
    // Performance should be reasonable (less than 1 second for 150 items)
    assert!(duration.as_millis() < 1000);
    
    // Phase 4: Filtering and Search Performance
    let start = std::time::Instant::now();
    
    let high_priority_todos = project.rhema.todo_list_filtered(Some("high"), None, None).await?;
    let performance_insights = project.rhema.insight_search("performance").await?;
    
    let duration = start.elapsed();
    
    // Verify filtering works
    assert!(!high_priority_todos.is_empty());
    
    // Performance should be reasonable
    assert!(duration.as_millis() < 500);
    
    // Phase 5: Batch Operations
    let batch_todos: Vec<Todo> = todos.iter().take(10).map(|t| {
        let mut todo = t.clone();
        todo.status = "completed".to_string();
        todo.updated_at = chrono::Utc::now();
        todo
    }).collect();
    
    for todo in &batch_todos {
        project.rhema.todo_update(todo).await?;
    }
    
    // Verify batch update worked
    let completed_todos = project.rhema.todo_list_filtered(Some("completed"), None, None).await?;
    assert_eq!(completed_todos.len(), 10);
    
    Ok(())
}

#[tokio::test]
async fn test_migration_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProject::new()?;
    project.create_source_files()?;
    
    // Phase 1: Create Legacy Data
    let legacy_todo = Todo {
        id: "LEGACY-001".to_string(),
        title: "Legacy task".to_string(),
        description: Some("This is a legacy task".to_string()),
        status: "pending".to_string(),
        priority: "medium".to_string(),
        assignee: Some("legacy-user".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        due_date: None,
        tags: vec!["legacy".to_string()],
        related: vec![],
    };
    
    project.rhema.todo_add(&legacy_todo).await?;
    
    // Phase 2: Simulate Configuration Migration
    let old_config = r#"
project:
  name: "old-project"
  version: "0.1.0"
"#;
    
    let new_config = r#"
project:
  name: "new-project"
  version: "1.0.0"
  auto_backup: true
  backup_interval: "1h"
"#;
    
    // Write old config
    fs::write(project.project_path.join(".rhema").join("config.yaml"), old_config)?;
    
    // Simulate migration
    fs::write(project.project_path.join(".rhema").join("config.yaml"), new_config)?;
    
    // Phase 3: Data Migration
    // In a real scenario, this would involve schema migrations
    let migrated_todos = project.rhema.todo_list().await?;
    assert_eq!(migrated_todos.len(), 1);
    
    // Phase 4: Validation After Migration
    let validation_result = project.rhema.validate().await?;
    assert!(validation_result.is_valid());
    
    // Phase 5: Health Check After Migration
    let health_result = project.rhema.health().await?;
    assert!(health_result.is_healthy());
    
    Ok(())
}

#[tokio::test]
async fn test_error_recovery_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProject::new()?;
    project.create_source_files()?;
    
    // Phase 1: Corrupt Data Scenario
    let todo = Todo {
        id: "TODO-001".to_string(),
        title: "Test task".to_string(),
        description: Some("Test description".to_string()),
        status: "pending".to_string(),
        priority: "high".to_string(),
        assignee: Some("developer".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        due_date: None,
        tags: vec!["test".to_string()],
        related: vec![],
    };
    
    project.rhema.todo_add(&todo).await?;
    
    // Simulate corruption by writing invalid YAML
    let corrupt_data = r#"
todos:
  - id: "TODO-001"
    title: "Test task"
    status: "pending"
    # Missing required fields - this should cause validation errors
"#;
    
    fs::write(project.project_path.join(".rhema").join("todos.yaml"), corrupt_data)?;
    
    // Phase 2: Recovery Attempt
    // The system should handle corruption gracefully
    let recovery_result = project.rhema.validate().await?;
    
    // In a real implementation, this might trigger automatic recovery
    // For now, we just verify the system doesn't crash
    
    // Phase 3: Manual Recovery
    // Re-add the valid todo
    project.rhema.todo_add(&todo).await?;
    
    // Verify recovery
    let todos = project.rhema.todo_list().await?;
    assert_eq!(todos.len(), 1);
    
    Ok(())
}

#[tokio::test]
async fn test_performance_optimization_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let project = TestProject::new()?;
    project.create_source_files()?;
    
    // Phase 1: Baseline Performance
    let start = std::time::Instant::now();
    
    // Add 1000 items
    for i in 0..1000 {
        let todo = Todo {
            id: format!("TODO-{:04}", i),
            title: format!("Task {}", i),
            description: Some(format!("Description {}", i)),
            status: "pending".to_string(),
            priority: "medium".to_string(),
            assignee: Some("developer".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            due_date: None,
            tags: vec![format!("tag-{}", i % 20)],
            related: vec![],
        };
        
        project.rhema.todo_add(&todo).await?;
    }
    
    let baseline_duration = start.elapsed();
    
    // Phase 2: Search Performance
    let start = std::time::Instant::now();
    
    let search_results = project.rhema.search("task").await?;
    
    let search_duration = start.elapsed();
    
    // Verify search performance
    assert!(!search_results.is_empty());
    assert!(search_duration.as_millis() < 1000); // Should be under 1 second
    
    // Phase 3: Filtering Performance
    let start = std::time::Instant::now();
    
    let filtered_todos = project.rhema.todo_list_filtered(Some("pending"), None, None).await?;
    
    let filter_duration = start.elapsed();
    
    // Verify filtering performance
    assert_eq!(filtered_todos.len(), 1000);
    assert!(filter_duration.as_millis() < 500); // Should be under 500ms
    
    // Phase 4: Memory Usage Check
    // In a real implementation, we would check memory usage
    // For now, we just verify the operations complete successfully
    
    Ok(())
} 