use crate::{
    ComprehensiveValidationResult, Config, ConfigIssue, ConfigIssueSeverity, GlobalConfig,
    ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration feedback provider
#[derive(Debug, Clone)]
pub struct ConfigFeedbackProvider {
    /// Feedback templates for different types of issues
    feedback_templates: HashMap<String, FeedbackTemplate>,
    /// Suggestion patterns for common configuration improvements
    suggestion_patterns: Vec<SuggestionPattern>,
}

/// Feedback template for configuration issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackTemplate {
    /// Template ID
    pub id: String,
    /// Issue severity this template applies to
    pub severity: ConfigIssueSeverity,
    /// Template message with placeholders
    pub message_template: String,
    /// Suggested actions to resolve the issue
    pub suggested_actions: Vec<String>,
    /// Related documentation links
    pub documentation_links: Vec<String>,
}

/// Suggestion pattern for configuration improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionPattern {
    /// Pattern ID
    pub id: String,
    /// Pattern description
    pub description: String,
    /// Pattern to match in configuration
    pub pattern: String,
    /// Suggested improvement
    pub suggestion: String,
    /// Priority of the suggestion
    pub priority: SuggestionPriority,
}

/// Suggestion priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuggestionPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Configuration feedback result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFeedback {
    /// Validation feedback
    pub validation_feedback: Vec<ValidationFeedback>,
    /// Configuration suggestions
    pub suggestions: Vec<ConfigurationSuggestion>,
    /// Overall feedback summary
    pub summary: FeedbackSummary,
}

/// Validation feedback for specific issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFeedback {
    /// Issue that generated this feedback
    pub issue: ConfigIssue,
    /// User-friendly message
    pub message: String,
    /// Suggested actions to resolve
    pub suggested_actions: Vec<String>,
    /// Related documentation
    pub documentation_links: Vec<String>,
    /// Estimated time to fix
    pub estimated_fix_time: Option<String>,
}

/// Configuration suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSuggestion {
    /// Suggestion ID
    pub id: String,
    /// Suggestion description
    pub description: String,
    /// Configuration path affected
    pub config_path: String,
    /// Current value (if applicable)
    pub current_value: Option<String>,
    /// Suggested value
    pub suggested_value: String,
    /// Priority of the suggestion
    pub priority: SuggestionPriority,
    /// Expected impact of the change
    pub expected_impact: String,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
}

/// Feedback summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackSummary {
    /// Total number of issues
    pub total_issues: usize,
    /// Number of critical issues
    pub critical_issues: usize,
    /// Number of high priority issues
    pub high_priority_issues: usize,
    /// Number of medium priority issues
    pub medium_priority_issues: usize,
    /// Number of low priority issues
    pub low_priority_issues: usize,
    /// Number of suggestions
    pub total_suggestions: usize,
    /// Overall health score (0-100)
    pub health_score: u8,
    /// Recommended next actions
    pub recommended_actions: Vec<String>,
}

impl ConfigFeedbackProvider {
    /// Create a new feedback provider with default templates
    pub fn new() -> Self {
        let mut provider = Self {
            feedback_templates: HashMap::new(),
            suggestion_patterns: Vec::new(),
        };
        provider.initialize_default_templates();
        provider.initialize_suggestion_patterns();
        provider
    }

    /// Initialize default feedback templates
    fn initialize_default_templates(&mut self) {
        let templates = vec![
            FeedbackTemplate {
                id: "validation_error".to_string(),
                severity: ConfigIssueSeverity::Error,
                message_template: "Configuration validation failed: {message}".to_string(),
                suggested_actions: vec![
                    "Review the configuration schema".to_string(),
                    "Check for missing required fields".to_string(),
                    "Validate data types and formats".to_string(),
                ],
                documentation_links: vec![
                    "https://docs.rhema.ai/configuration/validation".to_string()
                ],
            },
            FeedbackTemplate {
                id: "security_warning".to_string(),
                severity: ConfigIssueSeverity::Warning,
                message_template: "Security concern detected: {message}".to_string(),
                suggested_actions: vec![
                    "Review security settings".to_string(),
                    "Enable encryption for sensitive data".to_string(),
                    "Update access controls".to_string(),
                ],
                documentation_links: vec![
                    "https://docs.rhema.ai/configuration/security".to_string()
                ],
            },
            FeedbackTemplate {
                id: "performance_optimization".to_string(),
                severity: ConfigIssueSeverity::Info,
                message_template: "Performance optimization opportunity: {message}".to_string(),
                suggested_actions: vec![
                    "Consider enabling caching".to_string(),
                    "Optimize resource allocation".to_string(),
                    "Review parallel processing settings".to_string(),
                ],
                documentation_links: vec![
                    "https://docs.rhema.ai/configuration/performance".to_string()
                ],
            },
        ];

        for template in templates {
            self.feedback_templates
                .insert(template.id.clone(), template);
        }
    }

