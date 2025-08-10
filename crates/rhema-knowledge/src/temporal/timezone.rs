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

use chrono::{DateTime, Utc, TimeZone, Timelike};
use std::collections::HashMap;
use tracing::{debug, info, trace};

use super::{TimezoneContext, TimezoneConfig, Content};
use super::{TemporalResult, TemporalError};

/// Timezone-aware context manager for temporal adjustments
pub struct TimezoneAwareContextManager {
    config: TimezoneConfig,
    timezone_cache: HashMap<String, String>,
}

impl TimezoneAwareContextManager {
    /// Create a new timezone-aware context manager with default configuration
    pub fn new() -> Self {
        Self {
            config: TimezoneConfig::default(),
            timezone_cache: HashMap::new(),
        }
    }

    /// Create a new timezone-aware context manager with custom configuration
    pub fn with_config(config: TimezoneConfig) -> Self {
        Self {
            config,
            timezone_cache: HashMap::new(),
        }
    }

    /// Calculate timezone adjustment for content relevance
    pub async fn calculate_timezone_adjustment(
        &self,
        content: &Content,
        query_time: DateTime<Utc>,
        user_timezone: &str,
    ) -> TemporalResult<f64> {
        if !self.config.enabled {
            return Ok(1.0);
        }

        let timezone_context = TimezoneContext::new(user_timezone, query_time)
            .map_err(|e| TemporalError::TimezoneError(format!("Invalid timezone: {}", e)))?;

        let adjustment = self.calculate_business_hours_adjustment(&timezone_context);
        
        trace!(
            "Timezone adjustment for content {} in {}: {:.3}",
            content.id, user_timezone, adjustment
        );

        Ok(adjustment)
    }

    /// Create timezone context for a specific timezone and query time
    pub fn create_timezone_context(
        &self,
        timezone: &str,
        query_time: DateTime<Utc>,
    ) -> TemporalResult<TimezoneContext> {
        TimezoneContext::new(timezone, query_time)
            .map_err(|e| TemporalError::TimezoneError(format!("Invalid timezone: {}", e)))
    }

    /// Calculate business hours adjustment based on timezone context
    fn calculate_business_hours_adjustment(&self, context: &TimezoneContext) -> f64 {
        let mut adjustment = 1.0;

        // Business hours adjustment
        if context.is_business_hours {
            adjustment *= self.config.business_hours_boost;
        } else {
            adjustment *= self.config.off_hours_penalty;
        }

        // Weekend adjustment
        if context.day_of_week == 0 || context.day_of_week == 6 {
            adjustment *= 0.8; // Lower relevance on weekends
        }

        // Early morning and late night adjustments
        let hour = context.query_time_local.hour();
        if hour < 6 || hour > 22 {
            adjustment *= 0.7; // Lower relevance during off-hours
        }

        adjustment.min(1.5_f64).max(0.3_f64) // Clamp to reasonable range
    }

    /// Get timezone relationships for collaborative adjustments
    pub fn get_timezone_relationships(&self, timezone: &str) -> TemporalResult<Vec<String>> {
        // This would typically load from a configuration or database
        // For now, we'll return some common related timezones
        let relationships = match timezone {
            "America/New_York" => vec![
                "America/Chicago".to_string(),
                "America/Denver".to_string(),
                "America/Los_Angeles".to_string(),
                "Europe/London".to_string(),
            ],
            "America/Los_Angeles" => vec![
                "America/Denver".to_string(),
                "America/Chicago".to_string(),
                "America/New_York".to_string(),
                "Asia/Tokyo".to_string(),
            ],
            "Europe/London" => vec![
                "Europe/Paris".to_string(),
                "Europe/Berlin".to_string(),
                "America/New_York".to_string(),
                "Asia/Dubai".to_string(),
            ],
            "Asia/Tokyo" => vec![
                "Asia/Shanghai".to_string(),
                "Asia/Seoul".to_string(),
                "America/Los_Angeles".to_string(),
                "Australia/Sydney".to_string(),
            ],
            _ => vec![], // No specific relationships for other timezones
        };

        Ok(relationships)
    }

    /// Calculate collaborative timezone adjustment for distributed teams
    pub async fn calculate_collaborative_adjustment(
        &self,
        content: &Content,
        query_time: DateTime<Utc>,
        user_timezone: &str,
        team_timezones: &[String],
    ) -> TemporalResult<f64> {
        if !self.config.enabled || team_timezones.is_empty() {
            return Ok(1.0);
        }

        let mut total_adjustment = 0.0;
        let mut valid_timezones = 0;

        // Calculate adjustment for each team member's timezone
        for team_tz in team_timezones {
            if let Ok(adjustment) = self.calculate_timezone_adjustment(content, query_time, team_tz).await {
                total_adjustment += adjustment;
                valid_timezones += 1;
            }
        }

        if valid_timezones == 0 {
            return Ok(1.0);
        }

        let collaborative_adjustment = total_adjustment / valid_timezones as f64;
        
        debug!(
            "Collaborative timezone adjustment: {} timezones, avg_adjustment={:.3}",
            valid_timezones, collaborative_adjustment
        );

        Ok(collaborative_adjustment)
    }

    /// Extract local time context from content metadata
    pub fn extract_local_time_context(
        &self,
        content: &Content,
        user_timezone: &str,
    ) -> TemporalResult<Option<LocalTimeContext>> {
        // Look for timezone-related metadata in content
        if let Some(timezone_str) = content.metadata.get("timezone") {
            let content_timezone = timezone_str.as_str();
            
            // If content has a different timezone than user, calculate time difference
            if content_timezone != user_timezone {
                let time_diff = self.calculate_timezone_difference(content_timezone, user_timezone)?;
                
                return Ok(Some(LocalTimeContext {
                    content_timezone: content_timezone.to_string(),
                    user_timezone: user_timezone.to_string(),
                    time_difference_hours: time_diff,
                    is_same_timezone: false,
                }));
            }
        }

        Ok(Some(LocalTimeContext {
            content_timezone: user_timezone.to_string(),
            user_timezone: user_timezone.to_string(),
            time_difference_hours: 0,
            is_same_timezone: true,
        }))
    }

