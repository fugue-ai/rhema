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

use chrono::{Utc, Duration as ChronoDuration, Datelike};
use std::collections::HashMap;
use std::time::Duration;

use rhema_knowledge::temporal::*;
use rhema_knowledge::types::ContentType;

/// Comprehensive example demonstrating Temporal Context Awareness
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Temporal Context Awareness Example");
    println!("============================================\n");

    // Phase 1: Initialize Temporal Context Manager
    println!("📋 Phase 1: Initializing Temporal Context Manager");
    let temporal_config = create_temporal_config();
    let temporal_manager = TemporalContextManager::new(temporal_config).await?;
    println!("✅ Temporal context manager initialized\n");

    // Phase 2: Create Sample Content
    println!("📋 Phase 2: Creating Sample Content");
    let sample_content = create_sample_content();
    println!("✅ Created {} sample content items\n", sample_content.len());

    // Phase 3: Demonstrate Temporal Relevance Calculation
    println!("📋 Phase 3: Temporal Relevance Calculation");
    demonstrate_temporal_relevance(&temporal_manager, &sample_content).await?;
    println!();

    // Phase 4: Demonstrate Seasonal Pattern Detection
    println!("📋 Phase 4: Seasonal Pattern Detection");
    demonstrate_seasonal_patterns(&temporal_manager, &sample_content).await?;
    println!();

    // Phase 5: Demonstrate Timezone Awareness
    println!("📋 Phase 5: Timezone Awareness");
    demonstrate_timezone_awareness(&temporal_manager, &sample_content).await?;
    println!();

    // Phase 6: Demonstrate Temporal Relationships
    println!("📋 Phase 6: Temporal Relationship Detection");
    demonstrate_temporal_relationships(&temporal_manager, &sample_content).await?;
    println!();

    // Phase 7: Demonstrate Temporal Search Enhancement
    println!("📋 Phase 7: Temporal Search Enhancement");
    demonstrate_temporal_search(&temporal_manager, &sample_content).await?;
    println!();

    // Phase 8: Demonstrate Temporal Filters
    println!("📋 Phase 8: Temporal Filters");
    demonstrate_temporal_filters(&sample_content).await?;
    println!();

    println!("🎉 Temporal Context Awareness Example Complete!");
    Ok(())
}

/// Create a comprehensive temporal configuration
fn create_temporal_config() -> TemporalConfig {
    let mut decay_functions = HashMap::new();
    decay_functions.insert(ContentType::Documentation, DecayFunction::Documentation { half_life_days: 365.0 });
    decay_functions.insert(ContentType::Code, DecayFunction::Code { half_life_hours: 168.0 });
    decay_functions.insert(ContentType::Decision, DecayFunction::Decisions { half_life_weeks: 52.0 });
    decay_functions.insert(ContentType::Knowledge, DecayFunction::Knowledge { 
        adaptive_decay: AdaptiveDecayConfig::default() 
    });
    decay_functions.insert(ContentType::Pattern, DecayFunction::Patterns { 
        stable_period_days: 90.0, 
        update_cycle_days: 30.0 
    });

    TemporalConfig {
        enabled: true,
        decay_functions,
        seasonal_config: SeasonalConfig {
            enabled: true,
            confidence_threshold: 0.7,
            historical_window_days: 365,
        },
        timezone_config: TimezoneConfig {
            enabled: true,
            default_timezone: "UTC".to_string(),
            business_hours_boost: 1.2,
            off_hours_penalty: 0.8,
        },
        relationship_config: RelationshipConfig {
            enabled: true,
            max_relationships: 10,
            confidence_threshold: 0.6,
        },
        search_config: TemporalSearchConfig {
            enabled: true,
            max_enhanced_results: 50,
            temporal_score_threshold: 0.3,
        },
        weights: TemporalWeights {
            base_decay_weight: 0.4,
            seasonal_weight: 0.2,
            timezone_weight: 0.1,
            relationship_weight: 0.2,
            freshness_weight: 0.1,
        },
    }
}

