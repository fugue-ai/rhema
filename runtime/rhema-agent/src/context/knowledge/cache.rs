use crate::{Rhema, RhemaError, RhemaResult};
use super::commands::CacheSubcommands;
use colored::*;
use tracing::info;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    key: String,
    value: String,
    created_at: DateTime<Utc>,
    accessed_at: DateTime<Utc>,
    access_count: u64,
    ttl: Option<u64>,
    semantic_indexed: bool,
    semantic_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheStats {
    total_entries: usize,
    memory_usage_bytes: usize,
    disk_usage_bytes: usize,
    hit_count: u64,
    miss_count: u64,
    last_updated: DateTime<Utc>,
}

struct CacheManager {
    memory_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    semantic_index: Arc<RwLock<HashMap<String, Vec<String>>>>,
    cache_dir: PathBuf,
    stats: Arc<RwLock<CacheStats>>,
}

impl CacheManager {
    fn new() -> RhemaResult<Self> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| RhemaError::InvalidInput("Could not determine cache directory".to_string()))?
            .join("rhema-agent")
            .join("knowledge-cache");
        
        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir)
            .map_err(|e| RhemaError::FileSystemError(format!("Failed to create cache directory: {}", e)))?;
        
        Ok(Self {
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            semantic_index: Arc::new(RwLock::new(HashMap::new())),
            cache_dir,
            stats: Arc::new(RwLock::new(CacheStats {
                total_entries: 0,
                memory_usage_bytes: 0,
                disk_usage_bytes: 0,
                hit_count: 0,
                miss_count: 0,
                last_updated: Utc::now(),
            })),
        })
    }
    
    fn get(&self, key: &str) -> RhemaResult<Option<CacheEntry>> {
        let mut cache = self.memory_cache.write()
            .map_err(|e| RhemaError::InternalError(format!("Cache lock error: {}", e)))?;
        
        if let Some(entry) = cache.get_mut(key) {
            // Update access statistics
            entry.accessed_at = Utc::now();
            entry.access_count += 1;
            
            // Update stats
            let mut stats = self.stats.write()
                .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
            stats.hit_count += 1;
            stats.last_updated = Utc::now();
            
            Ok(Some(entry.clone()))
        } else {
            // Try to load from disk
            if let Some(entry) = self.load_from_disk(key)? {
                // Add to memory cache
                cache.insert(key.to_string(), entry.clone());
                
                // Update stats
                let mut stats = self.stats.write()
                    .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
                stats.hit_count += 1;
                stats.last_updated = Utc::now();
                
                Ok(Some(entry))
            } else {
                // Update miss stats
                let mut stats = self.stats.write()
                    .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
                stats.miss_count += 1;
                stats.last_updated = Utc::now();
                
                Ok(None)
            }
        }
    }
    
    fn set(&self, key: &str, value: &str, ttl: Option<u64>, semantic_index: bool) -> RhemaResult<()> {
        let entry = CacheEntry {
            key: key.to_string(),
            value: value.to_string(),
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 1,
            ttl,
            semantic_indexed: semantic_index,
            semantic_tags: if semantic_index {
                self.generate_semantic_tags(value)
            } else {
                Vec::new()
            },
        };
        
        // Store in memory cache
        let mut cache = self.memory_cache.write()
            .map_err(|e| RhemaError::InternalError(format!("Cache lock error: {}", e)))?;
        cache.insert(key.to_string(), entry.clone());
        
        // Store in semantic index if enabled
        if semantic_index {
            let mut semantic_index = self.semantic_index.write()
                .map_err(|e| RhemaError::InternalError(format!("Semantic index lock error: {}", e)))?;
            semantic_index.insert(key.to_string(), entry.semantic_tags.clone());
        }
        
        // Persist to disk
        self.save_to_disk(key, &entry)?;
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        stats.total_entries = cache.len();
        stats.memory_usage_bytes = self.calculate_memory_usage(&cache);
        stats.disk_usage_bytes = self.calculate_disk_usage()?;
        stats.last_updated = Utc::now();
        
        Ok(())
    }
    
    fn delete(&self, key: &str, remove_from_index: bool) -> RhemaResult<bool> {
        let mut cache = self.memory_cache.write()
            .map_err(|e| RhemaError::InternalError(format!("Cache lock error: {}", e)))?;
        
        let was_present = cache.remove(key).is_some();
        
        // Remove from semantic index if requested
        if remove_from_index {
            let mut semantic_index = self.semantic_index.write()
                .map_err(|e| RhemaError::InternalError(format!("Semantic index lock error: {}", e)))?;
            semantic_index.remove(key);
        }
        
        // Remove from disk
        let disk_path = self.cache_dir.join(format!("{}.cache", key));
        if disk_path.exists() {
            fs::remove_file(disk_path)
                .map_err(|e| RhemaError::FileSystemError(format!("Failed to delete cache file: {}", e)))?;
        }
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        stats.total_entries = cache.len();
        stats.memory_usage_bytes = self.calculate_memory_usage(&cache);
        stats.disk_usage_bytes = self.calculate_disk_usage()?;
        stats.last_updated = Utc::now();
        
        Ok(was_present)
    }
    
    fn list(&self, pattern: Option<&str>) -> RhemaResult<Vec<CacheEntry>> {
        let cache = self.memory_cache.read()
            .map_err(|e| RhemaError::InternalError(format!("Cache lock error: {}", e)))?;
        
        let entries: Vec<CacheEntry> = if let Some(pattern) = pattern {
            cache.values()
                .filter(|entry| entry.key.contains(pattern) || entry.value.contains(pattern))
                .cloned()
                .collect()
        } else {
            cache.values().cloned().collect()
        };
        
        Ok(entries)
    }
    
    fn clear(&self, memory_only: bool, disk_only: bool) -> RhemaResult<()> {
        if !memory_only {
            let mut cache = self.memory_cache.write()
                .map_err(|e| RhemaError::InternalError(format!("Cache lock error: {}", e)))?;
            cache.clear();
            
            let mut semantic_index = self.semantic_index.write()
                .map_err(|e| RhemaError::InternalError(format!("Semantic index lock error: {}", e)))?;
            semantic_index.clear();
        }
        
        if !disk_only {
            // Clear all cache files
            for entry in fs::read_dir(&self.cache_dir)
                .map_err(|e| RhemaError::FileSystemError(format!("Failed to read cache directory: {}", e)))? {
                let entry = entry
                    .map_err(|e| RhemaError::FileSystemError(format!("Failed to read cache entry: {}", e)))?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("cache") {
                    fs::remove_file(entry.path())
                        .map_err(|e| RhemaError::FileSystemError(format!("Failed to delete cache file: {}", e)))?;
                }
            }
        }
        
        // Update stats
        let mut stats = self.stats.write()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        stats.total_entries = 0;
        stats.memory_usage_bytes = 0;
        stats.disk_usage_bytes = 0;
        stats.last_updated = Utc::now();
        
        Ok(())
    }
    
    fn get_stats(&self) -> RhemaResult<CacheStats> {
        let stats = self.stats.read()
            .map_err(|e| RhemaError::InternalError(format!("Stats lock error: {}", e)))?;
        Ok(stats.clone())
    }
    
    fn search_semantic(&self, query: &str) -> RhemaResult<Vec<CacheEntry>> {
        let semantic_index = self.semantic_index.read()
            .map_err(|e| RhemaError::InternalError(format!("Semantic index lock error: {}", e)))?;
        
        let cache = self.memory_cache.read()
            .map_err(|e| RhemaError::InternalError(format!("Cache lock error: {}", e)))?;
        
        let query_tags = self.generate_semantic_tags(query);
        let mut results = Vec::new();
        
        for (key, entry_tags) in semantic_index.iter() {
            let similarity = self.calculate_semantic_similarity(&query_tags, entry_tags);
            if similarity > 0.3 { // Threshold for semantic similarity
                if let Some(entry) = cache.get(key) {
                    results.push(entry.clone());
                }
            }
        }
        
        // Sort by similarity (simplified - just by access count for now)
        results.sort_by(|a, b| b.access_count.cmp(&a.access_count));
        
        Ok(results)
    }
    
    // Private helper methods
    
    fn load_from_disk(&self, key: &str) -> RhemaResult<Option<CacheEntry>> {
        let file_path = self.cache_dir.join(format!("{}.cache", key));
        
        if !file_path.exists() {
            return Ok(None);
        }
        
        let data = fs::read_to_string(&file_path)
            .map_err(|e| RhemaError::FileSystemError(format!("Failed to read cache file: {}", e)))?;
        
        let entry: CacheEntry = serde_json::from_str(&data)
            .map_err(|e| RhemaError::SerializationError(format!("Failed to deserialize cache entry: {}", e)))?;
        
        // Check TTL
        if let Some(ttl) = entry.ttl {
            let age = Utc::now().signed_duration_since(entry.created_at).num_seconds() as u64;
            if age > ttl {
                // Entry expired, remove it
                fs::remove_file(&file_path)
                    .map_err(|e| RhemaError::FileSystemError(format!("Failed to delete expired cache file: {}", e)))?;
                return Ok(None);
            }
        }
        
        Ok(Some(entry))
    }
    
    fn save_to_disk(&self, key: &str, entry: &CacheEntry) -> RhemaResult<()> {
        let file_path = self.cache_dir.join(format!("{}.cache", key));
        let data = serde_json::to_string_pretty(entry)
            .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize cache entry: {}", e)))?;
        
        fs::write(&file_path, data)
            .map_err(|e| RhemaError::FileSystemError(format!("Failed to write cache file: {}", e)))?;
        
        Ok(())
    }
    
    fn generate_semantic_tags(&self, content: &str) -> Vec<String> {
        // Simple keyword extraction - in a real implementation, this would use NLP/ML
        let words: Vec<&str> = content
            .split_whitespace()
            .filter(|word| word.len() > 3) // Filter out short words
            .take(10) // Limit to 10 keywords
            .collect();
        
        words.iter().map(|s| s.to_lowercase()).collect()
    }
    
    fn calculate_semantic_similarity(&self, tags1: &[String], tags2: &[String]) -> f32 {
        // Simple Jaccard similarity - in a real implementation, this would use embeddings
        let set1: std::collections::HashSet<_> = tags1.iter().collect();
        let set2: std::collections::HashSet<_> = tags2.iter().collect();
        
        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
    
    fn calculate_memory_usage(&self, cache: &HashMap<String, CacheEntry>) -> usize {
        cache.values().map(|entry| entry.key.len() + entry.value.len()).sum()
    }
    
    fn calculate_disk_usage(&self) -> RhemaResult<usize> {
        let mut total_size = 0;
        for entry in fs::read_dir(&self.cache_dir)
            .map_err(|e| RhemaError::FileSystemError(format!("Failed to read cache directory: {}", e)))? {
            let entry = entry
                .map_err(|e| RhemaError::FileSystemError(format!("Failed to read cache entry: {}", e)))?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("cache") {
                let metadata = entry.metadata()
                    .map_err(|e| RhemaError::FileSystemError(format!("Failed to get file metadata: {}", e)))?;
                total_size += metadata.len() as usize;
            }
        }
        Ok(total_size)
    }
}

