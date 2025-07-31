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

use crate::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// use std::time::{Duration, Instant};
use uuid::Uuid;
use base64::Engine;

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
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub authenticated: bool,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub token_id: Option<String>,
    pub error: Option<String>,
}

/// Authentication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStats {
    pub total_requests: u64,
    pub successful_auths: u64,
    pub failed_auths: u64,
    pub active_tokens: usize,
    pub last_auth_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Authentication manager
pub struct AuthManager {
    config: super::AuthConfig,
    api_keys: Arc<RwLock<HashMap<String, AuthToken>>>,
    jwt_secret: Option<String>,
    active_sessions: Arc<RwLock<HashMap<String, AuthToken>>>,
    stats: Arc<RwLock<AuthStats>>,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(config: &super::AuthConfig) -> RhemaResult<Self> {
        let api_keys = Arc::new(RwLock::new(HashMap::new()));
        let active_sessions = Arc::new(RwLock::new(HashMap::new()));
        
        let stats = Arc::new(RwLock::new(AuthStats {
            total_requests: 0,
            successful_auths: 0,
            failed_auths: 0,
            active_tokens: 0,
            last_auth_time: None,
        }));

        Ok(Self {
            config: config.clone(),
            api_keys,
            jwt_secret: config.jwt_secret.clone(),
            active_sessions,
            stats,
        })
    }

    /// Authenticate a request
    pub async fn authenticate(&self, auth_header: Option<&str>) -> RhemaResult<AuthResult> {
        if !self.config.enabled {
            return Ok(AuthResult {
                authenticated: true,
                user_id: None,
                permissions: vec!["*".to_string()],
                token_id: None,
                error: None,
            });
        }

        let api_keys_count = self.api_keys.read().await.len();
        let sessions_count = self.active_sessions.read().await.len();
        
        let mut stats = self.stats.write().await;
        stats.active_tokens = api_keys_count + sessions_count;
        stats.total_requests += 1;
        stats.last_auth_time = Some(chrono::Utc::now());

        let auth_header = match auth_header {
            Some(header) => header,
            None => {
                stats.failed_auths += 1;
                return Ok(AuthResult {
                    authenticated: false,
                    user_id: None,
                    permissions: Vec::new(),
                    token_id: None,
                    error: Some("Missing authorization header".to_string()),
                });
            }
        };

        let result = if auth_header.starts_with("Bearer ") {
            self.authenticate_bearer_token(&auth_header[7..]).await
        } else if auth_header.starts_with("ApiKey ") {
            self.authenticate_api_key(&auth_header[7..]).await
        } else {
            self.authenticate_api_key(auth_header).await
        };

        match &result {
            Ok(auth_result) => {
                if auth_result.authenticated {
                    stats.successful_auths += 1;
                } else {
                    stats.failed_auths += 1;
                }
            }
            Err(_) => {
                stats.failed_auths += 1;
            }
        }

        result
    }

    /// Authenticate API key
    async fn authenticate_api_key(&self, api_key: &str) -> RhemaResult<AuthResult> {
        // Check if API key is configured
        if let Some(configured_key) = &self.config.api_key {
            if api_key == configured_key {
                return Ok(AuthResult {
                    authenticated: true,
                    user_id: Some("api_user".to_string()),
                    permissions: vec!["*".to_string()],
                    token_id: Some(Uuid::new_v4().to_string()),
                    error: None,
                });
            }
        }

        // Check stored API keys
        let api_keys = self.api_keys.read().await;
        if let Some(token) = api_keys.get(api_key) {
            if self.is_token_valid(token).await {
                return Ok(AuthResult {
                    authenticated: true,
                    user_id: token.user_id.clone(),
                    permissions: token.permissions.clone(),
                    token_id: Some(token.id.clone()),
                    error: None,
                });
            }
        }

        Ok(AuthResult {
            authenticated: false,
            user_id: None,
            permissions: Vec::new(),
            token_id: None,
            error: Some("Invalid API key".to_string()),
        })
    }

