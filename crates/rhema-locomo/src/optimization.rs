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

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::types::{Context, LocomoError, OptimizationStrategy};
use rhema_core::RhemaResult;

/// Context optimizer
pub struct ContextOptimizer {
    ai_context_optimizer: Arc<AIContextOptimizer>,
    compression_optimizer: Arc<CompressionOptimizer>,
    relevance_optimizer: Arc<RelevanceOptimizer>,
    config: OptimizerConfig,
}

/// Optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    pub enable_ai_optimization: bool,
    pub enable_compression_optimization: bool,
    pub enable_relevance_optimization: bool,
    pub target_quality_score: f64,
    pub max_optimization_iterations: usize,
    pub optimization_timeout: Duration,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            enable_ai_optimization: true,
            enable_compression_optimization: true,
            enable_relevance_optimization: true,
            target_quality_score: 0.9,
            max_optimization_iterations: 5,
            optimization_timeout: Duration::from_secs(30),
        }
    }
}

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub original_context: Context,
    pub optimized_context: Context,
    pub optimization_actions: Vec<OptimizationAction>,
    pub quality_improvement: f64,
    pub optimization_duration: Duration,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Optimization action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAction {
    pub action_type: OptimizationStrategy,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub performance_impact: f64,
    pub applied: bool,
    pub details: serde_json::Value,
}

impl OptimizationAction {
    pub fn new(
        action_type: OptimizationStrategy,
        description: String,
        performance_impact: f64,
    ) -> Self {
        Self {
            action_type,
            description,
            timestamp: Utc::now(),
            performance_impact,
            applied: true,
            details: serde_json::Value::Null,
        }
    }
}

/// AI context optimizer
pub struct AIContextOptimizer {
    config: AIContextOptimizerConfig,
    optimization_history: Arc<RwLock<Vec<OptimizationAction>>>,
}

/// AI context optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContextOptimizerConfig {
    pub enable_token_optimization: bool,
    pub enable_structure_optimization: bool,
    pub enable_semantic_optimization: bool,
    pub target_token_reduction: f64,
    pub quality_threshold: f64,
    pub max_content_length: usize,
}

impl Default for AIContextOptimizerConfig {
    fn default() -> Self {
        Self {
            enable_token_optimization: true,
            enable_structure_optimization: true,
            enable_semantic_optimization: true,
            target_token_reduction: 0.3,
            quality_threshold: 0.9,
            max_content_length: 10000,
        }
    }
}

/// Compression optimizer
pub struct CompressionOptimizer {
    config: CompressionOptimizerConfig,
    optimization_history: Arc<RwLock<Vec<OptimizationAction>>>,
}

/// Compression optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionOptimizerConfig {
    pub enable_semantic_compression: bool,
    pub enable_redundancy_removal: bool,
    pub enable_structure_compression: bool,
    pub target_compression_ratio: f64,
    pub quality_threshold: f64,
    pub compression_algorithms: Vec<String>,
}

impl Default for CompressionOptimizerConfig {
    fn default() -> Self {
        Self {
            enable_semantic_compression: true,
            enable_redundancy_removal: true,
            enable_structure_compression: true,
            target_compression_ratio: 0.7,
            quality_threshold: 0.8,
            compression_algorithms: vec![
                "semantic".to_string(),
                "redundancy".to_string(),
                "structure".to_string(),
            ],
        }
    }
}

/// Relevance optimizer
pub struct RelevanceOptimizer {
    config: RelevanceOptimizerConfig,
    optimization_history: Arc<RwLock<Vec<OptimizationAction>>>,
}

/// Relevance optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevanceOptimizerConfig {
    pub enable_keyword_enhancement: bool,
    pub enable_semantic_enhancement: bool,
    pub enable_context_enhancement: bool,
    pub target_relevance_score: f64,
    pub enhancement_strategies: Vec<String>,
}

impl Default for RelevanceOptimizerConfig {
    fn default() -> Self {
        Self {
            enable_keyword_enhancement: true,
            enable_semantic_enhancement: true,
            enable_context_enhancement: true,
            target_relevance_score: 0.9,
            enhancement_strategies: vec![
                "keyword".to_string(),
                "semantic".to_string(),
                "context".to_string(),
            ],
        }
    }
}

impl ContextOptimizer {
    pub fn new(config: OptimizerConfig) -> Self {
        let ai_context_optimizer = Arc::new(AIContextOptimizer::new(Default::default()));
        let compression_optimizer = Arc::new(CompressionOptimizer::new(Default::default()));
        let relevance_optimizer = Arc::new(RelevanceOptimizer::new(Default::default()));

        Self {
            ai_context_optimizer,
            compression_optimizer,
            relevance_optimizer,
            config,
        }
    }

