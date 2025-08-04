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

use rhema_config::{
    ComprehensiveValidator, ComprehensiveValidationResult, ValidationLevel, ValidationCategory,
    SchemaType, GlobalConfig, RepositoryConfig, ScopeConfig, Config, ConfigIssueSeverity,
    RhemaResult,
};
use serde_json::json;
use std::path::PathBuf;
use tracing::{info, warn, error};

/// Example demonstrating comprehensive configuration validation
#[tokio::main]
async fn main() -> RhemaResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting comprehensive configuration validation example");

    // Create a global configuration
    let global_config = create_sample_global_config()?;

    // Create comprehensive validator with strict validation
    let mut validator = ComprehensiveValidator::with_settings(
        &global_config,
        300, // 5 minute cache TTL
        ValidationLevel::Strict,
        true, // Enable auto-fix
    ).await?;

    // Example 1: Validate a single configuration file
    validate_single_config(&validator).await?;

    // Example 2: Validate multiple configuration files in a directory
    validate_directory_configs(&validator).await?;

    // Example 3: Validate configuration with cross-references
    validate_cross_references(&validator).await?;

    // Example 4: Validate configuration with custom rules
    validate_with_custom_rules(&validator).await?;

    // Example 5: Performance validation
    validate_performance_config(&validator).await?;

    // Example 6: Security validation
    validate_security_config(&validator).await?;

    // Example 7: Compliance validation
    validate_compliance_config(&validator).await?;

    // Example 8: Dependency validation
    validate_dependencies(&validator).await?;

    // Example 9: Auto-fix demonstration
    demonstrate_auto_fix(&validator).await?;

    // Example 10: Validation statistics
    show_validation_statistics(&validator).await?;

    info!("Comprehensive configuration validation example completed successfully");
    Ok(())
}

/// Create a sample global configuration
fn create_sample_global_config() -> RhemaResult<GlobalConfig> {
    // Use the default configuration instead of trying to parse JSON
    Ok(GlobalConfig::new())
}

