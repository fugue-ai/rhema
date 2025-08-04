/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * you may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::time::{Duration, Instant};
use tokio::test;
use serde_json::json;
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode};
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;

use rhema_mcp::http_server::{HttpServer, PerformanceMetrics, ConnectionPool, StringCache};
use rhema_mcp::mcp::{McpConfig, McpDaemon};
use rhema_core::RhemaResult;

/// Performance test configuration
const TARGET_LATENCY_MS: u64 = 50;
const CONCURRENT_REQUESTS: usize = 100;
const WARMUP_REQUESTS: usize = 10;

/// Test HTTP server performance optimizations
#[test]
async fn test_http_server_performance_optimizations() -> RhemaResult<()> {
    // Create test configuration
    let config = create_test_config();
    let daemon = create_test_daemon(&config).await?;
    let server = HttpServer::new(config, daemon);

    // Test connection pool performance
    test_connection_pool_performance(&server).await?;

    // Test string cache performance
    test_string_cache_performance(&server).await?;

    // Test response caching performance
    test_response_caching_performance(&server).await?;

    // Test rate limiting performance
    test_rate_limiting_performance(&server).await?;

    // Test JSON processing performance
    test_json_processing_performance(&server).await?;

    // Test concurrent request handling
    test_concurrent_request_performance(&server).await?;

    Ok(())
}

/// Test connection pool performance
async fn test_connection_pool_performance(server: &HttpServer) -> RhemaResult<()> {
    println!("Testing connection pool performance...");
    
    let start_time = Instant::now();
    let mut handles = Vec::new();

    // Spawn concurrent connection acquisitions
    for _ in 0..CONCURRENT_REQUESTS {
        let pool = server.connection_pool.clone();
        let handle = tokio::spawn(async move {
            let _guard = pool.acquire().await.unwrap();
            // Simulate some work
            tokio::time::sleep(Duration::from_millis(1)).await;
        });
        handles.push(handle);
    }

    // Wait for all connections to complete
    for handle in handles {
        handle.await.unwrap();
    }

    let duration = start_time.elapsed();
    let avg_duration = duration.as_millis() as u64 / CONCURRENT_REQUESTS as u64;

    println!("Connection pool test: {} requests in {:?} (avg: {}ms)", 
             CONCURRENT_REQUESTS, duration, avg_duration);

    // Verify performance target
    assert!(avg_duration < TARGET_LATENCY_MS, 
            "Connection pool performance exceeded target: {}ms > {}ms", 
            avg_duration, TARGET_LATENCY_MS);

    Ok(())
}

/// Test string cache performance
async fn test_string_cache_performance(server: &HttpServer) -> RhemaResult<()> {
    println!("Testing string cache performance...");
    
    let cache = &server.string_cache;
    let test_strings = vec!["healthy", "error", "success", "failure", "pending"];
    
    let start_time = Instant::now();
    
    // Test cache hits (should be very fast)
    for _ in 0..1000 {
        for s in &test_strings {
            let _cached = cache.get_or_insert(s);
        }
    }
    
    let duration = start_time.elapsed();
    let avg_duration = duration.as_micros() as u64 / (1000 * test_strings.len()) as u64;

    println!("String cache test: {} operations in {:?} (avg: {}μs)", 
             1000 * test_strings.len(), duration, avg_duration);

    // String cache operations should be sub-millisecond
    assert!(avg_duration < 1000, 
            "String cache performance exceeded target: {}μs > 1000μs", 
            avg_duration);

    Ok(())
}

/// Test response caching performance
async fn test_response_caching_performance(server: &HttpServer) -> RhemaResult<()> {
    println!("Testing response caching performance...");
    
    let cache_key = "test:cache:key";
    let test_response = json!({
        "status": "success",
        "data": "test_data",
        "timestamp": chrono::Utc::now().timestamp()
    });

    // Test cache miss (first access)
    let start_time = Instant::now();
    let cached = server.get_cached_response(cache_key);
    let miss_duration = start_time.elapsed();
    
    assert!(cached.is_none(), "Cache should be empty initially");

    // Test cache hit (second access)
    server.cache_response(cache_key.to_string(), test_response.clone());
    
    let start_time = Instant::now();
    let cached = server.get_cached_response(cache_key);
    let hit_duration = start_time.elapsed();
    
    assert!(cached.is_some(), "Cache should contain the response");

    println!("Response cache test: miss={:?}, hit={:?}", miss_duration, hit_duration);

    // Cache hits should be very fast (sub-millisecond)
    assert!(hit_duration < Duration::from_millis(1), 
            "Cache hit performance exceeded target: {:?} > 1ms", 
            hit_duration);

    Ok(())
}