    /// Authenticate Bearer token (JWT)
    async fn authenticate_bearer_token(&self, token: &str) -> RhemaResult<AuthResult> {
        if let Some(secret) = &self.jwt_secret {
            match self.verify_jwt_token(token, secret).await {
                Ok(claims) => Ok(AuthResult {
                    authenticated: true,
                    user_id: claims.get("sub").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    permissions: claims.get("permissions")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                        .unwrap_or_default(),
                    token_id: Some(Uuid::new_v4().to_string()),
                    error: None,
                }),
                Err(e) => Ok(AuthResult {
                    authenticated: false,
                    user_id: None,
                    permissions: Vec::new(),
                    token_id: None,
                    error: Some(format!("JWT verification failed: {}", e)),
                }),
            }
        } else {
            Ok(AuthResult {
                authenticated: false,
                user_id: None,
                permissions: Vec::new(),
                token_id: None,
                error: Some("JWT authentication not configured".to_string()),
            })
        }
    }

    /// Verify JWT token
    async fn verify_jwt_token(&self, token: &str, _secret: &str) -> RhemaResult<serde_json::Value> {
        // In a real implementation, you would use a JWT library like jsonwebtoken
        // For now, we'll implement a simple verification
        
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(RhemaError::InvalidInput("Invalid JWT format".to_string()));
        }

