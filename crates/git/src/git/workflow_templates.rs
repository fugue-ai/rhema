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

use crate::git::workflow::{
    WorkflowConfig, WorkflowType, BranchConventions, ContextRules, ReleaseManagement,
    PullRequestSettings, AutomationSettings, AdvancedWorkflowFeatures,
    ContextAwareWorkflowSettings, ContextAwareFeatureBranching, ContextAwareReleaseManagement,
    ContextAwareHotfixManagement, ContextAwarePrAnalysis, ContextAwareMergeStrategies,
    ContextMergeStrategy, ContextConflictResolution, ConflictResolutionType,
    ContextInheritanceRule, ContextInheritanceType, ContextBoundaryRule, ContextBoundaryType,
    ContextBoundaryEnforcement, ContextValidationRule, ContextValidationType,
    ReleaseValidationRule, ReleaseValidationType, ReleasePreparationStep, ReleasePreparationStepType,
    ReleaseCleanupStep, ReleaseCleanupStepType, HotfixValidationRule, HotfixValidationType,
    HotfixMergeStrategy, HotfixMergeStrategyType, PrAnalysisRule, PrAnalysisType,
    PrValidationRule, PrValidationType, PrAutomationRule, PrAutomationType,
    PrAutomationTrigger, PrAutomationAction, MergeValidationRule, MergeValidationType,
    ConflictResolutionRule, VersioningStrategy, IsolationRules
};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workflow template types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowTemplateType {
    /// Standard GitFlow workflow
    GitFlow,
    /// GitHub Flow workflow
    GitHubFlow,
    /// GitLab Flow workflow
    GitLabFlow,
    /// Trunk-based development workflow
    TrunkBased,
    /// Feature branch workflow
    FeatureBranch,
    /// Release-focused workflow
    ReleaseFocused,
    /// Hotfix-focused workflow
    HotfixFocused,
    /// Enterprise workflow with strict controls
    Enterprise,
    /// Open source workflow
    OpenSource,
    /// Microservices workflow
    Microservices,
    /// Monorepo workflow
    Monorepo,
    /// Custom workflow template
    Custom(String),
}

/// Workflow template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Template type
    pub template_type: WorkflowTemplateType,
    
    /// Workflow configuration
    pub config: WorkflowConfig,
    
    /// Template metadata
    pub metadata: TemplateMetadata,
    
    /// Template validation rules
    pub validation_rules: Vec<TemplateValidationRule>,
    
    /// Template customization options
    pub customization_options: Vec<TemplateCustomizationOption>,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template version
    pub version: String,
    
    /// Template author
    pub author: String,
    
    /// Template tags
    pub tags: Vec<String>,
    
    /// Template complexity level
    pub complexity: ComplexityLevel,
    
    /// Template maturity level
    pub maturity: MaturityLevel,
    
    /// Template usage examples
    pub examples: Vec<String>,
    
    /// Template requirements
    pub requirements: Vec<String>,
}

/// Complexity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    Enterprise,
}

/// Maturity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaturityLevel {
    Experimental,
    Beta,
    Stable,
    Production,
}

/// Template validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateValidationRule {
    /// Rule name
    pub name: String,
    
    /// Rule description
    pub description: String,
    
    /// Validation condition
    pub condition: String,
    
    /// Error message
    pub error_message: String,
    
    /// Whether this rule is required
    pub required: bool,
}

/// Template customization option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateCustomizationOption {
    /// Option name
    pub name: String,
    
    /// Option description
    pub description: String,
    
    /// Option type
    pub option_type: CustomizationOptionType,
    
    /// Default value
    pub default_value: serde_json::Value,
    
    /// Available choices (for enum types)
    pub choices: Option<Vec<String>>,
    
    /// Whether this option is required
    pub required: bool,
}

/// Customization option types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomizationOptionType {
    String,
    Boolean,
    Number,
    Enum,
    Array,
    Object,
}

/// Workflow template manager
pub struct WorkflowTemplateManager;

