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

pub mod benchmark_engine;
pub mod quality_assessor;
pub mod metrics;
pub mod types;
pub mod validation;
pub mod optimization;
pub mod reporting;

// Re-export main types for convenience
pub use benchmark_engine::{
    LocomoBenchmarkEngine, LocomoBenchmarkSuite, LocomoBenchmarkResult
};
pub use types::{BenchmarkConfig, BenchmarkScenario};

pub use quality_assessor::{
    ContextQualityAssessor, ContextQualityScore, RelevanceScorer,
    CompressionAnalyzer, PersistenceTracker, AIConsumptionAnalyzer
};

pub use metrics::{
    LocomoMetrics, LocomoMetricsCollector, LocomoPerformanceAnalyzer,
    LocomoBenchmarkMetrics, ContextRetrievalMetrics, ContextCompressionMetrics,
    AIOptimizationMetrics
};

pub use types::{
    LocomoError, BenchmarkType, QualityMetric, OptimizationStrategy,
    ContextSize, CompressionAlgorithm, RelevanceThreshold
};

pub use validation::{
    LocomoValidationFramework, ValidationResult, MetricValidation,
    LocomoImprovementThresholds
};

pub use optimization::{
    ContextOptimizer, OptimizationResult, OptimizationAction,
    AIContextOptimizer, CompressionOptimizer
};

pub use reporting::{
    LocomoReportingSystem, LocomoReport, ReportType, ReportSummary,
    DetailedMetrics, DashboardData, ChartData, Alert, TrendAnalysis,
    TrendDirection, DashboardGenerator, TrendAnalyzer
};

// Error type conversions
impl From<types::LocomoError> for rhema_core::RhemaError {
    fn from(err: types::LocomoError) -> Self {
        rhema_core::RhemaError::SystemError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_locomo_metrics_creation() {
        let metrics = LocomoMetrics::new();
        assert_eq!(metrics.context_retrieval_latency.as_secs(), 0);
        assert_eq!(metrics.context_relevance_score, 0.0);
    }

    #[tokio::test]
    async fn test_benchmark_engine_creation() {
        let engine = LocomoBenchmarkEngine::new_dummy();
        // Test that the engine was created successfully by running a simple benchmark
        let result = engine.run_all_benchmarks().await.unwrap();
        assert!(result.results.len() > 0);
    }

    #[tokio::test]
    async fn test_quality_assessor_creation() {
        let assessor = ContextQualityAssessor::new_dummy();
        let score = assessor.assess_context_quality_dummy().await;
        assert!(score.overall_score >= 0.0 && score.overall_score <= 1.0);
    }

    #[tokio::test]
    async fn test_benchmark_execution() {
        let engine = LocomoBenchmarkEngine::new_dummy();
        let result = engine.run_all_benchmarks().await.unwrap();
        assert!(result.results.len() > 0);
        assert!(result.summary.total_benchmarks > 0);
    }

    #[tokio::test]
    async fn test_validation_framework() {
        let baseline_metrics = LocomoMetrics::new();
        let framework = LocomoValidationFramework::new(baseline_metrics, Default::default());
        let validations = framework.validate_improvements().await.unwrap();
        assert!(validations.len() > 0);
    }

    #[tokio::test]
    async fn test_context_optimizer() {
        let optimizer = ContextOptimizer::new(Default::default());
        let context = types::Context {
            id: "test".to_string(),
            content: "This is a test context for optimization.".to_string(),
            size_bytes: 100,
            scope_path: Some("test-scope".to_string()),
            content_type: types::ContentType::Documentation,
            semantic_tags: vec!["test".to_string()],
            metadata: types::ContextMetadata {
                created_at: chrono::Utc::now(),
                last_modified: chrono::Utc::now(),
                version: "1.0.0".to_string(),
                author: Some("test".to_string()),
                tags: vec!["test".to_string()],
                dependencies: vec![],
                complexity_score: 0.5,
            },
        };
        
        let result = optimizer.optimize_context(&context, 0.9).await.unwrap();
        assert!(result.success);
        assert!(result.optimization_actions.len() > 0);
    }
} 