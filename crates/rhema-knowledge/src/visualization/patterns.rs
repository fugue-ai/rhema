//! Pattern detection and visualization module
//!
//! This module provides pattern detection and visualization capabilities
//! for knowledge data, including temporal patterns, usage patterns,
//! and relationship patterns.

use crate::temporal::types::ContentAccess;
use crate::types::KnowledgeResult;
use chrono::{DateTime, Datelike, Duration as ChronoDuration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pattern type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Temporal pattern
    Temporal,
    /// Usage pattern
    Usage,
    /// Relationship pattern
    Relationship,
    /// Sequential pattern
    Sequential,
    /// Cyclical pattern
    Cyclical,
    /// Seasonal pattern
    Seasonal,
}

/// Pattern detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternResult {
    /// Pattern type
    pub pattern_type: PatternType,
    /// Pattern confidence score
    pub confidence: f64,
    /// Pattern description
    pub description: String,
    /// Pattern data points
    pub data_points: Vec<PatternDataPoint>,
    /// Pattern metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Pattern data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDataPoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Value
    pub value: f64,
    /// Label
    pub label: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Pattern detector for knowledge data
pub struct PatternDetector;

impl PatternDetector {
    /// Create a new pattern detector
    pub fn new() -> Self {
        Self
    }

    /// Detect temporal patterns in content access
    pub fn detect_temporal_patterns(&self, content_access: &[ContentAccess]) -> Vec<PatternResult> {
        let mut patterns = Vec::new();

        // Detect daily patterns
        if let Some(daily_pattern) = self.detect_daily_pattern(content_access) {
            patterns.push(daily_pattern);
        }

        // Detect weekly patterns
        if let Some(weekly_pattern) = self.detect_weekly_pattern(content_access) {
            patterns.push(weekly_pattern);
        }

        // Detect monthly patterns
        if let Some(monthly_pattern) = self.detect_monthly_pattern(content_access) {
            patterns.push(monthly_pattern);
        }

        // Detect seasonal patterns
        if let Some(seasonal_pattern) = self.detect_seasonal_pattern(content_access) {
            patterns.push(seasonal_pattern);
        }

        patterns
    }

    /// Detect usage patterns
    pub fn detect_usage_patterns(&self, content_access: &[ContentAccess]) -> Vec<PatternResult> {
        let mut patterns = Vec::new();

        // Detect burst patterns
        if let Some(burst_pattern) = self.detect_burst_pattern(content_access) {
            patterns.push(burst_pattern);
        }

        // Detect regular patterns
        if let Some(regular_pattern) = self.detect_regular_pattern(content_access) {
            patterns.push(regular_pattern);
        }

        // Detect declining patterns
        if let Some(declining_pattern) = self.detect_declining_pattern(content_access) {
            patterns.push(declining_pattern);
        }

        patterns
    }

    /// Detect relationship patterns
    pub fn detect_relationship_patterns(
        &self,
        content_access: &[ContentAccess],
    ) -> Vec<PatternResult> {
        let mut patterns = Vec::new();

        // Detect co-access patterns
        if let Some(co_access_pattern) = self.detect_co_access_pattern(content_access) {
            patterns.push(co_access_pattern);
        }

        // Detect sequential access patterns
        if let Some(sequential_pattern) = self.detect_sequential_access_pattern(content_access) {
            patterns.push(sequential_pattern);
        }

        patterns
    }

    /// Detect daily patterns
    fn detect_daily_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 7 {
            return None;
        }

        let mut hourly_counts = vec![0; 24];
        for access in content_access {
            let hour = access.access_time.hour() as usize;
            hourly_counts[hour] += 1;
        }

