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

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn, Level};
use uuid::Uuid;

use crate::types::{KnowledgeError, KnowledgeResult};

/// Error categories for better handling and recovery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Temporary errors that can be retried
    Temporary,
    /// Permanent errors that require intervention
    Permanent,
    /// Configuration errors that can be fixed
    Configuration,
    /// Network errors that may resolve
    Network,
    /// Resource errors (memory, disk, etc.)
    Resource,
    /// Security errors
    Security,
    /// Data integrity errors
    DataIntegrity,
    /// Performance errors
    Performance,
    /// Unknown errors
    Unknown,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - error
    High,
    /// Critical severity - system failure
    Critical,
}

/// Enhanced error information with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub error_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub operation: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub stack_trace: Option<String>,
    pub additional_context: HashMap<String, String>,
    pub retry_count: u32,
    pub recovery_attempted: bool,
}

/// Error recovery strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Retry the operation
    Retry { max_attempts: u32, backoff_ms: u64 },
    /// Use fallback mechanism
    Fallback { fallback_operation: String },
    /// Degrade gracefully
    Degrade { degraded_mode: String },
    /// Skip the operation
    Skip,
    /// Require manual intervention
    ManualIntervention { description: String },
}

/// Error reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReportingConfig {
    pub enabled: bool,
    pub reporting_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub include_stack_traces: bool,
    pub include_context: bool,
    pub batch_size: usize,
    pub batch_timeout_ms: u64,
    pub retry_on_failure: bool,
}

impl Default for ErrorReportingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            reporting_endpoint: None,
            api_key: None,
            include_stack_traces: true,
            include_context: true,
            batch_size: 100,
            batch_timeout_ms: 5000,
            retry_on_failure: true,
        }
    }
}

/// Error logging configuration
#[derive(Debug, Clone)]
pub struct ErrorLoggingConfig {
    pub enabled: bool,
    pub log_level: Level,
    pub include_stack_traces: bool,
    pub include_context: bool,
    pub structured_logging: bool,
    pub log_file_path: Option<String>,
    pub max_log_size_mb: u64,
    pub log_retention_days: u32,
}

impl Default for ErrorLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: Level::ERROR,
            include_stack_traces: true,
            include_context: true,
            structured_logging: true,
            log_file_path: None,
            max_log_size_mb: 100,
            log_retention_days: 30,
        }
    }
}

/// Comprehensive error handler
pub struct ErrorHandler {
    config: ErrorHandlerConfig,
    error_contexts: Arc<RwLock<HashMap<String, ErrorContext>>>,
    recovery_strategies: Arc<RwLock<HashMap<ErrorCategory, RecoveryStrategy>>>,
    reporting_config: ErrorReportingConfig,
    logging_config: ErrorLoggingConfig,
    error_reporter: Option<Arc<dyn ErrorReporter>>,
    error_logger: Option<Arc<dyn ErrorLogger>>,
}

/// Error handler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlerConfig {
    pub enable_recovery: bool,
    pub enable_reporting: bool,
    pub enable_logging: bool,
    pub max_error_contexts: usize,
    pub error_cleanup_interval_ms: u64,
    pub default_retry_attempts: u32,
    pub default_backoff_ms: u64,
}

impl Default for ErrorHandlerConfig {
    fn default() -> Self {
        Self {
            enable_recovery: true,
            enable_reporting: true,
            enable_logging: true,
            max_error_contexts: 1000,
            error_cleanup_interval_ms: 3600000, // 1 hour
            default_retry_attempts: 3,
            default_backoff_ms: 1000,
        }
    }
}

/// Error reporter trait for external reporting systems
#[async_trait]
pub trait ErrorReporter: Send + Sync {
    async fn report_error(
        &self,
        error: &KnowledgeError,
        context: &ErrorContext,
    ) -> KnowledgeResult<()>;
    async fn report_batch(
        &self,
        errors: &[(&KnowledgeError, &ErrorContext)],
    ) -> KnowledgeResult<()>;
}