impl WorkflowTemplateManager {
    /// Get all available workflow templates
    pub fn get_available_templates() -> Vec<WorkflowTemplate> {
        vec![
            Self::gitflow_template(),
            Self::github_flow_template(),
            Self::gitlab_flow_template(),
            Self::trunk_based_template(),
            Self::feature_branch_template(),
            Self::release_focused_template(),
            Self::hotfix_focused_template(),
            Self::enterprise_template(),
            Self::open_source_template(),
            Self::microservices_template(),
            Self::monorepo_template(),
        ]
    }

    /// Get template by type
    pub fn get_template(template_type: &WorkflowTemplateType) -> RhemaResult<WorkflowTemplate> {
        let templates = Self::get_available_templates();
        templates
            .into_iter()
            .find(|t| t.template_type == *template_type)
            .ok_or_else(|| RhemaError::ConfigError(format!("Template not found: {:?}", template_type)))
    }

    /// Validate template configuration
    pub fn validate_template(template: &WorkflowTemplate) -> RhemaResult<Vec<String>> {
        let mut errors = Vec::new();

        // Validate required fields
        if template.name.is_empty() {
            errors.push("Template name cannot be empty".to_string());
        }

        if template.description.is_empty() {
            errors.push("Template description cannot be empty".to_string());
        }

        // Validate configuration
        if let Err(config_errors) = Self::validate_workflow_config(&template.config) {
            errors.extend(config_errors);
        }

        // Validate custom rules
        for rule in &template.validation_rules {
            if rule.name.is_empty() {
                errors.push(format!("Validation rule name cannot be empty"));
            }
            if rule.condition.is_empty() {
                errors.push(format!("Validation rule condition cannot be empty: {}", rule.name));
            }
        }

        Ok(errors)
    }

    /// Apply template customization
    pub fn apply_customization(
        template: &WorkflowTemplate,
        customizations: &HashMap<String, serde_json::Value>,
    ) -> RhemaResult<WorkflowConfig> {
        let mut config = template.config.clone();

        for (key, value) in customizations {
            Self::apply_customization_to_config(&mut config, key, value)?;
        }

        Ok(config)
    }

