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

use crate::agent::{
    Agent, AgentConfig, AgentContext, AgentId, AgentMessage, AgentRequest, AgentResponse,
    AgentType, BaseAgent, HealthStatus,
};
use crate::agent::AgentCapability;
use crate::error::{AgentError, AgentResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;
use chrono::{DateTime, Utc};
use std::process::Command;

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Monitoring interval in seconds
    pub interval: u64,
    /// Metrics to collect
    pub metrics: Vec<MetricType>,
    /// Alert thresholds
    pub thresholds: HashMap<String, Threshold>,
    /// Notification channels
    pub notifications: Vec<NotificationChannel>,
    /// Data retention period in days
    pub retention_days: u32,
    /// Custom monitoring rules
    pub custom_rules: Vec<CustomRule>,
}

/// Metric type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    CPU,
    Memory,
    Disk,
    Network,
    Process,
    Application,
    Custom(String),
}

/// Threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    /// Threshold value
    pub value: f64,
    /// Comparison operator
    pub operator: ThresholdOperator,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
}

/// Threshold operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThresholdOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Alert severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel type
    pub channel_type: NotificationType,
    /// Channel configuration
    pub config: HashMap<String, String>,
    /// Enabled alert severities
    pub severities: Vec<AlertSeverity>,
}

/// Notification type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    Email,
    Slack,
    Webhook,
    PagerDuty,
    Custom(String),
}

/// Custom monitoring rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Rule action
    pub action: String,
    /// Rule enabled
    pub enabled: bool,
}

/// Monitoring request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringRequest {
    /// Monitoring configuration
    pub config: MonitoringConfig,
    /// Target systems
    pub targets: Vec<MonitoringTarget>,
    /// Custom options
    pub options: HashMap<String, serde_json::Value>,
}

/// Monitoring target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringTarget {
    /// Target name
    pub name: String,
    /// Target type
    pub target_type: TargetType,
    /// Target address
    pub address: String,
    /// Target credentials
    pub credentials: Option<TargetCredentials>,
    /// Target configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Target type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    Local,
    Remote,
    Container,
    Kubernetes,
    Cloud,
    Custom(String),
}

/// Target credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCredentials {
    /// Username
    pub username: Option<String>,
    /// Password
    pub password: Option<String>,
    /// API key
    pub api_key: Option<String>,
    /// Certificate path
    pub certificate_path: Option<String>,
}

/// Monitoring result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringResult {
    /// Monitoring ID
    pub monitoring_id: String,
    /// Collection timestamp
    pub timestamp: DateTime<Utc>,
    /// Collected metrics
    pub metrics: HashMap<String, MetricValue>,
    /// Active alerts
    pub alerts: Vec<Alert>,
    /// System health status
    pub health_status: HealthStatus,
    /// Performance summary
    pub performance_summary: PerformanceSummary,
}

/// Metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: f64,
    /// Metric unit
    pub unit: String,
    /// Metric timestamp
    pub timestamp: DateTime<Utc>,
    /// Metric metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub alert_id: String,
    /// Alert name
    pub name: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Alert acknowledged
    pub acknowledged: bool,
    /// Alert resolved
    pub resolved: bool,
    /// Alert metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Network usage in MB/s
    pub network_usage: f64,
    /// Active processes
    pub active_processes: u32,
    /// System load average
    pub load_average: [f64; 3],
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU metrics
    pub cpu: CpuMetrics,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Disk metrics
    pub disk: DiskMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// Process metrics
    pub process: ProcessMetrics,
}

/// CPU metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// CPU usage percentage
    pub usage_percentage: f64,
    /// CPU load average (1, 5, 15 minutes)
    pub load_average: [f64; 3],
    /// CPU temperature (if available)
    pub temperature: Option<f64>,
    /// CPU frequency
    pub frequency: Option<f64>,
}

/// Memory metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Total memory in bytes
    pub total: u64,
    /// Used memory in bytes
    pub used: u64,
    /// Free memory in bytes
    pub free: u64,
    /// Available memory in bytes
    pub available: u64,
    /// Memory usage percentage
    pub usage_percentage: f64,
}

