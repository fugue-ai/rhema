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

use chrono::{DateTime, Datelike, Timelike, Utc};
use std::collections::HashMap;
use tracing::{debug, info, trace};

use super::{Content, ContentAccess, SeasonalConfig, SeasonalPattern, SeasonalPeriod};
use super::{TemporalError, TemporalResult};

/// Seasonal pattern detector for identifying temporal patterns in content
pub struct SeasonalPatternDetector {
    config: SeasonalConfig,
}

impl SeasonalPatternDetector {
    /// Create a new seasonal pattern detector with default configuration
    pub fn new() -> Self {
        Self {
            config: SeasonalConfig::default(),
        }
    }

    /// Create a new seasonal pattern detector with custom configuration
    pub fn with_config(config: SeasonalConfig) -> Self {
        Self { config }
    }

    /// Detect seasonal patterns in content access history
    pub async fn detect_seasonal_patterns(
        &self,
        content_history: &[ContentAccess],
    ) -> TemporalResult<Vec<SeasonalPattern>> {
        if !self.config.enabled {
            return Ok(Vec::new());
        }

        info!(
            "Detecting seasonal patterns in {} access records",
            content_history.len()
        );

        let mut patterns = Vec::new();

        // Detect yearly patterns
        if let Some(yearly_pattern) = self.detect_yearly_pattern(content_history).await? {
            patterns.push(yearly_pattern);
        }

        // Detect monthly patterns
        if let Some(monthly_pattern) = self.detect_monthly_pattern(content_history).await? {
            patterns.push(monthly_pattern);
        }

        // Detect weekly patterns
        if let Some(weekly_pattern) = self.detect_weekly_pattern(content_history).await? {
            patterns.push(weekly_pattern);
        }

        // Detect daily patterns
        if let Some(daily_pattern) = self.detect_daily_pattern(content_history).await? {
            patterns.push(daily_pattern);
        }

        debug!("Detected {} seasonal patterns", patterns.len());
        Ok(patterns)
    }

    /// Calculate seasonal adjustment for content at a specific time
    pub async fn calculate_seasonal_adjustment(
        &self,
        content: &Content,
        query_time: DateTime<Utc>,
    ) -> TemporalResult<f64> {
        if !self.config.enabled {
            return Ok(1.0);
        }

        // For now, we'll use a simplified seasonal adjustment
        // In a real implementation, this would use detected patterns from access history
        let adjustment = self.calculate_basic_seasonal_adjustment(query_time);

        trace!(
            "Seasonal adjustment for content {} at {}: {:.3}",
            content.id,
            query_time,
            adjustment
        );

        Ok(adjustment)
    }

    /// Detect yearly patterns (e.g., annual reviews, seasonal code)
    async fn detect_yearly_pattern(
        &self,
        content_history: &[ContentAccess],
    ) -> TemporalResult<Option<SeasonalPattern>> {
        if content_history.len() < 10 {
            return Ok(None); // Need more data for yearly patterns
        }

        let mut monthly_counts: HashMap<u32, usize> = HashMap::new();

        for access in content_history {
            let month = access.access_time.month();
            *monthly_counts.entry(month).or_insert(0) += 1;
        }

        // Find the month with the highest access count
        if let Some((peak_month, peak_count)) =
            monthly_counts.iter().max_by_key(|(_, &count)| count)
        {
            let total_accesses = content_history.len();
            let peak_ratio = *peak_count as f64 / total_accesses as f64;

            // Check if the pattern is significant enough
            if peak_ratio > 0.2 && *peak_count >= 3 {
                let pattern = SeasonalPattern {
                    pattern_type: SeasonalPeriod::Yearly {
                        month: *peak_month,
                        day: 1,
                    },
                    confidence: peak_ratio.min(1.0),
                    strength: peak_ratio,
                    detected_at: Utc::now(),
                };

                debug!(
                    "Detected yearly pattern: peak_month={}, confidence={:.3}, strength={:.3}",
                    peak_month, pattern.confidence, pattern.strength
                );

                return Ok(Some(pattern));
            }
        }

        Ok(None)
    }

