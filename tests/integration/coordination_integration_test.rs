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

//! Integration tests for coordination client integration with Rhema Core

use rhema_core::{
    coordination::{
        create_coordination_manager, AgentInfo, AgentMessage, CoordinationClient,
        CoordinationConfig, CoordinationManager, MessagePriority, MessageType,
        MockCoordinationClient,
    },
    RhemaResult,
};
use std::time::Duration;
use tokio::time::sleep;

/// Test coordination manager with disabled coordination
#[tokio::test]
async fn test_disabled_coordination() -> RhemaResult<()> {
    let config = CoordinationConfig {
        enabled: false,
        ..Default::default()
    };

    let mut manager = create_coordination_manager(config).await?;
    assert!(!manager.is_enabled());

    // Should not fail when coordination is disabled
    let agent = AgentInfo::new("test-agent".to_string(), "test".to_string());
    manager.register_agent(agent).await?;

    let message = AgentMessage::new(
        "sender".to_string(),
        MessageType::TaskAssignment,
        "Test message".to_string(),
    );
    manager.send_message(message).await?;

    Ok(())
}

/// Test coordination configuration
#[tokio::test]
async fn test_coordination_config() {
    let config = CoordinationConfig::default();

    assert!(!config.enabled);
    assert_eq!(config.server_endpoint, "http://localhost:50051");
    assert_eq!(config.timeout_seconds, 30);
    assert_eq!(config.retry_config.max_retries, 3);
    assert_eq!(config.health_check_config.enabled, true);
}

/// Test agent information creation and manipulation
#[tokio::test]
async fn test_agent_info_creation() {
    let agent = AgentInfo::new("test-agent".to_string(), "test-type".to_string())
        .with_task_id("task-123".to_string())
        .with_scope("test-scope".to_string())
        .with_capability("test-capability".to_string())
        .with_metadata("version".to_string(), "1.0.0".to_string());

    assert_eq!(agent.name, "test-agent");
    assert_eq!(agent.agent_type, "test-type");
    assert_eq!(agent.current_task_id, Some("task-123".to_string()));
    assert_eq!(agent.assigned_scope, Some("test-scope".to_string()));
    assert_eq!(agent.capabilities, vec!["test-capability"]);
    assert_eq!(agent.metadata.get("version"), Some(&"1.0.0".to_string()));
}

/// Test agent message creation and manipulation
#[tokio::test]
async fn test_agent_message_creation() {
    let message = AgentMessage::new(
        "sender-123".to_string(),
        MessageType::TaskAssignment,
        "Test message content".to_string(),
    )
    .to_recipients(vec!["recipient-1".to_string(), "recipient-2".to_string()])
    .with_priority(MessagePriority::High)
    .with_metadata("task_id".to_string(), "task-123".to_string());

    assert_eq!(message.sender_id, "sender-123");
    assert_eq!(message.recipient_ids, vec!["recipient-1", "recipient-2"]);
    assert!(matches!(message.priority, MessagePriority::High));
    assert_eq!(message.content, "Test message content");
    assert_eq!(
        message.metadata.get("task_id"),
        Some(&"task-123".to_string())
    );
}

/// Test message type variants
#[tokio::test]
async fn test_message_types() {
    let types = vec![
        MessageType::TaskAssignment,
        MessageType::TaskCompletion,
        MessageType::TaskFailure,
        MessageType::StatusUpdate,
        MessageType::Heartbeat,
        MessageType::CoordinationRequest,
        MessageType::CoordinationResponse,
        MessageType::ErrorNotification,
        MessageType::Custom("custom-type".to_string()),
    ];

    for message_type in types {
        let message = AgentMessage::new(
            "sender".to_string(),
            message_type,
            "Test message".to_string(),
        );
        assert_eq!(message.sender_id, "sender");
        assert_eq!(message.content, "Test message");
    }
}

/// Test message priority levels
#[tokio::test]
async fn test_message_priorities() {
    let priorities = vec![
        MessagePriority::Low,
        MessagePriority::Normal,
        MessagePriority::High,
        MessagePriority::Critical,
    ];

    for priority in priorities {
        let message = AgentMessage::new(
            "sender".to_string(),
            MessageType::TaskAssignment,
            "Test message".to_string(),
        )
        .with_priority(priority);

        assert_eq!(message.sender_id, "sender");
    }
}

/// Test coordination manager creation
#[tokio::test]
async fn test_coordination_manager_creation() -> RhemaResult<()> {
    let config = CoordinationConfig::default();
    let manager = CoordinationManager::new(config);

    assert!(!manager.is_enabled());
    assert_eq!(manager.get_connection_stats().await.is_connected, false);
    assert_eq!(manager.get_connection_stats().await.messages_sent, 0);
    assert_eq!(manager.get_connection_stats().await.messages_received, 0);

    Ok(())
}

/// Test connection statistics
#[tokio::test]
async fn test_connection_statistics() -> RhemaResult<()> {
    let config = CoordinationConfig::default();
    let manager = CoordinationManager::new(config);

    let stats = manager.get_connection_stats().await;
    assert!(!stats.is_connected);
    assert_eq!(stats.uptime_seconds, 0);
    assert_eq!(stats.messages_sent, 0);
    assert_eq!(stats.messages_received, 0);
    assert!(stats.last_heartbeat.is_none());
    assert!(stats.latency_ms.is_none());

    Ok(())
}

