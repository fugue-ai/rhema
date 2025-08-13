use rhema_core::schema::{
    AdvancedVariable, CompositionBlock, CompositionBlockType, ContextCacheConfig,
    ContextLearningConfig, ContextOptimizationConfig, ContextQualityMetrics, ContextRule,
    ContextInjectionMethod, PromptPattern, TemplateValidationRule, ValidationRuleType,
    ValidationSeverity, VariableConstraints, VariableType, VariableValidation,
};
use std::collections::HashMap;

#[test]
fn test_advanced_variable_creation() {
    let validation = VariableValidation {
        min_length: Some(3),
        max_length: Some(50),
        min_value: None,
        max_value: None,
        pattern: Some(r"^[a-zA-Z]+$".to_string()),
        custom_validator: None,
    };

    let constraints = VariableConstraints {
        allowed_values: Some(vec!["rust".to_string(), "typescript".to_string()]),
        forbidden_values: None,
        depends_on: None,
        visible_when: None,
    };

    let variable = AdvancedVariable {
        name: "language".to_string(),
        var_type: VariableType::Enum(vec!["rust".to_string(), "typescript".to_string()]),
        default_value: Some("rust".to_string()),
        validation: Some(validation),
        description: Some("Programming language".to_string()),
        required: true,
        constraints: Some(constraints),
    };

    assert_eq!(variable.name, "language");
    assert!(variable.required);
    assert_eq!(variable.default_value, Some("rust".to_string()));
    assert_eq!(variable.description, Some("Programming language".to_string()));
}

#[test]
fn test_composition_block_creation() {
    let variables = HashMap::from([
        ("security_level".to_string(), "high".to_string()),
        ("review_depth".to_string(), "comprehensive".to_string()),
    ]);

    let block = CompositionBlock {
        id: "security-checks".to_string(),
        block_type: CompositionBlockType::Conditional,
        condition: Some("security_focus == true".to_string()),
        loop_variable: None,
        loop_items: None,
        content: "Additional security checks:\n- Input validation\n- Authentication".to_string(),
        priority: Some(1),
        variables: Some(variables),
    };

    assert_eq!(block.id, "security-checks");
    assert_eq!(block.block_type, CompositionBlockType::Conditional);
    assert_eq!(block.condition, Some("security_focus == true".to_string()));
    assert_eq!(block.priority, Some(1));
    assert!(block.variables.is_some());
}

#[test]
fn test_validation_rule_creation() {
    let rule = TemplateValidationRule {
        id: "require-language".to_string(),
        rule_type: ValidationRuleType::Required,
        condition: "language IS NULL".to_string(),
        error_message: "Language is required".to_string(),
        severity: ValidationSeverity::Error,
        enabled: true,
    };

    assert_eq!(rule.id, "require-language");
    assert_eq!(rule.rule_type, ValidationRuleType::Required);
    assert_eq!(rule.severity, ValidationSeverity::Error);
    assert!(rule.enabled);
}

#[test]
fn test_context_rule_creation() {
    let variables = HashMap::from([
        ("security_level".to_string(), "high".to_string()),
        ("review_depth".to_string(), "comprehensive".to_string()),
    ]);

    let rule = ContextRule {
        condition: "task_type == 'security_review'".to_string(),
        context_files: vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
        injection_method: ContextInjectionMethod::Prepend,
        priority: Some(5),
        variables: Some(variables),
    };

    assert_eq!(rule.condition, "task_type == 'security_review'");
    assert_eq!(rule.context_files.len(), 2);
    assert_eq!(rule.injection_method, ContextInjectionMethod::Prepend);
    assert_eq!(rule.priority, Some(5));
    assert!(rule.variables.is_some());
}

#[test]
fn test_context_cache_config() {
    let config = ContextCacheConfig {
        enabled: true,
        ttl_seconds: 3600,
        max_size_bytes: 1024 * 1024, // 1MB
        invalidation_strategy: rhema_core::schema::CacheInvalidationStrategy::TimeBased,
        compression_enabled: true,
        persistence_enabled: true,
    };

    assert!(config.enabled);
    assert_eq!(config.ttl_seconds, 3600);
    assert_eq!(config.max_size_bytes, 1024 * 1024);
    assert!(config.compression_enabled);
    assert!(config.persistence_enabled);
}