/// Create sample content with various temporal characteristics
fn create_sample_content() -> Vec<Content> {
    let now = Utc::now();
    let mut content = Vec::new();

    // Recent code (1 day old)
    content.push(Content {
        id: "recent_code".to_string(),
        content_type: ContentType::Code,
        created_at: now - ChronoDuration::days(1),
        modified_at: now - ChronoDuration::hours(6),
        accessed_at: now - ChronoDuration::hours(2),
        access_count: 15,
        content: "Recent API implementation".to_string(),
        metadata: HashMap::new(),
    });

    // Old documentation (1 year old)
    content.push(Content {
        id: "old_docs".to_string(),
        content_type: ContentType::Documentation,
        created_at: now - ChronoDuration::days(365),
        modified_at: now - ChronoDuration::days(30),
        accessed_at: now - ChronoDuration::days(5),
        access_count: 150,
        content: "Legacy system documentation".to_string(),
        metadata: HashMap::new(),
    });

    // Recent decision (1 week old)
    content.push(Content {
        id: "recent_decision".to_string(),
        content_type: ContentType::Decision,
        created_at: now - ChronoDuration::days(7),
        modified_at: now - ChronoDuration::days(7),
        accessed_at: now - ChronoDuration::days(1),
        access_count: 8,
        content: "Architecture decision for new feature".to_string(),
        metadata: HashMap::new(),
    });

    // Seasonal content (created in December)
    let december_content = Content {
        id: "seasonal_content".to_string(),
        content_type: ContentType::Knowledge,
        created_at: now.with_month(12).unwrap().with_day(15).unwrap(),
        modified_at: now.with_month(12).unwrap().with_day(15).unwrap(),
        accessed_at: now - ChronoDuration::days(10),
        access_count: 25,
        content: "Year-end review process".to_string(),
        metadata: HashMap::new(),
    };
    content.push(december_content);

    // Frequently accessed knowledge
    content.push(Content {
        id: "frequent_knowledge".to_string(),
        content_type: ContentType::Knowledge,
        created_at: now - ChronoDuration::days(60),
        modified_at: now - ChronoDuration::days(5),
        accessed_at: now - ChronoDuration::hours(1),
        access_count: 200,
        content: "Frequently referenced knowledge base".to_string(),
        metadata: HashMap::new(),
    });

    content
}

/// Demonstrate temporal relevance calculation
async fn demonstrate_temporal_relevance(
    temporal_manager: &TemporalContextManager,
    content: &[Content],
) -> Result<(), Box<dyn std::error::Error>> {
    let query_time = Utc::now();
    let user_timezone = Some("America/New_York");

    println!("  Calculating temporal relevance for {} content items...", content.len());

    for item in content {
        let relevance = temporal_manager
            .calculate_temporal_relevance(item, query_time, user_timezone)
            .await?;

        println!("    📄 {} ({:?}): {:.3}", 
            item.id, 
            item.content_type, 
            relevance
        );
    }

    Ok(())
}

/// Demonstrate seasonal pattern detection
async fn demonstrate_seasonal_patterns(
    _temporal_manager: &TemporalContextManager,
    _content: &[Content],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Creating sample access history for seasonal pattern detection...");

    // Create sample access history with seasonal patterns
    let mut access_history = Vec::new();
    let now = Utc::now();

    // Add weekly pattern (more access on Mondays)
    for i in 0..30 {
        let access_time = now - ChronoDuration::days(i);
        let weekday = access_time.weekday().num_days_from_sunday();
        
        // More access on Mondays (weekday = 1)
        let access_count = if weekday == 1 { 3 } else { 1 };
        
        for _ in 0..access_count {
            access_history.push(ContentAccess {
                content_id: "seasonal_content".to_string(),
                access_time,
                access_type: AccessType::Read,
                user_id: Some("user1".to_string()),
                session_id: Some("session1".to_string()),
                relevance_score: Some(0.8),
            });
        }
    }

    println!("  Detecting seasonal patterns in {} access records...", access_history.len());

    // Note: Seasonal pattern detection would be available through public methods
    // For now, we'll demonstrate the concept without accessing private fields
    println!("    🗓️  Seasonal pattern detection would analyze access patterns");
    println!("    🗓️  Example patterns: Weekly cycles, monthly trends, seasonal variations");

    Ok(())
}

/// Demonstrate timezone awareness
async fn demonstrate_timezone_awareness(
    _temporal_manager: &TemporalContextManager,
    _content: &[Content],
) -> Result<(), Box<dyn std::error::Error>> {
    let _query_time = Utc::now();
    let timezones = vec!["America/New_York", "Europe/London", "Asia/Tokyo"];

    println!("  Testing timezone adjustments for different timezones...");

    // Note: Timezone adjustments would be available through public methods
    // For now, we'll demonstrate the concept without accessing private fields
    for timezone in timezones {
        println!("    🌍 {}: Timezone adjustment would be calculated", timezone);
    }

    // Test collaborative timezone adjustment
    let team_timezones = vec![
        "America/New_York".to_string(),
        "Europe/London".to_string(),
        "Asia/Tokyo".to_string(),
    ];

    println!("    👥 Collaborative adjustment: Would be calculated for team across {} timezones", team_timezones.len());

    Ok(())
}

