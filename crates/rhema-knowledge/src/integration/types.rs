use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{ContentType, SearchResultMetadata};

/// AI-enhanced knowledge request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIKnowledgeRequest {
    pub request_id: String,
    pub query: String,
    pub context_scope: Option<String>,
    pub content_types: Vec<ContentType>,
    pub max_results: usize,
    pub similarity_threshold: f32,
    pub include_metadata: bool,
    pub enable_synthesis: bool,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// AI-enhanced knowledge response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIKnowledgeResponse {
    pub request_id: String,
    pub results: Vec<AIKnowledgeResult>,
    pub synthesized_content: Option<String>,
    pub confidence_score: f32,
    pub processing_time_ms: u64,
    pub ai_enhancements: Vec<AIEnhancement>,
    pub suggestions: Vec<KnowledgeSuggestion>,
    pub created_at: DateTime<Utc>,
}

/// AI-enhanced knowledge result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIKnowledgeResult {
    pub id: String,
    pub content: String,
    pub relevance_score: f32,
    pub ai_enhanced_score: f32,
    pub content_type: ContentType,
    pub metadata: Option<SearchResultMetadata>,
    pub ai_insights: Vec<AIInsight>,
    pub related_concepts: Vec<String>,
    pub confidence_level: f32,
}

/// AI enhancement applied to knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEnhancement {
    pub enhancement_type: AIEnhancementType,
    pub description: String,
    pub impact_score: f32,
    pub applied_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Types of AI enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIEnhancementType {
    SemanticRelevanceBoost,
    ContextInjection,
    ContentSynthesis,
    QueryExpansion,
    ResultReranking,
    ConfidenceCalibration,
    RelatedContentDiscovery,
    QualityAssessment,
}

/// AI insight about content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInsight {
    pub insight_type: AIInsightType,
    pub title: String,
    pub description: String,
    pub confidence: f32,
    pub relevance_score: f32,
    pub metadata: serde_json::Value,
}

/// Types of AI insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIInsightType {
    ContentQuality,
    RelevanceAssessment,
    CompletenessCheck,
    AccuracyValidation,
    ContextualRelevance,
    TemporalRelevance,
    SourceCredibility,
    CrossReference,
}

/// Knowledge suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSuggestion {
    pub suggestion_id: String,
    pub title: String,
    pub description: String,
    pub suggestion_type: KnowledgeSuggestionType,
    pub priority: SuggestionPriority,
    pub confidence: f32,
    pub action_items: Vec<String>,
    pub estimated_impact: f32,
}

/// Types of knowledge suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeSuggestionType {
    ContentGap,
    QualityImprovement,
    ContextEnhancement,
    RelatedContent,
    UpdateRecommendation,
    Consolidation,
    Archival,
    Indexing,
}

/// Suggestion priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionPriority {
    Critical,
    High,
    Medium,
    Low,
}
