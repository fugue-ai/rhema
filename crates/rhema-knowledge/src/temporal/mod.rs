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

pub mod decay;
pub mod engine;
pub mod filters;
pub mod relationships;
pub mod search;
pub mod seasonal;
pub mod timezone;
pub mod types;

pub use decay::*;
pub use engine::*;
pub use filters::*;
pub use relationships::*;
pub use search::*;
pub use seasonal::*;
pub use timezone::*;
pub use types::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

use crate::types::{ContentType, KnowledgeResult};

/// Error types for temporal operations
#[derive(Error, Debug)]
pub enum TemporalError {
    #[error("Decay calculation error: {0}")]
    DecayError(String),

    #[error("Seasonal pattern error: {0}")]
    SeasonalError(String),

    #[error("Timezone error: {0}")]
    TimezoneError(String),

    #[error("Relationship detection error: {0}")]
    RelationshipError(String),

    #[error("Temporal search error: {0}")]
    SearchError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Invalid temporal data: {0}")]
    InvalidData(String),
}

/// Result type for temporal operations
pub type TemporalResult<T> = Result<T, TemporalError>;

/// Main temporal context manager that orchestrates all temporal functionality
pub struct TemporalContextManager {
    relevance_engine: TemporalRelevanceEngine,
    relationship_detector: TemporalRelationshipDetector,
    seasonal_detector: SeasonalPatternDetector,
    timezone_manager: TimezoneAwareContextManager,
    search_enhancer: TemporalSearchEnhancer,
    config: TemporalConfig,
}

impl TemporalContextManager {
    pub async fn new(config: TemporalConfig) -> TemporalResult<Self> {
        let relevance_engine = TemporalRelevanceEngine::new();
        let relationship_detector = TemporalRelationshipDetector::new();
        let seasonal_detector = SeasonalPatternDetector::new();
        let timezone_manager = TimezoneAwareContextManager::new();
        let search_enhancer = TemporalSearchEnhancer::new();

        Ok(Self {
            relevance_engine,
            relationship_detector,
            seasonal_detector,
            timezone_manager,
            search_enhancer,
            config,
        })
    }

    /// Calculate comprehensive temporal relevance for content
    pub async fn calculate_temporal_relevance(
        &self,
        content: &Content,
        query_time: DateTime<Utc>,
        user_timezone: Option<&str>,
    ) -> TemporalResult<f64> {
        // 1. Calculate base temporal relevance
        let base_relevance = self
            .relevance_engine
            .calculate_temporal_relevance(content, query_time, user_timezone)
            .await?;

        // 2. Apply seasonal adjustments
        let seasonal_adjustment = self
            .seasonal_detector
            .calculate_seasonal_adjustment(content, query_time)
            .await?;

        // 3. Apply timezone adjustments
        let timezone_adjustment = if let Some(tz) = user_timezone {
            self.timezone_manager
                .calculate_timezone_adjustment(content, query_time, tz)
                .await?
        } else {
            1.0
        };

        // 4. Combine all factors
        let final_relevance = base_relevance * seasonal_adjustment * timezone_adjustment;

        Ok(final_relevance.min(1.0).max(0.0))
    }

    /// Detect temporal relationships between content items
    pub async fn detect_temporal_relationships(
        &self,
        source_content: &Content,
        target_contents: &[Content],
    ) -> TemporalResult<Vec<TemporalContextRelationship>> {
        self.relationship_detector
            .detect_temporal_relationships(source_content, target_contents)
            .await
    }

    /// Enhance search results with temporal context
    pub async fn enhance_search_results(
        &self,
        search_results: &[SemanticResult],
        temporal_query: &TemporalSearchQuery,
    ) -> TemporalResult<Vec<TemporalEnhancedResult>> {
        self.search_enhancer
            .enhance_with_temporal_context(search_results, temporal_query)
            .await
    }

    /// Get temporal configuration
    pub fn config(&self) -> &TemporalConfig {
        &self.config
    }
}

/// Configuration for temporal context awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConfig {
    pub enabled: bool,
    pub decay_functions: HashMap<ContentType, DecayFunction>,
    pub seasonal_config: SeasonalConfig,
    pub timezone_config: TimezoneConfig,
    pub relationship_config: RelationshipConfig,
    pub search_config: TemporalSearchConfig,
    pub weights: TemporalWeights,
}

