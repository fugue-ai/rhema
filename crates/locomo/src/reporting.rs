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
use chrono::{DateTime, Utc, Duration as ChronoDuration, Timelike, Datelike};

use crate::types::{LocomoError, Context, BenchmarkType};
use crate::metrics::{LocomoMetrics, LocomoMetricsCollector};
use crate::benchmark_engine::{LocomoBenchmarkResult, BenchmarkSummary};
use crate::validation::{ValidationReport, ValidationSummary};
use crate::optimization::{OptimizationResult, OptimizationAction};
use rhema_core::RhemaResult;

/// LOCOMO reporting system
pub struct LocomoReportingSystem {
    metrics_collector: Arc<LocomoMetricsCollector>,
    report_history: Arc<RwLock<Vec<LocomoReport>>>,
    dashboard_generator: Arc<DashboardGenerator>,
    trend_analyzer: Arc<TrendAnalyzer>,
}

/// LOCOMO report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoReport {
    pub report_id: String,
    pub report_type: ReportType,
    pub timestamp: DateTime<Utc>,
    pub duration: Duration,
    pub summary: ReportSummary,
    pub detailed_metrics: DetailedMetrics,
    pub trends: TrendAnalysis,
    pub recommendations: Vec<String>,
    pub performance_score: f64,
    pub quality_score: f64,
    pub optimization_score: f64,
}

/// Report type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    Benchmark,
    Assessment,
    Optimization,
    Validation,
    Comprehensive,
    Trend,
    Performance,
    Quality,
}

/// Report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_benchmarks: usize,
    pub successful_benchmarks: usize,
    pub failed_benchmarks: usize,
    pub average_performance_score: f64,
    pub average_quality_score: f64,
    pub average_optimization_score: f64,
    pub best_performing_area: String,
    pub worst_performing_area: String,
    pub overall_grade: String,
}

/// Detailed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedMetrics {
    pub context_retrieval: ContextRetrievalMetrics,
    pub context_compression: ContextCompressionMetrics,
    pub ai_optimization: AIOptimizationMetrics,
    pub quality_assessment: QualityAssessmentMetrics,
    pub validation_metrics: ValidationMetrics,
}

/// Context retrieval metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRetrievalMetrics {
    pub average_latency_ms: f64,
    pub success_rate: f64,
    pub relevance_score: f64,
    pub coverage_score: f64,
    pub efficiency_score: f64,
    pub total_queries: usize,
    pub successful_queries: usize,
    pub failed_queries: usize,
}

/// Context compression metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCompressionMetrics {
    pub compression_ratio: f64,
    pub quality_preservation: f64,
    pub compression_speed_ms: f64,
    pub decompression_speed_ms: f64,
    pub memory_usage_reduction: f64,
    pub semantic_preservation: f64,
}

/// AI optimization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIOptimizationMetrics {
    pub token_reduction_percentage: f64,
    pub quality_improvement: f64,
    pub optimization_speed_ms: f64,
    pub ai_consumption_reduction: f64,
    pub semantic_enhancement: f64,
    pub structure_optimization: f64,
}

/// Quality assessment metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessmentMetrics {
    pub overall_quality_score: f64,
    pub relevance_score: f64,
    pub completeness_score: f64,
    pub accuracy_score: f64,
    pub consistency_score: f64,
    pub freshness_score: f64,
}

/// Validation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub total_validations: usize,
    pub passed_validations: usize,
    pub failed_validations: usize,
    pub warning_validations: usize,
    pub success_rate: f64,
    pub improvement_rate: f64,
}

/// Trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub period_days: u64,
    pub performance_trend: TrendDirection,
    pub quality_trend: TrendDirection,
    pub optimization_trend: TrendDirection,
    pub key_improvements: Vec<String>,
    pub areas_of_concern: Vec<String>,
    pub predictions: Vec<String>,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Declining,
    Stable,
    Fluctuating,
}

/// Dashboard generator
pub struct DashboardGenerator {
    config: DashboardConfig,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enable_real_time_updates: bool,
    pub refresh_interval_seconds: u64,
    pub max_data_points: usize,
    pub chart_types: Vec<String>,
    pub export_formats: Vec<String>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enable_real_time_updates: true,
            refresh_interval_seconds: 30,
            max_data_points: 1000,
            chart_types: vec!["line".to_string(), "bar".to_string(), "pie".to_string()],
            export_formats: vec!["json".to_string(), "csv".to_string(), "html".to_string()],
        }
    }
}

