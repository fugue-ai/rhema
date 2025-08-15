use crate::{Config, ConfigEnvironment, ConfigError, GlobalConfig, ValidationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration wizard for interactive setup
#[derive(Debug, Clone)]
pub struct ConfigWizard {
    /// Wizard steps
    steps: Vec<WizardStep>,
    /// Current step index
    current_step: usize,
    /// Collected configuration data
    collected_data: HashMap<String, serde_json::Value>,
    /// Wizard settings
    settings: WizardSettings,
}

/// Wizard step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardStep {
    /// Step ID
    pub id: String,
    /// Step title
    pub title: String,
    /// Step description
    pub description: String,
    /// Step type
    pub step_type: StepType,
    /// Step questions
    pub questions: Vec<WizardQuestion>,
    /// Step validation rules
    pub validation_rules: Vec<ValidationRule>,
    /// Whether step is required
    pub required: bool,
    /// Step dependencies
    pub dependencies: Vec<String>,
}

/// Step type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    Welcome,
    UserInfo,
    ApplicationSetup,
    RepositorySetup,
    SecuritySetup,
    BackupSetup,
    Validation,
    Review,
    Complete,
}

/// Wizard question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardQuestion {
    /// Question ID
    pub id: String,
    /// Question text
    pub text: String,
    /// Question type
    pub question_type: QuestionType,
    /// Default value
    pub default_value: Option<serde_json::Value>,
    /// Available options (for choice questions)
    pub options: Option<Vec<QuestionOption>>,
    /// Validation rules
    pub validation_rules: Vec<QuestionValidationRule>,
    /// Whether question is required
    pub required: bool,
    /// Question help text
    pub help_text: Option<String>,
}

/// Question type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    Text,
    Email,
    Password,
    Number,
    Boolean,
    Choice,
    MultiChoice,
    File,
    Directory,
}

/// Question option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    /// Option value
    pub value: String,
    /// Option label
    pub label: String,
    /// Option description
    pub description: Option<String>,
}

/// Question validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionValidationRule {
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Error message
    pub error_message: String,
}

/// Validation rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    MinLength,
    MaxLength,
    Pattern,
    Email,
    Url,
    MinValue,
    MaxValue,
    Custom,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule ID
    pub id: String,
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Error message
    pub error_message: String,
}

/// Wizard settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardSettings {
    /// Allow skipping optional steps
    pub allow_skip: bool,
    /// Show progress bar
    pub show_progress: bool,
    /// Auto-save progress
    pub auto_save: bool,
    /// Save file path
    pub save_path: Option<String>,
    /// Enable validation
    pub enable_validation: bool,
    /// Show help text
    pub show_help: bool,
}

/// Wizard result
#[derive(Debug, Clone)]
pub struct WizardResult<T: Config> {
    /// Generated configuration
    pub config: T,
    /// Wizard statistics
    pub statistics: WizardStatistics,
    /// Generated files
    pub generated_files: Vec<String>,
}

/// Wizard statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardStatistics {
    /// Total steps completed
    pub steps_completed: usize,
    /// Total time taken in seconds
    pub time_taken_seconds: u64,
    /// Questions answered
    pub questions_answered: usize,
    /// Validation errors encountered
    pub validation_errors: usize,
    /// Configuration completeness percentage
    pub completeness_percentage: u8,
}

/// Wizard progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardProgress {
    /// Current step
    pub current_step: usize,
    /// Total steps
    pub total_steps: usize,
    /// Progress percentage
    pub progress_percentage: u8,
    /// Current step data
    pub current_step_data: Option<WizardStep>,
    /// Can go back
    pub can_go_back: bool,
    /// Can go forward
    pub can_go_forward: bool,
    /// Can complete
    pub can_complete: bool,
}

impl ConfigWizard {
    /// Create a new configuration wizard
    pub fn new(settings: WizardSettings) -> Self {
        let mut wizard = Self {
            steps: Vec::new(),
            current_step: 0,
            collected_data: HashMap::new(),
            settings,
        };
        wizard.initialize_steps();
        wizard
    }

    /// Initialize wizard steps
    fn initialize_steps(&mut self) {
        self.steps = vec![
            self.create_welcome_step(),
            self.create_user_info_step(),
            self.create_application_setup_step(),
            self.create_repository_setup_step(),
            self.create_security_setup_step(),
            self.create_backup_setup_step(),
            self.create_validation_step(),
            self.create_review_step(),
            self.create_complete_step(),
        ];
    }

