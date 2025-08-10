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

use chrono::{DateTime, Utc, Datelike};
use std::time::Duration;
use tracing::{debug, info, trace};

use super::{
    TemporalRelationshipType, TemporalContextRelationship, CausalDirection, 
    SeasonalPeriod, Content, RelationshipConfig
};
use super::{TemporalResult, TemporalError};

/// Temporal relationship detector for analyzing content relationships
pub struct TemporalRelationshipDetector {
    config: RelationshipConfig,
}

impl TemporalRelationshipDetector {
    /// Create a new temporal relationship detector with default configuration
    pub fn new() -> Self {
        Self {
            config: RelationshipConfig::default(),
        }
    }

    /// Create a new temporal relationship detector with custom configuration
    pub fn with_config(config: RelationshipConfig) -> Self {
        Self { config }
    }

    /// Detect temporal relationships between source and target content
    pub async fn detect_temporal_relationships(
        &self,
        source_content: &Content,
        target_contents: &[Content],
    ) -> TemporalResult<Vec<TemporalContextRelationship>> {
        if !self.config.enabled {
            return Ok(Vec::new());
        }

        info!(
            "Detecting temporal relationships for content {} with {} targets",
            source_content.id, target_contents.len()
        );

        let mut relationships = Vec::new();

        for target_content in target_contents {
            if let Some(relationship) = self.analyze_temporal_relationship(
                source_content,
                target_content,
            ).await? {
                relationships.push(relationship);
            }
        }

        // Sort by confidence and limit results
        relationships.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        relationships.truncate(self.config.max_relationships);

        debug!("Detected {} temporal relationships", relationships.len());
        Ok(relationships)
    }

    /// Analyze temporal relationship between two content items
    async fn analyze_temporal_relationship(
        &self,
        source: &Content,
        target: &Content,
    ) -> TemporalResult<Option<TemporalContextRelationship>> {
        let temporal_distance = self.calculate_temporal_distance(source, target);
        let relationship_type = self.classify_relationship_type(source, target, &temporal_distance).await?;
        let confidence = self.calculate_relationship_confidence(source, target, &relationship_type).await?;

        // Only return relationships that meet the confidence threshold
        if confidence >= self.config.confidence_threshold {
            let relationship = TemporalContextRelationship {
                relationship_type: relationship_type.clone(),
                target_content_id: target.id.clone(),
                confidence,
                temporal_distance,
                relevance_score: confidence, // Use confidence as relevance score for now
            };

            trace!(
                "Detected relationship: {} -> {} (type={:?}, confidence={:.3})",
                source.id, target.id, relationship_type, confidence
            );

            Ok(Some(relationship))
        } else {
            Ok(None)
        }
    }

    /// Calculate temporal distance between two content items
    fn calculate_temporal_distance(&self, source: &Content, target: &Content) -> Duration {
        let source_time = source.created_at;
        let target_time = target.created_at;

        if source_time > target_time {
            Duration::from_secs(source_time.signed_duration_since(target_time).num_seconds() as u64)
        } else {
            Duration::from_secs(target_time.signed_duration_since(source_time).num_seconds() as u64)
        }
    }

    /// Classify the type of temporal relationship
    async fn classify_relationship_type(
        &self,
        source: &Content,
        target: &Content,
        temporal_distance: &Duration,
    ) -> TemporalResult<TemporalRelationshipType> {
        let source_time = source.created_at;
        let target_time = target.created_at;
        let distance_seconds = temporal_distance.as_secs();

        // Sequential relationship
        if distance_seconds <= 24 * 3600 { // Within 24 hours
            let order = if source_time < target_time { 1 } else { 2 };
            return Ok(TemporalRelationshipType::Sequential {
                order,
                gap_duration: *temporal_distance,
            });
        }

        // Concurrent relationship (overlapping creation times)
        if distance_seconds <= 3600 { // Within 1 hour
            return Ok(TemporalRelationshipType::Concurrent {
                overlap_duration: *temporal_distance,
            });
        }

        // Cyclical relationship (check for patterns)
        if let Some(cyclical) = self.detect_cyclical_pattern(source, target).await? {
            return Ok(cyclical);
        }

        // Causal relationship (simplified heuristic)
        if let Some(causal) = self.detect_causal_relationship(source, target).await? {
            return Ok(causal);
        }

        // Seasonal relationship
        if let Some(seasonal) = self.detect_seasonal_relationship(source, target).await? {
            return Ok(seasonal);
        }

        // Default to sequential if no other pattern is detected
        let order = if source_time < target_time { 1 } else { 2 };
        Ok(TemporalRelationshipType::Sequential {
            order,
            gap_duration: *temporal_distance,
        })
    }

