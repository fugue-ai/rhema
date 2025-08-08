use rhema_dependency::{
    DependencyManager, DependencyType, ImpactAnalysis, BusinessImpactMetrics, RiskFactors, RiskLevel
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Advanced Impact Analysis Example");
    println!("===================================\n");
    
    // Initialize the dependency manager
    let mut manager = DependencyManager::new().await?;
    
    // Create a complex dependency graph for analysis
    println!("📦 Setting up complex dependency graph...");
    
    // Core infrastructure dependencies
    manager.add_dependency(
        "aws-vpc".to_string(),
        "AWS VPC".to_string(),
        DependencyType::Infrastructure,
        "vpc-12345678".to_string(),
        vec!["network".to_string(), "routing".to_string()],
    ).await?;
    
    manager.add_dependency(
        "aws-rds".to_string(),
        "AWS RDS Database".to_string(),
        DependencyType::DataFlow,
        "rds-cluster-123".to_string(),
        vec!["read".to_string(), "write".to_string(), "backup".to_string()],
    ).await?;
    
    manager.add_dependency(
        "aws-elasticache".to_string(),
        "AWS ElastiCache Redis".to_string(),
        DependencyType::Infrastructure,
        "redis-cluster-456".to_string(),
        vec!["get".to_string(), "set".to_string(), "del".to_string()],
    ).await?;
    
    // Application services
    manager.add_dependency(
        "user-service".to_string(),
        "User Management Service".to_string(),
        DependencyType::ApiCall,
        "https://user-service.example.com".to_string(),
        vec!["GET /users".to_string(), "POST /users".to_string(), "PUT /users".to_string()],
    ).await?;
    
    manager.add_dependency(
        "payment-service".to_string(),
        "Payment Processing Service".to_string(),
        DependencyType::ApiCall,
        "https://payment-service.example.com".to_string(),
        vec!["POST /payments".to_string(), "GET /payments".to_string()],
    ).await?;
    
    manager.add_dependency(
        "notification-service".to_string(),
        "Notification Service".to_string(),
        DependencyType::ApiCall,
        "https://notification-service.example.com".to_string(),
        vec!["POST /notifications".to_string()],
    ).await?;
    
    // External dependencies
    manager.add_dependency(
        "stripe-api".to_string(),
        "Stripe Payment API".to_string(),
        DependencyType::ApiCall,
        "https://api.stripe.com".to_string(),
        vec!["POST /v1/charges".to_string(), "GET /v1/customers".to_string()],
    ).await?;
    
    manager.add_dependency(
        "sendgrid-api".to_string(),
        "SendGrid Email API".to_string(),
        DependencyType::ApiCall,
        "https://api.sendgrid.com".to_string(),
        vec!["POST /v3/mail/send".to_string()],
    ).await?;
    
    // Add dependency relationships
    println!("🔗 Establishing dependency relationships...");
    
    // Infrastructure dependencies
    manager.add_dependency_relationship("aws-rds", "aws-vpc", "depends_on".to_string(), 1.0, vec!["network".to_string()]).await?;
    manager.add_dependency_relationship("aws-elasticache", "aws-vpc", "depends_on".to_string(), 1.0, vec!["network".to_string()]).await?;
    
    // Application service dependencies
    manager.add_dependency_relationship("user-service", "aws-rds", "depends_on".to_string(), 0.9, vec!["read".to_string(), "write".to_string()]).await?;
    manager.add_dependency_relationship("user-service", "aws-elasticache", "depends_on".to_string(), 0.7, vec!["get".to_string(), "set".to_string()]).await?;
    
    manager.add_dependency_relationship("payment-service", "aws-rds", "depends_on".to_string(), 0.8, vec!["read".to_string(), "write".to_string()]).await?;
    manager.add_dependency_relationship("payment-service", "stripe-api", "depends_on".to_string(), 0.9, vec!["POST /v1/charges".to_string()]).await?;
    
    manager.add_dependency_relationship("notification-service", "sendgrid-api", "depends_on".to_string(), 0.8, vec!["POST /v3/mail/send".to_string()]).await?;
    
    // Service interdependencies
    manager.add_dependency_relationship("payment-service", "user-service", "depends_on".to_string(), 0.6, vec!["GET /users".to_string()]).await?;
    manager.add_dependency_relationship("notification-service", "user-service", "depends_on".to_string(), 0.5, vec!["GET /users".to_string()]).await?;
    
    println!("✅ Dependency graph created successfully!\n");
    
    // Perform impact analysis on different scenarios
    println!("📈 Impact Analysis Scenarios");
    println!("============================\n");
    
    // Scenario 1: AWS RDS failure
    println!("🔴 Scenario 1: AWS RDS Database Failure");
    println!("----------------------------------------");
    let rds_impact = manager.analyze_impact("aws-rds").await?;
    println!("  Business Impact Score: {:.2}", rds_impact.business_impact_score);
    println!("  Risk Level: {:?}", rds_impact.risk_level);
    println!("  Affected Services: {}", rds_impact.affected_services.len());
    println!("  Estimated Downtime: {:?}", rds_impact.estimated_downtime);
    println!("  Cost Impact: ${:.2}", rds_impact.cost_impact);
    println!("  Affected Services:");
    for service in &rds_impact.affected_services {
        println!("    - {}", service);
    }
    println!();
    
    // Scenario 2: Stripe API failure
    println!("🔴 Scenario 2: Stripe Payment API Failure");
    println!("------------------------------------------");
    let stripe_impact = manager.analyze_impact("stripe-api").await?;
    println!("  Business Impact Score: {:.2}", stripe_impact.business_impact_score);
    println!("  Risk Level: {:?}", stripe_impact.risk_level);
    println!("  Affected Services: {}", stripe_impact.affected_services.len());
    println!("  Estimated Downtime: {:?}", stripe_impact.estimated_downtime);
    println!("  Cost Impact: ${:.2}", stripe_impact.cost_impact);
    println!("  Affected Services:");
    for service in &stripe_impact.affected_services {
        println!("    - {}", service);
    }
    println!();
    
    // Scenario 3: User Service failure
    println!("🔴 Scenario 3: User Service Failure");
    println!("-----------------------------------");
    let user_service_impact = manager.analyze_impact("user-service").await?;
    println!("  Business Impact Score: {:.2}", user_service_impact.business_impact_score);
    println!("  Risk Level: {:?}", user_service_impact.risk_level);
    println!("  Affected Services: {}", user_service_impact.affected_services.len());
    println!("  Estimated Downtime: {:?}", user_service_impact.estimated_downtime);
    println!("  Cost Impact: ${:.2}", user_service_impact.cost_impact);
    println!("  Affected Services:");
    for service in &user_service_impact.affected_services {
        println!("    - {}", service);
    }
    println!();
    
    // Scenario 4: AWS VPC failure (infrastructure)
    println!("🔴 Scenario 4: AWS VPC Infrastructure Failure");
    println!("----------------------------------------------");
    let vpc_impact = manager.analyze_impact("aws-vpc").await?;
    println!("  Business Impact Score: {:.2}", vpc_impact.business_impact_score);
    println!("  Risk Level: {:?}", vpc_impact.risk_level);
    println!("  Affected Services: {}", vpc_impact.affected_services.len());
    println!("  Estimated Downtime: {:?}", vpc_impact.estimated_downtime);
    println!("  Cost Impact: ${:.2}", vpc_impact.cost_impact);
    println!("  Affected Services:");
    for service in &vpc_impact.affected_services {
        println!("    - {}", service);
    }
    println!();
    
    // Change impact analysis
    println!("🔄 Change Impact Analysis");
    println!("=========================\n");
    
    // Scenario: Adding a new service
    println!("🟢 Scenario: Adding New Analytics Service");
    println!("----------------------------------------");
    let change_impact = manager.analyze_change_impact(
        "Adding new analytics service that depends on user-service and payment-service",
        &["user-service".to_string(), "payment-service".to_string()],
    ).await?;
    println!("  Business Impact Score: {:.2}", change_impact.business_impact_score);
    println!("  Risk Level: {:?}", change_impact.risk_level);
    println!("  Affected Services: {}", change_impact.affected_services.len());
    println!("  Estimated Implementation Time: {:?}", change_impact.estimated_downtime);
    println!("  Implementation Cost: ${:.2}", change_impact.cost_impact);
    println!();
    
    // Risk assessment summary
    println!("⚠️  Risk Assessment Summary");
    println!("==========================\n");
    
    let dependencies = manager.list_dependencies().await?;
    let mut high_risk_dependencies = Vec::new();
    let mut medium_risk_dependencies = Vec::new();
    
    for dep in dependencies {
        let impact = manager.analyze_impact(&dep.id).await?;
        match impact.risk_level {
            RiskLevel::Critical | RiskLevel::High => {
                high_risk_dependencies.push((dep.name, impact.business_impact_score));
            }
            RiskLevel::Medium => {
                medium_risk_dependencies.push((dep.name, impact.business_impact_score));
            }
            _ => {}
        }
    }
    
    println!("🔴 High Risk Dependencies:");
    high_risk_dependencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (name, score) in high_risk_dependencies {
        println!("  - {} (Impact Score: {:.2})", name, score);
    }
    println!();
    
    println!("🟡 Medium Risk Dependencies:");
    medium_risk_dependencies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (name, score) in medium_risk_dependencies {
        println!("  - {} (Impact Score: {:.2})", name, score);
    }
    println!();
    
    // Business impact recommendations
    println!("💡 Business Impact Recommendations");
    println!("==================================\n");
    
    println!("1. 🔴 Critical Dependencies:");
    println!("   - Implement redundancy for AWS RDS (database clustering)");
    println!("   - Set up Stripe API fallback mechanisms");
    println!("   - Implement circuit breakers for external APIs");
    println!();
    
    println!("2. 🟡 High Priority Improvements:");
    println!("   - Add caching layers to reduce database load");
    println!("   - Implement graceful degradation for non-critical features");
    println!("   - Set up monitoring and alerting for all critical paths");
    println!();
    
    println!("3. 🟢 Risk Mitigation Strategies:");
    println!("   - Regular dependency health checks");
    println!("   - Automated failover procedures");
    println!("   - Business continuity planning");
    println!("   - Cost optimization for high-impact dependencies");
    println!();
    
    // Export detailed analysis
    println!("📊 Exporting Analysis Results");
    println!("=============================\n");
    
    let graph_stats = manager.get_graph_statistics().await;
    println!("Graph Statistics:");
    println!("  Total Dependencies: {}", graph_stats.total_nodes);
    println!("  Total Relationships: {}", graph_stats.total_edges);
    println!("  Average Dependencies per Service: {:.2}", 
        if graph_stats.total_nodes > 0 { graph_stats.total_edges as f64 / graph_stats.total_nodes as f64 } else { 0.0 });
    
    let health_report = manager.get_health_report().await?;
    println!("  System Health: {}", if health_report.is_healthy { "✅ Healthy" } else { "❌ Issues Detected" });
    println!("  Critical Dependencies: {}", health_report.critical_dependencies.len());
    
    println!("\n🎉 Impact Analysis Example Completed Successfully!");
    println!("📈 This analysis provides insights for:");
    println!("   - Business continuity planning");
    println!("   - Risk management strategies");
    println!("   - Infrastructure investment decisions");
    println!("   - Service architecture improvements");
    
    Ok(())
} 