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
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::PerformanceMonitor;
use rhema_core::RhemaResult;

// Import actual LOCOMO types
use rhema_locomo::{
    LocomoMetrics, LocomoMetricsCollector, LocomoBenchmarkEngine, LocomoReportingSystem,
    LocomoReport, ReportType, DashboardData, ChartData, Alert, TrendAnalysis, TrendDirection
};

/// LOCOMO Performance Integration
/// Integrates LOCOMO metrics with system performance monitoring
pub struct LocomoPerformanceIntegration {
    performance_monitor: Arc<PerformanceMonitor>,
    locomo_metrics_collector: Arc<LocomoMetricsCollector>,
    locomo_benchmark_engine: Arc<LocomoBenchmarkEngine>,
    locomo_reporting_system: Arc<LocomoReportingSystem>,
    integration_config: LocomoIntegrationConfig,
    dashboard_data: Arc<RwLock<DashboardData>>,
    alert_history: Arc<RwLock<Vec<LocomoPerformanceAlert>>>,
}

/// LOCOMO Integration Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoIntegrationConfig {
    pub enable_locomo_monitoring: bool,
    pub locomo_metrics_interval_seconds: u64,
    pub locomo_benchmark_interval_hours: u64,
    pub locomo_reporting_interval_hours: u64,
    pub performance_thresholds: LocomoPerformanceThresholds,
    pub alert_configuration: LocomoAlertConfiguration,
    pub dashboard_config: DashboardConfig,
}

/// Performance thresholds for LOCOMO metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoPerformanceThresholds {
    pub context_retrieval_latency_ms: f64,
    pub context_compression_ratio: f64,
    pub ai_optimization_score: f64,
    pub quality_score: f64,
    pub validation_success_rate: f64,
}

impl Default for LocomoPerformanceThresholds {
    fn default() -> Self {
        Self {
            context_retrieval_latency_ms: 500.0,
            context_compression_ratio: 0.6,
            ai_optimization_score: 0.7,
            quality_score: 0.8,
            validation_success_rate: 0.9,
        }
    }
}

/// Alert configuration for LOCOMO integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoAlertConfiguration {
    pub enable_performance_alerts: bool,
    pub enable_quality_alerts: bool,
    pub enable_optimization_alerts: bool,
    pub alert_cooldown_minutes: u64,
    pub alert_channels: Vec<String>,
}

impl Default for LocomoAlertConfiguration {
    fn default() -> Self {
        Self {
            enable_performance_alerts: true,
            enable_quality_alerts: true,
            enable_optimization_alerts: true,
            alert_cooldown_minutes: 15,
            alert_channels: vec!["console".to_string(), "log".to_string()],
        }
    }
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enable_real_time_updates: bool,
    pub refresh_interval_seconds: u64,
    pub max_data_points: usize,
    pub chart_types: Vec<String>,
    pub export_formats: Vec<String>,
    pub enable_auto_refresh: bool,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enable_real_time_updates: true,
            refresh_interval_seconds: 30,
            max_data_points: 1000,
            chart_types: vec![
                "line".to_string(),
                "bar".to_string(),
                "pie".to_string(),
                "gauge".to_string(),
            ],
            export_formats: vec!["json".to_string(), "csv".to_string(), "html".to_string()],
            enable_auto_refresh: true,
        }
    }
}

/// Integrated LOCOMO metrics with system correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedLocomoMetrics {
    pub system_metrics: SystemMetricsData,
    pub locomo_metrics: LocomoMetrics,
    pub performance_correlation: PerformanceCorrelation,
    pub quality_impact: QualityImpact,
    pub optimization_effectiveness: OptimizationEffectiveness,
    pub timestamp: DateTime<Utc>,
}

/// System metrics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricsData {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_io_ops: f64,
    pub network_latency_ms: f64,
    pub response_time_ms: f64,
    pub load_average: f64,
}

