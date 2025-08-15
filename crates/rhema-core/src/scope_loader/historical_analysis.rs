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

use crate::scope_loader::{pattern_recognition::*, ScopeContext, ScopeSuggestion, ScopeType};
use crate::{
    audit::{log_audit_event, AuditEvent, AuditEventType, AuditSeverity},
    validation::ValidationRules,
    RhemaError, RhemaResult,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Historical scope creation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeCreationRecord {
    /// Record ID
    pub id: String,
    /// Repository path
    pub repository_path: PathBuf,
    /// Scope suggestion that was created
    pub suggestion: ScopeSuggestion,
    /// When the scope was created
    pub created_at: DateTime<Utc>,
    /// Who created the scope
    pub created_by: String,
    /// Context when scope was created
    pub context: ScopeCreationContext,
    /// Success metrics
    pub success_metrics: ScopeSuccessMetrics,
    /// Patterns detected
    pub patterns_detected: Vec<ArchitecturalPattern>,
}

/// Context when scope was created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeCreationContext {
    /// Repository size at creation time
    pub repository_size: RepositorySize,
    /// Technology stack
    pub technology_stack: Vec<TechnologyStackPattern>,
    /// Project structure
    pub project_structure: Vec<ProjectStructurePattern>,
    /// Team size
    pub team_size: TeamSize,
    /// Development phase
    pub development_phase: DevelopmentPhase,
    /// Custom context data
    pub custom_data: HashMap<String, String>,
}

/// Repository size classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RepositorySize {
    /// Small repository (< 1MB)
    Small,
    /// Medium repository (1MB - 100MB)
    Medium,
    /// Large repository (100MB - 1GB)
    Large,
    /// Very large repository (> 1GB)
    VeryLarge,
}

/// Team size classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TeamSize {
    /// Solo developer
    Solo,
    /// Small team (2-5 developers)
    Small,
    /// Medium team (6-20 developers)
    Medium,
    /// Large team (21-100 developers)
    Large,
    /// Enterprise team (> 100 developers)
    Enterprise,
}

/// Development phase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DevelopmentPhase {
    /// Initial setup
    Initial,
    /// Active development
    Active,
    /// Maintenance
    Maintenance,
    /// Legacy
    Legacy,
}

/// Success metrics for scope creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeSuccessMetrics {
    /// Whether the scope is still in use
    pub still_in_use: bool,
    /// Usage frequency (0.0 - 1.0)
    pub usage_frequency: f64,
    /// User satisfaction score (0.0 - 1.0)
    pub satisfaction_score: f64,
    /// Performance impact score (0.0 - 1.0)
    pub performance_impact: f64,
    /// Maintenance burden score (0.0 - 1.0)
    pub maintenance_burden: f64,
    /// Last accessed timestamp
    pub last_accessed: Option<DateTime<Utc>>,
    /// Number of modifications
    pub modification_count: usize,
}

/// Historical analysis patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalPattern {
    /// Pattern ID
    pub id: String,
    /// Pattern name
    pub name: String,
    /// Pattern description
    pub description: String,
    /// Confidence score
    pub confidence: f64,
    /// Frequency of occurrence
    pub frequency: f64,
    /// Success rate
    pub success_rate: f64,
    /// Associated scope types
    pub associated_scope_types: Vec<ScopeType>,
    /// Associated patterns
    pub associated_patterns: Vec<ArchitecturalPattern>,
    /// Context conditions
    pub context_conditions: PatternContextConditions,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Context conditions for patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternContextConditions {
    /// Repository size conditions
    pub repository_size: Vec<RepositorySize>,
    /// Team size conditions
    pub team_size: Vec<TeamSize>,
    /// Technology stack conditions
    pub technology_stack: Vec<TechnologyStackPattern>,
    /// Development phase conditions
    pub development_phase: Vec<DevelopmentPhase>,
    /// Custom conditions
    pub custom_conditions: HashMap<String, String>,
}

