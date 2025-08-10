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

use rhema_coordination::grpc::coordination_client::SyneidesisConfig;
use rhema_coordination::{
    agent::real_time_coordination::{
        AgentInfo, AgentMessage, AgentStatus, MessagePriority, MessageType,
    },
    ai_service::{AIService, AIServiceConfig},
    CoordinationConfig,
};
use rhema_core::RhemaResult;
use tracing::info;

#[tokio::test]
async fn test_coordination_integration_creation() -> RhemaResult<()> {
    info!("🧪 Testing coordination integration creation");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;
    assert!(service.has_coordination_integration());

    info!("✅ Coordination integration creation test passed");
    Ok(())
}

#[tokio::test]
async fn test_syneidesis_integration_creation() -> RhemaResult<()> {
    info!("🧪 Testing Syneidesis integration creation");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;
    assert!(service.has_coordination_integration());

    // Check Syneidesis status
    let status = service.get_syneidesis_status().await;
    assert!(status.is_some());

    info!("✅ Syneidesis integration creation test passed");
    Ok(())
}

#[tokio::test]
async fn test_agent_registration_with_coordination() -> RhemaResult<()> {
    info!("🧪 Testing agent registration with coordination");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

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
        performance_metrics:
            rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
    };

    let result = service.register_agent_with_coordination(agent_info).await;
    assert!(result.is_ok());

    info!("✅ Agent registration with coordination test passed");
    Ok(())
}

#[tokio::test]
async fn test_message_sending_with_coordination() -> RhemaResult<()> {
    info!("🧪 Testing message sending with coordination");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
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

    let result = service.send_message_with_coordination(message).await;
    assert!(result.is_ok());

    info!("✅ Message sending with coordination test passed");
    Ok(())
}

#[tokio::test]
async fn test_session_creation_with_coordination() -> RhemaResult<()> {
    info!("🧪 Testing session creation with coordination");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

    let session_id = service
        .create_session(
            "Test Session".to_string(),
            vec!["agent-1".to_string(), "agent-2".to_string()],
        )
        .await?;

    assert!(!session_id.is_empty());

    info!("✅ Session creation with coordination test passed");
    Ok(())
}

#[tokio::test]
async fn test_session_join_and_message() -> RhemaResult<()> {
    info!("🧪 Testing session join and message sending");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

    // Create session
    let session_id = service
        .create_session(
            "Test Session".to_string(),
            vec!["agent-1".to_string(), "agent-2".to_string()],
        )
        .await?;

    // Join session
    let join_result = service.join_session(&session_id, "agent-1").await;
    assert!(join_result.is_ok());

    // Send session message
    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
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

    let send_result = service.send_session_message(&session_id, message).await;
    assert!(send_result.is_ok());

    info!("✅ Session join and message test passed");
    Ok(())
}

#[tokio::test]
async fn test_coordination_statistics() -> RhemaResult<()> {
    info!("🧪 Testing coordination statistics");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

    // Register an agent
    let agent_info = AgentInfo {
        id: "stats-test-agent".to_string(),
        name: "Stats Test Agent".to_string(),
        agent_type: "test".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "default".to_string(),
        capabilities: vec!["test".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics:
            rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
    };

    service.register_agent_with_coordination(agent_info).await?;

    // Send a message
    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::StatusUpdate,
        priority: MessagePriority::Normal,
        sender_id: "system".to_string(),
        recipient_ids: vec!["stats-test-agent".to_string()],
        content: "Statistics test message".to_string(),
        payload: None,
        timestamp: chrono::Utc::now(),
        requires_ack: false,
        expires_at: None,
        metadata: std::collections::HashMap::new(),
    };

    service.send_message_with_coordination(message).await?;

    // Get statistics
    let stats = service.get_coordination_stats().await;
    assert!(stats.is_some());

    let stats = stats.unwrap();
    assert!(stats.rhema_agents > 0);
    assert!(stats.bridge_messages_sent > 0);

    info!("✅ Coordination statistics test passed");
    Ok(())
}

#[tokio::test]
async fn test_coordination_health_monitoring() -> RhemaResult<()> {
    info!("🧪 Testing coordination health monitoring");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

    // Start health monitoring
    let health_result = service.start_coordination_health_monitoring().await;
    assert!(health_result.is_ok());

    // Wait a bit for health monitoring to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    info!("✅ Coordination health monitoring test passed");
    Ok(())
}

#[tokio::test]
async fn test_coordination_shutdown() -> RhemaResult<()> {
    info!("🧪 Testing coordination shutdown");

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
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

    // Shutdown coordination
    let shutdown_result = service.shutdown_coordination().await;
    assert!(shutdown_result.is_ok());

    info!("✅ Coordination shutdown test passed");
    Ok(())
}

#[tokio::test]
async fn test_coordination_disabled() -> RhemaResult<()> {
    info!("🧪 Testing coordination disabled functionality");

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
        enable_coordination_integration: false,
        coordination_config: None,
        enable_advanced_conflict_prevention: false,
        advanced_conflict_prevention_config: None,
    };

    let service = AIService::new(config).await?;

    // Check that coordination is disabled
    assert!(!service.has_coordination_integration());

    // Try to register an agent (should fail)
    let agent_info = AgentInfo {
        id: "disabled-test-agent".to_string(),
        name: "Disabled Test Agent".to_string(),
        agent_type: "test".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "default".to_string(),
        capabilities: vec!["test".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics:
            rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
    };

    let register_result = service.register_agent_with_coordination(agent_info).await;
    assert!(register_result.is_err());

    // Try to send a message (should fail)
    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::Normal,
        sender_id: "system".to_string(),
        recipient_ids: vec!["agent-1".to_string()],
        content: "Test message".to_string(),
        payload: None,
        timestamp: chrono::Utc::now(),
        requires_ack: false,
        expires_at: None,
        metadata: std::collections::HashMap::new(),
    };

    let send_result = service.send_message_with_coordination(message).await;
    assert!(send_result.is_err());

    // Get statistics (should return None)
    let stats = service.get_coordination_stats().await;
    assert!(stats.is_none());

    info!("✅ Coordination disabled test passed");
    Ok(())
}