        // Calculate variance to determine if there's a pattern
        let total = hourly_counts.iter().sum::<usize>();
        let mean = total as f64 / 24.0;
        let variance = hourly_counts
            .iter()
            .map(|&count| {
                let diff = count as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / 24.0;

        let confidence = (variance / (mean * mean)).min(1.0);

        if confidence > 0.1 {
            let data_points = hourly_counts
                .iter()
                .enumerate()
                .map(|(hour, &count)| PatternDataPoint {
                    timestamp: Utc::now().with_hour(hour as u32).unwrap(),
                    value: count as f64,
                    label: Some(format!("{:02}:00", hour)),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Temporal,
                confidence,
                description: "Daily access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect weekly patterns
    fn detect_weekly_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 14 {
            return None;
        }

        let mut daily_counts = vec![0; 7];
        for access in content_access {
            let day = access.access_time.weekday().num_days_from_monday() as usize;
            daily_counts[day] += 1;
        }

        let total = daily_counts.iter().sum::<usize>();
        let mean = total as f64 / 7.0;
        let variance = daily_counts
            .iter()
            .map(|&count| {
                let diff = count as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / 7.0;

        let confidence = (variance / (mean * mean)).min(1.0);

        if confidence > 0.1 {
            let day_names = [
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
            ];
            let data_points = daily_counts
                .iter()
                .enumerate()
                .map(|(day, &count)| PatternDataPoint {
                    timestamp: Utc::now(),
                    value: count as f64,
                    label: Some(day_names[day].to_string()),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Temporal,
                confidence,
                description: "Weekly access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect monthly patterns
    fn detect_monthly_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 30 {
            return None;
        }

        let mut monthly_counts = vec![0; 12];
        for access in content_access {
            let month = access.access_time.month() as usize - 1;
            monthly_counts[month] += 1;
        }

        let total = monthly_counts.iter().sum::<usize>();
        let mean = total as f64 / 12.0;
        let variance = monthly_counts
            .iter()
            .map(|&count| {
                let diff = count as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / 12.0;

        let confidence = (variance / (mean * mean)).min(1.0);

        if confidence > 0.1 {
            let month_names = [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ];
            let data_points = monthly_counts
                .iter()
                .enumerate()
                .map(|(month, &count)| PatternDataPoint {
                    timestamp: Utc::now(),
                    value: count as f64,
                    label: Some(month_names[month].to_string()),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Temporal,
                confidence,
                description: "Monthly access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect seasonal patterns
    fn detect_seasonal_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 90 {
            return None;
        }

        let mut seasonal_counts = vec![0; 4];
        for access in content_access {
            let month = access.access_time.month();
            let season = match month {
                12 | 1 | 2 => 0,  // Winter
                3 | 4 | 5 => 1,   // Spring
                6 | 7 | 8 => 2,   // Summer
                9 | 10 | 11 => 3, // Fall
                _ => 0,
            };
            seasonal_counts[season] += 1;
        }

        let total = seasonal_counts.iter().sum::<usize>();
        let mean = total as f64 / 4.0;
        let variance = seasonal_counts
            .iter()
            .map(|&count| {
                let diff = count as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / 4.0;

        let confidence = (variance / (mean * mean)).min(1.0);

        if confidence > 0.1 {
            let season_names = ["Winter", "Spring", "Summer", "Fall"];
            let data_points = seasonal_counts
                .iter()
                .enumerate()
                .map(|(season, &count)| PatternDataPoint {
                    timestamp: Utc::now(),
                    value: count as f64,
                    label: Some(season_names[season].to_string()),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Seasonal,
                confidence,
                description: "Seasonal access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect burst patterns
    fn detect_burst_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 10 {
            return None;
        }

        // Sort by timestamp
        let mut sorted_access = content_access.to_vec();
        sorted_access.sort_by_key(|access| access.access_time);

        // Calculate time intervals between accesses
        let mut intervals = Vec::new();
        for i in 1..sorted_access.len() {
            let interval = sorted_access[i].access_time - sorted_access[i - 1].access_time;
            intervals.push(interval.num_seconds() as f64);
        }

        // Detect bursts (short intervals)
        let mean_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let burst_threshold = mean_interval * 0.5;

        let burst_count = intervals
            .iter()
            .filter(|&&interval| interval < burst_threshold)
            .count();
        let confidence = burst_count as f64 / intervals.len() as f64;

        if confidence > 0.3 {
            let data_points = intervals
                .iter()
                .enumerate()
                .map(|(i, &interval)| PatternDataPoint {
                    timestamp: sorted_access[i + 1].access_time,
                    value: interval,
                    label: Some(format!("Interval {}", i + 1)),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Usage,
                confidence,
                description: "Burst access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect regular patterns
    fn detect_regular_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 10 {
            return None;
        }

        // Sort by timestamp
        let mut sorted_access = content_access.to_vec();
        sorted_access.sort_by_key(|access| access.access_time);

        // Calculate time intervals between accesses
        let mut intervals = Vec::new();
        for i in 1..sorted_access.len() {
            let interval = sorted_access[i].access_time - sorted_access[i - 1].access_time;
            intervals.push(interval.num_seconds() as f64);
        }

        // Calculate regularity (low variance in intervals)
        let mean_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let variance = intervals
            .iter()
            .map(|&interval| {
                let diff = interval - mean_interval;
                diff * diff
            })
            .sum::<f64>()
            / intervals.len() as f64;

        let coefficient_of_variation = (variance.sqrt() / mean_interval).min(1.0);
        let confidence = 1.0 - coefficient_of_variation;

        if confidence > 0.5 {
            let data_points = intervals
                .iter()
                .enumerate()
                .map(|(i, &interval)| PatternDataPoint {
                    timestamp: sorted_access[i + 1].access_time,
                    value: interval,
                    label: Some(format!("Interval {}", i + 1)),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Usage,
                confidence,
                description: "Regular access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect declining patterns
    fn detect_declining_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 10 {
            return None;
        }

        // Sort by timestamp
        let mut sorted_access = content_access.to_vec();
        sorted_access.sort_by_key(|access| access.access_time);

        // Group by time windows and count accesses
        let window_size = ChronoDuration::days(7);
        let mut window_counts = Vec::new();
        let mut current_window_start = sorted_access[0].access_time;
        let mut current_count = 0;

        for access in &sorted_access {
            if access.access_time < current_window_start + window_size {
                current_count += 1;
            } else {
                window_counts.push(current_count);
                current_window_start = access.access_time;
                current_count = 1;
            }
        }
        window_counts.push(current_count);

        if window_counts.len() < 3 {
            return None;
        }

        // Calculate trend (negative slope indicates decline)
        let n = window_counts.len() as f64;
        let sum_x = (0..window_counts.len()).map(|i| i as f64).sum::<f64>();
        let sum_y = window_counts.iter().map(|&x| x as f64).sum::<f64>();
        let sum_xy = window_counts
            .iter()
            .enumerate()
            .map(|(i, &y)| i as f64 * y as f64)
            .sum::<f64>();
        let sum_x2 = (0..window_counts.len())
            .map(|i| (i as f64).powi(2))
            .sum::<f64>();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let confidence = (-slope / 10.0).max(0.0).min(1.0);

        if confidence > 0.3 {
            let data_points = window_counts
                .iter()
                .enumerate()
                .map(|(i, &count)| PatternDataPoint {
                    timestamp: sorted_access[0].access_time + ChronoDuration::days(7 * i as i64),
                    value: count as f64,
                    label: Some(format!("Week {}", i + 1)),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Usage,
                confidence,
                description: "Declining access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect co-access patterns
    fn detect_co_access_pattern(&self, content_access: &[ContentAccess]) -> Option<PatternResult> {
        if content_access.len() < 10 {
            return None;
        }

        // Group accesses by time windows
        let window_size = ChronoDuration::minutes(30);
        let mut window_groups = HashMap::new();
        let mut current_window_start = content_access[0].access_time;

        for access in content_access {
            let window_key = (access.access_time - current_window_start).num_minutes() / 30;
            window_groups
                .entry(window_key)
                .or_insert_with(Vec::new)
                .push(access.content_id.clone());
        }

        // Count co-occurrences
        let mut co_occurrences = HashMap::new();
        for content_ids in window_groups.values() {
            for i in 0..content_ids.len() {
                for j in (i + 1)..content_ids.len() {
                    let key = if content_ids[i] < content_ids[j] {
                        (content_ids[i].clone(), content_ids[j].clone())
                    } else {
                        (content_ids[j].clone(), content_ids[i].clone())
                    };
                    *co_occurrences.entry(key).or_insert(0) += 1;
                }
            }
        }

        if co_occurrences.is_empty() {
            return None;
        }

        let max_co_occurrence = co_occurrences.values().max().unwrap();
        let confidence = (*max_co_occurrence as f64 / window_groups.len() as f64).min(1.0);

        if confidence > 0.1 {
            let data_points = co_occurrences
                .iter()
                .map(|((content1, content2), &count)| PatternDataPoint {
                    timestamp: Utc::now(),
                    value: count as f64,
                    label: Some(format!("{} + {}", content1, content2)),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Relationship,
                confidence,
                description: "Co-access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Detect sequential access patterns
    fn detect_sequential_access_pattern(
        &self,
        content_access: &[ContentAccess],
    ) -> Option<PatternResult> {
        if content_access.len() < 10 {
            return None;
        }

        // Sort by timestamp
        let mut sorted_access = content_access.to_vec();
        sorted_access.sort_by_key(|access| access.access_time);

        // Find sequential patterns
        let mut sequences = HashMap::new();
        for window_size in 2..=4 {
            for i in 0..=(sorted_access.len() - window_size) {
                let sequence: Vec<String> = sorted_access[i..i + window_size]
                    .iter()
                    .map(|access| access.content_id.clone())
                    .collect();
                *sequences.entry(sequence).or_insert(0) += 1;
            }
        }

        if sequences.is_empty() {
            return None;
        }

        let max_sequence_count = sequences.values().max().unwrap();
        let confidence = (*max_sequence_count as f64 / (sorted_access.len() as f64 - 1.0)).min(1.0);

        if confidence > 0.1 {
            let data_points = sequences
                .iter()
                .map(|(sequence, &count)| PatternDataPoint {
                    timestamp: Utc::now(),
                    value: count as f64,
                    label: Some(sequence.join(" -> ")),
                    metadata: HashMap::new(),
                })
                .collect();

            Some(PatternResult {
                pattern_type: PatternType::Sequential,
                confidence,
                description: "Sequential access pattern detected".to_string(),
                data_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_daily_pattern_detection() {
        let detector = PatternDetector::new();
        let mut content_access = Vec::new();

        // Create test data with a daily pattern (more access during work hours)
        for day in 0..7 {
            for hour in 0..24 {
                let count = if hour >= 9 && hour <= 17 { 5 } else { 1 };
                for _ in 0..count {
                    content_access.push(ContentAccess {
                        content_id: "test_content".to_string(),
                        access_time: Utc.ymd(2023, 1, 1 + day).and_hms(hour, 0, 0),
                        access_type: crate::temporal::types::AccessType::Read,
                        user_id: Some("test_user".to_string()),
                        session_id: Some("test_session".to_string()),
                        relevance_score: Some(0.8),
                    });
                }
            }
        }

        let patterns = detector.detect_temporal_patterns(&content_access);
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.description.contains("Daily")));
    }

    #[test]
    fn test_weekly_pattern_detection() {
        let detector = PatternDetector::new();
        let mut content_access = Vec::new();

        // Create test data with a weekly pattern (more access on weekdays)
        for day in 0..14 {
            let count = if day % 7 < 5 { 10 } else { 2 }; // Weekdays vs weekends
            for _ in 0..count {
                content_access.push(ContentAccess {
                    content_id: "test_content".to_string(),
                    access_time: Utc.ymd(2023, 1, 1 + day).and_hms(12, 0, 0),
                    access_type: crate::temporal::types::AccessType::Read,
                    user_id: Some("test_user".to_string()),
                    session_id: Some("test_session".to_string()),
                    relevance_score: Some(0.8),
                });
            }
        }

        let patterns = detector.detect_temporal_patterns(&content_access);
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.description.contains("Weekly")));
    }

    #[test]
    fn test_burst_pattern_detection() {
        let detector = PatternDetector::new();
        let mut content_access = Vec::new();

        // Create test data with burst pattern
        let mut current_time = Utc.ymd(2023, 1, 1).and_hms(0, 0, 0);
        for i in 0..20 {
            let interval = if i % 5 == 0 { 3600 } else { 60 }; // Burst every 5th access
            current_time = current_time + ChronoDuration::seconds(interval);
            content_access.push(ContentAccess {
                content_id: "test_content".to_string(),
                access_time: current_time,
                access_type: crate::temporal::types::AccessType::Read,
                user_id: Some("test_user".to_string()),
                session_id: Some("test_session".to_string()),
                relevance_score: Some(0.8),
            });
        }

        let patterns = detector.detect_usage_patterns(&content_access);
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.description.contains("Burst")));
    }
}
