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

use aes_gcm::{Aes256Gcm, Key};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

/// Authentication token types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    ApiKey,
    Jwt,
    Session,
}

/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub id: String,
    pub token_type: TokenType,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub usage_count: u64,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub authenticated: bool,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub token_id: Option<String>,
    pub error: Option<String>,
    pub session_id: Option<String>,
}

/// Authentication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStats {
    pub total_requests: u64,
    pub successful_auths: u64,
    pub failed_auths: u64,
    pub active_tokens: usize,
    pub active_sessions: usize,
    pub last_auth_time: Option<chrono::DateTime<chrono::Utc>>,
    pub rate_limit_violations: u64,
    pub security_violations: u64,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub user_id: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub resource: Option<String>,
    pub action: String,
    pub result: AuditResult,
    pub details: HashMap<String, serde_json::Value>,
    pub session_id: Option<String>,
}

/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    Authentication,
    Authorization,
    ResourceAccess,
    SecurityViolation,
    RateLimitViolation,
    SessionManagement,
    TokenManagement,
}

/// Audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure,
    Denied,
    RateLimited,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub client_info: ClientInfo,
    pub permissions: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub client_type: ClientType,
    pub fingerprint: Option<String>,
}

/// Client type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientType {
    Http,
    WebSocket,
    UnixSocket,
    Api,
}

/// Rate limiting information
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub client_id: String,
    pub requests: Vec<Instant>,
    pub limit: u32,
    pub window: Duration,
    pub last_violation: Option<Instant>,
}

/// Authentication manager
#[derive(Clone)]
pub struct AuthManager {
    config: crate::mcp::AuthConfig,
    api_keys: Arc<RwLock<HashMap<String, AuthToken>>>,
    jwt_secret: Option<String>,
    active_sessions: Arc<RwLock<HashMap<String, Session>>>,
    rate_limiters: Arc<RwLock<HashMap<String, RateLimitInfo>>>,
    stats: Arc<RwLock<AuthStats>>,
    audit_logger: Arc<AuditLogger>,
    // Enhanced security features
    encryption_key: Option<Key<Aes256Gcm>>,
    jwt_encoding_key: Option<EncodingKey>,
    jwt_decoding_key: Option<DecodingKey>,
    security_monitor: Arc<SecurityMonitor>,
}

/// Security monitoring and alerting
#[derive(Debug)]
pub struct SecurityMonitor {
    failed_attempts: Arc<RwLock<HashMap<String, u32>>>,
    suspicious_ips: Arc<RwLock<HashMap<String, Instant>>>,
    security_events: Arc<RwLock<Vec<SecurityEvent>>>,
    alert_threshold: u32,
    lockout_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub timestamp: Instant,
    pub event_type: SecurityEventType,
    pub client_ip: Option<String>,
    pub user_id: Option<String>,
    pub details: String,
    pub severity: SecuritySeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityEventType {
    FailedAuthentication,
    RateLimitViolation,
    SuspiciousActivity,
    TokenCompromise,
    BruteForceAttempt,
    UnauthorizedAccess,
}

#[derive(Debug, Clone, Copy)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Audit logger
pub struct AuditLogger {
    log_file: Option<PathBuf>,
    enabled: bool,
    log_level: AuditLogLevel,
}

/// Audit log level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditLogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(config: &crate::mcp::AuthConfig) -> RhemaResult<Self> {
        let api_keys = Arc::new(RwLock::new(HashMap::new()));
        let active_sessions = Arc::new(RwLock::new(HashMap::new()));
        let rate_limiters = Arc::new(RwLock::new(HashMap::new()));

        let stats = Arc::new(RwLock::new(AuthStats {
            total_requests: 0,
            successful_auths: 0,
            failed_auths: 0,
            active_tokens: 0,
            active_sessions: 0,
            last_auth_time: None,
            rate_limit_violations: 0,
            security_violations: 0,
        }));

        let audit_logger = Arc::new(AuditLogger::new(
            config.audit_logging.enabled,
            config.audit_logging.log_file.clone(),
            AuditLogLevel::Info, // Default to Info level
        ));

        // Initialize JWT keys if JWT secret is provided
        let jwt_encoding_key = config.jwt_secret.as_ref().map(|secret| {
            jsonwebtoken::EncodingKey::from_secret(secret.as_ref())
        });
        let jwt_decoding_key = config.jwt_secret.as_ref().map(|secret| {
            jsonwebtoken::DecodingKey::from_secret(secret.as_ref())
        });