    /// Detect cyclical patterns in content creation
    async fn detect_cyclical_pattern(
        &self,
        source: &Content,
        target: &Content,
    ) -> TemporalResult<Option<TemporalRelationshipType>> {
        let source_time = source.created_at;
        let target_time = target.created_at;
        let distance = target_time.signed_duration_since(source_time);

        // Check for weekly patterns
        if distance.num_days() >= 6 && distance.num_days() <= 8 {
            return Ok(Some(TemporalRelationshipType::Cyclical {
                cycle_period: Duration::from_secs(7 * 24 * 3600), // 1 week
                phase_offset: Duration::from_secs(distance.num_seconds() as u64),
            }));
        }

        // Check for monthly patterns
        if distance.num_days() >= 28 && distance.num_days() <= 32 {
            return Ok(Some(TemporalRelationshipType::Cyclical {
                cycle_period: Duration::from_secs(30 * 24 * 3600), // ~1 month
                phase_offset: Duration::from_secs(distance.num_seconds() as u64),
            }));
        }

        // Check for quarterly patterns
        if distance.num_days() >= 85 && distance.num_days() <= 95 {
            return Ok(Some(TemporalRelationshipType::Cyclical {
                cycle_period: Duration::from_secs(90 * 24 * 3600), // ~3 months
                phase_offset: Duration::from_secs(distance.num_seconds() as u64),
            }));
        }

        Ok(None)
    }

    /// Detect causal relationships between content
    async fn detect_causal_relationship(
        &self,
        source: &Content,
        target: &Content,
    ) -> TemporalResult<Option<TemporalRelationshipType>> {
        let source_time = source.created_at;
        let target_time = target.created_at;
        let distance = target_time.signed_duration_since(source_time);

        // Simple heuristic: if target was created shortly after source, it might be causal
        if distance.num_hours() >= 1 && distance.num_hours() <= 24 {
            let confidence = 0.6; // Moderate confidence for this heuristic
            let direction = if source_time < target_time {
                CausalDirection::Forward
            } else {
                CausalDirection::Backward
            };

            return Ok(Some(TemporalRelationshipType::Causal {
                confidence,
                direction,
            }));
        }

        Ok(None)
    }

    /// Detect seasonal relationships between content
    async fn detect_seasonal_relationship(
        &self,
        source: &Content,
        target: &Content,
    ) -> TemporalResult<Option<TemporalRelationshipType>> {
        let source_time = source.created_at;
        let target_time = target.created_at;

        // Check for yearly patterns
        let source_month = source_time.month();
        let target_month = target_time.month();
        let source_day = source_time.day();
        let target_day = target_time.day();

        if source_month == target_month && (source_day as i32 - target_day as i32).abs() <= 7 {
            return Ok(Some(TemporalRelationshipType::Seasonal {
                seasonal_period: SeasonalPeriod::Yearly {
                    month: source_month,
                    day: source_day,
                },
                strength: 0.8,
            }));
        }

        // Check for monthly patterns
        if source_day == target_day {
            return Ok(Some(TemporalRelationshipType::Seasonal {
                seasonal_period: SeasonalPeriod::Monthly { day: source_day },
                strength: 0.7,
            }));
        }

        // Check for weekly patterns
        let source_weekday = source_time.weekday().num_days_from_sunday();
        let target_weekday = target_time.weekday().num_days_from_sunday();

        if source_weekday == target_weekday {
            return Ok(Some(TemporalRelationshipType::Seasonal {
                seasonal_period: SeasonalPeriod::Weekly { weekday: source_weekday },
                strength: 0.6,
            }));
        }

        Ok(None)
    }

