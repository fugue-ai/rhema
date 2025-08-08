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
use prometheus::{Counter, Gauge, Histogram, HistogramOpts};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::types::{BenchmarkMetrics, LocomoError, PerformanceMetrics, QualityMetrics};
use rhema_core::RhemaResult;

/// LOCOMO metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoMetrics {
    pub context_retrieval_latency: Duration,
    pub context_relevance_score: f64,
    pub context_compression_ratio: f64,
    pub cross_scope_integration_quality: f64,
    pub context_persistence_accuracy: f64,
    pub ai_agent_optimization_score: f64,
    pub context_quality_assessment: f64,
    pub context_evolution_tracking: f64,
}

impl LocomoMetrics {
    pub fn new() -> Self {
        Self {
            context_retrieval_latency: Duration::from_millis(0),
            context_relevance_score: 0.0,
            context_compression_ratio: 1.0,
            cross_scope_integration_quality: 0.0,
            context_persistence_accuracy: 0.0,
            ai_agent_optimization_score: 0.0,
            context_quality_assessment: 0.0,
            context_evolution_tracking: 0.0,
        }
    }

    pub fn from_benchmark_metrics(metrics: &BenchmarkMetrics) -> Self {
        Self {
            context_retrieval_latency: metrics.context_retrieval_latency,
            context_relevance_score: metrics.context_relevance_score,
            context_compression_ratio: metrics.context_compression_ratio,
            cross_scope_integration_quality: metrics.cross_scope_integration_quality,
            context_persistence_accuracy: metrics.context_persistence_accuracy,
            ai_agent_optimization_score: metrics.ai_agent_optimization_score,
            context_quality_assessment: metrics.context_quality_assessment,
            context_evolution_tracking: metrics.context_evolution_tracking,
        }
    }

    pub fn overall_score(&self) -> f64 {
        let scores = vec![
            self.context_relevance_score,
            self.context_compression_ratio,
            self.cross_scope_integration_quality,
            self.context_persistence_accuracy,
            self.ai_agent_optimization_score,
            self.context_quality_assessment,
            self.context_evolution_tracking,
        ];

        scores.iter().sum::<f64>() / scores.len() as f64
    }
}

/// LOCOMO metrics collector
pub struct LocomoMetricsCollector {
    metrics: Arc<LocomoMetricsStore>,
    prometheus_metrics: Arc<LocomoPrometheusMetrics>,
}

/// LOCOMO metrics store
#[derive(Clone)]
pub struct LocomoMetricsStore {
    pub context_retrieval_latency: Histogram,
    pub context_relevance_score: Gauge,
    pub context_compression_ratio: Gauge,
    pub cross_scope_integration_quality: Gauge,
    pub context_persistence_accuracy: Gauge,
    pub ai_agent_optimization_score: Gauge,
    pub context_quality_assessment: Gauge,
    pub context_evolution_tracking: Gauge,
    pub overall_score: Gauge,
    pub benchmark_count: Counter,
    pub error_count: Counter,
}

/// Prometheus metrics for LOCOMO
#[derive(Clone)]
pub struct LocomoPrometheusMetrics {
    pub context_retrieval_latency: Histogram,
    pub context_relevance_score: Gauge,
    pub context_compression_ratio: Gauge,
    pub cross_scope_integration_quality: Gauge,
    pub context_persistence_accuracy: Gauge,
    pub ai_agent_optimization_score: Gauge,
    pub context_quality_assessment: Gauge,
    pub context_evolution_tracking: Gauge,
    pub overall_score: Gauge,
    pub benchmark_count: Counter,
    pub error_count: Counter,
}

impl LocomoMetricsCollector {
    pub fn new() -> RhemaResult<Self> {
        let prometheus_metrics = Arc::new(LocomoPrometheusMetrics {
            context_retrieval_latency: Histogram::with_opts(HistogramOpts::new(
                "locomo_context_retrieval_latency_seconds",
                "Context retrieval latency in seconds",
            ))?,
            context_relevance_score: Gauge::new(
                "locomo_context_relevance_score",
                "Context relevance score (0-1)",
            )?,
            context_compression_ratio: Gauge::new(
                "locomo_context_compression_ratio",
                "Context compression ratio (0-1)",
            )?,
            cross_scope_integration_quality: Gauge::new(
                "locomo_cross_scope_integration_quality",
                "Cross-scope integration quality (0-1)",
            )?,
            context_persistence_accuracy: Gauge::new(
                "locomo_context_persistence_accuracy",
                "Context persistence accuracy (0-1)",
            )?,
            ai_agent_optimization_score: Gauge::new(
                "locomo_ai_agent_optimization_score",
                "AI agent optimization score (0-1)",
            )?,
            context_quality_assessment: Gauge::new(
                "locomo_context_quality_assessment",
                "Context quality assessment (0-1)",
            )?,
            context_evolution_tracking: Gauge::new(
                "locomo_context_evolution_tracking",
                "Context evolution tracking (0-1)",
            )?,
            overall_score: Gauge::new("locomo_overall_score", "Overall LOCOMO score (0-1)")?,
            benchmark_count: Counter::new(
                "locomo_benchmark_count_total",
                "Total number of benchmarks run",
            )?,
            error_count: Counter::new(
                "locomo_error_count_total",
                "Total number of benchmark errors",
            )?,
        });

        let metrics = Arc::new(LocomoMetricsStore {
            context_retrieval_latency: prometheus_metrics.context_retrieval_latency.clone(),
            context_relevance_score: prometheus_metrics.context_relevance_score.clone(),
            context_compression_ratio: prometheus_metrics.context_compression_ratio.clone(),
            cross_scope_integration_quality: prometheus_metrics
                .cross_scope_integration_quality
                .clone(),
            context_persistence_accuracy: prometheus_metrics.context_persistence_accuracy.clone(),
            ai_agent_optimization_score: prometheus_metrics.ai_agent_optimization_score.clone(),
            context_quality_assessment: prometheus_metrics.context_quality_assessment.clone(),
            context_evolution_tracking: prometheus_metrics.context_evolution_tracking.clone(),
            overall_score: prometheus_metrics.overall_score.clone(),
            benchmark_count: prometheus_metrics.benchmark_count.clone(),
            error_count: prometheus_metrics.error_count.clone(),
        });

        Ok(Self {
            metrics,
            prometheus_metrics,
        })
    }

