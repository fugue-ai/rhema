use rhema_dependency::{
    DependencyManager, DependencyType, HealthStatus, HealthMetrics, ImpactScore
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏥 Health Monitoring Example");
    println!("===========================\n");
    
    // Initialize the dependency manager
    let mut manager = DependencyManager::new().await?;
    
    // Add dependencies with health monitoring
    println!("📦 Setting up dependencies for health monitoring...");
    
    // Database dependency
    manager.add_dependency(
        "postgres-db".to_string(),
        "PostgreSQL Database".to_string(),
        DependencyType::DataFlow,
        "postgresql://localhost:5432/myapp".to_string(),
        vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    ).await?;
    
    // API service dependency
    manager.add_dependency(
        "user-api".to_string(),
        "User Management API".to_string(),
        DependencyType::ApiCall,
        "https://user-api.example.com".to_string(),
        vec!["GET /users".to_string(), "POST /users".to_string(), "PUT /users".to_string()],
    ).await?;
    
    // Cache dependency
    manager.add_dependency(
        "redis-cache".to_string(),
        "Redis Cache".to_string(),
        DependencyType::Infrastructure,
        "redis://localhost:6379".to_string(),
        vec!["get".to_string(), "set".to_string(), "del".to_string()],
    ).await?;
    
    // External API dependency
    manager.add_dependency(
        "payment-gateway".to_string(),
        "Payment Gateway API".to_string(),
        DependencyType::ApiCall,
        "https://api.payment-gateway.com".to_string(),
        vec!["POST /payments".to_string(), "GET /payments".to_string()],
    ).await?;
    
    // Add relationships
    manager.add_dependency_relationship("user-api", "postgres-db", "depends_on".to_string(), 0.9, vec!["read".to_string(), "write".to_string()]).await?;
    manager.add_dependency_relationship("user-api", "redis-cache", "depends_on".to_string(), 0.7, vec!["get".to_string(), "set".to_string()]).await?;
    
    println!("✅ Dependencies configured for monitoring!\n");
    
    // Start health monitoring
    println!("🔍 Starting health monitoring...");
    manager.start_health_monitoring().await?;
    println!("✅ Health monitoring started!\n");
    
    // Simulate health status changes over time
    println!("📊 Simulating Health Status Changes");
    println!("===================================\n");
    
    // Initial healthy state
    println!("🟢 Initial State - All services healthy");
    update_health_statuses(&mut manager, HealthStatus::Healthy).await?;
    print_health_summary(&manager).await?;
    sleep(Duration::from_secs(2)).await;
    
    // Simulate database issues
    println!("\n🟡 Simulating Database Performance Issues");
    let degraded_metrics = HealthMetrics::new(
        500.0,  // High response time
        0.85,   // Reduced availability
        0.05,   // Some errors
        800.0,  // Reduced throughput
        0.8,    // High CPU usage
        0.7,    // High memory usage
        50.0,   // High network latency
        0.6,    // Moderate disk usage
    )?;
    
    manager.update_health_metrics("postgres-db", ImpactScore::new(0.6, 0.5, 0.4, 0.3, 0.2, 0.1)?).await?;
    manager.update_health_status("postgres-db", HealthStatus::Degraded).await?;
    print_health_summary(&manager).await?;
    sleep(Duration::from_secs(2)).await;
    
    // Simulate API service failure
    println!("\n🔴 Simulating API Service Failure");
    manager.update_health_status("user-api", HealthStatus::Down).await?;
    manager.update_health_metrics("user-api", ImpactScore::new(0.9, 0.8, 0.7, 0.6, 0.5, 0.4)?).await?;
    print_health_summary(&manager).await?;
    sleep(Duration::from_secs(2)).await;
    
    // Simulate cache issues
    println!("\n🟡 Simulating Cache Performance Issues");
    manager.update_health_status("redis-cache", HealthStatus::Unhealthy).await?;
    manager.update_health_metrics("redis-cache", ImpactScore::new(0.7, 0.6, 0.5, 0.4, 0.3, 0.2)?).await?;
    print_health_summary(&manager).await?;
    sleep(Duration::from_secs(2)).await;
    
    // Simulate external API issues
    println!("\n🟡 Simulating External API Issues");
    manager.update_health_status("payment-gateway", HealthStatus::Degraded).await?;
    manager.update_health_metrics("payment-gateway", ImpactScore::new(0.8, 0.7, 0.6, 0.5, 0.4, 0.3)?).await?;
    print_health_summary(&manager).await?;
    sleep(Duration::from_secs(2)).await;
    
    // Simulate recovery
    println!("\n🟢 Simulating Service Recovery");
    update_health_statuses(&mut manager, HealthStatus::Healthy).await?;
    print_health_summary(&manager).await?;
    sleep(Duration::from_secs(2)).await;
    
    // Perform health checks
    println!("\n🔍 Performing Manual Health Checks");
    println!("===================================\n");
    
    for dependency_id in ["postgres-db", "user-api", "redis-cache", "payment-gateway"] {
        println!("Checking health for {}...", dependency_id);
        match manager.perform_health_check(dependency_id).await {
            Ok(result) => {
                println!("  ✅ Health check successful");
                println!("    Response time: {:.2}ms", result.response_time_ms);
                println!("    Status code: {:?}", result.status_code);
                println!("    Duration: {:?}", result.duration);
                println!("    Message: {}", result.message);
            }
            Err(e) => {
                println!("  ❌ Health check failed: {}", e);
            }
        }
        println!();
    }
    
    // Get detailed health information
    println!("📋 Detailed Health Information");
    println!("==============================\n");
    
    let health_statuses = manager.get_all_health_statuses().await;
    for (dependency_id, status) in health_statuses {
        println!("🔍 {}", dependency_id);
        println!("  Status: {:?}", status.status);
        println!("  Health Score: {:.2}", status.health_score);
        println!("  Last Updated: {}", status.last_updated);
        
        if let Some(metrics) = status.metrics {
            println!("  Metrics:");
            println!("    Response Time: {:.2}ms", metrics.response_time_ms);
            println!("    Availability: {:.1}%", metrics.availability * 100.0);
            println!("    Error Rate: {:.1}%", metrics.error_rate * 100.0);
            println!("    Throughput: {:.0} req/s", metrics.throughput);
            println!("    CPU Usage: {:.1}%", metrics.cpu_usage * 100.0);
            println!("    Memory Usage: {:.1}%", metrics.memory_usage * 100.0);
        }
        
        if let Some(check) = status.last_check {
            println!("  Last Health Check:");
            println!("    Success: {}", check.success);
            println!("    Duration: {:?}", check.duration);
            println!("    Message: {}", check.message);
        }
        println!();
    }
    
    // Get health statistics
    println!("📊 Health Statistics");
    println!("===================\n");
    
    let health_stats = manager.get_health_statistics().await;
    println!("Overall Health Statistics:");
    println!("  Total Dependencies: {}", health_stats.total_dependencies);
    println!("  Healthy: {}", health_stats.healthy_count);
    println!("  Degraded: {}", health_stats.degraded_count);
    println!("  Unhealthy: {}", health_stats.unhealthy_count);
    println!("  Down: {}", health_stats.down_count);
    println!("  Unknown: {}", health_stats.unknown_count);
    println!("  Average Health Score: {:.2}", health_stats.average_health_score);
    println!("  Last Updated: {}", health_stats.last_updated);
    println!();
    
    // Impact analysis based on health status
    println!("📈 Health-Based Impact Analysis");
    println!("===============================\n");
    
    for dependency_id in ["postgres-db", "user-api", "redis-cache", "payment-gateway"] {
        println!("Analyzing impact of {} health issues...", dependency_id);
        let impact = manager.analyze_impact(dependency_id).await?;
        println!("  Business Impact Score: {:.2}", impact.business_impact_score);
        println!("  Risk Level: {:?}", impact.risk_level);
        println!("  Affected Services: {}", impact.affected_services.len());
        println!("  Estimated Downtime: {:?}", impact.estimated_downtime);
        println!("  Cost Impact: ${:.2}", impact.cost_impact);
        println!();
    }
    
    // Health-based recommendations
    println!("💡 Health-Based Recommendations");
    println!("===============================\n");
    
    let unhealthy_dependencies: Vec<_> = health_statuses
        .iter()
        .filter(|(_, status)| status.status != HealthStatus::Healthy)
        .collect();
    
    if !unhealthy_dependencies.is_empty() {
        println!("🔴 Unhealthy Dependencies Detected:");
        for (id, status) in unhealthy_dependencies {
            println!("  - {}: {:?} (Health Score: {:.2})", id, status.status, status.health_score);
        }
        println!();
        
        println!("🛠️  Recommended Actions:");
        println!("  1. Investigate root causes of unhealthy dependencies");
        println!("  2. Implement automated recovery procedures");
        println!("  3. Set up alerting for health status changes");
        println!("  4. Review and optimize health check configurations");
        println!("  5. Consider implementing circuit breakers");
        println!("  6. Plan for graceful degradation strategies");
        println!();
    } else {
        println!("✅ All dependencies are healthy!");
        println!("🛠️  Maintenance Recommendations:");
        println!("  1. Continue regular health monitoring");
        println!("  2. Review health check thresholds");
        println!("  3. Optimize performance where possible");
        println!("  4. Plan for capacity scaling");
        println!();
    }
    
    // Stop health monitoring
    println!("🛑 Stopping health monitoring...");
    manager.stop_health_monitoring().await?;
    println!("✅ Health monitoring stopped!");
    
    // Final health report
    println!("\n📋 Final Health Report");
    println!("=====================\n");
    
    let final_report = manager.get_health_report().await?;
    println!("System Health: {}", if final_report.is_healthy { "✅ Healthy" } else { "❌ Issues Detected" });
    println!("Critical Dependencies: {}", final_report.critical_dependencies.len());
    println!("Graph Statistics:");
    println!("  Total Dependencies: {}", final_report.graph_statistics.total_nodes);
    println!("  Total Relationships: {}", final_report.graph_statistics.total_edges);
    println!("  Healthy: {}", final_report.graph_statistics.healthy_count);
    println!("  Degraded: {}", final_report.graph_statistics.degraded_count);
    println!("  Unhealthy: {}", final_report.graph_statistics.unhealthy_count);
    println!("  Down: {}", final_report.graph_statistics.down_count);
    
    println!("\n🎉 Health Monitoring Example Completed Successfully!");
    println!("🏥 This example demonstrates:");
    println!("   - Real-time health status tracking");
    println!("   - Health metrics collection");
    println!("   - Impact analysis based on health");
    println!("   - Health-based recommendations");
    println!("   - Automated health monitoring");
    
    Ok(())
}

async fn update_health_statuses(manager: &mut DependencyManager, status: HealthStatus) -> Result<(), Box<dyn std::error::Error>> {
    for dependency_id in ["postgres-db", "user-api", "redis-cache", "payment-gateway"] {
        manager.update_health_status(dependency_id, status.clone()).await?;
    }
    Ok(())
}

async fn print_health_summary(manager: &DependencyManager) -> Result<(), Box<dyn std::error::Error>> {
    let health_statuses = manager.get_all_health_statuses().await;
    let mut healthy = 0;
    let mut degraded = 0;
    let mut unhealthy = 0;
    let mut down = 0;
    
    for (_, status) in health_statuses {
        match status.status {
            HealthStatus::Healthy => healthy += 1,
            HealthStatus::Degraded => degraded += 1,
            HealthStatus::Unhealthy => unhealthy += 1,
            HealthStatus::Down => down += 1,
            HealthStatus::Unknown => {}
        }
    }
    
    println!("Health Summary: 🟢{} 🟡{} 🟠{} 🔴{}", healthy, degraded, unhealthy, down);
    Ok(())
} 