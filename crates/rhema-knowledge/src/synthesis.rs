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
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, info, instrument};

use crate::types::{
    ContentType, KnowledgeResult, KnowledgeSynthesis, SemanticResult, SynthesisMetadata,
    SynthesisMethod,
};
use chrono::Datelike;

use super::{embedding::EmbeddingManager, search::SemanticSearchEngine, vector::VectorStore};

/// Error types for knowledge synthesis
#[derive(Error, Debug)]
pub enum SynthesisError {
    #[error("Search error: {0}")]
    SearchError(String),

    #[error("Embedding error: {0}")]
    EmbeddingError(String),

    #[error("Content processing error: {0}")]
    ContentProcessingError(String),

    #[error("Synthesis method error: {0}")]
    SynthesisMethodError(String),

    #[error("Insufficient data: {0}")]
    InsufficientData(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Knowledge synthesizer for combining information from multiple sources
pub struct KnowledgeSynthesizer {
    embedding_manager: Arc<EmbeddingManager>,
    vector_store: Arc<dyn VectorStore>,
    search_engine: Arc<SemanticSearchEngine>,
    config: SynthesisConfig,
}

/// Synthesis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisConfig {
    pub min_source_count: usize,
    pub max_source_count: usize,
    pub confidence_threshold: f32,
    pub enable_cross_scope: bool,
    pub enable_temporal_analysis: bool,
    pub enable_pattern_recognition: bool,
    pub synthesis_methods: Vec<SynthesisMethod>,
}

impl Default for SynthesisConfig {
    fn default() -> Self {
        Self {
            min_source_count: 2,
            max_source_count: 20,
            confidence_threshold: 0.7,
            enable_cross_scope: true,
            enable_temporal_analysis: true,
            enable_pattern_recognition: true,
            synthesis_methods: vec![
                SynthesisMethod::SemanticClustering,
                SynthesisMethod::CrossScopeCorrelation,
                SynthesisMethod::Hybrid,
            ],
        }
    }
}

impl KnowledgeSynthesizer {
    pub fn new_dummy() -> Self {
        Self {
            embedding_manager: Arc::new(EmbeddingManager::new_dummy()),
            vector_store: Arc::new(crate::vector::MockVectorStore::new(
                "synthesis_mock_collection".to_string(),
                1536,
                crate::types::DistanceMetric::Cosine,
            )),
            search_engine: Arc::new(SemanticSearchEngine::new_dummy()),
            config: SynthesisConfig::default(),
        }
    }

    pub async fn new(
        embedding_manager: Arc<EmbeddingManager>,
        vector_store: Arc<dyn VectorStore>,
    ) -> KnowledgeResult<Self> {
        let search_config = crate::types::SemanticSearchConfig {
            similarity_threshold: 0.6,
            max_results: 50,
            hybrid_search_enabled: true,
            reranking_enabled: true,
        };

        let search_engine = Arc::new(
            SemanticSearchEngine::new(
                embedding_manager.clone(),
                vector_store.clone(),
                search_config,
            )
            .await?,
        );

        Ok(Self {
            embedding_manager,
            vector_store,
            search_engine,
            config: SynthesisConfig::default(),
        })
    }

    /// Synthesize knowledge on a specific topic
    #[instrument(skip(self, topic))]
    pub async fn synthesize(
        &self,
        topic: &str,
        scope_path: Option<&str>,
    ) -> KnowledgeResult<KnowledgeSynthesis> {
        info!("Starting knowledge synthesis for topic: {}", topic);

        // Search for relevant content
        let search_results = if let Some(scope) = scope_path {
            self.search_engine
                .search_by_scope(topic, scope, self.config.max_source_count)
                .await?
        } else {
            self.search_engine
                .search_semantic(topic, self.config.max_source_count)
                .await?
        };

        if search_results.len() < self.config.min_source_count {
            return Err(SynthesisError::InsufficientData(format!(
                "Only {} sources found, need at least {}",
                search_results.len(),
                self.config.min_source_count
            ))
            .into());
        }

        // Group results by synthesis method
        let mut synthesis_results = Vec::new();

        for method in &self.config.synthesis_methods {
            let synthesized_content = self
                .synthesize_with_method(topic, &search_results, method)
                .await?;
            synthesis_results.push(synthesized_content);
        }

        // Combine synthesis results
        let final_synthesis = self
            .combine_synthesis_results(topic, synthesis_results)
            .await?;

        // Create synthesis metadata
        let metadata = SynthesisMetadata {
            synthesis_method: SynthesisMethod::Hybrid,
            source_count: search_results.len(),
            cross_scope: scope_path.is_none(),
            temporal_range: self.extract_temporal_range(&search_results).await?,
            semantic_clusters: self.extract_semantic_clusters(&search_results).await?,
        };

        let synthesis = KnowledgeSynthesis {
            synthesis_id: uuid::Uuid::new_v4().to_string(),
            topic: topic.to_string(),
            synthesized_content: final_synthesis,
            source_keys: search_results.iter().map(|r| r.cache_key.clone()).collect(),
            confidence_score: self.calculate_confidence_score(&search_results).await?,
            created_at: chrono::Utc::now(),
            metadata,
        };

        debug!(
            "Knowledge synthesis completed for topic: {} (confidence: {:.2})",
            topic, synthesis.confidence_score
        );

        Ok(synthesis)
    }