    /// GitFlow template
    fn gitflow_template() -> WorkflowTemplate {
        WorkflowTemplate {
            name: "GitFlow".to_string(),
            description: "A robust Git workflow that provides a framework for managing larger projects with multiple parallel developments".to_string(),
            template_type: WorkflowTemplateType::GitFlow,
            config: WorkflowConfig {
                workflow_type: WorkflowType::GitFlow,
                branch_conventions: BranchConventions {
                    main_branch: "main".to_string(),
                    develop_branch: Some("develop".to_string()),
                    feature_prefix: "feature/".to_string(),
                    release_prefix: "release/".to_string(),
                    hotfix_prefix: "hotfix/".to_string(),
                    support_prefix: "support/".to_string(),
                },
                context_rules: ContextRules {
                    require_feature_validation: true,
                    require_release_validation: true,
                    require_hotfix_validation: true,
                    merge_strategies: HashMap::new(),
                    isolation_rules: IsolationRules {
                        isolate_feature: true,
                        isolate_release: true,
                        isolate_hotfix: true,
                        shared_files: vec!["README.md".to_string(), "LICENSE".to_string()],
                    },
                },
                release_management: ReleaseManagement {
                    versioning: VersioningStrategy::Semantic,
                    branch_preparation: crate::git::workflow::BranchPreparation {
                        prepare_context: true,
                        update_version: true,
                        generate_notes: true,
                        validate_readiness: true,
                    },
                    validation: crate::git::workflow::ReleaseValidation {
                        validate_context: true,
                        validate_dependencies: true,
                        validate_breaking_changes: true,
                        run_tests: true,
                    },
                    automation: crate::git::workflow::ReleaseAutomation {
                        auto_create_branch: true,
                        auto_version_bump: true,
                        auto_release_notes: true,
                        auto_deploy: false,
                    },
                },
                pull_request_settings: PullRequestSettings {
                    require_context_analysis: true,
                    require_impact_analysis: true,
                    require_dependency_review: true,
                    require_health_checks: true,
                    automated_checks: vec![
                        "context-validation".to_string(),
                        "dependency-check".to_string(),
                        "security-scan".to_string(),
                    ],
                },
                automation: AutomationSettings {
                    auto_context_updates: true,
                    auto_synchronization: true,
                    auto_notifications: true,
                    auto_backups: true,
                },
                advanced_features: AdvancedWorkflowFeatures {
                    context_aware_branching: true,
                    auto_context_sync: true,
                    context_conflict_resolution: true,
                    context_validation_workflows: true,
                    context_evolution_tracking: true,
                    context_analytics: true,
                    context_optimization: true,
                    context_backup_workflows: true,
                },
                context_aware: ContextAwareWorkflowSettings {
                    context_aware_feature_branching: ContextAwareFeatureBranching {
                        auto_isolate_context: true,
                        auto_sync_parent: true,
                        auto_validate_before_merge: true,
                        auto_resolve_conflicts: true,
                        inheritance_rules: vec![],
                        boundary_rules: vec![],
                        validation_rules: vec![],
                    },
                    context_aware_release_management: ContextAwareReleaseManagement {
                        auto_prepare_context: true,
                        auto_validate_release_context: true,
                        auto_generate_release_notes: true,
                        auto_update_version: true,
                        validation_rules: vec![],
                        preparation_steps: vec![],
                        cleanup_steps: vec![],
                    },
                    context_aware_hotfix_management: ContextAwareHotfixManagement {
                        auto_isolate_context: true,
                        auto_validate_context: true,
                        auto_merge_context: true,
                        validation_rules: vec![],
                        merge_strategies: vec![],
                    },
                    context_aware_pr_analysis: ContextAwarePrAnalysis {
                        auto_analyze_context_changes: true,
                        auto_detect_conflicts: true,
                        auto_generate_impact_report: true,
                        auto_suggest_improvements: true,
                        analysis_rules: vec![],
                        validation_rules: vec![],
                        automation_rules: vec![],
                    },
                    context_aware_merge_strategies: ContextAwareMergeStrategies {
                        feature_merge_strategy: ContextMergeStrategy {
                            name: "feature-merge".to_string(),
                            description: "Feature branch merge strategy".to_string(),
                            strategy_type: crate::git::workflow::ContextMergeStrategyType::SemiAuto,
                            conflict_resolution: ContextConflictResolution {
                                resolution_type: ConflictResolutionType::Merge,
                                auto_resolve_simple: true,
                                manual_resolution_required: false,
                                resolution_rules: vec![],
                            },
                            validation_rules: vec![],
                        },
                        release_merge_strategy: ContextMergeStrategy {
                            name: "release-merge".to_string(),
                            description: "Release branch merge strategy".to_string(),
                            strategy_type: crate::git::workflow::ContextMergeStrategyType::Auto,
                            conflict_resolution: ContextConflictResolution {
                                resolution_type: ConflictResolutionType::TakeSource,
                                auto_resolve_simple: true,
                                manual_resolution_required: false,
                                resolution_rules: vec![],
                            },
                            validation_rules: vec![],
                        },
                        hotfix_merge_strategy: ContextMergeStrategy {
                            name: "hotfix-merge".to_string(),
                            description: "Hotfix branch merge strategy".to_string(),
                            strategy_type: crate::git::workflow::ContextMergeStrategyType::Auto,
                            conflict_resolution: ContextConflictResolution {
                                resolution_type: ConflictResolutionType::TakeSource,
                                auto_resolve_simple: true,
                                manual_resolution_required: false,
                                resolution_rules: vec![],
                            },
                            validation_rules: vec![],
                        },
                        custom_strategies: vec![],
                    },
                },
                integrations: crate::git::workflow::WorkflowIntegrationSettings {
                    ci_cd: None,
                    issue_tracker: None,
                    chat: None,
                    monitoring: None,
                },
            },
            metadata: TemplateMetadata {
                version: "1.0.0".to_string(),
                author: "Rhema Team".to_string(),
                tags: vec!["gitflow".to_string(), "enterprise".to_string(), "robust".to_string()],
                complexity: ComplexityLevel::Moderate,
                maturity: MaturityLevel::Production,
                examples: vec![
                    "Large enterprise projects".to_string(),
                    "Projects with multiple parallel developments".to_string(),
                    "Projects requiring strict release management".to_string(),
                ],
                requirements: vec![
                    "Git repository".to_string(),
                    "Team coordination".to_string(),
                    "Release management process".to_string(),
                ],
            },
            validation_rules: vec![
                TemplateValidationRule {
                    name: "develop-branch-required".to_string(),
                    description: "GitFlow requires a develop branch".to_string(),
                    condition: "develop_branch != null".to_string(),
                    error_message: "GitFlow workflow requires a develop branch".to_string(),
                    required: true,
                },
            ],
            customization_options: vec![
                TemplateCustomizationOption {
                    name: "main_branch".to_string(),
                    description: "Name of the main branch".to_string(),
                    option_type: CustomizationOptionType::String,
                    default_value: serde_json::Value::String("main".to_string()),
                    choices: None,
                    required: true,
                },
                TemplateCustomizationOption {
                    name: "develop_branch".to_string(),
                    description: "Name of the develop branch".to_string(),
                    option_type: CustomizationOptionType::String,
                    default_value: serde_json::Value::String("develop".to_string()),
                    choices: None,
                    required: true,
                },
            ],
        }
    }