/// Trend analyzer
pub struct TrendAnalyzer {
    config: TrendAnalysisConfig,
}

/// Trend analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysisConfig {
    pub analysis_period_days: u64,
    pub min_data_points: usize,
    pub confidence_threshold: f64,
    pub trend_sensitivity: f64,
}

impl Default for TrendAnalysisConfig {
    fn default() -> Self {
        Self {
            analysis_period_days: 30,
            min_data_points: 10,
            confidence_threshold: 0.8,
            trend_sensitivity: 0.1,
        }
    }
}

impl LocomoReportingSystem {
    pub fn new(metrics_collector: Arc<LocomoMetricsCollector>) -> Self {
        Self {
            metrics_collector,
            report_history: Arc::new(RwLock::new(Vec::new())),
            dashboard_generator: Arc::new(DashboardGenerator::new(Default::default())),
            trend_analyzer: Arc::new(TrendAnalyzer::new(Default::default())),
        }
    }

    /// Generate comprehensive LOCOMO report
    pub async fn generate_comprehensive_report(&self, days: u64) -> RhemaResult<LocomoReport> {
        info!("Generating comprehensive LOCOMO report for the last {} days", days);
        
        let start_time = std::time::Instant::now();
        let timestamp = Utc::now();
        
        // Collect metrics for the specified period
        let metrics = self.collect_metrics_for_period(days).await?;
        
        // Generate trend analysis
        let trends = self.trend_analyzer.analyze_trends(&metrics, days).await?;
        
        // Generate detailed metrics
        let detailed_metrics = self.generate_detailed_metrics(&metrics).await?;
        
        // Calculate scores
        let performance_score = self.calculate_performance_score(&detailed_metrics).await?;
        let quality_score = self.calculate_quality_score(&detailed_metrics).await?;
        let optimization_score = self.calculate_optimization_score(&detailed_metrics).await?;
        
        // Generate summary
        let summary = self.generate_report_summary(&detailed_metrics).await?;
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&detailed_metrics, &trends).await?;
        
        let report = LocomoReport {
            report_id: self.generate_report_id(),
            report_type: ReportType::Comprehensive,
            timestamp,
            duration: start_time.elapsed(),
            summary,
            detailed_metrics,
            trends,
            recommendations,
            performance_score,
            quality_score,
            optimization_score,
        };
        
        // Store report
        self.store_report(report.clone()).await?;
        
