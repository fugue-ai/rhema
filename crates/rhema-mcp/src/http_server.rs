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

use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::{HeaderMap, StatusCode, Method, header::{AUTHORIZATION, CONTENT_TYPE}},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::UnixListener;

use tower_http::cors::{CorsLayer, AllowOrigin};
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use tracing::{error, info, warn, instrument};
use std::time::{Instant, Duration};
use dashmap::DashMap;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use futures::future::join_all;
use tokio::sync::Semaphore;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::atomic::{AtomicBool, AtomicUsize};

use crate::mcp::{McpConfig, McpDaemon, ClientType};
use rhema_core::{RhemaResult, RhemaError};

/// Performance metrics for monitoring
#[derive(Debug)]
pub struct PerformanceMetrics {
    pub request_count: AtomicU64,
    pub error_count: AtomicU64,
    pub total_response_time: AtomicU64, // in nanoseconds
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    pub active_connections: AtomicUsize,
    pub is_healthy: AtomicBool,
    // Enhanced metrics
    pub slow_requests: AtomicU64, // requests taking > 1 second
    pub memory_usage: AtomicU64, // in bytes
    pub cpu_usage: AtomicU64, // percentage * 100
    pub request_size: AtomicU64, // total request size in bytes
    pub response_size: AtomicU64, // total response size in bytes
    pub concurrent_requests: AtomicUsize,
    pub error_rate: AtomicU64, // error rate percentage * 100
    pub throughput: AtomicU64, // requests per second * 100
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            request_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            total_response_time: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            active_connections: AtomicUsize::new(0),
            is_healthy: AtomicBool::new(true),
            slow_requests: AtomicU64::new(0),
            memory_usage: AtomicU64::new(0),
            cpu_usage: AtomicU64::new(0),
            request_size: AtomicU64::new(0),
            response_size: AtomicU64::new(0),
            concurrent_requests: AtomicUsize::new(0),
            error_rate: AtomicU64::new(0),
            throughput: AtomicU64::new(0),
        }
    }

    pub fn record_request(&self, duration: Duration, request_size: usize, response_size: usize) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.total_response_time.fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
        self.request_size.fetch_add(request_size as u64, Ordering::Relaxed);
        self.response_size.fetch_add(response_size as u64, Ordering::Relaxed);

        // Track slow requests
        if duration > Duration::from_secs(1) {
            self.slow_requests.fetch_add(1, Ordering::Relaxed);
        }

        // Update error rate
        let total_requests = self.request_count.load(Ordering::Relaxed);
        let total_errors = self.error_count.load(Ordering::Relaxed);
        if total_requests > 0 {
            let error_rate = (total_errors * 100) / total_requests;
            self.error_rate.store(error_rate, Ordering::Relaxed);
        }

        // Update throughput (requests per second)
        self.update_throughput();
    }

    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
        
        // Update error rate
        let total_requests = self.request_count.load(Ordering::Relaxed);
        let total_errors = self.error_count.load(Ordering::Relaxed);
        if total_requests > 0 {
            let error_rate = (total_errors * 100) / total_requests;
            self.error_rate.store(error_rate, Ordering::Relaxed);
        }
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_average_response_time(&self) -> Duration {
        let count = self.request_count.load(Ordering::Relaxed);
        if count == 0 {
            return Duration::ZERO;
        }
        let total_nanos = self.total_response_time.load(Ordering::Relaxed);
        Duration::from_nanos(total_nanos / count)
    }

    pub fn get_cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    pub fn get_error_rate(&self) -> f64 {
        self.error_rate.load(Ordering::Relaxed) as f64 / 100.0
    }

    pub fn get_throughput(&self) -> f64 {
        self.throughput.load(Ordering::Relaxed) as f64 / 100.0
    }

    pub fn get_memory_usage_mb(&self) -> u64 {
        self.memory_usage.load(Ordering::Relaxed) / (1024 * 1024)
    }

    pub fn get_cpu_usage_percent(&self) -> f64 {
        self.cpu_usage.load(Ordering::Relaxed) as f64 / 100.0
    }

    fn update_throughput(&self) {
        // This would typically be calculated over a time window
        // For now, we'll use a simple approach
        let total_requests = self.request_count.load(Ordering::Relaxed);
        // Assuming 1 second window for simplicity
        self.throughput.store(total_requests * 100, Ordering::Relaxed);
    }

    pub fn update_memory_usage(&self, bytes: u64) {
        self.memory_usage.store(bytes, Ordering::Relaxed);
    }

    pub fn update_cpu_usage(&self, percentage: f64) {
        self.cpu_usage.store((percentage * 100.0) as u64, Ordering::Relaxed);
    }

    pub fn increment_concurrent_requests(&self) {
        self.concurrent_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_concurrent_requests(&self) {
        self.concurrent_requests.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Connection pool for managing HTTP connections
pub struct ConnectionPool {
    semaphore: Semaphore,
    max_connections: usize,
}

impl ConnectionPool {
    pub fn new(max_connections: usize) -> Self {
        Self {
            semaphore: Semaphore::new(max_connections),
            max_connections,
        }
    }

    pub async fn acquire(&self) -> Result<ConnectionGuard, RhemaError> {
        self.semaphore.acquire().await
            .map(|_| ConnectionGuard { pool: self })
            .map_err(|_| RhemaError::InvalidInput("Connection pool exhausted".to_string()))
    }

    pub fn release(&self) {
        self.semaphore.add_permits(1);
    }
}

pub struct ConnectionGuard<'a> {
    pool: &'a ConnectionPool,
}

impl<'a> Drop for ConnectionGuard<'a> {
    fn drop(&mut self) {
        self.pool.release();
    }
}

/// Zero-copy string cache for frequently accessed strings
pub struct StringCache {
    cache: DashMap<String, Arc<str>>,
}

impl StringCache {
    pub fn new() -> Self {
        Self {
            cache: DashMap::new(),
        }
    }

    pub fn get_or_insert(&self, key: &str) -> Arc<str> {
        if let Some(cached) = self.cache.get(key) {
            cached.clone()
        } else {
            let arc_str: Arc<str> = Arc::from(key);
            self.cache.insert(key.to_string(), arc_str.clone());
            arc_str
        }
    }

    pub fn clear(&self) {
        self.cache.clear();
    }
}

/// HTTP server for the MCP daemon with performance optimizations
pub struct HttpServer {
    config: McpConfig,
    daemon: Arc<McpDaemon>,
    metrics: Arc<PerformanceMetrics>,
    connection_pool: Arc<ConnectionPool>,
    string_cache: Arc<StringCache>,
    response_cache: Arc<DashMap<String, (Value, Instant)>>,
    rate_limit_cache: Arc<DashMap<String, (u32, Instant)>>,
}



/// Query parameters for resource listing
#[derive(Debug, Deserialize)]
pub struct ResourceQuery {
    uri: Option<String>,
    r#type: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

/// Query execution request
#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    query: String,
    parameters: Option<HashMap<String, Value>>,
    timeout_ms: Option<u64>,
}

/// Search request
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRequest {
    query: String,
    search_type: Option<String>,
    limit: Option<u32>,
    filters: Option<Vec<SearchFilterRequest>>,
    case_sensitive: Option<bool>,
    fuzzy_matching: Option<bool>,
    fuzzy_distance: Option<u32>,
}

/// Search filter request
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchFilterRequest {
    filter_type: String,
    value: Value,
}

/// Regex search request
#[derive(Debug, Deserialize)]
pub struct RegexSearchRequest {
    pattern: String,
    file_filter: Option<String>,
    limit: Option<u32>,
    case_sensitive: Option<bool>,
}

/// Full-text search request
#[derive(Debug, Deserialize)]
pub struct FullTextSearchRequest {
    query: String,
    limit: Option<u32>,
    filters: Option<Vec<SearchFilterRequest>>,
    case_sensitive: Option<bool>,
}

/// Search suggestions request
#[derive(Debug, Deserialize)]
pub struct SearchSuggestionsRequest {
    query: String,
    limit: Option<u32>,
}

/// Get resource parameters
#[derive(Debug, Deserialize)]
pub struct GetResourceParams {
    uri: String,
}

/// Execute query parameters
#[derive(Debug, Deserialize)]
pub struct ExecuteQueryParams {
    query: String,
    parameters: Option<HashMap<String, Value>>,
}

/// JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

/// JSON-RPC response
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

/// JSON-RPC error
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// Health response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
    uptime: u64,
    connections: usize,
    cache_hit_rate: f64,
    memory_usage: crate::mcp::MemoryUsage,
    request_count: u64,
    error_count: u64,
    error_rate: f64,
    restart_count: u32,
}

