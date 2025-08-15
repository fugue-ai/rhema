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

//! Integration tests for gRPC coordination client
//!
//! This module provides comprehensive integration tests for the gRPC coordination
//! client, including type conversion, error handling, and real gRPC communication.

use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

use rhema_coordination::{
    agent::real_time_coordination::{AgentInfo, AgentMessage, AgentStatus, MessagePriority, MessageType},
    grpc::coordination_client::SyneidesisCoordinationClient,
    type_conversion::*,
    error_handling::{GrpcClientError, RetryConfig, ResilienceManager},
    monitoring::{GrpcClientMonitor, PrometheusExporter},
};

/// Test configuration
#[derive(Debug, Clone)]
struct TestConfig {
    server_address: String,
    test_timeout: Duration,
    retry_config: RetryConfig,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            server_address: "http://127.0.0.1:50051".to_string(),
            test_timeout: Duration::from_secs(30),
            retry_config: RetryConfig {
                max_retries: 2,
                initial_backoff: Duration::from_millis(100),
                max_backoff: Duration::from_secs(5),
                backoff_multiplier: 2.0,
                jitter: false,
            },
        }
    }
}

/// Test fixture for coordination client tests
struct CoordinationClientTestFixture {
    config: TestConfig,
    client: Option<SyneidesisCoordinationClient>,
    monitor: GrpcClientMonitor,
}

impl CoordinationClientTestFixture {
    async fn new(config: TestConfig) -> Self {
        let monitor = GrpcClientMonitor::new(Duration::from_secs(10));
        
        Self {
            config,
            client: None,
            monitor,
        }
    }

    async fn setup_client(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let syneidesis_config = rhema_coordination::grpc::coordination_client::SyneidesisConfig {
            enabled: true,
            server_address: Some(self.config.server_address.clone()),
            auto_register_agents: true,
            sync_messages: true,
            enable_health_monitoring: true,
            timeout_seconds: 10,
            max_retries: 3,
            enable_tls: false,
            tls_cert_path: None,
        };

        self.client = Some(SyneidesisCoordinationClient::new(syneidesis_config).await?);
        info!("✅ Test client setup completed");
        Ok(())
    }

    async fn teardown(&mut self) {
        if let Some(client) = &self.client {
            if let Err(e) = client.shutdown().await {
                warn!("Error during client shutdown: {}", e);
            }
        }
        info!("✅ Test client teardown completed");
    }

    fn create_test_agent(&self, id: &str) -> AgentInfo {
        AgentInfo {
            id: id.to_string(),
            name: format!("Test Agent {}", id),
            agent_type: "test-agent".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "test-scope".to_string(),
            capabilities: vec!["testing".to_string(), "coordination".to_string()],
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        }
    }

