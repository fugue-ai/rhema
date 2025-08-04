//! Unit tests for Rhema crate functionality
//! 
//! This example demonstrates comprehensive unit testing for the Rhema crate,
//! including initialization, query execution, performance monitoring, and security features.

use rhema::{Rhema, RhemaResult, ApiInput, RateLimitConfig, PerformanceMonitor, PerformanceMetrics, SecurityManager, SecurityConfig, InputSanitizer, AccessControl, AuditLogger, AuditLogEntry, ApiDocumentation, PerformanceGuard, ResourceManager, PerformanceOptimizer};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;
use std::collections::HashMap;
use std::sync::Arc;
use chrono;

/// Test fixture for creating temporary repositories
struct TestFixture {
    temp_dir: TempDir,
    repo_path: PathBuf,
}

impl TestFixture {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path().join("test_repo");
        fs::create_dir_all(&repo_path)?;
        
        // Initialize git repository
        std::process::Command::new("git")
            .args(&["init"])
            .current_dir(&repo_path)
            .output()?;

        Ok(Self {
            temp_dir,
            repo_path,
        })
    }

    fn create_scope_file(&self, scope_name: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let scope_path = self.repo_path.join(scope_name);
        fs::create_dir_all(&scope_path)?;
        
        let rhema_file = scope_path.join("rhema.yaml");
        fs::write(rhema_file, content)?;
        
        Ok(())
    }

    fn create_data_file(&self, scope_name: &str, filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.repo_path.join(scope_name).join(filename);
        fs::write(file_path, content)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("🧪 Running Rhema Unit Tests...\n");

    // Test 1: Rhema initialization
    test_rhema_initialization().await?;
    println!("✅ Rhema initialization tests passed");

    // Test 2: API input validation
    test_api_input_validation().await?;
    println!("✅ API input validation tests passed");

    // Test 3: Query execution with error recovery
    test_query_with_error_recovery().await?;
    println!("✅ Query execution tests passed");

    // Test 4: Scope discovery and management
    test_scope_operations().await?;
    println!("✅ Scope operations tests passed");

    // Test 5: Performance monitoring
    test_performance_monitoring().await?;
    println!("✅ Performance monitoring tests passed");

    // Test 6: Security features
    test_security_features().await?;
    println!("✅ Security feature tests passed");

    // Test 7: Error handling
    test_error_handling().await?;
    println!("✅ Error handling tests passed");

    // Test 8: Concurrent operations
    test_concurrent_operations().await?;
    println!("✅ Concurrent operations tests passed");

    println!("\n🎉 All unit tests passed successfully!");
    Ok(())
}

async fn test_rhema_initialization() -> RhemaResult<()> {
    let fixture = TestFixture::new()?;
    
    // Test basic initialization
    std::env::set_current_dir(&fixture.repo_path)?;
    let rhema = Rhema::new()?;
    assert_eq!(rhema.repo_root(), &fixture.repo_path);
    assert_eq!(rhema.api_version(), "1.0.0");

    // Test initialization from path
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;
    assert_eq!(rhema.repo_root(), &fixture.repo_path);

    // Test initialization with rate limiting
    let rate_limit_config = RateLimitConfig {
        requests_per_minute: 500,
        burst_size: 50,
    };
    let rhema = Rhema::new_with_rate_limit(fixture.repo_path.clone(), rate_limit_config)?;
    assert_eq!(rhema.repo_root(), &fixture.repo_path);

    Ok(())
}

async fn test_api_input_validation() -> RhemaResult<()> {
    let mut input = ApiInput {
        query: Some("SELECT * FROM todos".to_string()),
        scope_name: Some("test_scope".to_string()),
        file_path: None,
        operation: "query".to_string(),
        parameters: HashMap::new(),
    };

    // Valid input
    assert!(input.validate().is_ok());

    // Invalid input - empty operation
    input.operation = "".to_string();
    assert!(input.validate().is_err());

    // Invalid input - empty query
    input.operation = "query".to_string();
    input.query = Some("".to_string());
    assert!(input.validate().is_err());

    // Invalid input - empty scope name
    input.query = Some("SELECT * FROM todos".to_string());
    input.scope_name = Some("".to_string());
    assert!(input.validate().is_err());

    Ok(())
}