        Ok(Self {
            config: config.clone(),
            api_keys,
            jwt_secret: config.jwt_secret.clone(),
            active_sessions,
            rate_limiters,
            stats,
            audit_logger,
            // Enhanced security features
            encryption_key: None,
            jwt_encoding_key,
            jwt_decoding_key,
            security_monitor: Arc::new(SecurityMonitor::new(
                config.security.max_failed_attempts,
                Duration::from_secs(config.security.lockout_duration_seconds),
            )),
        })
    }

    /// Authenticate a request with enhanced security
    pub async fn authenticate(
        &self,
        auth_header: Option<&str>,
        client_info: Option<ClientInfo>,
    ) -> RhemaResult<AuthResult> {
        let client_ip = client_info.as_ref().and_then(|c| c.ip_address.clone());
        let user_agent = client_info.as_ref().and_then(|c| c.user_agent.clone());
        let _client_type = client_info.as_ref().map(|c| c.client_type.clone());

        // Enhanced security: Check for brute force attempts
        let unknown = "unknown".to_string();
        let client_id = client_ip.as_ref().unwrap_or(&unknown);
        if self.security_monitor.is_locked_out(client_id).await {
            self.security_monitor
                .record_security_event(
                    SecurityEventType::BruteForceAttempt,
                    client_ip.clone(),
                    None,
                    "Account locked due to multiple failed attempts".to_string(),
                    SecuritySeverity::High,
                )
                .await;

            return Ok(AuthResult {
                authenticated: false,
                user_id: None,
                permissions: Vec::new(),
                token_id: None,
                error: Some("Account temporarily locked due to security policy".to_string()),
                session_id: None,
            });
        }

        // Log authentication attempt
        self.audit_logger
            .log(
                AuditEventType::Authentication,
                "Authentication attempt",
                AuditResult::Success,
                None,
                client_ip.clone(),
                user_agent.clone(),
                None,
                None,
                HashMap::new(),
            )
            .await;

        if !self.config.enabled {
            return Ok(AuthResult {
                authenticated: true,
                user_id: None,
                permissions: vec!["*".to_string()],
                token_id: None,
                error: None,
                session_id: None,
            });
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        stats.last_auth_time = Some(chrono::Utc::now());
        stats.active_tokens = self.api_keys.read().await.len();
        stats.active_sessions = self.active_sessions.read().await.len();

        let auth_header = match auth_header {
            Some(header) => header,
            None => {
                stats.failed_auths += 1;
                stats.security_violations += 1;

                self.audit_logger
                    .log(
                        AuditEventType::SecurityViolation,
                        "Missing authorization header",
                        AuditResult::Failure,
                        None,
                        client_ip,
                        user_agent,
                        None,
                        None,
                        HashMap::new(),
                    )
                    .await;

                return Ok(AuthResult {
                    authenticated: false,
                    user_id: None,
                    permissions: Vec::new(),
                    token_id: None,
                    error: Some("Missing authorization header".to_string()),
                    session_id: None,
                });
            }
        };

        // Validate input
        if !self.validate_auth_header(auth_header) {
            stats.failed_auths += 1;
            stats.security_violations += 1;

            self.audit_logger
                .log(
                    AuditEventType::SecurityViolation,
                    "Invalid authorization header format",
                    AuditResult::Failure,
                    None,
                    client_ip,
                    user_agent,
                    None,
                    None,
                    HashMap::new(),
                )
                .await;

            return Ok(AuthResult {
                authenticated: false,
                user_id: None,
                permissions: Vec::new(),
                token_id: None,
                error: Some("Invalid authorization header format".to_string()),
                session_id: None,
            });
        }

        let result = if auth_header.starts_with("Bearer ") {
            self.authenticate_bearer_token(&auth_header[7..], client_info)
                .await
        } else if auth_header.starts_with("ApiKey ") {
            self.authenticate_api_key(&auth_header[7..], client_info)
                .await
        } else {
            self.authenticate_api_key(auth_header, client_info).await
        };

        match &result {
            Ok(auth_result) => {
                if auth_result.authenticated {
                    stats.successful_auths += 1;

                    // Reset failed attempts on successful authentication
                    self.security_monitor.reset_failed_attempts(client_id).await;

                    self.audit_logger
                        .log(
                            AuditEventType::Authentication,
                            "Authentication successful",
                            AuditResult::Success,
                            auth_result.user_id.clone(),
                            client_ip,
                            user_agent,
                            None,
                            auth_result.session_id.clone(),
                            HashMap::new(),
                        )
                        .await;
                } else {
                    stats.failed_auths += 1;

                    // Record failed attempt for security monitoring
                    self.security_monitor.record_failed_attempt(client_id).await;

                    self.audit_logger
                        .log(
                            AuditEventType::Authentication,
                            "Authentication failed",
                            AuditResult::Failure,
                            None,
                            client_ip,
                            user_agent,
                            None,
                            None,
                            HashMap::new(),
                        )
                        .await;
                }
            }
            Err(_) => {
                stats.failed_auths += 1;
                stats.security_violations += 1;

                // Record failed attempt for security monitoring
                self.security_monitor.record_failed_attempt(client_id).await;

                self.audit_logger
                    .log(
                        AuditEventType::SecurityViolation,
                        "Authentication error",
                        AuditResult::Failure,
                        None,
                        client_ip,
                        user_agent,
                        None,
                        None,
                        HashMap::new(),
                    )
                    .await;
            }
        }

        result
    }

    /// Validate authorization header format
    fn validate_auth_header(&self, header: &str) -> bool {
        if header.is_empty() {
            return false;
        }

        // Check for common injection patterns
        let dangerous_patterns = [
            "javascript:",
            "data:",
            "vbscript:",
            "onload=",
            "onerror=",
            "<script",
            "</script>",
            "<?php",
            "<%",
            "%>",
            "{{",
            "}}",
        ];

        let header_lower = header.to_lowercase();
        for pattern in &dangerous_patterns {
            if header_lower.contains(pattern) {
                return false;
            }
        }

        true
    }

    /// Authenticate API key with enhanced security
    async fn authenticate_api_key(
        &self,
        api_key: &str,
        client_info: Option<ClientInfo>,
    ) -> RhemaResult<AuthResult> {
        // Validate API key format
        if !self.validate_api_key_format(api_key) {
            return Ok(AuthResult {
                authenticated: false,
                user_id: None,
                permissions: Vec::new(),
                token_id: None,
                error: Some("Invalid API key format".to_string()),
                session_id: None,
            });
        }

        // Check if API key is configured
        if let Some(configured_key) = &self.config.api_key {
            if api_key == configured_key {
                let session_id = self
                    .create_session("api_user", vec!["*".to_string()], client_info)
                    .await?;

                return Ok(AuthResult {
                    authenticated: true,
                    user_id: Some("api_user".to_string()),
                    permissions: vec!["*".to_string()],
                    token_id: Some(Uuid::new_v4().to_string()),
                    error: None,
                    session_id: Some(session_id),
                });
            }
        }

        // Check stored API keys
        let mut api_keys = self.api_keys.write().await;
        if let Some(token) = api_keys.get_mut(api_key) {
            if self.is_token_valid(token).await {
                // Update token usage
                token.last_used = Some(chrono::Utc::now());
                token.usage_count += 1;

                let session_id = self
                    .create_session(
                        token.user_id.as_ref().unwrap_or(&"unknown".to_string()),
                        token.permissions.clone(),
                        client_info,
                    )
                    .await?;

                return Ok(AuthResult {
                    authenticated: true,
                    user_id: token.user_id.clone(),
                    permissions: token.permissions.clone(),
                    token_id: Some(token.id.clone()),
                    error: None,
                    session_id: Some(session_id),
                });
            }
        }

        Ok(AuthResult {
            authenticated: false,
            user_id: None,
            permissions: Vec::new(),
            token_id: None,
            error: Some("Invalid API key".to_string()),
            session_id: None,
        })
    }

    /// Validate API key format
    fn validate_api_key_format(&self, api_key: &str) -> bool {
        if api_key.len() < 16 || api_key.len() > 256 {
            return false;
        }

        // Check for valid characters (alphanumeric, hyphens, underscores)
        if !api_key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return false;
        }

        true
    }

    /// Authenticate Bearer token (JWT) with enhanced security
    async fn authenticate_bearer_token(
        &self,
        token: &str,
        client_info: Option<ClientInfo>,
    ) -> RhemaResult<AuthResult> {
        // Validate JWT format
        if !self.validate_jwt_format(token) {
            return Ok(AuthResult {
                authenticated: false,
                user_id: None,
                permissions: Vec::new(),
                token_id: None,
                error: Some("Invalid JWT format".to_string()),
                session_id: None,
            });
        }

        if let Some(secret) = &self.jwt_secret {
            match self.verify_jwt_token(token, secret).await {
                Ok(claims) => {
                    let user_id = claims
                        .get("sub")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    let permissions: Vec<String> = claims
                        .get("permissions")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect()
                        })
                        .unwrap_or_default();

                    let session_id = if let Some(ref uid) = user_id {
                        self.create_session(uid, permissions.clone(), client_info)
                            .await?
                    } else {
                        Uuid::new_v4().to_string()
                    };

                    Ok(AuthResult {
                        authenticated: true,
                        user_id,
                        permissions,
                        token_id: Some(Uuid::new_v4().to_string()),
                        error: None,
                        session_id: Some(session_id),
                    })
                }
                Err(e) => Ok(AuthResult {
                    authenticated: false,
                    user_id: None,
                    permissions: Vec::new(),
                    token_id: None,
                    error: Some(format!("JWT verification failed: {}", e)),
                    session_id: None,
                }),
            }
        } else {
            Ok(AuthResult {
                authenticated: false,
                user_id: None,
                permissions: Vec::new(),
                token_id: None,
                error: Some("JWT authentication not configured".to_string()),
                session_id: None,
            })
        }
    }

    /// Validate JWT format
    fn validate_jwt_format(&self, token: &str) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        // Check each part is valid base64 (including URL-safe base64)
        for part in &parts {
            if part.is_empty() {
                return false;
            }

            // Check for valid base64 characters (both standard and URL-safe)
            if !part
                .chars()
                .all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_' || c == '=')
            {
                return false;
            }
        }

        true
    }

    /// Create a new session
    async fn create_session(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        client_info: Option<ClientInfo>,
    ) -> RhemaResult<String> {
        let session_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(24); // 24 hour session

        let session = Session {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            last_activity: now,
            expires_at,
            client_info: client_info.unwrap_or_else(|| ClientInfo {
                ip_address: None,
                user_agent: None,
                client_type: ClientType::Api,
                fingerprint: None,
            }),
            permissions,
            metadata: HashMap::new(),
        };

        self.active_sessions
            .write()
            .await
            .insert(session_id.clone(), session);

        Ok(session_id)
    }

    /// Check rate limiting for a client
    pub async fn check_rate_limit(&self, client_id: &str, client_type: &str) -> bool {
        let limit = match client_type {
            "http" => self.config.rate_limiting.http_requests_per_minute,
            "websocket" => self.config.rate_limiting.websocket_messages_per_minute,
            "unix_socket" => self.config.rate_limiting.unix_socket_messages_per_minute,
            _ => self.config.rate_limiting.http_requests_per_minute,
        };

        let mut rate_limiters = self.rate_limiters.write().await;
        let rate_limiter = rate_limiters
            .entry(client_id.to_string())
            .or_insert_with(|| RateLimitInfo {
                client_id: client_id.to_string(),
                requests: Vec::new(),
                limit,
                window: Duration::from_secs(60),
                last_violation: None,
            });

        let now = Instant::now();

        // Remove expired requests
        rate_limiter
            .requests
            .retain(|&time| now.duration_since(time) < rate_limiter.window);

        if rate_limiter.requests.len() < rate_limiter.limit as usize {
            rate_limiter.requests.push(now);
            true
        } else {
            rate_limiter.last_violation = Some(now);

            // Update statistics
            let mut stats = self.stats.write().await;
            stats.rate_limit_violations += 1;

            // Log rate limit violation
            self.audit_logger
                .log(
                    AuditEventType::RateLimitViolation,
                    "Rate limit exceeded",
                    AuditResult::RateLimited,
                    None,
                    None,
                    None,
                    None,
                    None,
                    HashMap::new(),
                )
                .await;

            false
        }
    }

    /// Verify JWT token with enhanced security
    async fn verify_jwt_token(&self, token: &str, secret: &str) -> RhemaResult<serde_json::Value> {
        // Use proper JWT library for secure verification
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.leeway = 0; // No leeway for security

        match decode::<serde_json::Value>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;

                // Additional security checks
                if let Some(exp) = claims.get("exp").and_then(|v| v.as_u64()) {
                    let now = chrono::Utc::now().timestamp() as u64;
                    if exp < now {
                        return Err(RhemaError::InvalidInput("JWT token expired".to_string()));
                    }
                }

                if let Some(iat) = claims.get("iat").and_then(|v| v.as_u64()) {
                    let now = chrono::Utc::now().timestamp() as u64;
                    if iat > now {
                        return Err(RhemaError::InvalidInput(
                            "JWT token issued in the future".to_string(),
                        ));
                    }
                }

                // Check for token reuse (basic implementation)
                if let Some(jti) = claims.get("jti").and_then(|v| v.as_str()) {
                    // In a production system, you'd check against a blacklist
                    // For now, we'll just validate the format
                    if jti.is_empty() {
                        return Err(RhemaError::InvalidInput("Invalid JWT ID".to_string()));
                    }
                }

                Ok(claims)
            }
            Err(e) => {
                // Log security event for failed JWT verification
                self.security_monitor
                    .record_security_event(
                        SecurityEventType::FailedAuthentication,
                        None,
                        None,
                        format!("JWT verification failed: {}", e),
                        SecuritySeverity::Medium,
                    )
                    .await;

                Err(RhemaError::InvalidInput(format!(
                    "JWT verification failed: {}",
                    e
                )))
            }
        }
    }

    /// Check if a token is valid
    async fn is_token_valid(&self, token: &AuthToken) -> bool {
        if let Some(expires_at) = token.expires_at {
            if chrono::Utc::now() > expires_at {
                return false;
            }
        }
        true
    }

    /// Create a new API key with enhanced security
    pub async fn create_api_key(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        ttl_hours: Option<u64>,
    ) -> RhemaResult<String> {
        // Validate user_id
        if !self.validate_user_id(user_id) {
            return Err(RhemaError::InvalidInput(
                "Invalid user ID format".to_string(),
            ));
        }

        // Validate permissions
        if !self.validate_permissions(&permissions) {
            return Err(RhemaError::InvalidInput("Invalid permissions".to_string()));
        }

        let api_key = Uuid::new_v4().to_string();
        let expires_at =
            ttl_hours.map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64));

        let token = AuthToken {
            id: Uuid::new_v4().to_string(),
            token_type: TokenType::ApiKey,
            user_id: Some(user_id.to_string()),
            permissions,
            created_at: chrono::Utc::now(),
            expires_at,
            metadata: HashMap::new(),
            last_used: None,
            usage_count: 0,
        };

        self.api_keys.write().await.insert(api_key.clone(), token);

        // Log API key creation
        self.audit_logger
            .log(
                AuditEventType::TokenManagement,
                "API key created",
                AuditResult::Success,
                Some(user_id.to_string()),
                None,
                None,
                None,
                None,
                HashMap::new(),
            )
            .await;

        Ok(api_key)
    }

    /// Validate user ID format
    fn validate_user_id(&self, user_id: &str) -> bool {
        if user_id.is_empty() || user_id.len() > 100 {
            return false;
        }

        // Check for valid characters
        if !user_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
        {
            return false;
        }

        true
    }

    /// Validate permissions
    fn validate_permissions(&self, permissions: &[String]) -> bool {
        for permission in permissions {
            if permission.is_empty() || permission.len() > 50 {
                return false;
            }

            // Check for valid characters
            if !permission
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':')
            {
                return false;
            }
        }

        true
    }

    /// Create JWT token with enhanced security
    pub async fn create_jwt_token(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        ttl_hours: u64,
    ) -> RhemaResult<String> {
        // Validate inputs
        if !self.validate_user_id(user_id) {
            return Err(RhemaError::InvalidInput(
                "Invalid user ID format".to_string(),
            ));
        }

        if !self.validate_permissions(&permissions) {
            return Err(RhemaError::InvalidInput("Invalid permissions".to_string()));
        }

        if ttl_hours == 0 || ttl_hours > 8760 {
            // Max 1 year
            return Err(RhemaError::InvalidInput("Invalid TTL hours".to_string()));
        }

        if let Some(secret) = &self.jwt_secret {
            let now = chrono::Utc::now();
            let exp = now + chrono::Duration::hours(ttl_hours as i64);

            let claims = JwtClaims {
                sub: user_id.to_string(),
                iat: now.timestamp(),
                exp: exp.timestamp(),
                permissions,
            };

            // Use proper JWT library for secure token creation
            let encoding_key = EncodingKey::from_secret(secret.as_ref());

            match encode(&Header::default(), &claims, &encoding_key) {
                Ok(token) => {
                    // Log JWT token creation
                    self.audit_logger
                        .log(
                            AuditEventType::TokenManagement,
                            "JWT token created",
                            AuditResult::Success,
                            Some(user_id.to_string()),
                            None,
                            None,
                            None,
                            None,
                            HashMap::new(),
                        )
                        .await;

                    Ok(token)
                }
                Err(e) => {
                    error!("Failed to create JWT token: {}", e);
                    Err(RhemaError::InvalidInput(format!(
                        "JWT creation failed: {}",
                        e
                    )))
                }
            }
        } else {
            Err(RhemaError::InvalidInput(
                "JWT secret not configured".to_string(),
            ))
        }
    }

    /// Revoke API key
    pub async fn revoke_api_key(&self, api_key: &str) -> RhemaResult<bool> {
        let was_present = self.api_keys.write().await.remove(api_key).is_some();

        if was_present {
            // Log API key revocation
            self.audit_logger
                .log(
                    AuditEventType::TokenManagement,
                    "API key revoked",
                    AuditResult::Success,
                    None,
                    None,
                    None,
                    None,
                    None,
                    HashMap::new(),
                )
                .await;
        }

        Ok(was_present)
    }

    /// Get authentication statistics
    pub async fn stats(&self) -> AuthStats {
        let mut stats = self.stats.read().await.clone();
        stats.active_tokens = self.api_keys.read().await.len();
        stats.active_sessions = self.active_sessions.read().await.len();
        stats
    }

    /// Check if user has permission
    pub async fn has_permission(&self, auth_result: &AuthResult, permission: &str) -> bool {
        if !auth_result.authenticated {
            return false;
        }

        auth_result.permissions.contains(&"*".to_string())
            || auth_result.permissions.contains(&permission.to_string())
    }

    /// Validate origin for CORS
    pub fn validate_origin(&self, origin: &str) -> bool {
        if self.config.allowed_origins.is_empty() {
            return true;
        }

        self.config
            .allowed_origins
            .iter()
            .any(|allowed| allowed == "*" || allowed == origin || origin.ends_with(allowed))
    }

    /// Cleanup expired tokens and sessions
    pub async fn cleanup_expired_tokens(&self) -> RhemaResult<usize> {
        let now = chrono::Utc::now();
        let mut cleaned_count = 0;

        // Cleanup expired API keys
        let mut api_keys = self.api_keys.write().await;
        api_keys.retain(|_, token| {
            if let Some(expires_at) = token.expires_at {
                if now > expires_at {
                    cleaned_count += 1;
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });

        // Cleanup expired sessions
        let mut sessions = self.active_sessions.write().await;
        sessions.retain(|_, session| {
            if now > session.expires_at {
                cleaned_count += 1;
                false
            } else {
                true
            }
        });

        // Cleanup expired rate limiters
        let mut rate_limiters = self.rate_limiters.write().await;
        rate_limiters.retain(|_, limiter| {
            let now_instant = Instant::now();
            limiter
                .requests
                .retain(|&time| now_instant.duration_since(time) < limiter.window);
            !limiter.requests.is_empty()
        });

        Ok(cleaned_count)
    }

    /// Get session information
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        self.active_sessions.read().await.get(session_id).cloned()
    }

    /// Update session activity
    pub async fn update_session_activity(&self, session_id: &str) -> RhemaResult<bool> {
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.last_activity = chrono::Utc::now();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Revoke session
    pub async fn revoke_session(&self, session_id: &str) -> RhemaResult<bool> {
        let was_present = self
            .active_sessions
            .write()
            .await
            .remove(session_id)
            .is_some();

        if was_present {
            // Log session revocation
            self.audit_logger
                .log(
                    AuditEventType::SessionManagement,
                    "Session revoked",
                    AuditResult::Success,
                    None,
                    None,
                    None,
                    None,
                    Some(session_id.to_string()),
                    HashMap::new(),
                )
                .await;
        }

        Ok(was_present)
    }

    /// Enhanced JWT token validation with proper signature verification
    async fn verify_jwt_token_enhanced(&self, token: &str) -> RhemaResult<serde_json::Value> {
        if let Some(decoding_key) = &self.jwt_decoding_key {
            let token_data = decode::<JwtClaims>(token, decoding_key, &Validation::default())
                .map_err(|e| {
                    error!("JWT token validation failed: {}", e);
                    RhemaError::AuthenticationError(format!("Invalid JWT token: {}", e))
                })?;

            // Check if token is expired
            let now = chrono::Utc::now().timestamp();
            if token_data.claims.exp < now {
                return Err(RhemaError::AuthenticationError(
                    "JWT token has expired".to_string(),
                ));
            }

            // Check if token is issued in the future (clock skew protection)
            if token_data.claims.iat > now + 300 {
                // 5 minutes clock skew tolerance
                return Err(RhemaError::AuthenticationError(
                    "JWT token issued in the future".to_string(),
                ));
            }

            // Convert claims to JSON for return
            let claims_json = serde_json::json!({
                "sub": token_data.claims.sub,
                "iat": token_data.claims.iat,
                "exp": token_data.claims.exp,
                "permissions": token_data.claims.permissions
            });

            Ok(claims_json)
        } else {
            Err(RhemaError::AuthenticationError(
                "JWT secret not configured".to_string(),
            ))
        }
    }

    /// Create a refresh token for JWT
    pub async fn create_refresh_token(&self, user_id: &str, ttl_hours: u64) -> RhemaResult<String> {
        if let Some(encoding_key) = &self.jwt_encoding_key {
            let now = chrono::Utc::now();
            let exp = now + chrono::Duration::hours(ttl_hours as i64);

            let claims = JwtClaims {
                sub: user_id.to_string(),
                iat: now.timestamp(),
                exp: exp.timestamp(),
                permissions: vec!["refresh".to_string()],
            };

            let header = Header::default();
            encode(&header, &claims, encoding_key).map_err(|e| {
                error!("Failed to create refresh token: {}", e);
                RhemaError::AuthenticationError(format!("Failed to create refresh token: {}", e))
            })
        } else {
            Err(RhemaError::AuthenticationError(
                "JWT secret not configured".to_string(),
            ))
        }
    }

    /// Refresh an access token using a refresh token
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
        access_token_ttl_hours: u64,
    ) -> RhemaResult<String> {
        // Verify the refresh token
        let claims = self.verify_jwt_token_enhanced(refresh_token).await?;

        // Check if it's actually a refresh token
        let permissions = claims["permissions"].as_array().ok_or_else(|| {
            RhemaError::AuthenticationError("Invalid token permissions".to_string())
        })?;

        if !permissions.iter().any(|p| p.as_str() == Some("refresh")) {
            return Err(RhemaError::AuthenticationError(
                "Token is not a refresh token".to_string(),
            ));
        }

        // Extract user ID from refresh token
        let user_id = claims["sub"]
            .as_str()
            .ok_or_else(|| RhemaError::AuthenticationError("Invalid token subject".to_string()))?;

        // Create new access token
        self.create_jwt_token(
            user_id,
            vec!["read".to_string(), "write".to_string()],
            access_token_ttl_hours,
        )
        .await
    }

    /// Enhanced API key validation with additional security checks
    async fn validate_api_key_enhanced(&self, api_key: &str) -> RhemaResult<AuthToken> {
        let api_keys = self.api_keys.read().await;

        if let Some(token) = api_keys.get(api_key) {
            // Check if token is expired
            if let Some(expires_at) = token.expires_at {
                if expires_at < chrono::Utc::now() {
                    return Err(RhemaError::AuthenticationError(
                        "API key has expired".to_string(),
                    ));
                }
            }

            // Check if token is revoked
            if token
                .metadata
                .get("revoked")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                return Err(RhemaError::AuthenticationError(
                    "API key has been revoked".to_string(),
                ));
            }

            // Check usage limits
            if let Some(max_usage) = token.metadata.get("max_usage").and_then(|v| v.as_u64()) {
                if token.usage_count >= max_usage {
                    return Err(RhemaError::AuthenticationError(
                        "API key usage limit exceeded".to_string(),
                    ));
                }
            }

            Ok(token.clone())
        } else {
            Err(RhemaError::AuthenticationError(
                "Invalid API key".to_string(),
            ))
        }
    }

    /// Create API key with enhanced security features
    pub async fn create_enhanced_api_key(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        ttl_hours: Option<u64>,
        max_usage: Option<u64>,
        description: Option<String>,
    ) -> RhemaResult<String> {
        // Validate inputs
        if !self.validate_user_id(user_id) {
            return Err(RhemaError::ValidationError(
                "Invalid user ID format".to_string(),
            ));
        }

        if !self.validate_permissions(&permissions) {
            return Err(RhemaError::ValidationError(
                "Invalid permissions".to_string(),
            ));
        }

        // Generate secure API key
        let api_key = self.generate_secure_api_key();

        // Calculate expiration
        let expires_at =
            ttl_hours.map(|hours| chrono::Utc::now() + chrono::Duration::hours(hours as i64));

        // Create token with enhanced metadata
        let mut metadata = HashMap::new();
        if let Some(max_usage) = max_usage {
            metadata.insert(
                "max_usage".to_string(),
                serde_json::Value::Number(max_usage.into()),
            );
        }
        if let Some(ref desc) = description {
            metadata.insert(
                "description".to_string(),
                serde_json::Value::String(desc.clone()),
            );
        }
        metadata.insert(
            "created_by".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        metadata.insert(
            "key_type".to_string(),
            serde_json::Value::String("enhanced".to_string()),
        );

        let token = AuthToken {
            id: Uuid::new_v4().to_string(),
            token_type: TokenType::ApiKey,
            user_id: Some(user_id.to_string()),
            permissions,
            created_at: chrono::Utc::now(),
            expires_at,
            metadata,
            last_used: None,
            usage_count: 0,
        };

        // Store token
        let mut api_keys = self.api_keys.write().await;
        api_keys.insert(api_key.clone(), token);

        // Log the creation
        let mut details = HashMap::new();
        details.insert(
            "key_type".to_string(),
            serde_json::Value::String("enhanced".to_string()),
        );
        if let Some(max_usage) = max_usage {
            details.insert(
                "max_usage".to_string(),
                serde_json::Value::Number(max_usage.into()),
            );
        }
        if let Some(ref desc) = description {
            details.insert(
                "description".to_string(),
                serde_json::Value::String(desc.clone()),
            );
        }

        self.audit_logger
            .log(
                crate::auth::AuditEventType::TokenManagement,
                "create_enhanced_api_key",
                crate::auth::AuditResult::Success,
                Some(user_id.to_string()),
                None,
                None,
                None,
                None,
                details,
            )
            .await;

        Ok(api_key)
    }

    /// Generate a cryptographically secure API key
    fn generate_secure_api_key(&self) -> String {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use rand::{thread_rng, RngCore};

        let mut bytes = [0u8; 32];
        thread_rng().fill_bytes(&mut bytes);

        // Format: rhema_<base64>_<timestamp>
        let base64_part = URL_SAFE_NO_PAD.encode(bytes);
        let timestamp = chrono::Utc::now().timestamp();

        format!("rhema_{}_{}", base64_part, timestamp)
    }

    /// Enhanced session management with security features
    pub async fn create_secure_session(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        client_info: Option<ClientInfo>,
        session_ttl_hours: u64,
    ) -> RhemaResult<String> {
        let session_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(session_ttl_hours as i64);

        let client_info_clone = client_info.clone();
        let session = Session {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            last_activity: now,
            expires_at,
            client_info: client_info.unwrap_or_else(|| ClientInfo {
                ip_address: None,
                user_agent: None,
                client_type: ClientType::Http,
                fingerprint: None,
            }),
            permissions,
            metadata: HashMap::new(),
        };

        let mut sessions = self.active_sessions.write().await;
        sessions.insert(session_id.clone(), session);

        // Log session creation
        self.audit_logger
            .log(
                crate::auth::AuditEventType::Authentication,
                "create_secure_session",
                crate::auth::AuditResult::Success,
                Some(user_id.to_string()),
                client_info_clone
                    .as_ref()
                    .and_then(|c| c.ip_address.clone()),
                client_info_clone
                    .as_ref()
                    .and_then(|c| c.user_agent.clone()),
                None,
                Some(session_id.clone()),
                HashMap::new(),
            )
            .await;

        Ok(session_id)
    }

    /// Validate session with enhanced security checks
    pub async fn validate_session_enhanced(
        &self,
        session_id: &str,
        client_info: Option<ClientInfo>,
    ) -> RhemaResult<AuthResult> {
        if let Some(session) = self.get_session(session_id).await {
            // Check if session is expired
            if session.expires_at < Utc::now() {
                self.revoke_session(session_id).await?;
                return Ok(AuthResult {
                    authenticated: false,
                    user_id: None,
                    permissions: vec![],
                    token_id: None,
                    error: Some("Session expired".to_string()),
                    session_id: None,
                });
            }

            // Check IP address change if configured
            if self.config.security.invalidate_session_on_ip_change {
                if let Some(current_client_info) = &client_info {
                    if let Some(current_ip) = &current_client_info.ip_address {
                        if let Some(session_ip) = &session.client_info.ip_address {
                            if current_ip != session_ip {
                                // IP address changed, invalidate session
                                self.revoke_session(session_id).await?;
                                return Ok(AuthResult {
                                    authenticated: false,
                                    user_id: None,
                                    permissions: vec![],
                                    token_id: None,
                                    error: Some("Session invalidated due to IP address change".to_string()),
                                    session_id: None,
                                });
                            }
                        }
                    }
                }
            }

            // Update session activity
            self.update_session_activity(session_id).await?;

            Ok(AuthResult {
                authenticated: true,
                user_id: Some(session.user_id),
                permissions: session.permissions,
                token_id: None,
                error: None,
                session_id: Some(session_id.to_string()),
            })
        } else {
            Ok(AuthResult {
                authenticated: false,
                user_id: None,
                permissions: vec![],
                token_id: None,
                error: Some("Invalid session".to_string()),
                session_id: None,
            })
        }
    }

    /// Get a reference to the security monitor (for testing)
    pub fn security_monitor(&self) -> &SecurityMonitor {
        &self.security_monitor
    }

    /// Get a reference to the audit logger (for testing)
    pub fn audit_logger(&self) -> &AuditLogger {
        &self.audit_logger
    }
}

