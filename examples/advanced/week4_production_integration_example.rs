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

use rhema_ai::{
    AIService, AIServiceConfig, AIRequest, AIResponse,
    CoordinationIntegration, CoordinationConfig,
    ProductionIntegration, ProductionConfig,
    PatternCompositionEngine, PatternTemplate, TemplateParameter,
    CompositionRule, ComposedPattern, PatternValidationEngine,
    AgentInfo, AgentStatus, AgentMessage, MessageType, MessagePriority,
    PatternContext, PatternMetadata, PatternCategory, PatternState,
    ValidationResult, PatternResult, PatternError
};
use rhema_core::RhemaResult;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::Utc;
use tracing::{info, warn, error};

/// Week 4: Production Integration and Advanced Pattern Features Example
/// 
/// This example demonstrates:
/// 1. Production Integration - Connect coordination with AI service
/// 2. Advanced Pattern Features - Pattern validation and composition
/// 3. Circuit breaker and load balancing for fault tolerance
/// 4. Pattern templates and composition rules
/// 5. Advanced validation and monitoring

#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Week 4: Production Integration and Advanced Pattern Features Example");

    // Step 1: Set up AI Service with production configuration
    let ai_service = setup_ai_service().await?;
    
    // Step 2: Set up coordination integration
    let coordination = setup_coordination_integration().await?;
    
    // Step 3: Create production integration
    let production_integration = setup_production_integration(ai_service, coordination).await?;
    
    // Step 4: Set up pattern composition engine
    let composition_engine = setup_pattern_composition_engine().await?;
    
    // Step 5: Demonstrate production features
    demonstrate_production_features(&production_integration).await?;
    
    // Step 6: Demonstrate pattern composition
    demonstrate_pattern_composition(&composition_engine).await?;
    
    // Step 7: Demonstrate advanced validation
    demonstrate_advanced_validation(&composition_engine).await?;
    
    // Step 8: Demonstrate fault tolerance
    demonstrate_fault_tolerance(&production_integration).await?;
    
    info!("✅ Week 4 example completed successfully!");
    Ok(())
}

/// Set up AI service with production-ready configuration
async fn setup_ai_service() -> RhemaResult<AIService> {
    info!("🔧 Setting up AI Service with production configuration...");
    
    let config = AIServiceConfig {
        api_key: "production-api-key".to_string(),
        base_url: "https://api.production.ai".to_string(),
        timeout_seconds: 30,
        max_concurrent_requests: 100,
        rate_limit_per_minute: 1000,
        cache_ttl_seconds: 300,
        model_version: "gpt-4-turbo".to_string(),
        enable_caching: true,
        enable_rate_limiting: true,
        enable_monitoring: true,
        enable_lock_file_awareness: true,
        lock_file_path: Some(PathBuf::from("Cargo.lock")),
        auto_validate_lock_file: true,
        conflict_prevention_enabled: true,
        dependency_version_consistency: true,
        enable_agent_state_management: true,
        max_concurrent_agents: 50,
        max_block_time_seconds: 60,
        agent_persistence_config: None,
        enable_coordination_integration: true,
        coordination_config: Some(CoordinationConfig::default()),
        enable_advanced_conflict_prevention: true,
        advanced_conflict_prevention_config: None,
    };

    let ai_service = AIService::new(config).await?;
    info!("✅ AI Service configured and ready");
    Ok(ai_service)
}

/// Set up coordination integration
async fn setup_coordination_integration() -> RhemaResult<CoordinationIntegration> {
    info!("🔧 Setting up Coordination Integration...");
    
    let config = CoordinationConfig {
        run_local_server: true,
        server_address: Some("localhost:8080".to_string()),
        auto_register_agents: true,
        sync_messages: true,
        sync_tasks: true,
        enable_health_monitoring: true,
        syneidesis: None,
    };

    let coordination = CoordinationIntegration::new(
        rhema_ai::agent::real_time_coordination::RealTimeCoordinationSystem::new(),
        Some(config)
    ).await?;
    
    info!("✅ Coordination Integration configured and ready");
    Ok(coordination)
}

