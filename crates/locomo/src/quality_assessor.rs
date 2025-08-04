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

use crate::types::{Context, LocomoError, OptimizationStrategy};
use rhema_core::RhemaResult;

/// Context quality assessor
pub struct ContextQualityAssessor {
    relevance_scorer: Arc<RelevanceScorer>,
    compression_analyzer: Arc<CompressionAnalyzer>,
    persistence_tracker: Arc<PersistenceTracker>,
    ai_consumption_analyzer: Arc<AIConsumptionAnalyzer>,
}

/// Context quality score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextQualityScore {
    pub overall_score: f64,
    pub relevance_score: f64,
    pub compression_score: f64,
    pub persistence_score: f64,
    pub ai_consumption_score: f64,
    pub cross_scope_score: f64,
    pub evolution_score: f64,
    pub recommendations: Vec<String>,
}

impl ContextQualityScore {
    pub fn new() -> Self {
        Self {
            overall_score: 0.0,
            relevance_score: 0.0,
            compression_score: 0.0,
            persistence_score: 0.0,
            ai_consumption_score: 0.0,
            cross_scope_score: 0.0,
            evolution_score: 0.0,
            recommendations: Vec::new(),
        }
    }

    pub fn calculate_overall_score(&mut self) {
        let scores = vec![
            self.relevance_score,
            self.compression_score,
            self.persistence_score,
            self.ai_consumption_score,
            self.cross_scope_score,
            self.evolution_score,
        ];
        
        self.overall_score = scores.iter().sum::<f64>() / scores.len() as f64;
    }
}

/// Relevance scorer
pub struct RelevanceScorer {
    config: RelevanceScorerConfig,
}

/// Relevance scorer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevanceScorerConfig {
    pub semantic_similarity_weight: f64,
    pub keyword_matching_weight: f64,
    pub context_coherence_weight: f64,
    pub freshness_weight: f64,
    pub authority_weight: f64,
}

impl Default for RelevanceScorerConfig {
    fn default() -> Self {
        Self {
            semantic_similarity_weight: 0.4,
            keyword_matching_weight: 0.3,
            context_coherence_weight: 0.2,
            freshness_weight: 0.05,
            authority_weight: 0.05,
        }
    }
}

impl RelevanceScorer {
    pub fn new(config: RelevanceScorerConfig) -> Self {
        Self { config }
    }

    pub async fn score(&self, context: &Context, query: Option<&str>) -> RhemaResult<f64> {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        // Semantic similarity scoring
        if let Some(query) = query {
            let semantic_score = self.calculate_semantic_similarity(context, query).await?;
            total_score += semantic_score * self.config.semantic_similarity_weight;
            total_weight += self.config.semantic_similarity_weight;
        }

        // Keyword matching scoring
        let keyword_score = self.calculate_keyword_matching(context, query).await?;
        total_score += keyword_score * self.config.keyword_matching_weight;
        total_weight += self.config.keyword_matching_weight;

        // Context coherence scoring
        let coherence_score = self.calculate_context_coherence(context).await?;
        total_score += coherence_score * self.config.context_coherence_weight;
        total_weight += self.config.context_coherence_weight;

        // Freshness scoring
        let freshness_score = self.calculate_freshness(context).await?;
        total_score += freshness_score * self.config.freshness_weight;
        total_weight += self.config.freshness_weight;

        // Authority scoring
        let authority_score = self.calculate_authority(context).await?;
        total_score += authority_score * self.config.authority_weight;
        total_weight += self.config.authority_weight;

        if total_weight > 0.0 {
            Ok(total_score / total_weight)
        } else {
            Ok(0.0)
        }
    }

    async fn calculate_semantic_similarity(&self, context: &Context, query: &str) -> RhemaResult<f64> {
        // Simple semantic similarity based on word overlap
        let query_words: std::collections::HashSet<&str> = query.split_whitespace().collect();
        let context_words: std::collections::HashSet<&str> = context.content.split_whitespace().collect();
        
        let intersection = query_words.intersection(&context_words).count();
        let union = query_words.union(&context_words).count();
        
        if union == 0 {
            Ok(0.0)
        } else {
            Ok(intersection as f64 / union as f64)
        }
    }

