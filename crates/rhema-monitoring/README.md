# Rhema Monitoring Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-monitoring)](https://crates.io/crates/rhema-monitoring)
[![Documentation](https://docs.rs/rhema-monitoring/badge.svg)](https://docs.rs/rhema-monitoring)

Monitoring, observability, and performance tracking for Rhema, providing comprehensive system health and performance insights.

## Overview

The `rhema-monitoring` crate provides comprehensive monitoring and observability capabilities for Rhema, including performance tracking, health monitoring, metrics collection, and alerting. It ensures that Rhema systems are healthy, performant, and observable.

## Features

### 📊 Performance Monitoring
- **Performance Metrics**: Track response times, throughput, and resource usage
- **Performance Analysis**: Analyze performance patterns and bottlenecks
- **Performance Optimization**: Identify optimization opportunities
- **Performance Reporting**: Generate performance reports and dashboards

### 🏥 Health Monitoring
- **System Health Checks**: Comprehensive health check system
- **Component Health**: Monitor individual component health
- **Dependency Health**: Monitor external dependency health
- **Health Reporting**: Real-time health status reporting

### 📈 Metrics Collection
- **Custom Metrics**: Define and collect custom application metrics
- **System Metrics**: Collect system-level metrics (CPU, memory, disk)
- **Business Metrics**: Track business-relevant metrics
- **Metrics Aggregation**: Aggregate metrics across time and dimensions

### 🚨 Alerting and Notifications
- **Alert Rules**: Configurable alert rules and thresholds
- **Alert Channels**: Multiple alert notification channels
- **Alert Escalation**: Alert escalation and routing
- **Alert History**: Track alert history and resolution

### 🔍 Observability
- **Distributed Tracing**: Trace requests across system components
- **Log Aggregation**: Centralized log collection and analysis
- **Error Tracking**: Track and analyze errors and exceptions
- **Debugging Tools**: Tools for debugging and troubleshooting

### 📋 Dashboards and Visualization
- **Real-time Dashboards**: Real-time monitoring dashboards
- **Historical Data**: Historical data visualization and analysis
- **Custom Dashboards**: Create custom monitoring dashboards
- **Export Capabilities**: Export monitoring data and reports

## Architecture

```
rhema-monitoring/
├── monitoring.rs     # Core monitoring functionality
├── performance.rs    # Performance monitoring
├── health.rs         # Health monitoring
├── metrics.rs        # Metrics collection
├── alerting.rs       # Alerting and notifications
├── tracing.rs        # Distributed tracing
└── dashboards.rs     # Dashboard and visualization
```

## Usage

### Performance Monitoring

```rust
use rhema_monitoring::performance::PerformanceMonitor;

let monitor = PerformanceMonitor::new();

// Track operation performance
let operation = monitor.track_operation("query_execution");
let result = perform_query();
operation.complete();

// Get performance statistics
let stats = monitor.get_statistics()?;
println!("Average response time: {:?}", stats.avg_response_time);
```

### Health Monitoring

```rust
use rhema_monitoring::health::HealthMonitor;

let health_monitor = HealthMonitor::new();

// Register health checks
health_monitor.register_check("database", check_database_health)?;
health_monitor.register_check("cache", check_cache_health)?;

// Run health checks
let health_status = health_monitor.check_health().await?;

if health_status.is_healthy() {
    println!("System is healthy");
} else {
    println!("System has issues: {:?}", health_status.issues());
}
```

### Metrics Collection

```rust
use rhema_monitoring::metrics::MetricsCollector;

let collector = MetricsCollector::new();

// Record custom metrics
collector.record_counter("queries_total", 1)?;
collector.record_gauge("active_connections", 42)?;
collector.record_histogram("query_duration", 150.0)?;

// Get metrics
let metrics = collector.get_metrics()?;
```

### Alerting

```rust
use rhema_monitoring::alerting::AlertManager;

let alert_manager = AlertManager::new();

// Configure alert rules
alert_manager.add_rule(AlertRule {
    name: "high_error_rate",
    condition: "error_rate > 0.05",
    severity: AlertSeverity::Critical,
    channels: vec!["email", "slack"],
})?;

// Send alerts
alert_manager.send_alert("high_error_rate", "Error rate is 7%")?;
```

### Distributed Tracing

```rust
use rhema_monitoring::tracing::Tracer;

let tracer = Tracer::new();

// Start a trace
let span = tracer.start_span("user_authentication");

// Add context to span
span.set_attribute("user_id", "12345");
span.set_attribute("auth_method", "jwt");

// End span
span.end();
```

## Configuration

### Monitoring Configuration

```yaml
# .rhema/monitoring.yaml
monitoring:
  performance:
    enabled: true
    sampling_rate: 0.1
    retention_days: 30
    
  health:
    enabled: true
    check_interval: 30s
    timeout: 10s
    
  metrics:
    enabled: true
    collection_interval: 60s
    retention_days: 90
    
  alerting:
    enabled: true
    channels:
      email:
        smtp_server: "smtp.example.com"
        from: "alerts@example.com"
      slack:
        webhook_url: "https://hooks.slack.com/..."
```

### Alert Rules Configuration

```yaml
monitoring:
  alert_rules:
    - name: "high_error_rate"
      condition: "error_rate > 0.05"
      severity: "critical"
      channels: ["email", "slack"]
      
    - name: "high_latency"
      condition: "avg_response_time > 1000ms"
      severity: "warning"
      channels: ["slack"]
      
    - name: "low_disk_space"
      condition: "disk_usage > 90%"
      severity: "critical"
      channels: ["email", "slack", "pagerduty"]
```

### Dashboard Configuration

```yaml
monitoring:
  dashboards:
    - name: "System Overview"
      refresh_interval: 30s
      panels:
        - title: "Response Time"
          type: "line"
          query: "avg(response_time)"
          
        - title: "Error Rate"
          type: "gauge"
          query: "error_rate"
          
        - title: "Active Connections"
          type: "stat"
          query: "active_connections"
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **prometheus**: Metrics collection
- **tracing**: Distributed tracing
- **tokio**: Async runtime
- **serde**: Serialization support
- **chrono**: Date and time handling

## Development Status

### ✅ Completed Features
- Basic monitoring framework
- Performance tracking infrastructure
- Health check system
- Metrics collection

### 🔄 In Progress
- Advanced alerting system
- Distributed tracing
- Dashboard implementation
- Performance optimization

### 📋 Planned Features
- Advanced visualization
- Machine learning insights
- Predictive monitoring
- Enterprise features

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all monitoring operations are properly tested
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 