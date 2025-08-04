/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error};
use uuid::Uuid;

// Use Syneidesis gRPC instead of building our own
use syneidesis_grpc::{
    CoordinationClient, CoordinationServer, CoordinationServerBuilder,
    AgentHealth, AgentInfo, AgentMessage, AgentPerformanceMetrics, AgentStatus,
    ConflictStrategy, MessagePriority, MessageType,
};
use syneidesis_config::types::{CoordinationConfig, GrpcConfig, GrpcClientConfig};

/// Example Rhema agent that uses Syneidesis gRPC coordination
struct RhemaAgent {
    id: String,
    name: String,
    client: CoordinationClient,
    message_receiver: Option<tokio::sync::mpsc::Receiver<AgentMessage>>,
}

impl RhemaAgent {
    async fn new(name: &str, server_address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Use Syneidesis client configuration
        let config = GrpcClientConfig {
            server_addr: server_address.to_string(),
            connection_timeout: 30,
            request_timeout: 60,
            max_message_size: 1024 * 1024, // 1MB
        };

        let mut client = CoordinationClient::new(config).await?;
        
        // Create agent info with Syneidesis types
        let agent_info = AgentInfo {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            agent_type: "rhema-agent".to_string(),
            status: AgentStatus::AgentStatusIdle as i32,
            health: AgentHealth::AgentHealthHealthy as i32,
            current_task_id: None,
            assigned_scope: "rhema-scope".to_string(),
            capabilities: vec!["messaging".to_string(), "coordination".to_string(), "rhema-integration".to_string()],
            last_heartbeat: Some(prost_types::Timestamp::from(chrono::Utc::now())),
            is_online: true,
            performance_metrics: Some(AgentPerformanceMetrics {
                tasks_completed: 0,
                tasks_failed: 0,
                avg_completion_time_seconds: 0.0,
                success_rate: 1.0,
                collaboration_score: 0.0,
                avg_response_time_ms: 0.0,
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0.0,
                active_connections: 1,
            }),
            priority: 1,
            version: env!("CARGO_PKG_VERSION").to_string(),
            endpoint: None,
            metadata: std::collections::HashMap::new(),
            created_at: Some(prost_types::Timestamp::from(chrono::Utc::now())),
            last_updated: Some(prost_types::Timestamp::from(chrono::Utc::now())),
        };

        client.register_agent(agent_info).await?;
        
        info!("Rhema agent {} registered with Syneidesis coordination", name);

        Ok(Self {
            id: agent_info.id.clone(),
            name: name.to_string(),
            client,
            message_receiver: None,
        })
    }

    async fn send_message(&self, content: &str, message_type: MessageType) -> Result<(), Box<dyn std::error::Error>> {
        let message = AgentMessage {
            id: Uuid::new_v4().to_string(),
            message_type: message_type as i32,
            priority: MessagePriority::MessagePriorityNormal as i32,
            sender_id: self.id.clone(),
            recipient_ids: vec![], // Empty for broadcast
            content: content.to_string(),
            payload: None, // Could add structured payload here
            timestamp: Some(prost_types::Timestamp::from(chrono::Utc::now())),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        };

        self.client.send_message(message).await?;
        info!("Rhema agent {} sent message: {}", self.name, content);
        Ok(())
    }

    async fn update_status(&self, status: AgentStatus, health: AgentHealth) -> Result<(), Box<dyn std::error::Error>> {
        self.client.update_agent_status(
            self.id.clone(),
            status,
            health,
            None, // current_task_id
        ).await?;
        info!("Rhema agent {} status updated", self.name);
        Ok(())
    }

    async fn create_session(&self, topic: &str, participants: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = self.client.create_session(topic.to_string(), participants).await?;
        info!("Rhema agent {} created session: {}", self.name, session_id);
        Ok(session_id)
    }

    async fn join_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client.join_session(session_id.to_string(), self.id.clone()).await?;
        info!("Rhema agent {} joined session: {}", self.name, session_id);
        Ok(())
    }

    async fn send_session_message(&self, session_id: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message = AgentMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::MessageTypeKnowledgeShare as i32,
            priority: MessagePriority::MessagePriorityNormal as i32,
            sender_id: self.id.clone(),
            recipient_ids: vec![],
            content: content.to_string(),
            payload: None,
            timestamp: Some(prost_types::Timestamp::from(chrono::Utc::now())),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        };

        self.client.send_session_message(session_id.to_string(), message).await?;
        info!("Rhema agent {} sent session message: {}", self.name, content);
        Ok(())
    }

    async fn detect_conflict(&self, resource_id: &str, conflict_type: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let conflict = self.client.detect_conflict(
            resource_id.to_string(),
            vec![self.id.clone()],
            conflict_type.to_string(),
            "Rhema agent conflict detection".to_string(),
        ).await?;

        if let Some(conflict) = conflict {
            info!("Rhema agent {} detected conflict: {}", self.name, conflict.id);
            Ok(Some(conflict.id))
        } else {
            Ok(None)
        }
    }

    async fn resolve_conflict(&self, conflict_id: &str, strategy: ConflictStrategy) -> Result<(), Box<dyn std::error::Error>> {
        self.client.resolve_conflict(
            conflict_id.to_string(),
            strategy,
            std::collections::HashMap::new(),
        ).await?;
        info!("Rhema agent {} resolved conflict: {}", self.name, conflict_id);
        Ok(())
    }

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
            None, // current_task_id
            Some(metrics),
        ).await?;

        if !pending_messages.is_empty() {
            info!("Rhema agent {} received {} pending messages", self.name, pending_messages.len());
        }

        Ok(())
    }
}