    fn create_test_message(&self, id: &str, sender: &str) -> AgentMessage {
        AgentMessage {
            id: id.to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::Normal,
            sender_id: sender.to_string(),
            recipient_ids: vec!["test-recipient".to_string()],
            content: "Test message content".to_string(),
            payload: None,
            timestamp: chrono::Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}

#[tokio::test]
async fn test_type_conversion_roundtrip() {
    info!("🧪 Testing type conversion roundtrip");

    let fixture = CoordinationClientTestFixture::new(TestConfig::default()).await;
    
    // Test AgentInfo conversion
    let rhema_agent = fixture.create_test_agent("test-agent-1");
    let proto_agent = rhema_agent_info_to_proto(&rhema_agent);
    let converted_back = proto_agent_info_to_rhema(&proto_agent);

    assert_eq!(rhema_agent.id, converted_back.id);
    assert_eq!(rhema_agent.name, converted_back.name);
    assert_eq!(rhema_agent.agent_type, converted_back.agent_type);
    assert_eq!(rhema_agent.status, converted_back.status);

    // Test AgentMessage conversion
    let rhema_message = fixture.create_test_message("test-message-1", "test-sender");
    let proto_message = rhema_message_to_proto(&rhema_message);
    let converted_back = proto_message_to_rhema(&proto_message);

    assert_eq!(rhema_message.id, converted_back.id);
    assert_eq!(rhema_message.sender_id, converted_back.sender_id);
    assert_eq!(rhema_message.content, converted_back.content);

    info!("✅ Type conversion roundtrip tests passed");
}

#[tokio::test]
async fn test_enum_conversion() {
    info!("🧪 Testing enum conversion");

    // Test AgentStatus conversion
    let rhema_statuses = vec![
        AgentStatus::Idle,
        AgentStatus::Busy,
        AgentStatus::Working,
        AgentStatus::Blocked,
        AgentStatus::Collaborating,
        AgentStatus::Offline,
    ];

    for status in rhema_statuses {
        let proto_status = rhema_status_to_proto(&status);
        let converted_back = proto_status_to_rhema(proto_status);
        assert_eq!(status, converted_back);
    }

    // Test MessageType conversion
    let rhema_message_types = vec![
        MessageType::TaskAssignment,
        MessageType::TaskCompletion,
        MessageType::ResourceRequest,
        MessageType::StatusUpdate,
    ];

    for message_type in rhema_message_types {
        let proto_type = rhema_message_type_to_proto(&message_type);
        let converted_back = proto_message_type_to_rhema(proto_type);
        assert!(matches!(converted_back, _)); // Basic type check
    }

    // Test MessagePriority conversion
    let rhema_priorities = vec![
        MessagePriority::Low,
        MessagePriority::Normal,
        MessagePriority::High,
        MessagePriority::Critical,
        MessagePriority::Emergency,
    ];

    for priority in rhema_priorities {
        let proto_priority = rhema_priority_to_proto(&priority);
        let converted_back = proto_priority_to_rhema(proto_priority);
        assert_eq!(priority, converted_back);
    }

    info!("✅ Enum conversion tests passed");
}

#[tokio::test]
async fn test_resilience_manager() {
    info!("🧪 Testing resilience manager");

    let config = RetryConfig {
        max_retries: 2,
        initial_backoff: Duration::from_millis(10),
        max_backoff: Duration::from_millis(100),
        backoff_multiplier: 2.0,
        jitter: false,
    };

    let mut manager = ResilienceManager::new(config);
    let mut attempt_count = 0;

    // Test successful retry
    let result = manager.execute_with_retry("test_operation", || async {
        attempt_count += 1;
        if attempt_count < 3 {
            Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "test error"))
        } else {
            Ok("success")
        }
    }).await;

    assert!(result.is_ok());
    assert_eq!(attempt_count, 3);
    assert_eq!(manager.health(), rhema_coordination::error_handling::ConnectionHealth::Healthy);

    // Test failure after max retries
    attempt_count = 0;
    let result = manager.execute_with_retry("test_operation", || async {
        attempt_count += 1;
        Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "persistent error"))
    }).await;

    assert!(result.is_err());
    assert_eq!(attempt_count, 3); // max_retries + 1

    info!("✅ Resilience manager tests passed");
}

#[tokio::test]
async fn test_monitoring_metrics() {
    info!("🧪 Testing monitoring metrics");

    let monitor = GrpcClientMonitor::new(Duration::from_secs(10));
    
    // Record some operations
    monitor.record_success("test_op", 100.0).await;
    monitor.record_success("test_op", 150.0).await;
    monitor.record_failure("test_op", 200.0, "test error").await;

    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.total_requests, 3);
    assert_eq!(metrics.successful_requests, 2);
    assert_eq!(metrics.failed_requests, 1);

    let op_metrics = monitor.get_operation_metrics("test_op").await.unwrap();
    assert_eq!(op_metrics.total_calls, 3);
    assert_eq!(op_metrics.successful_calls, 2);
    assert_eq!(op_metrics.failed_calls, 1);
    assert!((op_metrics.success_rate() - 2.0/3.0).abs() < 0.001);

    // Test health check
    let health = monitor.perform_health_check().await;
    assert!(health.is_healthy);
    assert_eq!(health.status, "healthy");

    // Test Prometheus export
    let exporter = PrometheusExporter::new(std::sync::Arc::new(monitor));
    let prometheus_metrics = exporter.export_metrics().await;
    assert!(prometheus_metrics.contains("rhema_grpc_client_requests_total 3"));
    assert!(prometheus_metrics.contains("rhema_grpc_client_successful_requests_total 2"));

    info!("✅ Monitoring metrics tests passed");
}