    /// Detect monthly patterns (e.g., monthly reports, recurring tasks)
    async fn detect_monthly_pattern(
        &self,
        content_history: &[ContentAccess],
    ) -> TemporalResult<Option<SeasonalPattern>> {
        if content_history.len() < 20 {
            return Ok(None); // Need more data for monthly patterns
        }

        let mut day_counts: HashMap<u32, usize> = HashMap::new();

        for access in content_history {
            let day = access.access_time.day();
            *day_counts.entry(day).or_insert(0) += 1;
        }

        // Find the day with the highest access count
        if let Some((peak_day, peak_count)) = day_counts.iter().max_by_key(|(_, &count)| count) {
            let total_accesses = content_history.len();
            let peak_ratio = *peak_count as f64 / total_accesses as f64;

            // Check if the pattern is significant enough
            if peak_ratio > 0.15 && *peak_count >= 2 {
                let pattern = SeasonalPattern {
                    pattern_type: SeasonalPeriod::Monthly { day: *peak_day },
                    confidence: peak_ratio.min(1.0),
                    strength: peak_ratio,
                    detected_at: Utc::now(),
                };

                debug!(
                    "Detected monthly pattern: peak_day={}, confidence={:.3}, strength={:.3}",
                    peak_day, pattern.confidence, pattern.strength
                );

                return Ok(Some(pattern));
            }
        }

        Ok(None)
    }

    /// Detect weekly patterns (e.g., weekly standups, weekend deployments)
    async fn detect_weekly_pattern(
        &self,
        content_history: &[ContentAccess],
    ) -> TemporalResult<Option<SeasonalPattern>> {
        if content_history.len() < 15 {
            return Ok(None); // Need more data for weekly patterns
        }

        let mut weekday_counts: HashMap<u32, usize> = HashMap::new();

        for access in content_history {
            let weekday = access.access_time.weekday().num_days_from_sunday();
            *weekday_counts.entry(weekday).or_insert(0) += 1;
        }

        // Find the weekday with the highest access count
        if let Some((peak_weekday, peak_count)) =
            weekday_counts.iter().max_by_key(|(_, &count)| count)
        {
            let total_accesses = content_history.len();
            let peak_ratio = *peak_count as f64 / total_accesses as f64;

            // Check if the pattern is significant enough
            if peak_ratio > 0.25 && *peak_count >= 3 {
                let pattern = SeasonalPattern {
                    pattern_type: SeasonalPeriod::Weekly {
                        weekday: *peak_weekday,
                    },
                    confidence: peak_ratio.min(1.0),
                    strength: peak_ratio,
                    detected_at: Utc::now(),
                };

                debug!(
                    "Detected weekly pattern: peak_weekday={}, confidence={:.3}, strength={:.3}",
                    peak_weekday, pattern.confidence, pattern.strength
                );

                return Ok(Some(pattern));
            }
        }

        Ok(None)
    }

    /// Detect daily patterns (e.g., business hours, daily builds)
    async fn detect_daily_pattern(
        &self,
        content_history: &[ContentAccess],
    ) -> TemporalResult<Option<SeasonalPattern>> {
        if content_history.len() < 10 {
            return Ok(None); // Need more data for daily patterns
        }

        let mut hour_counts: HashMap<u32, usize> = HashMap::new();

        for access in content_history {
            let hour = access.access_time.hour();
            *hour_counts.entry(hour).or_insert(0) += 1;
        }

        // Find the hour with the highest access count
        if let Some((peak_hour, peak_count)) = hour_counts.iter().max_by_key(|(_, &count)| count) {
            let total_accesses = content_history.len();
            let peak_ratio = *peak_count as f64 / total_accesses as f64;

            // Check if the pattern is significant enough
            if peak_ratio > 0.2 && *peak_count >= 2 {
                let pattern = SeasonalPattern {
                    pattern_type: SeasonalPeriod::Daily { hour: *peak_hour },
                    confidence: peak_ratio.min(1.0),
                    strength: peak_ratio,
                    detected_at: Utc::now(),
                };

                debug!(
                    "Detected daily pattern: peak_hour={}, confidence={:.3}, strength={:.3}",
                    peak_hour, pattern.confidence, pattern.strength
                );

                return Ok(Some(pattern));
            }
        }

        Ok(None)
    }

    /// Calculate basic seasonal adjustment based on time of year
    fn calculate_basic_seasonal_adjustment(&self, query_time: DateTime<Utc>) -> f64 {
        let month = query_time.month();
        let day = query_time.day();
        let weekday = query_time.weekday().num_days_from_sunday();
        let hour = query_time.hour();

        let mut adjustment: f64 = 1.0;

        // Business hours adjustment (Monday-Friday, 9-17)
        if weekday >= 1 && weekday <= 5 && hour >= 9 && hour < 17 {
            adjustment *= 1.1; // Boost during business hours
        } else {
            adjustment *= 0.9; // Slight penalty outside business hours
        }

        // Weekend adjustment
        if weekday == 0 || weekday == 6 {
            adjustment *= 0.8; // Lower relevance on weekends
        }

        // Seasonal adjustments (simplified)
        match month {
            1..=2 => adjustment *= 0.9,  // Winter (slightly lower)
            3..=5 => adjustment *= 1.0,  // Spring (neutral)
            6..=8 => adjustment *= 1.05, // Summer (slightly higher)
            9..=11 => adjustment *= 1.0, // Fall (neutral)
            12 => adjustment *= 0.95,    // December (slightly lower)
            _ => adjustment *= 1.0,
        }

        // End of month/quarter adjustments
        if day >= 25 {
            adjustment *= 1.05; // Slightly higher relevance near month end
        }

        adjustment.min(1.5_f64).max(0.5_f64) // Clamp to reasonable range
    }

