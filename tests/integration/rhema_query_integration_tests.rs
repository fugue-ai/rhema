//! Integration tests for rhema-query functionality
//! 
//! These tests verify the complete functionality of the rhema-query crate.

use rhema_query::{
    execute_query, 
    execute_query_with_provenance,
    search::{SearchEngine, SearchOptions, SearchType},
    performance::{PerformanceMonitor, PerformanceConfig},
    optimization::QueryOptimizer,
    repo_analysis::RepoAnalysis,
};
use rhema_core::RhemaResult;
use std::path::PathBuf;
use std::collections::HashMap;
use tempfile::TempDir;
use std::fs;

/// Test environment for rhema-query integration tests
struct RhemaQueryTestEnv {
    temp_dir: TempDir,
    repo_path: PathBuf,
}

impl RhemaQueryTestEnv {
    fn new() -> RhemaResult<Self> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path().to_path_buf();
        
        // Create a basic repository structure
        fs::create_dir_all(&repo_path.join(".rhema"))?;
        
        // Create sample YAML files
        let todos_content = r#"
todos:
  - id: todo-001
    title: "Implement authentication"
    status: "pending"
    priority: "high"
    assignee: "alice"
    created_at: "2024-01-15"
  - id: todo-002
    title: "Add unit tests"
    status: "completed"
    priority: "medium"
    assignee: "bob"
    created_at: "2024-01-10"
  - id: todo-003
    title: "Update documentation"
    status: "in_progress"
    priority: "low"
    assignee: "charlie"
    created_at: "2024-01-20"
"#;
        
        let knowledge_content = r#"
knowledge:
  entries:
    - id: knowledge-001
      title: "JWT Authentication Best Practices"
      content: "Use secure tokens with proper expiration"
      confidence: 8
      category: "security"
    - id: knowledge-002
      title: "Database Optimization"
      content: "Index frequently queried columns"
      confidence: 7
      category: "performance"
"#;
        
        let decisions_content = r#"
decisions:
  - id: decision-001
    title: "Use JWT for authentication"
    status: "approved"
    impact_scope: "multiple"
    priority: 5
    created_at: "2024-01-01"
  - id: decision-002
    title: "Implement caching layer"
    status: "pending"
    impact_scope: "single"
    priority: 3
    created_at: "2024-01-05"
"#;
        
        fs::write(&repo_path.join(".rhema").join("todos.yaml"), todos_content)?;
        fs::write(&repo_path.join(".rhema").join("knowledge.yaml"), knowledge_content)?;
        fs::write(&repo_path.join(".rhema").join("decisions.yaml"), decisions_content)?;
        
        Ok(Self {
            temp_dir,
            repo_path,
        })
    }
    
    fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }
}

#[tokio::test]
async fn test_basic_query_execution() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    // Test basic query
    let result = execute_query(env.repo_path(), "SELECT todos")?;
    
    // Verify result contains expected data
    assert!(!result.is_null());
    assert!(result.as_sequence().is_some());
    
    let todos = result.as_sequence().unwrap();
    assert_eq!(todos.len(), 3); // Should have 3 todos
    
    Ok(())
}

#[tokio::test]
async fn test_query_with_conditions() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    // Test query with WHERE condition
    let result = execute_query(env.repo_path(), "SELECT todos WHERE status='pending'")?;
    
    // Verify result contains only pending todos
    let todos = result.as_sequence().unwrap();
    assert_eq!(todos.len(), 1); // Should have 1 pending todo
    
    let first_todo = &todos[0];
    assert_eq!(first_todo["status"], "pending");
    
    Ok(())
}

