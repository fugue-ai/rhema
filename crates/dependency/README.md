# Rhema Dependency Management

A comprehensive dependency management system for the Rhema project that provides semantic dependency types, impact analysis, health monitoring, and advanced validation capabilities.

## Features

### 🎯 Semantic Dependency Types
- **Data Flow Dependencies**: Track data dependencies between components
- **API Call Dependencies**: Monitor external API dependencies and their health
- **Infrastructure Dependencies**: Track infrastructure and deployment dependencies
- **Business Logic Dependencies**: Map business process dependencies

### 🔧 Advanced Dependency Resolution
- **Version Constraint Management**: Define and enforce version requirements
- **Multiple Resolution Strategies**: Latest, Minimum, Specific, MostStable, BestHealth
- **Conflict Detection**: Identify and resolve dependency conflicts
- **Smart Version Selection**: Choose optimal versions based on health and stability
- **Caching and Performance**: Optimized resolution with intelligent caching

### 🔮 Predictive Analytics
- **ML-based Failure Prediction**: Predict dependency failures before they occur
- **Trend Analysis**: Identify improving or declining dependency health trends
- **Anomaly Detection**: Detect unusual patterns in dependency behavior
- **Multiple Prediction Models**: Moving average, exponential smoothing, anomaly detection
- **Ensemble Predictions**: Combine multiple models for better accuracy
- **Risk Factor Identification**: Identify contributing factors to potential issues

### 🔒 Enhanced Security Scanning
- **Vulnerability Detection**: Scan dependencies for known security vulnerabilities
- **Compliance Checking**: Verify compliance with security standards (OWASP, NIST, etc.)
- **Security Scoring**: Comprehensive security assessment with risk scoring
- **Automated Recommendations**: Generate actionable security improvement suggestions
- **Risk Assessment**: Evaluate overall security risk and acceptability
- **External Database Integration**: Check against NVD, CVE, and vendor databases

### 📊 Impact Analysis Engine
- **Business Impact Assessment**: Quantify the business impact of dependency changes
- **Risk Analysis**: Identify high-risk dependencies and potential failure points
- **Change Impact Prediction**: Predict the impact of proposed changes
- **Cost-Benefit Analysis**: Evaluate the cost of dependency changes vs. benefits

### 🏥 Health Monitoring Integration
- **Real-time Dependency Tracking**: Monitor dependency health in real-time
- **Health Metrics Collection**: Collect comprehensive health metrics
- **Alerting System**: Proactive alerts for dependency issues
- **Performance Monitoring**: Track dependency performance and SLAs

### ✅ Advanced Validation
- **Circular Dependency Detection**: Identify and prevent circular dependencies
- **Dependency Validation**: Validate dependency configurations
- **Schema Validation**: Ensure dependency schemas are correct
- **Security Validation**: Validate dependency security requirements

## Architecture

### Core Components

1. **Dependency Graph Engine**: Manages the dependency graph and relationships
2. **Impact Analysis Engine**: Analyzes the impact of changes and failures
3. **Health Monitoring System**: Monitors dependency health and performance
4. **Validation Engine**: Validates dependencies and configurations
5. **Real-time Tracking**: Provides real-time updates and notifications

### Data Models

- **Dependency Types**: Different types of dependencies (data, API, infrastructure)
- **Impact Metrics**: Business impact metrics and risk assessments
- **Health Metrics**: Health and performance metrics
- **Validation Rules**: Rules for dependency validation

## Usage

### Enhanced Dependency Management

```rust
use rhema_dependency::{
    DependencyManager, DependencyType, DependencyResolver, ResolutionStrategy,
    PredictiveAnalytics, SecurityScanner, VersionConstraint, semver::Version
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize enhanced components
    let mut manager = DependencyManager::new().await?;
    let mut resolver = DependencyResolver::new();
    let predictive_analytics = PredictiveAnalytics::new();
    let security_scanner = SecurityScanner::new();
    
    // Add dependencies with version constraints
    let constraint = VersionConstraint::new(
        ">=1.0.0, <2.0.0".to_string(),
        Some(Version::parse("1.5.0").unwrap()),
        true,
    )?;
    
    resolver.set_version_constraint("my-dependency".to_string(), constraint);
    
    // Resolve dependencies using different strategies
    let dependencies = manager.list_dependencies().await?;
    let result = resolver.resolve_dependencies(&dependencies, ResolutionStrategy::BestHealth)?;
    
    // Perform predictive analysis
    let prediction = predictive_analytics.predict_health("my-dependency").await?;
    println!("Predicted health: {:?}", prediction.predicted_health);
    
    // Perform security scanning
    let config = manager.get_dependency_config("my-dependency").await?;
    let security_scan = security_scanner.scan_dependency(&config).await?;
    println!("Security score: {:.2}", security_scan.security_score);
    
    Ok(())
}
```

### Advanced Dependency Resolution

```rust
use rhema_dependency::{DependencyResolver, ResolutionStrategy, VersionConstraint};

let mut resolver = DependencyResolver::new();

// Set version constraints
let constraint = VersionConstraint::new(">=1.0.0, <2.0.0".to_string(), None, true)?;
resolver.set_version_constraint("my-dependency".to_string(), constraint);

// Add available versions with health scores
let versions = vec![
    Version::parse("1.0.0").unwrap(),
    Version::parse("1.1.0").unwrap(),
    Version::parse("1.2.0").unwrap(),
];
resolver.add_available_versions("my-dependency".to_string(), versions);

// Resolve using different strategies
let latest_result = resolver.resolve_dependencies(&dependencies, ResolutionStrategy::Latest)?;
let health_result = resolver.resolve_dependencies(&dependencies, ResolutionStrategy::BestHealth)?;
let stable_result = resolver.resolve_dependencies(&dependencies, ResolutionStrategy::MostStable)?;

println!("Latest strategy resolved: {}", latest_result.resolved_dependencies.len());
println!("Best health strategy resolved: {}", health_result.resolved_dependencies.len());
println!("Most stable strategy resolved: {}", stable_result.resolved_dependencies.len());
```

