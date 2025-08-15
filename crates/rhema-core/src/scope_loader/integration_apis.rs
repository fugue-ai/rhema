use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::sse::{Event, Sse},
    routing::{delete, get, post, put},
    Json, Router,
};
use futures::stream::{self, Stream, StreamExt};
use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

use super::advanced_analytics::{AdvancedAnalyticsManager, AnalyticsStats};
use super::background::{BackgroundProcessingManager, BackgroundTask, BackgroundTaskResult};
use super::service::ScopeLoaderService;
use super::types::*;

/// REST API configuration
#[derive(Debug, Clone)]
pub struct RestApiConfig {
    /// API host
    pub host: String,
    /// API port
    pub port: u16,
    /// Enable CORS
    pub enable_cors: bool,
    /// API rate limiting
    pub rate_limit: Option<u32>,
    /// Authentication required
    pub require_auth: bool,
    /// API version
    pub version: String,
}

impl Default for RestApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            rate_limit: Some(1000),
            require_auth: false,
            version: "v1".to_string(),
        }
    }
}

/// GraphQL API configuration
#[derive(Debug, Clone)]
pub struct GraphQLApiConfig {
    /// GraphQL endpoint
    pub endpoint: String,
    /// Enable introspection
    pub enable_introspection: bool,
    /// Enable playground
    pub enable_playground: bool,
    /// Query complexity limit
    pub query_complexity_limit: Option<u32>,
    /// Depth limit
    pub depth_limit: Option<u32>,
}

impl Default for GraphQLApiConfig {
    fn default() -> Self {
        Self {
            endpoint: "/graphql".to_string(),
            enable_introspection: true,
            enable_playground: true,
            query_complexity_limit: Some(100),
            depth_limit: Some(10),
        }
    }
}

/// WebSocket API configuration
#[derive(Debug, Clone)]
pub struct WebSocketApiConfig {
    /// WebSocket endpoint
    pub endpoint: String,
    /// Max connections
    pub max_connections: usize,
    /// Heartbeat interval
    pub heartbeat_interval: std::time::Duration,
    /// Connection timeout
    pub connection_timeout: std::time::Duration,
}

impl Default for WebSocketApiConfig {
    fn default() -> Self {
        Self {
            endpoint: "/ws".to_string(),
            max_connections: 1000,
            heartbeat_interval: std::time::Duration::from_secs(30),
            connection_timeout: std::time::Duration::from_secs(300),
        }
    }
}

/// gRPC API configuration
#[derive(Debug, Clone)]
pub struct GrpcApiConfig {
    /// gRPC host
    pub host: String,
    /// gRPC port
    pub port: u16,
    /// Enable reflection
    pub enable_reflection: bool,
    /// Max concurrent streams
    pub max_concurrent_streams: Option<u32>,
    /// Max message size
    pub max_message_size: Option<usize>,
}

impl Default for GrpcApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 9090,
            enable_reflection: true,
            max_concurrent_streams: Some(100),
            max_message_size: Some(1024 * 1024), // 1MB
        }
    }
}

