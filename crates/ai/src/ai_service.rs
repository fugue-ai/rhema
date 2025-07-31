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

use cached::TimedCache;
use chrono::{DateTime, Utc};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;
use uuid::Uuid;
use std::path::PathBuf;

// Import lock file context types
use crate::agent::lock_context::*;
use crate::agent::lock_context_integration::LockFileAIIntegration;
use crate::context_injection::{EnhancedContextInjector, TaskType, LockFileContextRequirement};

// Re-export types from lock_context to avoid duplication
pub use crate::agent::lock_context::{
    VersionConflict, CircularDependency, OutdatedDependency, SecurityConcern,
    ConflictSeverity, SecuritySeverity, RecommendationCategory, RecommendationPriority,
    ConflictAnalysis, LockFileAIContext
};

/// AI Service configuration with lock file awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_concurrent_requests: usize,
    pub rate_limit_per_minute: u32,
    pub cache_ttl_seconds: u64,
    pub model_version: String,
    pub enable_caching: bool,
    pub enable_rate_limiting: bool,
    pub enable_monitoring: bool,
    // Lock file awareness configuration
    pub enable_lock_file_awareness: bool,
    pub lock_file_path: Option<PathBuf>,
    pub auto_validate_lock_file: bool,
    pub conflict_prevention_enabled: bool,
    pub dependency_version_consistency: bool,
}

/// AI Request structure with lock file context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub id: String,
    pub prompt: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub created_at: DateTime<Utc>,
    // Lock file context
    pub lock_file_context: Option<LockFileRequestContext>,
    pub task_type: Option<TaskType>,
    pub scope_path: Option<String>,
}

/// Lock file context for AI requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileRequestContext {
    pub include_dependency_versions: bool,
    pub include_conflict_prevention: bool,
    pub include_health_info: bool,
    pub include_recommendations: bool,
    pub target_scopes: Option<Vec<String>>,
    pub include_transitive_deps: bool,
    pub validate_before_processing: bool,
    pub conflict_resolution_mode: ConflictResolutionMode,
}

/// Conflict resolution modes for AI operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionMode {
    Automatic,
    Manual,
    Prompt,
    Skip,
    Fail,
}

/// AI Response structure with lock file awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub id: String,
    pub request_id: String,
    pub content: String,
    pub model_used: String,
    pub model_version: String,
    pub tokens_used: u32,
    pub processing_time_ms: u64,
    pub cached: bool,
    pub created_at: DateTime<Utc>,
    // Lock file awareness
    pub lock_file_validation: Option<LockFileValidationResult>,
    pub dependency_consistency_check: Option<DependencyConsistencyResult>,
    pub conflict_analysis: Option<ConflictAnalysisResult>,
    pub recommendations: Option<Vec<AIRecommendation>>,
}

/// Lock file validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileValidationResult {
    pub is_valid: bool,
    pub validation_score: f64,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Dependency consistency result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConsistencyResult {
    pub is_consistent: bool,
    pub version_conflicts: Vec<VersionConflict>,
    pub circular_dependencies: Vec<CircularDependency>,
    pub outdated_dependencies: Vec<OutdatedDependency>,
    pub security_concerns: Vec<SecurityConcern>,
}

/// Conflict analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictAnalysisResult {
    pub conflicts_detected: bool,
    pub conflict_count: usize,
    pub resolution_suggestions: Vec<ConflictResolutionSuggestion>,
    pub affected_scopes: Vec<String>,
    pub severity: ConflictSeverity,
}

/// AI recommendation for lock file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRecommendation {
    pub category: RecommendationCategory,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub action: String,
    pub confidence: f64,
    pub impact_analysis: Option<String>,
    pub implementation_steps: Option<Vec<String>>,
}

/// Conflict resolution suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionSuggestion {
    pub conflict_type: ConflictType,
    pub description: String,
    pub suggested_action: String,
    pub confidence: f64,
    pub affected_dependencies: Vec<String>,
    pub potential_risks: Vec<String>,
    pub rollback_plan: Option<String>,
}

/// Types of conflicts that can be resolved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    VersionConflict,
    CircularDependency,
    SecurityVulnerability,
    PerformanceIssue,
    CompatibilityIssue,
}

