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

use syneidesis_coordination::{
    agent::coordinator::AgentCoordinator,
    agent::state::{AgentHealth, AgentState, AgentStatus},
    config::CoordinationConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Syneidesis Coordination Example");

    // Create configuration
    let config = CoordinationConfig::default();

    // Create the agent coordinator
    let mut coordinator = AgentCoordinator::with_config(config);

    // Start the coordinator
    coordinator.start().await?;
    println!("✅ Coordinator started successfully");

    // Create and register agents
    let agent1 = AgentState::new(
        "agent-001".to_string(),
        "Worker Agent 1".to_string(),
        "worker".to_string(),
    );

    let agent2 = AgentState::new(
        "agent-002".to_string(),
        "Worker Agent 2".to_string(),
        "worker".to_string(),
    );

    // Register agents
    coordinator.register_agent(agent1.clone()).await?;
    coordinator.register_agent(agent2.clone()).await?;
    println!("✅ Agents registered successfully");

    // Update agent status
    let mut agent1_updated = agent1.clone();
    agent1_updated.update_status(AgentStatus::Busy);
    agent1_updated.update_health(AgentHealth::Healthy);
    agent1_updated.set_current_task(Some("task-001".to_string()));

    coordinator
        .update_agent_state(&agent1.id, agent1_updated)
        .await?;
    println!("✅ Agent status updated");

    // Get agent information
    let retrieved_agent = coordinator.get_agent(&agent1.id).await?;
    println!("📊 Agent Info:");
    println!("   ID: {}", retrieved_agent.id);
    println!("   Name: {}", retrieved_agent.name);
    println!("   Status: {:?}", retrieved_agent.status);
    println!("   Health: {:?}", retrieved_agent.health);
    println!("   Current Task: {:?}", retrieved_agent.current_task);

    // Get all available agents
    let available_agents = coordinator.get_available_agents().await?;
    println!("📋 Available Agents: {}", available_agents.len());

    for agent in available_agents {
        println!("   - {} ({}) - {:?}", agent.name, agent.id, agent.status);
    }

    // Get statistics
    let stats = coordinator.get_statistics().await?;
    println!("📈 Statistics:");
    println!("   Total Agents: {}", stats.total_agents);
    println!("   Available Agents: {}", stats.available_agents);
    println!("   Success Rate: {:.2}%", stats.success_rate);

    // Unregister agents
    coordinator.unregister_agent(&agent1.id).await?;
    coordinator.unregister_agent(&agent2.id).await?;
    println!("✅ Agents unregistered successfully");

    // Stop the coordinator
    coordinator.stop().await?;
    println!("🛑 Coordinator stopped");

    println!("🎉 Example completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_coordination() {
        let config = CoordinationConfig::default();
        let mut coordinator = AgentCoordinator::with_config(config);

        // Test coordinator creation
        assert!(coordinator.start().await.is_ok());

        // Test agent registration
        let agent = AgentState::new(
            "test-agent".to_string(),
            "Test Agent".to_string(),
            "worker".to_string(),
        );

        assert!(coordinator.register_agent(agent.clone()).await.is_ok());

        // Test agent retrieval
        let retrieved = coordinator.get_agent(&agent.id).await;
        assert!(retrieved.is_ok());

        // Test agent unregistration
        assert!(coordinator.unregister_agent(&agent.id).await.is_ok());

        // Test coordinator shutdown
        assert!(coordinator.stop().await.is_ok());
    }

    #[tokio::test]
    async fn test_agent_state_management() {
        let mut agent = AgentState::new(
            "test-agent".to_string(),
            "Test Agent".to_string(),
            "worker".to_string(),
        );

        // Test initial state
        assert_eq!(agent.status, AgentStatus::Idle);
        assert_eq!(agent.health, AgentHealth::Healthy);

        // Test status updates
        agent.update_status(AgentStatus::Busy);
        assert_eq!(agent.status, AgentStatus::Busy);

        // Test health updates
        agent.update_health(AgentHealth::Degraded);
        assert_eq!(agent.health, AgentHealth::Degraded);

        // Test task assignment
        agent.set_current_task(Some("test-task".to_string()));
        assert_eq!(agent.current_task, Some("test-task".to_string()));

        // Test validation
        assert!(agent.validate().is_ok());
    }
}
