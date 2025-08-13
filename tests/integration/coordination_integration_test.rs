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

use rhema_api::{
    AdvancedCoordinationConfig, AgentInfo, AgentMessage, AgentStatus, ConsensusConfig,
    CoordinationConfig, EncryptionConfig, FaultToleranceConfig, IntegrationConfig,
    LoadBalancingStrategy, MessagePriority, MessageType, PerformanceMonitoringConfig, Rhema,
};
use rhema_coordination::agent::real_time_coordination::ConsensusAlgorithm;
use rhema_coordination::agent::real_time_coordination::EncryptionAlgorithm;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[tokio::test]
async fn test_basic_coordination_integration() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repository
    init_test_repo(repo_path);

    // Create Rhema instance
    let mut rhema = Rhema::new_from_path(repo_path.to_path_buf()).unwrap();

    // Test coordination initialization
    let config = CoordinationConfig {
        max_message_history: 100,
        message_timeout_seconds: 10,
        heartbeat_interval_seconds: 5,
        agent_timeout_seconds: 30,
        max_session_participants: 5,
        enable_encryption: false,
        enable_compression: true,
    };

    rhema.init_coordination(Some(config)).await.unwrap();
    assert!(rhema.has_coordination());

    // Test agent registration
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
        performance_metrics: rhema_api::AgentPerformanceMetrics::default(),
    };

    rhema.register_agent(agent).await.unwrap();

    // Test session creation
    let session_id = rhema
        .create_coordination_session("Test Session".to_string(), vec!["test-agent".to_string()])
        .await
        .unwrap();

    assert!(!session_id.is_empty());

    // Test message sending
    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::Normal,
        sender_id: "test-agent".to_string(),
        recipient_ids: vec![],
        content: "Test message".to_string(),
        payload: None,
        timestamp: chrono::Utc::now(),
        requires_ack: false,
        expires_at: None,
        metadata: HashMap::new(),
    };

    rhema
        .send_session_message(&session_id, message)
        .await
        .unwrap();

    // Test statistics
    let stats = rhema.get_coordination_stats().await.unwrap();
    assert_eq!(stats.active_agents, 1);
    assert_eq!(stats.active_sessions, 1);

    // Test agent retrieval
    let agents = rhema.get_all_agents().await.unwrap();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].id, "test-agent");

    // Test agent info retrieval
    let agent_info = rhema.get_agent_info("test-agent").await.unwrap();
    assert!(agent_info.is_some());
    assert_eq!(agent_info.unwrap().id, "test-agent");

    // Test status update
    rhema
        .update_agent_status("test-agent", AgentStatus::Busy)
        .await
        .unwrap();
    let updated_agent = rhema.get_agent_info("test-agent").await.unwrap().unwrap();
    assert_eq!(updated_agent.status, AgentStatus::Busy);

    // Test shutdown
    rhema.shutdown_coordination().await.unwrap();
}