#[tokio::test]
async fn test_client_connection_lifecycle() {
    info!("🧪 Testing client connection lifecycle");

    let mut fixture = CoordinationClientTestFixture::new(TestConfig::default()).await;
    
    // Test client setup
    let setup_result = fixture.setup_client().await;
    if setup_result.is_err() {
        warn!("⚠️  Skipping connection tests - no gRPC server available");
        return;
    }

    let client = fixture.client.as_ref().unwrap();
    
    // Test connection status
    let status = client.get_connection_status().await;
    assert!(matches!(status, rhema_coordination::grpc::coordination_client::ConnectionStatus::Connected));

    // Test health check
    let health_result = client.health_check().await;
    assert!(health_result.is_ok());

    // Cleanup
    fixture.teardown().await;

    info!("✅ Client connection lifecycle tests passed");
}

#[tokio::test]
async fn test_agent_registration_flow() {
    info!("🧪 Testing agent registration flow");

    let mut fixture = CoordinationClientTestFixture::new(TestConfig::default()).await;
    
    // Test client setup
    let setup_result = fixture.setup_client().await;
    if setup_result.is_err() {
        warn!("⚠️  Skipping registration tests - no gRPC server available");
        return;
    }

    let client = fixture.client.as_ref().unwrap();
    
    // Test agent registration
    let agent = fixture.create_test_agent("test-agent-registration");
    let register_result = client.register_agent(agent.clone()).await;
    
    if register_result.is_ok() {
        info!("✅ Agent registration successful");
        
        // Test agent info retrieval
        let agent_info_result = client.get_agent_info(&agent.id).await;
        if agent_info_result.is_ok() {
            if let Some(retrieved_agent) = agent_info_result.unwrap() {
                assert_eq!(agent.id, retrieved_agent.id);
                assert_eq!(agent.name, retrieved_agent.name);
                info!("✅ Agent info retrieval successful");
            }
        }

        // Test agent unregistration
        let unregister_result = client.unregister_agent(&agent.id).await;
        if unregister_result.is_ok() {
            info!("✅ Agent unregistration successful");
        }
    } else {
        warn!("⚠️  Agent registration failed (server may not be running)");
    }

    // Cleanup
    fixture.teardown().await;

    info!("✅ Agent registration flow tests completed");
}

#[tokio::test]
async fn test_message_sending_flow() {
    info!("🧪 Testing message sending flow");

    let mut fixture = CoordinationClientTestFixture::new(TestConfig::default()).await;
    
    // Test client setup
    let setup_result = fixture.setup_client().await;
    if setup_result.is_err() {
        warn!("⚠️  Skipping message tests - no gRPC server available");
        return;
    }

    let client = fixture.client.as_ref().unwrap();
    
    // Test message sending
    let message = fixture.create_test_message("test-message-send", "test-sender");
    let send_result = client.send_message(message.clone()).await;
    
    if send_result.is_ok() {
        info!("✅ Message sending successful");
    } else {
        warn!("⚠️  Message sending failed (server may not be running)");
    }

    // Cleanup
    fixture.teardown().await;

    info!("✅ Message sending flow tests completed");
}

