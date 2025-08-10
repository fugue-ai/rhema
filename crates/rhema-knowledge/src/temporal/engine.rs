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
use std::collections::HashMap;
use tracing::{debug, info, trace};

use crate::types::ContentType;
use super::{DecayFunction, Content, ContentAccess, TemporalWeights};
use super::decay::DecayCalculator;
use super::{TemporalResult, TemporalError};

/// Temporal relevance engine for calculating comprehensive temporal scores
pub struct TemporalRelevanceEngine {
    decay_calculator: DecayCalculator,
    weights: TemporalWeights,
}

impl TemporalRelevanceEngine {
    /// Create a new temporal relevance engine with default configuration
    pub fn new() -> Self {
        Self {
            decay_calculator: DecayCalculator::new(),
            weights: TemporalWeights::default(),
        }
    }

    /// Create a new temporal relevance engine with custom decay functions
    pub fn with_decay_functions(decay_functions: HashMap<ContentType, DecayFunction>) -> Self {
        Self {
            decay_calculator: DecayCalculator::with_functions(decay_functions),
            weights: TemporalWeights::default(),
        }
    }

    /// Create a new temporal relevance engine with custom weights
    pub fn with_weights(weights: TemporalWeights) -> Self {
        Self {
            decay_calculator: DecayCalculator::new(),
            weights,
        }
    }

    /// Calculate comprehensive temporal relevance for content
    pub async fn calculate_temporal_relevance(
        &self,
        content: &Content,
        query_time: DateTime<Utc>,
        user_timezone: Option<&str>,
    ) -> TemporalResult<f64> {
        info!("Calculating temporal relevance for content: {}", content.id);

        // 1. Calculate base decay score
        let base_decay_score = self.calculate_base_decay_score(content, &query_time).await?;
        trace!("Base decay score: {:.3}", base_decay_score);

        // 2. Calculate access pattern score
        let access_pattern_score = self.calculate_access_pattern_score(content, &query_time).await?;
        trace!("Access pattern score: {:.3}", access_pattern_score);

        // 3. Calculate content type relevance
        let content_type_score = self.calculate_content_type_relevance(content, &query_time).await?;
        trace!("Content type score: {:.3}", content_type_score);

        // 4. Calculate freshness score
        let freshness_score = self.calculate_freshness_score(content, &query_time).await?;
        trace!("Freshness score: {:.3}", freshness_score);

        // 5. Combine scores with weights
        let final_score = self.combine_scores(
            base_decay_score,
            access_pattern_score,
            content_type_score,
            freshness_score,
        );

        debug!(
            "Temporal relevance calculation complete: final_score={:.3}",
            final_score
        );

        Ok(final_score.min(1.0).max(0.0))
    }

    /// Calculate base decay score using content-type specific decay functions
    async fn calculate_base_decay_score(
        &self,
        content: &Content,
        query_time: &DateTime<Utc>,
    ) -> TemporalResult<f64> {
        // For now, we'll use a placeholder access history
        // In a real implementation, this would be retrieved from storage
        let access_history: Option<&[ContentAccess]> = None;

        let decay_score = self.decay_calculator.calculate_decay(
            &content.content_type,
            &content.created_at,
            query_time,
            access_history,
        )?;

        Ok(decay_score)
    }

