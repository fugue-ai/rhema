use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio::time::{Duration, Instant};

use super::{PatternContext, PatternError, PatternPerformanceMetrics, PatternPhase, PatternResult};

/// Monitoring event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringEvent {
    /// Pattern execution started
    PatternStarted {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        agent_count: usize,
        resource_usage: ResourceUsageSnapshot,
    },
    /// Pattern execution completed
    PatternCompleted {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        success: bool,
        duration_seconds: f64,
        performance_metrics: PatternPerformanceMetrics,
    },
    /// Pattern execution failed
    PatternFailed {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        error: String,
        duration_seconds: f64,
    },
    /// Pattern phase changed
    PhaseChanged {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        from_phase: PatternPhase,
        to_phase: PatternPhase,
        progress: f64,
    },
    /// Agent status changed
    AgentStatusChanged {
        pattern_id: String,
        agent_id: String,
        timestamp: DateTime<Utc>,
        from_status: super::AgentStatus,
        to_status: super::AgentStatus,
        workload_change: f64,
    },
    /// Resource usage changed
    ResourceUsageChanged {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        resource_type: String,
        current_usage: f64,
        max_capacity: f64,
        utilization_percentage: f64,
    },
    /// Performance metric recorded
    PerformanceMetric {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        metric_name: String,
        metric_value: f64,
        unit: String,
    },
    /// Error occurred
    Error {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        error_type: String,
        error_message: String,
        severity: ErrorSeverity,
    },
    /// Warning issued
    Warning {
        pattern_id: String,
        timestamp: DateTime<Utc>,
        warning_type: String,
        warning_message: String,
    },
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Resource usage snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageSnapshot {
    pub memory_usage_mb: u64,
    pub memory_utilization: f64,
    pub cpu_usage_cores: u32,
    pub cpu_utilization: f64,
    pub network_bandwidth_mbps: u64,
    pub network_utilization: f64,
    pub active_file_locks: usize,
    pub custom_resources: HashMap<String, f64>,
}

/// Real-time metrics for pattern execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    pub pattern_id: String,
    pub timestamp: DateTime<Utc>,
    pub execution_time_seconds: f64,
    pub progress_percentage: f64,
    pub current_phase: PatternPhase,
    pub active_agents: usize,
    pub idle_agents: usize,
    pub resource_utilization: ResourceUsageSnapshot,
    pub performance_metrics: PatternPerformanceMetrics,
    pub error_count: usize,
    pub warning_count: usize,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable real-time monitoring
    pub enable_real_time: bool,
    /// Metrics collection interval (seconds)
    pub metrics_interval_seconds: u64,
    /// Event history retention (hours)
    pub event_retention_hours: u64,
    /// Maximum events to store in memory
    pub max_events_in_memory: usize,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Enable resource monitoring
    pub enable_resource_monitoring: bool,
    /// Enable agent monitoring
    pub enable_agent_monitoring: bool,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Alert thresholds for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub max_execution_time_seconds: f64,
    pub max_memory_utilization: f64,
    pub max_cpu_utilization: f64,
    pub max_network_utilization: f64,
    pub min_agent_availability: f64,
    pub max_error_rate: f64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_real_time: true,
            metrics_interval_seconds: 5,
            event_retention_hours: 24,
            max_events_in_memory: 10000,
            enable_profiling: true,
            enable_resource_monitoring: true,
            enable_agent_monitoring: true,
            alert_thresholds: AlertThresholds {
                max_execution_time_seconds: 300.0,
                max_memory_utilization: 0.9,
                max_cpu_utilization: 0.8,
                max_network_utilization: 0.7,
                min_agent_availability: 0.5,
                max_error_rate: 0.1,
            },
        }
    }
}

/// Pattern execution monitor
pub struct PatternMonitor {
    config: MonitoringConfig,
    events: Arc<RwLock<Vec<MonitoringEvent>>>,
    metrics: Arc<RwLock<HashMap<String, RealTimeMetrics>>>,
    event_sender: broadcast::Sender<MonitoringEvent>,
    start_times: Arc<RwLock<HashMap<String, Instant>>>,
    performance_profiles: Arc<RwLock<HashMap<String, PerformanceProfile>>>,
}

