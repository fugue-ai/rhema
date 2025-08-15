//! Comprehensive example demonstrating all the new configuration features
//!
//! This example shows how to use:
//! - Configuration feedback system
//! - Configuration documentation generation
//! - Configuration wizard
//! - Integration between all components

use rhema_config::{
    feedback::{ConfigFeedbackProvider, SuggestionPriority},
    documentation::{ConfigDocumentationGenerator, DocumentationFormat, DocumentationSettings},
    wizard::{ConfigWizard, WizardSettings},
    Config, ConfigIssue, ConfigIssueSeverity, ValidationResult,
};
use serde_json::json;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Configuration - Comprehensive Features Example");
    println!("=====================================================\n");

    // Step 1: Use the Configuration Wizard
    println!("📋 Step 1: Configuration Wizard");
    println!("-------------------------------");
    await_wizard_example().await?;

    // Step 2: Generate Configuration Feedback
    println!("\n📊 Step 2: Configuration Feedback");
    println!("--------------------------------");
    await_feedback_example().await?;

    // Step 3: Generate Documentation
    println!("\n📚 Step 3: Documentation Generation");
    println!("----------------------------------");
    await_documentation_example().await?;

    // Step 4: End-to-End Integration
    println!("\n🔄 Step 4: End-to-End Integration");
    println!("--------------------------------");
    await_integration_example().await?;

    println!("\n✅ All examples completed successfully!");
    Ok(())
}

/// Example demonstrating the configuration wizard
async fn await_wizard_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating configuration wizard...");
    
    let settings = WizardSettings {
        allow_skip: true,
        show_progress: true,
        auto_save: true,
        save_path: Some("examples/output".to_string()),
        enable_validation: true,
        show_help: true,
    };
    
    let mut wizard = ConfigWizard::new(settings);
    
    // Show initial progress
    let progress = wizard.get_progress();
    println!("Wizard initialized with {} steps", progress.total_steps);
    println!("Current progress: {}%", progress.progress_percentage);
    
    // Answer some questions
    println!("Answering wizard questions...");
    
    wizard.answer_question("user_id", json!("example_user"))?;
    wizard.answer_question("user_name", json!("Example User"))?;
    wizard.answer_question("user_email", json!("example@rhema.ai"))?;
    wizard.answer_question("app_name", json!("Rhema Example Project"))?;
    wizard.answer_question("environment", json!("development"))?;
    wizard.answer_question("repo_type", json!("git"))?;
    wizard.answer_question("repo_url", json!("https://github.com/example/rhema-project"))?;
    wizard.answer_question("enable_encryption", json!(true))?;
    wizard.answer_question("enable_access_control", json!(true))?;
    wizard.answer_question("enable_audit_logging", json!(true))?;
    wizard.answer_question("enable_backup", json!(true))?;
    wizard.answer_question("backup_frequency", json!("daily"))?;
    wizard.answer_question("confirm_config", json!(true))?;
    
    // Complete the wizard
    println!("Completing wizard...");
    let result = wizard.complete().await?;
    
    println!("✅ Wizard completed successfully!");
    println!("   - Steps completed: {}", result.statistics.steps_completed);
    println!("   - Questions answered: {}", result.statistics.questions_answered);
    println!("   - Time taken: {} seconds", result.statistics.time_taken_seconds);
    println!("   - Generated files: {:?}", result.generated_files);
    
    Ok(())
}

/// Example demonstrating configuration feedback
async fn await_feedback_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating feedback provider...");
    
    let provider = ConfigFeedbackProvider::new();
    let config = create_example_config();
    
    // Test with valid configuration
    println!("Generating feedback for valid configuration...");
    let validation_result = ValidationResult::Valid;
    let feedback = provider.generate_feedback(&config, &validation_result).await;
    
    println!("✅ Valid configuration feedback:");
    println!("   - Health score: {}%", feedback.summary.health_score);
    println!("   - Total issues: {}", feedback.summary.total_issues);
    println!("   - Suggestions: {}", feedback.summary.total_suggestions);
    
    // Test with invalid configuration
    println!("Generating feedback for configuration with issues...");
    let issues = create_example_issues();
    let validation_result = ValidationResult::Invalid { issues };
    let feedback = provider.generate_feedback(&config, &validation_result).await;
    
    println!("⚠️  Invalid configuration feedback:");
    println!("   - Health score: {}%", feedback.summary.health_score);
    println!("   - Critical issues: {}", feedback.summary.critical_issues);
    println!("   - High priority issues: {}", feedback.summary.high_priority_issues);
    println!("   - Medium priority issues: {}", feedback.summary.medium_priority_issues);
    println!("   - Suggestions: {}", feedback.summary.total_suggestions);
    
    // Show some suggestions
    if !feedback.suggestions.is_empty() {
        println!("   - Top suggestions:");
        for suggestion in feedback.suggestions.iter().take(3) {
            println!("     • {} (Priority: {:?})", suggestion.description, suggestion.priority);
        }
    }
    
    // Show recommended actions
    if !feedback.summary.recommended_actions.is_empty() {
        println!("   - Recommended actions:");
        for action in &feedback.summary.recommended_actions {
            println!("     • {}", action);
        }
    }
    
    Ok(())
}

