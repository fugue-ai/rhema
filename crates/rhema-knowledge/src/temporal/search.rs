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

use chrono::{DateTime, Utc, Datelike, Timelike};
use std::collections::HashMap;
use tracing::{debug, info, trace};

use crate::types::ContentType;
use super::{
    TemporalSearchQuery, TemporalFilter, FreshnessPreference, SeasonalPreference,
    TimeRange, TemporalEnhancedResult, SemanticResult, SearchResultMetadata,
    TemporalSearchConfig, SeasonalPeriod, Content
};
use super::engine::TemporalRelevanceEngine;
use super::seasonal::SeasonalPatternDetector;
use super::timezone::TimezoneAwareContextManager;
use super::relationships::TemporalRelationshipDetector;
use super::{TemporalResult, TemporalError};

/// Temporal search enhancer for applying temporal context to search results
pub struct TemporalSearchEnhancer {
    config: TemporalSearchConfig,
    relevance_engine: TemporalRelevanceEngine,
    seasonal_detector: SeasonalPatternDetector,
    timezone_manager: TimezoneAwareContextManager,
    relationship_detector: TemporalRelationshipDetector,
}

impl TemporalSearchEnhancer {
    /// Create a new temporal search enhancer with default configuration
    pub fn new() -> Self {
        Self {
            config: TemporalSearchConfig::default(),
            relevance_engine: TemporalRelevanceEngine::new(),
            seasonal_detector: SeasonalPatternDetector::new(),
            timezone_manager: TimezoneAwareContextManager::new(),
            relationship_detector: TemporalRelationshipDetector::new(),
        }
    }

    /// Create a new temporal search enhancer with custom configuration
    pub fn with_config(config: TemporalSearchConfig) -> Self {
        Self {
            config,
            relevance_engine: TemporalRelevanceEngine::new(),
            seasonal_detector: SeasonalPatternDetector::new(),
            timezone_manager: TimezoneAwareContextManager::new(),
            relationship_detector: TemporalRelationshipDetector::new(),
        }
    }

    /// Enhance search results with temporal context
    pub async fn enhance_with_temporal_context(
        &self,
        search_results: &[SemanticResult],
        temporal_query: &TemporalSearchQuery,
    ) -> TemporalResult<Vec<TemporalEnhancedResult>> {
        if !self.config.enabled {
            return Ok(search_results.iter().map(|r| self.convert_to_temporal_result(r, 1.0)).collect());
        }

        info!("Enhancing {} search results with temporal context", search_results.len());

        let mut enhanced_results = Vec::new();

        for result in search_results {
            // Apply temporal filters
            if !self.apply_temporal_filters(result, temporal_query).await? {
                continue; // Skip this result if it doesn't pass filters
            }

            // Calculate temporal relevance
            let temporal_score = self.calculate_temporal_relevance(result, temporal_query).await?;

            // Apply freshness preferences
            let freshness_adjustment = self.apply_freshness_preferences(result, temporal_query).await?;

            // Apply seasonal preferences
            let seasonal_adjustment = self.apply_seasonal_preferences(result, temporal_query).await?;

            // Apply timezone adjustments
            let timezone_adjustment = self.apply_timezone_adjustments(result, temporal_query).await?;

            // Calculate relationship score
            let relationship_score = self.calculate_relationship_score(result, search_results).await?;

            // Calculate final score
            let final_score = self.calculate_final_score(
                result.relevance_score as f64,
                temporal_score,
                freshness_adjustment,
                seasonal_adjustment,
                timezone_adjustment,
                relationship_score,
            );

            // Create enhanced result
            let enhanced_result = self.create_enhanced_result(
                result,
                temporal_score,
                seasonal_adjustment,
                timezone_adjustment,
                relationship_score,
                final_score,
            ).await?;

            enhanced_results.push(enhanced_result);
        }

        // Sort by final score and limit results
        enhanced_results.sort_by(|a, b| b.final_score.partial_cmp(&a.final_score).unwrap());
        enhanced_results.truncate(temporal_query.max_results);

        debug!("Enhanced {} search results", enhanced_results.len());
        Ok(enhanced_results)
    }

    /// Apply temporal filters to a search result
    async fn apply_temporal_filters(
        &self,
        result: &SemanticResult,
        temporal_query: &TemporalSearchQuery,
    ) -> TemporalResult<bool> {
        for filter in &temporal_query.temporal_filters {
            if !self.evaluate_temporal_filter(result, filter).await? {
                return Ok(false); // Result doesn't pass this filter
            }
        }
        Ok(true) // Result passes all filters
    }

