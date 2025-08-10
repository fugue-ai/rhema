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
use std::time::Duration;
use tracing::{debug, trace};

use crate::types::ContentType;
use super::types::{DecayFunction, AdaptiveDecayConfig, ContentAccess, AccessType};
use super::{TemporalResult, TemporalError};

/// Decay function calculator for temporal relevance
pub struct DecayCalculator {
    decay_functions: std::collections::HashMap<ContentType, DecayFunction>,
}

impl DecayCalculator {
    /// Create a new decay calculator with default functions
    pub fn new() -> Self {
        Self {
            decay_functions: Self::default_decay_functions(),
        }
    }

    /// Create a new decay calculator with custom functions
    pub fn with_functions(decay_functions: std::collections::HashMap<ContentType, DecayFunction>) -> Self {
        Self { decay_functions }
    }

    /// Calculate decay for content based on its type and age
    pub fn calculate_decay(
        &self,
        content_type: &ContentType,
        created_at: &DateTime<Utc>,
        query_time: &DateTime<Utc>,
        access_history: Option<&[ContentAccess]>,
    ) -> TemporalResult<f64> {
        let age = query_time.signed_duration_since(*created_at);
        let age_duration = Duration::from_secs(age.num_seconds() as u64);

        let decay_function = self.decay_functions
            .get(content_type)
            .ok_or_else(|| TemporalError::ConfigurationError(
                format!("No decay function configured for content type: {:?}", content_type)
            ))?;

        let base_decay = decay_function.calculate_decay(age_duration)?;

        // Apply adaptive adjustments if access history is available
        if let Some(history) = access_history {
            let adaptive_adjustment = self.calculate_adaptive_adjustment(decay_function, history)?;
            let final_decay = base_decay * adaptive_adjustment;
            
            debug!(
                "Decay calculation for {:?}: base={:.3}, adaptive={:.3}, final={:.3}",
                content_type, base_decay, adaptive_adjustment, final_decay
            );
            
            Ok(final_decay.min(1.0).max(0.0))
        } else {
            Ok(base_decay.min(1.0).max(0.0))
        }
    }

    /// Calculate adaptive adjustment based on access patterns
    fn calculate_adaptive_adjustment(
        &self,
        decay_function: &DecayFunction,
        access_history: &[ContentAccess],
    ) -> TemporalResult<f64> {
        match decay_function {
            DecayFunction::Knowledge { adaptive_decay } => {
                self.calculate_knowledge_adaptive_adjustment(adaptive_decay, access_history)
            }
            _ => Ok(1.0), // No adaptive adjustment for other types
        }
    }

    /// Calculate adaptive adjustment for knowledge content
    fn calculate_knowledge_adaptive_adjustment(
        &self,
        config: &AdaptiveDecayConfig,
        access_history: &[ContentAccess],
    ) -> TemporalResult<f64> {
        let now = Utc::now();
        let recent_window = Duration::from_secs((30 * 24 * 3600) as u64); // 30 days

        // Calculate recent access frequency
        let recent_accesses = access_history
            .iter()
            .filter(|access| {
                let access_age = now.signed_duration_since(access.access_time);
                access_age.num_seconds() as u64 <= recent_window.as_secs()
            })
            .count();

        // Calculate average relevance score from recent accesses
        let recent_relevance: f64 = access_history
            .iter()
            .filter(|access| {
                let access_age = now.signed_duration_since(access.access_time);
                access_age.num_seconds() as u64 <= recent_window.as_secs()
            })
            .filter_map(|access| access.relevance_score)
            .sum::<f64>();

        let avg_relevance = if recent_accesses > 0 {
            recent_relevance / recent_accesses as f64
        } else {
            0.0
        };

        // Calculate boost based on access frequency and relevance
        let frequency_boost = (recent_accesses as f64 * config.access_boost_factor).min(config.max_boost);
        let relevance_boost = if avg_relevance > config.relevance_threshold {
            (avg_relevance - config.relevance_threshold) * 2.0
        } else {
            0.0
        };

        let total_boost = 1.0 + frequency_boost + relevance_boost;

        trace!(
            "Knowledge adaptive adjustment: accesses={}, avg_relevance={:.3}, boost={:.3}",
            recent_accesses, avg_relevance, total_boost
        );

        Ok(total_boost)
    }

    /// Get default decay functions for all content types
    fn default_decay_functions() -> std::collections::HashMap<ContentType, DecayFunction> {
        let mut functions = std::collections::HashMap::new();
        
        functions.insert(ContentType::Documentation, DecayFunction::Documentation { half_life_days: 365.0 });
        functions.insert(ContentType::Code, DecayFunction::Code { half_life_hours: 168.0 });
        functions.insert(ContentType::Decision, DecayFunction::Decisions { half_life_weeks: 52.0 });
        functions.insert(ContentType::Knowledge, DecayFunction::Knowledge { 
            adaptive_decay: AdaptiveDecayConfig::default() 
        });
        functions.insert(ContentType::Pattern, DecayFunction::Patterns { 
            stable_period_days: 90.0, 
            update_cycle_days: 30.0 
        });
        functions.insert(ContentType::Configuration, DecayFunction::Documentation { half_life_days: 180.0 });
        functions.insert(ContentType::Todo, DecayFunction::Code { half_life_hours: 72.0 });
        functions.insert(ContentType::Insight, DecayFunction::Knowledge { 
            adaptive_decay: AdaptiveDecayConfig::default() 
        });
        functions.insert(ContentType::Unknown, DecayFunction::Documentation { half_life_days: 365.0 });

        functions
    }
}

