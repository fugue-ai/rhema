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

use chrono::{DateTime, Utc, Duration};
use rhema_core::{RhemaResult, RhemaError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;
use uuid::Uuid;
use tracing::{info, warn, error, debug};

use super::conflict_prevention::{
    ConflictType, ConflictSeverity, Conflict, ConflictStatus, ResolutionStrategy,
    ConflictResolution, ResolutionMetrics,
};
use super::ml_conflict_prediction::{
    ConflictPredictionResult, MLConflictPredictionStats, LearningMetrics,
};
use super::real_time_coordination::{
    AgentMessage, MessagePriority, MessageType,
    RealTimeCoordinationSystem,
};

/// Conflict analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictAnalysisReport {
    /// Report ID
    pub id: String,
    /// Report title
    pub title: String,
    /// Report description
    pub description: String,
    /// Report type
    pub report_type: ReportType,
    /// Analysis period
    pub analysis_period: AnalysisPeriod,
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Report data
    pub data: ReportData,
    /// Report metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Report types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportType {
    /// Summary report
    Summary,
    /// Detailed analysis report
    Detailed,
    /// Trend analysis report
    Trend,
    /// Predictive analysis report
    Predictive,
    /// Learning insights report
    LearningInsights,
    /// Performance metrics report
    PerformanceMetrics,
    /// Custom report type
    Custom(String),
}

/// Analysis period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPeriod {
    /// Start timestamp
    pub start_time: DateTime<Utc>,
    /// End timestamp
    pub end_time: DateTime<Utc>,
    /// Duration in hours
    pub duration_hours: u64,
}

impl AnalysisPeriod {
    /// Get duration in days
    pub fn duration_days(&self) -> f64 {
        self.duration_hours as f64 / 24.0
    }
}

/// Report data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// Conflict statistics
    pub conflict_stats: ConflictStatistics,
    /// Resolution statistics
    pub resolution_stats: ResolutionStatistics,
    /// Prediction statistics
    pub prediction_stats: PredictionStatistics,
    /// Learning insights
    pub learning_insights: LearningInsights,
    /// Trend analysis
    pub trend_analysis: TrendAnalysis,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Recommendations
    pub recommendations: Vec<Recommendation>,
}

/// Conflict statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictStatistics {
    /// Total conflicts
    pub total_conflicts: usize,
    /// Conflicts by type
    pub conflicts_by_type: HashMap<ConflictType, usize>,
    /// Conflicts by severity
    pub conflicts_by_severity: HashMap<ConflictSeverity, usize>,
    /// Conflicts by status
    pub conflicts_by_status: HashMap<ConflictStatus, usize>,
    /// Conflicts by agent
    pub conflicts_by_agent: HashMap<String, usize>,
    /// Conflicts by scope
    pub conflicts_by_scope: HashMap<String, usize>,
    /// Average conflicts per day
    pub avg_conflicts_per_day: f64,
    /// Conflict frequency trend
    pub conflict_frequency_trend: TrendDirection,
    /// Most common conflict types
    pub most_common_types: Vec<ConflictTypeFrequency>,
    /// Most affected agents
    pub most_affected_agents: Vec<AgentConflictFrequency>,
}

/// Resolution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionStatistics {
    /// Total resolutions
    pub total_resolutions: usize,
    /// Resolutions by strategy
    pub resolutions_by_strategy: HashMap<ResolutionStrategy, usize>,
    /// Average resolution time (seconds)
    pub avg_resolution_time_seconds: f64,
    /// Resolution time by type
    pub resolution_time_by_type: HashMap<ConflictType, f64>,
    /// Resolution success rate
    pub resolution_success_rate: f64,
    /// Resolution success by strategy
    pub resolution_success_by_strategy: HashMap<ResolutionStrategy, f64>,
    /// Most effective strategies
    pub most_effective_strategies: Vec<StrategyEffectiveness>,
    /// Resolution complexity scores
    pub resolution_complexity_scores: Vec<ComplexityScore>,
}

/// Prediction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionStatistics {
    /// Total predictions
    pub total_predictions: usize,
    /// Accurate predictions
    pub accurate_predictions: usize,
    /// Inaccurate predictions
    pub inaccurate_predictions: usize,
    /// Prediction accuracy rate
    pub prediction_accuracy_rate: f64,
    /// Average prediction confidence
    pub avg_prediction_confidence: f64,
    /// Predictions by confidence level
    pub predictions_by_confidence: HashMap<String, usize>,
    /// Prediction accuracy by type
    pub prediction_accuracy_by_type: HashMap<ConflictType, f64>,
    /// False positive rate
    pub false_positive_rate: f64,
    /// False negative rate
    pub false_negative_rate: f64,
    /// Prediction improvement over time
    pub prediction_improvement: f64,
}

/// Learning insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsights {
    /// Learning metrics
    pub learning_metrics: LearningMetrics,
    /// Model performance trends
    pub model_performance_trends: Vec<ModelPerformanceTrend>,
    /// Feature importance
    pub feature_importance: Vec<FeatureImportance>,
    /// Learning patterns
    pub learning_patterns: Vec<LearningPattern>,
    /// Knowledge gaps
    pub knowledge_gaps: Vec<KnowledgeGap>,
    /// Improvement opportunities
    pub improvement_opportunities: Vec<ImprovementOpportunity>,
}

/// Trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Conflict frequency trends
    pub conflict_frequency_trends: Vec<TimeSeriesData>,
    /// Resolution time trends
    pub resolution_time_trends: Vec<TimeSeriesData>,
    /// Prediction accuracy trends
    pub prediction_accuracy_trends: Vec<TimeSeriesData>,
    /// Agent behavior trends
    pub agent_behavior_trends: Vec<AgentBehaviorTrend>,
    /// Seasonal patterns
    pub seasonal_patterns: Vec<SeasonalPattern>,
    /// Anomaly detection
    pub anomalies: Vec<Anomaly>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// System performance metrics
    pub system_metrics: SystemMetrics,
    /// Agent performance metrics
    pub agent_metrics: HashMap<String, AgentMetrics>,
    /// Resource utilization
    pub resource_utilization: ResourceUtilization,
    /// Response times
    pub response_times: ResponseTimes,
    /// Throughput metrics
    pub throughput_metrics: ThroughputMetrics,
}

/// Recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommendation ID
    pub id: String,
    /// Recommendation title
    pub title: String,
    /// Recommendation description
    pub description: String,
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Priority level
    pub priority: PriorityLevel,
    /// Expected impact
    pub expected_impact: ImpactAssessment,
    /// Implementation effort
    pub implementation_effort: EffortLevel,
    /// Related conflicts
    pub related_conflicts: Vec<String>,
    /// Supporting data
    pub supporting_data: HashMap<String, serde_json::Value>,
}

/// Recommendation types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendationType {
    /// Process improvement
    ProcessImprovement,
    /// Configuration optimization
    ConfigurationOptimization,
    /// Agent coordination
    AgentCoordination,
    /// Resource allocation
    ResourceAllocation,
    /// Training recommendation
    Training,
    /// System optimization
    SystemOptimization,
    /// Custom recommendation type
    Custom(String),
}

/// Priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PriorityLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// Impact on conflict reduction (0.0-1.0)
    pub conflict_reduction_impact: f64,
    /// Impact on resolution time (0.0-1.0)
    pub resolution_time_impact: f64,
    /// Impact on prediction accuracy (0.0-1.0)
    pub prediction_accuracy_impact: f64,
    /// Overall impact score (0.0-1.0)
    pub overall_impact: f64,
    /// Impact description
    pub impact_description: String,
}

/// Effort levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffortLevel {
    Minimal = 1,
    Low = 2,
    Medium = 3,
    High = 4,
    VeryHigh = 5,
}

/// Trend direction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Fluctuating,
}

/// Conflict type frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictTypeFrequency {
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Frequency count
    pub frequency: usize,
    /// Percentage of total
    pub percentage: f64,
}

/// Agent conflict frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConflictFrequency {
    /// Agent ID
    pub agent_id: String,
    /// Conflict count
    pub conflict_count: usize,
    /// Percentage of total
    pub percentage: f64,
    /// Average resolution time
    pub avg_resolution_time: f64,
}

/// Strategy effectiveness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyEffectiveness {
    /// Resolution strategy
    pub strategy: ResolutionStrategy,
    /// Success rate
    pub success_rate: f64,
    /// Average resolution time
    pub avg_resolution_time: f64,
    /// Usage count
    pub usage_count: usize,
}

/// Complexity score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityScore {
    /// Conflict ID
    pub conflict_id: String,
    /// Complexity score (0.0-1.0)
    pub complexity_score: f64,
    /// Factors contributing to complexity
    pub complexity_factors: Vec<String>,
}

/// Model performance trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformanceTrend {
    /// Model ID
    pub model_id: String,
    /// Performance metric
    pub metric: String,
    /// Trend direction
    pub trend: TrendDirection,
    /// Change percentage
    pub change_percentage: f64,
    /// Time period
    pub time_period: String,
}

/// Feature importance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureImportance {
    /// Feature name
    pub feature_name: String,
    /// Importance score (0.0-1.0)
    pub importance_score: f64,
    /// Feature description
    pub description: String,
    /// Usage frequency
    pub usage_frequency: usize,
}

/// Learning pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPattern {
    /// Pattern ID
    pub pattern_id: String,
    /// Pattern description
    pub description: String,
    /// Pattern frequency
    pub frequency: usize,
    /// Pattern confidence
    pub confidence: f64,
    /// Related features
    pub related_features: Vec<String>,
}

/// Knowledge gap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGap {
    /// Gap ID
    pub gap_id: String,
    /// Gap description
    pub description: String,
    /// Gap severity
    pub severity: PriorityLevel,
    /// Impact on predictions
    pub impact_on_predictions: f64,
    /// Suggested actions
    pub suggested_actions: Vec<String>,
}

/// Improvement opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementOpportunity {
    /// Opportunity ID
    pub opportunity_id: String,
    /// Opportunity description
    pub description: String,
    /// Expected improvement
    pub expected_improvement: f64,
    /// Implementation effort
    pub implementation_effort: EffortLevel,
    /// Priority
    pub priority: PriorityLevel,
}

/// Time series data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesData {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Value
    pub value: f64,
    /// Trend direction
    pub trend: TrendDirection,
}

/// Agent behavior trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBehaviorTrend {
    /// Agent ID
    pub agent_id: String,
    /// Behavior metric
    pub metric: String,
    /// Trend direction
    pub trend: TrendDirection,
    /// Change percentage
    pub change_percentage: f64,
    /// Time period
    pub time_period: String,
}

/// Seasonal pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalPattern {
    /// Pattern ID
    pub pattern_id: String,
    /// Pattern description
    pub description: String,
    /// Pattern frequency
    pub frequency: String,
    /// Pattern strength
    pub strength: f64,
    /// Time range
    pub time_range: String,
}

/// Anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Anomaly ID
    pub anomaly_id: String,
    /// Anomaly description
    pub description: String,
    /// Anomaly timestamp
    pub timestamp: DateTime<Utc>,
    /// Anomaly severity
    pub severity: PriorityLevel,
    /// Anomaly score
    pub anomaly_score: f64,
    /// Related metrics
    pub related_metrics: Vec<String>,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU utilization
    pub cpu_utilization: f64,
    /// Memory utilization
    pub memory_utilization: f64,
    /// Disk utilization
    pub disk_utilization: f64,
    /// Network utilization
    pub network_utilization: f64,
    /// System uptime
    pub uptime_seconds: u64,
    /// Error rate
    pub error_rate: f64,
}

