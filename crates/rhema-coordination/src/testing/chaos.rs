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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Chaos testing system for production coordination
pub struct ChaosTestingSystem {
    /// Chaos testing configuration
    config: ChaosTestingConfig,
    /// Active chaos experiments
    active_experiments: Arc<RwLock<HashMap<String, ChaosExperiment>>>,
    /// Experiment history
    experiment_history: Arc<RwLock<Vec<ChaosExperiment>>>,
    /// Chaos monkeys
    chaos_monkeys: Vec<Box<dyn ChaosMonkey + Send + Sync>>,
    /// Chaos statistics
    statistics: Arc<RwLock<ChaosStatistics>>,
}

/// Chaos testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosTestingConfig {
    /// Enable chaos testing
    pub enabled: bool,
    /// Chaos testing mode
    pub mode: ChaosMode,
    /// Experiment duration (seconds)
    pub experiment_duration_seconds: u64,
    /// Recovery timeout (seconds)
    pub recovery_timeout_seconds: u64,
    /// Maximum concurrent experiments
    pub max_concurrent_experiments: usize,
    /// Enable automatic recovery
    pub auto_recovery_enabled: bool,
    /// Enable experiment scheduling
    pub scheduling_enabled: bool,
    /// Chaos testing schedule
    pub schedule: ChaosSchedule,
}

/// Chaos testing modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChaosMode {
    /// Manual mode - experiments must be triggered manually
    Manual,
    /// Scheduled mode - experiments run on schedule
    Scheduled,
    /// Continuous mode - experiments run continuously
    Continuous,
    /// Adaptive mode - experiments adapt based on system health
    Adaptive,
}

/// Chaos testing schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosSchedule {
    /// Schedule enabled
    pub enabled: bool,
    /// Schedule interval (minutes)
    pub interval_minutes: u64,
    /// Schedule start time
    pub start_time: Option<String>,
    /// Schedule end time
    pub end_time: Option<String>,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u8>,
    /// Experiment types to run
    pub experiment_types: Vec<ChaosExperimentType>,
}

/// Chaos experiment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChaosExperimentType {
    /// Network latency injection
    NetworkLatency { latency_ms: u64, jitter_ms: u64 },
    /// Network packet loss
    NetworkPacketLoss { loss_percentage: f64 },
    /// Network partition
    NetworkPartition { partition_duration_seconds: u64 },
    /// CPU stress
    CpuStress {
        cpu_percentage: f64,
        duration_seconds: u64,
    },
    /// Memory stress
    MemoryStress {
        memory_percentage: f64,
        duration_seconds: u64,
    },
    /// Disk I/O stress
    DiskIoStress {
        io_percentage: f64,
        duration_seconds: u64,
    },
    /// Process kill
    ProcessKill { process_name: String },
    /// Service restart
    ServiceRestart { service_name: String },
    /// Random failures
    RandomFailures { failure_rate: f64 },
    /// Time skew
    TimeSkew { skew_seconds: i64 },
    /// Custom experiment
    Custom {
        name: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Chaos experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosExperiment {
    /// Unique experiment ID
    pub id: String,
    /// Experiment name
    pub name: String,
    /// Experiment description
    pub description: String,
    /// Experiment type
    pub experiment_type: ChaosExperimentType,
    /// Experiment status
    pub status: ChaosExperimentStatus,
    /// Experiment start time
    pub start_time: DateTime<Utc>,
    /// Experiment end time
    pub end_time: Option<DateTime<Utc>>,
    /// Experiment duration (seconds)
    pub duration_seconds: Option<u64>,
    /// Target components
    pub target_components: Vec<String>,
    /// Experiment parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Experiment results
    pub results: Option<ChaosExperimentResults>,
    /// Recovery status
    pub recovery_status: RecoveryStatus,
    /// Experiment metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Chaos experiment status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChaosExperimentStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Recovered,
}

/// Recovery status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecoveryStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    NotRequired,
}

