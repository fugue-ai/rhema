use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::f64::consts::PI;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, HealthMetrics, HealthStatus, ImpactScore};

/// Prediction model types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PredictionModel {
    /// Simple moving average
    MovingAverage,
    /// Exponential smoothing
    ExponentialSmoothing,
    /// Linear regression
    LinearRegression,
    /// Anomaly detection using statistical methods
    AnomalyDetection,
    /// Time series forecasting
    TimeSeriesForecast,
    /// Machine learning model (placeholder for future ML integration)
    MachineLearning,
}

/// Prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    /// Dependency ID
    pub dependency_id: String,
    /// Predicted health status
    pub predicted_health: HealthStatus,
    /// Prediction confidence (0.0 to 1.0)
    pub confidence: f64,
    /// Predicted metrics
    pub predicted_metrics: HealthMetrics,
    /// Time until predicted failure (if applicable)
    pub time_to_failure: Option<Duration>,
    /// Risk factors identified
    pub risk_factors: Vec<RiskFactor>,
    /// Prediction timestamp
    pub timestamp: DateTime<Utc>,
    /// Model used for prediction
    pub model: PredictionModel,
}

/// Risk factor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Risk factor name
    pub name: String,
    /// Risk factor description
    pub description: String,
    /// Risk level (0.0 to 1.0)
    pub risk_level: f64,
    /// Contributing metrics
    pub contributing_metrics: Vec<String>,
    /// Mitigation suggestions
    pub mitigation_suggestions: Vec<String>,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    /// Dependency ID
    pub dependency_id: String,
    /// Whether an anomaly was detected
    pub is_anomaly: bool,
    /// Anomaly score (0.0 to 1.0)
    pub anomaly_score: f64,
    /// Anomaly type
    pub anomaly_type: AnomalyType,
    /// Affected metrics
    pub affected_metrics: Vec<String>,
    /// Severity level
    pub severity: AnomalySeverity,
    /// Detection timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of anomalies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnomalyType {
    /// Sudden spike in response time
    ResponseTimeSpike,
    /// Sudden drop in availability
    AvailabilityDrop,
    /// Unusual error rate
    ErrorRateAnomaly,
    /// Performance degradation
    PerformanceDegradation,
    /// Resource exhaustion
    ResourceExhaustion,
    /// Security anomaly
    SecurityAnomaly,
}

/// Anomaly severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnomalySeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Trend analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Dependency ID
    pub dependency_id: String,
    /// Trend direction
    pub trend: TrendDirection,
    /// Trend strength (0.0 to 1.0)
    pub strength: f64,
    /// Trend duration
    pub duration: Duration,
    /// Affected metrics
    pub affected_metrics: Vec<String>,
    /// Trend description
    pub description: String,
    /// Analysis timestamp
    pub timestamp: DateTime<Utc>,
}

/// Trend directions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrendDirection {
    /// Improving trend
    Improving,
    /// Declining trend
    Declining,
    /// Stable trend
    Stable,
    /// Cyclical pattern
    Cyclical,
}

/// Predictive analytics engine
pub struct PredictiveAnalytics {
    /// Historical data for each dependency
    historical_data: Arc<RwLock<HashMap<String, VecDeque<HealthMetrics>>>>,
    /// Prediction models configuration
    models: HashMap<PredictionModel, ModelConfig>,
    /// Anomaly detection thresholds
    anomaly_thresholds: AnomalyThresholds,
    /// Trend analysis window
    trend_window: Duration,
    /// Maximum historical data points
    max_data_points: usize,
    /// Prediction cache
    prediction_cache: Arc<RwLock<HashMap<String, PredictionResult>>>,
    /// Cache TTL
    cache_ttl: Duration,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model type
    pub model_type: PredictionModel,
    /// Model parameters
    pub parameters: HashMap<String, f64>,
    /// Whether the model is enabled
    pub enabled: bool,
    /// Model weight for ensemble predictions
    pub weight: f64,
}