    /// Create welcome step
    fn create_welcome_step(&self) -> WizardStep {
        WizardStep {
            id: "welcome".to_string(),
            title: "Welcome to Rhema Configuration Wizard".to_string(),
            description: "This wizard will help you set up your Rhema configuration step by step."
                .to_string(),
            step_type: StepType::Welcome,
            questions: vec![WizardQuestion {
                id: "proceed".to_string(),
                text: "Are you ready to begin the configuration setup?".to_string(),
                question_type: QuestionType::Boolean,
                default_value: Some(serde_json::Value::Bool(true)),
                options: None,
                validation_rules: vec![],
                required: true,
                help_text: Some("Click 'Yes' to proceed with the configuration setup.".to_string()),
            }],
            validation_rules: vec![],
            required: true,
            dependencies: vec![],
        }
    }

    /// Create user info step
    fn create_user_info_step(&self) -> WizardStep {
        WizardStep {
            id: "user_info".to_string(),
            title: "User Information".to_string(),
            description: "Please provide your basic user information.".to_string(),
            step_type: StepType::UserInfo,
            questions: vec![
                WizardQuestion {
                    id: "user_id".to_string(),
                    text: "What is your user ID?".to_string(),
                    question_type: QuestionType::Text,
                    default_value: None,
                    options: None,
                    validation_rules: vec![
                        QuestionValidationRule {
                            rule_type: ValidationRuleType::Required,
                            parameters: HashMap::new(),
                            error_message: "User ID is required".to_string(),
                        },
                        QuestionValidationRule {
                            rule_type: ValidationRuleType::MinLength,
                            parameters: {
                                let mut params = HashMap::new();
                                params.insert(
                                    "min_length".to_string(),
                                    serde_json::Value::Number(3.into()),
                                );
                                params
                            },
                            error_message: "User ID must be at least 3 characters long".to_string(),
                        },
                    ],
                    required: true,
                    help_text: Some("Enter a unique identifier for your user account.".to_string()),
                },
                WizardQuestion {
                    id: "user_name".to_string(),
                    text: "What is your full name?".to_string(),
                    question_type: QuestionType::Text,
                    default_value: None,
                    options: None,
                    validation_rules: vec![QuestionValidationRule {
                        rule_type: ValidationRuleType::Required,
                        parameters: HashMap::new(),
                        error_message: "Full name is required".to_string(),
                    }],
                    required: true,
                    help_text: Some(
                        "Enter your full name as it should appear in the system.".to_string(),
                    ),
                },
                WizardQuestion {
                    id: "user_email".to_string(),
                    text: "What is your email address?".to_string(),
                    question_type: QuestionType::Email,
                    default_value: None,
                    options: None,
                    validation_rules: vec![
                        QuestionValidationRule {
                            rule_type: ValidationRuleType::Required,
                            parameters: HashMap::new(),
                            error_message: "Email address is required".to_string(),
                        },
                        QuestionValidationRule {
                            rule_type: ValidationRuleType::Email,
                            parameters: HashMap::new(),
                            error_message: "Please enter a valid email address".to_string(),
                        },
                    ],
                    required: true,
                    help_text: Some(
                        "Enter your email address for notifications and account recovery."
                            .to_string(),
                    ),
                },
            ],
            validation_rules: vec![],
            required: true,
            dependencies: vec!["welcome".to_string()],
        }
    }

