//! Performance tests and benchmarks for Rhema CLI

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rhema::{Rhema, RhemaResult};
use tests::common::{TestEnv, TestFixtures, helpers::TestHelpers};
use std::time::{Duration, Instant};

/// Benchmark query execution performance
fn benchmark_query_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_performance");
    
    // Setup test environment with sample data
    let env = TestEnv::with_sample_data().unwrap();
    
    // Benchmark simple query
    group.bench_function("simple_query", |b| {
        b.iter(|| {
            env.rhema.query("todos").unwrap();
        });
    });
    
    // Benchmark filtered query
    group.bench_function("filtered_query", |b| {
        b.iter(|| {
            env.rhema.query("todos WHERE status=pending").unwrap();
        });
    });
    
    // Benchmark complex query
    group.bench_function("complex_query", |b| {
        b.iter(|| {
            env.rhema.query("todos WHERE status=pending AND priority=high").unwrap();
        });
    });
    
    // Benchmark query with stats
    group.bench_function("query_with_stats", |b| {
        b.iter(|| {
            env.rhema.query_with_stats("todos").unwrap();
        });
    });
    
    group.finish();
}

/// Benchmark scope discovery performance
fn benchmark_scope_discovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("scope_discovery");
    
    // Setup test environment
    let env = TestEnv::with_scope().unwrap();
    
    // Benchmark scope discovery
    group.bench_function("discover_scopes", |b| {
        b.iter(|| {
            env.rhema.discover_scopes().unwrap();
        });
    });
    
    // Benchmark get scope
    group.bench_function("get_scope", |b| {
        b.iter(|| {
            env.rhema.get_scope(".rhema").unwrap();
        });
    });
    
    group.finish();
}

/// Benchmark search performance
fn benchmark_search_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_performance");
    
    // Setup test environment
    let env = TestEnv::with_sample_data().unwrap();
    
    // Benchmark regex search
    group.bench_function("regex_search", |b| {
        b.iter(|| {
            env.rhema.search_regex("todo", None).unwrap();
        });
    });
    
    // Benchmark regex search with file filter
    group.bench_function("regex_search_with_filter", |b| {
        b.iter(|| {
            env.rhema.search_regex("todo", Some("*.yaml")).unwrap();
        });
    });
    
    group.finish();
}

/// Benchmark large dataset performance
fn benchmark_large_dataset(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_dataset");
    
    // Test different dataset sizes
    let sizes = vec![100, 1000, 10000];
    
    for size in sizes {
        // Setup test environment with large dataset
        let (temp_dir, repo_path) = TestHelpers::create_temp_git_repo().unwrap();
        TestHelpers::create_large_dataset(&repo_path, size).unwrap();
        let rhema = Rhema::new_from_path(repo_path).unwrap();
        
        // Benchmark query on large dataset
        group.bench_with_input(BenchmarkId::new("query_large_dataset", size), &size, |b, &size| {
            b.iter(|| {
                rhema.query("todos").unwrap();
            });
        });
        
        // Benchmark filtered query on large dataset
        group.bench_with_input(BenchmarkId::new("filtered_query_large_dataset", size), &size, |b, &size| {
            b.iter(|| {
                rhema.query("todos WHERE status=pending").unwrap();
            });
        });
    }
    
    group.finish();
}

/// Benchmark memory usage
fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    // Setup test environment
    let env = TestEnv::with_sample_data().unwrap();
    
    // Benchmark memory usage for multiple queries
    group.bench_function("multiple_queries", |b| {
        b.iter(|| {
            for i in 0..100 {
                let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
                env.rhema.query(&query).unwrap();
            }
        });
    });
    
    // Benchmark memory usage for large result sets
    group.bench_function("large_result_sets", |b| {
        b.iter(|| {
            let result = env.rhema.query("todos").unwrap();
            let result_str = serde_yaml::to_string(&result).unwrap();
            assert!(result_str.len() > 0);
        });
    });
    
    group.finish();
}

/// Benchmark file operations
fn benchmark_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_operations");
    
    // Setup test environment
    let env = TestEnv::with_sample_data().unwrap();
    
    // Benchmark YAML parsing
    group.bench_function("yaml_parsing", |b| {
        let yaml_content = TestFixtures::todos_data();
        b.iter(|| {
            serde_yaml::from_str::<serde_yaml::Value>(yaml_content).unwrap();
        });
    });
    
    // Benchmark YAML serialization
    group.bench_function("yaml_serialization", |b| {
        let value = serde_yaml::from_str::<serde_yaml::Value>(TestFixtures::todos_data()).unwrap();
        b.iter(|| {
            serde_yaml::to_string(&value).unwrap();
        });
    });
    
    group.finish();
}