/// Demonstrate temporal relationship detection
async fn demonstrate_temporal_relationships(
    temporal_manager: &TemporalContextManager,
    content: &[Content],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Detecting temporal relationships between content items...");

    let source_content = &content[0]; // Recent code
    let target_contents = &content[1..]; // Other content

    let relationships = temporal_manager
        .detect_temporal_relationships(source_content, target_contents)
        .await?;

    println!("  Found {} temporal relationships:", relationships.len());

    for relationship in &relationships {
        println!("    🔗 {} -> {}: {:?} (confidence: {:.3})", 
            source_content.id,
            relationship.target_content_id,
            relationship.relationship_type,
            relationship.confidence
        );
    }

    Ok(())
}

/// Demonstrate temporal search enhancement
async fn demonstrate_temporal_search(
    temporal_manager: &TemporalContextManager,
    content: &[Content],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Creating sample search results...");

    // Convert content to semantic results
    let search_results: Vec<SemanticResult> = content.iter().map(|c| SemanticResult {
        cache_key: c.id.clone(),
        content: c.content.clone(),
        embedding: vec![0.1, 0.2, 0.3],
        relevance_score: 0.8,
        semantic_tags: vec!["test".to_string()],
        metadata: SearchResultMetadata {
            source_type: c.content_type.clone(),
            scope_path: None,
            created_at: c.created_at,
            last_modified: c.modified_at,
            size_bytes: 1024,
            chunk_id: None,
        },
        cache_info: Some(CacheInfo {
            is_cached: true,
            cache_tier: CacheTier::Memory,
            access_count: c.access_count,
            last_accessed: c.accessed_at,
            ttl_remaining: Duration::from_secs(3600),
        }),
    }).collect();

    // Create temporal search query
    let temporal_query = TemporalSearchQuery::new("API documentation".to_string())
        .with_freshness_preference(FreshnessPreference::PreferRecent { weight: 0.3 })
        .with_filter(TemporalFilter::AgeLessThan(Duration::from_secs(30 * 24 * 3600))) // Last 30 days
        .with_max_results(5);

    println!("  Enhancing search results with temporal context...");

    let enhanced_results = temporal_manager
        .enhance_search_results(&search_results, &temporal_query)
        .await?;

    println!("  Enhanced {} search results:", enhanced_results.len());

    for (i, result) in enhanced_results.iter().enumerate() {
        println!("    {}. {} (final score: {:.3}, temporal score: {:.3})", 
            i + 1,
            result.base_result.cache_key,
            result.final_score,
            result.temporal_score
        );
    }

    Ok(())
}

/// Demonstrate temporal filters
async fn demonstrate_temporal_filters(_content: &[Content]) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Demonstrating temporal filter utilities...");

    // Create various temporal filters
    let recent_filters = TemporalFilterUtils::recent_content();
    let established_filters = TemporalFilterUtils::established_content();
    let seasonal_filters = TemporalFilterUtils::yearly_seasonal_content(12, 25); // Christmas
    let time_range_filters = TemporalFilterUtils::time_range_content(
        Utc::now() - ChronoDuration::days(30),
        Utc::now()
    );

    println!("    📅 Recent content filters: {} filters", recent_filters.len());
    println!("    📅 Established content filters: {} filters", established_filters.len());
    println!("    📅 Seasonal content filters: {} filters", seasonal_filters.len());
    println!("    📅 Time range filters: {} filters", time_range_filters.len());

    // Validate filters
    println!("  Validating temporal filters...");
    TemporalFilterValidator::validate_filters(&recent_filters)?;
    TemporalFilterValidator::validate_filters(&established_filters)?;
    TemporalFilterValidator::validate_filters(&seasonal_filters)?;
    TemporalFilterValidator::validate_filters(&time_range_filters)?;
    println!("    ✅ All filters are valid");

    // Build custom filters
    let custom_filters = TemporalFilterBuilder::new()
        .created_within_days(7)
        .recently_active(5, Duration::from_secs(7 * 24 * 3600))
        .seasonal_content(SeasonalPeriod::Weekly { weekday: 1 }) // Mondays
        .build();

    println!("    🔧 Custom filters: {} filters", custom_filters.len());

    Ok(())
}