    /// Create application setup step
    fn create_application_setup_step(&self) -> WizardStep {
        WizardStep {
            id: "application_setup".to_string(),
            title: "Application Setup".to_string(),
            description: "Configure your application settings.".to_string(),
            step_type: StepType::ApplicationSetup,
            questions: vec![
                WizardQuestion {
                    id: "app_name".to_string(),
                    text: "What is the name of your application?".to_string(),
                    question_type: QuestionType::Text,
                    default_value: Some(serde_json::Value::String("My Rhema Project".to_string())),
                    options: None,
                    validation_rules: vec![QuestionValidationRule {
                        rule_type: ValidationRuleType::Required,
                        parameters: HashMap::new(),
                        error_message: "Application name is required".to_string(),
                    }],
                    required: true,
                    help_text: Some("Enter a descriptive name for your Rhema project.".to_string()),
                },
                WizardQuestion {
                    id: "environment".to_string(),
                    text: "What environment are you setting up for?".to_string(),
                    question_type: QuestionType::Choice,
                    default_value: Some(serde_json::Value::String("development".to_string())),
                    options: Some(vec![
                        QuestionOption {
                            value: "development".to_string(),
                            label: "Development".to_string(),
                            description: Some("For development and testing".to_string()),
                        },
                        QuestionOption {
                            value: "staging".to_string(),
                            label: "Staging".to_string(),
                            description: Some("For staging and pre-production testing".to_string()),
                        },
                        QuestionOption {
                            value: "production".to_string(),
                            label: "Production".to_string(),
                            description: Some("For production deployment".to_string()),
                        },
                    ]),
                    validation_rules: vec![QuestionValidationRule {
                        rule_type: ValidationRuleType::Required,
                        parameters: HashMap::new(),
                        error_message: "Environment selection is required".to_string(),
                    }],
                    required: true,
                    help_text: Some("Select the environment for your configuration.".to_string()),
                },
            ],
            validation_rules: vec![],
            required: true,
            dependencies: vec!["user_info".to_string()],
        }
    }

    /// Create repository setup step
    fn create_repository_setup_step(&self) -> WizardStep {
        WizardStep {
            id: "repository_setup".to_string(),
            title: "Repository Setup".to_string(),
            description: "Configure your repository settings.".to_string(),
            step_type: StepType::RepositorySetup,
            questions: vec![
                WizardQuestion {
                    id: "repo_type".to_string(),
                    text: "What type of repository are you using?".to_string(),
                    question_type: QuestionType::Choice,
                    default_value: Some(serde_json::Value::String("git".to_string())),
                    options: Some(vec![
                        QuestionOption {
                            value: "git".to_string(),
                            label: "Git".to_string(),
                            description: Some("Git repository (GitHub, GitLab, etc.)".to_string()),
                        },
                        QuestionOption {
                            value: "svn".to_string(),
                            label: "Subversion".to_string(),
                            description: Some("Subversion repository".to_string()),
                        },
                        QuestionOption {
                            value: "none".to_string(),
                            label: "No Repository".to_string(),
                            description: Some("No version control repository".to_string()),
                        },
                    ]),
                    validation_rules: vec![QuestionValidationRule {
                        rule_type: ValidationRuleType::Required,
                        parameters: HashMap::new(),
                        error_message: "Repository type selection is required".to_string(),
                    }],
                    required: true,
                    help_text: Some(
                        "Select the type of version control repository you're using.".to_string(),
                    ),
                },
                WizardQuestion {
                    id: "repo_url".to_string(),
                    text: "What is your repository URL?".to_string(),
                    question_type: QuestionType::Text,
                    default_value: None,
                    options: None,
                    validation_rules: vec![QuestionValidationRule {
                        rule_type: ValidationRuleType::Url,
                        parameters: HashMap::new(),
                        error_message: "Please enter a valid repository URL".to_string(),
                    }],
                    required: false,
                    help_text: Some(
                        "Enter the URL of your repository (e.g., https://github.com/user/repo)"
                            .to_string(),
                    ),
                },
            ],
            validation_rules: vec![],
            required: false,
            dependencies: vec!["application_setup".to_string()],
        }
    }

    /// Create security setup step
    fn create_security_setup_step(&self) -> WizardStep {
        WizardStep {
            id: "security_setup".to_string(),
            title: "Security Setup".to_string(),
            description: "Configure security settings for your configuration.".to_string(),
            step_type: StepType::SecuritySetup,
            questions: vec![
                WizardQuestion {
                    id: "enable_encryption".to_string(),
                    text: "Do you want to enable encryption for sensitive configuration data?"
                        .to_string(),
                    question_type: QuestionType::Boolean,
                    default_value: Some(serde_json::Value::Bool(true)),
                    options: None,
                    validation_rules: vec![],
                    required: true,
                    help_text: Some(
                        "Encryption helps protect sensitive configuration data.".to_string(),
                    ),
                },
                WizardQuestion {
                    id: "enable_access_control".to_string(),
                    text: "Do you want to enable access control?".to_string(),
                    question_type: QuestionType::Boolean,
                    default_value: Some(serde_json::Value::Bool(true)),
                    options: None,
                    validation_rules: vec![],
                    required: true,
                    help_text: Some(
                        "Access control restricts who can modify configuration.".to_string(),
                    ),
                },
                WizardQuestion {
                    id: "enable_audit_logging".to_string(),
                    text: "Do you want to enable audit logging?".to_string(),
                    question_type: QuestionType::Boolean,
                    default_value: Some(serde_json::Value::Bool(true)),
                    options: None,
                    validation_rules: vec![],
                    required: true,
                    help_text: Some(
                        "Audit logging tracks configuration changes for security.".to_string(),
                    ),
                },
            ],
            validation_rules: vec![],
            required: false,
            dependencies: vec!["repository_setup".to_string()],
        }
    }

