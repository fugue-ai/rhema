# Security and Performance Features

This document provides comprehensive documentation for the security and performance features implemented in the Rhema gRPC coordination client.

## Table of Contents

1. [Security Features](#security-features)
2. [Performance Features](#performance-features)
3. [Configuration](#configuration)
4. [Usage Examples](#usage-examples)
5. [Best Practices](#best-practices)
6. [Troubleshooting](#troubleshooting)

## Security Features

### TLS Encryption

The gRPC client supports Transport Layer Security (TLS) for encrypted communication between clients and servers.

#### Configuration

```rust
use rhema_coordination::grpc::{SecurityConfig, SyneidesisConfig};

let security_config = SecurityConfig {
    enable_tls: true,
    ca_cert_path: Some("certs/ca.pem".to_string()),
    client_cert_path: Some("certs/client.pem".to_string()),
    client_key_path: Some("certs/client-key.pem".to_string()),
    skip_cert_verification: false, // Set to true for development only
    ..Default::default()
};

let mut config = SyneidesisConfig::default();
config.security = security_config;
```

#### Certificate Management

- **CA Certificate**: Root certificate authority for server verification
- **Client Certificate**: Client identity certificate for mutual TLS
- **Client Key**: Private key for client certificate
- **Certificate Verification**: Automatic validation of server certificates

#### Development vs Production

```rust
// Development (skip certificate verification)
let dev_config = SecurityConfig {
    enable_tls: true,
    skip_cert_verification: true, // ⚠️ Only for development
    ..Default::default()
};

// Production (full certificate validation)
let prod_config = SecurityConfig {
    enable_tls: true,
    ca_cert_path: Some("/etc/ssl/certs/ca.pem".to_string()),
    client_cert_path: Some("/etc/ssl/certs/client.pem".to_string()),
    client_key_path: Some("/etc/ssl/private/client-key.pem".to_string()),
    skip_cert_verification: false, // ✅ Always validate certificates
    ..Default::default()
};
```

### Authentication

#### JWT Token Authentication

The client supports JSON Web Token (JWT) authentication with automatic token refresh.

```rust
let security_config = SecurityConfig {
    jwt_secret: Some("your-secret-key".to_string()),
    token_refresh_interval: 3600, // Refresh every hour
    ..Default::default()
};
```

#### Static Token Authentication

For simple authentication scenarios, you can use static tokens.

```rust
let security_config = SecurityConfig {
    auth_token: Some("your-static-token".to_string()),
    ..Default::default()
};
```

#### Token Management

- **Automatic Refresh**: JWT tokens are automatically refreshed before expiration
- **Token Validation**: Server-side token validation with configurable secrets
- **Secure Storage**: Tokens are stored in memory with proper cleanup

## Performance Features

### Connection Pooling

Connection pooling improves performance by reusing connections instead of creating new ones for each request.

#### Configuration

```rust
use rhema_coordination::grpc::{PerformanceConfig, CompressionAlgorithm};

let performance_config = PerformanceConfig {
    enable_connection_pooling: true,
    max_connections: 20,
    pool_timeout: 30, // seconds
    ..Default::default()
};
```

#### Pool Management

- **Connection Reuse**: Efficiently reuse established connections
- **Pool Size**: Configurable maximum number of connections
- **Timeout Handling**: Automatic cleanup of idle connections
- **Load Balancing**: Distribute requests across available connections

### Message Compression

The client supports multiple compression algorithms to reduce network bandwidth usage.

#### Supported Algorithms

```rust
use rhema_coordination::grpc::CompressionAlgorithm;

// Gzip compression (good balance of speed and compression)
let gzip_config = PerformanceConfig {
    enable_compression: true,
    compression_algorithm: CompressionAlgorithm::Gzip,
    compression_level: 6, // 0-9, higher = more compression
    ..Default::default()
};

// Brotli compression (excellent compression ratio)
let brotli_config = PerformanceConfig {
    enable_compression: true,
    compression_algorithm: CompressionAlgorithm::Brotli,
    compression_level: 8, // 0-11, higher = more compression
    ..Default::default()
};

// Zstd compression (fast with good compression)
let zstd_config = PerformanceConfig {
    enable_compression: true,
    compression_algorithm: CompressionAlgorithm::Zstd,
    compression_level: 6, // 0-22, higher = more compression
    ..Default::default()
};
```

#### Compression Levels

| Algorithm | Level Range | Recommended | Use Case |
|-----------|-------------|-------------|----------|
| Gzip | 0-9 | 6 | General purpose |
| Brotli | 0-11 | 8 | Web content |
| Zstd | 0-22 | 6 | High-performance |

### Keep-Alive Connections

Keep-alive connections maintain persistent connections to reduce connection overhead.

```rust
let performance_config = PerformanceConfig {
    enable_keep_alive: true,
    keep_alive_interval: 30, // seconds
    keep_alive_timeout: 5,   // seconds
    ..Default::default()
};
```

### Message Size Limits

Configure maximum message sizes to prevent memory issues and ensure reliable communication.

```rust
let performance_config = PerformanceConfig {
    max_message_size: 16 * 1024 * 1024, // 16MB
    request_timeout: 120, // seconds
    ..Default::default()
};
```

## Configuration

### Complete Configuration Example

```rust
use rhema_coordination::grpc::{
    SyneidesisConfig, SecurityConfig, PerformanceConfig, CompressionAlgorithm
};

let security_config = SecurityConfig {
    enable_tls: true,
    ca_cert_path: Some("certs/ca.pem".to_string()),
    client_cert_path: Some("certs/client.pem".to_string()),
    client_key_path: Some("certs/client-key.pem".to_string()),
    skip_cert_verification: false,
    auth_token: Some("your-auth-token".to_string()),
    jwt_secret: Some("your-jwt-secret".to_string()),
    token_refresh_interval: 3600,
};

let performance_config = PerformanceConfig {
    enable_connection_pooling: true,
    max_connections: 20,
    pool_timeout: 30,
    enable_compression: true,
    compression_algorithm: CompressionAlgorithm::Gzip,
    compression_level: 6,
    enable_keep_alive: true,
    keep_alive_interval: 30,
    keep_alive_timeout: 5,
    max_message_size: 8 * 1024 * 1024, // 8MB
    request_timeout: 60,
};

let mut config = SyneidesisConfig::default();
config.enabled = true;
config.server_address = Some("https://localhost:50051".to_string());
config.security = security_config;
config.performance = performance_config;
config.enable_health_monitoring = true;
config.enable_metrics = true;
```

### Configuration Validation

The client validates configuration settings and provides helpful error messages for invalid configurations.

```rust
match SyneidesisCoordinationClient::new(config).await {
    Ok(client) => {
        println!("✅ Client created successfully");
    }
    Err(e) => {
        eprintln!("❌ Configuration error: {}", e);
    }
}
```

## Usage Examples

### Basic Secure Client

```rust
use rhema_coordination::grpc::{SyneidesisCoordinationClient, SyneidesisConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("https://localhost:50051".to_string());
    
    // Enable TLS
    config.security.enable_tls = true;
    config.security.ca_cert_path = Some("certs/ca.pem".to_string());
    
    // Enable compression
    config.performance.enable_compression = true;
    config.performance.compression_algorithm = CompressionAlgorithm::Gzip;
    
    let client = SyneidesisCoordinationClient::new(config).await?;
    
    // Use the client...
    Ok(())
}
```

### High-Performance Client

```rust
use rhema_coordination::grpc::{
    SyneidesisCoordinationClient, SyneidesisConfig, PerformanceConfig, CompressionAlgorithm
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let performance_config = PerformanceConfig {
        enable_connection_pooling: true,
        max_connections: 50,
        pool_timeout: 60,
        enable_compression: true,
        compression_algorithm: CompressionAlgorithm::Zstd,
        compression_level: 6,
        enable_keep_alive: true,
        keep_alive_interval: 30,
        keep_alive_timeout: 5,
        max_message_size: 16 * 1024 * 1024, // 16MB
        request_timeout: 120,
    };

    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("http://localhost:50051".to_string());
    config.performance = performance_config;
    
    let client = SyneidesisCoordinationClient::new(config).await?;
    
    // Use the client for high-performance operations...
    Ok(())
}
```

### Production-Ready Client

```rust
use rhema_coordination::grpc::{
    SyneidesisCoordinationClient, SyneidesisConfig, SecurityConfig, PerformanceConfig,
    CompressionAlgorithm, CoordinationMonitor, MonitoringConfig, LoggingAlertHandler
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Security configuration
    let security_config = SecurityConfig {
        enable_tls: true,
        ca_cert_path: Some("/etc/ssl/certs/ca.pem".to_string()),
        client_cert_path: Some("/etc/ssl/certs/client.pem".to_string()),
        client_key_path: Some("/etc/ssl/private/client-key.pem".to_string()),
        skip_cert_verification: false,
        jwt_secret: Some(std::env::var("JWT_SECRET")?),
        token_refresh_interval: 3600,
        ..Default::default()
    };

    // Performance configuration
    let performance_config = PerformanceConfig {
        enable_connection_pooling: true,
        max_connections: 20,
        pool_timeout: 30,
        enable_compression: true,
        compression_algorithm: CompressionAlgorithm::Brotli,
        compression_level: 8,
        enable_keep_alive: true,
        keep_alive_interval: 30,
        keep_alive_timeout: 5,
        max_message_size: 8 * 1024 * 1024,
        request_timeout: 60,
    };

    // Monitoring configuration
    let monitoring_config = MonitoringConfig {
        enable_health_monitoring: true,
        health_check_interval: 30,
        enable_performance_monitoring: true,
        performance_check_interval: 60,
        enable_connection_monitoring: true,
        connection_check_interval: 15,
        enable_alerting: true,
        performance_thresholds: PerformanceThresholds {
            max_response_time_ms: 1000,
            max_error_rate: 0.05,
            max_connection_failures: 3,
            min_throughput_ops_per_sec: 10,
        },
        health_thresholds: HealthThresholds {
            max_latency_ms: 500,
            min_success_rate: 0.95,
            max_consecutive_failures: 2,
        },
    };

    // Create client configuration
    let mut config = SyneidesisConfig::default();
    config.enabled = true;
    config.server_address = Some("https://coordination.example.com:50051".to_string());
    config.security = security_config;
    config.performance = performance_config;
    config.enable_health_monitoring = true;
    config.enable_metrics = true;

    // Create monitoring instance
    let monitor = CoordinationMonitor::new(
        monitoring_config,
        Arc::new(ClientMetrics::new()),
        Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
    );

    // Add alert handlers
    monitor.add_alert_handler(Box::new(LoggingAlertHandler::new()));
    
    // Start monitoring
    monitor.start().await;

    // Create client
    let client = SyneidesisCoordinationClient::new(config).await?;
    
    // Use the client...
    
    // Graceful shutdown
    client.shutdown().await;
    monitor.stop().await;
    
    Ok(())
}
```

## Best Practices

### Security Best Practices

1. **Always use TLS in production**
   ```rust
   config.security.enable_tls = true;
   config.security.skip_cert_verification = false; // Never skip in production
   ```

2. **Use strong JWT secrets**
   ```rust
   config.security.jwt_secret = Some(std::env::var("JWT_SECRET")?);
   ```

3. **Rotate certificates regularly**
   - Implement certificate rotation procedures
   - Monitor certificate expiration dates
   - Use automated certificate management

4. **Validate all inputs**
   - Always validate server addresses
   - Check certificate paths exist
   - Validate configuration parameters

### Performance Best Practices

1. **Choose appropriate compression**
   ```rust
   // For general use
   config.performance.compression_algorithm = CompressionAlgorithm::Gzip;
   
   // For web content
   config.performance.compression_algorithm = CompressionAlgorithm::Brotli;
   
   // For high-performance scenarios
   config.performance.compression_algorithm = CompressionAlgorithm::Zstd;
   ```

2. **Size connection pools appropriately**
   ```rust
   // For low-traffic applications
   config.performance.max_connections = 5;
   
   // For high-traffic applications
   config.performance.max_connections = 50;
   ```

3. **Monitor performance metrics**
   ```rust
   config.enable_metrics = true;
   config.enable_health_monitoring = true;
   ```

4. **Set appropriate timeouts**
   ```rust
   config.performance.request_timeout = 60; // 60 seconds
   config.performance.keep_alive_timeout = 5; // 5 seconds
   ```

### Monitoring Best Practices

1. **Set realistic thresholds**
   ```rust
   let thresholds = PerformanceThresholds {
       max_response_time_ms: 1000, // 1 second
       max_error_rate: 0.05,       // 5%
       max_connection_failures: 3,
       min_throughput_ops_per_sec: 10,
   };
   ```

2. **Use multiple alert handlers**
   ```rust
   monitor.add_alert_handler(Box::new(LoggingAlertHandler::new()));
   monitor.add_alert_handler(Box::new(WebhookAlertHandler::new(url, secret)));
   monitor.add_alert_handler(Box::new(SlackAlertHandler::new(webhook_url, channel)));
   ```

3. **Monitor key metrics**
   - Response times
   - Error rates
   - Connection health
   - Throughput

## Troubleshooting

### Common Issues

#### TLS Connection Failures

**Problem**: TLS handshake fails
```rust
// Check certificate paths
config.security.ca_cert_path = Some("certs/ca.pem".to_string());
config.security.client_cert_path = Some("certs/client.pem".to_string());
config.security.client_key_path = Some("certs/client-key.pem".to_string());

// Verify certificates exist
use std::path::Path;
assert!(Path::new("certs/ca.pem").exists());
assert!(Path::new("certs/client.pem").exists());
assert!(Path::new("certs/client-key.pem").exists());
```

**Solution**: Verify certificate paths and permissions

#### Compression Issues

**Problem**: Compression not working
```rust
// Enable compression
config.performance.enable_compression = true;
config.performance.compression_algorithm = CompressionAlgorithm::Gzip;
config.performance.compression_level = 6;
```

**Solution**: Ensure compression is enabled and server supports it

#### Connection Pool Exhaustion

**Problem**: No connections available in pool
```rust
// Increase pool size
config.performance.max_connections = 50;
config.performance.pool_timeout = 60;
```

**Solution**: Increase pool size or reduce connection usage

#### Performance Degradation

**Problem**: Slow response times
```rust
// Optimize performance settings
config.performance.enable_compression = true;
config.performance.enable_keep_alive = true;
config.performance.keep_alive_interval = 30;
config.performance.request_timeout = 120;
```

**Solution**: Enable compression and keep-alive, increase timeouts

### Debugging

#### Enable Debug Logging

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_env_filter("debug")
    .init();
```

#### Monitor Metrics

```rust
let metrics = client.get_metrics();
println!("Total requests: {}", metrics.total_requests.load(Ordering::Relaxed));
println!("Successful requests: {}", metrics.successful_requests.load(Ordering::Relaxed));
println!("Failed requests: {}", metrics.failed_requests.load(Ordering::Relaxed));
println!("Average response time: {}ms", metrics.average_response_time_ms.load(Ordering::Relaxed));
```

#### Health Checks

```rust
match client.health_check().await {
    Ok(health) => println!("Health: {:?}", health),
    Err(e) => println!("Health check failed: {}", e),
}
```

## Performance Benchmarks

### Compression Comparison

| Algorithm | Compression Ratio | Speed | CPU Usage | Use Case |
|-----------|------------------|-------|-----------|----------|
| Gzip | 2.5:1 | Medium | Low | General purpose |
| Brotli | 3.5:1 | Slow | High | Web content |
| Zstd | 3.0:1 | Fast | Medium | High-performance |

### Connection Pooling Impact

| Pool Size | Latency | Throughput | Memory Usage |
|-----------|---------|------------|--------------|
| 1 | High | Low | Low |
| 10 | Medium | Medium | Medium |
| 50 | Low | High | High |

### TLS Overhead

| Feature | Latency Impact | Throughput Impact | Security Benefit |
|---------|----------------|-------------------|------------------|
| TLS 1.3 | +5-10ms | -5-10% | High |
| Certificate Validation | +1-2ms | -1-2% | High |
| Client Certificates | +2-3ms | -2-3% | Very High |

## Conclusion

The security and performance features provide enterprise-grade capabilities for the Rhema gRPC coordination client. By following the best practices outlined in this document, you can build secure, high-performance coordination systems that scale to meet your application's needs.

For additional information, see:
- [API Documentation](../api/README.md)
- [Configuration Reference](../config/README.md)
- [Monitoring Guide](../monitoring/README.md)
- [Security Checklist](../security/CHECKLIST.md)
