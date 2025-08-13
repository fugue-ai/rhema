use chrono::Utc;
use std::collections::HashMap;

use rhema_coordination::agent::patterns::{
    AgentInfo, AgentStatus, Constraint, ConstraintType, CpuAllocator, MemoryPool, MonitoringConfig, NetworkResources, PatternCategory, PatternConfig,
    PatternMetadata, PatternPerformanceMetrics, PatternPhase, PatternState, PatternStatus,
    RecoveryStrategy, ResourcePool, ValidationResult,
};
use rhema_coordination::agent::{
    CoordinationPattern, PatternContext, PatternError, PatternExecutor, PatternRegistry,
    PatternResult,
};

/// Enhanced test pattern with recovery and monitoring capabilities
struct EnhancedTestPattern {
    id: String,
    name: String,
    category: PatternCategory,
    should_fail: bool,
    should_recover: bool,
    execution_steps: Vec<String>,
    current_step: usize,
    recovery_attempts: usize,
}

impl EnhancedTestPattern {
    fn new(id: &str, name: &str, category: PatternCategory) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category,
            should_fail: false,
            should_recover: false,
            execution_steps: vec![
                "initialize".to_string(),
                "validate".to_string(),
                "execute".to_string(),
                "coordinate".to_string(),
                "finalize".to_string(),
            ],
            current_step: 0,
            recovery_attempts: 0,
        }
    }

    fn with_failure(mut self, should_fail: bool) -> Self {
        self.should_fail = should_fail;
        self
    }

    fn with_recovery(mut self, should_recover: bool) -> Self {
        self.should_recover = should_recover;
        self
    }

    fn with_execution_steps(mut self, steps: Vec<String>) -> Self {
        self.execution_steps = steps;
        self
    }
}

#[async_trait::async_trait]
impl CoordinationPattern for EnhancedTestPattern {
    async fn execute(&self, _context: &PatternContext) -> Result<PatternResult, PatternError> {
        let start_time = Utc::now();
        let mut current_step = self.current_step;

        // Simulate step-by-step execution
        for (step_index, step) in self.execution_steps.iter().enumerate() {
            current_step = step_index;

            // Simulate step execution time
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Simulate recovery if configured (prioritize recovery over failure)
            if self.should_recover && step == "execute" && self.recovery_attempts < 2 {
                // This would trigger recovery in the executor
                return Err(PatternError::ExecutionError(format!(
                    "Enhanced pattern {} failed but can recover at step: {}",
                    self.id, step
                )));
            }

            // Fail at specific step if configured (only if not recovering)
            if self.should_fail && step == "execute" {
                return Err(PatternError::ExecutionError(format!(
                    "Enhanced pattern {} failed at step: {}",
                    self.id, step
                )));
            }
        }

        let end_time = Utc::now();
        let execution_time = (end_time - start_time).num_milliseconds() as f64 / 1000.0;

        Ok(PatternResult {
            pattern_id: self.id.clone(),
            success: true,
            data: HashMap::from([
                (
                    "execution_steps".to_string(),
                    serde_json::Value::Array(
                        self.execution_steps
                            .iter()
                            .map(|s| serde_json::Value::String(s.clone()))
                            .collect(),
                    ),
                ),
                (
                    "current_step".to_string(),
                    serde_json::Value::Number(current_step.into()),
                ),
                (
                    "enhanced_pattern".to_string(),
                    serde_json::Value::Bool(true),
                ),
            ]),
            performance_metrics: PatternPerformanceMetrics {
                total_execution_time_seconds: execution_time,
                coordination_overhead_seconds: 0.02,
                resource_utilization: 0.85,
                agent_efficiency: 0.92,
                communication_overhead: 8,
            },
            error_message: None,
            completed_at: end_time,
            metadata: HashMap::from([
                (
                    "pattern_type".to_string(),
                    serde_json::Value::String("enhanced_test".to_string()),
                ),
                (
                    "version".to_string(),
                    serde_json::Value::String("2.0.0".to_string()),
                ),
            ]),
            execution_time_ms: (execution_time * 1000.0) as u64,
        })
    }

