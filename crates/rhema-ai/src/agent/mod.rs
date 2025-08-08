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

pub mod advanced_conflict_prevention;
pub mod conflict_analysis;
pub mod conflict_prevention;
pub mod constraint_system;
pub mod coordination;
pub mod lock_context;
pub mod lock_context_integration;
pub mod ml_conflict_prediction;
pub mod patterns;
pub mod real_time_coordination;
pub mod state;
pub mod task_scoring;

// Re-export main components
pub use advanced_conflict_prevention::{
    AdvancedConflictPreventionConfig, AdvancedConflictPreventionSystem, AdvancedConflictStats,
    AdvancedResolutionStrategy, ConflictPrediction, ConflictPredictionModel, ConsensusConfig,
    CoordinationSession, PreventiveAction,
};
pub use conflict_analysis::{
    ConflictAnalysisConfig, ConflictAnalysisReport, ConflictAnalysisSystem, ConflictStatistics,
    LearningInsights, PerformanceMetrics, PredictionStatistics, Recommendation, ReportData,
    ReportType, ResolutionStatistics, TrendAnalysis,
};
pub use conflict_prevention::{ConflictPreventionSystem, ConflictType, ResolutionStrategy};
pub use constraint_system::{Constraint, ConstraintSystem, ConstraintViolation};
pub use coordination::{SyncCoordinator, SyncError, SyncStatus};
pub use lock_context::{LockFileAIContext, LockFileContextProvider};
pub use lock_context_integration::LockFileAIIntegration;
pub use ml_conflict_prediction::{
    ConflictLearningSystem, ConflictPredictionResult, LearningMetrics, MLConflictPredictionConfig,
    MLConflictPredictionModel, MLConflictPredictionStats, MLConflictPredictionSystem,
    PreventionAction as MLPreventionAction,
};
pub use patterns::{
    CoordinationPattern, PatternContext, PatternError, PatternExecutor, PatternRegistry,
    PatternResult,
};
pub use real_time_coordination::{AgentMessage, AgentStatus, RealTimeCoordinationSystem};
pub use state::{AgentManager, AgentState, StateTransition};
pub use task_scoring::{TaskScore, TaskScoringFactors, TaskScoringSystem};