impl PatternMonitor {
    pub fn new(config: MonitoringConfig) -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            config,
            events: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            start_times: Arc::new(RwLock::new(HashMap::new())),
            performance_profiles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start monitoring a pattern
    pub async fn start_monitoring(&self, pattern_id: &str, context: &PatternContext) {
        let start_time = Instant::now();

        {
            let mut start_times = self.start_times.write().await;
            start_times.insert(pattern_id.to_string(), start_time);
        }

        // Create performance profile
        {
            let mut profiles = self.performance_profiles.write().await;
            profiles.insert(pattern_id.to_string(), PerformanceProfile::new(pattern_id));
        }

        // Record start event
        let event = MonitoringEvent::PatternStarted {
            pattern_id: pattern_id.to_string(),
            timestamp: Utc::now(),
            agent_count: context.agents.len(),
            resource_usage: self.create_resource_snapshot(context),
        };

        self.record_event(event).await;

        // Start real-time monitoring if enabled
        if self.config.enable_real_time {
            self.start_real_time_monitoring(pattern_id, context).await;
        }
    }

    /// Stop monitoring a pattern
    pub async fn stop_monitoring(&self, pattern_id: &str, result: &PatternResult) {
        let execution_time = {
            let start_times = self.start_times.read().await;
            if let Some(start_time) = start_times.get(pattern_id) {
                start_time.elapsed().as_secs_f64()
            } else {
                0.0
            }
        };

        let event = if result.success {
            MonitoringEvent::PatternCompleted {
                pattern_id: pattern_id.to_string(),
                timestamp: Utc::now(),
                success: true,
                duration_seconds: execution_time,
                performance_metrics: result.performance_metrics.clone(),
            }
        } else {
            MonitoringEvent::PatternFailed {
                pattern_id: pattern_id.to_string(),
                timestamp: Utc::now(),
                error: result.error_message.clone().unwrap_or_default(),
                duration_seconds: execution_time,
            }
        };

        self.record_event(event).await;

        // Clean up monitoring data
        {
            let mut start_times = self.start_times.write().await;
            start_times.remove(pattern_id);
        }

        {
            let mut metrics = self.metrics.write().await;
            metrics.remove(pattern_id);
        }

        {
            let mut profiles = self.performance_profiles.write().await;
            profiles.remove(pattern_id);
        }
    }

    /// Record a monitoring event
    pub async fn record_event(&self, event: MonitoringEvent) {
        // Store event in memory
        {
            let mut events = self.events.write().await;
            events.push(event.clone());

            // Maintain event limit
            if events.len() > self.config.max_events_in_memory {
                events.remove(0);
            }
        }

        // Broadcast event to subscribers
        let _ = self.event_sender.send(event);
    }

    /// Update pattern phase
    pub async fn update_phase(
        &self,
        pattern_id: &str,
        from_phase: PatternPhase,
        to_phase: PatternPhase,
        progress: f64,
    ) {
        let event = MonitoringEvent::PhaseChanged {
            pattern_id: pattern_id.to_string(),
            timestamp: Utc::now(),
            from_phase,
            to_phase: to_phase.clone(),
            progress,
        };

        self.record_event(event).await;

        // Update real-time metrics
        if let Some(metrics) = self.metrics.read().await.get(pattern_id) {
            let mut updated_metrics = metrics.clone();
            updated_metrics.current_phase = to_phase;
            updated_metrics.progress_percentage = progress;
            updated_metrics.timestamp = Utc::now();

            {
                let mut metrics_map = self.metrics.write().await;
                metrics_map.insert(pattern_id.to_string(), updated_metrics);
            }
        }
    }

    /// Update agent status
    pub async fn update_agent_status(
        &self,
        pattern_id: &str,
        agent_id: &str,
        from_status: super::AgentStatus,
        to_status: super::AgentStatus,
        workload_change: f64,
    ) {
        let event = MonitoringEvent::AgentStatusChanged {
            pattern_id: pattern_id.to_string(),
            agent_id: agent_id.to_string(),
            timestamp: Utc::now(),
            from_status,
            to_status: to_status.clone(),
            workload_change,
        };

        self.record_event(event).await;
    }