/// Chaos experiment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosExperimentResults {
    /// System health before experiment
    pub health_before: SystemHealth,
    /// System health during experiment
    pub health_during: SystemHealth,
    /// System health after experiment
    pub health_after: SystemHealth,
    /// Recovery time (seconds)
    pub recovery_time_seconds: Option<u64>,
    /// Impact assessment
    pub impact: ImpactAssessment,
    /// Lessons learned
    pub lessons_learned: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// System health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Network latency (ms)
    pub network_latency: f64,
    /// Error rate
    pub error_rate: f64,
    /// Response time (ms)
    pub response_time: f64,
    /// Throughput (requests/second)
    pub throughput: f64,
    /// Availability percentage
    pub availability: f64,
}

/// Impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// Impact severity
    pub severity: ImpactSeverity,
    /// Impact description
    pub description: String,
    /// Affected components
    pub affected_components: Vec<String>,
    /// User impact
    pub user_impact: String,
    /// Business impact
    pub business_impact: String,
    /// Mitigation strategies
    pub mitigation_strategies: Vec<String>,
}

/// Impact severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactSeverity {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Chaos statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosStatistics {
    /// Total experiments run
    pub total_experiments: u64,
    /// Successful experiments
    pub successful_experiments: u64,
    /// Failed experiments
    pub failed_experiments: u64,
    /// Experiments by type
    pub experiments_by_type: HashMap<String, u64>,
    /// Average recovery time (seconds)
    pub avg_recovery_time_seconds: f64,
    /// System resilience score
    pub resilience_score: f64,
    /// Last experiment timestamp
    pub last_experiment_timestamp: Option<DateTime<Utc>>,
}

/// Chaos monkey trait
pub trait ChaosMonkey: Send + Sync {
    /// Execute chaos experiment
    fn execute_experiment(&self, experiment: &ChaosExperiment) -> Result<(), String>;
    /// Recover from chaos experiment
    fn recover_experiment(&self, experiment: &ChaosExperiment) -> Result<(), String>;
    /// Get monkey name
    fn name(&self) -> &str;
    /// Check if monkey is enabled
    fn is_enabled(&self) -> bool;
}

/// Network chaos monkey
pub struct NetworkChaosMonkey;

impl ChaosMonkey for NetworkChaosMonkey {
    fn execute_experiment(&self, experiment: &ChaosExperiment) -> Result<(), String> {
        match &experiment.experiment_type {
            ChaosExperimentType::NetworkLatency {
                latency_ms,
                jitter_ms,
            } => {
                info!(
                    "[NETWORK_CHAOS] Injecting {}ms latency with {}ms jitter",
                    latency_ms, jitter_ms
                );
                // In a real implementation, this would inject network latency
                Ok(())
            }
            ChaosExperimentType::NetworkPacketLoss { loss_percentage } => {
                info!("[NETWORK_CHAOS] Injecting {}% packet loss", loss_percentage);
                // In a real implementation, this would inject packet loss
                Ok(())
            }
            ChaosExperimentType::NetworkPartition {
                partition_duration_seconds,
            } => {
                info!(
                    "[NETWORK_CHAOS] Creating network partition for {} seconds",
                    partition_duration_seconds
                );
                // In a real implementation, this would create network partitions
                Ok(())
            }
            _ => Err("Unsupported experiment type for network chaos monkey".to_string()),
        }
    }

    fn recover_experiment(&self, _experiment: &ChaosExperiment) -> Result<(), String> {
        info!("[NETWORK_CHAOS] Recovering network conditions");
        // In a real implementation, this would restore network conditions
        Ok(())
    }

