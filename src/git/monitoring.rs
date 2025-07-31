use crate::RhemaResult;
use git2::Repository;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration as TokioDuration};
use std::thread;


/// Monitoring configuration for Git integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    
    /// Performance monitoring settings
    pub performance: PerformanceMonitoringConfig,
    
    /// Metrics collection settings
    pub metrics: MetricsConfig,
    
    /// Real-time monitoring settings
    pub realtime: RealtimeMonitoringConfig,
    
    /// Alerting settings
    pub alerting: MonitoringAlertingConfig,
    
    /// Dashboard settings
    pub dashboard: DashboardConfig,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    
    /// Monitor Git operations
    pub monitor_git_operations: bool,
    
    /// Monitor context operations
    pub monitor_context_operations: bool,
    
    /// Monitor hook execution
    pub monitor_hook_execution: bool,
    
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
    
    /// Sampling rate (percentage)
    pub sampling_rate: f64,
}

/// Performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Slow operation threshold (ms)
    pub slow_operation_threshold: u64,
    
    /// Very slow operation threshold (ms)
    pub very_slow_threshold: u64,
    
    /// Memory usage threshold (MB)
    pub memory_threshold: u64,
    
    /// CPU usage threshold (percentage)
    pub cpu_threshold: f64,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    
    /// Metrics storage
    pub storage: MetricsStorage,
    
    /// Collection intervals
    pub intervals: MetricsIntervals,
    
    /// Metrics to collect
    pub metrics: Vec<MetricType>,
    
    /// Retention policy
    pub retention: MetricsRetention,
}

/// Metrics storage options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsStorage {
    File(PathBuf),
    Database(String),
    InMemory,
    Custom(String),
}

/// Metrics collection intervals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsIntervals {
    /// Git operation metrics interval (seconds)
    pub git_operations: u64,
    
    /// Context metrics interval (seconds)
    pub context_metrics: u64,
    
    /// Performance metrics interval (seconds)
    pub performance_metrics: u64,
    
    /// System metrics interval (seconds)
    pub system_metrics: u64,
}

/// Metric types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MetricType {
    GitOperations,
    ContextOperations,
    PerformanceMetrics,
    SystemMetrics,
    Custom(String),
}

/// Metrics retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsRetention {
    /// Keep metrics for N days
    pub retention_days: u32,
    
    /// Aggregate old metrics
    pub aggregate_old_metrics: bool,
    
    /// Archive old metrics
    pub archive_old_metrics: bool,
    
    /// Archive directory
    pub archive_directory: Option<PathBuf>,
}

/// Real-time monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMonitoringConfig {
    /// Enable real-time monitoring
    pub enabled: bool,
    
    /// WebSocket server
    pub websocket: WebSocketConfig,
    
    /// Event streaming
    pub event_streaming: EventStreamingConfig,
    
    /// Live dashboards
    pub live_dashboards: bool,
    
    /// Real-time alerts
    pub realtime_alerts: bool,
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// WebSocket server port
    pub port: u16,
    
    /// WebSocket server host
    pub host: String,
    
    /// Enable SSL
    pub ssl_enabled: bool,
    
    /// SSL certificate path
    pub ssl_cert_path: Option<PathBuf>,
    
    /// SSL key path
    pub ssl_key_path: Option<PathBuf>,
}

/// Event streaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStreamingConfig {
    /// Enable event streaming
    pub enabled: bool,
    
    /// Stream buffer size
    pub buffer_size: usize,
    
    /// Event types to stream
    pub event_types: Vec<EventType>,
    
    /// Stream format
    pub format: StreamFormat,
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    GitOperation,
    ContextChange,
    PerformanceEvent,
    SecurityEvent,
    SystemEvent,
    Custom(String),
}

/// Stream formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamFormat {
    JSON,
    MessagePack,
    Protobuf,
    Custom(String),
}

/// Monitoring alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringAlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    
    /// Alert channels
    pub channels: Vec<AlertChannel>,
    
    /// Alert rules
    pub rules: Vec<AlertRule>,
    
    /// Alert severity levels
    pub severity_levels: Vec<AlertSeverity>,
}

/// Alert channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    Email(EmailAlertChannel),
    Slack(SlackAlertChannel),
    Webhook(WebhookAlertChannel),
    Custom(CustomAlertChannel),
}

/// Email alert channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAlertChannel {
    pub recipients: Vec<String>,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
}