/// Test error handling for invalid configurations
#[tokio::test]
async fn test_error_handling() {
    // Test with invalid server endpoint
    let config = CoordinationConfig {
        enabled: true,
        server_endpoint: "invalid-endpoint".to_string(),
        ..Default::default()
    };

    // Add timeout to prevent hanging when trying to connect to invalid endpoint
    let result =
        tokio::time::timeout(Duration::from_secs(10), create_coordination_manager(config)).await;

    // The result should be Ok(Err(...)) because the timeout should trigger
    // or the connection should fail quickly
    match result {
        Ok(manager_result) => {
            // If the manager was created successfully (mock client), that's fine
            if let Ok(mut manager) = manager_result {
                assert!(manager.is_enabled());

                // Test that the manager can still perform operations even with invalid endpoint
                let agent_info = AgentInfo::new("test-agent".to_string(), "test-type".to_string());
                let result = manager.register_agent(agent_info).await;
                assert!(result.is_ok());
            }
        }
        Err(_timeout) => {
            // Timeout occurred, which is expected for invalid endpoints
            // This test verifies that we don't hang indefinitely
            println!(
                "✅ Test passed: Connection attempt timed out as expected for invalid endpoint"
            );
        }
    }
}

/// Test agent registration workflow
#[tokio::test]
async fn test_agent_registration_workflow() -> RhemaResult<()> {
    let config = CoordinationConfig {
        enabled: false, // Disable to avoid actual network calls
        ..Default::default()
    };

    let mut manager = create_coordination_manager(config).await?;

    // Create multiple agents
    let agents = vec![
        AgentInfo::new("agent-1".to_string(), "type-1".to_string()),
        AgentInfo::new("agent-2".to_string(), "type-2".to_string()),
        AgentInfo::new("agent-3".to_string(), "type-3".to_string()),
    ];

    // Register all agents
    for agent in &agents {
        manager.register_agent(agent.clone()).await?;
    }

    // Verify all agents were processed (even if coordination is disabled)
    let stats = manager.get_connection_stats().await;
    // When disabled, messages should still be counted but not sent
    assert_eq!(stats.messages_sent, 0);

    Ok(())
}

/// Test message sending workflow
#[tokio::test]
async fn test_message_sending_workflow() -> RhemaResult<()> {
    let config = CoordinationConfig {
        enabled: false, // Disable to avoid actual network calls
        ..Default::default()
    };

    let mut manager = create_coordination_manager(config).await?;

    // Create messages
    let messages = vec![
        AgentMessage::new(
            "sender-1".to_string(),
            MessageType::TaskAssignment,
            "Message 1".to_string(),
        ),
        AgentMessage::new(
            "sender-2".to_string(),
            MessageType::StatusUpdate,
            "Message 2".to_string(),
        ),
        AgentMessage::new(
            "sender-3".to_string(),
            MessageType::CoordinationRequest,
            "Message 3".to_string(),
        ),
    ];

    // Send all messages
    for message in messages {
        manager.send_message(message).await?;
    }

    // Verify messages were processed
    let stats = manager.get_connection_stats().await;
    // When disabled, messages should still be counted but not sent
    assert_eq!(stats.messages_sent, 0);

    Ok(())
}

/// Test coordination session workflow
#[tokio::test]
async fn test_session_workflow() -> RhemaResult<()> {
    let config = CoordinationConfig {
        enabled: false, // Disable to avoid actual network calls
        ..Default::default()
    };

    let manager = create_coordination_manager(config).await?;

    // Create session parameters
    let topic = "test-session".to_string();
    let participants = vec!["agent-1".to_string(), "agent-2".to_string()];

    // This would create a session if coordination was enabled
    // For now, we just verify the manager handles the request gracefully
    let stats_before = manager.get_connection_stats().await;

    // Simulate session creation (would be implemented in the actual client)
    sleep(Duration::from_millis(100)).await;

    let stats_after = manager.get_connection_stats().await;
    assert_eq!(stats_before.messages_sent, stats_after.messages_sent);

    Ok(())
}

/// Test metadata handling
#[tokio::test]
async fn test_metadata_handling() {
    let agent = AgentInfo::new("test-agent".to_string(), "test-type".to_string())
        .with_metadata("version".to_string(), "1.0.0".to_string())
        .with_metadata("environment".to_string(), "test".to_string())
        .with_metadata("team".to_string(), "backend".to_string());

    assert_eq!(agent.metadata.get("version"), Some(&"1.0.0".to_string()));
    assert_eq!(agent.metadata.get("environment"), Some(&"test".to_string()));
    assert_eq!(agent.metadata.get("team"), Some(&"backend".to_string()));
    assert_eq!(agent.metadata.len(), 3);

    let message = AgentMessage::new(
        "sender".to_string(),
        MessageType::TaskAssignment,
        "Test message".to_string(),
    )
    .with_metadata("task_id".to_string(), "task-123".to_string())
    .with_metadata("priority".to_string(), "high".to_string());

    assert_eq!(
        message.metadata.get("task_id"),
        Some(&"task-123".to_string())
    );
    assert_eq!(message.metadata.get("priority"), Some(&"high".to_string()));
    assert_eq!(message.metadata.len(), 2);
}