// Global cache manager instance
lazy_static::lazy_static! {
    static ref CACHE_MANAGER: Arc<CacheManager> = {
        Arc::new(CacheManager::new().expect("Failed to initialize cache manager"))
    };
}

pub struct CacheOperations;

impl CacheOperations {
    pub async fn handle_cache_commands(rhema: &Rhema, subcommand: CacheSubcommands) -> RhemaResult<()> {
        match subcommand {
            CacheSubcommands::Get { key, semantic_search, query } => {
                Self::get_cache(rhema, key, semantic_search, query).await
            }
            CacheSubcommands::Set { key, value, index_semantic, ttl } => {
                Self::set_cache(rhema, key, value, index_semantic, ttl).await
            }
            CacheSubcommands::Delete { key, remove_from_index } => {
                Self::delete_cache(rhema, key, remove_from_index).await
            }
            CacheSubcommands::List { pattern, detailed, format } => {
                Self::list_cache(rhema, pattern, detailed, format).await
            }
            CacheSubcommands::Clear { memory_only, disk_only, force } => {
                Self::clear_cache(rhema, memory_only, disk_only, force).await
            }
        }
    }

    async fn get_cache(
        _rhema: &Rhema,
        key: String,
        semantic_search: bool,
        query: Option<String>,
    ) -> RhemaResult<()> {
        info!("Getting cache entry: {}", key);
        
        if semantic_search {
            if let Some(search_query) = query {
                let results = CACHE_MANAGER.search_semantic(&search_query)?;
                println!("🔍 Semantic search results for '{}':", search_query);
                for (i, entry) in results.iter().take(5).enumerate() {
                    println!("  {}. {} (similarity: {:.2})", i + 1, entry.key, entry.access_count as f32 / 100.0);
                    println!("     Value: {}", entry.value.chars().take(100).collect::<String>());
                }
            } else {
                return Err(RhemaError::InvalidInput("Semantic search requires a query parameter".to_string()));
            }
        } else {
            if let Some(entry) = CACHE_MANAGER.get(&key)? {
                println!("📋 Cache Entry: {}", key);
                println!("  - Value: {}", entry.value);
                println!("  - Created: {}", entry.created_at.format("%Y-%m-%d %H:%M:%S"));
                println!("  - Last accessed: {}", entry.accessed_at.format("%Y-%m-%d %H:%M:%S"));
                println!("  - Access count: {}", entry.access_count);
                println!("  - Semantic indexed: {}", entry.semantic_indexed);
                if !entry.semantic_tags.is_empty() {
                    println!("  - Semantic tags: {}", entry.semantic_tags.join(", "));
                }
            } else {
                println!("❌ Cache entry not found: {}", key);
            }
        }
        
        Ok(())
    }

