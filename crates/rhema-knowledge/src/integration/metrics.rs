use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// AI integration metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIIntegrationMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: u64,
    pub ai_enhancement_count: u64,
    pub synthesis_count: u64,
    pub suggestion_count: u64,
    pub confidence_improvement: f32,
    pub last_updated: DateTime<Utc>,
}

impl Default for AIIntegrationMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0,
            ai_enhancement_count: 0,
            synthesis_count: 0,
            suggestion_count: 0,
            confidence_improvement: 0.0,
            last_updated: Utc::now(),
        }
    }
}
