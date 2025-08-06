use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt};
use chrono::{DateTime, Utc};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, HealthStatus, ImpactScore};
use crate::graph::DependencyGraph;

/// Real-time event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealtimeEvent {
    /// Dependency health status changed
    HealthStatusChanged {
        dependency_id: String,
        old_status: HealthStatus,
        new_status: HealthStatus,
        timestamp: DateTime<Utc>,
    },
    /// Dependency added
    DependencyAdded {
        dependency: DependencyConfig,
        timestamp: DateTime<Utc>,
    },
    /// Dependency removed
    DependencyRemoved {
        dependency_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Dependency relationship added
    RelationshipAdded {
        source_id: String,
        target_id: String,
        relationship_type: String,
        timestamp: DateTime<Utc>,
    },
    /// Dependency relationship removed
    RelationshipRemoved {
        source_id: String,
        target_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Impact analysis updated
    ImpactAnalysisUpdated {
        dependency_id: String,
        impact_score: ImpactScore,
        timestamp: DateTime<Utc>,
    },
    /// Circular dependency detected
    CircularDependencyDetected {
        dependency_ids: Vec<String>,
        timestamp: DateTime<Utc>,
    },
    /// Health check failed
    HealthCheckFailed {
        dependency_id: String,
        error_message: String,
        timestamp: DateTime<Utc>,
    },
    /// Alert triggered
    AlertTriggered {
        alert_name: String,
        dependency_id: String,
        severity: String,
        message: String,
        timestamp: DateTime<Utc>,
    },
    /// System status update
    SystemStatusUpdate {
        total_dependencies: usize,
        healthy_count: usize,
        degraded_count: usize,
        unhealthy_count: usize,
        down_count: usize,
        timestamp: DateTime<Utc>,
    },
}

/// Real-time client connection
pub struct RealtimeClient {
    /// Client ID
    pub id: String,
    /// WebSocket stream
    pub stream: WebSocketStream<TcpStream>,
    /// Client subscriptions
    pub subscriptions: Vec<String>,
    /// Client metadata
    pub metadata: HashMap<String, String>,
}

/// Real-time server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeServerConfig {
    /// Server address
    pub address: String,
    /// Server port
    pub port: u16,
    /// Enable SSL/TLS
    pub enable_ssl: bool,
    /// SSL certificate path
    pub ssl_certificate_path: Option<String>,
    /// SSL private key path
    pub ssl_private_key_path: Option<String>,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
    /// Enable authentication
    pub enable_authentication: bool,
    /// Authentication token
    pub authentication_token: Option<String>,
}

/// Real-time server
pub struct RealtimeServer {
    /// Server configuration
    config: RealtimeServerConfig,
    /// Connected clients
    clients: Arc<RwLock<HashMap<String, RealtimeClient>>>,
    /// Event sender
    event_sender: mpsc::Sender<RealtimeEvent>,
    /// Event receiver
    event_receiver: Option<mpsc::Receiver<RealtimeEvent>>,
    /// Server task handle
    server_task: Option<tokio::task::JoinHandle<()>>,
    /// Dependency graph reference
    graph: Arc<RwLock<DependencyGraph>>,
}

/// Real-time client message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Subscribe to events
    Subscribe {
        event_types: Vec<String>,
    },
    /// Unsubscribe from events
    Unsubscribe {
        event_types: Vec<String>,
    },
    /// Ping message
    Ping {
        timestamp: DateTime<Utc>,
    },
    /// Pong response
    Pong {
        timestamp: DateTime<Utc>,
    },
    /// Authentication message
    Authenticate {
        token: String,
    },
    /// Request dependency information
    GetDependency {
        dependency_id: String,
    },
    /// Request all dependencies
    GetAllDependencies,
    /// Request health status
    GetHealthStatus {
        dependency_id: String,
    },
    /// Request impact analysis
    GetImpactAnalysis {
        dependency_id: String,
    },
}

/// Real-time server message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Event notification
    Event {
        event: RealtimeEvent,
    },
    /// Response to client request
    Response {
        request_id: Option<String>,
        data: serde_json::Value,
        success: bool,
        error_message: Option<String>,
    },
    /// Pong response
    Pong {
        timestamp: DateTime<Utc>,
    },
    /// Authentication result
    AuthenticationResult {
        success: bool,
        message: String,
    },
    /// Heartbeat message
    Heartbeat {
        timestamp: DateTime<Utc>,
    },
}