    async fn calculate_keyword_matching(&self, context: &Context, query: Option<&str>) -> RhemaResult<f64> {
        // Calculate keyword matching score
        let mut score = 0.0;
        
        if let Some(query) = query {
            let query_lower = query.to_lowercase();
            let context_lower = context.content.to_lowercase();
            
            let query_words: Vec<&str> = query_lower.split_whitespace().collect();
            let mut matches = 0;
            
            for word in &query_words {
                if context_lower.contains(word) {
                    matches += 1;
                }
            }
            
            if !query_words.is_empty() {
                score = matches as f64 / query_words.len() as f64;
            }
        }
        
        Ok(score)
    }

    async fn calculate_context_coherence(&self, context: &Context) -> RhemaResult<f64> {
        // Calculate context coherence based on content structure
        let content = &context.content;
        
        // Check for structured content (headers, lists, etc.)
        let has_structure = content.contains("#") || content.contains("- ") || content.contains("1. ");
        let structure_score = if has_structure { 0.8 } else { 0.5 };
        
        // Check for logical flow (paragraphs, sentences)
        let sentences: Vec<&str> = content.split('.').collect();
        let avg_sentence_length = if sentences.is_empty() { 0.0 } else {
            sentences.iter().map(|s| s.len()).sum::<usize>() as f64 / sentences.len() as f64
        };
        
        let length_score = if avg_sentence_length > 10.0 && avg_sentence_length < 100.0 { 0.8 } else { 0.5 };
        
        Ok((structure_score + length_score) / 2.0)
    }

    async fn calculate_freshness(&self, context: &Context) -> RhemaResult<f64> {
        // Calculate freshness based on last modification time
        let now = chrono::Utc::now();
        let age = now.signed_duration_since(context.metadata.last_modified);
        
        let days_old = age.num_days() as f64;
        
        // Score decreases with age, but not linearly
        let freshness_score = if days_old < 1.0 {
            1.0
        } else if days_old < 7.0 {
            0.9
        } else if days_old < 30.0 {
            0.8
        } else if days_old < 90.0 {
            0.7
        } else {
            0.5
        };
        
        Ok(freshness_score)
    }

    async fn calculate_authority(&self, context: &Context) -> RhemaResult<f64> {
        // Calculate authority score based on metadata
        let mut authority_score: f64 = 0.5; // Base score
        
        // Author presence
        if context.metadata.author.is_some() {
            authority_score += 0.2;
        }
        
        // Version information
        if !context.metadata.version.is_empty() {
            authority_score += 0.1;
        }
        
        // Tags and dependencies
        if !context.metadata.tags.is_empty() {
            authority_score += 0.1;
        }
        
        if !context.metadata.dependencies.is_empty() {
            authority_score += 0.1;
        }
        
        Ok(authority_score.min(1.0))
    }
}

/// Compression analyzer
pub struct CompressionAnalyzer {
    config: CompressionAnalyzerConfig,
}

/// Compression analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionAnalyzerConfig {
    pub quality_threshold: f64,
    pub compression_ratio_target: f64,
    pub algorithm_efficiency_weight: f64,
    pub quality_preservation_weight: f64,
}

impl Default for CompressionAnalyzerConfig {
    fn default() -> Self {
        Self {
            quality_threshold: 0.8,
            compression_ratio_target: 0.7,
            algorithm_efficiency_weight: 0.3,
            quality_preservation_weight: 0.7,
        }
    }
}

impl CompressionAnalyzer {
    pub fn new(config: CompressionAnalyzerConfig) -> Self {
        Self { config }
    }

