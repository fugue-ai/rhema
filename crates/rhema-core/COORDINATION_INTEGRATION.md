# Coordination Client Integration with Rhema Core

This document describes the integration of the coordination client with Rhema Core, enabling agents to communicate and coordinate their activities through a unified interface.

## Overview

The coordination client integration provides a bridge between Rhema Core and the coordination system, allowing agents to:

- Register and unregister with the coordination system
- Send and receive messages between agents
- Create and participate in coordination sessions
- Monitor connection health and statistics
- Handle errors gracefully with proper error types

## Architecture

### Core Components

1. **CoordinationManager**: High-level manager that provides a unified interface for coordination operations
2. **CoordinationClient**: Trait defining the interface for coordination clients
3. **GrpcCoordinationClientBridge**: Implementation that bridges to the gRPC coordination client
4. **Configuration**: Comprehensive configuration system for coordination settings

### Integration Points

- **Error Handling**: Integration with `RhemaError` for consistent error handling
- **Configuration**: Integration with Rhema's configuration system
- **Logging**: Integration with Rhema's logging system via tracing
- **Serialization**: Full serde support for configuration and data structures

## Usage

### Basic Setup

```rust
use rhema_core::{
    coordination::{CoordinationConfig, create_coordination_manager},
    RhemaResult,
};

#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Create coordination configuration
    let config = CoordinationConfig {
        enabled: true,
        server_endpoint: "http://localhost:50051".to_string(),
        timeout_seconds: 30,
        ..Default::default()
    };

    // Create coordination manager
    let mut manager = create_coordination_manager(config).await?;

    // Use the manager for coordination operations
    // ...
    
    Ok(())
}
```

### Agent Registration

```rust
use rhema_core::coordination::AgentInfo;

// Create agent information
let agent = AgentInfo::new("my-agent".to_string(), "code-review".to_string())
    .with_scope("backend".to_string())
    .with_capability("code-review".to_string())
    .with_capability("security-analysis".to_string())
    .with_metadata("version".to_string(), "1.0.0".to_string());

// Register the agent
manager.register_agent(agent).await?;
```

### Message Sending

```rust
use rhema_core::coordination::{AgentMessage, MessageType, MessagePriority};

// Create a message
let message = AgentMessage::new(
    "sender-agent-id".to_string(),
    MessageType::TaskAssignment,
    "Please review the authentication module".to_string(),
)
.to_recipients(vec!["recipient-agent-id".to_string()])
.with_priority(MessagePriority::High)
.with_metadata("task_id".to_string(), "auth-review-001".to_string());

// Send the message
manager.send_message(message).await?;
```

### Session Management

```rust
// Create a coordination session
let session_topic = "authentication-system-deployment".to_string();
let participants = vec!["agent-1".to_string(), "agent-2".to_string(), "agent-3".to_string()];

// This would create a session if coordination was enabled
// For now, we just demonstrate the API
println!("Session Topic: {}", session_topic);
println!("Participants: {}", participants.join(", "));
```

### Connection Monitoring

```rust
// Get connection statistics
let stats = manager.get_connection_stats().await;
println!("Connected: {}", stats.is_connected);
println!("Messages Sent: {}", stats.messages_sent);
println!("Messages Received: {}", stats.messages_received);
println!("Uptime: {} seconds", stats.uptime_seconds);

if let Some(latency) = stats.latency_ms {
    println!("Latency: {} ms", latency);
}
```

## Configuration

### CoordinationConfig

The main configuration structure for coordination:

```rust
pub struct CoordinationConfig {
    pub enabled: bool,                    // Whether coordination is enabled
    pub server_endpoint: String,          // gRPC server endpoint
    pub timeout_seconds: u64,             // Client timeout
    pub retry_config: RetryConfig,        // Retry configuration
    pub health_check_config: HealthCheckConfig, // Health check configuration
    pub tls_config: Option<TlsConfig>,    // TLS configuration
}
```

### RetryConfig

Configuration for retry behavior:

```rust
pub struct RetryConfig {
    pub max_retries: u32,                 // Maximum number of retries
    pub initial_delay_ms: u64,            // Initial backoff delay
    pub max_delay_ms: u64,                // Maximum backoff delay
    pub backoff_multiplier: f64,          // Backoff multiplier
}
```

### HealthCheckConfig

Configuration for health checks:

```rust
pub struct HealthCheckConfig {
    pub enabled: bool,                    // Whether health checks are enabled
    pub interval_seconds: u64,            // Health check interval
    pub timeout_seconds: u64,             // Health check timeout
}
```

## Data Structures

### AgentInfo

Represents agent information for coordination:

```rust
pub struct AgentInfo {
    pub id: String,                       // Unique agent identifier
    pub name: String,                     // Agent name
    pub agent_type: String,               // Agent type
    pub current_task_id: Option<String>,  // Current task ID
    pub assigned_scope: Option<String>,   // Assigned scope
    pub capabilities: Vec<String>,        // Agent capabilities
    pub last_heartbeat: Option<DateTime<Utc>>, // Last heartbeat
    pub metadata: HashMap<String, String>, // Agent metadata
}
```

### AgentMessage

Represents messages between agents:

```rust
pub struct AgentMessage {
    pub id: String,                       // Message ID
    pub sender_id: String,                // Sender agent ID
    pub recipient_ids: Vec<String>,       // Recipient agent IDs
    pub message_type: MessageType,        // Message type
    pub priority: MessagePriority,        // Message priority
    pub content: String,                  // Message content
    pub metadata: HashMap<String, String>, // Message metadata
    pub timestamp: DateTime<Utc>,         // Timestamp
}
```

### MessageType

Supported message types:

- `TaskAssignment`: Task assignment messages
- `TaskCompletion`: Task completion notifications
- `TaskFailure`: Task failure notifications
- `StatusUpdate`: Status update messages
- `Heartbeat`: Heartbeat messages
- `CoordinationRequest`: Coordination requests
- `CoordinationResponse`: Coordination responses
- `ErrorNotification`: Error notifications
- `Custom(String)`: Custom message types

### MessagePriority

Message priority levels:

- `Low`: Low priority messages
- `Normal`: Normal priority messages
- `High`: High priority messages
- `Critical`: Critical priority messages

## Error Handling

The integration provides comprehensive error handling through the `RhemaError` type:

```rust
// Coordination-specific errors
RhemaError::CoordinationError(String)    // General coordination errors
RhemaError::ConstraintError(String)      // Constraint violation errors
RhemaError::TaskScoringError(String)     // Task scoring errors
RhemaError::ConflictPreventionError(String) // Conflict prevention errors
```

## Testing

### Unit Tests

The integration includes comprehensive unit tests covering:

- Configuration creation and validation
- Agent information creation and manipulation
- Message creation and manipulation
- Error handling scenarios
- Serialization and deserialization

### Integration Tests

Integration tests cover:

- Coordination manager lifecycle
- Agent registration workflows
- Message sending workflows
- Session management
- Connection monitoring
- Error handling with actual network scenarios

### Running Tests

```bash
# Run unit tests
cargo test --package rhema-core

# Run integration tests
cargo test --test coordination_integration_test

# Run with logging
RUST_LOG=debug cargo test --package rhema-core
```

## Examples

### Complete Example

See `examples/coordination_integration_example.rs` for a complete example demonstrating:

1. Configuration setup
2. Agent registration
3. Message sending
4. Session creation
5. Connection monitoring
6. Error handling

### Running the Example

```bash
cargo run --example coordination_integration_example
```

## Integration with Existing Systems

### Rhema Agents

The coordination client integrates seamlessly with Rhema agents:

```rust
use rhema_core::coordination::{CoordinationManager, AgentInfo};

// In your agent implementation
pub struct MyAgent {
    coordination_manager: CoordinationManager,
    agent_info: AgentInfo,
}

impl MyAgent {
    pub async fn new() -> RhemaResult<Self> {
        let config = CoordinationConfig::default();
        let manager = create_coordination_manager(config).await?;
        
        let agent_info = AgentInfo::new("my-agent".to_string(), "my-type".to_string());
        
        Ok(Self {
            coordination_manager: manager,
            agent_info,
        })
    }
    
    pub async fn start(&self) -> RhemaResult<()> {
        // Register with coordination system
        self.coordination_manager.register_agent(self.agent_info.clone()).await?;
        
        // Start coordination activities
        // ...
        
        Ok(())
    }
}
```