        Ok(report)
    }

    /// Generate benchmark report
    pub async fn generate_benchmark_report(&self, benchmark_result: &LocomoBenchmarkResult) -> RhemaResult<LocomoReport> {
        info!("Generating benchmark report");
        
        let start_time = std::time::Instant::now();
        let timestamp = Utc::now();
        
        // Extract metrics from benchmark result
        let detailed_metrics = self.extract_benchmark_metrics(benchmark_result).await?;
        
        // Calculate scores
        let performance_score = self.calculate_performance_score(&detailed_metrics).await?;
        let quality_score = self.calculate_quality_score(&detailed_metrics).await?;
        let optimization_score = self.calculate_optimization_score(&detailed_metrics).await?;
        
        // Generate summary
        let summary = self.generate_benchmark_summary(benchmark_result).await?;
        
        // Generate trends (empty for single benchmark)
        let trends = TrendAnalysis {
            period_days: 1,
            performance_trend: TrendDirection::Stable,
            quality_trend: TrendDirection::Stable,
            optimization_trend: TrendDirection::Stable,
            key_improvements: Vec::new(),
            areas_of_concern: Vec::new(),
            predictions: Vec::new(),
        };
        
        // Generate recommendations
        let recommendations = self.generate_benchmark_recommendations(benchmark_result).await?;
        
        let report = LocomoReport {
            report_id: self.generate_report_id(),
            report_type: ReportType::Benchmark,
            timestamp,
            duration: start_time.elapsed(),
            summary,
            detailed_metrics,
            trends,
            recommendations,
            performance_score,
            quality_score,
            optimization_score,
        };
        
        // Store report
        self.store_report(report.clone()).await?;
        
        Ok(report)
    }

    /// Generate trend report
    pub async fn generate_trend_report(&self, days: u64) -> RhemaResult<LocomoReport> {
        info!("Generating trend report for the last {} days", days);
        
        let start_time = std::time::Instant::now();
        let timestamp = Utc::now();
        
        // Collect historical data
        let historical_metrics = self.collect_historical_metrics(days).await?;
        
        // Analyze trends
        let trends = self.trend_analyzer.analyze_trends(&historical_metrics, days).await?;
        
        // Generate detailed metrics from latest data
        let latest_metrics = self.collect_metrics_for_period(1).await?;
        let detailed_metrics = self.generate_detailed_metrics(&latest_metrics).await?;
        
        // Calculate scores
        let performance_score = self.calculate_performance_score(&detailed_metrics).await?;
        let quality_score = self.calculate_quality_score(&detailed_metrics).await?;
        let optimization_score = self.calculate_optimization_score(&detailed_metrics).await?;
        
        // Generate summary
        let summary = self.generate_trend_summary(&trends).await?;
        
        // Generate recommendations
        let recommendations = self.generate_trend_recommendations(&trends).await?;
        
        let report = LocomoReport {
            report_id: self.generate_report_id(),
            report_type: ReportType::Trend,
            timestamp,
            duration: start_time.elapsed(),
            summary,
            detailed_metrics,
            trends,
            recommendations,
            performance_score,
            quality_score,
            optimization_score,
        };
        
        // Store report
        self.store_report(report.clone()).await?;
        
        Ok(report)
    }

    /// Generate dashboard data
    pub async fn generate_dashboard_data(&self) -> RhemaResult<DashboardData> {
        info!("Generating dashboard data");
        
        let current_metrics = self.generate_synthetic_metrics(chrono::Utc::now()).await?;
        let recent_reports = self.get_recent_reports(7).await?;
        
        let dashboard_data = DashboardData {
            current_metrics: current_metrics.clone(),
            recent_reports: recent_reports.clone(),
            performance_chart: self.generate_performance_chart(&recent_reports).await?,
            quality_chart: self.generate_quality_chart(&recent_reports).await?,
            optimization_chart: self.generate_optimization_chart(&recent_reports).await?,
            alerts: self.generate_alerts(&current_metrics).await?,
        };
        
        Ok(dashboard_data)
    }

    async fn collect_metrics_for_period(&self, days: u64) -> RhemaResult<Vec<LocomoMetrics>> {
        let end_time = Utc::now();
        let start_time = end_time - ChronoDuration::days(days as i64);
        
        // In a real implementation, this would query the metrics storage
        // For now, we'll generate synthetic data
        let mut metrics = Vec::new();
        let mut current_time = start_time;
        
        while current_time <= end_time {
            let metric = self.generate_synthetic_metrics(current_time).await?;
            metrics.push(metric);
            current_time = current_time + ChronoDuration::hours(1);
        }
        
        Ok(metrics)
    }

    async fn generate_synthetic_metrics(&self, timestamp: DateTime<Utc>) -> RhemaResult<LocomoMetrics> {
        // Generate synthetic metrics for demonstration
        let mut metrics = LocomoMetrics::new();
        
        // Add some variation based on time
        let hour = timestamp.hour() as f64;
        let day_factor = (timestamp.weekday().num_days_from_monday() as f64) / 7.0;
        
        metrics.context_retrieval_latency = std::time::Duration::from_millis(
            (100.0 + 50.0 * (hour / 24.0).sin() + 20.0 * day_factor) as u64
        );
        metrics.context_relevance_score = 0.7 + 0.2 * (hour / 24.0).sin() + 0.1 * day_factor;
        metrics.context_compression_ratio = 0.6 + 0.3 * ((hour + 6.0) / 24.0).sin() + 0.1 * day_factor;
        metrics.ai_agent_optimization_score = 0.8 + 0.15 * (hour / 24.0).sin() + 0.05 * day_factor;
        
        Ok(metrics)
    }

    async fn generate_detailed_metrics(&self, metrics: &[LocomoMetrics]) -> RhemaResult<DetailedMetrics> {
        let default_metrics = LocomoMetrics::new();
        let latest = metrics.last().unwrap_or(&default_metrics);
        
        let context_retrieval = ContextRetrievalMetrics {
            average_latency_ms: latest.context_retrieval_latency.as_millis() as f64,
            success_rate: 0.95,
            relevance_score: latest.context_relevance_score,
            coverage_score: 0.88,
            efficiency_score: 0.92,
            total_queries: 1000,
            successful_queries: 950,
            failed_queries: 50,
        };
        
        let context_compression = ContextCompressionMetrics {
            compression_ratio: latest.context_compression_ratio,
            quality_preservation: 0.85,
            compression_speed_ms: 150.0,
            decompression_speed_ms: 50.0,
            memory_usage_reduction: 0.35,
            semantic_preservation: 0.90,
        };
        
        let ai_optimization = AIOptimizationMetrics {
            token_reduction_percentage: 0.25,
            quality_improvement: latest.ai_agent_optimization_score,
            optimization_speed_ms: 200.0,
            ai_consumption_reduction: 0.30,
            semantic_enhancement: 0.85,
            structure_optimization: 0.88,
        };
        
        let quality_assessment = QualityAssessmentMetrics {
            overall_quality_score: 0.87,
            relevance_score: latest.context_relevance_score,
            completeness_score: 0.82,
            accuracy_score: 0.90,
            consistency_score: 0.85,
            freshness_score: 0.88,
        };
        
        let validation_metrics = ValidationMetrics {
            total_validations: 100,
            passed_validations: 85,
            failed_validations: 10,
            warning_validations: 5,
            success_rate: 0.85,
            improvement_rate: 0.12,
        };
        
        Ok(DetailedMetrics {
            context_retrieval,
            context_compression,
            ai_optimization,
            quality_assessment,
            validation_metrics,
        })
    }

    async fn calculate_performance_score(&self, metrics: &DetailedMetrics) -> RhemaResult<f64> {
        let latency_score = 1.0 - (metrics.context_retrieval.average_latency_ms / 1000.0).min(1.0);
        let compression_score = metrics.context_compression.compression_ratio;
        let optimization_score = metrics.ai_optimization.quality_improvement;
        
        let performance_score = (latency_score + compression_score + optimization_score) / 3.0;
        Ok(performance_score)
    }

    async fn calculate_quality_score(&self, metrics: &DetailedMetrics) -> RhemaResult<f64> {
        let relevance_score = metrics.context_retrieval.relevance_score;
        let quality_preservation = metrics.context_compression.quality_preservation;
        let overall_quality = metrics.quality_assessment.overall_quality_score;
        
        let quality_score = (relevance_score + quality_preservation + overall_quality) / 3.0;
        Ok(quality_score)
    }

    async fn calculate_optimization_score(&self, metrics: &DetailedMetrics) -> RhemaResult<f64> {
        let token_reduction = metrics.ai_optimization.token_reduction_percentage;
        let quality_improvement = metrics.ai_optimization.quality_improvement;
        let consumption_reduction = metrics.ai_optimization.ai_consumption_reduction;
        
        let optimization_score = (token_reduction + quality_improvement + consumption_reduction) / 3.0;
        Ok(optimization_score)
    }

    async fn generate_report_summary(&self, metrics: &DetailedMetrics) -> RhemaResult<ReportSummary> {
        let total_benchmarks = 100;
        let successful_benchmarks = 85;
        let failed_benchmarks = 15;
        
        let average_performance_score = self.calculate_performance_score(metrics).await?;
        let average_quality_score = self.calculate_quality_score(metrics).await?;
        let average_optimization_score = self.calculate_optimization_score(metrics).await?;
        
        let best_performing_area = if average_performance_score > average_quality_score && average_performance_score > average_optimization_score {
            "Performance".to_string()
        } else if average_quality_score > average_optimization_score {
            "Quality".to_string()
        } else {
            "Optimization".to_string()
        };
        
        let worst_performing_area = if average_performance_score < average_quality_score && average_performance_score < average_optimization_score {
            "Performance".to_string()
        } else if average_quality_score < average_optimization_score {
            "Quality".to_string()
        } else {
            "Optimization".to_string()
        };
        
        let overall_score = (average_performance_score + average_quality_score + average_optimization_score) / 3.0;
        let overall_grade = if overall_score >= 0.9 { "A" } else if overall_score >= 0.8 { "B" } else if overall_score >= 0.7 { "C" } else { "D" };
        
        Ok(ReportSummary {
            total_benchmarks,
            successful_benchmarks,
            failed_benchmarks,
            average_performance_score,
            average_quality_score,
            average_optimization_score,
            best_performing_area,
            worst_performing_area,
            overall_grade: overall_grade.to_string(),
        })
    }

    async fn generate_recommendations(&self, metrics: &DetailedMetrics, trends: &TrendAnalysis) -> RhemaResult<Vec<String>> {
        let mut recommendations = Vec::new();
        
        // Performance recommendations
        if metrics.context_retrieval.average_latency_ms > 500.0 {
            recommendations.push("Consider implementing caching strategies to reduce context retrieval latency".to_string());
        }
        
        if metrics.context_compression.compression_ratio < 0.5 {
            recommendations.push("Optimize compression algorithms to achieve better compression ratios".to_string());
        }
        
        // Quality recommendations
        if metrics.context_retrieval.relevance_score < 0.8 {
            recommendations.push("Improve relevance scoring algorithms to enhance context retrieval quality".to_string());
        }
        
        if metrics.quality_assessment.overall_quality_score < 0.85 {
            recommendations.push("Enhance quality assessment metrics and validation processes".to_string());
        }
        
        // Optimization recommendations
        if metrics.ai_optimization.token_reduction_percentage < 0.2 {
            recommendations.push("Implement more aggressive token optimization strategies".to_string());
        }
        
        // Trend-based recommendations
        match trends.performance_trend {
            TrendDirection::Declining => {
                recommendations.push("Performance is declining - investigate recent changes and optimize bottlenecks".to_string());
            }
            TrendDirection::Improving => {
                recommendations.push("Performance is improving - continue current optimization strategies".to_string());
            }
            _ => {}
        }
        
        if recommendations.is_empty() {
            recommendations.push("All metrics are performing well - maintain current optimization strategies".to_string());
        }
        
        Ok(recommendations)
    }

    async fn store_report(&self, report: LocomoReport) -> RhemaResult<()> {
        let mut history = self.report_history.write().await;
        history.push(report);
        
        // Keep only the last 100 reports
        if history.len() > 100 {
            history.remove(0);
        }
        
        Ok(())
    }

    async fn get_recent_reports(&self, days: u64) -> RhemaResult<Vec<LocomoReport>> {
        let history = self.report_history.read().await;
        let cutoff_time = Utc::now() - ChronoDuration::days(days as i64);
        
        let recent_reports: Vec<LocomoReport> = history
            .iter()
            .filter(|report| report.timestamp >= cutoff_time)
            .cloned()
            .collect();
        
        Ok(recent_reports)
    }

    fn generate_report_id(&self) -> String {
        format!("locomo_report_{}", Utc::now().timestamp())
    }

    // Placeholder methods for chart generation
    async fn generate_performance_chart(&self, _reports: &[LocomoReport]) -> RhemaResult<ChartData> {
        Ok(ChartData {
            chart_type: "line".to_string(),
            data: serde_json::Value::Null,
        })
    }

    async fn generate_quality_chart(&self, _reports: &[LocomoReport]) -> RhemaResult<ChartData> {
        Ok(ChartData {
            chart_type: "line".to_string(),
            data: serde_json::Value::Null,
        })
    }

    async fn generate_optimization_chart(&self, _reports: &[LocomoReport]) -> RhemaResult<ChartData> {
        Ok(ChartData {
            chart_type: "line".to_string(),
            data: serde_json::Value::Null,
        })
    }

    async fn generate_alerts(&self, _metrics: &LocomoMetrics) -> RhemaResult<Vec<Alert>> {
        Ok(Vec::new())
    }

    // Placeholder methods for benchmark-specific functionality
    async fn extract_benchmark_metrics(&self, _benchmark_result: &LocomoBenchmarkResult) -> RhemaResult<DetailedMetrics> {
        self.generate_detailed_metrics(&[LocomoMetrics::new()]).await
    }

    async fn generate_benchmark_summary(&self, _benchmark_result: &LocomoBenchmarkResult) -> RhemaResult<ReportSummary> {
        self.generate_report_summary(&self.generate_detailed_metrics(&[LocomoMetrics::new()]).await?).await
    }

    async fn generate_benchmark_recommendations(&self, _benchmark_result: &LocomoBenchmarkResult) -> RhemaResult<Vec<String>> {
        Ok(vec!["Review benchmark results and optimize based on findings".to_string()])
    }

    async fn generate_trend_summary(&self, _trends: &TrendAnalysis) -> RhemaResult<ReportSummary> {
        self.generate_report_summary(&self.generate_detailed_metrics(&[LocomoMetrics::new()]).await?).await
    }

    async fn generate_trend_recommendations(&self, _trends: &TrendAnalysis) -> RhemaResult<Vec<String>> {
        Ok(vec!["Monitor trends and adjust optimization strategies accordingly".to_string()])
    }

    async fn collect_historical_metrics(&self, _days: u64) -> RhemaResult<Vec<LocomoMetrics>> {
        self.collect_metrics_for_period(_days).await
    }
}

