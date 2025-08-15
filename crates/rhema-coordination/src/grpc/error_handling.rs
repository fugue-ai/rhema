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

//! Error handling and resilience utilities for gRPC coordination client
//!
//! This module provides comprehensive error handling, retry logic, and connection
//! recovery mechanisms for the gRPC coordination client.

use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, warn, info, debug};

/// Error types for gRPC coordination client
#[derive(Debug, thiserror::Error)]
pub enum GrpcClientError {
    #[error("Connection failed: {message}")]
    Connection { message: String },
    
    #[error("Request timeout: {operation}")]
    Timeout { operation: String },
    
    #[error("gRPC error: {message}")]
    Grpc { message: String },
    
    #[error("Authentication failed: {message}")]
    Authentication { message: String },
    
    #[error("Rate limited: {message}")]
    RateLimited { message: String },
    
    #[error("Server error: {message}")]
    Server { message: String },
    
    #[error("Client error: {message}")]
    Client { message: String },
    
    #[error("Network error: {message}")]
    Network { message: String },
    
    #[error("Serialization error: {message}")]
    Serialization { message: String },
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Connection health status
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Disconnected,
}

/// Resilience manager for gRPC client operations
pub struct ResilienceManager {
    retry_config: RetryConfig,
    connection_health: ConnectionHealth,
    consecutive_failures: u32,
    last_success: Option<std::time::Instant>,
}

impl ResilienceManager {
    pub fn new(retry_config: RetryConfig) -> Self {
        Self {
            retry_config,
            connection_health: ConnectionHealth::Healthy,
            consecutive_failures: 0,
            last_success: None,
        }
    }

    /// Execute an operation with retry logic
    pub async fn execute_with_retry<F, Fut, T, E>(
        &mut self,
        operation: &str,
        f: F,
    ) -> Result<T, GrpcClientError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::error::Error + 'static,
    {
        let mut attempt = 0;
        let mut backoff = self.retry_config.initial_backoff;

        loop {
            attempt += 1;
            debug!("Executing {} (attempt {}/{})", operation, attempt, self.retry_config.max_retries + 1);

            match f().await {
                Ok(result) => {
                    self.on_success();
                    return Ok(result);
                }
                Err(e) => {
                    let error = GrpcClientError::Grpc { message: e.to_string() };
                    self.on_failure(&error);

                    if attempt > self.retry_config.max_retries {
                        error!("Operation {} failed after {} attempts: {:?}", operation, attempt, error);
                        return Err(error);
                    }

                    // Determine if we should retry based on error type
                    if !self.should_retry(&error) {
                        return Err(error);
                    }

                    warn!("Operation {} failed (attempt {}), retrying in {:?}: {:?}", 
                          operation, attempt, backoff, error);

                    // Apply jitter if enabled
                    let sleep_duration = if self.retry_config.jitter {
                        self.add_jitter(backoff)
                    } else {
                        backoff
                    };

                    sleep(sleep_duration).await;

                    // Calculate next backoff
                    backoff = Duration::from_secs_f64(
                        (backoff.as_secs_f64() * self.retry_config.backoff_multiplier)
                            .min(self.retry_config.max_backoff.as_secs_f64())
                    );
                }
            }
        }
    }

    /// Check if an error should be retried
    fn should_retry(&self, error: &GrpcClientError) -> bool {
        match error {
            GrpcClientError::Connection { .. } => true,
            GrpcClientError::Timeout { .. } => true,
            GrpcClientError::Network { .. } => true,
            GrpcClientError::Server { .. } => true,
            GrpcClientError::RateLimited { .. } => true,
            GrpcClientError::Grpc { message } => {
                // Retry on certain gRPC status codes
                message.contains("UNAVAILABLE") || 
                message.contains("DEADLINE_EXCEEDED") ||
                message.contains("RESOURCE_EXHAUSTED")
            }
            _ => false,
        }
    }

    /// Add jitter to backoff duration
    fn add_jitter(&self, duration: Duration) -> Duration {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let jitter_factor = rng.gen_range(0.8..1.2);
        Duration::from_secs_f64(duration.as_secs_f64() * jitter_factor)
    }

    /// Handle successful operation
    fn on_success(&mut self) {
        self.consecutive_failures = 0;
        self.last_success = Some(std::time::Instant::now());
        self.update_health();
        debug!("Operation succeeded, connection health: {:?}", self.connection_health);
    }

