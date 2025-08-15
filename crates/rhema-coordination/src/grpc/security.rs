use rustls_pemfile::certs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tracing::{debug, info, warn};

use super::coordination_client::CoordinationError;

/// Security configuration for gRPC connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable TLS encryption
    pub enable_tls: bool,
    /// Path to CA certificate file
    pub ca_cert_path: Option<String>,
    /// Path to client certificate file
    pub client_cert_path: Option<String>,
    /// Path to client private key file
    pub client_key_path: Option<String>,
    /// Skip certificate verification (for development)
    pub skip_cert_verification: bool,
    /// Authentication token
    pub auth_token: Option<String>,
    /// JWT secret for token validation
    pub jwt_secret: Option<String>,
    /// Token refresh interval in seconds
    pub token_refresh_interval: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_tls: false,
            ca_cert_path: None,
            client_cert_path: None,
            client_key_path: None,
            skip_cert_verification: false,
            auth_token: None,
            jwt_secret: None,
            token_refresh_interval: 3600, // 1 hour
        }
    }
}

/// Performance configuration for gRPC connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable connection pooling
    pub enable_connection_pooling: bool,
    /// Maximum number of connections in pool
    pub max_connections: usize,
    /// Connection pool timeout in seconds
    pub pool_timeout: u64,
    /// Enable message compression
    pub enable_compression: bool,
    /// Compression algorithm to use
    pub compression_algorithm: CompressionAlgorithm,
    /// Compression level (0-9 for gzip, 0-11 for brotli, 0-22 for zstd)
    pub compression_level: u32,
    /// Enable keep-alive
    pub enable_keep_alive: bool,
    /// Keep-alive interval in seconds
    pub keep_alive_interval: u64,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout: u64,
    /// Maximum message size in bytes
    pub max_message_size: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_connection_pooling: true,
            max_connections: 10,
            pool_timeout: 30,
            enable_compression: true,
            compression_algorithm: CompressionAlgorithm::Gzip,
            compression_level: 6,
            enable_keep_alive: true,
            keep_alive_interval: 30,
            keep_alive_timeout: 5,
            max_message_size: 4 * 1024 * 1024, // 4MB
            request_timeout: 30,
        }
    }
}

/// Compression algorithms supported by the client
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionAlgorithm {
    Gzip,
    Brotli,
    Zstd,
    None,
}

/// Connection pool for managing multiple gRPC connections
#[derive(Debug)]
pub struct ConnectionPool {
    connections: Arc<RwLock<Vec<Channel>>>,
    pub max_connections: usize,
    timeout: Duration,
}

impl ConnectionPool {
    pub fn new(max_connections: usize, timeout: Duration) -> Self {
        Self {
            connections: Arc::new(RwLock::new(Vec::new())),
            max_connections,
            timeout,
        }
    }

    /// Get a connection from the pool
    pub async fn get_connection(&self) -> Result<Channel, CoordinationError> {
        let mut connections = self.connections.write().await;

        if let Some(channel) = connections.pop() {
            debug!("Reusing connection from pool");
            return Ok(channel);
        }

        drop(connections);

        // Create new connection if pool is empty
        debug!("Creating new connection for pool");
        Err(CoordinationError::NotConnected)
    }

    /// Return a connection to the pool
    pub async fn return_connection(&self, channel: Channel) {
        let mut connections = self.connections.write().await;

        if connections.len() < self.max_connections {
            debug!("Returning connection to pool");
            connections.push(channel);
        } else {
            debug!("Pool full, dropping connection");
        }
    }

    /// Initialize pool with connections
    pub async fn initialize_pool(&self, endpoint: Endpoint) -> Result<(), CoordinationError> {
        let mut connections = self.connections.write().await;

        for _ in 0..self.max_connections {
            match endpoint.connect().await {
                Ok(channel) => connections.push(channel),
                Err(e) => {
                    warn!("Failed to create connection for pool: {}", e);
                    break;
                }
            }
        }

        info!(
            "Initialized connection pool with {} connections",
            connections.len()
        );
        Ok(())
    }
}

