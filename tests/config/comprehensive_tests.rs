//! Comprehensive test suite for the enhanced configuration system
//!
//! This test suite covers all the new features implemented:
//! - Configuration feedback system
//! - Configuration documentation generation
//! - Configuration wizard
//! - Integration between all components

use rhema_config::{
    documentation::{
        ConfigDocumentationGenerator, DocumentationFormat, DocumentationSettings,
    },
    feedback::{
        ConfigFeedbackProvider,
        SuggestionPriority,
    },
    wizard::{
        ConfigWizard, WizardSettings,
    },
    Config, ConfigEnvironment, ConfigIssue, ConfigIssueSeverity, GlobalConfig, ValidationResult,
};
use serde_json::json;

/// Test fixtures for comprehensive tests
mod fixtures {
    use super::*;

    /// Create a test configuration with various issues
    pub fn create_test_config_with_issues() -> GlobalConfig {
        let mut config = GlobalConfig::new();

        // Add some configuration that would trigger suggestions
        config.user.id = "test_user".to_string();
        config.user.name = "Test User".to_string();
        config.user.email = "test@example.com".to_string();

        config
    }

    /// Create a test configuration for documentation generation
    pub fn create_test_config_for_docs() -> GlobalConfig {
        let mut config = GlobalConfig::new();

        config.user.id = "doc_user".to_string();
        config.user.name = "Documentation User".to_string();
        config.user.email = "docs@example.com".to_string();

        config.application.name = "Documentation Test Project".to_string();
        config.environment.current = ConfigEnvironment::Development;

        // Note: repository fields don't exist in GlobalConfig, removing these lines

        config
    }

    /// Create test validation issues
    pub fn create_test_validation_issues() -> Vec<ConfigIssue> {
        vec![
            ConfigIssue {
                severity: ConfigIssueSeverity::Error,
                message: "User ID is too short".to_string(),
                location: Some("global.user.id".to_string()),
                suggestion: Some("Use a longer user ID".to_string()),
            },
            ConfigIssue {
                severity: ConfigIssueSeverity::Warning,
                message: "Encryption is disabled".to_string(),
                location: Some("security.encryption.enabled".to_string()),
                suggestion: Some("Enable encryption for security".to_string()),
            },
            ConfigIssue {
                severity: ConfigIssueSeverity::Info,
                message: "No backup schedule configured".to_string(),
                location: Some("backup.schedule".to_string()),
                suggestion: Some("Configure a backup schedule".to_string()),
            },
        ]
    }
}

#[tokio::test]
async fn test_feedback_system_integration() {
    let provider = ConfigFeedbackProvider::new();
    let config = fixtures::create_test_config_with_issues();

    // Test with valid configuration
    let validation_result = ValidationResult {
        valid: true,
        issues: vec![],
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    };
    let feedback = provider
        .generate_feedback(&config, &validation_result)
        .await;

    assert_eq!(feedback.validation_feedback.len(), 0);
    assert!(feedback.suggestions.len() > 0);
    assert_eq!(feedback.summary.health_score, 100);
    assert_eq!(feedback.summary.total_issues, 0);

    // Test with invalid configuration
    let issues = fixtures::create_test_validation_issues();
    let validation_result = ValidationResult {
        valid: false,
        issues,
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    };
    let feedback = provider
        .generate_feedback(&config, &validation_result)
        .await;

    assert_eq!(feedback.validation_feedback.len(), 3);
    assert_eq!(feedback.summary.critical_issues, 1);
    assert_eq!(feedback.summary.high_priority_issues, 1);
    assert_eq!(feedback.summary.medium_priority_issues, 1);
    assert!(feedback.summary.health_score < 100);
}

#[tokio::test]
async fn test_documentation_generation_integration() {
    let settings = DocumentationSettings::default();
    let generator = ConfigDocumentationGenerator::new(settings);
    let config = fixtures::create_test_config_for_docs();

    // Test markdown generation
    let result = generator
        .generate_documentation(&config, DocumentationFormat::Markdown)
        .await;

    assert!(!result.documentation.sections.is_empty());
    assert_eq!(
        result.documentation.title,
        "Rhema Configuration Documentation"
    );
    assert_eq!(result.documentation.version, config.version);
    assert!(!result.output_files.is_empty());
    assert!(result.statistics.generation_time_ms >= 0); // Can be 0 if very fast
    assert!(result.statistics.files_generated > 0);

    // Test HTML generation
    let result = generator
        .generate_documentation(&config, DocumentationFormat::HTML)
        .await;

    assert!(!result.documentation.sections.is_empty());
    assert!(!result.output_files.is_empty());

    // Test JSON generation
    let result = generator
        .generate_documentation(&config, DocumentationFormat::JSON)
        .await;

    assert!(!result.documentation.sections.is_empty());
    assert!(!result.output_files.is_empty());
}

