# Rhema Agent Framework

A comprehensive, asynchronous agent framework for building intelligent, coordinated multi-agent systems in Rust.

## Overview

The Rhema Agent Framework provides a robust foundation for creating, managing, and coordinating intelligent agents that can work together to solve complex problems. It's designed with scalability, reliability, and extensibility in mind, making it suitable for both simple automation tasks and complex distributed systems.

## Features

### 🏗️ **Core Framework**
- **Agent Registry**: Centralized agent management and discovery
- **Agent Coordinator**: Intelligent coordination between multiple agents
- **Message Broker**: Asynchronous communication system
- **Capability Manager**: Dynamic capability discovery and management
- **Policy Engine**: Configurable policies for agent behavior
- **Metrics Collector**: Comprehensive performance monitoring
- **Workflow Engine**: Complex workflow orchestration

### 🤖 **Built-in Agents**
- **CodeReviewAgent**: Security analysis and code quality assessment
- **TestRunnerAgent**: Automated test generation and execution
- **DeploymentAgent**: CI/CD and deployment management
- **DocumentationAgent**: Documentation generation and maintenance
- **MonitoringAgent**: System monitoring and alerting

### 🔧 **Advanced Features**
- **Async/Await**: Full async support with Tokio
- **Distributed**: Redis-backed state management
- **Observable**: Comprehensive logging and metrics
- **Extensible**: Plugin-based architecture
- **Type-Safe**: Strong typing throughout the framework
- **Error Handling**: Robust error handling with custom error types

## Quick Start

### Basic Usage

```rust
use rhema_agent::{
    RhemaAgentFramework, CodeReviewAgent, TestRunnerAgent,
    AgentRequest, AgentMessage
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the framework
    let mut framework = RhemaAgentFramework::new();
    framework.initialize().await?;

    // Register and start agents
    let code_review_agent = Box::new(CodeReviewAgent::new("code-review-1".to_string()));
    let code_review_id = framework.register_agent(code_review_agent).await?;
    framework.start_agent(&code_review_id).await?;

    // Send a task to the agent
    let request = AgentRequest::new(
        "code_review".to_string(),
        serde_json::json!({
            "code_path": "./src",
            "security_analysis": true
        })
    );

    let message = AgentMessage::TaskRequest(request);
    framework.send_message(&code_review_id, message).await?;

    // Get framework statistics
    let stats = framework.get_framework_stats().await?;
    println!("Active agents: {}", stats.active_agents);

    Ok(())
}
```

### Creating Custom Agents

```rust
use rhema_agent::{
    Agent, AgentId, AgentType, AgentCapability, AgentState,
    AgentContext, AgentMessage, AgentResponse, AgentRequest, AgentResult
};

pub struct MyCustomAgent {
    id: AgentId,
    state: AgentState,
}

impl Agent for MyCustomAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn agent_type(&self) -> AgentType {
        AgentType::Custom("my_agent".to_string())
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![
            AgentCapability::new("custom_task".to_string()),
            AgentCapability::new("data_processing".to_string()),
        ]
    }

    fn state(&self) -> AgentState {
        self.state.clone()
    }

    async fn handle_message(
        &mut self,
        message: AgentMessage,
        context: &AgentContext,
    ) -> AgentResult<AgentResponse> {
        match message {
            AgentMessage::TaskRequest(request) => {
                // Handle task request
                Ok(AgentResponse::Success(serde_json::json!({
                    "result": "task completed"
                })))
            }
            _ => Ok(AgentResponse::Ignored),
        }
    }

    async fn initialize(&mut self) -> AgentResult<()> {
        self.state = AgentState::Ready;
        Ok(())
    }

    async fn shutdown(&mut self) -> AgentResult<()> {
        self.state = AgentState::Stopped;
        Ok(())
    }
}
```

## Architecture

### Core Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Agent Registry│    │   Coordinator   │    │  Message Broker │
│                 │    │                 │    │                 │
│ • Agent Discovery│    │ • Task Distribution│  │ • Async Messaging│
│ • State Management│   │ • Load Balancing │   │ • Message Routing│
│ • Health Monitoring│  │ • Conflict Resolution│ │ • Priority Queuing│
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
         ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
         │ Capability Mgr  │    │  Policy Engine  │    │ Metrics Collector│
         │                 │    │                 │    │                 │
         │ • Capability Discovery│ │ • Behavior Policies│ │ • Performance Metrics│
         │ • Resource Management│ │ • Security Rules │   │ • Health Monitoring│
         │ • Dynamic Loading │   │ • Compliance Checks│  │ • Alert Generation│
         └─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Agent Lifecycle