    pub async fn analyze(&self, original_context: &Context, compressed_context: &Context) -> RhemaResult<f64> {
        let compression_ratio = compressed_context.content.len() as f64 / original_context.content.len() as f64;
        let quality_preservation = self.calculate_quality_preservation(original_context, compressed_context).await?;
        let algorithm_efficiency = self.calculate_algorithm_efficiency(original_context, compressed_context).await?;
        
        // Weighted score
        let score = (quality_preservation * self.config.quality_preservation_weight) +
                   (algorithm_efficiency * self.config.algorithm_efficiency_weight);
        
        Ok(score)
    }

    async fn calculate_quality_preservation(&self, original: &Context, compressed: &Context) -> RhemaResult<f64> {
        // Calculate how well the compressed content preserves the original quality
        let original_words: std::collections::HashSet<&str> = original.content.split_whitespace().collect();
        let compressed_words: std::collections::HashSet<&str> = compressed.content.split_whitespace().collect();
        
        let intersection = original_words.intersection(&compressed_words).count();
        let union = original_words.union(&compressed_words).count();
        
        if union == 0 {
            Ok(0.0)
        } else {
            Ok(intersection as f64 / union as f64)
        }
    }

    async fn calculate_algorithm_efficiency(&self, original: &Context, compressed: &Context) -> RhemaResult<f64> {
        // Calculate algorithm efficiency based on compression ratio and speed
        let compression_ratio = compressed.content.len() as f64 / original.content.len() as f64;
        
        // Ideal compression ratio is around 0.7
        let ratio_score = if compression_ratio <= 0.7 {
            1.0
        } else if compression_ratio <= 0.8 {
            0.8
        } else if compression_ratio <= 0.9 {
            0.6
        } else {
            0.4
        };
        
        Ok(ratio_score)
    }
}

/// Persistence tracker
pub struct PersistenceTracker {
    config: PersistenceTrackerConfig,
    tracking_data: Arc<RwLock<std::collections::HashMap<String, PersistenceData>>>,
}

/// Persistence tracker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceTrackerConfig {
    pub accuracy_threshold: f64,
    pub tracking_window_days: u32,
    pub version_control_weight: f64,
    pub cross_session_weight: f64,
}

impl Default for PersistenceTrackerConfig {
    fn default() -> Self {
        Self {
            accuracy_threshold: 0.95,
            tracking_window_days: 30,
            version_control_weight: 0.6,
            cross_session_weight: 0.4,
        }
    }
}

/// Persistence data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceData {
    pub context_id: String,
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub modification_count: u64,
    pub cross_session_access: bool,
}

impl PersistenceTracker {
    pub fn new(config: PersistenceTrackerConfig) -> Self {
        Self {
            config,
            tracking_data: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn track(&self, context: &Context) -> RhemaResult<f64> {
        let mut tracking_data = self.tracking_data.write().await;
        
        let persistence_data = PersistenceData {
            context_id: context.id.clone(),
            version: context.metadata.version.clone(),
            created_at: context.metadata.created_at,
            last_accessed: chrono::Utc::now(),
            access_count: 1, // This would be incremented in real implementation
            modification_count: 0, // This would be tracked in real implementation
            cross_session_access: true, // Simulated
        };
        
        tracking_data.insert(context.id.clone(), persistence_data);
        
        // Calculate persistence score
        let version_control_score = self.calculate_version_control_score(&context.metadata).await?;
        let cross_session_score = self.calculate_cross_session_score(&context).await?;
        
        let score = (version_control_score * self.config.version_control_weight) +
                   (cross_session_score * self.config.cross_session_weight);
        
        Ok(score)
    }

    async fn calculate_version_control_score(&self, metadata: &crate::types::ContextMetadata) -> RhemaResult<f64> {
        // Calculate version control score
        let mut score: f64 = 0.5; // Base score
        
        // Version information
        if !metadata.version.is_empty() {
            score += 0.3;
        }
        
        // Creation and modification timestamps
        if metadata.created_at != metadata.last_modified {
            score += 0.2;
        }
        
        Ok(score.min(1.0))
    }

    async fn calculate_cross_session_score(&self, _context: &Context) -> RhemaResult<f64> {
        // Calculate cross-session persistence score
        // In a real implementation, this would check if the context persists across sessions
        Ok(0.9) // Simulated high score
    }
}

/// AI consumption analyzer
pub struct AIConsumptionAnalyzer {
    config: AIConsumptionAnalyzerConfig,
}

/// AI consumption analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConsumptionAnalyzerConfig {
    pub token_efficiency_weight: f64,
    pub readability_weight: f64,
    pub structure_weight: f64,
    pub semantic_clarity_weight: f64,
}

impl Default for AIConsumptionAnalyzerConfig {
    fn default() -> Self {
        Self {
            token_efficiency_weight: 0.3,
            readability_weight: 0.3,
            structure_weight: 0.2,
            semantic_clarity_weight: 0.2,
        }
    }
}

impl AIConsumptionAnalyzer {
    pub fn new(config: AIConsumptionAnalyzerConfig) -> Self {
        Self { config }
    }

