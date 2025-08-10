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
use rhema_coordination::agent::real_time_coordination::{
    AgentInfo, AgentStatus, AgentMessage, MessageType, MessagePriority,
    CoordinationConfig, AdvancedCoordinationConfig, LoadBalancingStrategy,
    FaultToleranceConfig, EncryptionConfig, PerformanceMonitoringConfig,
    ConsensusConfig, ConsensusAlgorithm, AgentPerformanceMetrics, PerformanceThresholds,
    EncryptionAlgorithm
};
use rhema_coordination::coordination_integration::{CoordinationIntegration, CoordinationConfig as IntegrationConfig};
use std::collections::HashMap;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Rhema Coordination Production Integration Example");
    println!("===================================================");

    // Example 1: Basic Coordination Integration
    basic_coordination_integration().await?;

    // Example 2: Advanced Coordination with Production Features
    advanced_coordination_integration().await?;

    // Example 3: Multi-Agent Coordination Workflow
    multi_agent_coordination_workflow().await?;

    // Example 4: Coordination with External Integration
    coordination_with_external_integration().await?;

    println!("\n✅ All coordination integration examples completed successfully!");
    Ok(())
}

/// Example 1: Basic Coordination Integration
async fn basic_coordination_integration() -> rhema_core::RhemaResult<()> {
    println!("\n📋 Example 1: Basic Coordination Integration");
    println!("--------------------------------------------");

    // Create Rhema instance
    let mut rhema = Rhema::new()?;
    println!("✅ Rhema instance created");

    // Initialize basic coordination system
    let coordination_config = CoordinationConfig {
        max_message_history: 1000,
        message_timeout_seconds: 30,
        heartbeat_interval_seconds: 10,
        agent_timeout_seconds: 60,
        max_session_participants: 10,
        enable_encryption: false,
        enable_compression: true,
    };

    rhema.init_coordination(Some(coordination_config)).await?;
    println!("✅ Basic coordination system initialized");

    // Create test agents
    let agent1 = AgentInfo {
        id: "agent-1".to_string(),
        name: "Development Agent".to_string(),
        agent_type: "developer".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "main".to_string(),
        capabilities: vec!["coding".to_string(), "testing".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: AgentPerformanceMetrics::default(),
    };

    let agent2 = AgentInfo {
        id: "agent-2".to_string(),
        name: "Testing Agent".to_string(),
        agent_type: "tester".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "main".to_string(),
        capabilities: vec!["testing".to_string(), "validation".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: AgentPerformanceMetrics::default(),
    };

    // Register agents
    rhema.register_agent(agent1).await?;
    rhema.register_agent(agent2).await?;
    println!("✅ Agents registered");

    // Create a coordination session
    let session_id = rhema.create_coordination_session(
        "Development Workflow".to_string(),
        vec!["agent-1".to_string(), "agent-2".to_string()]
    ).await?;
    println!("✅ Coordination session created: {}", session_id);

    // Join session
    rhema.join_coordination_session(&session_id, "agent-1").await?;
    rhema.join_coordination_session(&session_id, "agent-2").await?;
    println!("✅ Agents joined session");

    // Send messages
    let message1 = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::High,
        sender_id: "agent-1".to_string(),
        recipient_ids: vec!["agent-2".to_string()],
        content: "Please review the new feature implementation".to_string(),
        payload: Some(serde_json::json!({
            "task_id": "task-123",
            "feature": "user-authentication",
            "priority": "high"
        })),
        timestamp: chrono::Utc::now(),
        requires_ack: true,
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        metadata: HashMap::new(),
    };

    rhema.send_session_message(&session_id, message1).await?;
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

    // Start health monitoring
    rhema.start_coordination_health_monitoring().await?;
    println!("✅ Health monitoring started");

    // Shutdown coordination
    rhema.shutdown_coordination().await?;
    println!("✅ Coordination system shutdown");

    Ok(())
}

/// Example 2: Advanced Coordination with Production Features
async fn advanced_coordination_integration() -> rhema_core::RhemaResult<()> {
    println!("\n📋 Example 2: Advanced Coordination with Production Features");
    println!("------------------------------------------------------------");

    // Create Rhema instance
    let mut rhema = Rhema::new()?;

    // Configure advanced coordination
    let coordination_config = CoordinationConfig {
        max_message_history: 5000,
        message_timeout_seconds: 60,
        heartbeat_interval_seconds: 5,
        agent_timeout_seconds: 30,
        max_session_participants: 50,
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
        load_balancing_strategy: LoadBalancingStrategy::LeastResponseTime,
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
            thresholds: PerformanceThresholds {
                max_message_latency_ms: 1000,
                max_agent_response_time_ms: 5000,
                max_session_creation_time_ms: 2000,
                max_memory_usage_percent: 80.0,
                max_cpu_usage_percent: 80.0,
            },
        },
    };

    rhema.init_advanced_coordination(coordination_config, advanced_config).await?;
    println!("✅ Advanced coordination system initialized");

    // Create multiple agents with different capabilities
    let agents = vec![
        ("agent-dev-1", "Senior Developer", "developer", vec!["coding", "architecture", "review"]),
        ("agent-dev-2", "Junior Developer", "developer", vec!["coding", "testing"]),
        ("agent-qa-1", "QA Engineer", "tester", vec!["testing", "automation", "validation"]),
        ("agent-ops-1", "DevOps Engineer", "operations", vec!["deployment", "monitoring", "infrastructure"]),
        ("agent-arch-1", "Solution Architect", "architect", vec!["architecture", "design", "planning"]),
    ];

    for (id, name, agent_type, capabilities) in agents {
        let agent = AgentInfo {
            id: id.to_string(),
            name: name.to_string(),
            agent_type: agent_type.to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "main".to_string(),
            capabilities: capabilities.into_iter().map(|s| s.to_string()).collect(),
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: AgentPerformanceMetrics::default(),
        };
        rhema.register_agent(agent).await?;
    }
    println!("✅ Multiple agents registered with different capabilities");

    // Create advanced session with consensus
    let consensus_config = ConsensusConfig {
        algorithm: ConsensusAlgorithm::Raft,
        min_participants: 3,
        timeout_seconds: 30,
        enable_leader_election: true,
        leader_election_timeout_seconds: 60,
    };

    let session_id = rhema.get_coordination_system()
        .unwrap()
        .create_advanced_session(
            "Production Deployment Planning".to_string(),
            vec!["agent-dev-1".to_string(), "agent-qa-1".to_string(), "agent-ops-1".to_string(), "agent-arch-1".to_string()],
            Some(consensus_config)
        ).await?;
    println!("✅ Advanced session created with consensus: {}", session_id);

    // Simulate a deployment planning workflow
    let workflow_messages = vec![
        ("agent-arch-1", MessageType::TaskAssignment, "Design deployment architecture", MessagePriority::Critical),
        ("agent-dev-1", MessageType::TaskCompletion, "Code review completed", MessagePriority::High),
        ("agent-qa-1", MessageType::TaskCompletion, "Testing completed successfully", MessagePriority::High),
        ("agent-ops-1", MessageType::ResourceRequest, "Request deployment resources", MessagePriority::Critical),
    ];

    for (agent_id, msg_type, content, priority) in workflow_messages {
        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: msg_type,
            priority,
            sender_id: agent_id.to_string(),
            recipient_ids: vec![],
            content: content.to_string(),
            payload: Some(serde_json::json!({
                "workflow_step": "deployment_planning",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "agent_id": agent_id
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(2)),
            metadata: HashMap::new(),
        };

        rhema.send_session_message(&session_id, message).await?;
        println!("✅ Workflow message sent from {}", agent_id);
    }

    // Get performance metrics
    if let Some(performance_metrics) = rhema.get_coordination_system()
        .unwrap()
        .get_performance_metrics().await {
        println!("📊 Performance Metrics:");
        println!("  Total Messages Processed: {}", performance_metrics.total_messages_processed);
        println!("  Average Message Latency: {:.2}ms", performance_metrics.average_message_latency_ms);
        println!("  Average Agent Response Time: {:.2}ms", performance_metrics.average_agent_response_time_ms);
        println!("  Memory Usage: {:.2}%", performance_metrics.memory_usage_percent);
        println!("  CPU Usage: {:.2}%", performance_metrics.cpu_usage_percent);
    }

    // Get performance alerts
    let alerts = rhema.get_coordination_system()
        .unwrap()
        .get_performance_alerts();
    if !alerts.is_empty() {
        println!("⚠️  Performance Alerts:");
        for alert in alerts {
            println!("  [{}] {}: {}", alert.severity, alert.alert_type, alert.message);
        }
    } else {
        println!("✅ No performance alerts");
    }

    // Shutdown
    rhema.shutdown_coordination().await?;
    println!("✅ Advanced coordination system shutdown");

    Ok(())
}

/// Example 3: Multi-Agent Coordination Workflow
async fn multi_agent_coordination_workflow() -> rhema_core::RhemaResult<()> {
    println!("\n📋 Example 3: Multi-Agent Coordination Workflow");
    println!("------------------------------------------------");

    // Create Rhema instance with coordination
    let mut rhema = Rhema::new()?;
    rhema.init_coordination(None).await?;

    // Create a development team workflow
    let team_agents = vec![
        ("product-manager", "Product Manager", vec!["planning", "prioritization"]),
        ("tech-lead", "Technical Lead", vec!["architecture", "code-review", "planning"]),
        ("frontend-dev", "Frontend Developer", vec!["react", "typescript", "ui"]),
        ("backend-dev", "Backend Developer", vec!["rust", "api", "database"]),
        ("qa-engineer", "QA Engineer", vec!["testing", "automation", "validation"]),
        ("devops-engineer", "DevOps Engineer", vec!["deployment", "monitoring", "ci-cd"]),
    ];

    // Register all team agents
    for (id, name, capabilities) in team_agents {
        let agent = AgentInfo {
            id: id.to_string(),
            name: name.to_string(),
            agent_type: "team-member".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "feature-development".to_string(),
            capabilities: capabilities.into_iter().map(|s| s.to_string()).collect(),
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: AgentPerformanceMetrics::default(),
        };
        rhema.register_agent(agent).await?;
    }
    println!("✅ Development team agents registered");

    // Create workflow sessions
    let planning_session = rhema.create_coordination_session(
        "Feature Planning".to_string(),
        vec!["product-manager".to_string(), "tech-lead".to_string()]
    ).await?;

    let development_session = rhema.create_coordination_session(
        "Feature Development".to_string(),
        vec!["tech-lead".to_string(), "frontend-dev".to_string(), "backend-dev".to_string()]
    ).await?;

    let testing_session = rhema.create_coordination_session(
        "Feature Testing".to_string(),
        vec!["qa-engineer".to_string(), "frontend-dev".to_string(), "backend-dev".to_string()]
    ).await?;

    let deployment_session = rhema.create_coordination_session(
        "Feature Deployment".to_string(),
        vec!["devops-engineer".to_string(), "tech-lead".to_string(), "qa-engineer".to_string()]
    ).await?;

    println!("✅ Workflow sessions created");

    // Simulate feature development workflow
    let workflow_steps = vec![
        // Planning Phase
        ("product-manager", &planning_session, MessageType::TaskAssignment, "Define user authentication feature requirements", MessagePriority::Critical),
        ("tech-lead", &planning_session, MessageType::TaskCompletion, "Architecture design completed", MessagePriority::High),
        
        // Development Phase
        ("tech-lead", &development_session, MessageType::TaskAssignment, "Implement authentication API endpoints", MessagePriority::High),
        ("backend-dev", &development_session, MessageType::TaskCompletion, "Backend authentication API completed", MessagePriority::High),
        ("frontend-dev", &development_session, MessageType::TaskCompletion, "Frontend authentication UI completed", MessagePriority::High),
        
        // Testing Phase
        ("qa-engineer", &testing_session, MessageType::TaskAssignment, "Perform comprehensive testing", MessagePriority::High),
        ("qa-engineer", &testing_session, MessageType::TaskCompletion, "All tests passed", MessagePriority::High),
        
        // Deployment Phase
        ("devops-engineer", &deployment_session, MessageType::TaskAssignment, "Deploy to production", MessagePriority::Critical),
        ("devops-engineer", &deployment_session, MessageType::TaskCompletion, "Deployment successful", MessagePriority::Critical),
    ];

    for (agent_id, session_id, msg_type, content, priority) in workflow_steps {
        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: msg_type,
            priority,
            sender_id: agent_id.to_string(),
            recipient_ids: vec![],
            content: content.to_string(),
            payload: Some(serde_json::json!({
                "workflow_phase": match *session_id {
                    ref s if s == &planning_session => "planning",
                    ref s if s == &development_session => "development",
                    ref s if s == &testing_session => "testing",
                    ref s if s == &deployment_session => "deployment",
                    _ => "unknown"
                },
                "feature": "user-authentication",
                "timestamp": chrono::Utc::now().to_rfc3339()
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(4)),
            metadata: HashMap::new(),
        };

        rhema.send_session_message(session_id, message).await?;
        println!("✅ Workflow step completed: {} - {}", agent_id, content);
        
        // Simulate some processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Get final statistics
    let stats = rhema.get_coordination_stats().await?;
    println!("📊 Final Workflow Statistics:");
    println!("  Total Messages: {}", stats.total_messages);
    println!("  Messages Delivered: {}", stats.messages_delivered);
    println!("  Messages Failed: {}", stats.messages_failed);
    println!("  Active Agents: {}", stats.active_agents);
    println!("  Active Sessions: {}", stats.active_sessions);
    println!("  Coordination Efficiency: {:.2}%", stats.coordination_efficiency * 100.0);

    // Shutdown
    rhema.shutdown_coordination().await?;
    println!("✅ Multi-agent workflow completed");

    Ok(())
}

/// Example 4: Coordination with External Integration
async fn coordination_with_external_integration() -> rhema_core::RhemaResult<()> {
    println!("\n📋 Example 4: Coordination with External Integration");
    println!("----------------------------------------------------");

    // Create Rhema instance
    let mut rhema = Rhema::new()?;

    // Initialize coordination system
    rhema.init_coordination(None).await?;

    // Initialize coordination integration
    let integration_config = IntegrationConfig {
        run_local_server: true,
        server_address: None,
        auto_register_agents: true,
        sync_messages: true,
        sync_tasks: true,
        enable_health_monitoring: true,
        syneidesis: None, // No external Syneidesis for this example
    };

    rhema.init_coordination_integration(Some(integration_config)).await?;
    println!("✅ Coordination integration initialized");

    // Create agents
    let external_agent = AgentInfo {
        id: "external-agent-1".to_string(),
        name: "External System Agent".to_string(),
        agent_type: "external".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "external-integration".to_string(),
        capabilities: vec!["api-integration".to_string(), "data-sync".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: AgentPerformanceMetrics::default(),
    };

    let internal_agent = AgentInfo {
        id: "internal-agent-1".to_string(),
        name: "Internal System Agent".to_string(),
        agent_type: "internal".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "internal-system".to_string(),
        capabilities: vec!["data-processing".to_string(), "validation".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: AgentPerformanceMetrics::default(),
    };

    // Register agents
    rhema.register_agent(external_agent).await?;
    rhema.register_agent(internal_agent).await?;
    println!("✅ Internal and external agents registered");

    // Create integration session
    let session_id = rhema.create_coordination_session(
        "System Integration".to_string(),
        vec!["external-agent-1".to_string(), "internal-agent-1".to_string()]
    ).await?;
    println!("✅ Integration session created: {}", session_id);

    // Simulate data synchronization workflow
    let sync_messages = vec![
        ("external-agent-1", MessageType::TaskAssignment, "Sync user data from external system", MessagePriority::High),
        ("external-agent-1", MessageType::TaskCompletion, "User data sync completed", MessagePriority::High),
        ("internal-agent-1", MessageType::TaskAssignment, "Validate and process synced data", MessagePriority::High),
        ("internal-agent-1", MessageType::TaskCompletion, "Data validation and processing completed", MessagePriority::High),
        ("external-agent-1", MessageType::TaskAssignment, "Update external system with processed results", MessagePriority::High),
        ("external-agent-1", MessageType::TaskCompletion, "External system updated successfully", MessagePriority::High),
    ];

    for (agent_id, msg_type, content, priority) in sync_messages {
        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: msg_type,
            priority,
            sender_id: agent_id.to_string(),
            recipient_ids: vec![],
            content: content.to_string(),
            payload: Some(serde_json::json!({
                "integration_workflow": "data_synchronization",
                "step": match msg_type {
                    MessageType::TaskAssignment => "start",
                    MessageType::TaskCompletion => "complete",
                    _ => "unknown"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            metadata: HashMap::new(),
        };

        // Send through coordination system
        rhema.send_session_message(&session_id, message.clone()).await?;
        
        // Bridge through integration
        rhema.bridge_coordination_message(&message).await?;
        
        println!("✅ Integration message sent and bridged: {} - {}", agent_id, content);
        
        // Simulate processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    // Start health monitoring
    rhema.start_integration_health_monitoring().await?;
    println!("✅ Integration health monitoring started");

    // Get integration statistics
    let integration_stats = rhema.get_integration_stats().await?;
    println!("📊 Integration Statistics:");
    println!("  Rhema Agents: {}", integration_stats.rhema_agents);
    println!("  Rhema Messages: {}", integration_stats.rhema_messages);
    println!("  Syneidesis Agents: {}", integration_stats.syneidesis_agents);
    println!("  Bridge Messages Sent: {}", integration_stats.bridge_messages_sent);
    println!("  Bridge Messages Received: {}", integration_stats.bridge_messages_received);

    // Get coordination statistics
    let coordination_stats = rhema.get_coordination_stats().await?;
    println!("📊 Coordination Statistics:");
    println!("  Total Messages: {}", coordination_stats.total_messages);
    println!("  Active Agents: {}", coordination_stats.active_agents);
    println!("  Active Sessions: {}", coordination_stats.active_sessions);
    println!("  Average Response Time: {:.2}ms", coordination_stats.avg_response_time_ms);

    // Shutdown
    rhema.shutdown_coordination().await?;
    println!("✅ External integration completed");

    Ok(())
} 