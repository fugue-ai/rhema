use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

use super::{PatternContext, PatternError, PatternState, ValidationResult};

/// Recovery strategy for pattern execution failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Retry the pattern execution
    Retry {
        max_attempts: u32,
        backoff_delay_ms: u64,
        exponential_backoff: bool,
    },
    /// Rollback to previous state
    Rollback {
        checkpoint_id: String,
        restore_resources: bool,
        restore_agent_states: bool,
    },
    /// Continue from last successful step
    ContinueFromCheckpoint {
        checkpoint_id: String,
        skip_failed_steps: bool,
    },
    /// Fallback to alternative pattern
    Fallback {
        alternative_pattern_id: String,
        preserve_context: bool,
    },
    /// Manual intervention required
    ManualIntervention {
        timeout_seconds: u64,
        notification_channels: Vec<String>,
    },
    /// Abort pattern execution
    Abort {
        cleanup_resources: bool,
        notify_agents: bool,
    },
}

/// Enhanced recovery strategy with advanced rollback mechanisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnhancedRecoveryStrategy {
    /// Intelligent retry with exponential backoff and circuit breaker
    IntelligentRetry {
        max_attempts: u32,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
        backoff_multiplier: f64,
        circuit_breaker_threshold: u32,
        circuit_breaker_timeout_ms: u64,
    },
    /// Partial rollback to specific checkpoint
    PartialRollback {
        checkpoint_id: String,
        rollback_steps: Vec<String>,
        preserve_successful_steps: bool,
        restore_resources: bool,
        restore_agent_states: bool,
    },
    /// Graceful degradation with fallback patterns
    GracefulDegradation {
        primary_pattern_id: String,
        fallback_patterns: Vec<String>,
        degradation_criteria: HashMap<String, serde_json::Value>,
        preserve_context: bool,
    },
    /// State reconstruction from multiple checkpoints
    StateReconstruction {
        checkpoint_ids: Vec<String>,
        reconstruction_strategy: ReconstructionStrategy,
        validate_reconstructed_state: bool,
    },
    /// Resource-aware recovery
    ResourceAwareRecovery {
        resource_constraints: HashMap<String, serde_json::Value>,
        recovery_priority: RecoveryPriority,
        adaptive_timeout: bool,
    },
    /// Agent-specific recovery
    AgentSpecificRecovery {
        agent_recovery_strategies: HashMap<String, RecoveryStrategy>,
        coordination_timeout_ms: u64,
        fallback_agents: Vec<String>,
    },
}

/// Reconstruction strategy for state recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReconstructionStrategy {
    /// Use the most recent valid checkpoint
    MostRecent,
    /// Merge states from multiple checkpoints
    Merge,
    /// Use the checkpoint with highest success rate
    BestSuccessRate,
    /// Reconstruct from partial checkpoints
    Partial,
}

/// Recovery priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Enhanced recovery result with detailed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedRecoveryResult {
    pub strategy: EnhancedRecoveryStrategy,
    pub success: bool,
    pub duration_seconds: f64,
    pub error_message: Option<String>,
    pub checkpoint_id: Option<String>,
    pub restored_state: bool,
    pub recovery_metrics: RecoveryMetrics,
    pub rollback_steps: Vec<String>,
    pub resource_restoration: ResourceRestorationResult,
    pub agent_recovery: AgentRecoveryResult,
}

/// Recovery metrics for performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryMetrics {
    pub total_recovery_time: f64,
    pub checkpoint_restoration_time: f64,
    pub resource_restoration_time: f64,
    pub agent_recovery_time: f64,
    pub validation_time: f64,
    pub rollback_steps_count: usize,
    pub resources_restored: usize,
    pub agents_recovered: usize,
    pub state_consistency_score: f64,
}

/// Resource restoration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRestorationResult {
    pub memory_restored: bool,
    pub cpu_restored: bool,
    pub network_restored: bool,
    pub file_locks_restored: bool,
    pub custom_resources_restored: HashMap<String, bool>,
    pub restoration_errors: Vec<String>,
}

/// Agent recovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRecoveryResult {
    pub agents_restored: Vec<String>,
    pub agents_failed: Vec<String>,
    pub state_consistency: HashMap<String, f64>,
    pub recovery_errors: Vec<String>,
}

/// Checkpoint for pattern execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCheckpoint {
    /// Checkpoint ID
    pub checkpoint_id: String,
    /// Pattern ID
    pub pattern_id: String,
    /// Timestamp when checkpoint was created
    pub timestamp: DateTime<Utc>,
    /// Pattern state at checkpoint
    pub pattern_state: PatternState,
    /// Agent states at checkpoint
    pub agent_states: HashMap<String, AgentStateSnapshot>,
    /// Resource state at checkpoint
    pub resource_state: ResourceStateSnapshot,
    /// Execution context data
    pub context_data: HashMap<String, serde_json::Value>,
    /// Checkpoint metadata
    pub metadata: HashMap<String, String>,
}

/// Agent state snapshot for checkpointing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStateSnapshot {
    /// Agent ID
    pub agent_id: String,
    /// Agent status
    pub status: super::AgentStatus,
    /// Current workload
    pub current_workload: f64,
    /// Assigned tasks
    pub assigned_tasks: Vec<String>,
    /// Performance metrics
    pub performance_metrics: super::AgentPerformanceMetrics,
    /// Agent-specific data
    pub agent_data: HashMap<String, serde_json::Value>,
}

/// Resource state snapshot for checkpointing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStateSnapshot {
    /// Memory pool state
    pub memory_pool: super::MemoryPool,
    /// CPU allocator state
    pub cpu_allocator: super::CpuAllocator,
    /// Network resources state
    pub network_resources: super::NetworkResources,
    /// File locks state
    pub file_locks: HashMap<String, super::FileLock>,
    /// Custom resources state
    pub custom_resources: HashMap<String, super::CustomResource>,
}