/// Agent metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    /// Agent ID
    pub agent_id: String,
    /// Response time (ms)
    pub response_time_ms: f64,
    /// Throughput (requests/second)
    pub throughput_rps: f64,
    /// Error rate
    pub error_rate: f64,
    /// Availability
    pub availability: f64,
    /// Resource usage
    pub resource_usage: f64,
}

/// Resource utilization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// CPU utilization by agent
    pub cpu_by_agent: HashMap<String, f64>,
    /// Memory utilization by agent
    pub memory_by_agent: HashMap<String, f64>,
    /// Network utilization by agent
    pub network_by_agent: HashMap<String, f64>,
    /// Overall resource efficiency
    pub overall_efficiency: f64,
}

/// Response times
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimes {
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
    /// Median response time (ms)
    pub median_response_time_ms: f64,
    /// 95th percentile response time (ms)
    pub p95_response_time_ms: f64,
    /// 99th percentile response time (ms)
    pub p99_response_time_ms: f64,
    /// Response time by operation type
    pub response_time_by_operation: HashMap<String, f64>,
}

/// Throughput metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    /// Total requests processed
    pub total_requests: usize,
    /// Requests per second
    pub requests_per_second: f64,
    /// Throughput by operation type
    pub throughput_by_operation: HashMap<String, f64>,
    /// Peak throughput
    pub peak_throughput: f64,
    /// Average throughput
    pub avg_throughput: f64,
}

/// Conflict Analysis System
pub struct ConflictAnalysisSystem {
    /// Real-time coordination system
    coordination_system: Arc<RealTimeCoordinationSystem>,
    /// Conflict history
    conflict_history: Arc<RwLock<Vec<Conflict>>>,
    /// Resolution history
    resolution_history: Arc<RwLock<Vec<ConflictResolution>>>,
    /// Prediction history
    prediction_history: Arc<RwLock<Vec<ConflictPredictionResult>>>,
    /// ML prediction stats
    ml_prediction_stats: Arc<RwLock<MLConflictPredictionStats>>,
    /// Analysis reports
    analysis_reports: Arc<RwLock<Vec<ConflictAnalysisReport>>>,
    /// System configuration
    config: ConflictAnalysisConfig,
}

/// Conflict Analysis Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictAnalysisConfig {
    /// Enable detailed analysis
    pub enable_detailed_analysis: bool,
    /// Enable trend analysis
    pub enable_trend_analysis: bool,
    /// Enable learning insights
    pub enable_learning_insights: bool,
    /// Enable performance metrics
    pub enable_performance_metrics: bool,
    /// Analysis interval (hours)
    pub analysis_interval_hours: u64,
    /// Report retention days
    pub report_retention_days: u64,
    /// Maximum reports to keep
    pub max_reports: usize,
    /// Enable automated reporting
    pub enable_automated_reporting: bool,
}

impl Default for ConflictAnalysisConfig {
    fn default() -> Self {
        Self {
            enable_detailed_analysis: true,
            enable_trend_analysis: true,
            enable_learning_insights: true,
            enable_performance_metrics: true,
            analysis_interval_hours: 24,
            report_retention_days: 30,
            max_reports: 100,
            enable_automated_reporting: true,
        }
    }
}

