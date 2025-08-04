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

pub mod coordination;
pub mod lock_context;
pub mod lock_context_integration;
pub mod constraint_system;
pub mod task_scoring;
pub mod real_time_coordination;
pub mod conflict_prevention;
pub mod advanced_conflict_prevention;
pub mod state;
pub mod patterns;

// Re-export main components
pub use coordination::{SyncCoordinator, SyncStatus, SyncError};
pub use lock_context::{LockFileContextProvider, LockFileAIContext};
pub use lock_context_integration::{LockFileAIIntegration};
pub use constraint_system::{ConstraintSystem, Constraint, ConstraintViolation};
pub use task_scoring::{TaskScoringSystem, TaskScore, TaskScoringFactors};
pub use real_time_coordination::{RealTimeCoordinationSystem, AgentMessage, AgentStatus};
pub use conflict_prevention::{ConflictPreventionSystem, ConflictType, ResolutionStrategy};
pub use advanced_conflict_prevention::{
    AdvancedConflictPreventionSystem, AdvancedConflictPreventionConfig, AdvancedResolutionStrategy,
    ConflictPredictionModel, ConsensusConfig, CoordinationSession, AdvancedConflictStats,
    ConflictPrediction, PreventiveAction,
};
pub use state::{AgentState, AgentManager, StateTransition};
pub use patterns::{CoordinationPattern, PatternRegistry, PatternExecutor, PatternContext, PatternResult, PatternError}; 