impl SecurityMonitor {
    pub fn new(alert_threshold: u32, lockout_duration: Duration) -> Self {
        Self {
            failed_attempts: Arc::new(RwLock::new(HashMap::new())),
            suspicious_ips: Arc::new(RwLock::new(HashMap::new())),
            security_events: Arc::new(RwLock::new(Vec::new())),
            alert_threshold,
            lockout_duration,
        }
    }

    pub async fn record_security_event(
        &self,
        event_type: SecurityEventType,
        client_ip: Option<String>,
        user_id: Option<String>,
        details: String,
        severity: SecuritySeverity,
    ) {
        let event = SecurityEvent {
            timestamp: Instant::now(),
            event_type: event_type.clone(),
            client_ip,
            user_id,
            details: details.clone(),
            severity: severity.clone(),
        };

        let mut events = self.security_events.write().await;
        events.push(event);

        // Keep only last 1000 events to prevent memory issues
        if events.len() > 1000 {
            events.remove(0);
        }

        // Log high severity events
        match severity {
            SecuritySeverity::Critical | SecuritySeverity::High => {
                error!("SECURITY ALERT: {:?} - {}", event_type, details);
            }
            SecuritySeverity::Medium => {
                warn!("Security event: {:?} - {}", event_type, details);
            }
            SecuritySeverity::Low => {
                info!("Security event: {:?} - {}", event_type, details);
            }
        }
    }