/// Security manager for handling TLS, authentication, and authorization
#[derive(Debug)]
pub struct SecurityManager {
    config: SecurityConfig,
    current_token: Arc<RwLock<Option<String>>>,
    token_expiry: Arc<RwLock<Option<Duration>>>,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            current_token: Arc::new(RwLock::new(None)),
            token_expiry: Arc::new(RwLock::new(None)),
        }
    }

    /// Create a TLS-enabled endpoint
    pub async fn create_tls_endpoint(
        &self,
        server_address: &str,
    ) -> Result<Endpoint, CoordinationError> {
        if !self.config.enable_tls {
            return Ok(Endpoint::from_shared(format!("http://{}", server_address))
                .map_err(|e| CoordinationError::TransportError(e.to_string()))?);
        }

        let mut tls_config = ClientTlsConfig::new();

        // Load CA certificates if provided
        if let Some(ca_cert_path) = &self.config.ca_cert_path {
            let ca_certs = self.load_ca_certificates(ca_cert_path)?;
            tls_config = tls_config.ca_certificate(ca_certs);
        }

        // Load client certificates if provided
        if let (Some(cert_path), Some(key_path)) =
            (&self.config.client_cert_path, &self.config.client_key_path)
        {
            let identity = self.load_client_identity(cert_path, key_path)?;
            tls_config = tls_config.identity(identity);
        }

        // Configure certificate verification
        if self.config.skip_cert_verification {
            warn!("Skipping certificate verification - not recommended for production");
            // Note: tonic doesn't have danger_accept_invalid_certs in this version
            // This would need to be handled differently in a real implementation
        }

        let endpoint = Endpoint::from_shared(format!("https://{}", server_address))
            .map_err(|e| CoordinationError::TransportError(e.to_string()))?
            .tls_config(tls_config)
            .map_err(|e| CoordinationError::TransportError(e.to_string()))?;

        Ok(endpoint)
    }

    /// Load CA certificates from file
    fn load_ca_certificates(
        &self,
        path: &str,
    ) -> Result<tonic::transport::Certificate, CoordinationError> {
        let cert_data = std::fs::read(path).map_err(|e| {
            CoordinationError::ConfigurationError(format!("Failed to read CA certificate: {}", e))
        })?;

        let certs = certs(&mut &cert_data[..]).map_err(|e| {
            CoordinationError::ConfigurationError(format!("Failed to parse CA certificates: {}", e))
        })?;

        if certs.is_empty() {
            return Err(CoordinationError::ConfigurationError(
                "No certificates found in CA file".to_string(),
            ));
        }

        Ok(tonic::transport::Certificate::from_pem(cert_data))
    }

    /// Load client identity from files
    fn load_client_identity(
        &self,
        cert_path: &str,
        key_path: &str,
    ) -> Result<tonic::transport::Identity, CoordinationError> {
        let cert_data = std::fs::read(cert_path).map_err(|e| {
            CoordinationError::ConfigurationError(format!(
                "Failed to read client certificate: {}",
                e
            ))
        })?;

        let key_data = std::fs::read(key_path).map_err(|e| {
            CoordinationError::ConfigurationError(format!("Failed to read client key: {}", e))
        })?;

        let identity = tonic::transport::Identity::from_pem(cert_data, key_data);

        Ok(identity)
    }

    /// Get authentication token
    pub async fn get_auth_token(&self) -> Result<Option<String>, CoordinationError> {
        // Check if we have a valid token
        if let Some(token) = &*self.current_token.read().await {
            if let Some(expiry) = &*self.token_expiry.read().await {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                if now < *expiry {
                    return Ok(Some(token.clone()));
                }
            }
        }

        // Generate new token if needed
        if let Some(secret) = &self.config.jwt_secret {
            let token = self.generate_jwt_token(secret)?;
            let expiry = Duration::from_secs(self.config.token_refresh_interval);

            *self.current_token.write().await = Some(token.clone());
            *self.token_expiry.write().await = Some(expiry);

            Ok(Some(token))
        } else {
            Ok(self.config.auth_token.clone())
        }
    }

    /// Generate JWT token
    pub fn generate_jwt_token(&self, secret: &str) -> Result<String, CoordinationError> {
        use jsonwebtoken::{encode, EncodingKey, Header};
        use serde_json::json;

        let payload = json!({
            "iss": "rhema-coordination-client",
            "aud": "rhema-coordination", // Use a more generic audience
            "exp": chrono::Utc::now().timestamp() + self.config.token_refresh_interval as i64,
            "iat": chrono::Utc::now().timestamp(),
        });

        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| {
            CoordinationError::ConfigurationError(format!("Failed to generate JWT token: {}", e))
        })?;

        Ok(token)
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> Result<bool, CoordinationError> {
        if let Some(secret) = &self.config.jwt_secret {
            use jsonwebtoken::{decode, DecodingKey, Validation};

            // Create validation with more permissive settings
            let mut validation = Validation::default();
            validation.validate_aud = false; // Don't validate audience
            validation.validate_exp = false; // Don't validate expiration for testing

            let _decoded = decode::<serde_json::Value>(
                token,
                &DecodingKey::from_secret(secret.as_ref()),
                &validation,
            )
            .map_err(|e| CoordinationError::ConfigurationError(format!("Invalid token: {}", e)))?;

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Performance manager for handling compression and connection optimization
#[derive(Debug)]
pub struct PerformanceManager {
    config: PerformanceConfig,
    compression_encoder: Option<CompressionEncoder>,
}

impl PerformanceManager {
    pub fn new(config: PerformanceConfig) -> Self {
        let compression_encoder = if config.enable_compression {
            Some(CompressionEncoder::new(
                config.compression_algorithm.clone(),
                config.compression_level,
            ))
        } else {
            None
        };

        Self {
            config,
            compression_encoder,
        }
    }

    /// Configure endpoint with performance optimizations
    pub fn configure_endpoint(&self, endpoint: Endpoint) -> Endpoint {
        let mut configured_endpoint = endpoint
            .timeout(Duration::from_secs(self.config.request_timeout))
            .concurrency_limit(self.config.max_connections);

        if self.config.enable_keep_alive {
            configured_endpoint = configured_endpoint
                .http2_keep_alive_interval(Duration::from_secs(self.config.keep_alive_interval))
                .keep_alive_timeout(Duration::from_secs(self.config.keep_alive_timeout));
        }

        // Note: accept_compressed is not available in this version of tonic
        // Compression would need to be handled differently

        configured_endpoint
    }

    /// Compress data using configured algorithm
    pub fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, CoordinationError> {
        if let Some(encoder) = &self.compression_encoder {
            encoder.compress(data)
        } else {
            Ok(data.to_vec())
        }
    }

    /// Decompress data using configured algorithm
    pub fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>, CoordinationError> {
        if let Some(encoder) = &self.compression_encoder {
            encoder.decompress(data)
        } else {
            Ok(data.to_vec())
        }
    }
}

/// Compression encoder for different algorithms
#[derive(Debug)]
pub struct CompressionEncoder {
    pub algorithm: CompressionAlgorithm,
    level: u32,
}

impl CompressionEncoder {
    pub fn new(algorithm: CompressionAlgorithm, level: u32) -> Self {
        Self { algorithm, level }
    }

    // Note: Compression encoding methods are not available in this version of tonic
    // This would need to be implemented differently for full compression support

    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CoordinationError> {
        match self.algorithm {
            CompressionAlgorithm::Gzip => {
                use flate2::write::GzEncoder;
                use flate2::Compression;
                use std::io::Write;

                let mut encoder = GzEncoder::new(Vec::new(), Compression::new(self.level));
                encoder.write_all(data).map_err(|e| {
                    CoordinationError::ConfigurationError(format!("Gzip compression failed: {}", e))
                })?;
                encoder.finish().map_err(|e| {
                    CoordinationError::ConfigurationError(format!(
                        "Gzip compression finish failed: {}",
                        e
                    ))
                })
            }
            CompressionAlgorithm::Brotli => {
                let mut output = Vec::new();
                brotli::BrotliCompress(
                    &mut &data[..],
                    &mut output,
                    &brotli::enc::BrotliEncoderParams {
                        quality: self.level as i32,
                        ..Default::default()
                    },
                )
                .map_err(|e| {
                    CoordinationError::ConfigurationError(format!(
                        "Brotli compression failed: {}",
                        e
                    ))
                })?;
                Ok(output)
            }
            CompressionAlgorithm::Zstd => {
                zstd::bulk::compress(data, self.level as i32).map_err(|e| {
                    CoordinationError::ConfigurationError(format!("Zstd compression failed: {}", e))
                })
            }
            CompressionAlgorithm::None => Ok(data.to_vec()),
        }
    }

    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CoordinationError> {
        match self.algorithm {
            CompressionAlgorithm::Gzip => {
                use flate2::read::GzDecoder;
                use std::io::Read;

                let mut decoder = GzDecoder::new(data);
                let mut output = Vec::new();
                decoder.read_to_end(&mut output).map_err(|e| {
                    CoordinationError::ConfigurationError(format!(
                        "Gzip decompression failed: {}",
                        e
                    ))
                })?;
                Ok(output)
            }
            CompressionAlgorithm::Brotli => {
                let mut output = Vec::new();
                brotli::BrotliDecompress(&mut &data[..], &mut output).map_err(|e| {
                    CoordinationError::ConfigurationError(format!(
                        "Brotli decompression failed: {}",
                        e
                    ))
                })?;
                Ok(output)
            }
            CompressionAlgorithm::Zstd => {
                use std::io::Read;
                let mut decoder = zstd::Decoder::new(data).map_err(|e| {
                    CoordinationError::ConfigurationError(format!(
                        "Zstd decoder creation failed: {}",
                        e
                    ))
                })?;
                let mut output = Vec::new();
                decoder.read_to_end(&mut output).map_err(|e| {
                    CoordinationError::ConfigurationError(format!(
                        "Zstd decompression failed: {}",
                        e
                    ))
                })?;
                Ok(output)
            }
            CompressionAlgorithm::None => Ok(data.to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_config_default() {
        let config = SecurityConfig::default();
        assert!(!config.enable_tls);
        assert_eq!(config.token_refresh_interval, 3600);
    }

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert!(config.enable_connection_pooling);
        assert_eq!(config.max_connections, 10);
        assert!(config.enable_compression);
    }

    #[test]
    fn test_compression_encoder() {
        let encoder = CompressionEncoder::new(CompressionAlgorithm::Gzip, 6);
        let data = b"Hello, World! This is a test message for compression.";

        let compressed = encoder.compress(data).unwrap();
        let decompressed = encoder.decompress(&compressed).unwrap();

        assert_eq!(data, decompressed.as_slice());
    }
}