        // Decode header
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[0])
            .map_err(|_| RhemaError::InvalidInput("Invalid JWT header".to_string()))?;
        let _header: serde_json::Value = serde_json::from_slice(&header)
            .map_err(|_| RhemaError::InvalidInput("Invalid JWT header format".to_string()))?;

        // Decode payload
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|_| RhemaError::InvalidInput("Invalid JWT payload".to_string()))?;
        let claims: serde_json::Value = serde_json::from_slice(&payload)
            .map_err(|_| RhemaError::InvalidInput("Invalid JWT payload format".to_string()))?;

        // Check expiration
        if let Some(exp) = claims.get("exp").and_then(|v| v.as_u64()) {
            let now = chrono::Utc::now().timestamp() as u64;
            if exp < now {
                return Err(RhemaError::InvalidInput("JWT token expired".to_string()));
            }
        }

        // In a real implementation, you would verify the signature here
        // For now, we'll just return the claims
        Ok(claims)
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

    /// Create a new API key
    pub async fn create_api_key(&self, user_id: &str, permissions: Vec<String>, ttl_hours: Option<u64>) -> RhemaResult<String> {
        let api_key = Uuid::new_v4().to_string();
        let expires_at = ttl_hours.map(|hours| {
            chrono::Utc::now() + chrono::Duration::hours(hours as i64)
        });

        let token = AuthToken {
            id: Uuid::new_v4().to_string(),
            token_type: TokenType::ApiKey,
            user_id: Some(user_id.to_string()),
            permissions,
            created_at: chrono::Utc::now(),
            expires_at,
            metadata: HashMap::new(),
        };

        let mut api_keys = self.api_keys.write().await;
        api_keys.insert(api_key.clone(), token);

        Ok(api_key)
    }

    /// Create a JWT token
    pub async fn create_jwt_token(&self, user_id: &str, permissions: Vec<String>, ttl_hours: u64) -> RhemaResult<String> {
        let _secret = self.jwt_secret.as_ref()
            .ok_or_else(|| RhemaError::InvalidInput("JWT secret not configured".to_string()))?;

        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(ttl_hours as i64);

        let claims = serde_json::json!({
            "sub": user_id,
            "iat": now.timestamp(),
            "exp": expires_at.timestamp(),
            "permissions": permissions,
        });

        // In a real implementation, you would use a JWT library
        // For now, we'll create a simple token
        let header = serde_json::json!({
            "alg": "HS256",
            "typ": "JWT"
        });

        let header_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::to_string(&header).unwrap().as_bytes());
        let payload_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::to_string(&claims).unwrap().as_bytes());

        // In a real implementation, you would sign the token
        // For now, we'll just concatenate the parts
        let token = format!("{}.{}.signature", header_b64, payload_b64);

        Ok(token)
    }

    /// Revoke an API key
    pub async fn revoke_api_key(&self, api_key: &str) -> RhemaResult<bool> {
        let mut api_keys = self.api_keys.write().await;
        Ok(api_keys.remove(api_key).is_some())
    }

    /// Get authentication statistics
    pub async fn stats(&self) -> AuthStats {
        let api_keys = self.api_keys.read().await;
        let sessions = self.active_sessions.read().await;
        let mut stats = self.stats.write().await;
        
        stats.active_tokens = api_keys.len() + sessions.len();
        let result = stats.clone();
        result
    }

    /// Check if a user has a specific permission
    pub async fn has_permission(&self, auth_result: &AuthResult, permission: &str) -> bool {
        if !auth_result.authenticated {
            return false;
        }

        auth_result.permissions.contains(&"*".to_string()) || 
        auth_result.permissions.contains(&permission.to_string())
    }

    /// Validate CORS origin
    pub fn validate_origin(&self, origin: &str) -> bool {
        if self.config.allowed_origins.contains(&"*".to_string()) {
            return true;
        }

        self.config.allowed_origins.contains(&origin.to_string())
    }

    /// Clean up expired tokens
    pub async fn cleanup_expired_tokens(&self) -> RhemaResult<usize> {
        let mut cleaned = 0;

        // Clean up API keys
        let mut api_keys = self.api_keys.write().await;
        let expired_keys: Vec<String> = {
            let mut expired = Vec::new();
            for (key, token) in api_keys.iter() {
                if !self.is_token_valid(token).await {
                    expired.push(key.clone());
                }
            }
            expired
        };

        for key in expired_keys {
            api_keys.remove(&key);
            cleaned += 1;
        }

        // Clean up sessions
        let mut sessions = self.active_sessions.write().await;
        let expired_sessions: Vec<String> = {
            let mut expired = Vec::new();
            for (id, token) in sessions.iter() {
                if !self.is_token_valid(token).await {
                    expired.push(id.clone());
                }
            }
            expired
        };

        for id in expired_sessions {
            sessions.remove(&id);
            cleaned += 1;
        }

        Ok(cleaned)
    }
}

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    iat: i64,
    exp: i64,
    permissions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auth_disabled() {
        let config = super::super::AuthConfig {
            enabled: false,
            api_key: None,
            jwt_secret: None,
            allowed_origins: vec!["*".to_string()],
        };

        let auth_manager = AuthManager::new(&config).unwrap();
        let result = auth_manager.authenticate(None).await.unwrap();
        
        assert!(result.authenticated);
        assert_eq!(result.permissions, vec!["*"]);
    }

    #[tokio::test]
    async fn test_api_key_auth() {
        let config = super::super::AuthConfig {
            enabled: true,
            api_key: Some("test_key".to_string()),
            jwt_secret: None,
            allowed_origins: vec!["*".to_string()],
        };

        let auth_manager = AuthManager::new(&config).unwrap();
        
        // Test valid API key
        let result = auth_manager.authenticate(Some("ApiKey test_key")).await.unwrap();
        assert!(result.authenticated);
        assert_eq!(result.user_id, Some("api_user".to_string()));

        // Test invalid API key
        let result = auth_manager.authenticate(Some("ApiKey invalid_key")).await.unwrap();
        assert!(!result.authenticated);
    }

    #[tokio::test]
    async fn test_missing_auth_header() {
        let config = super::super::AuthConfig {
            enabled: true,
            api_key: Some("test_key".to_string()),
            jwt_secret: None,
            allowed_origins: vec!["*".to_string()],
        };

        let auth_manager = AuthManager::new(&config).unwrap();
        let result = auth_manager.authenticate(None).await.unwrap();
        
        assert!(!result.authenticated);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_permission_check() {
        let config = super::super::AuthConfig {
            enabled: true,
            api_key: Some("test_key".to_string()),
            jwt_secret: None,
            allowed_origins: vec!["*".to_string()],
        };

        let auth_manager = AuthManager::new(&config).unwrap();
        let auth_result = auth_manager.authenticate(Some("ApiKey test_key")).await.unwrap();
        
        assert!(auth_manager.has_permission(&auth_result, "read").await);
        assert!(auth_manager.has_permission(&auth_result, "write").await);
    }

    #[tokio::test]
    async fn test_cors_validation() {
        let config = super::super::AuthConfig {
            enabled: true,
            api_key: None,
            jwt_secret: None,
            allowed_origins: vec!["https://example.com".to_string()],
        };

        let auth_manager = AuthManager::new(&config).unwrap();
        
        assert!(auth_manager.validate_origin("https://example.com"));
        assert!(!auth_manager.validate_origin("https://malicious.com"));
    }
} 