    /// GitHub Flow template
    fn github_flow_template() -> WorkflowTemplate {
        WorkflowTemplate {
            name: "GitHub Flow".to_string(),
            description: "A lightweight, branch-based workflow that supports teams and projects that deploy regularly".to_string(),
            template_type: WorkflowTemplateType::GitHubFlow,
            config: WorkflowConfig {
                workflow_type: WorkflowType::GitHubFlow,
                branch_conventions: BranchConventions {
                    main_branch: "main".to_string(),
                    develop_branch: None,
                    feature_prefix: "feature/".to_string(),
                    release_prefix: "release/".to_string(),
                    hotfix_prefix: "hotfix/".to_string(),
                    support_prefix: "support/".to_string(),
                },
                context_rules: ContextRules {
                    require_feature_validation: true,
                    require_release_validation: false,
                    require_hotfix_validation: true,
                    merge_strategies: HashMap::new(),
                    isolation_rules: IsolationRules {
                        isolate_feature: true,
                        isolate_release: false,
                        isolate_hotfix: true,
                        shared_files: vec!["README.md".to_string(), "LICENSE".to_string()],
                    },
                },
                release_management: ReleaseManagement {
                    versioning: VersioningStrategy::Semantic,
                    branch_preparation: crate::git::workflow::BranchPreparation {
                        prepare_context: false,
                        update_version: true,
                        generate_notes: true,
                        validate_readiness: true,
                    },
                    validation: crate::git::workflow::ReleaseValidation {
                        validate_context: true,
                        validate_dependencies: true,
                        validate_breaking_changes: true,
                        run_tests: true,
                    },
                    automation: crate::git::workflow::ReleaseAutomation {
                        auto_create_branch: false,
                        auto_version_bump: true,
                        auto_release_notes: true,
                        auto_deploy: true,
                    },
                },
                pull_request_settings: PullRequestSettings {
                    require_context_analysis: true,
                    require_impact_analysis: false,
                    require_dependency_review: true,
                    require_health_checks: true,
                    automated_checks: vec![
                        "context-validation".to_string(),
                        "security-scan".to_string(),
                    ],
                },
                automation: AutomationSettings {
                    auto_context_updates: true,
                    auto_synchronization: true,
                    auto_notifications: true,
                    auto_backups: false,
                },
                advanced_features: AdvancedWorkflowFeatures {
                    context_aware_branching: true,
                    auto_context_sync: true,
                    context_conflict_resolution: true,
                    context_validation_workflows: true,
                    context_evolution_tracking: false,
                    context_analytics: false,
                    context_optimization: false,
                    context_backup_workflows: false,
                },
                context_aware: ContextAwareWorkflowSettings {
                    context_aware_feature_branching: ContextAwareFeatureBranching {
                        auto_isolate_context: true,
                        auto_sync_parent: true,
                        auto_validate_before_merge: true,
                        auto_resolve_conflicts: true,
                        inheritance_rules: vec![],
                        boundary_rules: vec![],
                        validation_rules: vec![],
                    },
                    context_aware_release_management: ContextAwareReleaseManagement {
                        auto_prepare_context: false,
                        auto_validate_release_context: true,
                        auto_generate_release_notes: true,
                        auto_update_version: true,
                        validation_rules: vec![],
                        preparation_steps: vec![],
                        cleanup_steps: vec![],
                    },
                    context_aware_hotfix_management: ContextAwareHotfixManagement {
                        auto_isolate_context: true,
                        auto_validate_context: true,
                        auto_merge_context: true,
                        validation_rules: vec![],
                        merge_strategies: vec![],
                    },
                    context_aware_pr_analysis: ContextAwarePrAnalysis {
                        auto_analyze_context_changes: true,
                        auto_detect_conflicts: true,
                        auto_generate_impact_report: false,
                        auto_suggest_improvements: true,
                        analysis_rules: vec![],
                        validation_rules: vec![],
                        automation_rules: vec![],
                    },
                    context_aware_merge_strategies: ContextAwareMergeStrategies {
                        feature_merge_strategy: ContextMergeStrategy {
                            name: "feature-merge".to_string(),
                            description: "Feature branch merge strategy".to_string(),
                            strategy_type: crate::git::workflow::ContextMergeStrategyType::SemiAuto,
                            conflict_resolution: ContextConflictResolution {
                                resolution_type: ConflictResolutionType::Merge,
                                auto_resolve_simple: true,
                                manual_resolution_required: false,
                                resolution_rules: vec![],
                            },
                            validation_rules: vec![],
                        },
                        release_merge_strategy: ContextMergeStrategy {
                            name: "release-merge".to_string(),
                            description: "Release branch merge strategy".to_string(),
                            strategy_type: crate::git::workflow::ContextMergeStrategyType::Auto,
                            conflict_resolution: ContextConflictResolution {
                                resolution_type: ConflictResolutionType::TakeSource,
                                auto_resolve_simple: true,
                                manual_resolution_required: false,
                                resolution_rules: vec![],
                            },
                            validation_rules: vec![],
                        },
                        hotfix_merge_strategy: ContextMergeStrategy {
                            name: "hotfix-merge".to_string(),
                            description: "Hotfix branch merge strategy".to_string(),
                            strategy_type: crate::git::workflow::ContextMergeStrategyType::Auto,
                            conflict_resolution: ContextConflictResolution {
                                resolution_type: ConflictResolutionType::TakeSource,
                                auto_resolve_simple: true,
                                manual_resolution_required: false,
                                resolution_rules: vec![],
                            },
                            validation_rules: vec![],
                        },
                        custom_strategies: vec![],
                    },
                },
                integrations: crate::git::workflow::WorkflowIntegrationSettings {
                    ci_cd: None,
                    issue_tracker: None,
                    chat: None,
                    monitoring: None,
                },
            },
            metadata: TemplateMetadata {
                version: "1.0.0".to_string(),
                author: "Rhema Team".to_string(),
                tags: vec!["github-flow".to_string(), "simple".to_string(), "deployment".to_string()],
                complexity: ComplexityLevel::Simple,
                maturity: MaturityLevel::Production,
                examples: vec![
                    "Web applications with continuous deployment".to_string(),
                    "Small to medium teams".to_string(),
                    "Projects with frequent releases".to_string(),
                ],
                requirements: vec![
                    "Git repository".to_string(),
                    "Automated testing".to_string(),
                    "Continuous deployment setup".to_string(),
                ],
            },
            validation_rules: vec![],
            customization_options: vec![
                TemplateCustomizationOption {
                    name: "main_branch".to_string(),
                    description: "Name of the main branch".to_string(),
                    option_type: CustomizationOptionType::String,
                    default_value: serde_json::Value::String("main".to_string()),
                    choices: None,
                    required: true,
                },
            ],
        }
    }