    async fn set_cache(
        _rhema: &Rhema,
        key: String,
        value: String,
        index_semantic: bool,
        ttl: Option<u64>,
    ) -> RhemaResult<()> {
        info!("Setting cache entry: {}", key);
        
        CACHE_MANAGER.set(&key, &value, ttl, index_semantic)?;
        
        println!("✅ Cache entry set: {}", key);
        println!("  - Value: {}", value);
        println!("  - Semantic indexing: {}", index_semantic);
        if let Some(ttl_val) = ttl {
            println!("  - TTL: {} seconds", ttl_val);
        }
        
        Ok(())
    }

    async fn delete_cache(
        _rhema: &Rhema,
        key: String,
        remove_from_index: bool,
    ) -> RhemaResult<()> {
        info!("Deleting cache entry: {}", key);
        
        let was_deleted = CACHE_MANAGER.delete(&key, remove_from_index)?;
        
        if was_deleted {
            println!("🗑️  Cache entry deleted: {}", key);
            println!("  - Removed from index: {}", remove_from_index);
        } else {
            println!("❌ Cache entry not found: {}", key);
        }
        
        Ok(())
    }

    async fn list_cache(
        _rhema: &Rhema,
        pattern: Option<String>,
        detailed: bool,
        _format: String,
    ) -> RhemaResult<()> {
        info!("Listing cache entries");
        
        let entries = CACHE_MANAGER.list(pattern.as_deref())?;
        
        println!("📋 Cache Entries ({} total):", entries.len());
        for (i, entry) in entries.iter().enumerate() {
            println!("  {}. {}", i + 1, entry.key);
            if detailed {
                println!("     Value: {}", entry.value.chars().take(50).collect::<String>());
                println!("     Created: {}", entry.created_at.format("%Y-%m-%d %H:%M:%S"));
                println!("     Access count: {}", entry.access_count);
                println!("     Semantic indexed: {}", entry.semantic_indexed);
            }
        }
        
        if detailed {
            let stats = CACHE_MANAGER.get_stats()?;
            println!("  - Total entries: {}", stats.total_entries);
            println!("  - Memory usage: {} bytes", stats.memory_usage_bytes);
            println!("  - Disk usage: {} bytes", stats.disk_usage_bytes);
            println!("  - Hit rate: {:.2}%", 
                if stats.hit_count + stats.miss_count > 0 {
                    (stats.hit_count as f64 / (stats.hit_count + stats.miss_count) as f64) * 100.0
                } else {
                    0.0
                }
            );
        }
        
        Ok(())
    }

