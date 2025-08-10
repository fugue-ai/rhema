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

use chrono::{DateTime, Utc, Duration as ChronoDuration};
use std::collections::HashMap;
use std::time::Duration;

use rhema_knowledge::temporal::*;
use rhema_knowledge::types::ContentType;

/// Test utilities for temporal context tests
mod test_utils {
    use super::*;

    pub fn create_test_content(id: &str, content_type: ContentType, age_days: i64) -> Content {
        let now = Utc::now();
        let created_at = now - ChronoDuration::days(age_days);
        let modified_at = created_at + ChronoDuration::days(age_days / 2);
        let accessed_at = now - ChronoDuration::days(age_days / 4);

        Content {
            id: id.to_string(),
            content_type,
            created_at,
            modified_at,
            accessed_at,
            access_count: 10,
            content: format!("Test content for {}", id),
            metadata: HashMap::new(),
        }
    }

    pub fn create_test_semantic_result(id: &str, age_days: i64) -> SemanticResult {
        let now = Utc::now();
        let created_at = now - ChronoDuration::days(age_days);

        SemanticResult {
            cache_key: id.to_string(),
            content: format!("Test content for {}", id),
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
            cache_info: Some(CacheInfo {
                is_cached: true,
                cache_tier: CacheTier::Memory,
                access_count: 10,
                last_accessed: created_at,
                ttl_remaining: Duration::from_secs(3600),
            }),
        }
    }

    pub fn create_test_access_history(content_id: &str, days_back: usize) -> Vec<ContentAccess> {
        let mut history = Vec::new();
        let now = Utc::now();

        for i in 0..days_back {
            let access_time = now - ChronoDuration::days(i as i64);
            history.push(ContentAccess {
                content_id: content_id.to_string(),
                access_time,
                access_type: AccessType::Read,
                user_id: Some("test_user".to_string()),
                session_id: Some("test_session".to_string()),
                relevance_score: Some(0.8),
            });
        }

        history
    }
}

#[tokio::test]
async fn test_temporal_context_manager_initialization() {
    let config = TemporalConfig::default();
    let manager = TemporalContextManager::new(config).await.unwrap();

    assert!(manager.config().enabled);
    assert!(!manager.config().decay_functions.is_empty());
}

#[tokio::test]
async fn test_temporal_relevance_calculation() {
    let manager = TemporalContextManager::new(TemporalConfig::default()).await.unwrap();
    
    let recent_content = test_utils::create_test_content("recent", ContentType::Code, 1);
    let old_content = test_utils::create_test_content("old", ContentType::Documentation, 365);

    let query_time = Utc::now();
    let user_timezone = Some("America/New_York");

    let recent_relevance = manager
        .calculate_temporal_relevance(&recent_content, query_time, user_timezone)
        .await
        .unwrap();

    let old_relevance = manager
        .calculate_temporal_relevance(&old_content, query_time, user_timezone)
        .await
        .unwrap();

    // Recent content should have higher temporal relevance
    assert!(recent_relevance > old_relevance);
    assert!(recent_relevance >= 0.0 && recent_relevance <= 1.0);
    assert!(old_relevance >= 0.0 && old_relevance <= 1.0);
}

#[tokio::test]
async fn test_content_type_specific_decay() {
    let manager = TemporalContextManager::new(TemporalConfig::default()).await.unwrap();
    
    let query_time = Utc::now();
    let user_timezone = Some("America/New_York");

    // Test different content types with same age
    let code_content = test_utils::create_test_content("code", ContentType::Code, 30);
    let docs_content = test_utils::create_test_content("docs", ContentType::Documentation, 30);
    let decision_content = test_utils::create_test_content("decision", ContentType::Decision, 30);

    let code_relevance = manager
        .calculate_temporal_relevance(&code_content, query_time, user_timezone)
        .await
        .unwrap();

    let docs_relevance = manager
        .calculate_temporal_relevance(&docs_content, query_time, user_timezone)
        .await
        .unwrap();

    let decision_relevance = manager
        .calculate_temporal_relevance(&decision_content, query_time, user_timezone)
        .await
        .unwrap();

    // Documentation should decay slower than code
    assert!(docs_relevance > code_relevance);
    
    // All relevance scores should be valid
    assert!(code_relevance >= 0.0 && code_relevance <= 1.0);
    assert!(docs_relevance >= 0.0 && docs_relevance <= 1.0);
    assert!(decision_relevance >= 0.0 && decision_relevance <= 1.0);
}