/// Benchmark concurrent operations
fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    // Setup test environment
    let env = TestEnv::with_sample_data().unwrap();
    
    use std::thread;
    use std::sync::Arc;
    
    // Benchmark concurrent queries
    group.bench_function("concurrent_queries", |b| {
        b.iter(|| {
            let rhema = Arc::new(env.rhema.clone());
            let mut handles = vec![];
            
            for i in 0..10 {
                let rhema_clone = Arc::clone(&rhema);
                let handle = thread::spawn(move || {
                    let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
                    rhema_clone.query(&query)
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap().unwrap();
            }
        });
    });
    
    group.finish();
}

/// Performance test for query execution with different data types
#[test]
fn test_query_performance_different_types() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    let queries = vec![
        "todos",
        "insights",
        "patterns",
        "decisions",
        "todos WHERE status=pending",
        "insights WHERE confidence>=8",
        "patterns WHERE pattern_type=architectural",
        "decisions WHERE status=approved",
    ];
    
    for query in queries {
        let start = Instant::now();
        let result = env.rhema.query(query)?;
        let duration = start.elapsed();
        
        // Verify query completes within reasonable time (100ms)
        assert!(duration.as_millis() < 100, "Query '{}' took too long: {:?}", query, duration);
        
        // Verify result is not empty
        let result_str = serde_yaml::to_string(&result)?;
        assert!(!result_str.is_empty());
    }
    
    Ok(())
}

/// Performance test for large dataset handling
#[test]
fn test_large_dataset_performance() -> RhemaResult<()> {
    let sizes = vec![100, 1000, 10000];
    
    for size in sizes {
        let (temp_dir, repo_path) = TestHelpers::create_temp_git_repo()?;
        TestHelpers::create_large_dataset(&repo_path, size)?;
        let rhema = Rhema::new_from_path(repo_path)?;
        
        let start = Instant::now();
        let result = rhema.query("todos")?;
        let duration = start.elapsed();
        
        // Verify query completes within reasonable time based on dataset size
        let max_time = match size {
            100 => 50,   // 50ms for small dataset
            1000 => 200, // 200ms for medium dataset
            10000 => 1000, // 1s for large dataset
            _ => 5000,   // 5s for very large dataset
        };
        
        assert!(
            duration.as_millis() < max_time,
            "Query on dataset size {} took too long: {:?}",
            size,
            duration
        );
        
        // Verify result contains expected number of items
        let result_str = serde_yaml::to_string(&result)?;
        let expected_count = format!("todo-{:06}", size - 1);
        assert!(result_str.contains(&expected_count));
    }
    
    Ok(())
}

/// Performance test for memory usage
#[test]
fn test_memory_usage_performance() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Execute multiple queries to test memory usage
    for i in 0..1000 {
        let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
        let result = env.rhema.query(&query)?;
        
        // Verify result is valid
        let result_str = serde_yaml::to_string(&result)?;
        assert!(!result_str.is_empty());
        
        // Every 100 queries, check if we're still responsive
        if i % 100 == 0 {
            let start = Instant::now();
            let _ = env.rhema.query("todos")?;
            let duration = start.elapsed();
            
            // Verify query still completes quickly
            assert!(duration.as_millis() < 100);
        }
    }
    
    Ok(())
}