    fn name(&self) -> &str {
        "network"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

/// System chaos monkey
pub struct SystemChaosMonkey;

impl ChaosMonkey for SystemChaosMonkey {
    fn execute_experiment(&self, experiment: &ChaosExperiment) -> Result<(), String> {
        match &experiment.experiment_type {
            ChaosExperimentType::CpuStress {
                cpu_percentage,
                duration_seconds,
            } => {
                info!(
                    "[SYSTEM_CHAOS] Stressing CPU to {}% for {} seconds",
                    cpu_percentage, duration_seconds
                );
                // In a real implementation, this would stress the CPU
                Ok(())
            }
            ChaosExperimentType::MemoryStress {
                memory_percentage,
                duration_seconds,
            } => {
                info!(
                    "[SYSTEM_CHAOS] Stressing memory to {}% for {} seconds",
                    memory_percentage, duration_seconds
                );
                // In a real implementation, this would stress memory
                Ok(())
            }
            ChaosExperimentType::DiskIoStress {
                io_percentage,
                duration_seconds,
            } => {
                info!(
                    "[SYSTEM_CHAOS] Stressing disk I/O to {}% for {} seconds",
                    io_percentage, duration_seconds
                );
                // In a real implementation, this would stress disk I/O
                Ok(())
            }
            _ => Err("Unsupported experiment type for system chaos monkey".to_string()),
        }
    }

    fn recover_experiment(&self, _experiment: &ChaosExperiment) -> Result<(), String> {
        info!("[SYSTEM_CHAOS] Recovering system conditions");
        // In a real implementation, this would restore system conditions
        Ok(())
    }

    fn name(&self) -> &str {
        "system"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

impl ChaosTestingSystem {
    /// Create new chaos testing system
    pub fn new(config: ChaosTestingConfig) -> Self {
        let mut chaos_monkeys: Vec<Box<dyn ChaosMonkey + Send + Sync>> = Vec::new();

        // Add default chaos monkeys
        chaos_monkeys.push(Box::new(NetworkChaosMonkey));
        chaos_monkeys.push(Box::new(SystemChaosMonkey));

        Self {
            config,
            active_experiments: Arc::new(RwLock::new(HashMap::new())),
            experiment_history: Arc::new(RwLock::new(Vec::new())),
            chaos_monkeys,
            statistics: Arc::new(RwLock::new(ChaosStatistics::default())),
        }
    }

    /// Add chaos monkey
    pub fn add_chaos_monkey(&mut self, monkey: Box<dyn ChaosMonkey + Send + Sync>) {
        self.chaos_monkeys.push(monkey);
    }

    /// Start chaos experiment
    pub async fn start_experiment(
        &self,
        name: String,
        description: String,
        experiment_type: ChaosExperimentType,
        target_components: Vec<String>,
        parameters: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, String> {
        if !self.config.enabled {
            return Err("Chaos testing is disabled".to_string());
        }

        // Check concurrent experiment limit
        let active_experiments = self.active_experiments.read().await;
        if active_experiments.len() >= self.config.max_concurrent_experiments {
            return Err("Maximum concurrent experiments reached".to_string());
        }

        let experiment_id = format!("chaos_{}", chrono::Utc::now().timestamp_millis());
        let experiment = ChaosExperiment {
            id: experiment_id.clone(),
            name,
            description,
            experiment_type,
            status: ChaosExperimentStatus::Pending,
            start_time: Utc::now(),
            end_time: None,
            duration_seconds: None,
            target_components,
            parameters: parameters.unwrap_or_default(),
            results: None,
            recovery_status: RecoveryStatus::NotStarted,
            metadata: HashMap::new(),
        };

        // Store experiment
        {
            let mut active_experiments = self.active_experiments.write().await;
            active_experiments.insert(experiment_id.clone(), experiment.clone());
        }

        // Execute experiment
        self.execute_experiment(&experiment).await?;

        info!(
            "Chaos experiment started: {} ({})",
            experiment.name, experiment_id
        );
        Ok(experiment_id)
    }

    /// Stop chaos experiment
    pub async fn stop_experiment(&self, experiment_id: &str) -> Result<(), String> {
        let mut active_experiments = self.active_experiments.write().await;

        if let Some(experiment) = active_experiments.get_mut(experiment_id) {
            experiment.status = ChaosExperimentStatus::Cancelled;
            experiment.end_time = Some(Utc::now());

            let start_time = experiment.start_time;
            experiment.duration_seconds =
                Some((experiment.end_time.unwrap() - start_time).num_seconds() as u64);

            // Recover from experiment
            self.recover_experiment(experiment).await?;

            // Move to history
            let experiment = active_experiments.remove(experiment_id).unwrap();
            let mut experiment_history = self.experiment_history.write().await;
            experiment_history.push(experiment);

            info!("Chaos experiment stopped: {}", experiment_id);
            Ok(())
        } else {
            Err(format!("Experiment not found: {}", experiment_id))
        }
    }

    /// Get active experiments
    pub async fn get_active_experiments(&self) -> Vec<ChaosExperiment> {
        let active_experiments = self.active_experiments.read().await;
        active_experiments.values().cloned().collect()
    }

    /// Get experiment by ID
    pub async fn get_experiment(&self, experiment_id: &str) -> Option<ChaosExperiment> {
        let active_experiments = self.active_experiments.read().await;
        if let Some(experiment) = active_experiments.get(experiment_id) {
            return Some(experiment.clone());
        }

        let experiment_history = self.experiment_history.read().await;
        experiment_history
            .iter()
            .find(|e| e.id == experiment_id)
            .cloned()
    }

    /// Get chaos statistics
    pub async fn get_statistics(&self) -> ChaosStatistics {
        self.statistics.read().await.clone()
    }

    /// Run scheduled experiments
    pub async fn run_scheduled_experiments(&self) -> Result<Vec<String>, String> {
        if !self.config.scheduling_enabled || !self.config.schedule.enabled {
            return Ok(Vec::new());
        }

        let mut experiment_ids = Vec::new();

        for experiment_type in &self.config.schedule.experiment_types {
            let experiment_id = self
                .start_experiment(
                    format!("Scheduled {}", experiment_type.name()),
                    format!(
                        "Automatically scheduled experiment: {}",
                        experiment_type.name()
                    ),
                    experiment_type.clone(),
                    vec!["system".to_string()],
                    None,
                )
                .await?;

            experiment_ids.push(experiment_id);
        }

        Ok(experiment_ids)
    }

    /// Execute experiment
    async fn execute_experiment(&self, experiment: &ChaosExperiment) -> Result<(), String> {
        let mut active_experiments = self.active_experiments.write().await;

        if let Some(experiment) = active_experiments.get_mut(&experiment.id) {
            experiment.status = ChaosExperimentStatus::Running;

            // Find appropriate chaos monkey
            for monkey in &self.chaos_monkeys {
                if monkey.is_enabled() {
                    if let Ok(()) = monkey.execute_experiment(experiment) {
                        info!(
                            "Chaos experiment executed by {}: {}",
                            monkey.name(),
                            experiment.id
                        );
                        return Ok(());
                    }
                }
            }

            experiment.status = ChaosExperimentStatus::Failed;
            Err("No suitable chaos monkey found for experiment".to_string())
        } else {
            Err("Experiment not found".to_string())
        }
    }

    /// Recover from experiment
    async fn recover_experiment(&self, experiment: &ChaosExperiment) -> Result<(), String> {
        let mut active_experiments = self.active_experiments.write().await;

        if let Some(experiment) = active_experiments.get_mut(&experiment.id) {
            experiment.recovery_status = RecoveryStatus::InProgress;

            // Find appropriate chaos monkey for recovery
            for monkey in &self.chaos_monkeys {
                if monkey.is_enabled() {
                    if let Ok(()) = monkey.recover_experiment(experiment) {
                        experiment.recovery_status = RecoveryStatus::Completed;
                        info!(
                            "Chaos experiment recovered by {}: {}",
                            monkey.name(),
                            experiment.id
                        );
                        return Ok(());
                    }
                }
            }

            experiment.recovery_status = RecoveryStatus::Failed;
            Err("Failed to recover from chaos experiment".to_string())
        } else {
            Err("Experiment not found".to_string())
        }
    }
}

impl ChaosExperimentType {
    /// Get experiment type name
    pub fn name(&self) -> &str {
        match self {
            ChaosExperimentType::NetworkLatency { .. } => "Network Latency",
            ChaosExperimentType::NetworkPacketLoss { .. } => "Network Packet Loss",
            ChaosExperimentType::NetworkPartition { .. } => "Network Partition",
            ChaosExperimentType::CpuStress { .. } => "CPU Stress",
            ChaosExperimentType::MemoryStress { .. } => "Memory Stress",
            ChaosExperimentType::DiskIoStress { .. } => "Disk I/O Stress",
            ChaosExperimentType::ProcessKill { .. } => "Process Kill",
            ChaosExperimentType::ServiceRestart { .. } => "Service Restart",
            ChaosExperimentType::RandomFailures { .. } => "Random Failures",
            ChaosExperimentType::TimeSkew { .. } => "Time Skew",
            ChaosExperimentType::Custom { name, .. } => name,
        }
    }
}

impl Default for ChaosTestingConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for safety
            mode: ChaosMode::Manual,
            experiment_duration_seconds: 300, // 5 minutes
            recovery_timeout_seconds: 600,    // 10 minutes
            max_concurrent_experiments: 3,
            auto_recovery_enabled: true,
            scheduling_enabled: false,
            schedule: ChaosSchedule::default(),
        }
    }
}

impl Default for ChaosSchedule {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_minutes: 60,
            start_time: None,
            end_time: None,
            days_of_week: vec![1, 2, 3, 4, 5], // Monday to Friday
            experiment_types: vec![
                ChaosExperimentType::NetworkLatency {
                    latency_ms: 100,
                    jitter_ms: 50,
                },
                ChaosExperimentType::CpuStress {
                    cpu_percentage: 50.0,
                    duration_seconds: 60,
                },
            ],
        }
    }
}