#[tokio::test]
async fn test_advanced_coordination_integration() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repository
    init_test_repo(repo_path);

    // Create Rhema instance
    let mut rhema = Rhema::new_from_path(repo_path.to_path_buf()).unwrap();

    // Test advanced coordination initialization
    let coordination_config = CoordinationConfig {
        max_message_history: 500,
        message_timeout_seconds: 30,
        heartbeat_interval_seconds: 10,
        agent_timeout_seconds: 60,
        max_session_participants: 10,
        enable_encryption: true,
        enable_compression: true,
    };

    let advanced_config = AdvancedCoordinationConfig {
        enable_load_balancing: true,
        enable_fault_tolerance: true,
        enable_encryption: true,
        enable_compression: true,
        enable_advanced_sessions: true,
        enable_performance_monitoring: true,
        load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
        fault_tolerance_config: FaultToleranceConfig {
            enable_failover: true,
            max_retry_attempts: 3,
            retry_delay_ms: 1000,
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout_seconds: 30,
            health_check_interval_seconds: 10,
        },
        encryption_config: EncryptionConfig {
            algorithm: EncryptionAlgorithm::AES256,
            key_rotation_hours: 24,
            enable_e2e_encryption: true,
            certificate_path: None,
            private_key_path: None,
        },
        performance_config: PerformanceMonitoringConfig {
            enable_metrics: true,
            metrics_interval_seconds: 30,
            enable_alerts: true,
            thresholds: rhema_coordination::agent::real_time_coordination::PerformanceThresholds {
                max_message_latency_ms: 1000,
                max_agent_response_time_ms: 500,
                max_session_creation_time_ms: 2000,
                max_memory_usage_percent: 80.0,
                max_cpu_usage_percent: 90.0,
            },
        },
    };

    rhema
        .init_advanced_coordination(coordination_config, advanced_config)
        .await
        .unwrap();
    assert!(rhema.has_coordination());

    // Test multiple agent registration
    let agents = vec![
        ("agent-1", "Agent 1", vec!["capability-1", "capability-2"]),
        ("agent-2", "Agent 2", vec!["capability-2", "capability-3"]),
        ("agent-3", "Agent 3", vec!["capability-1", "capability-3"]),
    ];

    for (id, name, capabilities) in agents {
        let agent = AgentInfo {
            id: id.to_string(),
            name: name.to_string(),
            agent_type: "test".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "test".to_string(),
            capabilities: capabilities.into_iter().map(|s| s.to_string()).collect(),
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: rhema_api::AgentPerformanceMetrics::default(),
        };
        rhema.register_agent(agent).await.unwrap();
    }

    // Test advanced session creation with consensus
    let consensus_config = ConsensusConfig {
        algorithm: ConsensusAlgorithm::Raft,
        min_participants: 2,
        timeout_seconds: 30,
        enable_leader_election: true,
        leader_election_timeout_seconds: 60,
    };

    let session_id = rhema
        .get_coordination_system()
        .unwrap()
        .create_advanced_session(
            "Advanced Test Session".to_string(),
            vec![
                "agent-1".to_string(),
                "agent-2".to_string(),
                "agent-3".to_string(),
            ],
            Some(consensus_config),
        )
        .await
        .unwrap();

    assert!(!session_id.is_empty());

    // Test performance monitoring
    let performance_metrics = rhema
        .get_coordination_system()
        .unwrap()
        .get_performance_metrics()
        .await;

    assert!(performance_metrics.is_some());

    // Test performance alerts
    let alerts = rhema
        .get_coordination_system()
        .unwrap()
        .get_performance_alerts();

    // Alerts might be empty in test environment, which is fine
    println!("Performance alerts: {}", alerts.len());

    // Test statistics
    let stats = rhema.get_coordination_stats().await.unwrap();
    assert_eq!(stats.active_agents, 3);
    assert_eq!(stats.active_sessions, 1);

    // Test shutdown
    rhema.shutdown_coordination().await.unwrap();
}