/// Error logger trait for structured logging
#[async_trait]
pub trait ErrorLogger: Send + Sync {
    async fn log_error(
        &self,
        error: &KnowledgeError,
        context: &ErrorContext,
    ) -> KnowledgeResult<()>;
    async fn log_batch(&self, errors: &[(&KnowledgeError, &ErrorContext)]) -> KnowledgeResult<()>;
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new(config: ErrorHandlerConfig) -> Self {
        let mut handler = Self {
            config,
            error_contexts: Arc::new(RwLock::new(HashMap::new())),
            recovery_strategies: Arc::new(RwLock::new(HashMap::new())),
            reporting_config: ErrorReportingConfig::default(),
            logging_config: ErrorLoggingConfig::default(),
            error_reporter: None,
            error_logger: None,
        };

        // Initialize default recovery strategies
        handler.initialize_default_recovery_strategies();

        handler
    }

    /// Initialize default recovery strategies
    fn initialize_default_recovery_strategies(&mut self) {
        let mut strategies = self.recovery_strategies.blocking_write();

        strategies.insert(
            ErrorCategory::Temporary,
            RecoveryStrategy::Retry {
                max_attempts: 3,
                backoff_ms: 1000,
            },
        );

        strategies.insert(
            ErrorCategory::Network,
            RecoveryStrategy::Retry {
                max_attempts: 5,
                backoff_ms: 2000,
            },
        );

        strategies.insert(
            ErrorCategory::Resource,
            RecoveryStrategy::Degrade {
                degraded_mode: "reduced_performance".to_string(),
            },
        );

        strategies.insert(
            ErrorCategory::Configuration,
            RecoveryStrategy::ManualIntervention {
                description: "Configuration error requires manual review".to_string(),
            },
        );

        strategies.insert(
            ErrorCategory::Security,
            RecoveryStrategy::ManualIntervention {
                description: "Security error requires immediate attention".to_string(),
            },
        );
    }

    /// Set error reporter
    pub fn with_reporter(mut self, reporter: Arc<dyn ErrorReporter>) -> Self {
        self.error_reporter = Some(reporter);
        self
    }

    /// Set error logger
    pub fn with_logger(mut self, logger: Arc<dyn ErrorLogger>) -> Self {
        self.error_logger = Some(logger);
        self
    }

    /// Set reporting configuration
    pub fn with_reporting_config(mut self, config: ErrorReportingConfig) -> Self {
        self.reporting_config = config;
        self
    }

    /// Set logging configuration
    pub fn with_logging_config(mut self, config: ErrorLoggingConfig) -> Self {
        self.logging_config = config;
        self
    }

    /// Handle an error with comprehensive context
    #[instrument(skip(self, error, operation))]
    pub async fn handle_error(
        &self,
        error: &KnowledgeError,
        operation: &str,
        user_id: Option<&str>,
        session_id: Option<&str>,
        request_id: Option<&str>,
    ) -> KnowledgeResult<()> {
        let error_id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();

        // Categorize the error
        let category = self.categorize_error(error);
        let severity = self.determine_severity(error, &category);

        // Create error context
        let context = ErrorContext {
            error_id: error_id.clone(),
            timestamp,
            category: category.clone(),
            severity,
            operation: operation.to_string(),
            user_id: user_id.map(|s| s.to_string()),
            session_id: session_id.map(|s| s.to_string()),
            request_id: request_id.map(|s| s.to_string()),
            stack_trace: self.capture_stack_trace(),
            additional_context: self.extract_additional_context(error),
            retry_count: 0,
            recovery_attempted: false,
        };

        // Store error context
        self.store_error_context(&error_id, &context).await;

        // Log the error
        if self.config.enable_logging {
            self.log_error(error, &context).await?;
        }

        // Report the error
        if self.config.enable_reporting {
            self.report_error(error, &context).await?;
        }

        // Attempt recovery if enabled
        if self.config.enable_recovery {
            self.attempt_recovery(error, &context).await?;
        }

        Ok(())
    }