    /// Evaluate a single temporal filter
    async fn evaluate_temporal_filter(
        &self,
        result: &SemanticResult,
        filter: &TemporalFilter,
    ) -> TemporalResult<bool> {
        match filter {
            TemporalFilter::CreatedAfter(datetime) => {
                Ok(result.metadata.created_at > *datetime)
            }
            TemporalFilter::CreatedBefore(datetime) => {
                Ok(result.metadata.created_at < *datetime)
            }
            TemporalFilter::ModifiedAfter(datetime) => {
                Ok(result.metadata.last_modified > *datetime)
            }
            TemporalFilter::ModifiedBefore(datetime) => {
                Ok(result.metadata.last_modified < *datetime)
            }
            TemporalFilter::AgeLessThan(duration) => {
                let age = Utc::now().signed_duration_since(result.metadata.created_at);
                Ok(age.num_seconds() as u64 <= duration.as_secs())
            }
            TemporalFilter::AgeGreaterThan(duration) => {
                let age = Utc::now().signed_duration_since(result.metadata.created_at);
                Ok(age.num_seconds() as u64 >= duration.as_secs())
            }
            TemporalFilter::RecentlyActive { min_access_count, within_duration } => {
                if let Some(cache_info) = &result.cache_info {
                    let last_access_age = Utc::now().signed_duration_since(cache_info.last_accessed);
                    Ok(cache_info.access_count >= *min_access_count && 
                       last_access_age.num_seconds() as u64 <= within_duration.as_secs())
                } else {
                    Ok(false)
                }
            }
            TemporalFilter::FrequentlyUpdated { min_update_count, within_duration } => {
                // This would require tracking update history, simplified for now
                Ok(true)
            }
            TemporalFilter::SeasonalContent { seasonal_period } => {
                // Check if content was created during the seasonal period
                self.check_seasonal_content(result, seasonal_period).await
            }
            TemporalFilter::NonSeasonalContent => {
                // Check if content was NOT created during any seasonal period
                Ok(!self.check_seasonal_content(result, &SeasonalPeriod::Yearly { month: 1, day: 1 }).await?)
            }
        }
    }

    /// Check if content is seasonal
    async fn check_seasonal_content(
        &self,
        result: &SemanticResult,
        seasonal_period: &super::types::SeasonalPeriod,
    ) -> TemporalResult<bool> {
        let created_at = result.metadata.created_at;
        
        match seasonal_period {
            super::types::SeasonalPeriod::Yearly { month, day } => {
                Ok(created_at.month() == *month && created_at.day() == *day)
            }
            super::types::SeasonalPeriod::Monthly { day } => {
                Ok(created_at.day() == *day)
            }
            super::types::SeasonalPeriod::Weekly { weekday } => {
                Ok(created_at.weekday().num_days_from_sunday() == *weekday)
            }
            super::types::SeasonalPeriod::Daily { hour } => {
                Ok(created_at.hour() == *hour)
            }
        }
    }

    /// Calculate temporal relevance score
    async fn calculate_temporal_relevance(
        &self,
        result: &SemanticResult,
        temporal_query: &TemporalSearchQuery,
    ) -> TemporalResult<f64> {
        // Convert SemanticResult to Content for temporal relevance calculation
        let content = self.convert_to_content(result);
        
        let query_time = Utc::now();
        let user_timezone = temporal_query.timezone_context.as_ref()
            .map(|ctx| ctx.user_timezone.as_str());

        let temporal_relevance = self.relevance_engine
            .calculate_temporal_relevance(&content, query_time, user_timezone)
            .await?;

        Ok(temporal_relevance)
    }

    /// Apply freshness preferences
    async fn apply_freshness_preferences(
        &self,
        result: &SemanticResult,
        temporal_query: &TemporalSearchQuery,
    ) -> TemporalResult<f64> {
        let age = Utc::now().signed_duration_since(result.metadata.created_at);
        let age_days = age.num_days() as f64;

        match &temporal_query.freshness_preference {
            FreshnessPreference::PreferRecent { weight } => {
                let freshness_score = if age_days < 1.0 {
                    1.0
                } else if age_days < 7.0 {
                    0.8
                } else if age_days < 30.0 {
                    0.6
                } else {
                    0.4
                };
                Ok(1.0 + (freshness_score * weight))
            }
            FreshnessPreference::PreferEstablished { min_age_days, weight } => {
                let established_score = if age_days >= *min_age_days {
                    1.0
                } else {
                    age_days / min_age_days
                };
                Ok(1.0 + (established_score * weight))
            }
            FreshnessPreference::Balanced { recent_weight, established_weight } => {
                let recent_score = if age_days < 7.0 { 1.0 } else { 0.5 };
                let established_score = if age_days >= 30.0 { 1.0 } else { age_days / 30.0 };
                Ok(1.0 + (recent_score * recent_weight + established_score * established_weight))
            }
            FreshnessPreference::ContentTypeSpecific { preferences } => {
                let content_type = &result.metadata.source_type;
                let weight = preferences.get(content_type).unwrap_or(&0.5);
                let freshness_score = if age_days < 7.0 { 1.0 } else { 0.5 };
                Ok(1.0 + (freshness_score * weight))
            }
        }
    }