impl DecayFunction {
    /// Calculate decay value for a given age
    pub fn calculate_decay(&self, age: Duration) -> TemporalResult<f64> {
        match self {
            DecayFunction::Documentation { half_life_days } => {
                self.calculate_exponential_decay(age, *half_life_days * 24.0 * 3600.0)
            }
            DecayFunction::Code { half_life_hours } => {
                self.calculate_exponential_decay(age, *half_life_hours * 3600.0)
            }
            DecayFunction::Decisions { half_life_weeks } => {
                self.calculate_exponential_decay(age, *half_life_weeks * 7.0 * 24.0 * 3600.0)
            }
            DecayFunction::Knowledge { adaptive_decay } => {
                self.calculate_exponential_decay(age, adaptive_decay.base_half_life_days * 24.0 * 3600.0)
            }
            DecayFunction::Patterns { stable_period_days, update_cycle_days } => {
                self.calculate_pattern_decay(age, *stable_period_days, *update_cycle_days)
            }
        }
    }

    /// Calculate exponential decay
    fn calculate_exponential_decay(&self, age: Duration, half_life_seconds: f64) -> TemporalResult<f64> {
        if half_life_seconds <= 0.0 {
            return Err(TemporalError::ConfigurationError(
                "Half-life must be positive".to_string()
            ));
        }

        let age_seconds = age.as_secs_f64();
        let decay = (-age_seconds / half_life_seconds).exp();
        
        trace!(
            "Exponential decay: age={:.1}s, half_life={:.1}s, decay={:.3}",
            age_seconds, half_life_seconds, decay
        );

        Ok(decay)
    }

    /// Calculate pattern-specific decay with stable periods and update cycles
    fn calculate_pattern_decay(
        &self,
        age: Duration,
        stable_period_days: f64,
        update_cycle_days: f64,
    ) -> TemporalResult<f64> {
        let age_days = age.as_secs_f64() / (24.0 * 3600.0);
        
        // During stable period, decay is minimal
        if age_days <= stable_period_days {
            let stability_factor = 1.0 - (age_days / stable_period_days) * 0.1;
            return Ok(stability_factor);
        }

        // After stable period, apply cyclical decay based on update cycles
        let cycles_since_stable = (age_days - stable_period_days) / update_cycle_days;
        let cycle_decay = (-cycles_since_stable * 0.5).exp();
        
        trace!(
            "Pattern decay: age={:.1} days, stable_period={:.1}, cycles={:.1}, decay={:.3}",
            age_days, stable_period_days, cycles_since_stable, cycle_decay
        );

        Ok(cycle_decay)
    }
}

impl Default for DecayCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;

    #[test]
    fn test_exponential_decay() {
        let decay_fn = DecayFunction::Documentation { half_life_days: 365.0 };
        
        // Test at half-life
        let half_life_duration = Duration::from_secs((365 * 24 * 3600) as u64);
        let decay = decay_fn.calculate_decay(half_life_duration).unwrap();
        assert!((decay - 0.5).abs() < 0.01);

        // Test at zero age
        let zero_duration = Duration::from_secs(0);
        let decay = decay_fn.calculate_decay(zero_duration).unwrap();
        assert!((decay - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_pattern_decay() {
        let decay_fn = DecayFunction::Patterns { 
            stable_period_days: 90.0, 
            update_cycle_days: 30.0 
        };

        // Test during stable period
        let stable_duration = Duration::from_secs((45 * 24 * 3600) as u64); // 45 days
        let decay = decay_fn.calculate_decay(stable_duration).unwrap();
        assert!(decay > 0.9); // Should be very stable

        // Test after stable period
        let post_stable_duration = Duration::from_secs((120 * 24 * 3600) as u64); // 120 days
        let decay = decay_fn.calculate_decay(post_stable_duration).unwrap();
        assert!(decay < 0.9); // Should have some decay
    }

    #[test]
    fn test_decay_calculator() {
        let calculator = DecayCalculator::new();
        let now = Utc::now();
        let created_at = now - ChronoDuration::days(30);

        let decay = calculator.calculate_decay(
            &ContentType::Documentation,
            &created_at,
            &now,
            None,
        ).unwrap();

        assert!(decay > 0.0 && decay < 1.0);
    }

    #[test]
    fn test_adaptive_knowledge_decay() {
        let calculator = DecayCalculator::new();
        let now = Utc::now();
        let created_at = now - ChronoDuration::days(60);

        let access_history = vec![
            ContentAccess {
                content_id: "test".to_string(),
                access_time: now - ChronoDuration::days(1),
                access_type: AccessType::Read,
                user_id: Some("user1".to_string()),
                session_id: Some("session1".to_string()),
                relevance_score: Some(0.8),
            },
            ContentAccess {
                content_id: "test".to_string(),
                access_time: now - ChronoDuration::days(2),
                access_type: AccessType::Search,
                user_id: Some("user2".to_string()),
                session_id: Some("session2".to_string()),
                relevance_score: Some(0.9),
            },
        ];

        let decay = calculator.calculate_decay(
            &ContentType::Knowledge,
            &created_at,
            &now,
            Some(&access_history),
        ).unwrap();

        assert!(decay > 0.0 && decay <= 1.0);
    }
}