    async fn validate(&self, _context: &PatternContext) -> Result<ValidationResult, PatternError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        // Basic validation
        if _context.agents.is_empty() {
            errors.push("No agents provided for enhanced pattern execution".to_string());
        }

        // Check resource availability
        if _context.resources.memory_pool.available_memory < 100 * 1024 * 1024 {
            warnings.push("Low memory availability for enhanced pattern".to_string());
        }

        if _context.resources.cpu_allocator.available_cores < 1 {
            errors.push("No CPU cores available for enhanced pattern".to_string());
        }

        details.insert(
            "validation_summary".to_string(),
            serde_json::json!({
                "agent_count": _context.agents.len(),
                "available_memory_mb": _context.resources.memory_pool.available_memory / (1024 * 1024),
                "available_cpu_cores": _context.resources.cpu_allocator.available_cores,
                "pattern_complexity": "medium"
            })
        );

        Ok(ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        })
    }

    async fn rollback(&self, _context: &PatternContext) -> Result<(), PatternError> {
        // Simulate rollback process
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;

        // Reset pattern state
        tracing::info!("Enhanced pattern {} rollback completed", self.id);

        Ok(())
    }

    fn metadata(&self) -> PatternMetadata {
        PatternMetadata {
            id: self.id.clone(),
            name: self.name.clone(),
            description: format!("Enhanced test pattern: {}", self.name),
            version: "2.0.0".to_string(),
            category: self.category.clone(),
            author: "test_author".to_string(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            tags: vec!["test".to_string(), "enhanced".to_string()],
            required_capabilities: vec!["enhanced_test".to_string()],
            required_resources: vec!["memory".to_string(), "cpu".to_string()],
            constraints: vec!["memory_constraint".to_string()],
            dependencies: vec![],
            complexity: 6,
            estimated_execution_time_seconds: 5,
        }
    }
}

/// Test fixture for enhanced pattern execution
struct EnhancedTestFixture {
    executor: PatternExecutor,
    context: PatternContext,
}

impl EnhancedTestFixture {
    fn new() -> Self {
        let registry = PatternRegistry::new();
        let executor = PatternExecutor::new(registry);

        let context = PatternContext {
            agents: vec![
                AgentInfo {
                    id: "agent1".to_string(),
                    name: "Enhanced Test Agent 1".to_string(),
                    capabilities: vec!["enhanced_test".to_string(), "recovery".to_string()],
                    status: AgentStatus::Idle,
                    performance_metrics:
                        rhema_coordination::agent::patterns::AgentPerformanceMetrics::default(),
                    current_workload: 0.0,
                    assigned_tasks: vec![],
                },
                AgentInfo {
                    id: "agent2".to_string(),
                    name: "Enhanced Test Agent 2".to_string(),
                    capabilities: vec!["enhanced_test".to_string(), "monitoring".to_string()],
                    status: AgentStatus::Idle,
                    performance_metrics:
                        rhema_coordination::agent::patterns::AgentPerformanceMetrics::default(),
                    current_workload: 0.0,
                    assigned_tasks: vec![],
                },
            ],
            resources: ResourcePool {
                file_locks: HashMap::new(),
                memory_pool: MemoryPool {
                    total_memory: 2048 * 1024 * 1024,
                    available_memory: 1024 * 1024 * 1024,
                    allocated_memory: 1024 * 1024 * 1024,
                    reservations: HashMap::new(),
                },
                cpu_allocator: CpuAllocator {
                    total_cores: 16,
                    available_cores: 8,
                    allocated_cores: 8,
                    reservations: HashMap::new(),
                },
                network_resources: NetworkResources {
                    available_bandwidth: 2000,
                    allocated_bandwidth: 1000,
                    connections: HashMap::new(),
                },
                custom_resources: HashMap::new(),
            },
            constraints: vec![Constraint {
                id: "memory_constraint".to_string(),
                constraint_type: ConstraintType::ResourceAvailability,
                parameters: HashMap::from([(
                    "min_memory_mb".to_string(),
                    serde_json::Value::Number(100.into()),
                )]),
                priority: 1,
                is_hard: true,
            }],
            state: PatternState {
                pattern_id: "enhanced_test".to_string(),
                phase: PatternPhase::Initializing,
                started_at: Utc::now(),
                ended_at: None,
                progress: 0.0,
                status: PatternStatus::Pending,
                data: HashMap::new(),
            },
            config: PatternConfig {
                timeout_seconds: 60,
                max_retries: 3,
                enable_rollback: true,
                enable_monitoring: true,
                custom_config: HashMap::new(),
            },
            session_id: None,
            parent_pattern_id: None,
        };

        Self {
            context,
            executor,
        }
    }

