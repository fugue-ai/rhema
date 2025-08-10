use serde::{Deserialize, Serialize};

/// AI service integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIIntegrationConfig {
    pub ai_service_url: String,
    pub ai_service_api_key: String,
    pub enable_knowledge_enhancement: bool,
    pub enable_semantic_search: bool,
    pub enable_context_injection: bool,
    pub max_context_length: usize,
    pub context_injection_threshold: f32,
    pub enable_ai_optimization: bool,
    pub optimization_interval_minutes: u64,
    pub enable_ai_monitoring: bool,
    pub monitoring_interval_seconds: u64,
}

impl Default for AIIntegrationConfig {
    fn default() -> Self {
        Self {
            ai_service_url: "http://localhost:8000".to_string(),
            ai_service_api_key: "".to_string(),
            enable_knowledge_enhancement: true,
            enable_semantic_search: true,
            enable_context_injection: true,
            max_context_length: 4096,
            context_injection_threshold: 0.7,
            enable_ai_optimization: true,
            optimization_interval_minutes: 60,
            enable_ai_monitoring: true,
            monitoring_interval_seconds: 300,
        }
    }
}
