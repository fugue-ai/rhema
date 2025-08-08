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

//! Basic usage example for the Syneidesis coordination library

use std::time::Duration;
use syneidesis_coordination::{
    agent::{
        AgentCoordinator, AgentHealth, AgentMetrics, AgentState, AgentStatus, ConflictResolver,
        ConflictStrategy, Task, TaskPriority,
    },
    init,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the coordination library
    println!("🚀 Initializing Syneidesis coordination library...");
    init().await?;

    // Create coordinator with default configuration
    let mut coordinator = AgentCoordinator::new();

    // Start the coordinator
    println!("📡 Starting agent coordinator...");
    coordinator.start().await?;

    // Create and register agents
    println!("🤖 Registering agents...");

    // Create verification agent
    let mut verification_agent = AgentState::new(
        "verification-agent-1".to_string(),
        "Code Verification Agent".to_string(),
        "verification".to_string(),
    );

    // Add capabilities
    verification_agent.add_capability("code_verification".to_string());
    verification_agent.add_capability("security_scanning".to_string());
    verification_agent.add_capability("static_analysis".to_string());

    // Set priority
    verification_agent.priority = 200; // High priority

    // Register agent
    coordinator
        .register_agent(verification_agent.clone())
        .await?;

    // Create monitoring agent
    let mut monitoring_agent = AgentState::new(
        "monitoring-agent-1".to_string(),
        "System Monitoring Agent".to_string(),
        "monitoring".to_string(),
    );

    // Add capabilities
    monitoring_agent.add_capability("system_monitoring".to_string());
    monitoring_agent.add_capability("performance_tracking".to_string());
    monitoring_agent.add_capability("alert_generation".to_string());

    // Register agent
    coordinator.register_agent(monitoring_agent.clone()).await?;

    println!("✅ Agents registered successfully!");

    // Create and assign tasks
    println!("📋 Creating and assigning tasks...");

    // Create verification task
    let verification_task = Task::new(
        "verify_security_code".to_string(),
        "Verify security-critical code for vulnerabilities".to_string(),
        "verification".to_string(),
        serde_json::json!({
            "file_path": "/path/to/security/code",
            "security_level": "critical",
            "scan_type": "comprehensive"
        }),
    )
    .with_priority(TaskPriority::Critical)
    .with_capabilities(vec![
        "code_verification".to_string(),
        "security_scanning".to_string(),
    ]);

    // Assign task
    let task_id = coordinator.assign_task(verification_task).await?;
    println!("📝 Task assigned with ID: {task_id}");

    // Create monitoring task
    let monitoring_task = Task::new(
        "monitor_system_performance".to_string(),
        "Monitor system performance metrics".to_string(),
        "monitoring".to_string(),
        serde_json::json!({
            "metrics": ["cpu", "memory", "disk", "network"],
            "interval": 60,
            "alert_threshold": 80.0
        }),
    )
    .with_priority(TaskPriority::Normal)
    .with_capabilities(vec!["system_monitoring".to_string()]);

    // Assign task
    let monitoring_task_id = coordinator.assign_task(monitoring_task).await?;
    println!("📝 Monitoring task assigned with ID: {monitoring_task_id}");

    // Simulate task completion
    println!("⏳ Simulating task completion...");
    sleep(Duration::from_secs(2)).await;

    // Complete verification task
    let verification_result = serde_json::json!({
        "status": "completed",
        "vulnerabilities_found": 2,
        "security_score": 85,
        "recommendations": [
            "Update dependency X to version Y",
            "Add input validation for user data"
        ]
    });

    coordinator
        .complete_task(&task_id, verification_result)
        .await?;
    println!("✅ Verification task completed!");

    // Complete monitoring task
    let monitoring_result = serde_json::json!({
        "status": "completed",
        "cpu_usage": 45.2,
        "memory_usage": 67.8,
        "disk_usage": 23.1,
        "network_usage": 12.5,
        "alerts_generated": 0
    });

    coordinator
        .complete_task(&monitoring_task_id, monitoring_result)
        .await?;
    println!("✅ Monitoring task completed!");

    // Get and display statistics
    println!("📊 Getting coordinator statistics...");
    let stats = coordinator.get_statistics().await?;

    println!("\n📈 Coordinator Statistics:");
    println!("  Total agents: {}", stats.total_agents);
    println!("  Healthy agents: {}", stats.healthy_agents);
    println!("  Available agents: {}", stats.available_agents);
    println!("  Total tasks: {}", stats.total_tasks);
    println!("  Completed tasks: {}", stats.completed_tasks);
    println!("  Success rate: {:.2}%", stats.success_rate);
    println!("  Uptime: {} seconds", stats.uptime);

    // Demonstrate conflict resolution
    println!("\n🔄 Demonstrating conflict resolution...");

    let resolver = ConflictResolver::with_strategy(ConflictStrategy::AutoMerge);

    // Simulate a conflict
    let local_state = serde_json::json!({
        "agent_id": "verification-agent-1",
        "status": "busy",
        "current_task": "verify_security_code",
        "metrics": {
            "cpu_usage": 45.2,
            "memory_usage": 67.8
        }
    });

    let remote_state = serde_json::json!({
        "agent_id": "verification-agent-1",
        "status": "idle",
        "current_task": null,
        "metrics": {
            "cpu_usage": 25.1,
            "memory_usage": 45.3,
            "disk_usage": 12.5
        }
    });

    // Detect conflict
    let conflict = resolver
        .detect_conflict(
            syneidesis_coordination::agent::conflict::ConflictType::AgentState,
            local_state,
            remote_state,
        )
        .await?;

    if let Some(conflict) = conflict {
        println!("  Conflict detected: {}", conflict.id);

        // Resolve conflict
        let resolution = resolver.attempt_resolution(&conflict.id).await?;
        println!("  Conflict resolved: {}", resolution.message);

        // Get conflict statistics
        let conflict_stats = resolver.get_statistics().await;
        println!("  Total conflicts: {}", conflict_stats.total_conflicts);
        println!(
            "  Resolution success rate: {:.2}%",
            conflict_stats.success_rate
        );
    }

    // Update agent health and status
    println!("\n🏥 Updating agent health...");

    // Update verification agent health
    let mut updated_verification_agent = verification_agent.clone();
    updated_verification_agent.update_health(AgentHealth::Healthy);
    updated_verification_agent.update_status(AgentStatus::Idle);

    // Update metrics
    let mut metrics = AgentMetrics::default();
    metrics.update_cpu_usage(35.5);
    metrics.update_memory_usage(1024 * 1024 * 512, 1024 * 1024 * 1024); // 512MB used, 1GB total
    metrics.record_task_completion(2500); // 2.5 seconds
    updated_verification_agent.update_metrics(metrics);

    coordinator
        .update_agent_state("verification-agent-1", updated_verification_agent)
        .await?;
    println!("  Verification agent health updated");

    // Get available agents
    println!("\n🔍 Getting available agents...");
    let available_agents = coordinator.get_available_agents().await?;

    println!("  Available agents:");
    for agent in available_agents {
        println!(
            "    - {} ({}) - Health: {:?}, Status: {:?}",
            agent.name, agent.id, agent.health, agent.status
        );
    }

    // Demonstrate error handling
    println!("\n⚠️  Demonstrating error handling...");

    // Try to register the same agent again
    match coordinator.register_agent(verification_agent.clone()).await {
        Ok(_) => println!("  Unexpected: Agent registered successfully"),
        Err(syneidesis_coordination::AgentError::AlreadyExists { agent_id }) => {
            println!("  Expected error: Agent {agent_id} already exists");
        }
        Err(e) => println!("  Unexpected error: {e}"),
    }

    // Try to get non-existent agent
    match coordinator.get_agent("non-existent-agent").await {
        Ok(_) => println!("  Unexpected: Agent found"),
        Err(syneidesis_coordination::AgentError::NotFound { agent_id }) => {
            println!("  Expected error: Agent {agent_id} not found");
        }
        Err(e) => println!("  Unexpected error: {e}"),
    }

    // Stop the coordinator
    println!("\n🛑 Stopping coordinator...");
    coordinator.stop().await?;
    println!("✅ Coordinator stopped successfully");

    println!("\n🎉 Basic usage example completed successfully!");
    println!("The Syneidesis coordination library is ready for integration!");

    Ok(())
}
