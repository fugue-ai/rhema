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

//! Example demonstrating coordination client integration with Rhema Core
//!
//! This example shows how to:
//! 1. Configure and initialize a coordination manager
//! 2. Register agents with the coordination system
//! 3. Send messages between agents
//! 4. Create coordination sessions
//! 5. Monitor connection statistics

use rhema_core::{
    coordination::{
        create_coordination_manager, AgentInfo, AgentMessage, CoordinationConfig,
        CoordinationManager, MessagePriority, MessageType,
    },
    RhemaResult,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Rhema Core Coordination Integration Example");
    println!("=============================================");

    // Create coordination configuration
    let config = CoordinationConfig {
        enabled: true,
        server_endpoint: "http://localhost:50051".to_string(),
        timeout_seconds: 30,
        retry_config: rhema_core::coordination::RetryConfig {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        },
        health_check_config: rhema_core::coordination::HealthCheckConfig {
            enabled: true,
            interval_seconds: 30,
            timeout_seconds: 5,
        },
        tls_config: None,
    };

    println!("📋 Configuration:");
    println!("  - Enabled: {}", config.enabled);
    println!("  - Server Endpoint: {}", config.server_endpoint);
    println!("  - Timeout: {} seconds", config.timeout_seconds);

    // Create coordination manager
    println!("\n🔧 Creating coordination manager...");
    let mut manager = create_coordination_manager(config).await?;

    if !manager.is_enabled() {
        println!("⚠️  Coordination is disabled. Exiting.");
        return Ok(());
    }

    // Create agent information
    let agent1 = AgentInfo::new("code-review-agent".to_string(), "code-review".to_string())
        .with_scope("backend".to_string())
        .with_capability("code-review".to_string())
        .with_capability("security-analysis".to_string())
        .with_metadata("version".to_string(), "1.0.0".to_string());

    let agent2 = AgentInfo::new("test-agent".to_string(), "testing".to_string())
        .with_scope("backend".to_string())
        .with_capability("unit-testing".to_string())
        .with_capability("integration-testing".to_string())
        .with_metadata("version".to_string(), "1.0.0".to_string());

    let agent3 = AgentInfo::new("deployment-agent".to_string(), "deployment".to_string())
        .with_scope("infrastructure".to_string())
        .with_capability("deployment".to_string())
        .with_capability("monitoring".to_string())
        .with_metadata("version".to_string(), "1.0.0".to_string());

    println!("\n🤖 Registering agents...");
    println!("  - {}", agent1.name);
    println!("  - {}", agent2.name);
    println!("  - {}", agent3.name);

    // Register agents
    manager.register_agent(agent1.clone()).await?;
    manager.register_agent(agent2.clone()).await?;
    manager.register_agent(agent3.clone()).await?;

    println!("✅ All agents registered successfully!");

    // Send messages between agents
    println!("\n📨 Sending messages between agents...");

    // Agent 1 sends a task assignment to Agent 2
    let task_assignment = AgentMessage::new(
        agent1.id.clone(),
        MessageType::TaskAssignment,
        "Please review the authentication module for security vulnerabilities".to_string(),
    )
    .to_recipients(vec![agent2.id.clone()])
    .with_priority(MessagePriority::High)
    .with_metadata("task_id".to_string(), "auth-review-001".to_string());

    manager.send_message(task_assignment).await?;
    println!(
        "  📤 {} → {}: Task assignment sent",
        agent1.name, agent2.name
    );

    // Agent 2 sends a status update
    let status_update = AgentMessage::new(
        agent2.id.clone(),
        MessageType::StatusUpdate,
        "Starting security review of authentication module".to_string(),
    )
    .to_recipients(vec![agent1.id.clone()])
    .with_priority(MessagePriority::Normal)
    .with_metadata("status".to_string(), "in-progress".to_string());

    manager.send_message(status_update).await?;
    println!("  📤 {} → {}: Status update sent", agent2.name, agent1.name);

    // Agent 3 sends a coordination request
    let coordination_request = AgentMessage::new(
        agent3.id.clone(),
        MessageType::CoordinationRequest,
        "Requesting coordination for deployment of new authentication system".to_string(),
    )
    .to_recipients(vec![agent1.id.clone(), agent2.id.clone()])
    .with_priority(MessagePriority::Critical)
    .with_metadata("deployment_id".to_string(), "auth-deploy-001".to_string());

    manager.send_message(coordination_request).await?;
    println!(
        "  📤 {} → {}: Coordination request sent",
        agent3.name, "all"
    );

    // Create a coordination session
    println!("\n👥 Creating coordination session...");
    let session_topic = "authentication-system-deployment".to_string();
    let participants = vec![agent1.id.clone(), agent2.id.clone(), agent3.id.clone()];

    // Note: This would require the coordination client to be properly initialized
    // For now, we'll just demonstrate the API
    println!("  📋 Session Topic: {}", session_topic);
    println!("  👥 Participants: {}", participants.join(", "));

    // Wait a bit to simulate processing
    sleep(Duration::from_secs(2)).await;

    // Get connection statistics
    println!("\n📊 Connection Statistics:");
    let stats = manager.get_connection_stats().await;
    println!("  - Connected: {}", stats.is_connected);
    println!("  - Messages Sent: {}", stats.messages_sent);
    println!("  - Messages Received: {}", stats.messages_received);
    println!("  - Uptime: {} seconds", stats.uptime_seconds);
    if let Some(latency) = stats.latency_ms {
        println!("  - Latency: {} ms", latency);
    }
    if let Some(heartbeat) = stats.last_heartbeat {
        println!("  - Last Heartbeat: {}", heartbeat);
    }

    // Demonstrate error handling
    println!("\n🔍 Testing error handling...");
    let invalid_message = AgentMessage::new(
        "invalid-agent-id".to_string(),
        MessageType::TaskAssignment,
        "This message should fail".to_string(),
    );

    match manager.send_message(invalid_message).await {
        Ok(_) => println!("  ✅ Message sent successfully (unexpected)"),
        Err(e) => println!("  ❌ Expected error: {}", e),
    }

    println!("\n✅ Coordination integration example completed successfully!");
    println!("🎉 The coordination client is now integrated with Rhema Core!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_coordination_integration() {
        // Test with disabled coordination
        let config = CoordinationConfig {
            enabled: false,
            ..Default::default()
        };

        let manager = create_coordination_manager(config).await.unwrap();
        assert!(!manager.is_enabled());
    }

    #[tokio::test]
    async fn test_agent_message_creation() {
        let agent = AgentInfo::new("test-agent".to_string(), "test".to_string());
        let message = AgentMessage::new(
            agent.id.clone(),
            MessageType::TaskAssignment,
            "Test message".to_string(),
        );

        assert_eq!(message.sender_id, agent.id);
        assert!(matches!(message.message_type, MessageType::TaskAssignment));
        assert_eq!(message.content, "Test message");
    }
}
