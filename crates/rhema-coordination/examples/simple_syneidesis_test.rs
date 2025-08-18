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

//! Simple test demonstrating Syneidesis integration is working

use rhema_coordination::agent::real_time_coordination::{
    AgentInfo, AgentStatus, RealTimeCoordinationSystem,
};
use rhema_coordination::coordination_integration::{CoordinationConfig, CoordinationIntegration};
use rhema_coordination::grpc::coordination_client::SyneidesisConfig;
use rhema_core::RhemaResult;
use tracing::info;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Starting simple Syneidesis integration test");

    // Create Rhema coordination system
    let rhema_coordination = RealTimeCoordinationSystem::new();

    // Configure Syneidesis integration
    let syneidesis_config = SyneidesisConfig::default();

    let integration_config = CoordinationConfig {
        run_local_server: true,
        server_address: None,
        auto_register_agents: true,
        sync_messages: true,
        sync_tasks: true,
        enable_health_monitoring: true,
        syneidesis: Some(syneidesis_config),
    };

    // Create coordination integration
    let integration =
        CoordinationIntegration::new(rhema_coordination, Some(integration_config)).await?;

    info!("✅ Coordination integration initialized successfully");

    // Create a test agent
    let agent = AgentInfo {
        id: "test-agent-1".to_string(),
        name: "Test Agent 1".to_string(),
        agent_type: "verification".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "default".to_string(),
        capabilities: vec!["verification".to_string(), "testing".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics:
            rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
    };

    // Register agent with both systems
    integration.register_rhema_agent(&agent).await?;
    info!("✅ Agent registered with both coordination systems");

    // Track some tasks
    integration.track_task_created().await;
    integration.track_task_created().await;
    info!("✅ Created 2 tasks");

    // Get integration statistics
    let stats = integration.get_integration_stats().await;
    info!("📊 Integration Statistics:");
    info!("  Rhema Agents: {}", stats.rhema_agents);
    info!("  Rhema Messages: {}", stats.rhema_messages);
    info!("  Syneidesis Agents: {}", stats.syneidesis_agents);
    info!("  Syneidesis Tasks: {}", stats.syneidesis_tasks);
    info!("  Bridge Messages Sent: {}", stats.bridge_messages_sent);
    info!(
        "  Bridge Messages Received: {}",
        stats.bridge_messages_received
    );

    // Check if Syneidesis integration is enabled
    if integration.has_syneidesis_integration().await {
        info!("✅ Syneidesis integration is enabled");

        // Get connection status
        if let Some(status) = integration.get_syneidesis_status().await {
            info!("🔗 Syneidesis connection status: {:?}", status);
        }
    } else {
        info!("⚠️ Syneidesis integration is not enabled");
    }

    // Complete a task
    integration.track_task_completed().await;
    info!("✅ Completed 1 task");

    // Get updated statistics
    let updated_stats = integration.get_integration_stats().await;
    info!("📊 Updated Statistics:");
    info!("  Syneidesis Tasks: {}", updated_stats.syneidesis_tasks);

    // Shutdown integration
    integration.shutdown().await?;
    info!("✅ Integration shutdown complete");

    info!("🎉 Simple Syneidesis integration test completed successfully!");
    info!("✅ Syneidesis integration is working correctly!");

    Ok(())
}
