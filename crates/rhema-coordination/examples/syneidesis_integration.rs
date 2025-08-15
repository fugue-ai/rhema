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

//! Example demonstrating Syneidesis integration with Rhema coordination
//!
//! This example shows how to:
//! 1. Initialize Rhema coordination with Syneidesis integration
//! 2. Register agents with both systems
//! 3. Send messages through the integration layer
//! 4. Monitor coordination statistics

use rhema_coordination::{
    agent::real_time_coordination::{
        AgentInfo, AgentMessage, AgentStatus, MessagePriority, MessageType,
        RealTimeCoordinationSystem,
    },
    coordination_integration::{CoordinationConfig, CoordinationIntegration, SyneidesisConfig},
};
use rhema_core::RhemaResult;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Rhema-Syneidesis integration example");

    // Create Rhema coordination system
    let rhema_coordination = RealTimeCoordinationSystem::new();

    // Configure Syneidesis integration
    let syneidesis_config = SyneidesisConfig {
        enabled: true,
        server_address: Some("http://127.0.0.1:50051".to_string()),
        auto_register_agents: true,
        sync_messages: true,
        enable_health_monitoring: true,
        timeout_seconds: 30,
        max_retries: 3,
        enable_tls: false,
        tls_cert_path: None,
    };

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

    info!("✅ Coordination integration initialized");

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

    // Create a test message
    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::Normal,
        sender_id: "system".to_string(),
        recipient_ids: vec![agent.id.clone()],
        content: "Please verify the test data".to_string(),
        payload: None,
        timestamp: chrono::Utc::now(),
        requires_ack: false,
        expires_at: None,
    };

    // Send message through integration
    integration.send_message_with_coordination(message).await?;
    info!("✅ Message sent through coordination integration");

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

    // Start health monitoring
    integration.start_health_monitoring().await?;
    info!("✅ Health monitoring started");

    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Complete a task
    integration.track_task_completed().await;
    info!("✅ Completed 1 task");

    // Get updated statistics
    let updated_stats = integration.get_integration_stats().await;
    info!("📊 Updated Statistics:");
    info!("  Syneidesis Tasks: {}", updated_stats.syneidesis_tasks);

    // Check if Syneidesis integration is enabled
    if integration.has_syneidesis_integration() {
        info!("✅ Syneidesis integration is enabled");

        // Get connection status
        if let Some(status) = integration.get_syneidesis_status().await {
            info!("🔗 Syneidesis connection status: {:?}", status);
        }
    } else {
        warn!("⚠️ Syneidesis integration is not enabled");
    }

    // Shutdown integration
    integration.shutdown().await?;
    info!("✅ Integration shutdown complete");

    info!("🎉 Example completed successfully!");
    Ok(())
}
