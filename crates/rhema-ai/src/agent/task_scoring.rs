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
use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Task priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
    Emergency = 4,
}

/// Task status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
    Failed,
}

/// Task complexity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskComplexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Task type categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    BugFix,
    Feature,
    Refactor,
    Documentation,
    Testing,
    Performance,
    Security,
    Maintenance,
    Research,
    Custom(String),
}

/// Task scoring factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskScoringFactors {
    /// Business value (0.0-1.0)
    pub business_value: f64,
    /// Technical debt impact (0.0-1.0)
    pub technical_debt_impact: f64,
    /// User impact (0.0-1.0)
    pub user_impact: f64,
    /// Dependencies count (negative impact)
    pub dependencies_count: usize,
    /// Estimated effort in hours
    pub estimated_effort_hours: f64,
    /// Risk level (0.0-1.0)
    pub risk_level: f64,
    /// Urgency (0.0-1.0)
    pub urgency: f64,
    /// Team capacity impact (0.0-1.0)
    pub team_capacity_impact: f64,
    /// Learning value (0.0-1.0)
    pub learning_value: f64,
    /// Strategic alignment (0.0-1.0)
    pub strategic_alignment: f64,
}

/// Task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier
    pub id: String,
    /// Task title
    pub title: String,
    /// Task description
    pub description: String,
    /// Task type
    pub task_type: TaskType,
    /// Task priority
    pub priority: TaskPriority,
    /// Task status
    pub status: TaskStatus,
    /// Task complexity
    pub complexity: TaskComplexity,
    /// Task scoring factors
    pub scoring_factors: TaskScoringFactors,
    /// Scope this task belongs to
    pub scope: String,
    /// Assigned agent/team member
    pub assigned_to: Option<String>,
    /// Dependencies (task IDs)
    pub dependencies: Vec<String>,
    /// Blocking tasks (task IDs)
    pub blocking: Vec<String>,
    /// Related tasks (task IDs)
    pub related_tasks: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Due date
    pub due_date: Option<DateTime<Utc>>,
    /// Estimated completion time
    pub estimated_completion: Option<DateTime<Utc>>,
    /// Actual completion time
    pub completed_at: Option<DateTime<Utc>>,
    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Task score calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskScore {
    /// Task ID
    pub task_id: String,
    /// Overall score (0.0-1.0)
    pub overall_score: f64,
    /// Priority score (0.0-1.0)
    pub priority_score: f64,
    /// Business value score (0.0-1.0)
    pub business_value_score: f64,
    /// Technical debt score (0.0-1.0)
    pub technical_debt_score: f64,
    /// User impact score (0.0-1.0)
    pub user_impact_score: f64,
    /// Dependency score (0.0-1.0)
    pub dependency_score: f64,
    /// Effort efficiency score (0.0-1.0)
    pub effort_efficiency_score: f64,
    /// Risk-adjusted score (0.0-1.0)
    pub risk_adjusted_score: f64,
    /// Urgency score (0.0-1.0)
    pub urgency_score: f64,
    /// Team capacity score (0.0-1.0)
    pub team_capacity_score: f64,
    /// Learning value score (0.0-1.0)
    pub learning_value_score: f64,
    /// Strategic alignment score (0.0-1.0)
    pub strategic_alignment_score: f64,
    /// Score calculation timestamp
    pub calculated_at: DateTime<Utc>,
    /// Score explanation
    pub explanation: String,
}

/// Task prioritization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPrioritization {
    /// Prioritized task list
    pub prioritized_tasks: Vec<TaskScore>,
    /// Prioritization strategy used
    pub strategy: PrioritizationStrategy,
    /// Prioritization statistics
    pub stats: PrioritizationStats,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Prioritization strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrioritizationStrategy {
    /// Weighted scoring approach
    WeightedScoring,
    /// Business value first
    BusinessValueFirst,
    /// Technical debt reduction
    TechnicalDebtFirst,
    /// User impact focused
    UserImpactFirst,
    /// Risk-adjusted return
    RiskAdjustedReturn,
    /// Effort efficiency
    EffortEfficiency,
    /// Strategic alignment
    StrategicAlignment,
    /// Custom strategy
    Custom(String),
}

/// Prioritization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizationStats {
    /// Total tasks processed
    pub total_tasks: usize,
    /// Tasks prioritized
    pub prioritized_tasks: usize,
    /// Average score
    pub average_score: f64,
    /// Score distribution
    pub score_distribution: HashMap<String, usize>,
    /// Prioritization time in milliseconds
    pub prioritization_time_ms: u64,
}