/// AI Model information with lock file awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub name: String,
    pub version: String,
    pub max_tokens: u32,
    pub cost_per_token: f64,
    pub performance_score: f32,
    pub last_updated: DateTime<Utc>,
    // Lock file awareness capabilities
    pub supports_lock_file_context: bool,
    pub lock_file_context_max_tokens: Option<u32>,
    pub conflict_resolution_capabilities: Vec<ConflictType>,
}

/// AI Service metrics with lock file awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub average_response_time_ms: u64,
    pub total_tokens_processed: u64,
    pub total_cost: f64,
    pub last_updated: DateTime<Utc>,
    // Lock file awareness metrics
    pub lock_file_validation_requests: u64,
    pub conflict_resolution_requests: u64,
    pub dependency_consistency_checks: u64,
    pub ai_assisted_resolutions: u64,
    pub validation_failures: u64,
    pub conflict_detections: u64,
}

/// AI Service with lock file awareness
pub struct AIService {
    config: AIServiceConfig,
    _cache: Arc<TimedCache<String, AIResponse>>,
    models: Arc<RwLock<HashMap<String, AIModel>>>,
    metrics: Arc<RwLock<AIServiceMetrics>>,
    client: reqwest::Client,
    // Lock file awareness components
    lock_file_integration: Option<Arc<LockFileAIIntegration>>,
    context_injector: Option<Arc<EnhancedContextInjector>>,
    lock_file_cache: Arc<RwLock<Option<LockFileAIContext>>>,
    dependency_version_cache: Arc<RwLock<HashMap<String, String>>>,
}

impl AIService {
    /// Create a new AI service instance with lock file awareness
    pub async fn new(config: AIServiceConfig) -> RhemaResult<Self> {
        let cache = Arc::new(TimedCache::with_lifespan(config.cache_ttl_seconds));

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| RhemaError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        let models = Arc::new(RwLock::new(HashMap::new()));
        let metrics = Arc::new(RwLock::new(AIServiceMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
            average_response_time_ms: 0,
            total_tokens_processed: 0,
            total_cost: 0.0,
            last_updated: Utc::now(),
            lock_file_validation_requests: 0,
            conflict_resolution_requests: 0,
            dependency_consistency_checks: 0,
            ai_assisted_resolutions: 0,
            validation_failures: 0,
            conflict_detections: 0,
        }));

        // Initialize lock file awareness components
        let (lock_file_integration, context_injector) = if config.enable_lock_file_awareness {
            let lock_file_path = config.lock_file_path.clone().unwrap_or_else(|| PathBuf::from("."));
            let integration = Arc::new(LockFileAIIntegration::new(lock_file_path.clone()));
            let injector = Arc::new(EnhancedContextInjector::new(lock_file_path));
            
            (Some(integration), Some(injector))
        } else {
            (None, None)
        };

