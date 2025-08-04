use rhema::Rhema;
use rhema_ai::agent::real_time_coordination::{CoordinationConfig, AgentInfo, AgentStatus, AgentMessage, MessageType, MessagePriority, AgentPerformanceMetrics};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> rhema_core::RhemaResult<()> {
    println!("Testing coordination methods...");
    
    // Create Rhema instance
    let mut rhema = Rhema::new()?;
    println!("✅ Rhema instance created");

    // Test if init_coordination method exists
    let coordination_config = CoordinationConfig {
        max_message_history: 100,
        message_timeout_seconds: 30,
        heartbeat_interval_seconds: 10,
        agent_timeout_seconds: 60,
        max_session_participants: 5,
        enable_encryption: false,
        enable_compression: true,
    };

    // This should work if the method is properly exposed
    rhema.init_coordination(Some(coordination_config)).await?;
    println!("✅ init_coordination method works");

    // Test if register_agent method exists
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

    rhema.register_agent(agent).await?;
    println!("✅ register_agent method works");

    println!("All coordination methods are accessible!");
    Ok(())
} 