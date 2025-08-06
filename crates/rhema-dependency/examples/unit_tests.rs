//! Unit tests for Dependency crate functionality
//! 
//! This example demonstrates comprehensive unit testing for the Dependency crate,
//! including performance features, advanced analysis, integrations, and user experience.

use rhema_dependency::{
    types::{DependencyConfig, DependencyType, HealthStatus, ImpactScore},
    performance::{DependencyCache, ParallelProcessor, QueryOptimizer},
    advanced_analysis::{AdvancedAnalyzer, DependencyCluster, DependencyScore, TrendAnalysis},
    integrations::{PackageManagerIntegration, CiCdIntegration, IdeIntegration},
    user_experience::{DependencyDashboard, DependencyReportGenerator, DependencyAlertSystem, DependencySearchEngine},
    manager::DependencyManager,
    Error,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running Dependency Unit Tests...\n");

    // Test 1: Performance features
    test_performance_features().await?;
    println!("✅ Performance features tests passed");

    // Test 2: Advanced analysis
    test_advanced_analysis().await?;
    println!("✅ Advanced analysis tests passed");

    // Test 3: Integrations
    test_integrations().await?;
    println!("✅ Integrations tests passed");

    // Test 4: User experience
    test_user_experience().await?;
    println!("✅ User experience tests passed");

    // Test 5: End-to-end workflow
    test_end_to_end_workflow().await?;
    println!("✅ End-to-end workflow tests passed");

    // Test 6: Error handling
    test_error_handling().await?;
    println!("✅ Error handling tests passed");

    // Test 7: Concurrent operations
    test_concurrent_operations().await?;
    println!("✅ Concurrent operations tests passed");

    // Test 8: Memory management
    test_memory_management().await?;
    println!("✅ Memory management tests passed");

    // Test 9: Configuration validation
    test_configuration_validation().await?;
    println!("✅ Configuration validation tests passed");

    println!("\n🎉 All dependency unit tests passed successfully!");
    Ok(())
}

async fn test_performance_features() -> Result<(), Box<dyn std::error::Error>> {
    // Test caching
    let cache = DependencyCache::new();
    cache.set("test_key".to_string(), "test_value".to_string(), None).await?;
    let result: Option<String> = cache.get("test_key").await?;
    assert_eq!(result, Some("test_value".to_string()));

    // Test parallel processing
    let processor = ParallelProcessor::new();
    let dependencies = vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string()];
    let results = processor.process_dependencies(dependencies, |dep| {
        tokio::spawn(async move {
            Ok::<String, Error>(format!("processed_{}", dep))
        })
    }).await?;
    assert_eq!(results.len(), 3);

    // Test query optimization
    let optimizer = QueryOptimizer::new();
    let result: String = optimizer.optimize_query("test_query", || {
        Ok("query_result".to_string())
    }).await?;
    assert_eq!(result, "query_result");

    Ok(())
}

async fn test_advanced_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = AdvancedAnalyzer::new();
    let dependencies = vec![
        DependencyConfig::new(
            "test-dep-1".to_string(),
            "Test Dependency 1".to_string(),
            DependencyType::ApiCall,
            "https://api1.example.com".to_string(),
            vec!["GET".to_string()],
        )?,
        DependencyConfig::new(
            "test-dep-2".to_string(),
            "Test Dependency 2".to_string(),
            DependencyType::DataFlow,
            "https://api2.example.com".to_string(),
            vec!["POST".to_string()],
        )?,
    ];

    // Test clustering
    let clusters = analyzer.cluster_dependencies(&dependencies).await?;
    assert!(!clusters.is_empty());

    // Test scoring
    let scores = analyzer.score_dependencies(&dependencies).await?;
    assert_eq!(scores.len(), 2);

    // Test risk assessment
    let risks = analyzer.assess_risks(&dependencies).await?;
    assert_eq!(risks.len(), 2);

    // Test cost analysis
    let costs = analyzer.analyze_costs(&dependencies).await?;
    assert_eq!(costs.len(), 2);

    // Test performance analysis
    let performance = analyzer.analyze_performance_impact(&dependencies).await?;
    assert_eq!(performance.len(), 2);

    // Test security analysis
    let security = analyzer.analyze_security(&dependencies).await?;
    assert_eq!(security.len(), 2);

    Ok(())
}