impl RealtimeServer {
    /// Create a new real-time server
    pub fn new(config: RealtimeServerConfig, graph: Arc<RwLock<DependencyGraph>>) -> Self {
        let (event_sender, event_receiver) = mpsc::channel(1000);
        
        Self {
            config,
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            event_receiver: Some(event_receiver),
            server_task: None,
            graph,
        }
    }

    /// Start the real-time server
    pub async fn start(&mut self) -> Result<()> {
        if self.server_task.is_some() {
            return Err(Error::RealtimeCommunication("Server already started".to_string()));
        }

        let config = self.config.clone();
        let clients = self.clients.clone();
        let event_sender = self.event_sender.clone();
        let graph = self.graph.clone();

        let event_receiver = self.event_receiver.take();
        let clients_clone = clients.clone();
        let task = tokio::spawn(async move {
            let addr = format!("{}:{}", config.address, config.port);
            let listener = TcpListener::bind(&addr).await.unwrap();
            tracing::info!("Real-time server listening on {}", addr);

            let event_task = tokio::spawn(async move {
                if let Some(mut event_receiver) = event_receiver {
                    Self::handle_events(&mut event_receiver, clients_clone.clone()).await;
                }
            });

            while let Ok((stream, addr)) = listener.accept().await {
                let clients_inner = clients.clone();
                let graph = graph.clone();
                let config = config.clone();

                tokio::spawn(async move {
                    if let Err(e) = Self::handle_connection(stream, addr, clients_inner, graph, config).await {
                        tracing::error!("Connection error: {}", e);
                    }
                });
            }

            event_task.abort();
        });

        self.server_task = Some(task);
        Ok(())
    }

