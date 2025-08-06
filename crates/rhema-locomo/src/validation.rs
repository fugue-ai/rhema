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
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::types::LocomoError;
use crate::metrics::LocomoMetrics;
use rhema_core::RhemaResult;

/// LOCOMO validation framework
pub struct LocomoValidationFramework {
    baseline_metrics: LocomoMetrics,
    current_metrics: Arc<RwLock<LocomoMetrics>>,
    improvement_thresholds: LocomoImprovementThresholds,
    validation_history: Arc<RwLock<Vec<ValidationResult>>>,
}

/// LOCOMO improvement thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomoImprovementThresholds {
    pub retrieval_improvement: f64,
    pub compression_improvement: f64,
    pub ai_optimization_improvement: f64,
    pub overall_improvement: f64,
    pub relevance_improvement: f64,
    pub persistence_improvement: f64,
}

impl Default for LocomoImprovementThresholds {
    fn default() -> Self {
        Self {
            retrieval_improvement: 0.1,    // 10% improvement
            compression_improvement: 0.15,  // 15% improvement
            ai_optimization_improvement: 0.2, // 20% improvement
            overall_improvement: 0.1,       // 10% improvement
            relevance_improvement: 0.1,     // 10% improvement
            persistence_improvement: 0.05,  // 5% improvement
        }
    }
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub validation_type: ValidationType,
    pub status: ValidationStatus,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub metric_name: String,
    pub baseline_value: f64,
    pub current_value: f64,
    pub improvement_percentage: f64,
    pub meets_threshold: bool,
}

/// Validation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    RetrievalImprovement,
    CompressionImprovement,
    AIOptimizationImprovement,
    OverallImprovement,
    RelevanceImprovement,
    PersistenceImprovement,
    CrossScopeImprovement,
    QualityImprovement,
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Passed,
    Failed,
    Warning,
    Inconclusive,
}

/// Metric validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValidation {
    pub metric_name: String,
    pub improvement_percentage: f64,
    pub meets_threshold: bool,
    pub baseline_value: f64,
    pub current_value: f64,
    pub validation_status: ValidationStatus,
    pub recommendation: String,
}