/// Slack alert channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackAlertChannel {
    pub webhook_url: String,
    pub channel: String,
    pub username: String,
}

/// Webhook alert channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAlertChannel {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub timeout: u64,
}

/// Custom alert channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAlertChannel {
    pub name: String,
    pub config: HashMap<String, String>,
}

/// Alert rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    
    /// Rule condition
    pub condition: AlertCondition,
    
    /// Alert severity
    pub severity: AlertSeverity,
    
    /// Alert channels
    pub channels: Vec<String>,
    
    /// Cooldown period (seconds)
    pub cooldown: u64,
}

/// Alert conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    Threshold {
        metric: String,
        operator: ThresholdOperator,
        value: f64,
    },
    Anomaly {
        metric: String,
        sensitivity: f64,
    },
    Pattern {
        pattern: String,
        count: usize,
        window: Duration,
    },
}

/// Threshold operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThresholdOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Enable dashboard
    pub enabled: bool,
    
    /// Dashboard server
    pub server: DashboardServer,
    
    /// Dashboard widgets
    pub widgets: Vec<DashboardWidget>,
    
    /// Auto-refresh interval (seconds)
    pub auto_refresh: u64,
}

/// Dashboard server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardServer {
    /// Server port
    pub port: u16,
    
    /// Server host
    pub host: String,
    
    /// Enable authentication
    pub auth_enabled: bool,
    
    /// Static files directory
    pub static_dir: Option<PathBuf>,
}

/// Dashboard widgets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    /// Widget name
    pub name: String,
    
    /// Widget type
    pub widget_type: WidgetType,
    
    /// Widget configuration
    pub config: HashMap<String, String>,
    
    /// Widget position
    pub position: WidgetPosition,
}

/// Widget types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    LineChart,
    BarChart,
    PieChart,
    Gauge,
    Table,
    Text,
    Custom(String),
}

/// Widget position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Git monitoring manager
pub struct GitMonitoringManager {
    _repo: Repository,
    config: MonitoringConfig,
    metrics_collector: Arc<Mutex<MetricsCollector>>,
    performance_monitor: Arc<Mutex<PerformanceMonitor>>,
    realtime_monitor: Arc<Mutex<RealtimeMonitor>>,
    alert_manager: Arc<Mutex<AlertManager>>,
    running: Arc<Mutex<bool>>,
}

/// Metrics collector
pub struct MetricsCollector {
    metrics: HashMap<String, MetricValue>,
    _storage: MetricsStorage,
    _intervals: MetricsIntervals,
}

/// Metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// Performance monitor
pub struct PerformanceMonitor {
    operations: HashMap<String, OperationMetrics>,
    thresholds: PerformanceThresholds,
    _sampling_rate: f64,
}

/// Operation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetrics {
    pub operation_name: String,
    pub count: u64,
    pub total_duration: Duration,
    pub average_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub slow_operations: u64,
    pub very_slow_operations: u64,
}

/// Real-time monitor
pub struct RealtimeMonitor {
    event_stream: Vec<MonitoringEvent>,
    _websocket_config: WebSocketConfig,
    _event_streaming_config: EventStreamingConfig,
}

/// Monitoring event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringEvent {
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub data: HashMap<String, String>,
    pub severity: AlertSeverity,
}

/// Alert manager
pub struct AlertManager {
    _rules: Vec<AlertRule>,
    _channels: Vec<AlertChannel>,
    _last_alerts: HashMap<String, DateTime<Utc>>,
}