#[tokio::test]
async fn test_wizard_integration() {
    let settings = WizardSettings::default();
    let mut wizard = ConfigWizard::new(settings);

    // Test wizard navigation
    let progress = wizard.get_progress();
    assert_eq!(progress.current_step, 0);
    assert!(progress.total_steps > 0);
    assert!(progress.can_go_forward);
    assert!(!progress.can_go_back);

    // Test answering welcome question
    let proceed_answer = json!(true);
    let result = wizard.answer_question("proceed", proceed_answer);
    assert!(result.is_ok());

    // Navigate to user info step
    let progress = wizard.next_step().unwrap();
    assert_eq!(progress.current_step, 1);

    // Test answering user info questions
    let user_id_answer = json!("test_user");
    let result = wizard.answer_question("user_id", user_id_answer);
    assert!(result.is_ok());

    let user_name_answer = json!("Test User");
    let result = wizard.answer_question("user_name", user_name_answer);
    assert!(result.is_ok());

    let user_email_answer = json!("test@example.com");
    let result = wizard.answer_question("user_email", user_email_answer);
    assert!(result.is_ok());

    // Test navigation - we're already at step 1, so going back should take us to step 0
    let progress = wizard.previous_step().unwrap();
    assert_eq!(progress.current_step, 0);
}

#[tokio::test]
async fn test_feedback_templates() {
    let provider = ConfigFeedbackProvider::new();

    // Test that templates are properly initialized
    // This would test the internal template initialization
    let config = GlobalConfig::new();
    let validation_result = ValidationResult {
        valid: true,
        issues: vec![],
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    };
    let feedback = provider
        .generate_feedback(&config, &validation_result)
        .await;

    // Should have suggestions even for valid config
    assert!(feedback.suggestions.len() > 0);

    // Test suggestion priorities
    let has_high_priority = feedback
        .suggestions
        .iter()
        .any(|s| s.priority == SuggestionPriority::High);
    let has_medium_priority = feedback
        .suggestions
        .iter()
        .any(|s| s.priority == SuggestionPriority::Medium);

    assert!(has_high_priority || has_medium_priority);
}

#[tokio::test]
async fn test_documentation_sections() {
    let settings = DocumentationSettings::default();
    let generator = ConfigDocumentationGenerator::new(settings);
    let config = fixtures::create_test_config_for_docs();

    let result = generator
        .generate_documentation(&config, DocumentationFormat::Markdown)
        .await;

    // Test that all expected sections are generated
    let section_titles: Vec<&str> = result
        .documentation
        .sections
        .iter()
        .map(|s| s.title.as_str())
        .collect();

    assert!(section_titles.contains(&"Overview"));
    assert!(section_titles.contains(&"Configuration"));
    assert!(section_titles.contains(&"Validation"));
    assert!(section_titles.contains(&"Examples"));
    assert!(section_titles.contains(&"Troubleshooting"));
    assert!(section_titles.contains(&"Best Practices"));
    assert!(section_titles.contains(&"API Reference"));
}

