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

use rhema_coordination::{
    AgenticDevelopmentService, CoordinationConfig, AgentInfo, AgentStatus, AgentMessage,
    MessageType, MessagePriority
};
use rhema_core::RhemaResult;
use std::path::PathBuf;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Rhema with Coordination Integration Example");
    
    // Example 1: Basic Coordination Integration
    basic_integration_example().await?;
    
    // Example 2: Advanced Configuration
    advanced_configuration_example().await?;
    
    // Example 3: Multi-Agent Coordination
    multi_agent_coordination_example().await?;
    
    info!("✅ All examples completed successfully!");
    Ok(())
}

/// Example 1: Basic Coordination Integration
async fn basic_integration_example() -> RhemaResult<()> {
    info!("📋 Example 1: Basic Coordination Integration");
    
    // Create a lock file path for the example
    let lock_file_path = PathBuf::from("example.lock");
    
    // Create service with default Coordination configuration
    let mut service = AgenticDevelopmentService::new_with_coordination(
        lock_file_path,
        Some(CoordinationConfig::default()),
    ).await?;
    
    // Initialize the service
    service.initialize().await?;
    
    // Create a test agent
    let agent_info = AgentInfo {
        id: "test-agent-1".to_string(),
        name: "Test Agent 1".to_string(),
        agent_type: "development".to_string(),
        status: AgentStatus::Idle,
        current_task_id: None,
        assigned_scope: "default".to_string(),
        capabilities: vec!["code_review".to_string(), "testing".to_string()],
        last_heartbeat: chrono::Utc::now(),
        is_online: true,
        performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
    };
    
    // Register agent with both Rhema and Coordination
    service.register_agent_with_coordination(agent_info).await?;
    
    // Send a test message
    let message = AgentMessage {
        id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::High,
        sender_id: "system".to_string(),
        recipient_ids: vec!["test-agent-1".to_string()],
        content: "Review pull request #123".to_string(),
        payload: Some(serde_json::json!({
            "description": "Please review the changes in PR #123",
            "repository": "rhema-ai/rhema",
            "branch": "feature/coordination-integration"
        })),
        timestamp: chrono::Utc::now(),
        requires_ack: true,
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
        metadata: std::collections::HashMap::new(),
    };
    
    // Send message with Coordination bridging
    service.send_message_with_coordination(message).await?;
    
    // Check if Coordination integration is enabled
    if service.has_coordination_integration() {
        info!("✅ Coordination integration is active");
        
        // Start health monitoring
        service.start_coordination_health_monitoring().await?;
        
        // Get system statistics
        let stats = service.get_system_statistics();
        info!("📊 System Statistics: {}", stats);
    }
    
    // Cleanup
    service.shutdown_coordination().await?;
    
    info!("✅ Basic integration example completed");
    Ok(())
}