### Configuration Integration

The coordination configuration can be integrated with Rhema's configuration system:

```rust
use rhema_core::coordination::CoordinationConfig;

// Load from configuration file
let config: CoordinationConfig = serde_yaml::from_str(&config_content)?;

// Or create programmatically
let config = CoordinationConfig {
    enabled: true,
    server_endpoint: "http://localhost:50051".to_string(),
    timeout_seconds: 30,
    ..Default::default()
};
```

## Performance Considerations

### Connection Management

- The coordination client manages connections efficiently
- Automatic reconnection on connection failures
- Connection pooling for better performance
- Health monitoring to detect connection issues

### Message Handling

- Asynchronous message processing
- Message queuing for high-throughput scenarios
- Priority-based message routing
- Message compression for large payloads

### Resource Management

- Automatic cleanup of unused resources
- Memory-efficient data structures
- Configurable timeouts to prevent resource leaks
- Graceful shutdown procedures

## Security

### TLS Support

The coordination client supports TLS for secure communication:

```rust
let config = CoordinationConfig {
    enabled: true,
    server_endpoint: "https://localhost:50051".to_string(),
    tls_config: Some(TlsConfig {
        ca_cert_path: "ca.crt".to_string(),
        client_cert_path: "client.crt".to_string(),
        client_key_path: "client.key".to_string(),
    }),
    ..Default::default()
};
```

### Authentication

Authentication can be implemented through:

- TLS client certificates
- API keys in message metadata
- JWT tokens in headers
- Custom authentication mechanisms

## Monitoring and Observability

### Metrics

The coordination client provides comprehensive metrics:

- Connection health and status
- Message throughput and latency
- Error rates and types
- Resource usage statistics

### Logging

Integration with Rhema's logging system:

```rust
use tracing::{info, warn, error};

// Log coordination events
info!("Agent registered: {}", agent_info.name);
warn!("Connection lost, attempting reconnection...");
error!("Failed to send message: {}", error);
```

### Health Checks

Automatic health checking with configurable intervals:

```rust
let config = CoordinationConfig {
    health_check_config: HealthCheckConfig {
        enabled: true,
        interval_seconds: 30,
        timeout_seconds: 5,
    },
    ..Default::default()
};
```

## Future Enhancements

### Planned Features

1. **Streaming Support**: Bidirectional streaming for real-time updates
2. **Advanced Sessions**: Multi-party sessions with consensus algorithms
3. **Load Balancing**: Client-side load balancing for multiple servers
4. **Message Persistence**: Persistent message storage and replay
5. **Advanced Security**: End-to-end encryption and advanced authentication

### Extension Points

The integration is designed to be extensible:

- Custom message types can be added
- New coordination protocols can be implemented
- Custom authentication mechanisms can be plugged in
- Monitoring and metrics can be extended

## Troubleshooting

### Common Issues

1. **Connection Failures**: Check server endpoint and network connectivity
2. **Authentication Errors**: Verify TLS certificates and credentials
3. **Message Delivery Failures**: Check recipient agent IDs and network status
4. **Performance Issues**: Monitor connection statistics and adjust timeouts

### Debugging

Enable debug logging for detailed information:

```bash
RUST_LOG=debug cargo run --example coordination_integration_example
```

### Error Recovery

The coordination client includes automatic error recovery:

- Automatic reconnection on connection failures
- Retry logic for failed operations
- Circuit breaker protection against cascading failures
- Graceful degradation when coordination is unavailable

## Contributing

When contributing to the coordination client integration:

1. Follow Rust coding standards
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Ensure backward compatibility
5. Add examples for new functionality

## License

This integration is part of the Rhema project and is licensed under the Apache License 2.0.