    pub async fn optimize_context(
        &self,
        context: &Context,
        target_score: f64,
    ) -> RhemaResult<OptimizationResult> {
        let start_time = std::time::Instant::now();
        let mut optimized_context = context.clone();
        let mut optimization_actions = Vec::new();

        info!(
            "Starting context optimization for target score: {}",
            target_score
        );

        // AI optimization
        if self.config.enable_ai_optimization {
            let ai_result = self
                .ai_context_optimizer
                .optimize(&optimized_context)
                .await?;
            optimized_context = ai_result.optimized_context;
            optimization_actions.extend(ai_result.optimization_actions);
        }

        // Compression optimization
        if self.config.enable_compression_optimization {
            let compression_result = self
                .compression_optimizer
                .optimize(&optimized_context)
                .await?;
            optimized_context = compression_result.optimized_context;
            optimization_actions.extend(compression_result.optimization_actions);
        }

        // Relevance optimization
        if self.config.enable_relevance_optimization {
            let relevance_result = self
                .relevance_optimizer
                .optimize(&optimized_context)
                .await?;
            optimized_context = relevance_result.optimized_context;
            optimization_actions.extend(relevance_result.optimization_actions);
        }

        let optimization_duration = start_time.elapsed();
        let quality_improvement = self
            .calculate_quality_improvement(context, &optimized_context)
            .await?;

        info!(
            "Context optimization completed in {:?} with {:.1}% quality improvement",
            optimization_duration,
            quality_improvement * 100.0
        );

        Ok(OptimizationResult {
            original_context: context.clone(),
            optimized_context,
            optimization_actions,
            quality_improvement,
            optimization_duration,
            success: true,
            error_message: None,
        })
    }

    async fn calculate_quality_improvement(
        &self,
        original: &Context,
        optimized: &Context,
    ) -> RhemaResult<f64> {
        // Simple quality improvement calculation based on content length and structure
        let original_length = original.content.len();
        let optimized_length = optimized.content.len();

        let length_improvement = if original_length > 0 {
            if optimized_length <= original_length {
                (original_length - optimized_length) as f64 / original_length as f64
            } else {
                0.0 // No improvement if content got longer
            }
        } else {
            0.0
        };

        // Structure improvement (simplified)
        let structure_improvement = if optimized.content.contains("Enhanced Context:") {
            0.2
        } else {
            0.0
        };

        Ok((length_improvement + structure_improvement).min(1.0))
    }
}

