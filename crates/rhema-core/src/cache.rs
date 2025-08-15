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

use crate::RhemaResult;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Cache entry with expiration
#[derive(Clone)]
struct CacheEntry<T> {
    value: T,
    created_at: Instant,
    expires_at: Option<Instant>,
}

impl<T> CacheEntry<T> {
    fn new(value: T, ttl: Option<Duration>) -> Self {
        let created_at = Instant::now();
        let expires_at = ttl.map(|ttl| created_at + ttl);

        Self {
            value,
            created_at,
            expires_at,
        }
    }

    fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Instant::now() > expires_at
        } else {
            false
        }
    }

    fn age(&self) -> Duration {
        Instant::now().duration_since(self.created_at)
    }
}

/// Thread-safe cache implementation
pub struct Cache<K, V> {
    data: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    max_size: usize,
    default_ttl: Option<Duration>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new cache with the specified maximum size and TTL
    pub fn new(max_size: usize, default_ttl: Option<Duration>) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            default_ttl,
        }
    }

    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.write().unwrap();

        if let Some(entry) = data.get(key) {
            if entry.is_expired() {
                data.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }

    /// Set a value in the cache
    pub fn set(&self, key: K, value: V) -> RhemaResult<()> {
        let mut data = self.data.write().unwrap();

        // Check if we need to evict entries
        if data.len() >= self.max_size {
            self.evict_oldest(&mut data);
        }

        let entry = CacheEntry::new(value, self.default_ttl);
        data.insert(key, entry);

        Ok(())
    }

    /// Set a value with a custom TTL
    pub fn set_with_ttl(&self, key: K, value: V, ttl: Duration) -> RhemaResult<()> {
        let mut data = self.data.write().unwrap();

        // Check if we need to evict entries
        if data.len() >= self.max_size {
            self.evict_oldest(&mut data);
        }

        let entry = CacheEntry::new(value, Some(ttl));
        data.insert(key, entry);

        Ok(())
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.write().unwrap();
        data.remove(key).map(|entry| entry.value)
    }

    /// Clear all entries from the cache
    pub fn clear(&self) {
        let mut data = self.data.write().unwrap();
        data.clear();
    }

    /// Get the current size of the cache
    pub fn len(&self) -> usize {
        let data = self.data.read().unwrap();
        data.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        let data = self.data.read().unwrap();
        data.is_empty()
    }

    /// Clean up expired entries
    pub fn cleanup(&self) -> usize {
        let mut data = self.data.write().unwrap();
        let initial_size = data.len();

        data.retain(|_, entry| !entry.is_expired());

        initial_size - data.len()
    }

    /// Evict the oldest entry
    fn evict_oldest(&self, data: &mut HashMap<K, CacheEntry<V>>) {
        if let Some((oldest_key, _)) = data.iter().min_by_key(|(_, entry)| entry.created_at) {
            let key = oldest_key.clone();
            data.remove(&key);
        }
    }
}

/// Async cache implementation for better performance
pub struct AsyncCache<K, V> {
    data: Arc<Mutex<HashMap<K, CacheEntry<V>>>>,
    max_size: usize,
    default_ttl: Option<Duration>,
}

impl<K, V> AsyncCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    /// Create a new async cache
    pub fn new(max_size: usize, default_ttl: Option<Duration>) -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            default_ttl,
        }
    }

    /// Get a value from the cache asynchronously
    pub async fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().await;

        if let Some(entry) = data.get(key) {
            if entry.is_expired() {
                data.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }

    /// Set a value in the cache asynchronously
    pub async fn set(&self, key: K, value: V) -> RhemaResult<()> {
        let mut data = self.data.lock().await;

        // Check if we need to evict entries
        if data.len() >= self.max_size {
            self.evict_oldest(&mut data).await;
        }

        let entry = CacheEntry::new(value, self.default_ttl);
        data.insert(key, entry);

        Ok(())
    }

    /// Set a value with a custom TTL asynchronously
    pub async fn set_with_ttl(&self, key: K, value: V, ttl: Duration) -> RhemaResult<()> {
        let mut data = self.data.lock().await;

        // Check if we need to evict entries
        if data.len() >= self.max_size {
            self.evict_oldest(&mut data).await;
        }

        let entry = CacheEntry::new(value, Some(ttl));
        data.insert(key, entry);

        Ok(())
    }

    /// Remove a value from the cache asynchronously
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().await;
        data.remove(key).map(|entry| entry.value)
    }

    /// Clear all entries from the cache asynchronously
    pub async fn clear(&self) {
        let mut data = self.data.lock().await;
        data.clear();
    }

    /// Get the current size of the cache asynchronously
    pub async fn len(&self) -> usize {
        let data = self.data.lock().await;
        data.len()
    }

    /// Check if the cache is empty asynchronously
    pub async fn is_empty(&self) -> bool {
        let data = self.data.lock().await;
        data.is_empty()
    }

    /// Clean up expired entries asynchronously
    pub async fn cleanup(&self) -> usize {
        let mut data = self.data.lock().await;
        let initial_size = data.len();

        data.retain(|_, entry| !entry.is_expired());

        initial_size - data.len()
    }

    /// Evict the oldest entry asynchronously
    async fn evict_oldest(&self, data: &mut HashMap<K, CacheEntry<V>>) {
        if let Some((oldest_key, _)) = data.iter().min_by_key(|(_, entry)| entry.created_at) {
            let key = oldest_key.clone();
            data.remove(&key);
        }
    }
}