#[tokio::test]
async fn test_seasonal_pattern_detection() {
    let detector = SeasonalPatternDetector::new();
    
    // Create access history with a clear weekly pattern
    let mut access_history = Vec::new();
    let now = Utc::now();

    for i in 0..30 {
        let access_time = now - ChronoDuration::days(i);
        let weekday = access_time.weekday().num_days_from_sunday();
        
        // More access on Mondays (weekday = 1)
        let access_count = if weekday == 1 { 3 } else { 1 };
        
        for _ in 0..access_count {
            access_history.push(ContentAccess {
                content_id: "test_content".to_string(),
                access_time,
                access_type: AccessType::Read,
                user_id: Some("test_user".to_string()),
                session_id: Some("test_session".to_string()),
                relevance_score: Some(0.8),
            });
        }
    }

    let patterns = detector.detect_seasonal_patterns(&access_history).await.unwrap();
    
    // Should detect at least one pattern
    assert!(!patterns.is_empty());
    
    // Check for weekly pattern
    let weekly_pattern = patterns.iter().find(|p| {
        matches!(p.pattern_type, SeasonalPeriod::Weekly { .. })
    });
    
    assert!(weekly_pattern.is_some());
    
    if let Some(pattern) = weekly_pattern {
        assert!(pattern.confidence > 0.5);
        assert!(pattern.strength > 0.0);
    }
}

#[tokio::test]
async fn test_timezone_awareness() {
    let manager = TimezoneAwareContextManager::new();
    
    let content = test_utils::create_test_content("test", ContentType::Code, 1);
    let query_time = Utc::now();

    // Test different timezones
    let timezones = vec!["America/New_York", "Europe/London", "Asia/Tokyo"];
    
    for timezone in timezones {
        let adjustment = manager
            .calculate_timezone_adjustment(&content, query_time, timezone)
            .await
            .unwrap();

        assert!(adjustment > 0.0 && adjustment <= 1.5);
    }

    // Test collaborative adjustment
    let team_timezones = vec![
        "America/New_York".to_string(),
        "Europe/London".to_string(),
        "Asia/Tokyo".to_string(),
    ];

    let collaborative_adjustment = manager
        .calculate_collaborative_adjustment(&content, query_time, "America/New_York", &team_timezones)
        .await
        .unwrap();

    assert!(collaborative_adjustment > 0.0 && collaborative_adjustment <= 1.5);
}

#[tokio::test]
async fn test_temporal_relationship_detection() {
    let detector = TemporalRelationshipDetector::new();
    
    let source_content = test_utils::create_test_content("source", ContentType::Code, 10);
    let target_content = test_utils::create_test_content("target", ContentType::Code, 9); // 1 day later

    let relationships = detector
        .detect_temporal_relationships(&source_content, &[target_content.clone()])
        .await
        .unwrap();

    // Should detect at least one relationship
    assert!(!relationships.is_empty());
    
    let relationship = &relationships[0];
    assert_eq!(relationship.target_content_id, target_content.id);
    assert!(relationship.confidence > 0.0 && relationship.confidence <= 1.0);
    assert!(relationship.relevance_score > 0.0 && relationship.relevance_score <= 1.0);
}

#[tokio::test]
async fn test_temporal_search_enhancement() {
    let enhancer = TemporalSearchEnhancer::new();
    
    let search_results = vec![
        test_utils::create_test_semantic_result("recent", 1),
        test_utils::create_test_semantic_result("old", 100),
    ];

    let temporal_query = TemporalSearchQuery::new("test query".to_string())
        .with_freshness_preference(FreshnessPreference::PreferRecent { weight: 0.3 })
        .with_max_results(5);

    let enhanced_results = enhancer
        .enhance_with_temporal_context(&search_results, &temporal_query)
        .await
        .unwrap();

    assert_eq!(enhanced_results.len(), 2);
    
    // Recent content should have higher final score
    assert!(enhanced_results[0].final_score > enhanced_results[1].final_score);
    
    for result in &enhanced_results {
        assert!(result.final_score >= 0.0 && result.final_score <= 1.0);
        assert!(result.temporal_score >= 0.0 && result.temporal_score <= 1.0);
    }
}