### Predictive Analytics

```rust
use rhema_dependency::PredictiveAnalytics;

let analytics = PredictiveAnalytics::new();

// Add historical data
for i in 0..20 {
    let metrics = HealthMetrics::new(
        100.0 + (i as f64 * 5.0), // response_time
        0.99 - (i as f64 * 0.001), // availability
        0.01 + (i as f64 * 0.0005), // error_rate
        100.0, // throughput
        0.5,   // cpu_usage
        0.6,   // memory_usage
        50.0,  // network_latency
        0.4,   // disk_usage
    )?;
    
    analytics.add_data_point("my-dependency".to_string(), metrics).await?;
}

// Predict health
let prediction = analytics.predict_health("my-dependency").await?;
println!("Predicted health: {:?}", prediction.predicted_health);
println!("Confidence: {:.2}", prediction.confidence);

// Analyze trends
let trend = analytics.analyze_trends("my-dependency").await?;
println!("Trend: {:?}", trend.trend);
println!("Strength: {:.2}", trend.strength);
```

### Security Scanning

```rust
use rhema_dependency::SecurityScanner;

let scanner = SecurityScanner::new();

// Scan dependency for security issues
let security_scan = scanner.scan_dependency(&dependency_config).await?;

println!("Security score: {:.2}", security_scan.security_score);
println!("Security status: {:?}", security_scan.security_status);
println!("Vulnerabilities: {}", security_scan.vulnerabilities.len());
println!("Compliance checks: {}", security_scan.compliance_checks.len());

// Get risk assessment
let risk = &security_scan.risk_assessment;
println!("Risk score: {:.2}", risk.risk_score);
println!("Risk level: {:?}", risk.risk_level);
println!("Risk acceptable: {}", risk.risk_acceptable);
```

### Real-time Health Monitoring

```rust
use rhema_dependency::HealthMonitor;

let monitor = HealthMonitor::new()
    .with_metrics(vec![
        "response_time",
        "availability",
        "error_rate"
    ])
    .with_alerts(vec![
        "high_latency",
        "service_down",
        "error_spike"
    ])
    .start()
    .await?;

// Monitor will automatically track health and send alerts
```

## Configuration

### Dependency Configuration

```yaml
dependencies:
  - name: "user-service"
    type: "api"
    endpoints:
      - "GET /users"
      - "POST /users"
    health_checks:
      - url: "http://user-service/health"
        interval: "30s"
    impact_metrics:
      - "user_registration"
      - "profile_management"
    risk_factors:
      - "high_availability_required"
      - "sensitive_data"
```

### Monitoring Configuration

```yaml
monitoring:
  metrics:
    - name: "response_time"
      threshold: "200ms"
      alert: "high_latency"
    - name: "availability"
      threshold: "99.9%"
      alert: "service_down"
  alerts:
    - name: "high_latency"
      channels: ["slack", "email"]
      severity: "warning"
    - name: "service_down"
      channels: ["pagerduty", "slack"]
      severity: "critical"
```

## API Reference

### DependencyManager

The main entry point for dependency management operations.

#### Methods

- `new() -> DependencyManager`: Create a new dependency manager
- `add_dependency(name, target, type, operations)`: Add a dependency
- `remove_dependency(name)`: Remove a dependency
- `get_dependency(name)`: Get dependency information
- `list_dependencies()`: List all dependencies
- `analyze_impact(name)`: Analyze the impact of a dependency
- `get_health(name)`: Get dependency health status

### ImpactAnalysis

Engine for analyzing the impact of dependency changes and failures.

#### Methods

- `new() -> ImpactAnalysis`: Create a new impact analysis engine
- `with_business_metrics(metrics)`: Set business impact metrics
- `with_risk_factors(factors)`: Set risk factors
- `analyze(graph)`: Analyze the dependency graph
- `predict_impact(change)`: Predict the impact of a change

### HealthMonitor

Real-time health monitoring system for dependencies.

#### Methods

- `new() -> HealthMonitor`: Create a new health monitor
- `with_metrics(metrics)`: Set health metrics to track
- `with_alerts(alerts)`: Configure alerts
- `start()`: Start monitoring
- `stop()`: Stop monitoring
- `get_status()`: Get current monitoring status

### ValidationEngine

Advanced validation system for dependencies.

#### Methods

- `new() -> ValidationEngine`: Create a new validation engine
- `validate_circular_dependencies(graph)`: Check for circular dependencies
- `validate_schema(schema)`: Validate dependency schema
- `validate_security(dependency)`: Validate security requirements
- `get_validation_report()`: Get validation results

## Metrics and Monitoring

### Prometheus Metrics

The system exposes the following Prometheus metrics:

- `dependency_health_status`: Current health status of dependencies
- `dependency_response_time`: Response time of dependencies
- `dependency_availability`: Availability percentage of dependencies
- `dependency_error_rate`: Error rate of dependencies
- `impact_analysis_score`: Business impact scores
- `validation_errors`: Number of validation errors

### Grafana Dashboards

Pre-configured Grafana dashboards are available for:

- Dependency Health Overview
- Impact Analysis Dashboard
- Performance Metrics
- Alert History

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running Examples

```bash
cargo run --example basic_usage
cargo run --example impact_analysis
cargo run --example health_monitoring
cargo run --example enhanced_usage
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

Apache 2.0 License - see LICENSE file for details. 