    pub async fn record_failed_attempt(&self, identifier: &str) -> bool {
        let mut attempts = self.failed_attempts.write().await;
        let count = attempts.entry(identifier.to_string()).or_insert(0);
        *count += 1;

        if *count >= self.alert_threshold {
            self.record_security_event(
                SecurityEventType::BruteForceAttempt,
                Some(identifier.to_string()),
                None,
                format!("Multiple failed attempts: {}", count),
                SecuritySeverity::High,
            )
            .await;
            return false; // Locked out
        }
        true // Still allowed
    }

    pub async fn is_locked_out(&self, identifier: &str) -> bool {
        let attempts = self.failed_attempts.read().await;
        if let Some(count) = attempts.get(identifier) {
            *count >= self.alert_threshold
        } else {
            false
        }
    }

    pub async fn reset_failed_attempts(&self, identifier: &str) {
        let mut attempts = self.failed_attempts.write().await;
        attempts.remove(identifier);
    }

    pub async fn get_security_events(&self) -> Vec<SecurityEvent> {
        self.security_events.read().await.clone()
    }
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(enabled: bool, log_file: Option<PathBuf>, log_level: AuditLogLevel) -> Self {
        Self {
            log_file,
            enabled,
            log_level,
        }
    }

    /// Log an audit event
    pub async fn log(
        &self,
        event_type: AuditEventType,
        action: &str,
        result: AuditResult,
        user_id: Option<String>,
        client_ip: Option<String>,
        user_agent: Option<String>,
        resource: Option<String>,
        session_id: Option<String>,
        details: HashMap<String, serde_json::Value>,
    ) {
        if !self.enabled {
            return;
        }

        let entry = AuditLogEntry {
            timestamp: chrono::Utc::now(),
            event_type,
            user_id,
            client_ip,
            user_agent,
            resource,
            action: action.to_string(),
            result,
            details,
            session_id,
        };

        let log_line = serde_json::to_string(&entry)
            .unwrap_or_else(|_| format!("AUDIT_LOG_ERROR: Failed to serialize audit entry"));

        // Log to console
        match entry.result {
            AuditResult::Success => info!("AUDIT: {}", log_line),
            AuditResult::Failure => error!("AUDIT: {}", log_line),
            AuditResult::Denied => warn!("AUDIT: {}", log_line),
            AuditResult::RateLimited => warn!("AUDIT: {}", log_line),
        }

        // Log to file if configured
        if let Some(ref log_file) = self.log_file {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_file) {
                let _ = writeln!(file, "{}", log_line);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    iat: i64,
    exp: i64,
    permissions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::{AuthConfig, RateLimitConfig};

    #[tokio::test]
    async fn test_auth_manager_creation() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: true,
            api_key: Some("test_key".to_string()),
            jwt_secret: Some("test_secret".to_string()),
            allowed_origins: vec!["*".to_string()],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;
        assert!(auth_manager.config.enabled);
        Ok(())
    }

    #[tokio::test]
    async fn test_auth_disabled() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: false,
            api_key: None,
            jwt_secret: None,
            allowed_origins: vec![],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;
        let result = auth_manager.authenticate(None, None).await?;
        assert!(result.authenticated);
        Ok(())
    }

