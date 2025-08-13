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
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::types::ContentType;

/// Decay function types for different content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecayFunction {
    /// Documentation decays slowly over years
    Documentation { half_life_days: f64 },
    /// Code decays faster, typically over weeks
    Code { half_life_hours: f64 },
    /// Decisions decay over months to years
    Decisions { half_life_weeks: f64 },
    /// Knowledge uses adaptive decay based on usage patterns
    Knowledge { adaptive_decay: AdaptiveDecayConfig },
    /// Patterns have stable periods with update cycles
    Patterns {
        stable_period_days: f64,
        update_cycle_days: f64,
    },
}

/// Adaptive decay configuration for knowledge content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveDecayConfig {
    pub base_half_life_days: f64,
    pub access_boost_factor: f64,
    pub relevance_threshold: f64,
    pub max_boost: f64,
}

impl Default for AdaptiveDecayConfig {
    fn default() -> Self {
        Self {
            base_half_life_days: 30.0,
            access_boost_factor: 0.1,
            relevance_threshold: 0.5,
            max_boost: 5.0,
        }
    }
}

/// Temporal relationship types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalRelationshipType {
    /// Sequential relationship with order and gap
    Sequential {
        order: usize,
        gap_duration: Duration,
    },
    /// Concurrent relationship with overlap
    Concurrent { overlap_duration: Duration },
    /// Cyclical relationship with period and phase
    Cyclical {
        cycle_period: Duration,
        phase_offset: Duration,
    },
    /// Causal relationship with confidence and direction
    Causal {
        confidence: f64,
        direction: CausalDirection,
    },
    /// Seasonal relationship with period and strength
    Seasonal {
        seasonal_period: SeasonalPeriod,
        strength: f64,
    },
}

/// Causal direction for temporal relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CausalDirection {
    Forward,       // A causes B
    Backward,      // B causes A
    Bidirectional, // A and B influence each other
}

/// Seasonal periods for pattern detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeasonalPeriod {
    /// Yearly patterns (e.g., annual reviews, seasonal code)
    Yearly { month: u32, day: u32 },
    /// Monthly patterns (e.g., monthly reports, recurring tasks)
    Monthly { day: u32 },
    /// Weekly patterns (e.g., weekly standups, weekend deployments)
    Weekly { weekday: u32 },
    /// Daily patterns (e.g., business hours, daily builds)
    Daily { hour: u32 },
}

/// Time range for temporal queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    /// Create a time range for the last N days
    pub fn last_n_days(days: u64) -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::days(days as i64);
        Self { start, end }
    }

    /// Create a time range for the last N hours
    pub fn last_n_hours(hours: u64) -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::hours(hours as i64);
        Self { start, end }
    }

    /// Create a time range for the last N weeks
    pub fn last_n_weeks(weeks: u64) -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::weeks(weeks as i64);
        Self { start, end }
    }

    /// Check if a datetime is within this range
    pub fn contains(&self, dt: &DateTime<Utc>) -> bool {
        dt >= &self.start && dt <= &self.end
    }

    /// Get the duration of this range
    pub fn duration(&self) -> Duration {
        Duration::from_secs((self.end - self.start).num_seconds() as u64)
    }
}

/// Temporal filters for search queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalFilter {
    /// Filter by creation time
    CreatedAfter(DateTime<Utc>),
    CreatedBefore(DateTime<Utc>),
    /// Filter by modification time
    ModifiedAfter(DateTime<Utc>),
    ModifiedBefore(DateTime<Utc>),
    /// Filter by age
    AgeLessThan(Duration),
    AgeGreaterThan(Duration),
    /// Filter by activity patterns
    RecentlyActive {
        min_access_count: u64,
        within_duration: Duration,
    },
    FrequentlyUpdated {
        min_update_count: u64,
        within_duration: Duration,
    },
    /// Filter by seasonal content
    SeasonalContent {
        seasonal_period: SeasonalPeriod,
    },
    NonSeasonalContent,
}

/// Freshness preferences for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FreshnessPreference {
    /// Prefer recent content
    PreferRecent { weight: f64 },
    /// Prefer established content
    PreferEstablished { min_age_days: f64, weight: f64 },
    /// Balanced approach
    Balanced {
        recent_weight: f64,
        established_weight: f64,
    },
    /// Content-type specific preferences
    ContentTypeSpecific {
        preferences: std::collections::HashMap<ContentType, f64>,
    },
}

