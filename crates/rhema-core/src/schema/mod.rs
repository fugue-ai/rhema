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

pub mod core;
pub mod knowledge;
pub mod lock;
pub mod prompts;
pub mod templates;
pub mod validation;

// Re-export core types
pub use core::*;

// Re-export knowledge types (excluding conflicts)
pub use knowledge::{
    ConventionEntry, Conventions, DecisionEntry, DecisionStatus, Decisions, EnforcementLevel,
    Knowledge, KnowledgeEntry, PatternEntry, PatternMaturity, PatternTemplate, PatternTemplates,
    PatternUsage, PatternUsageStats, Patterns, Priority, ReviewStatus, SnippetType,
    TemplateContent, TemplateMaturity, TemplateMetadata, TestTemplate, TestType, TodoEntry,
    TodoStatus, Todos, VariableType,
};

// Re-export lock types
pub use lock::*;

// Re-export prompts types (excluding conflicts)
pub use prompts::{
    AdvancedVariable, ChainMetadata, ChainStep, ChainUsageStats, CompositionBlock,
    CompositionBlockType, ContextCacheConfig, ContextInjectionMethod, ContextLearningConfig,
    ContextOptimizationConfig, ContextQualityMetrics, ContextRule, FeedbackEntry,
    LearningAlgorithm, OptimizationAlgorithm, PromptChain, PromptInjectionMethod, PromptPattern,
    PromptVersion, Prompts, RetryConfig, TemplatePerformanceMetrics, TemplateValidationRule,
    UsageAnalytics, ValidationRuleType, ValidationSeverity, VariableConstraints,
    VariableType as PromptVariableType, VariableValidation, VersionEntry, Workflows,
};

// Re-export templates types (excluding conflicts)
pub use templates::{
    ExportMetadata, SharedTemplate, TemplateAccessControl, TemplateComplexity, TemplateExport,
    TemplateLibrary, TemplateMetadata as TemplateModuleMetadata, TemplateUsageStats,
};

// Re-export validation types
pub use validation::*;

// Re-export constants
pub use core::CURRENT_SCHEMA_VERSION;