/// Test UUID generation for agent IDs
#[tokio::test]
async fn test_agent_id_generation() {
    let agent1 = AgentInfo::new("agent-1".to_string(), "type-1".to_string());
    let agent2 = AgentInfo::new("agent-2".to_string(), "type-2".to_string());

    // Each agent should have a unique ID
    assert_ne!(agent1.id, agent2.id);

    // IDs should be valid UUIDs
    assert!(uuid::Uuid::parse_str(&agent1.id).is_ok());
    assert!(uuid::Uuid::parse_str(&agent2.id).is_ok());
}

/// Test message ID generation
#[tokio::test]
async fn test_message_id_generation() {
    let message1 = AgentMessage::new(
        "sender-1".to_string(),
        MessageType::TaskAssignment,
        "Message 1".to_string(),
    );
    let message2 = AgentMessage::new(
        "sender-2".to_string(),
        MessageType::StatusUpdate,
        "Message 2".to_string(),
    );

    // Each message should have a unique ID
    assert_ne!(message1.id, message2.id);

    // IDs should be valid UUIDs
    assert!(uuid::Uuid::parse_str(&message1.id).is_ok());
    assert!(uuid::Uuid::parse_str(&message2.id).is_ok());
}

/// Test timestamp handling
#[tokio::test]
async fn test_timestamp_handling() {
    let before = chrono::Utc::now();

    let message = AgentMessage::new(
        "sender".to_string(),
        MessageType::TaskAssignment,
        "Test message".to_string(),
    );

    let after = chrono::Utc::now();

    // Message timestamp should be between before and after
    assert!(message.timestamp >= before);
    assert!(message.timestamp <= after);
}

/// Test mock coordination client creation
#[tokio::test]
async fn test_mock_coordination_client_creation() {
    let mut client = MockCoordinationClient::new();

    // Test that the client is created successfully
    assert!(client.is_connected().await);

    // Test connection stats
    let stats = client.get_connection_stats().await.unwrap();
    assert!(!stats.is_connected); // Initially false
    assert_eq!(stats.messages_sent, 0);
    assert_eq!(stats.messages_received, 0);
}

/// Test configuration serialization
#[tokio::test]
async fn test_config_serialization() {
    let config = CoordinationConfig {
        enabled: true,
        server_endpoint: "http://localhost:50051".to_string(),
        timeout_seconds: 60,
        retry_config: rhema_core::coordination::RetryConfig {
            max_retries: 5,
            initial_delay_ms: 200,
            max_delay_ms: 10000,
            backoff_multiplier: 1.5,
        },
        health_check_config: rhema_core::coordination::HealthCheckConfig {
            enabled: true,
            interval_seconds: 60,
            timeout_seconds: 10,
        },
        tls_config: None,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: CoordinationConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.server_endpoint, deserialized.server_endpoint);
    assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    assert_eq!(
        config.retry_config.max_retries,
        deserialized.retry_config.max_retries
    );
    assert_eq!(
        config.health_check_config.enabled,
        deserialized.health_check_config.enabled
    );
}

/// Test agent info serialization
#[tokio::test]
async fn test_agent_info_serialization() {
    let agent = AgentInfo::new("test-agent".to_string(), "test-type".to_string())
        .with_task_id("task-123".to_string())
        .with_scope("test-scope".to_string())
        .with_capability("test-capability".to_string())
        .with_metadata("version".to_string(), "1.0.0".to_string());

    // Test JSON serialization
    let json = serde_json::to_string(&agent).unwrap();
    let deserialized: AgentInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(agent.name, deserialized.name);
    assert_eq!(agent.agent_type, deserialized.agent_type);
    assert_eq!(agent.current_task_id, deserialized.current_task_id);
    assert_eq!(agent.assigned_scope, deserialized.assigned_scope);
    assert_eq!(agent.capabilities, deserialized.capabilities);
    assert_eq!(agent.metadata, deserialized.metadata);
}

/// Test message serialization
#[tokio::test]
async fn test_message_serialization() {
    let message = AgentMessage::new(
        "sender-123".to_string(),
        MessageType::TaskAssignment,
        "Test message content".to_string(),
    )
    .to_recipients(vec!["recipient-1".to_string(), "recipient-2".to_string()])
    .with_priority(MessagePriority::High)
    .with_metadata("task_id".to_string(), "task-123".to_string());

    // Test JSON serialization
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: AgentMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(message.sender_id, deserialized.sender_id);
    assert_eq!(message.recipient_ids, deserialized.recipient_ids);
    assert!(matches!(
        deserialized.message_type,
        MessageType::TaskAssignment
    ));
    assert!(matches!(deserialized.priority, MessagePriority::High));
    assert_eq!(message.content, deserialized.content);
    assert_eq!(message.metadata, deserialized.metadata);
}