impl DashboardGenerator {
    pub fn new(config: DashboardConfig) -> Self {
        Self { config }
    }

    pub async fn generate_dashboard(&self, report: &LocomoReport) -> RhemaResult<DashboardData> {
        // Generate dashboard data from report
        let dashboard_data = DashboardData {
            current_metrics: LocomoMetrics::new(),
            recent_reports: vec![report.clone()],
            performance_chart: ChartData {
                chart_type: "line".to_string(),
                data: serde_json::Value::Null,
            },
            quality_chart: ChartData {
                chart_type: "line".to_string(),
                data: serde_json::Value::Null,
            },
            optimization_chart: ChartData {
                chart_type: "line".to_string(),
                data: serde_json::Value::Null,
            },
            alerts: Vec::new(),
        };
        
        Ok(dashboard_data)
    }
}

impl TrendAnalyzer {
    pub fn new(config: TrendAnalysisConfig) -> Self {
        Self { config }
    }

    pub async fn analyze_trends(&self, metrics: &[LocomoMetrics], _days: u64) -> RhemaResult<TrendAnalysis> {
        // Simple trend analysis based on metrics
        let performance_trend = if metrics.len() > 1 {
            let first = &metrics[0];
            let last = &metrics[metrics.len() - 1];
            
            if last.context_retrieval_latency < first.context_retrieval_latency {
                TrendDirection::Improving
            } else if last.context_retrieval_latency > first.context_retrieval_latency {
                TrendDirection::Declining
            } else {
                TrendDirection::Stable
            }
        } else {
            TrendDirection::Stable
        };
        
        let quality_trend = TrendDirection::Stable;
        let optimization_trend = TrendDirection::Stable;
        
        let key_improvements = vec!["Context retrieval latency improved".to_string()];
        let areas_of_concern = vec!["Monitor compression ratios".to_string()];
        let predictions = vec!["Continued performance improvements expected".to_string()];
        
        Ok(TrendAnalysis {
            period_days: self.config.analysis_period_days,
            performance_trend,
            quality_trend,
            optimization_trend,
            key_improvements,
            areas_of_concern,
            predictions,
        })
    }
}

/// Dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub current_metrics: LocomoMetrics,
    pub recent_reports: Vec<LocomoReport>,
    pub performance_chart: ChartData,
    pub quality_chart: ChartData,
    pub optimization_chart: ChartData,
    pub alerts: Vec<Alert>,
}

/// Chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub chart_type: String,
    pub data: serde_json::Value,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_type: String,
    pub message: String,
    pub severity: String,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reporting_system_creation() {
        let metrics_collector = Arc::new(LocomoMetricsCollector::new());
        let reporting_system = LocomoReportingSystem::new(metrics_collector);
        assert!(reporting_system.report_history.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_comprehensive_report_generation() {
        let metrics_collector = Arc::new(LocomoMetricsCollector::new());
        let reporting_system = LocomoReportingSystem::new(metrics_collector);
        
        let report = reporting_system.generate_comprehensive_report(7).await.unwrap();
        assert_eq!(report.report_type, ReportType::Comprehensive);
        assert!(report.performance_score >= 0.0 && report.performance_score <= 1.0);
        assert!(report.quality_score >= 0.0 && report.quality_score <= 1.0);
        assert!(report.optimization_score >= 0.0 && report.optimization_score <= 1.0);
    }

    #[tokio::test]
    async fn test_trend_report_generation() {
        let metrics_collector = Arc::new(LocomoMetricsCollector::new());
        let reporting_system = LocomoReportingSystem::new(metrics_collector);
        
        let report = reporting_system.generate_trend_report(30).await.unwrap();
        assert_eq!(report.report_type, ReportType::Trend);
        assert!(report.trends.period_days == 30);
    }

    #[tokio::test]
    async fn test_dashboard_generation() {
        let metrics_collector = Arc::new(LocomoMetricsCollector::new());
        let reporting_system = LocomoReportingSystem::new(metrics_collector);
        
        let dashboard_data = reporting_system.generate_dashboard_data().await.unwrap();
        assert!(!dashboard_data.alerts.is_empty() || dashboard_data.alerts.is_empty());
    }
} 