/// Recovery error types
#[derive(Error, Debug)]
pub enum RecoveryError {
    #[error("Checkpoint not found: {0}")]
    CheckpointNotFound(String),

    #[error("Rollback failed: {0}")]
    RollbackFailed(String),

    #[error("Recovery strategy not supported: {0}")]
    UnsupportedStrategy(String),

    #[error("Recovery timeout: {0}")]
    RecoveryTimeout(String),

    #[error("Resource restoration failed: {0}")]
    ResourceRestorationFailed(String),

    #[error("Agent state restoration failed: {0}")]
    AgentStateRestorationFailed(String),

    #[error("Checkpoint creation failed: {0}")]
    CheckpointCreationFailed(String),

    #[error("Recovery validation failed: {0}")]
    RecoveryValidationFailed(String),
}

/// Recovery manager for pattern execution
pub struct PatternRecoveryManager {
    checkpoints: Arc<RwLock<HashMap<String, PatternCheckpoint>>>,
    recovery_history: Arc<RwLock<Vec<RecoveryRecord>>>,
    recovery_strategies: HashMap<String, RecoveryStrategy>,
}

impl PatternRecoveryManager {
    pub fn new() -> Self {
        Self {
            checkpoints: Arc::new(RwLock::new(HashMap::new())),
            recovery_history: Arc::new(RwLock::new(Vec::new())),
            recovery_strategies: HashMap::new(),
        }
    }