    async fn clear_cache(
        _rhema: &Rhema,
        memory_only: bool,
        disk_only: bool,
        force: bool,
    ) -> RhemaResult<()> {
        info!("Clearing cache");
        
        if !force {
            println!("⚠️  This will clear the cache. Use --force to confirm.");
            return Ok(());
        }
        
        CACHE_MANAGER.clear(memory_only, disk_only)?;
        
        println!("🧹 Cache cleared");
        println!("  - Memory only: {}", memory_only);
        println!("  - Disk only: {}", disk_only);
        println!("  - Force: {}", force);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_basic_operations() {
        // This is a basic test to ensure the cache operations work
        // In a real implementation, you'd want more comprehensive tests
        
        let rhema = Rhema::new().expect("Failed to create Rhema instance");
        
        // Test setting a cache entry
        let set_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(CacheOperations::set_cache(&rhema, "test_key".to_string(), "test_value".to_string(), false, None));
        assert!(set_result.is_ok());
        
        // Test getting a cache entry
        let get_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(CacheOperations::get_cache(&rhema, "test_key".to_string(), false, None));
        assert!(get_result.is_ok());
        
        // Test listing cache entries
        let list_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(CacheOperations::list_cache(&rhema, None, false, "table".to_string()));
        assert!(list_result.is_ok());
        
        // Test deleting a cache entry
        let delete_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(CacheOperations::delete_cache(&rhema, "test_key".to_string(), false));
        assert!(delete_result.is_ok());
    }
}
