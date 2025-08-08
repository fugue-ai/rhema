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

use tracing::info;

use syneidesis_coordination::{
    agent::state::AgentState, config::CoordinationConfig, grpc::server::CoordinationServer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting gRPC Coordination Example");

    // Create configuration
    let config = CoordinationConfig::default();

    // Create and start the coordination server
    let mut server = CoordinationServer::new(config)?;
    server.start().await?;

    info!("Coordination server started successfully");

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    info!("Shutting down server...");

    // Stop the server
    server.stop().await?;

    Ok(())
}

/// Example client that demonstrates the coordination library
#[tokio::main]
async fn client_example() -> Result<(), Box<dyn std::error::Error>> {
    use syneidesis_coordination::grpc::client::CoordinationClient;

    let mut client =
        CoordinationClient::new(syneidesis_coordination::grpc::GrpcClientConfig::default()).await?;

    // Create an agent
    let agent_state = AgentState::new(
        "agent-001".to_string(),
        "Test Agent".to_string(),
        "worker".to_string(),
    );

    // Register the agent
    let agent_info = syneidesis_coordination::grpc::coordination::AgentInfo {
        id: agent_state.id.clone(),
        name: agent_state.name.clone(),
        agent_type: agent_state.agent_type.clone(),
        status: syneidesis_coordination::grpc::coordination::AgentStatus::Idle as i32,
        health: syneidesis_coordination::grpc::coordination::AgentHealth::Healthy as i32,
        current_task_id: None,
        assigned_scope: "default".to_string(),
        capabilities: vec!["task_execution".to_string()],
        last_heartbeat: None,
        is_online: true,
        performance_metrics: None,
        priority: 1,
        version: "1.0.0".to_string(),
        endpoint: None,
        metadata: std::collections::HashMap::new(),
        created_at: None,
        last_updated: None,
    };

    let response = client.register_agent(agent_info).await?;
    info!("Agent registered: {:?}", response);

    // Update agent status
    let response = client
        .update_agent_status(
            agent_state.id.clone(),
            syneidesis_coordination::grpc::AgentStatus::Busy,
            syneidesis_coordination::grpc::AgentHealth::Healthy,
            Some("task-001".to_string()),
        )
        .await?;
    info!("Agent status updated: {:?}", response);

    // Send a message
    let message = syneidesis_coordination::grpc::coordination::AgentMessage {
        id: "msg-001".to_string(),
        sender_id: agent_state.id.clone(),
        recipient_ids: vec!["agent-002".to_string()],
        message_type: syneidesis_coordination::grpc::coordination::MessageType::Custom as i32,
        priority: 0,
        content: "Hello from agent-001!".to_string(),
        timestamp: Some(std::time::SystemTime::now().into()),
        requires_ack: false,
        expires_at: None,
        payload: None,
        metadata: std::collections::HashMap::new(),
    };

    let response = client.send_message(message).await?;
    info!("Message sent: {:?}", response);

    // Get agent info
    let response = client.get_agent_info(agent_state.id.clone()).await?;
    info!("Agent info: {:?}", response);

    // Get all agents
    let response = client.get_all_agents().await?;
    info!("All agents: {:?}", response);

    // Get statistics
    let response = client.get_stats().await?;
    info!("Statistics: {:?}", response);

    // Unregister the agent
    let response = client.unregister_agent(agent_state.id.clone()).await?;
    info!("Agent unregistered: {:?}", response);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_lifecycle() {
        // This test demonstrates the complete agent lifecycle
        let config = CoordinationConfig::default();
        let _service = RealTimeCoordinationServiceImpl::new(config).unwrap();

        // Create an agent
        let agent = AgentState::new(
            "test-agent".to_string(),
            "Test Agent".to_string(),
            "worker".to_string(),
        );

        // Test agent validation
        assert!(agent.validate().is_ok());

        // Test agent status updates
        let mut agent = agent;
        agent.update_status(syneidesis_coordination::agent::AgentStatus::Busy);
        agent.update_health(syneidesis_coordination::agent::AgentHealth::Healthy);

        assert_eq!(
            agent.status,
            syneidesis_coordination::agent::AgentStatus::Busy
        );
        assert_eq!(
            agent.health,
            syneidesis_coordination::agent::AgentHealth::Healthy
        );
    }
}