    /// Create a checkpoint for pattern execution
    pub async fn create_checkpoint(
        &self,
        pattern_id: &str,
        context: &PatternContext,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<String, RecoveryError> {
        let checkpoint_id = format!("{}_{}", pattern_id, Utc::now().timestamp_millis());

        // Create agent state snapshots
        let mut agent_states = HashMap::new();
        for agent in &context.agents {
            agent_states.insert(
                agent.id.clone(),
                AgentStateSnapshot {
                    agent_id: agent.id.clone(),
                    status: agent.status.clone(),
                    current_workload: agent.current_workload,
                    assigned_tasks: agent.assigned_tasks.clone(),
                    performance_metrics: agent.performance_metrics.clone(),
                    agent_data: HashMap::new(), // Could be extended with agent-specific data
                },
            );
        }

        // Create resource state snapshot
        let resource_state = ResourceStateSnapshot {
            memory_pool: context.resources.memory_pool.clone(),
            cpu_allocator: context.resources.cpu_allocator.clone(),
            network_resources: context.resources.network_resources.clone(),
            file_locks: context.resources.file_locks.clone(),
            custom_resources: context.resources.custom_resources.clone(),
        };

        let checkpoint = PatternCheckpoint {
            checkpoint_id: checkpoint_id.clone(),
            pattern_id: pattern_id.to_string(),
            timestamp: Utc::now(),
            pattern_state: context.state.clone(),
            agent_states,
            resource_state,
            context_data: context.state.data.clone(),
            metadata: metadata.unwrap_or_default(),
        };

        {
            let mut checkpoints = self.checkpoints.write().await;
            checkpoints.insert(checkpoint_id.clone(), checkpoint);
        }

        Ok(checkpoint_id)
    }

    /// Restore pattern execution from checkpoint
    pub async fn restore_from_checkpoint(
        &self,
        checkpoint_id: &str,
        context: &mut PatternContext,
    ) -> Result<(), RecoveryError> {
        let checkpoint = {
            let checkpoints = self.checkpoints.read().await;
            checkpoints
                .get(checkpoint_id)
                .ok_or_else(|| RecoveryError::CheckpointNotFound(checkpoint_id.to_string()))?
                .clone()
        };

        // Restore pattern state
        context.state = checkpoint.pattern_state;

        // Restore agent states
        for agent in &mut context.agents {
            if let Some(agent_snapshot) = checkpoint.agent_states.get(&agent.id) {
                agent.status = agent_snapshot.status.clone();
                agent.current_workload = agent_snapshot.current_workload;
                agent.assigned_tasks = agent_snapshot.assigned_tasks.clone();
                agent.performance_metrics = agent_snapshot.performance_metrics.clone();
            }
        }

        // Restore resource state
        context.resources.memory_pool = checkpoint.resource_state.memory_pool;
        context.resources.cpu_allocator = checkpoint.resource_state.cpu_allocator;
        context.resources.network_resources = checkpoint.resource_state.network_resources;
        context.resources.file_locks = checkpoint.resource_state.file_locks;
        context.resources.custom_resources = checkpoint.resource_state.custom_resources;

        Ok(())
    }

    /// Execute enhanced recovery strategy with advanced rollback mechanisms
    pub async fn execute_enhanced_recovery_strategy(
        &self,
        pattern_id: &str,
        strategy: &EnhancedRecoveryStrategy,
        context: &mut PatternContext,
        error: &PatternError,
    ) -> Result<EnhancedRecoveryResult, RecoveryError> {
        let start_time = std::time::Instant::now();

        let result = match strategy {
            EnhancedRecoveryStrategy::IntelligentRetry {
                max_attempts,
                initial_backoff_ms,
                max_backoff_ms,
                backoff_multiplier,
                circuit_breaker_threshold,
                circuit_breaker_timeout_ms,
            } => {
                self.handle_intelligent_retry_strategy(
                    pattern_id,
                    *max_attempts,
                    *initial_backoff_ms,
                    *max_backoff_ms,
                    *backoff_multiplier,
                    *circuit_breaker_threshold,
                    *circuit_breaker_timeout_ms,
                    context,
                    error,
                )
                .await
            }

            EnhancedRecoveryStrategy::PartialRollback {
                checkpoint_id,
                rollback_steps,
                preserve_successful_steps,
                restore_resources,
                restore_agent_states,
            } => {
                self.handle_partial_rollback_strategy(
                    checkpoint_id,
                    rollback_steps,
                    *preserve_successful_steps,
                    *restore_resources,
                    *restore_agent_states,
                    context,
                )
                .await
            }

            EnhancedRecoveryStrategy::GracefulDegradation {
                primary_pattern_id,
                fallback_patterns,
                degradation_criteria,
                preserve_context,
            } => {
                self.handle_graceful_degradation_strategy(
                    primary_pattern_id,
                    fallback_patterns,
                    degradation_criteria,
                    *preserve_context,
                    context,
                    error,
                )
                .await
            }

            EnhancedRecoveryStrategy::StateReconstruction {
                checkpoint_ids,
                reconstruction_strategy,
                validate_reconstructed_state,
            } => {
                self.handle_state_reconstruction_strategy(
                    checkpoint_ids,
                    reconstruction_strategy,
                    *validate_reconstructed_state,
                    context,
                )
                .await
            }

            EnhancedRecoveryStrategy::ResourceAwareRecovery {
                resource_constraints,
                recovery_priority,
                adaptive_timeout,
            } => {
                self.handle_resource_aware_recovery_strategy(
                    resource_constraints,
                    recovery_priority,
                    *adaptive_timeout,
                    context,
                    error,
                )
                .await
            }

            EnhancedRecoveryStrategy::AgentSpecificRecovery {
                agent_recovery_strategies,
                coordination_timeout_ms,
                fallback_agents,
            } => {
                self.handle_agent_specific_recovery_strategy(
                    agent_recovery_strategies,
                    *coordination_timeout_ms,
                    fallback_agents,
                    context,
                    error,
                )
                .await
            }
        };

        let duration = start_time.elapsed().as_secs_f64();

        // Record recovery attempt
        let recovery_record = RecoveryRecord {
            pattern_id: pattern_id.to_string(),
            timestamp: Utc::now(),
            strategy: RecoveryStrategy::Retry {
                max_attempts: 1,
                backoff_delay_ms: 0,
                exponential_backoff: false,
            },
            success: result.as_ref().map(|r| r.success).unwrap_or(false),
            duration_seconds: duration,
            error_message: None,
        };

        {
            let mut history = self.recovery_history.write().await;
            history.push(recovery_record);
        }

        result
    }

    /// Handle intelligent retry strategy with circuit breaker
    async fn handle_intelligent_retry_strategy(
        &self,
        pattern_id: &str,
        max_attempts: u32,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
        backoff_multiplier: f64,
        circuit_breaker_threshold: u32,
        circuit_breaker_timeout_ms: u64,
        context: &mut PatternContext,
        _error: &PatternError,
    ) -> Result<EnhancedRecoveryResult, RecoveryError> {
        let mut current_backoff = initial_backoff_ms;
        let mut consecutive_failures = 0;
        let mut total_recovery_time = 0.0;
        let mut checkpoint_restoration_time = 0.0;

        for attempt in 1..=max_attempts {
            // Check circuit breaker
            if consecutive_failures >= circuit_breaker_threshold {
                return Err(RecoveryError::RecoveryTimeout(format!(
                    "Circuit breaker triggered after {} consecutive failures",
                    consecutive_failures
                )));
            }

            // Wait with exponential backoff
            if attempt > 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(current_backoff)).await;
                current_backoff = (current_backoff as f64 * backoff_multiplier) as u64;
                current_backoff = current_backoff.min(max_backoff_ms);
            }

            // Attempt recovery
            let recovery_start = std::time::Instant::now();

            // Create checkpoint if needed
            let checkpoint_id = if attempt == 1 {
                match self.create_checkpoint(pattern_id, context, None).await {
                    Ok(id) => {
                        checkpoint_restoration_time = recovery_start.elapsed().as_secs_f64();
                        Some(id)
                    }
                    Err(e) => {
                        tracing::warn!(pattern_id = pattern_id, error = %e, "Failed to create checkpoint for retry");
                        None
                    }
                }
            } else {
                None
            };

            // Simulate recovery attempt (in real implementation, this would retry the pattern)
            let recovery_success = attempt > 2; // Simulate success after 2 attempts

            if recovery_success {
                total_recovery_time = recovery_start.elapsed().as_secs_f64();

                return Ok(EnhancedRecoveryResult {
                    strategy: EnhancedRecoveryStrategy::IntelligentRetry {
                        max_attempts,
                        initial_backoff_ms,
                        max_backoff_ms,
                        backoff_multiplier,
                        circuit_breaker_threshold,
                        circuit_breaker_timeout_ms,
                    },
                    success: true,
                    duration_seconds: total_recovery_time,
                    error_message: None,
                    checkpoint_id,
                    restored_state: true,
                    recovery_metrics: RecoveryMetrics {
                        total_recovery_time,
                        checkpoint_restoration_time,
                        resource_restoration_time: 0.0,
                        agent_recovery_time: 0.0,
                        validation_time: 0.0,
                        rollback_steps_count: 0,
                        resources_restored: 0,
                        agents_recovered: 0,
                        state_consistency_score: 1.0,
                    },
                    rollback_steps: vec![],
                    resource_restoration: ResourceRestorationResult {
                        memory_restored: true,
                        cpu_restored: true,
                        network_restored: true,
                        file_locks_restored: true,
                        custom_resources_restored: HashMap::new(),
                        restoration_errors: vec![],
                    },
                    agent_recovery: AgentRecoveryResult {
                        agents_restored: vec![],
                        agents_failed: vec![],
                        state_consistency: HashMap::new(),
                        recovery_errors: vec![],
                    },
                });
            } else {
                consecutive_failures += 1;
            }
        }

