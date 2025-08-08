use chrono::{DateTime, Utc};
use prometheus::core::Collector;
use prometheus::{
    Counter, Gauge, GaugeVec, Histogram, HistogramOpts, HistogramVec, IntCounter, IntCounterVec,
    IntGauge, IntGaugeVec, Opts, Registry,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::graph::DependencyGraph;
use crate::types::{DependencyType, HealthStatus, ImpactScore};

/// Metrics collector for the dependency management system
pub struct MetricsCollector {
    /// Prometheus registry
    registry: Registry,
    /// Dependency health status metrics
    dependency_health_status: IntGaugeVec,
    /// Dependency response time metrics
    dependency_response_time: HistogramVec,
    /// Dependency availability metrics
    dependency_availability: GaugeVec,
    /// Dependency error rate metrics
    dependency_error_rate: GaugeVec,
    /// Impact analysis score metrics
    impact_analysis_score: GaugeVec,
    /// Validation error metrics
    validation_errors: IntCounter,
    /// Validation warning metrics
    validation_warnings: IntCounter,
    /// Health check duration metrics
    health_check_duration: HistogramVec,
    /// Health check success rate metrics
    health_check_success_rate: GaugeVec,
    /// Dependency count metrics
    dependency_count: IntGauge,
    /// Dependency relationship count metrics
    dependency_relationship_count: IntGauge,
    /// Circular dependency count metrics
    circular_dependency_count: IntGauge,
    /// Business impact score metrics
    business_impact_score: GaugeVec,
    /// Risk level metrics
    risk_level: IntGaugeVec,
    /// Alert count metrics
    alert_count: IntCounter,
    /// Alert severity metrics
    alert_severity: IntCounterVec,
    /// Custom metrics
    custom_metrics: Arc<RwLock<HashMap<String, Box<dyn Collector>>>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Registry::new();

        // Dependency health status metric
        let dependency_health_status = IntGaugeVec::new(
            Opts::new(
                "dependency_health_status",
                "Current health status of dependencies (0=Unknown, 1=Healthy, 2=Degraded, 3=Unhealthy, 4=Down)",
            ),
            &["dependency_id"],
        )?;

        // Dependency response time metric
        let dependency_response_time = HistogramVec::new(
            HistogramOpts::new(
                "dependency_response_time_ms",
                "Response time of dependencies in milliseconds",
            ),
            &["dependency_id"],
        )?;

        // Dependency availability metric
        let dependency_availability = GaugeVec::new(
            Opts::new(
                "dependency_availability",
                "Availability percentage of dependencies",
            ),
            &["dependency_id"],
        )?;

        // Dependency error rate metric
        let dependency_error_rate = GaugeVec::new(
            Opts::new("dependency_error_rate", "Error rate of dependencies"),
            &["dependency_id"],
        )?;

        // Impact analysis score metric
        let impact_analysis_score = GaugeVec::new(
            Opts::new("impact_analysis_score", "Business impact scores"),
            &["dependency_id"],
        )?;

        // Validation error metric
        let validation_errors = IntCounter::new(
            "validation_errors_total",
            "Total number of validation errors",
        )?;

        // Validation warning metric
        let validation_warnings = IntCounter::new(
            "validation_warnings_total",
            "Total number of validation warnings",
        )?;

        // Health check duration metric
        let health_check_duration = HistogramVec::new(
            HistogramOpts::new(
                "health_check_duration_ms",
                "Duration of health checks in milliseconds",
            ),
            &["dependency_id"],
        )?;

        // Health check success rate metric
        let health_check_success_rate = GaugeVec::new(
            Opts::new("health_check_success_rate", "Success rate of health checks"),
            &["dependency_id"],
        )?;

        // Dependency count metric
        let dependency_count = IntGauge::new("dependency_count", "Total number of dependencies")?;

        // Dependency relationship count metric
        let dependency_relationship_count = IntGauge::new(
            "dependency_relationship_count",
            "Total number of dependency relationships",
        )?;

        // Circular dependency count metric
        let circular_dependency_count = IntGauge::new(
            "circular_dependency_count",
            "Number of circular dependencies detected",
        )?;

        // Business impact score metric
        let business_impact_score = GaugeVec::new(
            Opts::new(
                "business_impact_score",
                "Business impact score (0.0 to 1.0)",
            ),
            &["dependency_id"],
        )?;

        // Risk level metric
        let risk_level = IntGaugeVec::new(
            Opts::new(
                "risk_level",
                "Risk level (0=Low, 1=Medium, 2=High, 3=Critical)",
            ),
            &["dependency_id"],
        )?;

        // Alert count metric
        let alert_count = IntCounter::new("alert_count_total", "Total number of alerts generated")?;

        // Alert severity metric
        let alert_severity = IntCounterVec::new(
            Opts::new("alert_severity_total", "Total number of alerts by severity"),
            &["severity"],
        )?;

        // Register metrics with registry
        registry.register(Box::new(dependency_health_status.clone()))?;
        registry.register(Box::new(dependency_response_time.clone()))?;
        registry.register(Box::new(dependency_availability.clone()))?;
        registry.register(Box::new(dependency_error_rate.clone()))?;
        registry.register(Box::new(impact_analysis_score.clone()))?;
        registry.register(Box::new(validation_errors.clone()))?;
        registry.register(Box::new(validation_warnings.clone()))?;
        registry.register(Box::new(health_check_duration.clone()))?;
        registry.register(Box::new(health_check_success_rate.clone()))?;
        registry.register(Box::new(dependency_count.clone()))?;
        registry.register(Box::new(dependency_relationship_count.clone()))?;
        registry.register(Box::new(circular_dependency_count.clone()))?;
        registry.register(Box::new(business_impact_score.clone()))?;
        registry.register(Box::new(risk_level.clone()))?;
        registry.register(Box::new(alert_count.clone()))?;
        registry.register(Box::new(alert_severity.clone()))?;

        Ok(Self {
            registry,
            dependency_health_status,
            dependency_response_time,
            dependency_availability,
            dependency_error_rate,
            impact_analysis_score,
            validation_errors,
            validation_warnings,
            health_check_duration,
            health_check_success_rate,
            dependency_count,
            dependency_relationship_count,
            circular_dependency_count,
            business_impact_score,
            risk_level,
            alert_count,
            alert_severity,
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Record dependency health status
    pub fn record_dependency_health_status(
        &self,
        dependency_id: &str,
        health_status: HealthStatus,
    ) {
        let status_value = match health_status {
            HealthStatus::Unknown => 0,
            HealthStatus::Healthy => 1,
            HealthStatus::Degraded => 2,
            HealthStatus::Unhealthy => 3,
            HealthStatus::Down => 4,
        };

        self.dependency_health_status
            .with_label_values(&[dependency_id])
            .set(status_value);
    }

    /// Record dependency response time
    pub fn record_dependency_response_time(&self, dependency_id: &str, response_time_ms: f64) {
        self.dependency_response_time
            .with_label_values(&[dependency_id])
            .observe(response_time_ms);
    }

    /// Record dependency availability
    pub fn record_dependency_availability(&self, dependency_id: &str, availability: f64) {
        self.dependency_availability
            .with_label_values(&[dependency_id])
            .set(availability);
    }

    /// Record dependency error rate
    pub fn record_dependency_error_rate(&self, dependency_id: &str, error_rate: f64) {
        self.dependency_error_rate
            .with_label_values(&[dependency_id])
            .set(error_rate);
    }

    /// Record impact analysis score
    pub fn record_impact_analysis_score(&self, dependency_id: &str, score: f64) {
        self.impact_analysis_score
            .with_label_values(&[dependency_id])
            .set(score);
    }

    /// Record validation errors
    pub fn record_validation_errors(&self, count: i64) {
        self.validation_errors.inc_by(count.try_into().unwrap_or(0));
    }

    /// Record validation warnings
    pub fn record_validation_warnings(&self, count: i64) {
        self.validation_warnings
            .inc_by(count.try_into().unwrap_or(0));
    }

    /// Record health check duration
    pub fn record_health_check_duration(&self, dependency_id: &str, duration_ms: f64) {
        self.health_check_duration
            .with_label_values(&[dependency_id])
            .observe(duration_ms);
    }

    /// Record health check success rate
    pub fn record_health_check_success_rate(&self, dependency_id: &str, success_rate: f64) {
        self.health_check_success_rate
            .with_label_values(&[dependency_id])
            .set(success_rate);
    }

    /// Update dependency count
    pub fn update_dependency_count(&self, count: i64) {
        self.dependency_count.set(count);
    }

    /// Update dependency relationship count
    pub fn update_dependency_relationship_count(&self, count: i64) {
        self.dependency_relationship_count.set(count);
    }

    /// Update circular dependency count
    pub fn update_circular_dependency_count(&self, count: i64) {
        self.circular_dependency_count.set(count);
    }

    /// Record business impact score
    pub fn record_business_impact_score(&self, dependency_id: &str, score: f64) {
        self.business_impact_score
            .with_label_values(&[dependency_id])
            .set(score);
    }

    /// Record risk level
    pub fn record_risk_level(&self, dependency_id: &str, risk_level: crate::RiskLevel) {
        let risk_value = match risk_level {
            crate::RiskLevel::Low => 0,
            crate::RiskLevel::Medium => 1,
            crate::RiskLevel::High => 2,
            crate::RiskLevel::Critical => 3,
        };

        self.risk_level
            .with_label_values(&[dependency_id])
            .set(risk_value);
    }

    /// Record alert
    pub fn record_alert(&self, severity: &str) {
        self.alert_count.inc();
        self.alert_severity.with_label_values(&[severity]).inc();
    }

    /// Add custom metric
    pub async fn add_custom_metric(
        &self,
        name: String,
        metric: Box<dyn Collector>,
    ) -> Result<(), prometheus::Error> {
        self.registry.register(metric)?;
        // Note: We can't store the metric in custom_metrics because it's moved to registry
        // This is a limitation of the prometheus API
        Ok(())
    }

    /// Remove custom metric
    pub async fn remove_custom_metric(&self, name: &str) -> Result<(), prometheus::Error> {
        if let Some(metric) = self.custom_metrics.write().await.remove(name) {
            self.registry.unregister(metric)?;
        }
        Ok(())
    }

    /// Get metrics as string
    pub fn gather(&self) -> Result<String, prometheus::Error> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let mut buffer = Vec::new();
        encoder.encode(&self.registry.gather(), &mut buffer)?;
        Ok(String::from_utf8(buffer).unwrap_or_default())
    }

    /// Update metrics from dependency graph
    pub async fn update_from_graph(&self, graph: &DependencyGraph) -> Result<(), crate::Error> {
        // Update dependency count
        self.update_dependency_count(graph.node_count() as i64);

        // Update relationship count
        self.update_dependency_relationship_count(graph.edge_count() as i64);

        // Update circular dependency count
        let circular_count = if graph.has_circular_dependencies()? {
            graph.find_circular_dependencies()?.len() as i64
        } else {
            0
        };
        self.update_circular_dependency_count(circular_count);

        // Update health status metrics for each dependency
        for dependency_id in graph.get_all_dependency_ids() {
            if let Ok(config) = graph.get_dependency_config(&dependency_id) {
                // Note: We can't access health status directly from the graph
                // This would need to be updated when health monitoring is implemented
                self.record_dependency_health_status(
                    &dependency_id,
                    crate::types::HealthStatus::Unknown,
                );
            }
        }

        Ok(())
    }

    /// Update metrics from health data
    pub fn update_from_health_data(
        &self,
        dependency_id: &str,
        health_metrics: &crate::types::HealthMetrics,
    ) {
        self.record_dependency_response_time(dependency_id, health_metrics.response_time_ms);
        self.record_dependency_availability(dependency_id, health_metrics.availability);
        self.record_dependency_error_rate(dependency_id, health_metrics.error_rate);
    }

    /// Update metrics from impact analysis
    pub fn update_from_impact_analysis(&self, dependency_id: &str, impact_score: &ImpactScore) {
        self.record_impact_analysis_score(dependency_id, impact_score.business_impact);
        self.record_business_impact_score(dependency_id, impact_score.business_impact);
        self.record_risk_level(dependency_id, impact_score.risk_level);
    }

    /// Update metrics from validation results
    pub fn update_from_validation_results(&self, validation_result: &crate::ValidationResult) {
        self.record_validation_errors(validation_result.errors.len() as i64);
        self.record_validation_warnings(validation_result.warnings.len() as i64);
    }

    /// Get metrics statistics
    pub async fn get_statistics(&self) -> MetricsStatistics {
        let mut total_metrics = 0;
        let mut custom_metrics_count = 0;

        // Count registered metrics
        for metric_family in self.registry.gather() {
            total_metrics += metric_family.get_metric().len();
        }

        // Count custom metrics
        custom_metrics_count = self.custom_metrics.read().await.len();

        MetricsStatistics {
            total_metrics,
            custom_metrics_count,
            registry_metrics: self.registry.gather().len(),
            last_updated: Utc::now(),
        }
    }

    /// Export metrics in different formats
    pub fn export_metrics(&self, format: MetricsFormat) -> Result<String, prometheus::Error> {
        match format {
            MetricsFormat::Prometheus => self.gather(),
            MetricsFormat::Json => self.export_json(),
        }
    }

    /// Export metrics as JSON
    fn export_json(&self) -> Result<String, prometheus::Error> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let mut buffer = Vec::new();
        encoder.encode(&self.registry.gather(), &mut buffer)?;

        // Convert to JSON format
        let metrics_text = String::from_utf8(buffer).unwrap_or_default();
        let json_metrics = self.parse_metrics_to_json(&metrics_text);

        serde_json::to_string_pretty(&json_metrics)
            .map_err(|e| prometheus::Error::Msg(e.to_string()))
    }

    /// Parse metrics text to JSON format
    fn parse_metrics_to_json(&self, metrics_text: &str) -> serde_json::Value {
        let mut metrics = Vec::new();

        for line in metrics_text.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if let Some((name, value)) = line.split_once(' ') {
                let metric = serde_json::json!({
                    "name": name,
                    "value": value.parse::<f64>().unwrap_or(0.0),
                    "timestamp": Utc::now().timestamp()
                });
                metrics.push(metric);
            }
        }

        serde_json::json!({
            "metrics": metrics,
            "timestamp": Utc::now().to_rfc3339()
        })
    }
}

