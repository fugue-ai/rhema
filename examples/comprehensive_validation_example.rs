/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rhema_config::{
    ComprehensiveValidator, GlobalConfig, SchemaType, ValidationLevel, ValidationRulesConfig,
    ValidationRule, RuleType, RuleCondition, ConditionOperator, RuleAction, ActionType,
    ValidationRulesManager, ConfigIssueSeverity,
};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rhema Comprehensive Configuration Validation Example ===\n");

    // 1. Create a global configuration
    let global_config = GlobalConfig::default();

    // 2. Create a comprehensive validator
    let validator = ComprehensiveValidator::with_settings(
        &global_config,
        300, // cache TTL
        ValidationLevel::Complete, // validation level
        true, // auto-fix
    ).await?;

    println!("✅ Comprehensive validator created successfully");

    // 3. Create sample configuration data
    let sample_config = json!({
        "rhema": {
            "version": "1.0.0",
            "scope": {
                "type": "repository",
                "name": "example-repo"
            }
        },
        "dependencies": [
            {
                "name": "rhema-core",
                "version": "1.0.0"
            },
            {
                "name": "rhema-ai",
                "version": "1.0.0"
            }
        ],
        "knowledge": [
            {
                "title": "Project Architecture",
                "content": "This is a comprehensive validation example",
                "tags": ["architecture", "validation"]
            }
        ],
        "todos": [
            {
                "id": "TODO-001",
                "title": "Implement validation",
                "status": "in-progress",
                "dependencies": []
            }
        ],
        "decisions": [
            {
                "id": "ADR-001",
                "title": "Use comprehensive validation",
                "status": "accepted",
                "context": "Need robust validation for configuration files"
            }
        ]
    });

    println!("✅ Sample configuration created");

    // 4. Validate the configuration
    println!("\n🔍 Validating configuration...");
    let validation_result = validator
        .validate_config_value(&sample_config, &SchemaType::Rhema, Path::new("example.yaml"))
        .await?;

    println!("Validation completed:");
    println!("  - Overall valid: {}", validation_result.valid);
    println!("  - Schema valid: {}", validation_result.schema_valid);
    println!("  - Business valid: {}", validation_result.business_valid);
    println!("  - Issues found: {}", validation_result.issues.len());
    println!("  - Warnings: {}", validation_result.warnings.len());

    // 5. Display validation issues
    if !validation_result.issues.is_empty() {
        println!("\n📋 Validation Issues:");
        for (i, issue) in validation_result.issues.iter().enumerate() {
            println!("  {}. [{}] {}: {}", 
                i + 1, 
                issue.severity, 
                issue.category, 
                issue.message
            );
            println!("     Path: {}", issue.path);
            if issue.auto_fixable {
                println!("     Auto-fixable: Yes");
            }
        }
    }

    // 6. Create custom validation rules
    println!("\n🔧 Creating custom validation rules...");
    let mut rules_config = ValidationRulesConfig::new();

    // Rule 1: Check for required fields
    let required_fields_rule = ValidationRule {
        id: "required-fields".to_string(),
        name: "Required Fields Check".to_string(),
        description: "Ensure all required fields are present".to_string(),
        rule_type: RuleType::Schema,
        severity: ConfigIssueSeverity::Error,
        enabled: true,
        conditions: vec![
            RuleCondition {
                field: "rhema.version".to_string(),
                operator: ConditionOperator::Exists,
                value: json!(true),
                case_sensitive: None,
            },
            RuleCondition {
                field: "rhema.scope.name".to_string(),
                operator: ConditionOperator::IsNotEmpty,
                value: json!(""),
                case_sensitive: None,
            },
        ],
        actions: vec![
            RuleAction {
                action_type: ActionType::Error,
                parameters: HashMap::new(),
                enabled: true,
            },
        ],
        metadata: HashMap::new(),
    };

    // Rule 2: Check for sensitive information
    let sensitive_info_rule = ValidationRule {
        id: "sensitive-info".to_string(),
        name: "Sensitive Information Check".to_string(),
        description: "Check for exposed sensitive information".to_string(),
        rule_type: RuleType::Security,
        severity: ConfigIssueSeverity::Warning,
        enabled: true,
        conditions: vec![
            RuleCondition {
                field: "password".to_string(),
                operator: ConditionOperator::Exists,
                value: json!(true),
                case_sensitive: None,
            },
        ],
        actions: vec![
            RuleAction {
                action_type: ActionType::Warn,
                parameters: HashMap::new(),
                enabled: true,
            },
        ],
        metadata: HashMap::new(),
    };

    // Rule 3: Performance check
    let performance_rule = ValidationRule {
        id: "performance-check".to_string(),
        name: "Performance Check".to_string(),
        description: "Check for performance issues".to_string(),
        rule_type: RuleType::Performance,
        severity: ConfigIssueSeverity::Warning,
        enabled: true,
        conditions: vec![
            RuleCondition {
                field: "knowledge".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: json!(100),
                case_sensitive: None,
            },
        ],
        actions: vec![
            RuleAction {
                action_type: ActionType::Warn,
                parameters: HashMap::new(),
                enabled: true,
            },
        ],
        metadata: HashMap::new(),
    };

    rules_config.add_rule(required_fields_rule);
    rules_config.add_rule(sensitive_info_rule);
    rules_config.add_rule(performance_rule);

    // 7. Create validation rules manager
    let rules_manager = ValidationRulesManager::new(rules_config)?;
    println!("✅ Custom validation rules created");

    // 8. Evaluate rules against configuration
    println!("\n🔍 Evaluating custom rules...");
    let rule_results = rules_manager.evaluate_rules(&sample_config, "example.yaml").await?;

    println!("Rule evaluation completed:");
    println!("  - Rules evaluated: {}", rule_results.len());
    
    for result in rule_results {
        println!("  - Rule '{}': conditions met = {}, actions executed = {}", 
            result.rule.name, 
            result.conditions_met, 
            result.actions_executed.len()
        );
        
        if !result.errors.is_empty() {
            println!("    Errors: {}", result.errors.len());
        }
    }

    // 9. Test with invalid configuration
    println!("\n🧪 Testing with invalid configuration...");
    let invalid_config = json!({
        "rhema": {
            "version": "invalid-version",
            "scope": {
                "type": "invalid-type",
                "name": ""
            }
        },
        "password": "exposed-password",
        "knowledge": (0..150).map(|i| json!({
            "title": format!("Knowledge {}", i),
            "content": "Large knowledge base"
        })).collect::<Vec<_>>()
    });

    let invalid_result = validator
        .validate_config_value(&invalid_config, &SchemaType::Rhema, Path::new("invalid.yaml"))
        .await?;

    println!("Invalid configuration validation:");
    println!("  - Overall valid: {}", invalid_result.valid);
    println!("  - Issues found: {}", invalid_result.issues.len());

    if !invalid_result.issues.is_empty() {
        println!("\n📋 Issues in invalid configuration:");
        for (i, issue) in invalid_result.issues.iter().enumerate() {
            println!("  {}. [{}] {}: {}", 
                i + 1, 
                issue.severity, 
                issue.category, 
                issue.message
            );
        }
    }

    // 10. Demonstrate different validation levels
    println!("\n📊 Demonstrating different validation levels...");
    
    let levels = [
        ValidationLevel::Basic,
        ValidationLevel::Standard,
        ValidationLevel::Strict,
        ValidationLevel::Complete,
    ];

    for level in levels {
        let mut level_validator = ComprehensiveValidator::with_settings(
            &global_config,
            300,
            level.clone(),
            false,
        ).await?;

        let result = level_validator
            .validate_config_value(&sample_config, &SchemaType::Rhema, Path::new("example.yaml"))
            .await?;

        println!("  {}: {} issues, {} warnings", 
            format!("{:?}", level), 
            result.issues.len(), 
            result.warnings.len()
        );
    }

    // 11. Show validation statistics
    println!("\n📈 Validation Statistics:");
    let stats = validator.get_statistics().await;
    println!("  - Cached results: {}", stats.cached_results);
    println!("  - Cache TTL: {} seconds", stats.cache_ttl);
    println!("  - Validation level: {:?}", stats.validation_level);
    println!("  - Auto-fix enabled: {}", stats.auto_fix);
    println!("  - Schema statistics: {} loaded schemas", stats.schema_statistics.loaded_schemas);

    let rules_stats = rules_manager.get_statistics();
    println!("  - Total rules: {}", rules_stats.total_rules);
    println!("  - Enabled rules: {}", rules_stats.enabled_rules);
    println!("  - Rule sets: {}", rules_stats.rule_sets);

    println!("\n✅ Comprehensive validation example completed successfully!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_comprehensive_validation_example() {
        let global_config = GlobalConfig::default();
        let validator = ComprehensiveValidator::new(&global_config).await.unwrap();
        
        let sample_config = json!({
            "rhema": {
                "version": "1.0.0",
                "scope": {
                    "type": "repository",
                    "name": "test-repo"
                }
            }
        });

        let result = validator
            .validate_config_value(&sample_config, &SchemaType::Rhema, Path::new("test.yaml"))
            .await
            .unwrap();

        assert!(result.valid);
        assert!(result.schema_valid);
    }

    #[tokio::test]
    async fn test_custom_validation_rules() {
        let mut rules_config = ValidationRulesConfig::new();
        
        let rule = ValidationRule {
            id: "test-rule".to_string(),
            name: "Test Rule".to_string(),
            description: "A test rule".to_string(),
            rule_type: RuleType::Schema,
            severity: ConfigIssueSeverity::Warning,
            enabled: true,
            conditions: vec![
                RuleCondition {
                    field: "test_field".to_string(),
                    operator: ConditionOperator::Equals,
                    value: json!("test_value"),
                    case_sensitive: None,
                }
            ],
            actions: vec![
                RuleAction {
                    action_type: ActionType::Log,
                    parameters: HashMap::new(),
                    enabled: true,
                }
            ],
            metadata: HashMap::new(),
        };

        rules_config.add_rule(rule);
        let manager = ValidationRulesManager::new(rules_config).unwrap();

        let config = json!({
            "test_field": "test_value"
        });

        let results = manager.evaluate_rules(&config, "test").await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].conditions_met);
    }
} 