/// Disk metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Total disk space in bytes
    pub total: u64,
    /// Used disk space in bytes
    pub used: u64,
    /// Free disk space in bytes
    pub free: u64,
    /// Disk usage percentage
    pub usage_percentage: f64,
    /// Disk I/O operations per second
    pub iops: Option<u64>,
    /// Disk read/write speed in MB/s
    pub read_speed: Option<f64>,
    pub write_speed: Option<f64>,
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Network interface name
    pub interface: String,
    /// Bytes received
    pub bytes_received: u64,
    /// Bytes transmitted
    pub bytes_transmitted: u64,
    /// Packets received
    pub packets_received: u64,
    /// Packets transmitted
    pub packets_transmitted: u64,
    /// Network errors
    pub errors: u64,
    /// Network drops
    pub drops: u64,
}

/// Process metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMetrics {
    /// Total processes
    pub total_processes: u32,
    /// Running processes
    pub running_processes: u32,
    /// Sleeping processes
    pub sleeping_processes: u32,
    /// Stopped processes
    pub stopped_processes: u32,
    /// Zombie processes
    pub zombie_processes: u32,
    /// Top processes by CPU usage
    pub top_cpu_processes: Vec<ProcessInfo>,
    /// Top processes by memory usage
    pub top_memory_processes: Vec<ProcessInfo>,
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub name: String,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Process state
    pub state: String,
    /// Process command
    pub command: String,
}

/// Monitoring Agent
pub struct MonitoringAgent {
    /// Base agent
    base: BaseAgent,
    /// Monitoring configuration
    config: MonitoringConfig,
    /// Active monitoring sessions
    active_sessions: Arc<RwLock<HashMap<String, MonitoringSession>>>,
    /// Metrics history
    metrics_history: Arc<RwLock<Vec<MonitoringResult>>>,
    /// Alert history
    alert_history: Arc<RwLock<Vec<Alert>>>,
    /// Monitoring targets
    targets: Vec<MonitoringTarget>,
    /// Monitoring task handle
    monitoring_task: Option<tokio::task::JoinHandle<()>>,
}

/// Monitoring session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSession {
    /// Session ID
    pub session_id: String,
    /// Target
    pub target: MonitoringTarget,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// Last collection time
    pub last_collection: DateTime<Utc>,
    /// Collection count
    pub collection_count: u64,
    /// Session status
    pub status: SessionStatus,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Paused,
    Stopped,
    Error,
}

impl MonitoringAgent {
    /// Create a new monitoring agent
    pub fn new(id: AgentId) -> Self {
        let config = AgentConfig {
            name: "Monitoring Agent".to_string(),
            description: Some("Agent for system monitoring and alerting".to_string()),
            agent_type: AgentType::Monitoring,
            capabilities: vec![
                AgentCapability::Monitoring,
                AgentCapability::Analysis,
                AgentCapability::Communication,
            ],
            max_concurrent_tasks: 10,
            task_timeout: 300, // 5 minutes
            memory_limit: Some(512), // 512 MB
            cpu_limit: Some(25.0), // 25% CPU
            retry_attempts: 3,
            retry_delay: 5,
            parameters: HashMap::new(),
            tags: vec![
                "monitoring".to_string(),
                "metrics".to_string(),
                "alerting".to_string(),
            ],
        };

        let monitoring_config = MonitoringConfig {
            interval: 60, // 1 minute default
            metrics: vec![
                MetricType::CPU,
                MetricType::Memory,
                MetricType::Disk,
                MetricType::Network,
                MetricType::Process,
            ],
            thresholds: HashMap::new(),
            notifications: Vec::new(),
            retention_days: 30,
            custom_rules: Vec::new(),
        };

        Self {
            base: BaseAgent::new(id, config),
            config: monitoring_config,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            targets: Vec::new(),
            monitoring_task: None,
        }
    }