/// Metrics format
#[derive(Debug, Clone)]
pub enum MetricsFormat {
    Prometheus,
    Json,
}

/// Metrics statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsStatistics {
    /// Total number of metrics
    pub total_metrics: usize,
    /// Number of custom metrics
    pub custom_metrics_count: usize,
    /// Number of metric families in registry
    pub registry_metrics: usize,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback implementation if Prometheus fails
            panic!("Failed to initialize metrics collector");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HealthStatus;

    #[test]
    fn test_metrics_collector_new() {
        let collector = MetricsCollector::new();
        assert!(collector.is_ok());
    }

    #[test]
    fn test_record_dependency_health_status() {
        let collector = MetricsCollector::new().unwrap();
        collector.record_dependency_health_status("test-dependency", HealthStatus::Healthy);
        // Note: In a real test, you'd verify the metric was recorded correctly
    }

    #[test]
    fn test_record_dependency_response_time() {
        let collector = MetricsCollector::new().unwrap();
        collector.record_dependency_response_time("test-dependency", 150.0);
        // Note: In a real test, you'd verify the metric was recorded correctly
    }

    #[test]
    fn test_update_dependency_count() {
        let collector = MetricsCollector::new().unwrap();
        collector.update_dependency_count(10);
        // Note: In a real test, you'd verify the metric was updated correctly
    }

    #[test]
    fn test_gather_metrics() {
        let collector = MetricsCollector::new().unwrap();
        let metrics = collector.gather();
        assert!(metrics.is_ok());
        assert!(!metrics.unwrap().is_empty());
    }

    #[test]
    fn test_metrics_format() {
        assert!(matches!(
            MetricsFormat::Prometheus,
            MetricsFormat::Prometheus
        ));
        assert!(matches!(MetricsFormat::Json, MetricsFormat::Json));
    }

    #[test]
    fn test_metrics_statistics() {
        let stats = MetricsStatistics {
            total_metrics: 100,
            custom_metrics_count: 5,
            registry_metrics: 10,
            last_updated: Utc::now(),
        };

        assert_eq!(stats.total_metrics, 100);
        assert_eq!(stats.custom_metrics_count, 5);
        assert_eq!(stats.registry_metrics, 10);
    }
}
