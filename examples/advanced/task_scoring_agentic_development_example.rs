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
    AgenticDevelopmentService, Task, TaskPriority, TaskStatus, TaskType, TaskScoringFactors,
    TaskComplexity, PrioritizationStrategy, ConstraintContext, AgentInfo, AgentStatus,
    AgentMessage, MessageType, MessagePriority, ConflictType, ResolutionStrategy,
    Constraint, ConstraintType, ConstraintSeverity, EnforcementMode,
};
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Task Scoring for Agentic Development System Demo");
    println!("==================================================");

    // Initialize the agentic development service
    let lock_file_path = PathBuf::from("examples/test.lock");
    let mut service = AgenticDevelopmentService::new(lock_file_path);
    
    // Initialize the service
    service.initialize().await?;
    println!("✅ Service initialized successfully");

    // Demo 1: Constraint Definition and Enforcement System
    demo_constraint_system(&mut service).await?;

    // Demo 2: Task Scoring and Prioritization Algorithms
    demo_task_scoring(&mut service).await?;

    // Demo 3: Real-time Agent Coordination Mechanisms
    demo_real_time_coordination(&mut service).await?;

    // Demo 4: Conflict Prevention and Resolution Systems
    demo_conflict_prevention(&mut service).await?;

    // Demo 5: Integrated Workflow
    demo_integrated_workflow(&mut service).await?;

    // Display final system statistics
    let stats = service.get_system_statistics();
    println!("\n📊 Final System Statistics:");
    println!("{}", stats);

    println!("\n🎉 Task Scoring for Agentic Development System Demo Completed!");
    Ok(())
}

/// Demo 1: Constraint Definition and Enforcement System
async fn demo_constraint_system(service: &mut AgenticDevelopmentService) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔒 Demo 1: Constraint Definition and Enforcement System");
    println!("-----------------------------------------------------");

    // Create resource usage constraint
    let resource_constraint = Constraint {
        id: "resource-constraint-1".to_string(),
        name: "Resource Usage Limit".to_string(),
        description: "Limit CPU and memory usage for development tasks".to_string(),
        constraint_type: ConstraintType::ResourceUsage,
        severity: ConstraintSeverity::Warning,
        enforcement_mode: EnforcementMode::Soft,
        scope: "development".to_string(),
        active: true,
        created_at: Utc::now(),
        modified_at: Utc::now(),
        parameters: rhema_coordination::agent::constraint_system::ConstraintParameters {
            resource: Some(rhema_coordination::agent::constraint_system::ResourceConstraint {
                max_cpu_percent: Some(80.0),
                max_memory_mb: Some(2048),
                max_disk_mb: Some(10240),
                max_network_mbps: Some(100.0),
                max_concurrent_ops: Some(5),
            }),
            file_access: None,
            time: None,
            quality: None,
            security: None,
            performance: None,
            collaboration: None,
            custom: HashMap::new(),
        },
        metadata: HashMap::new(),
    };

    service.constraint_system.add_constraint(resource_constraint)?;
    println!("✅ Added resource usage constraint");

    // Create file access constraint
    let file_constraint = Constraint {
        id: "file-constraint-1".to_string(),
        name: "File Access Control".to_string(),
        description: "Control access to critical files".to_string(),
        constraint_type: ConstraintType::FileAccess,
        severity: ConstraintSeverity::Error,
        enforcement_mode: EnforcementMode::Hard,
        scope: "security".to_string(),
        active: true,
        created_at: Utc::now(),
        modified_at: Utc::now(),
        parameters: rhema_coordination::agent::constraint_system::ConstraintParameters {
            resource: None,
            file_access: Some(rhema_coordination::agent::constraint_system::FileAccessConstraint {
                allowed_patterns: vec!["src/**/*.rs".to_string(), "tests/**/*.rs".to_string()],
                denied_patterns: vec!["**/secrets/**".to_string(), "**/config/prod/**".to_string()],
                read_only_files: vec!["README.md".to_string()],
                required_files: vec!["Cargo.toml".to_string()],
                max_file_size_bytes: Some(1024 * 1024), // 1MB
            }),
            time: None,
            quality: None,
            security: None,
            performance: None,
            collaboration: None,
            custom: HashMap::new(),
        },
        metadata: HashMap::new(),
    };

    service.constraint_system.add_constraint(file_constraint)?;
    println!("✅ Added file access constraint");

    // Test constraint enforcement
    let context = ConstraintContext {
        current_cpu_percent: 75.0,
        current_memory_mb: 1500,
        current_disk_mb: 5000,
        accessed_files: vec!["src/main.rs".to_string(), "tests/test.rs".to_string()],
        network_endpoints: vec!["api.github.com".to_string()],
        response_time_ms: 150,
        custom_data: HashMap::new(),
    };

    let enforcement_result = service.enforce_constraints("development", &context).await?;
    println!("✅ Constraint enforcement result: {}", enforcement_result.satisfied);
    
    if !enforcement_result.violations.is_empty() {
        println!("⚠️  Violations detected:");
        for violation in &enforcement_result.violations {
            println!("   - {}: {}", violation.constraint_id, violation.description);
        }
    }

    Ok(())
}

