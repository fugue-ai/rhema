# Syneidesis Coordination Library - Integration Guide

## 🚀 Quick Start Integration

The Syneidesis Coordination Library provides real-time agent coordination capabilities for distributed multi-agent systems. This guide will help you integrate it into your services quickly and effectively.

### 1. Add to Your Project

#### Option A: Published Crate (Recommended for production)

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
syneidesis-coordination = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

#### Option B: Local Development (For relative projects)

If your project is relative to the syneidesis workspace (e.g., `../syneidesis`), you can use a local path dependency:

```toml
[dependencies]
syneidesis-coordination = { path = "../syneidesis/crates/coordination" }
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

**Note**: When using local paths, make sure your project's `Cargo.toml` is compatible with the syneidesis workspace dependencies. You may need to add workspace dependencies if they're not already included in your project.

### 2. Basic Integration Pattern

```rust
use syneidesis_coordination::{
    init, AgentCoordinator, AgentState, AgentHealth, AgentStatus,
    CoordinationConfig, Task, TaskPriority
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the coordination library
    init().await?;
    
    // 2. Create and start coordinator
    let mut coordinator = AgentCoordinator::new();
    coordinator.start().await?;
    
    // 3. Register your agents
    let mut my_agent = AgentState::new(
        "my-service-agent".to_string(),
        "My Service Agent".to_string(),
        "service".to_string(),
    );
    
    // Add capabilities your agent supports
    my_agent.add_capability("data_processing".to_string());
    my_agent.add_capability("api_calls".to_string());
    
    // Register the agent
    coordinator.register_agent(my_agent.clone()).await?;
    
    // 4. Your service is now coordinated!
    println!("✅ Service integrated with Syneidesis coordination!");
    
    Ok(())
}
```

## 🔧 Integration Patterns

### Pattern 1: Service Integration

For services that need to coordinate with other agents:

```rust
use syneidesis_coordination::{
    AgentCoordinator, AgentState, AgentStatus, Task, TaskPriority
};

pub struct MyService {
    coordinator: AgentCoordinator,
    agent_id: String,
}

impl MyService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize coordination
        syneidesis_coordination::init().await?;
        
        let mut coordinator = AgentCoordinator::new();
        coordinator.start().await?;
        
        // Register this service as an agent
        let mut agent = AgentState::new(
            "my-service".to_string(),
            "My Service".to_string(),
            "service".to_string(),
        );
        agent.add_capability("my_capability".to_string());
        
        coordinator.register_agent(agent.clone()).await?;
        
        Ok(Self {
            coordinator,
            agent_id: agent.id,
        })
    }
    
    pub async fn process_task(&mut self, task_data: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        // Update status to busy
        self.coordinator.update_agent_status(&self.agent_id, AgentStatus::Busy).await?;
        
        // Process your task
        let result = self.process_data(task_data).await?;
        
        // Update status back to idle
        self.coordinator.update_agent_status(&self.agent_id, AgentStatus::Idle).await?;
        
        Ok(result)
    }
    
    async fn process_data(&self, data: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // Your service logic here
        Ok(data)
    }
}
```

### Pattern 2: gRPC Server Integration

For services that need to expose coordination capabilities via gRPC:

```rust
use syneidesis_coordination::{
    config::CoordinationConfig,
    grpc::{CoordinationServer, service::RealTimeCoordinationServiceImpl},
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Create coordination service
    let config = CoordinationConfig::default();
    let service = RealTimeCoordinationServiceImpl::new(config)?;
    
    // Start the service
    let mut service = service;
    service.start().await?;
    
    // Create gRPC server
    let addr = "[::1]:50051".parse()?;
    let server = Server::builder()
        .add_service(CoordinationServer::new(service))
        .serve(addr);
    
    println!("gRPC coordination server listening on {}", addr);
    server.await?;
    
    Ok(())
}
```

### Pattern 3: Client Integration

For services that need to connect to a coordination server:

```rust
use syneidesis_coordination::grpc::CoordinationClient;
use tonic::transport::Channel;