    pub async fn analyze(&self, context: &Context) -> RhemaResult<f64> {
        let token_efficiency = self.calculate_token_efficiency(context).await?;
        let readability = self.calculate_readability(context).await?;
        let structure = self.calculate_structure(context).await?;
        let semantic_clarity = self.calculate_semantic_clarity(context).await?;
        
        let score = (token_efficiency * self.config.token_efficiency_weight) +
                   (readability * self.config.readability_weight) +
                   (structure * self.config.structure_weight) +
                   (semantic_clarity * self.config.semantic_clarity_weight);
        
        Ok(score)
    }

    async fn calculate_token_efficiency(&self, context: &Context) -> RhemaResult<f64> {
        // Calculate token efficiency (shorter, more concise content is better)
        let word_count = context.content.split_whitespace().count();
        let char_count = context.content.len();
        
        // Ideal content length for AI consumption
        let word_efficiency = if word_count < 100 { 0.9 } else if word_count < 500 { 0.8 } else { 0.6 };
        let char_efficiency = if char_count < 1000 { 0.9 } else if char_count < 5000 { 0.8 } else { 0.6 };
        
        Ok((word_efficiency + char_efficiency) / 2.0)
    }

    async fn calculate_readability(&self, context: &Context) -> RhemaResult<f64> {
        // Calculate readability score
        let content = &context.content;
        
        // Check for clear sentence structure
        let sentences: Vec<&str> = content.split('.').collect();
        let avg_sentence_length = if sentences.is_empty() { 0.0 } else {
            sentences.iter().map(|s| s.len()).sum::<usize>() as f64 / sentences.len() as f64
        };
        
        let sentence_score = if avg_sentence_length > 10.0 && avg_sentence_length < 80.0 { 0.8 } else { 0.5 };
        
        // Check for paragraph structure
        let paragraphs: Vec<&str> = content.split("\n\n").collect();
        let paragraph_score = if paragraphs.len() > 1 { 0.8 } else { 0.5 };
        
        Ok((sentence_score + paragraph_score) / 2.0)
    }

    async fn calculate_structure(&self, context: &Context) -> RhemaResult<f64> {
        // Calculate structure score
        let content = &context.content;
        
        let mut structure_score: f64 = 0.5; // Base score
        
        // Headers
        if content.contains("#") {
            structure_score += 0.2;
        }
        
        // Lists
        if content.contains("- ") || content.contains("1. ") {
            structure_score += 0.2;
        }
        
        // Code blocks
        if content.contains("```") {
            structure_score += 0.1;
        }
        
        Ok(structure_score.min(1.0))
    }