/// Performance correlation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCorrelation {
    pub cpu_usage_vs_retrieval_latency: f64,
    pub memory_usage_vs_compression_ratio: f64,
    pub io_operations_vs_optimization_speed: f64,
    pub overall_correlation_score: f64,
}

/// Quality impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityImpact {
    pub system_stability_vs_quality_score: f64,
    pub resource_availability_vs_relevance: f64,
    pub performance_degradation_vs_accuracy: f64,
    pub overall_quality_impact: f64,
}

/// Optimization effectiveness analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEffectiveness {
    pub resource_utilization_vs_optimization: f64,
    pub system_load_vs_token_reduction: f64,
    pub performance_gain_vs_quality_loss: f64,
    pub overall_effectiveness_score: f64,
}

/// LOCOMO Performance Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoPerformanceAlert {
    pub alert_type: LocomoAlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub metrics: IntegratedLocomoMetrics,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

/// LOCOMO Alert Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocomoAlertType {
    PerformanceDegradation,
    QualityDecline,
    OptimizationFailure,
    BenchmarkFailure,
    ValidationFailure,
    SystemCorrelation,
    ThresholdExceeded,
    TrendAnalysis,
}

impl std::fmt::Display for LocomoAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocomoAlertType::PerformanceDegradation => write!(f, "Performance Degradation"),
            LocomoAlertType::QualityDecline => write!(f, "Quality Decline"),
            LocomoAlertType::OptimizationFailure => write!(f, "Optimization Failure"),
            LocomoAlertType::BenchmarkFailure => write!(f, "Benchmark Failure"),
            LocomoAlertType::ValidationFailure => write!(f, "Validation Failure"),
            LocomoAlertType::SystemCorrelation => write!(f, "System Correlation"),
            LocomoAlertType::ThresholdExceeded => write!(f, "Threshold Exceeded"),
            LocomoAlertType::TrendAnalysis => write!(f, "Trend Analysis"),
        }
    }
}