/// API request/response types
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeDiscoveryRequest {
    pub path: String,
    pub options: Option<DiscoveryOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryOptions {
    pub auto_create: bool,
    pub confidence_threshold: Option<f64>,
    pub max_depth: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeDiscoveryResponse {
    pub scopes: Vec<ScopeInfo>,
    pub suggestions: Vec<ScopeSuggestion>,
    pub processing_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeInfo {
    pub name: String,
    pub path: String,
    pub scope_type: String,
    pub confidence: f64,
    pub created_at: String,
}

/// REST API server
pub struct RestApiServer {
    config: RestApiConfig,
    scope_loader: Arc<ScopeLoaderService>,
    app_state: Arc<AppState>,
}

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub scope_loader: Arc<ScopeLoaderService>,
    pub analytics: Arc<AdvancedAnalyticsManager>,
    pub background_processor: Arc<BackgroundProcessingManager>,
}

impl RestApiServer {
    /// Create a new REST API server
    pub fn new(
        config: RestApiConfig,
        scope_loader: Arc<ScopeLoaderService>,
        analytics: Arc<AdvancedAnalyticsManager>,
        background_processor: Arc<BackgroundProcessingManager>,
    ) -> Self {
        let app_state = Arc::new(AppState {
            scope_loader,
            analytics,
            background_processor,
        });

        Self {
            config,
            scope_loader: app_state.scope_loader.clone(),
            app_state,
        }
    }

    /// Create the API router
    pub fn create_router(&self) -> Router {
        Router::new()
            .route("/api/v1/scopes", get(Self::list_scopes))
            .route("/api/v1/scopes", post(Self::discover_scopes))
            .route("/api/v1/scopes/:id", get(Self::get_scope))
            .route("/api/v1/scopes/:id", put(Self::update_scope))
            .route("/api/v1/scopes/:id", delete(Self::delete_scope))
            .route("/api/v1/suggestions", get(Self::get_suggestions))
            // .route("/api/v1/analytics", get(Self::get_analytics))
            // .route("/api/v1/background-tasks", get(Self::list_background_tasks))
            // .route("/api/v1/background-tasks", post(Self::submit_background_task))
            .route("/api/v1/health", get(Self::health_check))
            .with_state(self.app_state.clone())
    }

    /// Start the REST API server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let app = self.create_router();
        let addr =
            format!("{}:{}", self.config.host, self.config.port).parse::<std::net::SocketAddr>()?;

        println!("REST API server starting on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    /// List scopes
    async fn list_scopes(
        State(state): State<Arc<AppState>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<ScopeInfo>>>, StatusCode> {
        let page = params
            .get("page")
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(1);
        let per_page = params
            .get("per_page")
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(20);

        // This would fetch scopes from the scope loader
        let scopes = vec![ScopeInfo {
            name: "example-scope".to_string(),
            path: "/example/path".to_string(),
            scope_type: "package".to_string(),
            confidence: 0.95,
            created_at: chrono::Utc::now().to_rfc3339(),
        }];

        let response = PaginatedResponse {
            data: scopes,
            total: 1,
            page,
            per_page,
            has_next: false,
            has_prev: false,
        };

        Ok(Json(ApiResponse {
            success: true,
            data: Some(response),
            error: None,
            message: None,
        }))
    }

    /// Discover scopes
    async fn discover_scopes(
        State(state): State<Arc<AppState>>,
        Json(request): Json<ScopeDiscoveryRequest>,
    ) -> Result<Json<ApiResponse<ScopeDiscoveryResponse>>, StatusCode> {
        let start_time = std::time::Instant::now();

        let path = std::path::Path::new(&request.path);

        // Discover scopes using the scope loader
        let scopes = match state.scope_loader.discover_scopes(path).await {
            Ok(scopes) => scopes,
            Err(e) => {
                return Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    message: None,
                }));
            }
        };

        let processing_time = start_time.elapsed().as_secs_f64();

        let scope_infos: Vec<ScopeInfo> = scopes
            .iter()
            .map(|scope| ScopeInfo {
                name: scope.definition.name.clone(),
                path: scope.path.to_string_lossy().to_string(),
                scope_type: "package".to_string(),
                confidence: 0.95,
                created_at: chrono::Utc::now().to_rfc3339(),
            })
            .collect();

        let response = ScopeDiscoveryResponse {
            scopes: scope_infos,
            suggestions: vec![],
            processing_time,
        };

        Ok(Json(ApiResponse {
            success: true,
            data: Some(response),
            error: None,
            message: None,
        }))
    }

    /// Get scope by ID
    async fn get_scope(
        State(_state): State<Arc<AppState>>,
        Path(id): Path<String>,
    ) -> Result<Json<ApiResponse<ScopeInfo>>, StatusCode> {
        // This would fetch a specific scope
        let scope = ScopeInfo {
            name: id,
            path: "/example/path".to_string(),
            scope_type: "package".to_string(),
            confidence: 0.95,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok(Json(ApiResponse {
            success: true,
            data: Some(scope),
            error: None,
            message: None,
        }))
    }

    /// Update scope
    async fn update_scope(
        State(_state): State<Arc<AppState>>,
        Path(id): Path<String>,
        Json(_update): Json<serde_json::Value>,
    ) -> Result<Json<ApiResponse<ScopeInfo>>, StatusCode> {
        // This would update a scope
        let scope = ScopeInfo {
            name: id,
            path: "/example/path".to_string(),
            scope_type: "package".to_string(),
            confidence: 0.95,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok(Json(ApiResponse {
            success: true,
            data: Some(scope),
            error: None,
            message: Some("Scope updated successfully".to_string()),
        }))
    }

    /// Delete scope
    async fn delete_scope(
        State(_state): State<Arc<AppState>>,
        Path(id): Path<String>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        // This would delete a scope
        Ok(Json(ApiResponse {
            success: true,
            data: None,
            error: None,
            message: Some(format!("Scope {} deleted successfully", id)),
        }))
    }

    /// Get suggestions
    async fn get_suggestions(
        State(_state): State<Arc<AppState>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<Vec<ScopeSuggestion>>>, StatusCode> {
        // This would get scope suggestions
        let suggestions = vec![];

        Ok(Json(ApiResponse {
            success: true,
            data: Some(suggestions),
            error: None,
            message: None,
        }))
    }

    /// Get analytics
    async fn get_analytics(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<ApiResponse<AnalyticsStats>>, StatusCode> {
        let stats = state.analytics.get_stats().await;

        Ok(Json(ApiResponse {
            success: true,
            data: Some(stats),
            error: None,
            message: None,
        }))
    }

    /// List background tasks
    async fn list_background_tasks(
        State(state): State<Arc<AppState>>,
    ) -> Result<Json<ApiResponse<Vec<BackgroundTaskResult>>>, StatusCode> {
        let tasks = state.background_processor.get_completed_tasks().await;

        Ok(Json(ApiResponse {
            success: true,
            data: Some(tasks),
            error: None,
            message: None,
        }))
    }

    /// Submit background task
    async fn submit_background_task(
        State(state): State<Arc<AppState>>,
        Json(task): Json<BackgroundTask>,
    ) -> Result<Json<ApiResponse<String>>, StatusCode> {
        match state.background_processor.submit_task(task).await {
            Ok(_) => Ok(Json(ApiResponse {
                success: true,
                data: Some("Task submitted successfully".to_string()),
                error: None,
                message: None,
            })),
            Err(e) => Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
                message: None,
            })),
        }
    }

    /// Health check
    async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
        Json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({
                "status": "healthy",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "version": "1.0.0"
            })),
            error: None,
            message: None,
        })
    }
}