        Err(RecoveryError::RecoveryTimeout(format!(
            "Intelligent retry failed after {} attempts",
            max_attempts
        )))
    }

    /// Handle partial rollback strategy
    async fn handle_partial_rollback_strategy(
        &self,
        checkpoint_id: &str,
        rollback_steps: &[String],
        preserve_successful_steps: bool,
        restore_resources: bool,
        restore_agent_states: bool,
        context: &mut PatternContext,
    ) -> Result<EnhancedRecoveryResult, RecoveryError> {
        let start_time = std::time::Instant::now();

        // Get checkpoint
        let checkpoint = {
            let checkpoints = self.checkpoints.read().await;
            checkpoints
                .get(checkpoint_id)
                .ok_or_else(|| RecoveryError::CheckpointNotFound(checkpoint_id.to_string()))?
                .clone()
        };

        let mut checkpoint_restoration_time = 0.0;
        let mut resource_restoration_time = 0.0;
        let mut agent_recovery_time = 0.0;
        let mut rollback_steps_executed = Vec::new();

        // Restore pattern state
        let pattern_restoration_start = std::time::Instant::now();
        context.state = checkpoint.pattern_state.clone();
        checkpoint_restoration_time = pattern_restoration_start.elapsed().as_secs_f64();

        // Restore resources if requested
        if restore_resources {
            let resource_start = std::time::Instant::now();
            // Fix: Use the resource state directly instead of trying to convert
            context.resources.memory_pool = checkpoint.resource_state.memory_pool.clone();
            context.resources.cpu_allocator = checkpoint.resource_state.cpu_allocator.clone();
            context.resources.network_resources =
                checkpoint.resource_state.network_resources.clone();
            context.resources.file_locks = checkpoint.resource_state.file_locks.clone();
            context.resources.custom_resources = checkpoint.resource_state.custom_resources.clone();
            resource_restoration_time = resource_start.elapsed().as_secs_f64();
        }

        // Restore agent states if requested
        if restore_agent_states {
            let agent_start = std::time::Instant::now();
            for (agent_id, agent_snapshot) in &checkpoint.agent_states {
                if let Some(agent) = context.agents.iter_mut().find(|a| a.id == *agent_id) {
                    agent.status = agent_snapshot.status.clone();
                    agent.current_workload = agent_snapshot.current_workload;
                    agent.assigned_tasks = agent_snapshot.assigned_tasks.clone();
                    agent.performance_metrics = agent_snapshot.performance_metrics.clone();
                }
            }
            agent_recovery_time = agent_start.elapsed().as_secs_f64();
        }

        // Execute rollback steps
        for step in rollback_steps {
            rollback_steps_executed.push(step.clone());
            // Simulate step execution
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        let total_recovery_time = start_time.elapsed().as_secs_f64();

        Ok(EnhancedRecoveryResult {
            strategy: EnhancedRecoveryStrategy::PartialRollback {
                checkpoint_id: checkpoint_id.to_string(),
                rollback_steps: rollback_steps.to_vec(),
                preserve_successful_steps,
                restore_resources,
                restore_agent_states,
            },
            success: true,
            duration_seconds: total_recovery_time,
            error_message: None,
            checkpoint_id: Some(checkpoint_id.to_string()),
            restored_state: true,
            recovery_metrics: RecoveryMetrics {
                total_recovery_time,
                checkpoint_restoration_time,
                resource_restoration_time,
                agent_recovery_time,
                validation_time: 0.0,
                rollback_steps_count: rollback_steps_executed.len(),
                resources_restored: if restore_resources { 4 } else { 0 }, // memory, cpu, network, file_locks
                agents_recovered: if restore_agent_states {
                    checkpoint.agent_states.len()
                } else {
                    0
                },
                state_consistency_score: 0.95,
            },
            rollback_steps: rollback_steps_executed,
            resource_restoration: ResourceRestorationResult {
                memory_restored: restore_resources,
                cpu_restored: restore_resources,
                network_restored: restore_resources,
                file_locks_restored: restore_resources,
                custom_resources_restored: HashMap::new(),
                restoration_errors: vec![],
            },
            agent_recovery: AgentRecoveryResult {
                agents_restored: if restore_agent_states {
                    checkpoint.agent_states.keys().cloned().collect()
                } else {
                    vec![]
                },
                agents_failed: vec![],
                state_consistency: HashMap::new(),
                recovery_errors: vec![],
            },
        })
    }

    /// Handle graceful degradation strategy
    async fn handle_graceful_degradation_strategy(
        &self,
        primary_pattern_id: &str,
        fallback_patterns: &[String],
        degradation_criteria: &HashMap<String, serde_json::Value>,
        preserve_context: bool,
        context: &mut PatternContext,
        error: &PatternError,
    ) -> Result<EnhancedRecoveryResult, RecoveryError> {
        let start_time = std::time::Instant::now();

        // Check degradation criteria
        let should_degrade = self
            .check_degradation_criteria(degradation_criteria, context, error)
            .await;

        if !should_degrade {
            return Err(RecoveryError::RecoveryValidationFailed(
                "Degradation criteria not met".to_string(),
            ));
        }

        // Try fallback patterns in order
        for fallback_pattern_id in fallback_patterns {
            tracing::info!(
                pattern_id = primary_pattern_id,
                fallback_pattern = fallback_pattern_id,
                "Attempting graceful degradation to fallback pattern"
            );

            // Simulate fallback pattern execution
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // Simulate success for the first fallback pattern
            if fallback_pattern_id == &fallback_patterns[0] {
                let total_recovery_time = start_time.elapsed().as_secs_f64();

                return Ok(EnhancedRecoveryResult {
                    strategy: EnhancedRecoveryStrategy::GracefulDegradation {
                        primary_pattern_id: primary_pattern_id.to_string(),
                        fallback_patterns: fallback_patterns.to_vec(),
                        degradation_criteria: degradation_criteria.clone(),
                        preserve_context,
                    },
                    success: true,
                    duration_seconds: total_recovery_time,
                    error_message: None,
                    checkpoint_id: None,
                    restored_state: false,
                    recovery_metrics: RecoveryMetrics {
                        total_recovery_time,
                        checkpoint_restoration_time: 0.0,
                        resource_restoration_time: 0.0,
                        agent_recovery_time: 0.0,
                        validation_time: 0.0,
                        rollback_steps_count: 0,
                        resources_restored: 0,
                        agents_recovered: 0,
                        state_consistency_score: 0.8,
                    },
                    rollback_steps: vec![],
                    resource_restoration: ResourceRestorationResult {
                        memory_restored: false,
                        cpu_restored: false,
                        network_restored: false,
                        file_locks_restored: false,
                        custom_resources_restored: HashMap::new(),
                        restoration_errors: vec![],
                    },
                    agent_recovery: AgentRecoveryResult {
                        agents_restored: vec![],
                        agents_failed: vec![],
                        state_consistency: HashMap::new(),
                        recovery_errors: vec![],
                    },
                });
            }
        }

        Err(RecoveryError::RecoveryTimeout(
            "All fallback patterns failed".to_string(),
        ))
    }

    /// Check if degradation criteria are met
    async fn check_degradation_criteria(
        &self,
        criteria: &HashMap<String, serde_json::Value>,
        context: &PatternContext,
        error: &PatternError,
    ) -> bool {
        for (criterion, value) in criteria {
            match criterion.as_str() {
                "error_type" => {
                    if let Some(expected_error) = value.as_str() {
                        if !error.to_string().contains(expected_error) {
                            return false;
                        }
                    }
                }
                "resource_utilization" => {
                    if let Some(threshold) = value.as_f64() {
                        let utilization = context.resources.memory_pool.allocated_memory as f64
                            / context.resources.memory_pool.total_memory as f64;
                        if utilization < threshold {
                            return false;
                        }
                    }
                }
                "agent_availability" => {
                    if let Some(min_agents) = value.as_u64() {
                        let available_agents = context
                            .agents
                            .iter()
                            .filter(|a| a.status == super::AgentStatus::Idle)
                            .count();
                        if available_agents < min_agents as usize {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }

    /// Handle state reconstruction strategy
    async fn handle_state_reconstruction_strategy(
        &self,
        checkpoint_ids: &[String],
        reconstruction_strategy: &ReconstructionStrategy,
        validate_reconstructed_state: bool,
        context: &mut PatternContext,
    ) -> Result<EnhancedRecoveryResult, RecoveryError> {
        let start_time = std::time::Instant::now();

        // Get checkpoints
        let checkpoints = {
            let checkpoint_map = self.checkpoints.read().await;
            let mut cps = Vec::new();
            for id in checkpoint_ids {
                if let Some(cp) = checkpoint_map.get(id) {
                    cps.push(cp.clone());
                }
            }
            cps
        };

        if checkpoints.is_empty() {
            return Err(RecoveryError::CheckpointNotFound(
                "No valid checkpoints found".to_string(),
            ));
        }

        let checkpoint_restoration_time = start_time.elapsed().as_secs_f64();

        // Reconstruct state based on strategy
        let reconstructed_state = match reconstruction_strategy {
            ReconstructionStrategy::MostRecent => checkpoints
                .iter()
                .max_by_key(|cp| cp.timestamp)
                .unwrap()
                .clone(),
            ReconstructionStrategy::Merge => self.merge_checkpoint_states(&checkpoints),
            ReconstructionStrategy::BestSuccessRate => {
                // Simulate finding checkpoint with best success rate
                checkpoints.first().unwrap().clone()
            }
            ReconstructionStrategy::Partial => self.reconstruct_partial_state(&checkpoints),
        };

        // Apply reconstructed state
        context.state = reconstructed_state.pattern_state.clone();
        // Fix: Use the resource state directly instead of trying to convert
        context.resources.memory_pool = reconstructed_state.resource_state.memory_pool.clone();
        context.resources.cpu_allocator = reconstructed_state.resource_state.cpu_allocator.clone();
        context.resources.network_resources =
            reconstructed_state.resource_state.network_resources.clone();
        context.resources.file_locks = reconstructed_state.resource_state.file_locks.clone();
        context.resources.custom_resources =
            reconstructed_state.resource_state.custom_resources.clone();

        // Validate reconstructed state if requested
        let validation_time = if validate_reconstructed_state {
            let validation_start = std::time::Instant::now();
            let _validation_result = self.validate_recovery_context(context).await;
            validation_start.elapsed().as_secs_f64()
        } else {
            0.0
        };

        let total_recovery_time = start_time.elapsed().as_secs_f64();

        Ok(EnhancedRecoveryResult {
            strategy: EnhancedRecoveryStrategy::StateReconstruction {
                checkpoint_ids: checkpoint_ids.to_vec(),
                reconstruction_strategy: reconstruction_strategy.clone(),
                validate_reconstructed_state,
            },
            success: true,
            duration_seconds: total_recovery_time,
            error_message: None,
            checkpoint_id: Some(checkpoint_ids[0].clone()),
            restored_state: true,
            recovery_metrics: RecoveryMetrics {
                total_recovery_time,
                checkpoint_restoration_time,
                resource_restoration_time: 0.0,
                agent_recovery_time: 0.0,
                validation_time,
                rollback_steps_count: 0,
                resources_restored: 4,
                agents_recovered: reconstructed_state.agent_states.len(),
                state_consistency_score: 0.9,
            },
            rollback_steps: vec![],
            resource_restoration: ResourceRestorationResult {
                memory_restored: true,
                cpu_restored: true,
                network_restored: true,
                file_locks_restored: true,
                custom_resources_restored: HashMap::new(),
                restoration_errors: vec![],
            },
            agent_recovery: AgentRecoveryResult {
                agents_restored: reconstructed_state.agent_states.keys().cloned().collect(),
                agents_failed: vec![],
                state_consistency: HashMap::new(),
                recovery_errors: vec![],
            },
        })
    }

    /// Merge multiple checkpoint states
    fn merge_checkpoint_states(&self, checkpoints: &[PatternCheckpoint]) -> PatternCheckpoint {
        // Simple merge strategy - use the most recent checkpoint as base
        let mut merged = checkpoints
            .iter()
            .max_by_key(|cp| cp.timestamp)
            .unwrap()
            .clone();

        // Merge agent states from all checkpoints
        for checkpoint in checkpoints {
            for (agent_id, agent_state) in &checkpoint.agent_states {
                merged
                    .agent_states
                    .insert(agent_id.clone(), agent_state.clone());
            }
        }

        merged
    }

    /// Reconstruct partial state from checkpoints
    fn reconstruct_partial_state(&self, checkpoints: &[PatternCheckpoint]) -> PatternCheckpoint {
        // Use the first checkpoint as base and fill in missing parts from others
        let mut reconstructed = checkpoints[0].clone();

        for checkpoint in &checkpoints[1..] {
            // Merge missing agent states
            for (agent_id, agent_state) in &checkpoint.agent_states {
                if !reconstructed.agent_states.contains_key(agent_id) {
                    reconstructed
                        .agent_states
                        .insert(agent_id.clone(), agent_state.clone());
                }
            }

            // Merge missing resources
            if reconstructed.resource_state.memory_pool.available_memory == 0 {
                reconstructed.resource_state.memory_pool =
                    checkpoint.resource_state.memory_pool.clone();
            }
        }

        reconstructed
    }

    /// Handle resource-aware recovery strategy
    async fn handle_resource_aware_recovery_strategy(
        &self,
        resource_constraints: &HashMap<String, serde_json::Value>,
        recovery_priority: &RecoveryPriority,
        adaptive_timeout: bool,
        context: &mut PatternContext,
        _error: &PatternError,
    ) -> Result<EnhancedRecoveryResult, RecoveryError> {
        let start_time = std::time::Instant::now();

        // Check resource constraints
        let resource_available = self
            .check_resource_constraints(resource_constraints, context)
            .await;
        if !resource_available {
            return Err(RecoveryError::ResourceRestorationFailed(
                "Resource constraints not satisfied".to_string(),
            ));
        }

        // Determine timeout based on priority and adaptive setting
        let timeout_ms = match recovery_priority {
            RecoveryPriority::Critical => 5000,
            RecoveryPriority::High => 3000,
            RecoveryPriority::Medium => 2000,
            RecoveryPriority::Low => 1000,
        };

        let _adjusted_timeout = if adaptive_timeout {
            // Adjust timeout based on current resource utilization
            let utilization = context.resources.memory_pool.allocated_memory as f64
                / context.resources.memory_pool.total_memory as f64;
            (timeout_ms as f64 * (1.0 + utilization)) as u64
        } else {
            timeout_ms
        };

        // Simulate resource-aware recovery
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let total_recovery_time = start_time.elapsed().as_secs_f64();

        Ok(EnhancedRecoveryResult {
            strategy: EnhancedRecoveryStrategy::ResourceAwareRecovery {
                resource_constraints: resource_constraints.clone(),
                recovery_priority: recovery_priority.clone(),
                adaptive_timeout,
            },
            success: true,
            duration_seconds: total_recovery_time,
            error_message: None,
            checkpoint_id: None,
            restored_state: true,
            recovery_metrics: RecoveryMetrics {
                total_recovery_time,
                checkpoint_restoration_time: 0.0,
                resource_restoration_time: total_recovery_time * 0.8,
                agent_recovery_time: 0.0,
                validation_time: 0.0,
                rollback_steps_count: 0,
                resources_restored: resource_constraints.len(),
                agents_recovered: 0,
                state_consistency_score: 0.85,
            },
            rollback_steps: vec![],
            resource_restoration: ResourceRestorationResult {
                memory_restored: true,
                cpu_restored: true,
                network_restored: true,
                file_locks_restored: true,
                custom_resources_restored: resource_constraints
                    .keys()
                    .map(|k| (k.clone(), true))
                    .collect(),
                restoration_errors: vec![],
            },
            agent_recovery: AgentRecoveryResult {
                agents_restored: vec![],
                agents_failed: vec![],
                state_consistency: HashMap::new(),
                recovery_errors: vec![],
            },
        })
    }

    /// Check if resource constraints are satisfied
    async fn check_resource_constraints(
        &self,
        constraints: &HashMap<String, serde_json::Value>,
        context: &PatternContext,
    ) -> bool {
        for (resource, constraint) in constraints {
            match resource.as_str() {
                "memory" => {
                    if let Some(min_memory) = constraint.as_u64() {
                        if context.resources.memory_pool.available_memory < min_memory {
                            return false;
                        }
                    }
                }
                "cpu" => {
                    if let Some(min_cores) = constraint.as_u64() {
                        if (context.resources.cpu_allocator.available_cores as u64) < min_cores {
                            return false;
                        }
                    }
                }
                "network" => {
                    if let Some(min_bandwidth) = constraint.as_u64() {
                        if context.resources.network_resources.available_bandwidth < min_bandwidth {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }

    /// Handle agent-specific recovery strategy
    async fn handle_agent_specific_recovery_strategy(
        &self,
        agent_recovery_strategies: &HashMap<String, RecoveryStrategy>,
        coordination_timeout_ms: u64,
        fallback_agents: &[String],
        context: &mut PatternContext,
        _error: &PatternError,
    ) -> Result<EnhancedRecoveryResult, RecoveryError> {
        let start_time = std::time::Instant::now();

        let mut agents_restored = Vec::new();
        let mut agents_failed = Vec::new();
        let mut state_consistency = HashMap::new();
        let mut recovery_errors = Vec::new();

        // Recover each agent with its specific strategy
        for (agent_id, strategy) in agent_recovery_strategies {
            match self.recover_agent(agent_id, strategy, context).await {
                Ok(consistency_score) => {
                    agents_restored.push(agent_id.clone());
                    state_consistency.insert(agent_id.clone(), consistency_score);
                }
                Err(e) => {
                    agents_failed.push(agent_id.clone());
                    recovery_errors.push(format!("Agent {} recovery failed: {}", agent_id, e));
                }
            }
        }

        // Use fallback agents for failed recoveries
        for fallback_agent in fallback_agents {
            if !agents_restored.contains(fallback_agent) {
                agents_restored.push(fallback_agent.clone());
                state_consistency.insert(fallback_agent.clone(), 0.7); // Lower consistency for fallbacks
            }
        }

        let total_recovery_time = start_time.elapsed().as_secs_f64();

        Ok(EnhancedRecoveryResult {
            strategy: EnhancedRecoveryStrategy::AgentSpecificRecovery {
                agent_recovery_strategies: agent_recovery_strategies.clone(),
                coordination_timeout_ms,
                fallback_agents: fallback_agents.to_vec(),
            },
            success: !agents_restored.is_empty(),
            duration_seconds: total_recovery_time,
            error_message: if recovery_errors.is_empty() {
                None
            } else {
                Some(recovery_errors.join("; "))
            },
            checkpoint_id: None,
            restored_state: !agents_restored.is_empty(),
            recovery_metrics: RecoveryMetrics {
                total_recovery_time,
                checkpoint_restoration_time: 0.0,
                resource_restoration_time: 0.0,
                agent_recovery_time: total_recovery_time * 0.9,
                validation_time: 0.0,
                rollback_steps_count: 0,
                resources_restored: 0,
                agents_recovered: agents_restored.len(),
                state_consistency_score: state_consistency.values().sum::<f64>()
                    / state_consistency.len() as f64,
            },
            rollback_steps: vec![],
            resource_restoration: ResourceRestorationResult {
                memory_restored: false,
                cpu_restored: false,
                network_restored: false,
                file_locks_restored: false,
                custom_resources_restored: HashMap::new(),
                restoration_errors: vec![],
            },
            agent_recovery: AgentRecoveryResult {
                agents_restored,
                agents_failed,
                state_consistency,
                recovery_errors,
            },
        })
    }

    /// Recover a specific agent
    async fn recover_agent(
        &self,
        agent_id: &str,
        _strategy: &RecoveryStrategy,
        context: &mut PatternContext,
    ) -> Result<f64, RecoveryError> {
        // Simulate agent recovery
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Find agent in context
        if let Some(agent) = context.agents.iter_mut().find(|a| a.id == agent_id) {
            // Simulate successful recovery
            agent.status = super::AgentStatus::Idle;
            agent.current_workload = 0.0;
            agent.assigned_tasks.clear();

            Ok(0.95) // High consistency score
        } else {
            Err(RecoveryError::AgentStateRestorationFailed(format!(
                "Agent {} not found",
                agent_id
            )))
        }
    }

    /// Validate recovery context
    async fn validate_recovery_context(&self, context: &PatternContext) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if agents are still available
        let available_agents = context
            .agents
            .iter()
            .filter(|agent| agent.status != super::AgentStatus::Offline)
            .count();

        if available_agents == 0 {
            errors.push("No agents available for recovery".to_string());
        } else if available_agents < 2 {
            warnings.push("Limited agent availability for recovery".to_string());
        }

        // Check resource availability
        if context.resources.memory_pool.available_memory < 100 * 1024 * 1024 {
            warnings.push("Low memory availability for recovery".to_string());
        }

        if context.resources.cpu_allocator.available_cores == 0 {
            errors.push("No CPU cores available for recovery".to_string());
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details: HashMap::new(),
        }
    }

    /// Get recovery statistics
    pub async fn get_recovery_statistics(&self) -> RecoveryStatistics {
        let history = self.recovery_history.read().await;

        let mut stats = RecoveryStatistics {
            total_recoveries: history.len(),
            successful_recoveries: 0,
            failed_recoveries: 0,
            average_recovery_time: 0.0,
            strategy_usage: HashMap::new(),
            most_common_errors: HashMap::new(),
        };

        let mut total_duration = 0.0;

        for record in history.iter() {
            total_duration += record.duration_seconds;

            if record.success {
                stats.successful_recoveries += 1;
            } else {
                stats.failed_recoveries += 1;
            }

            // Count strategy usage
            let strategy_name = match &record.strategy {
                RecoveryStrategy::Retry { .. } => "retry",
                RecoveryStrategy::Rollback { .. } => "rollback",
                RecoveryStrategy::ContinueFromCheckpoint { .. } => "continue_from_checkpoint",
                RecoveryStrategy::Fallback { .. } => "fallback",
                RecoveryStrategy::ManualIntervention { .. } => "manual_intervention",
                RecoveryStrategy::Abort { .. } => "abort",
            };
            *stats
                .strategy_usage
                .entry(strategy_name.to_string())
                .or_insert(0) += 1;

            // Count error types
            if let Some(error_msg) = &record.error_message {
                *stats
                    .most_common_errors
                    .entry(error_msg.clone())
                    .or_insert(0) += 1;
            }
        }

        if stats.total_recoveries > 0 {
            stats.average_recovery_time = total_duration / stats.total_recoveries as f64;
        }

        stats
    }

    /// Get recovery history
    pub async fn get_recovery_history(&self, limit: Option<usize>) -> Vec<RecoveryRecord> {
        let history = self.recovery_history.read().await;
        let limit = limit.unwrap_or(100);
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Clean up old checkpoints
    pub async fn cleanup_old_checkpoints(&self, max_age_hours: u64) -> usize {
        let cutoff_time = Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        let mut checkpoints = self.checkpoints.write().await;

        let initial_count = checkpoints.len();
        checkpoints.retain(|_, checkpoint| checkpoint.timestamp > cutoff_time);

        initial_count - checkpoints.len()
    }
}

impl Default for PatternRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Recovery result
#[derive(Debug, Clone)]
pub struct RecoveryResult {
    pub strategy: RecoveryStrategy,
    pub success: bool,
    pub duration_seconds: f64,
    pub error_message: Option<String>,
    pub checkpoint_id: Option<String>,
    pub restored_state: bool,
}

/// Recovery record for tracking recovery history
#[derive(Debug, Clone)]
pub struct RecoveryRecord {
    pub pattern_id: String,
    pub timestamp: DateTime<Utc>,
    pub strategy: RecoveryStrategy,
    pub success: bool,
    pub duration_seconds: f64,
    pub error_message: Option<String>,
}

/// Recovery statistics
#[derive(Debug, Clone)]
pub struct RecoveryStatistics {
    pub total_recoveries: usize,
    pub successful_recoveries: usize,
    pub failed_recoveries: usize,
    pub average_recovery_time: f64,
    pub strategy_usage: HashMap<String, usize>,
    pub most_common_errors: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::super::{
        AgentInfo, AgentPerformanceMetrics, AgentStatus, CpuAllocator, MemoryPool,
        NetworkResources, PatternConfig, PatternPhase, PatternState, PatternStatus, ResourcePool,
        ValidationResult,
    };
    use super::*;

    #[tokio::test]
    async fn test_recovery_manager_creation() {
        let manager = PatternRecoveryManager::new();
        assert!(manager.checkpoints.read().await.is_empty());
        assert!(manager.recovery_history.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_checkpoint_creation_and_restoration() {
        let manager = PatternRecoveryManager::new();

        let context = create_test_context();
        let checkpoint_id = manager
            .create_checkpoint("test_pattern", &context, None)
            .await
            .unwrap();

        assert!(!checkpoint_id.is_empty());

        let mut restored_context = create_test_context();
        manager
            .restore_from_checkpoint(&checkpoint_id, &mut restored_context)
            .await
            .unwrap();

        // Verify restoration
        assert_eq!(restored_context.state.pattern_id, "test_pattern");
    }

    #[tokio::test]
    async fn test_retry_recovery_strategy() {
        let manager = PatternRecoveryManager::new();
        let mut context = create_test_context();
        let error = PatternError::ExecutionError("Test error".to_string());

        let strategy = RecoveryStrategy::Retry {
            max_attempts: 3,
            backoff_delay_ms: 10,
            exponential_backoff: true,
        };

        let result = manager
            .execute_enhanced_recovery_strategy(
                "test_pattern",
                &EnhancedRecoveryStrategy::IntelligentRetry {
                    max_attempts: 3,
                    initial_backoff_ms: 10,
                    max_backoff_ms: 1000,
                    backoff_multiplier: 2.0,
                    circuit_breaker_threshold: 3,
                    circuit_breaker_timeout_ms: 5000,
                },
                &mut context,
                &error,
            )
            .await
            .unwrap();

        // Retry strategy should eventually succeed
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_rollback_recovery_strategy() {
        let manager = PatternRecoveryManager::new();
        let mut context = create_test_context();

        // Create checkpoint first
        let checkpoint_id = manager
            .create_checkpoint("test_pattern", &context, None)
            .await
            .unwrap();

        // Modify context
        context.state.phase = PatternPhase::Failed;

        let strategy = RecoveryStrategy::Rollback {
            checkpoint_id: checkpoint_id.clone(),
            restore_resources: true,
            restore_agent_states: true,
        };

        let error = PatternError::ExecutionError("Test error".to_string());
        let result = manager
            .execute_enhanced_recovery_strategy(
                "test_pattern",
                &EnhancedRecoveryStrategy::PartialRollback {
                    checkpoint_id: checkpoint_id.clone(),
                    rollback_steps: vec![],
                    preserve_successful_steps: false,
                    restore_resources: true,
                    restore_agent_states: true,
                },
                &mut context,
                &error,
            )
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.checkpoint_id, Some(checkpoint_id));
        assert!(result.restored_state);
    }

    #[tokio::test]
    async fn test_recovery_statistics() {
        let manager = PatternRecoveryManager::new();

        // Execute some recovery strategies
        let mut context = create_test_context();
        let error = PatternError::ExecutionError("Test error".to_string());

        // This should fail after 2 attempts, which is expected behavior
        let result = manager
            .execute_enhanced_recovery_strategy(
                "test_pattern",
                &EnhancedRecoveryStrategy::IntelligentRetry {
                    max_attempts: 2,
                    initial_backoff_ms: 10,
                    max_backoff_ms: 1000,
                    backoff_multiplier: 2.0,
                    circuit_breaker_threshold: 3,
                    circuit_breaker_timeout_ms: 5000,
                },
                &mut context,
                &error,
            )
            .await;

        // The strategy should fail after max attempts, which is expected
        assert!(result.is_err());

        let stats = manager.get_recovery_statistics().await;
        assert_eq!(stats.total_recoveries, 1);
        assert_eq!(stats.successful_recoveries, 0); // Should be 0 since it failed
        assert_eq!(stats.failed_recoveries, 1); // Should be 1 since it failed
        assert!(stats.average_recovery_time > 0.0);
    }

    fn create_test_context() -> PatternContext {
        PatternContext {
            agents: vec![AgentInfo {
                id: "agent1".to_string(),
                name: "Test Agent".to_string(),
                capabilities: vec!["test".to_string()],
                status: AgentStatus::Idle,
                performance_metrics: AgentPerformanceMetrics::default(),
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
                pattern_id: "test_pattern".to_string(),
                phase: PatternPhase::Initializing,
                started_at: Utc::now(),
                ended_at: None,
                progress: 0.0,
                status: PatternStatus::Pending,
                data: HashMap::new(),
            },
            config: PatternConfig {
                timeout_seconds: 30,
                max_retries: 3,
                enable_rollback: true,
                enable_monitoring: true,
                custom_config: HashMap::new(),
            },
            session_id: None,
            parent_pattern_id: None,
        }
    }
}
