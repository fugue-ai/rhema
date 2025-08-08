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

use rhema_core::RhemaResult;
use rhema_ai::agent::real_time_coordination::{
    AgentInfo, AgentStatus, AgentMessage, MessageType, MessagePriority,
    CoordinationConfig, AgentPerformanceMetrics
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("🚀 Simple Rhema Coordination Integration Example");
    println!("================================================");

    // Create coordination configuration
    let coordination_config = CoordinationConfig {
        max_message_history: 100,
        message_timeout_seconds: 30,
        heartbeat_interval_seconds: 10,
        agent_timeout_seconds: 60,
        max_session_participants: 5,
        enable_encryption: false,
        enable_compression: true,
    };

    println!("✅ Coordination configuration created");

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

    println!("✅ Test agent created");

    // Create a test message
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

    println!("✅ Test message created");

    println!("📊 Example Data:");
    println!("  Agent: {} ({}) - {:?}", agent.name, agent.id, agent.status);
    println!("  Message: {} - {}", message.id, message.content);
    println!("  Config: {} participants, {}s timeout", 
             coordination_config.max_session_participants, 
             coordination_config.message_timeout_seconds);

    println!("🎉 Example completed successfully!");
    Ok(())
} 