    /// Calculate confidence score for a temporal relationship
    async fn calculate_relationship_confidence(
        &self,
        source: &Content,
        target: &Content,
        relationship_type: &TemporalRelationshipType,
    ) -> TemporalResult<f64> {
        let mut confidence = 0.5; // Base confidence

        match relationship_type {
            TemporalRelationshipType::Sequential { gap_duration, .. } => {
                // Shorter gaps indicate stronger relationships
                let gap_hours = gap_duration.as_secs() as f64 / 3600.0;
                if gap_hours <= 1.0 {
                    confidence += 0.3;
                } else if gap_hours <= 24.0 {
                    confidence += 0.2;
                } else if gap_hours <= 168.0 { // 1 week
                    confidence += 0.1;
                }
            }
            TemporalRelationshipType::Concurrent { overlap_duration } => {
                // Shorter overlaps indicate stronger relationships
                let overlap_minutes = overlap_duration.as_secs() as f64 / 60.0;
                if overlap_minutes <= 30.0 {
                    confidence += 0.4;
                } else if overlap_minutes <= 60.0 {
                    confidence += 0.2;
                }
            }
            TemporalRelationshipType::Cyclical { .. } => {
                confidence += 0.3; // Cyclical patterns are significant
            }
            TemporalRelationshipType::Causal { confidence: causal_confidence, .. } => {
                confidence += causal_confidence * 0.5;
            }
            TemporalRelationshipType::Seasonal { strength, .. } => {
                confidence += strength * 0.4;
            }
        }

        // Content type compatibility bonus
        if source.content_type == target.content_type {
            confidence += 0.1;
        }

        // Access pattern similarity bonus
        let access_similarity = self.calculate_access_similarity(source, target);
        confidence += access_similarity * 0.2;

        Ok(confidence.min(1.0).max(0.0))
    }

    /// Calculate similarity in access patterns between content
    fn calculate_access_similarity(&self, source: &Content, target: &Content) -> f64 {
        let source_access_rate = source.access_count as f64 / 
            source.created_at.signed_duration_since(Utc::now()).num_days().max(1) as f64;
        let target_access_rate = target.access_count as f64 / 
            target.created_at.signed_duration_since(Utc::now()).num_days().max(1) as f64;

        if source_access_rate == 0.0 && target_access_rate == 0.0 {
            return 1.0; // Both have no access
        }

        if source_access_rate == 0.0 || target_access_rate == 0.0 {
            return 0.0; // One has access, the other doesn't
        }

        let ratio = source_access_rate.min(target_access_rate) / source_access_rate.max(target_access_rate);
        ratio
    }

    /// Get the configuration
    pub fn config(&self) -> &RelationshipConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: RelationshipConfig) {
        self.config = config;
        info!("Updated temporal relationship detector configuration");
    }
}

impl Default for TemporalRelationshipDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;
    use std::collections::HashMap;

    fn create_test_content(id: &str, age_days: i64) -> Content {
        let now = Utc::now();
        let created_at = now - ChronoDuration::days(age_days);

        Content {
            id: id.to_string(),
            content_type: crate::types::ContentType::Code,
            created_at,
            modified_at: created_at,
            accessed_at: created_at,
            access_count: 10,
            content: "test content".to_string(),
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_temporal_relationship_detection() {
        let detector = TemporalRelationshipDetector::new();
        
        let source = create_test_content("source", 10);
        let target = create_test_content("target", 9); // 1 day later

        let relationships = detector.detect_temporal_relationships(&source, &[target.clone()]).await.unwrap();
        
        assert!(!relationships.is_empty());
        assert_eq!(relationships[0].target_content_id, target.id);
    }

    #[tokio::test]
    async fn test_sequential_relationship() {
        let detector = TemporalRelationshipDetector::new();
        
        let source = create_test_content("source", 10);
        let target = create_test_content("target", 9); // 1 day later

        let relationship_type = detector.classify_relationship_type(&source, &target, &Duration::from_secs(24 * 3600)).await.unwrap();
        
        match relationship_type {
            TemporalRelationshipType::Sequential { order, .. } => {
                assert_eq!(order, 1); // source comes before target
            }
            _ => panic!("Expected sequential relationship"),
        }
    }

    #[tokio::test]
    async fn test_cyclical_pattern_detection() {
        let detector = TemporalRelationshipDetector::new();
        
        let source = create_test_content("source", 14);
        let target = create_test_content("target", 7); // 7 days later

        let cyclical = detector.detect_cyclical_pattern(&source, &target).await.unwrap();
        
        assert!(cyclical.is_some());
        match cyclical.unwrap() {
            TemporalRelationshipType::Cyclical { cycle_period, .. } => {
                assert_eq!(cycle_period, Duration::from_secs(7 * 24 * 3600)); // 1 week
            }
            _ => panic!("Expected cyclical relationship"),
        }
    }

    #[tokio::test]
    async fn test_confidence_calculation() {
        let detector = TemporalRelationshipDetector::new();
        
        let source = create_test_content("source", 10);
        let target = create_test_content("target", 9);

        let relationship_type = TemporalRelationshipType::Sequential {
            order: 1,
            gap_duration: Duration::from_secs(3600), // 1 hour
        };

        let confidence = detector.calculate_relationship_confidence(&source, &target, &relationship_type).await.unwrap();
        
        assert!(confidence > 0.5); // Should have reasonable confidence
        assert!(confidence <= 1.0);
    }
}