impl Default for ChaosStatistics {
    fn default() -> Self {
        Self {
            total_experiments: 0,
            successful_experiments: 0,
            failed_experiments: 0,
            experiments_by_type: HashMap::new(),
            avg_recovery_time_seconds: 0.0,
            resilience_score: 100.0,
            last_experiment_timestamp: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chaos_testing_system_creation() {
        let config = ChaosTestingConfig::default();
        let chaos_system = ChaosTestingSystem::new(config);

        assert!(!chaos_system.config.enabled); // Should be disabled by default
        assert_eq!(chaos_system.config.mode, ChaosMode::Manual);
    }

    #[tokio::test]
    async fn test_start_experiment_disabled() {
        let config = ChaosTestingConfig::default();
        let chaos_system = ChaosTestingSystem::new(config);

        let result = chaos_system
            .start_experiment(
                "Test Experiment".to_string(),
                "This is a test experiment".to_string(),
                ChaosExperimentType::NetworkLatency {
                    latency_ms: 100,
                    jitter_ms: 50,
                },
                vec!["network".to_string()],
                None,
            )
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Chaos testing is disabled");
    }

    #[tokio::test]
    async fn test_start_experiment_enabled() {
        let mut config = ChaosTestingConfig::default();
        config.enabled = true;

        let chaos_system = ChaosTestingSystem::new(config);

        let experiment_id = chaos_system
            .start_experiment(
                "Test Experiment".to_string(),
                "This is a test experiment".to_string(),
                ChaosExperimentType::NetworkLatency {
                    latency_ms: 100,
                    jitter_ms: 50,
                },
                vec!["network".to_string()],
                None,
            )
            .await
            .unwrap();

        assert!(!experiment_id.is_empty());

        let active_experiments = chaos_system.get_active_experiments().await;
        assert_eq!(active_experiments.len(), 1);
        assert_eq!(active_experiments[0].name, "Test Experiment");
    }

    #[tokio::test]
    async fn test_stop_experiment() {
        let mut config = ChaosTestingConfig::default();
        config.enabled = true;

        let chaos_system = ChaosTestingSystem::new(config);

        let experiment_id = chaos_system
            .start_experiment(
                "Test Experiment".to_string(),
                "This is a test experiment".to_string(),
                ChaosExperimentType::NetworkLatency {
                    latency_ms: 100,
                    jitter_ms: 50,
                },
                vec!["network".to_string()],
                None,
            )
            .await
            .unwrap();

        chaos_system.stop_experiment(&experiment_id).await.unwrap();

        let active_experiments = chaos_system.get_active_experiments().await;
        assert_eq!(active_experiments.len(), 0);

        let experiment = chaos_system.get_experiment(&experiment_id).await.unwrap();
        assert_eq!(experiment.status, ChaosExperimentStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_experiment_type_name() {
        let latency_type = ChaosExperimentType::NetworkLatency {
            latency_ms: 100,
            jitter_ms: 50,
        };
        assert_eq!(latency_type.name(), "Network Latency");

        let cpu_type = ChaosExperimentType::CpuStress {
            cpu_percentage: 50.0,
            duration_seconds: 60,
        };
        assert_eq!(cpu_type.name(), "CPU Stress");

        let custom_type = ChaosExperimentType::Custom {
            name: "Custom Test".to_string(),
            parameters: HashMap::new(),
        };
        assert_eq!(custom_type.name(), "Custom Test");
    }
}
