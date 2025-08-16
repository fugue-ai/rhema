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

//! Example demonstrating real gRPC coordination client integration
//!
//! This example shows how to use the real gRPC coordination client with the
//! syneidesis-grpc backend for agent coordination and communication.
//!
//! ## Prerequisites
//!
//! 1. A running syneidesis coordination server
//! 2. The coordination feature enabled: `cargo run --features coordination`
//!
//! ## Usage
//!
//! ```bash
//! # Start the coordination server (in another terminal)
//! cd crates/syneidesis/grpc
//! cargo run --bin coordination-server
//!
//! # Run this example
//! cargo run --example grpc_coordination_example --features coordination
//! ```

use rhema_core::{
    coordination::{
        create_coordination_manager, AgentInfo, AgentMessage, CoordinationConfig, MessageType,
    },
    RhemaResult,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting gRPC Coordination Example");
    println!("Make sure you have a syneidesis coordination server running on localhost:50051");

    // Create coordination configuration for gRPC
    let config = CoordinationConfig {
        enabled: true,
        server_endpoint: "127.0.0.1:50051".to_string(),
        timeout_seconds: 30,
        retry_config: Default::default(),
        health_check_config: Default::default(),
        tls_config: None, // Disable TLS for local development
    };

    println!(
        "📡 Connecting to coordination server at {}",
        config.server_endpoint
    );

    // Create coordination manager with gRPC client
    let mut manager = create_coordination_manager(config).await?;

    println!("✅ Connected to coordination server");

    // Create and register multiple agents
    let agents = vec![
        AgentInfo::new("code-review-agent".to_string(), "code-review".to_string()),
        AgentInfo::new("test-agent".to_string(), "testing".to_string()),
        AgentInfo::new("deploy-agent".to_string(), "deployment".to_string()),
    ];

    for agent in &agents {
        println!(
            "🤖 Registering agent: {} ({})",
            agent.name, agent.agent_type
        );
        manager.register_agent(agent.clone()).await?;
        sleep(Duration::from_millis(100)).await;
    }

    // Register all agents
    println!("📋 Registering all agents");
    for agent in &agents {
        println!("👥 Registering agent: {}", agent.name);
        manager.register_agent(agent.clone()).await?;
        sleep(Duration::from_millis(100)).await;
    }

    // Send various types of messages
    let messages = vec![
        AgentMessage::new(
            agents[0].id.clone(),
            MessageType::TaskAssignment,
            "Please review the authentication module".to_string(),
        ),
        AgentMessage::new(
            agents[1].id.clone(),
            MessageType::StatusUpdate,
            "Starting test suite execution".to_string(),
        ),
        AgentMessage::new(
            agents[2].id.clone(),
            MessageType::CoordinationRequest,
            "Requesting deployment approval".to_string(),
        ),
    ];

    for message in &messages {
        println!("📤 Sending message: {:?}", message.message_type);
        manager.send_message(message.clone()).await?;
        sleep(Duration::from_millis(200)).await;
    }

    // Send additional messages
    let additional_messages = vec![
        AgentMessage::new(
            agents[0].id.clone(),
            MessageType::TaskCompletion,
            "Code review completed - all tests passing".to_string(),
        ),
        AgentMessage::new(
            agents[1].id.clone(),
            MessageType::TaskCompletion,
            "Test suite completed - 100% coverage achieved".to_string(),
        ),
        AgentMessage::new(
            agents[2].id.clone(),
            MessageType::TaskCompletion,
            "Deployment successful - service is live".to_string(),
        ),
    ];

    for message in &additional_messages {
        println!("📤 Sending additional message: {:?}", message.message_type);
        manager.send_message(message.clone()).await?;
        sleep(Duration::from_millis(200)).await;
    }

    // Get connection statistics
    println!("📈 Getting connection statistics");
    let stats = manager.get_connection_stats().await;
    println!("Connection Stats:");
    println!("  Connected: {}", stats.is_connected);
    println!("  Messages Sent: {}", stats.messages_sent);
    println!("  Messages Received: {}", stats.messages_received);
    println!("  Uptime: {} seconds", stats.uptime_seconds);
    if let Some(latency) = stats.latency_ms {
        println!("  Average Latency: {latency}ms");
    }

    println!("✅ gRPC Coordination Example completed successfully!");
    println!("📊 Final Statistics:");
    let final_stats = manager.get_connection_stats().await;
    println!("  Total Messages Sent: {}", final_stats.messages_sent);
    println!(
        "  Total Messages Received: {}",
        final_stats.messages_received
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grpc_coordination_config() {
        let config = CoordinationConfig {
            enabled: true,
            server_endpoint: "127.0.0.1:50051".to_string(),
            timeout_seconds: 30,
            ..Default::default()
        };

        assert!(config.enabled);
        assert_eq!(config.server_endpoint, "127.0.0.1:50051");
        assert_eq!(config.timeout_seconds, 30);
    }

    #[tokio::test]
    async fn test_agent_creation() {
        let agent = AgentInfo::new("test-agent".to_string(), "testing".to_string());

        assert_eq!(agent.name, "test-agent");
        assert_eq!(agent.agent_type, "testing");
        assert!(agent.is_online);
        assert!(matches!(
            agent.status,
            rhema_core::coordination::AgentStatus::Idle
        ));
    }

    #[tokio::test]
    async fn test_message_creation() {
        let message = AgentMessage::new(
            "sender-id".to_string(),
            MessageType::TaskAssignment,
            "Test message".to_string(),
        );

        assert_eq!(message.sender_id, "sender-id");
        assert!(matches!(message.message_type, MessageType::TaskAssignment));
        assert_eq!(message.content, "Test message");
        assert!(matches!(message.priority, MessagePriority::Normal));
    }
}
