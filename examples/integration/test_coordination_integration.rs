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

use rhema_ai::{
    ai_service::{AIService, AIServiceConfig},
    coordination_integration::CoordinationConfig,
    agent::real_time_coordination::{AgentInfo, AgentStatus, AgentMessage, MessageType, MessagePriority, AgentPerformanceMetrics},
};
use rhema_ai::grpc::coordination_client::SyneidesisConfig;
use rhema_core::RhemaResult;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("🚀 Testing Rhema Coordination Integration");
    
    // Test 1: Basic coordination integration
    test_basic_coordination_integration().await?;
    
    // Test 2: Syneidesis integration
    test_syneidesis_integration().await?;
    
    // Test 3: Agent registration and messaging
    test_agent_registration_and_messaging().await?;
    
    // Test 4: Session management
    test_session_management().await?;
    
    println!("✅ All coordination integration tests completed successfully!");
    Ok(())
}

async fn test_basic_coordination_integration() -> RhemaResult<()> {
    println!("🧪 Test 1: Basic Coordination Integration");
    
    let config = AIServiceConfig {
        api_key: "test-key".to_string(),
        base_url: "https://api.openai.com".to_string(),
        timeout_seconds: 30,
        max_concurrent_requests: 10,
        rate_limit_per_minute: 60,
        cache_ttl_seconds: 3600,
        model_version: "1.0".to_string(),
        enable_caching: true,
        enable_rate_limiting: false,
        enable_monitoring: true,
        enable_lock_file_awareness: false,
        lock_file_path: None,
        auto_validate_lock_file: false,
        conflict_prevention_enabled: false,
        dependency_version_consistency: false,
        enable_agent_state_management: false,
        max_concurrent_agents: 5,
        max_block_time_seconds: 300,
        agent_persistence_config: None,
        enable_coordination_integration: true,
        coordination_config: Some(CoordinationConfig::default()),
    };

    let service = AIService::new(config).await?;
    assert!(service.has_coordination_integration());
    
    println!("✅ Basic coordination integration test passed");
    Ok(())
}

async fn test_syneidesis_integration() -> RhemaResult<()> {
    println!("🧪 Test 2: Syneidesis Integration");
    
    let mut coordination_config = CoordinationConfig::default();
    coordination_config.syneidesis = Some(SyneidesisConfig {
        enabled: true,
        ..Default::default()
    });
    
    let config = AIServiceConfig {
        api_key: "test-key".to_string(),
        base_url: "https://api.openai.com".to_string(),
        timeout_seconds: 30,
        max_concurrent_requests: 10,
        rate_limit_per_minute: 60,
        cache_ttl_seconds: 3600,
        model_version: "1.0".to_string(),
        enable_caching: true,
        enable_rate_limiting: false,
        enable_monitoring: true,
        enable_lock_file_awareness: false,
        lock_file_path: None,
        auto_validate_lock_file: false,
        conflict_prevention_enabled: false,
        dependency_version_consistency: false,
        enable_agent_state_management: false,
        max_concurrent_agents: 5,
        max_block_time_seconds: 300,
        agent_persistence_config: None,
        enable_coordination_integration: true,
        coordination_config: Some(coordination_config),
    };

    let service = AIService::new(config).await?;
    assert!(service.has_coordination_integration());
    
    // Check Syneidesis status
    let status = service.get_syneidesis_status().await;
    assert!(status.is_some());
    
    println!("✅ Syneidesis integration test passed");
    Ok(())
}