/// Set up production integration
async fn setup_production_integration(
    ai_service: AIService,
    coordination: CoordinationIntegration,
) -> RhemaResult<ProductionIntegration> {
    info!("🔧 Setting up Production Integration...");
    
    let config = ProductionConfig {
        production_mode: true,
        health_check_interval: 30,
        circuit_breaker: rhema_ai::production_integration::CircuitBreakerConfig {
            failure_threshold: 5,
            timeout_seconds: 60,
            success_threshold: 2,
            enabled: true,
        },
        load_balancing: rhema_ai::production_integration::LoadBalancingConfig {
            strategy: rhema_ai::production_integration::LoadBalancingStrategy::RoundRobin,
            max_concurrent_per_node: 100,
            health_check_enabled: true,
            enabled: true,
        },
        monitoring: rhema_ai::production_integration::MonitoringConfig {
            enable_metrics: true,
            enable_tracing: true,
            enable_logging: true,
            metrics_export_interval: 60,
            alert_thresholds: rhema_ai::production_integration::AlertThresholds {
                error_rate_threshold: 0.1,
                latency_threshold_ms: 1000,
                memory_usage_threshold: 0.8,
                cpu_usage_threshold: 0.8,
            },
        },
        scaling: rhema_ai::production_integration::ScalingConfig {
            auto_scaling_enabled: false,
            min_instances: 1,
            max_instances: 10,
            scale_up_threshold: 0.7,
            scale_down_threshold: 0.3,
            cooldown_seconds: 300,
        },
        security: rhema_ai::production_integration::SecurityConfig {
            enable_auth: false,
            enable_authorization: false,
            rate_limiting_enabled: true,
            rate_limit_per_minute: 1000,
            enable_audit_logging: false,
        },
    };

    let production_integration = ProductionIntegration::new(
        ai_service,
        coordination,
        config
    ).await?;
    
    info!("✅ Production Integration configured and ready");
    Ok(production_integration)
}

/// Set up pattern composition engine
async fn setup_pattern_composition_engine() -> RhemaResult<PatternCompositionEngine> {
    info!("🔧 Setting up Pattern Composition Engine...");
    
    let engine = PatternCompositionEngine::new();
    
    // Create pattern templates
    create_pattern_templates(&engine).await?;
    
    // Create composition rules
    create_composition_rules(&engine).await?;
    
    info!("✅ Pattern Composition Engine configured and ready");
    Ok(engine)
}