    /// Calculate time difference between two timezones
    fn calculate_timezone_difference(
        &self,
        tz1: &str,
        tz2: &str,
    ) -> TemporalResult<i32> {
        let timezone1: String = tz1.to_string();
        let timezone2: String = tz2.to_string();

        let now = Utc::now();
        // For now, we'll use UTC as a fallback since we don't have proper timezone parsing
        // In a real implementation, this would convert to the actual timezones
        let time1 = now;
        let time2 = now;
        
        let diff = time1.signed_duration_since(time2);
        let diff_hours = diff.num_hours();

        Ok(diff_hours.try_into().unwrap_or(0))
    }

    /// Get timezone-specific business hours
    pub fn get_business_hours(&self, timezone: &str) -> TemporalResult<BusinessHours> {
        // This would typically load from a configuration or database
        // For now, we'll return standard business hours for common timezones
        let business_hours = match timezone {
            "America/New_York" | "America/Chicago" | "America/Denver" | "America/Los_Angeles" => {
                BusinessHours {
                    start_hour: 9,
                    end_hour: 17,
                    days_of_week: vec![1, 2, 3, 4, 5], // Monday-Friday
                }
            }
            "Europe/London" | "Europe/Paris" | "Europe/Berlin" => {
                BusinessHours {
                    start_hour: 9,
                    end_hour: 18,
                    days_of_week: vec![1, 2, 3, 4, 5], // Monday-Friday
                }
            }
            "Asia/Tokyo" | "Asia/Shanghai" | "Asia/Seoul" => {
                BusinessHours {
                    start_hour: 9,
                    end_hour: 18,
                    days_of_week: vec![1, 2, 3, 4, 5], // Monday-Friday
                }
            }
            _ => {
                // Default business hours
                BusinessHours {
                    start_hour: 9,
                    end_hour: 17,
                    days_of_week: vec![1, 2, 3, 4, 5], // Monday-Friday
                }
            }
        };

        Ok(business_hours)
    }

    /// Get the configuration
    pub fn config(&self) -> &TimezoneConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: TimezoneConfig) {
        self.config = config;
        info!("Updated timezone-aware context manager configuration");
    }
}

impl Default for TimezoneAwareContextManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Local time context for content
#[derive(Debug, Clone)]
pub struct LocalTimeContext {
    pub content_timezone: String,
    pub user_timezone: String,
    pub time_difference_hours: i32,
    pub is_same_timezone: bool,
}

/// Business hours configuration for a timezone
#[derive(Debug, Clone)]
pub struct BusinessHours {
    pub start_hour: u32,
    pub end_hour: u32,
    pub days_of_week: Vec<u32>, // 0 = Sunday, 1 = Monday, etc.
}

impl BusinessHours {
    /// Check if a given time is within business hours
    pub fn is_business_hours(&self, hour: u32, day_of_week: u32) -> bool {
        self.days_of_week.contains(&day_of_week) && 
        hour >= self.start_hour && 
        hour < self.end_hour
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_content() -> Content {
        Content {
            id: "test_content".to_string(),
            content_type: crate::types::ContentType::Code,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            content: "test content".to_string(),
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_timezone_adjustment() {
        let manager = TimezoneAwareContextManager::new();
        let content = create_test_content();
        let query_time = Utc::now();

        let adjustment = manager.calculate_timezone_adjustment(
            &content,
            query_time,
            "America/New_York",
        ).await.unwrap();

        assert!(adjustment > 0.0 && adjustment <= 1.5);
    }

    #[tokio::test]
    async fn test_collaborative_adjustment() {
        let manager = TimezoneAwareContextManager::new();
        let content = create_test_content();
        let query_time = Utc::now();
        let team_timezones = vec![
            "America/New_York".to_string(),
            "America/Los_Angeles".to_string(),
            "Europe/London".to_string(),
        ];

        let adjustment = manager.calculate_collaborative_adjustment(
            &content,
            query_time,
            "America/New_York",
            &team_timezones,
        ).await.unwrap();

        assert!(adjustment > 0.0 && adjustment <= 1.5);
    }

    #[test]
    fn test_timezone_relationships() {
        let manager = TimezoneAwareContextManager::new();
        
        let relationships = manager.get_timezone_relationships("America/New_York").unwrap();
        assert!(!relationships.is_empty());
        assert!(relationships.contains(&"America/Chicago".to_string()));
    }

    #[test]
    fn test_business_hours() {
        let manager = TimezoneAwareContextManager::new();
        
        let business_hours = manager.get_business_hours("America/New_York").unwrap();
        assert_eq!(business_hours.start_hour, 9);
        assert_eq!(business_hours.end_hour, 17);
        assert!(business_hours.days_of_week.contains(&1)); // Monday
    }

    #[test]
    fn test_business_hours_check() {
        let business_hours = BusinessHours {
            start_hour: 9,
            end_hour: 17,
            days_of_week: vec![1, 2, 3, 4, 5],
        };

        assert!(business_hours.is_business_hours(14, 1)); // Monday 2 PM
        assert!(!business_hours.is_business_hours(8, 1)); // Monday 8 AM
        assert!(!business_hours.is_business_hours(14, 0)); // Sunday 2 PM
    }
}
