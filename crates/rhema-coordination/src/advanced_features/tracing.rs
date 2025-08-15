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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use uuid::Uuid;

/// Distributed tracing system for production coordination
pub struct TracingSystem {
    /// Tracing configuration
    config: TracingConfig,
    /// Active traces
    active_traces: Arc<RwLock<HashMap<String, Trace>>>,
    /// Trace history
    trace_history: Arc<RwLock<Vec<Trace>>>,
    /// Trace exporters
    exporters: Vec<Box<dyn TraceExporter + Send + Sync>>,
    /// Trace statistics
    statistics: Arc<RwLock<TraceStatistics>>,
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable tracing system
    pub enabled: bool,
    /// Sampling rate (0.0 to 1.0)
    pub sampling_rate: f64,
    /// Maximum trace duration (seconds)
    pub max_trace_duration_seconds: u64,
    /// Trace retention days
    pub retention_days: u32,
    /// Enable distributed tracing
    pub distributed_enabled: bool,
    /// Enable correlation IDs
    pub correlation_enabled: bool,
    /// Enable performance tracing
    pub performance_enabled: bool,
    /// Enable error tracing
    pub error_enabled: bool,
    /// Export formats
    pub export_formats: Vec<TraceExportFormat>,
}

/// Trace export formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceExportFormat {
    Json,
    Jaeger,
    Zipkin,
    OpenTelemetry,
    Custom(String),
}

/// Trace information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    /// Unique trace ID
    pub id: String,
    /// Parent trace ID (for distributed tracing)
    pub parent_id: Option<String>,
    /// Correlation ID
    pub correlation_id: Option<String>,
    /// Trace name
    pub name: String,
    /// Trace description
    pub description: Option<String>,
    /// Trace start time
    pub start_time: DateTime<Utc>,
    /// Trace end time
    pub end_time: Option<DateTime<Utc>>,
    /// Trace duration (milliseconds)
    pub duration_ms: Option<u64>,
    /// Trace status
    pub status: TraceStatus,
    /// Trace spans
    pub spans: Vec<Span>,
    /// Trace metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Trace tags
    pub tags: HashMap<String, String>,
    /// Trace source
    pub source: String,
    /// Trace service
    pub service: String,
}

/// Trace status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceStatus {
    Active,
    Completed,
    Failed,
    Timeout,
}

impl std::fmt::Display for TraceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceStatus::Active => write!(f, "Active"),
            TraceStatus::Completed => write!(f, "Completed"),
            TraceStatus::Failed => write!(f, "Failed"),
            TraceStatus::Timeout => write!(f, "Timeout"),
        }
    }
}

/// Span information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// Unique span ID
    pub id: String,
    /// Parent span ID
    pub parent_id: Option<String>,
    /// Span name
    pub name: String,
    /// Span description
    pub description: Option<String>,
    /// Span start time
    pub start_time: DateTime<Utc>,
    /// Span end time
    pub end_time: Option<DateTime<Utc>>,
    /// Span duration (milliseconds)
    pub duration_ms: Option<u64>,
    /// Span status
    pub status: SpanStatus,
    /// Span metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Span tags
    pub tags: HashMap<String, String>,
    /// Span logs
    pub logs: Vec<SpanLog>,
    /// Span events
    pub events: Vec<SpanEvent>,
}

/// Span status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpanStatus {
    Active,
    Completed,
    Failed,
    Timeout,
}

impl std::fmt::Display for SpanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpanStatus::Active => write!(f, "Active"),
            SpanStatus::Completed => write!(f, "Completed"),
            SpanStatus::Failed => write!(f, "Failed"),
            SpanStatus::Timeout => write!(f, "Timeout"),
        }
    }
}

/// Span log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanLog {
    /// Log timestamp
    pub timestamp: DateTime<Utc>,
    /// Log level
    pub level: LogLevel,
    /// Log message
    pub message: String,
    /// Log metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Log level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Span event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event name
    pub name: String,
    /// Event description
    pub description: Option<String>,
    /// Event metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Trace statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceStatistics {
    /// Total traces created
    pub total_traces: u64,
    /// Active traces count
    pub active_traces: u64,
    /// Completed traces count
    pub completed_traces: u64,
    /// Failed traces count
    pub failed_traces: u64,
    /// Average trace duration (ms)
    pub avg_trace_duration_ms: f64,
    /// Traces by status
    pub traces_by_status: HashMap<TraceStatus, u64>,
    /// Traces by service
    pub traces_by_service: HashMap<String, u64>,
    /// Last trace timestamp
    pub last_trace_timestamp: Option<DateTime<Utc>>,
}