    /// Start monitoring
    async fn start_monitoring(&mut self, request: MonitoringRequest) -> AgentResult<MonitoringResult> {
        let monitoring_id = uuid::Uuid::new_v4().to_string();
        
        // Update configuration
        self.config = request.config;
        self.targets = request.targets;

        // Create monitoring sessions for each target
        for target in &self.targets {
            let session = MonitoringSession {
                session_id: uuid::Uuid::new_v4().to_string(),
                target: target.clone(),
                start_time: Utc::now(),
                last_collection: Utc::now(),
                collection_count: 0,
                status: SessionStatus::Active,
            };

            self.active_sessions.write().await.insert(session.session_id.clone(), session);
        }

        // Start monitoring task
        let active_sessions = self.active_sessions.clone();
        let metrics_history = self.metrics_history.clone();
        let alert_history = self.alert_history.clone();
        let config = self.config.clone();

        let monitoring_id_clone = monitoring_id.clone();
        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(config.interval));
            
            loop {
                interval.tick().await;
                
                // Collect metrics from all active sessions
                let sessions = active_sessions.read().await;
                for (_session_id, session) in sessions.iter() {
                    if let SessionStatus::Active = session.status {
                        // Collect metrics (simplified for this example)
                        let metrics = Self::collect_system_metrics().await;
                        let alerts = Self::check_thresholds(&metrics, &config.thresholds).await;
                        
                        let result = MonitoringResult {
                            monitoring_id: monitoring_id_clone.clone(),
                            timestamp: Utc::now(),
                            metrics: Self::convert_metrics_to_map(&metrics),
                            alerts: alerts.clone(),
                            health_status: Self::determine_health_status(&alerts),
                            performance_summary: Self::create_performance_summary(&metrics),
                        };

                        // Store results
                        metrics_history.write().await.push(result);
                        
                        // Store alerts
                        for alert in alerts {
                            alert_history.write().await.push(alert);
                        }
                    }
                }
            }
        });

        self.monitoring_task = Some(task);

        // Return initial monitoring result
        let initial_metrics = Self::collect_system_metrics().await;
        Ok(MonitoringResult {
            monitoring_id,
            timestamp: Utc::now(),
            metrics: Self::convert_metrics_to_map(&initial_metrics),
            alerts: Vec::new(),
            health_status: HealthStatus::Healthy,
            performance_summary: Self::create_performance_summary(&initial_metrics),
        })
    }

    /// Collect system metrics
    async fn collect_system_metrics() -> SystemMetrics {
        // CPU metrics
        let cpu_usage = Self::get_cpu_usage().await;
        let load_average = Self::get_load_average().await;
        
        let cpu_metrics = CpuMetrics {
            usage_percentage: cpu_usage,
            load_average,
            temperature: None,
            frequency: None,
        };

        // Memory metrics
        let memory_metrics = Self::get_memory_metrics().await;

        // Disk metrics
        let disk_metrics = Self::get_disk_metrics().await;

        // Network metrics
        let network_metrics = Self::get_network_metrics().await;

        // Process metrics
        let process_metrics = Self::get_process_metrics().await;

        SystemMetrics {
            cpu: cpu_metrics,
            memory: memory_metrics,
            disk: disk_metrics,
            network: network_metrics,
            process: process_metrics,
        }
    }

    /// Get CPU usage
    async fn get_cpu_usage() -> f64 {
        // Simplified CPU usage calculation
        // In a real implementation, this would read from /proc/stat or use system APIs
        let output = Command::new("top")
            .args(&["-l", "1", "-n", "0"])
            .output();

        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Parse CPU usage from top output (simplified)
                if let Some(line) = output_str.lines().find(|line| line.contains("CPU usage")) {
                    if let Some(usage_str) = line.split(':').nth(1) {
                        if let Some(percentage) = usage_str.split('%').next() {
                            if let Ok(usage) = percentage.trim().parse::<f64>() {
                                return usage;
                            }
                        }
                    }
                }
                0.0
            }
            Err(_) => 0.0,
        }
    }

    /// Get load average
    async fn get_load_average() -> [f64; 3] {
        // Simplified load average
        // In a real implementation, this would read from /proc/loadavg
        [0.5, 0.3, 0.2]
    }

    /// Get memory metrics
    async fn get_memory_metrics() -> MemoryMetrics {
        // Simplified memory metrics
        // In a real implementation, this would read from /proc/meminfo or use system APIs
        let total = 16 * 1024 * 1024 * 1024; // 16 GB
        let used = 8 * 1024 * 1024 * 1024;   // 8 GB
        let free = 4 * 1024 * 1024 * 1024;   // 4 GB
        let available = 6 * 1024 * 1024 * 1024; // 6 GB

        MemoryMetrics {
            total,
            used,
            free,
            available,
            usage_percentage: (used as f64 / total as f64) * 100.0,
        }
    }

    /// Get disk metrics
    async fn get_disk_metrics() -> DiskMetrics {
        // Simplified disk metrics
        let total = 500 * 1024 * 1024 * 1024; // 500 GB
        let used = 200 * 1024 * 1024 * 1024;  // 200 GB
        let free = 300 * 1024 * 1024 * 1024;  // 300 GB

        DiskMetrics {
            total,
            used,
            free,
            usage_percentage: (used as f64 / total as f64) * 100.0,
            iops: Some(1000),
            read_speed: Some(50.0),
            write_speed: Some(30.0),
        }
    }

    /// Get network metrics
    async fn get_network_metrics() -> NetworkMetrics {
        // Simplified network metrics
        NetworkMetrics {
            interface: "eth0".to_string(),
            bytes_received: 1024 * 1024 * 100, // 100 MB
            bytes_transmitted: 1024 * 1024 * 50, // 50 MB
            packets_received: 1000,
            packets_transmitted: 500,
            errors: 0,
            drops: 0,
        }
    }

    /// Get process metrics
    async fn get_process_metrics() -> ProcessMetrics {
        // Simplified process metrics
        let top_processes = vec![
            ProcessInfo {
                pid: 1234,
                name: "systemd".to_string(),
                cpu_usage: 2.5,
                memory_usage: 1024 * 1024 * 50, // 50 MB
                state: "S".to_string(),
                command: "/usr/lib/systemd/systemd".to_string(),
            },
            ProcessInfo {
                pid: 5678,
                name: "bash".to_string(),
                cpu_usage: 1.2,
                memory_usage: 1024 * 1024 * 10, // 10 MB
                state: "S".to_string(),
                command: "/bin/bash".to_string(),
            },
        ];

        ProcessMetrics {
            total_processes: 150,
            running_processes: 120,
            sleeping_processes: 25,
            stopped_processes: 3,
            zombie_processes: 2,
            top_cpu_processes: top_processes.clone(),
            top_memory_processes: top_processes,
        }
    }

    /// Check thresholds and generate alerts
    async fn check_thresholds(metrics: &SystemMetrics, thresholds: &HashMap<String, Threshold>) -> Vec<Alert> {
        let mut alerts = Vec::new();

        for (metric_name, threshold) in thresholds {
            let value = match metric_name.as_str() {
                "cpu_usage" => metrics.cpu.usage_percentage,
                "memory_usage" => metrics.memory.usage_percentage,
                "disk_usage" => metrics.disk.usage_percentage,
                _ => continue,
            };

            let should_alert = match threshold.operator {
                ThresholdOperator::GreaterThan => value > threshold.value,
                ThresholdOperator::LessThan => value < threshold.value,
                ThresholdOperator::Equal => (value - threshold.value).abs() < 0.01,
                ThresholdOperator::NotEqual => (value - threshold.value).abs() >= 0.01,
                ThresholdOperator::GreaterThanOrEqual => value >= threshold.value,
                ThresholdOperator::LessThanOrEqual => value <= threshold.value,
            };

            if should_alert {
                alerts.push(Alert {
                    alert_id: uuid::Uuid::new_v4().to_string(),
                    name: metric_name.clone(),
                    severity: threshold.severity.clone(),
                    message: threshold.message.clone(),
                    timestamp: Utc::now(),
                    acknowledged: false,
                    resolved: false,
                    metadata: HashMap::new(),
                });
            }
        }

        alerts
    }

    /// Convert metrics to map format
    fn convert_metrics_to_map(metrics: &SystemMetrics) -> HashMap<String, MetricValue> {
        let mut map = HashMap::new();
        let timestamp = Utc::now();

        map.insert("cpu_usage".to_string(), MetricValue {
            name: "cpu_usage".to_string(),
            value: metrics.cpu.usage_percentage,
            unit: "%".to_string(),
            timestamp,
            metadata: HashMap::new(),
        });

        map.insert("memory_usage".to_string(), MetricValue {
            name: "memory_usage".to_string(),
            value: metrics.memory.usage_percentage,
            unit: "%".to_string(),
            timestamp,
            metadata: HashMap::new(),
        });

        map.insert("disk_usage".to_string(), MetricValue {
            name: "disk_usage".to_string(),
            value: metrics.disk.usage_percentage,
            unit: "%".to_string(),
            timestamp,
            metadata: HashMap::new(),
        });

        map
    }

    /// Determine health status based on alerts
    fn determine_health_status(alerts: &[Alert]) -> HealthStatus {
        for alert in alerts {
            match alert.severity {
                AlertSeverity::Critical | AlertSeverity::Emergency => {
                    return HealthStatus::Critical;
                }
                AlertSeverity::Warning => {
                    return HealthStatus::Warning;
                }
                _ => {}
            }
        }
        HealthStatus::Healthy
    }

    /// Create performance summary
    fn create_performance_summary(metrics: &SystemMetrics) -> PerformanceSummary {
        PerformanceSummary {
            cpu_usage: metrics.cpu.usage_percentage,
            memory_usage: metrics.memory.usage_percentage,
            disk_usage: metrics.disk.usage_percentage,
            network_usage: (metrics.network.bytes_received + metrics.network.bytes_transmitted) as f64 / 1024.0 / 1024.0,
            active_processes: metrics.process.running_processes,
            load_average: metrics.cpu.load_average,
        }
    }

    /// Get monitoring history
    pub async fn get_monitoring_history(&self) -> Vec<MonitoringResult> {
        self.metrics_history.read().await.clone()
    }

    /// Get alert history
    pub async fn get_alert_history(&self) -> Vec<Alert> {
        self.alert_history.read().await.clone()
    }

    /// Get active sessions
    pub async fn get_active_sessions(&self) -> HashMap<String, MonitoringSession> {
        self.active_sessions.read().await.clone()
    }
}