#[tokio::test]
async fn test_temporal_filters() {
    // Test filter creation
    let recent_filters = TemporalFilterUtils::recent_content();
    let established_filters = TemporalFilterUtils::established_content();
    let seasonal_filters = TemporalFilterUtils::yearly_seasonal_content(12, 25);

    assert!(!recent_filters.is_empty());
    assert!(!established_filters.is_empty());
    assert!(!seasonal_filters.is_empty());

    // Test filter validation
    TemporalFilterValidator::validate_filters(&recent_filters).unwrap();
    TemporalFilterValidator::validate_filters(&established_filters).unwrap();
    TemporalFilterValidator::validate_filters(&seasonal_filters).unwrap();

    // Test custom filter builder
    let custom_filters = TemporalFilterBuilder::new()
        .created_within_days(7)
        .recently_active(5, Duration::from_secs(7 * 24 * 3600))
        .build();

    assert_eq!(custom_filters.len(), 2);
    TemporalFilterValidator::validate_filters(&custom_filters).unwrap();
}

#[tokio::test]
async fn test_decay_functions() {
    let calculator = DecayCalculator::new();
    let now = Utc::now();
    let created_at = now - ChronoDuration::days(30);

    // Test different content types
    let content_types = vec![
        ContentType::Code,
        ContentType::Documentation,
        ContentType::Decision,
        ContentType::Knowledge,
    ];

    for content_type in content_types {
        let decay = calculator
            .calculate_decay(&content_type, &created_at, &now, None)
            .unwrap();

        assert!(decay > 0.0 && decay <= 1.0);
    }
}

#[tokio::test]
async fn test_temporal_relevance_engine() {
    let engine = TemporalRelevanceEngine::new();
    
    let content = test_utils::create_test_content("test", ContentType::Code, 30);
    let query_time = Utc::now();

    let relevance = engine
        .calculate_temporal_relevance(&content, query_time, None)
        .await
        .unwrap();

    assert!(relevance >= 0.0 && relevance <= 1.0);
}

#[tokio::test]
async fn test_timezone_context_creation() {
    let timezone = "America/New_York";
    let query_time = Utc::now();

    let context = TimezoneContext::new(timezone, query_time).unwrap();

    assert_eq!(context.user_timezone, timezone);
    assert!(context.business_hours_start == 9);
    assert!(context.business_hours_end == 17);
}

#[tokio::test]
async fn test_temporal_search_query_builder() {
    let query = TemporalSearchQuery::new("test query".to_string())
        .with_time_range(TimeRange::last_n_days(7))
        .with_freshness_preference(FreshnessPreference::PreferRecent { weight: 0.5 })
        .with_max_results(10);

    assert_eq!(query.base_query, "test query");
    assert!(query.time_range.is_some());
    assert_eq!(query.max_results, 10);
}

#[tokio::test]
async fn test_temporal_filter_evaluation() {
    let enhancer = TemporalSearchEnhancer::new();
    let result = test_utils::create_test_semantic_result("test", 5);

    // Test age filter
    let age_filter = TemporalFilter::AgeLessThan(Duration::from_secs(7 * 24 * 3600)); // 7 days
    let passes_filter = enhancer.evaluate_temporal_filter(&result, &age_filter).await.unwrap();
    assert!(passes_filter);

    // Test older content
    let old_result = test_utils::create_test_semantic_result("old", 100);
    let passes_old_filter = enhancer.evaluate_temporal_filter(&old_result, &age_filter).await.unwrap();
    assert!(!passes_old_filter);
}

#[tokio::test]
async fn test_freshness_preferences() {
    let enhancer = TemporalSearchEnhancer::new();
    let recent_result = test_utils::create_test_semantic_result("recent", 1);
    let old_result = test_utils::create_test_semantic_result("old", 100);

    let temporal_query = TemporalSearchQuery::new("test".to_string())
        .with_freshness_preference(FreshnessPreference::PreferRecent { weight: 0.3 });

    let recent_adjustment = enhancer
        .apply_freshness_preferences(&recent_result, &temporal_query)
        .await
        .unwrap();

    let old_adjustment = enhancer
        .apply_freshness_preferences(&old_result, &temporal_query)
        .await
        .unwrap();

    // Recent content should get a higher adjustment
    assert!(recent_adjustment > old_adjustment);
}

