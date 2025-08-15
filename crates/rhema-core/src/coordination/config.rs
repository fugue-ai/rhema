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

use serde::{Deserialize, Serialize};

/// Configuration for coordination client integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    /// Whether coordination is enabled
    pub enabled: bool,
    /// gRPC server endpoint
    pub server_endpoint: String,
    /// Client timeout in seconds
    pub timeout_seconds: u64,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Health check configuration
    pub health_check_config: HealthCheckConfig,
    /// TLS configuration
    pub tls_config: Option<TlsConfig>,
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_endpoint: "http://localhost:50051".to_string(),
            timeout_seconds: 30,
            retry_config: RetryConfig::default(),
            health_check_config: HealthCheckConfig::default(),
            tls_config: None,
        }
    }
}

/// Retry configuration for coordination operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial backoff delay in milliseconds
    pub initial_delay_ms: u64,
    /// Maximum backoff delay in milliseconds
    pub max_delay_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Whether health checks are enabled
    pub enabled: bool,
    /// Health check interval in seconds
    pub interval_seconds: u64,
    /// Health check timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 30,
            timeout_seconds: 5,
        }
    }
}

/// TLS configuration for secure connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate authority file path
    pub ca_cert_path: String,
    /// Client certificate file path
    pub client_cert_path: String,
    /// Client key file path
    pub client_key_path: String,
}