/// Demo 2: Task Scoring and Prioritization Algorithms
async fn demo_task_scoring(service: &mut AgenticDevelopmentService) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Demo 2: Task Scoring and Prioritization Algorithms");
    println!("---------------------------------------------------");

    // Create various tasks with different characteristics
    let tasks = vec![
        // High business value, low effort task
        Task {
            id: "task-1".to_string(),
            title: "Fix Critical Security Bug".to_string(),
            description: "Fix authentication bypass vulnerability".to_string(),
            task_type: TaskType::Security,
            priority: TaskPriority::Critical,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Moderate,
            scoring_factors: TaskScoringFactors {
                business_value: 0.95,
                technical_debt_impact: 0.1,
                user_impact: 0.9,
                dependencies_count: 0,
                estimated_effort_hours: 4.0,
                risk_level: 0.3,
                urgency: 0.9,
                team_capacity_impact: 0.2,
                learning_value: 0.3,
                strategic_alignment: 0.8,
            },
            scope: "security".to_string(),
            assigned_to: None,
            dependencies: vec![],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: Some(Utc::now() + chrono::Duration::hours(24)),
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        },
        // High technical debt impact task
        Task {
            id: "task-2".to_string(),
            title: "Refactor Legacy Code".to_string(),
            description: "Modernize old authentication system".to_string(),
            task_type: TaskType::Refactor,
            priority: TaskPriority::High,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Complex,
            scoring_factors: TaskScoringFactors {
                business_value: 0.6,
                technical_debt_impact: 0.9,
                user_impact: 0.4,
                dependencies_count: 3,
                estimated_effort_hours: 16.0,
                risk_level: 0.5,
                urgency: 0.4,
                team_capacity_impact: 0.6,
                learning_value: 0.7,
                strategic_alignment: 0.6,
            },
            scope: "backend".to_string(),
            assigned_to: None,
            dependencies: vec!["task-1".to_string()],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        },
        // High user impact feature
        Task {
            id: "task-3".to_string(),
            title: "Add Dark Mode Support".to_string(),
            description: "Implement dark mode for better user experience".to_string(),
            task_type: TaskType::Feature,
            priority: TaskPriority::Normal,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Moderate,
            scoring_factors: TaskScoringFactors {
                business_value: 0.7,
                technical_debt_impact: 0.2,
                user_impact: 0.8,
                dependencies_count: 1,
                estimated_effort_hours: 12.0,
                risk_level: 0.2,
                urgency: 0.5,
                team_capacity_impact: 0.4,
                learning_value: 0.5,
                strategic_alignment: 0.7,
            },
            scope: "frontend".to_string(),
            assigned_to: None,
            dependencies: vec![],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        },
    ];

    // Add tasks to the system
    for task in tasks {
        service.add_task(task)?;
    }
    println!("✅ Added {} tasks to the system", 3);

    // Test different prioritization strategies
    let strategies = vec![
        ("Weighted Scoring", PrioritizationStrategy::WeightedScoring),
        ("Business Value First", PrioritizationStrategy::BusinessValueFirst),
        ("Technical Debt First", PrioritizationStrategy::TechnicalDebtFirst),
        ("User Impact First", PrioritizationStrategy::UserImpactFirst),
        ("Risk-Adjusted Return", PrioritizationStrategy::RiskAdjustedReturn),
    ];

    for (strategy_name, strategy) in strategies {
        let prioritization = service.prioritize_tasks("all", strategy).await?;
        println!("\n📋 {} Strategy Results:", strategy_name);
        println!("   Total tasks: {}", prioritization.prioritized_tasks.len());
        println!("   Average score: {:.3}", prioritization.stats.average_score);
        
        if let Some(top_task) = prioritization.prioritized_tasks.first() {
            println!("   Top task: {} (Score: {:.3})", top_task.task_id, top_task.overall_score);
        }
        
        println!("   Recommendations:");
        for recommendation in &prioritization.recommendations {
            println!("     - {}", recommendation);
        }
    }

    Ok(())
}