/// Create pattern templates
async fn create_pattern_templates(engine: &PatternCompositionEngine) -> RhemaResult<()> {
    info!("📋 Creating pattern templates...");
    
    // Template 1: Code Review Pattern
    let code_review_template = PatternTemplate {
        id: "code_review_template".to_string(),
        name: "Code Review Pattern".to_string(),
        description: "Automated code review with multiple reviewers".to_string(),
        version: "1.0.0".to_string(),
        category: PatternCategory::Collaboration,
        parameters: HashMap::from([
            ("reviewers_count".to_string(), TemplateParameter {
                name: "reviewers_count".to_string(),
                parameter_type: rhema_ai::agent::patterns::composition::ParameterType::Integer,
                default_value: Some(json!(2)),
                required: true,
                description: "Number of reviewers required".to_string(),
                validation_rules: vec![
                    rhema_ai::agent::patterns::composition::ValidationRule {
                        rule_type: rhema_ai::agent::patterns::composition::ValidationRuleType::MinValue,
                        value: json!(1),
                        error_message: "At least 1 reviewer required".to_string(),
                    },
                    rhema_ai::agent::patterns::composition::ValidationRule {
                        rule_type: rhema_ai::agent::patterns::composition::ValidationRuleType::MaxValue,
                        value: json!(5),
                        error_message: "Maximum 5 reviewers allowed".to_string(),
                    },
                ],
            }),
            ("review_timeout_hours".to_string(), TemplateParameter {
                name: "review_timeout_hours".to_string(),
                parameter_type: rhema_ai::agent::patterns::composition::ParameterType::Integer,
                default_value: Some(json!(24)),
                required: false,
                description: "Review timeout in hours".to_string(),
                validation_rules: vec![
                    rhema_ai::agent::patterns::composition::ValidationRule {
                        rule_type: rhema_ai::agent::patterns::composition::ValidationRuleType::MinValue,
                        value: json!(1),
                        error_message: "Minimum 1 hour timeout".to_string(),
                    },
                ],
            }),
        ]),
        constraints: vec![],
        dependencies: vec![],
        metadata: PatternMetadata {
            id: "code_review_template".to_string(),
            name: "Code Review Pattern".to_string(),
            description: "Automated code review with multiple reviewers".to_string(),
            category: PatternCategory::Collaboration,
            version: "1.0.0".to_string(),
            author: "Rhema AI".to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            tags: vec!["code-review".to_string(), "collaboration".to_string()],
            required_capabilities: vec!["code-review".to_string()],
            required_resources: vec!["memory".to_string()],
            constraints: vec![],
            dependencies: vec![],
        },
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    
    engine.create_template(code_review_template).await?;
    
    // Template 2: Testing Pattern
    let testing_template = PatternTemplate {
        id: "testing_template".to_string(),
        name: "Testing Pattern".to_string(),
        description: "Automated testing with multiple test types".to_string(),
        version: "1.0.0".to_string(),
        category: PatternCategory::TaskDistribution,
        parameters: HashMap::from([
            ("test_types".to_string(), TemplateParameter {
                name: "test_types".to_string(),
                parameter_type: rhema_ai::agent::patterns::composition::ParameterType::Array,
                default_value: Some(json!(["unit", "integration"])),
                required: true,
                description: "Types of tests to run".to_string(),
                validation_rules: vec![],
            }),
        ]),
        constraints: vec![],
        dependencies: vec![],
        metadata: PatternMetadata {
            id: "testing_template".to_string(),
            name: "Testing Pattern".to_string(),
            description: "Automated testing with multiple test types".to_string(),
            category: PatternCategory::TaskDistribution,
            version: "1.0.0".to_string(),
            author: "Rhema AI".to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            tags: vec!["testing".to_string(), "automation".to_string()],
            required_capabilities: vec!["testing".to_string()],
            required_resources: vec!["cpu".to_string(), "memory".to_string()],
            constraints: vec![],
            dependencies: vec![],
        },
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    
    engine.create_template(testing_template).await?;
    
    info!("✅ Pattern templates created");
    Ok(())
}

/// Create composition rules
async fn create_composition_rules(engine: &PatternCompositionEngine) -> RhemaResult<()> {
    info!("📋 Creating composition rules...");
    
    // Rule 1: Auto-add testing after code review
    let testing_rule = CompositionRule {
        id: "auto_testing_rule".to_string(),
        name: "Auto Testing Rule".to_string(),
        description: "Automatically add testing pattern after code review".to_string(),
        priority: 1,
        conditions: vec![
            rhema_ai::agent::patterns::composition::CompositionCondition {
                condition_type: rhema_ai::agent::patterns::composition::ConditionType::PatternExists,
                parameters: HashMap::from([
                    ("pattern_id".to_string(), json!("code_review_template")),
                ]),
            },
        ],
        actions: vec![
            rhema_ai::agent::patterns::composition::CompositionAction {
                action_type: rhema_ai::agent::patterns::composition::ActionType::AddPattern,
                parameters: HashMap::from([
                    ("pattern_id".to_string(), json!("testing_template")),
                ]),
            },
        ],
        enabled: true,
    };
    
    engine.add_composition_rule(testing_rule).await;
    
    // Rule 2: Add monitoring for complex patterns
    let monitoring_rule = CompositionRule {
        id: "monitoring_rule".to_string(),
        name: "Monitoring Rule".to_string(),
        description: "Add monitoring for patterns with high complexity".to_string(),
        priority: 2,
        conditions: vec![
            rhema_ai::agent::patterns::composition::CompositionCondition {
                condition_type: rhema_ai::agent::patterns::composition::ConditionType::Custom,
                parameters: HashMap::from([
                    ("complexity_threshold".to_string(), json!(7)),
                ]),
            },
        ],
        actions: vec![
            rhema_ai::agent::patterns::composition::CompositionAction {
                action_type: rhema_ai::agent::patterns::composition::ActionType::AddConstraint,
                parameters: HashMap::from([
                    ("constraint_type".to_string(), json!("monitoring")),
                ]),
            },
        ],
        enabled: true,
    };
    
    engine.add_composition_rule(monitoring_rule).await;
    
    info!("✅ Composition rules created");
    Ok(())
}

/// Demonstrate production features
async fn demonstrate_production_features(
    production_integration: &ProductionIntegration,
) -> RhemaResult<()> {
    info!("🎯 Demonstrating Production Features...");
    
    // 1. Register agents with production features
    let agent1 = AgentInfo {
        id: "agent-1".to_string(),
        name: "Code Reviewer Agent".to_string(),
        capabilities: vec!["code-review".to_string(), "testing".to_string()],
        status: AgentStatus::Idle,
        performance_metrics: Default::default(),
        current_workload: 0.0,
        assigned_tasks: vec![],
    };
    
    production_integration.register_agent(agent1).await?;
    info!("✅ Agent registered with production features");
    
    // 2. Send coordination message with production features
    let message = AgentMessage {
        id: "msg-1".to_string(),
        sender_id: "agent-1".to_string(),
        recipient_id: "agent-2".to_string(),
        message_type: MessageType::Task,
        priority: MessagePriority::High,
        content: "Please review this code change".to_string(),
        timestamp: Utc::now(),
        metadata: HashMap::new(),
    };
    
    production_integration.send_coordination_message(message).await?;
    info!("✅ Coordination message sent with production features");
    
    // 3. Process AI request with production features
    let ai_request = AIRequest {
        id: "req-1".to_string(),
        prompt: "Analyze this code for potential issues".to_string(),
        model: "gpt-4-turbo".to_string(),
        temperature: 0.1,
        max_tokens: 1000,
        user_id: Some("user-1".to_string()),
        session_id: Some("session-1".to_string()),
        created_at: Utc::now(),
        lock_file_context: None,
        task_type: None,
        scope_path: None,
    };
    
    match production_integration.process_ai_request(ai_request).await {
        Ok(response) => {
            info!("✅ AI request processed successfully: {}", response.id);
        }
        Err(e) => {
            warn!("⚠️ AI request failed: {}", e);
        }
    }
    
    // 4. Check health status
    let health_status = production_integration.get_health_status().await;
    info!("🏥 Health Status: {:?}", health_status);
    
    // 5. Get production metrics
    let metrics = production_integration.get_metrics().await;
    info!("📊 Production Metrics: {} requests, {} successful, {} failed", 
          metrics.total_requests, metrics.successful_requests, metrics.failed_requests);
    
    // 6. Check circuit breaker status
    let circuit_breaker_status = production_integration.get_circuit_breaker_status().await;
    info!("🔌 Circuit Breaker Status: {:?}", circuit_breaker_status);
    
    Ok(())
}

/// Demonstrate pattern composition
async fn demonstrate_pattern_composition(
    engine: &PatternCompositionEngine,
) -> RhemaResult<()> {
    info!("🎯 Demonstrating Pattern Composition...");
    
    // 1. Create pattern context
    let context = PatternContext {
        agents: vec![
            AgentInfo {
                id: "agent-1".to_string(),
                name: "Reviewer 1".to_string(),
                capabilities: vec!["code-review".to_string()],
                status: AgentStatus::Idle,
                performance_metrics: Default::default(),
                current_workload: 0.0,
                assigned_tasks: vec![],
            },
            AgentInfo {
                id: "agent-2".to_string(),
                name: "Reviewer 2".to_string(),
                capabilities: vec!["code-review".to_string()],
                status: AgentStatus::Idle,
                performance_metrics: Default::default(),
                current_workload: 0.0,
                assigned_tasks: vec![],
            },
            AgentInfo {
                id: "agent-3".to_string(),
                name: "Tester".to_string(),
                capabilities: vec!["testing".to_string()],
                status: AgentStatus::Idle,
                performance_metrics: Default::default(),
                current_workload: 0.0,
                assigned_tasks: vec![],
            },
        ],
        resources: Default::default(),
        constraints: vec![],
        state: PatternState {
            pattern_id: "composed-pattern".to_string(),
            phase: rhema_ai::agent::patterns::PatternPhase::Initializing,
            started_at: Utc::now(),
            ended_at: None,
            progress: 0.0,
            status: rhema_ai::agent::patterns::PatternStatus::Pending,
            data: HashMap::new(),
        },
        config: Default::default(),
        session_id: Some("session-1".to_string()),
        parent_pattern_id: None,
    };
    
    // 2. Compose patterns
    let pattern_ids = vec![
        "code_review_template".to_string(),
    ];
    
    let composed_pattern = engine.compose_patterns(
        "composed-workflow".to_string(),
        pattern_ids,
        &context,
    ).await?;
    
    info!("✅ Patterns composed successfully: {}", composed_pattern.id);
    info!("📋 Composed patterns: {:?}", composed_pattern.patterns);
    
    // 3. Execute composed pattern
    let result = engine.execute_composed_pattern(&composed_pattern, &context).await?;
    info!("✅ Composed pattern executed successfully: {}", result.pattern_id);
    
    // 4. Get composition statistics
    let stats = engine.get_composition_statistics().await;
    info!("📊 Composition Statistics: {} templates, {} patterns, {} compositions", 
          stats.total_templates, stats.total_patterns, stats.total_compositions);
    
    Ok(())
}

/// Demonstrate advanced validation
async fn demonstrate_advanced_validation(
    engine: &PatternCompositionEngine,
) -> RhemaResult<()> {
    info!("🎯 Demonstrating Advanced Validation...");
    
    // 1. Create validation engine
    let validation_engine = PatternValidationEngine::new();
    
    // 2. Create pattern context with constraints
    let context = PatternContext {
        agents: vec![
            AgentInfo {
                id: "agent-1".to_string(),
                name: "Limited Agent".to_string(),
                capabilities: vec!["basic-review".to_string()], // Limited capabilities
                status: AgentStatus::Busy, // Busy agent
                performance_metrics: Default::default(),
                current_workload: 0.9, // High workload
                assigned_tasks: vec!["task-1".to_string(), "task-2".to_string()],
            },
        ],
        resources: Default::default(),
        constraints: vec![
            rhema_ai::agent::patterns::Constraint {
                id: "constraint-1".to_string(),
                constraint_type: rhema_ai::agent::patterns::ConstraintType::AgentCapability,
                parameters: HashMap::from([
                    ("capability".to_string(), json!("advanced-review")),
                ]),
                priority: 1,
                is_hard: true,
            },
        ],
        state: Default::default(),
        config: Default::default(),
        session_id: None,
        parent_pattern_id: None,
    };
    
    // 3. Create pattern metadata with high requirements
    let metadata = PatternMetadata {
        id: "complex-pattern".to_string(),
        name: "Complex Pattern".to_string(),
        description: "A complex pattern with high requirements".to_string(),
        category: PatternCategory::Collaboration,
        version: "1.0.0".to_string(),
        author: "Test".to_string(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
        tags: vec!["complex".to_string()],
        required_capabilities: vec!["advanced-review".to_string(), "testing".to_string()],
        required_resources: vec!["memory".to_string(), "cpu".to_string()],
        constraints: vec![],
        dependencies: vec![],
    };
    
    // 4. Validate pattern (should fail)
    let validation_result = validation_engine.validate_pattern(
        "complex-pattern",
        &context,
        &metadata,
    ).await;
    
    info!("🔍 Validation Result: Valid = {}", validation_result.is_valid);
    if !validation_result.is_valid {
        info!("❌ Validation Errors: {:?}", validation_result.errors);
        info!("⚠️ Validation Warnings: {:?}", validation_result.warnings);
    }
    
    // 5. Create better context for validation
    let better_context = PatternContext {
        agents: vec![
            AgentInfo {
                id: "agent-2".to_string(),
                name: "Advanced Agent".to_string(),
                capabilities: vec!["advanced-review".to_string(), "testing".to_string()],
                status: AgentStatus::Idle,
                performance_metrics: Default::default(),
                current_workload: 0.1,
                assigned_tasks: vec![],
            },
        ],
        resources: Default::default(),
        constraints: vec![],
        state: Default::default(),
        config: Default::default(),
        session_id: None,
        parent_pattern_id: None,
    };
    
    // 6. Validate with better context (should pass)
    let better_validation_result = validation_engine.validate_pattern(
        "complex-pattern",
        &better_context,
        &metadata,
    ).await;
    
    info!("🔍 Better Validation Result: Valid = {}", better_validation_result.is_valid);
    if better_validation_result.is_valid {
        info!("✅ Validation passed with better context");
    }
    
    Ok(())
}

/// Demonstrate fault tolerance
async fn demonstrate_fault_tolerance(
    production_integration: &ProductionIntegration,
) -> RhemaResult<()> {
    info!("🎯 Demonstrating Fault Tolerance...");
    
    // 1. Add nodes to load balancer
    let node1 = rhema_ai::production_integration::NodeInfo {
        id: "node-1".to_string(),
        address: "localhost:8081".to_string(),
        weight: 1.0,
        current_load: 0,
        max_capacity: 100,
    };
    
    let node2 = rhema_ai::production_integration::NodeInfo {
        id: "node-2".to_string(),
        address: "localhost:8082".to_string(),
        weight: 1.0,
        current_load: 0,
        max_capacity: 100,
    };
    
    production_integration.add_node(node1).await;
    production_integration.add_node(node2).await;
    info!("✅ Load balancer nodes added");
    
    // 2. Simulate failures to test circuit breaker
    info!("🔌 Testing circuit breaker with simulated failures...");
    
    for i in 0..10 {
        let ai_request = AIRequest {
            id: format!("fail-test-{}", i),
            prompt: "This will fail".to_string(),
            model: "invalid-model".to_string(),
            temperature: 0.1,
            max_tokens: 100,
            user_id: None,
            session_id: None,
            created_at: Utc::now(),
            lock_file_context: None,
            task_type: None,
            scope_path: None,
        };
        
        match production_integration.process_ai_request(ai_request).await {
            Ok(_) => {
                info!("✅ Request {} succeeded", i);
            }
            Err(e) => {
                warn!("⚠️ Request {} failed as expected: {}", i, e);
            }
        }
    }
    
    // 3. Check circuit breaker status after failures
    let circuit_breaker_status = production_integration.get_circuit_breaker_status().await;
    info!("🔌 Circuit Breaker Status after failures: {:?}", circuit_breaker_status);
    
    // 4. Reset circuit breaker
    production_integration.reset_circuit_breaker().await;
    info!("🔄 Circuit breaker reset");
    
    // 5. Test successful requests
    let successful_request = AIRequest {
        id: "success-test".to_string(),
        prompt: "This should work".to_string(),
        model: "gpt-4-turbo".to_string(),
        temperature: 0.1,
        max_tokens: 100,
        user_id: None,
        session_id: None,
        created_at: Utc::now(),
        lock_file_context: None,
        task_type: None,
        scope_path: None,
    };
    
    match production_integration.process_ai_request(successful_request).await {
        Ok(response) => {
            info!("✅ Successful request processed: {}", response.id);
        }
        Err(e) => {
            warn!("⚠️ Successful request failed: {}", e);
        }
    }
    
    // 6. Get final metrics
    let final_metrics = production_integration.get_metrics().await;
    info!("📊 Final Metrics: {} requests, {} successful, {} failed, {} circuit breaker trips", 
          final_metrics.total_requests, 
          final_metrics.successful_requests, 
          final_metrics.failed_requests,
          final_metrics.circuit_breaker_trips);
    
    Ok(())
} 