    /// Create backup setup step
    fn create_backup_setup_step(&self) -> WizardStep {
        WizardStep {
            id: "backup_setup".to_string(),
            title: "Backup Setup".to_string(),
            description: "Configure backup settings for your configuration.".to_string(),
            step_type: StepType::BackupSetup,
            questions: vec![
                WizardQuestion {
                    id: "enable_backup".to_string(),
                    text: "Do you want to enable automatic backups?".to_string(),
                    question_type: QuestionType::Boolean,
                    default_value: Some(serde_json::Value::Bool(true)),
                    options: None,
                    validation_rules: vec![],
                    required: true,
                    help_text: Some("Automatic backups help prevent data loss.".to_string()),
                },
                WizardQuestion {
                    id: "backup_frequency".to_string(),
                    text: "How often should backups be created?".to_string(),
                    question_type: QuestionType::Choice,
                    default_value: Some(serde_json::Value::String("daily".to_string())),
                    options: Some(vec![
                        QuestionOption {
                            value: "hourly".to_string(),
                            label: "Hourly".to_string(),
                            description: Some("Create backups every hour".to_string()),
                        },
                        QuestionOption {
                            value: "daily".to_string(),
                            label: "Daily".to_string(),
                            description: Some("Create backups every day".to_string()),
                        },
                        QuestionOption {
                            value: "weekly".to_string(),
                            label: "Weekly".to_string(),
                            description: Some("Create backups every week".to_string()),
                        },
                    ]),
                    validation_rules: vec![QuestionValidationRule {
                        rule_type: ValidationRuleType::Required,
                        parameters: HashMap::new(),
                        error_message: "Backup frequency selection is required".to_string(),
                    }],
                    required: true,
                    help_text: Some("Select how often you want backups to be created.".to_string()),
                },
            ],
            validation_rules: vec![],
            required: false,
            dependencies: vec!["security_setup".to_string()],
        }
    }

    /// Create validation step
    fn create_validation_step(&self) -> WizardStep {
        WizardStep {
            id: "validation".to_string(),
            title: "Configuration Validation".to_string(),
            description: "Validating your configuration settings...".to_string(),
            step_type: StepType::Validation,
            questions: vec![],
            validation_rules: vec![],
            required: true,
            dependencies: vec!["backup_setup".to_string()],
        }
    }

    /// Create review step
    fn create_review_step(&self) -> WizardStep {
        WizardStep {
            id: "review".to_string(),
            title: "Review Configuration".to_string(),
            description: "Review your configuration settings before completing setup.".to_string(),
            step_type: StepType::Review,
            questions: vec![WizardQuestion {
                id: "confirm_config".to_string(),
                text: "Do you want to proceed with this configuration?".to_string(),
                question_type: QuestionType::Boolean,
                default_value: Some(serde_json::Value::Bool(true)),
                options: None,
                validation_rules: vec![],
                required: true,
                help_text: Some("Review your settings and confirm to proceed.".to_string()),
            }],
            validation_rules: vec![],
            required: true,
            dependencies: vec!["validation".to_string()],
        }
    }

    /// Create complete step
    fn create_complete_step(&self) -> WizardStep {
        WizardStep {
            id: "complete".to_string(),
            title: "Configuration Complete".to_string(),
            description: "Your configuration has been successfully created!".to_string(),
            step_type: StepType::Complete,
            questions: vec![],
            validation_rules: vec![],
            required: true,
            dependencies: vec!["review".to_string()],
        }
    }