impl Default for TemporalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            decay_functions: Self::default_decay_functions(),
            seasonal_config: SeasonalConfig::default(),
            timezone_config: TimezoneConfig::default(),
            relationship_config: RelationshipConfig::default(),
            search_config: TemporalSearchConfig::default(),
            weights: TemporalWeights::default(),
        }
    }
}

impl TemporalConfig {
    fn default_decay_functions() -> HashMap<ContentType, DecayFunction> {
        let mut functions = HashMap::new();
        functions.insert(
            ContentType::Documentation,
            DecayFunction::Documentation {
                half_life_days: 365.0,
            },
        );
        functions.insert(
            ContentType::Code,
            DecayFunction::Code {
                half_life_hours: 168.0,
            },
        );
        functions.insert(
            ContentType::Decision,
            DecayFunction::Decisions {
                half_life_weeks: 52.0,
            },
        );
        functions.insert(
            ContentType::Knowledge,
            DecayFunction::Knowledge {
                adaptive_decay: AdaptiveDecayConfig::default(),
            },
        );
        functions.insert(
            ContentType::Pattern,
            DecayFunction::Patterns {
                stable_period_days: 90.0,
                update_cycle_days: 30.0,
            },
        );
        functions
    }
}

/// Temporal weights for different factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalWeights {
    pub base_decay_weight: f64,
    pub seasonal_weight: f64,
    pub timezone_weight: f64,
    pub relationship_weight: f64,
    pub freshness_weight: f64,
}

impl Default for TemporalWeights {
    fn default() -> Self {
        Self {
            base_decay_weight: 0.4,
            seasonal_weight: 0.2,
            timezone_weight: 0.1,
            relationship_weight: 0.2,
            freshness_weight: 0.1,
        }
    }
}

/// Content with temporal metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: String,
    pub content_type: ContentType,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub access_count: u64,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

/// Semantic search result with temporal enhancement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticResult {
    pub cache_key: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub relevance_score: f32,
    pub semantic_tags: Vec<String>,
    pub metadata: SearchResultMetadata,
    pub cache_info: Option<CacheInfo>,
}

/// Search result metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultMetadata {
    pub source_type: ContentType,
    pub scope_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub size_bytes: u64,
    pub chunk_id: Option<String>,
}

/// Cache information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub is_cached: bool,
    pub cache_tier: CacheTier,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub ttl_remaining: Duration,
}

/// Cache tier enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheTier {
    Memory,
    Disk,
    Network,
}

/// Temporal enhanced search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEnhancedResult {
    pub base_result: SemanticResult,
    pub temporal_score: f64,
    pub seasonal_adjustment: f64,
    pub timezone_adjustment: f64,
    pub relationship_score: f64,
    pub final_score: f64,
    pub temporal_relationships: Vec<TemporalContextRelationship>,
    pub seasonal_patterns: Vec<SeasonalPattern>,
}

/// Temporal context relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContextRelationship {
    pub relationship_type: TemporalRelationshipType,
    pub target_content_id: String,
    pub confidence: f64,
    pub temporal_distance: Duration,
    pub relevance_score: f64,
}

/// Seasonal pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalPattern {
    pub pattern_type: SeasonalPeriod,
    pub confidence: f64,
    pub strength: f64,
    pub detected_at: DateTime<Utc>,
}

/// Seasonal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalConfig {
    pub enabled: bool,
    pub confidence_threshold: f64,
    pub historical_window_days: u64,
}

impl Default for SeasonalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            confidence_threshold: 0.7,
            historical_window_days: 365,
        }
    }
}

/// Timezone configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneConfig {
    pub enabled: bool,
    pub default_timezone: String,
    pub business_hours_boost: f64,
    pub off_hours_penalty: f64,
}

impl Default for TimezoneConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_timezone: "UTC".to_string(),
            business_hours_boost: 1.2,
            off_hours_penalty: 0.8,
        }
    }
}

/// Relationship configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipConfig {
    pub enabled: bool,
    pub max_relationships: usize,
    pub confidence_threshold: f64,
}

impl Default for RelationshipConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_relationships: 10,
            confidence_threshold: 0.6,
        }
    }
}

/// Temporal search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalSearchConfig {
    pub enabled: bool,
    pub max_enhanced_results: usize,
    pub temporal_score_threshold: f64,
}

impl Default for TemporalSearchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_enhanced_results: 50,
            temporal_score_threshold: 0.3,
        }
    }
}