/// Trace exporter trait
pub trait TraceExporter: Send + Sync {
    /// Export trace
    fn export_trace(&self, trace: &Trace) -> Result<(), String>;
    /// Get exporter name
    fn name(&self) -> &str;
    /// Check if exporter is enabled
    fn is_enabled(&self) -> bool;
}

/// Console trace exporter
pub struct ConsoleTraceExporter;

impl TraceExporter for ConsoleTraceExporter {
    fn export_trace(&self, trace: &Trace) -> Result<(), String> {
        info!(
            "[TRACE] {} - {} ({}ms)",
            trace.name,
            trace.status,
            trace.duration_ms.unwrap_or(0)
        );
        for span in &trace.spans {
            debug!(
                "  [SPAN] {} - {} ({}ms)",
                span.name,
                span.status,
                span.duration_ms.unwrap_or(0)
            );
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "console"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

/// JSON trace exporter
pub struct JsonTraceExporter {
    output_file: Option<String>,
}

impl JsonTraceExporter {
    pub fn new(output_file: Option<String>) -> Self {
        Self { output_file }
    }
}

impl TraceExporter for JsonTraceExporter {
    fn export_trace(&self, trace: &Trace) -> Result<(), String> {
        let json = serde_json::to_string_pretty(trace)
            .map_err(|e| format!("Failed to serialize trace: {}", e))?;

        if let Some(file_path) = &self.output_file {
            // In a real implementation, this would write to a file
            // For now, we'll just log it
            info!("[JSON_TRACE] Writing to {}: {}", file_path, json);
        } else {
            info!("[JSON_TRACE] {}", json);
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "json"
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

/// Trace context for distributed tracing
#[derive(Debug, Clone)]
pub struct TraceContext {
    /// Trace ID
    pub trace_id: String,
    /// Parent span ID
    pub parent_span_id: Option<String>,
    /// Correlation ID
    pub correlation_id: Option<String>,
    /// Service name
    pub service: String,
}

impl TraceContext {
    /// Create new trace context
    pub fn new(service: String) -> Self {
        Self {
            trace_id: Uuid::new_v4().to_string(),
            parent_span_id: None,
            correlation_id: None,
            service,
        }
    }

    /// Create child context
    pub fn child(&self, service: String) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            parent_span_id: Some(Uuid::new_v4().to_string()),
            correlation_id: self.correlation_id.clone(),
            service,
        }
    }

    /// Set correlation ID
    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
}

impl TracingSystem {
    /// Create new tracing system
    pub fn new(config: TracingConfig) -> Self {
        let mut exporters: Vec<Box<dyn TraceExporter + Send + Sync>> = Vec::new();

        // Add default exporters
        exporters.push(Box::new(ConsoleTraceExporter));
        exporters.push(Box::new(JsonTraceExporter::new(None)));

        Self {
            config,
            active_traces: Arc::new(RwLock::new(HashMap::new())),
            trace_history: Arc::new(RwLock::new(Vec::new())),
            exporters,
            statistics: Arc::new(RwLock::new(TraceStatistics::default())),
        }
    }

    /// Add trace exporter
    pub fn add_exporter(&mut self, exporter: Box<dyn TraceExporter + Send + Sync>) {
        self.exporters.push(exporter);
    }

    /// Start a new trace
    pub async fn start_trace(
        &self,
        name: String,
        description: Option<String>,
        context: TraceContext,
        metadata: Option<HashMap<String, serde_json::Value>>,
        tags: Option<HashMap<String, String>>,
    ) -> Result<String, String> {
        if !self.config.enabled {
            return Ok("tracing_disabled".to_string());
        }

        // Apply sampling
        if !self.should_sample() {
            return Ok("sampled_out".to_string());
        }

        let trace_id = context.trace_id.clone();
        let trace = Trace {
            id: trace_id.clone(),
            parent_id: None,
            correlation_id: context.correlation_id,
            name,
            description,
            start_time: Utc::now(),
            end_time: None,
            duration_ms: None,
            status: TraceStatus::Active,
            spans: Vec::new(),
            metadata: metadata.unwrap_or_default(),
            tags: tags.unwrap_or_default(),
            source: context.service.clone(),
            service: context.service,
        };

        // Store trace
        {
            let mut active_traces = self.active_traces.write().await;
            active_traces.insert(trace_id.clone(), trace.clone());
        }

        // Update statistics
        self.update_statistics(&trace).await;

        info!("Trace started: {} ({})", trace.name, trace_id);
        Ok(trace_id)
    }

    /// End a trace
    pub async fn end_trace(&self, trace_id: &str, status: TraceStatus) -> Result<(), String> {
        let mut active_traces = self.active_traces.write().await;

        if let Some(trace) = active_traces.get_mut(trace_id) {
            let end_time = Utc::now();
            let duration_ms = (end_time - trace.start_time).num_milliseconds() as u64;

            trace.end_time = Some(end_time);
            trace.duration_ms = Some(duration_ms);
            trace.status = status.clone();

            // Check for timeout
            if duration_ms > self.config.max_trace_duration_seconds * 1000 {
                trace.status = TraceStatus::Timeout;
            }

            // Move to history
            let trace = active_traces.remove(trace_id).unwrap();
            let mut trace_history = self.trace_history.write().await;
            trace_history.push(trace.clone());

            // Export trace
            for exporter in &self.exporters {
                if exporter.is_enabled() {
                    if let Err(e) = exporter.export_trace(&trace) {
                        error!("Failed to export trace to {}: {}", exporter.name(), e);
                    }
                }
            }

            info!(
                "Trace ended: {} - {:?} ({}ms)",
                trace.name, status, duration_ms
            );
            Ok(())
        } else {
            Err(format!("Trace not found: {}", trace_id))
        }
    }

    /// Add span to trace
    pub async fn add_span(
        &self,
        trace_id: &str,
        name: String,
        description: Option<String>,
        parent_span_id: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
        tags: Option<HashMap<String, String>>,
    ) -> Result<String, String> {
        let span_id = Uuid::new_v4().to_string();
        let span = Span {
            id: span_id.clone(),
            parent_id: parent_span_id,
            name,
            description,
            start_time: Utc::now(),
            end_time: None,
            duration_ms: None,
            status: SpanStatus::Active,
            metadata: metadata.unwrap_or_default(),
            tags: tags.unwrap_or_default(),
            logs: Vec::new(),
            events: Vec::new(),
        };

        let mut active_traces = self.active_traces.write().await;

        if let Some(trace) = active_traces.get_mut(trace_id) {
            trace.spans.push(span);
            debug!("Span added to trace {}: {}", trace_id, span_id);
            Ok(span_id)
        } else {
            Err(format!("Trace not found: {}", trace_id))
        }
    }

    /// End a span
    pub async fn end_span(
        &self,
        trace_id: &str,
        span_id: &str,
        status: SpanStatus,
    ) -> Result<(), String> {
        let mut active_traces = self.active_traces.write().await;

        if let Some(trace) = active_traces.get_mut(trace_id) {
            if let Some(span) = trace.spans.iter_mut().find(|s| s.id == span_id) {
                let end_time = Utc::now();
                let duration_ms = (end_time - span.start_time).num_milliseconds() as u64;

                span.end_time = Some(end_time);
                span.duration_ms = Some(duration_ms);
                span.status = status;

                debug!(
                    "Span ended in trace {}: {} ({:?}ms)",
                    trace_id, span_id, duration_ms
                );
                Ok(())
            } else {
                Err(format!("Span not found: {}", span_id))
            }
        } else {
            Err(format!("Trace not found: {}", trace_id))
        }
    }

    /// Add log to span
    pub async fn add_span_log(
        &self,
        trace_id: &str,
        span_id: &str,
        level: LogLevel,
        message: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(), String> {
        let log = SpanLog {
            timestamp: Utc::now(),
            level,
            message,
            metadata: metadata.unwrap_or_default(),
        };

        let mut active_traces = self.active_traces.write().await;

        if let Some(trace) = active_traces.get_mut(trace_id) {
            if let Some(span) = trace.spans.iter_mut().find(|s| s.id == span_id) {
                span.logs.push(log);
                Ok(())
            } else {
                Err(format!("Span not found: {}", span_id))
            }
        } else {
            Err(format!("Trace not found: {}", trace_id))
        }
    }

    /// Add event to span
    pub async fn add_span_event(
        &self,
        trace_id: &str,
        span_id: &str,
        name: String,
        description: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(), String> {
        let event = SpanEvent {
            timestamp: Utc::now(),
            name,
            description,
            metadata: metadata.unwrap_or_default(),
        };

        let mut active_traces = self.active_traces.write().await;

        if let Some(trace) = active_traces.get_mut(trace_id) {
            if let Some(span) = trace.spans.iter_mut().find(|s| s.id == span_id) {
                span.events.push(event);
                Ok(())
            } else {
                Err(format!("Span not found: {}", span_id))
            }
        } else {
            Err(format!("Trace not found: {}", trace_id))
        }
    }

    /// Get active traces
    pub async fn get_active_traces(&self) -> Vec<Trace> {
        let active_traces = self.active_traces.read().await;
        active_traces.values().cloned().collect()
    }

    /// Get trace by ID
    pub async fn get_trace(&self, trace_id: &str) -> Option<Trace> {
        let active_traces = self.active_traces.read().await;
        if let Some(trace) = active_traces.get(trace_id) {
            return Some(trace.clone());
        }

        let trace_history = self.trace_history.read().await;
        trace_history.iter().find(|t| t.id == trace_id).cloned()
    }

    /// Get trace statistics
    pub async fn get_statistics(&self) -> TraceStatistics {
        self.statistics.read().await.clone()
    }

    /// Clean up old traces
    pub async fn cleanup_old_traces(&self) -> Result<usize, String> {
        let retention_days = self.config.retention_days as i64;
        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days);

        let mut trace_history = self.trace_history.write().await;
        let initial_count = trace_history.len();

        trace_history.retain(|trace| trace.start_time > cutoff_date);

        let removed_count = initial_count - trace_history.len();
        info!("Cleaned up {} old traces", removed_count);

        Ok(removed_count)
    }

    /// Check if trace should be sampled
    fn should_sample(&self) -> bool {
        if self.config.sampling_rate >= 1.0 {
            return true;
        }

        if self.config.sampling_rate <= 0.0 {
            return false;
        }

        let random_value = rand::random::<f64>();
        random_value <= self.config.sampling_rate
    }

    /// Update trace statistics
    async fn update_statistics(&self, trace: &Trace) {
        let mut stats = self.statistics.write().await;

        stats.total_traces += 1;
        stats.last_trace_timestamp = Some(trace.start_time);

        // Update status counts
        *stats
            .traces_by_status
            .entry(trace.status.clone())
            .or_insert(0) += 1;

        // Update service counts
        *stats
            .traces_by_service
            .entry(trace.service.clone())
            .or_insert(0) += 1;

        // Update active traces count
        if trace.status == TraceStatus::Active {
            stats.active_traces += 1;
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sampling_rate: 1.0,              // 100% sampling
            max_trace_duration_seconds: 300, // 5 minutes
            retention_days: 7,
            distributed_enabled: true,
            correlation_enabled: true,
            performance_enabled: true,
            error_enabled: true,
            export_formats: vec![TraceExportFormat::Json],
        }
    }
}

impl Default for TraceStatistics {
    fn default() -> Self {
        Self {
            total_traces: 0,
            active_traces: 0,
            completed_traces: 0,
            failed_traces: 0,
            avg_trace_duration_ms: 0.0,
            traces_by_status: HashMap::new(),
            traces_by_service: HashMap::new(),
            last_trace_timestamp: None,
        }
    }
}

/// Macro for creating a traced function
#[macro_export]
macro_rules! traced_function {
    ($tracing_system:expr, $name:expr, $context:expr, $block:block) => {{
        let trace_id = $tracing_system
            .start_trace($name.to_string(), None, $context, None, None)
            .await?;

        let result = $block;

        let status = if result.is_ok() {
            $crate::advanced_features::tracing::TraceStatus::Completed
        } else {
            $crate::advanced_features::tracing::TraceStatus::Failed
        };

        $tracing_system.end_trace(&trace_id, status).await?;
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracing_system_creation() {
        let config = TracingConfig::default();
        let tracing_system = TracingSystem::new(config);

        assert!(tracing_system.config.enabled);
        assert_eq!(tracing_system.config.sampling_rate, 1.0);
    }

    #[tokio::test]
    async fn test_start_and_end_trace() {
        let config = TracingConfig::default();
        let tracing_system = TracingSystem::new(config);

        let context = TraceContext::new("test_service".to_string());
        let trace_id = tracing_system
            .start_trace(
                "Test Trace".to_string(),
                Some("This is a test trace".to_string()),
                context,
                None,
                None,
            )
            .await
            .unwrap();

        assert!(!trace_id.is_empty());

        let active_traces = tracing_system.get_active_traces().await;
        assert_eq!(active_traces.len(), 1);
        assert_eq!(active_traces[0].name, "Test Trace");

        tracing_system
            .end_trace(&trace_id, TraceStatus::Completed)
            .await
            .unwrap();

        let active_traces = tracing_system.get_active_traces().await;
        assert_eq!(active_traces.len(), 0);

        let trace = tracing_system.get_trace(&trace_id).await.unwrap();
        assert_eq!(trace.status, TraceStatus::Completed);
        assert!(trace.duration_ms.is_some());
    }

    #[tokio::test]
    async fn test_add_span() {
        let config = TracingConfig::default();
        let tracing_system = TracingSystem::new(config);

        let context = TraceContext::new("test_service".to_string());
        let trace_id = tracing_system
            .start_trace("Test Trace".to_string(), None, context, None, None)
            .await
            .unwrap();

        let span_id = tracing_system
            .add_span(
                &trace_id,
                "Test Span".to_string(),
                Some("This is a test span".to_string()),
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert!(!span_id.is_empty());

        let trace = tracing_system.get_trace(&trace_id).await.unwrap();
        assert_eq!(trace.spans.len(), 1);
        assert_eq!(trace.spans[0].name, "Test Span");

        tracing_system
            .end_span(&trace_id, &span_id, SpanStatus::Completed)
            .await
            .unwrap();

        let trace = tracing_system.get_trace(&trace_id).await.unwrap();
        assert_eq!(trace.spans[0].status, SpanStatus::Completed);
    }

    #[tokio::test]
    async fn test_add_span_log() {
        let config = TracingConfig::default();
        let tracing_system = TracingSystem::new(config);

        let context = TraceContext::new("test_service".to_string());
        let trace_id = tracing_system
            .start_trace("Test Trace".to_string(), None, context, None, None)
            .await
            .unwrap();

        let span_id = tracing_system
            .add_span(&trace_id, "Test Span".to_string(), None, None, None, None)
            .await
            .unwrap();

        tracing_system
            .add_span_log(
                &trace_id,
                &span_id,
                LogLevel::Info,
                "Test log message".to_string(),
                None,
            )
            .await
            .unwrap();

        let trace = tracing_system.get_trace(&trace_id).await.unwrap();
        assert_eq!(trace.spans[0].logs.len(), 1);
        assert_eq!(trace.spans[0].logs[0].message, "Test log message");
    }

    #[tokio::test]
    async fn test_trace_statistics() {
        let config = TracingConfig::default();
        let tracing_system = TracingSystem::new(config);

        // Create multiple traces
        for i in 0..3 {
            let context = TraceContext::new("test_service".to_string());
            let trace_id = tracing_system
                .start_trace(format!("Test Trace {}", i), None, context, None, None)
                .await
                .unwrap();

            tracing_system
                .end_trace(&trace_id, TraceStatus::Completed)
                .await
                .unwrap();
        }

        let stats = tracing_system.get_statistics().await;
        assert_eq!(stats.total_traces, 3);
        assert_eq!(stats.completed_traces, 3);
        assert_eq!(stats.traces_by_service["test_service"], 3);
    }

    #[tokio::test]
    async fn test_sampling() {
        let mut config = TracingConfig::default();
        config.sampling_rate = 0.5; // 50% sampling

        let tracing_system = TracingSystem::new(config);

        let mut sampled_count = 0;
        for _ in 0..100 {
            let context = TraceContext::new("test_service".to_string());
            let trace_id = tracing_system
                .start_trace("Test Trace".to_string(), None, context, None, None)
                .await
                .unwrap();

            if trace_id != "sampled_out" {
                sampled_count += 1;
                tracing_system
                    .end_trace(&trace_id, TraceStatus::Completed)
                    .await
                    .unwrap();
            }
        }

        // Should be roughly 50% (allowing for some variance)
        assert!(sampled_count > 30 && sampled_count < 70);
    }
}