async fn test_agent_registration_and_messaging() -> RhemaResult<()> {
    println!("🧪 Test 3: Agent Registration and Messaging");
    
    let config = AIServiceConfig {
        api_key: "test-key".to_string(),
        base_url: "https://api.openai.com".to_string(),
        timeout_seconds: 30,
        max_concurrent_requests: 10,
        rate_limit_per_minute: 60,
        cache_ttl_seconds: 3600,
        model_version: "1.0".to_string(),
        enable_caching: true,
        enable_rate_limiting: false,
        enable_monitoring: true,
        enable_lock_file_awareness: false,
        lock_file_path: None,
        auto_validate_lock_file: false,
        conflict_prevention_enabled: false,
        dependency_version_consistency: false,
        enable_agent_state_management: false,
        max_concurrent_agents: 5,
        max_block_time_seconds: 300,
        agent_persistence_config: None,
        enable_coordination_integration: true,
        coordination_config: Some(CoordinationConfig::default()),
    };

    let service = AIService::new(config).await?;

    // Register an agent
    let agent_info = AgentInfo {
        id: "test-agent-1".to_string(),
        name: "Test Agent 1".to_string(),
        agent_type: "test".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "default".to_string(),
        capabilities: vec!["test".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: AgentPerformanceMetrics {
            tasks_completed: 0,
            tasks_failed: 0,
            avg_completion_time_seconds: 0.0,
            success_rate: 1.0,
            collaboration_score: 0.0,
            avg_response_time_ms: 0.0,
        },
    };

    service.register_agent_with_coordination(agent_info).await?;
    println!("✅ Agent registered successfully");

    // Send a message
    let message = AgentMessage {
        id: format!("msg-{}", chrono::Utc::now().timestamp()),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::High,
        sender_id: "system".to_string(),
        recipient_ids: vec!["test-agent-1".to_string()],
        content: "Test message".to_string(),
        payload: Some(serde_json::json!({
            "test": "data"
        })),
        timestamp: chrono::Utc::now(),
        requires_ack: true,
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        metadata: std::collections::HashMap::new(),
    };

    service.send_message_with_coordination(message).await?;
    println!("✅ Message sent successfully");

    // Get statistics
    let stats = service.get_coordination_stats().await;
    assert!(stats.is_some());
    
    let stats = stats.unwrap();
    println!("📊 Coordination Stats: {}", stats);
    
    println!("✅ Agent registration and messaging test passed");
    Ok(())
}

async fn test_session_management() -> RhemaResult<()> {
    println!("🧪 Test 4: Session Management");
    
    let config = AIServiceConfig {
        api_key: "test-key".to_string(),
        base_url: "https://api.openai.com".to_string(),
        timeout_seconds: 30,
        max_concurrent_requests: 10,
        rate_limit_per_minute: 60,
        cache_ttl_seconds: 3600,
        model_version: "1.0".to_string(),
        enable_caching: true,
        enable_rate_limiting: false,
        enable_monitoring: true,
        enable_lock_file_awareness: false,
        lock_file_path: None,
        auto_validate_lock_file: false,
        conflict_prevention_enabled: false,
        dependency_version_consistency: false,
        enable_agent_state_management: false,
        max_concurrent_agents: 5,
        max_block_time_seconds: 300,
        agent_persistence_config: None,
        enable_coordination_integration: true,
        coordination_config: Some(CoordinationConfig::default()),
    };

    let service = AIService::new(config).await?;

    // Create a session
    let session_id = service.create_session(
        "Test Session".to_string(),
        vec!["agent-1".to_string(), "agent-2".to_string()],
    ).await?;

    assert!(!session_id.is_empty());
    println!("✅ Session created: {}", session_id);

    // Join session
    service.join_session(&session_id, "agent-1").await?;
    println!("✅ Agent joined session");

    // Send session message
    let message = AgentMessage {
        id: format!("session-msg-{}", chrono::Utc::now().timestamp()),
        message_type: MessageType::CoordinationRequest,
        priority: MessagePriority::Normal,
        sender_id: "agent-1".to_string(),
        recipient_ids: vec!["agent-2".to_string()],
        content: "Session test message".to_string(),
        payload: Some(serde_json::json!({
            "session_id": session_id,
            "test": "session_message"
        })),
        timestamp: chrono::Utc::now(),
        requires_ack: true,
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        metadata: std::collections::HashMap::new(),
    };

    service.send_session_message(&session_id, message).await?;
    println!("✅ Session message sent successfully");
    
    println!("✅ Session management test passed");
    Ok(())
} 