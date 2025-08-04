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

use rhema::Rhema;
use rhema_ai::agent::real_time_coordination::{
    AgentInfo, AgentStatus, AgentMessage, MessageType, MessagePriority,
    CoordinationConfig, AgentPerformanceMetrics
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    println!("🚀 Simple Rhema Coordination Integration Example");
    println!("================================================");

    // Create Rhema instance
    let mut rhema = Rhema::new()?;
    println!("✅ Rhema instance created");

    // Initialize basic coordination system
    let coordination_config = CoordinationConfig {
        max_message_history: 100,
        message_timeout_seconds: 30,
        heartbeat_interval_seconds: 10,
        agent_timeout_seconds: 60,
        max_session_participants: 5,
        enable_encryption: false,
        enable_compression: true,
    };

    rhema.init_coordination(Some(coordination_config)).await?;
    println!("✅ Basic coordination system initialized");

    // Create test agent
    let agent = AgentInfo {
        id: "test-agent".to_string(),
        name: "Test Agent".to_string(),
        agent_type: "test".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "test".to_string(),
        capabilities: vec!["test".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: AgentPerformanceMetrics::default(),
    };

    // Register agent
    rhema.register_agent(agent).await?;
    println!("✅ Agent registered");

    // Create a coordination session
    let session_id = rhema.create_coordination_session(
        "Test Session".to_string(),
        vec!["test-agent".to_string()]
    ).await?;
    println!("✅ Coordination session created: {}", session_id);

    // Join session
    rhema.join_coordination_session(&session_id, "test-agent").await?;
    println!("✅ Agent joined session");

    // Send a test message
    let message = AgentMessage {
        id: "test-message-1".to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::Normal,
        sender_id: "test-agent".to_string(),
        recipient_ids: vec![],
        content: "Test message from coordination integration".to_string(),
        payload: Some(serde_json::json!({
            "test": "coordination_integration",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
        timestamp: chrono::Utc::now(),
        requires_ack: false,
        expires_at: None,
        metadata: HashMap::new(),
    };

    rhema.send_session_message(&session_id, message).await?;
    println!("✅ Message sent through coordination system");

    // Get coordination statistics
    let stats = rhema.get_coordination_stats().await?;
    println!("📊 Coordination Statistics:");
    println!("  Total Messages: {}", stats.total_messages);
    println!("  Active Agents: {}", stats.active_agents);
    println!("  Active Sessions: {}", stats.active_sessions);
    println!("  Average Response Time: {:.2}ms", stats.avg_response_time_ms);

    // Get all agents
    let agents = rhema.get_all_agents().await?;
    println!("👥 Registered Agents:");
    for agent in agents {
        println!("  {} ({}) - {:?}", agent.name, agent.id, agent.status);
    }

    // Update agent status
    rhema.update_agent_status("test-agent", AgentStatus::Busy).await?;
    println!("✅ Agent status updated to Busy");

    // Get updated agent info
    let agent_info = rhema.get_agent_info("test-agent").await?;
    if let Some(agent) = agent_info {
        println!("📋 Updated Agent Info:");
        println!("  Name: {}", agent.name);
        println!("  Status: {:?}", agent.status);
        println!("  Capabilities: {:?}", agent.capabilities);
    }

    // Start health monitoring
    rhema.start_coordination_health_monitoring().await?;
    println!("✅ Health monitoring started");

    // Wait a moment to see health monitoring in action
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Shutdown coordination
    rhema.shutdown_coordination().await?;
    println!("✅ Coordination system shutdown");

    println!("\n🎉 Simple coordination integration example completed successfully!");
    Ok(())
} 