impl Default for FreshnessPreference {
    fn default() -> Self {
        Self::Balanced {
            recent_weight: 0.6,
            established_weight: 0.4,
        }
    }
}

/// Seasonal preferences for search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalPreference {
    pub seasonal_period: SeasonalPeriod,
    pub weight: f64,
    pub tolerance_days: u32,
}

/// Timezone context for temporal awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneContext {
    pub user_timezone: String,
    pub query_time_local: DateTime<chrono::Local>,
    pub business_hours_start: u32, // Hour in 24-hour format
    pub business_hours_end: u32,   // Hour in 24-hour format
    pub is_business_hours: bool,
    pub day_of_week: u32, // 0 = Sunday, 1 = Monday, etc.
}

impl TimezoneContext {
    /// Create a timezone context from a timezone string
    pub fn new(timezone: &str, query_time: DateTime<Utc>) -> Result<Self, chrono::ParseError> {
        // For now, we'll use UTC as a fallback since chrono_tz is not available
        // In a real implementation, this would parse the timezone properly
        let local_time = query_time; // Use UTC as local time for now

        let hour = local_time.hour();
        let day_of_week = local_time.weekday().num_days_from_sunday();

        let is_business_hours = day_of_week >= 1 && day_of_week <= 5 && hour >= 9 && hour < 17;

        Ok(Self {
            user_timezone: timezone.to_string(),
            query_time_local: local_time.into(),
            business_hours_start: 9,
            business_hours_end: 17,
            is_business_hours,
            day_of_week,
        })
    }
}

/// Content access record for temporal analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAccess {
    pub content_id: String,
    pub access_time: DateTime<Utc>,
    pub access_type: AccessType,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub relevance_score: Option<f64>,
}

/// Types of content access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    Search,
    Cache,
    Synthesis,
}

/// Temporal search query with enhanced context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalSearchQuery {
    pub base_query: String,
    pub time_range: Option<TimeRange>,
    pub temporal_filters: Vec<TemporalFilter>,
    pub freshness_preference: FreshnessPreference,
    pub seasonal_preferences: Vec<SeasonalPreference>,
    pub timezone_context: Option<TimezoneContext>,
    pub content_types: Vec<ContentType>,
    pub max_results: usize,
}

impl Default for TemporalSearchQuery {
    fn default() -> Self {
        Self {
            base_query: String::new(),
            time_range: None,
            temporal_filters: Vec::new(),
            freshness_preference: FreshnessPreference::default(),
            seasonal_preferences: Vec::new(),
            timezone_context: None,
            content_types: Vec::new(),
            max_results: 10,
        }
    }
}

impl TemporalSearchQuery {
    /// Create a new temporal search query
    pub fn new(base_query: String) -> Self {
        Self {
            base_query,
            ..Default::default()
        }
    }

    /// Add a time range filter
    pub fn with_time_range(mut self, time_range: TimeRange) -> Self {
        self.time_range = Some(time_range);
        self
    }

    /// Add a temporal filter
    pub fn with_filter(mut self, filter: TemporalFilter) -> Self {
        self.temporal_filters.push(filter);
        self
    }

    /// Set freshness preference
    pub fn with_freshness_preference(mut self, preference: FreshnessPreference) -> Self {
        self.freshness_preference = preference;
        self
    }

    /// Add seasonal preference
    pub fn with_seasonal_preference(mut self, preference: SeasonalPreference) -> Self {
        self.seasonal_preferences.push(preference);
        self
    }

    /// Set timezone context
    pub fn with_timezone_context(mut self, context: TimezoneContext) -> Self {
        self.timezone_context = Some(context);
        self
    }

    /// Filter by content types
    pub fn with_content_types(mut self, content_types: Vec<ContentType>) -> Self {
        self.content_types = content_types;
        self
    }

    /// Set maximum results
    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = max_results;
        self
    }
}

/// Temporal relevance score breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalRelevanceBreakdown {
    pub base_decay_score: f64,
    pub seasonal_adjustment: f64,
    pub timezone_adjustment: f64,
    pub relationship_score: f64,
    pub freshness_score: f64,
    pub final_score: f64,
    pub contributing_factors: Vec<TemporalFactor>,
}

/// Contributing factors to temporal relevance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalFactor {
    pub factor_type: TemporalFactorType,
    pub weight: f64,
    pub score: f64,
    pub description: String,
}

/// Types of temporal factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalFactorType {
    BaseDecay,
    Seasonal,
    Timezone,
    Relationship,
    Freshness,
    ContentType,
    AccessPattern,
}