1. **Registration**: Agent is registered with the framework
2. **Initialization**: Agent initializes its resources
3. **Ready**: Agent is ready to receive messages
4. **Active**: Agent is processing tasks
5. **Shutdown**: Agent gracefully shuts down

### Message Flow

```
Client Request → Message Broker → Agent Registry → Target Agent
                                                      ↓
Agent Response ← Message Broker ← Agent Processing ← Agent
```

## Built-in Agents

### CodeReviewAgent

Performs comprehensive code analysis with security, quality, and performance assessments.

```rust
use rhema_agent::{CodeReviewAgent, CodeReviewRequest};

let agent = CodeReviewAgent::new("code-review-1".to_string());
let request = CodeReviewRequest {
    code_path: "./src".to_string(),
    file_extensions: vec!["rs".to_string()],
    security_analysis: true,
    quality_analysis: true,
    performance_analysis: true,
    custom_rules: vec![],
    ignore_patterns: vec![],
};

let result = agent.perform_code_review(request).await?;
```

### TestRunnerAgent

Automatically generates and executes tests for various programming languages.

```rust
use rhema_agent::{TestRunnerAgent, TestGenerationRequest};

let agent = TestRunnerAgent::new("test-runner-1".to_string());
let request = TestGenerationRequest {
    code_path: "./src".to_string(),
    test_framework: "rust".to_string(),
    test_types: vec!["unit".to_string(), "integration".to_string()],
    coverage_target: 80.0,
};

let result = agent.generate_tests(request).await?;
```

## Configuration

The framework supports extensive configuration through the `rhema-config` crate:

```yaml
# agent_config.yaml
framework:
  registry:
    redis_url: "redis://localhost:6379"
    health_check_interval: 30s
  
  coordinator:
    max_concurrent_tasks: 10
    load_balancing_strategy: "round_robin"
  
  messaging:
    broker_type: "redis"
    message_ttl: 300s
    retry_attempts: 3
  
  policies:
    security:
      max_execution_time: 300s
      allowed_capabilities: ["code_review", "test_generation"]
    
    resource:
      max_memory_usage: "1GB"
      max_cpu_usage: 80.0

agents:
  code_review:
    enabled: true
    instances: 2
    security_rules:
      - "sql_injection"
      - "xss"
      - "hardcoded_credentials"
  
  test_runner:
    enabled: true
    instances: 3
    supported_frameworks:
      - "rust"
      - "python"
      - "javascript"
```

## Monitoring and Observability

The framework provides comprehensive monitoring capabilities:

### Metrics

- Agent performance metrics
- Message throughput
- Error rates
- Resource utilization
- Workflow execution statistics

### Logging

Structured logging with different levels:

```rust
use tracing::{info, warn, error};

info!("Agent {} started successfully", agent_id);
warn!("High memory usage detected: {}MB", memory_usage);
error!("Task execution failed: {}", error);
```

### Health Checks

Automatic health monitoring with customizable checks:

```rust
let health_status = framework.get_agent_health(&agent_id).await?;
if health_status.is_healthy() {
    println!("Agent is healthy");
} else {
    println!("Agent health issues: {:?}", health_status.issues);
}
```

## Error Handling

The framework uses custom error types for robust error handling:

```rust
use rhema_agent::AgentError;

match result {
    Ok(response) => {
        // Handle successful response
    }
    Err(AgentError::AgentNotFound(id)) => {
        println!("Agent {} not found", id);
    }
    Err(AgentError::CapabilityNotSupported(capability)) => {
        println!("Capability {} not supported", capability);
    }
    Err(AgentError::PolicyViolation(violation)) => {
        println!("Policy violation: {}", violation);
    }
    Err(e) => {
        println!("Unexpected error: {}", e);
    }
}
```

## Examples

See the `examples/` directory for comprehensive examples:

- `agent_usage_example.rs`: Basic framework usage
- `comprehensive_agent_workflow.rs`: Complex workflow orchestration

## Testing

The framework includes comprehensive testing utilities:

```rust
use rhema_agent::test_utils::*;

#[tokio::test]
async fn test_agent_lifecycle() {
    let mut framework = create_test_framework().await;
    let agent = create_test_agent("test-agent").await;
    
    let agent_id = framework.register_agent(agent).await.unwrap();
    framework.start_agent(&agent_id).await.unwrap();
    
    assert_eq!(framework.get_agent_state(&agent_id).await.unwrap(), AgentState::Ready);
}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details.

## Related Crates

- `rhema-core`: Core utilities and types
- `rhema-ai`: AI/ML integration capabilities
- `rhema-config`: Configuration management
- `rhema-knowledge`: Knowledge base integration
- `rhema-monitoring`: Advanced monitoring capabilities 