/// Scoring weights configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    /// Business value weight
    pub business_value_weight: f64,
    /// Technical debt weight
    pub technical_debt_weight: f64,
    /// User impact weight
    pub user_impact_weight: f64,
    /// Dependency weight
    pub dependency_weight: f64,
    /// Effort efficiency weight
    pub effort_efficiency_weight: f64,
    /// Risk weight
    pub risk_weight: f64,
    /// Urgency weight
    pub urgency_weight: f64,
    /// Team capacity weight
    pub team_capacity_weight: f64,
    /// Learning value weight
    pub learning_value_weight: f64,
    /// Strategic alignment weight
    pub strategic_alignment_weight: f64,
}

/// Task scoring errors
#[derive(Debug, Error)]
pub enum TaskScoringError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Invalid scoring factors: {0}")]
    InvalidScoringFactors(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Prioritization failed: {0}")]
    PrioritizationFailed(String),

    #[error("Invalid weights configuration: {0}")]
    InvalidWeights(String),
}

/// Task scoring and prioritization system
pub struct TaskScoringSystem {
    /// Tasks in the system
    tasks: HashMap<String, Task>,
    /// Task scores cache
    scores_cache: HashMap<String, TaskScore>,
    /// Scoring weights
    weights: ScoringWeights,
    /// Prioritization history
    prioritization_history: Vec<TaskPrioritization>,
    /// Task dependencies graph
    dependencies_graph: HashMap<String, Vec<String>>,
}

