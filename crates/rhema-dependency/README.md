# Rhema Dependency Management

[![Crates.io](https://img.shields.io/crates/v/rhema-dependency)](https://crates.io/crates/rhema-dependency)
[![Documentation](https://docs.rs/rhema-dependency/badge.svg)](https://docs.rs/rhema-dependency)

A comprehensive dependency management system for the Rhema project that provides semantic dependency types, impact analysis, health monitoring, and advanced validation capabilities.

## Overview

The `rhema-dependency` crate provides a sophisticated dependency management system that goes beyond traditional package management. It offers semantic dependency classification, real-time health monitoring, predictive analytics, security scanning, and comprehensive impact analysis for modern software systems.

## Features

### 🎯 Semantic Dependency Types
- **Data Flow Dependencies**: Track data dependencies between components (databases, data sources)
- **API Call Dependencies**: Monitor external API dependencies and their health
- **Infrastructure Dependencies**: Track infrastructure and deployment dependencies (servers, networks, cloud services)
- **Business Logic Dependencies**: Map business process dependencies
- **Security Dependencies**: Authentication, authorization, encryption dependencies
- **Monitoring Dependencies**: Logging, metrics, alerting dependencies
- **Configuration Dependencies**: Configuration management dependencies
- **Deployment Dependencies**: CI/CD, containers, orchestration dependencies

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
- **Revenue Impact Tracking**: Monitor revenue impact of dependency issues
- **User Experience Impact**: Assess impact on user experience
- **Operational Cost Impact**: Calculate operational cost implications

### 🏥 Health Monitoring Integration
- **Real-time Dependency Tracking**: Monitor dependency health in real-time
- **Health Metrics Collection**: Collect comprehensive health metrics (response time, availability, error rate, throughput, CPU, memory, network latency, disk usage)
- **Alerting System**: Proactive alerts for dependency issues
- **Performance Monitoring**: Track dependency performance and SLAs
- **Health Score Calculation**: Weighted health scoring based on multiple metrics

### ✅ Advanced Validation
- **Circular Dependency Detection**: Identify and prevent circular dependencies
- **Dependency Validation**: Validate dependency configurations
- **Schema Validation**: Ensure dependency schemas are correct
- **Security Validation**: Validate dependency security requirements
- **Performance Validation**: Validate performance requirements
- **Parallel Validation**: Enable parallel validation for improved performance

### 🔄 Real-time Operations
- **WebSocket Integration**: Real-time updates and notifications
- **Live Health Monitoring**: Continuous health status updates
- **Real-time Alerts**: Immediate notification of issues
- **Live Metrics**: Real-time performance metrics

### 🎨 User Experience Features
- **Dependency Dashboard**: Visual dashboard for dependency management
- **Report Generation**: Comprehensive dependency reports
- **Search Engine**: Advanced dependency search capabilities
- **Alert System**: Configurable alerting system
- **Performance Optimization**: Optimized for large dependency graphs

### 🚀 Performance & Scalability
- **Parallel Processing**: Multi-threaded operations for improved performance
- **Intelligent Caching**: Smart caching strategies for dependency data
- **Memory Optimization**: Efficient memory usage for large dependency graphs
- **Async Operations**: Full async/await support for non-blocking operations

## Architecture

### Core Components

1. **DependencyManager**: Main orchestrator for all dependency operations
2. **DependencyGraph**: Manages the dependency graph and relationships using petgraph
3. **ImpactAnalysis**: Analyzes the impact of changes and failures
4. **HealthMonitor**: Monitors dependency health and performance
5. **ValidationEngine**: Validates dependencies and configurations
6. **DependencyResolver**: Handles version resolution and conflict detection
7. **PredictiveAnalytics**: Provides ML-based failure prediction
8. **SecurityScanner**: Scans dependencies for security vulnerabilities
9. **Real-time System**: Provides WebSocket-based real-time updates
10. **Storage System**: Persistent storage with SQLite, PostgreSQL, and MySQL support
11. **Performance Optimizer**: Parallel processing and caching optimizations
12. **Advanced Analyzer**: Sophisticated analysis capabilities
13. **Integration Layer**: Package manager and CI/CD integrations

### Data Models

- **DependencyConfig**: Complete dependency configuration with health checks, impact config, security requirements
- **HealthMetrics**: Comprehensive health metrics (response time, availability, error rate, etc.)
- **ImpactScore**: Business impact assessment with multiple dimensions
- **HealthStatus**: Health status enumeration (Healthy, Degraded, Unhealthy, Down, Unknown)
- **DependencyType**: Semantic dependency type classification
- **RiskLevel**: Risk assessment levels (Low, Medium, High, Critical)

## Usage

### Basic Dependency Management

```rust
use rhema_dependency::{
    DependencyManager, DependencyType, HealthStatus, init
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the dependency manager
    let mut manager = init().await?;
    
    // Add dependencies
    manager.add_dependency(
        "postgres-db".to_string(),
        "PostgreSQL Database".to_string(),
        DependencyType::DataFlow,
        "postgresql://localhost:5432/myapp".to_string(),
        vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    ).await?;
    
    // Add dependency relationships
    manager.add_dependency_relationship(
        "user-api",
        "postgres-db",
        "depends_on".to_string(),
        0.9,
        vec!["read".to_string(), "write".to_string()],
    ).await?;
    
    // List all dependencies
    let dependencies = manager.list_dependencies().await?;
    
    // Update health status
    manager.update_health_status("postgres-db", HealthStatus::Healthy).await?;
    
    // Perform impact analysis
    let impact = manager.analyze_impact("postgres-db").await?;
    println!("Business Impact Score: {:.2}", impact.business_impact);
    
    Ok(())
}
```

### Advanced Dependency Resolution

```rust
use rhema_dependency::{DependencyResolver, ResolutionStrategy, VersionConstraint, semver::Version};

let mut resolver = DependencyResolver::new();

// Set version constraints
let constraint = VersionConstraint::new(
    ">=1.0.0, <2.0.0".to_string(),
    Some(Version::parse("1.5.0").unwrap()),
    true,
)?;
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
```

### Predictive Analytics

```rust
use rhema_dependency::{PredictiveAnalytics, HealthMetrics};

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
```

### Performance Optimization

```rust
use rhema_dependency::{ParallelProcessor, DependencyCache};

// Use parallel processing for large operations
let processor = ParallelProcessor::new();
let results = processor.process_dependencies_parallel(&dependencies).await?;

// Use intelligent caching
let cache = DependencyCache::new();
let cached_data = cache.get_or_compute("dependency-key", || {
    // Expensive computation
    compute_dependency_data()
}).await?;
```

## Configuration

### Dependency Configuration

```yaml
dependencies:
  - id: "user-service"
    name: "User Management Service"
    type: "ApiCall"
    target: "https://api.example.com/users"
    operations: ["GET", "POST", "PUT", "DELETE"]
    health_check:
      url: "http://user-service/health"
      interval_seconds: 30
      timeout_seconds: 5
      expected_status: 200
      method: "GET"
      headers:
        Authorization: "Bearer ${API_TOKEN}"
    impact_config:
      business_impact_weight: 0.8
      revenue_impact_weight: 0.7
      user_experience_impact_weight: 0.6
      operational_cost_impact_weight: 0.5
      security_impact_weight: 0.9
      compliance_impact_weight: 0.7
      cost_per_hour: 1000.0
      critical_functions: ["user_registration", "profile_management"]
    security_requirements:
      authentication_required: true
      authorization_required: true
      encryption_required: true
      audit_logging_required: true
      security_level: "High"
      compliance_standards: ["SOC2", "GDPR"]
    performance_requirements:
      max_response_time_ms: 200.0
      min_availability: 0.999
      max_error_rate: 0.001
      min_throughput: 100.0
      sla_level: "Gold"
    metadata:
      team: "backend"
      environment: "production"
      priority: "high"
```

### Monitoring Configuration

```yaml
monitoring:
  health_check_interval: 30
  health_check_timeout: 5
  enable_realtime_monitoring: true
  enable_alerting: true
  metrics_retention_hours: 168
  health_score_weights:
    response_time: 0.25
    availability: 0.30
    error_rate: 0.25
    throughput: 0.10
    cpu_usage: 0.05
    memory_usage: 0.05
  parallel_processing:
    enabled: true
    max_workers: 8
  caching:
    enabled: true
    ttl_seconds: 300
    max_size: 1000
```

### Storage Configuration

```yaml
storage:
  database:
    type: "sqlite"  # or "postgres", "mysql"
    url: "sqlite:dependencies.db"
    pool_size: 10
  backup:
    enabled: true
    interval_hours: 24
    retention_days: 30
```

## API Reference

### DependencyManager

The main entry point for dependency management operations.

#### Key Methods

- `new() -> DependencyManager`: Create a new dependency manager
- `with_config(config) -> DependencyManager`: Create with custom configuration
- `add_dependency(id, name, type, target, operations)`: Add a dependency
- `remove_dependency(id)`: Remove a dependency
- `get_dependency_config(id)`: Get dependency configuration
- `list_dependencies()`: List all dependencies
- `analyze_impact(id)`: Analyze the impact of a dependency
- `get_health(id)`: Get dependency health status
- `update_health_status(id, status)`: Update dependency health status
- `add_dependency_relationship(source, target, type, strength, operations)`: Add dependency relationship
- `has_circular_dependencies()`: Check for circular dependencies
- `validate_graph()`: Validate the dependency graph
- `get_graph_statistics()`: Get graph statistics
- `export_graph_dot()`: Export graph as DOT format
- `get_health_report()`: Get comprehensive health report

### HealthMonitor

Real-time health monitoring system for dependencies.

#### Key Methods

- `new() -> HealthMonitor`: Create a new health monitor
- `with_config(graph, config)`: Create with configuration
- `start()`: Start monitoring
- `stop()`: Stop monitoring
- `get_status()`: Get current monitoring status
- `get_metrics(id)`: Get health metrics for a dependency

### ValidationEngine

Advanced validation system for dependencies.

#### Key Methods

- `new() -> ValidationEngine`: Create a new validation engine
- `with_config(config)`: Create with configuration
- `validate_dependency(config, graph)`: Validate a dependency
- `validate_graph(graph)`: Validate the entire graph
- `get_validation_report()`: Get validation results
- `validate_parallel(dependencies)`: Parallel validation

### DependencyResolver

Handles version resolution and conflict detection.

#### Key Methods

- `new() -> DependencyResolver`: Create a new resolver
- `set_version_constraint(name, constraint)`: Set version constraint
- `add_available_versions(name, versions)`: Add available versions
- `resolve_dependencies(dependencies, strategy)`: Resolve dependencies
- `detect_conflicts(dependencies)`: Detect version conflicts

### PredictiveAnalytics

ML-based failure prediction and trend analysis.

#### Key Methods

- `new() -> PredictiveAnalytics`: Create a new analytics engine
- `add_data_point(name, metrics)`: Add historical data
- `predict_health(name)`: Predict health status
- `analyze_trends(name)`: Analyze health trends
- `detect_anomalies(name)`: Detect anomalous behavior

### SecurityScanner

Security vulnerability scanning and compliance checking.

#### Key Methods

- `new() -> SecurityScanner`: Create a new security scanner
- `scan_dependency(config)`: Scan a dependency for vulnerabilities
- `check_compliance(config)`: Check compliance with standards
- `generate_security_report()`: Generate comprehensive security report

### ParallelProcessor

High-performance parallel processing for large dependency operations.

#### Key Methods

- `new() -> ParallelProcessor`: Create a new processor
- `process_dependencies_parallel(dependencies)`: Process dependencies in parallel
- `set_max_workers(count)`: Set maximum number of worker threads
- `get_performance_metrics()`: Get processing performance metrics

### DependencyCache

Intelligent caching system for dependency data.

#### Key Methods

- `new() -> DependencyCache`: Create a new cache
- `get_or_compute(key, compute_fn)`: Get cached value or compute
- `invalidate(key)`: Invalidate cached entry
- `clear()`: Clear all cached data
- `get_stats()`: Get cache statistics

## Metrics and Monitoring

### Prometheus Metrics

The system exposes the following Prometheus metrics:

- `dependency_health_status`: Current health status of dependencies
- `dependency_response_time`: Response time of dependencies
- `dependency_availability`: Availability percentage of dependencies
- `dependency_error_rate`: Error rate of dependencies
- `impact_analysis_score`: Business impact scores
- `validation_errors`: Number of validation errors
- `security_vulnerabilities`: Number of security vulnerabilities
- `prediction_accuracy`: Accuracy of health predictions
- `cache_hit_rate`: Cache hit rate percentage
- `parallel_processing_duration`: Parallel processing duration

### Health Metrics

Comprehensive health metrics are collected for each dependency:

- **Response Time**: HTTP response time in milliseconds
- **Availability**: Service availability percentage (0.0 to 1.0)
- **Error Rate**: Error rate percentage (0.0 to 1.0)
- **Throughput**: Requests per second
- **CPU Usage**: CPU utilization percentage
- **Memory Usage**: Memory utilization percentage
- **Network Latency**: Network latency in milliseconds
- **Disk Usage**: Disk utilization percentage

### Performance Metrics

- **Processing Time**: Time taken for various operations
- **Memory Usage**: Memory consumption patterns
- **Cache Performance**: Cache hit/miss ratios
- **Parallel Efficiency**: Parallel processing efficiency
- **Database Performance**: Database query performance

## Examples

The module includes several comprehensive examples:

- **basic_usage.rs**: Basic dependency management operations
- **enhanced_usage.rs**: Advanced features including version resolution and predictive analytics
- **health_monitoring.rs**: Health monitoring and alerting
- **impact_analysis.rs**: Business impact analysis
- **unit_tests.rs**: Unit testing examples

Run examples with:

```bash
cargo run --example basic_usage
cargo run --example enhanced_usage
cargo run --example health_monitoring
cargo run --example impact_analysis
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Documentation

```bash
cargo doc --open
```

### Code Quality

```bash
cargo clippy
cargo fmt
```

### Performance Testing

```bash
cargo bench
```

## Dependencies

### Core Dependencies

- **serde**: Serialization/deserialization
- **tokio**: Async runtime with full features
- **petgraph**: Graph data structures
- **reqwest**: HTTP client for API calls
- **sqlx**: Database operations (SQLite, PostgreSQL, MySQL)
- **prometheus**: Metrics collection
- **tracing**: Logging and tracing
- **semver**: Semantic versioning
- **validator**: Data validation
- **chrono**: Date/time handling
- **uuid**: Unique identifier generation
- **anyhow**: Error handling
- **thiserror**: Custom error types

### Optional Dependencies

- **tokio-tungstenite**: WebSocket support for real-time updates
- **jsonschema**: JSON schema validation
- **dot**: Graph visualization
- **colored**: Terminal color output
- **indicatif**: Progress indicators
- **walkdir**: Directory traversal
- **regex**: Regular expressions
- **lazy_static**: Lazy static initialization
- **sha2**: Cryptographic hashing
- **hmac**: HMAC for security
- **rand**: Random number generation
- **time**: Time utilities
- **num_cpus**: CPU core detection
- **futures-util**: Future utilities

## Development Status

### ✅ Completed Features
- Core dependency management system
- Semantic dependency types
- Health monitoring and metrics
- Impact analysis engine
- Validation system
- Security scanning
- Predictive analytics
- Real-time monitoring
- Parallel processing
- Intelligent caching
- Storage system (SQLite, PostgreSQL, MySQL)
- Advanced analysis capabilities
- Integration layer
- User experience features

### 🔄 In Progress
- Enhanced ML models for prediction
- Advanced visualization features
- Enterprise-grade security features
- Performance optimizations

### 📋 Planned Features
- AI-powered dependency recommendations
- Advanced graph analytics
- Machine learning model training
- Enterprise integrations
- Cloud-native features

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite: `cargo test`
6. Ensure code quality: `cargo clippy && cargo fmt`
7. Submit a pull request

## License

Apache 2.0 License - see LICENSE file for details. 