/// Demo 3: Real-time Agent Coordination Mechanisms
async fn demo_real_time_coordination(service: &mut AgenticDevelopmentService) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🤝 Demo 3: Real-time Agent Coordination Mechanisms");
    println!("------------------------------------------------");

    // Register multiple agents
    let agents = vec![
        AgentInfo {
            id: "agent-frontend".to_string(),
            name: "Frontend Agent".to_string(),
            agent_type: "frontend-developer".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "frontend".to_string(),
            capabilities: vec!["react".to_string(), "typescript".to_string(), "ui-design".to_string()],
            last_heartbeat: Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics {
                tasks_completed: 15,
                tasks_failed: 1,
                avg_completion_time_seconds: 3600.0,
                success_rate: 0.94,
                collaboration_score: 0.8,
                avg_response_time_ms: 120.0,
            },
        },
        AgentInfo {
            id: "agent-backend".to_string(),
            name: "Backend Agent".to_string(),
            agent_type: "backend-developer".to_string(),
            status: AgentStatus::Working,
            current_task_id: Some("task-2".to_string()),
            assigned_scope: "backend".to_string(),
            capabilities: vec!["rust".to_string(), "postgresql".to_string(), "api-design".to_string()],
            last_heartbeat: Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics {
                tasks_completed: 22,
                tasks_failed: 2,
                avg_completion_time_seconds: 4800.0,
                success_rate: 0.92,
                collaboration_score: 0.7,
                avg_response_time_ms: 180.0,
            },
        },
        AgentInfo {
            id: "agent-security".to_string(),
            name: "Security Agent".to_string(),
            agent_type: "security-specialist".to_string(),
            status: AgentStatus::Busy,
            current_task_id: Some("task-1".to_string()),
            assigned_scope: "security".to_string(),
            capabilities: vec!["security-audit".to_string(), "penetration-testing".to_string(), "compliance".to_string()],
            last_heartbeat: Utc::now(),
            is_online: true,
            performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics {
                tasks_completed: 8,
                tasks_failed: 0,
                avg_completion_time_seconds: 2400.0,
                success_rate: 1.0,
                collaboration_score: 0.9,
                avg_response_time_ms: 90.0,
            },
        },
    ];

    for agent in agents {
        service.register_agent(agent).await?;
    }
    println!("✅ Registered {} agents", 3);

    // Create a coordination session
    let session_id = service.create_session(
        "Security Bug Fix Coordination".to_string(),
        vec!["agent-security".to_string(), "agent-backend".to_string()],
    ).await?;
    println!("✅ Created coordination session: {}", session_id);

    // Send messages between agents
    let messages = vec![
        AgentMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::Critical,
            sender_id: "system".to_string(),
            recipient_ids: vec!["agent-security".to_string()],
            content: "Critical security bug detected. Please investigate immediately.".to_string(),
            payload: Some(serde_json::json!({
                "task_id": "task-1",
                "severity": "critical",
                "affected_components": ["auth", "api"]
            })),
            timestamp: Utc::now(),
            requires_ack: true,
            expires_at: Some(Utc::now() + chrono::Duration::hours(1)),
            metadata: HashMap::new(),
        },
        AgentMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::CoordinationRequest,
            priority: MessagePriority::High,
            sender_id: "agent-security".to_string(),
            recipient_ids: vec!["agent-backend".to_string()],
            content: "Need backend changes to fix auth vulnerability. Can we coordinate?".to_string(),
            payload: Some(serde_json::json!({
                "session_id": session_id,
                "required_changes": ["auth-middleware", "user-model"]
            })),
            timestamp: Utc::now(),
            requires_ack: true,
            expires_at: None,
            metadata: HashMap::new(),
        },
    ];

    for message in messages {
        service.send_message(message).await?;
    }
    println!("✅ Sent {} coordination messages", 2);

    // Display coordination statistics
    let coord_stats = service.coordination_system.get_stats();
    println!("📊 Coordination Statistics:");
    println!("   Active agents: {}", coord_stats.active_agents);
    println!("   Active sessions: {}", coord_stats.active_sessions);
    println!("   Total messages: {}", coord_stats.total_messages);
    println!("   Messages delivered: {}", coord_stats.messages_delivered);
    println!("   Coordination efficiency: {:.2}", coord_stats.coordination_efficiency);

    Ok(())
}

