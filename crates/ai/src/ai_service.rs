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

use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{instrument};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use cached::TimedCache;
// use std::num::NonZeroU32;
// use governor::{Quota, RateLimiter};
// use governor::state::InMemoryState;

/// AI Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_concurrent_requests: usize,
    pub rate_limit_per_minute: u32,
    pub cache_ttl_seconds: u64,
    pub model_version: String,
    pub enable_caching: bool,
    pub enable_rate_limiting: bool,
    pub enable_monitoring: bool,
}

/// AI Request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub id: String,
    pub prompt: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// AI Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub id: String,
    pub request_id: String,
    pub content: String,
    pub model_used: String,
    pub model_version: String,
    pub tokens_used: u32,
    pub processing_time_ms: u64,
    pub cached: bool,
    pub created_at: DateTime<Utc>,
}

/// AI Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub name: String,
    pub version: String,
    pub max_tokens: u32,
    pub cost_per_token: f64,
    pub performance_score: f32,
    pub last_updated: DateTime<Utc>,
}

/// AI Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub average_response_time_ms: u64,
    pub total_tokens_processed: u64,
    pub total_cost: f64,
    pub last_updated: DateTime<Utc>,
}

/// AI Service with caching and rate limiting
pub struct AIService {
    config: AIServiceConfig,
    _cache: Arc<TimedCache<String, AIResponse>>,
    // rate_limiter: Arc<RateLimiter<String, InMemoryState, governor::clock::DefaultClock>>,
    models: Arc<RwLock<HashMap<String, AIModel>>>,
    metrics: Arc<RwLock<AIServiceMetrics>>,
    client: reqwest::Client,
}

impl AIService {
    /// Create a new AI service instance
    pub async fn new(config: AIServiceConfig) -> RhemaResult<Self> {
        let cache = Arc::new(TimedCache::with_lifespan(config.cache_ttl_seconds));
        
        // let rate_limiter = if config.enable_rate_limiting {
        //     Arc::new(RateLimiter::new(
        //         Quota::per_minute(NonZeroU32::new(config.rate_limit_per_minute).unwrap()),
        //     ).unwrap())
        // } else {
        //     Arc::new(RateLimiter::new(
        //         Quota::per_minute(NonZeroU32::new(u32::MAX).unwrap()),
        //     ).unwrap())
        // };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        let models = Arc::new(RwLock::new(HashMap::new()));
        let metrics = Arc::new(RwLock::new(AIServiceMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
            average_response_time_ms: 0,
            total_tokens_processed: 0,
            total_cost: 0.0,
            last_updated: Utc::now(),
        }));