    /// Update resource usage
    pub async fn update_resource_usage(&self, pattern_id: &str, context: &PatternContext) {
        let resource_snapshot = self.create_resource_snapshot(context);

        // Record memory usage
        let memory_utilization = resource_snapshot.memory_utilization;
        if memory_utilization > self.config.alert_thresholds.max_memory_utilization {
            let event = MonitoringEvent::Warning {
                pattern_id: pattern_id.to_string(),
                timestamp: Utc::now(),
                warning_type: "high_memory_usage".to_string(),
                warning_message: format!(
                    "Memory utilization at {:.1}%",
                    memory_utilization * 100.0
                ),
            };
            self.record_event(event).await;
        }

        // Record CPU usage
        let cpu_utilization = resource_snapshot.cpu_utilization;
        if cpu_utilization > self.config.alert_thresholds.max_cpu_utilization {
            let event = MonitoringEvent::Warning {
                pattern_id: pattern_id.to_string(),
                timestamp: Utc::now(),
                warning_type: "high_cpu_usage".to_string(),
                warning_message: format!("CPU utilization at {:.1}%", cpu_utilization * 100.0),
            };
            self.record_event(event).await;
        }

        // Record network usage
        let network_utilization = resource_snapshot.network_utilization;
        if network_utilization > self.config.alert_thresholds.max_network_utilization {
            let event = MonitoringEvent::Warning {
                pattern_id: pattern_id.to_string(),
                timestamp: Utc::now(),
                warning_type: "high_network_usage".to_string(),
                warning_message: format!(
                    "Network utilization at {:.1}%",
                    network_utilization * 100.0
                ),
            };
            self.record_event(event).await;
        }

        // Update real-time metrics
        if let Some(metrics) = self.metrics.read().await.get(pattern_id) {
            let mut updated_metrics = metrics.clone();
            updated_metrics.resource_utilization = resource_snapshot;
            updated_metrics.timestamp = Utc::now();

            {
                let mut metrics_map = self.metrics.write().await;
                metrics_map.insert(pattern_id.to_string(), updated_metrics);
            }
        }
    }

    /// Record performance metric
    pub async fn record_performance_metric(
        &self,
        pattern_id: &str,
        metric_name: &str,
        metric_value: f64,
        unit: &str,
    ) {
        let event = MonitoringEvent::PerformanceMetric {
            pattern_id: pattern_id.to_string(),
            timestamp: Utc::now(),
            metric_name: metric_name.to_string(),
            metric_value,
            unit: unit.to_string(),
        };

        self.record_event(event).await;

        // Update performance profile
        {
            let mut profiles = self.performance_profiles.write().await;
            if let Some(profile) = profiles.get_mut(pattern_id) {
                profile.record_metric(metric_name, metric_value, unit);
            } else {
                // Create new profile if it doesn't exist
                let mut new_profile = PerformanceProfile::new(pattern_id);
                new_profile.record_metric(metric_name, metric_value, unit);
                profiles.insert(pattern_id.to_string(), new_profile);
            }
        }
    }

    /// Record error
    pub async fn record_error(
        &self,
        pattern_id: &str,
        error: &PatternError,
        severity: ErrorSeverity,
    ) {
        let event = MonitoringEvent::Error {
            pattern_id: pattern_id.to_string(),
            timestamp: Utc::now(),
            error_type: format!("{:?}", std::mem::discriminant(error)),
            error_message: error.to_string(),
            severity,
        };

        self.record_event(event).await;

        // Update error count in metrics
        if let Some(metrics) = self.metrics.read().await.get(pattern_id) {
            let mut updated_metrics = metrics.clone();
            updated_metrics.error_count += 1;
            updated_metrics.timestamp = Utc::now();

            {
                let mut metrics_map = self.metrics.write().await;
                metrics_map.insert(pattern_id.to_string(), updated_metrics);
            }
        }
    }