    /// Check if a specific time matches a seasonal pattern
    pub fn matches_pattern(&self, pattern: &SeasonalPattern, query_time: DateTime<Utc>) -> bool {
        match &pattern.pattern_type {
            SeasonalPeriod::Yearly { month, day } => {
                query_time.month() == *month && query_time.day() == *day
            }
            SeasonalPeriod::Monthly { day } => query_time.day() == *day,
            SeasonalPeriod::Weekly { weekday } => {
                query_time.weekday().num_days_from_sunday() == *weekday
            }
            SeasonalPeriod::Daily { hour } => query_time.hour() == *hour,
        }
    }

    /// Get the configuration
    pub fn config(&self) -> &SeasonalConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: SeasonalConfig) {
        self.config = config;
        info!("Updated seasonal pattern detector configuration");
    }
}

impl Default for SeasonalPatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;

    fn create_test_access(offset_days: i64, hour: u32) -> ContentAccess {
        let access_time = Utc::now() - ChronoDuration::days(offset_days);
        let access_time = access_time.with_hour(hour).unwrap();

        ContentAccess {
            content_id: "test_content".to_string(),
            access_time,
            access_type: super::AccessType::Read,
            user_id: Some("test_user".to_string()),
            session_id: Some("test_session".to_string()),
            relevance_score: Some(0.8),
        }
    }

    #[tokio::test]
    async fn test_seasonal_adjustment() {
        let detector = SeasonalPatternDetector::new();

        // Test business hours
        let business_hours = Utc::now().with_hour(14).unwrap();
        let business_adjustment = detector
            .calculate_seasonal_adjustment(
                &Content {
                    id: "test".to_string(),
                    content_type: crate::types::ContentType::Code,
                    created_at: Utc::now(),
                    modified_at: Utc::now(),
                    accessed_at: Utc::now(),
                    access_count: 0,
                    content: "test".to_string(),
                    metadata: std::collections::HashMap::new(),
                },
                business_hours,
            )
            .await
            .unwrap();

        assert!(business_adjustment > 1.0); // Should be boosted during business hours
    }

    #[tokio::test]
    async fn test_weekly_pattern_detection() {
        let detector = SeasonalPatternDetector::new();

        // Create access history with a clear weekly pattern (more on Mondays)
        let mut history = Vec::new();
        for i in 0..30 {
            let weekday = if i % 7 == 1 { 1 } else { 3 }; // Monday vs Wednesday
            let access_time = Utc::now() - ChronoDuration::days(i);
            let access_time = access_time
                .with_weekday(chrono::Weekday::try_from(weekday as u8).unwrap())
                .unwrap();

            history.push(ContentAccess {
                content_id: "test".to_string(),
                access_time,
                access_type: super::AccessType::Read,
                user_id: Some("test_user".to_string()),
                session_id: Some("test_session".to_string()),
                relevance_score: Some(0.8),
            });
        }

        let patterns = detector.detect_seasonal_patterns(&history).await.unwrap();

        // Should detect a weekly pattern
        assert!(!patterns.is_empty());

        let weekly_pattern = patterns
            .iter()
            .find(|p| matches!(p.pattern_type, SeasonalPeriod::Weekly { .. }));

        assert!(weekly_pattern.is_some());
    }

    #[tokio::test]
    async fn test_pattern_matching() {
        let detector = SeasonalPatternDetector::new();

        let pattern = SeasonalPattern {
            pattern_type: SeasonalPeriod::Weekly { weekday: 1 }, // Monday
            confidence: 0.8,
            strength: 0.8,
            detected_at: Utc::now(),
        };

        let monday = Utc::now().with_weekday(chrono::Weekday::Mon).unwrap();
        let tuesday = Utc::now().with_weekday(chrono::Weekday::Tue).unwrap();

        assert!(detector.matches_pattern(&pattern, monday));
        assert!(!detector.matches_pattern(&pattern, tuesday));
    }
}