    /// Apply seasonal preferences
    async fn apply_seasonal_preferences(
        &self,
        result: &SemanticResult,
        temporal_query: &TemporalSearchQuery,
    ) -> TemporalResult<f64> {
        let mut total_adjustment = 1.0;

        for preference in &temporal_query.seasonal_preferences {
            let content_created_at = result.metadata.created_at;
            let matches = self.matches_seasonal_preference(content_created_at, preference);
            
            if matches {
                total_adjustment *= (1.0 + preference.weight);
            }
        }

        Ok(total_adjustment)
    }

    /// Check if content matches a seasonal preference
    fn matches_seasonal_preference(
        &self,
        created_at: DateTime<Utc>,
        preference: &SeasonalPreference,
    ) -> bool {
        match &preference.seasonal_period {
            super::types::SeasonalPeriod::Yearly { month, day } => {
                let month_match = created_at.month() == *month;
                let day_match = (created_at.day() as i32 - *day as i32).abs() <= preference.tolerance_days as i32;
                month_match && day_match
            }
            super::types::SeasonalPeriod::Monthly { day } => {
                (created_at.day() as i32 - *day as i32).abs() <= preference.tolerance_days as i32
            }
            super::types::SeasonalPeriod::Weekly { weekday } => {
                created_at.weekday().num_days_from_sunday() == *weekday
            }
            super::types::SeasonalPeriod::Daily { hour } => {
                created_at.hour() == *hour
            }
        }
    }

    /// Apply timezone adjustments
    async fn apply_timezone_adjustments(
        &self,
        result: &SemanticResult,
        temporal_query: &TemporalSearchQuery,
    ) -> TemporalResult<f64> {
        if let Some(timezone_context) = &temporal_query.timezone_context {
            let content = self.convert_to_content(result);
            let query_time = Utc::now();
            
            let adjustment = self.timezone_manager
                .calculate_timezone_adjustment(&content, query_time, &timezone_context.user_timezone)
                .await?;
            
            Ok(adjustment)
        } else {
            Ok(1.0) // No timezone adjustment
        }
    }

    /// Calculate relationship score with other results
    async fn calculate_relationship_score(
        &self,
        result: &SemanticResult,
        all_results: &[SemanticResult],
    ) -> TemporalResult<f64> {
        if all_results.len() <= 1 {
            return Ok(0.5); // No relationships to compare
        }

        let content = self.convert_to_content(result);
        let other_contents: Vec<_> = all_results.iter()
            .filter(|r| r.cache_key != result.cache_key)
            .map(|r| self.convert_to_content(r))
            .collect();

        let relationships = self.relationship_detector
            .detect_temporal_relationships(&content, &other_contents)
            .await?;

        if relationships.is_empty() {
            return Ok(0.5);
        }

        // Calculate average relationship score
        let total_score: f64 = relationships.iter().map(|r| r.relevance_score).sum();
        let avg_score = total_score / relationships.len() as f64;

        Ok(avg_score)
    }

    /// Calculate final enhanced score
    fn calculate_final_score(
        &self,
        base_score: f64,
        temporal_score: f64,
        freshness_adjustment: f64,
        seasonal_adjustment: f64,
        timezone_adjustment: f64,
        relationship_score: f64,
    ) -> f64 {
        let adjusted_score = base_score * 
            temporal_score * 
            freshness_adjustment * 
            seasonal_adjustment * 
            timezone_adjustment * 
            relationship_score;

        adjusted_score.min(1.0).max(0.0)
    }

    /// Create enhanced result
    async fn create_enhanced_result(
        &self,
        result: &SemanticResult,
        temporal_score: f64,
        seasonal_adjustment: f64,
        timezone_adjustment: f64,
        relationship_score: f64,
        final_score: f64,
    ) -> TemporalResult<TemporalEnhancedResult> {
        // For now, we'll create empty relationships and patterns
        // In a real implementation, these would be populated from actual analysis
        let temporal_relationships = Vec::new();
        let seasonal_patterns = Vec::new();

        Ok(TemporalEnhancedResult {
            base_result: result.clone(),
            temporal_score,
            seasonal_adjustment,
            timezone_adjustment,
            relationship_score,
            final_score,
            temporal_relationships,
            seasonal_patterns,
        })
    }