    /// Categorize an error based on its type and content
    fn categorize_error(&self, error: &KnowledgeError) -> ErrorCategory {
        match error {
            KnowledgeError::NetworkError(_) => ErrorCategory::Network,
            KnowledgeError::TimeoutError(_) => ErrorCategory::Temporary,
            KnowledgeError::ConfigurationError(_) => ErrorCategory::Configuration,
            KnowledgeError::FileSystemError(_) => ErrorCategory::Resource,
            KnowledgeError::CompressionError(_) => ErrorCategory::Resource,
            KnowledgeError::SerializationError(_) => ErrorCategory::DataIntegrity,
            KnowledgeError::InvalidData(_) => ErrorCategory::DataIntegrity,
            KnowledgeError::EmbeddingError(_) => ErrorCategory::Performance,
            KnowledgeError::VectorError(_) => ErrorCategory::Performance,
            KnowledgeError::SearchError(_) => ErrorCategory::Performance,
            KnowledgeError::StorageError(_) => ErrorCategory::Resource,
            KnowledgeError::CacheError(_) => ErrorCategory::Performance,
            KnowledgeError::EngineError(_) => ErrorCategory::Performance,
            KnowledgeError::IndexingError(_) => ErrorCategory::Performance,
            KnowledgeError::ProactiveError(_) => ErrorCategory::Performance,
            KnowledgeError::FileWatchingError(_) => ErrorCategory::Temporary,
            KnowledgeError::RedisError(_) => ErrorCategory::Network,
            KnowledgeError::SerdeJsonError(_) => ErrorCategory::DataIntegrity,
            KnowledgeError::AnyhowError(_) => ErrorCategory::Unknown,
            KnowledgeError::SynthesisError(_) => ErrorCategory::Performance,
            KnowledgeError::SecurityError(_) => ErrorCategory::Security,
            KnowledgeError::MonitoringError(_) => ErrorCategory::Performance,
            KnowledgeError::TemporaryError(_) => ErrorCategory::Temporary,
            KnowledgeError::ResourceError(_) => ErrorCategory::Resource,
        }
    }

    /// Determine error severity based on category and content
    fn determine_severity(
        &self,
        error: &KnowledgeError,
        category: &ErrorCategory,
    ) -> ErrorSeverity {
        match category {
            ErrorCategory::Security => ErrorSeverity::Critical,
            ErrorCategory::DataIntegrity => ErrorSeverity::High,
            ErrorCategory::Configuration => ErrorSeverity::High,
            ErrorCategory::Resource => ErrorSeverity::High,
            ErrorCategory::Network => ErrorSeverity::Medium,
            ErrorCategory::Performance => ErrorSeverity::Medium,
            ErrorCategory::Temporary => ErrorSeverity::Low,
            ErrorCategory::Permanent => ErrorSeverity::High,
            ErrorCategory::Unknown => ErrorSeverity::Medium,
        }
    }

    /// Capture stack trace if enabled
    fn capture_stack_trace(&self) -> Option<String> {
        if self.logging_config.include_stack_traces {
            Some(format!("{:?}", std::backtrace::Backtrace::capture()))
        } else {
            None
        }
    }

    /// Extract additional context from error
    fn extract_additional_context(&self, error: &KnowledgeError) -> HashMap<String, String> {
        let mut context = HashMap::new();

        match error {
            KnowledgeError::NetworkError(msg) => {
                context.insert("error_type".to_string(), "network".to_string());
                context.insert("error_message".to_string(), msg.clone());
            }
            KnowledgeError::TimeoutError(msg) => {
                context.insert("error_type".to_string(), "timeout".to_string());
                context.insert("error_message".to_string(), msg.clone());
            }
            KnowledgeError::ConfigurationError(msg) => {
                context.insert("error_type".to_string(), "configuration".to_string());
                context.insert("error_message".to_string(), msg.clone());
            }
            _ => {
                context.insert("error_type".to_string(), "unknown".to_string());
                context.insert("error_message".to_string(), error.to_string());
            }
        }

        context
    }