impl TaskScoringSystem {
    /// Create a new task scoring system
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            scores_cache: HashMap::new(),
            weights: ScoringWeights::default(),
            prioritization_history: Vec::new(),
            dependencies_graph: HashMap::new(),
        }
    }

    /// Create a new task scoring system with custom weights
    pub fn with_weights(weights: ScoringWeights) -> Self {
        Self {
            tasks: HashMap::new(),
            scores_cache: HashMap::new(),
            weights,
            prioritization_history: Vec::new(),
            dependencies_graph: HashMap::new(),
        }
    }

    /// Add a task to the system
    pub fn add_task(&mut self, task: Task) -> RhemaResult<()> {
        // Validate task
        self.validate_task(&task)?;

        // Check for circular dependencies
        if self.has_circular_dependency(&task.id, &task.dependencies) {
            return Err(TaskScoringError::CircularDependency(task.id.clone()).into());
        }

        // Add task
        self.tasks.insert(task.id.clone(), task.clone());

        // Update dependencies graph
        self.dependencies_graph
            .insert(task.id.clone(), task.dependencies.clone());

        // Clear score cache for this task
        self.scores_cache.remove(&task.id);

        Ok(())
    }

    /// Remove a task from the system
    pub fn remove_task(&mut self, task_id: &str) -> RhemaResult<()> {
        if !self.tasks.contains_key(task_id) {
            return Err(TaskScoringError::TaskNotFound(task_id.to_string()).into());
        }

        // Check if task is depended upon by others
        for (_, deps) in &self.dependencies_graph {
            if deps.contains(&task_id.to_string()) {
                return Err(TaskScoringError::PrioritizationFailed(
                    "Cannot remove task that is depended upon".to_string(),
                )
                .into());
            }
        }

        self.tasks.remove(task_id);
        self.scores_cache.remove(task_id);
        self.dependencies_graph.remove(task_id);

        Ok(())
    }

    /// Get a task by ID
    pub fn get_task(&self, task_id: &str) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    /// Get all tasks for a scope
    pub fn get_scope_tasks(&self, scope: &str) -> Vec<&Task> {
        self.tasks.values().filter(|t| t.scope == scope).collect()
    }

    /// Calculate score for a task
    pub fn calculate_task_score(&mut self, task_id: &str) -> RhemaResult<TaskScore> {
        let task = self
            .tasks
            .get(task_id)
            .ok_or_else(|| TaskScoringError::TaskNotFound(task_id.to_string()))?;

        // Check cache first
        if let Some(cached_score) = self.scores_cache.get(task_id) {
            return Ok(cached_score.clone());
        }

        let factors = &task.scoring_factors;

        // Calculate individual scores
        let business_value_score = factors.business_value;
        let technical_debt_score = factors.technical_debt_impact;
        let user_impact_score = factors.user_impact;
        let dependency_score = self.calculate_dependency_score(task);
        let effort_efficiency_score = self.calculate_effort_efficiency_score(factors);
        let risk_adjusted_score = self.calculate_risk_adjusted_score(factors);
        let urgency_score = factors.urgency;
        let team_capacity_score = factors.team_capacity_impact;
        let learning_value_score = factors.learning_value;
        let strategic_alignment_score = factors.strategic_alignment;

        // Calculate priority score
        let priority_score = match task.priority {
            TaskPriority::Low => 0.2,
            TaskPriority::Normal => 0.4,
            TaskPriority::High => 0.7,
            TaskPriority::Critical => 0.9,
            TaskPriority::Emergency => 1.0,
        };

        // Calculate overall score using weighted average
        let overall_score = (business_value_score * self.weights.business_value_weight
            + technical_debt_score * self.weights.technical_debt_weight
            + user_impact_score * self.weights.user_impact_weight
            + dependency_score * self.weights.dependency_weight
            + effort_efficiency_score * self.weights.effort_efficiency_weight
            + risk_adjusted_score * self.weights.risk_weight
            + urgency_score * self.weights.urgency_weight
            + team_capacity_score * self.weights.team_capacity_weight
            + learning_value_score * self.weights.learning_value_weight
            + strategic_alignment_score * self.weights.strategic_alignment_weight)
            / self.get_total_weight();

        let score = TaskScore {
            task_id: task_id.to_string(),
            overall_score,
            priority_score,
            business_value_score,
            technical_debt_score,
            user_impact_score,
            dependency_score,
            effort_efficiency_score,
            risk_adjusted_score,
            urgency_score,
            team_capacity_score,
            learning_value_score,
            strategic_alignment_score,
            calculated_at: Utc::now(),
            explanation: self.generate_score_explanation(task, &overall_score),
        };

        // Cache the score
        self.scores_cache.insert(task_id.to_string(), score.clone());

        Ok(score)
    }

    /// Prioritize tasks for a scope
    pub fn prioritize_tasks(
        &mut self,
        scope: &str,
        strategy: PrioritizationStrategy,
    ) -> RhemaResult<TaskPrioritization> {
        let start_time = std::time::Instant::now();

        let scope_tasks: Vec<_> = self
            .get_scope_tasks(scope)
            .iter()
            .map(|t| t.id.clone())
            .collect();
        let mut task_scores = Vec::new();

        // Calculate scores for all tasks
        for task_id in scope_tasks {
            let score = self.calculate_task_score(&task_id)?;
            task_scores.push(score);
        }

        // Apply prioritization strategy
        let prioritized_tasks = match strategy {
            PrioritizationStrategy::WeightedScoring => {
                self.prioritize_by_weighted_scoring(task_scores)
            }
            PrioritizationStrategy::BusinessValueFirst => {
                self.prioritize_by_business_value(task_scores)
            }
            PrioritizationStrategy::TechnicalDebtFirst => {
                self.prioritize_by_technical_debt(task_scores)
            }
            PrioritizationStrategy::UserImpactFirst => self.prioritize_by_user_impact(task_scores),
            PrioritizationStrategy::RiskAdjustedReturn => {
                self.prioritize_by_risk_adjusted_return(task_scores)
            }
            PrioritizationStrategy::EffortEfficiency => {
                self.prioritize_by_effort_efficiency(task_scores)
            }
            PrioritizationStrategy::StrategicAlignment => {
                self.prioritize_by_strategic_alignment(task_scores)
            }
            PrioritizationStrategy::Custom(_) => self.prioritize_by_weighted_scoring(task_scores),
        };

        let prioritization_time = start_time.elapsed().as_millis() as u64;

        let stats = self.calculate_prioritization_stats(&prioritized_tasks, prioritization_time);
        let recommendations = self.generate_prioritization_recommendations(&prioritized_tasks);

        let result = TaskPrioritization {
            prioritized_tasks,
            strategy,
            stats,
            recommendations,
        };

        // Store in history
        self.prioritization_history.push(result.clone());

        Ok(result)
    }

    /// Calculate dependency score
    fn calculate_dependency_score(&self, task: &Task) -> f64 {
        if task.dependencies.is_empty() {
            return 1.0; // No dependencies = high score
        }

        // Calculate based on dependency count and complexity
        let dependency_count = task.dependencies.len() as f64;
        let max_dependencies = 10.0; // Assume 10 is max reasonable dependencies

        // Score decreases with more dependencies
        (max_dependencies - dependency_count.min(max_dependencies)) / max_dependencies
    }

    /// Calculate effort efficiency score
    fn calculate_effort_efficiency_score(&self, factors: &TaskScoringFactors) -> f64 {
        if factors.estimated_effort_hours <= 0.0 {
            return 0.5; // Neutral score for unknown effort
        }

        // Higher score for tasks with high value and low effort
        let value_per_hour =
            (factors.business_value + factors.user_impact) / factors.estimated_effort_hours;
        let max_value_per_hour = 2.0; // Assume 2.0 is maximum reasonable value per hour

        (value_per_hour / max_value_per_hour).min(1.0)
    }

    /// Calculate risk-adjusted score
    fn calculate_risk_adjusted_score(&self, factors: &TaskScoringFactors) -> f64 {
        // Risk-adjusted score considers both value and risk
        let base_value = factors.business_value + factors.user_impact;
        let risk_adjustment = 1.0 - factors.risk_level;

        base_value * risk_adjustment
    }

    /// Get total weight for normalization
    fn get_total_weight(&self) -> f64 {
        self.weights.business_value_weight
            + self.weights.technical_debt_weight
            + self.weights.user_impact_weight
            + self.weights.dependency_weight
            + self.weights.effort_efficiency_weight
            + self.weights.risk_weight
            + self.weights.urgency_weight
            + self.weights.team_capacity_weight
            + self.weights.learning_value_weight
            + self.weights.strategic_alignment_weight
    }

    /// Generate score explanation
    fn generate_score_explanation(&self, task: &Task, overall_score: &f64) -> String {
        let factors = &task.scoring_factors;
        let mut explanation = format!(
            "Task '{}' scored {:.2} overall. ",
            task.title, overall_score
        );

        if factors.business_value > 0.7 {
            explanation.push_str("High business value. ");
        }
        if factors.user_impact > 0.7 {
            explanation.push_str("High user impact. ");
        }
        if factors.technical_debt_impact > 0.7 {
            explanation.push_str("High technical debt impact. ");
        }
        if factors.urgency > 0.7 {
            explanation.push_str("High urgency. ");
        }
        if factors.risk_level > 0.7 {
            explanation.push_str("High risk. ");
        }

        if task.dependencies.len() > 5 {
            explanation.push_str("Many dependencies may slow progress. ");
        }

        explanation
    }

    /// Prioritize by weighted scoring
    fn prioritize_by_weighted_scoring(&self, mut task_scores: Vec<TaskScore>) -> Vec<TaskScore> {
        task_scores.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());
        task_scores
    }

    /// Prioritize by business value
    fn prioritize_by_business_value(&self, mut task_scores: Vec<TaskScore>) -> Vec<TaskScore> {
        task_scores.sort_by(|a, b| {
            b.business_value_score
                .partial_cmp(&a.business_value_score)
                .unwrap()
        });
        task_scores
    }

    /// Prioritize by technical debt
    fn prioritize_by_technical_debt(&self, mut task_scores: Vec<TaskScore>) -> Vec<TaskScore> {
        task_scores.sort_by(|a, b| {
            b.technical_debt_score
                .partial_cmp(&a.technical_debt_score)
                .unwrap()
        });
        task_scores
    }

    /// Prioritize by user impact
    fn prioritize_by_user_impact(&self, mut task_scores: Vec<TaskScore>) -> Vec<TaskScore> {
        task_scores.sort_by(|a, b| {
            b.user_impact_score
                .partial_cmp(&a.user_impact_score)
                .unwrap()
        });
        task_scores
    }

    /// Prioritize by risk-adjusted return
    fn prioritize_by_risk_adjusted_return(
        &self,
        mut task_scores: Vec<TaskScore>,
    ) -> Vec<TaskScore> {
        task_scores.sort_by(|a, b| {
            b.risk_adjusted_score
                .partial_cmp(&a.risk_adjusted_score)
                .unwrap()
        });
        task_scores
    }

    /// Prioritize by effort efficiency
    fn prioritize_by_effort_efficiency(&self, mut task_scores: Vec<TaskScore>) -> Vec<TaskScore> {
        task_scores.sort_by(|a, b| {
            b.effort_efficiency_score
                .partial_cmp(&a.effort_efficiency_score)
                .unwrap()
        });
        task_scores
    }

    /// Prioritize by strategic alignment
    fn prioritize_by_strategic_alignment(&self, mut task_scores: Vec<TaskScore>) -> Vec<TaskScore> {
        task_scores.sort_by(|a, b| {
            b.strategic_alignment_score
                .partial_cmp(&a.strategic_alignment_score)
                .unwrap()
        });
        task_scores
    }

    /// Calculate prioritization statistics
    fn calculate_prioritization_stats(
        &self,
        tasks: &[TaskScore],
        prioritization_time: u64,
    ) -> PrioritizationStats {
        let total_tasks = tasks.len();
        let average_score = if total_tasks > 0 {
            tasks.iter().map(|t| t.overall_score).sum::<f64>() / total_tasks as f64
        } else {
            0.0
        };

        let mut score_distribution = HashMap::new();
        for task in tasks {
            let score_range = if task.overall_score >= 0.8 {
                "High (0.8-1.0)"
            } else if task.overall_score >= 0.6 {
                "Medium-High (0.6-0.8)"
            } else if task.overall_score >= 0.4 {
                "Medium (0.4-0.6)"
            } else if task.overall_score >= 0.2 {
                "Low-Medium (0.2-0.4)"
            } else {
                "Low (0.0-0.2)"
            };
            *score_distribution
                .entry(score_range.to_string())
                .or_insert(0) += 1;
        }

        PrioritizationStats {
            total_tasks,
            prioritized_tasks: total_tasks,
            average_score,
            score_distribution,
            prioritization_time_ms: prioritization_time,
        }
    }

    /// Generate prioritization recommendations
    fn generate_prioritization_recommendations(&self, tasks: &[TaskScore]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if tasks.is_empty() {
            recommendations.push("No tasks to prioritize".to_string());
            return recommendations;
        }

        // Always provide a general recommendation
        recommendations.push(
            "Review task priorities regularly and adjust based on changing requirements"
                .to_string(),
        );

        // Analyze top tasks
        let top_tasks: Vec<_> = tasks.iter().take(3).collect();
        let avg_score = tasks.iter().map(|t| t.overall_score).sum::<f64>() / tasks.len() as f64;

        if avg_score < 0.5 {
            recommendations
                .push("Consider reviewing task scoring factors - average score is low".to_string());
        }

        if top_tasks.iter().any(|t| t.business_value_score > 0.8) {
            recommendations
                .push("High business value tasks identified - consider fast-tracking".to_string());
        }

        if top_tasks.iter().any(|t| t.technical_debt_score > 0.8) {
            recommendations.push(
                "High technical debt impact tasks - consider addressing technical debt".to_string(),
            );
        }

        if top_tasks.iter().any(|t| t.risk_adjusted_score < 0.3) {
            recommendations.push(
                "High-risk tasks identified - consider risk mitigation strategies".to_string(),
            );
        }

        recommendations
    }

    /// Validate task
    fn validate_task(&self, task: &Task) -> RhemaResult<()> {
        if task.id.is_empty() {
            return Err(TaskScoringError::InvalidScoringFactors(
                "Task ID cannot be empty".to_string(),
            )
            .into());
        }

        if task.title.is_empty() {
            return Err(TaskScoringError::InvalidScoringFactors(
                "Task title cannot be empty".to_string(),
            )
            .into());
        }

        if task.scope.is_empty() {
            return Err(TaskScoringError::InvalidScoringFactors(
                "Task scope cannot be empty".to_string(),
            )
            .into());
        }

        // Validate scoring factors
        let factors = &task.scoring_factors;
        if !(0.0..=1.0).contains(&factors.business_value)
            || !(0.0..=1.0).contains(&factors.technical_debt_impact)
            || !(0.0..=1.0).contains(&factors.user_impact)
            || !(0.0..=1.0).contains(&factors.risk_level)
            || !(0.0..=1.0).contains(&factors.urgency)
            || !(0.0..=1.0).contains(&factors.team_capacity_impact)
            || !(0.0..=1.0).contains(&factors.learning_value)
            || !(0.0..=1.0).contains(&factors.strategic_alignment)
        {
            return Err(TaskScoringError::InvalidScoringFactors(
                "Scoring factors must be between 0.0 and 1.0".to_string(),
            )
            .into());
        }

        if factors.estimated_effort_hours < 0.0 {
            return Err(TaskScoringError::InvalidScoringFactors(
                "Estimated effort cannot be negative".to_string(),
            )
            .into());
        }

        Ok(())
    }

    /// Check for circular dependencies
    fn has_circular_dependency(&self, task_id: &str, dependencies: &[String]) -> bool {
        // Simplified circular dependency check
        // In a real implementation, this would traverse the dependency graph
        dependencies.contains(&task_id.to_string())
    }

    /// Get prioritization history
    pub fn get_prioritization_history(&self) -> &[TaskPrioritization] {
        &self.prioritization_history
    }

    /// Clear score cache
    pub fn clear_cache(&mut self) {
        self.scores_cache.clear();
    }
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            business_value_weight: 0.25,
            technical_debt_weight: 0.15,
            user_impact_weight: 0.20,
            dependency_weight: 0.10,
            effort_efficiency_weight: 0.10,
            risk_weight: 0.05,
            urgency_weight: 0.10,
            team_capacity_weight: 0.02,
            learning_value_weight: 0.02,
            strategic_alignment_weight: 0.01,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_scoring_system_creation() {
        let system = TaskScoringSystem::new();
        assert_eq!(system.tasks.len(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut system = TaskScoringSystem::new();

        let task = Task {
            id: "test-task".to_string(),
            title: "Test Task".to_string(),
            description: "A test task".to_string(),
            task_type: TaskType::Feature,
            priority: TaskPriority::High,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Moderate,
            scoring_factors: TaskScoringFactors {
                business_value: 0.8,
                technical_debt_impact: 0.3,
                user_impact: 0.7,
                dependencies_count: 2,
                estimated_effort_hours: 8.0,
                risk_level: 0.2,
                urgency: 0.6,
                team_capacity_impact: 0.4,
                learning_value: 0.5,
                strategic_alignment: 0.6,
            },
            scope: "test-scope".to_string(),
            assigned_to: None,
            dependencies: vec![],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        };

        assert!(system.add_task(task).is_ok());
        assert_eq!(system.tasks.len(), 1);
    }

    #[tokio::test]
    async fn test_calculate_task_score() {
        let mut system = TaskScoringSystem::new();

        let task = Task {
            id: "test-task".to_string(),
            title: "Test Task".to_string(),
            description: "A test task".to_string(),
            task_type: TaskType::Feature,
            priority: TaskPriority::High,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Moderate,
            scoring_factors: TaskScoringFactors {
                business_value: 0.8,
                technical_debt_impact: 0.3,
                user_impact: 0.7,
                dependencies_count: 2,
                estimated_effort_hours: 8.0,
                risk_level: 0.2,
                urgency: 0.6,
                team_capacity_impact: 0.4,
                learning_value: 0.5,
                strategic_alignment: 0.6,
            },
            scope: "test-scope".to_string(),
            assigned_to: None,
            dependencies: vec![],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        };

        system.add_task(task).unwrap();
        let score = system.calculate_task_score("test-task").unwrap();

        assert!(score.overall_score > 0.0);
        assert!(score.overall_score <= 1.0);
        assert_eq!(score.task_id, "test-task");
    }

    #[tokio::test]
    async fn test_prioritize_tasks() {
        let mut system = TaskScoringSystem::new();

        // Add multiple tasks
        for i in 0..3 {
            let task = Task {
                id: format!("task-{}", i),
                title: format!("Task {}", i),
                description: format!("Task description {}", i),
                task_type: TaskType::Feature,
                priority: TaskPriority::High,
                status: TaskStatus::Pending,
                complexity: TaskComplexity::Moderate,
                scoring_factors: TaskScoringFactors {
                    business_value: 0.8 - (i as f64 * 0.1),
                    technical_debt_impact: 0.3,
                    user_impact: 0.7,
                    dependencies_count: 2,
                    estimated_effort_hours: 8.0,
                    risk_level: 0.2,
                    urgency: 0.6,
                    team_capacity_impact: 0.4,
                    learning_value: 0.5,
                    strategic_alignment: 0.6,
                },
                scope: "test-scope".to_string(),
                assigned_to: None,
                dependencies: vec![],
                blocking: vec![],
                related_tasks: vec![],
                created_at: Utc::now(),
                modified_at: Utc::now(),
                due_date: None,
                estimated_completion: None,
                completed_at: None,
                metadata: HashMap::new(),
            };
            system.add_task(task).unwrap();
        }

        let prioritization = system
            .prioritize_tasks("test-scope", PrioritizationStrategy::WeightedScoring)
            .unwrap();

        assert_eq!(prioritization.prioritized_tasks.len(), 3);
        assert!(prioritization.stats.total_tasks == 3);
        // Recommendations should be generated for any prioritization
        assert!(
            !prioritization.recommendations.is_empty(),
            "Expected recommendations to be generated"
        );
    }
}
