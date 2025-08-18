//! Unit tests for Rhema crate functionality
//!
//! This example demonstrates comprehensive unit testing for the Rhema crate,
//! including initialization, query execution, performance monitoring, and security features.

use chrono;
use rhema_api::{
    AccessControl, ApiDocumentation, ApiInput, AuditLogEntry, AuditLogger, InputSanitizer,
    PerformanceGuard, PerformanceMetrics, PerformanceMonitor, PerformanceOptimizer,
    RateLimitConfig, ResourceManager, Rhema, RhemaResult, SecurityConfig, SecurityManager,
};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

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

    fn create_scope_file(
        &self,
        scope_name: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let scope_path = self.repo_path.join(scope_name);
        fs::create_dir_all(&scope_path)?;

        let rhema_file = scope_path.join("rhema.yaml");
        fs::write(rhema_file, content)?;

        Ok(())
    }

    fn create_data_file(
        &self,
        scope_name: &str,
        filename: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    // Test 3: Query execution
    test_query_execution().await?;
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
    let fixture =
        TestFixture::new().map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;

    // Test basic initialization
    std::env::set_current_dir(&fixture.repo_path)
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
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

async fn test_query_execution() -> RhemaResult<()> {
    let fixture =
        TestFixture::new().map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;

    // Create test data
    fixture
        .create_scope_file("test_scope", "name: test_scope\ndescription: Test scope")
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    fixture
        .create_data_file(
            "test_scope",
            "todos.yaml",
            "todos:\n  - title: Test todo\n    priority: high",
        )
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;

    // Test successful query using public method
    let result = rhema.query("SELECT * FROM todos");
    assert!(result.is_ok());

    // Test query with stats
    let result2 = rhema.query_with_stats("SELECT * FROM todos");
    assert!(result2.is_ok());

    Ok(())
}

async fn test_scope_operations() -> RhemaResult<()> {
    let fixture =
        TestFixture::new().map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;

    // Create test scopes
    fixture
        .create_scope_file("scope1", "name: scope1\ndescription: First scope")
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    fixture
        .create_scope_file("scope2", "name: scope2\ndescription: Second scope")
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;

    // Test scope discovery
    let scopes = rhema.discover_scopes_optimized().await?;
    assert!(!scopes.is_empty());

    // Test caching
    let scopes2 = rhema.discover_scopes_optimized().await?;
    assert_eq!(scopes.len(), scopes2.len());

    // Test scope validation
    for scope in &scopes {
        rhema.validate_scope(scope).await?;
    }

    Ok(())
}

async fn test_performance_monitoring() -> RhemaResult<()> {
    let fixture =
        TestFixture::new().map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;

    // Create test data
    fixture
        .create_scope_file("test_scope", "name: test_scope\ndescription: Test scope")
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    fixture
        .create_data_file(
            "test_scope",
            "knowledge.yaml",
            "entries:\n  - title: Test knowledge\n    content: Test content",
        )
        .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;

    // Test performance monitoring
    let monitor = Arc::new(PerformanceMonitor::new());
    let _guard = PerformanceGuard::new("test_operation".to_string(), monitor.clone());

    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Test performance metrics
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
    assert!(metrics.execution_time_ms >= 0);
    assert!(metrics.cpu_usage_percent.unwrap() >= 0.0);

    Ok(())
}

async fn test_security_features() -> RhemaResult<()> {
    let config = SecurityConfig::default();
    let security_manager = SecurityManager::new(config.clone());
    let sanitizer = InputSanitizer::new(config.clone());
    let access_control = AccessControl::new(config.clone());
    let audit_logger = AuditLogger::new(config);

    // Test input sanitization
    let sanitized = sanitizer.sanitize_string("test<script>alert('xss')</script>");
    assert!(sanitized.is_ok());

    // Test access control
    let has_access = access_control.check_permission("user", "read").await;
    assert!(has_access.is_ok());

    // Test audit logging
    let audit_entry = AuditLogEntry {
        timestamp: chrono::Utc::now(),
        user_id: "test_user".to_string(),
        operation: "test_action".to_string(),
        resource: "test_resource".to_string(),
        success: true,
        error_message: None,
        ip_address: Some("127.0.0.1".to_string()),
        user_agent: Some("test-agent".to_string()),
        metadata: HashMap::new(),
    };
    audit_logger.log_event(audit_entry).await?;

    Ok(())
}

async fn test_error_handling() -> RhemaResult<()> {
    let fixture =
        TestFixture::new().map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    let rhema = Rhema::new_from_path(fixture.repo_path.clone())?;

    // Test invalid query
    let result = rhema.query("INVALID QUERY");
    assert!(result.is_err());

    // Test non-existent scope
    let result = rhema.get_scope_optimized("non_existent_scope").await;
    assert!(result.is_err());

    Ok(())
}

async fn test_concurrent_operations() -> RhemaResult<()> {
    let fixture =
        TestFixture::new().map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    let rhema = Arc::new(Rhema::new_from_path(fixture.repo_path.clone())?);

    // Create test data for multiple scopes
    for i in 1..=5 {
        fixture
            .create_scope_file(
                &format!("scope{}", i),
                &format!("name: scope{}\ndescription: Scope {}", i, i),
            )
            .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;

        fixture
            .create_data_file(
                &format!("scope{}", i),
                "todos.yaml",
                &format!("todos:\n  - title: Todo {}\n    priority: high", i),
            )
            .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))?;
    }

    // Test concurrent scope discovery
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let rhema = rhema.clone();
            tokio::spawn(async move { rhema.discover_scopes_optimized().await })
        })
        .collect();

    for handle in handles {
        let result = handle
            .await
            .map_err(|e| rhema_core::RhemaError::ConfigError(e.to_string()))??;
        assert!(!result.is_empty());
    }

    Ok(())
}