    /// Store error context
    async fn store_error_context(&self, error_id: &str, context: &ErrorContext) {
        let mut contexts = self.error_contexts.write().await;

        // Cleanup old contexts if we exceed the limit
        if contexts.len() >= self.config.max_error_contexts {
            let to_remove = contexts.len() - self.config.max_error_contexts + 1;

            // Collect and sort contexts to find the oldest ones
            let mut sorted_contexts: Vec<_> = contexts.iter().collect();
            sorted_contexts.sort_by_key(|(_, ctx)| ctx.timestamp);

            // Extract IDs of oldest contexts to remove
            let ids_to_remove: Vec<String> = sorted_contexts
                .iter()
                .take(to_remove)
                .map(|(id, _)| (*id).clone())
                .collect();

            // Now remove them from the HashMap
            for id in ids_to_remove {
                contexts.remove(&id);
            }
        }

        contexts.insert(error_id.to_string(), context.clone());
    }

    /// Log error with structured logging
    async fn log_error(
        &self,
        error: &KnowledgeError,
        context: &ErrorContext,
    ) -> KnowledgeResult<()> {
        if let Some(logger) = &self.error_logger {
            logger.log_error(error, context).await?;
        } else {
            // Default logging
            let log_message = if self.logging_config.structured_logging {
                serde_json::to_string(&serde_json::json!({
                    "error_id": context.error_id,
                    "timestamp": context.timestamp,
                    "category": context.category,
                    "severity": context.severity,
                    "operation": context.operation,
                    "error": error.to_string(),
                    "user_id": context.user_id,
                    "session_id": context.session_id,
                    "request_id": context.request_id,
                    "retry_count": context.retry_count,
                }))?
            } else {
                format!(
                    "Error [{}] {:?} in operation '{}': {}",
                    context.error_id, context.category, context.operation, error
                )
            };

            match context.severity {
                ErrorSeverity::Low => info!("{}", log_message),
                ErrorSeverity::Medium => warn!("{}", log_message),
                ErrorSeverity::High => error!("{}", log_message),
                ErrorSeverity::Critical => error!("CRITICAL: {}", log_message),
            }
        }

        Ok(())
    }

    /// Report error to external systems
    async fn report_error(
        &self,
        error: &KnowledgeError,
        context: &ErrorContext,
    ) -> KnowledgeResult<()> {
        if let Some(reporter) = &self.error_reporter {
            reporter.report_error(error, context).await?;
        }
        Ok(())
    }

    /// Attempt error recovery
    async fn attempt_recovery(
        &self,
        error: &KnowledgeError,
        context: &ErrorContext,
    ) -> KnowledgeResult<()> {
        let strategies = self.recovery_strategies.read().await;

        if let Some(strategy) = strategies.get(&context.category) {
            match strategy {
                RecoveryStrategy::Retry {
                    max_attempts,
                    backoff_ms,
                } => {
                    if context.retry_count < *max_attempts {
                        info!(
                            "Attempting retry {} for error {}",
                            context.retry_count + 1,
                            context.error_id
                        );
                        // In a real implementation, this would retry the operation
                        tokio::time::sleep(Duration::from_millis(*backoff_ms)).await;
                    }
                }
                RecoveryStrategy::Fallback { fallback_operation } => {
                    info!(
                        "Using fallback operation '{}' for error {}",
                        fallback_operation, context.error_id
                    );
                    // In a real implementation, this would execute the fallback
                }
                RecoveryStrategy::Degrade { degraded_mode } => {
                    info!(
                        "Degrading to mode '{}' for error {}",
                        degraded_mode, context.error_id
                    );
                    // In a real implementation, this would switch to degraded mode
                }
                RecoveryStrategy::Skip => {
                    info!("Skipping operation for error {}", context.error_id);
                    // In a real implementation, this would skip the operation
                }
                RecoveryStrategy::ManualIntervention { description } => {
                    error!(
                        "Manual intervention required for error {}: {}",
                        context.error_id, description
                    );
                    // In a real implementation, this would trigger alerts
                }
            }
        }

        Ok(())
    }