#[tokio::test]
async fn test_seasonal_preferences() {
    let enhancer = TemporalSearchEnhancer::new();
    let result = test_utils::create_test_semantic_result("test", 30);

    let temporal_query = TemporalSearchQuery::new("test".to_string())
        .with_seasonal_preference(SeasonalPreference {
            seasonal_period: SeasonalPeriod::Yearly { month: 12, day: 25 },
            weight: 0.5,
            tolerance_days: 7,
        });

    let adjustment = enhancer
        .apply_seasonal_preferences(&result, &temporal_query)
        .await
        .unwrap();

    assert!(adjustment >= 1.0); // Should be at least neutral
}

#[tokio::test]
async fn test_timezone_adjustments() {
    let enhancer = TemporalSearchEnhancer::new();
    let result = test_utils::create_test_semantic_result("test", 30);

    let timezone_context = TimezoneContext::new("America/New_York", Utc::now()).unwrap();
    let temporal_query = TemporalSearchQuery::new("test".to_string())
        .with_timezone_context(timezone_context);

    let adjustment = enhancer
        .apply_timezone_adjustments(&result, &temporal_query)
        .await
        .unwrap();

    assert!(adjustment > 0.0 && adjustment <= 1.5);
}

#[tokio::test]
async fn test_relationship_score_calculation() {
    let enhancer = TemporalSearchEnhancer::new();
    let results = vec![
        test_utils::create_test_semantic_result("result1", 10),
        test_utils::create_test_semantic_result("result2", 9),
    ];

    let relationship_score = enhancer
        .calculate_relationship_score(&results[0], &results)
        .await
        .unwrap();

    assert!(relationship_score >= 0.0 && relationship_score <= 1.0);
}

#[tokio::test]
async fn test_temporal_config_serialization() {
    let config = TemporalConfig::default();
    
    // Test serialization
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: TemporalConfig = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.decay_functions.len(), deserialized.decay_functions.len());
}

#[tokio::test]
async fn test_temporal_error_handling() {
    // Test invalid timezone
    let invalid_timezone = "Invalid/Timezone";
    let query_time = Utc::now();
    
    let result = TimezoneContext::new(invalid_timezone, query_time);
    assert!(result.is_err());

    // Test invalid filter
    let invalid_filter = TemporalFilter::AgeLessThan(Duration::from_secs(0));
    let result = TemporalFilterValidator::validate_single_filter(&invalid_filter);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_performance_benchmarks() {
    let manager = TemporalContextManager::new(TemporalConfig::default()).await.unwrap();
    let content = test_utils::create_test_content("test", ContentType::Code, 30);
    let query_time = Utc::now();

    // Benchmark temporal relevance calculation
    let start = std::time::Instant::now();
    
    for _ in 0..100 {
        let _relevance = manager
            .calculate_temporal_relevance(&content, query_time, None)
            .await
            .unwrap();
    }
    
    let duration = start.elapsed();
    
    // Should complete 100 calculations in under 1 second
    assert!(duration.as_millis() < 1000);
}

#[tokio::test]
async fn test_integration_scenario() {
    // Test a complete integration scenario
    let config = TemporalConfig::default();
    let manager = TemporalContextManager::new(config).await.unwrap();
    
    let content = vec![
        test_utils::create_test_content("recent_code", ContentType::Code, 1),
        test_utils::create_test_content("old_docs", ContentType::Documentation, 365),
        test_utils::create_test_content("decision", ContentType::Decision, 7),
    ];

    let query_time = Utc::now();
    let user_timezone = Some("America/New_York");

    // Calculate temporal relevance for all content
    let mut relevances = Vec::new();
    for item in &content {
        let relevance = manager
            .calculate_temporal_relevance(item, query_time, user_timezone)
            .await
            .unwrap();
        relevances.push((item.id.clone(), relevance));
    }

    // Sort by relevance
    relevances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Recent code should have highest relevance
    assert_eq!(relevances[0].0, "recent_code");
    
    // Old docs should have lowest relevance
    assert_eq!(relevances[2].0, "old_docs");
    
    // All relevances should be valid
    for (_, relevance) in &relevances {
        assert!(*relevance >= 0.0 && *relevance <= 1.0);
    }
}