        let lock_file_cache = Arc::new(RwLock::new(None));
        let dependency_version_cache = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            config,
            _cache: cache,
            models,
            metrics,
            client,
            lock_file_integration,
            context_injector,
            lock_file_cache,
            dependency_version_cache,
        })
    }

    /// Process an AI request with lock file awareness
    #[instrument(skip(self, request))]
    pub async fn process_request(&self, request: AIRequest) -> RhemaResult<AIResponse> {
        let start_time = std::time::Instant::now();

        // Validate lock file if required
        let lock_file_validation = if self.config.enable_lock_file_awareness && 
            request.lock_file_context.as_ref().map_or(false, |ctx| ctx.validate_before_processing) {
            Some(self.validate_lock_file_before_processing(&request).await?)
        } else {
            None
        };

        // Check dependency consistency if enabled
        let dependency_consistency = if self.config.dependency_version_consistency {
            Some(self.check_dependency_consistency(&request).await?)
        } else {
            None
        };

        // Check for conflicts if prevention is enabled
        let conflict_analysis = if self.config.conflict_prevention_enabled {
            Some(self.analyze_conflicts(&request).await?)
        } else {
            None
        };

        // Enhance prompt with lock file context if available
        let enhanced_request = self.enhance_request_with_lock_file_context(request).await?;

        // Process the enhanced request
        let mut response = self.call_ai_api(&enhanced_request).await?;

        // Add lock file awareness results to response
        response.lock_file_validation = lock_file_validation;
        response.dependency_consistency_check = dependency_consistency;
        response.conflict_analysis = conflict_analysis;

        // Generate AI recommendations if lock file context is available
        if self.config.enable_lock_file_awareness {
            response.recommendations = Some(self.generate_ai_recommendations(&response).await?);
        }

        // Update metrics
        let processing_time = start_time.elapsed().as_millis() as u64;
        self.update_metrics(
            false,
            processing_time,
            response.tokens_used,
            self.calculate_cost(&response),
            &response,
        )
        .await;

        Ok(response)
    }

    /// Validate lock file before processing AI request
    async fn validate_lock_file_before_processing(&self, request: &AIRequest) -> RhemaResult<LockFileValidationResult> {
        if let Some(integration) = &self.lock_file_integration {
            match integration.get_comprehensive_context() {
                Ok(context) => {
                    let health = context.health_assessment;
                    let is_valid = health.overall_score >= 70.0;
                    
                    // Update metrics
                    {
                        let mut metrics = self.metrics.write().await;
                        metrics.lock_file_validation_requests += 1;
                        if !is_valid {
                            metrics.validation_failures += 1;
                        }
                    }

                    Ok(LockFileValidationResult {
                        is_valid,
                        validation_score: health.overall_score,
                        issues: health.issues,
                        warnings: health.warnings,
                        recommendations: health.recommendations,
                    })
                }
                Err(e) => {
                    tracing::warn!("Lock file validation failed: {}", e);
                    Ok(LockFileValidationResult {
                        is_valid: false,
                        validation_score: 0.0,
                        issues: vec![format!("Lock file validation error: {}", e)],
                        warnings: Vec::new(),
                        recommendations: vec!["Fix lock file issues before proceeding".to_string()],
                    })
                }
            }
        } else {
            Ok(LockFileValidationResult {
                is_valid: true,
                validation_score: 100.0,
                issues: Vec::new(),
                warnings: Vec::new(),
                recommendations: Vec::new(),
            })
        }
    }

    /// Check dependency consistency across scopes
    async fn check_dependency_consistency(&self, request: &AIRequest) -> RhemaResult<DependencyConsistencyResult> {
        if let Some(integration) = &self.lock_file_integration {
            match integration.get_conflict_analysis() {
                Ok(conflict_analysis) => {
                    let is_consistent = conflict_analysis.version_conflicts.is_empty() && 
                                       conflict_analysis.circular_dependencies.is_empty();

                    // Update metrics
                    {
                        let mut metrics = self.metrics.write().await;
                        metrics.dependency_consistency_checks += 1;
                        if !is_consistent {
                            metrics.conflict_detections += 1;
                        }
                    }

                    // Get outdated dependencies and security concerns
                    let (outdated_deps, security_concerns) = if let Some(scope_path) = &request.scope_path {
                        if let Ok(scope_context) = integration.get_scope_context(scope_path) {
                            let outdated = scope_context.recommendations.iter()
                                .filter(|r| matches!(r.category, RecommendationCategory::Dependencies))
                                .map(|r| OutdatedDependency {
                                    name: r.title.clone(),
                                    current_version: "unknown".to_string(),
                                    scope: scope_path.clone(),
                                    reason: r.description.clone(),
                                })
                                .collect();

                            let security = scope_context.recommendations.iter()
                                .filter(|r| matches!(r.category, RecommendationCategory::Security))
                                .map(|r| SecurityConcern {
                                    dependency_name: r.title.clone(),
                                    scope: scope_path.clone(),
                                    concern: r.description.clone(),
                                    severity: SecuritySeverity::Medium,
                                })
                                .collect();

                            (outdated, security)
                        } else {
                            (Vec::new(), Vec::new())
                        }
                    } else {
                        (Vec::new(), Vec::new())
                    };

                    Ok(DependencyConsistencyResult {
                        is_consistent,
                        version_conflicts: conflict_analysis.version_conflicts,
                        circular_dependencies: conflict_analysis.circular_dependencies,
                        outdated_dependencies: outdated_deps,
                        security_concerns,
                    })
                }
                Err(e) => {
                    tracing::warn!("Dependency consistency check failed: {}", e);
                    Ok(DependencyConsistencyResult {
                        is_consistent: false,
                        version_conflicts: Vec::new(),
                        circular_dependencies: Vec::new(),
                        outdated_dependencies: Vec::new(),
                        security_concerns: Vec::new(),
                    })
                }
            }
        } else {
            Ok(DependencyConsistencyResult {
                is_consistent: true,
                version_conflicts: Vec::new(),
                circular_dependencies: Vec::new(),
                outdated_dependencies: Vec::new(),
                security_concerns: Vec::new(),
            })
        }
    }

    /// Analyze conflicts for the request
    async fn analyze_conflicts(&self, request: &AIRequest) -> RhemaResult<ConflictAnalysisResult> {
        if let Some(integration) = &self.lock_file_integration {
            match integration.get_conflict_analysis() {
                Ok(conflict_analysis) => {
                    let conflicts_detected = !conflict_analysis.version_conflicts.is_empty() || 
                                           !conflict_analysis.circular_dependencies.is_empty();
                    
                    let conflict_count = conflict_analysis.version_conflicts.len() + 
                                        conflict_analysis.circular_dependencies.len();

                    let resolution_suggestions = self.generate_conflict_resolution_suggestions(&conflict_analysis).await?;
                    
                    let affected_scopes = conflict_analysis.dependency_graph.keys().cloned().collect();
                    
                    let severity = if conflict_analysis.circular_dependencies.is_empty() {
                        ConflictSeverity::Medium
                    } else {
                        ConflictSeverity::High
                    };

                    // Update metrics
                    {
                        let mut metrics = self.metrics.write().await;
                        metrics.conflict_resolution_requests += 1;
                        if conflicts_detected {
                            metrics.conflict_detections += 1;
                        }
                    }

                    Ok(ConflictAnalysisResult {
                        conflicts_detected,
                        conflict_count,
                        resolution_suggestions,
                        affected_scopes,
                        severity,
                    })
                }
                Err(e) => {
                    tracing::warn!("Conflict analysis failed: {}", e);
                    Ok(ConflictAnalysisResult {
                        conflicts_detected: false,
                        conflict_count: 0,
                        resolution_suggestions: Vec::new(),
                        affected_scopes: Vec::new(),
                        severity: ConflictSeverity::Low,
                    })
                }
            }
        } else {
            Ok(ConflictAnalysisResult {
                conflicts_detected: false,
                conflict_count: 0,
                resolution_suggestions: Vec::new(),
                affected_scopes: Vec::new(),
                severity: ConflictSeverity::Low,
            })
        }
    }

    /// Generate conflict resolution suggestions
    async fn generate_conflict_resolution_suggestions(&self, conflict_analysis: &ConflictAnalysis) -> RhemaResult<Vec<ConflictResolutionSuggestion>> {
        let mut suggestions = Vec::new();

        // Generate suggestions for version conflicts
        for conflict in &conflict_analysis.version_conflicts {
            suggestions.push(ConflictResolutionSuggestion {
                conflict_type: ConflictType::VersionConflict,
                description: format!("Version conflict for {}: {} vs {}", 
                    conflict.dependency_name, conflict.version1, conflict.version2),
                suggested_action: format!("Update {} to version {} in scope {}", 
                    conflict.dependency_name, conflict.version2, conflict.scope1),
                confidence: 0.8,
                affected_dependencies: vec![conflict.dependency_name.clone()],
                potential_risks: vec!["Breaking changes may occur".to_string()],
                rollback_plan: Some("Revert to previous version if issues arise".to_string()),
            });
        }

        // Generate suggestions for circular dependencies
        for circular in &conflict_analysis.circular_dependencies {
            suggestions.push(ConflictResolutionSuggestion {
                conflict_type: ConflictType::CircularDependency,
                description: circular.description.clone(),
                suggested_action: "Break circular dependency by restructuring dependencies".to_string(),
                confidence: 0.9,
                affected_dependencies: circular.affected_scopes.clone(),
                potential_risks: vec!["Build failures".to_string(), "Runtime issues".to_string()],
                rollback_plan: Some("Restore previous dependency structure".to_string()),
            });
        }

        Ok(suggestions)
    }

    /// Enhance request with lock file context
    async fn enhance_request_with_lock_file_context(&self, mut request: AIRequest) -> RhemaResult<AIRequest> {
        if let Some(lock_context) = &request.lock_file_context {
            if let Some(injector) = &self.context_injector {
                let pattern = rhema_core::schema::PromptPattern {
                    id: "ai_service_context".to_string(),
                    name: "AI Service Context".to_string(),
                    description: Some("Context injection for AI service requests".to_string()),
                    template: request.prompt.clone(),
                    injection: rhema_core::schema::PromptInjectionMethod::Prepend,
                    usage_analytics: rhema_core::schema::UsageAnalytics::new(),
                    version: rhema_core::schema::PromptVersion::new("1.0.0"),
                    tags: None,
                };

                let requirement = LockFileContextRequirement {
                    include_dependency_versions: lock_context.include_dependency_versions,
                    include_conflict_prevention: lock_context.include_conflict_prevention,
                    include_health_info: lock_context.include_health_info,
                    include_recommendations: lock_context.include_recommendations,
                    target_scopes: lock_context.target_scopes.clone(),
                    include_transitive_deps: lock_context.include_transitive_deps,
                };

                let scope_path = request.scope_path.as_deref().unwrap_or(".");
                let enhanced_prompt = injector.inject_lock_file_context(&pattern, scope_path, &requirement).await?;
                request.prompt = enhanced_prompt;
            }
        }

        Ok(request)
    }

    /// Generate AI recommendations based on response and lock file context
    async fn generate_ai_recommendations(&self, response: &AIResponse) -> RhemaResult<Vec<AIRecommendation>> {
        let mut recommendations = Vec::new();

        // Generate recommendations based on validation results
        if let Some(validation) = &response.lock_file_validation {
            if !validation.is_valid {
                recommendations.push(AIRecommendation {
                    category: RecommendationCategory::Validation,
                    priority: RecommendationPriority::High,
                    title: "Fix Lock File Validation Issues".to_string(),
                    description: format!("Lock file validation score: {:.1}/100", validation.validation_score),
                    action: "Review and fix validation issues before proceeding".to_string(),
                    confidence: 0.9,
                    impact_analysis: Some("Validation issues may cause build or runtime problems".to_string()),
                    implementation_steps: Some(validation.recommendations.clone()),
                });
            }
        }

        // Generate recommendations based on dependency consistency
        if let Some(consistency) = &response.dependency_consistency_check {
            if !consistency.is_consistent {
                for conflict in &consistency.version_conflicts {
                    recommendations.push(AIRecommendation {
                        category: RecommendationCategory::Dependencies,
                        priority: RecommendationPriority::High,
                        title: format!("Resolve Version Conflict: {}", conflict.dependency_name),
                        description: format!("Version conflict between {} and {}", conflict.version1, conflict.version2),
                        action: format!("Update {} to version {} in scope {}", 
                            conflict.dependency_name, conflict.version2, conflict.scope1),
                        confidence: 0.8,
                        impact_analysis: Some("Version conflicts may cause runtime issues".to_string()),
                        implementation_steps: Some(vec![
                            "Update dependency version".to_string(),
                            "Test for compatibility".to_string(),
                            "Update lock file".to_string(),
                        ]),
                    });
                }
            }
        }

        // Generate recommendations based on conflict analysis
        if let Some(conflict_analysis) = &response.conflict_analysis {
            if conflict_analysis.conflicts_detected {
                recommendations.push(AIRecommendation {
                    category: RecommendationCategory::Dependencies,
                    priority: RecommendationPriority::High,
                    title: "Resolve Dependency Conflicts".to_string(),
                    description: format!("{} conflicts detected", conflict_analysis.conflict_count),
                    action: "Review and resolve all detected conflicts".to_string(),
                    confidence: 0.9,
                    impact_analysis: Some("Unresolved conflicts may cause build failures".to_string()),
                    implementation_steps: Some(vec![
                        "Review conflict analysis".to_string(),
                        "Apply resolution suggestions".to_string(),
                        "Test changes".to_string(),
                        "Update lock file".to_string(),
                    ]),
                });
            }
        }

        Ok(recommendations)
    }

    /// Call the AI API
    #[instrument(skip(self, request))]
    async fn call_ai_api(&self, request: &AIRequest) -> RhemaResult<AIResponse> {
        let request_body = serde_json::json!({
            "model": request.model,
            "prompt": request.prompt,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
        });

        let response = self.client
            .post(&format!("{}/v1/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| RhemaError::NetworkError(format!("API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(RhemaError::AgentError(format!("API error: {}", error_text)));
        }

        let response_json: serde_json::Value = response.json().await
            .map_err(|e| RhemaError::ParseError(format!("Failed to parse response: {}", e)))?;

        // Extract response content (simplified for this example)
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No content")
            .to_string();

        let tokens_used = response_json["usage"]["total_tokens"]
            .as_u64()
            .unwrap_or(0) as u32;

        Ok(AIResponse {
            id: Uuid::new_v4().to_string(),
            request_id: request.id.clone(),
            content,
            model_used: request.model.clone(),
            model_version: self.config.model_version.clone(),
            tokens_used,
            processing_time_ms: 0, // Will be set by caller
            cached: false,
            created_at: Utc::now(),
            lock_file_validation: None, // Will be set by caller
            dependency_consistency_check: None, // Will be set by caller
            conflict_analysis: None, // Will be set by caller
            recommendations: None, // Will be set by caller
        })
    }

    /// Generate cache key for request
    fn generate_cache_key(&self, request: &AIRequest) -> String {
        let mut key_parts = vec![
            request.model.clone(),
            request.prompt.clone(),
            request.temperature.to_string(),
            request.max_tokens.to_string(),
        ];

        // Include lock file context in cache key if available
        if let Some(lock_context) = &request.lock_file_context {
            key_parts.push(format!("lock_context_{}", serde_json::to_string(lock_context).unwrap_or_default()));
        }

        if let Some(task_type) = &request.task_type {
            key_parts.push(format!("task_{:?}", task_type));
        }

        if let Some(scope_path) = &request.scope_path {
            key_parts.push(format!("scope_{}", scope_path));
        }

        Sha256::digest(key_parts.join("|").as_bytes())
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    /// Update metrics with lock file awareness
    async fn update_metrics(
        &self,
        cache_hit: bool,
        processing_time_ms: u64,
        tokens_used: u32,
        cost: f64,
        response: &AIResponse,
    ) {
        let mut metrics = self.metrics.write().await;
        
        metrics.total_requests += 1;
        if cache_hit {
            metrics.cache_hits += 1;
        } else {
            metrics.cache_misses += 1;
        }
        
        metrics.total_tokens_processed += tokens_used as u64;
        metrics.total_cost += cost;
        
        // Update average response time
        let total_time = metrics.average_response_time_ms * (metrics.total_requests - 1) + processing_time_ms;
        metrics.average_response_time_ms = total_time / metrics.total_requests;
        
        // Update lock file awareness metrics
        if response.lock_file_validation.is_some() {
            metrics.lock_file_validation_requests += 1;
        }
        
        if response.conflict_analysis.is_some() {
            metrics.conflict_resolution_requests += 1;
        }
        
        if response.dependency_consistency_check.is_some() {
            metrics.dependency_consistency_checks += 1;
        }
        
        if let Some(recommendations) = &response.recommendations {
            if !recommendations.is_empty() {
                metrics.ai_assisted_resolutions += 1;
            }
        }
        
        metrics.last_updated = Utc::now();
    }

    /// Calculate cost for response
    fn calculate_cost(&self, response: &AIResponse) -> f64 {
        // Simplified cost calculation
        response.tokens_used as f64 * 0.0001
    }

    /// Get comprehensive metrics including lock file awareness
    pub async fn get_metrics(&self) -> AIServiceMetrics {
        self.metrics.read().await.clone()
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        // Simplified cache stats
        (0, 0)
    }

    /// Clear cache
    pub fn clear_cache(&self) {
        // Cache clearing implementation
    }

    /// Update model information
    pub async fn update_model(&self, model: AIModel) {
        let mut models = self.models.write().await;
        models.insert(model.name.clone(), model);
    }

    /// Get model by name
    pub async fn get_model(&self, name: &str) -> Option<AIModel> {
        let models = self.models.read().await;
        models.get(name).cloned()
    }

    /// Get all models
    pub async fn get_all_models(&self) -> Vec<AIModel> {
        let models = self.models.read().await;
        models.values().cloned().collect()
    }

    /// Health check with lock file awareness
    pub async fn health_check(&self) -> RhemaResult<()> {
        // Basic health check
        if self.config.api_key.is_empty() {
            return Err(RhemaError::ConfigError("API key is empty".to_string()));
        }

        // Lock file health check if enabled
        if self.config.enable_lock_file_awareness {
            if let Some(integration) = &self.lock_file_integration {
                if let Ok(context) = integration.get_comprehensive_context() {
                    if context.health_assessment.overall_score < 50.0 {
                        return Err(RhemaError::ConfigError(
                            format!("Lock file health is poor: {:.1}/100", context.health_assessment.overall_score)
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Get lock file context for AI operations
    pub async fn get_lock_file_context(&self) -> RhemaResult<Option<LockFileAIContext>> {
        if let Some(integration) = &self.lock_file_integration {
            integration.get_comprehensive_context().map(Some)
        } else {
            Ok(None)
        }
    }

    /// Validate lock file consistency
    pub async fn validate_lock_file_consistency(&self) -> RhemaResult<DependencyConsistencyResult> {
        if let Some(integration) = &self.lock_file_integration {
            self.check_dependency_consistency(&AIRequest {
                id: Uuid::new_v4().to_string(),
                prompt: "".to_string(),
                model: "".to_string(),
                temperature: 0.0,
                max_tokens: 0,
                user_id: None,
                session_id: None,
                created_at: Utc::now(),
                lock_file_context: None,
                task_type: None,
                scope_path: None,
            }).await
        } else {
            Ok(DependencyConsistencyResult {
                is_consistent: true,
                version_conflicts: Vec::new(),
                circular_dependencies: Vec::new(),
                outdated_dependencies: Vec::new(),
                security_concerns: Vec::new(),
            })
        }
    }

    /// Generate AI-assisted conflict resolution
    pub async fn generate_conflict_resolution(&self, conflict_type: ConflictType, context: &str) -> RhemaResult<Vec<ConflictResolutionSuggestion>> {
        let prompt = format!(
            "Analyze the following conflict and provide resolution suggestions:\n\nConflict Type: {:?}\nContext: {}\n\nProvide specific, actionable steps to resolve this conflict.",
            conflict_type, context
        );

        let request = AIRequest {
            id: Uuid::new_v4().to_string(),
            prompt,
            model: "gpt-4".to_string(),
            temperature: 0.3,
            max_tokens: 1000,
            user_id: None,
            session_id: None,
            created_at: Utc::now(),
            lock_file_context: Some(LockFileRequestContext {
                include_dependency_versions: true,
                include_conflict_prevention: true,
                include_health_info: true,
                include_recommendations: true,
                target_scopes: None,
                include_transitive_deps: true,
                validate_before_processing: false,
                conflict_resolution_mode: ConflictResolutionMode::Automatic,
            }),
            task_type: Some(TaskType::ConflictResolution),
            scope_path: None,
        };

        let response = self.process_request(request).await?;
        
        // Parse response to extract resolution suggestions
        // This is a simplified implementation - in practice, you'd want more sophisticated parsing
        let suggestions = vec![ConflictResolutionSuggestion {
            conflict_type,
            description: "AI-generated resolution".to_string(),
            suggested_action: response.content,
            confidence: 0.7,
            affected_dependencies: Vec::new(),
            potential_risks: vec!["Review changes before applying".to_string()],
            rollback_plan: Some("Test thoroughly before committing".to_string()),
        }];

        Ok(suggestions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_service_creation() {
        let config = AIServiceConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com".to_string(),
            timeout_seconds: 30,
            max_concurrent_requests: 10,
            rate_limit_per_minute: 60,
            cache_ttl_seconds: 3600,
            model_version: "1.0".to_string(),
            enable_caching: true,
            enable_rate_limiting: false,
            enable_monitoring: true,
            enable_lock_file_awareness: true,
            lock_file_path: Some(PathBuf::from(".")),
            auto_validate_lock_file: true,
            conflict_prevention_enabled: true,
            dependency_version_consistency: true,
        };

        let service = AIService::new(config).await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_cache_key_generation() {
        let config = AIServiceConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com".to_string(),
            timeout_seconds: 30,
            max_concurrent_requests: 10,
            rate_limit_per_minute: 60,
            cache_ttl_seconds: 3600,
            model_version: "1.0".to_string(),
            enable_caching: true,
            enable_rate_limiting: false,
            enable_monitoring: true,
            enable_lock_file_awareness: false,
            lock_file_path: None,
            auto_validate_lock_file: false,
            conflict_prevention_enabled: false,
            dependency_version_consistency: false,
        };

        let service = AIService::new(config).await.unwrap();
        
        let request = AIRequest {
            id: "test-id".to_string(),
            prompt: "test prompt".to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.5,
            max_tokens: 100,
            user_id: None,
            session_id: None,
            created_at: Utc::now(),
            lock_file_context: None,
            task_type: None,
            scope_path: None,
        };

        let cache_key = service.generate_cache_key(&request);
        assert!(!cache_key.is_empty());
    }
}