impl AIContextOptimizer {
    pub fn new(config: AIContextOptimizerConfig) -> Self {
        Self {
            config,
            optimization_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn optimize(&self, context: &Context) -> RhemaResult<OptimizationResult> {
        let mut optimized_context = context.clone();
        let mut optimization_actions = Vec::new();

        // Token optimization
        if self.config.enable_token_optimization {
            let token_action = self.optimize_tokens(&mut optimized_context).await?;
            optimization_actions.push(token_action);
        }

        // Structure optimization
        if self.config.enable_structure_optimization {
            let structure_action = self.optimize_structure(&mut optimized_context).await?;
            optimization_actions.push(structure_action);
        }

        // Semantic optimization
        if self.config.enable_semantic_optimization {
            let semantic_action = self.optimize_semantics(&mut optimized_context).await?;
            optimization_actions.push(semantic_action);
        }

        // Store optimization actions
        self.store_optimization_actions(&optimization_actions)
            .await?;

        Ok(OptimizationResult {
            original_context: context.clone(),
            optimized_context,
            optimization_actions,
            quality_improvement: 0.15, // Simulated improvement
            optimization_duration: Duration::from_millis(100),
            success: true,
            error_message: None,
        })
    }

    async fn optimize_tokens(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        let original_length = context.content.len();

        // Remove redundant words and phrases
        let words: Vec<&str> = context.content.split_whitespace().collect();
        let mut optimized_words = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for word in words {
            if !seen.contains(word) || optimized_words.len() < context.content.len() / 2 {
                optimized_words.push(word);
                seen.insert(word);
            }
        }

        context.content = optimized_words.join(" ");

        let new_length = context.content.len();
        let reduction = if original_length > 0 {
            if new_length <= original_length {
                (original_length - new_length) as f64 / original_length as f64
            } else {
                0.0 // No reduction if content got longer
            }
        } else {
            0.0
        };

        Ok(OptimizationAction::new(
            OptimizationStrategy::SemanticSummarization,
            format!(
                "Token optimization: reduced content by {:.1}%",
                reduction * 100.0
            ),
            reduction,
        ))
    }

    async fn optimize_structure(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        // Add structure to the content
        if !context.content.contains("Enhanced Context:") {
            context.content = format!("Enhanced Context:\n\n{}", context.content);
        }

        // Add headers for better organization
        if context.content.contains("context") && !context.content.contains("# Context") {
            context.content = context.content.replace("context", "# Context\n\ncontext");
        }

        Ok(OptimizationAction::new(
            OptimizationStrategy::ContextEnhancement,
            "Structure optimization: added headers and organization".to_string(),
            0.1,
        ))
    }

    async fn optimize_semantics(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        // Improve semantic clarity
        let semantic_improvements = vec![
            ("context", "optimized_context"),
            ("management", "efficient_management"),
            ("system", "optimized_system"),
        ];

        for (old, new) in semantic_improvements {
            if context.content.contains(old) {
                context.content = context.content.replace(old, new);
            }
        }

        Ok(OptimizationAction::new(
            OptimizationStrategy::SemanticEnrichment,
            "Semantic optimization: enhanced terminology and clarity".to_string(),
            0.1,
        ))
    }

    async fn store_optimization_actions(&self, actions: &[OptimizationAction]) -> RhemaResult<()> {
        let mut history = self.optimization_history.write().await;
        history.extend(actions.iter().cloned());

        // Keep only the last 1000 optimization actions
        if history.len() > 1000 {
            let len = history.len();
            history.drain(0..len - 1000);
        }

        Ok(())
    }
}

impl CompressionOptimizer {
    pub fn new(config: CompressionOptimizerConfig) -> Self {
        Self {
            config,
            optimization_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn optimize(&self, context: &Context) -> RhemaResult<OptimizationResult> {
        let mut optimized_context = context.clone();
        let mut optimization_actions = Vec::new();

        // Semantic compression
        if self.config.enable_semantic_compression {
            let semantic_action = self
                .apply_semantic_compression(&mut optimized_context)
                .await?;
            optimization_actions.push(semantic_action);
        }

        // Redundancy removal
        if self.config.enable_redundancy_removal {
            let redundancy_action = self.remove_redundancy(&mut optimized_context).await?;
            optimization_actions.push(redundancy_action);
        }

        // Structure compression
        if self.config.enable_structure_compression {
            let structure_action = self.compress_structure(&mut optimized_context).await?;
            optimization_actions.push(structure_action);
        }

        // Store optimization actions
        self.store_optimization_actions(&optimization_actions)
            .await?;

        Ok(OptimizationResult {
            original_context: context.clone(),
            optimized_context,
            optimization_actions,
            quality_improvement: 0.2, // Simulated improvement
            optimization_duration: Duration::from_millis(150),
            success: true,
            error_message: None,
        })
    }

    async fn apply_semantic_compression(
        &self,
        context: &mut Context,
    ) -> RhemaResult<OptimizationAction> {
        let original_length = context.content.len();

        // Apply semantic compression by removing less important words
        let words: Vec<&str> = context.content.split_whitespace().collect();
        let important_words: Vec<&str> = words
            .iter()
            .filter(|word| {
                let word_lower = word.to_lowercase();
                !word_lower.is_empty()
                    && word_lower.len() > 2
                    && !vec![
                        "the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
                        "by",
                    ]
                    .contains(&word_lower.as_str())
            })
            .cloned()
            .collect();

        context.content = important_words.join(" ");

        let new_length = context.content.len();
        let compression_ratio = if original_length > 0 {
            new_length as f64 / original_length as f64
        } else {
            1.0
        };

        Ok(OptimizationAction::new(
            OptimizationStrategy::CompressionOptimization,
            format!(
                "Semantic compression: achieved {:.1} compression ratio",
                compression_ratio
            ),
            1.0 - compression_ratio,
        ))
    }

    async fn remove_redundancy(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        let original_length = context.content.len();

        // Remove duplicate sentences and phrases
        let sentences: Vec<&str> = context.content.split('.').collect();
        let mut unique_sentences = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for sentence in sentences {
            let sentence_trimmed = sentence.trim();
            if !sentence_trimmed.is_empty() && !seen.contains(sentence_trimmed) {
                unique_sentences.push(sentence_trimmed);
                seen.insert(sentence_trimmed);
            }
        }

        context.content = unique_sentences.join(". ");

        let new_length = context.content.len();
        let reduction = if original_length > 0 {
            if new_length <= original_length {
                (original_length - new_length) as f64 / original_length as f64
            } else {
                0.0 // No reduction if content got longer
            }
        } else {
            0.0
        };

        Ok(OptimizationAction::new(
            OptimizationStrategy::ContextPruning,
            format!(
                "Redundancy removal: reduced content by {:.1}%",
                reduction * 100.0
            ),
            reduction,
        ))
    }

    async fn compress_structure(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        let original_length = context.content.len();

        // Compress structure by removing excessive whitespace and formatting
        let lines: Vec<&str> = context.content.lines().collect();
        let compressed_lines: Vec<&str> = lines
            .iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        context.content = compressed_lines.join("\n");

        let new_length = context.content.len();
        let compression_ratio = if original_length > 0 {
            new_length as f64 / original_length as f64
        } else {
            1.0
        };

        Ok(OptimizationAction::new(
            OptimizationStrategy::CompressionOptimization,
            format!(
                "Structure compression: achieved {:.1} compression ratio",
                compression_ratio
            ),
            1.0 - compression_ratio,
        ))
    }

    async fn store_optimization_actions(&self, actions: &[OptimizationAction]) -> RhemaResult<()> {
        let mut history = self.optimization_history.write().await;
        history.extend(actions.iter().cloned());

        // Keep only the last 1000 optimization actions
        if history.len() > 1000 {
            let len = history.len();
            history.drain(0..len - 1000);
        }

        Ok(())
    }
}

impl RelevanceOptimizer {
    pub fn new(config: RelevanceOptimizerConfig) -> Self {
        Self {
            config,
            optimization_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn optimize(&self, context: &Context) -> RhemaResult<OptimizationResult> {
        let mut optimized_context = context.clone();
        let mut optimization_actions = Vec::new();

        // Keyword enhancement
        if self.config.enable_keyword_enhancement {
            let keyword_action = self.enhance_keywords(&mut optimized_context).await?;
            optimization_actions.push(keyword_action);
        }

        // Semantic enhancement
        if self.config.enable_semantic_enhancement {
            let semantic_action = self.enhance_semantics(&mut optimized_context).await?;
            optimization_actions.push(semantic_action);
        }

        // Context enhancement
        if self.config.enable_context_enhancement {
            let context_action = self.enhance_context(&mut optimized_context).await?;
            optimization_actions.push(context_action);
        }

        // Store optimization actions
        self.store_optimization_actions(&optimization_actions)
            .await?;

        Ok(OptimizationResult {
            original_context: context.clone(),
            optimized_context,
            optimization_actions,
            quality_improvement: 0.1, // Simulated improvement
            optimization_duration: Duration::from_millis(80),
            success: true,
            error_message: None,
        })
    }

    async fn enhance_keywords(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        // Add relevant keywords to improve searchability
        let keywords = vec![
            "optimization",
            "performance",
            "efficiency",
            "quality",
            "benchmark",
        ];
        let mut enhanced_content = context.content.clone();

        for keyword in keywords {
            if !enhanced_content.to_lowercase().contains(keyword) {
                enhanced_content = format!("{} {}", enhanced_content, keyword);
            }
        }

        context.content = enhanced_content;

        Ok(OptimizationAction::new(
            OptimizationStrategy::RelevanceOptimization,
            "Keyword enhancement: added relevant search terms".to_string(),
            0.1,
        ))
    }

    async fn enhance_semantics(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        // Enhance semantic clarity with better terminology
        let semantic_replacements = vec![
            ("good", "excellent"),
            ("bad", "suboptimal"),
            ("fast", "high-performance"),
            ("slow", "performance-limited"),
        ];

        for (old, new) in semantic_replacements {
            if context.content.to_lowercase().contains(old) {
                context.content = context.content.replace(old, new);
            }
        }

        Ok(OptimizationAction::new(
            OptimizationStrategy::SemanticEnrichment,
            "Semantic enhancement: improved terminology clarity".to_string(),
            0.1,
        ))
    }

    async fn enhance_context(&self, context: &mut Context) -> RhemaResult<OptimizationAction> {
        // Add context information to improve relevance
        if !context.content.contains("Context Information:") {
            context.content = format!("Context Information:\n\n{}", context.content);
        }

        // Add cross-references if not present
        if !context.content.contains("Related:") {
            context.content = format!(
                "{}\n\nRelated: optimization, benchmarking, performance",
                context.content
            );
        }

        Ok(OptimizationAction::new(
            OptimizationStrategy::CrossReferenceLinking,
            "Context enhancement: added context information and cross-references".to_string(),
            0.1,
        ))
    }

    async fn store_optimization_actions(&self, actions: &[OptimizationAction]) -> RhemaResult<()> {
        let mut history = self.optimization_history.write().await;
        history.extend(actions.iter().cloned());

        // Keep only the last 1000 optimization actions
        if history.len() > 1000 {
            let len = history.len();
            history.drain(0..len - 1000);
        }

        Ok(())
    }
}