pub struct CoordinationClient {
    client: syneidesis_coordination::grpc::CoordinationClient<Channel>,
}

impl CoordinationClient {
    pub async fn new(server_addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(server_addr)?
            .connect()
            .await?;
        
        let client = syneidesis_coordination::grpc::CoordinationClient::new(channel);
        
        Ok(Self { client })
    }
    
    pub async fn register_my_service(&mut self, service_info: AgentInfo) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(
            syneidesis_coordination::grpc::coordination::RegisterAgentRequest {
                agent_info: Some(service_info),
            },
        );
        
        self.client.register_agent(request).await?;
        Ok(())
    }
    
    pub async fn update_status(&mut self, agent_id: String, status: AgentStatus) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(
            syneidesis_coordination::grpc::coordination::UpdateAgentStatusRequest {
                agent_id,
                status: status as i32,
                health: AgentHealth::Healthy as i32,
                current_task_id: None,
            },
        );
        
        self.client.update_agent_status(request).await?;
        Ok(())
    }
}
```

## 🎯 Common Integration Scenarios

### Scenario 1: Microservice Coordination

```rust
// In your microservice
use syneidesis_coordination::{AgentCoordinator, AgentState, AgentStatus};

pub struct Microservice {
    coordinator: AgentCoordinator,
    agent_id: String,
}

impl Microservice {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        syneidesis_coordination::init().await?;
        
        let mut coordinator = AgentCoordinator::new();
        coordinator.start().await?;
        
        let mut agent = AgentState::new(
            "user-service".to_string(),
            "User Management Service".to_string(),
            "microservice".to_string(),
        );
        agent.add_capability("user_management".to_string());
        agent.add_capability("authentication".to_string());
        
        coordinator.register_agent(agent.clone()).await?;
        
        Ok(Self {
            coordinator,
            agent_id: agent.id,
        })
    }
    
    pub async fn handle_request(&mut self, request: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        // Update status to busy
        self.coordinator.update_agent_status(&self.agent_id, AgentStatus::Busy).await?;
        
        // Process request
        let response = self.process_request(request).await?;
        
        // Update status back to idle
        self.coordinator.update_agent_status(&self.agent_id, AgentStatus::Idle).await?;
        
        Ok(response)
    }
}
```

### Scenario 2: Task Distribution System

```rust
use syneidesis_coordination::{AgentCoordinator, Task, TaskPriority};

pub struct TaskDistributor {
    coordinator: AgentCoordinator,
}

impl TaskDistributor {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        syneidesis_coordination::init().await?;
        
        let mut coordinator = AgentCoordinator::new();
        coordinator.start().await?;
        
        Ok(Self { coordinator })
    }
    
    pub async fn distribute_task(&mut self, task_type: String, task_data: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        let task = Task::new(
            format!("task-{}", uuid::Uuid::new_v4()),
            format!("Process {}", task_type),
            task_type.clone(),
            task_data,
        )
        .with_priority(TaskPriority::Normal)
        .with_capabilities(vec![task_type]);
        
        let task_id = self.coordinator.assign_task(task).await?;
        Ok(task_id)
    }
    
    pub async fn get_available_agents(&self) -> Result<Vec<AgentState>, Box<dyn std::error::Error>> {
        self.coordinator.get_available_agents().await
    }
}
```

### Scenario 3: Health Monitoring Integration

```rust
use syneidesis_coordination::{AgentCoordinator, AgentHealth, AgentMetrics};

pub struct HealthMonitor {
    coordinator: AgentCoordinator,
    agent_id: String,
}

impl HealthMonitor {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        syneidesis_coordination::init().await?;
        
        let mut coordinator = AgentCoordinator::new();
        coordinator.start().await?;
        