/// Start the Syneidesis coordination server
async fn start_syneidesis_server() -> Result<CoordinationServer, Box<dyn std::error::Error>> {
    // Use Syneidesis coordination configuration
    let config = CoordinationConfig::default();
    let grpc_config = GrpcConfig {
        addr: "127.0.0.1:50051".to_string(),
        connection_timeout: 30,
        max_message_size: 1024 * 1024, // 1MB
    };

    let mut server = CoordinationServerBuilder::new()
        .with_config(config)
        .with_grpc_config(grpc_config)
        .build()?;

    info!("Starting Syneidesis coordination server...");
    server.start().await?;
    
    Ok(server)
}

/// Run the integration example with multiple Rhema agents
async fn run_integration_example() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Rhema-Syneidesis gRPC integration example...");

    // Start the Syneidesis server in a separate task
    let server_handle = tokio::spawn(async {
        match start_syneidesis_server().await {
            Ok(mut server) => {
                info!("Syneidesis server started successfully");
                // Keep the server running
                loop {
                    sleep(Duration::from_secs(1)).await;
                }
            }
            Err(e) => {
                error!("Syneidesis server error: {}", e);
            }
        }
    });

    // Wait for server to start
    sleep(Duration::from_secs(2)).await;

    // Create multiple Rhema agents
    let mut agents = Vec::new();
    
    for i in 1..=3 {
        let agent_name = format!("RhemaAgent-{}", i);
        match RhemaAgent::new(&agent_name, "127.0.0.1:50051").await {
            Ok(agent) => {
                info!("Created Rhema agent: {}", agent_name);
                agents.push(agent);
            }
            Err(e) => {
                error!("Failed to create Rhema agent {}: {}", agent_name, e);
            }
        }
    }

    if agents.is_empty() {
        error!("No Rhema agents created, exiting");
        return Ok(());
    }

    // Simulate Rhema agent interactions using Syneidesis coordination
    for (i, agent) in agents.iter().enumerate() {
        // Send initial messages
        agent.send_message(
            &format!("Hello from Rhema agent {}!", agent.name),
            MessageType::MessageTypeStatusUpdate
        ).await?;

        // Update status
        agent.update_status(
            AgentStatus::AgentStatusWorking,
            AgentHealth::AgentHealthHealthy
        ).await?;

        // Create a coordination session
        let session_id = agent.create_session(
            &format!("rhema-session-{}", i),
            vec!["RhemaAgent-1".to_string(), "RhemaAgent-2".to_string(), "RhemaAgent-3".to_string()]
        ).await?;

        // Join the session
        agent.join_session(&session_id).await?;

        // Send session message
        agent.send_session_message(
            &session_id,
            &format!("Rhema session message from {}", agent.name)
        ).await?;

        sleep(Duration::from_millis(500)).await;
    }

    // Demonstrate conflict detection and resolution
    info!("Demonstrating conflict detection and resolution...");
    for agent in &agents {
        // Simulate a conflict
        if let Some(conflict_id) = agent.detect_conflict("shared-resource", "resource-contention").await? {
            // Resolve the conflict
            agent.resolve_conflict(&conflict_id, ConflictStrategy::ConflictStrategyAutoMerge).await?;
        }
    }

    // Let agents communicate for a while
    info!("Rhema agents are communicating via Syneidesis...");
    for _ in 0..10 {
        for agent in &agents {
            agent.heartbeat().await?;
        }
        sleep(Duration::from_secs(1)).await;
    }

    // Send final messages
    for agent in &agents {
        agent.send_message(
            &format!("Final message from Rhema agent {}", agent.name),
            MessageType::MessageTypeTaskCompletion
        ).await?;
    }

    // Wait a bit more
    sleep(Duration::from_secs(5)).await;

    info!("Rhema-Syneidesis integration example completed successfully!");
    
    // Cancel the server
    server_handle.abort();
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Run the integration example
    run_integration_example().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rhema_agent_creation() {
        // This test would require a running Syneidesis server
        // For now, we'll just test the error handling
        let result = RhemaAgent::new("test-agent", "http://localhost:9999").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_syneidesis_server_builder() {
        let config = CoordinationConfig::default();
        let grpc_config = GrpcConfig {
            addr: "127.0.0.1:0".to_string(),
            connection_timeout: 30,
            max_message_size: 1024 * 1024,
        };

        let server = CoordinationServerBuilder::new()
            .with_config(config)
            .with_grpc_config(grpc_config)
            .build();

        assert!(server.is_ok());
    }
} 