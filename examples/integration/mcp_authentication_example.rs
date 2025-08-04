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

use rhema_core::RhemaResult;
use rhema_mcp::{
    mcp::{McpConfig, AuthConfig, RateLimitConfig, AuditLoggingConfig},
    auth::{AuthManager, ClientInfo, ClientType},
};
use std::path::PathBuf;
use std::collections::HashMap;

/// Example demonstrating MCP Authentication & Security features
#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("🔐 MCP Authentication & Security Example");
    println!("========================================");

    // 1. Configure authentication with enhanced security
    let auth_config = AuthConfig {
        enabled: true,
        api_key: Some("your-secure-api-key-here".to_string()),
        jwt_secret: Some("your-jwt-secret-key-here".to_string()),
        allowed_origins: vec![
            "https://yourdomain.com".to_string(),
            "http://localhost:3000".to_string(),
        ],
        rate_limiting: RateLimitConfig {
            http_requests_per_minute: 100,
            websocket_messages_per_minute: 1000,
            unix_socket_messages_per_minute: 500,
        },
        audit_logging: AuditLoggingConfig {
            enabled: true,
            log_file: Some(PathBuf::from("audit.log")),
            log_level: "info".to_string(),
            events: vec![
                "authentication".to_string(),
                "authorization".to_string(),
                "security_violation".to_string(),
                "rate_limit_violation".to_string(),
                "session_management".to_string(),
                "token_management".to_string(),
            ],
        },
    };

    // 2. Create authentication manager
    let auth_manager = AuthManager::new(&auth_config)?;
    println!("✅ Authentication manager created successfully");

    // 3. Demonstrate API key authentication
    println!("\n🔑 API Key Authentication:");
    let client_info = ClientInfo {
        ip_address: Some("192.168.1.100".to_string()),
        user_agent: Some("Mozilla/5.0 (Example Browser)".to_string()),
        client_type: ClientType::Http,
        fingerprint: None,
    };

    let auth_result = auth_manager
        .authenticate(Some("your-secure-api-key-here"), Some(client_info.clone()))
        .await?;

    if auth_result.authenticated {
        println!("✅ API key authentication successful");
        println!("   User ID: {:?}", auth_result.user_id);
        println!("   Session ID: {:?}", auth_result.session_id);
        println!("   Permissions: {:?}", auth_result.permissions);
    } else {
        println!("❌ API key authentication failed: {:?}", auth_result.error);
    }

    // 4. Demonstrate JWT token creation and authentication
    println!("\n🎫 JWT Token Authentication:");
    let jwt_token = auth_manager
        .create_jwt_token("user123", vec!["read".to_string(), "write".to_string()], 24)
        .await?;
    println!("✅ JWT token created: {}", jwt_token);

    let jwt_auth_result = auth_manager
        .authenticate(Some(&format!("Bearer {}", jwt_token)), Some(client_info.clone()))
        .await?;

    if jwt_auth_result.authenticated {
        println!("✅ JWT authentication successful");
        println!("   User ID: {:?}", jwt_auth_result.user_id);
        println!("   Session ID: {:?}", jwt_auth_result.session_id);
    } else {
        println!("❌ JWT authentication failed: {:?}", jwt_auth_result.error);
    }

    // 5. Demonstrate permission checking
    println!("\n🔒 Permission Checking:");
    if auth_manager.has_permission(&auth_result, "read").await {
        println!("✅ User has 'read' permission");
    } else {
        println!("❌ User lacks 'read' permission");
    }

    if auth_manager.has_permission(&auth_result, "admin").await {
        println!("✅ User has 'admin' permission");
    } else {
        println!("❌ User lacks 'admin' permission");
    }

    // 6. Demonstrate rate limiting
    println!("\n⏱️ Rate Limiting:");
    let client_id = "test_client_123";
    
    // First few requests should succeed
    for i in 1..=5 {
        let allowed = auth_manager.check_rate_limit(client_id, "http").await;
        println!("   Request {}: {}", i, if allowed { "✅ Allowed" } else { "❌ Rate limited" });
    }

    // 7. Demonstrate session management
    println!("\n🔄 Session Management:");
    if let Some(session_id) = &auth_result.session_id {
        // Get session information
        if let Some(session) = auth_manager.get_session(session_id).await {
            println!("✅ Session retrieved");
            println!("   Session ID: {}", session.id);
            println!("   User ID: {}", session.user_id);
            println!("   Created: {}", session.created_at);
            println!("   Last Activity: {}", session.last_activity);
            println!("   Expires: {}", session.expires_at);
        }

        // Update session activity
        if auth_manager.update_session_activity(session_id).await? {
            println!("✅ Session activity updated");
        }

        // Revoke session
        if auth_manager.revoke_session(session_id).await? {
            println!("✅ Session revoked");
        }
    }

    // 8. Demonstrate input validation
    println!("\n🛡️ Input Validation:");
    
    // Test malicious input
    let malicious_auth = auth_manager
        .authenticate(Some("<script>alert('xss')</script>"), Some(client_info.clone()))
        .await?;
    println!("   Malicious input: {}", if malicious_auth.authenticated { "❌ Accepted" } else { "✅ Rejected" });

    // Test invalid API key format
    let invalid_key_auth = auth_manager
        .authenticate(Some("invalid-key"), Some(client_info.clone()))
        .await?;
    println!("   Invalid API key: {}", if invalid_key_auth.authenticated { "❌ Accepted" } else { "✅ Rejected" });

    // 9. Demonstrate audit logging
    println!("\n📝 Audit Logging:");
    let stats = auth_manager.stats().await;
    println!("   Total requests: {}", stats.total_requests);
    println!("   Successful auths: {}", stats.successful_auths);
    println!("   Failed auths: {}", stats.failed_auths);
    println!("   Rate limit violations: {}", stats.rate_limit_violations);
    println!("   Security violations: {}", stats.security_violations);
    println!("   Active tokens: {}", stats.active_tokens);
    println!("   Active sessions: {}", stats.active_sessions);

    // 10. Demonstrate token management
    println!("\n🔑 Token Management:");
    
    // Create a new API key
    let new_api_key = auth_manager
        .create_api_key("user456", vec!["read".to_string()], Some(24))
        .await?;
    println!("✅ New API key created: {}", new_api_key);

    // Test the new API key
    let new_key_auth = auth_manager
        .authenticate(Some(&new_api_key), Some(client_info.clone()))
        .await?;
    println!("   New key authentication: {}", if new_key_auth.authenticated { "✅ Success" } else { "❌ Failed" });

    // Revoke the API key
    if auth_manager.revoke_api_key(&new_api_key).await? {
        println!("✅ API key revoked");
    }

    // 11. Demonstrate cleanup
    println!("\n🧹 Cleanup:");
    let cleaned_count = auth_manager.cleanup_expired_tokens().await?;
    println!("   Cleaned {} expired tokens/sessions", cleaned_count);

    // 12. Demonstrate CORS validation
    println!("\n🌐 CORS Validation:");
    let valid_origin = auth_manager.validate_origin("https://yourdomain.com");
    println!("   Valid origin: {}", if valid_origin { "✅ Allowed" } else { "❌ Denied" });

    let invalid_origin = auth_manager.validate_origin("https://malicious.com");
    println!("   Invalid origin: {}", if invalid_origin { "❌ Allowed" } else { "✅ Denied" });

    println!("\n🎉 MCP Authentication & Security demonstration completed!");
    println!("Check 'audit.log' for detailed audit trail.");

    Ok(())
}