    /// Handle failed operation
    fn on_failure(&mut self, error: &GrpcClientError) {
        self.consecutive_failures += 1;
        self.update_health();
        debug!("Operation failed, consecutive failures: {}, health: {:?}", 
               self.consecutive_failures, self.connection_health);
    }

    /// Update connection health based on failure patterns
    fn update_health(&mut self) {
        self.connection_health = match self.consecutive_failures {
            0 => ConnectionHealth::Healthy,
            1..=3 => ConnectionHealth::Degraded,
            4..=10 => ConnectionHealth::Unhealthy,
            _ => ConnectionHealth::Disconnected,
        };
    }

    /// Get current connection health
    pub fn health(&self) -> ConnectionHealth {
        self.connection_health.clone()
    }

    /// Check if connection is healthy enough for operations
    pub fn is_healthy(&self) -> bool {
        matches!(self.connection_health, ConnectionHealth::Healthy | ConnectionHealth::Degraded)
    }

    /// Get connection statistics
    pub fn stats(&self) -> ConnectionStats {
        ConnectionStats {
            health: self.connection_health.clone(),
            consecutive_failures: self.consecutive_failures,
            last_success: self.last_success,
        }
    }

    /// Reset connection health (useful after manual recovery)
    pub fn reset_health(&mut self) {
        self.consecutive_failures = 0;
        self.connection_health = ConnectionHealth::Healthy;
        self.last_success = Some(std::time::Instant::now());
        info!("Connection health reset to healthy");
    }
}

/// Connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub health: ConnectionHealth,
    pub consecutive_failures: u32,
    pub last_success: Option<std::time::Instant>,
}

/// Circuit breaker for protecting against cascading failures
pub struct CircuitBreaker {
    failure_threshold: u32,
    recovery_timeout: Duration,
    state: CircuitBreakerState,
    failure_count: u32,
    last_failure_time: Option<std::time::Instant>,
}

#[derive(Debug, Clone, PartialEq)]
enum CircuitBreakerState {
    Closed,    // Normal operation
    Open,      // Circuit is open, requests fail fast
    HalfOpen,  // Testing if service is recovered
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            last_failure_time: None,
        }
    }

    /// Check if operation is allowed
    pub fn is_allowed(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() >= self.recovery_timeout {
                        self.state = CircuitBreakerState::HalfOpen;
                        info!("Circuit breaker transitioning to half-open state");
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    /// Record a successful operation
    pub fn on_success(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Closed;
                self.failure_count = 0;
                info!("Circuit breaker closed - service recovered");
            }
            CircuitBreakerState::Open => {
                // Should not happen in normal operation
            }
        }
    }

    /// Record a failed operation
    pub fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(std::time::Instant::now());

        match self.state {
            CircuitBreakerState::Closed => {
                if self.failure_count >= self.failure_threshold {
                    self.state = CircuitBreakerState::Open;
                    warn!("Circuit breaker opened after {} failures", self.failure_count);
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Open;
                warn!("Circuit breaker reopened after failed half-open test");
            }
            CircuitBreakerState::Open => {
                // Already open, just update failure time
            }
        }
    }

    /// Get current state
    pub fn state(&self) -> &CircuitBreakerState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_resilience_manager_retry() {
        let config = RetryConfig {
            max_retries: 2,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            jitter: false,
        };

        let mut manager = ResilienceManager::new(config);
        let mut attempt_count = 0;

        let result = manager.execute_with_retry("test_operation", || async {
            attempt_count += 1;
            if attempt_count < 3 {
                Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "test error"))
            } else {
                Ok("success")
            }
        }).await;

        assert!(result.is_ok());
        assert_eq!(attempt_count, 3);
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let mut cb = CircuitBreaker::new(2, Duration::from_millis(100));

        // Initially closed
        assert!(cb.is_allowed());

        // First failure
        cb.on_failure();
        assert!(cb.is_allowed());

        // Second failure - should open
        cb.on_failure();
        assert!(!cb.is_allowed());

        // Wait for recovery timeout
        sleep(Duration::from_millis(150)).await;
        assert!(cb.is_allowed()); // Should be half-open

        // Success should close the circuit
        cb.on_success();
        assert_eq!(*cb.state(), CircuitBreakerState::Closed);
    }
}