    /// Get error statistics
    pub async fn get_error_statistics(&self) -> ErrorStatistics {
        let contexts = self.error_contexts.read().await;
        let mut stats = ErrorStatistics::default();

        for context in contexts.values() {
            stats.total_errors += 1;

            // Count by category
            *stats
                .errors_by_category
                .entry(context.category.clone())
                .or_insert(0) += 1;

            // Count by severity
            *stats
                .errors_by_severity
                .entry(context.severity.clone())
                .or_insert(0) += 1;

            // Track recent errors
            let age = chrono::Utc::now() - context.timestamp;
            if age.num_hours() < 24 {
                stats.errors_last_24h += 1;
            }
            if age.num_hours() < 1 {
                stats.errors_last_hour += 1;
            }
        }

        stats
    }

    /// Cleanup old error contexts
    pub async fn cleanup_old_contexts(&self) {
        let mut contexts = self.error_contexts.write().await;
        let cutoff = chrono::Utc::now() - chrono::Duration::days(7);

        contexts.retain(|_, context| context.timestamp > cutoff);
    }
}

/// Error statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorStatistics {
    pub total_errors: u64,
    pub errors_last_24h: u64,
    pub errors_last_hour: u64,
    pub errors_by_category: HashMap<ErrorCategory, u64>,
    pub errors_by_severity: HashMap<ErrorSeverity, u64>,
}

/// Default error reporter implementation
pub struct DefaultErrorReporter {
    config: ErrorReportingConfig,
}

#[async_trait]
impl ErrorReporter for DefaultErrorReporter {
    async fn report_error(
        &self,
        error: &KnowledgeError,
        context: &ErrorContext,
    ) -> KnowledgeResult<()> {
        // In a real implementation, this would send to external monitoring systems
        debug!("Reporting error {} to external system", context.error_id);
        Ok(())
    }

    async fn report_batch(
        &self,
        errors: &[(&KnowledgeError, &ErrorContext)],
    ) -> KnowledgeResult<()> {
        // In a real implementation, this would batch send to external monitoring systems
        debug!("Reporting {} errors to external system", errors.len());
        Ok(())
    }
}

/// Default error logger implementation
pub struct DefaultErrorLogger {
    config: ErrorLoggingConfig,
}

#[async_trait]
impl ErrorLogger for DefaultErrorLogger {
    async fn log_error(
        &self,
        error: &KnowledgeError,
        context: &ErrorContext,
    ) -> KnowledgeResult<()> {
        // In a real implementation, this would write to structured log files
        debug!("Logging error {} to structured log", context.error_id);
        Ok(())
    }

    async fn log_batch(&self, errors: &[(&KnowledgeError, &ErrorContext)]) -> KnowledgeResult<()> {
        // In a real implementation, this would batch write to structured log files
        debug!("Logging {} errors to structured log", errors.len());
        Ok(())
    }
}

/// Enhanced error result type with recovery information
pub struct ErrorResult<T> {
    pub result: KnowledgeResult<T>,
    pub error_context: Option<ErrorContext>,
    pub recovery_attempted: bool,
    pub recovery_successful: bool,
}

impl<T> ErrorResult<T> {
    /// Create a successful result
    pub fn success(value: T) -> Self {
        Self {
            result: Ok(value),
            error_context: None,
            recovery_attempted: false,
            recovery_successful: false,
        }
    }

    /// Create an error result
    pub fn error(error: KnowledgeError, context: ErrorContext) -> Self {
        Self {
            result: Err(error),
            error_context: Some(context),
            recovery_attempted: false,
            recovery_successful: false,
        }
    }

    /// Check if the result is successful
    pub fn is_success(&self) -> bool {
        self.result.is_ok()
    }

    /// Get the inner result
    pub fn into_result(self) -> KnowledgeResult<T> {
        self.result
    }
}

/// Error handling utilities
pub mod utils {
    use super::*;