    /// Stop the real-time server
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(task) = self.server_task.take() {
            task.abort();
        }
        Ok(())
    }

    /// Handle incoming connections
    async fn handle_connection(
        stream: TcpStream,
        addr: std::net::SocketAddr,
        clients: Arc<RwLock<HashMap<String, RealtimeClient>>>,
        graph: Arc<RwLock<DependencyGraph>>,
        config: RealtimeServerConfig,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await
            .map_err(|e| Error::WebSocket(e.to_string()))?;

        let client_id = uuid::Uuid::new_v4().to_string();
        let mut client = RealtimeClient {
            id: client_id.clone(),
            stream: ws_stream,
            subscriptions: Vec::new(),
            metadata: HashMap::new(),
        };

        // Add client to registry
        {
            let mut clients_guard = clients.write().await;
            clients_guard.insert(client_id.clone(), client);
        }

        tracing::info!("Client connected: {} from {}", client_id, addr);

        // Handle client messages
        let mut client_guard = clients.write().await;
        let client = client_guard.get_mut(&client_id).unwrap();
        while let Some(msg) = client.stream.next().await {
            match msg {
                Ok(msg) => {
                    if let Err(e) = Self::handle_client_message(client, msg, &graph, &config).await {
                        tracing::error!("Error handling client message: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
            }
        }

        // Remove client from registry
        {
            let mut clients_guard = clients.write().await;
            clients_guard.remove(&client_id);
        }

        tracing::info!("Client disconnected: {}", client_id);
        Ok(())
    }

    /// Handle client messages
    async fn handle_client_message(
        client: &mut RealtimeClient,
        msg: tokio_tungstenite::tungstenite::Message,
        graph: &Arc<RwLock<DependencyGraph>>,
        config: &RealtimeServerConfig,
    ) -> Result<()> {
        match msg {
            tokio_tungstenite::tungstenite::Message::Text(text) => {
                let client_msg: ClientMessage = serde_json::from_str(&text)
                    .map_err(|e| Error::RealtimeCommunication(format!("Invalid message format: {}", e)))?;

                match client_msg {
                    ClientMessage::Subscribe { event_types } => {
                        client.subscriptions.extend(event_types);
                        let response = ServerMessage::Response {
                            request_id: None,
                            data: serde_json::json!({ "message": "Subscribed successfully" }),
                            success: true,
                            error_message: None,
                        };
                        let response_text = serde_json::to_string(&response)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(response_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                    ClientMessage::Unsubscribe { event_types } => {
                        client.subscriptions.retain(|sub| !event_types.contains(sub));
                        let response = ServerMessage::Response {
                            request_id: None,
                            data: serde_json::json!({ "message": "Unsubscribed successfully" }),
                            success: true,
                            error_message: None,
                        };
                        let response_text = serde_json::to_string(&response)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(response_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                    ClientMessage::Ping { timestamp } => {
                        let pong = ServerMessage::Pong { timestamp };
                        let pong_text = serde_json::to_string(&pong)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(pong_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                    ClientMessage::Pong { .. } => {
                        // Ignore pong messages from clients
                    }
                    ClientMessage::Authenticate { token } => {
                        let success = if let Some(expected_token) = &config.authentication_token {
                            token == *expected_token
                        } else {
                            true
                        };

                        let response = ServerMessage::AuthenticationResult {
                            success,
                            message: if success { "Authentication successful".to_string() } else { "Authentication failed".to_string() },
                        };
                        let response_text = serde_json::to_string(&response)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(response_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                    ClientMessage::GetDependency { dependency_id } => {
                        let graph_guard = graph.read().await;
                        let dependency = graph_guard.get_dependency_config(&dependency_id);
                        
                        let response = ServerMessage::Response {
                            request_id: None,
                            data: if let Ok(config) = dependency {
                                serde_json::to_value(config)
                                    .map_err(|e| Error::Serialization(e.into()))?
                            } else {
                                serde_json::json!(null)
                            },
                            success: dependency.is_ok(),
                            error_message: dependency.err().map(|e| e.to_string()),
                        };
                        let response_text = serde_json::to_string(&response)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(response_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                    ClientMessage::GetAllDependencies => {
                        let graph_guard = graph.read().await;
                        let dependencies = graph_guard.get_all_dependency_configs();
                        
                        let response = ServerMessage::Response {
                            request_id: None,
                            data: serde_json::to_value(dependencies)
                                .map_err(|e| Error::Serialization(e.into()))?,
                            success: true,
                            error_message: None,
                        };
                        let response_text = serde_json::to_string(&response)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(response_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                    ClientMessage::GetHealthStatus { dependency_id } => {
                        // This would need access to health monitoring data
                        let response = ServerMessage::Response {
                            request_id: None,
                            data: serde_json::json!({ "status": "unknown" }),
                            success: true,
                            error_message: None,
                        };
                        let response_text = serde_json::to_string(&response)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(response_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                    ClientMessage::GetImpactAnalysis { dependency_id } => {
                        // This would need access to impact analysis data
                        let response = ServerMessage::Response {
                            request_id: None,
                            data: serde_json::json!({ "impact": "unknown" }),
                            success: true,
                            error_message: None,
                        };
                        let response_text = serde_json::to_string(&response)
                            .map_err(|e| Error::Serialization(e.into()))?;
                        client.stream.send(tokio_tungstenite::tungstenite::Message::Text(response_text)).await
                            .map_err(|e| Error::WebSocket(e.to_string()))?;
                    }
                }
            }
            tokio_tungstenite::tungstenite::Message::Close(_) => {
                return Ok(());
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle events and broadcast to clients
    async fn handle_events(
        event_receiver: &mut mpsc::Receiver<RealtimeEvent>,
        clients: Arc<RwLock<HashMap<String, RealtimeClient>>>,
    ) {
        while let Some(event) = event_receiver.recv().await {
            let server_msg = ServerMessage::Event { event };
            let event_text = match serde_json::to_string(&server_msg) {
                Ok(text) => text,
                Err(e) => {
                    tracing::error!("Failed to serialize event: {}", e);
                    continue;
                }
            };

            let mut clients_guard = clients.write().await;
            let mut disconnected_clients = Vec::new();

            for (client_id, client) in clients_guard.iter_mut() {
                // Check if client is subscribed to this event type
                let event_type = match &server_msg {
                    ServerMessage::Event { event } => match event {
                        RealtimeEvent::HealthStatusChanged { .. } => "health_status_changed",
                        RealtimeEvent::DependencyAdded { .. } => "dependency_added",
                        RealtimeEvent::DependencyRemoved { .. } => "dependency_removed",
                        RealtimeEvent::RelationshipAdded { .. } => "relationship_added",
                        RealtimeEvent::RelationshipRemoved { .. } => "relationship_removed",
                        RealtimeEvent::ImpactAnalysisUpdated { .. } => "impact_analysis_updated",
                        RealtimeEvent::CircularDependencyDetected { .. } => "circular_dependency_detected",
                        RealtimeEvent::HealthCheckFailed { .. } => "health_check_failed",
                        RealtimeEvent::AlertTriggered { .. } => "alert_triggered",
                        RealtimeEvent::SystemStatusUpdate { .. } => "system_status_update",
                    },
                    _ => continue,
                };

                if client.subscriptions.contains(&event_type.to_string()) {
                    if let Err(e) = client.stream.send(tokio_tungstenite::tungstenite::Message::Text(event_text.clone())).await {
                        tracing::error!("Failed to send event to client {}: {}", client_id, e);
                        disconnected_clients.push(client_id.clone());
                    }
                }
            }

            // Remove disconnected clients
            for client_id in disconnected_clients {
                clients_guard.remove(&client_id);
                tracing::info!("Removed disconnected client: {}", client_id);
            }
        }
    }

    /// Broadcast event to all connected clients
    pub async fn broadcast_event(&self, event: RealtimeEvent) -> Result<()> {
        self.event_sender.send(event).await
            .map_err(|e| Error::RealtimeCommunication(format!("Failed to send event: {}", e)))?;
        Ok(())
    }

    /// Get connected clients count
    pub async fn get_connected_clients_count(&self) -> usize {
        self.clients.read().await.len()
    }

    /// Get server statistics
    pub async fn get_statistics(&self) -> RealtimeServerStatistics {
        let clients_count = self.get_connected_clients_count().await;
        
        RealtimeServerStatistics {
            connected_clients: clients_count,
            server_address: format!("{}:{}", self.config.address, self.config.port),
            enable_ssl: self.config.enable_ssl,
            max_connections: self.config.max_connections,
            last_updated: Utc::now(),
        }
    }
}

impl Default for RealtimeServerConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 8080,
            enable_ssl: false,
            ssl_certificate_path: None,
            ssl_private_key_path: None,
            max_connections: 1000,
            connection_timeout: 30,
            heartbeat_interval: 30,
            enable_authentication: false,
            authentication_token: None,
        }
    }
}

/// Real-time server statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeServerStatistics {
    /// Number of connected clients
    pub connected_clients: usize,
    /// Server address
    pub server_address: String,
    /// Whether SSL is enabled
    pub enable_ssl: bool,
    /// Maximum connections
    pub max_connections: u32,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyType;

    #[test]
    fn test_realtime_server_config_default() {
        let config = RealtimeServerConfig::default();
        assert_eq!(config.address, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert!(!config.enable_ssl);
        assert_eq!(config.max_connections, 1000);
    }

    #[test]
    fn test_realtime_event_serialization() {
        let event = RealtimeEvent::HealthStatusChanged {
            dependency_id: "test-1".to_string(),
            old_status: HealthStatus::Healthy,
            new_status: HealthStatus::Degraded,
            timestamp: Utc::now(),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: RealtimeEvent = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            RealtimeEvent::HealthStatusChanged { dependency_id, old_status, new_status, .. } => {
                assert_eq!(dependency_id, "test-1");
                assert_eq!(old_status, HealthStatus::Healthy);
                assert_eq!(new_status, HealthStatus::Degraded);
            }
            _ => panic!("Unexpected event type"),
        }
    }

    #[test]
    fn test_client_message_serialization() {
        let message = ClientMessage::Subscribe {
            event_types: vec!["health_status_changed".to_string(), "dependency_added".to_string()],
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: ClientMessage = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            ClientMessage::Subscribe { event_types } => {
                assert_eq!(event_types.len(), 2);
                assert!(event_types.contains(&"health_status_changed".to_string()));
                assert!(event_types.contains(&"dependency_added".to_string()));
            }
            _ => panic!("Unexpected message type"),
        }
    }

    #[test]
    fn test_server_message_serialization() {
        let event = RealtimeEvent::DependencyAdded {
            dependency: crate::types::DependencyConfig::new(
                "test-1".to_string(),
                "Test Dependency".to_string(),
                DependencyType::ApiCall,
                "http://test.example.com".to_string(),
                vec!["GET".to_string()],
            ).unwrap(),
            timestamp: Utc::now(),
        };

        let message = ServerMessage::Event { event };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: ServerMessage = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            ServerMessage::Event { event } => {
                match event {
                    RealtimeEvent::DependencyAdded { dependency, .. } => {
                        assert_eq!(dependency.id, "test-1");
                        assert_eq!(dependency.name, "Test Dependency");
                    }
                    _ => panic!("Unexpected event type"),
                }
            }
            _ => panic!("Unexpected message type"),
        }
    }

    #[test]
    fn test_realtime_server_statistics() {
        let stats = RealtimeServerStatistics {
            connected_clients: 5,
            server_address: "127.0.0.1:8080".to_string(),
            enable_ssl: false,
            max_connections: 1000,
            last_updated: Utc::now(),
        };

        assert_eq!(stats.connected_clients, 5);
        assert_eq!(stats.server_address, "127.0.0.1:8080");
        assert!(!stats.enable_ssl);
        assert_eq!(stats.max_connections, 1000);
    }
} 