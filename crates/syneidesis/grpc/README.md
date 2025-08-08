# syneidesis-grpc

gRPC services for real-time agent coordination in the Syneidesis ecosystem.

## Overview

This crate provides gRPC-based communication services for agent coordination, including:

- **Server Implementation**: Complete gRPC server for the coordination service
- **Client Utilities**: High-level client for connecting to coordination services
- **Protocol Definitions**: Protobuf definitions for all coordination messages
- **Service Implementation**: Full implementation of the RealTimeCoordinationService

## Features

- **Agent Management**: Register, unregister, and manage agent states
- **Message Passing**: Send messages between agents with delivery tracking
- **Session Management**: Create and manage coordination sessions
- **Resource Management**: Request and release shared resources
- **Conflict Resolution**: Detect and resolve conflicts between agents
- **Real-time Streaming**: Stream updates and messages to agents
- **Performance Metrics**: Track agent performance and coordination statistics

## Usage

### Server

```rust
use syneidesis_grpc::{CoordinationServer, GrpcConfig};
use syneidesis_agent::config::CoordinationConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CoordinationConfig::default();
    let mut server = CoordinationServer::new(config)?;
    
    server.start().await?;
    
    // Server will run until stopped
    tokio::signal::ctrl_c().await?;
    server.stop().await?;
    
    Ok(())
}
```

### Client

```rust
use syneidesis_grpc::{CoordinationClient, GrpcClientConfig};
use syneidesis_grpc::coordination::{AgentInfo, AgentStatus, AgentHealth};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GrpcClientConfig::default();
    let mut client = CoordinationClient::new(config).await?;
    
    // Register an agent
    let agent_info = AgentInfo {
        id: "agent-1".to_string(),
        name: "Test Agent".to_string(),
        agent_type: "worker".to_string(),
        status: AgentStatus::Idle as i32,
        health: AgentHealth::Healthy as i32,
        current_task_id: None,
        assigned_scope: "default".to_string(),
        capabilities: vec!["task_processing".to_string()],
        metadata: std::collections::HashMap::new(),
    };
    
    let response = client.register_agent(agent_info).await?;
    println!("Agent registered: {:?}", response);
    
    Ok(())
}
```

## Configuration

### Server Configuration

```rust
use syneidesis_grpc::GrpcConfig;

let config = GrpcConfig {
    addr: "127.0.0.1:50051".to_string(),
    max_message_size: 10 * 1024 * 1024, // 10MB
    connection_timeout: 30,
    keep_alive_interval: 30,
    keep_alive_timeout: 10,
};
```

### Client Configuration

```rust
use syneidesis_grpc::GrpcClientConfig;

let config = GrpcClientConfig {
    server_addr: "127.0.0.1:50051".to_string(),
    connection_timeout: 30,
    request_timeout: 60,
    max_message_size: 10 * 1024 * 1024, // 10MB
};
```

## Dependencies

- `syneidesis-core`: Core types and utilities
- `syneidesis-agent`: Agent management functionality
- `tonic`: gRPC framework
- `tokio`: Async runtime
- `serde`: Serialization

## License

MIT License - see LICENSE file for details. 