    /// Synthesize using a specific method
    async fn synthesize_with_method(
        &self,
        topic: &str,
        results: &[SemanticResult],
        method: &SynthesisMethod,
    ) -> KnowledgeResult<String> {
        match method {
            SynthesisMethod::SemanticClustering => {
                self.semantic_clustering_synthesis(topic, results).await
            }
            SynthesisMethod::TemporalAnalysis => {
                self.temporal_analysis_synthesis(topic, results).await
            }
            SynthesisMethod::CrossScopeCorrelation => {
                self.cross_scope_correlation_synthesis(topic, results).await
            }
            SynthesisMethod::PatternRecognition => {
                self.pattern_recognition_synthesis(topic, results).await
            }
            SynthesisMethod::DecisionTree => self.decision_tree_synthesis(topic, results).await,
            SynthesisMethod::Hybrid => self.hybrid_synthesis(topic, results).await,
        }
    }

    /// Semantic clustering synthesis
    async fn semantic_clustering_synthesis(
        &self,
        topic: &str,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        // Group results by semantic similarity
        let clusters = self.cluster_by_semantic_similarity(results).await?;

        let mut synthesis = format!("# Knowledge Synthesis: {}\n\n", topic);
        synthesis.push_str("## Overview\n\n");
        synthesis.push_str(&format!(
            "This synthesis combines information from {} sources across {} semantic clusters.\n\n",
            results.len(),
            clusters.len()
        ));

        for (i, cluster) in clusters.iter().enumerate() {
            synthesis.push_str(&format!("### Cluster {}\n\n", i + 1));

            // Extract common themes from cluster
            let themes = self.extract_cluster_themes(cluster).await?;
            synthesis.push_str(&format!("**Key Themes:** {}\n\n", themes.join(", ")));

            // Summarize cluster content
            let summary = self.summarize_cluster_content(cluster).await?;
            synthesis.push_str(&format!("**Summary:** {}\n\n", summary));

            // Add representative examples
            synthesis.push_str("**Representative Examples:**\n");
            for result in cluster.iter().take(3) {
                synthesis.push_str(&format!(
                    "- {}\n",
                    result.content.lines().next().unwrap_or("")
                ));
            }
            synthesis.push_str("\n");
        }

        Ok(synthesis)
    }

    /// Temporal analysis synthesis
    async fn temporal_analysis_synthesis(
        &self,
        topic: &str,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        // Sort results by creation time
        let mut sorted_results = results.to_vec();
        sorted_results.sort_by(|a, b| a.metadata.created_at.cmp(&b.metadata.created_at));

        let mut synthesis = format!("# Temporal Knowledge Synthesis: {}\n\n", topic);
        synthesis.push_str("## Timeline Analysis\n\n");

        // Group by time periods
        let time_periods = self.group_by_time_periods(&sorted_results).await?;

        for (period, period_results) in time_periods {
            synthesis.push_str(&format!("### {}\n\n", period));

            let period_summary = self.summarize_period_content(&period_results).await?;
            synthesis.push_str(&format!("**Period Summary:** {}\n\n", period_summary));

            // Identify trends and changes
            let trends = self.identify_temporal_trends(&period_results).await?;
            if !trends.is_empty() {
                synthesis.push_str("**Key Trends:**\n");
                for trend in trends {
                    synthesis.push_str(&format!("- {}\n", trend));
                }
                synthesis.push_str("\n");
            }
        }

        Ok(synthesis)
    }