    /// Calculate access pattern score based on recent access behavior
    async fn calculate_access_pattern_score(
        &self,
        content: &Content,
        query_time: &DateTime<Utc>,
    ) -> TemporalResult<f64> {
        let age = query_time.signed_duration_since(content.created_at);
        let access_age = query_time.signed_duration_since(content.accessed_at);

        // Calculate access frequency score
        let access_frequency_score = if content.access_count > 0 {
            let days_since_creation = age.num_days() as f64;
            let accesses_per_day = content.access_count as f64 / days_since_creation.max(1.0);
            
            // Normalize to 0-1 range, with diminishing returns
            (1.0_f64 - (-accesses_per_day * 0.1).exp()).min(0.95_f64)
        } else {
            0.0
        };

        // Calculate recency score
        let recency_score = if access_age.num_days() < 1 {
            1.0
        } else if access_age.num_days() < 7 {
            0.8
        } else if access_age.num_days() < 30 {
            0.6
        } else if access_age.num_days() < 90 {
            0.4
        } else {
            0.2
        };

        // Combine frequency and recency
        let pattern_score = (access_frequency_score * 0.6 + recency_score * 0.4);
        
        trace!(
            "Access pattern score: frequency={:.3}, recency={:.3}, combined={:.3}",
            access_frequency_score, recency_score, pattern_score
        );

        Ok(pattern_score)
    }

    /// Calculate content type specific relevance
    async fn calculate_content_type_relevance(
        &self,
        content: &Content,
        query_time: &DateTime<Utc>,
    ) -> TemporalResult<f64> {
        let age = query_time.signed_duration_since(content.created_at);
        let age_days = age.num_days() as f64;

        let type_score = match content.content_type {
            ContentType::Code => {
                // Code is most relevant when recent, but some established code is also valuable
                if age_days < 7.0 {
                    1.0
                } else if age_days < 30.0 {
                    0.8
                } else if age_days < 90.0 {
                    0.6
                } else {
                    0.4
                }
            }
            ContentType::Documentation => {
                // Documentation is valuable for longer periods
                if age_days < 30.0 {
                    1.0
                } else if age_days < 180.0 {
                    0.9
                } else if age_days < 365.0 {
                    0.8
                } else {
                    0.7
                }
            }
            ContentType::Decision => {
                // Decisions are valuable for reference but may become outdated
                if age_days < 90.0 {
                    1.0
                } else if age_days < 365.0 {
                    0.8
                } else {
                    0.6
                }
            }
            ContentType::Knowledge => {
                // Knowledge adapts based on usage patterns
                if age_days < 30.0 {
                    1.0
                } else if age_days < 180.0 {
                    0.9
                } else {
                    0.7
                }
            }
            ContentType::Pattern => {
                // Patterns are valuable when established but not too old
                if age_days < 180.0 {
                    1.0
                } else if age_days < 365.0 {
                    0.8
                } else {
                    0.6
                }
            }
            ContentType::Configuration => {
                // Configuration is most relevant when recent
                if age_days < 7.0 {
                    1.0
                } else if age_days < 30.0 {
                    0.8
                } else {
                    0.5
                }
            }
            ContentType::Todo => {
                // Todos are most relevant when recent
                if age_days < 1.0 {
                    1.0
                } else if age_days < 7.0 {
                    0.7
                } else {
                    0.3
                }
            }
            ContentType::Insight => {
                // Insights are valuable for longer periods
                if age_days < 90.0 {
                    1.0
                } else if age_days < 365.0 {
                    0.8
                } else {
                    0.6
                }
            }
            ContentType::Unknown => {
                // Default to moderate relevance
                0.5
            }
        };

        trace!(
            "Content type relevance for {:?}: age={:.1} days, score={:.3}",
            content.content_type, age_days, type_score
        );

        Ok(type_score)
    }

    /// Calculate freshness score based on modification time
    async fn calculate_freshness_score(
        &self,
        content: &Content,
        query_time: &DateTime<Utc>,
    ) -> TemporalResult<f64> {
        let modification_age = query_time.signed_duration_since(content.modified_at);
        let modification_days = modification_age.num_days() as f64;

        let freshness_score = if modification_days < 1.0 {
            1.0
        } else if modification_days < 7.0 {
            0.9
        } else if modification_days < 30.0 {
            0.7
        } else if modification_days < 90.0 {
            0.5
        } else if modification_days < 365.0 {
            0.3
        } else {
            0.1
        };

        trace!(
            "Freshness score: modification_age={:.1} days, score={:.3}",
            modification_days, freshness_score
        );

        Ok(freshness_score)
    }