/// Global cache instances for common use cases
pub struct GlobalCaches {
    pub file_content: Arc<Cache<String, Vec<u8>>>,
    pub yaml_data: Arc<Cache<String, serde_yaml::Value>>,
    pub scope_data: Arc<Cache<String, crate::scope::Scope>>,
}

impl GlobalCaches {
    pub fn new() -> Self {
        Self {
            file_content: Arc::new(Cache::new(1000, Some(Duration::from_secs(300)))), // 5 minutes
            yaml_data: Arc::new(Cache::new(500, Some(Duration::from_secs(600)))),     // 10 minutes
            scope_data: Arc::new(Cache::new(100, Some(Duration::from_secs(1800)))),   // 30 minutes
        }
    }
}

impl Default for GlobalCaches {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_cache_basic_operations() {
        let cache = Cache::new(10, Some(Duration::from_secs(1)));

        // Test set and get
        cache.set("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        // Test non-existent key
        assert_eq!(cache.get(&"key2".to_string()), None);

        // Test remove
        assert_eq!(
            cache.remove(&"key1".to_string()),
            Some("value1".to_string())
        );
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_expiration() {
        let cache = Cache::new(10, Some(Duration::from_millis(100)));

        cache.set("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        // Wait for expiration
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_max_size() {
        let cache = Cache::new(2, None);

        cache.set("key1".to_string(), "value1".to_string()).unwrap();
        cache.set("key2".to_string(), "value2".to_string()).unwrap();
        cache.set("key3".to_string(), "value3".to_string()).unwrap();

        // Should have evicted the oldest entry
        assert_eq!(cache.len(), 2);
    }

    #[tokio::test]
    async fn test_async_cache_basic_operations() {
        let cache = AsyncCache::new(10, Some(Duration::from_secs(1)));

        // Test set and get
        cache
            .set("key1".to_string(), "value1".to_string())
            .await
            .unwrap();
        assert_eq!(
            cache.get(&"key1".to_string()).await,
            Some("value1".to_string())
        );

        // Test non-existent key
        assert_eq!(cache.get(&"key2".to_string()).await, None);

        // Test remove
        assert_eq!(
            cache.remove(&"key1".to_string()).await,
            Some("value1".to_string())
        );
        assert_eq!(cache.get(&"key1".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_async_cache_expiration() {
        let cache = AsyncCache::new(10, Some(Duration::from_millis(100)));

        cache
            .set("key1".to_string(), "value1".to_string())
            .await
            .unwrap();
        assert_eq!(
            cache.get(&"key1".to_string()).await,
            Some("value1".to_string())
        );

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert_eq!(cache.get(&"key1".to_string()).await, None);
    }

    #[test]
    fn test_global_caches() {
        let caches = GlobalCaches::new();

        // Test file content cache
        caches
            .file_content
            .set("test.txt".to_string(), b"test content".to_vec())
            .unwrap();
        assert_eq!(
            caches.file_content.get(&"test.txt".to_string()),
            Some(b"test content".to_vec())
        );

        // Test cleanup
        let cleaned = caches.file_content.cleanup();
        assert_eq!(cleaned, 0); // No expired entries yet
    }
}
