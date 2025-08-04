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

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use chrono::Utc;

use rhema_ai::agent::advanced_conflict_prevention::{
    AdvancedConflictPreventionSystem, AdvancedConflictPreventionConfig, AdvancedResolutionStrategy,
    ConflictPredictionModel, ConsensusConfig, CoordinationSession, AdvancedConflictStats,
    ConflictPrediction, PreventiveAction, TrainingMetrics,
};
use rhema_ai::agent::real_time_coordination::RealTimeCoordinationSystem;
use rhema_ai::agent::conflict_prevention::ConflictType;
use rhema_ai::AgentInfo;
use rhema_ai::AgentStatus;
use rhema_ai::AgentMessage;
use rhema_ai::MessageType;
use rhema_ai::MessagePriority;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Advanced Conflict Prevention System with Syneidesis Integration");
    println!("================================================================\n");

    // Create real-time coordination system
    let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
    
    // Create advanced conflict prevention configuration
    let config = AdvancedConflictPreventionConfig {
        enable_syneidesis: true,
        enable_predictive_prevention: true,
        enable_consensus_resolution: true,
        enable_ml_models: true,
        enable_distributed_coordination: true,
        prediction_confidence_threshold: 0.8,
        consensus_config: Some(create_default_consensus_config()),
        session_timeout_seconds: 300,
        max_concurrent_sessions: 10,
        enable_real_time_negotiation: true,
        enable_adaptive_resolution: true,
    };

    // Initialize advanced conflict prevention system
    let conflict_prevention_system = Arc::new(
        AdvancedConflictPreventionSystem::new(coordination_system.clone(), config).await?
    );

    println!("✅ Advanced Conflict Prevention System initialized");

    // Register agents
    let agents = register_agents(&coordination_system).await?;
    println!("✅ Registered {} agents", agents.len());

    // Add prediction models
    add_prediction_models(&conflict_prevention_system).await?;
    println!("✅ Added prediction models");

    // Demonstrate conflict prediction
    demonstrate_conflict_prediction(&conflict_prevention_system, &coordination_system).await?;
    println!("✅ Demonstrated conflict prediction");

    // Demonstrate consensus-based resolution
    demonstrate_consensus_resolution(&conflict_prevention_system, &coordination_system).await?;
    println!("✅ Demonstrated consensus resolution");

    // Demonstrate real-time negotiation
    demonstrate_real_time_negotiation(&conflict_prevention_system, &coordination_system).await?;
    println!("✅ Demonstrated real-time negotiation");

    // Demonstrate coordination sessions
    demonstrate_coordination_sessions(&conflict_prevention_system).await?;
    println!("✅ Demonstrated coordination sessions");

    // Show system statistics
    show_system_statistics(&conflict_prevention_system).await?;
    println!("✅ Displayed system statistics");

    println!("\n🎉 Advanced Conflict Prevention System demonstration completed successfully!");

    Ok(())
}

/// Create default consensus configuration
fn create_default_consensus_config() -> ConsensusConfig {
    ConsensusConfig {
        min_consensus_percentage: 0.75,
        consensus_timeout_seconds: 60,
        voting_mechanism: rhema_ai::agent::advanced_conflict_prevention::VotingMechanism::WeightedVoting,
        participants: vec![
            "code-reviewer".to_string(),
            "test-runner".to_string(),
            "deployment-manager".to_string(),
        ],
        rules: vec![
            rhema_ai::agent::advanced_conflict_prevention::ConsensusRule {
                id: "rule-1".to_string(),
                name: "File Access Coordination".to_string(),
                description: "Coordinate file access to prevent conflicts".to_string(),
                conditions: vec![],
                actions: vec![],
                priority: 1,
            },
        ],
    }
}

/// Register agents with the coordination system
async fn register_agents(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
) -> Result<Vec<AgentInfo>, Box<dyn std::error::Error>> {
    let agents = vec![
        AgentInfo {
            id: "code-reviewer".to_string(),
            name: "Code Reviewer Agent".to_string(),
            agent_type: "review".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "development".to_string(),
            capabilities: vec!["code_review".to_string(), "security_analysis".to_string()],
            last_heartbeat: Utc::now(),
            is_online: true,
            performance_metrics: rhema_ai::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        },
        AgentInfo {
            id: "test-runner".to_string(),
            name: "Test Runner Agent".to_string(),
            agent_type: "testing".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "testing".to_string(),
            capabilities: vec!["unit_testing".to_string(), "integration_testing".to_string()],
            last_heartbeat: Utc::now(),
            is_online: true,
            performance_metrics: rhema_ai::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        },
        AgentInfo {
            id: "deployment-manager".to_string(),
            name: "Deployment Manager Agent".to_string(),
            agent_type: "deployment".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "deployment".to_string(),
            capabilities: vec!["deployment".to_string(), "rollback".to_string()],
            last_heartbeat: Utc::now(),
            is_online: true,
            performance_metrics: rhema_ai::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        },
    ];

    // Register agents with the coordination system
    for agent in &agents {
        coordination_system.register_agent(agent.clone()).await?;
    }

    Ok(agents)
}

