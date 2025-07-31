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

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use rhema_core::{RhemaError, RhemaResult};

/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub created_at: Instant,
    pub accessed_at: Instant,
    pub access_count: u64,
    pub ttl: Duration,
}

// Custom serialization for CacheEntry
impl<T> Serialize for CacheEntry<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CacheEntry", 5)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("created_at", &self.created_at.elapsed().as_secs())?;
        state.serialize_field("accessed_at", &self.accessed_at.elapsed().as_secs())?;
        state.serialize_field("access_count", &self.access_count)?;
        state.serialize_field("ttl", &self.ttl.as_secs())?;
        state.end()
    }
}

// Custom deserialization for CacheEntry
impl<'de, T> Deserialize<'de> for CacheEntry<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CacheEntryHelper<T> {
            data: T,
            created_at: u64,
            accessed_at: u64,
            access_count: u64,
            ttl: u64,
        }

        let helper = CacheEntryHelper::deserialize(deserializer)?;
        let now = Instant::now();

        Ok(CacheEntry {
            data: helper.data,
            created_at: now
                .checked_sub(Duration::from_secs(helper.created_at))
                .unwrap_or(now),
            accessed_at: now
                .checked_sub(Duration::from_secs(helper.accessed_at))
                .unwrap_or(now),
            access_count: helper.access_count,
            ttl: Duration::from_secs(helper.ttl),
        })
    }
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            data,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    pub fn touch(&mut self) {
        self.accessed_at = Instant::now();
        self.access_count += 1;
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub memory_usage_bytes: u64,
    pub eviction_count: u64,
}

/// Cache manager with in-memory and Redis layers
pub struct CacheManager {
    memory_cache: Arc<DashMap<String, CacheEntry<Value>>>,
    redis_client: Option<Arc<redis::Client>>,
    config: CacheConfig,
    stats: Arc<RwLock<CacheStats>>,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub memory_enabled: bool,
    pub redis_enabled: bool,
    pub redis_url: Option<String>,
    pub ttl_seconds: u64,
    pub max_size: usize,
}

impl CacheManager {
    /// Create a new cache manager
    pub async fn new(config: &super::CacheManagerConfig) -> RhemaResult<Self> {
        let memory_cache = Arc::new(DashMap::new());

        let redis_client = if config.redis_enabled {
            if let Some(redis_url) = &config.redis_url {
                let client = redis::Client::open(redis_url.as_str()).map_err(|e| {
                    RhemaError::InvalidInput(format!("Failed to create Redis client: {}", e))
                })?;
                Some(Arc::new(client))
            } else {
                None
            }
        } else {
            None
        };

        let cache_config = CacheConfig {
            memory_enabled: config.memory_enabled,
            redis_enabled: config.redis_enabled,
            redis_url: config.redis_url.clone(),
            ttl_seconds: config.ttl_seconds,
            max_size: config.max_size,
        };

        let stats = Arc::new(RwLock::new(CacheStats {
            total_entries: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
            memory_usage_bytes: 0,
            eviction_count: 0,
        }));

        Ok(Self {
            memory_cache,
            redis_client,
            config: cache_config,
            stats,
        })
    }