    #[tokio::test]
    async fn test_api_key_auth() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: true,
            api_key: Some("test_api_key_12345".to_string()),
            jwt_secret: None,
            allowed_origins: vec![],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;
        let client_info = ClientInfo {
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Test Client".to_string()),
            client_type: ClientType::Http,
            fingerprint: None,
        };
        let result = auth_manager
            .authenticate(Some("test_api_key_12345"), Some(client_info))
            .await?;
        assert!(result.authenticated);
        assert_eq!(result.user_id, Some("api_user".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_permission_check() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: true,
            api_key: Some("test_api_key_12345".to_string()),
            jwt_secret: None,
            allowed_origins: vec![],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;
        let client_info = ClientInfo {
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Test Client".to_string()),
            client_type: ClientType::Http,
            fingerprint: None,
        };
        let result = auth_manager
            .authenticate(Some("test_api_key_12345"), Some(client_info))
            .await?;

        assert!(auth_manager.has_permission(&result, "read").await);
        assert!(auth_manager.has_permission(&result, "write").await);
        Ok(())
    }

    #[tokio::test]
    async fn test_cors_validation() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: true,
            api_key: None,
            jwt_secret: None,
            allowed_origins: vec!["https://example.com".to_string()],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;

        assert!(auth_manager.validate_origin("https://example.com"));
        assert!(!auth_manager.validate_origin("https://malicious.com"));
        Ok(())
    }