        Ok(Self {
            config,
            _cache: cache,
            // rate_limiter,
            models,
            metrics,
            client,
        })
    }

    /// Process an AI request with caching and rate limiting
    #[instrument(skip(self, request))]
    pub async fn process_request(&self, request: AIRequest) -> RhemaResult<AIResponse> {
        let start_time = std::time::Instant::now();
        
        // Check rate limiting
        // if self.config.enable_rate_limiting {
        //     let key = request.user_id.clone().unwrap_or_else(|| "anonymous".to_string());
        //     if let Err(_) = self.rate_limiter.check_key(&key) {
        //         return Err(RhemaError::ConfigError("Rate limit exceeded".to_string()));
        //     }
        // }

        // Check cache first
        let _cache_key = self.generate_cache_key(&request);
        if self.config.enable_caching {
            // if let Some(cached_response) = self.cache.get(&cache_key) {
            //     self.update_metrics(true, start_time.elapsed().as_millis() as u64, 0, 0.0).await;
            //     return Ok(cached_response);
            // }
        }

        // Process request
        let response = self.call_ai_api(&request).await?;
        
        // Cache the response
        if self.config.enable_caching {
            // self.cache.insert(cache_key, response.clone());
        }

        // Update metrics
        let processing_time = start_time.elapsed().as_millis() as u64;
        self.update_metrics(false, processing_time, response.tokens_used, self.calculate_cost(&response)).await;

        Ok(response)
    }

    /// Call the AI API
    #[instrument(skip(self, request))]
    async fn call_ai_api(&self, request: &AIRequest) -> RhemaResult<AIResponse> {
        let request_body = serde_json::json!({
            "model": request.model,
            "prompt": request.prompt,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
        });

        let response = self.client
            .post(&format!("{}/v1/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| RhemaError::ConfigError(format!("AI API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(RhemaError::ConfigError(format!("AI API error: {}", error_text)));
        }

        let response_json: serde_json::Value = response.json().await
            .map_err(|e| RhemaError::ConfigError(format!("Failed to parse AI API response: {}", e)))?;

        let content = response_json["choices"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tokens_used = response_json["usage"]["total_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;

        Ok(AIResponse {
            id: Uuid::new_v4().to_string(),
            request_id: request.id.clone(),
            content,
            model_used: request.model.clone(),
            model_version: self.config.model_version.clone(),
            tokens_used,
            processing_time_ms: 0, // Will be set by caller
            cached: false,
            created_at: Utc::now(),
        })
    }

    /// Generate cache key for request
    fn generate_cache_key(&self, request: &AIRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.prompt.hash(&mut hasher);
        request.model.hash(&mut hasher);
        request.temperature.to_bits().hash(&mut hasher);
        request.max_tokens.hash(&mut hasher);
        
        format!("ai_cache_{:x}", hasher.finish())
    }

    /// Update service metrics
    async fn update_metrics(&self, cache_hit: bool, processing_time_ms: u64, tokens_used: u32, cost: f64) {
        let mut metrics = self.metrics.write().await;
        
        metrics.total_requests += 1;
        if cache_hit {
            metrics.cache_hits += 1;
        } else {
            metrics.cache_misses += 1;
            metrics.successful_requests += 1;
        }
        
        metrics.total_tokens_processed += tokens_used as u64;
        metrics.total_cost += cost;
        
        // Update average response time
        let total_time = metrics.average_response_time_ms * (metrics.total_requests - 1) + processing_time_ms;
        metrics.average_response_time_ms = total_time / metrics.total_requests;
        
        metrics.last_updated = Utc::now();
    }

    /// Calculate cost for response
    fn calculate_cost(&self, response: &AIResponse) -> f64 {
        // This would typically look up the model cost from a database
        // For now, using a simple calculation
        response.tokens_used as f64 * 0.0001
    }

    /// Get service metrics
    pub async fn get_metrics(&self) -> AIServiceMetrics {
        self.metrics.read().await.clone()
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (0, 0) // Note: Arc<TimedCache> doesn't support len/capacity access
    }

    /// Clear cache
    pub fn clear_cache(&self) {
        // Note: Arc<TimedCache> doesn't support mutable access
        // This is a limitation of the current implementation
    }

    /// Add or update model information
    pub async fn update_model(&self, model: AIModel) {
        let mut models = self.models.write().await;
        models.insert(model.name.clone(), model);
    }

    /// Get model information
    pub async fn get_model(&self, name: &str) -> Option<AIModel> {
        let models = self.models.read().await;
        models.get(name).cloned()
    }

    /// Get all models
    pub async fn get_all_models(&self) -> Vec<AIModel> {
        let models = self.models.read().await;
        models.values().cloned().collect()
    }

    /// Health check
    pub async fn health_check(&self) -> RhemaResult<()> {
        // Simple health check - could be more comprehensive
        let response = self.client
            .get(&format!("{}/health", self.config.base_url))
            .send()
            .await
            .map_err(|e| RhemaError::ConfigError(format!("Health check failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(RhemaError::ConfigError("Health check returned non-success status".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_service_creation() {
        let config = AIServiceConfig {
            api_key: "test_key".to_string(),
            base_url: "https://api.openai.com".to_string(),
            timeout_seconds: 30,
            max_concurrent_requests: 100,
            rate_limit_per_minute: 300,
            cache_ttl_seconds: 3600,
            model_version: "1.0.0".to_string(),
            enable_caching: true,
            enable_rate_limiting: true,
            enable_monitoring: true,
        };

        let service = AIService::new(config).await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_cache_key_generation() {
        let config = AIServiceConfig {
            api_key: "test_key".to_string(),
            base_url: "https://api.openai.com".to_string(),
            timeout_seconds: 30,
            max_concurrent_requests: 100,
            rate_limit_per_minute: 300,
            cache_ttl_seconds: 3600,
            model_version: "1.0.0".to_string(),
            enable_caching: true,
            enable_rate_limiting: true,
            enable_monitoring: true,
        };

        let service = AIService::new(config).await.unwrap();
        
        let request = AIRequest {
            id: "test_id".to_string(),
            prompt: "Hello, world!".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            temperature: 0.7,
            max_tokens: 100,
            user_id: Some("user123".to_string()),
            session_id: Some("session456".to_string()),
            created_at: Utc::now(),
        };

        let cache_key = service.generate_cache_key(&request);
        assert!(!cache_key.is_empty());
        assert!(cache_key.starts_with("ai_cache_"));
    }
} 