    // Additional template implementations would follow the same pattern...
    fn gitlab_flow_template() -> WorkflowTemplate {
        // Implementation similar to GitHub Flow but with GitLab-specific features
        Self::github_flow_template() // Placeholder
    }

    fn trunk_based_template() -> WorkflowTemplate {
        // Implementation for trunk-based development
        Self::github_flow_template() // Placeholder
    }

    fn feature_branch_template() -> WorkflowTemplate {
        // Implementation for feature branch workflow
        Self::github_flow_template() // Placeholder
    }

    fn release_focused_template() -> WorkflowTemplate {
        // Implementation for release-focused workflow
        Self::gitflow_template() // Placeholder
    }

    fn hotfix_focused_template() -> WorkflowTemplate {
        // Implementation for hotfix-focused workflow
        Self::gitflow_template() // Placeholder
    }

    fn enterprise_template() -> WorkflowTemplate {
        // Implementation for enterprise workflow
        Self::gitflow_template() // Placeholder
    }

    fn open_source_template() -> WorkflowTemplate {
        // Implementation for open source workflow
        Self::github_flow_template() // Placeholder
    }

    fn microservices_template() -> WorkflowTemplate {
        // Implementation for microservices workflow
        Self::github_flow_template() // Placeholder
    }

    fn monorepo_template() -> WorkflowTemplate {
        // Implementation for monorepo workflow
        Self::gitflow_template() // Placeholder
    }

