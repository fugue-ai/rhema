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

use rhema::lock::{
    LockSystem, LockFileCache, CacheConfig, InvalidationStrategy, WarmingStrategy,
    init_global_cache, get_global_cache, cache_utils,
};
use rhema::{RhemaError, RhemaResult};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_cache_initialization() -> RhemaResult<()> {
    // Initialize the lock system with caching
    LockSystem::initialize()?;
    
    // Verify global cache is available
    let cache = get_global_cache()?;
    assert!(cache.config().enable_persistent);
    
    Ok(())
}

#[test]
fn test_cache_configuration() -> RhemaResult<()> {
    let config = CacheConfig {
        max_size_bytes: 50 * 1024 * 1024, // 50MB
        default_ttl: Some(1800), // 30 minutes
        invalidation_strategy: InvalidationStrategy::Hybrid,
        enable_persistent: true,
        persistent_cache_dir: Some(PathBuf::from(".test-cache")),
        enable_compression: true,
        max_entries: 5000,
        cleanup_interval: 600, // 10 minutes
        enable_stats: true,
        enable_warming: true,
        warming_strategy: WarmingStrategy::FrequentAccess,
    };
    
    // Initialize cache with custom config
    init_global_cache(config.clone())?;
    
    let cache = get_global_cache()?;
    assert_eq!(cache.config().max_size_bytes, 50 * 1024 * 1024);
    assert_eq!(cache.config().default_ttl, Some(1800));
    
    Ok(())
}

#[test]
fn test_cache_operations() -> RhemaResult<()> {
    // Initialize cache
    LockSystem::initialize()?;
    
    // Test cache statistics
    let stats = LockSystem::get_cache_stats()?;
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.total_entries, 0);
    
    // Test cache clearing
    LockSystem::clear_caches()?;
    
    // Test cache performance report
    let report = LockSystem::get_cache_performance_report()?;
    assert!(report.contains("Cache Performance Report"));
    
    // Test cache optimization
    LockSystem::optimize_cache()?;
    
    Ok(())
}

#[test]
fn test_cache_utils() -> RhemaResult<()> {
    // Test cache utilities
    cache_utils::clear_expired_entries()?;
    
    let report = cache_utils::get_performance_report()?;
    assert!(report.contains("Hit Rate"));
    
    cache_utils::optimize_cache()?;
    
    Ok(())
}

#[test]
fn test_cache_with_temp_directory() -> RhemaResult<()> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().join("cache");
    
    let config = CacheConfig {
        persistent_cache_dir: Some(cache_dir.clone()),
        enable_persistent: true,
        ..Default::default()
    };
    
    // Create cache with temp directory
    let cache = LockFileCache::new(config)?;
    
    // Test basic operations
    let test_data = vec![1, 2, 3, 4, 5];
    let key = rhema::lock::CacheKey::Custom("test_key".to_string());
    
    cache.set_serializable(&key, &test_data, Some(3600), 5)?;
    
    let retrieved: Option<Vec<u8>> = cache.get_serializable(&key)?;
    assert_eq!(retrieved, Some(test_data));
    
    // Test cache statistics
    let stats = cache.stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 0);
    
    // Test cache removal
    let removed = cache.remove(&key)?;
    assert!(removed);
    
    let retrieved: Option<Vec<u8>> = cache.get_serializable(&key)?;
    assert_eq!(retrieved, None);
    
    Ok(())
}

#[test]
fn test_cache_invalidation_strategies() -> RhemaResult<()> {
    let config = CacheConfig {
        invalidation_strategy: InvalidationStrategy::Lru,
        max_entries: 3,
        ..Default::default()
    };
    
    let cache = LockFileCache::new(config)?;
    
    // Add entries to trigger LRU eviction
    for i in 0..5 {
        let key = rhema::lock::CacheKey::Custom(format!("key_{}", i));
        let data = vec![i; 1000]; // 1KB each
        cache.set_serializable(&key, &data, Some(3600), 1)?;
    }
    
    let stats = cache.stats();
    assert!(stats.evictions > 0);
    
    Ok(())
}

#[test]
fn test_cache_ttl_expiration() -> RhemaResult<()> {
    let cache = LockFileCache::new(CacheConfig::default())?;
    
    let key = rhema::lock::CacheKey::Custom("expiring_key".to_string());
    let data = vec![1, 2, 3];
    
    // Set with very short TTL
    cache.set_serializable(&key, &data, Some(1), 5)?;
    
    // Should be available immediately
    let retrieved: Option<Vec<u8>> = cache.get_serializable(&key)?;
    assert_eq!(retrieved, Some(data));
    
    // Wait for expiration
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Should be expired now
    let retrieved: Option<Vec<u8>> = cache.get_serializable(&key)?;
    assert_eq!(retrieved, None);
    
    Ok(())
}

#[test]
fn test_cache_warming() -> RhemaResult<()> {
    let config = CacheConfig {
        enable_warming: true,
        warming_strategy: WarmingStrategy::FrequentAccess,
        ..Default::default()
    };
    
    let cache = LockFileCache::new(config)?;
    
    // Test cache warming (this would normally analyze access patterns)
    let temp_dir = TempDir::new()?;
    cache.warm_cache(temp_dir.path())?;
    
    Ok(())
}

#[test]
fn test_cache_error_handling() {
    // Test with invalid cache directory
    let config = CacheConfig {
        persistent_cache_dir: Some(PathBuf::from("/invalid/path/that/does/not/exist")),
        enable_persistent: true,
        ..Default::default()
    };
    
    let result = LockFileCache::new(config);
    assert!(result.is_err());
}

#[test]
fn test_cache_concurrent_access() -> RhemaResult<()> {
    use std::sync::Arc;
    use std::thread;
    
    let cache = Arc::new(LockFileCache::new(CacheConfig::default())?);
    let mut handles = vec![];
    
    // Spawn multiple threads to test concurrent access
    for i in 0..10 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let key = rhema::lock::CacheKey::Custom(format!("concurrent_key_{}", i));
            let data = vec![i; 100];
            
            cache_clone.set_serializable(&key, &data, Some(3600), 1).unwrap();
            
            let retrieved: Option<Vec<u8>> = cache_clone.get_serializable(&key).unwrap();
            assert_eq!(retrieved, Some(data));
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let stats = cache.stats();
    assert_eq!(stats.hits, 10);
    assert_eq!(stats.total_entries, 10);
    
    Ok(())
} 