    /// Cross-scope correlation synthesis
    async fn cross_scope_correlation_synthesis(
        &self,
        topic: &str,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        // Group results by scope
        let scope_groups = self.group_by_scope(results).await?;

        let mut synthesis = format!("# Cross-Scope Knowledge Synthesis: {}\n\n", topic);
        synthesis.push_str("## Scope Analysis\n\n");

        for (scope, scope_results) in &scope_groups {
            synthesis.push_str(&format!("### Scope: {}\n\n", scope));

            let scope_summary = self.summarize_scope_content(scope_results).await?;
            synthesis.push_str(&format!("**Scope Summary:** {}\n\n", scope_summary));

            // Identify scope-specific insights
            let insights = self.extract_scope_insights(scope_results).await?;
            if !insights.is_empty() {
                synthesis.push_str("**Key Insights:**\n");
                for insight in insights {
                    synthesis.push_str(&format!("- {}\n", insight));
                }
                synthesis.push_str("\n");
            }
        }

        // Identify cross-scope patterns
        let cross_scope_patterns = self.identify_cross_scope_patterns(&scope_groups).await?;
        if !cross_scope_patterns.is_empty() {
            synthesis.push_str("## Cross-Scope Patterns\n\n");
            for pattern in cross_scope_patterns {
                synthesis.push_str(&format!("- {}\n", pattern));
            }
            synthesis.push_str("\n");
        }

        Ok(synthesis)
    }

    /// Pattern recognition synthesis
    async fn pattern_recognition_synthesis(
        &self,
        topic: &str,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        let mut synthesis = format!("# Pattern-Based Knowledge Synthesis: {}\n\n", topic);
        synthesis.push_str("## Pattern Analysis\n\n");

        // Extract patterns from content
        let patterns = self.extract_content_patterns(results).await?;

        for (pattern_type, pattern_data) in patterns {
            synthesis.push_str(&format!("### {}\n\n", pattern_type));

            for (pattern, occurrences) in pattern_data {
                synthesis.push_str(&format!("**Pattern:** {}\n", pattern));
                synthesis.push_str(&format!("**Occurrences:** {}\n", occurrences));
                synthesis.push_str("\n");
            }
        }

        Ok(synthesis)
    }

    /// Decision tree synthesis
    async fn decision_tree_synthesis(
        &self,
        topic: &str,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        let mut synthesis = format!("# Decision Tree Knowledge Synthesis: {}\n\n", topic);
        synthesis.push_str("## Decision Framework\n\n");

        // Extract decision points and alternatives
        let decision_tree = self.build_decision_tree(results).await?;

        synthesis.push_str(&format!("### Decision Tree Structure\n\n"));
        synthesis.push_str(&format!(
            "**Root Decision:** {}\n\n",
            decision_tree.root_decision
        ));

        for (i, branch) in decision_tree.branches.iter().enumerate() {
            synthesis.push_str(&format!("**Branch {}:** {}\n", i + 1, branch.condition));
            synthesis.push_str(&format!("**Outcome:** {}\n", branch.outcome));
            synthesis.push_str(&format!("**Confidence:** {:.2}\n\n", branch.confidence));
        }

        Ok(synthesis)
    }

    /// Hybrid synthesis combining multiple methods
    async fn hybrid_synthesis(
        &self,
        topic: &str,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        let mut synthesis = format!("# Hybrid Knowledge Synthesis: {}\n\n", topic);
        synthesis.push_str("## Multi-Method Analysis\n\n");

        // Combine insights from different methods
        let semantic_insights = self.semantic_clustering_synthesis(topic, results).await?;
        let temporal_insights = self.temporal_analysis_synthesis(topic, results).await?;
        let cross_scope_insights = self
            .cross_scope_correlation_synthesis(topic, results)
            .await?;

        synthesis.push_str("### Semantic Insights\n\n");
        synthesis.push_str(&semantic_insights);
        synthesis.push_str("\n\n### Temporal Insights\n\n");
        synthesis.push_str(&temporal_insights);
        synthesis.push_str("\n\n### Cross-Scope Insights\n\n");
        synthesis.push_str(&cross_scope_insights);

        // Add integrated conclusions
        synthesis.push_str("\n\n## Integrated Conclusions\n\n");
        let conclusions = self.generate_integrated_conclusions(results).await?;
        synthesis.push_str(&conclusions);

        Ok(synthesis)
    }

