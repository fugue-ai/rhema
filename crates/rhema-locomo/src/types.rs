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

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Error types for LOCOMO operations
#[derive(Error, Debug)]
pub enum LocomoError {
    #[error("Benchmark error: {0}")]
    BenchmarkError(String),

    #[error("Quality assessment error: {0}")]
    QualityAssessmentError(String),

    #[error("Metrics collection error: {0}")]
    MetricsCollectionError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Optimization error: {0}")]
    OptimizationError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Context error: {0}")]
    ContextError(String),

    #[error("Performance error: {0}")]
    PerformanceError(String),
}

/// Benchmark types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenchmarkType {
    ContextRetrieval,
    ContextCompression,
    ContextPersistence,
    AIAgentOptimization,
    CrossScopeIntegration,
    QualityAssessment,
}

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityMetric {
    Relevance,
    Compression,
    Persistence,
    AIOptimization,
    CrossScopeIntegration,
    ContextEvolution,
}

/// Optimization strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    SemanticSummarization,
    ContextPruning,
    IntelligentFiltering,
    ContextEnhancement,
    SemanticEnrichment,
    CrossReferenceLinking,
    CompressionOptimization,
    RelevanceOptimization,
}

/// Context size categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextSize {
    Small,      // 1-10KB
    Medium,     // 10-100KB
    Large,      // 100KB-1MB
    VeryLarge,  // 1MB+
    MultiScope, // Cross-scope context
}

/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Gzip,
    Brotli,
    Zstd,
    SemanticCompression,
    KnowledgeGraphCompression,
    HierarchicalCompression,
}

/// Relevance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevanceThreshold {
    pub minimum_score: f64,
    pub target_score: f64,
    pub high_score: f64,
}

impl Default for RelevanceThreshold {
    fn default() -> Self {
        Self {
            minimum_score: 0.6,
            target_score: 0.8,
            high_score: 0.9,
        }
    }
}

/// Context information for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub id: String,
    pub content: String,
    pub size_bytes: usize,
    pub scope_path: Option<String>,
    pub content_type: ContentType,
    pub semantic_tags: Vec<String>,
    pub metadata: ContextMetadata,
}

/// Content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Documentation,
    Code,
    Configuration,
    Knowledge,
    Decision,
    Pattern,
    Todo,
    Insight,
}

/// Context metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub complexity_score: f64,
}

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub name: String,
    pub benchmark_type: BenchmarkType,
    pub context_size: ContextSize,
    pub expected_latency: Duration,
    pub relevance_threshold: RelevanceThreshold,
    pub compression_target: Option<f64>,
    pub quality_threshold: Option<f64>,
    pub test_scenarios: Vec<String>,
    pub iterations: usize,
    pub warmup_iterations: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            name: "default_benchmark".to_string(),
            benchmark_type: BenchmarkType::ContextRetrieval,
            context_size: ContextSize::Small,
            expected_latency: Duration::from_millis(50),
            relevance_threshold: RelevanceThreshold::default(),
            compression_target: Some(0.7),
            quality_threshold: Some(0.8),
            test_scenarios: vec!["default_scenario".to_string()],
            iterations: 100,
            warmup_iterations: 10,
        }
    }
}

/// Benchmark scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScenario {
    pub name: String,
    pub description: String,
    pub context_generator: ScenarioContextGenerator,
    pub query_generator: Option<ScenarioQueryGenerator>,
    pub expected_outcomes: Vec<ExpectedOutcome>,
}

/// Context generator for scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioContextGenerator {
    pub generator_type: ContextGeneratorType,
    pub parameters: serde_json::Value,
}

/// Context generator types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextGeneratorType {
    Synthetic,
    RealWorld,
    Mixed,
    StressTest,
}

/// Query generator for scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioQueryGenerator {
    pub generator_type: QueryGeneratorType,
    pub parameters: serde_json::Value,
}

/// Query generator types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryGeneratorType {
    Random,
    Semantic,
    Structured,
    Complex,
}

/// Expected outcomes for benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedOutcome {
    pub metric: QualityMetric,
    pub minimum_value: f64,
    pub target_value: f64,
    pub maximum_value: f64,
}

/// Benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub benchmark_name: String,
    pub scenario_name: String,
    pub metrics: BenchmarkMetrics,
    pub performance: PerformanceMetrics,
    pub quality: QualityMetrics,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub duration: Duration,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Benchmark metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    pub context_retrieval_latency: Duration,
    pub context_relevance_score: f64,
    pub context_compression_ratio: f64,
    pub cross_scope_integration_quality: f64,
    pub context_persistence_accuracy: f64,
    pub ai_agent_optimization_score: f64,
    pub context_quality_assessment: f64,
    pub context_evolution_tracking: f64,
}

impl Default for BenchmarkMetrics {
    fn default() -> Self {
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
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub mean_duration: Duration,
    pub median_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub standard_deviation: Duration,
    pub throughput: f64,
    pub latency_p95: Duration,
    pub latency_p99: Duration,
    pub success_rate: f64,
    pub error_count: usize,
}

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub overall_quality_score: f64,
    pub relevance_score: f64,
    pub compression_score: f64,
    pub persistence_score: f64,
    pub ai_optimization_score: f64,
    pub cross_scope_score: f64,
    pub evolution_score: f64,
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            overall_quality_score: 0.0,
            relevance_score: 0.0,
            compression_score: 0.0,
            persistence_score: 0.0,
            ai_optimization_score: 0.0,
            cross_scope_score: 0.0,
            evolution_score: 0.0,
        }
    }
}