/// Demo 4: Conflict Prevention and Resolution Systems
async fn demo_conflict_prevention(service: &mut AgenticDevelopmentService) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🛡️ Demo 4: Conflict Prevention and Resolution Systems");
    println!("---------------------------------------------------");

    // Track file access to simulate conflicts
    service.conflict_prevention_system.track_file_access(
        PathBuf::from("src/auth/mod.rs"),
        "agent-security".to_string(),
        rhema_coordination::agent::conflict_prevention::FileAccessType::Write,
    );

    service.conflict_prevention_system.track_file_access(
        PathBuf::from("src/auth/mod.rs"),
        "agent-backend".to_string(),
        rhema_coordination::agent::conflict_prevention::FileAccessType::Write,
    );

    // Track dependency usage conflicts
    service.conflict_prevention_system.track_dependency_usage(
        "tokio".to_string(),
        "1.28".to_string(),
        "agent-backend".to_string(),
    );

    service.conflict_prevention_system.track_dependency_usage(
        "tokio".to_string(),
        "1.32".to_string(),
        "agent-frontend".to_string(),
    );

    // Track resource access conflicts
    service.conflict_prevention_system.track_resource_access(
        "database-connection-pool".to_string(),
        "connection-pool".to_string(),
        "agent-backend".to_string(),
        "acquire".to_string(),
    );

    service.conflict_prevention_system.track_resource_access(
        "database-connection-pool".to_string(),
        "connection-pool".to_string(),
        "agent-security".to_string(),
        "acquire".to_string(),
    );

    println!("✅ Tracked various resource accesses");

    // Detect conflicts
    let conflicts = service.detect_conflicts().await?;
    println!("🔍 Detected {} conflicts", conflicts.len());

    for conflict in &conflicts {
        println!("\n⚠️  Conflict Details:");
        println!("   ID: {}", conflict.id);
        println!("   Type: {:?}", conflict.conflict_type);
        println!("   Severity: {:?}", conflict.severity);
        println!("   Description: {}", conflict.description);
        println!("   Involved agents: {:?}", conflict.involved_agents);
        println!("   Status: {:?}", conflict.status);
    }

    // Resolve conflicts using different strategies
    for conflict in &conflicts {
        let strategy = match conflict.conflict_type {
            ConflictType::FileModification => ResolutionStrategy::Collaborative,
            ConflictType::Dependency => ResolutionStrategy::Automatic,
            ConflictType::Resource => ResolutionStrategy::Manual,
            _ => ResolutionStrategy::Automatic,
        };

        let resolution = service.resolve_conflict(&conflict.id, strategy).await?;
        println!("\n✅ Resolved conflict {} using {:?} strategy", conflict.id, strategy);
        println!("   Resolution successful: {}", resolution.successful);
        println!("   Time to resolution: {} seconds", resolution.metrics.time_to_resolution_seconds);
        println!("   Actions taken: {}", resolution.actions.len());
    }

    // Add prevention rules
    let prevention_rule = rhema_coordination::agent::conflict_prevention::PreventionRule {
        id: "file-access-rule".to_string(),
        name: "File Access Prevention Rule".to_string(),
        description: "Prevent concurrent modifications to critical files".to_string(),
        rule_type: rhema_coordination::agent::conflict_prevention::PreventionRuleType::FileAccess,
        conditions: vec![rhema_coordination::agent::conflict_prevention::RuleCondition {
            condition_type: "concurrent_file_access".to_string(),
            parameters: HashMap::new(),
            operator: "greater_than".to_string(),
            value: serde_json::json!(1),
        }],
        actions: vec![rhema_coordination::agent::conflict_prevention::RuleAction {
            action_type: "notify_agents".to_string(),
            parameters: HashMap::new(),
            priority: 1,
        }],
        active: true,
        priority: 1,
    };

    service.conflict_prevention_system.add_prevention_rule(prevention_rule)?;
    println!("✅ Added file access prevention rule");

    Ok(())
}