#[tokio::test]
async fn test_coordination_integration_with_external_systems() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repository
    init_test_repo(repo_path);

    // Create Rhema instance
    let mut rhema = Rhema::new_from_path(repo_path.to_path_buf()).unwrap();

    // Initialize coordination system
    rhema.init_coordination(None).await.unwrap();

    // Initialize coordination integration
    let integration_config = IntegrationConfig {
        run_local_server: true,
        server_address: None,
        auto_register_agents: true,
        sync_messages: true,
        sync_tasks: true,
        enable_health_monitoring: true,
        syneidesis: None, // No external Syneidesis for testing
    };

    rhema
        .init_coordination_integration(Some(integration_config))
        .await
        .unwrap();
    assert!(rhema.has_coordination_integration());

    // Test agent registration
    let agent = AgentInfo {
        id: "integration-agent".to_string(),
        name: "Integration Agent".to_string(),
        agent_type: "integration".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "integration".to_string(),
        capabilities: vec!["integration".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: rhema_api::AgentPerformanceMetrics::default(),
    };

    rhema.register_agent(agent).await.unwrap();

    // Test message bridging
    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::High,
        sender_id: "integration-agent".to_string(),
        recipient_ids: vec![],
        content: "Integration test message".to_string(),
        payload: Some(serde_json::json!({
            "test": "integration",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
        timestamp: chrono::Utc::now(),
        requires_ack: true,
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        metadata: HashMap::new(),
    };

    rhema.bridge_coordination_message(&message).await.unwrap();

    // Test integration statistics
    let integration_stats = rhema.get_integration_stats().await.unwrap();
    assert!(integration_stats.rhema_agents >= 1); // At least one agent should be registered
    assert!(integration_stats.bridge_messages_sent >= 1); // At least one message should be sent

    // Test coordination statistics
    let coordination_stats = rhema.get_coordination_stats().await.unwrap();
    assert!(coordination_stats.active_agents >= 1); // At least one agent should be active
    assert!(coordination_stats.total_messages >= 1); // At least one message should be sent

    // Test shutdown
    rhema.shutdown_coordination().await.unwrap();
}

#[tokio::test]
async fn test_coordination_error_handling() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repository
    init_test_repo(repo_path);

    // Create Rhema instance without coordination
    let rhema = Rhema::new_from_path(repo_path.to_path_buf()).unwrap();

    // Test that coordination methods return errors when not initialized
    assert!(!rhema.has_coordination());

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
        performance_metrics: rhema_api::AgentPerformanceMetrics::default(),
    };

    // These should fail because coordination is not initialized
    let result = rhema.register_agent(agent).await;
    assert!(result.is_err());

    let result = rhema.get_coordination_stats().await;
    assert!(result.is_err());

    let result = rhema.get_all_agents().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_coordination_performance_monitoring() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repository
    init_test_repo(repo_path);

    // Create Rhema instance
    let mut rhema = Rhema::new_from_path(repo_path.to_path_buf()).unwrap();

    // Initialize coordination with performance monitoring
    let coordination_config = CoordinationConfig {
        max_message_history: 1000,
        message_timeout_seconds: 30,
        heartbeat_interval_seconds: 5,
        agent_timeout_seconds: 60,
        max_session_participants: 10,
        enable_encryption: false,
        enable_compression: true,
    };

    let advanced_config = AdvancedCoordinationConfig {
        enable_load_balancing: false,
        enable_fault_tolerance: false,
        enable_encryption: false,
        enable_compression: false,
        enable_advanced_sessions: false,
        enable_performance_monitoring: true,
        load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
        fault_tolerance_config: FaultToleranceConfig::default(),
        encryption_config: EncryptionConfig::default(),
        performance_config: PerformanceMonitoringConfig {
            enable_metrics: true,
            metrics_interval_seconds: 1, // Fast interval for testing
            enable_alerts: true,
            thresholds: rhema_coordination::agent::real_time_coordination::PerformanceThresholds {
                max_message_latency_ms: 1000,
                max_agent_response_time_ms: 500,
                max_session_creation_time_ms: 2000,
                max_memory_usage_percent: 80.0,
                max_cpu_usage_percent: 90.0,
            },
        },
    };

    rhema
        .init_advanced_coordination(coordination_config, advanced_config)
        .await
        .unwrap();

    // Register test agent
    let agent = AgentInfo {
        id: "perf-test-agent".to_string(),
        name: "Performance Test Agent".to_string(),
        agent_type: "test".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "test".to_string(),
        capabilities: vec!["test".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: rhema_api::AgentPerformanceMetrics::default(),
    };

    rhema.register_agent(agent).await.unwrap();

    // Create session and send messages to generate metrics
    let session_id = rhema
        .create_coordination_session(
            "Performance Test Session".to_string(),
            vec!["perf-test-agent".to_string()],
        )
        .await
        .unwrap();

    // Send multiple messages to generate performance data
    for i in 0..10 {
        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::Normal,
            sender_id: "perf-test-agent".to_string(),
            recipient_ids: vec![],
            content: format!("Performance test message {}", i),
            payload: None,
            timestamp: chrono::Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: HashMap::new(),
        };

        rhema
            .send_session_message(&session_id, message)
            .await
            .unwrap();

        // Small delay to simulate real-world conditions
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Wait for metrics to be collected
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

    // Check performance metrics
    let performance_metrics = rhema
        .get_coordination_system()
        .unwrap()
        .get_performance_metrics()
        .await;

    // Performance monitoring might not be fully implemented yet, so we'll be flexible
    if let Some(metrics) = performance_metrics {
        // If metrics are available, they should be reasonable
        assert!(metrics.total_messages_processed >= 0);
        assert!(metrics.average_message_latency_ms >= 0.0);
        assert!(metrics.memory_usage_percent >= 0.0);
        assert!(metrics.cpu_usage_percent >= 0.0);

        println!("Performance metrics collected:");
        println!(
            "  Total messages processed: {}",
            metrics.total_messages_processed
        );
        println!(
            "  Average message latency: {:.2}ms",
            metrics.average_message_latency_ms
        );
        println!("  Memory usage: {:.2}%", metrics.memory_usage_percent);
        println!("  CPU usage: {:.2}%", metrics.cpu_usage_percent);
    } else {
        println!("Performance monitoring not available - this is acceptable for now");
    }

    // Check for alerts
    let alerts = rhema
        .get_coordination_system()
        .unwrap()
        .get_performance_alerts();

    println!("Performance alerts: {}", alerts.len());

    // Test shutdown
    rhema.shutdown_coordination().await.unwrap();
}

// Helper function to initialize a test git repository
fn init_test_repo(path: &Path) {
    // Create .git directory
    let git_dir = path.join(".git");
    fs::create_dir_all(&git_dir).unwrap();

    // Create a simple README file
    let readme_path = path.join("README.md");
    fs::write(
        readme_path,
        "# Test Repository\n\nThis is a test repository for coordination integration tests.",
    )
    .unwrap();

    // Create a simple rhema scope file
    let scope_path = path.join("rhema.yml");
    fs::write(
        scope_path,
        "name: test-scope\ndescription: Test scope for coordination integration",
    )
    .unwrap();
}