async fn test_query_with_error_recovery() -> RhemaResult<()> {
    let fixture = TestFixture::new()?;
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;

    // Create test data
    fixture.create_scope_file("test_scope", "name: test_scope\ndescription: Test scope")?;
    fixture.create_data_file("test_scope", "todos.yaml", "todos:\n  - title: Test todo\n    priority: high")?;

    // Test successful query
    let result = rhema.query_with_error_recovery("SELECT * FROM todos").await;
    assert!(result.is_ok());

    // Test query caching
    let result2 = rhema.query_with_error_recovery("SELECT * FROM todos").await;
    assert!(result2.is_ok());
    assert_eq!(result?, result2?);

    Ok(())
}

async fn test_scope_operations() -> RhemaResult<()> {
    let fixture = TestFixture::new()?;
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;

    // Create test scopes
    fixture.create_scope_file("scope1", "name: scope1\ndescription: First scope")?;
    fixture.create_scope_file("scope2", "name: scope2\ndescription: Second scope")?;

    // Test scope discovery
    let scopes = rhema.discover_scopes_optimized().await?;
    assert!(!scopes.is_empty());

    // Test caching
    let scopes2 = rhema.discover_scopes_optimized().await?;
    assert_eq!(scopes.len(), scopes2.len());

    // Test getting specific scope
    let scope = rhema.get_scope_optimized("scope1").await?;
    assert_eq!(scope.definition.name, "scope1");

    // Test scope validation
    let result = rhema.validate_scope(&scope).await;
    assert!(result.is_ok());

    // Test knowledge loading
    fixture.create_data_file("scope1", "knowledge.yaml", "entries:\n  - title: Test knowledge\n    content: Test content")?;
    let knowledge = rhema.load_knowledge_async("scope1").await?;
    assert!(!knowledge.entries.is_empty());

    Ok(())
}

async fn test_performance_monitoring() -> RhemaResult<()> {
    let monitor = PerformanceMonitor::new();

    // Record some metrics
    let metrics = PerformanceMetrics {
        operation_name: "test_operation".to_string(),
        execution_time_ms: 100,
        memory_usage_bytes: Some(1024),
        cpu_usage_percent: Some(5.0),
        files_processed: Some(10),
        cache_hit_rate: Some(0.8),
        error_count: 0,
        success_count: 1,
        custom_metrics: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    monitor.record_metrics(metrics).await;

    // Get metrics
    let recorded_metrics = monitor.get_metrics("test_operation").await;
    assert_eq!(recorded_metrics.len(), 1);
    assert_eq!(recorded_metrics[0].execution_time_ms, 100);

    // Get aggregated metrics
    let aggregated = monitor.get_aggregated_metrics("test_operation").await;
    assert!(aggregated.is_some());
    let aggregated = aggregated.unwrap();
    assert_eq!(aggregated.total_executions, 1);
    assert_eq!(aggregated.avg_execution_time_ms, 100.0);

    // Test performance guard
    let monitor = Arc::new(PerformanceMonitor::new());
    {
        let _guard = PerformanceGuard::new("test_operation".to_string(), monitor.clone());
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(10));
    } // Guard is dropped here

    // Check that metrics were recorded
    let metrics = monitor.get_metrics("test_operation").await;
    assert_eq!(metrics.len(), 1);
    assert!(metrics[0].execution_time_ms > 0);

    // Test resource manager
    let resource_manager = ResourceManager::new();
    let status = resource_manager.check_resource_usage();
    assert!(status.within_limits);

    // Test performance optimizer
    let optimized_query = PerformanceOptimizer::optimize_query("SELECT * FROM todos WHERE priority = 'high'");
    assert!(!optimized_query.is_empty());

    let files = vec!["file1.yaml".to_string(), "file2.yaml".to_string()];
    let optimized_files = PerformanceOptimizer::optimize_file_operations(&files);
    assert_eq!(optimized_files.len(), 2);

    Ok(())
}