    /// Combine multiple synthesis results
    async fn combine_synthesis_results(
        &self,
        topic: &str,
        synthesis_results: Vec<String>,
    ) -> KnowledgeResult<String> {
        let mut combined = format!("# Comprehensive Knowledge Synthesis: {}\n\n", topic);
        combined.push_str("## Executive Summary\n\n");

        // Extract key points from each synthesis
        for (i, synthesis) in synthesis_results.iter().enumerate() {
            combined.push_str(&format!("### Method {}\n\n", i + 1));
            combined.push_str(&synthesis.lines().take(10).collect::<Vec<_>>().join("\n"));
            combined.push_str("\n\n");
        }

        Ok(combined)
    }

    // Helper methods for synthesis

    async fn cluster_by_semantic_similarity(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<Vec<Vec<SemanticResult>>> {
        // Simple clustering based on relevance scores
        let mut clusters = Vec::new();
        let mut used = vec![false; results.len()];

        for i in 0..results.len() {
            if used[i] {
                continue;
            }

            let mut cluster = vec![results[i].clone()];
            used[i] = true;

            for j in (i + 1)..results.len() {
                if !used[j] && (results[i].relevance_score - results[j].relevance_score).abs() < 0.1
                {
                    cluster.push(results[j].clone());
                    used[j] = true;
                }
            }

            clusters.push(cluster);
        }

        Ok(clusters)
    }

    async fn extract_cluster_themes(
        &self,
        cluster: &[SemanticResult],
    ) -> KnowledgeResult<Vec<String>> {
        // Extract common words as themes
        let mut word_counts: HashMap<String, usize> = HashMap::new();

        for result in cluster {
            for word in result.content.split_whitespace() {
                if word.len() > 4 {
                    *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
                }
            }
        }

        let themes: Vec<String> = word_counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(word, _)| word)
            .take(5)
            .collect();

        Ok(themes)
    }

    async fn summarize_cluster_content(
        &self,
        cluster: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        // Simple summary based on first few lines
        let summaries: Vec<String> = cluster
            .iter()
            .map(|r| r.content.lines().next().unwrap_or("").to_string())
            .collect();

        Ok(summaries.join("; "))
    }

