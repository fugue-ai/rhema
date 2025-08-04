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
    ValidationRulesConfig, ValidationRule, RuleType, RuleCondition, 
    ConditionOperator, RuleAction, ActionType, ValidationRulesManager,
    ConfigIssueSeverity,
};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Validation Test ===\n");

    // Create a validation rules configuration
    let mut rules_config = ValidationRulesConfig::new();

    // Create a simple validation rule
    let rule = ValidationRule {
        id: "test-rule".to_string(),
        name: "Test Rule".to_string(),
        description: "A simple test rule".to_string(),
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
    println!("✅ Created validation rule");

    // Create validation rules manager
    let manager = ValidationRulesManager::new(rules_config)?;
    println!("✅ Created validation rules manager");

    // Test configuration
    let config = json!({
        "test_field": "test_value"
    });

    // Evaluate rules
    let results = manager.evaluate_rules(&config, "test").await?;
    println!("✅ Evaluated rules");

    // Display results
    println!("\nResults:");
    for result in results {
        println!("  - Rule '{}': conditions met = {}", 
            result.rule.name, result.conditions_met);
        println!("    Actions executed: {}", result.actions_executed.len());
        if !result.errors.is_empty() {
            println!("    Errors: {}", result.errors.len());
        }
    }

    // Test with different configuration
    let config2 = json!({
        "test_field": "different_value"
    });

    let results2 = manager.evaluate_rules(&config2, "test2").await?;
    println!("\nResults with different value:");
    for result in results2 {
        println!("  - Rule '{}': conditions met = {}", 
            result.rule.name, result.conditions_met);
    }

    println!("\n✅ Simple validation test completed successfully!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_validation() {
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