    #[tokio::test]
    async fn test_rate_limiting() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: true,
            api_key: None,
            jwt_secret: None,
            allowed_origins: vec![],
            rate_limiting: RateLimitConfig {
                http_requests_per_minute: 2,
                websocket_messages_per_minute: 10,
                unix_socket_messages_per_minute: 10,
            },
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;

        // First two requests should be allowed
        assert!(auth_manager.check_rate_limit("test_client", "http").await);
        assert!(auth_manager.check_rate_limit("test_client", "http").await);

        // Third request should be rate limited
        assert!(!auth_manager.check_rate_limit("test_client", "http").await);

        Ok(())
    }

    #[tokio::test]
    async fn test_session_management() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: true,
            api_key: Some("test_api_key_12345".to_string()),
            jwt_secret: None,
            allowed_origins: vec![],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;
        let client_info = ClientInfo {
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Test Client".to_string()),
            client_type: ClientType::Http,
            fingerprint: None,
        };
        let result = auth_manager
            .authenticate(Some("test_api_key_12345"), Some(client_info))
            .await?;

        assert!(result.session_id.is_some());

        let session_id = result.session_id.unwrap();
        let session = auth_manager.get_session(&session_id).await;
        assert!(session.is_some());

        assert!(auth_manager.update_session_activity(&session_id).await?);
        assert!(auth_manager.revoke_session(&session_id).await?);

        let session = auth_manager.get_session(&session_id).await;
        assert!(session.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_input_validation() -> RhemaResult<()> {
        let config = AuthConfig {
            enabled: true,
            api_key: None,
            jwt_secret: None,
            allowed_origins: vec![],
            rate_limiting: RateLimitConfig::default(),
            audit_logging: crate::mcp::AuditLoggingConfig::default(),
            security: crate::mcp::SecurityConfig::default(),
        };

        let auth_manager = AuthManager::new(&config)?;

        let client_info = ClientInfo {
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Test Client".to_string()),
            client_type: ClientType::Http,
            fingerprint: None,
        };

        // Test invalid auth header
        let result = auth_manager
            .authenticate(
                Some("<script>alert('xss')</script>"),
                Some(client_info.clone()),
            )
            .await?;
        assert!(!result.authenticated);

        // Test invalid API key format
        let result = auth_manager
            .authenticate(Some("invalid_key"), Some(client_info))
            .await?;
        assert!(!result.authenticated);

        Ok(())
    }
}