    /// Initialize suggestion patterns
    fn initialize_suggestion_patterns(&mut self) {
        self.suggestion_patterns = vec![
            SuggestionPattern {
                id: "enable_caching".to_string(),
                description: "Enable caching for better performance".to_string(),
                pattern: r#""cache":\s*\{\s*"enabled":\s*false"#.to_string(),
                suggestion: "Consider enabling caching to improve performance".to_string(),
                priority: SuggestionPriority::Medium,
            },
            SuggestionPattern {
                id: "security_encryption".to_string(),
                description: "Enable encryption for sensitive data".to_string(),
                pattern: r#""encryption":\s*\{\s*"enabled":\s*false"#.to_string(),
                suggestion: "Enable encryption to protect sensitive configuration data".to_string(),
                priority: SuggestionPriority::High,
            },
            SuggestionPattern {
                id: "backup_schedule".to_string(),
                description: "Configure automatic backup schedule".to_string(),
                pattern: r#""backup":\s*\{\s*"schedule":\s*null"#.to_string(),
                suggestion: "Configure automatic backup schedule to prevent data loss".to_string(),
                priority: SuggestionPriority::High,
            },
        ];
    }

    /// Generate feedback for validation results
    pub async fn generate_feedback<T: Config>(
        &self,
        config: &T,
        validation_result: &ValidationResult,
    ) -> ConfigFeedback {
        let mut feedback = ConfigFeedback {
            validation_feedback: Vec::new(),
            suggestions: Vec::new(),
            summary: FeedbackSummary {
                total_issues: 0,
                critical_issues: 0,
                high_priority_issues: 0,
                medium_priority_issues: 0,
                low_priority_issues: 0,
                total_suggestions: 0,
                health_score: 100,
                recommended_actions: Vec::new(),
            },
        };

        // Generate validation feedback
        if !validation_result.valid {
            for issue in &validation_result.issues {
                let validation_feedback = self.create_validation_feedback(issue);
                feedback.validation_feedback.push(validation_feedback);
            }
        }

        // Generate configuration suggestions
        feedback.suggestions = self.generate_suggestions(config).await;

        // Calculate summary
        feedback.summary = self.calculate_feedback_summary(&feedback);

        feedback
    }

    /// Create validation feedback for a specific issue
    fn create_validation_feedback(&self, issue: &ConfigIssue) -> ValidationFeedback {
        let template = self.get_template_for_issue(issue);

        ValidationFeedback {
            issue: issue.clone(),
            message: template
                .message_template
                .replace("{message}", &issue.message),
            suggested_actions: template.suggested_actions.clone(),
            documentation_links: template.documentation_links.clone(),
            estimated_fix_time: self.estimate_fix_time(issue),
        }
    }

    /// Get appropriate template for an issue
    fn get_template_for_issue(&self, issue: &ConfigIssue) -> &FeedbackTemplate {
        // Determine template based on issue type and severity
        let template_id = match issue.severity {
            ConfigIssueSeverity::Critical => "critical_error",
            ConfigIssueSeverity::Error => "validation_error",
            ConfigIssueSeverity::Warning => "security_warning",
            ConfigIssueSeverity::Info => "performance_optimization",
        };

        self.feedback_templates
            .get(template_id)
            .unwrap_or_else(|| self.feedback_templates.get("validation_error").unwrap())
    }

    /// Estimate time to fix an issue
    fn estimate_fix_time(&self, issue: &ConfigIssue) -> Option<String> {
        match issue.severity {
            ConfigIssueSeverity::Critical => Some("10-30 minutes".to_string()),
            ConfigIssueSeverity::Error => Some("5-15 minutes".to_string()),
            ConfigIssueSeverity::Warning => Some("2-5 minutes".to_string()),
            ConfigIssueSeverity::Info => Some("1-2 minutes".to_string()),
        }
    }