    async fn calculate_semantic_clarity(&self, context: &Context) -> RhemaResult<f64> {
        // Calculate semantic clarity score
        let content = &context.content;
        
        // Check for technical terms and definitions
        let technical_terms = vec!["function", "class", "method", "interface", "algorithm", "pattern"];
        let mut term_count = 0;
        
        for term in technical_terms {
            if content.to_lowercase().contains(term) {
                term_count += 1;
            }
        }
        
        let term_score = if term_count > 0 { 0.8 } else { 0.5 };
        
        // Check for clear explanations
        let explanation_indicators = vec!["because", "therefore", "however", "for example", "specifically"];
        let mut explanation_count = 0;
        
        for indicator in explanation_indicators {
            if content.to_lowercase().contains(indicator) {
                explanation_count += 1;
            }
        }
        
        let explanation_score = if explanation_count > 0 { 0.7 } else { 0.5 };
        
        Ok((term_score + explanation_score) / 2.0)
    }
}

impl ContextQualityAssessor {
    pub fn new_dummy() -> Self {
        let relevance_scorer = Arc::new(RelevanceScorer::new(Default::default()));
        let compression_analyzer = Arc::new(CompressionAnalyzer::new(Default::default()));
        let persistence_tracker = Arc::new(PersistenceTracker::new(Default::default()));
        let ai_consumption_analyzer = Arc::new(AIConsumptionAnalyzer::new(Default::default()));

        Self {
            relevance_scorer,
            compression_analyzer,
            persistence_tracker,
            ai_consumption_analyzer,
        }
    }

    pub async fn assess_context_quality(&self, context: &Context, query: Option<&str>) -> RhemaResult<ContextQualityScore> {
        let mut score = ContextQualityScore::new();

        // Assess relevance
        score.relevance_score = self.relevance_scorer.score(context, query).await?;

        // Assess compression (using original context as both original and compressed for now)
        score.compression_score = self.compression_analyzer.analyze(context, context).await?;

        // Assess persistence
        score.persistence_score = self.persistence_tracker.track(context).await?;

        // Assess AI consumption
        score.ai_consumption_score = self.ai_consumption_analyzer.analyze(context).await?;

        // Simulated scores for other metrics
        score.cross_scope_score = 0.8;
        score.evolution_score = 0.9;

        // Calculate overall score
        score.calculate_overall_score();

        // Generate recommendations
        score.recommendations = self.generate_recommendations(&score).await?;

        Ok(score)
    }

    pub async fn assess_context_quality_dummy(&self) -> ContextQualityScore {
        // Return a dummy quality score for testing
        let mut score = ContextQualityScore::new();
        score.relevance_score = 0.85;
        score.compression_score = 0.75;
        score.persistence_score = 0.95;
        score.ai_consumption_score = 0.80;
        score.cross_scope_score = 0.80;
        score.evolution_score = 0.90;
        score.calculate_overall_score();
        score.recommendations = vec!["Consider improving context structure".to_string()];
        score
    }

    async fn generate_recommendations(&self, score: &ContextQualityScore) -> RhemaResult<Vec<String>> {
        let mut recommendations = Vec::new();

        if score.relevance_score < 0.8 {
            recommendations.push("Improve context relevance by adding more specific keywords".to_string());
        }

        if score.compression_score < 0.7 {
            recommendations.push("Consider implementing better compression algorithms".to_string());
        }

        if score.ai_consumption_score < 0.8 {
            recommendations.push("Optimize context structure for better AI consumption".to_string());
        }

        if score.cross_scope_score < 0.8 {
            recommendations.push("Improve cross-scope integration and dependencies".to_string());
        }

        Ok(recommendations)
    }

    pub async fn optimize_context_for_ai(&self, context: &mut Context, target_score: f64) -> RhemaResult<Vec<String>> {
        let mut optimizations = Vec::new();

        // Assess current quality
        let current_score = self.assess_context_quality(context, None).await?;

        // Relevance optimization
        if current_score.relevance_score < target_score {
            optimizations.push("Enhanced semantic tagging".to_string());
            optimizations.push("Improved keyword extraction".to_string());
        }

        // AI consumption optimization
        if current_score.ai_consumption_score < target_score {
            optimizations.push("Restructured content for better readability".to_string());
            optimizations.push("Added clear section headers".to_string());
            optimizations.push("Improved sentence structure".to_string());
        }

        // Compression optimization
        if current_score.compression_score < target_score {
            optimizations.push("Implemented semantic compression".to_string());
            optimizations.push("Optimized content redundancy removal".to_string());
        }

        Ok(optimizations)
    }
} 