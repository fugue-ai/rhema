use rhema_dependency::{
    DependencyManager, DependencyType, HealthStatus, ImpactScore, HealthMetrics,
    DependencyResolver, ResolutionStrategy, PredictiveAnalytics, SecurityScanner,
    VersionConstraint, semver::Version
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Enhanced Dependency Management Example");
    println!("==========================================\n");

    // Initialize the dependency manager
    println!("📦 Initializing dependency manager...");
    let mut manager = DependencyManager::new().await?;
    
    // Initialize enhanced components
    let mut resolver = DependencyResolver::new();
    let predictive_analytics = PredictiveAnalytics::new();
    let security_scanner = SecurityScanner::new();
    
    // Add dependencies with version constraints
    println!("🔧 Adding dependencies with version constraints...");
    
    // Add a database dependency
    let db_dependency = manager.add_dependency(
        "postgres-db".to_string(),
        "PostgreSQL Database".to_string(),
        DependencyType::DataFlow,
        "postgresql://localhost:5432/myapp".to_string(),
        vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    ).await?;

    // Add an API dependency
    let api_dependency = manager.add_dependency(
        "user-api".to_string(),
        "User Management API".to_string(),
        DependencyType::ApiCall,
        "https://api.example.com/users".to_string(),
        vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
    ).await?;

    // Add an infrastructure dependency
    let cache_dependency = manager.add_dependency(
        "redis-cache".to_string(),
        "Redis Cache".to_string(),
        DependencyType::Infrastructure,
        "redis://localhost:6379".to_string(),
        vec!["get".to_string(), "set".to_string(), "del".to_string()],
    ).await?;

    // Configure version constraints
    println!("📋 Configuring version constraints...");
    
    let postgres_constraint = VersionConstraint::new(
        ">=13.0.0, <14.0.0".to_string(),
        Some(Version::parse("13.5.0").unwrap()),
        true,
    )?;
    
    let api_constraint = VersionConstraint::new(
        ">=2.0.0".to_string(),
        None,
        false,
    )?;
    
    let redis_constraint = VersionConstraint::new(
        ">=6.0.0, <7.0.0".to_string(),
        Some(Version::parse("6.2.0").unwrap()),
        true,
    )?;

    resolver.set_version_constraint("postgres-db".to_string(), postgres_constraint);
    resolver.set_version_constraint("user-api".to_string(), api_constraint);
    resolver.set_version_constraint("redis-cache".to_string(), redis_constraint);

    // Add available versions
    println!("📦 Adding available versions...");
    
    let postgres_versions = vec![
        Version::parse("13.0.0").unwrap(),
        Version::parse("13.1.0").unwrap(),
        Version::parse("13.5.0").unwrap(),
        Version::parse("13.6.0").unwrap(),
        Version::parse("14.0.0").unwrap(),
    ];
    
    let api_versions = vec![
        Version::parse("1.9.0").unwrap(),
        Version::parse("2.0.0").unwrap(),
        Version::parse("2.1.0").unwrap(),
        Version::parse("2.2.0").unwrap(),
    ];
    
    let redis_versions = vec![
        Version::parse("6.0.0").unwrap(),
        Version::parse("6.1.0").unwrap(),
        Version::parse("6.2.0").unwrap(),
        Version::parse("6.3.0").unwrap(),
    ];

    resolver.add_available_versions("postgres-db".to_string(), postgres_versions);
    resolver.add_available_versions("user-api".to_string(), api_versions);
    resolver.add_available_versions("redis-cache".to_string(), redis_versions);

    // Set health scores for versions
    println!("🏥 Setting health scores for versions...");
    
    let mut postgres_health_scores = std::collections::HashMap::new();
    postgres_health_scores.insert(Version::parse("13.0.0").unwrap(), 0.7);
    postgres_health_scores.insert(Version::parse("13.1.0").unwrap(), 0.8);
    postgres_health_scores.insert(Version::parse("13.5.0").unwrap(), 0.95);
    postgres_health_scores.insert(Version::parse("13.6.0").unwrap(), 0.9);
    postgres_health_scores.insert(Version::parse("14.0.0").unwrap(), 0.85);
    
    let mut api_health_scores = std::collections::HashMap::new();
    api_health_scores.insert(Version::parse("1.9.0").unwrap(), 0.6);
    api_health_scores.insert(Version::parse("2.0.0").unwrap(), 0.8);
    api_health_scores.insert(Version::parse("2.1.0").unwrap(), 0.9);
    api_health_scores.insert(Version::parse("2.2.0").unwrap(), 0.85);
    
    let mut redis_health_scores = std::collections::HashMap::new();
    redis_health_scores.insert(Version::parse("6.0.0").unwrap(), 0.7);
    redis_health_scores.insert(Version::parse("6.1.0").unwrap(), 0.8);
    redis_health_scores.insert(Version::parse("6.2.0").unwrap(), 0.95);
    redis_health_scores.insert(Version::parse("6.3.0").unwrap(), 0.9);

    resolver.set_health_scores("postgres-db".to_string(), postgres_health_scores);
    resolver.set_health_scores("user-api".to_string(), api_health_scores);
    resolver.set_health_scores("redis-cache".to_string(), redis_health_scores);

    // Resolve dependencies using different strategies
    println!("🔍 Resolving dependencies with different strategies...");
    
    let dependencies = manager.list_dependencies().await?;
    
    // Latest strategy
    let latest_result = resolver.resolve_dependencies(&dependencies, ResolutionStrategy::Latest)?;
    println!("  Latest strategy:");
    println!("    Successful: {}", latest_result.successful);
    println!("    Resolved: {}", latest_result.resolved_dependencies.len());
    println!("    Conflicts: {}", latest_result.conflicts.len());
    
    // Best health strategy
    let health_result = resolver.resolve_dependencies(&dependencies, ResolutionStrategy::BestHealth)?;
    println!("  Best health strategy:");
    println!("    Successful: {}", health_result.successful);
    println!("    Resolved: {}", health_result.resolved_dependencies.len());
    println!("    Conflicts: {}", health_result.conflicts.len());
    
    // Most stable strategy
    let stable_result = resolver.resolve_dependencies(&dependencies, ResolutionStrategy::MostStable)?;
    println!("  Most stable strategy:");
    println!("    Successful: {}", stable_result.successful);
    println!("    Resolved: {}", stable_result.resolved_dependencies.len());
    println!("    Conflicts: {}", stable_result.conflicts.len());

    // Add dependency relationships
    println!("\n🔗 Adding dependency relationships...");
    
    manager.add_dependency_relationship(
        "user-api",
        "postgres-db",
        "depends_on".to_string(),
        0.9,
        vec!["read".to_string(), "write".to_string()],
    ).await?;
    
    manager.add_dependency_relationship(
        "user-api",
        "redis-cache",
        "depends_on".to_string(),
        0.7,
        vec!["get".to_string(), "set".to_string()],
    ).await?;

    // Add historical data for predictive analytics
    println!("📊 Adding historical data for predictive analytics...");
    
    for i in 0..20 {
        let timestamp = chrono::Utc::now() - chrono::Duration::hours(i as i64);
        
        // Simulate varying health metrics over time
        let response_time = 100.0 + (i as f64 * 5.0) + (rand::random::<f64>() * 20.0);
        let availability = 0.99 - (i as f64 * 0.001) + (rand::random::<f64>() * 0.01);
        let error_rate = 0.01 + (i as f64 * 0.0005) + (rand::random::<f64>() * 0.005);
        
        let metrics = HealthMetrics::new(
            response_time,
            availability.max(0.0).min(1.0),
            error_rate.max(0.0).min(1.0),
            100.0 + (rand::random::<f64>() * 50.0),
            0.3 + (rand::random::<f64>() * 0.4),
            0.4 + (rand::random::<f64>() * 0.3),
            50.0 + (rand::random::<f64>() * 30.0),
            0.2 + (rand::random::<f64>() * 0.3),
        )?;
        
        predictive_analytics.add_data_point("postgres-db".to_string(), metrics).await?;
    }

    // Perform predictive analysis
    println!("🔮 Performing predictive analysis...");
    
    let prediction = predictive_analytics.predict_health("postgres-db").await?;
    println!("  Predicted health: {:?}", prediction.predicted_health);
    println!("  Confidence: {:.2}", prediction.confidence);
    println!("  Risk factors: {}", prediction.risk_factors.len());
    
    // Analyze trends
    let trend_analysis = predictive_analytics.analyze_trends("postgres-db").await?;
    println!("  Trend direction: {:?}", trend_analysis.trend);
    println!("  Trend strength: {:.2}", trend_analysis.strength);
    println!("  Trend description: {}", trend_analysis.description);

    // Perform security scanning
    println!("🔒 Performing security scanning...");
    
    let db_config = manager.get_dependency_config("postgres-db").await?;
    let security_scan = security_scanner.scan_dependency(&db_config).await?;
    
    println!("  Security score: {:.2}", security_scan.security_score);
    println!("  Security status: {:?}", security_scan.security_status);
    println!("  Vulnerabilities found: {}", security_scan.vulnerabilities.len());
    println!("  Compliance checks: {}", security_scan.compliance_checks.len());
    println!("  Recommendations: {}", security_scan.recommendations.len());
    
    // Display risk assessment
    println!("  Risk assessment:");
    println!("    Risk score: {:.2}", security_scan.risk_assessment.risk_score);
    println!("    Risk level: {:?}", security_scan.risk_assessment.risk_level);
    println!("    Risk acceptable: {}", security_scan.risk_assessment.risk_acceptable);

    // Update health statuses
    println!("\n🏥 Updating health statuses...");
    manager.update_health_status("postgres-db", HealthStatus::Healthy).await?;
    manager.update_health_status("user-api", HealthStatus::Degraded).await?;
    manager.update_health_status("redis-cache", HealthStatus::Healthy).await?;

    // Perform comprehensive impact analysis
    println!("📈 Performing comprehensive impact analysis...");
    let impact = manager.analyze_impact("postgres-db").await?;
    println!("  Business Impact Score: {:.2}", impact.business_impact_score);
    println!("  Risk Level: {:?}", impact.risk_level);
    println!("  Affected Services: {}", impact.affected_services.len());
    println!("  Estimated Downtime: {:?}", impact.estimated_downtime);
    println!("  Cost Impact: ${:.2}", impact.cost_impact);

    // Validate the dependency graph
    println!("\n✅ Validating dependency graph...");
    let validation = manager.validate_graph().await?;
    if validation.valid {
        println!("  ✅ Graph is valid");
    } else {
        println!("  ❌ Graph validation failed:");
        for error in validation.errors {
            println!("    - {}", error);
        }
    }

    // Check for circular dependencies
    println!("\n🔄 Checking for circular dependencies...");
    let has_circular = manager.has_circular_dependencies().await?;
    if has_circular {
        println!("  ⚠️  Circular dependencies detected!");
        let circular = manager.find_circular_dependencies().await?;
        for cycle in circular {
            println!("    Cycle: {}", cycle.join(" -> "));
        }
    } else {
        println!("  ✅ No circular dependencies found");
    }

    // Get comprehensive statistics
    println!("\n📊 Comprehensive statistics:");
    
    let graph_stats = manager.get_graph_statistics().await;
    println!("  Graph statistics:");
    println!("    Total Dependencies: {}", graph_stats.total_nodes);
    println!("    Total Relationships: {}", graph_stats.total_edges);
    println!("    Healthy: {}", graph_stats.healthy_count);
    println!("    Degraded: {}", graph_stats.degraded_count);
    println!("    Unhealthy: {}", graph_stats.unhealthy_count);
    println!("    Down: {}", graph_stats.down_count);

    let health_stats = manager.get_health_statistics().await;
    println!("  Health statistics:");
    println!("    Total health checks: {}", health_stats.total_health_checks);
    println!("    Successful checks: {}", health_stats.successful_checks);
    println!("    Failed checks: {}", health_stats.failed_checks);
    println!("    Average response time: {:.2}ms", health_stats.average_response_time);

    let validation_stats = manager.get_validation_statistics().await;
    println!("  Validation statistics:");
    println!("    Total validations: {}", validation_stats.total_validations);
    println!("    Successful validations: {}", validation_stats.successful_validations);
    println!("    Failed validations: {}", validation_stats.failed_validations);
    println!("    Total errors: {}", validation_stats.total_errors);

    let prediction_stats = predictive_analytics.get_statistics().await;
    println!("  Prediction statistics:");
    println!("    Total dependencies tracked: {}", prediction_stats.total_dependencies);
    println!("    Total data points: {}", prediction_stats.total_data_points);
    println!("    Cached predictions: {}", prediction_stats.cached_predictions);
    println!("    Models configured: {}", prediction_stats.models_configured);

    let security_stats = security_scanner.get_statistics().await;
    println!("  Security statistics:");
    println!("    Total scans: {}", security_stats.total_scans);
    println!("    Total vulnerabilities: {}", security_stats.total_vulnerabilities);
    println!("    Critical vulnerabilities: {}", security_stats.critical_vulnerabilities);
    println!("    High vulnerabilities: {}", security_stats.high_vulnerabilities);
    println!("    Average security score: {:.2}", security_stats.average_security_score);

    // Check system health
    println!("\n🏥 System health check:");
    let is_healthy = manager.is_healthy().await?;
    if is_healthy {
        println!("  ✅ System is healthy");
    } else {
        println!("  ❌ System has issues");
    }

    // Get comprehensive health report
    println!("\n📋 Comprehensive health report:");
    let report = manager.get_health_report().await?;
    println!("  Overall Health: {}", if report.is_healthy { "✅ Healthy" } else { "❌ Unhealthy" });
    println!("  Critical Dependencies: {}", report.critical_dependencies.len());
    println!("  Graph Statistics: {} nodes, {} edges", 
             report.graph_statistics.total_nodes, 
             report.graph_statistics.total_edges);
    println!("  Health Statistics: {} checks, {:.2}ms avg response", 
             report.health_statistics.total_health_checks,
             report.health_statistics.average_response_time);
    println!("  Validation Statistics: {} validations, {} errors", 
             report.validation_statistics.total_validations,
             report.validation_statistics.total_errors);

    // Export graph as DOT format
    println!("\n🔄 Exporting graph as DOT format...");
    let dot = manager.export_graph_dot().await?;
    println!("  Graph DOT format generated ({} characters)", dot.len());

    // Clear expired cache entries
    println!("\n🧹 Clearing expired cache entries...");
    resolver.clear_expired_cache();
    predictive_analytics.clear_expired_cache().await;
    security_scanner.clear_expired_cache().await;
    println!("  ✅ Cache cleared");

    println!("\n🎉 Enhanced dependency management example completed successfully!");
    println!("✨ All new features demonstrated:");
    println!("   - Advanced dependency resolution with version constraints");
    println!("   - Multiple resolution strategies (Latest, BestHealth, MostStable)");
    println!("   - Predictive analytics with trend analysis");
    println!("   - Security scanning with vulnerability detection");
    println!("   - Comprehensive risk assessment");
    println!("   - Enhanced caching and performance optimization");
    
    Ok(())
} 