    /// Get real-time metrics for a pattern
    pub async fn get_real_time_metrics(&self, pattern_id: &str) -> Option<RealTimeMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(pattern_id).cloned()
    }

    /// Get all real-time metrics
    pub async fn get_all_real_time_metrics(&self) -> HashMap<String, RealTimeMetrics> {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    /// Get monitoring events
    pub async fn get_events(
        &self,
        pattern_id: Option<&str>,
        limit: Option<usize>,
    ) -> Vec<MonitoringEvent> {
        let events = self.events.read().await;
        let limit = limit.unwrap_or(100);

        let filtered_events: Vec<MonitoringEvent> = events
            .iter()
            .filter(|event| {
                if let Some(pid) = pattern_id {
                    match event {
                        MonitoringEvent::PatternStarted {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::PatternCompleted {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::PatternFailed {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::PhaseChanged {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::AgentStatusChanged {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::ResourceUsageChanged {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::PerformanceMetric {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::Error {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                        MonitoringEvent::Warning {
                            pattern_id: pid2, ..
                        } => pid2 == pid,
                    }
                } else {
                    true
                }
            })
            .rev()
            .take(limit)
            .cloned()
            .collect();

        filtered_events
    }

    /// Get performance profile for a pattern
    pub async fn get_performance_profile(&self, pattern_id: &str) -> Option<PerformanceProfile> {
        let profiles = self.performance_profiles.read().await;
        profiles.get(pattern_id).cloned()
    }

    /// Subscribe to monitoring events
    pub fn subscribe(&self) -> broadcast::Receiver<MonitoringEvent> {
        self.event_sender.subscribe()
    }

    /// Get monitoring statistics
    pub async fn get_monitoring_statistics(&self) -> MonitoringStatistics {
        let events = self.events.read().await;
        let metrics = self.metrics.read().await;
        let profiles = self.performance_profiles.read().await;

        // Count unique patterns from events
        let mut unique_patterns = std::collections::HashSet::new();
        for event in events.iter() {
            let pattern_id = match event {
                MonitoringEvent::PatternStarted { pattern_id, .. } => pattern_id,
                MonitoringEvent::PatternCompleted { pattern_id, .. } => pattern_id,
                MonitoringEvent::PatternFailed { pattern_id, .. } => pattern_id,
                MonitoringEvent::PhaseChanged { pattern_id, .. } => pattern_id,
                MonitoringEvent::AgentStatusChanged { pattern_id, .. } => pattern_id,
                MonitoringEvent::ResourceUsageChanged { pattern_id, .. } => pattern_id,
                MonitoringEvent::PerformanceMetric { pattern_id, .. } => pattern_id,
                MonitoringEvent::Error { pattern_id, .. } => pattern_id,
                MonitoringEvent::Warning { pattern_id, .. } => pattern_id,
            };
            unique_patterns.insert(pattern_id.clone());
        }

        let mut stats = MonitoringStatistics {
            total_patterns_monitored: unique_patterns.len(),
            active_patterns: metrics.len(),
            total_events: events.len(),
            event_types: HashMap::new(),
            average_execution_time: 0.0,
            success_rate: 0.0,
            error_rate: 0.0,
        };

        let mut total_execution_time = 0.0;
        let mut successful_patterns = 0;
        let mut total_patterns = 0;
        let mut total_errors = 0;

        for event in events.iter() {
            // Count event types
            let event_type = match event {
                MonitoringEvent::PatternStarted { .. } => "pattern_started",
                MonitoringEvent::PatternCompleted { .. } => "pattern_completed",
                MonitoringEvent::PatternFailed { .. } => "pattern_failed",
                MonitoringEvent::PhaseChanged { .. } => "phase_changed",
                MonitoringEvent::AgentStatusChanged { .. } => "agent_status_changed",
                MonitoringEvent::ResourceUsageChanged { .. } => "resource_usage_changed",
                MonitoringEvent::PerformanceMetric { .. } => "performance_metric",
                MonitoringEvent::Error { .. } => "error",
                MonitoringEvent::Warning { .. } => "warning",
            };
            *stats.event_types.entry(event_type.to_string()).or_insert(0) += 1;

            // Calculate statistics
            match event {
                MonitoringEvent::PatternCompleted {
                    duration_seconds, ..
                } => {
                    total_execution_time += duration_seconds;
                    successful_patterns += 1;
                    total_patterns += 1;
                }
                MonitoringEvent::PatternFailed {
                    duration_seconds, ..
                } => {
                    total_execution_time += duration_seconds;
                    total_patterns += 1;
                }
                MonitoringEvent::Error { .. } => {
                    total_errors += 1;
                }
                _ => {}
            }
        }

        if total_patterns > 0 {
            stats.average_execution_time = total_execution_time / total_patterns as f64;
            stats.success_rate = successful_patterns as f64 / total_patterns as f64;
        }

        if stats.total_events > 0 {
            stats.error_rate = total_errors as f64 / stats.total_events as f64;
        }

        stats
    }

    /// Start real-time monitoring for a pattern
    async fn start_real_time_monitoring(&self, pattern_id: &str, context: &PatternContext) {
        let pattern_id = pattern_id.to_string();
        let monitor = self.clone();
        let config = self.config.clone();

        // Clone necessary data from context to avoid lifetime issues
        let _agent_count = context.agents.len();
        let active_agents_count = context
            .agents
            .iter()
            .filter(|a| a.status == super::AgentStatus::Working)
            .count();
        let idle_agents_count = context
            .agents
            .iter()
            .filter(|a| a.status == super::AgentStatus::Idle)
            .count();

        // Clone resource data
        let memory_pool = context.resources.memory_pool.clone();
        let cpu_allocator = context.resources.cpu_allocator.clone();
        let network_resources = context.resources.network_resources.clone();
        let custom_resources = context.resources.custom_resources.clone();
        let file_locks_len = context.resources.file_locks.len();

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(Duration::from_secs(config.metrics_interval_seconds));

            loop {
                interval.tick().await;

                // Check if pattern is still being monitored
                let start_times = monitor.start_times.read().await;
                if !start_times.contains_key(&pattern_id) {
                    break;
                }
                drop(start_times);

                // Update real-time metrics
                if let Some(execution_time) = monitor.get_execution_time(&pattern_id).await {
                    // Create resource snapshot from cloned data
                    let memory_utilization = if memory_pool.total_memory > 0 {
                        memory_pool.allocated_memory as f64 / memory_pool.total_memory as f64
                    } else {
                        0.0
                    };

                    let cpu_utilization = if cpu_allocator.total_cores > 0 {
                        cpu_allocator.allocated_cores as f64 / cpu_allocator.total_cores as f64
                    } else {
                        0.0
                    };

                    let network_utilization = {
                        let total_bandwidth = network_resources.available_bandwidth
                            + network_resources.allocated_bandwidth;
                        if total_bandwidth > 0 {
                            network_resources.allocated_bandwidth as f64 / total_bandwidth as f64
                        } else {
                            0.0
                        }
                    };

                    let mut custom_resources_map = HashMap::new();
                    for (name, resource) in &custom_resources {
                        if let Some(value) = resource.data.as_f64() {
                            custom_resources_map.insert(name.clone(), value);
                        }
                    }

                    let resource_utilization = ResourceUsageSnapshot {
                        memory_usage_mb: memory_pool.allocated_memory / (1024 * 1024),
                        memory_utilization,
                        cpu_usage_cores: cpu_allocator.allocated_cores,
                        cpu_utilization,
                        network_bandwidth_mbps: network_resources.allocated_bandwidth,
                        network_utilization,
                        active_file_locks: file_locks_len,
                        custom_resources: custom_resources_map,
                    };

                    let metrics = RealTimeMetrics {
                        pattern_id: pattern_id.clone(),
                        timestamp: Utc::now(),
                        execution_time_seconds: execution_time,
                        progress_percentage: 0.0, // Would be updated by pattern
                        current_phase: PatternPhase::Executing, // Would be updated by pattern
                        active_agents: active_agents_count,
                        idle_agents: idle_agents_count,
                        resource_utilization,
                        performance_metrics: PatternPerformanceMetrics {
                            total_execution_time_seconds: execution_time,
                            coordination_overhead_seconds: 0.0,
                            resource_utilization: 0.0,
                            agent_efficiency: 0.0,
                            communication_overhead: 0,
                        },
                        error_count: 0,
                        warning_count: 0,
                    };

                    {
                        let mut metrics_map = monitor.metrics.write().await;
                        metrics_map.insert(pattern_id.clone(), metrics);
                    }
                }
            }
        });
    }

    /// Get execution time for a pattern
    async fn get_execution_time(&self, pattern_id: &str) -> Option<f64> {
        let start_times = self.start_times.read().await;
        start_times
            .get(pattern_id)
            .map(|start_time| start_time.elapsed().as_secs_f64())
    }

    /// Create resource usage snapshot
    fn create_resource_snapshot(&self, context: &PatternContext) -> ResourceUsageSnapshot {
        let memory_utilization = if context.resources.memory_pool.total_memory > 0 {
            context.resources.memory_pool.allocated_memory as f64
                / context.resources.memory_pool.total_memory as f64
        } else {
            0.0
        };

        let cpu_utilization = if context.resources.cpu_allocator.total_cores > 0 {
            context.resources.cpu_allocator.allocated_cores as f64
                / context.resources.cpu_allocator.total_cores as f64
        } else {
            0.0
        };

        let network_utilization = {
            let total_bandwidth = context.resources.network_resources.available_bandwidth
                + context.resources.network_resources.allocated_bandwidth;
            if total_bandwidth > 0 {
                context.resources.network_resources.allocated_bandwidth as f64
                    / total_bandwidth as f64
            } else {
                0.0
            }
        };

        let mut custom_resources = HashMap::new();
        for (name, resource) in &context.resources.custom_resources {
            if let Some(value) = resource.data.as_f64() {
                custom_resources.insert(name.clone(), value);
            }
        }

        ResourceUsageSnapshot {
            memory_usage_mb: context.resources.memory_pool.allocated_memory / (1024 * 1024),
            memory_utilization,
            cpu_usage_cores: context.resources.cpu_allocator.allocated_cores,
            cpu_utilization,
            network_bandwidth_mbps: context.resources.network_resources.allocated_bandwidth,
            network_utilization,
            active_file_locks: context.resources.file_locks.len(),
            custom_resources,
        }
    }
}

impl Clone for PatternMonitor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            events: Arc::clone(&self.events),
            metrics: Arc::clone(&self.metrics),
            event_sender: self.event_sender.clone(),
            start_times: Arc::clone(&self.start_times),
            performance_profiles: Arc::clone(&self.performance_profiles),
        }
    }
}

/// Performance profile for a pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub pattern_id: String,
    pub metrics: HashMap<String, MetricHistory>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl PerformanceProfile {
    pub fn new(pattern_id: &str) -> Self {
        Self {
            pattern_id: pattern_id.to_string(),
            metrics: HashMap::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }

    pub fn record_metric(&mut self, name: &str, value: f64, unit: &str) {
        let entry = self
            .metrics
            .entry(name.to_string())
            .or_insert_with(|| MetricHistory {
                name: name.to_string(),
                unit: unit.to_string(),
                values: Vec::new(),
                min_value: f64::MAX,
                max_value: f64::MIN,
                average_value: 0.0,
            });

        entry.values.push(MetricValue {
            value,
            timestamp: Utc::now(),
        });

        // Update statistics
        entry.min_value = entry.min_value.min(value);
        entry.max_value = entry.max_value.max(value);

        let total: f64 = entry.values.iter().map(|v| v.value).sum();
        entry.average_value = total / entry.values.len() as f64;

        // Keep only last 1000 values
        if entry.values.len() > 1000 {
            entry.values.remove(0);
        }

        self.last_updated = Utc::now();
    }
}

/// Metric history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricHistory {
    pub name: String,
    pub unit: String,
    pub values: Vec<MetricValue>,
    pub min_value: f64,
    pub max_value: f64,
    pub average_value: f64,
}

/// Metric value with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

/// Monitoring statistics
#[derive(Debug, Clone)]
pub struct MonitoringStatistics {
    pub total_patterns_monitored: usize,
    pub active_patterns: usize,
    pub total_events: usize,
    pub event_types: HashMap<String, usize>,
    pub average_execution_time: f64,
    pub success_rate: f64,
    pub error_rate: f64,
}

impl Default for PatternMonitor {
    fn default() -> Self {
        Self::new(MonitoringConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        AgentInfo, AgentPerformanceMetrics, AgentStatus, CpuAllocator, MemoryPool,
        NetworkResources, PatternConfig, PatternPerformanceMetrics, PatternPhase, PatternResult,
        PatternState, PatternStatus, ResourcePool,
    };
    use super::*;

    #[tokio::test]
    async fn test_monitor_creation() {
        let config = MonitoringConfig::default();
        let monitor = PatternMonitor::new(config);

        assert_eq!(monitor.events.read().await.len(), 0);
        assert_eq!(monitor.metrics.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_pattern_monitoring_lifecycle() {
        let monitor = PatternMonitor::new(MonitoringConfig::default());
        let context = create_test_context();

        // Start monitoring
        monitor.start_monitoring("test_pattern", &context).await;

        // Check that monitoring started
        assert!(monitor
            .start_times
            .read()
            .await
            .contains_key("test_pattern"));

        // Update phase
        monitor
            .update_phase(
                "test_pattern",
                PatternPhase::Initializing,
                PatternPhase::Executing,
                0.5,
            )
            .await;

        // Check events
        let events = monitor.get_events(Some("test_pattern"), None).await;
        assert!(events.len() >= 2); // Started + phase change

        // Stop monitoring
        let result = PatternResult {
            pattern_id: "test_pattern".to_string(),
            success: true,
            data: HashMap::new(),
            performance_metrics: PatternPerformanceMetrics {
                total_execution_time_seconds: 10.0,
                coordination_overhead_seconds: 0.1,
                resource_utilization: 0.8,
                agent_efficiency: 0.9,
                communication_overhead: 5,
            },
            error_message: None,
            completed_at: Utc::now(),
            execution_time_ms: 10000,
            metadata: HashMap::new(),
        };

        monitor.stop_monitoring("test_pattern", &result).await;

        // Check that monitoring stopped
        assert!(!monitor
            .start_times
            .read()
            .await
            .contains_key("test_pattern"));
    }

    #[tokio::test]
    async fn test_event_subscription() {
        let monitor = PatternMonitor::new(MonitoringConfig::default());
        let mut receiver = monitor.subscribe();

        // Record an event
        let event = MonitoringEvent::PatternStarted {
            pattern_id: "test".to_string(),
            timestamp: Utc::now(),
            agent_count: 2,
            resource_usage: ResourceUsageSnapshot {
                memory_usage_mb: 100,
                memory_utilization: 0.5,
                cpu_usage_cores: 2,
                cpu_utilization: 0.3,
                network_bandwidth_mbps: 50,
                network_utilization: 0.2,
                active_file_locks: 0,
                custom_resources: HashMap::new(),
            },
        };

        monitor.record_event(event.clone()).await;

        // Receive the event
        let received_event = receiver.recv().await.unwrap();
        assert!(matches!(
            received_event,
            MonitoringEvent::PatternStarted { .. }
        ));
    }

    #[tokio::test]
    async fn test_performance_profile() {
        let monitor = PatternMonitor::new(MonitoringConfig::default());

        // Record some metrics
        monitor
            .record_performance_metric("test_pattern", "execution_time", 10.5, "seconds")
            .await;
        monitor
            .record_performance_metric("test_pattern", "memory_usage", 512.0, "MB")
            .await;

        // Get performance profile
        let profile = monitor.get_performance_profile("test_pattern").await;
        assert!(profile.is_some());

        let profile = profile.unwrap();
        assert_eq!(profile.pattern_id, "test_pattern");
        assert_eq!(profile.metrics.len(), 2);

        // Check metric statistics
        if let Some(execution_metric) = profile.metrics.get("execution_time") {
            assert_eq!(execution_metric.min_value, 10.5);
            assert_eq!(execution_metric.max_value, 10.5);
            assert_eq!(execution_metric.average_value, 10.5);
            assert_eq!(execution_metric.unit, "seconds");
        }
    }

    #[tokio::test]
    async fn test_monitoring_statistics() {
        let monitor = PatternMonitor::new(MonitoringConfig::default());
        let context = create_test_context();

        // Start and complete a pattern
        monitor.start_monitoring("test_pattern", &context).await;

        let result = PatternResult {
            pattern_id: "test_pattern".to_string(),
            success: true,
            data: HashMap::new(),
            performance_metrics: PatternPerformanceMetrics {
                total_execution_time_seconds: 15.0,
                coordination_overhead_seconds: 0.2,
                resource_utilization: 0.7,
                agent_efficiency: 0.8,
                communication_overhead: 3,
            },
            error_message: None,
            completed_at: Utc::now(),
            execution_time_ms: 15000,
            metadata: HashMap::new(),
        };

        monitor.stop_monitoring("test_pattern", &result).await;

        // Get statistics
        let stats = monitor.get_monitoring_statistics().await;
        assert_eq!(stats.total_patterns_monitored, 1);
        assert_eq!(stats.active_patterns, 0);
        assert!(stats.total_events > 0);
        assert!(stats.success_rate > 0.0);
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