/// Predictive scope suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveSuggestion {
    /// Base suggestion
    pub suggestion: ScopeSuggestion,
    /// Prediction confidence
    pub prediction_confidence: f64,
    /// Historical pattern used
    pub historical_pattern: Option<HistoricalPattern>,
    /// Similar successful scopes
    pub similar_scopes: Vec<ScopeCreationRecord>,
    /// Predicted success metrics
    pub predicted_metrics: ScopeSuccessMetrics,
    /// Reasoning for prediction
    pub reasoning: String,
}

/// Historical analysis engine
pub struct HistoricalAnalysisEngine {
    /// Historical records
    records: Arc<Mutex<Vec<ScopeCreationRecord>>>,
    /// Historical patterns
    patterns: Arc<Mutex<Vec<HistoricalPattern>>>,
    /// Pattern cache
    pattern_cache: Arc<Mutex<HashMap<String, Vec<HistoricalPattern>>>>,
    /// Analysis configuration
    config: HistoricalAnalysisConfig,
}

/// Historical analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalAnalysisConfig {
    /// Minimum confidence for pattern recognition
    pub min_pattern_confidence: f64,
    /// Minimum frequency for pattern recognition
    pub min_pattern_frequency: f64,
    /// Maximum age for historical data (days)
    pub max_data_age_days: u32,
    /// Pattern update interval (hours)
    pub pattern_update_interval_hours: u32,
    /// Enable predictive suggestions
    pub enable_predictions: bool,
    /// Enable pattern learning
    pub enable_learning: bool,
    /// Analysis depth
    pub analysis_depth: AnalysisDepth,
}

/// Analysis depth configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalysisDepth {
    /// Basic analysis
    Basic,
    /// Standard analysis
    Standard,
    /// Deep analysis
    Deep,
    /// Comprehensive analysis
    Comprehensive,
}

impl HistoricalAnalysisEngine {
    /// Create a new historical analysis engine
    pub fn new(config: HistoricalAnalysisConfig) -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
            patterns: Arc::new(Mutex::new(Vec::new())),
            pattern_cache: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Add a scope creation record
    pub async fn add_record(&self, record: ScopeCreationRecord) -> RhemaResult<()> {
        // Validate record
        ValidationRules::validate_scope_path_static(&record.repository_path)?;
        ValidationRules::validate_title_static(&record.suggestion.name)?;

        let mut records = self.records.lock().unwrap();
        records.push(record.clone());

        // Log record addition
        let event = AuditEvent::new(
            AuditEventType::SystemEvent,
            AuditSeverity::Info,
            "historical_record_added".to_string(),
            true,
        )
        .with_detail("record_id".to_string(), record.id)
        .with_detail("scope_name".to_string(), record.suggestion.name)
        .with_detail(
            "repository_path".to_string(),
            record.repository_path.to_string_lossy().to_string(),
        );

        log_audit_event(event)?;

        // Update patterns if learning is enabled
        if self.config.enable_learning {
            self.update_patterns().await?;
        }

        Ok(())
    }