/// Example 2: Advanced Configuration
async fn advanced_configuration_example() -> RhemaResult<()> {
    info!("📋 Example 2: Advanced Coordination Configuration");
    
    // Create advanced configuration
    let config = CoordinationConfig {
        run_local_server: true,
        server_address: Some("127.0.0.1:50051".to_string()),
        auto_register_agents: true,
        sync_messages: true,
        sync_tasks: true,
        enable_health_monitoring: true,
    };
    
    let lock_file_path = PathBuf::from("advanced_example.lock");
    let mut service = AgenticDevelopmentService::new_with_coordination(
        lock_file_path,
        Some(config),
    ).await?;
    
    service.initialize().await?;
    
    // Register multiple agents with different capabilities
    let agents = vec![
        AgentInfo {
            id: "code-reviewer".to_string(),
            name: "Code Reviewer Agent".to_string(),
            agent_type: "review".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "code-review".to_string(),
            capabilities: vec!["code_review".to_string(), "security_analysis".to_string()],
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        },
        AgentInfo {
            id: "test-runner".to_string(),
            name: "Test Runner Agent".to_string(),
            agent_type: "testing".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "testing".to_string(),
            capabilities: vec!["unit_testing".to_string(), "integration_testing".to_string()],
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        },
        AgentInfo {
            id: "deployment-manager".to_string(),
            name: "Deployment Manager Agent".to_string(),
            agent_type: "deployment".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "deployment".to_string(),
            capabilities: vec!["deployment".to_string(), "rollback".to_string()],
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        },
    ];
    
    // Register all agents
    for agent in agents {
        service.register_agent_with_coordination(agent).await?;
    }
    
    // Create a coordination session
    let session_id = service.create_session(
        "Release Planning".to_string(),
        vec!["code-reviewer".to_string(), "test-runner".to_string(), "deployment-manager".to_string()],
    ).await?;
    
    info!("📋 Created coordination session: {}", session_id);
    
    // Send coordinated messages
    let messages = vec![
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::CoordinationRequest,
            priority: MessagePriority::High,
            sender_id: "system".to_string(),
            recipient_ids: vec!["code-reviewer".to_string()],
            content: "Please review the release candidate".to_string(),
            payload: Some(serde_json::json!({
                "session_id": session_id,
                "action": "review_release_candidate"
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(2)),
            metadata: std::collections::HashMap::new(),
        },
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::Normal,
            sender_id: "system".to_string(),
            recipient_ids: vec!["test-runner".to_string()],
            content: "Run full test suite".to_string(),
            payload: Some(serde_json::json!({
                "session_id": session_id,
                "test_suite": "full",
                "timeout": 3600
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            metadata: std::collections::HashMap::new(),
        },
    ];
    
    // Send all messages with Coordination bridging
    for message in messages {
        service.send_message_with_coordination(message).await?;
    }
    
    // Wait a bit for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Get final statistics
    let stats = service.get_system_statistics();
    info!("📊 Final System Statistics: {}", stats);
    
    // Cleanup
    service.shutdown_coordination().await?;
    
    info!("✅ Advanced configuration example completed");
    Ok(())
}

/// Example 3: Multi-Agent Coordination
async fn multi_agent_coordination_example() -> RhemaResult<()> {
    info!("📋 Example 3: Multi-Agent Coordination with Coordination");
    
    let lock_file_path = PathBuf::from("multi_agent_example.lock");
    let mut service = AgenticDevelopmentService::new_with_coordination(
        lock_file_path,
        Some(CoordinationConfig::default()),
    ).await?;
    
    service.initialize().await?;
    
    // Simulate a complex development workflow
    let workflow_agents = vec![
        ("feature-developer", "Feature Developer", vec!["feature_development".to_string()]),
        ("code-reviewer", "Code Reviewer", vec!["code_review".to_string(), "security_analysis".to_string()]),
        ("test-automation", "Test Automation", vec!["automated_testing".to_string(), "performance_testing".to_string()]),
        ("deployment-engineer", "Deployment Engineer", vec!["deployment".to_string(), "infrastructure".to_string()]),
        ("monitoring-agent", "Monitoring Agent", vec!["monitoring".to_string(), "alerting".to_string()]),
    ];
    
    // Register all workflow agents
    for (id, name, capabilities) in workflow_agents {
        let agent_info = AgentInfo {
            id: id.to_string(),
            name: name.to_string(),
            agent_type: "workflow".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "development_workflow".to_string(),
            capabilities,
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        };
        
        service.register_agent_with_coordination(agent_info).await?;
    }
    
    // Simulate a feature development workflow
    let workflow_messages = vec![
        // 1. Feature development starts
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::High,
            sender_id: "workflow-manager".to_string(),
            recipient_ids: vec!["feature-developer".to_string()],
            content: "Implement new authentication feature".to_string(),
            payload: Some(serde_json::json!({
                "feature": "oauth2_integration",
                "requirements": ["OAuth2 support", "JWT tokens", "Role-based access"],
                "deadline": "2024-01-15T23:59:59Z"
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::days(7)),
            metadata: std::collections::HashMap::new(),
        },
        // 2. Code review request
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::CoordinationRequest,
            priority: MessagePriority::Normal,
            sender_id: "feature-developer".to_string(),
            recipient_ids: vec!["code-reviewer".to_string()],
            content: "Ready for code review".to_string(),
            payload: Some(serde_json::json!({
                "pull_request": "#456",
                "changes": "OAuth2 integration implementation",
                "files_changed": 15
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
            metadata: std::collections::HashMap::new(),
        },
        // 3. Testing request
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::Normal,
            sender_id: "workflow-manager".to_string(),
            recipient_ids: vec!["test-automation".to_string()],
            content: "Run comprehensive test suite".to_string(),
            payload: Some(serde_json::json!({
                "test_suite": "comprehensive",
                "include_performance": true,
                "include_security": true
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(4)),
            metadata: std::collections::HashMap::new(),
        },
        // 4. Deployment preparation
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::CoordinationRequest,
            priority: MessagePriority::High,
            sender_id: "workflow-manager".to_string(),
            recipient_ids: vec!["deployment-engineer".to_string()],
            content: "Prepare deployment for OAuth2 feature".to_string(),
            payload: Some(serde_json::json!({
                "environment": "staging",
                "feature_flags": ["oauth2_enabled"],
                "rollback_plan": "available"
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(2)),
            metadata: std::collections::HashMap::new(),
        },
        // 5. Monitoring setup
        AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::Normal,
            sender_id: "workflow-manager".to_string(),
            recipient_ids: vec!["monitoring-agent".to_string()],
            content: "Set up monitoring for OAuth2 feature".to_string(),
            payload: Some(serde_json::json!({
                "metrics": ["auth_success_rate", "auth_response_time", "error_rate"],
                "alerts": ["high_error_rate", "slow_response_time"]
            })),
            timestamp: chrono::Utc::now(),
            requires_ack: true,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            metadata: std::collections::HashMap::new(),
        },
    ];
    
    // Send all workflow messages with Coordination bridging
    for (i, message) in workflow_messages.into_iter().enumerate() {
        info!("📤 Sending workflow message {}: {}", i + 1, message.content);
        service.send_message_with_coordination(message).await?;
        
        // Small delay between messages
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Wait for processing
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // Get final workflow statistics
    let stats = service.get_system_statistics();
    info!("📊 Workflow Statistics: {}", stats);
    
    // Cleanup
    service.shutdown_coordination().await?;
    
    info!("✅ Multi-agent coordination example completed");
    Ok(())
} 