/// Demo 5: Integrated Workflow
async fn demo_integrated_workflow(service: &mut AgenticDevelopmentService) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Demo 5: Integrated Workflow");
    println!("-----------------------------");

    // Simulate a complete development workflow
    println!("1. 📋 Task Planning Phase");
    let planning_session = service.create_session(
        "Sprint Planning".to_string(),
        vec!["agent-frontend".to_string(), "agent-backend".to_string(), "agent-security".to_string()],
    ).await?;
    println!("   Created planning session: {}", planning_session);

    println!("2. 🎯 Task Prioritization");
    let prioritization = service.prioritize_tasks("all", PrioritizationStrategy::WeightedScoring).await?;
    println!("   Prioritized {} tasks", prioritization.prioritized_tasks.len());

    println!("3. 🔒 Constraint Enforcement");
    let workflow_context = ConstraintContext {
        current_cpu_percent: 60.0,
        current_memory_mb: 1200,
        current_disk_mb: 3000,
        accessed_files: vec!["src/main.rs".to_string(), "Cargo.toml".to_string()],
        network_endpoints: vec!["api.github.com".to_string()],
        response_time_ms: 100,
        custom_data: HashMap::new(),
    };
    let constraint_result = service.enforce_constraints("development", &workflow_context).await?;
    println!("   Constraints satisfied: {}", constraint_result.satisfied);

    println!("4. 🤝 Agent Coordination");
    let coordination_message = AgentMessage {
        id: Uuid::new_v4().to_string(),
        message_type: MessageType::TaskAssignment,
        priority: MessagePriority::High,
        sender_id: "system".to_string(),
        recipient_ids: vec!["agent-frontend".to_string(), "agent-backend".to_string()],
        content: "New feature development starting. Please coordinate on API design.".to_string(),
        payload: Some(serde_json::json!({
            "feature": "user-preferences",
            "priority": "high",
            "deadline": "2024-02-15"
        })),
        timestamp: Utc::now(),
        requires_ack: true,
        expires_at: None,
        metadata: HashMap::new(),
    };
    service.send_message(coordination_message).await?;
    println!("   Sent coordination message");

    println!("5. 🛡️ Conflict Prevention");
    let conflicts = service.detect_conflicts().await?;
    println!("   Detected {} potential conflicts", conflicts.len());

    if !conflicts.is_empty() {
        for conflict in &conflicts {
            let resolution = service.resolve_conflict(&conflict.id, ResolutionStrategy::Collaborative).await?;
            println!("   Resolved conflict {}: {}", conflict.id, resolution.successful);
        }
    }

    println!("6. 📊 Workflow Completion");
    println!("   All phases completed successfully!");
    println!("   System ready for production development");

    Ok(())
} 