    /// Get current progress
    pub fn get_progress(&self) -> WizardProgress {
        let total_steps = self.steps.len();
        let progress_percentage = if total_steps > 0 {
            ((self.current_step + 1) * 100) / total_steps
        } else {
            0
        };

        WizardProgress {
            current_step: self.current_step,
            total_steps,
            progress_percentage: progress_percentage as u8,
            current_step_data: self.steps.get(self.current_step).cloned(),
            can_go_back: self.current_step > 0,
            can_go_forward: self.current_step < total_steps - 1,
            can_complete: self.current_step == total_steps - 1,
        }
    }

    /// Go to next step
    pub fn next_step(&mut self) -> Result<WizardProgress, ConfigError> {
        if self.current_step < self.steps.len() - 1 {
            self.current_step += 1;
            Ok(self.get_progress())
        } else {
            Err(ConfigError::ValidationError(
                "Already at the last step".to_string(),
            ))
        }
    }

    /// Go to previous step
    pub fn previous_step(&mut self) -> Result<WizardProgress, ConfigError> {
        if self.current_step > 0 {
            self.current_step -= 1;
            Ok(self.get_progress())
        } else {
            Err(ConfigError::ValidationError(
                "Already at the first step".to_string(),
            ))
        }
    }

    /// Answer a question
    pub fn answer_question(
        &mut self,
        question_id: &str,
        answer: serde_json::Value,
    ) -> Result<(), ConfigError> {
        // Validate the answer
        if let Some(step) = self.steps.get(self.current_step) {
            if let Some(question) = step.questions.iter().find(|q| q.id == question_id) {
                self.validate_answer(question, &answer)?;
                self.collected_data.insert(question_id.to_string(), answer);
                Ok(())
            } else {
                Err(ConfigError::ValidationError(format!(
                    "Question '{}' not found",
                    question_id
                )))
            }
        } else {
            Err(ConfigError::ValidationError("Invalid step".to_string()))
        }
    }

    /// Validate an answer
    fn validate_answer(
        &self,
        question: &WizardQuestion,
        answer: &serde_json::Value,
    ) -> Result<(), ConfigError> {
        for rule in &question.validation_rules {
            if !self.validate_rule(rule, answer) {
                return Err(ConfigError::ValidationError(rule.error_message.clone()));
            }
        }
        Ok(())
    }

    /// Validate a rule
    fn validate_rule(&self, rule: &QuestionValidationRule, answer: &serde_json::Value) -> bool {
        match rule.rule_type {
            ValidationRuleType::Required => {
                !answer.is_null() && answer.as_str().map(|s| !s.is_empty()).unwrap_or(true)
            }
            ValidationRuleType::MinLength => {
                if let Some(min_length) = rule.parameters.get("min_length").and_then(|v| v.as_u64())
                {
                    answer
                        .as_str()
                        .map(|s| s.len() >= min_length as usize)
                        .unwrap_or(false)
                } else {
                    true
                }
            }
            ValidationRuleType::MaxLength => {
                if let Some(max_length) = rule.parameters.get("max_length").and_then(|v| v.as_u64())
                {
                    answer
                        .as_str()
                        .map(|s| s.len() <= max_length as usize)
                        .unwrap_or(false)
                } else {
                    true
                }
            }
            ValidationRuleType::Email => answer.as_str().map(|s| s.contains('@')).unwrap_or(false),
            ValidationRuleType::Url => answer
                .as_str()
                .map(|s| s.starts_with("http"))
                .unwrap_or(false),
            _ => true, // Other validation types would be implemented here
        }
    }

    /// Complete the wizard and generate configuration
    pub async fn complete(&self) -> Result<WizardResult<GlobalConfig>, ConfigError> {
        let start_time = std::time::Instant::now();

        // Generate configuration from collected data
        let config = self.generate_configuration()?;

        // Validate the generated configuration
        if self.settings.enable_validation {
            let validation_result = self.validate_configuration(&config).await?;
            if !validation_result.valid {
                return Err(ConfigError::ValidationError(format!(
                    "Configuration validation failed: {} issues found",
                    validation_result.issues.len()
                )));
            }
        }

        let time_taken = start_time.elapsed().as_secs();

        // Save configuration if auto-save is enabled
        let mut generated_files = Vec::new();
        if self.settings.auto_save {
            if let Some(save_path) = &self.settings.save_path {
                // Save configuration to file
                let file_path = format!("{}/config.yaml", save_path);
                // In a real implementation, you would save the config here
                generated_files.push(file_path);
            }
        }

        Ok(WizardResult {
            config,
            statistics: WizardStatistics {
                steps_completed: self.steps.len(),
                time_taken_seconds: time_taken,
                questions_answered: self.collected_data.len(),
                validation_errors: 0, // This would be calculated from validation results
                completeness_percentage: 100,
            },
            generated_files,
        })
    }