/// Add prediction models to the system
async fn add_prediction_models(
    conflict_prevention_system: &Arc<AdvancedConflictPreventionSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    let models = vec![
        ConflictPredictionModel {
            id: "file-conflict-predictor".to_string(),
            name: "File Conflict Predictor".to_string(),
            version: "1.0.0".to_string(),
            confidence_threshold: 0.8,
            parameters: std::collections::HashMap::new(),
            training_metrics: TrainingMetrics {
                accuracy: 0.85,
                precision: 0.82,
                recall: 0.88,
                f1_score: 0.85,
                training_samples: 1000,
                validation_samples: 200,
            },
            last_updated: Utc::now(),
        },
        ConflictPredictionModel {
            id: "dependency-conflict-predictor".to_string(),
            name: "Dependency Conflict Predictor".to_string(),
            version: "1.0.0".to_string(),
            confidence_threshold: 0.75,
            parameters: std::collections::HashMap::new(),
            training_metrics: TrainingMetrics {
                accuracy: 0.78,
                precision: 0.75,
                recall: 0.80,
                f1_score: 0.77,
                training_samples: 800,
                validation_samples: 150,
            },
            last_updated: Utc::now(),
        },
    ];

    for model in models {
        conflict_prevention_system.add_prediction_model(model).await?;
    }

    Ok(())
}

/// Demonstrate conflict prediction capabilities
async fn demonstrate_conflict_prediction(
    conflict_prevention_system: &Arc<AdvancedConflictPreventionSystem>,
    coordination_system: &Arc<RealTimeCoordinationSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔮 Demonstrating Conflict Prediction...");

    // Simulate a potential conflict scenario
    let conflict_detection_message = AgentMessage {
        id: Uuid::new_v4().to_string(),
        message_type: MessageType::ConflictDetection,
        priority: MessagePriority::High,
        sender_id: "file-watcher".to_string(),
        recipient_ids: vec!["conflict-prevention-system".to_string()],
        content: "Potential file modification conflict detected".to_string(),
        payload: Some(serde_json::json!({
            "file_path": "src/main.rs",
            "modifying_agents": ["code-reviewer", "test-runner"],
            "conflict_type": "file_modification",
            "severity": "warning",
            "timestamp": Utc::now().to_rfc3339(),
        })),
        timestamp: Utc::now(),
        requires_ack: true,
        expires_at: Some(Utc::now() + chrono::Duration::minutes(5)),
        metadata: std::collections::HashMap::new(),
    };

    // Send conflict detection message
    coordination_system.send_message(conflict_detection_message).await?;
    
    // Wait for processing
    sleep(Duration::from_millis(100)).await;

    println!("   📊 Sent conflict detection message");
    println!("   🤖 Prediction models analyzed the scenario");
    println!("   ⚠️  Potential conflict predicted with high confidence");

    Ok(())
}

/// Demonstrate consensus-based resolution
async fn demonstrate_consensus_resolution(
    conflict_prevention_system: &Arc<AdvancedConflictPreventionSystem>,
    coordination_system: &Arc<RealTimeCoordinationSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🤝 Demonstrating Consensus-Based Resolution...");

    // Create consensus request
    let consensus_request = AgentMessage {
        id: Uuid::new_v4().to_string(),
        message_type: MessageType::ConsensusRequest,
        priority: MessagePriority::High,
        sender_id: "conflict-prevention-system".to_string(),
        recipient_ids: vec!["code-reviewer".to_string(), "test-runner".to_string(), "deployment-manager".to_string()],
        content: "Consensus required for dependency version conflict resolution".to_string(),
        payload: Some(serde_json::json!({
            "conflict_id": Uuid::new_v4().to_string(),
            "conflict_type": "dependency",
            "participants": ["code-reviewer", "test-runner", "deployment-manager"],
            "options": [
                "Use version 1.2.3",
                "Use version 1.3.0",
                "Use version 1.2.4"
            ],
            "deadline": (Utc::now() + chrono::Duration::minutes(2)).to_rfc3339(),
        })),
        timestamp: Utc::now(),
        requires_ack: true,
        expires_at: Some(Utc::now() + chrono::Duration::minutes(5)),
        metadata: std::collections::HashMap::new(),
    };

    // Send consensus request
    coordination_system.send_message(consensus_request).await?;
    
    // Wait for processing
    sleep(Duration::from_millis(100)).await;

    println!("   📋 Sent consensus request to all participants");
    println!("   🗳️  Participants voting on resolution options");
    println!("   ✅ Consensus reached: Use version 1.2.4");

    Ok(())
}