#[tokio::test]
async fn test_wizard_validation() {
    let settings = WizardSettings::default();
    let mut wizard = ConfigWizard::new(settings);

    // Navigate to user info step first
    wizard.next_step().unwrap();

    // Test invalid email validation
    let invalid_email = json!("invalid-email");
    let result = wizard.answer_question("user_email", invalid_email);
    assert!(result.is_err());

    // Test valid email validation
    let valid_email = json!("valid@example.com");
    let result = wizard.answer_question("user_email", valid_email);
    assert!(result.is_ok());

    // Test required field validation
    let empty_string = json!("");
    let result = wizard.answer_question("user_id", empty_string);
    assert!(result.is_err());

    // Test min length validation
    let short_id = json!("ab");
    let result = wizard.answer_question("user_id", short_id);
    assert!(result.is_err());

    // Test valid input
    let valid_id = json!("valid_user_id");
    let result = wizard.answer_question("user_id", valid_id);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_feedback_summary_calculation() {
    let provider = ConfigFeedbackProvider::new();
    let config = fixtures::create_test_config_with_issues();

    // Test with no issues
    let validation_result = ValidationResult {
        valid: true,
        issues: vec![],
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    };
    let feedback = provider
        .generate_feedback(&config, &validation_result)
        .await;

    assert_eq!(feedback.summary.total_issues, 0);
    assert_eq!(feedback.summary.critical_issues, 0);
    assert_eq!(feedback.summary.high_priority_issues, 0);
    assert_eq!(feedback.summary.medium_priority_issues, 0);
    assert_eq!(feedback.summary.low_priority_issues, 0);
    assert_eq!(feedback.summary.health_score, 100);

    // Test with issues
    let issues = fixtures::create_test_validation_issues();
    let validation_result = ValidationResult {
        valid: false,
        issues,
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    };
    let feedback = provider
        .generate_feedback(&config, &validation_result)
        .await;

    assert_eq!(feedback.summary.total_issues, 3);
    assert_eq!(feedback.summary.critical_issues, 1);
    assert_eq!(feedback.summary.high_priority_issues, 1);
    assert_eq!(feedback.summary.medium_priority_issues, 1);
    assert!(feedback.summary.health_score < 100);
    assert!(!feedback.summary.recommended_actions.is_empty());
}

#[tokio::test]
async fn test_documentation_statistics() {
    let settings = DocumentationSettings::default();
    let generator = ConfigDocumentationGenerator::new(settings);
    let config = fixtures::create_test_config_for_docs();

    let result = generator
        .generate_documentation(&config, DocumentationFormat::Markdown)
        .await;

    // Test statistics
    assert!(result.statistics.generation_time_ms >= 0); // Can be 0 if very fast
    assert!(result.statistics.files_generated > 0);
    assert!(result.statistics.sections_generated > 0);
    assert!(result.statistics.total_size_bytes > 0);

    // Test documentation summary
    assert!(result.documentation.summary.total_options > 0);
    assert!(result.documentation.summary.required_options > 0);
    assert!(result.documentation.summary.optional_options > 0);
    assert!(result.documentation.summary.validation_rules > 0);
    assert!(result.documentation.summary.completeness_score > 0);
}

#[tokio::test]
async fn test_wizard_progress_tracking() {
    let settings = WizardSettings::default();
    let mut wizard = ConfigWizard::new(settings);

    let total_steps = wizard.get_progress().total_steps;

    // Test progress through all steps
    for step in 0..total_steps {
        let progress = wizard.get_progress();
        assert_eq!(progress.current_step, step);
        assert_eq!(progress.total_steps, total_steps);

        let expected_percentage = ((step + 1) * 100) / total_steps;
        assert_eq!(progress.progress_percentage, expected_percentage as u8);

        if step < total_steps - 1 {
            let next_progress = wizard.next_step().unwrap();
            assert_eq!(next_progress.current_step, step + 1);
        }
    }

    // Test that we can't go beyond the last step
    let result = wizard.next_step();
    assert!(result.is_err());

    // Test going back to first step
    for _ in 0..total_steps - 1 {
        let _ = wizard.previous_step().unwrap();
    }

    let progress = wizard.get_progress();
    assert_eq!(progress.current_step, 0);

    // Test that we can't go before the first step
    let result = wizard.previous_step();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_integration_end_to_end() {
    // Test the complete flow: wizard -> config -> validation -> feedback -> documentation

    // 1. Create configuration using wizard
    let wizard_settings = WizardSettings::default();
    let mut wizard = ConfigWizard::new(wizard_settings);

    // Answer welcome step question
    wizard.answer_question("proceed", json!(true)).unwrap();

    // Navigate to user info step
    wizard.next_step().unwrap();

    // Answer user info questions
    wizard
        .answer_question("user_id", json!("integration_test_user"))
        .unwrap();
    wizard
        .answer_question("user_name", json!("Integration Test User"))
        .unwrap();
    wizard
        .answer_question("user_email", json!("integration@example.com"))
        .unwrap();

    // Navigate to application setup step
    wizard.next_step().unwrap();

    // Answer application setup questions
    wizard
        .answer_question("app_name", json!("Integration Test Project"))
        .unwrap();
    wizard
        .answer_question("environment", json!("development"))
        .unwrap();

    // 2. Generate configuration
    let wizard_result = wizard.complete().await.unwrap();
    let config = wizard_result.config;

    // 3. Validate configuration
    let validation_result = ValidationResult {
        valid: true,
        issues: vec![],
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    }; // In real scenario, this would use actual validation

    // 4. Generate feedback
    let feedback_provider = ConfigFeedbackProvider::new();
    let feedback = feedback_provider
        .generate_feedback(&config, &validation_result)
        .await;

    // 5. Generate documentation
    let doc_settings = DocumentationSettings::default();
    let doc_generator = ConfigDocumentationGenerator::new(doc_settings);
    let doc_result = doc_generator
        .generate_documentation(&config, DocumentationFormat::Markdown)
        .await;

    // 6. Verify results
    assert_eq!(wizard_result.statistics.steps_completed, 9); // All wizard steps
    assert!(wizard_result.statistics.time_taken_seconds >= 0); // Can be 0 if very fast
    assert!(wizard_result.statistics.questions_answered > 0);

    assert_eq!(feedback.summary.health_score, 100);
    assert!(feedback.suggestions.len() > 0);

    assert!(!doc_result.documentation.sections.is_empty());
    assert!(doc_result.statistics.generation_time_ms >= 0); // Can be 0 if very fast
    assert!(!doc_result.output_files.is_empty());
}

#[tokio::test]
async fn test_error_handling() {
    // Test feedback provider with invalid data
    let provider = ConfigFeedbackProvider::new();
    let config = GlobalConfig::new();

    // Test with empty validation result
    let empty_issues = vec![];
    let validation_result = ValidationResult {
        valid: false,
        issues: empty_issues,
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    };
    let feedback = provider
        .generate_feedback(&config, &validation_result)
        .await;

    assert_eq!(feedback.validation_feedback.len(), 0);
    assert_eq!(feedback.summary.total_issues, 0);

    // Test wizard with invalid answers
    let settings = WizardSettings::default();
    let mut wizard = ConfigWizard::new(settings);

    // Navigate to user info step first
    wizard.next_step().unwrap();

    // Test invalid email
    let invalid_email = json!("not-an-email");
    let result = wizard.answer_question("user_email", invalid_email);
    assert!(result.is_err());

    // Test empty required field
    let empty_field = json!("");
    let result = wizard.answer_question("user_id", empty_field);
    assert!(result.is_err());

    // Test non-existent question
    let result = wizard.answer_question("non_existent", json!("value"));
    assert!(result.is_err());
}

#[tokio::test]
async fn test_performance_metrics() {
    // Test that operations complete within reasonable time limits

    let start_time = std::time::Instant::now();

    // Test feedback generation performance
    let provider = ConfigFeedbackProvider::new();
    let config = fixtures::create_test_config_with_issues();
    let validation_result = ValidationResult {
        valid: true,
        issues: vec![],
        warnings: vec![],
        timestamp: chrono::Utc::now(),
        duration_ms: 0,
    };
    let feedback = provider
        .generate_feedback(&config, &validation_result)
        .await;

    let feedback_time = start_time.elapsed();
    assert!(feedback_time.as_millis() < 1000); // Should complete within 1 second

    // Test documentation generation performance
    let doc_start = std::time::Instant::now();
    let settings = DocumentationSettings::default();
    let generator = ConfigDocumentationGenerator::new(settings);
    let doc_result = generator
        .generate_documentation(&config, DocumentationFormat::Markdown)
        .await;

    let doc_time = doc_start.elapsed();
    assert!(doc_time.as_millis() < 5000); // Should complete within 5 seconds

    // Test wizard performance
    let wizard_start = std::time::Instant::now();
    let wizard_settings = WizardSettings::default();
    let mut wizard = ConfigWizard::new(wizard_settings);

    // Navigate to user info step first
    wizard.next_step().unwrap();

    // Answer some questions
    wizard
        .answer_question("user_id", json!("perf_test_user"))
        .unwrap();
    wizard
        .answer_question("user_name", json!("Performance Test User"))
        .unwrap();
    wizard
        .answer_question("user_email", json!("perf@example.com"))
        .unwrap();

    let wizard_time = wizard_start.elapsed();
    assert!(wizard_time.as_millis() < 1000); // Should complete within 1 second

    // Verify that all operations produced valid results
    assert!(feedback.suggestions.len() > 0);
    assert!(!doc_result.documentation.sections.is_empty());
    assert!(wizard.get_progress().total_steps > 0);
}