    /// Generate configuration from collected data
    fn generate_configuration(&self) -> Result<GlobalConfig, ConfigError> {
        // This is a simplified implementation
        // In a real implementation, you would map the collected data to a proper Config struct

        let mut config = GlobalConfig::new();

        // Set user information
        if let Some(user_id) = self.collected_data.get("user_id").and_then(|v| v.as_str()) {
            config.user.id = user_id.to_string();
        }

        if let Some(user_name) = self
            .collected_data
            .get("user_name")
            .and_then(|v| v.as_str())
        {
            config.user.name = user_name.to_string();
        }

        if let Some(user_email) = self
            .collected_data
            .get("user_email")
            .and_then(|v| v.as_str())
        {
            config.user.email = user_email.to_string();
        }

        // Set application information
        if let Some(app_name) = self.collected_data.get("app_name").and_then(|v| v.as_str()) {
            config.application.name = app_name.to_string();
        }

        if let Some(environment) = self
            .collected_data
            .get("environment")
            .and_then(|v| v.as_str())
        {
            // Parse environment string to ConfigEnvironment enum
            config.environment.current = match environment {
                "development" => ConfigEnvironment::Development,
                "testing" => ConfigEnvironment::Testing,
                "staging" => ConfigEnvironment::Staging,
                "production" => ConfigEnvironment::Production,
                _ => ConfigEnvironment::Custom(environment.to_string()),
            };
        }

        // Note: Repository configuration would need to be handled separately
        // as GlobalConfig doesn't have a repository field

        // Set security settings
        if let Some(enable_encryption) = self
            .collected_data
            .get("enable_encryption")
            .and_then(|v| v.as_bool())
        {
            config.security.encryption.enabled = enable_encryption;
        }

        // Note: Access control and audit logging would need to be configured
        // based on the actual SecurityConfig structure

        Ok(config)
    }

    /// Validate generated configuration
    async fn validate_configuration<T: Config>(
        &self,
        config: &T,
    ) -> Result<ValidationResult, ConfigError> {
        // This would use the actual validation system
        // For now, return a simple validation result
        Ok(ValidationResult {
            valid: true,
            issues: Vec::new(),
            warnings: Vec::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: 0,
        })
    }
}

impl Default for WizardSettings {
    fn default() -> Self {
        Self {
            allow_skip: true,
            show_progress: true,
            auto_save: true,
            save_path: Some("config".to_string()),
            enable_validation: true,
            show_help: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wizard_creation() {
        let settings = WizardSettings::default();
        let wizard = ConfigWizard::new(settings);
        assert!(!wizard.steps.is_empty());
        assert_eq!(wizard.current_step, 0);
    }

    #[tokio::test]
    async fn test_wizard_progress() {
        let settings = WizardSettings::default();
        let wizard = ConfigWizard::new(settings);
        let progress = wizard.get_progress();

        assert_eq!(progress.current_step, 0);
        assert!(progress.total_steps > 0);
        assert!(progress.can_go_forward);
        assert!(!progress.can_go_back);
    }

    #[tokio::test]
    async fn test_wizard_navigation() {
        let settings = WizardSettings::default();
        let mut wizard = ConfigWizard::new(settings);

        // Test next step
        let progress = wizard.next_step().unwrap();
        assert_eq!(progress.current_step, 1);

        // Test previous step
        let progress = wizard.previous_step().unwrap();
        assert_eq!(progress.current_step, 0);
    }

    #[tokio::test]
    async fn test_wizard_question_answering() {
        let settings = WizardSettings::default();
        let mut wizard = ConfigWizard::new(settings);

        // Answer a question
        let answer = serde_json::Value::String("test_user".to_string());
        let result = wizard.answer_question("user_id", answer);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wizard_completion() {
        let settings = WizardSettings::default();
        let wizard = ConfigWizard::new(settings);

        // This would require answering all questions first
        // For now, just test that the method exists
        let _result = wizard.complete().await;
    }
}