        let mut agent = AgentState::new(
            "health-monitor".to_string(),
            "Health Monitoring Service".to_string(),
            "monitor".to_string(),
        );
        agent.add_capability("health_monitoring".to_string());
        
        coordinator.register_agent(agent.clone()).await?;
        
        Ok(Self {
            coordinator,
            agent_id: agent.id,
        })
    }
    
    pub async fn update_health_metrics(&mut self, cpu_usage: f64, memory_usage: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut metrics = AgentMetrics::default();
        metrics.update_cpu_usage(cpu_usage);
        metrics.update_memory_usage(memory_usage, 1024 * 1024 * 1024); // 1GB total
        
        self.coordinator.update_agent_metrics(&self.agent_id, metrics).await?;
        Ok(())
    }
    
    pub async fn report_health_status(&mut self, is_healthy: bool) -> Result<(), Box<dyn std::error::Error>> {
        let health = if is_healthy { AgentHealth::Healthy } else { AgentHealth::Unhealthy };
        self.coordinator.update_agent_health(&self.agent_id, health).await?;
        Ok(())
    }
}
```

## ⚙️ Configuration

### Basic Configuration

```rust
use syneidesis_coordination::config::CoordinationConfig;

let config = CoordinationConfig {
    grpc: GrpcConfig {
        address: "127.0.0.1:50051".to_string(),
        max_concurrent_streams: 100,
        max_frame_size: 1024 * 1024,
    },
    agent: AgentConfig {
        max_agents: 1000,
        health_check_interval: Duration::from_secs(30),
        heartbeat_timeout: Duration::from_secs(60),
    },
    communication: CommunicationConfig {
        websocket_url: "ws://localhost:8080".to_string(),
        max_message_size: 1024 * 1024,
        reconnect_attempts: 5,
    },
    conflict: ConflictConfig {
        default_strategy: ConflictStrategy::LastWriterWins,
        auto_resolve_timeout: Duration::from_secs(300),
    },
};
```

### Environment-based Configuration

```rust
use std::env;

fn load_config() -> CoordinationConfig {
    let grpc_addr = env::var("COORDINATION_GRPC_ADDR").unwrap_or_else(|_| "127.0.0.1:50051".to_string());
    let max_agents = env::var("COORDINATION_MAX_AGENTS")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);
    
    CoordinationConfig {
        grpc: GrpcConfig {
            address: grpc_addr,
            max_concurrent_streams: 100,
            max_frame_size: 1024 * 1024,
        },
        agent: AgentConfig {
            max_agents,
            health_check_interval: Duration::from_secs(30),
            heartbeat_timeout: Duration::from_secs(60),
        },
        ..Default::default()
    }
}
```

## 🔍 Monitoring and Debugging

### Enable Logging

```rust
use tracing::{info, warn, error};

// In your main function
tracing_subscriber::fmt::init();

// In your service
info!("Service started with coordination");
warn!("Agent health check failed");
error!("Failed to register agent: {}", e);
```

### Get Statistics

```rust
// Get coordination statistics
let stats = coordinator.get_statistics().await?;
println!("Total agents: {}", stats.total_agents);
println!("Healthy agents: {}", stats.healthy_agents);
println!("Success rate: {:.2}%", stats.success_rate);
```

### Health Checks

```rust
// Check if agent is healthy
let agent = coordinator.get_agent("my-agent").await?;
if agent.health == AgentHealth::Healthy {
    println!("Agent is healthy");
} else {
    println!("Agent health: {:?}", agent.health);
}
```

## 🚨 Error Handling

### Common Error Patterns

```rust
use syneidesis_coordination::error::{CoordinationError, AgentError};

