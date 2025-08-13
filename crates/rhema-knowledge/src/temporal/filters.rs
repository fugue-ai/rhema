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

use chrono::{DateTime, Datelike, Duration as ChronoDuration, Timelike, Utc};
use std::time::Duration;
use tracing::{debug, trace};

use super::types::{SeasonalPeriod, TemporalFilter, TimeRange};
use super::{TemporalError, TemporalResult};

/// Builder for creating temporal filters
pub struct TemporalFilterBuilder {
    filters: Vec<TemporalFilter>,
}

impl TemporalFilterBuilder {
    /// Create a new temporal filter builder
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// Add a filter for content created after a specific time
    pub fn created_after(mut self, datetime: DateTime<Utc>) -> Self {
        self.filters.push(TemporalFilter::CreatedAfter(datetime));
        self
    }

    /// Add a filter for content created before a specific time
    pub fn created_before(mut self, datetime: DateTime<Utc>) -> Self {
        self.filters.push(TemporalFilter::CreatedBefore(datetime));
        self
    }

    /// Add a filter for content modified after a specific time
    pub fn modified_after(mut self, datetime: DateTime<Utc>) -> Self {
        self.filters.push(TemporalFilter::ModifiedAfter(datetime));
        self
    }

    /// Add a filter for content modified before a specific time
    pub fn modified_before(mut self, datetime: DateTime<Utc>) -> Self {
        self.filters.push(TemporalFilter::ModifiedBefore(datetime));
        self
    }

    /// Add a filter for content created within the last N days
    pub fn created_within_days(mut self, days: u64) -> Self {
        let cutoff = Utc::now() - ChronoDuration::days(days as i64);
        self.filters.push(TemporalFilter::CreatedAfter(cutoff));
        self
    }

    /// Add a filter for content created within the last N hours
    pub fn created_within_hours(mut self, hours: u64) -> Self {
        let cutoff = Utc::now() - ChronoDuration::hours(hours as i64);
        self.filters.push(TemporalFilter::CreatedAfter(cutoff));
        self
    }

    /// Add a filter for content created within the last N weeks
    pub fn created_within_weeks(mut self, weeks: u64) -> Self {
        let cutoff = Utc::now() - ChronoDuration::weeks(weeks as i64);
        self.filters.push(TemporalFilter::CreatedAfter(cutoff));
        self
    }

    /// Add a filter for content older than N days
    pub fn older_than_days(mut self, days: u64) -> Self {
        let cutoff = Utc::now() - ChronoDuration::days(days as i64);
        self.filters.push(TemporalFilter::CreatedBefore(cutoff));
        self
    }

    /// Add a filter for content older than N hours
    pub fn older_than_hours(mut self, hours: u64) -> Self {
        let cutoff = Utc::now() - ChronoDuration::hours(hours as i64);
        self.filters.push(TemporalFilter::CreatedBefore(cutoff));
        self
    }

    /// Add a filter for content with age less than specified duration
    pub fn age_less_than(mut self, duration: Duration) -> Self {
        self.filters.push(TemporalFilter::AgeLessThan(duration));
        self
    }

    /// Add a filter for content with age greater than specified duration
    pub fn age_greater_than(mut self, duration: Duration) -> Self {
        self.filters.push(TemporalFilter::AgeGreaterThan(duration));
        self
    }

    /// Add a filter for recently active content
    pub fn recently_active(mut self, min_access_count: u64, within_duration: Duration) -> Self {
        self.filters.push(TemporalFilter::RecentlyActive {
            min_access_count,
            within_duration,
        });
        self
    }

    /// Add a filter for frequently updated content
    pub fn frequently_updated(mut self, min_update_count: u64, within_duration: Duration) -> Self {
        self.filters.push(TemporalFilter::FrequentlyUpdated {
            min_update_count,
            within_duration,
        });
        self
    }

    /// Add a filter for seasonal content
    pub fn seasonal_content(mut self, seasonal_period: SeasonalPeriod) -> Self {
        self.filters
            .push(TemporalFilter::SeasonalContent { seasonal_period });
        self
    }

    /// Add a filter for non-seasonal content
    pub fn non_seasonal_content(mut self) -> Self {
        self.filters.push(TemporalFilter::NonSeasonalContent);
        self
    }

    /// Add a filter for content created in a specific time range
    pub fn in_time_range(mut self, time_range: TimeRange) -> Self {
        self.filters
            .push(TemporalFilter::CreatedAfter(time_range.start));
        self.filters
            .push(TemporalFilter::CreatedBefore(time_range.end));
        self
    }

    /// Add a filter for content created this week
    pub fn this_week(mut self) -> Self {
        let now = Utc::now();
        let start_of_week = now - ChronoDuration::days(now.weekday().num_days_from_monday() as i64);
        let end_of_week = start_of_week + ChronoDuration::days(6);

        self.filters
            .push(TemporalFilter::CreatedAfter(start_of_week));
        self.filters
            .push(TemporalFilter::CreatedBefore(end_of_week));
        self
    }