/// WebSocket API server
pub struct WebSocketApiServer {
    config: WebSocketApiConfig,
    connections: Arc<RwLock<HashMap<String, WebSocketConnection>>>,
}

/// WebSocket connection
#[derive(Debug)]
pub struct WebSocketConnection {
    pub id: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
}

impl WebSocketApiServer {
    /// Create a new WebSocket API server
    pub fn new(config: WebSocketApiConfig) -> Self {
        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Handle WebSocket connection
    pub async fn handle_connection(
        &self,
        connection_id: String,
    ) -> impl Stream<Item = Result<Event, axum::Error>> {
        // Add connection to registry
        {
            let mut connections = self.connections.write().await;
            connections.insert(
                connection_id.clone(),
                WebSocketConnection {
                    id: connection_id.clone(),
                    connected_at: chrono::Utc::now(),
                    last_heartbeat: chrono::Utc::now(),
                },
            );
        }

        // Create event stream
        stream::repeat_with(move || {
            Event::default().data(
                serde_json::json!({
                    "type": "heartbeat",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "connection_id": connection_id
                })
                .to_string(),
            )
        })
        .map(|event| Ok::<_, axum::Error>(event))
    }

    /// Broadcast message to all connections
    pub async fn broadcast(&self, message: serde_json::Value) {
        let connections = self.connections.read().await;
        for connection in connections.values() {
            // This would send the message to the WebSocket connection
            eprintln!(
                "Broadcasting to connection {}: {:?}",
                connection.id, message
            );
        }
    }

    /// Get active connections count
    pub async fn get_connection_count(&self) -> usize {
        self.connections.read().await.len()
    }
}

/// gRPC API server
pub struct GrpcApiServer {
    config: GrpcApiConfig,
    scope_loader: Arc<ScopeLoaderService>,
}

impl GrpcApiServer {
    /// Create a new gRPC API server
    pub fn new(config: GrpcApiConfig, scope_loader: Arc<ScopeLoaderService>) -> Self {
        Self {
            config,
            scope_loader,
        }
    }

    /// Start the gRPC server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr =
            format!("{}:{}", self.config.host, self.config.port).parse::<std::net::SocketAddr>()?;

        println!("gRPC server starting on {}", addr);

        // This would start the gRPC server with the scope loader service
        // For now, just return success
        Ok(())
    }
}