impl GitMonitoringManager {
    /// Create a new monitoring manager
    pub fn new(repo: Repository, config: MonitoringConfig) -> RhemaResult<Self> {
        let metrics_collector = Arc::new(Mutex::new(MetricsCollector::new(
            config.metrics.storage.clone(),
            config.metrics.intervals.clone(),
        )));
        
        let performance_monitor = Arc::new(Mutex::new(PerformanceMonitor::new(
            config.performance.thresholds.clone(),
            config.performance.sampling_rate,
        )));
        
        let realtime_monitor = Arc::new(Mutex::new(RealtimeMonitor::new(
            config.realtime.websocket.clone(),
            config.realtime.event_streaming.clone(),
        )));
        
        let alert_manager = Arc::new(Mutex::new(AlertManager::new(
            config.alerting.rules.clone(),
            config.alerting.channels.clone(),
        )));
        
        Ok(Self {
            _repo: repo,
            config,
            metrics_collector,
            performance_monitor,
            realtime_monitor,
            alert_manager,
            running: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Start monitoring
    pub fn start_monitoring(&self) -> RhemaResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        let mut running = self.running.lock().unwrap();
        if *running {
            return Ok(());
        }
        
        *running = true;
        drop(running);
        
        // Start metrics collection
        if self.config.metrics.enabled {
            self.start_metrics_collection()?;
        }
        
        // Start performance monitoring
        if self.config.performance.enabled {
            self.start_performance_monitoring()?;
        }
        
        // Start real-time monitoring
        if self.config.realtime.enabled {
            self.start_realtime_monitoring()?;
        }
        
        // Start alerting
        if self.config.alerting.enabled {
            self.start_alerting()?;
        }
        
        println!("Git monitoring started successfully!");
        
        Ok(())
    }
    
    /// Stop monitoring
    pub fn stop_monitoring(&self) -> RhemaResult<()> {
        let mut running = self.running.lock().unwrap();
        *running = false;
        
        println!("Git monitoring stopped successfully!");
        
        Ok(())
    }
    
    /// Record Git operation
    pub fn record_git_operation(&self, operation: &str, duration: Duration) -> RhemaResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Record performance metrics
        if self.config.performance.enabled {
            let mut monitor = self.performance_monitor.lock().unwrap();
            monitor.record_operation(operation, duration);
        }
        
        // Record metrics
        if self.config.metrics.enabled {
            let mut collector = self.metrics_collector.lock().unwrap();
            collector.record_metric(
                "git_operation_duration",
                duration.num_milliseconds() as f64,
                &[("operation", operation)],
            );
        }
        
        // Send real-time event
        if self.config.realtime.enabled {
            let mut monitor = self.realtime_monitor.lock().unwrap();
            monitor.send_event(
                EventType::GitOperation,
                &[("operation", operation), ("duration", &duration.num_milliseconds().to_string())],
                AlertSeverity::Info,
            );
        }
        
        Ok(())
    }
    
    /// Record context operation
    pub fn record_context_operation(&self, operation: &str, duration: Duration) -> RhemaResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Record performance metrics
        if self.config.performance.enabled {
            let mut monitor = self.performance_monitor.lock().unwrap();
            monitor.record_operation(operation, duration);
        }
        
        // Record metrics
        if self.config.metrics.enabled {
            let mut collector = self.metrics_collector.lock().unwrap();
            collector.record_metric(
                "context_operation_duration",
                duration.num_milliseconds() as f64,
                &[("operation", operation)],
            );
        }
        
        // Send real-time event
        if self.config.realtime.enabled {
            let mut monitor = self.realtime_monitor.lock().unwrap();
            monitor.send_event(
                EventType::ContextChange,
                &[("operation", operation), ("duration", &duration.num_milliseconds().to_string())],
                AlertSeverity::Info,
            );
        }
        
        Ok(())
    }
    
    /// Get monitoring status
    pub fn get_status(&self) -> RhemaResult<MonitoringStatus> {
        let running = *self.running.lock().unwrap();
        let metrics_collector = self.metrics_collector.lock().unwrap();
        let performance_monitor = self.performance_monitor.lock().unwrap();
        let realtime_monitor = self.realtime_monitor.lock().unwrap();
        
        Ok(MonitoringStatus {
            is_active: running,
            running,
            metrics_enabled: self.config.metrics.enabled,
            performance_enabled: self.config.performance.enabled,
            realtime_enabled: self.config.realtime.enabled,
            alerting_enabled: self.config.alerting.enabled,
            metrics_count: metrics_collector.metrics.len(),
            operations_count: performance_monitor.operations.len(),
            events_count: realtime_monitor.event_stream.len(),
            last_update: Utc::now(),
        })
    }
    
    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> RhemaResult<Vec<OperationMetrics>> {
        let monitor = self.performance_monitor.lock().unwrap();
        Ok(monitor.operations.values().cloned().collect())
    }
    
    /// Get metrics
    pub fn get_metrics(&self, metric_name: &str) -> RhemaResult<Vec<MetricValue>> {
        let collector = self.metrics_collector.lock().unwrap();
        Ok(collector.get_metrics(metric_name))
    }
    