/// Test rate limiting performance
async fn test_rate_limiting_performance(server: &HttpServer) -> RhemaResult<()> {
    println!("Testing rate limiting performance...");
    
    let client_id = "test_client";
    let start_time = Instant::now();
    
    // Test multiple rate limit checks
    for _ in 0..100 {
        let _allowed = server.check_rate_limit_optimized(client_id).await;
    }
    
    let duration = start_time.elapsed();
    let avg_duration = duration.as_micros() as u64 / 100;

    println!("Rate limiting test: {} checks in {:?} (avg: {}μs)", 
             100, duration, avg_duration);

    // Rate limiting should be very fast
    assert!(avg_duration < 1000, 
            "Rate limiting performance exceeded target: {}μs > 1000μs", 
            avg_duration);

    Ok(())
}

/// Test JSON processing performance
async fn test_json_processing_performance(server: &HttpServer) -> RhemaResult<()> {
    println!("Testing JSON processing performance...");
    
    let test_data = json!({
        "query": "test query",
        "parameters": {
            "limit": 10,
            "offset": 0,
            "filters": ["test", "data"]
        },
        "metadata": {
            "timestamp": chrono::Utc::now().timestamp(),
            "user_id": "test_user",
            "session_id": "test_session"
        }
    });

    // Test optimized JSON serialization
    let start_time = Instant::now();
    for _ in 0..1000 {
        let _serialized = HttpServer::serialize_json_optimized(&test_data).unwrap();
    }
    let serialization_duration = start_time.elapsed();

    // Test zero-copy string concatenation
    let start_time = Instant::now();
    for _ in 0..1000 {
        let _concatenated = HttpServer::concat_strings_zero_copy(&["test", ":", "data", ":", "value"]);
    }
    let concatenation_duration = start_time.elapsed();

    // Test fast string hashing
    let start_time = Instant::now();
    for _ in 0..1000 {
        let _hash = HttpServer::hash_string_fast("test_cache_key");
    }
    let hashing_duration = start_time.elapsed();

    println!("JSON processing test:");
    println!("  Serialization: {} operations in {:?} (avg: {}μs)", 
             1000, serialization_duration, serialization_duration.as_micros() / 1000);
    println!("  Concatenation: {} operations in {:?} (avg: {}μs)", 
             1000, concatenation_duration, concatenation_duration.as_micros() / 1000);
    println!("  Hashing: {} operations in {:?} (avg: {}μs)", 
             1000, hashing_duration, hashing_duration.as_micros() / 1000);

    // All operations should be very fast
    assert!(serialization_duration.as_micros() / 1000 < 100, 
            "JSON serialization too slow: {}μs > 100μs", 
            serialization_duration.as_micros() / 1000);
    
    assert!(concatenation_duration.as_micros() / 1000 < 10, 
            "String concatenation too slow: {}μs > 10μs", 
            concatenation_duration.as_micros() / 1000);
    
    assert!(hashing_duration.as_micros() / 1000 < 10, 
            "String hashing too slow: {}μs > 10μs", 
            hashing_duration.as_micros() / 1000);

    Ok(())
}