#[test]
fn test_context_optimization_config() {
    let config = ContextOptimizationConfig {
        enabled: true,
        max_tokens: 4096,
        min_relevance_score: 0.7,
        semantic_compression: true,
        structure_optimization: true,
        relevance_filtering: true,
        algorithm: rhema_core::schema::OptimizationAlgorithm::Hybrid,
    };

    assert!(config.enabled);
    assert_eq!(config.max_tokens, 4096);
    assert_eq!(config.min_relevance_score, 0.7);
    assert!(config.semantic_compression);
    assert!(config.structure_optimization);
    assert!(config.relevance_filtering);
}

#[test]
fn test_context_learning_config() {
    let config = ContextLearningConfig {
        enabled: true,
        learning_rate: 0.1,
        min_sample_size: 10,
        window_size: 100,
        feedback_weight: 0.5,
        success_threshold: 0.8,
        algorithm: rhema_core::schema::LearningAlgorithm::Reinforcement,
    };

    assert!(config.enabled);
    assert_eq!(config.learning_rate, 0.1);
    assert_eq!(config.min_sample_size, 10);
    assert_eq!(config.window_size, 100);
    assert_eq!(config.feedback_weight, 0.5);
    assert_eq!(config.success_threshold, 0.8);
}

#[test]
fn test_context_quality_metrics() {
    let metrics = ContextQualityMetrics {
        relevance_score: 0.85,
        completeness_score: 0.90,
        accuracy_score: 0.88,
        timeliness_score: 0.92,
        overall_score: 0.89,
        assessed_at: chrono::Utc::now(),
        improvement_suggestions: vec![
            "Add more security patterns".to_string(),
            "Include performance benchmarks".to_string(),
        ],
    };

    assert_eq!(metrics.relevance_score, 0.85);
    assert_eq!(metrics.completeness_score, 0.90);
    assert_eq!(metrics.accuracy_score, 0.88);
    assert_eq!(metrics.timeliness_score, 0.92);
    assert_eq!(metrics.overall_score, 0.89);
    assert_eq!(metrics.improvement_suggestions.len(), 2);
}