    /// Get recent events
    pub fn get_recent_events(&self, limit: Option<usize>) -> RhemaResult<Vec<MonitoringEvent>> {
        let monitor = self.realtime_monitor.lock().unwrap();
        let events = monitor.event_stream.clone();
        let limit = limit.unwrap_or(100);
        Ok(events.into_iter().rev().take(limit).collect())
    }
    
    /// Start metrics collection
    fn start_metrics_collection(&self) -> RhemaResult<()> {
        let config = self.config.clone();
        let metrics_collector = self.metrics_collector.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut interval = interval(TokioDuration::from_secs(config.metrics.intervals.git_operations));
                
                loop {
                    interval.tick().await;
                    
                    // Collect Git operation metrics
                    if let Ok(mut collector) = metrics_collector.lock() {
                        collector.collect_git_metrics();
                    }
                }
            });
        });
        
        Ok(())
    }
    
    /// Start performance monitoring
    fn start_performance_monitoring(&self) -> RhemaResult<()> {
        let config = self.config.clone();
        let performance_monitor = self.performance_monitor.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut interval = interval(TokioDuration::from_secs(config.performance.thresholds.slow_operation_threshold));
                
                loop {
                    interval.tick().await;
                    
                    // Check performance thresholds
                    if let Ok(mut monitor) = performance_monitor.lock() {
                        monitor.check_thresholds();
                    }
                }
            });
        });
        
        Ok(())
    }
    
    /// Start real-time monitoring
    fn start_realtime_monitoring(&self) -> RhemaResult<()> {
        let _config = self.config.clone();
        let realtime_monitor = self.realtime_monitor.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Start WebSocket server
                if let Ok(mut monitor) = realtime_monitor.lock() {
                    monitor.start_websocket_server();
                }
            });
        });
        
        Ok(())
    }
    
    /// Start alerting
    fn start_alerting(&self) -> RhemaResult<()> {
        let _config = self.config.clone();
        let alert_manager = self.alert_manager.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut interval = interval(TokioDuration::from_secs(60)); // Check every minute
                
                loop {
                    interval.tick().await;
                    
                    // Check alert rules
                    if let Ok(mut manager) = alert_manager.lock() {
                        manager.check_rules();
                    }
                }
            });
        });
        
        Ok(())
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(storage: MetricsStorage, intervals: MetricsIntervals) -> Self {
        Self {
            metrics: HashMap::new(),
            _storage: storage,
            _intervals: intervals,
        }
    }
    
    /// Record a metric
    pub fn record_metric(&mut self, name: &str, value: f64, tags: &[(&str, &str)]) {
        let key = format!("{}_{}", name, tags.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("_"));
        
        let metric = MetricValue {
            name: name.to_string(),
            value,
            timestamp: Utc::now(),
            tags: tags.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        };
        
        self.metrics.insert(key, metric);
    }
    
    /// Get metrics by name
    pub fn get_metrics(&self, name: &str) -> Vec<MetricValue> {
        self.metrics
            .values()
            .filter(|m| m.name == name)
            .cloned()
            .collect()
    }
    
    /// Collect Git metrics
    pub fn collect_git_metrics(&mut self) {
        // TODO: Implement Git metrics collection
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(thresholds: PerformanceThresholds, sampling_rate: f64) -> Self {
        Self {
            operations: HashMap::new(),
            thresholds,
            _sampling_rate: sampling_rate,
        }
    }
    
    /// Record an operation
    pub fn record_operation(&mut self, operation_name: &str, duration: Duration) {
        let entry = self.operations.entry(operation_name.to_string()).or_insert_with(|| OperationMetrics {
            operation_name: operation_name.to_string(),
            count: 0,
            total_duration: Duration::zero(),
            average_duration: Duration::zero(),
            min_duration: duration,
            max_duration: duration,
            slow_operations: 0,
            very_slow_operations: 0,
        });
        
        entry.count += 1;
        entry.total_duration = entry.total_duration + duration;
        entry.average_duration = Duration::milliseconds(entry.total_duration.num_milliseconds() / entry.count as i64);
        
        if duration.num_milliseconds() > entry.max_duration.num_milliseconds() {
            entry.max_duration = duration;
        }
        
        if duration.num_milliseconds() < entry.min_duration.num_milliseconds() {
            entry.min_duration = duration;
        }
        
        if duration.num_milliseconds() > self.thresholds.slow_operation_threshold as i64 {
            entry.slow_operations += 1;
        }
        
        if duration.num_milliseconds() > self.thresholds.very_slow_threshold as i64 {
            entry.very_slow_operations += 1;
        }
    }
    
    /// Check performance thresholds
    pub fn check_thresholds(&mut self) {
        // TODO: Implement threshold checking
    }
}