    async fn group_by_time_periods(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<Vec<(String, Vec<SemanticResult>)>> {
        // Group by year for now
        let mut periods: HashMap<i32, Vec<SemanticResult>> = HashMap::new();

        for result in results {
            let year = result.metadata.created_at.year();
            periods
                .entry(year)
                .or_insert_with(Vec::new)
                .push(result.clone());
        }

        let mut sorted_periods: Vec<_> = periods.into_iter().collect();
        sorted_periods.sort_by_key(|(year, _)| *year);

        Ok(sorted_periods
            .into_iter()
            .map(|(year, results)| (year.to_string(), results))
            .collect())
    }

    async fn summarize_period_content(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        let total_content: usize = results.iter().map(|r| r.content.len()).sum();
        Ok(format!(
            "{} sources with {} total characters",
            results.len(),
            total_content
        ))
    }

    async fn identify_temporal_trends(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<Vec<String>> {
        // Simple trend identification
        let mut trends = Vec::new();

        if results.len() > 1 {
            let first = &results[0];
            let last = &results[results.len() - 1];

            if first.content.len() < last.content.len() {
                trends.push("Increasing content complexity over time".to_string());
            }

            if first.relevance_score < last.relevance_score {
                trends.push("Improving relevance scores over time".to_string());
            }
        }

        Ok(trends)
    }

    async fn group_by_scope(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<Vec<(String, Vec<SemanticResult>)>> {
        let mut scope_groups: HashMap<String, Vec<SemanticResult>> = HashMap::new();

        for result in results {
            let scope = result
                .metadata
                .scope_path
                .clone()
                .unwrap_or_else(|| "unknown".to_string());
            scope_groups
                .entry(scope)
                .or_insert_with(Vec::new)
                .push(result.clone());
        }

        Ok(scope_groups.into_iter().collect())
    }

    async fn summarize_scope_content(&self, results: &[SemanticResult]) -> KnowledgeResult<String> {
        let content_types: Vec<String> = results
            .iter()
            .map(|r| format!("{:?}", r.metadata.source_type))
            .collect();

        Ok(format!(
            "{} sources of types: {}",
            results.len(),
            content_types.join(", ")
        ))
    }

    async fn extract_scope_insights(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<Vec<String>> {
        let mut insights = Vec::new();

        let avg_relevance: f32 =
            results.iter().map(|r| r.relevance_score).sum::<f32>() / results.len() as f32;
        insights.push(format!("Average relevance score: {:.2}", avg_relevance));

        let content_types: Vec<ContentType> = results
            .iter()
            .map(|r| r.metadata.source_type.clone())
            .collect();
        insights.push(format!("Content types: {:?}", content_types));

        Ok(insights)
    }

    async fn identify_cross_scope_patterns(
        &self,
        scope_groups: &[(String, Vec<SemanticResult>)],
    ) -> KnowledgeResult<Vec<String>> {
        let mut patterns = Vec::new();

        if scope_groups.len() > 1 {
            patterns.push("Multiple scopes contain related information".to_string());

            let total_sources: usize = scope_groups.iter().map(|(_, results)| results.len()).sum();
            patterns.push(format!(
                "Total sources across {} scopes: {}",
                scope_groups.len(),
                total_sources
            ));
        }

        Ok(patterns)
    }

    async fn extract_content_patterns(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<HashMap<String, HashMap<String, usize>>> {
        let mut patterns: HashMap<String, HashMap<String, usize>> = HashMap::new();

        // Extract common phrases
        let mut phrase_counts: HashMap<String, usize> = HashMap::new();
        for result in results {
            let words: Vec<&str> = result.content.split_whitespace().collect();
            for window in words.windows(3) {
                let phrase = window.join(" ");
                *phrase_counts.entry(phrase).or_insert(0) += 1;
            }
        }

        let common_phrases: HashMap<String, usize> = phrase_counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .collect();

        patterns.insert("Common Phrases".to_string(), common_phrases);

        Ok(patterns)
    }

    async fn build_decision_tree(
        &self,
        _results: &[SemanticResult],
    ) -> KnowledgeResult<DecisionTree> {
        // Implementation would build a decision tree from results
        // For now, return a simple decision tree
        Ok(DecisionTree {
            root_decision: "Default decision".to_string(),
            branches: vec![DecisionBranch {
                condition: "Default condition".to_string(),
                outcome: "Default outcome".to_string(),
                confidence: 0.5,
            }],
        })
    }

    async fn generate_integrated_conclusions(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<String> {
        let mut conclusions = String::new();

        let avg_relevance =
            results.iter().map(|r| r.relevance_score).sum::<f32>() / results.len() as f32;
        conclusions.push_str(&format!(
            "Overall relevance score: {:.2}\n\n",
            avg_relevance
        ));

        conclusions.push_str("Key findings:\n");
        conclusions.push_str("- Multiple sources provide complementary information\n");
        conclusions.push_str("- Temporal analysis shows evolution of knowledge\n");
        conclusions.push_str("- Cross-scope patterns reveal broader implications\n");

        Ok(conclusions)
    }

    async fn extract_temporal_range(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>>
    {
        if results.is_empty() {
            return Ok(None);
        }

        let earliest = results.iter().map(|r| r.metadata.created_at).min().unwrap();
        let latest = results
            .iter()
            .map(|r| r.metadata.last_modified)
            .max()
            .unwrap();

        Ok(Some((earliest, latest)))
    }

    async fn extract_semantic_clusters(
        &self,
        results: &[SemanticResult],
    ) -> KnowledgeResult<Vec<String>> {
        let clusters = self.cluster_by_semantic_similarity(results).await?;
        Ok(clusters
            .iter()
            .map(|c| format!("cluster_{}", c.len()))
            .collect())
    }

    async fn calculate_confidence_score(&self, results: &[SemanticResult]) -> KnowledgeResult<f32> {
        if results.is_empty() {
            return Ok(0.0);
        }

        let avg_relevance =
            results.iter().map(|r| r.relevance_score).sum::<f32>() / results.len() as f32;
        let source_diversity = results.len() as f32 / self.config.max_source_count as f32;

        Ok((avg_relevance + source_diversity) / 2.0)
    }
}

/// Decision tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTree {
    pub root_decision: String,
    pub branches: Vec<DecisionBranch>,
}

/// Decision branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionBranch {
    pub condition: String,
    pub outcome: String,
    pub confidence: f32,
}