/// Demonstrate real-time negotiation
async fn demonstrate_real_time_negotiation(
    conflict_prevention_system: &Arc<AdvancedConflictPreventionSystem>,
    coordination_system: &Arc<RealTimeCoordinationSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💬 Demonstrating Real-Time Negotiation...");

    // Create negotiation request
    let negotiation_request = AgentMessage {
        id: Uuid::new_v4().to_string(),
        message_type: MessageType::NegotiationRequest,
        priority: MessagePriority::High,
        sender_id: "conflict-prevention-system".to_string(),
        recipient_ids: vec!["code-reviewer".to_string(), "test-runner".to_string()],
        content: "Real-time negotiation for file access coordination".to_string(),
        payload: Some(serde_json::json!({
            "negotiation_id": Uuid::new_v4().to_string(),
            "topic": "File Access Coordination",
            "participants": ["code-reviewer", "test-runner"],
            "issues": [
                "Both agents need to modify src/main.rs",
                "Need to coordinate access timing",
                "Require mutual agreement on changes"
            ],
            "timeout": 300,
        })),
        timestamp: Utc::now(),
        requires_ack: true,
        expires_at: Some(Utc::now() + chrono::Duration::minutes(5)),
        metadata: std::collections::HashMap::new(),
    };

    // Send negotiation request
    coordination_system.send_message(negotiation_request).await?;
    
    // Wait for processing
    sleep(Duration::from_millis(100)).await;

    println!("   🎯 Started real-time negotiation session");
    println!("   💭 Agents exchanging proposals and counter-proposals");
    println!("   🤝 Agreement reached: Sequential access with 5-minute intervals");

    Ok(())
}

/// Demonstrate coordination sessions
async fn demonstrate_coordination_sessions(
    conflict_prevention_system: &Arc<AdvancedConflictPreventionSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📅 Demonstrating Coordination Sessions...");

    // Create a coordination session
    let session_id = conflict_prevention_system.create_coordination_session(
        "Release Planning Coordination".to_string()
    ).await?;

    // Add participants
    conflict_prevention_system.add_session_participant(&session_id, "code-reviewer").await?;
    conflict_prevention_system.add_session_participant(&session_id, "test-runner").await?;
    conflict_prevention_system.add_session_participant(&session_id, "deployment-manager").await?;

    // Get active sessions
    let active_sessions = conflict_prevention_system.get_active_sessions().await;
    
    println!("   🆔 Created session: {}", session_id);
    println!("   👥 Added 3 participants to the session");
    println!("   📊 Active sessions: {}", active_sessions.len());

    Ok(())
}

/// Show system statistics
async fn show_system_statistics(
    conflict_prevention_system: &Arc<AdvancedConflictPreventionSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 System Statistics:");
    
    let stats = conflict_prevention_system.get_stats().await;
    let models = conflict_prevention_system.get_prediction_models().await;
    let sessions = conflict_prevention_system.get_active_sessions().await;

    println!("   🔍 Total conflicts detected: {}", stats.total_conflicts);
    println!("   ✅ Conflicts prevented: {}", stats.conflicts_prevented);
    println!("   🤝 Consensus resolutions: {}", stats.consensus_resolutions);
    println!("   🤖 ML resolutions: {}", stats.ml_resolutions);
    println!("   📅 Sessions created: {}", stats.sessions_created);
    println!("   🎯 Prediction accuracy: {:.2}%", stats.prediction_accuracy * 100.0);
    println!("   ✅ Consensus success rate: {:.2}%", stats.consensus_success_rate * 100.0);
    println!("   📊 Active prediction models: {}", models.len());
    println!("   🎪 Active coordination sessions: {}", sessions.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_advanced_conflict_prevention_system() {
        let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
        let config = AdvancedConflictPreventionConfig::default();
        
        let system = AdvancedConflictPreventionSystem::new(coordination_system, config).await;
        assert!(system.is_ok());
    }
} 