impl LocomoValidationFramework {
    pub fn new(baseline_metrics: LocomoMetrics, thresholds: LocomoImprovementThresholds) -> Self {
        Self {
            baseline_metrics,
            current_metrics: Arc::new(RwLock::new(LocomoMetrics::new())),
            improvement_thresholds: thresholds,
            validation_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn update_current_metrics(&self, metrics: LocomoMetrics) -> RhemaResult<()> {
        let mut current = self.current_metrics.write().await;
        *current = metrics;
        Ok(())
    }

    pub async fn validate_improvements(&self) -> RhemaResult<Vec<ValidationResult>> {
        let current_metrics = self.current_metrics.read().await;
        let mut results = Vec::new();

        // Validate context retrieval improvements
        let retrieval_validation = self.validate_retrieval_improvement(&current_metrics).await?;
        results.push(retrieval_validation);

        // Validate compression improvements
        let compression_validation = self.validate_compression_improvement(&current_metrics).await?;
        results.push(compression_validation);

        // Validate AI optimization improvements
        let ai_optimization_validation = self.validate_ai_optimization_improvement(&current_metrics).await?;
        results.push(ai_optimization_validation);

        // Validate overall improvements
        let overall_validation = self.validate_overall_improvement(&current_metrics).await?;
        results.push(overall_validation);

        // Validate relevance improvements
        let relevance_validation = self.validate_relevance_improvement(&current_metrics).await?;
        results.push(relevance_validation);

        // Validate persistence improvements
        let persistence_validation = self.validate_persistence_improvement(&current_metrics).await?;
        results.push(persistence_validation);

        // Store validation results
        self.store_validation_results(&results).await?;

        Ok(results)
    }

    async fn validate_retrieval_improvement(&self, current: &LocomoMetrics) -> RhemaResult<ValidationResult> {
        let baseline_latency = self.baseline_metrics.context_retrieval_latency.as_secs_f64();
        let current_latency = current.context_retrieval_latency.as_secs_f64();
        
        let improvement = if baseline_latency > 0.0 {
            (baseline_latency - current_latency) / baseline_latency
        } else {
            0.0
        };

        let meets_threshold = improvement >= self.improvement_thresholds.retrieval_improvement;
        let status = if meets_threshold { ValidationStatus::Passed } else { ValidationStatus::Failed };

        let message = if meets_threshold {
            format!("Context retrieval latency improved by {:.1}%", improvement * 100.0)
        } else {
            format!("Context retrieval latency improvement of {:.1}% below threshold of {:.1}%", 
                   improvement * 100.0, self.improvement_thresholds.retrieval_improvement * 100.0)
        };

        Ok(ValidationResult {
            validation_type: ValidationType::RetrievalImprovement,
            status,
            message,
            timestamp: Utc::now(),
            metric_name: "context_retrieval_latency".to_string(),
            baseline_value: baseline_latency,
            current_value: current_latency,
            improvement_percentage: improvement * 100.0,
            meets_threshold,
        })
    }

    async fn validate_compression_improvement(&self, current: &LocomoMetrics) -> RhemaResult<ValidationResult> {
        let baseline_ratio = self.baseline_metrics.context_compression_ratio;
        let current_ratio = current.context_compression_ratio;
        
        // Lower compression ratio is better (more compression)
        let improvement = if baseline_ratio > 0.0 {
            (baseline_ratio - current_ratio) / baseline_ratio
        } else {
            0.0
        };

        let meets_threshold = improvement >= self.improvement_thresholds.compression_improvement;
        let status = if meets_threshold { ValidationStatus::Passed } else { ValidationStatus::Failed };

        let message = if meets_threshold {
            format!("Context compression improved by {:.1}%", improvement * 100.0)
        } else {
            format!("Context compression improvement of {:.1}% below threshold of {:.1}%", 
                   improvement * 100.0, self.improvement_thresholds.compression_improvement * 100.0)
        };

        Ok(ValidationResult {
            validation_type: ValidationType::CompressionImprovement,
            status,
            message,
            timestamp: Utc::now(),
            metric_name: "context_compression_ratio".to_string(),
            baseline_value: baseline_ratio,
            current_value: current_ratio,
            improvement_percentage: improvement * 100.0,
            meets_threshold,
        })
    }

    async fn validate_ai_optimization_improvement(&self, current: &LocomoMetrics) -> RhemaResult<ValidationResult> {
        let baseline_score = self.baseline_metrics.ai_agent_optimization_score;
        let current_score = current.ai_agent_optimization_score;
        
        let improvement = if baseline_score > 0.0 {
            (current_score - baseline_score) / baseline_score
        } else {
            0.0
        };

        let meets_threshold = improvement >= self.improvement_thresholds.ai_optimization_improvement;
        let status = if meets_threshold { ValidationStatus::Passed } else { ValidationStatus::Failed };

        let message = if meets_threshold {
            format!("AI optimization score improved by {:.1}%", improvement * 100.0)
        } else {
            format!("AI optimization improvement of {:.1}% below threshold of {:.1}%", 
                   improvement * 100.0, self.improvement_thresholds.ai_optimization_improvement * 100.0)
        };

        Ok(ValidationResult {
            validation_type: ValidationType::AIOptimizationImprovement,
            status,
            message,
            timestamp: Utc::now(),
            metric_name: "ai_agent_optimization_score".to_string(),
            baseline_value: baseline_score,
            current_value: current_score,
            improvement_percentage: improvement * 100.0,
            meets_threshold,
        })
    }

    async fn validate_overall_improvement(&self, current: &LocomoMetrics) -> RhemaResult<ValidationResult> {
        let baseline_overall = self.baseline_metrics.overall_score();
        let current_overall = current.overall_score();
        
        let improvement = if baseline_overall > 0.0 {
            (current_overall - baseline_overall) / baseline_overall
        } else {
            0.0
        };

        let meets_threshold = improvement >= self.improvement_thresholds.overall_improvement;
        let status = if meets_threshold { ValidationStatus::Passed } else { ValidationStatus::Failed };

        let message = if meets_threshold {
            format!("Overall LOCOMO score improved by {:.1}%", improvement * 100.0)
        } else {
            format!("Overall improvement of {:.1}% below threshold of {:.1}%", 
                   improvement * 100.0, self.improvement_thresholds.overall_improvement * 100.0)
        };

        Ok(ValidationResult {
            validation_type: ValidationType::OverallImprovement,
            status,
            message,
            timestamp: Utc::now(),
            metric_name: "overall_score".to_string(),
            baseline_value: baseline_overall,
            current_value: current_overall,
            improvement_percentage: improvement * 100.0,
            meets_threshold,
        })
    }

    async fn validate_relevance_improvement(&self, current: &LocomoMetrics) -> RhemaResult<ValidationResult> {
        let baseline_relevance = self.baseline_metrics.context_relevance_score;
        let current_relevance = current.context_relevance_score;
        
        let improvement = if baseline_relevance > 0.0 {
            (current_relevance - baseline_relevance) / baseline_relevance
        } else {
            0.0
        };

        let meets_threshold = improvement >= self.improvement_thresholds.relevance_improvement;
        let status = if meets_threshold { ValidationStatus::Passed } else { ValidationStatus::Failed };

        let message = if meets_threshold {
            format!("Context relevance improved by {:.1}%", improvement * 100.0)
        } else {
            format!("Relevance improvement of {:.1}% below threshold of {:.1}%", 
                   improvement * 100.0, self.improvement_thresholds.relevance_improvement * 100.0)
        };

        Ok(ValidationResult {
            validation_type: ValidationType::RelevanceImprovement,
            status,
            message,
            timestamp: Utc::now(),
            metric_name: "context_relevance_score".to_string(),
            baseline_value: baseline_relevance,
            current_value: current_relevance,
            improvement_percentage: improvement * 100.0,
            meets_threshold,
        })
    }

    async fn validate_persistence_improvement(&self, current: &LocomoMetrics) -> RhemaResult<ValidationResult> {
        let baseline_persistence = self.baseline_metrics.context_persistence_accuracy;
        let current_persistence = current.context_persistence_accuracy;
        
        let improvement = if baseline_persistence > 0.0 {
            (current_persistence - baseline_persistence) / baseline_persistence
        } else {
            0.0
        };

        let meets_threshold = improvement >= self.improvement_thresholds.persistence_improvement;
        let status = if meets_threshold { ValidationStatus::Passed } else { ValidationStatus::Failed };

        let message = if meets_threshold {
            format!("Context persistence improved by {:.1}%", improvement * 100.0)
        } else {
            format!("Persistence improvement of {:.1}% below threshold of {:.1}%", 
                   improvement * 100.0, self.improvement_thresholds.persistence_improvement * 100.0)
        };

        Ok(ValidationResult {
            validation_type: ValidationType::PersistenceImprovement,
            status,
            message,
            timestamp: Utc::now(),
            metric_name: "context_persistence_accuracy".to_string(),
            baseline_value: baseline_persistence,
            current_value: current_persistence,
            improvement_percentage: improvement * 100.0,
            meets_threshold,
        })
    }

    async fn store_validation_results(&self, results: &[ValidationResult]) -> RhemaResult<()> {
        let mut history = self.validation_history.write().await;
        history.extend(results.iter().cloned());
        
        // Keep only the last 1000 validation results
        if history.len() > 1000 {
            let len = history.len();
            history.drain(0..len - 1000);
        }
        
        Ok(())
    }

    pub async fn get_validation_history(&self, hours: u64) -> RhemaResult<Vec<ValidationResult>> {
        let history = self.validation_history.read().await;
        let cutoff = Utc::now() - chrono::Duration::hours(hours as i64);
        
        let recent_results: Vec<ValidationResult> = history
            .iter()
            .filter(|result| result.timestamp >= cutoff)
            .cloned()
            .collect();
        
        Ok(recent_results)
    }

    pub async fn get_validation_summary(&self) -> RhemaResult<ValidationSummary> {
        let history = self.validation_history.read().await;
        
        let total_validations = history.len();
        let passed_validations = history.iter().filter(|r| matches!(r.status, ValidationStatus::Passed)).count();
        let failed_validations = history.iter().filter(|r| matches!(r.status, ValidationStatus::Failed)).count();
        let warning_validations = history.iter().filter(|r| matches!(r.status, ValidationStatus::Warning)).count();
        
        let success_rate = if total_validations > 0 {
            passed_validations as f64 / total_validations as f64
        } else {
            0.0
        };

        Ok(ValidationSummary {
            total_validations,
            passed_validations,
            failed_validations,
            warning_validations,
            success_rate,
            last_validation: history.last().cloned(),
        })
    }

    pub async fn generate_validation_report(&self) -> RhemaResult<ValidationReport> {
        let summary = self.get_validation_summary().await?;
        let recent_results = self.get_validation_history(24).await?;
        
        let mut report = ValidationReport {
            summary,
            recent_results: recent_results.clone(),
            recommendations: Vec::new(),
            timestamp: Utc::now(),
        };

        // Generate recommendations based on validation results
        report.recommendations = self.generate_recommendations(&recent_results).await?;

        Ok(report)
    }

    async fn generate_recommendations(&self, results: &[ValidationResult]) -> RhemaResult<Vec<String>> {
        let mut recommendations = Vec::new();

        // Analyze failed validations and generate recommendations
        let failed_retrieval = results.iter()
            .filter(|r| matches!(r.validation_type, ValidationType::RetrievalImprovement) && !r.meets_threshold)
            .count();
        
        if failed_retrieval > 0 {
            recommendations.push("Consider optimizing context retrieval algorithms and caching strategies".to_string());
        }

        let failed_compression = results.iter()
            .filter(|r| matches!(r.validation_type, ValidationType::CompressionImprovement) && !r.meets_threshold)
            .count();
        
        if failed_compression > 0 {
            recommendations.push("Implement more efficient compression algorithms or adjust compression targets".to_string());
        }

        let failed_ai_optimization = results.iter()
            .filter(|r| matches!(r.validation_type, ValidationType::AIOptimizationImprovement) && !r.meets_threshold)
            .count();
        
        if failed_ai_optimization > 0 {
            recommendations.push("Enhance AI optimization strategies and context structuring".to_string());
        }

        let failed_relevance = results.iter()
            .filter(|r| matches!(r.validation_type, ValidationType::RelevanceImprovement) && !r.meets_threshold)
            .count();
        
        if failed_relevance > 0 {
            recommendations.push("Improve relevance scoring algorithms and semantic understanding".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("All validation metrics are meeting improvement thresholds".to_string());
        }

        Ok(recommendations)
    }
}

/// Validation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_validations: usize,
    pub passed_validations: usize,
    pub failed_validations: usize,
    pub warning_validations: usize,
    pub success_rate: f64,
    pub last_validation: Option<ValidationResult>,
}

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub summary: ValidationSummary,
    pub recent_results: Vec<ValidationResult>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
} 