    /// Add a filter for content created this month
    pub fn this_month(mut self) -> Self {
        let now = Utc::now();
        let start_of_month = now
            .with_day(1)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();
        let end_of_month = if now.month() == 12 {
            now.with_year(now.year() + 1)
                .unwrap()
                .with_month(1)
                .unwrap()
                .with_day(1)
                .unwrap()
        } else {
            now.with_month(now.month() + 1)
                .unwrap()
                .with_day(1)
                .unwrap()
        };

        self.filters
            .push(TemporalFilter::CreatedAfter(start_of_month));
        self.filters
            .push(TemporalFilter::CreatedBefore(end_of_month));
        self
    }

    /// Add a filter for content created this year
    pub fn this_year(mut self) -> Self {
        let now = Utc::now();
        let start_of_year = now
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();
        let end_of_year = now
            .with_month(12)
            .unwrap()
            .with_day(31)
            .unwrap()
            .with_hour(23)
            .unwrap()
            .with_minute(59)
            .unwrap()
            .with_second(59)
            .unwrap();

        self.filters
            .push(TemporalFilter::CreatedAfter(start_of_year));
        self.filters
            .push(TemporalFilter::CreatedBefore(end_of_year));
        self
    }

    /// Add a filter for business hours content (Monday-Friday, 9-17)
    pub fn business_hours(mut self) -> Self {
        // This is a simplified implementation
        // In a real implementation, you might want to check the actual creation time
        // For now, we'll use a placeholder that accepts all content
        // The actual business hours filtering would be done in the timezone module
        self
    }

    /// Add a filter for weekend content
    pub fn weekend_content(mut self) -> Self {
        // This would require checking the day of week for creation time
        // For now, we'll use a placeholder
        self
    }

    /// Build the final filter collection
    pub fn build(self) -> Vec<TemporalFilter> {
        debug!("Built {} temporal filters", self.filters.len());
        self.filters
    }
}

impl Default for TemporalFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for creating common temporal filters
pub struct TemporalFilterUtils;

impl TemporalFilterUtils {
    /// Create a filter for recent content (last 7 days)
    pub fn recent_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new().created_within_days(7).build()
    }

    /// Create a filter for recent content (last 24 hours)
    pub fn very_recent_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .created_within_hours(24)
            .build()
    }

    /// Create a filter for established content (older than 30 days)
    pub fn established_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new().older_than_days(30).build()
    }

    /// Create a filter for active content (accessed recently)
    pub fn active_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .recently_active(1, Duration::from_secs(7 * 24 * 3600)) // At least 1 access in last week
            .build()
    }

    /// Create a filter for frequently accessed content
    pub fn frequently_accessed_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .recently_active(5, Duration::from_secs(30 * 24 * 3600)) // At least 5 accesses in last month
            .build()
    }

    /// Create a filter for content from this week
    pub fn this_week_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new().this_week().build()
    }

    /// Create a filter for content from this month
    pub fn this_month_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new().this_month().build()
    }

    /// Create a filter for content from this year
    pub fn this_year_content() -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new().this_year().build()
    }

    /// Create a filter for seasonal content (e.g., yearly patterns)
    pub fn yearly_seasonal_content(month: u32, day: u32) -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .seasonal_content(SeasonalPeriod::Yearly { month, day })
            .build()
    }

    /// Create a filter for monthly seasonal content
    pub fn monthly_seasonal_content(day: u32) -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .seasonal_content(SeasonalPeriod::Monthly { day })
            .build()
    }

    /// Create a filter for weekly seasonal content
    pub fn weekly_seasonal_content(weekday: u32) -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .seasonal_content(SeasonalPeriod::Weekly { weekday })
            .build()
    }

    /// Create a filter for daily seasonal content
    pub fn daily_seasonal_content(hour: u32) -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .seasonal_content(SeasonalPeriod::Daily { hour })
            .build()
    }

    /// Create a filter for content in a specific time range
    pub fn time_range_content(start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .in_time_range(TimeRange { start, end })
            .build()
    }

    /// Create a filter for content older than a specific age
    pub fn old_content(days: u64) -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new().older_than_days(days).build()
    }

    /// Create a filter for content newer than a specific age
    pub fn new_content(days: u64) -> Vec<TemporalFilter> {
        TemporalFilterBuilder::new()
            .created_within_days(days)
            .build()
    }
}

/// Validator for temporal filters
pub struct TemporalFilterValidator;

impl TemporalFilterValidator {
    /// Validate a collection of temporal filters
    pub fn validate_filters(filters: &[TemporalFilter]) -> TemporalResult<()> {
        for filter in filters {
            Self::validate_single_filter(filter)?;
        }
        Ok(())
    }