impl RealtimeMonitor {
    /// Create a new real-time monitor
    pub fn new(websocket_config: WebSocketConfig, event_streaming_config: EventStreamingConfig) -> Self {
        Self {
            event_stream: Vec::new(),
            _websocket_config: websocket_config,
            _event_streaming_config: event_streaming_config,
        }
    }
    
    /// Send an event
    pub fn send_event(&mut self, event_type: EventType, data: &[(&str, &str)], severity: AlertSeverity) {
        let event = MonitoringEvent {
            event_type,
            timestamp: Utc::now(),
            data: data.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            severity,
        };
        
        self.event_stream.push(event);
        
        // Keep only recent events
        if self.event_stream.len() > 1000 {
            self.event_stream.remove(0);
        }
    }
    
    /// Start WebSocket server
    pub fn start_websocket_server(&mut self) {
        // TODO: Implement WebSocket server
    }
}

impl AlertManager {
    /// Create a new alert manager
    pub fn new(rules: Vec<AlertRule>, channels: Vec<AlertChannel>) -> Self {
        Self {
            _rules: rules,
            _channels: channels,
            _last_alerts: HashMap::new(),
        }
    }
    
    /// Check alert rules
    pub fn check_rules(&mut self) {
        // TODO: Implement alert rule checking
    }
}

/// Monitoring status
#[derive(Debug, Clone)]
pub struct MonitoringStatus {
    pub is_active: bool,
    pub running: bool,
    pub metrics_enabled: bool,
    pub performance_enabled: bool,
    pub realtime_enabled: bool,
    pub alerting_enabled: bool,
    pub metrics_count: usize,
    pub operations_count: usize,
    pub events_count: usize,
    pub last_update: DateTime<Utc>,
}

/// Default monitoring configuration
pub fn default_monitoring_config() -> MonitoringConfig {
    MonitoringConfig {
        enabled: true,
        performance: PerformanceMonitoringConfig {
            enabled: true,
            monitor_git_operations: true,
            monitor_context_operations: true,
            monitor_hook_execution: true,
            thresholds: PerformanceThresholds {
                slow_operation_threshold: 1000,
                very_slow_threshold: 5000,
                memory_threshold: 512,
                cpu_threshold: 80.0,
            },
            sampling_rate: 1.0,
        },
        metrics: MetricsConfig {
            enabled: true,
            storage: MetricsStorage::File(PathBuf::from(".rhema/monitoring/metrics")),
            intervals: MetricsIntervals {
                git_operations: 60,
                context_metrics: 300,
                performance_metrics: 60,
                system_metrics: 300,
            },
            metrics: vec![
                MetricType::GitOperations,
                MetricType::ContextOperations,
                MetricType::PerformanceMetrics,
            ],
            retention: MetricsRetention {
                retention_days: 30,
                aggregate_old_metrics: true,
                archive_old_metrics: true,
                archive_directory: Some(PathBuf::from(".rhema/monitoring/archive")),
            },
        },
        realtime: RealtimeMonitoringConfig {
            enabled: false,
            websocket: WebSocketConfig {
                port: 8080,
                host: "localhost".to_string(),
                ssl_enabled: false,
                ssl_cert_path: None,
                ssl_key_path: None,
            },
            event_streaming: EventStreamingConfig {
                enabled: true,
                buffer_size: 1000,
                event_types: vec![
                    EventType::GitOperation,
                    EventType::ContextChange,
                    EventType::PerformanceEvent,
                ],
                format: StreamFormat::JSON,
            },
            live_dashboards: false,
            realtime_alerts: false,
        },
        alerting: MonitoringAlertingConfig {
            enabled: false,
            channels: Vec::new(),
            rules: Vec::new(),
            severity_levels: vec![AlertSeverity::Warning, AlertSeverity::Error, AlertSeverity::Critical],
        },
        dashboard: DashboardConfig {
            enabled: false,
            server: DashboardServer {
                port: 3000,
                host: "localhost".to_string(),
                auth_enabled: false,
                static_dir: None,
            },
            widgets: Vec::new(),
            auto_refresh: 30,
        },
    }
} 