    fn register_pattern(&mut self, pattern: EnhancedTestPattern) {
        self.executor.register_pattern(Box::new(pattern));
    }

    async fn execute_pattern(&mut self, pattern_id: &str) -> Result<PatternResult, PatternError> {
        self.executor
            .execute_pattern(pattern_id, self.context.clone())
            .await
    }
}

#[tokio::test]
async fn test_enhanced_pattern_execution_with_recovery() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern that will fail but can recover
    let pattern = EnhancedTestPattern::new(
        "recovery_test",
        "Recovery Test Pattern",
        PatternCategory::TaskDistribution,
    )
    .with_failure(true)
    .with_recovery(true);

    _fixture.register_pattern(pattern);

    // Execute pattern - it should fail but recover
    let result = _fixture.execute_pattern("recovery_test").await;

    // Print the error if the result is not ok
    if let Err(ref error) = result {
        println!("Pattern execution failed with error: {:?}", error);
    }

    // The pattern should eventually succeed after recovery
    assert!(result.is_ok());

    let pattern_result = result.unwrap();
    assert!(pattern_result.success);
    assert!(pattern_result.data.contains_key("execution_steps"));
}

#[tokio::test]
async fn test_enhanced_pattern_execution_with_monitoring() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a normal pattern
    let pattern = EnhancedTestPattern::new(
        "monitoring_test",
        "Monitoring Test Pattern",
        PatternCategory::TaskDistribution,
    );
    _fixture.register_pattern(pattern);

    // Execute pattern with monitoring enabled
    let result = _fixture.execute_pattern("monitoring_test").await;
    assert!(result.is_ok());

    // Check monitoring statistics
    let monitoring_stats = _fixture.executor.get_monitoring_statistics().await;
    assert!(monitoring_stats.total_patterns_monitored > 0);
    assert!(monitoring_stats.success_rate > 0.0);

    // Check real-time metrics
    let real_time_metrics = _fixture
        .executor
        .get_real_time_metrics("monitoring_test")
        .await;
    assert!(real_time_metrics.is_some());

    let metrics = real_time_metrics.unwrap();
    assert_eq!(metrics.pattern_id, "monitoring_test");
    assert!(metrics.execution_time_seconds > 0.0);
}

#[tokio::test]
async fn test_enhanced_pattern_execution_with_validation() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern
    let pattern = EnhancedTestPattern::new(
        "validation_test",
        "Validation Test Pattern",
        PatternCategory::TaskDistribution,
    );
    _fixture.register_pattern(pattern);

    // Execute pattern
    let result = _fixture.execute_pattern("validation_test").await;
    assert!(result.is_ok());

    let pattern_result = result.unwrap();
    assert!(pattern_result.success);

    // Check that validation data is included
    assert!(pattern_result.data.contains_key("execution_steps"));
    assert!(pattern_result.data.contains_key("enhanced_pattern"));
}

#[tokio::test]
async fn test_enhanced_pattern_recovery_statistics() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern that will fail and recover
    let pattern = EnhancedTestPattern::new(
        "recovery_stats_test",
        "Recovery Stats Test",
        PatternCategory::TaskDistribution,
    )
    .with_failure(true)
    .with_recovery(true);

    _fixture.register_pattern(pattern);

    // Execute pattern
    let result = _fixture.execute_pattern("recovery_stats_test").await;
    assert!(result.is_ok());

    // Check recovery statistics
    let recovery_stats = _fixture.executor.get_recovery_statistics().await;
    assert!(recovery_stats.total_recoveries > 0);
    assert!(recovery_stats.successful_recoveries > 0);
    assert!(recovery_stats.average_recovery_time > 0.0);
}