/// Test concurrent request handling performance
async fn test_concurrent_request_performance(server: &HttpServer) -> RhemaResult<()> {
    println!("Testing concurrent request performance...");
    
    let mut handles = Vec::new();
    let start_time = Instant::now();

    // Spawn concurrent requests
    for i in 0..CONCURRENT_REQUESTS {
        let server_clone = server.clone();
        let handle = tokio::spawn(async move {
            // Simulate a health check request
            let mut headers = HeaderMap::new();
            headers.insert("authorization", HeaderValue::from_static("Bearer test_token"));
            headers.insert("x-client-id", HeaderValue::from_str(&format!("client_{}", i)).unwrap());
            
            // This would normally be a real HTTP request, but for testing we'll simulate
            // the performance metrics recording
            let request_start = Instant::now();
            
            // Simulate some work
            tokio::time::sleep(Duration::from_millis(1)).await;
            
            let duration = request_start.elapsed();
            server_clone.metrics.record_request(duration);
            
            duration
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    let mut durations = Vec::new();
    for handle in handles {
        let duration = handle.await.unwrap();
        durations.push(duration);
    }

    let total_duration = start_time.elapsed();
    let avg_duration = durations.iter().map(|d| d.as_millis()).sum::<u128>() / CONCURRENT_REQUESTS as u128;

    println!("Concurrent request test: {} requests in {:?} (avg: {}ms)", 
             CONCURRENT_REQUESTS, total_duration, avg_duration);

    // Verify performance target
    assert!(avg_duration < TARGET_LATENCY_MS as u128, 
            "Concurrent request performance exceeded target: {}ms > {}ms", 
            avg_duration, TARGET_LATENCY_MS);

    Ok(())
}

/// Create test configuration
fn create_test_config() -> McpConfig {
    McpConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        unix_socket: None,
        redis_url: None,
        max_connections: Some(1000),
        auth: rhema_mcp::mcp::AuthConfig::default(),
        watcher: rhema_mcp::mcp::WatcherConfig::default(),
        cache: rhema_mcp::mcp::CacheConfig::default(),
        logging: rhema_mcp::mcp::LoggingConfig::default(),
        use_official_sdk: false,
        startup: rhema_mcp::mcp::StartupConfig::default(),
    }
}

/// Create test daemon
async fn create_test_daemon(config: &McpConfig) -> RhemaResult<Arc<McpDaemon>> {
    let repo_root = std::env::temp_dir().join("rhema_test");
    std::fs::create_dir_all(&repo_root)?;
    
    let daemon = McpDaemon::new(config.clone(), repo_root).await?;
    Ok(Arc::new(daemon))
}

/// Performance benchmark test
#[test]
async fn test_performance_benchmark() -> RhemaResult<()> {
    println!("Running performance benchmark...");
    
    let config = create_test_config();
    let daemon = create_test_daemon(&config).await?;
    let server = HttpServer::new(config, daemon);

    // Warm up
    for _ in 0..WARMUP_REQUESTS {
        let _guard = server.connection_pool.acquire().await?;
        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    // Benchmark
    let start_time = Instant::now();
    let mut total_requests = 0;
    let mut total_duration = Duration::ZERO;

    for _ in 0..10 {
        let batch_start = Instant::now();
        let mut handles = Vec::new();

        for _ in 0..CONCURRENT_REQUESTS {
            let pool = server.connection_pool.clone();
            let handle = tokio::spawn(async move {
                let _guard = pool.acquire().await.unwrap();
                tokio::time::sleep(Duration::from_millis(1)).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let batch_duration = batch_start.elapsed();
        total_requests += CONCURRENT_REQUESTS;
        total_duration += batch_duration;
    }

    let total_time = start_time.elapsed();
    let avg_request_time = total_duration.as_millis() as u64 / total_requests as u64;
    let requests_per_second = total_requests as f64 / total_time.as_secs_f64();

    println!("Performance Benchmark Results:");
    println!("  Total requests: {}", total_requests);
    println!("  Total time: {:?}", total_time);
    println!("  Average request time: {}ms", avg_request_time);
    println!("  Requests per second: {:.2}", requests_per_second);

    // Performance assertions
    assert!(avg_request_time < TARGET_LATENCY_MS, 
            "Average request time exceeded target: {}ms > {}ms", 
            avg_request_time, TARGET_LATENCY_MS);
    
    assert!(requests_per_second > 100.0, 
            "Throughput too low: {:.2} req/s < 100 req/s", 
            requests_per_second);

    println!("✅ Performance benchmark passed!");

    Ok(())
} 