    /// Get a value from cache
    pub async fn get(&self, key: &str) -> RhemaResult<Option<Value>> {
        // Try memory cache first
        if self.config.memory_enabled {
            if let Some(mut entry) = self.memory_cache.get_mut(key) {
                if entry.is_expired() {
                    self.memory_cache.remove(key);
                    self.update_stats_miss().await;
                    return Ok(None);
                }

                entry.touch();
                self.update_stats_hit().await;
                return Ok(Some(entry.data.clone()));
            }
        }

        // Try Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                if let Ok(value) = self.get_from_redis(client, key).await {
                    // Store in memory cache for faster access
                    if self.config.memory_enabled {
                        self.set_in_memory(key, value.clone()).await;
                    }
                    self.update_stats_hit().await;
                    return Ok(Some(value));
                }
            }
        }

        self.update_stats_miss().await;
        Ok(None)
    }

    /// Set a value in cache
    pub async fn set(&self, key: &str, value: Value) -> RhemaResult<()> {
        let ttl = Duration::from_secs(self.config.ttl_seconds);

        // Set in memory cache
        if self.config.memory_enabled {
            self.set_in_memory(key, value.clone()).await;
        }

        // Set in Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                self.set_in_redis(client, key, value, ttl).await?;
            }
        }

        Ok(())
    }

    /// Delete a value from cache
    pub async fn delete(&self, key: &str) -> RhemaResult<()> {
        // Delete from memory cache
        if self.config.memory_enabled {
            self.memory_cache.remove(key);
        }

        // Delete from Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                self.delete_from_redis(client, key).await?;
            }
        }

        Ok(())
    }

    /// Clear all cache entries
    pub async fn clear(&self) -> RhemaResult<()> {
        // Clear memory cache
        if self.config.memory_enabled {
            self.memory_cache.clear();
        }

        // Clear Redis cache
        if self.config.redis_enabled {
            if let Some(client) = &self.redis_client {
                self.clear_redis(client).await?;
            }
        }

        // Reset stats
        let mut stats = self.stats.write().await;
        stats.total_entries = 0;
        stats.memory_usage_bytes = 0;

        Ok(())
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get cache hit rate
    pub async fn hit_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        stats.hit_rate
    }

    /// Evict expired entries
    pub async fn evict_expired(&self) -> RhemaResult<usize> {
        if !self.config.memory_enabled {
            return Ok(0);
        }

        let mut evicted = 0;
        let mut to_remove = Vec::new();

        for entry in self.memory_cache.iter() {
            if entry.value().is_expired() {
                to_remove.push(entry.key().clone());
            }
        }

        for key in to_remove {
            self.memory_cache.remove(&key);
            evicted += 1;
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.eviction_count += evicted as u64;
        stats.total_entries = self.memory_cache.len();

        Ok(evicted)
    }

    /// Get memory usage estimate
    pub async fn memory_usage(&self) -> u64 {
        if !self.config.memory_enabled {
            return 0;
        }

        let mut total_size = 0;
        for entry in self.memory_cache.iter() {
            // Rough estimate: key size + value size + overhead
            total_size += entry.key().len() as u64;
            total_size += serde_json::to_string(&entry.value().data)
                .map(|s| s.len() as u64)
                .unwrap_or(0);
            total_size += std::mem::size_of::<CacheEntry<Value>>() as u64;
        }

        total_size
    }

    async fn set_in_memory(&self, key: &str, value: Value) {
        let ttl = Duration::from_secs(self.config.ttl_seconds);
        let entry = CacheEntry::new(value, ttl);

        // Check if we need to evict entries
        if self.memory_cache.len() >= self.config.max_size {
            self.evict_lru().await;
        }

        self.memory_cache.insert(key.to_string(), entry);

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_entries = self.memory_cache.len();
        stats.memory_usage_bytes = self.memory_usage().await;
    }

    async fn evict_lru(&self) {
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for entry in self.memory_cache.iter() {
            if entry.value().accessed_at < oldest_time {
                oldest_time = entry.value().accessed_at;
                oldest_key = Some(entry.key().clone());
            }
        }

        if let Some(key) = oldest_key {
            self.memory_cache.remove(&key);
            let mut stats = self.stats.write().await;
            stats.eviction_count += 1;
        }
    }

    async fn get_from_redis(&self, client: &redis::Client, key: &str) -> RhemaResult<Value> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis GET failed: {}", e)))?;

        match result {
            Some(data) => {
                let value: Value = serde_json::from_str(&data).map_err(|e| {
                    RhemaError::InvalidInput(format!("Failed to deserialize Redis value: {}", e))
                })?;
                Ok(value)
            }
            None => Err(RhemaError::InvalidInput(
                "Key not found in Redis".to_string(),
            )),
        }
    }

    async fn set_in_redis(
        &self,
        client: &redis::Client,
        key: &str,
        value: Value,
        ttl: Duration,
    ) -> RhemaResult<()> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        let data = serde_json::to_string(&value)
            .map_err(|e| RhemaError::InvalidInput(format!("Failed to serialize value: {}", e)))?;

        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl.as_secs())
            .arg(data)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis SETEX failed: {}", e)))?;

        Ok(())
    }

    async fn delete_from_redis(&self, client: &redis::Client, key: &str) -> RhemaResult<()> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        redis::cmd("DEL")
            .arg(key)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis DEL failed: {}", e)))?;

        Ok(())
    }

    async fn clear_redis(&self, client: &redis::Client) -> RhemaResult<()> {
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis connection failed: {}", e)))?;

        redis::cmd("FLUSHDB")
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| RhemaError::InvalidInput(format!("Redis FLUSHDB failed: {}", e)))?;

        Ok(())
    }

    async fn update_stats_hit(&self) {
        let mut stats = self.stats.write().await;
        stats.hit_count += 1;
        stats.hit_rate = stats.hit_count as f64 / (stats.hit_count + stats.miss_count) as f64;
    }

    async fn update_stats_miss(&self) {
        let mut stats = self.stats.write().await;
        stats.miss_count += 1;
        stats.hit_rate = stats.hit_count as f64 / (stats.hit_count + stats.miss_count) as f64;
    }
}