    /// Generate configuration suggestions
    async fn generate_suggestions<T: Config>(&self, config: &T) -> Vec<ConfigurationSuggestion> {
        let mut suggestions = Vec::new();

        // Analyze configuration and generate suggestions based on patterns
        for pattern in &self.suggestion_patterns {
            if let Some(suggestion) = self.analyze_pattern(config, pattern).await {
                suggestions.push(suggestion);
            }
        }

        suggestions
    }

    /// Analyze a specific pattern against the configuration
    async fn analyze_pattern<T: Config>(
        &self,
        config: &T,
        pattern: &SuggestionPattern,
    ) -> Option<ConfigurationSuggestion> {
        // This is a simplified implementation
        // In a real implementation, you would parse the config and apply regex patterns

        Some(ConfigurationSuggestion {
            id: pattern.id.clone(),
            description: pattern.description.clone(),
            config_path: "config".to_string(),
            current_value: None,
            suggested_value: pattern.suggestion.clone(),
            priority: pattern.priority.clone(),
            expected_impact: "Improved performance and security".to_string(),
            implementation_steps: vec![
                "Review the current configuration".to_string(),
                "Apply the suggested changes".to_string(),
                "Test the configuration".to_string(),
                "Validate the changes".to_string(),
            ],
        })
    }

    /// Calculate feedback summary
    fn calculate_feedback_summary(&self, feedback: &ConfigFeedback) -> FeedbackSummary {
        let mut summary = FeedbackSummary {
            total_issues: feedback.validation_feedback.len(),
            critical_issues: 0,
            high_priority_issues: 0,
            medium_priority_issues: 0,
            low_priority_issues: 0,
            total_suggestions: feedback.suggestions.len(),
            health_score: 100,
            recommended_actions: Vec::new(),
        };

        // Count issues by severity
        for validation_feedback in &feedback.validation_feedback {
            match validation_feedback.issue.severity {
                ConfigIssueSeverity::Critical => summary.critical_issues += 1,
                ConfigIssueSeverity::Error => summary.critical_issues += 1,
                ConfigIssueSeverity::Warning => summary.high_priority_issues += 1,
                ConfigIssueSeverity::Info => summary.medium_priority_issues += 1,
            }
        }

        // Calculate health score
        let total_issues = summary.total_issues as f32;
        let total_suggestions = summary.total_suggestions as f32;

        if total_issues > 0.0 {
            summary.health_score = (100.0 - (total_issues * 10.0).min(100.0)) as u8;
        }

        // Generate recommended actions
        if summary.critical_issues > 0 {
            summary
                .recommended_actions
                .push("Fix critical validation errors first".to_string());
        }
        if summary.high_priority_issues > 0 {
            summary
                .recommended_actions
                .push("Address security warnings".to_string());
        }
        if summary.total_suggestions > 0 {
            summary
                .recommended_actions
                .push("Review and apply configuration suggestions".to_string());
        }

        summary
    }
}

impl Default for ConfigFeedbackProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConfigIssue;

    #[tokio::test]
    async fn test_feedback_provider_creation() {
        let provider = ConfigFeedbackProvider::new();
        assert!(!provider.feedback_templates.is_empty());
        assert!(!provider.suggestion_patterns.is_empty());
    }

    #[tokio::test]
    async fn test_feedback_generation() {
        let provider = ConfigFeedbackProvider::new();
        let config = GlobalConfig::new();
        let validation_result = ValidationResult {
            valid: true,
            issues: Vec::new(),
            warnings: Vec::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
        };

        let feedback = provider
            .generate_feedback(&config, &validation_result)
            .await;

        assert_eq!(feedback.validation_feedback.len(), 0);
        assert!(feedback.suggestions.len() > 0);
        assert_eq!(feedback.summary.health_score, 100);
    }

    #[tokio::test]
    async fn test_validation_feedback_with_issues() {
        let provider = ConfigFeedbackProvider::new();
        let config = GlobalConfig::new();

        let issues = vec![ConfigIssue {
            severity: ConfigIssueSeverity::Error,
            message: "Test error message".to_string(),
            location: Some("test.path".to_string()),
            suggestion: Some("Fix the validation error".to_string()),
        }];

        let validation_result = ValidationResult {
            valid: false,
            issues,
            warnings: Vec::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
        };

        let feedback = provider
            .generate_feedback(&config, &validation_result)
            .await;

        assert_eq!(feedback.validation_feedback.len(), 1);
        assert_eq!(feedback.summary.critical_issues, 1);
        assert!(feedback.summary.health_score < 100);
    }
}