/// Performance test for concurrent access
#[test]
fn test_concurrent_access_performance() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let rhema = Arc::new(env.rhema);
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    // Spawn multiple threads to access Rhema concurrently
    for i in 0..100 {
        let rhema_clone = Arc::clone(&rhema);
        let success_count_clone = Arc::clone(&success_count);
        
        let handle = thread::spawn(move || {
            let query = format!("todos WHERE id=todo-{:03}", (i % 3) + 1);
            match rhema_clone.query(&query) {
                Ok(_) => success_count_clone.fetch_add(1, Ordering::SeqCst),
                Err(_) => 0,
            };
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify most queries succeeded
    let success_count = success_count.load(Ordering::SeqCst);
    assert!(success_count > 90, "Only {} out of 100 concurrent queries succeeded", success_count);
    
    Ok(())
}

/// Performance test for file system operations
#[test]
fn test_file_system_performance() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test scope discovery performance
    let start = Instant::now();
    let scopes = env.rhema.discover_scopes()?;
    let discovery_duration = start.elapsed();
    
    // Verify discovery completes quickly
    assert!(discovery_duration.as_millis() < 50);
    assert_eq!(scopes.len(), 1);
    
    // Test getting scope performance
    let start = Instant::now();
    let scope = env.rhema.get_scope(".rhema")?;
    let get_scope_duration = start.elapsed();
    
    // Verify getting scope completes quickly
    assert!(get_scope_duration.as_millis() < 10);
    assert_eq!(scope.definition.name, "test-scope");
    
    Ok(())
}

/// Performance test for YAML operations
#[test]
fn test_yaml_operations_performance() -> RhemaResult<()> {
    let yaml_content = TestFixtures::todos_data();
    
    // Test YAML parsing performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _value: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;
    }
    let parse_duration = start.elapsed();
    
    // Verify parsing is fast
    assert!(parse_duration.as_millis() < 1000, "YAML parsing took too long: {:?}", parse_duration);
    
    // Test YAML serialization performance
    let value: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;
    let start = Instant::now();
    for _ in 0..1000 {
        let _result = serde_yaml::to_string(&value)?;
    }
    let serialize_duration = start.elapsed();
    
    // Verify serialization is fast
    assert!(serialize_duration.as_millis() < 1000, "YAML serialization took too long: {:?}", serialize_duration);
    
    Ok(())
}

/// Performance test for regex search
#[test]
fn test_regex_search_performance() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    let patterns = vec![
        "todo-\\d+",
        "insight-\\d+",
        "pattern-\\d+",
        "decision-\\d+",
        "\\d{4}-\\d{2}-\\d{2}",
    ];
    
    for pattern in patterns {
        let start = Instant::now();
        let results = env.rhema.search_regex(pattern, None)?;
        let duration = start.elapsed();
        
        // Verify search completes quickly
        assert!(duration.as_millis() < 100, "Regex search '{}' took too long: {:?}", pattern, duration);
        
        // Verify results are found
        assert!(!results.is_empty());
    }
    
    Ok(())
}

/// Performance test for query optimization
#[test]
fn test_query_optimization_performance() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    let queries = vec![
        "todos WHERE status=pending AND priority=high",
        "insights WHERE confidence>=8 AND category=performance",
        "patterns WHERE pattern_type=architectural AND usage=recommended",
        "decisions WHERE status=approved AND makers=architect",
    ];
    
    for query in queries {
        let start = Instant::now();
        let result = env.rhema.query(query)?;
        let duration = start.elapsed();
        
        // Verify optimized query completes quickly
        assert!(duration.as_millis() < 50, "Optimized query '{}' took too long: {:?}", query, duration);
        
        // Verify result is valid
        let result_str = serde_yaml::to_string(&result)?;
        assert!(!result_str.is_empty());
    }
    
    Ok(())
}

/// Performance test for error handling
#[test]
fn test_error_handling_performance() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    let invalid_queries = vec![
        "nonexistent",
        "todos WHERE invalid_field=value",
        "todos WHERE status=invalid_status",
        "todos WHERE priority=invalid_priority",
    ];
    
    for query in invalid_queries {
        let start = Instant::now();
        let result = env.rhema.query(query);
        let duration = start.elapsed();
        
        // Verify error handling is fast
        assert!(duration.as_millis() < 10, "Error handling for '{}' took too long: {:?}", query, duration);
        
        // Verify error is returned
        assert!(result.is_err());
    }
    
    Ok(())
}

/// Performance test for large file handling
#[test]
fn test_large_file_performance() -> RhemaResult<()> {
    let (temp_dir, repo_path) = TestHelpers::create_temp_git_repo()?;
    
    // Create large files
    let file_sizes = vec![1, 5, 10]; // MB
    
    for size in file_sizes {
        let large_file = repo_path.join(format!("large_file_{}mb.yaml", size));
        TestHelpers::create_large_file(&large_file, size)?;
        
        let start = Instant::now();
        let content = std::fs::read_to_string(&large_file)?;
        let read_duration = start.elapsed();
        
        // Verify file reading is reasonable
        let max_time = size * 100; // 100ms per MB
        assert!(read_duration.as_millis() < max_time, "Reading {}MB file took too long: {:?}", size, read_duration);
        
        // Verify content is valid
        assert!(content.len() > size * 1024 * 1024);
    }
    
    Ok(())
}

// Configure criterion benchmarks
criterion_group!(
    benches,
    benchmark_query_performance,
    benchmark_scope_discovery,
    benchmark_search_performance,
    benchmark_large_dataset,
    benchmark_memory_usage,
    benchmark_file_operations,
    benchmark_concurrent_operations,
);

criterion_main!(benches); 