async fn test_security_features() -> RhemaResult<()> {
    let config = SecurityConfig::default();
    let security_manager = SecurityManager::new(config);

    // Test input validation
    let result = security_manager.validate_input("user1", "test_operation", "safe input").await;
    assert!(result.is_ok());

    // Test malicious input
    let result = security_manager.validate_input("user1", "test_operation", "<script>alert('xss')</script>").await;
    assert!(result.is_err());

    // Test file path validation
    let result = security_manager.validate_file_access("user1", "safe_file.yaml").await;
    assert!(result.is_ok());

    // Test malicious file path
    let result = security_manager.validate_file_access("user1", "../../../etc/passwd").await;
    assert!(result.is_err());

    // Test input sanitizer
    let sanitizer = InputSanitizer::new(config);

    // Test safe input
    let result = sanitizer.sanitize_string("safe input");
    assert!(result.is_ok());

    // Test SQL injection
    let result = sanitizer.sanitize_string("SELECT * FROM users");
    assert!(result.is_err());

    // Test XSS
    let result = sanitizer.sanitize_string("<script>alert('xss')</script>");
    assert!(result.is_err());

    // Test access control
    let access_control = AccessControl::new(config);

    // Grant permission
    access_control.grant_permission("user1", "read").await?;

    // Check permission
    let has_permission = access_control.check_permission("user1", "read").await?;
    assert!(has_permission);

    // Check non-existent permission
    let has_permission = access_control.check_permission("user1", "write").await?;
    assert!(!has_permission);

    // Revoke permission
    access_control.revoke_permission("user1", "read").await?;
    let has_permission = access_control.check_permission("user1", "read").await?;
    assert!(!has_permission);

    // Test audit logger
    let audit_logger = AuditLogger::new(config);

    // Log an event
    let entry = AuditLogEntry {
        timestamp: chrono::Utc::now(),
        user_id: "user1".to_string(),
        operation: "read".to_string(),
        resource: "file.yaml".to_string(),
        success: true,
        error_message: None,
        ip_address: Some("127.0.0.1".to_string()),
        user_agent: Some("test-agent".to_string()),
        metadata: HashMap::new(),
    };

    audit_logger.log_event(entry).await?;

    // Get entries
    let entries = audit_logger.get_entries(Some("user1"), None).await;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].user_id, "user1");
    assert_eq!(entries[0].operation, "read");

    // Test API documentation
    let docs = ApiDocumentation::generate_rhema_api_docs();
    
    assert_eq!(docs.version, "1.0.0");
    assert_eq!(docs.title, "Rhema API");
    assert!(!docs.endpoints.is_empty());
    assert!(!docs.error_codes.is_empty());

    // Test markdown generation
    let markdown = docs.to_markdown();
    assert!(markdown.contains("# Rhema API"));
    assert!(markdown.contains("## Endpoints"));
    assert!(markdown.contains("## Error Codes"));

    Ok(())
}

async fn test_error_handling() -> RhemaResult<()> {
    let fixture = TestFixture::new()?;
    
    // Test with non-existent repository
    let result = Rhema::new_from_path(PathBuf::from("/non/existent/path"));
    assert!(result.is_err());

    // Test with invalid query
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;
    let result = rhema.query("");
    assert!(result.is_err());

    Ok(())
}

async fn test_concurrent_operations() -> RhemaResult<()> {
    let fixture = TestFixture::new()?;
    let rhema = Arc::new(Rhema::new_from_path(fixture.repo_path.clone())?);

    // Create test data
    fixture.create_scope_file("test_scope", "name: test_scope\ndescription: Test scope")?;
    fixture.create_data_file("test_scope", "todos.yaml", "todos:\n  - title: Test todo\n    priority: high")?;

    // Run concurrent operations
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let rhema = rhema.clone();
            tokio::spawn(async move {
                rhema.query(&format!("SELECT * FROM todos LIMIT {}", i + 1)).await
            })
        })
        .collect();

    // Wait for all operations to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    // Test stress test with multiple scopes
    for i in 0..5 {
        fixture.create_scope_file(&format!("scope{}", i), &format!("name: scope{}\ndescription: Scope {}", i, i))?;
        fixture.create_data_file(&format!("scope{}", i), "todos.yaml", &format!("todos:\n  - title: Todo {}\n    priority: high", i))?;
    }

    // Run many concurrent operations
    let handles: Vec<_> = (0..20)
        .map(|_| {
            let rhema = rhema.clone();
            tokio::spawn(async move {
                rhema.discover_scopes_optimized().await
            })
        })
        .collect();

    // Wait for all operations to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    Ok(())
} 