/// Anomaly detection thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyThresholds {
    /// Response time threshold (standard deviations)
    pub response_time_threshold: f64,
    /// Availability threshold (percentage)
    pub availability_threshold: f64,
    /// Error rate threshold (percentage)
    pub error_rate_threshold: f64,
    /// CPU usage threshold (percentage)
    pub cpu_usage_threshold: f64,
    /// Memory usage threshold (percentage)
    pub memory_usage_threshold: f64,
    /// Network latency threshold (standard deviations)
    pub network_latency_threshold: f64,
}

impl Default for AnomalyThresholds {
    fn default() -> Self {
        Self {
            response_time_threshold: 2.0,   // 2 standard deviations
            availability_threshold: 0.95,   // 95%
            error_rate_threshold: 0.05,     // 5%
            cpu_usage_threshold: 0.8,       // 80%
            memory_usage_threshold: 0.85,   // 85%
            network_latency_threshold: 2.0, // 2 standard deviations
        }
    }
}

impl PredictiveAnalytics {
    /// Create a new predictive analytics engine
    pub fn new() -> Self {
        let mut models = HashMap::new();
        models.insert(
            PredictionModel::MovingAverage,
            ModelConfig {
                model_type: PredictionModel::MovingAverage,
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("window_size".to_string(), 10.0);
                    params
                },
                enabled: true,
                weight: 0.3,
            },
        );
        models.insert(
            PredictionModel::ExponentialSmoothing,
            ModelConfig {
                model_type: PredictionModel::ExponentialSmoothing,
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("alpha".to_string(), 0.3);
                    params
                },
                enabled: true,
                weight: 0.3,
            },
        );
        models.insert(
            PredictionModel::AnomalyDetection,
            ModelConfig {
                model_type: PredictionModel::AnomalyDetection,
                parameters: HashMap::new(),
                enabled: true,
                weight: 0.4,
            },
        );

        Self {
            historical_data: Arc::new(RwLock::new(HashMap::new())),
            models,
            anomaly_thresholds: AnomalyThresholds::default(),
            trend_window: Duration::hours(24),
            max_data_points: 1000,
            prediction_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::minutes(15),
        }
    }

    /// Add health metrics data point
    pub async fn add_data_point(
        &self,
        dependency_id: String,
        metrics: HealthMetrics,
    ) -> Result<()> {
        let mut data = self.historical_data.write().await;
        let entry = data.entry(dependency_id).or_insert_with(VecDeque::new);

        entry.push_back(metrics);

        // Keep only the most recent data points
        while entry.len() > self.max_data_points {
            entry.pop_front();
        }

        Ok(())
    }

    /// Predict health status for a dependency
    pub async fn predict_health(&self, dependency_id: &str) -> Result<PredictionResult> {
        // Check cache first
        {
            let cache = self.prediction_cache.read().await;
            if let Some(cached_result) = cache.get(dependency_id) {
                if Utc::now() - cached_result.timestamp < self.cache_ttl {
                    return Ok(cached_result.clone());
                }
            }
        }

        let data = self.historical_data.read().await;
        let metrics_history = data
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        if metrics_history.len() < 5 {
            return Err(Error::InsufficientData(dependency_id.to_string()));
        }

        // Run multiple prediction models
        let mut predictions = Vec::new();
        let mut weights = Vec::new();

        for (model_type, config) in &self.models {
            if !config.enabled {
                continue;
            }

            if let Ok(prediction) = self.run_prediction_model(model_type, config, metrics_history) {
                predictions.push(prediction);
                weights.push(config.weight);
            }
        }

        if predictions.is_empty() {
            return Err(Error::PredictionFailed(dependency_id.to_string()));
        }

        // Ensemble prediction
        let ensemble_prediction = self.ensemble_predictions(&predictions, &weights);

        // Detect anomalies
        let anomalies = self
            .detect_anomalies(dependency_id, metrics_history)
            .await?;
        let risk_factors = self.identify_risk_factors(&anomalies);

        let result = PredictionResult {
            dependency_id: dependency_id.to_string(),
            predicted_health: ensemble_prediction.health_status,
            confidence: ensemble_prediction.confidence,
            predicted_metrics: ensemble_prediction.metrics,
            time_to_failure: ensemble_prediction.time_to_failure,
            risk_factors,
            timestamp: Utc::now(),
            model: PredictionModel::MachineLearning, // Ensemble
        };

        // Cache the result
        {
            let mut cache = self.prediction_cache.write().await;
            cache.insert(dependency_id.to_string(), result.clone());
        }

        Ok(result)
    }

    /// Run a specific prediction model
    fn run_prediction_model(
        &self,
        model_type: &PredictionModel,
        config: &ModelConfig,
        metrics_history: &VecDeque<HealthMetrics>,
    ) -> Result<ModelPrediction> {
        match model_type {
            PredictionModel::MovingAverage => {
                self.moving_average_prediction(config, metrics_history)
            }
            PredictionModel::ExponentialSmoothing => {
                self.exponential_smoothing_prediction(config, metrics_history)
            }
            PredictionModel::AnomalyDetection => {
                self.anomaly_detection_prediction(config, metrics_history)
            }
            _ => Err(Error::UnsupportedModel(format!("{:?}", model_type))),
        }
    }

    /// Moving average prediction
    fn moving_average_prediction(
        &self,
        config: &ModelConfig,
        metrics_history: &VecDeque<HealthMetrics>,
    ) -> Result<ModelPrediction> {
        let window_size = *config.parameters.get("window_size").unwrap_or(&10.0) as usize;
        let window_size = window_size.min(metrics_history.len());

        if window_size == 0 {
            return Err(Error::InvalidModelParameters(
                "window_size must be > 0".to_string(),
            ));
        }

        let recent_metrics: Vec<&HealthMetrics> =
            metrics_history.iter().rev().take(window_size).collect();

        // Calculate moving averages
        let avg_response_time = recent_metrics
            .iter()
            .map(|m| m.response_time_ms)
            .sum::<f64>()
            / window_size as f64;
        let avg_availability =
            recent_metrics.iter().map(|m| m.availability).sum::<f64>() / window_size as f64;
        let avg_error_rate =
            recent_metrics.iter().map(|m| m.error_rate).sum::<f64>() / window_size as f64;
        let avg_cpu_usage =
            recent_metrics.iter().map(|m| m.cpu_usage).sum::<f64>() / window_size as f64;
        let avg_memory_usage =
            recent_metrics.iter().map(|m| m.memory_usage).sum::<f64>() / window_size as f64;

        let predicted_metrics = HealthMetrics::new(
            avg_response_time,
            avg_availability,
            avg_error_rate,
            recent_metrics[0].throughput, // Use latest throughput
            avg_cpu_usage,
            avg_memory_usage,
            recent_metrics[0].network_latency_ms, // Use latest network latency
            recent_metrics[0].disk_usage,         // Use latest disk usage
        )?;

        let health_status = HealthStatus::from(predicted_metrics.health_score());
        let confidence = 0.7; // Medium confidence for moving average

        Ok(ModelPrediction {
            health_status,
            confidence,
            metrics: predicted_metrics,
            time_to_failure: None,
        })
    }

    /// Exponential smoothing prediction
    fn exponential_smoothing_prediction(
        &self,
        config: &ModelConfig,
        metrics_history: &VecDeque<HealthMetrics>,
    ) -> Result<ModelPrediction> {
        let alpha = config.parameters.get("alpha").unwrap_or(&0.3);

        if metrics_history.is_empty() {
            return Err(Error::InsufficientData("No metrics history".to_string()));
        }

        let mut smoothed_response_time = metrics_history[0].response_time_ms;
        let mut smoothed_availability = metrics_history[0].availability;
        let mut smoothed_error_rate = metrics_history[0].error_rate;
        let mut smoothed_cpu_usage = metrics_history[0].cpu_usage;
        let mut smoothed_memory_usage = metrics_history[0].memory_usage;

        for metrics in metrics_history.iter().skip(1) {
            smoothed_response_time =
                alpha * metrics.response_time_ms + (1.0 - alpha) * smoothed_response_time;
            smoothed_availability =
                alpha * metrics.availability + (1.0 - alpha) * smoothed_availability;
            smoothed_error_rate = alpha * metrics.error_rate + (1.0 - alpha) * smoothed_error_rate;
            smoothed_cpu_usage = alpha * metrics.cpu_usage + (1.0 - alpha) * smoothed_cpu_usage;
            smoothed_memory_usage =
                alpha * metrics.memory_usage + (1.0 - alpha) * smoothed_memory_usage;
        }

        let predicted_metrics = HealthMetrics::new(
            smoothed_response_time,
            smoothed_availability,
            smoothed_error_rate,
            metrics_history[0].throughput,
            smoothed_cpu_usage,
            smoothed_memory_usage,
            metrics_history[0].network_latency_ms,
            metrics_history[0].disk_usage,
        )?;

        let health_status = HealthStatus::from(predicted_metrics.health_score());
        let confidence = 0.8; // Higher confidence for exponential smoothing

        Ok(ModelPrediction {
            health_status,
            confidence,
            metrics: predicted_metrics,
            time_to_failure: None,
        })
    }

    /// Anomaly detection prediction
    fn anomaly_detection_prediction(
        &self,
        _config: &ModelConfig,
        metrics_history: &VecDeque<HealthMetrics>,
    ) -> Result<ModelPrediction> {
        if metrics_history.len() < 10 {
            return Err(Error::InsufficientData(
                "Need at least 10 data points for anomaly detection".to_string(),
            ));
        }

        let recent_metrics = metrics_history.iter().rev().take(10).collect::<Vec<_>>();

        // Calculate statistics for anomaly detection
        let response_times: Vec<f64> = recent_metrics.iter().map(|m| m.response_time_ms).collect();
        let mean_response_time = response_times.iter().sum::<f64>() / response_times.len() as f64;
        let std_response_time = self.calculate_std_dev(&response_times, mean_response_time);

        let latest_response_time = recent_metrics[0].response_time_ms;
        let response_time_z_score = (latest_response_time - mean_response_time) / std_response_time;

        // Check for anomalies
        let is_anomaly =
            response_time_z_score.abs() > self.anomaly_thresholds.response_time_threshold;

        let predicted_metrics = recent_metrics[0].clone();
        let health_status = if is_anomaly {
            HealthStatus::Degraded
        } else {
            HealthStatus::from(predicted_metrics.health_score())
        };

        let confidence = if is_anomaly { 0.9 } else { 0.6 };

        Ok(ModelPrediction {
            health_status,
            confidence,
            metrics: predicted_metrics,
            time_to_failure: None,
        })
    }

    /// Ensemble predictions from multiple models
    fn ensemble_predictions(
        &self,
        predictions: &[ModelPrediction],
        weights: &[f64],
    ) -> ModelPrediction {
        if predictions.is_empty() {
            panic!("No predictions to ensemble");
        }

        // Weighted average of health scores
        let total_weight: f64 = weights.iter().sum();
        let weighted_health_score = predictions
            .iter()
            .zip(weights.iter())
            .map(|(pred, weight)| pred.metrics.health_score() * weight)
            .sum::<f64>()
            / total_weight;

        // Weighted average of metrics
        let weighted_response_time = predictions
            .iter()
            .zip(weights.iter())
            .map(|(pred, weight)| pred.metrics.response_time_ms * weight)
            .sum::<f64>()
            / total_weight;

        let weighted_availability = predictions
            .iter()
            .zip(weights.iter())
            .map(|(pred, weight)| pred.metrics.availability * weight)
            .sum::<f64>()
            / total_weight;

        let weighted_error_rate = predictions
            .iter()
            .zip(weights.iter())
            .map(|(pred, weight)| pred.metrics.error_rate * weight)
            .sum::<f64>()
            / total_weight;

        let weighted_cpu_usage = predictions
            .iter()
            .zip(weights.iter())
            .map(|(pred, weight)| pred.metrics.cpu_usage * weight)
            .sum::<f64>()
            / total_weight;

        let weighted_memory_usage = predictions
            .iter()
            .zip(weights.iter())
            .map(|(pred, weight)| pred.metrics.memory_usage * weight)
            .sum::<f64>()
            / total_weight;

        // Use the first prediction for other metrics (they should be similar)
        let first_prediction = &predictions[0];
        let ensemble_metrics = HealthMetrics::new(
            weighted_response_time,
            weighted_availability,
            weighted_error_rate,
            first_prediction.metrics.throughput,
            weighted_cpu_usage,
            weighted_memory_usage,
            first_prediction.metrics.network_latency_ms,
            first_prediction.metrics.disk_usage,
        )
        .unwrap();

        let ensemble_health_status = HealthStatus::from(weighted_health_score);
        let ensemble_confidence = predictions
            .iter()
            .zip(weights.iter())
            .map(|(pred, weight)| pred.confidence * weight)
            .sum::<f64>()
            / total_weight;

        ModelPrediction {
            health_status: ensemble_health_status,
            confidence: ensemble_confidence,
            metrics: ensemble_metrics,
            time_to_failure: None,
        }
    }

    /// Detect anomalies in metrics
    async fn detect_anomalies(
        &self,
        dependency_id: &str,
        metrics_history: &VecDeque<HealthMetrics>,
    ) -> Result<Vec<AnomalyResult>> {
        if metrics_history.len() < 5 {
            return Ok(Vec::new());
        }

        let mut anomalies = Vec::new();
        let recent_metrics = metrics_history.iter().rev().take(10).collect::<Vec<_>>();

        // Check response time anomalies
        if let Some(anomaly) = self.check_response_time_anomaly(dependency_id, &recent_metrics) {
            anomalies.push(anomaly);
        }

        // Check availability anomalies
        if let Some(anomaly) = self.check_availability_anomaly(dependency_id, &recent_metrics) {
            anomalies.push(anomaly);
        }

        // Check error rate anomalies
        if let Some(anomaly) = self.check_error_rate_anomaly(dependency_id, &recent_metrics) {
            anomalies.push(anomaly);
        }

        Ok(anomalies)
    }

    /// Check for response time anomalies
    fn check_response_time_anomaly(
        &self,
        dependency_id: &str,
        metrics: &[&HealthMetrics],
    ) -> Option<AnomalyResult> {
        let response_times: Vec<f64> = metrics.iter().map(|m| m.response_time_ms).collect();
        let mean = response_times.iter().sum::<f64>() / response_times.len() as f64;
        let std_dev = self.calculate_std_dev(&response_times, mean);

        let latest_response_time = metrics[0].response_time_ms;
        let z_score = (latest_response_time - mean) / std_dev;

        if z_score.abs() > self.anomaly_thresholds.response_time_threshold {
            Some(AnomalyResult {
                dependency_id: dependency_id.to_string(),
                is_anomaly: true,
                anomaly_score: z_score.abs().min(1.0),
                anomaly_type: AnomalyType::ResponseTimeSpike,
                affected_metrics: vec!["response_time_ms".to_string()],
                severity: if z_score.abs() > 3.0 {
                    AnomalySeverity::Critical
                } else {
                    AnomalySeverity::High
                },
                timestamp: Utc::now(),
            })
        } else {
            None
        }
    }

    /// Check for availability anomalies
    fn check_availability_anomaly(
        &self,
        dependency_id: &str,
        metrics: &[&HealthMetrics],
    ) -> Option<AnomalyResult> {
        let latest_availability = metrics[0].availability;

        if latest_availability < self.anomaly_thresholds.availability_threshold {
            Some(AnomalyResult {
                dependency_id: dependency_id.to_string(),
                is_anomaly: true,
                anomaly_score: 1.0 - latest_availability,
                anomaly_type: AnomalyType::AvailabilityDrop,
                affected_metrics: vec!["availability".to_string()],
                severity: if latest_availability < 0.8 {
                    AnomalySeverity::Critical
                } else {
                    AnomalySeverity::High
                },
                timestamp: Utc::now(),
            })
        } else {
            None
        }
    }

    /// Check for error rate anomalies
    fn check_error_rate_anomaly(
        &self,
        dependency_id: &str,
        metrics: &[&HealthMetrics],
    ) -> Option<AnomalyResult> {
        let latest_error_rate = metrics[0].error_rate;

        if latest_error_rate > self.anomaly_thresholds.error_rate_threshold {
            Some(AnomalyResult {
                dependency_id: dependency_id.to_string(),
                is_anomaly: true,
                anomaly_score: latest_error_rate,
                anomaly_type: AnomalyType::ErrorRateAnomaly,
                affected_metrics: vec!["error_rate".to_string()],
                severity: if latest_error_rate > 0.1 {
                    AnomalySeverity::Critical
                } else {
                    AnomalySeverity::High
                },
                timestamp: Utc::now(),
            })
        } else {
            None
        }
    }

    /// Identify risk factors from anomalies
    fn identify_risk_factors(&self, anomalies: &[AnomalyResult]) -> Vec<RiskFactor> {
        anomalies
            .iter()
            .map(|anomaly| {
                let mitigation = match anomaly.anomaly_type {
                    AnomalyType::ResponseTimeSpike => vec![
                        "Check for resource bottlenecks".to_string(),
                        "Review recent deployments".to_string(),
                        "Monitor upstream dependencies".to_string(),
                    ],
                    AnomalyType::AvailabilityDrop => vec![
                        "Check service health endpoints".to_string(),
                        "Review infrastructure status".to_string(),
                        "Verify network connectivity".to_string(),
                    ],
                    AnomalyType::ErrorRateAnomaly => vec![
                        "Review application logs".to_string(),
                        "Check for recent code changes".to_string(),
                        "Verify external service dependencies".to_string(),
                    ],
                    _ => vec![
                        "Investigate root cause".to_string(),
                        "Review monitoring alerts".to_string(),
                    ],
                };

                RiskFactor {
                    name: format!("{:?}", anomaly.anomaly_type),
                    description: format!(
                        "Anomaly detected in {}",
                        anomaly.affected_metrics.join(", ")
                    ),
                    risk_level: anomaly.anomaly_score,
                    contributing_metrics: anomaly.affected_metrics.clone(),
                    mitigation_suggestions: mitigation,
                }
            })
            .collect()
    }

    /// Calculate standard deviation
    fn calculate_std_dev(&self, values: &[f64], mean: f64) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        let variance =
            values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;

        variance.sqrt()
    }

    /// Analyze trends for a dependency
    pub async fn analyze_trends(&self, dependency_id: &str) -> Result<TrendAnalysis> {
        let data = self.historical_data.read().await;
        let metrics_history = data
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        if metrics_history.len() < 10 {
            return Err(Error::InsufficientData(dependency_id.to_string()));
        }

        let recent_metrics: Vec<&HealthMetrics> = metrics_history.iter().rev().take(20).collect();

        // Calculate trend for health scores
        let health_scores: Vec<f64> = recent_metrics.iter().map(|m| m.health_score()).collect();
        let trend = self.calculate_trend(&health_scores);

        let trend_direction = if trend.slope > 0.01 {
            TrendDirection::Improving
        } else if trend.slope < -0.01 {
            TrendDirection::Declining
        } else {
            TrendDirection::Stable
        };

        let strength = trend.r_squared.min(1.0).max(0.0);
        let duration = Duration::hours(recent_metrics.len() as i64);

        Ok(TrendAnalysis {
            dependency_id: dependency_id.to_string(),
            trend: trend_direction,
            strength,
            duration,
            affected_metrics: vec!["health_score".to_string()],
            description: format!("Health score trend: {:.3} per data point", trend.slope),
            timestamp: Utc::now(),
        })
    }

    /// Calculate linear trend
    fn calculate_trend(&self, values: &[f64]) -> TrendResult {
        let n = values.len() as f64;
        let x_values: Vec<f64> = (0..values.len()).map(|i| i as f64).collect();

        let sum_x: f64 = x_values.iter().sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = x_values.iter().zip(values.iter()).map(|(x, y)| x * y).sum();
        let sum_x2: f64 = x_values.iter().map(|x| x * x).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared
        let mean_y = sum_y / n;
        let ss_tot: f64 = values.iter().map(|y| (y - mean_y).powi(2)).sum();
        let ss_res: f64 = x_values
            .iter()
            .zip(values.iter())
            .map(|(x, y)| (y - (slope * x + intercept)).powi(2))
            .sum();

        let r_squared = if ss_tot > 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        TrendResult {
            slope,
            intercept,
            r_squared,
        }
    }

    /// Clear expired cache entries
    pub async fn clear_expired_cache(&self) {
        let mut cache = self.prediction_cache.write().await;
        let now = Utc::now();
        cache.retain(|_, result| now - result.timestamp < self.cache_ttl);
    }

    /// Get prediction statistics
    pub async fn get_statistics(&self) -> PredictionStatistics {
        let data = self.historical_data.read().await;
        let cache = self.prediction_cache.read().await;

        PredictionStatistics {
            total_dependencies: data.len(),
            total_data_points: data.values().map(|v| v.len()).sum(),
            cached_predictions: cache.len(),
            models_configured: self.models.len(),
        }
    }
}