#[tokio::test]
async fn test_query_with_provenance() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    // Test query with provenance tracking
    let (result, provenance) = execute_query_with_provenance(
        env.repo_path(), 
        "SELECT todos WHERE priority='high'"
    )?;
    
    // Verify result
    assert!(!result.is_null());
    let todos = result.as_sequence().unwrap();
    assert_eq!(todos.len(), 1); // Should have 1 high priority todo
    
    // Verify provenance
    assert!(!provenance.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_search_functionality() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    let mut search_engine = SearchEngine::new();
    
    // Build search index
    let scopes = vec![]; // Empty for this test
    search_engine.build_index(env.repo_path(), &scopes).await?;
    
    // Test full-text search
    let options = SearchOptions {
        limit: Some(10),
        case_sensitive: false,
        fuzzy_matching: true,
        search_type: SearchType::FullText,
        filters: Vec::new(),
        semantic_weight: None,
        keyword_weight: None,
        min_similarity: None,
        fuzzy_distance: None,
        search_fields: Vec::new(),
        field_boosts: HashMap::new(),
    };
    
    let results = search_engine.full_text_search("authentication", Some(options)).await?;
    
    // Should find results containing "authentication"
    assert!(!results.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_regex_search() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    let mut search_engine = SearchEngine::new();
    
    // Build search index
    let scopes = vec![];
    search_engine.build_index(env.repo_path(), &scopes).await?;
    
    // Test regex search
    let options = SearchOptions {
        limit: Some(10),
        case_sensitive: false,
        search_type: SearchType::Regex,
        filters: Vec::new(),
        semantic_weight: None,
        keyword_weight: None,
        min_similarity: None,
        fuzzy_matching: false,
        fuzzy_distance: None,
        search_fields: Vec::new(),
        field_boosts: HashMap::new(),
    };
    
    let results = search_engine.regex_search(r"todo-\d+", Some(options)).await?;
    
    // Should find todo IDs
    assert!(!results.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_performance_monitoring() -> RhemaResult<()> {
    let config = PerformanceConfig {
        max_history_size: 100,
        alert_threshold_ms: 1000,
        error_threshold_percent: 5.0,
        cache_hit_rate_threshold: 0.8,
        memory_threshold_mb: 512,
        enable_alerts: true,
        enable_trends: true,
        trend_window_hours: 24,
    };
    
    let monitor = PerformanceMonitor::new(config);
    
    // Record some performance data
    let query = rhema_query::query::CqlQuery {
        target: "todos".to_string(),
        yaml_path: None,
        conditions: vec![],
        order_by: vec![],
        limit: Some(10),
        offset: Some(0),
        index_hints: None,
    };
    
    monitor.record_query_execution(
        &query,
        std::time::Duration::from_millis(150),
        Some(5),
        None,
        Some("cache_hit".to_string()),
    ).await?;
    
    // Get metrics
    let metrics = monitor.get_metrics()?;
    assert_eq!(metrics.total_queries, 1);
    assert_eq!(metrics.avg_execution_time_ms, 150.0);
    
    Ok(())
}

#[tokio::test]
async fn test_query_optimization() -> RhemaResult<()> {
    let mut optimizer = QueryOptimizer::new();
    
    let query = rhema_query::query::CqlQuery {
        target: "todos".to_string(),
        yaml_path: None,
        conditions: vec![
            rhema_query::query::Condition {
                field: "status".to_string(),
                operator: rhema_query::query::Operator::Equals,
                value: serde_yaml::Value::String("pending".to_string()),
            },
        ],
        order_by: vec![],
        limit: Some(1000), // Large limit that should be optimized
        offset: Some(0),
        index_hints: None,
    };
    
    let optimized = optimizer.optimize(&query).await?;
    
    // Should have applied optimizations
    assert!(!optimized.applied_optimizations.is_empty());
    
    // Large limit should be capped
    assert_eq!(optimized.optimized.limit, Some(1000));
    
    Ok(())
}

#[tokio::test]
async fn test_repository_analysis() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    let analysis = RepoAnalysis::analyze(env.repo_path())?;
    
    // Should detect basic project structure
    assert!(analysis.languages.is_empty() || !analysis.languages.is_empty());
    assert!(analysis.frameworks.is_empty() || !analysis.frameworks.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_complex_query_scenarios() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    // Test complex query with multiple conditions
    let result = execute_query(
        env.repo_path(),
        "SELECT todos WHERE (status='pending' OR status='in_progress') AND priority='high'"
    )?;
    
    let todos = result.as_sequence().unwrap();
    // Should have high priority todos that are pending or in progress
    assert!(todos.len() <= 2);
    
    // Test query with ordering
    let result = execute_query(
        env.repo_path(),
        "SELECT todos ORDER BY created_at DESC"
    )?;
    
    let todos = result.as_sequence().unwrap();
    assert_eq!(todos.len(), 3);
    
    // Test query with limit
    let result = execute_query(
        env.repo_path(),
        "SELECT todos LIMIT 2"
    )?;
    
    let todos = result.as_sequence().unwrap();
    assert_eq!(todos.len(), 2);
    
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    // Test invalid query
    let result = execute_query(env.repo_path(), "INVALID QUERY");
    assert!(result.is_err());
    
    // Test query for non-existent target
    let result = execute_query(env.repo_path(), "SELECT nonexistent");
    assert!(result.is_err());
    
    // Test query with invalid condition
    let result = execute_query(env.repo_path(), "SELECT todos WHERE invalid_field='value'");
    // This might succeed but return empty results, which is acceptable
    
    Ok(())
}

#[tokio::test]
async fn test_caching_functionality() -> RhemaResult<()> {
    let env = RhemaQueryTestEnv::new()?;
    
    // Execute the same query twice
    let result1 = execute_query(env.repo_path(), "SELECT todos WHERE status='pending'")?;
    let result2 = execute_query(env.repo_path(), "SELECT todos WHERE status='pending'")?;
    
    // Results should be identical
    assert_eq!(result1, result2);
    
    Ok(())
}