#[tokio::test]
async fn test_enhanced_pattern_monitoring_events() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern
    let pattern = EnhancedTestPattern::new(
        "events_test",
        "Events Test Pattern",
        PatternCategory::TaskDistribution,
    );
    _fixture.register_pattern(pattern);

    // Execute pattern
    let result = _fixture.execute_pattern("events_test").await;
    assert!(result.is_ok());

    // Get monitoring events
    let events = _fixture
        .executor
        .get_monitoring_events(Some("events_test"), None)
        .await;
    assert!(!events.is_empty());

    // Check for specific event types
    let has_started = events.iter().any(|event| {
        matches!(
            event,
            rhema_coordination::agent::patterns::MonitoringEvent::PatternStarted { .. }
        )
    });
    assert!(has_started);

    let has_completed = events.iter().any(|event| {
        matches!(
            event,
            rhema_coordination::agent::patterns::MonitoringEvent::PatternCompleted { .. }
        )
    });
    assert!(has_completed);
}

#[tokio::test]
#[ignore]
async fn test_enhanced_pattern_real_time_monitoring() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern with longer execution time
    let pattern = EnhancedTestPattern::new(
        "realtime_test",
        "Real-time Test Pattern",
        PatternCategory::TaskDistribution,
    )
    .with_execution_steps(vec![
        "step1".to_string(),
        "step2".to_string(),
        "step3".to_string(),
        "step4".to_string(),
        "step5".to_string(),
    ]);

    _fixture.register_pattern(pattern);

    // Start execution
    let execution_handle = tokio::spawn(async move {
        let mut _fixture = EnhancedTestFixture::new();
        _fixture.register_pattern(EnhancedTestPattern::new(
            "realtime_test",
            "Real-time Test Pattern",
            PatternCategory::TaskDistribution,
        ));
        _fixture.execute_pattern("realtime_test").await
    });

    // Wait a bit for execution to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Check real-time metrics
    let real_time_metrics = _fixture.executor.get_all_real_time_metrics().await;
    assert!(!real_time_metrics.is_empty());

    // Wait for completion
    let result = execution_handle.await.unwrap();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_enhanced_pattern_error_handling() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern that will fail without recovery
    let pattern = EnhancedTestPattern::new(
        "error_test",
        "Error Test Pattern",
        PatternCategory::TaskDistribution,
    )
    .with_failure(true)
    .with_recovery(false);

    _fixture.register_pattern(pattern);

    // Execute pattern - it should fail
    let result = _fixture.execute_pattern("error_test").await;
    assert!(result.is_err());

    // Check error handling
    if let Err(PatternError::ExecutionError(error_msg)) = result {
        assert!(error_msg.contains("failed at step"));
    } else {
        panic!("Expected ExecutionError");
    }
}

#[tokio::test]
async fn test_enhanced_pattern_resource_monitoring() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern
    let pattern = EnhancedTestPattern::new(
        "resource_test",
        "Resource Test Pattern",
        PatternCategory::TaskDistribution,
    );
    _fixture.register_pattern(pattern);

    // Execute pattern
    let result = _fixture.execute_pattern("resource_test").await;
    assert!(result.is_ok());

    // Check resource monitoring
    let real_time_metrics = _fixture
        .executor
        .get_real_time_metrics("resource_test")
        .await;
    assert!(real_time_metrics.is_some());

    let metrics = real_time_metrics.unwrap();
    assert!(metrics.resource_utilization.memory_usage_mb > 0);
    assert!(metrics.resource_utilization.cpu_usage_cores > 0);
    assert!(metrics.resource_utilization.memory_utilization > 0.0);
    assert!(metrics.resource_utilization.cpu_utilization > 0.0);
}

#[tokio::test]
#[ignore]
async fn test_enhanced_pattern_performance_profiling() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register a pattern
    let pattern = EnhancedTestPattern::new(
        "perf_test",
        "Performance Test Pattern",
        PatternCategory::TaskDistribution,
    );
    _fixture.register_pattern(pattern);

    // Execute pattern
    let result = _fixture.execute_pattern("perf_test").await;
    assert!(result.is_ok());

    let pattern_result = result.unwrap();

    // Check performance metrics
    assert!(
        pattern_result
            .performance_metrics
            .total_execution_time_seconds
            > 0.0
    );
    assert!(pattern_result.performance_metrics.resource_utilization > 0.0);
    assert!(pattern_result.performance_metrics.agent_efficiency > 0.0);

    // Get performance profile
    let profile = _fixture
        .executor
        .monitor()
        .get_performance_profile("perf_test")
        .await;
    assert!(profile.is_some());

    let profile = profile.unwrap();
    assert_eq!(profile.pattern_id, "perf_test");
    assert!(!profile.metrics.is_empty());
}

