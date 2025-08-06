# Rhema API

[![Crates.io](https://img.shields.io/crates/v/rhema-api)](https://crates.io/crates/rhema-api)
[![Documentation](https://docs.rs/rhema-api/badge.svg)](https://docs.rs/rhema-api)

Core API and integration layer for the Rhema Protocol, providing unified access to all Rhema functionality with advanced coordination, performance monitoring, and security features.

## Overview

The `rhema-api` crate serves as the main entry point and integration layer for the Rhema system. It provides a unified API that orchestrates all other Rhema crates, offering advanced features like real-time agent coordination, performance monitoring, security management, and comprehensive error handling.

## Features

### 🏗️ Core API
- **Unified API**: Single entry point for all Rhema functionality
- **Error Recovery**: Comprehensive error handling with automatic recovery
- **Input Validation**: Robust input validation and sanitization
- **Rate Limiting**: Built-in rate limiting and request throttling
- **Caching**: Intelligent caching with configurable policies

### 🤖 Agent Coordination
- **Real-Time Coordination**: Advanced agent communication system
- **Session Management**: Create and manage coordination sessions
- **Message Routing**: Intelligent message routing between agents
- **Performance Metrics**: Track agent performance and health
- **Load Balancing**: Automatic load balancing across agents
- **Fault Tolerance**: Built-in fault tolerance and recovery

### 📊 Performance & Monitoring
- **Performance Metrics**: Detailed operation performance tracking
- **Resource Management**: Memory and CPU usage monitoring
- **Performance Limits**: Configurable performance thresholds
- **Optimization**: Automatic query and operation optimization
- **Aggregated Metrics**: Statistical analysis of performance data

### 🔒 Security & Access Control
- **Input Sanitization**: Protection against injection attacks
- **Access Control**: Role-based permission system
- **Audit Logging**: Comprehensive audit trail
- **File Security**: Secure file access validation
- **Query Security**: SQL injection and XSS protection

### 🔧 Core Operations
- **Scope Management**: Discover and manage project scopes
- **Knowledge Operations**: Load and query knowledge data
- **Query Engine**: Execute CQL queries with provenance
- **Git Integration**: Seamless Git workflow integration
- **Repository Analysis**: Automatic repository structure analysis

### 🚀 Advanced Features
- **Async Operations**: Full async/await support
- **Concurrent Processing**: Parallel operation execution
- **Health Monitoring**: System and agent health tracking
- **Integration Bridge**: External system integration support
- **API Documentation**: Automatic API documentation generation

## Architecture

```
rhema-api/
├── lib.rs              # Main library entry point and API
├── main.rs             # CLI application
├── init.rs             # Repository initialization
├── performance.rs      # Performance monitoring and optimization
├── security.rs         # Security and access control
├── api_docs.rs         # API documentation generation
└── tests.rs            # Test utilities
```

## Quick Start

### Basic Usage

```rust
use rhema_api::Rhema;

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    // Initialize Rhema API
    let mut rhema = Rhema::new()?;
    
    // Discover scopes in the repository
    let scopes = rhema.discover_scopes_optimized().await?;
    println!("Found {} scopes", scopes.len());
    
    // Load knowledge from a scope
    let knowledge = rhema.load_knowledge_async("my-service").await?;
    println!("Loaded knowledge: {:?}", knowledge);
    
    // Execute a query
    let result = rhema.query("SELECT todos FROM scope('my-service') WHERE status = 'pending'")?;
    println!("Query result: {:?}", result);
    
    Ok(())
}
```

### Agent Coordination

```rust
use rhema_api::Rhema;
use rhema_ai::agent::real_time_coordination::{
    AgentInfo, AgentStatus, CoordinationConfig, AgentPerformanceMetrics
};

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    let mut rhema = Rhema::new()?;
    
    // Initialize coordination system
    let config = CoordinationConfig {
        max_message_history: 100,
        message_timeout_seconds: 30,
        heartbeat_interval_seconds: 10,
        agent_timeout_seconds: 60,
        max_session_participants: 5,
        enable_encryption: false,
        enable_compression: true,
    };
    
    rhema.init_coordination(Some(config)).await?;
    
    // Register an agent
    let agent = AgentInfo {
        id: "my-agent".to_string(),
        name: "My Agent".to_string(),
        agent_type: "worker".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "my-service".to_string(),
        capabilities: vec!["query".to_string(), "analysis".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: AgentPerformanceMetrics::default(),
    };
    
    rhema.register_agent(agent).await?;
    
    // Create a coordination session
    let session_id = rhema.create_coordination_session(
        "Task Session".to_string(),
        vec!["my-agent".to_string()]
    ).await?;
    
    // Get coordination statistics
    let stats = rhema.get_coordination_stats().await?;
    println!("Coordination stats: {:?}", stats);
    
    Ok(())
}
```

### Performance Monitoring

```rust
use rhema_api::{Rhema, PerformanceMonitor, PerformanceLimits};

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    let rhema = Rhema::new()?;
    
    // Create performance monitor
    let monitor = PerformanceMonitor::new();
    
    // Set performance limits
    let limits = PerformanceLimits {
        max_avg_execution_time_ms: 1000,
        max_execution_time_ms: 5000,
        max_memory_usage_bytes: 100 * 1024 * 1024, // 100 MB
        max_cpu_usage_percent: 80.0,
        max_error_rate: 0.05, // 5%
    };
    
    // Check performance
    let result = monitor.check_performance_limits("query_operation", &limits).await;
    if let Ok(check_result) = result {
        if !check_result.passed {
            println!("Performance violations: {:?}", check_result.violations);
        }
    }
    
    Ok(())
}
```

### Security Features

```rust
use rhema_api::{Rhema, SecurityManager, SecurityConfig};

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    let rhema = Rhema::new()?;
    
    // Create security manager
    let security_config = SecurityConfig::default();
    let security_manager = SecurityManager::new(security_config);
    
    // Validate user input
    let sanitized_input = security_manager
        .validate_input("user123", "query", "SELECT todos FROM scope('test')")
        .await?;
    
    // Validate file access
    let sanitized_path = security_manager
        .validate_file_access("user123", "knowledge.yaml")
        .await?;
    
    // Get audit log entries
    let audit_entries = security_manager.audit_logger.get_entries(
        Some("user123"), 
        Some("query")
    ).await;
    
    Ok(())
}
```

## Configuration

### Basic Configuration

```yaml
# .rhema/rhema.yaml
rhema:
  version: "1.0.0"
  api:
    rate_limit:
      requests_per_minute: 1000
      burst_size: 100
    
  coordination:
    max_message_history: 100
    message_timeout_seconds: 30
    heartbeat_interval_seconds: 10
    agent_timeout_seconds: 60
    max_session_participants: 5
    enable_encryption: false
    enable_compression: true
    
  performance:
    monitoring:
      enabled: true
      max_metrics_per_operation: 1000
    
  security:
    enable_input_sanitization: true
    enable_access_control: true
    enable_audit_logging: true
    max_file_size: 10485760  # 10 MB
```

### Advanced Coordination Configuration

```yaml
rhema:
  coordination:
    advanced:
      load_balancing:
        strategy: "round_robin"
        health_check_interval: 30
        
      fault_tolerance:
        max_retries: 3
        retry_delay_ms: 1000
        circuit_breaker_threshold: 5
        
      encryption:
        enabled: true
        algorithm: "AES-256-GCM"
        
      consensus:
        enabled: true
        quorum_size: 3
        timeout_ms: 5000
```

## CLI Usage

The `rhema-api` crate provides a command-line interface:

```bash
# Show API information
rhema-api info

# Run with verbose output
rhema-api --verbose

# Suppress output
rhema-api --quiet
```

## Examples

### Repository Initialization

```rust
use rhema_api::init_run;

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    // Initialize a new Rhema repository
    init_run().await?;
    
    println!("Repository initialized successfully!");
    Ok(())
}
```

### Error Recovery

```rust
use rhema_api::{Rhema, ApiInput};

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    let rhema = Rhema::new()?;
    
    // Create API input with error recovery
    let input = ApiInput {
        query: Some("SELECT todos FROM scope('invalid-scope')".to_string()),
        scope_name: Some("invalid-scope".to_string()),
        file_path: None,
        parameters: HashMap::new(),
    };
    
    // Handle operation with automatic error recovery
    let result = rhema.handle_operation_with_error_recovery(&input).await?;
    println!("Operation result: {:?}", result);
    
    Ok(())
}
```

## Dependencies

### Core Dependencies
- **rhema-core**: Core data structures and error handling
- **rhema-query**: Query engine and search functionality
- **rhema-git**: Git integration and workflow
- **rhema-ai**: AI service and agent coordination
- **rhema-mcp**: MCP protocol support
- **rhema-config**: Configuration management
- **rhema-monitoring**: Monitoring and observability
- **rhema-integrations**: External system integrations

### Runtime Dependencies
- **tokio**: Async runtime and concurrency
- **serde**: Serialization and deserialization
- **tracing**: Logging and tracing
- **chrono**: Date and time handling
- **uuid**: Unique identifier generation
- **regex**: Regular expression support
- **validator**: Input validation
- **reqwest**: HTTP client
- **redis**: Caching and session storage

## Development Status

### ✅ Completed Features
- Core API framework with unified interface
- Real-time agent coordination system
- Performance monitoring and optimization
- Security and access control
- Repository initialization
- Error recovery and validation
- Async operation support
- Comprehensive caching system

### 🔄 In Progress
- Advanced coordination features
- Performance optimization algorithms
- Enhanced security protocols
- API documentation generation

### 📋 Planned Features
- GraphQL API support
- WebSocket coordination
- Advanced analytics
- Enterprise integrations
- Plugin system

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific examples
cargo run --example unit_tests
cargo run --example init_unit_tests
cargo run --example simple_coordination_example
```

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all API operations are properly tested
4. Run the test suite: `cargo test`
5. Add examples for new features

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 