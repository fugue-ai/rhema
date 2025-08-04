# Rhema-Syneidesis gRPC Integration Guide

## Overview

This guide demonstrates how to integrate Rhema with Syneidesis gRPC coordination instead of building our own gRPC system. Syneidesis provides a production-ready, feature-rich coordination system that we can leverage directly.

## Why Use Syneidesis Instead of Building Our Own?

### ✅ **Advantages of Syneidesis Integration**

1. **Production-Ready**: Complete gRPC service with client/server implementation
2. **Advanced Features**: Health monitoring, conflict resolution, performance metrics
3. **Type Safety**: Full Protocol Buffer definitions with generated Rust types
4. **Conflict Resolution**: Built-in conflict detection and resolution strategies
5. **Health Monitoring**: Agent health states and performance tracking
6. **Bidirectional Streaming**: Real-time updates via gRPC streams
7. **Resource Management**: Resource locking and coordination
8. **Session Management**: Multi-agent coordination sessions

### 🚫 **Avoiding Duplication**

Instead of implementing:
- ❌ Custom gRPC service
- ❌ Protocol Buffer definitions
- ❌ Client/server infrastructure
- ❌ Conflict resolution logic
- ❌ Health monitoring system

We leverage:
- ✅ Syneidesis gRPC service
- ✅ Existing Protocol Buffer schemas
- ✅ Production-ready client/server
- ✅ Advanced coordination features

## Architecture

```
┌─────────────────┐    gRPC    ┌──────────────────┐
│   Rhema Agent   │ ────────── │ Syneidesis gRPC  │
│                 │            │   Coordination   │
│ - Agent Logic   │            │     Service      │
│ - Rhema Context │            │                  │
│ - Task Execution│            │ - Agent Registry │
└─────────────────┘            │ - Message Routing│
                               │ - Conflict Res.  │
┌─────────────────┐            │ - Health Monitor │
│   Rhema Agent   │ ────────── │ - Session Mgmt   │
│                 │            └──────────────────┘
│ - Agent Logic   │
│ - Rhema Context │
│ - Task Execution│
└─────────────────┘
```

## Integration Steps

### 1. Dependencies

Add Syneidesis dependencies to your `Cargo.toml`:

```toml
[dependencies]
syneidesis-grpc = { path = "../syneidesis/crates/grpc" }
syneidesis-config = { path = "../syneidesis/crates/config" }
```

### 2. Basic Agent Integration

```rust
use syneidesis_grpc::{
    CoordinationClient, AgentHealth, AgentInfo, AgentStatus,
    MessageType, MessagePriority, AgentMessage,
};
use syneidesis_config::types::GrpcClientConfig;

struct RhemaAgent {
    id: String,
    name: String,
    client: CoordinationClient,
}

impl RhemaAgent {
    async fn new(name: &str, server_address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = GrpcClientConfig {
            server_addr: server_address.to_string(),
            connection_timeout: 30,
            request_timeout: 60,
            max_message_size: 1024 * 1024,
        };

        let mut client = CoordinationClient::new(config).await?;
        
        let agent_info = AgentInfo {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            agent_type: "rhema-agent".to_string(),
            status: AgentStatus::AgentStatusIdle as i32,
            health: AgentHealth::AgentHealthHealthy as i32,
            // ... other fields
        };

        client.register_agent(agent_info).await?;
        
        Ok(Self {
            id: agent_info.id.clone(),
            name: name.to_string(),
            client,
        })
    }
}
```

### 3. Message Communication

```rust
impl RhemaAgent {
    async fn send_message(&self, content: &str, message_type: MessageType) -> Result<(), Box<dyn std::error::Error>> {
        let message = AgentMessage {
            id: Uuid::new_v4().to_string(),
            message_type: message_type as i32,
            priority: MessagePriority::MessagePriorityNormal as i32,
            sender_id: self.id.clone(),
            recipient_ids: vec![],
            content: content.to_string(),
            timestamp: Some(prost_types::Timestamp::from(chrono::Utc::now())),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        };

        self.client.send_message(message).await?;
        Ok(())
    }
}
```

### 4. Session Management

```rust
impl RhemaAgent {
    async fn create_coordination_session(&self, topic: &str, participants: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = self.client.create_session(topic.to_string(), participants).await?;
        Ok(session_id)
    }

    async fn join_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client.join_session(session_id.to_string(), self.id.clone()).await?;
        Ok(())
    }

    async fn send_session_message(&self, session_id: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message = AgentMessage {
            // ... message construction
        };
        self.client.send_session_message(session_id.to_string(), message).await?;
        Ok(())
    }
}
```

### 5. Conflict Resolution

```rust
impl RhemaAgent {
    async fn detect_conflict(&self, resource_id: &str, conflict_type: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let conflict = self.client.detect_conflict(
            resource_id.to_string(),
            vec![self.id.clone()],
            conflict_type.to_string(),
            "Rhema agent conflict detection".to_string(),
        ).await?;

        Ok(conflict.map(|c| c.id))
    }

    async fn resolve_conflict(&self, conflict_id: &str, strategy: ConflictStrategy) -> Result<(), Box<dyn std::error::Error>> {
        self.client.resolve_conflict(
            conflict_id.to_string(),
            strategy,
            std::collections::HashMap::new(),
        ).await?;
        Ok(())
    }
}
```

### 6. Health Monitoring