/// Example 1: Validate a single configuration file
async fn validate_single_config(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 1: Single Configuration Validation ===");

    let config_value = json!({
        "version": "1.0.0",
        "repository": {
            "name": "my-repo",
            "url": "https://github.com/user/my-repo",
            "branch": "main",
            "auto_sync": true
        },
        "scope": {
            "include": ["src/**/*.rs", "tests/**/*.rs"],
            "exclude": ["target/", "node_modules/"],
            "max_file_size": 1048576
        },
        "validation": {
            "enabled": true,
            "rules": ["syntax", "type_check", "security"]
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Scope, &PathBuf::from("config.yml"))
        .await?;

    print_validation_result(&result, "Single Configuration");
    Ok(())
}

/// Example 2: Validate multiple configuration files in a directory
async fn validate_directory_configs(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 2: Directory Configuration Validation ===");

    // Create temporary directory with sample configs
    let temp_dir = tempfile::tempdir()?;
    let config_dir = temp_dir.path();

    // Create sample configuration files
    create_sample_config_files(config_dir).await?;

    let report = validator.validate_directory(config_dir).await?;

    info!("Directory validation completed:");
    info!("  Total configs: {}", report.summary.total_configs);
    info!("  Valid configs: {}", report.summary.valid_configs);
    info!("  Invalid configs: {}", report.summary.invalid_configs);
    info!("  Total issues: {}", report.summary.total_issues);
    info!("  Duration: {}ms", report.duration_ms);

    Ok(())
}

/// Example 3: Validate configuration with cross-references
async fn validate_cross_references(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 3: Cross-Reference Validation ===");

    let config_value = json!({
        "version": "1.0.0",
        "repositories": {
            "main": {
                "name": "main-repo",
                "url": "https://github.com/user/main-repo",
                "dependencies": ["shared-lib", "utils"]
            },
            "shared-lib": {
                "name": "shared-lib",
                "url": "https://github.com/user/shared-lib",
                "dependencies": []
            },
            "utils": {
                "name": "utils",
                "url": "https://github.com/user/utils",
                "dependencies": ["shared-lib"]
            }
        },
        "workflows": {
            "build": {
                "steps": ["checkout", "install", "build", "test"],
                "depends_on": ["main", "shared-lib"]
            }
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Global, &PathBuf::from("global.yml"))
        .await?;

    print_validation_result(&result, "Cross-Reference Configuration");
    Ok(())
}

/// Example 4: Validate configuration with custom rules
async fn validate_with_custom_rules(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 4: Custom Rules Validation ===");

    let config_value = json!({
        "version": "1.0.0",
        "project": {
            "name": "my-project",
            "version": "0.1.0",
            "description": "A sample project",
            "authors": ["John Doe <john@example.com>"],
            "license": "MIT",
            "repository": "https://github.com/user/my-project"
        },
        "dependencies": {
            "runtime": {
                "tokio": "1.0",
                "serde": "1.0",
                "serde_json": "1.0"
            },
            "dev": {
                "criterion": "0.3",
                "mockall": "0.10"
            }
        },
        "build": {
            "target": "x86_64-unknown-linux-gnu",
            "release": true,
            "optimization": "speed"
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Project, &PathBuf::from("Cargo.toml"))
        .await?;

    print_validation_result(&result, "Custom Rules Configuration");
    Ok(())
}

/// Example 5: Performance validation
async fn validate_performance_config(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 5: Performance Configuration Validation ===");

    let config_value = json!({
        "version": "1.0.0",
        "performance": {
            "cache": {
                "enabled": true,
                "max_size": "1GB",
                "ttl": 3600,
                "eviction_policy": "lru"
            },
            "parallel": {
                "enabled": true,
                "max_workers": 8,
                "chunk_size": 1000
            },
            "optimization": {
                "level": "aggressive",
                "target": "speed",
                "inlining": true
            },
            "monitoring": {
                "enabled": true,
                "metrics": ["cpu", "memory", "disk", "network"],
                "sampling_rate": 0.1
            }
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Performance, &PathBuf::from("perf.yml"))
        .await?;

    print_validation_result(&result, "Performance Configuration");
    Ok(())
}

/// Example 6: Security validation
async fn validate_security_config(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 6: Security Configuration Validation ===");

    let config_value = json!({
        "version": "1.0.0",
        "security": {
            "encryption": {
                "enabled": true,
                "algorithm": "AES256GCM",
                "key_rotation": {
                    "enabled": true,
                    "interval_days": 90
                }
            },
            "access_control": {
                "enabled": true,
                "authentication": "oauth2",
                "authorization": "rbac",
                "session_timeout": 3600
            },
            "audit": {
                "enabled": true,
                "level": "detailed",
                "retention_days": 365,
                "events": ["read", "write", "delete", "access"]
            },
            "compliance": {
                "framework": "SOC2",
                "level": "standard",
                "checks": ["encryption", "access_control", "audit_logging"]
            }
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Security, &PathBuf::from("security.yml"))
        .await?;

    print_validation_result(&result, "Security Configuration");
    Ok(())
}

/// Example 7: Compliance validation
async fn validate_compliance_config(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 7: Compliance Configuration Validation ===");

    let config_value = json!({
        "version": "1.0.0",
        "compliance": {
            "framework": "GDPR",
            "level": "enhanced",
            "data_protection": {
                "encryption_at_rest": true,
                "encryption_in_transit": true,
                "data_minimization": true,
                "retention_policy": {
                    "enabled": true,
                    "max_retention_days": 2555
                }
            },
            "privacy": {
                "consent_management": true,
                "data_subject_rights": true,
                "breach_notification": true
            },
            "reporting": {
                "enabled": true,
                "frequency": "monthly",
                "recipients": ["dpo@company.com", "legal@company.com"]
            }
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Compliance, &PathBuf::from("compliance.yml"))
        .await?;

    print_validation_result(&result, "Compliance Configuration");
    Ok(())
}

/// Example 8: Dependency validation
async fn validate_dependencies(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 8: Dependency Validation ===");

    let config_value = json!({
        "version": "1.0.0",
        "dependencies": {
            "runtime": {
                "tokio": {
                    "version": "1.0",
                    "features": ["full"],
                    "optional": false
                },
                "serde": {
                    "version": "1.0",
                    "features": ["derive"],
                    "optional": false
                }
            },
            "dev": {
                "criterion": {
                    "version": "0.3",
                    "optional": true
                }
            }
        },
        "build_dependencies": {
            "cc": "1.0",
            "pkg-config": "0.3"
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Dependencies, &PathBuf::from("deps.yml"))
        .await?;

    print_validation_result(&result, "Dependencies Configuration");
    Ok(())
}

/// Example 9: Auto-fix demonstration
async fn demonstrate_auto_fix(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 9: Auto-Fix Demonstration ===");

    // Create a configuration with issues that can be auto-fixed
    let config_value = json!({
        "version": "1.0.0",
        "repository": {
            "name": "my-repo",
            "url": "https://github.com/user/my-repo",
            "branch": "main",
            "auto_sync": true,
            "invalid_field": "should_be_removed",
            "timeout": "30s" // Should be a number
        },
        "scope": {
            "include": ["src/**/*.rs"],
            "exclude": ["target/"],
            "max_file_size": "1MB" // Should be a number
        }
    });

    let result = validator
        .validate_config_value(&config_value, &SchemaType::Repository, &PathBuf::from("auto-fix.yml"))
        .await?;

    info!("Auto-fix demonstration completed:");
    info!("  Original issues: {}", result.issues.len());
    
    // Show auto-fixable issues
    let auto_fixable = result.issues.iter().filter(|i| i.auto_fixable).count();
    info!("  Auto-fixable issues: {}", auto_fixable);

    for issue in &result.issues {
        if issue.auto_fixable {
            info!("  Auto-fixable: {} - {}", issue.path, issue.message);
            if let Some(fix) = &issue.suggested_fix {
                info!("    Suggested fix: {:?}", fix);
            }
        }
    }

    Ok(())
}

/// Example 10: Validation statistics
async fn show_validation_statistics(validator: &ComprehensiveValidator) -> RhemaResult<()> {
    info!("=== Example 10: Validation Statistics ===");

    let stats = validator.get_statistics().await;
    
    info!("Validation Statistics:");
    info!("  Cached results: {}", stats.cached_results);
    info!("  Cache TTL: {} seconds", stats.cache_ttl);
    info!("  Validation level: {:?}", stats.validation_level);
    info!("  Auto-fix enabled: {}", stats.auto_fix);
    info!("  Schema validation statistics:");
    info!("    Loaded schemas: {}", stats.schema_statistics.loaded_schemas);
    info!("    Cached results: {}", stats.schema_statistics.cached_results);
    info!("    Cache TTL: {} seconds", stats.schema_statistics.cache_ttl);
    info!("    Strict mode: {}", stats.schema_statistics.strict_mode);

    Ok(())
}

/// Create sample configuration files for directory validation
async fn create_sample_config_files(config_dir: &std::path::Path) -> RhemaResult<()> {
    use std::fs;
    use std::io::Write;

    // Create repository config
    let repo_config = r#"version: "1.0.0"
repository:
  name: "sample-repo"
  url: "https://github.com/user/sample-repo"
  branch: "main""#;
    
    let repo_path = config_dir.join("repo.yml");
    let mut file = fs::File::create(&repo_path)?;
    file.write_all(repo_config.as_bytes())?;

    // Create scope config
    let scope_config = r#"version: "1.0.0"
scope:
  include: ["src/**/*.rs"]
  exclude: ["target/"]"#;
    
    let scope_path = config_dir.join("scope.yml");
    let mut file = fs::File::create(&scope_path)?;
    file.write_all(scope_config.as_bytes())?;

    // Create invalid config for testing
    let invalid_config = r#"version: "invalid-version"
repository:
  name: ""
  url: "not-a-url""#;
    
    let invalid_path = config_dir.join("invalid.yml");
    let mut file = fs::File::create(&invalid_path)?;
    file.write_all(invalid_config.as_bytes())?;

    Ok(())
}

/// Print validation result in a formatted way
fn print_validation_result(result: &ComprehensiveValidationResult, context: &str) {
    info!("{} Validation Result:", context);
    info!("  Valid: {}", result.valid);
    info!("  Schema valid: {}", result.schema_valid);
    info!("  Business valid: {}", result.business_valid);
    info!("  Duration: {}ms", result.duration_ms);
    info!("  Issues: {}", result.issues.len());
    info!("  Warnings: {}", result.warnings.len());

    if !result.issues.is_empty() {
        info!("  Issues:");
        for issue in &result.issues {
            let severity_icon = match issue.severity {
                ConfigIssueSeverity::Critical => "🔴",
                ConfigIssueSeverity::Error => "❌",
                ConfigIssueSeverity::Warning => "⚠️",
                ConfigIssueSeverity::Info => "ℹ️",
            };
            
            let category_icon = match issue.category {
                ValidationCategory::Schema => "📋",
                ValidationCategory::Business => "💼",
                ValidationCategory::Security => "🔒",
                ValidationCategory::Performance => "⚡",
                ValidationCategory::Compliance => "📊",
                ValidationCategory::CrossReference => "🔗",
                ValidationCategory::Dependency => "📦",
                ValidationCategory::Custom => "🔧",
            };

            info!("    {} {} {}: {} - {}", 
                severity_icon, 
                category_icon, 
                issue.path, 
                issue.message,
                if issue.auto_fixable { "(auto-fixable)" } else { "" }
            );
        }
    }

    if !result.warnings.is_empty() {
        info!("  Warnings:");
        for warning in &result.warnings {
            info!("    ⚠️  {}", warning);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_comprehensive_validation_example() {
        // This test ensures the example runs without panicking
        let global_config = create_sample_global_config().unwrap();
        let validator = ComprehensiveValidator::with_settings(
            &global_config,
            300,
            ValidationLevel::Standard,
            false,
        ).await.unwrap();

        // Test basic validation
        let config_value = json!({
            "version": "1.0.0",
            "repository": {
                "name": "test-repo",
                "url": "https://github.com/user/test-repo"
            }
        });

        let result = validator
            .validate_config_value(&config_value, &SchemaType::Repository, &PathBuf::from("test.yml"))
            .await
            .unwrap();

        assert!(result.valid);
    }
} 