#[async_trait]
impl Agent for MonitoringAgent {
    fn id(&self) -> &AgentId {
        self.base.id()
    }

    fn config(&self) -> &AgentConfig {
        self.base.config()
    }

    fn context(&self) -> &AgentContext {
        self.base.context()
    }

    fn context_mut(&mut self) -> &mut AgentContext {
        self.base.context_mut()
    }

    async fn initialize(&mut self) -> AgentResult<()> {
        self.base.initialize().await
    }

    async fn start(&mut self) -> AgentResult<()> {
        self.base.start().await
    }

    async fn stop(&mut self) -> AgentResult<()> {
        // Stop monitoring task
        if let Some(task) = self.monitoring_task.take() {
            task.abort();
        }

        self.base.stop().await
    }

    async fn handle_message(&mut self, message: AgentMessage) -> AgentResult<Option<AgentMessage>> {
        match message {
            AgentMessage::TaskRequest(request) => {
                let response = self.execute_task(request).await?;
                Ok(Some(AgentMessage::TaskResponse(response)))
            }
            _ => Ok(None),
        }
    }

    async fn execute_task(&mut self, request: AgentRequest) -> AgentResult<AgentResponse> {
        match request.request_type.as_str() {
            "start_monitoring" => {
                let monitoring_request: MonitoringRequest = serde_json::from_value(request.payload)
                    .map_err(|e| AgentError::SerializationError { reason: e.to_string() })?;

                let start_time = std::time::Instant::now();
                let result = self.start_monitoring(monitoring_request).await?;
                let execution_time = start_time.elapsed().as_millis() as u64;

                Ok(AgentResponse::success(request.id, serde_json::to_value(result).unwrap())
                    .with_execution_time(execution_time))
            }
            "get_monitoring_history" => {
                let history = self.get_monitoring_history().await;
                Ok(AgentResponse::success(request.id, serde_json::to_value(history).unwrap()))
            }
            "get_alert_history" => {
                let alerts = self.get_alert_history().await;
                Ok(AgentResponse::success(request.id, serde_json::to_value(alerts).unwrap()))
            }
            "get_active_sessions" => {
                let sessions = self.get_active_sessions().await;
                Ok(AgentResponse::success(request.id, serde_json::to_value(sessions).unwrap()))
            }
            _ => {
                Ok(AgentResponse::error(request.id, "Unknown task type".to_string()))
            }
        }
    }