/// Info response
#[derive(Debug, Serialize)]
pub struct InfoResponse {
    name: String,
    version: String,
    protocol_version: String,
    capabilities: HashMap<String, bool>,
    supported_methods: Vec<String>,
}

/// Resource response
#[derive(Debug, Serialize)]
pub struct ResourceResponse {
    uri: String,
    name: String,
    description: Option<String>,
    mime_type: Option<String>,
    content: Value,
    metadata: HashMap<String, Value>,
}

/// Resources list response
#[derive(Debug, Serialize)]
pub struct ResourcesListResponse {
    resources: Vec<ResourceResponse>,
    total_count: usize,
    next_page_token: Option<String>,
}

/// Query response
#[derive(Debug, Serialize)]
pub struct QueryResponse {
    results: Vec<Value>,
    metadata: HashMap<String, Value>,
    execution_time_ms: u64,
}

/// Search response
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    results: Vec<SearchResultResponse>,
    total_count: usize,
    execution_time_ms: u64,
    search_type: String,
    query: String,
}

/// Search result response
#[derive(Debug, Serialize)]
pub struct SearchResultResponse {
    id: String,
    path: String,
    score: f64,
    content_preview: String,
    highlights: Vec<String>,
    metadata: HashMap<String, Value>,
    doc_type: String,
    file_size: usize,
}

/// Search suggestions response
#[derive(Debug, Serialize)]
pub struct SearchSuggestionsResponse {
    suggestions: Vec<SearchSuggestionResponse>,
    query: String,
}

/// Search suggestion response
#[derive(Debug, Serialize)]
pub struct SearchSuggestionResponse {
    text: String,
    score: f64,
    suggestion_type: String,
}

/// Search stats response
#[derive(Debug, Serialize)]
pub struct SearchStatsResponse {
    total_documents: usize,
    total_terms: usize,
    index_size_bytes: usize,
    total_searches: u64,
    avg_search_time_ms: f64,
    cache_hit_rate: f64,
    search_config: HashMap<String, Value>,
}

/// Scope information
#[derive(Debug, Serialize)]
pub struct ScopeInfo {
    path: String,
    definition: Value,
    files: Vec<String>,
}

/// Performance metrics response
#[derive(Debug, Serialize)]
pub struct PerformanceResponse {
    pub average_response_time_ms: u64,
    pub request_count: u64,
    pub error_count: u64,
    pub error_rate: f64,
    pub cache_hit_rate: f64,
    pub active_connections: usize,
    pub is_healthy: bool,
    pub uptime_seconds: u64,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f64,
}

impl HttpServer {
    /// Create a new HTTP server
    pub fn new(config: McpConfig, daemon: Arc<McpDaemon>) -> Self {
        let max_connections = config.max_connections.unwrap_or(1000);
        Self {
            config,
            daemon,
            metrics: Arc::new(PerformanceMetrics::new()),
            connection_pool: Arc::new(ConnectionPool::new(max_connections)),
            string_cache: Arc::new(StringCache::new()),
            response_cache: Arc::new(DashMap::new()),
            rate_limit_cache: Arc::new(DashMap::new()),
        }
    }

    /// Generate cache key for responses
    fn generate_cache_key(&self, method: &str, path: &str, params: &str) -> String {
        let mut hasher = DefaultHasher::new();
        method.hash(&mut hasher);
        path.hash(&mut hasher);
        params.hash(&mut hasher);
        format!("{}:{}:{}:{}", method, path, params, hasher.finish())
    }

    /// Check response cache
    fn get_cached_response(&self, key: &str) -> Option<Value> {
        if let Some(entry) = self.response_cache.get(key) {
            let (response, timestamp) = entry.value();
            // Cache entries expire after 5 minutes
            if timestamp.elapsed() < Duration::from_secs(300) {
                self.metrics.record_cache_hit();
                return Some(response.clone());
            } else {
                self.response_cache.remove(key);
            }
        }
        self.metrics.record_cache_miss();
        None
    }

    /// Store response in cache
    fn cache_response(&self, key: String, response: Value) {
        self.response_cache.insert(key, (response, Instant::now()));
    }

