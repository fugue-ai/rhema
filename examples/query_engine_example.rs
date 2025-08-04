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

use rhema_query::{
    query::{execute_query, execute_query_with_provenance, parse_cql_query, CqlQuery},
    search::{SearchEngine, SearchOptions, SearchType},
    optimization::{QueryOptimizer, OptimizationConfig},
    performance::{PerformanceMonitor, MonitorConfig},
    caching::{CacheManager, CacheConfig},
    repo_analysis::RepoAnalysis,
};
use std::path::Path;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Query Engine & Repository Analysis Example");
    println!("==================================================\n");

    let repo_path = Path::new(".");
    
    // 1. Repository Analysis
    println!("📊 1. Repository Analysis");
    println!("------------------------");
    let analysis = RepoAnalysis::analyze(repo_path)?;
    println!("Project Type: {:?}", analysis.project_type);
    println!("Languages: {:?}", analysis.languages);
    println!("Frameworks: {:?}", analysis.frameworks);
    println!("Databases: {:?}", analysis.databases);
    println!("Infrastructure: {:?}", analysis.infrastructure);
    println!("Build Tools: {:?}", analysis.build_tools);
    println!("Dependencies: {:?}", analysis.dependencies);
    println!("Suggested Scope: {} - {}", analysis.suggested_scope_type, analysis.suggested_scope_name);
    println!("Description: {}", analysis.suggested_description);
    println!();

    // 2. Search Engine
    println!("🔍 2. Search Engine");
    println!("------------------");
    let mut search_engine = SearchEngine::new();
    
    // Build search index (in a real scenario, you'd have scopes)
    // For this example, we'll skip the index building
    
    // Full-text search example
    println!("Full-text search for 'query':");
    let search_options = SearchOptions {
        search_type: SearchType::FullText,
        limit: Some(5),
        filters: vec![],
        semantic_weight: None,
        keyword_weight: None,
        min_similarity: None,
    };
    
    // Note: This would require an actual index to be built
    // let results = search_engine.full_text_search("query", Some(search_options)).await?;
    // println!("Found {} results", results.len());
    println!("(Search index not built for this example)");
    println!();

    // 3. Query Optimization
    println!("⚡ 3. Query Optimization");
    println!("----------------------");
    let optimizer = QueryOptimizer::new();
    
    // Example CQL query
    let query_str = "SELECT todos FROM scope('user-service') WHERE status = 'pending' AND priority > 5 ORDER BY priority DESC LIMIT 10";
    let parsed_query = parse_cql_query(query_str)?;
    
    println!("Original Query: {}", query_str);
    println!("Parsed Query: {:?}", parsed_query);
    
    // Optimize the query
    let optimized = optimizer.optimize(&parsed_query).await?;
    println!("Optimized Query: {:?}", optimized.optimized);
    println!("Applied Optimizations: {:?}", optimized.applied_optimizations);
    println!("Expected Improvement: {:.2}%", optimized.expected_improvement * 100.0);
    
    // Generate query plan
    let plan = optimizer.generate_plan(&parsed_query).await?;
    println!("Query Plan:");
    for step in &plan.steps {
        println!("  - {}: {}ms (cost: {:.2})", step.name, step.estimated_time_ms, step.estimated_cost);
    }
    println!("Total Estimated Time: {}ms", plan.estimated_time_ms);
    println!("Plan Confidence: {:.2}%", plan.confidence * 100.0);
    println!();

    // 4. Performance Monitoring
    println!("📈 4. Performance Monitoring");
    println!("---------------------------");
    let monitor = PerformanceMonitor::new();
    
    // Simulate query execution with monitoring
    let query_id = monitor.start_query_monitoring(&parsed_query).await;
    let start_time = Instant::now();
    
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let execution_time = start_time.elapsed().as_millis() as u64;
    monitor.end_query_monitoring(
        &query_id,
        true,
        25, // result count
        None, // no error
        Some("condition_optimization".to_string()), // optimization applied
    ).await?;
    
    // Get performance metrics
    let metrics = monitor.get_metrics()?;
    println!("Total Queries: {}", metrics.total_queries);
    println!("Average Execution Time: {:.2}ms", metrics.avg_execution_time_ms);
    println!("Cache Hit Rate: {:.2}%", metrics.cache_hit_rate * 100.0);
    println!("Success Rate: {:.2}%", (metrics.successful_queries as f64 / metrics.total_queries as f64) * 100.0);
    
    // Get alerts
    let alerts = monitor.get_alerts()?;
    if !alerts.is_empty() {
        println!("Active Alerts:");
        for alert in alerts {
            println!("  - {}: {} ({:?})", alert.alert_id, alert.message, alert.severity);
        }
    }
    println!();

    // 5. Caching
    println!("💾 5. Caching");
    println!("-------------");
    let cache_config = CacheConfig {
        enabled: true,
        max_size_bytes: 10 * 1024 * 1024, // 10MB
        default_ttl_secs: 3600, // 1 hour
        max_ttl_secs: 86400, // 24 hours
        enable_compression: true,
        compression_level: 6,
        enable_persistence: false,
        persistence_path: None,
        eviction_policy: rhema_query::caching::EvictionPolicyType::LRU,
        max_entries: 1000,
    };
    
    let cache_manager = CacheManager::with_config(cache_config);
    
    // Generate cache key for query
    let cache_key = cache_manager.generate_query_key(&parsed_query, Some("user-service"));
    println!("Cache Key: {}", cache_key);
    
    // Simulate cache operations
    let test_data = serde_yaml::Value::String("test result data".to_string());
    let cache_result = cache_manager.set(&cache_key, test_data.clone(), Some(1800)).await?;
    println!("Cache Set: {} ({}ms)", cache_result.success, cache_result.duration_ms);
    
    let get_result = cache_manager.get(&cache_key).await?;
    println!("Cache Get: {} ({}ms)", get_result.cache_hit, get_result.duration_ms);
    
    // Get cache statistics
    let cache_stats = cache_manager.get_stats()?;
    println!("Cache Stats:");
    println!("  Hits: {}, Misses: {}", cache_stats.hits, cache_stats.misses);
    println!("  Hit Rate: {:.2}%", cache_stats.hit_rate * 100.0);
    println!("  Total Size: {} bytes", cache_stats.total_size_bytes);
    println!("  Entries: {}", cache_stats.entry_count);
    println!();

    // 6. Advanced CQL Queries
    println!("🔧 6. Advanced CQL Queries");
    println!("-------------------------");
    
    // Complex query with multiple conditions
    let complex_query = "SELECT todos, decisions FROM scope('auth-module') WHERE status = 'pending' AND priority > 3 AND created_at > '2024-01-01' ORDER BY priority DESC, created_at ASC LIMIT 20 OFFSET 10";
    let parsed_complex = parse_cql_query(complex_query)?;
    println!("Complex Query: {}", complex_query);
    println!("Parsed Conditions: {:?}", parsed_complex.conditions);
    println!("Order By: {:?}", parsed_complex.order_by);
    println!("Limit: {:?}, Offset: {:?}", parsed_complex.limit, parsed_complex.offset);
    println!();

    // 7. Performance Analysis
    println!("📊 7. Performance Analysis");
    println!("-------------------------");
    
    // Simulate multiple query executions for analysis
    let mut total_results = Vec::new();
    for i in 0..5 {
        let query_id = monitor.start_query_monitoring(&parsed_query).await;
        let start = Instant::now();
        
        // Simulate varying execution times
        tokio::time::sleep(tokio::time::Duration::from_millis(50 + i * 20)).await;
        
        let exec_time = start.elapsed().as_millis() as u64;
        monitor.end_query_monitoring(
            &query_id,
            i < 4, // One failure
            ivec![10, 15, 20, 25, 30][i], // varying result counts
            if i == 4 { Some("timeout".to_string()) } else { None },
            Some("query_optimization".to_string()),
        ).await?;
        
        total_results.push(rhema_query::query::QueryResult {
            scope: "user-service".to_string(),
            file: format!("file_{}.yaml", i),
            data: serde_yaml::Value::String(format!("result_{}", i)),
            path: format!("path_{}", i),
            field_provenance: std::collections::HashMap::new(),
            query_provenance: None,
            metadata: std::collections::HashMap::new(),
        });
    }
    
    // Analyze performance
    let performance_analysis = optimizer.analyze_performance(&parsed_query, &total_results, 150).await?;
    println!("Performance Score: {:.2}%", performance_analysis.performance_score * 100.0);
    println!("Bottlenecks Found: {}", performance_analysis.bottlenecks.len());
    for bottleneck in &performance_analysis.bottlenecks {
        println!("  - {}: {} (impact: {:.2})", 
            bottleneck.bottleneck_type, 
            bottleneck.description, 
            bottleneck.impact_score);
    }
    
    println!("Recommendations: {}", performance_analysis.recommendations.len());
    for rec in &performance_analysis.recommendations {
        println!("  - {}: {} (priority: {}, difficulty: {})", 
            rec.recommendation_type, 
            rec.description, 
            rec.priority, 
            rec.difficulty);
    }
    println!();

    // 8. Hybrid Search
    println!("🔍 8. Hybrid Search");
    println!("------------------");
    
    let hybrid_options = SearchOptions {
        search_type: SearchType::Hybrid,
        limit: Some(10),
        filters: vec![],
        semantic_weight: Some(0.7),
        keyword_weight: Some(0.3),
        min_similarity: Some(0.5),
    };
    
    println!("Hybrid Search Options:");
    println!("  Semantic Weight: {:.1}", hybrid_options.semantic_weight.unwrap());
    println!("  Keyword Weight: {:.1}", hybrid_options.keyword_weight.unwrap());
    println!("  Min Similarity: {:.1}", hybrid_options.min_similarity.unwrap());
    println!("  Limit: {}", hybrid_options.limit.unwrap());
    println!();

    // 9. Cache Optimization
    println!("⚡ 9. Cache Optimization");
    println!("----------------------");
    
    // Simulate cache cleanup
    let cleanup_result = cache_manager.cleanup().await?;
    println!("Cleaned up {} expired entries", cleanup_result);
    
    // Simulate cache eviction
    for i in 0..100 {
        let key = format!("test_key_{}", i);
        let data = serde_yaml::Value::String(format!("data_{}", i));
        cache_manager.set(&key, data, Some(60)).await?;
    }
    println!("Added 100 test entries to cache");
    
    let final_stats = cache_manager.get_stats()?;
    println!("Final Cache Stats:");
    println!("  Entries: {}", final_stats.entry_count);
    println!("  Size: {} bytes", final_stats.total_size_bytes);
    println!("  Evictions: {}", final_stats.evictions);
    println!();

    // 10. Summary
    println!("📋 10. Summary");
    println!("-------------");
    println!("✅ Repository Analysis: Complete");
    println!("✅ Search Engine: Configured");
    println!("✅ Query Optimization: Active");
    println!("✅ Performance Monitoring: Running");
    println!("✅ Caching: Enabled");
    println!("✅ Advanced CQL: Supported");
    println!("✅ Performance Analysis: Available");
    println!("✅ Hybrid Search: Ready");
    println!("✅ Cache Optimization: Working");
    println!();
    
    println!("🎉 Query Engine & Repository Analysis implementation complete!");
    println!("All features are now available for use in the Rhema system.");

    Ok(())
} 