match coordinator.register_agent(agent).await {
    Ok(_) => println!("Agent registered successfully"),
    Err(CoordinationError::Agent(AgentError::AlreadyExists { agent_id })) => {
        println!("Agent {} already exists", agent_id);
    }
    Err(CoordinationError::Agent(AgentError::NotFound { agent_id })) => {
        println!("Agent {} not found", agent_id);
    }
    Err(e) => {
        error!("Unexpected error: {}", e);
    }
}
```

### Graceful Shutdown

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut coordinator = AgentCoordinator::new();
    coordinator.start().await?;
    
    // Register your agents...
    
    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Shutting down...");
        }
        _ = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) => {
            println!("Received SIGTERM, shutting down...");
        }
    }
    
    // Graceful shutdown
    coordinator.stop().await?;
    println!("Coordinator stopped successfully");
    
    Ok(())
}
```

## 📊 Performance Tips

### 1. Connection Pooling

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CoordinationService {
    coordinator: Arc<Mutex<AgentCoordinator>>,
}

impl CoordinationService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        syneidesis_coordination::init().await?;
        
        let mut coordinator = AgentCoordinator::new();
        coordinator.start().await?;
        
        Ok(Self {
            coordinator: Arc::new(Mutex::new(coordinator)),
        })
    }
    
    pub async fn register_agent(&self, agent: AgentState) -> Result<(), Box<dyn std::error::Error>> {
        let mut coordinator = self.coordinator.lock().await;
        coordinator.register_agent(agent).await
    }
}
```

### 2. Batch Operations

```rust
// Register multiple agents efficiently
pub async fn register_multiple_agents(
    coordinator: &mut AgentCoordinator,
    agents: Vec<AgentState>,
) -> Result<(), Box<dyn std::error::Error>> {
    for agent in agents {
        coordinator.register_agent(agent).await?;
    }
    Ok(())
}
```

### 3. Async Task Management

```rust
use tokio::task;

pub async fn process_tasks_concurrently(
    coordinator: &mut AgentCoordinator,
    tasks: Vec<Task>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut handles = vec![];
    
    for task in tasks {
        let mut coordinator_clone = coordinator.clone();
        let handle = task::spawn(async move {
            coordinator_clone.assign_task(task).await
        });
        handles.push(handle);
    }
    
    let mut results = vec![];
    for handle in handles {
        let task_id = handle.await??;
        results.push(task_id);
    }
    
    Ok(results)
}
```

## 🔐 Security Considerations

### 1. Authentication

```rust
// Add authentication to your coordination service
pub struct SecureCoordinationService {
    coordinator: AgentCoordinator,
    auth_token: String,
}

impl SecureCoordinationService {
    pub async fn new(auth_token: String) -> Result<Self, Box<dyn std::error::Error>> {
        syneidesis_coordination::init().await?;
        
        let mut coordinator = AgentCoordinator::new();
        coordinator.start().await?;
        
        Ok(Self {
            coordinator,
            auth_token,
        })
    }
    
    pub async fn register_agent_with_auth(
        &mut self,
        agent: AgentState,
        token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if token != self.auth_token {
            return Err("Invalid authentication token".into());
        }
        
        self.coordinator.register_agent(agent).await
    }
}
```

### 2. Network Security

```rust
// Use TLS for gRPC connections
use tonic::transport::{Channel, ClientTlsConfig};

pub async fn create_secure_client() -> Result<CoordinationClient, Box<dyn std::error::Error>> {
    let tls_config = ClientTlsConfig::new();
    
    let channel = Channel::from_shared("https://localhost:50051".to_string())?
        .tls_config(tls_config)?
        .connect()
        .await?;
    
    let client = CoordinationClient::new(channel);
    Ok(client)
}
```

## 📚 Next Steps

1. **Explore Examples**: Check out the examples in `crates/coordination/examples/`
2. **Read Documentation**: Visit the main README for detailed API documentation
3. **Join Community**: Get help and share your integration experiences
4. **Contribute**: Help improve the library by contributing code or documentation

## 🆘 Getting Help

- **Documentation**: [README.md](README.md)
- **Examples**: [examples/](examples/)
- **Issues**: Create an issue on GitHub
- **Discord**: Join our community for real-time help

---

**Happy Coordinating! 🚀** 