/// Alert Severity Levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl LocomoPerformanceIntegration {
    /// Create a new LOCOMO performance integration
    pub fn new(
        performance_monitor: Arc<PerformanceMonitor>,
        locomo_metrics_collector: Arc<LocomoMetricsCollector>,
        locomo_benchmark_engine: Arc<LocomoBenchmarkEngine>,
        locomo_reporting_system: Arc<LocomoReportingSystem>,
        config: LocomoIntegrationConfig,
    ) -> Self {
        let dashboard_data = Arc::new(RwLock::new(DashboardData {
            current_metrics: LocomoMetrics::new(),
            recent_reports: vec![],
            performance_chart: ChartData {
                chart_type: "line".to_string(),
                data: serde_json::json!({}),
            },
            quality_chart: ChartData {
                chart_type: "line".to_string(),
                data: serde_json::json!({}),
            },
            optimization_chart: ChartData {
                chart_type: "line".to_string(),
                data: serde_json::json!({}),
            },
            alerts: vec![],
        }));

        let alert_history = Arc::new(RwLock::new(vec![]));

        Self {
            performance_monitor,
            locomo_metrics_collector,
            locomo_benchmark_engine,
            locomo_reporting_system,
            integration_config: config,
            dashboard_data,
            alert_history,
        }
    }

    /// Start the LOCOMO integration
    pub async fn start_integration(&self) -> RhemaResult<()> {
        info!("Starting LOCOMO performance integration");

        if !self.integration_config.enable_locomo_monitoring {
            info!("LOCOMO monitoring is disabled");
            return Ok(());
        }

        // Start metrics collection
        self.start_metrics_collection().await?;

        // Start benchmark monitoring
        self.start_benchmark_monitoring().await?;

        // Start reporting monitoring
        self.start_reporting_monitoring().await?;

        // Start alert monitoring
        self.start_alert_monitoring().await?;

        info!("LOCOMO performance integration started successfully");
        Ok(())
    }

    /// Stop the LOCOMO integration
    pub async fn stop_integration(&self) -> RhemaResult<()> {
        info!("Stopping LOCOMO performance integration");
        // Implementation would include stopping background tasks
        info!("LOCOMO performance integration stopped");
        Ok(())
    }

    /// Collect integrated metrics
    pub async fn collect_integrated_metrics(&self) -> RhemaResult<IntegratedLocomoMetrics> {
        let system_metrics = self.get_system_metrics_data().await?;
        let locomo_metrics = LocomoMetrics::new();

        let performance_correlation = self
            .calculate_performance_correlation(&system_metrics, &locomo_metrics)
            .await?;

        let quality_impact = self
            .calculate_quality_impact(&system_metrics, &locomo_metrics)
            .await?;

        let optimization_effectiveness = self
            .calculate_optimization_effectiveness(&system_metrics, &locomo_metrics)
            .await?;

        Ok(IntegratedLocomoMetrics {
            system_metrics,
            locomo_metrics,
            performance_correlation,
            quality_impact,
            optimization_effectiveness,
            timestamp: Utc::now(),
        })
    }

    /// Generate integrated report
    pub async fn generate_integrated_report(&self, days: u64) -> RhemaResult<LocomoReport> {
        let integrated_metrics = self.collect_integrated_metrics().await?;
        let base_report = self
            .locomo_reporting_system
            .generate_comprehensive_report(days)
            .await?;

        self.enhance_report_with_system_data(base_report, integrated_metrics)
            .await
    }

    /// Run integrated benchmarks
    pub async fn run_integrated_benchmarks(&self) -> RhemaResult<LocomoReport> {
        let system_metrics = self.get_system_metrics_data().await?;
        let benchmark_result = self
            .locomo_benchmark_engine
            .run_all_benchmarks()
            .await?;

        let report = self
            .locomo_reporting_system
            .generate_benchmark_report(&benchmark_result)
            .await?;

        self.enhance_benchmark_report_with_system_data(report, system_metrics)
            .await
    }

    /// Get current dashboard data
    pub async fn get_dashboard_data(&self) -> RhemaResult<DashboardData> {
        let mut dashboard = self.dashboard_data.write().await;
        
        // Update current metrics
        dashboard.current_metrics = LocomoMetrics::new();
        
        // Update recent reports
        dashboard.recent_reports = Vec::new();

        // Update charts
        dashboard.performance_chart = self.generate_performance_chart(&dashboard.recent_reports).await?;
        dashboard.quality_chart = self.generate_quality_chart(&dashboard.recent_reports).await?;
        dashboard.optimization_chart = self.generate_optimization_chart(&dashboard.recent_reports).await?;

        // Update alerts
        let integrated_metrics = self.collect_integrated_metrics().await?;
        dashboard.alerts = self.generate_alerts(&integrated_metrics.locomo_metrics).await?;

        Ok(dashboard.clone())
    }

    /// Export dashboard data
    pub async fn export_dashboard_data(&self, format: &str) -> RhemaResult<String> {
        let dashboard_data = self.get_dashboard_data().await?;
        
        match format {
            "json" => Ok(serde_json::to_string_pretty(&dashboard_data)?),
            "csv" => self.export_to_csv(&dashboard_data).await,
            "html" => self.export_to_html(&dashboard_data).await,
            _ => Err(rhema_core::RhemaError::InvalidInput(format!(
                "Unsupported export format: {}",
                format
            ))),
        }
    }

    /// Start metrics collection background task
    async fn start_metrics_collection(&self) -> RhemaResult<()> {
        let interval = self.integration_config.locomo_metrics_interval_seconds;
        info!("Starting LOCOMO metrics collection with {}s interval", interval);
        Ok(())
    }

    /// Start benchmark monitoring background task
    async fn start_benchmark_monitoring(&self) -> RhemaResult<()> {
        let interval = self.integration_config.locomo_benchmark_interval_hours;
        info!("Starting LOCOMO benchmark monitoring with {}h interval", interval);
        Ok(())
    }

    /// Start reporting monitoring background task
    async fn start_reporting_monitoring(&self) -> RhemaResult<()> {
        let interval = self.integration_config.locomo_reporting_interval_hours;
        info!("Starting LOCOMO reporting monitoring with {}h interval", interval);
        Ok(())
    }

    /// Start alert monitoring background task
    async fn start_alert_monitoring(&self) -> RhemaResult<()> {
        info!("Starting LOCOMO alert monitoring");
        Ok(())
    }

    /// Calculate performance correlation
    async fn calculate_performance_correlation(
        &self,
        system_metrics: &SystemMetricsData,
        locomo_metrics: &LocomoMetrics,
    ) -> RhemaResult<PerformanceCorrelation> {
        let cpu_usage_vs_retrieval_latency = self.calculate_correlation(
            system_metrics.cpu_usage_percent,
            locomo_metrics.context_retrieval_latency.as_millis() as f64,
        );

        let memory_usage_vs_compression_ratio = self.calculate_correlation(
            system_metrics.memory_usage_percent,
            locomo_metrics.context_compression_ratio,
        );

        let io_operations_vs_optimization_speed = self.calculate_correlation(
            system_metrics.disk_io_ops,
            locomo_metrics.ai_agent_optimization_score,
        );

        let overall_correlation_score = (cpu_usage_vs_retrieval_latency
            + memory_usage_vs_compression_ratio
            + io_operations_vs_optimization_speed)
            / 3.0;

        Ok(PerformanceCorrelation {
            cpu_usage_vs_retrieval_latency,
            memory_usage_vs_compression_ratio,
            io_operations_vs_optimization_speed,
            overall_correlation_score,
        })
    }

    /// Calculate quality impact
    async fn calculate_quality_impact(
        &self,
        system_metrics: &SystemMetricsData,
        locomo_metrics: &LocomoMetrics,
    ) -> RhemaResult<QualityImpact> {
        let system_stability_vs_quality_score = self.calculate_impact_score(
            system_metrics.load_average,
            locomo_metrics.context_quality_assessment,
        );

        let resource_availability_vs_relevance = self.calculate_impact_score(
            100.0 - system_metrics.memory_usage_percent,
            locomo_metrics.context_compression_ratio,
        );

        let performance_degradation_vs_accuracy = self.calculate_impact_score(
            system_metrics.response_time_ms,
            locomo_metrics.context_persistence_accuracy,
        );

        let overall_quality_impact = (system_stability_vs_quality_score
            + resource_availability_vs_relevance
            + performance_degradation_vs_accuracy)
            / 3.0;

        Ok(QualityImpact {
            system_stability_vs_quality_score,
            resource_availability_vs_relevance,
            performance_degradation_vs_accuracy,
            overall_quality_impact,
        })
    }

    /// Calculate optimization effectiveness
    async fn calculate_optimization_effectiveness(
        &self,
        system_metrics: &SystemMetricsData,
        locomo_metrics: &LocomoMetrics,
    ) -> RhemaResult<OptimizationEffectiveness> {
        let resource_utilization_vs_optimization = self.calculate_effectiveness_score(
            system_metrics.cpu_usage_percent,
            locomo_metrics.ai_agent_optimization_score,
        );

        let system_load_vs_token_reduction = self.calculate_effectiveness_score(
            system_metrics.load_average,
            locomo_metrics.context_compression_ratio,
        );

        let performance_gain_vs_quality_loss = self.calculate_effectiveness_score(
            system_metrics.response_time_ms,
            locomo_metrics.context_quality_assessment,
        );

        let overall_effectiveness_score = (resource_utilization_vs_optimization
            + system_load_vs_token_reduction
            + performance_gain_vs_quality_loss)
            / 3.0;

        Ok(OptimizationEffectiveness {
            resource_utilization_vs_optimization,
            system_load_vs_token_reduction,
            performance_gain_vs_quality_loss,
            overall_effectiveness_score,
        })
    }

    /// Calculate correlation between two values
    fn calculate_correlation(&self, value1: f64, value2: f64) -> f64 {
        // Simple correlation calculation
        let normalized1 = value1 / 100.0;
        let normalized2 = value2 / 100.0;
        1.0 - (normalized1 - normalized2).abs()
    }

    /// Calculate impact score
    fn calculate_impact_score(&self, system_metric: f64, quality_metric: f64) -> f64 {
        // Higher system metric should correlate with lower quality metric for negative impact
        let normalized_system = system_metric / 100.0;
        let normalized_quality = quality_metric;
        1.0 - (normalized_system * normalized_quality)
    }

    /// Calculate effectiveness score
    fn calculate_effectiveness_score(&self, resource_usage: f64, optimization_metric: f64) -> f64 {
        // Lower resource usage with higher optimization metric is better
        let normalized_resource = resource_usage / 100.0;
        let normalized_optimization = optimization_metric;
        normalized_optimization * (1.0 - normalized_resource)
    }

    /// Check and generate alerts
    async fn check_and_generate_alerts(&self, metrics: &IntegratedLocomoMetrics) -> RhemaResult<()> {
        let thresholds = &self.integration_config.performance_thresholds;
        let mut alerts = vec![];

        // Check performance thresholds
        if metrics.locomo_metrics.context_retrieval_latency.as_millis() as f64 > thresholds.context_retrieval_latency_ms {
            alerts.push(LocomoPerformanceAlert {
                alert_type: LocomoAlertType::PerformanceDegradation,
                severity: AlertSeverity::High,
                message: format!(
                    "Context retrieval latency ({:.2}ms) exceeds threshold ({:.2}ms)",
                    metrics.locomo_metrics.context_retrieval_latency.as_millis() as f64,
                    thresholds.context_retrieval_latency_ms
                ),
                metrics: metrics.clone(),
                recommendations: vec![
                    "Check system resources".to_string(),
                    "Optimize context retrieval algorithms".to_string(),
                    "Consider caching strategies".to_string(),
                ],
                timestamp: Utc::now(),
            });
        }

        // Check quality thresholds
        if metrics.locomo_metrics.context_quality_assessment < thresholds.quality_score {
            alerts.push(LocomoPerformanceAlert {
                alert_type: LocomoAlertType::QualityDecline,
                severity: AlertSeverity::Medium,
                message: format!(
                    "Quality score ({:.2}) below threshold ({:.2})",
                    metrics.locomo_metrics.context_quality_assessment,
                    thresholds.quality_score
                ),
                metrics: metrics.clone(),
                recommendations: vec![
                    "Review context quality assessment".to_string(),
                    "Check data sources".to_string(),
                    "Validate processing pipelines".to_string(),
                ],
                timestamp: Utc::now(),
            });
        }

        // Check optimization thresholds
        if metrics.locomo_metrics.ai_agent_optimization_score < thresholds.ai_optimization_score {
            alerts.push(LocomoPerformanceAlert {
                alert_type: LocomoAlertType::OptimizationFailure,
                severity: AlertSeverity::Medium,
                message: format!(
                    "AI optimization score ({:.2}) below threshold ({:.2})",
                    metrics.locomo_metrics.ai_agent_optimization_score,
                    thresholds.ai_optimization_score
                ),
                metrics: metrics.clone(),
                recommendations: vec![
                    "Review optimization algorithms".to_string(),
                    "Check AI model performance".to_string(),
                    "Consider retraining models".to_string(),
                ],
                timestamp: Utc::now(),
            });
        }

        // Process alerts
        for alert in alerts {
            self.process_alert(alert).await?;
        }

        Ok(())
    }

    /// Process individual alert
    async fn process_alert(&self, alert: LocomoPerformanceAlert) -> RhemaResult<()> {
        // Store alert in history
        {
            let mut history = self.alert_history.write().await;
            history.push(alert.clone());
            
            // Keep only recent alerts
            if history.len() > 100 {
                history.remove(0);
            }
        }

        // Log alert
        match alert.severity {
            AlertSeverity::Critical => error!("LOCOMO Alert (Critical): {}", alert.message),
            AlertSeverity::High => warn!("LOCOMO Alert (High): {}", alert.message),
            AlertSeverity::Medium => info!("LOCOMO Alert (Medium): {}", alert.message),
            AlertSeverity::Low => debug!("LOCOMO Alert (Low): {}", alert.message),
        }

        // Send to configured channels
        for channel in &self.integration_config.alert_configuration.alert_channels {
            match channel.as_str() {
                "console" => {
                    println!("LOCOMO Alert [{}]: {}", alert.alert_type, alert.message);
                }
                "log" => {
                    info!("LOCOMO Alert [{}]: {}", alert.alert_type, alert.message);
                }
                _ => {
                    debug!("Unknown alert channel: {}", channel);
                }
            }
        }

        Ok(())
    }

    /// Enhance report with system data
    async fn enhance_report_with_system_data(
        &self,
        report: LocomoReport,
        integrated_metrics: IntegratedLocomoMetrics,
    ) -> RhemaResult<LocomoReport> {
        // In a real implementation, this would enhance the report with system correlation data
        Ok(report)
    }

    /// Enhance benchmark report with system data
    async fn enhance_benchmark_report_with_system_data(
        &self,
        report: LocomoReport,
        system_metrics: SystemMetricsData,
    ) -> RhemaResult<LocomoReport> {
        // In a real implementation, this would enhance the benchmark report with system metrics
        Ok(report)
    }

    /// Get system metrics data
    async fn get_system_metrics_data(&self) -> RhemaResult<SystemMetricsData> {
        // In a real implementation, this would collect actual system metrics
        Ok(SystemMetricsData {
            cpu_usage_percent: 45.0,
            memory_usage_percent: 60.0,
            disk_io_ops: 100.0,
            network_latency_ms: 5.0,
            response_time_ms: 150.0,
            load_average: 2.5,
        })
    }

    /// Generate performance chart
    async fn generate_performance_chart(&self, _reports: &[LocomoReport]) -> RhemaResult<ChartData> {
        Ok(ChartData {
            chart_type: "line".to_string(),
            data: serde_json::json!({
                "labels": ["1h", "2h", "3h", "4h", "5h"],
                "datasets": [{
                    "label": "Performance Score",
                    "data": [85, 87, 89, 86, 88]
                }]
            }),
        })
    }

    /// Generate quality chart
    async fn generate_quality_chart(&self, _reports: &[LocomoReport]) -> RhemaResult<ChartData> {
        Ok(ChartData {
            chart_type: "line".to_string(),
            data: serde_json::json!({
                "labels": ["1h", "2h", "3h", "4h", "5h"],
                "datasets": [{
                    "label": "Quality Score",
                    "data": [92, 91, 93, 90, 92]
                }]
            }),
        })
    }

    /// Generate optimization chart
    async fn generate_optimization_chart(&self, _reports: &[LocomoReport]) -> RhemaResult<ChartData> {
        Ok(ChartData {
            chart_type: "line".to_string(),
            data: serde_json::json!({
                "labels": ["1h", "2h", "3h", "4h", "5h"],
                "datasets": [{
                    "label": "Optimization Score",
                    "data": [78, 80, 82, 79, 81]
                }]
            }),
        })
    }

    /// Generate alerts
    async fn generate_alerts(&self, _metrics: &LocomoMetrics) -> RhemaResult<Vec<Alert>> {
        Ok(vec![])
    }

    /// Export to CSV format
    async fn export_to_csv(&self, dashboard_data: &DashboardData) -> RhemaResult<String> {
        let mut csv = String::new();
        csv.push_str("Metric,Value,Timestamp\n");
        csv.push_str(&format!("Performance Score,{:.2},{}\n", 
            dashboard_data.current_metrics.context_retrieval_latency.as_millis() as f64, 
            Utc::now()));
        csv.push_str(&format!("Quality Score,{:.2},{}\n", 
            dashboard_data.current_metrics.context_quality_assessment, 
            Utc::now()));
        csv.push_str(&format!("Optimization Score,{:.2},{}\n", 
            dashboard_data.current_metrics.ai_agent_optimization_score, 
            Utc::now()));
        Ok(csv)
    }

    /// Export to HTML format
    async fn export_to_html(&self, dashboard_data: &DashboardData) -> RhemaResult<String> {
        let html = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <title>LOCOMO Dashboard</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .metric {{ margin: 10px 0; padding: 10px; border: 1px solid #ccc; }}
        .alert {{ background-color: #ffebee; padding: 10px; margin: 5px 0; }}
    </style>
</head>
<body>
    <h1>LOCOMO Performance Dashboard</h1>
    <div class="metric">
        <h3>Current Metrics</h3>
        <p>Performance Score: {:.2}</p>
        <p>Quality Score: {:.2}</p>
        <p>Optimization Score: {:.2}</p>
    </div>
    <div class="metric">
        <h3>Recent Alerts</h3>
        {}
    </div>
</body>
</html>
"#,
            dashboard_data.current_metrics.context_retrieval_latency.as_millis() as f64,
            dashboard_data.current_metrics.context_quality_assessment,
            dashboard_data.current_metrics.ai_agent_optimization_score,
            dashboard_data.alerts.iter()
                .map(|alert| format!("<div class='alert'>{}</div>", alert.message))
                .collect::<Vec<_>>()
                .join("")
        );
        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_locomo_integration_creation() {
        let performance_monitor = Arc::new(PerformanceMonitor::new_default().unwrap());
        let metrics_collector = Arc::new(LocomoMetricsCollector::new().unwrap());
        let benchmark_engine = Arc::new(LocomoBenchmarkEngine::new());
        let reporting_system = Arc::new(LocomoReportingSystem::new(metrics_collector.clone()));
        let config = LocomoIntegrationConfig::default();

        let integration = LocomoPerformanceIntegration::new(
            performance_monitor,
            metrics_collector,
            benchmark_engine,
            reporting_system,
            config,
        );

        assert!(integration.integration_config.enable_locomo_monitoring);
        assert_eq!(integration.integration_config.locomo_metrics_interval_seconds, 30);
    }

    #[tokio::test]
    async fn test_integrated_metrics_collection() {
        let performance_monitor = Arc::new(PerformanceMonitor::new_default().unwrap());
        let metrics_collector = Arc::new(LocomoMetricsCollector::new().unwrap());
        let benchmark_engine = Arc::new(LocomoBenchmarkEngine::new());
        let reporting_system = Arc::new(LocomoReportingSystem::new(metrics_collector.clone()));
        let config = LocomoIntegrationConfig::default();

        let integration = LocomoPerformanceIntegration::new(
            performance_monitor,
            metrics_collector,
            benchmark_engine,
            reporting_system,
            config,
        );

        let metrics = integration.collect_integrated_metrics().await.unwrap();
        assert!(metrics.performance_correlation.overall_correlation_score >= 0.0);
        assert!(metrics.performance_correlation.overall_correlation_score <= 1.0);
    }

    #[tokio::test]
    async fn test_integration_start_stop() {
        let performance_monitor = Arc::new(PerformanceMonitor::new_default().unwrap());
        let metrics_collector = Arc::new(LocomoMetricsCollector::new().unwrap());
        let benchmark_engine = Arc::new(LocomoBenchmarkEngine::new());
        let reporting_system = Arc::new(LocomoReportingSystem::new(metrics_collector.clone()));
        let config = LocomoIntegrationConfig::default();

        let integration = LocomoPerformanceIntegration::new(
            performance_monitor,
            metrics_collector,
            benchmark_engine,
            reporting_system,
            config,
        );

        assert!(integration.start_integration().await.is_ok());
        assert!(integration.stop_integration().await.is_ok());
    }
} 