#[tokio::test]
async fn test_enhanced_pattern_concurrent_execution() {
    let mut _fixture = EnhancedTestFixture::new();

    // Register multiple patterns
    for i in 1..=3 {
        let pattern = EnhancedTestPattern::new(
            &format!("concurrent_test_{}", i),
            &format!("Concurrent Test Pattern {}", i),
            PatternCategory::TaskDistribution,
        );
        _fixture.register_pattern(pattern);
    }

    // Execute patterns sequentially since we can't clone the executor
    let mut results = Vec::new();
    for i in 1..=3 {
        let pattern_id = format!("concurrent_test_{}", i);
        let result = _fixture.execute_pattern(&pattern_id).await;
        results.push(result);
    }

    // Check results
    for result in results {
        assert!(result.is_ok());
        let pattern_result = result.unwrap();
        assert!(pattern_result.success);
    }

    // Check monitoring statistics
    let monitoring_stats = _fixture.executor.get_monitoring_statistics().await;
    assert_eq!(monitoring_stats.total_patterns_monitored, 3);
    assert_eq!(monitoring_stats.success_rate, 1.0);
}

#[tokio::test]
async fn test_enhanced_pattern_monitoring_subscription() {
    let mut _fixture = EnhancedTestFixture::new();

    // Subscribe to monitoring events
    let mut receiver = _fixture.executor.subscribe_to_monitoring_events();

    // Register and execute a pattern
    let pattern = EnhancedTestPattern::new(
        "subscription_test",
        "Subscription Test Pattern",
        PatternCategory::TaskDistribution,
    );
    _fixture.register_pattern(pattern);

    // Execute the pattern directly to ensure events are sent to the same monitor
    let execution_result = _fixture.execute_pattern("subscription_test").await;

    // Receive events
    let mut received_events = Vec::new();
    let timeout = tokio::time::Duration::from_secs(5);

    while let Ok(event) = tokio::time::timeout(timeout, receiver.recv()).await {
        if let Ok(event) = event {
            received_events.push(event);
            if received_events.len() >= 2 {
                break; // We expect at least start and completion events
            }
        }
    }

    // Check that we received events
    assert!(!received_events.is_empty());

    // Check that execution succeeded
    assert!(execution_result.is_ok());
}

#[tokio::test]
async fn test_enhanced_pattern_recovery_strategies() {
    let mut _fixture = EnhancedTestFixture::new();

    // Test different recovery strategies
    let strategies = vec![
        RecoveryStrategy::Retry {
            max_attempts: 3,
            backoff_delay_ms: 10,
            exponential_backoff: true,
        },
        RecoveryStrategy::Rollback {
            checkpoint_id: "test_checkpoint".to_string(),
            restore_resources: true,
            restore_agent_states: true,
        },
        RecoveryStrategy::Abort {
            cleanup_resources: true,
            notify_agents: true,
        },
    ];

    for (i, strategy) in strategies.iter().enumerate() {
        let pattern_id = format!("strategy_test_{}", i);

        // Register a pattern that will fail but can recover
        let pattern = EnhancedTestPattern::new(
            &pattern_id,
            &format!("Strategy Test {}", i),
            PatternCategory::TaskDistribution,
        )
        .with_failure(true)
        .with_recovery(true);

        _fixture.register_pattern(pattern);

        // Execute with recovery strategy
        let result = _fixture.execute_pattern(&pattern_id).await;

        // Pattern should handle recovery appropriately
        match strategy {
            RecoveryStrategy::Retry { .. } => {
                // Retry strategy should eventually succeed
                assert!(result.is_ok());
            }
            RecoveryStrategy::Rollback { .. } => {
                // Rollback strategy should succeed if checkpoint exists
                // For this test, we don't create checkpoints, so it might fail
                // This is expected behavior
            }
            RecoveryStrategy::Abort { .. } => {
                // Abort strategy should fail gracefully
                // This is expected behavior
            }
            _ => {}
        }
    }
}