/// Example showing how to integrate authentication with HTTP requests
#[tokio::test]
async fn test_authentication_integration() -> RhemaResult<()> {
    let auth_config = AuthConfig {
        enabled: true,
        api_key: Some("test-api-key".to_string()),
        jwt_secret: Some("test-jwt-secret".to_string()),
        allowed_origins: vec!["*".to_string()],
        rate_limiting: RateLimitConfig::default(),
        audit_logging: AuditLoggingConfig::default(),
    };

    let auth_manager = AuthManager::new(&auth_config)?;

    // Test API key authentication
    let client_info = ClientInfo {
        ip_address: Some("127.0.0.1".to_string()),
        user_agent: Some("Test Client".to_string()),
        client_type: ClientType::Http,
        fingerprint: None,
    };

    let auth_result = auth_manager
        .authenticate(Some("test-api-key"), Some(client_info))
        .await?;

    assert!(auth_result.authenticated);
    assert_eq!(auth_result.user_id, Some("api_user".to_string()));
    assert!(auth_result.session_id.is_some());

    // Test permission checking
    assert!(auth_manager.has_permission(&auth_result, "read").await);
    assert!(auth_manager.has_permission(&auth_result, "write").await);

    // Test rate limiting
    assert!(auth_manager.check_rate_limit("test_client", "http").await);
    assert!(auth_manager.check_rate_limit("test_client", "http").await);

    Ok(())
}

/// Example showing security features
#[tokio::test]
async fn test_security_features() -> RhemaResult<()> {
    let auth_config = AuthConfig {
        enabled: true,
        api_key: Some("secure-key".to_string()),
        jwt_secret: Some("secure-jwt-secret".to_string()),
        allowed_origins: vec![],
        rate_limiting: RateLimitConfig::default(),
        audit_logging: AuditLoggingConfig::default(),
    };

    let auth_manager = AuthManager::new(&auth_config)?;

    // Test input validation
    let client_info = ClientInfo {
        ip_address: None,
        user_agent: None,
        client_type: ClientType::Http,
        fingerprint: None,
    };

    // Test XSS attempt
    let xss_result = auth_manager
        .authenticate(Some("<script>alert('xss')</script>"), Some(client_info.clone()))
        .await?;
    assert!(!xss_result.authenticated);

    // Test SQL injection attempt
    let sql_result = auth_manager
        .authenticate(Some("'; DROP TABLE users; --"), Some(client_info.clone()))
        .await?;
    assert!(!sql_result.authenticated);

    // Test missing authorization
    let missing_auth = auth_manager.authenticate(None, Some(client_info)).await?;
    assert!(!missing_auth.authenticated);

    Ok(())
} 