/// Model prediction result
#[derive(Debug, Clone)]
struct ModelPrediction {
    health_status: HealthStatus,
    confidence: f64,
    metrics: HealthMetrics,
    time_to_failure: Option<Duration>,
}

/// Trend calculation result
#[derive(Debug, Clone)]
struct TrendResult {
    slope: f64,
    intercept: f64,
    r_squared: f64,
}

/// Prediction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionStatistics {
    /// Total number of dependencies being tracked
    pub total_dependencies: usize,
    /// Total number of data points stored
    pub total_data_points: usize,
    /// Number of cached predictions
    pub cached_predictions: usize,
    /// Number of configured models
    pub models_configured: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HealthMetrics;

    fn create_test_metrics(
        response_time: f64,
        availability: f64,
        error_rate: f64,
    ) -> HealthMetrics {
        HealthMetrics::new(
            response_time,
            availability,
            error_rate,
            100.0, // throughput
            0.5,   // cpu_usage
            0.6,   // memory_usage
            50.0,  // network_latency
            0.4,   // disk_usage
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_predictive_analytics_new() {
        let analytics = PredictiveAnalytics::new();
        assert_eq!(analytics.models.len(), 3);
    }

    #[tokio::test]
    async fn test_add_data_point() {
        let analytics = PredictiveAnalytics::new();
        let metrics = create_test_metrics(100.0, 0.99, 0.01);

        analytics
            .add_data_point("test-dep".to_string(), metrics)
            .await
            .unwrap();

        let data = analytics.historical_data.read().await;
        assert_eq!(data.get("test-dep").unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_moving_average_prediction() {
        let analytics = PredictiveAnalytics::new();
        let mut metrics_history = VecDeque::new();

        // Add some test data
        for i in 0..10 {
            let response_time = 100.0 + (i as f64 * 10.0);
            metrics_history.push_back(create_test_metrics(response_time, 0.99, 0.01));
        }

        let config = ModelConfig {
            model_type: PredictionModel::MovingAverage,
            parameters: {
                let mut params = HashMap::new();
                params.insert("window_size".to_string(), 5.0);
                params
            },
            enabled: true,
            weight: 1.0,
        };

        let prediction = analytics
            .moving_average_prediction(&config, &metrics_history)
            .unwrap();
        assert!(prediction.confidence > 0.0);
    }

    #[test]
    fn test_calculate_std_dev() {
        let analytics = PredictiveAnalytics::new();
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = 3.0;
        let std_dev = analytics.calculate_std_dev(&values, mean);
        assert!((std_dev - 1.5811388300841898).abs() < 0.001);
    }

    #[test]
    fn test_calculate_trend() {
        let analytics = PredictiveAnalytics::new();
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let trend = analytics.calculate_trend(&values);
        assert!((trend.slope - 1.0).abs() < 0.001);
        assert!(trend.r_squared > 0.99);
    }
}