/// GraphQL API server
pub struct GraphQLApiServer {
    config: GraphQLApiConfig,
    scope_loader: Arc<ScopeLoaderService>,
}

impl GraphQLApiServer {
    /// Create a new GraphQL API server
    pub fn new(config: GraphQLApiConfig, scope_loader: Arc<ScopeLoaderService>) -> Self {
        Self {
            config,
            scope_loader,
        }
    }

    /// Create GraphQL schema
    pub fn create_schema(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // This would create a GraphQL schema using a library like juniper
        // For now, just return success
        Ok(())
    }

    /// Handle GraphQL query
    pub async fn handle_query(
        &self,
        query: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        // This would execute a GraphQL query
        // For now, return a placeholder response
        Ok(serde_json::json!({
            "data": {
                "scopes": [],
                "suggestions": []
            }
        }))
    }
}

/// API manager for coordinating all API servers
pub struct ApiManager {
    rest_server: Option<RestApiServer>,
    websocket_server: Option<WebSocketApiServer>,
    grpc_server: Option<GrpcApiServer>,
    graphql_server: Option<GraphQLApiServer>,
    config: ApiManagerConfig,
}

/// API manager configuration
#[derive(Debug, Clone)]
pub struct ApiManagerConfig {
    pub rest_config: RestApiConfig,
    pub websocket_config: WebSocketApiConfig,
    pub grpc_config: GrpcApiConfig,
    pub graphql_config: GraphQLApiConfig,
    pub enable_rest: bool,
    pub enable_websocket: bool,
    pub enable_grpc: bool,
    pub enable_graphql: bool,
}

impl Default for ApiManagerConfig {
    fn default() -> Self {
        Self {
            rest_config: RestApiConfig::default(),
            websocket_config: WebSocketApiConfig::default(),
            grpc_config: GrpcApiConfig::default(),
            graphql_config: GraphQLApiConfig::default(),
            enable_rest: true,
            enable_websocket: true,
            enable_grpc: true,
            enable_graphql: true,
        }
    }
}

impl ApiManager {
    /// Create a new API manager
    pub fn new(
        config: ApiManagerConfig,
        scope_loader: Arc<ScopeLoaderService>,
        analytics: Arc<AdvancedAnalyticsManager>,
        background_processor: Arc<BackgroundProcessingManager>,
    ) -> Self {
        let rest_server = if config.enable_rest {
            Some(RestApiServer::new(
                config.rest_config.clone(),
                scope_loader.clone(),
                analytics.clone(),
                background_processor.clone(),
            ))
        } else {
            None
        };

        let websocket_server = if config.enable_websocket {
            Some(WebSocketApiServer::new(config.websocket_config.clone()))
        } else {
            None
        };

        let grpc_server = if config.enable_grpc {
            Some(GrpcApiServer::new(
                config.grpc_config.clone(),
                scope_loader.clone(),
            ))
        } else {
            None
        };

        let graphql_server = if config.enable_graphql {
            Some(GraphQLApiServer::new(
                config.graphql_config.clone(),
                scope_loader,
            ))
        } else {
            None
        };

        Self {
            rest_server,
            websocket_server,
            grpc_server,
            graphql_server,
            config,
        }
    }

    /// Start all API servers
    pub async fn start_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Start REST server
        if let Some(rest_server) = &self.rest_server {
            let _ = rest_server.start().await;
        }

        // Start gRPC server
        if let Some(grpc_server) = &self.grpc_server {
            let _ = grpc_server.start().await;
        }

        Ok(())
    }

    /// Stop all API servers
    pub async fn stop_all(&self) {
        // This would stop all running servers
        eprintln!("Stopping all API servers");
    }

    /// Get API status
    pub async fn get_status(&self) -> ApiStatus {
        ApiStatus {
            rest_enabled: self.rest_server.is_some(),
            websocket_enabled: self.websocket_server.is_some(),
            grpc_enabled: self.grpc_server.is_some(),
            graphql_enabled: self.graphql_server.is_some(),
            websocket_connections: if let Some(ws_server) = &self.websocket_server {
                ws_server.get_connection_count().await
            } else {
                0
            },
        }
    }
}

/// API status
#[derive(Debug, Serialize)]
pub struct ApiStatus {
    pub rest_enabled: bool,
    pub websocket_enabled: bool,
    pub grpc_enabled: bool,
    pub graphql_enabled: bool,
    pub websocket_connections: usize,
}