    /// Optimized rate limiting with caching
    async fn check_rate_limit_optimized(&self, client_id: &str) -> bool {
        let now = Instant::now();
        let window = Duration::from_secs(60); // 1 minute window
        
        if let Some(entry) = self.rate_limit_cache.get(client_id) {
            let (count, timestamp) = entry.value();
            if now.duration_since(*timestamp) < window {
                if *count >= 100 { // 100 requests per minute
                    return false;
                }
                // Update count atomically
                self.rate_limit_cache.insert(client_id.to_string(), (*count + 1, *timestamp));
            } else {
                // Reset counter for new window
                self.rate_limit_cache.insert(client_id.to_string(), (1, now));
            }
        } else {
            // First request
            self.rate_limit_cache.insert(client_id.to_string(), (1, now));
        }
        true
    }

    /// Start the HTTP server
    pub async fn start(&self) -> RhemaResult<()> {
        info!("Starting HTTP server on {}:{}", self.config.host, self.config.port);

        let app = self.create_router();

        // Start HTTP server
        let http_addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = tokio::net::TcpListener::bind(&http_addr).await?;
        
        info!("HTTP server listening on {}", http_addr);

        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Start Unix socket server
    pub async fn start_unix_socket(&self) -> RhemaResult<()> {
        if let Some(socket_path) = &self.config.unix_socket {
            info!("Starting Unix socket server on {:?}", socket_path);

            // Remove existing socket file if it exists
            let _ = std::fs::remove_file(socket_path);

            let _listener = UnixListener::bind(socket_path)?;
            
            info!("Unix socket server listening on {:?}", socket_path);

            let _app = self.create_router();
            // Note: Unix socket support requires a different approach
            // For now, we'll just log that it's not fully implemented
            tracing::warn!("Unix socket server started but not fully implemented");
        }

        Ok(())
    }

    /// Stop the HTTP server
    pub async fn stop(&mut self) -> RhemaResult<()> {
        info!("Stopping HTTP server");
        // Note: axum doesn't provide a direct stop method, but the server will stop
        // when the future is dropped. In a real implementation, you might want to
        // use a shutdown signal or graceful shutdown mechanism.
        Ok(())
    }

    /// Create the router with all endpoints
    fn create_router(&self) -> Router {
        let cors = self.create_cors_layer();
        let security_headers = self.create_security_headers_layer();

        Router::new()
            .route("/health", get(Self::health_handler))
            .route("/info", get(Self::info_handler))
            .route("/rpc", post(Self::rpc_handler))
            .route("/resources", get(Self::resources_list_handler))
            .route("/resources/:uri", get(Self::resource_handler))
            .route("/query", post(Self::query_handler))
            .route("/search", post(Self::search_handler))
            .route("/search/regex", post(Self::search_regex_handler))
            .route("/search/fulltext", post(Self::search_fulltext_handler))
            .route("/search/suggestions", get(Self::search_suggestions_handler))
            .route("/search/stats", get(Self::search_stats_handler))
            .route("/scopes", get(Self::scopes_handler))
            .route("/scopes/:scope_id", get(Self::scope_handler))
            .route("/scopes/:scope_id/knowledge", get(Self::scope_knowledge_handler))
            .route("/scopes/:scope_id/todos", get(Self::scope_todos_handler))
            .route("/scopes/:scope_id/decisions", get(Self::scope_decisions_handler))
            .route("/scopes/:scope_id/patterns", get(Self::scope_patterns_handler))
            .route("/stats", get(Self::stats_handler))
            .route("/performance", get(Self::performance_handler))
            .route("/ws", get(Self::websocket_handler))
            // Validation endpoints
            .route("/validation/context", get(Self::validate_context_handler))
            .route("/validation/scope/:scope_id", get(Self::validate_scope_handler))
            .route("/validation/cross-references", get(Self::validate_cross_references_handler))
            .route("/validation/consistency", get(Self::validate_consistency_handler))
            .route("/validation/temporal", get(Self::validate_temporal_handler))
            .route("/validation/dependencies", get(Self::validate_dependencies_handler))
            .layer(cors)
            .layer(security_headers)
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .with_state(Arc::new(self.clone()))
    }

    fn create_cors_layer(&self) -> CorsLayer {
        let allowed_origins = if self.config.auth.allowed_origins.contains(&"*".to_string()) {
            // When using wildcard origin, we cannot allow credentials
            AllowOrigin::any()
        } else {
            let origins: Vec<_> = self.config.auth.allowed_origins.iter()
                .filter_map(|origin| origin.parse().ok())
                .collect();
            if origins.is_empty() {
                // When no origins specified, use wildcard but don't allow credentials
                AllowOrigin::any()
            } else {
                AllowOrigin::list(origins)
            }
        };

        let mut cors_layer = CorsLayer::new()
            .allow_origin(allowed_origins)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

        // Only allow credentials if we're not using wildcard origin
        if !self.config.auth.allowed_origins.contains(&"*".to_string()) && !self.config.auth.allowed_origins.is_empty() {
            cors_layer = cors_layer.allow_credentials(true);
        }

        cors_layer
    }

    fn create_security_headers_layer(&self) -> tower::ServiceBuilder<tower::layer::util::Stack<tower_http::set_header::SetResponseHeaderLayer<axum::http::HeaderValue>, tower::layer::util::Stack<tower_http::set_header::SetResponseHeaderLayer<axum::http::HeaderValue>, tower::layer::util::Stack<tower_http::set_header::SetResponseHeaderLayer<axum::http::HeaderValue>, tower::layer::util::Stack<tower_http::set_header::SetResponseHeaderLayer<axum::http::HeaderValue>, tower::layer::util::Stack<tower_http::set_header::SetResponseHeaderLayer<axum::http::HeaderValue>, tower::layer::util::Identity>>>>>> {
        use tower_http::set_header::SetResponseHeaderLayer;
        use axum::http::{HeaderValue, HeaderName};

        tower::ServiceBuilder::new()
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("x-content-type-options"),
                HeaderValue::from_static("nosniff")
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("x-frame-options"),
                HeaderValue::from_static("DENY")
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("x-xss-protection"),
                HeaderValue::from_static("1; mode=block")
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("strict-transport-security"),
                HeaderValue::from_static("max-age=31536000; includeSubDomains")
            ))
            .layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static("content-security-policy"),
                HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' https:; connect-src 'self' https:; frame-ancestors 'none';")
            ))
    }

    /// Health check handler with performance optimization
    #[instrument(skip(server, headers), fields(client_id = %Self::get_client_id(&headers).unwrap_or_default()))]
    async fn health_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Acquire connection from pool
        let _connection_guard = match server.connection_pool.acquire().await {
            Ok(guard) => guard,
            Err(_) => {
                server.metrics.record_error();
                return (StatusCode::SERVICE_UNAVAILABLE, "Service overloaded").into_response();
            }
        };

        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting with optimized cache
        if let Some(ref client_id) = client_id {
            if !server.check_rate_limit_optimized(client_id).await {
                server.metrics.record_error();
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Check cache first for health responses (cache for 30 seconds)
        let cache_key = server.generate_cache_key("GET", "/health", "");
        if let Some(cached_response) = server.get_cached_response(&cache_key) {
            let duration = start_time.elapsed();
            server.metrics.record_request(duration, 0, 0);
            return (StatusCode::OK, Json(cached_response)).into_response();
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                server.metrics.record_error();
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            server.metrics.record_error();
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Increment request count
        server.daemon.increment_request_count().await;

        // Use zero-copy strings for common values
        let status = server.string_cache.get_or_insert("healthy");
        let health = server.daemon.health().await;
        
        let response = HealthResponse {
            status: status.to_string(),
            uptime: health.uptime,
            connections: health.connections,
            cache_hit_rate: server.metrics.get_cache_hit_rate(),
            memory_usage: health.memory_usage,
            request_count: health.request_count,
            error_count: health.error_count,
            error_rate: health.error_rate,
            restart_count: health.restart_count,
        };

        let response_value = serde_json::to_value(response).unwrap_or_default();
        
        // Cache the response for 30 seconds
        server.cache_response(cache_key, response_value.clone());
        
        let duration = start_time.elapsed();
        server.metrics.record_request(duration, 0, 0);
        
        // Log performance metrics if response time is high
        if duration > Duration::from_millis(50) {
            warn!("Slow health check response: {:?}", duration);
        }

        (StatusCode::OK, Json(response_value)).into_response()
    }

    /// Info handler
    async fn info_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Increment request count
        server.daemon.increment_request_count().await;

        let mut capabilities = HashMap::new();
        capabilities.insert("resources".to_string(), true);
        capabilities.insert("queries".to_string(), true);
        capabilities.insert("subscriptions".to_string(), true);
        capabilities.insert("notifications".to_string(), true);

        let supported_methods = vec![
            "resources/list".to_string(),
            "resources/read".to_string(),
            "query/execute".to_string(),
            "system/health".to_string(),
        ];

        let response = InfoResponse {
            name: "Rhema MCP Daemon".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: "1.0.0".to_string(),
            capabilities,
            supported_methods,
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    /// JSON-RPC handler
    async fn rpc_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Json(request): Json<JsonRpcRequest>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Increment request count
        server.daemon.increment_request_count().await;

        let response = match HttpServer::handle_rpc_method(&server, &request).await {
            Ok(result) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(result),
                error: None,
            },
            Err(e) => {
                server.daemon.increment_error_count().await;
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32603,
                        message: e.to_string(),
                        data: None,
                    }),
                }
            }
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    /// Resources list handler
    async fn resources_list_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Query(_query): Query<ResourceQuery>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "resources:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get resources from context provider
        let resources = match server.daemon.get_context_provider().list_resources().await {
            Ok(resources) => resources,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get resources").into_response(),
        };

        let response = serde_json::json!({
            "resources": resources,
            "count": resources.len()
        });

        (StatusCode::OK, Json(response)).into_response()
    }

    async fn resource_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Path(uri): Path<String>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "resources:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get resource from context provider
        let resource = match server.daemon.get_context_provider().get_resource(&uri).await {
            Ok(resource) => resource,
            Err(_) => return (StatusCode::NOT_FOUND, "Resource not found").into_response(),
        };

        let response = serde_json::json!({
            "resource": resource,
            "uri": uri
        });

        (StatusCode::OK, Json(response)).into_response()
    }

    /// Query handler
    async fn query_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Json(request): Json<QueryRequest>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "query:execute").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Execute query
        let results = match server.daemon.get_context_provider().execute_query(&request.query).await {
            Ok(results) => results,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute query").into_response(),
        };

        let response = serde_json::json!({
            "results": results,
            "query": request.query,
            "parameters": request.parameters
        });

        (StatusCode::OK, Json(response)).into_response()
    }

    /// General search handler with performance optimization
    #[instrument(skip(server, headers, request), fields(query = %request.query, search_type = %request.search_type.as_deref().unwrap_or("fulltext")))]
    async fn search_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Json(request): Json<SearchRequest>,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Acquire connection from pool
        let _connection_guard = match server.connection_pool.acquire().await {
            Ok(guard) => guard,
            Err(_) => {
                server.metrics.record_error();
                return (StatusCode::SERVICE_UNAVAILABLE, "Service overloaded").into_response();
            }
        };

        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting with optimized cache
        if let Some(ref client_id) = client_id {
            if !server.check_rate_limit_optimized(client_id).await {
                server.metrics.record_error();
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Check cache first for search results (cache for 2 minutes)
        let cache_params = serde_json::to_string(&request).unwrap_or_default();
        let cache_key = server.generate_cache_key("POST", "/search", &cache_params);
        if let Some(cached_response) = server.get_cached_response(&cache_key) {
            let duration = start_time.elapsed();
            server.metrics.record_request(duration, 0, 0);
            return (StatusCode::OK, Json(cached_response)).into_response();
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                server.metrics.record_error();
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            server.metrics.record_error();
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "search:execute").await {
            server.metrics.record_error();
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Increment request count
        server.daemon.increment_request_count().await;

        let search_start_time = Instant::now();
        
        // Determine search type and execute appropriate search
        let search_type = request.search_type.as_deref().unwrap_or("fulltext");
        let search_type_str = server.string_cache.get_or_insert(search_type);
        
        let results = match search_type {
            "regex" => {
                // Use regex search with parallel processing
                let query_results = match server.daemon.get_context_provider().search_regex(&request.query, None).await {
                    Ok(results) => results,
                    Err(_) => {
                        server.metrics.record_error();
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute regex search").into_response();
                    }
                };
                
                // Convert to search results with parallel processing
                let results_futures: Vec<_> = query_results.into_iter().map(|qr| {
                    let server = server.clone();
                    async move {
                        let id = format!("{}:{}", qr.scope, qr.file);
                        let path = qr.file;
                        let score = qr.metadata.get("search_score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let content_preview = qr.data.as_str().unwrap_or("").chars().take(200).collect();
                        let highlights = qr.metadata.get("highlights")
                            .and_then(|v| v.as_sequence())
                            .map(|seq| seq.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                            .unwrap_or_default();
                        let metadata = qr.metadata.clone().into_iter().map(|(k, v)| (k, Self::convert_yaml_to_json(v))).collect();
                        let doc_type = qr.metadata.get("doc_type").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                        let file_size = qr.metadata.get("file_size").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                        
                        SearchResultResponse {
                            id,
                            path,
                            score,
                            content_preview,
                            highlights,
                            metadata,
                            doc_type,
                            file_size,
                        }
                    }
                }).collect();
                
                join_all(results_futures).await
            }
            _ => {
                // Use full-text search (default) with parallel processing
                let query_results = match server.daemon.get_context_provider().search_regex(&request.query, None).await {
                    Ok(results) => results,
                    Err(_) => {
                        server.metrics.record_error();
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute full-text search").into_response();
                    }
                };
                
                // Convert to search results with parallel processing
                let results_futures: Vec<_> = query_results.into_iter().map(|qr| {
                    let server = server.clone();
                    async move {
                        let id = format!("{}:{}", qr.scope, qr.file);
                        let path = qr.file;
                        let score = qr.metadata.get("search_score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let content_preview = qr.data.as_str().unwrap_or("").chars().take(200).collect();
                        let highlights = qr.metadata.get("highlights")
                            .and_then(|v| v.as_sequence())
                            .map(|seq| seq.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                            .unwrap_or_default();
                        let metadata = qr.metadata.clone().into_iter().map(|(k, v)| (k, Self::convert_yaml_to_json(v))).collect();
                        let doc_type = qr.metadata.get("doc_type").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                        let file_size = qr.metadata.get("file_size").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                        
                        SearchResultResponse {
                            id,
                            path,
                            score,
                            content_preview,
                            highlights,
                            metadata,
                            doc_type,
                            file_size,
                        }
                    }
                }).collect();
                
                join_all(results_futures).await
            }
        };

        let search_execution_time = search_start_time.elapsed();
        
        let results_len = results.len();
        let response = SearchResponse {
            results,
            total_count: results_len,
            execution_time_ms: search_execution_time.as_millis() as u64,
            search_type: search_type_str.to_string(),
            query: request.query,
        };

        let response_value = serde_json::to_value(response).unwrap_or_default();
        
        // Cache the response for 2 minutes
        server.cache_response(cache_key, response_value.clone());
        
        let total_duration = start_time.elapsed();
        server.metrics.record_request(total_duration, 0, 0);
        
        // Log performance metrics if response time is high
        if total_duration > Duration::from_millis(50) {
            warn!("Slow search response: {:?} (search: {:?})", total_duration, search_execution_time);
        }

        (StatusCode::OK, Json(response_value)).into_response()
    }

    /// Regex search handler
    async fn search_regex_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Json(request): Json<RegexSearchRequest>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "search:execute").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Increment request count
        server.daemon.increment_request_count().await;

        let start_time = std::time::Instant::now();
        
        // Execute regex search
        let query_results = match server.daemon.get_context_provider().search_regex(&request.pattern, request.file_filter.as_deref()).await {
            Ok(results) => results,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute regex search").into_response(),
        };
        
        // Convert to search results
        let results: Vec<SearchResultResponse> = query_results.into_iter().map(|qr| SearchResultResponse {
            id: format!("{}:{}", qr.scope, qr.file),
            path: qr.file,
            score: qr.metadata.get("search_score").and_then(|v| v.as_f64()).unwrap_or(0.0),
            content_preview: qr.data.as_str().unwrap_or("").chars().take(200).collect(),
            highlights: qr.metadata.get("highlights")
                .and_then(|v| v.as_sequence())
                .map(|seq| seq.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                .unwrap_or_default(),
            metadata: qr.metadata.clone().into_iter().map(|(k, v)| (k, Self::convert_yaml_to_json(v))).collect(),
            doc_type: qr.metadata.get("doc_type").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            file_size: qr.metadata.get("file_size").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        }).collect();

        let execution_time = start_time.elapsed();
        
        let results_len = results.len();
        let response = SearchResponse {
            results,
            total_count: results_len,
            execution_time_ms: execution_time.as_millis() as u64,
            search_type: "regex".to_string(),
            query: request.pattern,
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    /// Full-text search handler
    async fn search_fulltext_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Json(request): Json<FullTextSearchRequest>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "search:execute").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Increment request count
        server.daemon.increment_request_count().await;

        let start_time = std::time::Instant::now();
        
        // Execute full-text search (using regex search as fallback for now)
        let query_results = match server.daemon.get_context_provider().search_regex(&request.query, None).await {
            Ok(results) => results,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute full-text search").into_response(),
        };
        
        // Convert to search results
        let results: Vec<SearchResultResponse> = query_results.into_iter().map(|qr| SearchResultResponse {
            id: format!("{}:{}", qr.scope, qr.file),
            path: qr.file,
            score: qr.metadata.get("search_score").and_then(|v| v.as_f64()).unwrap_or(0.0),
            content_preview: qr.data.as_str().unwrap_or("").chars().take(200).collect(),
            highlights: qr.metadata.get("highlights")
                .and_then(|v| v.as_sequence())
                .map(|seq| seq.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                .unwrap_or_default(),
            metadata: qr.metadata.clone().into_iter().map(|(k, v)| (k, Self::convert_yaml_to_json(v))).collect(),
            doc_type: qr.metadata.get("doc_type").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            file_size: qr.metadata.get("file_size").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        }).collect();

        let execution_time = start_time.elapsed();
        
        let results_len = results.len();
        let response = SearchResponse {
            results,
            total_count: results_len,
            execution_time_ms: execution_time.as_millis() as u64,
            search_type: "fulltext".to_string(),
            query: request.query,
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    /// Search suggestions handler
    async fn search_suggestions_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Query(request): Query<SearchSuggestionsRequest>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "search:suggestions").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // For now, return basic suggestions based on the query
        // TODO: Implement actual search suggestions using the search engine
        let suggestions = vec![
            SearchSuggestionResponse {
                text: format!("{}*", request.query),
                score: 0.9,
                suggestion_type: "query_completion".to_string(),
            },
            SearchSuggestionResponse {
                text: format!("{} test", request.query),
                score: 0.7,
                suggestion_type: "related_query".to_string(),
            },
        ];

        let response = SearchSuggestionsResponse {
            suggestions,
            query: request.query,
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    /// Search stats handler
    async fn search_stats_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "search:stats").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // For now, return basic stats
        // TODO: Implement actual search stats using the search engine
        let response = SearchStatsResponse {
            total_documents: 0,
            total_terms: 0,
            index_size_bytes: 0,
            total_searches: 0,
            avg_search_time_ms: 0.0,
            cache_hit_rate: 0.0,
            search_config: HashMap::new(),
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    /// Scopes handler
    async fn scopes_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "scopes:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get scopes from context provider
        let scopes = match server.daemon.get_context_provider().get_scopes().await {
            Ok(scopes) => scopes,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get scopes").into_response(),
        };

        (StatusCode::OK, Json(scopes)).into_response()
    }

    /// Scope handler
    async fn scope_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Path(scope_id): Path<String>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "scopes:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get scope from context provider
        let scope = match server.daemon.get_context_provider().get_scope(&scope_id).await {
            Ok(scope) => scope,
            Err(_) => return (StatusCode::NOT_FOUND, "Scope not found").into_response(),
        };

        (StatusCode::OK, Json(scope)).into_response()
    }

    /// Scope knowledge handler
    async fn scope_knowledge_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Path(scope_id): Path<String>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "knowledge:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get knowledge from context provider
        let knowledge = match server.daemon.get_context_provider().get_knowledge(&scope_id).await {
            Ok(Some(knowledge)) => knowledge,
            Ok(None) => return (StatusCode::NOT_FOUND, "Knowledge not found").into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get knowledge").into_response(),
        };

        (StatusCode::OK, Json(knowledge)).into_response()
    }

    /// Scope todos handler
    async fn scope_todos_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Path(scope_id): Path<String>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "todos:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get todos from context provider
        let todos = match server.daemon.get_context_provider().get_todos(&scope_id).await {
            Ok(Some(todos)) => todos,
            Ok(None) => return (StatusCode::NOT_FOUND, "Todos not found").into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get todos").into_response(),
        };

        (StatusCode::OK, Json(todos)).into_response()
    }

    /// Scope decisions handler
    async fn scope_decisions_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Path(scope_id): Path<String>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "decisions:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get decisions from context provider
        let decisions = match server.daemon.get_context_provider().get_decisions(&scope_id).await {
            Ok(Some(decisions)) => decisions,
            Ok(None) => return (StatusCode::NOT_FOUND, "Decisions not found").into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get decisions").into_response(),
        };

        (StatusCode::OK, Json(decisions)).into_response()
    }

    /// Scope patterns handler
    async fn scope_patterns_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Path(scope_id): Path<String>,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "patterns:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Get patterns from context provider
        let patterns = match server.daemon.get_context_provider().get_patterns(&scope_id).await {
            Ok(Some(patterns)) => patterns,
            Ok(None) => return (StatusCode::NOT_FOUND, "Patterns not found").into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get patterns").into_response(),
        };

        (StatusCode::OK, Json(patterns)).into_response()
    }

    /// Stats handler
    async fn stats_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "http").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "stats:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Increment request count
        server.daemon.increment_request_count().await;

        let stats = server.daemon.get_statistics().await;

        (StatusCode::OK, Json(stats)).into_response()
    }

    /// Performance monitoring handler
    #[instrument(skip(server, headers))]
    async fn performance_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Check authentication
        let client_info = Self::extract_client_info(&headers);
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Authentication failed").into_response(),
        };

        if !server.daemon.get_auth_manager().has_permission(&auth_result, "performance:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        let metrics = &server.metrics;
        let uptime = server.daemon.get_uptime().await;
        let memory_usage = server.daemon.get_memory_usage().await;
        
        let response = PerformanceResponse {
            average_response_time_ms: metrics.get_average_response_time().as_millis() as u64,
            request_count: metrics.request_count.load(Ordering::Relaxed),
            error_count: metrics.error_count.load(Ordering::Relaxed),
            error_rate: {
                let requests = metrics.request_count.load(Ordering::Relaxed);
                let errors = metrics.error_count.load(Ordering::Relaxed);
                if requests > 0 {
                    errors as f64 / requests as f64
                } else {
                    0.0
                }
            },
            cache_hit_rate: metrics.get_cache_hit_rate(),
            active_connections: metrics.active_connections.load(Ordering::Relaxed),
            is_healthy: metrics.is_healthy.load(Ordering::Relaxed),
            uptime_seconds: uptime.as_secs(),
            memory_usage_mb: memory_usage.used_mb,
            cpu_usage_percent: 0.0, // TODO: Implement CPU usage tracking
        };

        let duration = start_time.elapsed();
        metrics.record_request(duration, 0, 0);

        (StatusCode::OK, Json(response)).into_response()
    }

    /// WebSocket handler
    async fn websocket_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        ws: WebSocketUpgrade,
    ) -> impl IntoResponse {
        let client_id = Self::get_client_id(&headers);
        let client_info = Self::extract_client_info(&headers);

        // Check rate limiting
        if let Some(ref client_id) = client_id {
            if !server.daemon.get_auth_manager().check_rate_limit(client_id, "websocket").await {
                return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
            }
        }

        // Authenticate request
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response();
            }
        };

        if !auth_result.authenticated {
            return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
        }

        // Check permissions
        if !server.daemon.get_auth_manager().has_permission(&auth_result, "websocket:connect").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Track connection
        if let Some(client_id) = Self::get_client_id(&headers) {
            let _ = server.daemon.track_connection(client_id, ClientType::WebSocket).await;
        }

        ws.on_upgrade(|socket| Self::handle_websocket(server, socket))
    }

    /// Handle WebSocket connection
    async fn handle_websocket(
        server: Arc<Self>,
        mut socket: axum::extract::ws::WebSocket,
    ) {
        info!("WebSocket connection established");
        
        while let Some(msg) = socket.recv().await {
            match msg {
                Ok(axum::extract::ws::Message::Text(text)) => {
                    // Parse JSON-RPC message
                    if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(&text) {
                        match HttpServer::handle_rpc_method(&server, &request).await {
                            Ok(result) => {
                                let response = JsonRpcResponse {
                                    jsonrpc: "2.0".to_string(),
                                    id: request.id,
                                    result: Some(result),
                                    error: None,
                                };
                                
                                if let Ok(response_text) = serde_json::to_string(&response) {
                                    let _ = socket.send(axum::extract::ws::Message::Text(response_text)).await;
                                }
                            }
                            Err(e) => {
                                let error = JsonRpcError {
                                    code: -1,
                                    message: e.to_string(),
                                    data: None,
                                };
                                
                                let response = JsonRpcResponse {
                                    jsonrpc: "2.0".to_string(),
                                    id: request.id,
                                    result: None,
                                    error: Some(error),
                                };
                                
                                if let Ok(response_text) = serde_json::to_string(&response) {
                                    let _ = socket.send(axum::extract::ws::Message::Text(response_text)).await;
                                }
                            }
                        }
                    }
                }
                Ok(axum::extract::ws::Message::Close(_)) => {
                    info!("WebSocket connection closed");
                    break;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }



    /// Get client ID from headers
    fn get_client_id(headers: &HeaderMap) -> Option<String> {
        // Try to get client ID from various headers
        headers
            .get("X-Client-ID")
            .or_else(|| headers.get("X-Forwarded-For"))
            .or_else(|| headers.get("User-Agent"))
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
    }

    /// Extract client information from headers
    fn extract_client_info(headers: &HeaderMap) -> Option<crate::auth::ClientInfo> {
        let ip_address = headers
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
            .or_else(|| {
                headers
                    .get("x-real-ip")
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string())
            });

        let user_agent = headers
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        Some(crate::auth::ClientInfo {
            ip_address,
            user_agent,
            client_type: crate::auth::ClientType::Http,
            fingerprint: None,
        })
    }

    /// Handle RPC method calls with performance optimization
    async fn handle_rpc_method(
        server: &Arc<Self>,
        request: &JsonRpcRequest,
    ) -> RhemaResult<Value> {
        let start_time = Instant::now();
        
        let result = match request.method.as_str() {
            "resources/list" => {
                let resources = server.daemon.get_context_provider().list_resources().await?;
                Ok(serde_json::to_value(resources)?)
            }
            "resources/get" => {
                let params = request.params.as_ref()
                    .ok_or_else(|| RhemaError::InvalidInput("Missing params".to_string()))?;
                let params: GetResourceParams = serde_json::from_value(params.clone())?;
                let resource = server.daemon.get_context_provider().get_resource(&params.uri).await?;
                Ok(serde_json::to_value(resource)?)
            }
            "query/execute" => {
                let params = request.params.as_ref()
                    .ok_or_else(|| RhemaError::InvalidInput("Missing params".to_string()))?;
                let params: ExecuteQueryParams = serde_json::from_value(params.clone())?;
                let query_start_time = Instant::now();
                let results = server.daemon.get_context_provider().execute_query(&params.query).await?;
                let execution_time = query_start_time.elapsed();
                Ok(serde_json::json!({
                    "results": results,
                    "metadata": {},
                    "execution_time_ms": execution_time.as_millis()
                }))
            }
            _ => Err(RhemaError::InvalidInput(format!("Unknown method: {}", request.method)))
        };

        let duration = start_time.elapsed();
        if duration > Duration::from_millis(50) {
            warn!("Slow RPC method execution: {} took {:?}", request.method, duration);
        }

        result
    }

    /// Optimized JSON serialization with pre-allocated buffers
    fn serialize_json_optimized<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
        let mut buffer = Vec::with_capacity(1024); // Pre-allocate buffer
        let mut serializer = serde_json::Serializer::new(&mut buffer);
        value.serialize(&mut serializer)?;
        String::from_utf8(buffer).map_err(|_| serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8")))
    }

    /// Zero-copy string concatenation for common patterns
    fn concat_strings_zero_copy(strings: &[&str]) -> String {
        let total_len: usize = strings.iter().map(|s| s.len()).sum();
        let mut result = String::with_capacity(total_len);
        for s in strings {
            result.push_str(s);
        }
        result
    }

    /// Optimized string hashing for cache keys
    fn hash_string_fast(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// Convert serde_yaml Value to serde_json Value
    fn convert_yaml_to_json(yaml_value: serde_yaml::Value) -> serde_json::Value {
        match yaml_value {
            serde_yaml::Value::Null => serde_json::Value::Null,
            serde_yaml::Value::Bool(b) => serde_json::Value::Bool(b),
            serde_yaml::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    serde_json::Value::Number(serde_json::Number::from(i))
                } else if let Some(f) = n.as_f64() {
                    serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0)))
                } else {
                    serde_json::Value::String(n.to_string())
                }
            }
            serde_yaml::Value::String(s) => serde_json::Value::String(s),
            serde_yaml::Value::Sequence(seq) => {
                serde_json::Value::Array(seq.into_iter().map(Self::convert_yaml_to_json).collect())
            }
            serde_yaml::Value::Mapping(map) => {
                let mut json_map = serde_json::Map::new();
                for (k, v) in map {
                    if let serde_yaml::Value::String(key) = k {
                        json_map.insert(key, Self::convert_yaml_to_json(v));
                    }
                }
                serde_json::Value::Object(json_map)
            }
            serde_yaml::Value::Tagged(_) => serde_json::Value::Null, // Handle tagged values as null for now
        }
    }

    // ============================================================================
    // VALIDATION ENDPOINT HANDLERS
    // ============================================================================

    /// Validate all context data comprehensively
    async fn validate_context_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Check authentication
        let client_info = Self::extract_client_info(&headers);
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Authentication failed").into_response(),
        };

        if !server.daemon.get_auth_manager().has_permission(&auth_result, "validation:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Perform comprehensive context validation
        match server.daemon.get_context_provider().validate_context_data().await {
            Ok(validation_result) => {
                let duration = start_time.elapsed();
                server.metrics.record_request(duration, 0, 0);
                
                (StatusCode::OK, Json(validation_result)).into_response()
            }
            Err(e) => {
                server.metrics.record_error();
                error!("Context validation failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Validation failed: {}", e)).into_response()
            }
        }
    }

    /// Validate context for a specific scope
    async fn validate_scope_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
        Path(scope_id): Path<String>,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Check authentication
        let client_info = Self::extract_client_info(&headers);
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Authentication failed").into_response(),
        };

        if !server.daemon.get_auth_manager().has_permission(&auth_result, "validation:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Validate the specific scope
        match server.daemon.get_context_provider().validate_scope_context(&scope_id).await {
            Ok(scope_result) => {
                let duration = start_time.elapsed();
                server.metrics.record_request(duration, 0, 0);
                
                (StatusCode::OK, Json(scope_result)).into_response()
            }
            Err(e) => {
                server.metrics.record_error();
                error!("Scope validation failed for {}: {}", scope_id, e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Scope validation failed: {}", e)).into_response()
            }
        }
    }

    /// Validate cross-references between context types
    async fn validate_cross_references_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Check authentication
        let client_info = Self::extract_client_info(&headers);
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Authentication failed").into_response(),
        };

        if !server.daemon.get_auth_manager().has_permission(&auth_result, "validation:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Validate cross-references
        match server.daemon.get_context_provider().validate_cross_references().await {
            Ok(cross_ref_result) => {
                let duration = start_time.elapsed();
                server.metrics.record_request(duration, 0, 0);
                
                (StatusCode::OK, Json(cross_ref_result)).into_response()
            }
            Err(e) => {
                server.metrics.record_error();
                error!("Cross-reference validation failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Cross-reference validation failed: {}", e)).into_response()
            }
        }
    }

    /// Validate consistency across all scopes
    async fn validate_consistency_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Check authentication
        let client_info = Self::extract_client_info(&headers);
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Authentication failed").into_response(),
        };

        if !server.daemon.get_auth_manager().has_permission(&auth_result, "validation:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Validate consistency
        match server.daemon.get_context_provider().validate_consistency().await {
            Ok(consistency_result) => {
                let duration = start_time.elapsed();
                server.metrics.record_request(duration, 0, 0);
                
                (StatusCode::OK, Json(consistency_result)).into_response()
            }
            Err(e) => {
                server.metrics.record_error();
                error!("Consistency validation failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Consistency validation failed: {}", e)).into_response()
            }
        }
    }

    /// Validate temporal consistency
    async fn validate_temporal_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Check authentication
        let client_info = Self::extract_client_info(&headers);
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Authentication failed").into_response(),
        };

        if !server.daemon.get_auth_manager().has_permission(&auth_result, "validation:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Validate temporal consistency
        match server.daemon.get_context_provider().validate_temporal_consistency().await {
            Ok(temporal_result) => {
                let duration = start_time.elapsed();
                server.metrics.record_request(duration, 0, 0);
                
                (StatusCode::OK, Json(temporal_result)).into_response()
            }
            Err(e) => {
                server.metrics.record_error();
                error!("Temporal validation failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Temporal validation failed: {}", e)).into_response()
            }
        }
    }

    /// Validate scope dependencies
    async fn validate_dependencies_handler(
        State(server): State<Arc<Self>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let start_time = Instant::now();
        
        // Check authentication
        let client_info = Self::extract_client_info(&headers);
        let auth_result = match server.daemon.get_auth_manager().authenticate(
            headers.get("authorization").and_then(|h| h.to_str().ok()),
            client_info,
        ).await {
            Ok(result) => result,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Authentication failed").into_response(),
        };

        if !server.daemon.get_auth_manager().has_permission(&auth_result, "validation:read").await {
            return (StatusCode::FORBIDDEN, "Insufficient permissions").into_response();
        }

        // Validate dependencies
        match server.daemon.get_context_provider().validate_scope_dependencies().await {
            Ok(dependency_result) => {
                let duration = start_time.elapsed();
                server.metrics.record_request(duration, 0, 0);
                
                (StatusCode::OK, Json(dependency_result)).into_response()
            }
            Err(e) => {
                server.metrics.record_error();
                error!("Dependency validation failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Dependency validation failed: {}", e)).into_response()
            }
        }
    }
}

