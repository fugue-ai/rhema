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

use rhema_coordination::{
    agent::real_time_coordination::{AgentInfo, AgentMessage, AgentStatus, MessagePriority, MessageType},
    grpc::{GrpcCoordinationClient, GrpcCoordinationServer, GrpcClientConfig, GrpcServerConfig},
    RealTimeCoordinationSystem,
};

/// Example agent that demonstrates gRPC coordination
struct ExampleAgent {
    id: String,
    name: String,
    client: GrpcCoordinationClient,
    message_receiver: Option<tokio::sync::mpsc::Receiver<AgentMessage>>,
}

impl ExampleAgent {
    async fn new(name: &str, server_address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = GrpcClientConfig {
            server_address: server_address.to_string(),
            connection_timeout_seconds: 30,
            request_timeout_seconds: 60,
            heartbeat_interval_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
            enable_connection_pooling: true,
            max_concurrent_requests: 100,
        };

        let mut client = GrpcCoordinationClient::new(config).await?;
        
        let agent_info = AgentInfo {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            agent_type: "example".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "example-scope".to_string(),
            capabilities: vec!["messaging".to_string(), "coordination".to_string()],
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: Default::default(),
        };

        client.register_agent(agent_info.clone()).await?;
        
        let message_receiver = client.get_message_stream().await.ok();

        Ok(Self {
            id: agent_info.id,
            name: name.to_string(),
            client,
            message_receiver,
        })
    }

    async fn send_message(&self, content: &str, message_type: MessageType) -> Result<(), Box<dyn std::error::Error>> {
        let message = AgentMessage {
            id: Uuid::new_v4().to_string(),
            message_type,
            priority: MessagePriority::Normal,
            sender_id: self.id.clone(),
            recipient_ids: vec![], // Empty for broadcast
            content: content.to_string(),
            payload: Some(serde_json::json!({
                "agent_name": self.name,
                "timestamp": chrono::Utc::now().timestamp_millis(),
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        };

        self.client.send_message(message).await?;
        info!("Agent {} sent message: {}", self.name, content);
        Ok(())
    }

    async fn listen_for_messages(&mut self) {
        if let Some(mut receiver) = self.message_receiver.take() {
            info!("Agent {} started listening for messages", self.name);
            
            while let Some(message) = receiver.recv().await {
                if message.sender_id != self.id {
                    info!("Agent {} received message from {}: {}", 
                          self.name, message.sender_id, message.content);
                    
                    // Echo the message back
                    if let Err(e) = self.send_message(
                        &format!("Echo: {}", message.content),
                        MessageType::StatusUpdate
                    ).await {
                        error!("Failed to echo message: {}", e);
                    }
                }
            }
        }
    }

    async fn update_status(&self, status: AgentStatus) -> Result<(), Box<dyn std::error::Error>> {
        self.client.update_status(status).await?;
        info!("Agent {} status updated", self.name);
        Ok(())
    }

    async fn create_session(&self, topic: &str, participants: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = self.client.create_session(topic.to_string(), participants).await?;
        info!("Agent {} created session: {}", self.name, session_id);
        Ok(session_id)
    }

    async fn join_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client.join_session(session_id).await?;
        info!("Agent {} joined session: {}", self.name, session_id);
        Ok(())
    }

    async fn send_session_message(&self, session_id: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message = AgentMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::KnowledgeShare,
            priority: MessagePriority::Normal,
            sender_id: self.id.clone(),
            recipient_ids: vec![],
            content: content.to_string(),
            payload: Some(serde_json::json!({
                "session_id": session_id,
                "agent_name": self.name,
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        };

        self.client.send_session_message(session_id, message).await?;
        info!("Agent {} sent session message: {}", self.name, content);
        Ok(())
    }
}

/// Start the gRPC coordination server
async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
    
    let config = GrpcServerConfig {
        bind_address: "[::1]:50051".to_string(),
        max_concurrent_requests: 1000,
        request_timeout_seconds: 60,
        enable_reflection: true,
        enable_cors: true,
        cors_allowed_origins: vec!["*".to_string()],
        enable_compression: true,
        enable_tls: false,
        tls_cert_path: None,
        tls_key_path: None,
    };

    let server = GrpcCoordinationServer::new(config, coordination_system);
    
    info!("Starting gRPC coordination server...");
    server.start().await?;
    
    Ok(())
}

/// Run the example with multiple agents
async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting gRPC coordination example...");

    // Start the server in a separate task
    let server_handle = tokio::spawn(async {
        if let Err(e) = start_server().await {
            error!("Server error: {}", e);
        }
    });

    // Wait for server to start
    sleep(Duration::from_secs(2)).await;

    // Create multiple agents
    let mut agents = Vec::new();
    
    for i in 1..=3 {
        let agent_name = format!("Agent-{}", i);
        match ExampleAgent::new(&agent_name, "http://[::1]:50051").await {
            Ok(mut agent) => {
                info!("Created agent: {}", agent_name);
                
                // Start message listening in background
                let listen_handle = tokio::spawn(async move {
                    agent.listen_for_messages().await;
                });
                
                agents.push((agent, listen_handle));
            }
            Err(e) => {
                error!("Failed to create agent {}: {}", agent_name, e);
            }
        }
    }

    if agents.is_empty() {
        error!("No agents created, exiting");
        return Ok(());
    }

    // Simulate agent interactions
    for (i, (agent, _)) in agents.iter().enumerate() {
        // Send initial messages
        agent.send_message(
            &format!("Hello from {}!", agent.name),
            MessageType::StatusUpdate
        ).await?;

        // Update status
        agent.update_status(AgentStatus::Working).await?;

        // Create a session
        let session_id = agent.create_session(
            &format!("session-{}", i),
            vec!["Agent-1".to_string(), "Agent-2".to_string(), "Agent-3".to_string()]
        ).await?;

        // Join the session
        agent.join_session(&session_id).await?;

        // Send session message
        agent.send_session_message(
            &session_id,
            &format!("Session message from {}", agent.name)
        ).await?;

        sleep(Duration::from_millis(500)).await;
    }

    // Let agents communicate for a while
    info!("Agents are communicating...");
    sleep(Duration::from_secs(10)).await;

    // Send some more messages
    for (agent, _) in &agents {
        agent.send_message(
            &format!("Final message from {}", agent.name),
            MessageType::TaskCompletion
        ).await?;
    }

    // Wait a bit more
    sleep(Duration::from_secs(5)).await;

    info!("Example completed successfully!");
    
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

    // Run the example
    run_example().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        // This test would require a running server
        // For now, we'll just test the error handling
        let result = ExampleAgent::new("test-agent", "http://localhost:9999").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_server_config() {
        let config = GrpcServerConfig::default();
        assert_eq!(config.bind_address, "[::1]:50051");
        assert_eq!(config.max_concurrent_requests, 1000);
        assert!(config.enable_compression);
        assert!(!config.enable_tls);
    }
} 