    /// Create a descriptive error message
    pub fn create_descriptive_error_message(error: &KnowledgeError, operation: &str) -> String {
        match error {
            KnowledgeError::NetworkError(msg) => {
                format!("Network error during {}: {}. Please check your internet connection and try again.", operation, msg)
            }
            KnowledgeError::TimeoutError(msg) => {
                format!("Operation '{}' timed out: {}. The system may be under heavy load. Please try again.", operation, msg)
            }
            KnowledgeError::ConfigurationError(msg) => {
                format!(
                    "Configuration error in {}: {}. Please check your configuration settings.",
                    operation, msg
                )
            }
            KnowledgeError::FileSystemError(io_error) => {
                format!("File system error during {}: {}. Please check file permissions and disk space.", operation, io_error)
            }
            KnowledgeError::InvalidData(msg) => {
                format!(
                    "Invalid data encountered during {}: {}. Please verify your input data.",
                    operation, msg
                )
            }
            KnowledgeError::ResourceError(msg) => {
                format!(
                    "Resource error during {}: {}. The system may be out of memory or disk space.",
                    operation, msg
                )
            }
            _ => {
                format!(
                    "Unexpected error during {}: {}. Please contact support if this persists.",
                    operation, error
                )
            }
        }
    }

    /// Check if an error is recoverable
    pub fn is_recoverable_error(error: &KnowledgeError) -> bool {
        matches!(
            error,
            KnowledgeError::NetworkError(_)
                | KnowledgeError::TimeoutError(_)
                | KnowledgeError::TemporaryError(_)
        )
    }

    /// Get suggested recovery actions
    pub fn get_recovery_suggestions(error: &KnowledgeError) -> Vec<String> {
        match error {
            KnowledgeError::NetworkError(_) => {
                vec![
                    "Check your internet connection".to_string(),
                    "Verify network configuration".to_string(),
                    "Try again in a few minutes".to_string(),
                ]
            }
            KnowledgeError::TimeoutError(_) => {
                vec![
                    "Try again later when system load is lower".to_string(),
                    "Increase timeout settings if possible".to_string(),
                    "Check system resources".to_string(),
                ]
            }
            KnowledgeError::ConfigurationError(_) => {
                vec![
                    "Review configuration settings".to_string(),
                    "Check configuration file syntax".to_string(),
                    "Verify required environment variables".to_string(),
                ]
            }
            KnowledgeError::FileSystemError(_) => {
                vec![
                    "Check disk space availability".to_string(),
                    "Verify file permissions".to_string(),
                    "Check for file system errors".to_string(),
                ]
            }
            _ => {
                vec![
                    "Contact system administrator".to_string(),
                    "Check system logs for more details".to_string(),
                    "Try restarting the service".to_string(),
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_handler_creation() {
        let config = ErrorHandlerConfig::default();
        let handler = ErrorHandler::new(config);

        assert!(handler.config.enable_recovery);
        assert!(handler.config.enable_reporting);
        assert!(handler.config.enable_logging);
    }

    #[tokio::test]
    async fn test_error_categorization() {
        let config = ErrorHandlerConfig::default();
        let handler = ErrorHandler::new(config);

        let network_error = KnowledgeError::NetworkError("Connection failed".to_string());
        let category = handler.categorize_error(&network_error);
        assert_eq!(category, ErrorCategory::Network);

        let config_error = KnowledgeError::ConfigurationError("Invalid config".to_string());
        let category = handler.categorize_error(&config_error);
        assert_eq!(category, ErrorCategory::Configuration);
    }

    #[tokio::test]
    async fn test_error_statistics() {
        let config = ErrorHandlerConfig::default();
        let handler = ErrorHandler::new(config);

        let stats = handler.get_error_statistics().await;
        assert_eq!(stats.total_errors, 0);
        assert_eq!(stats.errors_last_24h, 0);
        assert_eq!(stats.errors_last_hour, 0);
    }

    #[test]
    fn test_descriptive_error_messages() {
        let network_error = KnowledgeError::NetworkError("Connection timeout".to_string());
        let message = utils::create_descriptive_error_message(&network_error, "search");

        assert!(message.contains("Network error"));
        assert!(message.contains("search"));
        assert!(message.contains("Connection timeout"));
    }

    #[test]
    fn test_recovery_suggestions() {
        let network_error = KnowledgeError::NetworkError("Connection failed".to_string());
        let suggestions = utils::get_recovery_suggestions(&network_error);

        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("internet")));
    }
}