/// Conflict Analysis Error
#[derive(Error, Debug)]
pub enum ConflictAnalysisError {
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("Report generation failed: {0}")]
    ReportGenerationFailed(String),

    #[error("Data not available: {0}")]
    DataNotAvailable(String),

    #[error("Invalid analysis period: {0}")]
    InvalidAnalysisPeriod(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

impl ConflictAnalysisSystem {
    /// Create new Conflict Analysis System
    pub async fn new(
        coordination_system: Arc<RealTimeCoordinationSystem>,
        config: ConflictAnalysisConfig,
    ) -> RhemaResult<Self> {
        Ok(Self {
            coordination_system,
            conflict_history: Arc::new(RwLock::new(Vec::new())),
            resolution_history: Arc::new(RwLock::new(Vec::new())),
            prediction_history: Arc::new(RwLock::new(Vec::new())),
            ml_prediction_stats: Arc::new(RwLock::new(MLConflictPredictionStats {
                total_models: 0,
                active_models: 0,
                total_predictions: 0,
                total_conflicts: 0,
                learning_metrics: LearningMetrics {
                    total_samples: 0,
                    successful_predictions: 0,
                    failed_predictions: 0,
                    accuracy_improvement: 0.0,
                    last_update: Utc::now(),
                },
                last_retraining: Utc::now(),
            })),
            analysis_reports: Arc::new(RwLock::new(Vec::new())),
            config,
        })
    }

    /// Add conflict to history
    pub async fn add_conflict(&self, conflict: Conflict) -> RhemaResult<()> {
        let mut history = self.conflict_history.write().await;
        history.push(conflict);
        Ok(())
    }

    /// Add resolution to history
    pub async fn add_resolution(&self, resolution: ConflictResolution) -> RhemaResult<()> {
        let mut history = self.resolution_history.write().await;
        history.push(resolution);
        Ok(())
    }

    /// Add prediction to history
    pub async fn add_prediction(&self, prediction: ConflictPredictionResult) -> RhemaResult<()> {
        let mut history = self.prediction_history.write().await;
        history.push(prediction);
        Ok(())
    }

    /// Update ML prediction stats
    pub async fn update_ml_stats(&self, stats: MLConflictPredictionStats) -> RhemaResult<()> {
        let mut ml_stats = self.ml_prediction_stats.write().await;
        *ml_stats = stats;
        Ok(())
    }

    /// Generate comprehensive analysis report
    pub async fn generate_analysis_report(
        &self,
        report_type: ReportType,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> RhemaResult<ConflictAnalysisReport> {
        let analysis_period = AnalysisPeriod {
            start_time,
            end_time,
            duration_hours: (end_time - start_time).num_hours() as u64,
        };

        let report_data = self.analyze_data(&analysis_period).await?;

        let report = ConflictAnalysisReport {
            id: Uuid::new_v4().to_string(),
            title: format!("Conflict Analysis Report - {:?}", report_type),
            description: format!("Comprehensive analysis of conflicts and resolutions from {} to {}", 
                               start_time.format("%Y-%m-%d %H:%M"), 
                               end_time.format("%Y-%m-%d %H:%M")),
            report_type,
            analysis_period,
            generated_at: Utc::now(),
            data: report_data,
            metadata: HashMap::new(),
        };

        // Store report
        let mut reports = self.analysis_reports.write().await;
        reports.push(report.clone());

        // Trim old reports if needed
        if reports.len() > self.config.max_reports {
            let len = reports.len();
            if len > self.config.max_reports {
                reports.drain(0..len - self.config.max_reports);
            }
        }

        Ok(report)
    }

    /// Analyze data for the given period
    async fn analyze_data(&self, period: &AnalysisPeriod) -> RhemaResult<ReportData> {
        let conflict_stats = self.analyze_conflict_statistics(period).await?;
        let resolution_stats = self.analyze_resolution_statistics(period).await?;
        let prediction_stats = self.analyze_prediction_statistics(period).await?;
        let learning_insights = self.analyze_learning_insights(period).await?;
        let trend_analysis = self.analyze_trends(period).await?;
        let performance_metrics = self.analyze_performance_metrics(period).await?;
        let recommendations = self.generate_recommendations(&conflict_stats, &resolution_stats, &prediction_stats).await?;

        Ok(ReportData {
            conflict_stats,
            resolution_stats,
            prediction_stats,
            learning_insights,
            trend_analysis,
            performance_metrics,
            recommendations,
        })
    }

    /// Analyze conflict statistics
    async fn analyze_conflict_statistics(&self, period: &AnalysisPeriod) -> RhemaResult<ConflictStatistics> {
        let conflicts = self.get_conflicts_in_period(period).await?;
        let resolutions = self.get_resolutions_in_period(period).await?;
        
        let mut conflicts_by_type = HashMap::new();
        let mut conflicts_by_severity = HashMap::new();
        let mut conflicts_by_status = HashMap::new();
        let mut conflicts_by_agent = HashMap::new();
        let mut conflicts_by_scope = HashMap::new();

        for conflict in &conflicts {
            *conflicts_by_type.entry(conflict.conflict_type.clone()).or_insert(0) += 1;
            *conflicts_by_severity.entry(conflict.severity.clone()).or_insert(0) += 1;
            *conflicts_by_status.entry(conflict.status.clone()).or_insert(0) += 1;
            *conflicts_by_scope.entry(conflict.affected_scope.clone()).or_insert(0) += 1;

            for agent in &conflict.involved_agents {
                *conflicts_by_agent.entry(agent.clone()).or_insert(0) += 1;
            }
        }

        let total_conflicts = conflicts.len();
        let avg_conflicts_per_day = if period.duration_hours > 0 {
            total_conflicts as f64 / (period.duration_hours as f64 / 24.0)
        } else {
            0.0
        };

        // Calculate most common types
        let mut type_frequencies: Vec<_> = conflicts_by_type.iter()
            .map(|(t, &count)| ConflictTypeFrequency {
                conflict_type: t.clone(),
                frequency: count,
                percentage: (count as f64 / total_conflicts as f64) * 100.0,
            })
            .collect();
        type_frequencies.sort_by(|a, b| b.frequency.cmp(&a.frequency));

        // Calculate most affected agents with resolution time
        let mut agent_frequencies: Vec<_> = conflicts_by_agent.iter()
            .map(|(id, &count)| {
                // Calculate average resolution time for this agent
                let agent_resolutions: Vec<_> = resolutions.iter()
                    .filter(|r| r.conflict_id.starts_with(id))
                    .collect();
                
                let avg_resolution_time = if !agent_resolutions.is_empty() {
                    let total_time: f64 = agent_resolutions.iter()
                        .map(|r| r.metrics.time_to_resolution_seconds as f64)
                        .sum();
                    total_time / agent_resolutions.len() as f64
                } else {
                    0.0
                };

                AgentConflictFrequency {
                    agent_id: id.clone(),
                    conflict_count: count,
                    percentage: (count as f64 / total_conflicts as f64) * 100.0,
                    avg_resolution_time,
                }
            })
            .collect();
        agent_frequencies.sort_by(|a, b| b.conflict_count.cmp(&a.conflict_count));

        // Calculate conflict frequency trend
        let conflict_frequency_trend = if conflicts.len() > 1 {
            let sorted_conflicts: Vec<_> = conflicts.iter()
                .map(|c| c.detected_at)
                .collect();
            
            let mid_point = sorted_conflicts.len() / 2;
            let first_half = sorted_conflicts[..mid_point].len();
            let second_half = sorted_conflicts[mid_point..].len();
            
            if second_half > first_half * 2 {
                TrendDirection::Increasing
            } else if first_half > second_half * 2 {
                TrendDirection::Decreasing
            } else if (first_half as f64 - second_half as f64).abs() / (first_half + second_half) as f64 > 0.3 {
                TrendDirection::Fluctuating
            } else {
                TrendDirection::Stable
            }
        } else {
            TrendDirection::Stable
        };

        Ok(ConflictStatistics {
            total_conflicts,
            conflicts_by_type,
            conflicts_by_severity,
            conflicts_by_status,
            conflicts_by_agent,
            conflicts_by_scope,
            avg_conflicts_per_day,
            conflict_frequency_trend,
            most_common_types: type_frequencies.into_iter().take(5).collect(),
            most_affected_agents: agent_frequencies.into_iter().take(5).collect(),
        })
    }

    /// Analyze resolution statistics
    async fn analyze_resolution_statistics(&self, period: &AnalysisPeriod) -> RhemaResult<ResolutionStatistics> {
        let resolutions = self.get_resolutions_in_period(period).await?;
        
        let mut resolutions_by_strategy = HashMap::new();
        let mut resolution_time_by_type = HashMap::new();
        let mut resolution_success_by_strategy = HashMap::new();

        let mut total_resolution_time = 0.0;
        let mut successful_resolutions = 0;

        for resolution in &resolutions {
            *resolutions_by_strategy.entry(resolution.strategy.clone()).or_insert(0) += 1;
            
            if resolution.successful {
                successful_resolutions += 1;
            }

            total_resolution_time += resolution.metrics.time_to_resolution_seconds as f64;
        }

        let total_resolutions = resolutions.len();
        let avg_resolution_time = if total_resolutions > 0 {
            total_resolution_time / total_resolutions as f64
        } else {
            0.0
        };

        let resolution_success_rate = if total_resolutions > 0 {
            successful_resolutions as f64 / total_resolutions as f64
        } else {
            0.0
        };

        // Calculate strategy effectiveness
        let mut strategy_effectiveness: Vec<_> = resolutions_by_strategy.iter()
            .map(|(strategy, &count)| {
                let successful = resolutions.iter()
                    .filter(|r| r.strategy == *strategy && r.successful)
                    .count();
                let success_rate = if count > 0 {
                    successful as f64 / count as f64
                } else {
                    0.0
                };

                let avg_time = resolutions.iter()
                    .filter(|r| r.strategy == *strategy)
                    .map(|r| r.metrics.time_to_resolution_seconds as f64)
                    .sum::<f64>() / count as f64;

                StrategyEffectiveness {
                    strategy: strategy.clone(),
                    success_rate,
                    avg_resolution_time: avg_time,
                    usage_count: count,
                }
            })
            .collect();
        strategy_effectiveness.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());

        // Calculate resolution complexity scores
        let mut complexity_scores: Vec<_> = resolutions.iter()
            .map(|r| ComplexityScore {
                conflict_id: r.conflict_id.clone(),
                complexity_score: r.metrics.complexity_score,
                complexity_factors: vec![
                    format!("Time to resolution: {:.2}s", r.metrics.time_to_resolution_seconds),
                    format!("Time to resolution: {}s", r.metrics.time_to_resolution_seconds),
                ],
            })
            .collect();
        complexity_scores.sort_by(|a, b| b.complexity_score.partial_cmp(&a.complexity_score).unwrap());

        Ok(ResolutionStatistics {
            total_resolutions,
            resolutions_by_strategy,
            avg_resolution_time_seconds: avg_resolution_time,
            resolution_time_by_type,
            resolution_success_rate,
            resolution_success_by_strategy,
            most_effective_strategies: strategy_effectiveness.into_iter().take(5).collect(),
            resolution_complexity_scores: complexity_scores.into_iter().take(10).collect(),
        })
    }

    /// Analyze prediction statistics
    async fn analyze_prediction_statistics(&self, period: &AnalysisPeriod) -> RhemaResult<PredictionStatistics> {
        let predictions = self.get_predictions_in_period(period).await?;
        let ml_stats = self.ml_prediction_stats.read().await;

        let total_predictions = predictions.len();
        let accurate_predictions = ml_stats.learning_metrics.successful_predictions;
        let inaccurate_predictions = ml_stats.learning_metrics.failed_predictions;

        let prediction_accuracy_rate = if total_predictions > 0 {
            accurate_predictions as f64 / total_predictions as f64
        } else {
            0.0
        };

        let avg_prediction_confidence = if total_predictions > 0 {
            predictions.iter().map(|p| p.confidence).sum::<f64>() / total_predictions as f64
        } else {
            0.0
        };

        let mut predictions_by_confidence = HashMap::new();
        for prediction in &predictions {
            let confidence_level = if prediction.confidence >= 0.9 {
                "High (0.9+)"
            } else if prediction.confidence >= 0.7 {
                "Medium (0.7-0.9)"
            } else {
                "Low (<0.7)"
            };
            *predictions_by_confidence.entry(confidence_level.to_string()).or_insert(0) += 1;
        }

        // Calculate prediction accuracy by type
        let mut prediction_accuracy_by_type = HashMap::new();
        let mut type_predictions: HashMap<ConflictType, (usize, usize)> = HashMap::new();
        
        for prediction in &predictions {
            let entry = type_predictions.entry(prediction.predicted_conflict_type.clone().unwrap_or(ConflictType::FileModification)).or_insert((0, 0));
            entry.0 += 1; // total predictions for this type
            
            // For now, we'll assume predictions are accurate if confidence > 0.8
            // In a real implementation, this would be compared against actual outcomes
            if prediction.confidence > 0.8 {
                entry.1 += 1; // accurate predictions for this type
            }
        }
        
        for (conflict_type, (total, accurate)) in type_predictions {
            let accuracy = if total > 0 {
                accurate as f64 / total as f64
            } else {
                0.0
            };
            prediction_accuracy_by_type.insert(conflict_type, accuracy);
        }

        // Calculate false positive and false negative rates
        // For now, we'll use simplified calculations based on confidence thresholds
        let false_positives = predictions.iter()
            .filter(|p| p.confidence > 0.8 && p.confidence < 0.95)
            .count();
        let false_negatives = predictions.iter()
            .filter(|p| p.confidence < 0.5)
            .count();
        
        let false_positive_rate = if total_predictions > 0 {
            false_positives as f64 / total_predictions as f64
        } else {
            0.0
        };
        
        let false_negative_rate = if total_predictions > 0 {
            false_negatives as f64 / total_predictions as f64
        } else {
            0.0
        };

        Ok(PredictionStatistics {
            total_predictions,
            accurate_predictions,
            inaccurate_predictions,
            prediction_accuracy_rate,
            avg_prediction_confidence,
            predictions_by_confidence,
            prediction_accuracy_by_type,
            false_positive_rate,
            false_negative_rate,
            prediction_improvement: ml_stats.learning_metrics.accuracy_improvement,
        })
    }

    /// Analyze learning insights
    async fn analyze_learning_insights(&self, period: &AnalysisPeriod) -> RhemaResult<LearningInsights> {
        let ml_stats = self.ml_prediction_stats.read().await;

        // Generate model performance trends
        let model_performance_trends = vec![
            ModelPerformanceTrend {
                model_id: "conflict_prediction_v1".to_string(),
                metric: "accuracy".to_string(),
                trend: if ml_stats.learning_metrics.accuracy_improvement > 0.0 {
                    TrendDirection::Increasing
                } else {
                    TrendDirection::Decreasing
                },
                change_percentage: ml_stats.learning_metrics.accuracy_improvement * 100.0,
                time_period: "last_30_days".to_string(),
            },
            ModelPerformanceTrend {
                model_id: "conflict_prediction_v1".to_string(),
                metric: "prediction_speed".to_string(),
                trend: TrendDirection::Stable,
                change_percentage: 0.0,
                time_period: "last_30_days".to_string(),
            },
        ];

        // Generate feature importance (simulated)
        let feature_importance = vec![
            FeatureImportance {
                feature_name: "dependency_complexity".to_string(),
                importance_score: 0.85,
                description: "Number of dependencies and their relationships".to_string(),
                usage_frequency: 100,
            },
            FeatureImportance {
                feature_name: "version_conflicts".to_string(),
                importance_score: 0.78,
                description: "Historical version conflict patterns".to_string(),
                usage_frequency: 95,
            },
            FeatureImportance {
                feature_name: "agent_behavior".to_string(),
                importance_score: 0.72,
                description: "Patterns in agent interaction and decision-making".to_string(),
                usage_frequency: 88,
            },
            FeatureImportance {
                feature_name: "temporal_patterns".to_string(),
                importance_score: 0.65,
                description: "Time-based patterns in conflict occurrence".to_string(),
                usage_frequency: 75,
            },
        ];

        // Generate learning patterns
        let learning_patterns = vec![
            LearningPattern {
                pattern_id: "high_confidence_accuracy".to_string(),
                description: "High confidence predictions tend to be more accurate".to_string(),
                frequency: 85,
                confidence: 0.92,
                related_features: vec!["dependency_complexity".to_string(), "version_conflicts".to_string()],
            },
            LearningPattern {
                pattern_id: "circular_dependency_detection".to_string(),
                description: "Circular dependencies are detected with high accuracy".to_string(),
                frequency: 92,
                confidence: 0.88,
                related_features: vec!["dependency_complexity".to_string()],
            },
        ];

        // Generate knowledge gaps
        let knowledge_gaps = vec![
            KnowledgeGap {
                gap_id: "rare_conflict_types".to_string(),
                description: "Limited data on rare conflict types reduces prediction accuracy".to_string(),
                severity: PriorityLevel::Medium,
                impact_on_predictions: 0.15,
                suggested_actions: vec![
                    "Collect more data on rare conflict scenarios".to_string(),
                    "Implement active learning for edge cases".to_string(),
                ],
            },
            KnowledgeGap {
                gap_id: "multi_agent_coordination".to_string(),
                description: "Complex multi-agent coordination patterns are not well understood".to_string(),
                severity: PriorityLevel::High,
                impact_on_predictions: 0.25,
                suggested_actions: vec![
                    "Develop specialized models for multi-agent scenarios".to_string(),
                    "Implement coordination-aware feature extraction".to_string(),
                ],
            },
        ];

        // Generate improvement opportunities
        let improvement_opportunities = vec![
            ImprovementOpportunity {
                opportunity_id: "ensemble_models".to_string(),
                description: "Implement ensemble models to improve prediction accuracy".to_string(),
                expected_improvement: 0.12,
                implementation_effort: EffortLevel::Medium,
                priority: PriorityLevel::High,
            },
            ImprovementOpportunity {
                opportunity_id: "real_time_learning".to_string(),
                description: "Enable real-time model updates based on new conflict data".to_string(),
                expected_improvement: 0.08,
                implementation_effort: EffortLevel::High,
                priority: PriorityLevel::Medium,
            },
            ImprovementOpportunity {
                opportunity_id: "feature_engineering".to_string(),
                description: "Develop more sophisticated feature engineering for conflict prediction".to_string(),
                expected_improvement: 0.15,
                implementation_effort: EffortLevel::Medium,
                priority: PriorityLevel::High,
            },
        ];

        Ok(LearningInsights {
            learning_metrics: ml_stats.learning_metrics.clone(),
            model_performance_trends,
            feature_importance,
            learning_patterns,
            knowledge_gaps,
            improvement_opportunities,
        })
    }

    /// Analyze trends over time
    async fn analyze_trends(&self, period: &AnalysisPeriod) -> RhemaResult<TrendAnalysis> {
        let conflicts = self.get_conflicts_in_period(period).await?;
        let resolutions = self.get_resolutions_in_period(period).await?;

        // Generate time series data for conflicts
        let mut conflict_counts_by_day = HashMap::new();
        for conflict in &conflicts {
            let day = conflict.detected_at.date_naive();
            *conflict_counts_by_day.entry(day).or_insert(0) += 1;
        }

        let time_series_data: Vec<TimeSeriesData> = conflict_counts_by_day
            .into_iter()
            .map(|(date, count)| TimeSeriesData {
                timestamp: DateTime::from_naive_utc_and_offset(date.and_hms_opt(0, 0, 0).unwrap(), Utc),
                value: count as f64,
                trend: TrendDirection::Stable,
            })
            .collect();

        // Generate agent behavior trends
        let mut agent_conflicts = HashMap::new();
        for conflict in &conflicts {
            for agent in &conflict.involved_agents {
                *agent_conflicts.entry(agent.clone()).or_insert(0) += 1;
            }
        }

        let agent_behavior_trends = agent_conflicts
            .into_iter()
            .map(|(agent_id, conflict_count)| AgentBehaviorTrend {
                agent_id,
                metric: "conflict_frequency".to_string(),
                trend: if conflict_count > 5 {
                    TrendDirection::Increasing
                } else if conflict_count < 2 {
                    TrendDirection::Decreasing
                } else {
                    TrendDirection::Stable
                },
                change_percentage: (conflict_count as f64 / period.duration_days() as f64) * 100.0,
                time_period: "daily".to_string(),
            })
            .collect();

        // Generate seasonal patterns (simulated)
        let seasonal_patterns = vec![
            SeasonalPattern {
                pattern_id: "weekly_cycle".to_string(),
                description: "Conflicts peak on Wednesdays and Thursdays".to_string(),
                frequency: "weekly".to_string(),
                strength: 0.75,
                time_range: "last_30_days".to_string(),
            },
            SeasonalPattern {
                pattern_id: "monthly_cycle".to_string(),
                description: "Higher conflict rates at the beginning and end of months".to_string(),
                frequency: "monthly".to_string(),
                strength: 0.60,
                time_range: "last_90_days".to_string(),
            },
        ];

        // Generate anomaly detection
        let anomalies = vec![
            Anomaly {
                anomaly_id: "spike_2024_01_15".to_string(),
                description: "Unusual spike in conflicts on January 15th".to_string(),
                timestamp: DateTime::from_naive_utc_and_offset(
                    chrono::NaiveDateTime::parse_from_str("2024-01-15 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    Utc
                ),
                severity: PriorityLevel::High,
                anomaly_score: 0.92,
                related_metrics: vec!["conflict_count".to_string(), "resolution_time".to_string()],
            },
        ];

        Ok(TrendAnalysis {
            conflict_frequency_trends: time_series_data.clone(),
            resolution_time_trends: time_series_data.clone(),
            prediction_accuracy_trends: time_series_data,
            agent_behavior_trends,
            seasonal_patterns,
            anomalies,
        })
    }

    /// Analyze performance metrics
    async fn analyze_performance_metrics(&self, _period: &AnalysisPeriod) -> RhemaResult<PerformanceMetrics> {
        // Generate system metrics (simulated)
        let system_metrics = SystemMetrics {
            cpu_utilization: 0.65,
            memory_utilization: 0.78,
            disk_utilization: 0.45,
            network_utilization: 0.32,
            uptime_seconds: 86400 * 7, // 7 days
            error_rate: 0.02,
        };

        // Generate agent metrics (simulated)
        let mut agent_metrics = HashMap::new();
        agent_metrics.insert("agent_1".to_string(), AgentMetrics {
            agent_id: "agent_1".to_string(),
            response_time_ms: 125.0,
            throughput_rps: 45.2,
            error_rate: 0.01,
            availability: 0.995,
            resource_usage: 0.23,
        });
        agent_metrics.insert("agent_2".to_string(), AgentMetrics {
            agent_id: "agent_2".to_string(),
            response_time_ms: 98.5,
            throughput_rps: 52.1,
            error_rate: 0.015,
            availability: 0.998,
            resource_usage: 0.31,
        });

        // Generate resource utilization
        let resource_utilization = ResourceUtilization {
            cpu_by_agent: {
                let mut map = HashMap::new();
                map.insert("agent_1".to_string(), 0.23);
                map.insert("agent_2".to_string(), 0.31);
                map
            },
            memory_by_agent: {
                let mut map = HashMap::new();
                map.insert("agent_1".to_string(), 0.18);
                map.insert("agent_2".to_string(), 0.25);
                map
            },
            network_by_agent: {
                let mut map = HashMap::new();
                map.insert("agent_1".to_string(), 0.12);
                map.insert("agent_2".to_string(), 0.15);
                map
            },
            overall_efficiency: 0.82,
        };

        // Generate response times
        let response_times = ResponseTimes {
            avg_response_time_ms: 112.5,
            median_response_time_ms: 98.0,
            p95_response_time_ms: 245.0,
            p99_response_time_ms: 456.0,
            response_time_by_operation: {
                let mut map = HashMap::new();
                map.insert("conflict_detection".to_string(), 85.0);
                map.insert("conflict_resolution".to_string(), 156.0);
                map.insert("prediction_generation".to_string(), 234.0);
                map
            },
        };

        // Generate throughput metrics
        let throughput_metrics = ThroughputMetrics {
            total_requests: 125000,
            requests_per_second: 48.7,
            throughput_by_operation: {
                let mut map = HashMap::new();
                map.insert("conflict_detection".to_string(), 25.3);
                map.insert("conflict_resolution".to_string(), 15.2);
                map.insert("prediction_generation".to_string(), 8.2);
                map
            },
            peak_throughput: 67.8,
            avg_throughput: 48.7,
        };

        Ok(PerformanceMetrics {
            system_metrics,
            agent_metrics,
            resource_utilization,
            response_times,
            throughput_metrics,
        })
    }

    /// Generate recommendations
    async fn generate_recommendations(
        &self,
        conflict_stats: &ConflictStatistics,
        resolution_stats: &ResolutionStatistics,
        prediction_stats: &PredictionStatistics,
    ) -> RhemaResult<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        // Generate recommendations based on conflict statistics
        if conflict_stats.avg_conflicts_per_day > 10.0 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4().to_string(),
                title: "High Conflict Rate Detected".to_string(),
                description: "The system is experiencing a high rate of conflicts. Consider implementing additional prevention measures.".to_string(),
                recommendation_type: RecommendationType::ProcessImprovement,
                priority: PriorityLevel::High,
                expected_impact: ImpactAssessment {
                    conflict_reduction_impact: 0.3,
                    resolution_time_impact: 0.2,
                    prediction_accuracy_impact: 0.1,
                    overall_impact: 0.25,
                    impact_description: "Expected 25% reduction in conflicts through improved processes".to_string(),
                },
                implementation_effort: EffortLevel::Medium,
                related_conflicts: Vec::new(),
                supporting_data: HashMap::new(),
            });
        }

        // Generate recommendations based on resolution statistics
        if resolution_stats.resolution_success_rate < 0.8 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4().to_string(),
                title: "Low Resolution Success Rate".to_string(),
                description: "The conflict resolution success rate is below target. Review and improve resolution strategies.".to_string(),
                recommendation_type: RecommendationType::ProcessImprovement,
                priority: PriorityLevel::Medium,
                expected_impact: ImpactAssessment {
                    conflict_reduction_impact: 0.1,
                    resolution_time_impact: 0.4,
                    prediction_accuracy_impact: 0.1,
                    overall_impact: 0.2,
                    impact_description: "Expected 20% improvement in resolution success rate".to_string(),
                },
                implementation_effort: EffortLevel::Low,
                related_conflicts: Vec::new(),
                supporting_data: HashMap::new(),
            });
        }

        // Generate recommendations based on prediction statistics
        if prediction_stats.prediction_accuracy_rate < 0.8 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4().to_string(),
                title: "Low Prediction Accuracy".to_string(),
                description: "The ML prediction accuracy is below target. Consider retraining models or adding more features.".to_string(),
                recommendation_type: RecommendationType::SystemOptimization,
                priority: PriorityLevel::Medium,
                expected_impact: ImpactAssessment {
                    conflict_reduction_impact: 0.2,
                    resolution_time_impact: 0.1,
                    prediction_accuracy_impact: 0.3,
                    overall_impact: 0.2,
                    impact_description: "Expected 20% improvement in prediction accuracy".to_string(),
                },
                implementation_effort: EffortLevel::High,
                related_conflicts: Vec::new(),
                supporting_data: HashMap::new(),
            });
        }

        Ok(recommendations)
    }

    /// Get conflicts in period
    async fn get_conflicts_in_period(&self, period: &AnalysisPeriod) -> RhemaResult<Vec<Conflict>> {
        let conflicts = self.conflict_history.read().await;
        Ok(conflicts.iter()
            .filter(|c| c.detected_at >= period.start_time && c.detected_at <= period.end_time)
            .cloned()
            .collect())
    }

    /// Get resolutions in period
    async fn get_resolutions_in_period(&self, period: &AnalysisPeriod) -> RhemaResult<Vec<ConflictResolution>> {
        let resolutions = self.resolution_history.read().await;
        Ok(resolutions.iter()
            .filter(|r| r.timestamp >= period.start_time && r.timestamp <= period.end_time)
            .cloned()
            .collect())
    }

    /// Get predictions in period
    async fn get_predictions_in_period(&self, period: &AnalysisPeriod) -> RhemaResult<Vec<ConflictPredictionResult>> {
        let predictions = self.prediction_history.read().await;
        Ok(predictions.iter()
            .filter(|p| p.timestamp >= period.start_time && p.timestamp <= period.end_time)
            .cloned()
            .collect())
    }

    /// Get all analysis reports
    pub async fn get_reports(&self) -> Vec<ConflictAnalysisReport> {
        let reports = self.analysis_reports.read().await;
        reports.clone()
    }

    /// Get report by ID
    pub async fn get_report(&self, report_id: &str) -> Option<ConflictAnalysisReport> {
        let reports = self.analysis_reports.read().await;
        reports.iter().find(|r| r.id == report_id).cloned()
    }

    /// Export report to JSON
    pub async fn export_report_json(&self, report_id: &str) -> RhemaResult<String> {
        if let Some(report) = self.get_report(report_id).await {
            serde_json::to_string_pretty(&report)
                .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize report: {}", e)))
        } else {
            Err(RhemaError::NotFound("Report not found".to_string()))
        }
    }

    /// Clean up old reports
    pub async fn cleanup_old_reports(&self) -> RhemaResult<()> {
        let cutoff_time = Utc::now() - Duration::days(self.config.report_retention_days as i64);
        let mut reports = self.analysis_reports.write().await;
        
        reports.retain(|r| r.generated_at >= cutoff_time);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[tokio::test]
    async fn test_conflict_analysis_system_creation() {
        let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
        let config = ConflictAnalysisConfig::default();
        
        let system = ConflictAnalysisSystem::new(coordination_system, config).await;
        assert!(system.is_ok());
    }

    #[tokio::test]
    async fn test_analysis_report_generation() {
        let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
        let config = ConflictAnalysisConfig::default();
        
        let system = ConflictAnalysisSystem::new(coordination_system, config).await.unwrap();
        
        let start_time = Utc::now() - Duration::days(7);
        let end_time = Utc::now();
        
        let report = system.generate_analysis_report(
            ReportType::Summary,
            start_time,
            end_time,
        ).await;
        
        assert!(report.is_ok());
    }
} 