    pub async fn record_metrics(&self, metrics: &LocomoMetrics) -> RhemaResult<()> {
        debug!("Recording LOCOMO metrics: {:?}", metrics);

        // Record Prometheus metrics
        self.metrics
            .context_retrieval_latency
            .observe(metrics.context_retrieval_latency.as_secs_f64());
        self.metrics
            .context_relevance_score
            .set(metrics.context_relevance_score);
        self.metrics
            .context_compression_ratio
            .set(metrics.context_compression_ratio);
        self.metrics
            .cross_scope_integration_quality
            .set(metrics.cross_scope_integration_quality);
        self.metrics
            .context_persistence_accuracy
            .set(metrics.context_persistence_accuracy);
        self.metrics
            .ai_agent_optimization_score
            .set(metrics.ai_agent_optimization_score);
        self.metrics
            .context_quality_assessment
            .set(metrics.context_quality_assessment);
        self.metrics
            .context_evolution_tracking
            .set(metrics.context_evolution_tracking);
        self.metrics.overall_score.set(metrics.overall_score());

        // Increment benchmark count
        self.metrics.benchmark_count.inc();

        info!("LOCOMO metrics recorded successfully");
        Ok(())
    }

    pub async fn record_error(&self, error: &str) -> RhemaResult<()> {
        error!("LOCOMO benchmark error: {}", error);
        self.metrics.error_count.inc();
        Ok(())
    }

    pub fn get_prometheus_metrics(&self) -> Arc<LocomoPrometheusMetrics> {
        self.prometheus_metrics.clone()
    }
}

/// LOCOMO performance analyzer
pub struct LocomoPerformanceAnalyzer {
    historical_metrics: Arc<RwLock<Vec<HistoricalMetric>>>,
    config: PerformanceAnalyzerConfig,
}

/// Historical metric data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMetric {
    pub timestamp: DateTime<Utc>,
    pub metrics: LocomoMetrics,
    pub benchmark_name: String,
    pub scenario_name: String,
}

/// Performance analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalyzerConfig {
    pub retention_days: u32,
    pub analysis_window_hours: u64,
    pub trend_detection_enabled: bool,
    pub anomaly_detection_enabled: bool,
}

impl Default for PerformanceAnalyzerConfig {
    fn default() -> Self {
        Self {
            retention_days: 30,
            analysis_window_hours: 24,
            trend_detection_enabled: true,
            anomaly_detection_enabled: true,
        }
    }
}