    /// Validate workflow configuration
    fn validate_workflow_config(config: &WorkflowConfig) -> RhemaResult<Vec<String>> {
        let mut errors = Vec::new();

        // Validate branch conventions
        if config.branch_conventions.main_branch.is_empty() {
            errors.push("Main branch name cannot be empty".to_string());
        }

        // Validate workflow type specific requirements
        match config.workflow_type {
            WorkflowType::GitFlow => {
                if config.branch_conventions.develop_branch.is_none() {
                    errors.push("GitFlow requires a develop branch".to_string());
                }
            }
            _ => {}
        }

        Ok(errors)
    }

    /// Apply customization to workflow configuration
    fn apply_customization_to_config(
        config: &mut WorkflowConfig,
        key: &str,
        value: &serde_json::Value,
    ) -> RhemaResult<()> {
        match key {
            "main_branch" => {
                if let Some(branch_name) = value.as_str() {
                    config.branch_conventions.main_branch = branch_name.to_string();
                }
            }
            "develop_branch" => {
                if let Some(branch_name) = value.as_str() {
                    config.branch_conventions.develop_branch = Some(branch_name.to_string());
                }
            }
            "feature_prefix" => {
                if let Some(prefix) = value.as_str() {
                    config.branch_conventions.feature_prefix = prefix.to_string();
                }
            }
            "release_prefix" => {
                if let Some(prefix) = value.as_str() {
                    config.branch_conventions.release_prefix = prefix.to_string();
                }
            }
            "hotfix_prefix" => {
                if let Some(prefix) = value.as_str() {
                    config.branch_conventions.hotfix_prefix = prefix.to_string();
                }
            }
            _ => {
                return Err(RhemaError::ConfigError(format!("Unknown customization key: {}", key)));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_available_templates() {
        let templates = WorkflowTemplateManager::get_available_templates();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.template_type == WorkflowTemplateType::GitFlow));
        assert!(templates.iter().any(|t| t.template_type == WorkflowTemplateType::GitHubFlow));
    }

    #[test]
    fn test_get_template() {
        let template = WorkflowTemplateManager::get_template(&WorkflowTemplateType::GitFlow);
        assert!(template.is_ok());
        let template = template.unwrap();
        assert_eq!(template.template_type, WorkflowTemplateType::GitFlow);
        assert_eq!(template.name, "GitFlow");
    }

    #[test]
    fn test_validate_template() {
        let template = WorkflowTemplateManager::get_template(&WorkflowTemplateType::GitFlow).unwrap();
        let errors = WorkflowTemplateManager::validate_template(&template);
        assert!(errors.is_ok());
        let errors = errors.unwrap();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_apply_customization() {
        let template = WorkflowTemplateManager::get_template(&WorkflowTemplateType::GitFlow).unwrap();
        let mut customizations = HashMap::new();
        customizations.insert("main_branch".to_string(), serde_json::Value::String("master".to_string()));
        
        let config = WorkflowTemplateManager::apply_customization(&template, &customizations);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.branch_conventions.main_branch, "master");
    }
} 