    /// Combine individual scores using configured weights
    fn combine_scores(
        &self,
        base_decay_score: f64,
        access_pattern_score: f64,
        content_type_score: f64,
        freshness_score: f64,
    ) -> f64 {
        let weighted_sum = 
            base_decay_score * self.weights.base_decay_weight +
            access_pattern_score * self.weights.relationship_weight +
            content_type_score * self.weights.base_decay_weight +
            freshness_score * self.weights.freshness_weight;

        let total_weight = 
            self.weights.base_decay_weight +
            self.weights.relationship_weight +
            self.weights.freshness_weight;

        let normalized_score = if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        };

        trace!(
            "Score combination: decay={:.3}, pattern={:.3}, type={:.3}, freshness={:.3}, final={:.3}",
            base_decay_score, access_pattern_score, content_type_score, freshness_score, normalized_score
        );

        normalized_score
    }

    /// Get the current weights configuration
    pub fn weights(&self) -> &TemporalWeights {
        &self.weights
    }

    /// Update the weights configuration
    pub fn update_weights(&mut self, weights: TemporalWeights) {
        self.weights = weights;
        info!("Updated temporal relevance weights");
    }
}

impl Default for TemporalRelevanceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;

    fn create_test_content(content_type: ContentType, age_days: i64) -> Content {
        let now = Utc::now();
        let created_at = now - ChronoDuration::days(age_days);
        let modified_at = created_at + ChronoDuration::days(age_days / 2);
        let accessed_at = now - ChronoDuration::days(age_days / 4);

        Content {
            id: "test_content".to_string(),
            content_type,
            created_at,
            modified_at,
            accessed_at,
            access_count: 10,
            content: "Test content".to_string(),
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_temporal_relevance_calculation() {
        let engine = TemporalRelevanceEngine::new();
        let content = create_test_content(ContentType::Code, 30);
        let query_time = Utc::now();

        let relevance = engine.calculate_temporal_relevance(&content, query_time, None).await.unwrap();

        assert!(relevance >= 0.0 && relevance <= 1.0);
        assert!(relevance > 0.1); // Should have some relevance
    }

    #[tokio::test]
    async fn test_content_type_relevance() {
        let engine = TemporalRelevanceEngine::new();
        let query_time = Utc::now();

        // Test recent code
        let recent_code = create_test_content(ContentType::Code, 3);
        let code_relevance = engine.calculate_content_type_relevance(&recent_code, &query_time).await.unwrap();
        assert!(code_relevance > 0.8);

        // Test old documentation
        let old_docs = create_test_content(ContentType::Documentation, 200);
        let docs_relevance = engine.calculate_content_type_relevance(&old_docs, &query_time).await.unwrap();
        assert!(docs_relevance > 0.6); // Documentation should remain relevant longer
    }

    #[tokio::test]
    async fn test_freshness_score() {
        let engine = TemporalRelevanceEngine::new();
        let query_time = Utc::now();

        // Test very recent content
        let recent_content = create_test_content(ContentType::Code, 1);
        let recent_freshness = engine.calculate_freshness_score(&recent_content, &query_time).await.unwrap();
        assert!(recent_freshness > 0.9);

        // Test old content
        let old_content = create_test_content(ContentType::Code, 100);
        let old_freshness = engine.calculate_freshness_score(&old_content, &query_time).await.unwrap();
        assert!(old_freshness < 0.5);
    }

    #[tokio::test]
    async fn test_access_pattern_score() {
        let engine = TemporalRelevanceEngine::new();
        let query_time = Utc::now();

        let content = create_test_content(ContentType::Code, 30);
        let pattern_score = engine.calculate_access_pattern_score(&content, &query_time).await.unwrap();

        assert!(pattern_score >= 0.0 && pattern_score <= 1.0);
    }
}