    /// Validate a single temporal filter
    pub fn validate_single_filter(filter: &TemporalFilter) -> TemporalResult<()> {
        match filter {
            TemporalFilter::CreatedAfter(datetime)
            | TemporalFilter::CreatedBefore(datetime)
            | TemporalFilter::ModifiedAfter(datetime)
            | TemporalFilter::ModifiedBefore(datetime) => {
                if datetime > &(Utc::now() + ChronoDuration::days(365)) {
                    return Err(TemporalError::InvalidData(
                        "Filter datetime is too far in the future".to_string(),
                    ));
                }
            }
            TemporalFilter::AgeLessThan(duration) | TemporalFilter::AgeGreaterThan(duration) => {
                if duration.as_secs() == 0 {
                    return Err(TemporalError::InvalidData(
                        "Duration cannot be zero".to_string(),
                    ));
                }
            }
            TemporalFilter::RecentlyActive {
                min_access_count,
                within_duration,
            } => {
                if *min_access_count == 0 {
                    return Err(TemporalError::InvalidData(
                        "Minimum access count cannot be zero".to_string(),
                    ));
                }
                if within_duration.as_secs() == 0 {
                    return Err(TemporalError::InvalidData(
                        "Within duration cannot be zero".to_string(),
                    ));
                }
            }
            TemporalFilter::FrequentlyUpdated {
                min_update_count,
                within_duration,
            } => {
                if *min_update_count == 0 {
                    return Err(TemporalError::InvalidData(
                        "Minimum update count cannot be zero".to_string(),
                    ));
                }
                if within_duration.as_secs() == 0 {
                    return Err(TemporalError::InvalidData(
                        "Within duration cannot be zero".to_string(),
                    ));
                }
            }
            TemporalFilter::SeasonalContent { seasonal_period } => {
                Self::validate_seasonal_period(seasonal_period)?;
            }
            TemporalFilter::NonSeasonalContent => {
                // No validation needed
            }
        }
        Ok(())
    }

    /// Validate a seasonal period
    fn validate_seasonal_period(period: &SeasonalPeriod) -> TemporalResult<()> {
        match period {
            SeasonalPeriod::Yearly { month, day } => {
                if *month < 1 || *month > 12 {
                    return Err(TemporalError::InvalidData(format!(
                        "Invalid month: {}",
                        month
                    )));
                }
                if *day < 1 || *day > 31 {
                    return Err(TemporalError::InvalidData(format!("Invalid day: {}", day)));
                }
            }
            SeasonalPeriod::Monthly { day } => {
                if *day < 1 || *day > 31 {
                    return Err(TemporalError::InvalidData(format!("Invalid day: {}", day)));
                }
            }
            SeasonalPeriod::Weekly { weekday } => {
                if *weekday > 6 {
                    return Err(TemporalError::InvalidData(format!(
                        "Invalid weekday: {}",
                        weekday
                    )));
                }
            }
            SeasonalPeriod::Daily { hour } => {
                if *hour > 23 {
                    return Err(TemporalError::InvalidData(format!(
                        "Invalid hour: {}",
                        hour
                    )));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;

    #[test]
    fn test_temporal_filter_builder() {
        let filters = TemporalFilterBuilder::new()
            .created_within_days(7)
            .older_than_days(1)
            .build();

        assert_eq!(filters.len(), 2);
    }

    #[test]
    fn test_recent_content_filter() {
        let filters = TemporalFilterUtils::recent_content();
        assert_eq!(filters.len(), 1);

        match &filters[0] {
            TemporalFilter::CreatedAfter(_) => {}
            _ => panic!("Expected CreatedAfter filter"),
        }
    }

    #[test]
    fn test_established_content_filter() {
        let filters = TemporalFilterUtils::established_content();
        assert_eq!(filters.len(), 1);

        match &filters[0] {
            TemporalFilter::CreatedBefore(_) => {}
            _ => panic!("Expected CreatedBefore filter"),
        }
    }

    #[test]
    fn test_seasonal_content_filter() {
        let filters = TemporalFilterUtils::yearly_seasonal_content(12, 25);
        assert_eq!(filters.len(), 1);

        match &filters[0] {
            TemporalFilter::SeasonalContent { seasonal_period } => match seasonal_period {
                SeasonalPeriod::Yearly { month, day } => {
                    assert_eq!(*month, 12);
                    assert_eq!(*day, 25);
                }
                _ => panic!("Expected Yearly seasonal period"),
            },
            _ => panic!("Expected SeasonalContent filter"),
        }
    }

    #[test]
    fn test_filter_validation() {
        // Valid filter
        let valid_filter = TemporalFilter::CreatedAfter(Utc::now() - ChronoDuration::days(1));
        assert!(TemporalFilterValidator::validate_single_filter(&valid_filter).is_ok());

        // Invalid filter (future date too far)
        let invalid_filter = TemporalFilter::CreatedAfter(Utc::now() + ChronoDuration::days(400));
        assert!(TemporalFilterValidator::validate_single_filter(&invalid_filter).is_err());
    }

    #[test]
    fn test_time_range_filter() {
        let start = Utc::now() - ChronoDuration::days(30);
        let end = Utc::now();
        let filters = TemporalFilterUtils::time_range_content(start, end);

        assert_eq!(filters.len(), 2);

        match &filters[0] {
            TemporalFilter::CreatedAfter(datetime) => {
                assert_eq!(*datetime, start);
            }
            _ => panic!("Expected CreatedAfter filter"),
        }

        match &filters[1] {
            TemporalFilter::CreatedBefore(datetime) => {
                assert_eq!(*datetime, end);
            }
            _ => panic!("Expected CreatedBefore filter"),
        }
    }
}