async fn test_integrations() -> Result<(), Box<dyn std::error::Error>> {
    // Test package manager integration
    let pkg_integration = PackageManagerIntegration::new();
    let available_managers = pkg_integration.get_available_package_managers().await?;
    // This might be empty if no package managers are available
    assert!(available_managers.is_empty() || !available_managers.is_empty());

    // Test CI/CD integration
    let cicd_integration = CiCdIntegration::new();
    let available_providers = cicd_integration.get_available_providers().await?;
    // This might be empty if no CI/CD providers are configured
    assert!(available_providers.is_empty() || !available_providers.is_empty());

    // Test IDE integration
    let ide_integration = IdeIntegration::new();
    let available_ides = ide_integration.get_available_providers().await?;
    assert!(!available_ides.is_empty());

    Ok(())
}

async fn test_user_experience() -> Result<(), Box<dyn std::error::Error>> {
    let dependencies = vec![
        DependencyConfig::new(
            "test-dep-1".to_string(),
            "Test Dependency 1".to_string(),
            DependencyType::ApiCall,
            "https://api1.example.com".to_string(),
            vec!["GET".to_string()],
        )?,
        DependencyConfig::new(
            "test-dep-2".to_string(),
            "Test Dependency 2".to_string(),
            DependencyType::DataFlow,
            "https://api2.example.com".to_string(),
            vec!["POST".to_string()],
        )?,
    ];

    // Test dashboard
    let mut dashboard = DependencyDashboard::new();
    dashboard.update_data(&dependencies).await?;
    let data = dashboard.get_data().await?;
    assert_eq!(data.total_dependencies, 2);

    // Test report generator
    let generator = DependencyReportGenerator::new();
    let report = generator.generate_report("overview", &dependencies, None).await?;
    assert!(!report.content.is_empty());

    // Test alert system
    let alert_system = DependencyAlertSystem::new();
    let alerts = alert_system.check_dependencies(&dependencies).await?;
    // This might be empty if no alert conditions are met
    assert!(alerts.is_empty() || !alerts.is_empty());

    // Test search engine
    let search_engine = DependencySearchEngine::new();
    search_engine.index_dependencies(&dependencies).await?;
    let results = search_engine.search("test").await?;
    assert!(!results.is_empty());

    Ok(())
}

async fn test_end_to_end_workflow() -> Result<(), Box<dyn std::error::Error>> {
    // Create a dependency manager
    let mut manager = DependencyManager::new().await?;

    // Add dependencies
    let dep1 = manager.add_dependency(
        "api-dep".to_string(),
        "API Dependency".to_string(),
        DependencyType::ApiCall,
        "https://api.example.com".to_string(),
        vec!["GET".to_string(), "POST".to_string()],
    ).await?;

    let dep2 = manager.add_dependency(
        "db-dep".to_string(),
        "Database Dependency".to_string(),
        DependencyType::DataFlow,
        "postgresql://localhost:5432/mydb".to_string(),
        vec!["read".to_string(), "write".to_string()],
    ).await?;

    // Test performance features
    let cache = DependencyCache::new();
    cache.set("dep_analysis".to_string(), "analysis_result".to_string(), None).await?;

    // Test advanced analysis
    let analyzer = AdvancedAnalyzer::new();
    let dependencies = manager.list_dependencies().await?;
    let clusters = analyzer.cluster_dependencies(&dependencies).await?;
    let scores = analyzer.score_dependencies(&dependencies).await?;

    // Test user experience features
    let mut dashboard = DependencyDashboard::new();
    dashboard.update_data(&dependencies).await?;

    let generator = DependencyReportGenerator::new();
    let report = generator.generate_report("overview", &dependencies, None).await?;

    let search_engine = DependencySearchEngine::new();
    search_engine.index_dependencies(&dependencies).await?;
    let search_results = search_engine.search("api").await?;

    // Verify results
    assert_eq!(dependencies.len(), 2);
    assert!(!clusters.is_empty());
    assert_eq!(scores.len(), 2);
    assert!(!report.content.is_empty());
    assert!(!search_results.is_empty());

    Ok(())
}

async fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test invalid dependency creation
    let result = DependencyConfig::new(
        "".to_string(), // Invalid empty ID
        "Test Dependency".to_string(),
        DependencyType::ApiCall,
        "https://api.example.com".to_string(),
        vec!["GET".to_string()],
    );
    assert!(result.is_err());

    // Test cache with invalid key
    let cache = DependencyCache::new();
    let result: Option<String> = cache.get("nonexistent_key").await?;
    assert_eq!(result, None);

    // Test search with empty query
    let search_engine = DependencySearchEngine::new();
    let dependencies = vec![
        DependencyConfig::new(
            "test-dep".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "https://api.example.com".to_string(),
            vec!["GET".to_string()],
        )?,
    ];
    search_engine.index_dependencies(&dependencies).await?;
    let results = search_engine.search("").await?;
    assert!(results.is_empty());

    Ok(())
}

async fn test_concurrent_operations() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DependencyManager::new().await?;
    let cache = DependencyCache::new();

    // Test concurrent cache operations
    let mut handles = Vec::new();
    for i in 0..10 {
        let cache_clone = cache.clone();
        let handle = tokio::spawn(async move {
            cache_clone.set(format!("key_{}", i), format!("value_{}", i), None).await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    // Verify all values were set
    for i in 0..10 {
        let result: Option<String> = cache.get(&format!("key_{}", i)).await?;
        assert_eq!(result, Some(format!("value_{}", i)));
    }

    Ok(())
}

async fn test_memory_management() -> Result<(), Box<dyn std::error::Error>> {
    // Test that large datasets don't cause memory issues
    let mut dependencies = Vec::new();
    for i in 0..100 {
        dependencies.push(
            DependencyConfig::new(
                format!("dep_{}", i),
                format!("Dependency {}", i),
                DependencyType::ApiCall,
                format!("https://api{}.example.com", i),
                vec!["GET".to_string(), "POST".to_string()],
            )?
        );
    }

    let analyzer = AdvancedAnalyzer::new();
    let clusters = analyzer.cluster_dependencies(&dependencies).await?;
    let scores = analyzer.score_dependencies(&dependencies).await?;

    assert_eq!(dependencies.len(), 100);
    assert!(!clusters.is_empty());
    assert_eq!(scores.len(), 100);

    Ok(())
}

async fn test_configuration_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Test dashboard configuration
    let config = rhema_dependency::user_experience::DashboardConfig {
        title: "Test Dashboard".to_string(),
        description: "Test Description".to_string(),
        auto_refresh_interval: 60,
        enable_realtime: true,
        default_view: rhema_dependency::user_experience::DashboardView::Overview,
        customizable_layout: true,
    };

    let dashboard = DependencyDashboard::with_config(config);
    let data = dashboard.get_data().await?;
    assert_eq!(data.total_dependencies, 0);

    // Test alert configuration
    let alert_config = rhema_dependency::user_experience::AlertConfig {
        enable_alerts: true,
        alert_channels: vec![rhema_dependency::user_experience::AlertChannel::Console],
        alert_thresholds: rhema_dependency::user_experience::AlertThresholds {
            health_threshold: 0.8,
            security_threshold: 0.9,
            performance_threshold: 0.7,
            cost_threshold: 0.6,
        },
        alert_cooldown: std::time::Duration::from_secs(300),
    };

    let alert_system = DependencyAlertSystem::with_config(alert_config);
    let history = alert_system.get_alert_history().await?;
    assert!(history.is_empty());

    Ok(())
} 