    /// Convert SemanticResult to Content for temporal analysis
    fn convert_to_content(&self, result: &SemanticResult) -> Content {
        Content {
            id: result.cache_key.clone(),
            content_type: result.metadata.source_type.clone(),
            created_at: result.metadata.created_at,
            modified_at: result.metadata.last_modified,
            accessed_at: result.metadata.last_modified, // Use last_modified as accessed_at
            access_count: result.cache_info.as_ref().map(|ci| ci.access_count).unwrap_or(0),
            content: result.content.clone(),
            metadata: HashMap::new(),
        }
    }

    /// Convert SemanticResult to TemporalEnhancedResult with default temporal score
    fn convert_to_temporal_result(&self, result: &SemanticResult, temporal_score: f64) -> TemporalEnhancedResult {
        TemporalEnhancedResult {
            base_result: result.clone(),
            temporal_score,
            seasonal_adjustment: 1.0,
            timezone_adjustment: 1.0,
            relationship_score: 0.5,
            final_score: result.relevance_score as f64 * temporal_score,
            temporal_relationships: Vec::new(),
            seasonal_patterns: Vec::new(),
        }
    }

    /// Get the configuration
    pub fn config(&self) -> &TemporalSearchConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: TemporalSearchConfig) {
        self.config = config;
        info!("Updated temporal search enhancer configuration");
    }
}

impl Default for TemporalSearchEnhancer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;
    use std::collections::HashMap;

    fn create_test_semantic_result(id: &str, age_days: i64) -> SemanticResult {
        let now = Utc::now();
        let created_at = now - ChronoDuration::days(age_days);

        SemanticResult {
            cache_key: id.to_string(),
            content: "test content".to_string(),
            embedding: vec![0.1, 0.2, 0.3],
            relevance_score: 0.8,
            semantic_tags: vec!["test".to_string()],
            metadata: SearchResultMetadata {
                source_type: ContentType::Code,
                scope_path: None,
                created_at,
                last_modified: created_at,
                size_bytes: 1024,
                chunk_id: None,
            },
            cache_info: Some(super::types::CacheInfo {
                is_cached: true,
                cache_tier: super::types::CacheTier::Memory,
                access_count: 10,
                last_accessed: created_at,
                ttl_remaining: std::time::Duration::from_secs(3600),
            }),
        }
    }

    #[tokio::test]
    async fn test_temporal_search_enhancement() {
        let enhancer = TemporalSearchEnhancer::new();
        
        let results = vec![
            create_test_semantic_result("result1", 1),
            create_test_semantic_result("result2", 30),
        ];

        let temporal_query = TemporalSearchQuery::new("test query".to_string())
            .with_freshness_preference(FreshnessPreference::PreferRecent { weight: 0.3 });

        let enhanced_results = enhancer.enhance_with_temporal_context(&results, &temporal_query).await.unwrap();
        
        assert_eq!(enhanced_results.len(), 2);
        assert!(enhanced_results[0].final_score > enhanced_results[1].final_score); // Recent should score higher
    }

    #[tokio::test]
    async fn test_temporal_filters() {
        let enhancer = TemporalSearchEnhancer::new();
        
        let result = create_test_semantic_result("result1", 5);
        let temporal_query = TemporalSearchQuery::new("test query".to_string())
            .with_filter(TemporalFilter::AgeLessThan(std::time::Duration::from_secs(7 * 24 * 3600)));

        let passes_filter = enhancer.apply_temporal_filters(&result, &temporal_query).await.unwrap();
        assert!(passes_filter); // Should pass age filter

        let temporal_query_old = TemporalSearchQuery::new("test query".to_string())
            .with_filter(TemporalFilter::AgeLessThan(std::time::Duration::from_secs(1 * 24 * 3600)));

        let passes_old_filter = enhancer.apply_temporal_filters(&result, &temporal_query_old).await.unwrap();
        assert!(!passes_old_filter); // Should not pass stricter age filter
    }

    #[tokio::test]
    async fn test_freshness_preferences() {
        let enhancer = TemporalSearchEnhancer::new();
        
        let result = create_test_semantic_result("result1", 1);
        let temporal_query = TemporalSearchQuery::new("test query".to_string())
            .with_freshness_preference(FreshnessPreference::PreferRecent { weight: 0.3 });

        let adjustment = enhancer.apply_freshness_preferences(&result, &temporal_query).await.unwrap();
        assert!(adjustment > 1.0); // Should boost recent content
    }
}