/// Cached function for expensive operations
pub async fn cached_expensive_operation(key: &str) -> Value {
    // Simulate expensive operation
    tokio::time::sleep(Duration::from_millis(100)).await;
    serde_json::json!({
        "key": key,
        "result": "expensive_operation_result",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let config = super::super::CacheConfig {
            memory_enabled: true,
            redis_enabled: false,
            redis_url: None,
            ttl_seconds: 3600,
            max_size: 1000,
        };

        let cache = CacheManager::new(&config).await.unwrap();

        // Test set and get
        let key = "test_key";
        let value = serde_json::json!({"data": "test_value"});

        cache.set(key, value.clone()).await.unwrap();
        let retrieved = cache.get(key).await.unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), value);

        // Test delete
        cache.delete(key).await.unwrap();
        let retrieved = cache.get(key).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let config = super::super::CacheConfig {
            memory_enabled: true,
            redis_enabled: false,
            redis_url: None,
            ttl_seconds: 1, // 1 second TTL
            max_size: 100,
        };

        let cache = CacheManager::new(&config).await.unwrap();

        let key = "expire_key";
        let value = serde_json::json!({"data": "will_expire"});

        cache.set(key, value).await.unwrap();

        // Should be available immediately
        assert!(cache.get(key).await.unwrap().is_some());

        // Wait for expiration (reduced wait time to prevent hanging)
        tokio::time::sleep(Duration::from_millis(1100)).await;

        // Manually evict expired entries
        cache.evict_expired().await.unwrap();

        // Should be expired
        assert!(cache.get(key).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let config = super::super::CacheConfig {
            memory_enabled: true,
            redis_enabled: false,
            redis_url: None,
            ttl_seconds: 60,
            max_size: 100,
        };

        let cache = CacheManager::new(&config).await.unwrap();

        // Add some entries
        for i in 0..5 {
            let key = format!("key_{}", i);
            let value = serde_json::json!({"data": i});
            cache.set(&key, value).await.unwrap();
        }

        // Access some entries
        for i in 0..3 {
            let key = format!("key_{}", i);
            cache.get(&key).await.unwrap();
        }

        let stats = cache.stats().await;
        assert_eq!(stats.total_entries, 5);
        assert_eq!(stats.hit_count, 3);
        assert!(stats.hit_rate > 0.0);
    }
}