    async fn get_status(&self) -> AgentResult<crate::agent::AgentStatus> {
        self.base.get_status().await
    }

    async fn check_health(&self) -> AgentResult<HealthStatus> {
        self.base.check_health().await
    }

    fn capabilities(&self) -> &[AgentCapability] {
        self.base.capabilities()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_agent_creation() {
        let agent = MonitoringAgent::new("test-monitoring-agent".to_string());
        assert_eq!(agent.id(), "test-monitoring-agent");
        assert_eq!(agent.config().agent_type, AgentType::Monitoring);
    }

    #[tokio::test]
    async fn test_monitoring_agent_initialization() {
        let mut agent = MonitoringAgent::new("test-monitoring-agent".to_string());
        assert!(agent.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_monitoring_config_creation() {
        let config = MonitoringConfig {
            interval: 60,
            metrics: vec![MetricType::CPU, MetricType::Memory],
            thresholds: HashMap::new(),
            notifications: Vec::new(),
            retention_days: 30,
            custom_rules: Vec::new(),
        };

        assert_eq!(config.interval, 60);
        assert_eq!(config.metrics.len(), 2);
    }

    #[tokio::test]
    async fn test_monitoring_request_creation() {
        let config = MonitoringConfig {
            interval: 60,
            metrics: vec![MetricType::CPU, MetricType::Memory],
            thresholds: HashMap::new(),
            notifications: Vec::new(),
            retention_days: 30,
            custom_rules: Vec::new(),
        };

        let target = MonitoringTarget {
            name: "localhost".to_string(),
            target_type: TargetType::Local,
            address: "127.0.0.1".to_string(),
            credentials: None,
            config: HashMap::new(),
        };

        let request = MonitoringRequest {
            config,
            targets: vec![target],
            options: HashMap::new(),
        };

        assert_eq!(request.targets.len(), 1);
        assert_eq!(request.targets[0].name, "localhost");
    }

    #[tokio::test]
    async fn test_threshold_checking() {
        let mut thresholds = HashMap::new();
        thresholds.insert("cpu_usage".to_string(), Threshold {
            value: 80.0,
            operator: ThresholdOperator::GreaterThan,
            severity: AlertSeverity::Warning,
            message: "CPU usage is high".to_string(),
        });

        let metrics = SystemMetrics {
            cpu: CpuMetrics {
                usage_percentage: 85.0,
                load_average: [1.0, 1.0, 1.0],
                temperature: None,
                frequency: None,
            },
            memory: MemoryMetrics {
                total: 1024,
                used: 512,
                free: 512,
                available: 512,
                usage_percentage: 50.0,
            },
            disk: DiskMetrics {
                total: 1024,
                used: 512,
                free: 512,
                usage_percentage: 50.0,
                iops: None,
                read_speed: None,
                write_speed: None,
            },
            network: NetworkMetrics {
                interface: "eth0".to_string(),
                bytes_received: 0,
                bytes_transmitted: 0,
                packets_received: 0,
                packets_transmitted: 0,
                errors: 0,
                drops: 0,
            },
            process: ProcessMetrics {
                total_processes: 0,
                running_processes: 0,
                sleeping_processes: 0,
                stopped_processes: 0,
                zombie_processes: 0,
                top_cpu_processes: Vec::new(),
                top_memory_processes: Vec::new(),
            },
        };

        let alerts = MonitoringAgent::check_thresholds(&metrics, &thresholds).await;
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].severity, AlertSeverity::Warning);
    }
} 