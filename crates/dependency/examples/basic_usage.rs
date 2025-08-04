use rhema_dependency::{
    DependencyManager, DependencyType, HealthStatus, ImpactScore, HealthMetrics
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the dependency manager
    println!("🚀 Initializing dependency manager...");
    let mut manager = DependencyManager::new().await?;
    
    // Add some dependencies
    println!("📦 Adding dependencies...");
    
    // Add a database dependency
    manager.add_dependency(
        "postgres-db".to_string(),
        "PostgreSQL Database".to_string(),
        DependencyType::DataFlow,
        "postgresql://localhost:5432/myapp".to_string(),
        vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    ).await?;
    
    // Add an API dependency
    manager.add_dependency(
        "user-api".to_string(),
        "User Management API".to_string(),
        DependencyType::ApiCall,
        "https://api.example.com/users".to_string(),
        vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
    ).await?;
    
    // Add an infrastructure dependency
    manager.add_dependency(
        "redis-cache".to_string(),
        "Redis Cache".to_string(),
        DependencyType::Infrastructure,
        "redis://localhost:6379".to_string(),
        vec!["get".to_string(), "set".to_string(), "del".to_string()],
    ).await?;
    
    // Add dependency relationships
    println!("🔗 Adding dependency relationships...");
    
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
    
    // List all dependencies
    println!("📋 Listing all dependencies:");
    let dependencies = manager.list_dependencies().await?;
    for dep in dependencies {
        println!("  - {} ({}) -> {}", dep.name, dep.dependency_type, dep.target);
    }
    
    // Get dependents of a dependency
    println!("\n🔍 Dependents of postgres-db:");
    let dependents = manager.get_dependents("postgres-db").await?;
    for dependent in dependents {
        println!("  - {}", dependent);
    }
    
    // Update health status
    println!("\n🏥 Updating health status...");
    manager.update_health_status("postgres-db", HealthStatus::Healthy).await?;
    manager.update_health_status("user-api", HealthStatus::Degraded).await?;
    manager.update_health_status("redis-cache", HealthStatus::Healthy).await?;
    
    // Get health status
    println!("\n📊 Health status:");
    let health_statuses = manager.get_all_health_statuses().await;
    for (id, status) in health_statuses {
        println!("  - {}: {:?}", id, status.status);
    }
    
    // Perform impact analysis
    println!("\n📈 Performing impact analysis...");
    let impact = manager.analyze_impact("postgres-db").await?;
    println!("  Business Impact Score: {:.2}", impact.business_impact_score);
    println!("  Risk Level: {:?}", impact.risk_level);
    println!("  Affected Services: {}", impact.affected_services.len());
    println!("  Estimated Downtime: {:?}", impact.estimated_downtime);
    println!("  Cost Impact: ${:.2}", impact.cost_impact);
    
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
    
    // Get graph statistics
    println!("\n📊 Graph statistics:");
    let stats = manager.get_graph_statistics().await;
    println!("  Total Dependencies: {}", stats.total_nodes);
    println!("  Total Relationships: {}", stats.total_edges);
    println!("  Healthy: {}", stats.healthy_count);
    println!("  Degraded: {}", stats.degraded_count);
    println!("  Unhealthy: {}", stats.unhealthy_count);
    println!("  Down: {}", stats.down_count);
    
    // Check system health
    println!("\n🏥 System health check:");
    let is_healthy = manager.is_healthy().await?;
    if is_healthy {
        println!("  ✅ System is healthy");
    } else {
        println!("  ❌ System has issues");
    }
    
    // Get comprehensive health report
    println!("\n📋 Health report:");
    let report = manager.get_health_report().await?;
    println!("  Overall Health: {}", if report.is_healthy { "✅ Healthy" } else { "❌ Unhealthy" });
    println!("  Critical Dependencies: {}", report.critical_dependencies.len());
    
    // Export graph as DOT format
    println!("\n🔄 Exporting graph as DOT format...");
    let dot = manager.export_graph_dot().await?;
    println!("  Graph DOT format generated ({} characters)", dot.len());
    
    println!("\n🎉 Basic usage example completed successfully!");
    
    Ok(())
} 