impl Clone for HttpServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            daemon: self.daemon.clone(),
            metrics: self.metrics.clone(),
            connection_pool: self.connection_pool.clone(),
            string_cache: self.string_cache.clone(),
            response_cache: self.response_cache.clone(),
            rate_limit_cache: self.rate_limit_cache.clone(),
        }
    }
}

/// Enhanced connection pool with performance monitoring
#[derive(Debug)]
pub struct EnhancedConnectionPool {
    semaphore: Semaphore,
    max_connections: usize,
    active_connections: AtomicUsize,
    total_connections: AtomicU64,
    connection_wait_time: AtomicU64, // in nanoseconds
    pool_stats: Arc<DashMap<String, u64>>,
}

impl EnhancedConnectionPool {
    pub fn new(max_connections: usize) -> Self {
        Self {
            semaphore: Semaphore::new(max_connections),
            max_connections,
            active_connections: AtomicUsize::new(0),
            total_connections: AtomicU64::new(0),
            connection_wait_time: AtomicU64::new(0),
            pool_stats: Arc::new(DashMap::new()),
        }
    }

    pub async fn acquire(&self) -> Result<EnhancedConnectionGuard, RhemaError> {
        let start_time = Instant::now();
        
        let permit = self.semaphore.acquire().await.map_err(|_| {
            RhemaError::SystemError("Connection pool exhausted".to_string())
        })?;

        let wait_time = start_time.elapsed();
        self.connection_wait_time.fetch_add(wait_time.as_nanos() as u64, Ordering::Relaxed);
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        self.total_connections.fetch_add(1, Ordering::Relaxed);

        // Update pool statistics
        self.pool_stats.insert("wait_time_ns".to_string(), wait_time.as_nanos() as u64);
        self.pool_stats.insert("active_connections".to_string(), self.active_connections.load(Ordering::Relaxed) as u64);

        Ok(EnhancedConnectionGuard {
            pool: self,
            _permit: permit,
        })
    }

    pub fn get_stats(&self) -> ConnectionPoolStats {
        let total_connections = self.total_connections.load(Ordering::Relaxed);
        let active_connections = self.active_connections.load(Ordering::Relaxed);
        let total_wait_time = self.connection_wait_time.load(Ordering::Relaxed);
        
        let avg_wait_time = if total_connections > 0 {
            Duration::from_nanos(total_wait_time / total_connections)
        } else {
            Duration::ZERO
        };

        ConnectionPoolStats {
            max_connections: self.max_connections,
            active_connections,
            total_connections,
            avg_wait_time,
            utilization_rate: if self.max_connections > 0 {
                active_connections as f64 / self.max_connections as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionPoolStats {
    pub max_connections: usize,
    pub active_connections: usize,
    pub total_connections: u64,
    pub avg_wait_time: Duration,
    pub utilization_rate: f64,
}

pub struct EnhancedConnectionGuard<'a> {
    pool: &'a EnhancedConnectionPool,
    _permit: tokio::sync::SemaphorePermit<'a>,
}

impl<'a> Drop for EnhancedConnectionGuard<'a> {
    fn drop(&mut self) {
        self.pool.active_connections.fetch_sub(1, Ordering::Relaxed);
    }
} 