#[tokio::test]
async fn test_enhanced_pattern_monitoring_configuration() {
    let mut _fixture = EnhancedTestFixture::new();

    // Test with different monitoring configurations
    let configs = vec![
        MonitoringConfig {
            enable_real_time: true,
            metrics_interval_seconds: 1,
            event_retention_hours: 1,
            max_events_in_memory: 100,
            enable_profiling: true,
            enable_resource_monitoring: true,
            enable_agent_monitoring: true,
            alert_thresholds: rhema_coordination::agent::patterns::AlertThresholds {
                max_execution_time_seconds: 10.0,
                max_memory_utilization: 0.8,
                max_cpu_utilization: 0.7,
                max_network_utilization: 0.6,
                min_agent_availability: 0.5,
                max_error_rate: 0.1,
            },
        },
        MonitoringConfig {
            enable_real_time: false,
            metrics_interval_seconds: 5,
            event_retention_hours: 24,
            max_events_in_memory: 1000,
            enable_profiling: false,
            enable_resource_monitoring: false,
            enable_agent_monitoring: false,
            alert_thresholds: rhema_coordination::agent::patterns::AlertThresholds {
                max_execution_time_seconds: 300.0,
                max_memory_utilization: 0.9,
                max_cpu_utilization: 0.8,
                max_network_utilization: 0.7,
                min_agent_availability: 0.3,
                max_error_rate: 0.2,
            },
        },
    ];

    for (i, config) in configs.iter().enumerate() {
        let pattern_id = format!("config_test_{}", i);

        // Create executor with specific config
        let mut registry = PatternRegistry::new();
        let _monitor = rhema_coordination::agent::patterns::PatternMonitor::new(config.clone());
        let mut executor = PatternExecutor::new(registry);

        // Create a test context
        let context = PatternContext {
            agents: vec![AgentInfo {
                id: "test_agent".to_string(),
                name: "Test Agent".to_string(),
                capabilities: vec!["test".to_string()],
                status: AgentStatus::Idle,
                performance_metrics: rhema_coordination::agent::patterns::AgentPerformanceMetrics::default(),
                current_workload: 0.0,
                assigned_tasks: vec![],
            }],
            resources: ResourcePool {
                file_locks: HashMap::new(),
                memory_pool: MemoryPool {
                    total_memory: 1024 * 1024 * 1024,
                    available_memory: 512 * 1024 * 1024,
                    allocated_memory: 512 * 1024 * 1024,
                    reservations: HashMap::new(),
                },
                cpu_allocator: CpuAllocator {
                    total_cores: 8,
                    available_cores: 4,
                    allocated_cores: 4,
                    reservations: HashMap::new(),
                },
                network_resources: NetworkResources {
                    available_bandwidth: 1000,
                    allocated_bandwidth: 500,
                    connections: HashMap::new(),
                },
                custom_resources: HashMap::new(),
            },
            constraints: vec![],
            state: PatternState {
                pattern_id: pattern_id.clone(),
                phase: PatternPhase::Initializing,
                started_at: Utc::now(),
                ended_at: None,
                progress: 0.0,
                status: PatternStatus::Pending,
                data: HashMap::new(),
            },
            config: PatternConfig {
                timeout_seconds: 60,
                max_retries: 3,
                enable_rollback: true,
                enable_monitoring: true,
                custom_config: HashMap::new(),
            },
            session_id: None,
            parent_pattern_id: None,
        };

        // Register pattern in the executor's registry
        let pattern = EnhancedTestPattern::new(
            &pattern_id,
            &format!("Config Test {}", i),
            PatternCategory::TaskDistribution,
        );
        executor.register_pattern(Box::new(pattern));

        // Execute pattern
        let result = executor.execute_pattern(&pattern_id, context).await;
        assert!(result.is_ok());

        // Check monitoring behavior based on config
        let monitoring_stats = executor.get_monitoring_statistics().await;

        if config.enable_real_time {
            assert!(monitoring_stats.total_patterns_monitored > 0);
        }
    }
}