    /// Analyze historical patterns
    pub async fn analyze_patterns(&self) -> RhemaResult<Vec<HistoricalPattern>> {
        let records = self.records.lock().unwrap();

        if records.is_empty() {
            return Ok(Vec::new());
        }

        let mut patterns = Vec::new();
        let mut pattern_groups: HashMap<String, Vec<&ScopeCreationRecord>> = HashMap::new();

        // Group records by similar characteristics
        for record in records.iter() {
            let pattern_key = self.generate_pattern_key(record);
            pattern_groups.entry(pattern_key).or_default().push(record);
        }

        // Analyze each pattern group
        for (pattern_key, group_records) in pattern_groups {
            if group_records.len() < 2 {
                continue; // Need at least 2 records to form a pattern
            }

            let frequency = group_records.len() as f64 / records.len() as f64;

            if frequency < self.config.min_pattern_frequency {
                continue;
            }

            let success_rate = self.calculate_success_rate(&group_records);
            let confidence = self.calculate_pattern_confidence(&group_records);

            if confidence < self.config.min_pattern_confidence {
                continue;
            }

            let pattern = HistoricalPattern {
                id: Uuid::new_v4().to_string(),
                name: format!("Pattern_{}", pattern_key),
                description: self.generate_pattern_description(&group_records),
                confidence,
                frequency,
                success_rate,
                associated_scope_types: self.extract_scope_types(&group_records),
                associated_patterns: self.extract_architectural_patterns(&group_records),
                context_conditions: self.extract_context_conditions(&group_records),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            patterns.push(pattern);
        }

        // Update patterns cache
        let mut pattern_cache = self.pattern_cache.lock().unwrap();
        pattern_cache.clear();

        for pattern in &patterns {
            for scope_type in &pattern.associated_scope_types {
                pattern_cache
                    .entry(format!("{:?}", scope_type))
                    .or_default()
                    .push(pattern.clone());
            }
        }

        Ok(patterns)
    }

    /// Generate predictive suggestions
    pub async fn generate_predictive_suggestions(
        &self,
        context: &ScopeContext,
    ) -> RhemaResult<Vec<PredictiveSuggestion>> {
        if !self.config.enable_predictions {
            return Ok(Vec::new());
        }

        let mut suggestions = Vec::new();
        let patterns = self.patterns.lock().unwrap();
        let records = self.records.lock().unwrap();

        // Find relevant patterns for the context
        let relevant_patterns = self.find_relevant_patterns(context, &patterns).await?;

        for pattern in relevant_patterns {
            // Find similar successful scopes
            let similar_scopes = self
                .find_similar_scopes(context, &pattern, &records)
                .await?;

            if similar_scopes.is_empty() {
                continue;
            }

            // Generate suggestion based on pattern
            let suggestion = self
                .generate_suggestion_from_pattern(context, &pattern, &similar_scopes)
                .await?;

            // Calculate prediction confidence
            let prediction_confidence =
                self.calculate_prediction_confidence(&pattern, &similar_scopes);

            // Predict success metrics
            let predicted_metrics = self.predict_success_metrics(&pattern, &similar_scopes);

            // Generate reasoning
            let reasoning = self.generate_prediction_reasoning(&pattern, &similar_scopes);

            let predictive_suggestion = PredictiveSuggestion {
                suggestion,
                prediction_confidence,
                historical_pattern: Some(pattern.clone()),
                similar_scopes,
                predicted_metrics,
                reasoning,
            };

            suggestions.push(predictive_suggestion);
        }

        // Sort by prediction confidence
        suggestions.sort_by(|a, b| {
            b.prediction_confidence
                .partial_cmp(&a.prediction_confidence)
                .unwrap()
        });

        Ok(suggestions)
    }

    /// Learn from successful scope creations
    pub async fn learn_from_success(&self, record: &ScopeCreationRecord) -> RhemaResult<()> {
        if !self.config.enable_learning {
            return Ok(());
        }

        // Update success metrics
        let mut records = self.records.lock().unwrap();
        if let Some(existing_record) = records.iter_mut().find(|r| r.id == record.id) {
            existing_record.success_metrics = record.success_metrics.clone();
        }

        // Update patterns based on new success data
        self.update_patterns().await?;

        Ok(())
    }

    /// Get historical statistics
    pub async fn get_statistics(&self) -> RhemaResult<HistoricalStatistics> {
        let records = self.records.lock().unwrap();
        let patterns = self.patterns.lock().unwrap();

        let total_records = records.len();
        let total_patterns = patterns.len();

        let success_rate = if total_records > 0 {
            records
                .iter()
                .map(|r| r.success_metrics.satisfaction_score)
                .sum::<f64>()
                / total_records as f64
        } else {
            0.0
        };

        let avg_confidence = if total_records > 0 {
            records.iter().map(|r| r.suggestion.confidence).sum::<f64>() / total_records as f64
        } else {
            0.0
        };

        let scope_type_distribution = self.calculate_scope_type_distribution(&records);
        let pattern_distribution = self.calculate_pattern_distribution(&patterns);

        Ok(HistoricalStatistics {
            total_records,
            total_patterns,
            success_rate,
            avg_confidence,
            scope_type_distribution,
            pattern_distribution,
            last_updated: Utc::now(),
        })
    }

    /// Generate pattern key for grouping
    fn generate_pattern_key(&self, record: &ScopeCreationRecord) -> String {
        format!(
            "{:?}_{:?}_{:?}",
            record.suggestion.scope_type, record.context.technology_stack, record.context.team_size
        )
    }

    /// Calculate success rate for a group of records
    fn calculate_success_rate(&self, records: &[&ScopeCreationRecord]) -> f64 {
        let total = records.len();
        let successful = records
            .iter()
            .filter(|r| r.success_metrics.satisfaction_score > 0.7)
            .count();

        successful as f64 / total as f64
    }

    /// Calculate pattern confidence
    fn calculate_pattern_confidence(&self, records: &[&ScopeCreationRecord]) -> f64 {
        let avg_confidence =
            records.iter().map(|r| r.suggestion.confidence).sum::<f64>() / records.len() as f64;

        let consistency = self.calculate_consistency_score(records);

        (avg_confidence + consistency) / 2.0
    }

    /// Calculate consistency score
    fn calculate_consistency_score(&self, records: &[&ScopeCreationRecord]) -> f64 {
        if records.len() < 2 {
            return 0.0;
        }

        let scopes: Vec<&ScopeSuggestion> = records.iter().map(|r| &r.suggestion).collect();
        let mut consistency = 0.0;
        let mut comparisons = 0;

        for i in 0..scopes.len() {
            for j in (i + 1)..scopes.len() {
                consistency += self.calculate_similarity(&scopes[i], &scopes[j]);
                comparisons += 1;
            }
        }

        if comparisons > 0 {
            consistency / comparisons as f64
        } else {
            0.0
        }
    }

    /// Calculate similarity between two scope suggestions
    fn calculate_similarity(
        &self,
        suggestion1: &ScopeSuggestion,
        suggestion2: &ScopeSuggestion,
    ) -> f64 {
        let mut similarity = 0.0;
        let mut factors = 0;

        // Type similarity
        if suggestion1.scope_type == suggestion2.scope_type {
            similarity += 1.0;
        }
        factors += 1;

        // Name similarity
        let name_similarity =
            self.calculate_string_similarity(&suggestion1.name, &suggestion2.name);
        similarity += name_similarity;
        factors += 1;

        // Path similarity
        let path_similarity = self.calculate_path_similarity(&suggestion1.path, &suggestion2.path);
        similarity += path_similarity;
        factors += 1;

        similarity / factors as f64
    }

    /// Calculate string similarity
    fn calculate_string_similarity(&self, str1: &str, str2: &str) -> f64 {
        let longer = str1.len().max(str2.len());
        if longer == 0 {
            return 1.0;
        }

        let distance = self.levenshtein_distance(str1, str2);
        1.0 - (distance as f64 / longer as f64)
    }

    /// Calculate path similarity
    fn calculate_path_similarity(&self, path1: &Path, path2: &Path) -> f64 {
        let components1: Vec<_> = path1.components().collect();
        let components2: Vec<_> = path2.components().collect();

        let longer = components1.len().max(components2.len());
        if longer == 0 {
            return 1.0;
        }

        let common = components1
            .iter()
            .zip(components2.iter())
            .filter(|(a, b)| a == b)
            .count();

        common as f64 / longer as f64
    }

    /// Calculate Levenshtein distance
    fn levenshtein_distance(&self, str1: &str, str2: &str) -> usize {
        let len1 = str1.chars().count();
        let len2 = str2.chars().count();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for (i, char1) in str1.chars().enumerate() {
            for (j, char2) in str2.chars().enumerate() {
                let cost = if char1 == char2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                    .min(matrix[i + 1][j] + 1)
                    .min(matrix[i][j] + cost);
            }
        }

        matrix[len1][len2]
    }

    /// Generate pattern description
    fn generate_pattern_description(&self, records: &[&ScopeCreationRecord]) -> String {
        let scope_types: HashSet<_> = records
            .iter()
            .map(|r| format!("{:?}", r.suggestion.scope_type))
            .collect();

        let tech_stacks: HashSet<_> = records
            .iter()
            .flat_map(|r| r.context.technology_stack.iter())
            .map(|t| format!("{:?}", t))
            .collect();

        format!(
            "Pattern for {} scopes in {} projects",
            scope_types.iter().next().unwrap_or(&"Unknown".to_string()),
            tech_stacks.iter().next().unwrap_or(&"Unknown".to_string())
        )
    }

    /// Extract scope types from records
    fn extract_scope_types(&self, records: &[&ScopeCreationRecord]) -> Vec<ScopeType> {
        records
            .iter()
            .map(|r| r.suggestion.scope_type.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Extract architectural patterns from records
    fn extract_architectural_patterns(
        &self,
        records: &[&ScopeCreationRecord],
    ) -> Vec<ArchitecturalPattern> {
        records
            .iter()
            .flat_map(|r| r.patterns_detected.iter())
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Extract context conditions from records
    fn extract_context_conditions(
        &self,
        records: &[&ScopeCreationRecord],
    ) -> PatternContextConditions {
        let repository_sizes: HashSet<_> = records
            .iter()
            .map(|r| r.context.repository_size.clone())
            .collect();

        let team_sizes: HashSet<_> = records
            .iter()
            .map(|r| r.context.team_size.clone())
            .collect();

        let tech_stacks: HashSet<_> = records
            .iter()
            .flat_map(|r| r.context.technology_stack.iter())
            .cloned()
            .collect();

        let dev_phases: HashSet<_> = records
            .iter()
            .map(|r| r.context.development_phase.clone())
            .collect();

        PatternContextConditions {
            repository_size: repository_sizes.into_iter().collect(),
            team_size: team_sizes.into_iter().collect(),
            technology_stack: tech_stacks.into_iter().collect(),
            development_phase: dev_phases.into_iter().collect(),
            custom_conditions: HashMap::new(),
        }
    }

    /// Find relevant patterns for context
    async fn find_relevant_patterns(
        &self,
        context: &ScopeContext,
        patterns: &[HistoricalPattern],
    ) -> RhemaResult<Vec<HistoricalPattern>> {
        let mut relevant_patterns = Vec::new();

        for pattern in patterns {
            if self.pattern_matches_context(pattern, context).await? {
                relevant_patterns.push(pattern.clone());
            }
        }

        // Sort by confidence and success rate
        relevant_patterns.sort_by(|a, b| {
            let score_a = a.confidence * a.success_rate;
            let score_b = b.confidence * b.success_rate;
            score_b.partial_cmp(&score_a).unwrap()
        });

        Ok(relevant_patterns)
    }

    /// Check if pattern matches context
    async fn pattern_matches_context(
        &self,
        pattern: &HistoricalPattern,
        context: &ScopeContext,
    ) -> RhemaResult<bool> {
        // This is a simplified implementation
        // In a real implementation, you would analyze the context more thoroughly
        Ok(true)
    }

    /// Find similar scopes
    async fn find_similar_scopes(
        &self,
        context: &ScopeContext,
        pattern: &HistoricalPattern,
        records: &[ScopeCreationRecord],
    ) -> RhemaResult<Vec<ScopeCreationRecord>> {
        let mut similar_scopes = Vec::new();

        for record in records {
            if self.scope_matches_pattern(record, pattern) {
                similar_scopes.push(record.clone());
            }
        }

        // Sort by success metrics
        similar_scopes.sort_by(|a, b| {
            b.success_metrics
                .satisfaction_score
                .partial_cmp(&a.success_metrics.satisfaction_score)
                .unwrap()
        });

        // Limit to top results
        similar_scopes.truncate(10);

        Ok(similar_scopes)
    }

    /// Check if scope matches pattern
    fn scope_matches_pattern(
        &self,
        record: &ScopeCreationRecord,
        pattern: &HistoricalPattern,
    ) -> bool {
        pattern
            .associated_scope_types
            .contains(&record.suggestion.scope_type)
    }

    /// Generate suggestion from pattern
    async fn generate_suggestion_from_pattern(
        &self,
        context: &ScopeContext,
        pattern: &HistoricalPattern,
        similar_scopes: &[ScopeCreationRecord],
    ) -> RhemaResult<ScopeSuggestion> {
        // Find the most successful similar scope
        let best_scope = similar_scopes
            .iter()
            .max_by(|a, b| {
                a.success_metrics
                    .satisfaction_score
                    .partial_cmp(&b.success_metrics.satisfaction_score)
                    .unwrap()
            })
            .ok_or_else(|| RhemaError::NotFound("No similar scopes found".to_string()))?;

        // Create a new suggestion based on the best scope
        let mut suggestion = best_scope.suggestion.clone();
        // Note: ScopeContext doesn't have a path field, so we'll use the original path
        suggestion.confidence = pattern.confidence;

        Ok(suggestion)
    }

    /// Calculate prediction confidence
    fn calculate_prediction_confidence(
        &self,
        pattern: &HistoricalPattern,
        similar_scopes: &[ScopeCreationRecord],
    ) -> f64 {
        let pattern_confidence = pattern.confidence;
        let avg_similar_confidence = similar_scopes
            .iter()
            .map(|s| s.suggestion.confidence)
            .sum::<f64>()
            / similar_scopes.len() as f64;

        (pattern_confidence + avg_similar_confidence) / 2.0
    }

    /// Predict success metrics
    fn predict_success_metrics(
        &self,
        pattern: &HistoricalPattern,
        similar_scopes: &[ScopeCreationRecord],
    ) -> ScopeSuccessMetrics {
        let avg_satisfaction = similar_scopes
            .iter()
            .map(|s| s.success_metrics.satisfaction_score)
            .sum::<f64>()
            / similar_scopes.len() as f64;

        let avg_usage = similar_scopes
            .iter()
            .map(|s| s.success_metrics.usage_frequency)
            .sum::<f64>()
            / similar_scopes.len() as f64;

        let avg_performance = similar_scopes
            .iter()
            .map(|s| s.success_metrics.performance_impact)
            .sum::<f64>()
            / similar_scopes.len() as f64;

        let avg_maintenance = similar_scopes
            .iter()
            .map(|s| s.success_metrics.maintenance_burden)
            .sum::<f64>()
            / similar_scopes.len() as f64;

        ScopeSuccessMetrics {
            still_in_use: true,
            usage_frequency: avg_usage,
            satisfaction_score: avg_satisfaction,
            performance_impact: avg_performance,
            maintenance_burden: avg_maintenance,
            last_accessed: Some(Utc::now()),
            modification_count: 0,
        }
    }

    /// Generate prediction reasoning
    fn generate_prediction_reasoning(
        &self,
        pattern: &HistoricalPattern,
        similar_scopes: &[ScopeCreationRecord],
    ) -> String {
        format!(
            "Based on historical pattern '{}' with {:.1}% success rate and {} similar successful scopes",
            pattern.name,
            pattern.success_rate * 100.0,
            similar_scopes.len()
        )
    }

    /// Update patterns
    async fn update_patterns(&self) -> RhemaResult<()> {
        let new_patterns = self.analyze_patterns().await?;
        let mut patterns = self.patterns.lock().unwrap();
        *patterns = new_patterns;
        Ok(())
    }

    /// Calculate scope type distribution
    fn calculate_scope_type_distribution(
        &self,
        records: &[ScopeCreationRecord],
    ) -> HashMap<ScopeType, f64> {
        let mut distribution = HashMap::new();
        let total = records.len() as f64;

        for record in records {
            *distribution
                .entry(record.suggestion.scope_type.clone())
                .or_insert(0.0) += 1.0;
        }

        for count in distribution.values_mut() {
            *count /= total;
        }

        distribution
    }

    /// Calculate pattern distribution
    fn calculate_pattern_distribution(
        &self,
        patterns: &[HistoricalPattern],
    ) -> HashMap<String, f64> {
        let mut distribution = HashMap::new();
        let total = patterns.len() as f64;

        for pattern in patterns {
            *distribution.entry(pattern.name.clone()).or_insert(0.0) += 1.0;
        }

        for count in distribution.values_mut() {
            *count /= total;
        }

        distribution
    }
}

/// Historical statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalStatistics {
    /// Total number of records
    pub total_records: usize,
    /// Total number of patterns
    pub total_patterns: usize,
    /// Overall success rate
    pub success_rate: f64,
    /// Average confidence
    pub avg_confidence: f64,
    /// Scope type distribution
    pub scope_type_distribution: HashMap<ScopeType, f64>,
    /// Pattern distribution
    pub pattern_distribution: HashMap<String, f64>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_historical_analysis_engine_creation() {
        let config = HistoricalAnalysisConfig {
            min_pattern_confidence: 0.7,
            min_pattern_frequency: 0.1,
            max_data_age_days: 365,
            pattern_update_interval_hours: 24,
            enable_predictions: true,
            enable_learning: true,
            analysis_depth: AnalysisDepth::Standard,
        };

        let engine = HistoricalAnalysisEngine::new(config);
        let stats = engine.get_statistics().await.unwrap();

        assert_eq!(stats.total_records, 0);
        assert_eq!(stats.total_patterns, 0);
    }

    #[tokio::test]
    async fn test_add_record() {
        let config = HistoricalAnalysisConfig {
            min_pattern_confidence: 0.7,
            min_pattern_frequency: 0.1,
            max_data_age_days: 365,
            pattern_update_interval_hours: 24,
            enable_predictions: true,
            enable_learning: true,
            analysis_depth: AnalysisDepth::Standard,
        };

        let engine = HistoricalAnalysisEngine::new(config);

        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let test_repo_path = temp_dir.path().join("test-repo");
        std::fs::create_dir(&test_repo_path).unwrap();

        let record = ScopeCreationRecord {
            id: Uuid::new_v4().to_string(),
            repository_path: test_repo_path,
            suggestion: ScopeSuggestion {
                path: PathBuf::from("src"),
                name: "source".to_string(),
                scope_type: ScopeType::Library,
                confidence: 0.8,
                reasoning: "Source code".to_string(),
                files: Vec::new(),
                dependencies: Vec::new(),
                metadata: HashMap::new(),
            },
            created_at: Utc::now(),
            created_by: "test-user".to_string(),
            context: ScopeCreationContext {
                repository_size: RepositorySize::Small,
                technology_stack: vec![TechnologyStackPattern::RustEcosystem],
                project_structure: vec![ProjectStructurePattern::StandardSourceLayout],
                team_size: TeamSize::Solo,
                development_phase: DevelopmentPhase::Active,
                custom_data: HashMap::new(),
            },
            success_metrics: ScopeSuccessMetrics {
                still_in_use: true,
                usage_frequency: 0.9,
                satisfaction_score: 0.8,
                performance_impact: 0.7,
                maintenance_burden: 0.2,
                last_accessed: Some(Utc::now()),
                modification_count: 5,
            },
            patterns_detected: vec![ArchitecturalPattern::CleanArchitecture],
        };

        engine.add_record(record).await.unwrap();

        let stats = engine.get_statistics().await.unwrap();
        assert_eq!(stats.total_records, 1);
    }

    #[test]
    fn test_string_similarity() {
        let config = HistoricalAnalysisConfig {
            min_pattern_confidence: 0.7,
            min_pattern_frequency: 0.1,
            max_data_age_days: 365,
            pattern_update_interval_hours: 24,
            enable_predictions: true,
            enable_learning: true,
            analysis_depth: AnalysisDepth::Standard,
        };

        let engine = HistoricalAnalysisEngine::new(config);

        let similarity = engine.calculate_string_similarity("hello", "helo");
        assert!(similarity > 0.7); // "helo" is 1 edit away from "hello" (5 chars), so similarity is 0.8

        let similarity = engine.calculate_string_similarity("hello", "world");
        assert!(similarity < 0.5); // "world" is very different from "hello"
    }
}