```rust
impl RhemaAgent {
    async fn heartbeat(&self) -> Result<(), Box<dyn std::error::Error>> {
        let metrics = AgentPerformanceMetrics {
            tasks_completed: 0,
            tasks_failed: 0,
            avg_completion_time_seconds: 0.0,
            success_rate: 1.0,
            collaboration_score: 0.0,
            avg_response_time_ms: 0.0,
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            active_connections: 1,
        };

        let pending_messages = self.client.heartbeat(
            self.id.clone(),
            AgentStatus::AgentStatusIdle,
            AgentHealth::AgentHealthHealthy,
            None,
            Some(metrics),
        ).await?;

        // Process pending messages
        for message in pending_messages {
            self.process_message(message).await?;
        }

        Ok(())
    }
}
```

## Server Setup

### Starting Syneidesis Server

```rust
use syneidesis_grpc::{CoordinationServer, CoordinationServerBuilder};
use syneidesis_config::types::{CoordinationConfig, GrpcConfig};

async fn start_syneidesis_server() -> Result<CoordinationServer, Box<dyn std::error::Error>> {
    let config = CoordinationConfig::default();
    let grpc_config = GrpcConfig {
        addr: "127.0.0.1:50051".to_string(),
        connection_timeout: 30,
        max_message_size: 1024 * 1024,
    };

    let mut server = CoordinationServerBuilder::new()
        .with_config(config)
        .with_grpc_config(grpc_config)
        .build()?;

    server.start().await?;
    Ok(server)
}
```

## Advanced Features

### 1. Resource Management

```rust
impl RhemaAgent {
    async fn request_resource(&self, resource_id: &str, timeout_seconds: Option<u32>) -> Result<bool, Box<dyn std::error::Error>> {
        let acquired = self.client.request_resource(
            resource_id.to_string(),
            self.id.clone(),
            timeout_seconds,
        ).await?;
        Ok(acquired)
    }

    async fn release_resource(&self, resource_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client.release_resource(resource_id.to_string(), self.id.clone()).await?;
        Ok(())
    }
}
```

### 2. Statistics and Monitoring

```rust
impl RhemaAgent {
    async fn get_coordination_stats(&self) -> Result<CoordinationStats, Box<dyn std::error::Error>> {
        let stats = self.client.get_stats().await?;
        Ok(stats)
    }

    async fn get_all_agents(&self) -> Result<Vec<AgentInfo>, Box<dyn std::error::Error>> {
        let agents = self.client.get_all_agents().await?;
        Ok(agents)
    }
}
```

### 3. Message History

```rust
impl RhemaAgent {
    async fn get_message_history(&self, limit: u32, agent_id: Option<String>) -> Result<Vec<AgentMessage>, Box<dyn std::error::Error>> {
        let messages = self.client.get_message_history(limit, agent_id).await?;
        Ok(messages)
    }
}
```

## Configuration

### Syneidesis Configuration Schema

The `schemas/syneidesis_config.json` provides comprehensive configuration:

```json
{
  "syneidesis": {
    "enabled": true,
    "run_local_server": true,
    "server_address": "127.0.0.1:50051",
    "auto_register_agents": true,
    "sync_messages": true,
    "sync_tasks": true,
    "enable_health_monitoring": true,
    "health_check_interval": 30,
    "message_timeout": 60,
    "max_message_history": 1000,
    "encryption": {
      "enabled": false,
      "algorithm": "AES-256"
    },
    "compression": {
      "enabled": true,
      "algorithm": "zstd",
      "threshold": 1024
    }
  }
}
```

## Migration from Custom gRPC

### Before (Custom Implementation)

```rust
// Custom gRPC client
use rhema_ai::grpc::{GrpcCoordinationClient, GrpcClientConfig};

let config = GrpcClientConfig {
    server_address: "127.0.0.1:50051".to_string(),
    // ... custom config
};

let mut client = GrpcCoordinationClient::new(config).await?;
```

### After (Syneidesis Integration)

```rust
// Syneidesis gRPC client
use syneidesis_grpc::CoordinationClient;
use syneidesis_config::types::GrpcClientConfig;

let config = GrpcClientConfig {
    server_addr: "127.0.0.1:50051".to_string(),
    // ... syneidesis config
};

let mut client = CoordinationClient::new(config).await?;
```

## Benefits Summary

### 🚀 **Performance**
- Production-optimized gRPC implementation
- Efficient message routing and delivery
- Built-in connection pooling and retry logic

### 🛡️ **Reliability**
- Conflict detection and resolution
- Health monitoring and failure recovery
- Message persistence and history

### 🔧 **Maintainability**
- No custom gRPC code to maintain
- Standard Protocol Buffer schemas
- Comprehensive test coverage

### 📈 **Scalability**
- Horizontal scaling support
- Load balancing capabilities
- Resource management and coordination

### 🔒 **Security**
- Optional message encryption
- Authentication and authorization
- Secure communication channels

## Next Steps

1. **Update TODO**: Mark gRPC coordination as complete using Syneidesis
2. **Integration Testing**: Test Rhema agents with Syneidesis coordination
3. **Documentation**: Update architecture docs to reflect Syneidesis integration
4. **Performance Testing**: Benchmark coordination performance
5. **Production Deployment**: Deploy with Syneidesis coordination in production

## Conclusion

By leveraging Syneidesis gRPC coordination instead of building our own, we gain:

- **Immediate production readiness**
- **Advanced coordination features**
- **Reduced maintenance burden**
- **Better reliability and performance**
- **Standard compliance**

This integration demonstrates the power of leveraging existing, well-tested infrastructure rather than duplicating effort. Syneidesis provides everything we need for real-time AI agent coordination and more. 