#[test]
fn test_prompt_pattern_with_advanced_features() {
    let mut pattern = PromptPattern::new(
        "advanced-review",
        "Advanced Code Review",
        "Review this {{LANGUAGE}} code: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Add advanced variables
    let variable = AdvancedVariable {
        name: "language".to_string(),
        var_type: VariableType::Enum(vec!["rust".to_string(), "typescript".to_string()]),
        default_value: Some("rust".to_string()),
        validation: None,
        description: Some("Programming language".to_string()),
        required: true,
        constraints: None,
    };
    pattern.add_advanced_variable(variable);

    // Add context rules
    let context_rule = ContextRule {
        condition: "task_type == 'security_review'".to_string(),
        context_files: vec!["patterns.yaml".to_string(), "knowledge.yaml".to_string()],
        injection_method: ContextInjectionMethod::Prepend,
        priority: Some(5),
        variables: None,
    };
    pattern.add_context_rule(context_rule);

    // Add composition blocks
    let composition_block = CompositionBlock {
        id: "security-checks".to_string(),
        block_type: CompositionBlockType::Conditional,
        condition: Some("security_focus == true".to_string()),
        loop_variable: None,
        loop_items: None,
        content: "Additional security checks".to_string(),
        priority: Some(1),
        variables: None,
    };
    pattern.add_composition_block(composition_block);

    // Add validation rules
    let validation_rule = TemplateValidationRule {
        id: "require-language".to_string(),
        rule_type: ValidationRuleType::Required,
        condition: "language IS NULL".to_string(),
        error_message: "Language is required".to_string(),
        severity: ValidationSeverity::Error,
        enabled: true,
    };
    pattern.add_validation_rule(validation_rule);

    // Configure cache
    let cache_config = ContextCacheConfig {
        enabled: true,
        ttl_seconds: 3600,
        max_size_bytes: 1024 * 1024,
        invalidation_strategy: rhema_core::schema::CacheInvalidationStrategy::TimeBased,
        compression_enabled: true,
        persistence_enabled: true,
    };
    pattern.configure_context_cache(cache_config);

    // Configure optimization
    let optimization_config = ContextOptimizationConfig {
        enabled: true,
        max_tokens: 4096,
        min_relevance_score: 0.7,
        semantic_compression: true,
        structure_optimization: true,
        relevance_filtering: true,
        algorithm: rhema_core::schema::OptimizationAlgorithm::Hybrid,
    };
    pattern.configure_context_optimization(optimization_config);

    // Configure learning
    let learning_config = ContextLearningConfig {
        enabled: true,
        learning_rate: 0.1,
        min_sample_size: 10,
        window_size: 100,
        feedback_weight: 0.5,
        success_threshold: 0.8,
        algorithm: rhema_core::schema::LearningAlgorithm::Reinforcement,
    };
    pattern.configure_context_learning(learning_config);

    // Update quality metrics
    let quality_metrics = ContextQualityMetrics {
        relevance_score: 0.85,
        completeness_score: 0.90,
        accuracy_score: 0.88,
        timeliness_score: 0.92,
        overall_score: 0.89,
        assessed_at: chrono::Utc::now(),
        improvement_suggestions: vec!["Add more security patterns".to_string()],
    };
    pattern.update_context_quality(quality_metrics);

    // Verify all features are present
    assert_eq!(pattern.name, "Advanced Code Review");
    assert!(pattern.advanced_variables.is_some());
    assert_eq!(pattern.advanced_variables.as_ref().unwrap().len(), 1);
    assert!(pattern.context_rules.is_some());
    assert_eq!(pattern.context_rules.as_ref().unwrap().len(), 1);
    assert!(pattern.composition_blocks.is_some());
    assert_eq!(pattern.composition_blocks.as_ref().unwrap().len(), 1);
    assert!(pattern.validation_rules.is_some());
    assert_eq!(pattern.validation_rules.as_ref().unwrap().len(), 1);
    assert!(pattern.context_cache.is_some());
    assert!(pattern.context_optimization.is_some());
    assert!(pattern.context_learning.is_some());
    assert!(pattern.context_quality.is_some());
}

#[test]
fn test_variable_type_parsing() {
    // Test enum type parsing
    let enum_type = VariableType::Enum(vec!["rust".to_string(), "typescript".to_string()]);
    match enum_type {
        VariableType::Enum(values) => {
            assert_eq!(values.len(), 2);
            assert!(values.contains(&"rust".to_string()));
            assert!(values.contains(&"typescript".to_string()));
        }
        _ => panic!("Expected enum type"),
    }

    // Test basic types
    assert_eq!(format!("{:?}", VariableType::String), "String");
    assert_eq!(format!("{:?}", VariableType::Number), "Number");
    assert_eq!(format!("{:?}", VariableType::Boolean), "Boolean");
    assert_eq!(format!("{:?}", VariableType::Array), "Array");
    assert_eq!(format!("{:?}", VariableType::Object), "Object");
}

#[test]
fn test_composition_block_type_parsing() {
    assert_eq!(format!("{:?}", CompositionBlockType::Conditional), "Conditional");
    assert_eq!(format!("{:?}", CompositionBlockType::Loop), "Loop");
    assert_eq!(format!("{:?}", CompositionBlockType::Include), "Include");
    assert_eq!(format!("{:?}", CompositionBlockType::Switch), "Switch");
    assert_eq!(format!("{:?}", CompositionBlockType::Fallback), "Fallback");
}

#[test]
fn test_validation_rule_type_parsing() {
    assert_eq!(format!("{:?}", ValidationRuleType::Required), "Required");
    assert_eq!(format!("{:?}", ValidationRuleType::Format), "Format");
    assert_eq!(format!("{:?}", ValidationRuleType::Length), "Length");
    assert_eq!(format!("{:?}", ValidationRuleType::Range), "Range");
    assert_eq!(format!("{:?}", ValidationRuleType::Custom), "Custom");
    assert_eq!(format!("{:?}", ValidationRuleType::Dependency), "Dependency");
    assert_eq!(format!("{:?}", ValidationRuleType::Consistency), "Consistency");
}

#[test]
fn test_validation_severity_parsing() {
    assert_eq!(format!("{:?}", ValidationSeverity::Error), "Error");
    assert_eq!(format!("{:?}", ValidationSeverity::Warning), "Warning");
    assert_eq!(format!("{:?}", ValidationSeverity::Info), "Info");
}

#[test]
fn test_context_injection_method_parsing() {
    assert_eq!(format!("{:?}", ContextInjectionMethod::Prepend), "Prepend");
    assert_eq!(format!("{:?}", ContextInjectionMethod::Append), "Append");
    assert_eq!(format!("{:?}", ContextInjectionMethod::TemplateVariable), "TemplateVariable");
}

#[test]
fn test_prompt_pattern_serialization() {
    let mut pattern = PromptPattern::new(
        "test-pattern",
        "Test Pattern",
        "Test template: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Add some advanced features
    let variable = AdvancedVariable {
        name: "test_var".to_string(),
        var_type: VariableType::String,
        default_value: Some("default".to_string()),
        validation: None,
        description: Some("Test variable".to_string()),
        required: false,
        constraints: None,
    };
    pattern.add_advanced_variable(variable);

    // Test serialization
    let serialized = serde_yaml::to_string(&pattern).expect("Failed to serialize");
    let deserialized: PromptPattern = serde_yaml::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(deserialized.id, pattern.id);
    assert_eq!(deserialized.name, pattern.name);
    assert_eq!(deserialized.template, pattern.template);
    assert!(deserialized.advanced_variables.is_some());
    assert_eq!(deserialized.advanced_variables.as_ref().unwrap().len(), 1);
}

#[test]
fn test_prompt_pattern_validation() {
    let mut pattern = PromptPattern::new(
        "valid-pattern",
        "Valid Pattern",
        "Valid template: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Add validation rules
    let validation_rule = TemplateValidationRule {
        id: "test-rule".to_string(),
        rule_type: ValidationRuleType::Required,
        condition: "test_var IS NULL".to_string(),
        error_message: "Test variable is required".to_string(),
        severity: ValidationSeverity::Error,
        enabled: true,
    };
    pattern.add_validation_rule(validation_rule);

    // Test validation
    let context = "Test context";
    let variables = HashMap::new();
    let errors = pattern.validate_template(context, &variables);

    // Should have validation errors since test_var is not provided
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.contains("Test variable is required")));
}

#[test]
fn test_prompt_pattern_rendering() {
    let mut pattern = PromptPattern::new(
        "render-test",
        "Render Test",
        "Hello {{NAME}}, here is your code: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Add variables
    let mut variables = HashMap::new();
    variables.insert("NAME".to_string(), "Alice".to_string());

    let context = "function hello() { console.log('Hello World'); }";
    let rendered = pattern.render_with_composition(context, &variables);

    assert!(rendered.contains("Hello Alice"));
    assert!(rendered.contains("function hello()"));
}

#[test]
fn test_prompt_pattern_usage_tracking() {
    let mut pattern = PromptPattern::new(
        "usage-test",
        "Usage Test",
        "Test template: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Record some usage
    pattern.record_usage(true, Some("Great template!".to_string()));
    pattern.record_usage(true, Some("Works well".to_string()));
    pattern.record_usage(false, Some("Too verbose".to_string()));

    assert_eq!(pattern.total_uses(), 3);
    assert_eq!(pattern.successful_uses(), 2);
    assert_eq!(pattern.success_rate(), 2.0 / 3.0);

    let feedback_history = pattern.get_feedback_history();
    assert_eq!(feedback_history.len(), 3);
    assert!(feedback_history.iter().any(|f| f.feedback.contains("Great template!")));
    assert!(feedback_history.iter().any(|f| f.feedback.contains("Too verbose")));
}

#[test]
fn test_prompt_pattern_versioning() {
    let mut pattern = PromptPattern::new(
        "version-test",
        "Version Test",
        "Version 1: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Create a new version
    pattern.version.create_version(
        "2.0.0",
        "Version 2: {{CONTEXT}}",
        "Enhanced template",
        vec!["Added new features".to_string(), "Improved formatting".to_string()],
        Some("developer".to_string()),
    );

    assert_eq!(pattern.version.current, "2.0.0");
    assert_eq!(pattern.version.history.len(), 2);

    let latest = pattern.version.get_latest().expect("Should have latest version");
    assert_eq!(latest.version, "2.0.0");
    assert_eq!(latest.template, "Version 2: {{CONTEXT}}");
    assert_eq!(latest.author, Some("developer".to_string()));
}

#[test]
fn test_prompt_pattern_inheritance() {
    let mut base_pattern = PromptPattern::new(
        "base-pattern",
        "Base Pattern",
        "Base template: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    let mut extended_pattern = PromptPattern::new(
        "extended-pattern",
        "Extended Pattern",
        "{{BASE_TEMPLATE}}\nExtended content: {{EXTRA_CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    extended_pattern.set_extends("base-pattern");

    assert!(extended_pattern.has_extends());
    assert_eq!(extended_pattern.extends, Some("base-pattern".to_string()));
}

#[test]
fn test_prompt_pattern_context_rules() {
    let mut pattern = PromptPattern::new(
        "context-test",
        "Context Test",
        "Test template: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Add context rules
    let rule1 = ContextRule {
        condition: "task_type == 'security'".to_string(),
        context_files: vec!["security.yaml".to_string()],
        injection_method: ContextInjectionMethod::Prepend,
        priority: Some(5),
        variables: None,
    };

    let rule2 = ContextRule {
        condition: "task_type == 'performance'".to_string(),
        context_files: vec!["performance.yaml".to_string()],
        injection_method: ContextInjectionMethod::Append,
        priority: Some(3),
        variables: None,
    };

    pattern.add_context_rule(rule1);
    pattern.add_context_rule(rule2);

    // Test rule matching
    let matching_rules = pattern.get_matching_rules(Some("security"), None, None);
    assert_eq!(matching_rules.len(), 1);
    assert_eq!(matching_rules[0].condition, "task_type == 'security'");

    let matching_rules = pattern.get_matching_rules(Some("performance"), None, None);
    assert_eq!(matching_rules.len(), 1);
    assert_eq!(matching_rules[0].condition, "task_type == 'performance'");

    // Test context files collection
    let context_files = pattern.get_context_files(Some("security"), None, None);
    assert_eq!(context_files.len(), 1);
    assert_eq!(context_files[0], "security.yaml");
}

#[test]
fn test_prompt_pattern_composition_blocks() {
    let mut pattern = PromptPattern::new(
        "composition-test",
        "Composition Test",
        "Base template: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Add composition blocks
    let conditional_block = CompositionBlock {
        id: "conditional".to_string(),
        block_type: CompositionBlockType::Conditional,
        condition: Some("show_extra == true".to_string()),
        loop_variable: None,
        loop_items: None,
        content: "Extra content here".to_string(),
        priority: Some(1),
        variables: None,
    };

    let loop_block = CompositionBlock {
        id: "loop".to_string(),
        block_type: CompositionBlockType::Loop,
        condition: None,
        loop_variable: Some("item".to_string()),
        loop_items: Some(vec!["item1".to_string(), "item2".to_string()]),
        content: "Processing {{item}}".to_string(),
        priority: Some(2),
        variables: None,
    };

    pattern.add_composition_block(conditional_block);
    pattern.add_composition_block(loop_block);

    // Test block filtering
    let conditional_blocks = pattern.get_composition_blocks(Some(CompositionBlockType::Conditional));
    assert_eq!(conditional_blocks.len(), 1);
    assert_eq!(conditional_blocks[0].id, "conditional");

    let loop_blocks = pattern.get_composition_blocks(Some(CompositionBlockType::Loop));
    assert_eq!(loop_blocks.len(), 1);
    assert_eq!(loop_blocks[0].id, "loop");

    let all_blocks = pattern.get_composition_blocks(None);
    assert_eq!(all_blocks.len(), 2);
}

#[test]
fn test_prompt_pattern_advanced_variables() {
    let mut pattern = PromptPattern::new(
        "variables-test",
        "Variables Test",
        "Test template: {{CONTEXT}}",
        rhema_core::schema::PromptInjectionMethod::TemplateVariable,
    );

    // Add advanced variables
    let string_var = AdvancedVariable {
        name: "string_var".to_string(),
        var_type: VariableType::String,
        default_value: Some("default".to_string()),
        validation: None,
        description: Some("String variable".to_string()),
        required: false,
        constraints: None,
    };

    let enum_var = AdvancedVariable {
        name: "enum_var".to_string(),
        var_type: VariableType::Enum(vec!["option1".to_string(), "option2".to_string()]),
        default_value: Some("option1".to_string()),
        validation: None,
        description: Some("Enum variable".to_string()),
        required: true,
        constraints: None,
    };

    pattern.add_advanced_variable(string_var);
    pattern.add_advanced_variable(enum_var);

    // Test variable retrieval
    let string_var = pattern.get_advanced_variable("string_var");
    assert!(string_var.is_some());
    assert_eq!(string_var.unwrap().name, "string_var");
    assert_eq!(string_var.unwrap().var_type, VariableType::String);

    let enum_var = pattern.get_advanced_variable("enum_var");
    assert!(enum_var.is_some());
    assert_eq!(enum_var.unwrap().name, "enum_var");
    assert!(enum_var.unwrap().required);

    // Test variable validation
    let mut values = HashMap::new();
    values.insert("string_var".to_string(), "test".to_string());
    values.insert("enum_var".to_string(), "option1".to_string());

    let errors = pattern.validate_advanced_variables(&values);
    assert!(errors.is_empty());

    // Test validation with missing required variable
    let mut values = HashMap::new();
    values.insert("string_var".to_string(), "test".to_string());
    // Missing enum_var

    let errors = pattern.validate_advanced_variables(&values);
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.contains("enum_var")));
}