impl LocomoPerformanceAnalyzer {
    pub fn new(config: PerformanceAnalyzerConfig) -> Self {
        Self {
            historical_metrics: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }

    pub async fn add_metric(&self, metric: HistoricalMetric) -> RhemaResult<()> {
        let mut metrics = self.historical_metrics.write().await;
        metrics.push(metric);

        // Cleanup old metrics
        self.cleanup_old_metrics(&mut metrics).await;

        Ok(())
    }

    pub async fn analyze_trends(&self) -> RhemaResult<TrendAnalysis> {
        let metrics = self.historical_metrics.read().await;
        let recent_metrics = self.get_recent_metrics(&metrics).await;

        let trends = self.calculate_trends(&recent_metrics).await;
        let anomalies = if self.config.anomaly_detection_enabled {
            self.detect_anomalies(&recent_metrics).await
        } else {
            Vec::new()
        };

        Ok(TrendAnalysis {
            trends,
            anomalies,
            analysis_window: self.config.analysis_window_hours,
            timestamp: Utc::now(),
        })
    }

    async fn get_recent_metrics<'a>(
        &self,
        metrics: &'a [HistoricalMetric],
    ) -> Vec<&'a HistoricalMetric> {
        let cutoff = Utc::now() - chrono::Duration::hours(self.config.analysis_window_hours as i64);
        metrics.iter().filter(|m| m.timestamp >= cutoff).collect()
    }

    async fn calculate_trends(&self, metrics: &[&HistoricalMetric]) -> Vec<MetricTrend> {
        if metrics.len() < 2 {
            return Vec::new();
        }

        let mut trends = Vec::new();

        // Calculate trends for each metric
        trends.push(
            self.calculate_trend_for_metric(
                metrics,
                |m| m.context_relevance_score,
                "context_relevance_score",
            )
            .await,
        );
        trends.push(
            self.calculate_trend_for_metric(
                metrics,
                |m| m.context_compression_ratio,
                "context_compression_ratio",
            )
            .await,
        );
        trends.push(
            self.calculate_trend_for_metric(
                metrics,
                |m| m.ai_agent_optimization_score,
                "ai_agent_optimization_score",
            )
            .await,
        );
        trends.push(
            self.calculate_trend_for_metric(metrics, |m| m.overall_score(), "overall_score")
                .await,
        );

        trends
    }

    async fn calculate_trend_for_metric<F>(
        &self,
        metrics: &[&HistoricalMetric],
        extractor: F,
        metric_name: &str,
    ) -> MetricTrend
    where
        F: Fn(&LocomoMetrics) -> f64,
    {
        let values: Vec<f64> = metrics.iter().map(|m| extractor(&m.metrics)).collect();
        let first_value = values.first().unwrap_or(&0.0);
        let last_value = values.last().unwrap_or(&0.0);
        let change_percentage = if *first_value > 0.0 {
            ((last_value - first_value) / first_value) * 100.0
        } else {
            0.0
        };

        let direction = if change_percentage > 1.0 {
            TrendDirection::Improving
        } else if change_percentage < -1.0 {
            TrendDirection::Declining
        } else {
            TrendDirection::Stable
        };

        MetricTrend {
            metric_name: metric_name.to_string(),
            direction,
            change_percentage,
            first_value: *first_value,
            last_value: *last_value,
            data_points: values.len(),
        }
    }

    async fn detect_anomalies(&self, metrics: &[&HistoricalMetric]) -> Vec<MetricAnomaly> {
        let mut anomalies = Vec::new();

        // Simple anomaly detection based on standard deviation
        for metric_name in &[
            "context_relevance_score",
            "ai_agent_optimization_score",
            "overall_score",
        ] {
            let values: Vec<f64> = metrics
                .iter()
                .map(|m| match *metric_name {
                    "context_relevance_score" => m.metrics.context_relevance_score,
                    "ai_agent_optimization_score" => m.metrics.ai_agent_optimization_score,
                    "overall_score" => m.metrics.overall_score(),
                    _ => 0.0,
                })
                .collect();

            if values.len() > 1 {
                let mean = values.iter().sum::<f64>() / values.len() as f64;
                let variance =
                    values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
                let std_dev = variance.sqrt();

                for (i, &value) in values.iter().enumerate() {
                    if (value - mean).abs() > 2.0 * std_dev {
                        anomalies.push(MetricAnomaly {
                            metric_name: metric_name.to_string(),
                            value,
                            expected_range: (mean - 2.0 * std_dev, mean + 2.0 * std_dev),
                            timestamp: metrics[i].timestamp,
                            severity: AnomalySeverity::High,
                        });
                    }
                }
            }
        }

        anomalies
    }

    async fn cleanup_old_metrics(&self, metrics: &mut Vec<HistoricalMetric>) {
        let cutoff = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
        metrics.retain(|m| m.timestamp >= cutoff);
    }
}

/// Trend analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub trends: Vec<MetricTrend>,
    pub anomalies: Vec<MetricAnomaly>,
    pub analysis_window: u64,
    pub timestamp: DateTime<Utc>,
}

/// Metric trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTrend {
    pub metric_name: String,
    pub direction: TrendDirection,
    pub change_percentage: f64,
    pub first_value: f64,
    pub last_value: f64,
    pub data_points: usize,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Declining,
    Stable,
}

/// Metric anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAnomaly {
    pub metric_name: String,
    pub value: f64,
    pub expected_range: (f64, f64),
    pub timestamp: DateTime<Utc>,
    pub severity: AnomalySeverity,
}

/// Anomaly severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Context retrieval metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRetrievalMetrics {
    pub latency: Duration,
    pub relevance_score: f64,
    pub cache_hit_rate: f64,
    pub query_complexity: f64,
    pub scope_coverage: f64,
}

/// Context compression metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCompressionMetrics {
    pub compression_ratio: f64,
    pub quality_loss: f64,
    pub compression_speed: Duration,
    pub decompression_speed: Duration,
    pub algorithm_efficiency: f64,
}

/// AI optimization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIOptimizationMetrics {
    pub token_reduction: f64,
    pub response_quality: f64,
    pub context_fidelity: f64,
    pub optimization_speed: Duration,
    pub ai_satisfaction_score: f64,
}

/// LOCOMO benchmark metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoBenchmarkMetrics {
    pub retrieval: ContextRetrievalMetrics,
    pub compression: ContextCompressionMetrics,
    pub ai_optimization: AIOptimizationMetrics,
    pub overall_score: f64,
    pub timestamp: DateTime<Utc>,
}