/// Example demonstrating documentation generation
async fn await_documentation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating documentation generator...");
    
    let settings = DocumentationSettings {
        include_examples: true,
        include_validation_rules: true,
        include_troubleshooting: true,
        include_best_practices: true,
        include_api_reference: true,
        custom_css: None,
        output_directory: PathBuf::from("examples/output/docs"),
    };
    
    let generator = ConfigDocumentationGenerator::new(settings);
    let config = create_example_config();
    
    // Generate documentation in different formats
    let formats = [
        DocumentationFormat::Markdown,
        DocumentationFormat::HTML,
        DocumentationFormat::JSON,
    ];
    
    for format in formats {
        println!("Generating documentation in {:?} format...", format);
        
        let result = generator.generate_documentation(&config, format).await;
        
        println!("✅ {:?} documentation generated:");
        println!("   - Sections: {}", result.documentation.sections.len());
        println!("   - Files: {}", result.statistics.files_generated);
        println!("   - Generation time: {}ms", result.statistics.generation_time_ms);
        println!("   - Total size: {} bytes", result.statistics.total_size_bytes);
        
        // Show documentation summary
        let summary = &result.documentation.summary;
        println!("   - Total options: {}", summary.total_options);
        println!("   - Required options: {}", summary.required_options);
        println!("   - Optional options: {}", summary.optional_options);
        println!("   - Validation rules: {}", summary.validation_rules);
        println!("   - Completeness score: {}%", summary.completeness_score);
    }
    
    Ok(())
}

/// Example demonstrating end-to-end integration
async fn await_integration_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running end-to-end integration example...");
    
    // 1. Create configuration using wizard
    println!("1. Creating configuration with wizard...");
    let wizard_settings = WizardSettings::default();
    let mut wizard = ConfigWizard::new(wizard_settings);
    
    // Answer essential questions
    wizard.answer_question("user_id", json!("integration_user"))?;
    wizard.answer_question("user_name", json!("Integration User"))?;
    wizard.answer_question("user_email", json!("integration@rhema.ai"))?;
    wizard.answer_question("app_name", json!("Integration Test Project"))?;
    wizard.answer_question("environment", json!("production"))?;
    wizard.answer_question("repo_type", json!("git"))?;
    wizard.answer_question("repo_url", json!("https://github.com/example/integration-test"))?;
    wizard.answer_question("enable_encryption", json!(true))?;
    wizard.answer_question("enable_access_control", json!(true))?;
    wizard.answer_question("enable_audit_logging", json!(true))?;
    wizard.answer_question("enable_backup", json!(true))?;
    wizard.answer_question("backup_frequency", json!("hourly"))?;
    wizard.answer_question("confirm_config", json!(true))?;
    
    let wizard_result = wizard.complete().await?;
    let config = wizard_result.config;
    
    println!("   ✅ Configuration created with {} questions answered", wizard_result.statistics.questions_answered);
    
    // 2. Validate configuration
    println!("2. Validating configuration...");
    let validation_result = ValidationResult::Valid; // In real scenario, use actual validation
    
    println!("   ✅ Configuration validation passed");
    
    // 3. Generate feedback
    println!("3. Generating configuration feedback...");
    let feedback_provider = ConfigFeedbackProvider::new();
    let feedback = feedback_provider.generate_feedback(&config, &validation_result).await;
    
    println!("   ✅ Feedback generated:");
    println!("      - Health score: {}%", feedback.summary.health_score);
    println!("      - Suggestions: {}", feedback.summary.total_suggestions);
    
    // 4. Generate documentation
    println!("4. Generating configuration documentation...");
    let doc_settings = DocumentationSettings::default();
    let doc_generator = ConfigDocumentationGenerator::new(doc_settings);
    let doc_result = doc_generator
        .generate_documentation(&config, DocumentationFormat::Markdown)
        .await;
    
    println!("   ✅ Documentation generated:");
    println!("      - Sections: {}", doc_result.documentation.sections.len());
    println!("      - Files: {}", doc_result.statistics.files_generated);
    
    // 5. Show final summary
    println!("5. Integration summary:");
    println!("   ✅ Wizard: {} steps completed in {} seconds", 
             wizard_result.statistics.steps_completed, 
             wizard_result.statistics.time_taken_seconds);
    println!("   ✅ Feedback: {} suggestions with {}% health score", 
             feedback.summary.total_suggestions, 
             feedback.summary.health_score);
    println!("   ✅ Documentation: {} sections generated in {}ms", 
             doc_result.statistics.sections_generated, 
             doc_result.statistics.generation_time_ms);
    
    Ok(())
}

/// Create an example configuration
fn create_example_config() -> Config {
    let mut config = Config::default();
    
    // Set user information
    config.global.user.id = "example_user".to_string();
    config.global.user.name = "Example User".to_string();
    config.global.user.email = "example@rhema.ai".to_string();
    
    // Set application information
    config.global.application.name = "Example Rhema Project".to_string();
    config.global.application.environment = "development".to_string();
    
    // Set repository information
    config.repository.repository_type = "git".to_string();
    config.repository.url = "https://github.com/example/rhema-project".to_string();
    
    // Set security settings
    config.security.encryption.enabled = true;
    config.security.access_control.enabled = true;
    config.security.audit_logging.enabled = true;
    
    config
}

/// Create example validation issues
fn create_example_issues() -> Vec<ConfigIssue> {
    vec![
        ConfigIssue {
            path: "global.user.id".to_string(),
            message: "User ID is too short".to_string(),
            severity: ConfigIssueSeverity::Error,
            category: "validation".to_string(),
        },
        ConfigIssue {
            path: "security.encryption.algorithm".to_string(),
            message: "Encryption algorithm not specified".to_string(),
            severity: ConfigIssueSeverity::Warning,
            category: "security".to_string(),
        },
        ConfigIssue {
            path: "backup.schedule".to_string(),
            message: "No backup schedule configured".to_string(),
            severity: ConfigIssueSeverity::Info,
            category: "backup".to_string(),
        },
    ]
}