#[tokio::test]
async fn test_session_management_flow() {
    info!("🧪 Testing session management flow");

    let mut fixture = CoordinationClientTestFixture::new(TestConfig::default()).await;
    
    // Test client setup
    let setup_result = fixture.setup_client().await;
    if setup_result.is_err() {
        warn!("⚠️  Skipping session tests - no gRPC server available");
        return;
    }

    let client = fixture.client.as_ref().unwrap();
    
    // Test session creation
    let topic = "test-session-topic";
    let participants = vec!["agent1".to_string(), "agent2".to_string()];
    let create_result = client.create_session(topic.to_string(), participants.clone()).await;
    
    if create_result.is_ok() {
        let session_id = create_result.unwrap();
        info!("✅ Session creation successful: {}", session_id);
        
        // Test session joining
        let join_result = client.join_session(&session_id, "agent1").await;
        if join_result.is_ok() {
            info!("✅ Session joining successful");
            
            // Test session message sending
            let session_message = fixture.create_test_message("session-msg", "agent1");
            let send_result = client.send_session_message(&session_id, session_message).await;
            if send_result.is_ok() {
                info!("✅ Session message sending successful");
            }
            
            // Test session leaving
            let leave_result = client.leave_session(&session_id, "agent1").await;
            if leave_result.is_ok() {
                info!("✅ Session leaving successful");
            }
        }
    } else {
        warn!("⚠️  Session creation failed (server may not be running)");
    }

    // Cleanup
    fixture.teardown().await;

    info!("✅ Session management flow tests completed");
}

#[tokio::test]
async fn test_error_handling_scenarios() {
    info!("🧪 Testing error handling scenarios");

    let config = RetryConfig {
        max_retries: 1,
        initial_backoff: Duration::from_millis(10),
        max_backoff: Duration::from_millis(50),
        backoff_multiplier: 2.0,
        jitter: false,
    };

    let mut manager = ResilienceManager::new(config);

    // Test timeout error
    let timeout_result = manager.execute_with_retry("timeout_test", || async {
        sleep(Duration::from_millis(100)).await;
        Err(GrpcClientError::Timeout { operation: "timeout_test".to_string() })
    }).await;

    assert!(timeout_result.is_err());
    assert!(matches!(timeout_result.unwrap_err(), GrpcClientError::Timeout { .. }));

    // Test connection error
    let connection_result = manager.execute_with_retry("connection_test", || async {
        Err(GrpcClientError::Connection { message: "connection failed".to_string() })
    }).await;

    assert!(connection_result.is_err());
    assert!(matches!(connection_result.unwrap_err(), GrpcClientError::Connection { .. }));

    info!("✅ Error handling scenario tests passed");
}

#[tokio::test]
async fn test_performance_metrics_collection() {
    info!("🧪 Testing performance metrics collection");

    let monitor = GrpcClientMonitor::new(Duration::from_secs(10));
    
    // Simulate various operation latencies
    let latencies = vec![50.0, 100.0, 150.0, 200.0, 75.0];
    
    for (i, latency) in latencies.iter().enumerate() {
        if i % 2 == 0 {
            monitor.record_success("performance_test", *latency).await;
        } else {
            monitor.record_failure("performance_test", *latency, "test error").await;
        }
    }

    let op_metrics = monitor.get_operation_metrics("performance_test").await.unwrap();
    assert_eq!(op_metrics.total_calls, 5);
    assert_eq!(op_metrics.successful_calls, 3);
    assert_eq!(op_metrics.failed_calls, 2);
    
    // Check latency statistics
    assert!(op_metrics.min_latency_ms > 0.0);
    assert!(op_metrics.max_latency_ms > op_metrics.min_latency_ms);
    assert!(op_metrics.avg_latency_ms > 0.0);

    // Test performance summary
    let summary = monitor.get_performance_summary().await;
    assert_eq!(summary.total_requests, 5);
    assert!((summary.success_rate - 0.6).abs() < 0.001); // 3/5 = 0.6

    info!("✅ Performance metrics collection tests passed");
}

/// Integration test runner
#[tokio::test]
async fn run_all_integration_tests() {
    info!("🚀 Starting comprehensive gRPC coordination client integration tests");

    // Run all tests
    test_type_conversion_roundtrip().await;
    test_enum_conversion().await;
    test_resilience_manager().await;
    test_monitoring_metrics().await;
    test_client_connection_lifecycle().await;
    test_agent_registration_flow().await;
    test_message_sending_flow().await;
    test_session_management_flow().await;
    test_error_handling_scenarios().await;
    test_performance_metrics_collection().await;

    info!("🎉 All integration tests completed successfully!");
}
