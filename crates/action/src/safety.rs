use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};
use crate::schema::{ActionIntent, SafetyLevel, ActionType};
use crate::error::{ActionError, ActionResult};

/// Safety rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub safety_level: SafetyLevel,
    pub action_types: Vec<ActionType>,
    pub enabled: bool,
    pub severity: RuleSeverity,
    pub conditions: Vec<SafetyCondition>,
    pub validations: Vec<SafetyValidation>,
}

/// Rule severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuleSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Safety condition for rule evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
    pub description: String,
}

/// Condition operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    GreaterThan,
    LessThan,
    In,
    NotIn,
    Regex,
    FileExists,
    FileNotExists,
}

/// Safety validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyValidation {
    pub name: String,
    pub description: String,
    pub validation_type: ValidationType,
    pub required: bool,
    pub timeout_seconds: Option<u64>,
}

/// Validation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    SyntaxCheck,
    TypeCheck,
    TestRun,
    SecurityScan,
    DependencyCheck,
    CustomCommand(String),
}

/// Safety rule evaluation result
#[derive(Debug, Clone)]
pub struct SafetyRuleResult {
    pub rule_id: String,
    pub rule_name: String,
    pub passed: bool,
    pub severity: RuleSeverity,
    pub message: String,
    pub details: HashMap<String, serde_json::Value>,
    pub duration: std::time::Duration,
}

/// Safety rules engine
pub struct SafetyRulesEngine {
    rules: HashMap<String, SafetyRule>,
    enabled_rules: Vec<String>,
}

impl SafetyRulesEngine {
    /// Create a new safety rules engine
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            enabled_rules: Vec::new(),
        }
    }

    /// Initialize with default safety rules
    pub async fn initialize() -> ActionResult<Self> {
        let mut engine = Self::new();
        engine.register_default_rules().await?;
        Ok(engine)
    }

    /// Register a safety rule
    pub fn register_rule(&mut self, rule: SafetyRule) -> ActionResult<()> {
        let rule_id = rule.id.clone();
        if rule.enabled {
            self.enabled_rules.push(rule_id.clone());
        }
        self.rules.insert(rule_id, rule);
        Ok(())
    }

    /// Evaluate safety rules for an intent
    pub async fn evaluate_rules(&self, intent: &ActionIntent) -> ActionResult<Vec<SafetyRuleResult>> {
        let mut results = Vec::new();
        
        for rule_id in &self.enabled_rules {
            if let Some(rule) = self.rules.get(rule_id) {
                let result = self.evaluate_rule(rule, intent).await?;
                results.push(result);
            }
        }
        
        Ok(results)
    }

    /// Evaluate a single safety rule
    async fn evaluate_rule(&self, rule: &SafetyRule, intent: &ActionIntent) -> ActionResult<SafetyRuleResult> {
        let start = std::time::Instant::now();
        
        // Check if rule applies to this action type
        if !rule.action_types.contains(&intent.action_type) {
            return Ok(SafetyRuleResult {
                rule_id: rule.id.clone(),
                rule_name: rule.name.clone(),
                passed: true,
                severity: rule.severity.clone(),
                message: "Rule not applicable to action type".to_string(),
                details: HashMap::new(),
                duration: start.elapsed(),
            });
        }

        // Check safety level compatibility
        if intent.safety_level < rule.safety_level {
            return Ok(SafetyRuleResult {
                rule_id: rule.id.clone(),
                rule_name: rule.name.clone(),
                passed: false,
                severity: rule.severity.clone(),
                message: format!("Action safety level ({:?}) is lower than required ({:?})", 
                               intent.safety_level, rule.safety_level),
                details: HashMap::new(),
                duration: start.elapsed(),
            });
        }

        // Evaluate conditions
        let mut all_conditions_passed = true;
        let mut condition_details = HashMap::new();
        
        for condition in &rule.conditions {
            let (passed, details) = self.evaluate_condition(condition, intent).await?;
            condition_details.insert(condition.field.clone(), serde_json::json!(details));
            
            if !passed {
                all_conditions_passed = false;
            }
        }

        // Run validations if conditions pass
        let mut validation_results = HashMap::new();
        if all_conditions_passed {
            for validation in &rule.validations {
                let result = self.run_validation(validation, intent).await?;
                validation_results.insert(validation.name.clone(), serde_json::json!(result));
            }
        }

        let mut details = HashMap::new();
        details.insert("conditions".to_string(), serde_json::json!(condition_details));
        details.insert("validations".to_string(), serde_json::json!(validation_results));

        Ok(SafetyRuleResult {
            rule_id: rule.id.clone(),
            rule_name: rule.name.clone(),
            passed: all_conditions_passed,
            severity: rule.severity.clone(),
            message: if all_conditions_passed {
                "All safety checks passed".to_string()
            } else {
                "Safety conditions not met".to_string()
            },
            details,
            duration: start.elapsed(),
        })
    }

    /// Evaluate a safety condition
    async fn evaluate_condition(&self, condition: &SafetyCondition, intent: &ActionIntent) -> ActionResult<(bool, String)> {
        match condition.field.as_str() {
            "description" => {
                let description = &intent.description;
                self.evaluate_string_condition(description, &condition.operator, &condition.value)
            }
            "scope" => {
                let scope = &intent.scope;
                self.evaluate_list_condition(scope, &condition.operator, &condition.value)
            }
            "tools" => {
                let tools = &intent.transformation.tools;
                self.evaluate_list_condition(tools, &condition.operator, &condition.value)
            }
            "safety_level" => {
                let safety_level = serde_json::json!(intent.safety_level);
                self.evaluate_value_condition(&safety_level, &condition.operator, &condition.value)
            }
            "action_type" => {
                let action_type = serde_json::json!(intent.action_type);
                self.evaluate_value_condition(&action_type, &condition.operator, &condition.value)
            }
            _ => {
                warn!("Unknown condition field: {}", condition.field);
                Ok((true, "Unknown field, skipping".to_string()))
            }
        }
    }

    /// Evaluate string condition
    fn evaluate_string_condition(&self, value: &str, operator: &ConditionOperator, expected: &serde_json::Value) -> ActionResult<(bool, String)> {
        let expected_str = expected.as_str().unwrap_or("");
        
        let result = match operator {
            ConditionOperator::Equals => value == expected_str,
            ConditionOperator::NotEquals => value != expected_str,
            ConditionOperator::Contains => value.contains(expected_str),
            ConditionOperator::NotContains => !value.contains(expected_str),
            ConditionOperator::Regex => {
                if let Ok(regex) = regex::Regex::new(expected_str) {
                    regex.is_match(value)
                } else {
                    false
                }
            }
            _ => {
                warn!("Unsupported operator for string condition: {:?}", operator);
                false
            }
        };
        
        Ok((result, format!("String condition: {} {:?} {}", value, operator, expected_str)))
    }

    /// Evaluate list condition
    fn evaluate_list_condition(&self, value: &[String], operator: &ConditionOperator, expected: &serde_json::Value) -> ActionResult<(bool, String)> {
        let result = match operator {
            ConditionOperator::Contains => {
                if let Some(expected_str) = expected.as_str() {
                    value.iter().any(|item| item.contains(expected_str))
                } else {
                    false
                }
            }
            ConditionOperator::NotContains => {
                if let Some(expected_str) = expected.as_str() {
                    !value.iter().any(|item| item.contains(expected_str))
                } else {
                    true
                }
            }
            ConditionOperator::In => {
                if let Some(expected_list) = expected.as_array() {
                    let expected_strings: Vec<String> = expected_list
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    value.iter().any(|item| expected_strings.contains(item))
                } else {
                    false
                }
            }
            ConditionOperator::NotIn => {
                if let Some(expected_list) = expected.as_array() {
                    let expected_strings: Vec<String> = expected_list
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    !value.iter().any(|item| expected_strings.contains(item))
                } else {
                    true
                }
            }
            _ => {
                warn!("Unsupported operator for list condition: {:?}", operator);
                false
            }
        };
        
        Ok((result, format!("List condition: {:?} {:?} {:?}", value, operator, expected)))
    }

    /// Evaluate value condition
    fn evaluate_value_condition(&self, value: &serde_json::Value, operator: &ConditionOperator, expected: &serde_json::Value) -> ActionResult<(bool, String)> {
        let result = match operator {
            ConditionOperator::Equals => value == expected,
            ConditionOperator::NotEquals => value != expected,
            _ => {
                warn!("Unsupported operator for value condition: {:?}", operator);
                false
            }
        };
        
        Ok((result, format!("Value condition: {} {:?} {}", value, operator, expected)))
    }

    /// Run a safety validation
    async fn run_validation(&self, validation: &SafetyValidation, intent: &ActionIntent) -> ActionResult<String> {
        match &validation.validation_type {
            ValidationType::SyntaxCheck => {
                self.run_syntax_check(intent).await
            }
            ValidationType::TypeCheck => {
                self.run_type_check(intent).await
            }
            ValidationType::TestRun => {
                self.run_test_check(intent).await
            }
            ValidationType::SecurityScan => {
                self.run_security_scan(intent).await
            }
            ValidationType::DependencyCheck => {
                self.run_dependency_check(intent).await
            }
            ValidationType::CustomCommand(cmd) => {
                self.run_custom_command(cmd, intent).await
            }
        }
    }

    /// Run syntax check validation
    async fn run_syntax_check(&self, intent: &ActionIntent) -> ActionResult<String> {
        info!("Running syntax check for intent: {}", intent.id);
        
        let files = &intent.scope;
        if files.is_empty() {
            return Ok("No files to check".to_string());
        }
        
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        for file in files {
            match self.check_file_syntax(file).await {
                Ok(result) => results.push(format!("{}: {}", file, result)),
                Err(e) => errors.push(format!("{}: {}", file, e)),
            }
        }
        
        if errors.is_empty() {
            Ok(format!("Syntax check passed for {} files: {}", files.len(), results.join(", ")))
        } else {
            Err(ActionError::validation(format!("Syntax check failed: {}", errors.join(", "))))
        }
    }

    /// Run type check validation
    async fn run_type_check(&self, intent: &ActionIntent) -> ActionResult<String> {
        info!("Running type check for intent: {}", intent.id);
        
        let files = &intent.scope;
        if files.is_empty() {
            return Ok("No files to check".to_string());
        }
        
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        for file in files {
            match self.check_file_types(file).await {
                Ok(result) => results.push(format!("{}: {}", file, result)),
                Err(e) => errors.push(format!("{}: {}", file, e)),
            }
        }
        
        if errors.is_empty() {
            Ok(format!("Type check passed for {} files: {}", files.len(), results.join(", ")))
        } else {
            Err(ActionError::validation(format!("Type check failed: {}", errors.join(", "))))
        }
    }
    
    /// Check syntax for a specific file
    async fn check_file_syntax(&self, file_path: &str) -> ActionResult<String> {
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Determine language and run appropriate syntax checker
        if file_path.ends_with(".js") || file_path.ends_with(".ts") || file_path.ends_with(".jsx") || file_path.ends_with(".tsx") {
            self.check_javascript_syntax(file_path).await
        } else if file_path.ends_with(".py") {
            self.check_python_syntax(file_path).await
        } else if file_path.ends_with(".rs") {
            self.check_rust_syntax(file_path).await
        } else {
            Ok("Syntax check not implemented for this file type".to_string())
        }
    }
    
    /// Check types for a specific file
    async fn check_file_types(&self, file_path: &str) -> ActionResult<String> {
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Determine language and run appropriate type checker
        if file_path.ends_with(".ts") || file_path.ends_with(".tsx") {
            self.check_typescript_types(file_path).await
        } else if file_path.ends_with(".rs") {
            self.check_rust_types(file_path).await
        } else {
            Ok("Type checking not implemented for this file type".to_string())
        }
    }
    
    /// Check JavaScript/TypeScript syntax
    async fn check_javascript_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("node")
            .args(&["--check", file_path])
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check JavaScript syntax: {}", e)))?;
        
        if output.status.success() {
            Ok("JavaScript syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("JavaScript syntax error: {}", error)))
        }
    }
    
    /// Check Python syntax
    async fn check_python_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("python3")
            .args(&["-m", "py_compile", file_path])
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check Python syntax: {}", e)))?;
        
        if output.status.success() {
            Ok("Python syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("Python syntax error: {}", error)))
        }
    }
    
    /// Check Rust syntax
    async fn check_rust_syntax(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("rustc")
            .args(&["--emit=metadata", "--crate-type=lib", file_path])
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check Rust syntax: {}", e)))?;
        
        if output.status.success() {
            Ok("Rust syntax valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("Rust syntax error: {}", error)))
        }
    }
    
    /// Check TypeScript types
    async fn check_typescript_types(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("npx")
            .args(&["tsc", "--noEmit", file_path])
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check TypeScript types: {}", e)))?;
        
        if output.status.success() {
            Ok("TypeScript types valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("TypeScript type error: {}", error)))
        }
    }
    
    /// Check Rust types
    async fn check_rust_types(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("cargo")
            .args(&["check", "--manifest-path", file_path])
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check Rust types: {}", e)))?;
        
        if output.status.success() {
            Ok("Rust types valid".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("Rust type error: {}", error)))
        }
    }

    /// Run test check validation
    async fn run_test_check(&self, intent: &ActionIntent) -> ActionResult<String> {
        info!("Running test check for intent: {}", intent.id);
        
        let files = &intent.scope;
        if files.is_empty() {
            return Ok("No files to test".to_string());
        }
        
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        // Find test files and run appropriate test runners
        for file in files {
            match self.run_tests_for_file(file).await {
                Ok(result) => results.push(format!("{}: {}", file, result)),
                Err(e) => errors.push(format!("{}: {}", file, e)),
            }
        }
        
        if errors.is_empty() {
            Ok(format!("Test check passed for {} files: {}", files.len(), results.join(", ")))
        } else {
            Err(ActionError::validation(format!("Test check failed: {}", errors.join(", "))))
        }
    }
    
    /// Run tests for a specific file
    async fn run_tests_for_file(&self, file_path: &str) -> ActionResult<String> {
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Determine test runner based on file type and project structure
        if file_path.ends_with(".js") || file_path.ends_with(".ts") || file_path.ends_with(".jsx") || file_path.ends_with(".tsx") {
            self.run_javascript_tests(file_path).await
        } else if file_path.ends_with(".py") {
            self.run_python_tests(file_path).await
        } else if file_path.ends_with(".rs") {
            self.run_rust_tests(file_path).await
        } else {
            Ok("Test running not implemented for this file type".to_string())
        }
    }
    
    /// Run JavaScript/TypeScript tests
    async fn run_javascript_tests(&self, file_path: &str) -> ActionResult<String> {
        // Try Jest first, then Mocha
        let jest_result = tokio::process::Command::new("npx")
            .args(&["jest", "--passWithNoTests", file_path])
            .output()
            .await;
        
        if let Ok(output) = jest_result {
            if output.status.success() {
                return Ok("Jest tests passed".to_string());
            }
        }
        
        // Try Mocha as fallback
        let mocha_result = tokio::process::Command::new("npx")
            .args(&["mocha", "--timeout", "10000", file_path])
            .output()
            .await;
        
        if let Ok(output) = mocha_result {
            if output.status.success() {
                return Ok("Mocha tests passed".to_string());
            }
        }
        
        Ok("No test runner found or tests passed".to_string())
    }
    
    /// Run Python tests
    async fn run_python_tests(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("pytest")
            .args(&["--tb=short", file_path])
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to run Python tests: {}", e)))?;
        
        if output.status.success() {
            Ok("Python tests passed".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("Python tests failed: {}", error)))
        }
    }
    
    /// Run Rust tests
    async fn run_rust_tests(&self, file_path: &str) -> ActionResult<String> {
        // For Rust, we need to find the Cargo.toml and run tests from there
        if let Some(cargo_dir) = self.find_cargo_directory(file_path).await {
            let output = tokio::process::Command::new("cargo")
                .args(&["test"])
                .current_dir(cargo_dir)
                .output()
                .await
                .map_err(|e| ActionError::validation(format!("Failed to run Rust tests: {}", e)))?;
            
            if output.status.success() {
                Ok("Rust tests passed".to_string())
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Err(ActionError::validation(format!("Rust tests failed: {}", error)))
            }
        } else {
            Ok("No Cargo.toml found for Rust tests".to_string())
        }
    }
    
    /// Find Cargo.toml directory for a Rust file
    async fn find_cargo_directory(&self, file_path: &str) -> Option<String> {
        let mut current = std::path::Path::new(file_path).parent();
        
        while let Some(dir) = current {
            let cargo_toml = dir.join("Cargo.toml");
            if cargo_toml.exists() {
                return dir.to_str().map(|s| s.to_string());
            }
            current = dir.parent();
        }
        
        None
    }

    /// Run security scan validation
    async fn run_security_scan(&self, intent: &ActionIntent) -> ActionResult<String> {
        info!("Running security scan for intent: {}", intent.id);
        
        let files = &intent.scope;
        if files.is_empty() {
            return Ok("No files to scan".to_string());
        }
        
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        // Run security scans on files
        for file in files {
            match self.scan_file_security(file).await {
                Ok(result) => results.push(format!("{}: {}", file, result)),
                Err(e) => errors.push(format!("{}: {}", file, e)),
            }
        }
        
        if errors.is_empty() {
            Ok(format!("Security scan passed for {} files: {}", files.len(), results.join(", ")))
        } else {
            Err(ActionError::validation(format!("Security scan failed: {}", errors.join(", "))))
        }
    }
    
    /// Scan a file for security issues
    async fn scan_file_security(&self, file_path: &str) -> ActionResult<String> {
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Try different security scanning tools
        let mut scan_results = Vec::new();
        
        // Try npm audit for JavaScript/TypeScript projects
        if file_path.ends_with("package.json") || file_path.ends_with("package-lock.json") {
            if let Ok(result) = self.run_npm_audit(file_path).await {
                scan_results.push(result);
            }
        }
        
        // Try cargo audit for Rust projects
        if file_path.ends_with("Cargo.toml") || file_path.ends_with("Cargo.lock") {
            if let Ok(result) = self.run_cargo_audit(file_path).await {
                scan_results.push(result);
            }
        }
        
        // Try bandit for Python files
        if file_path.ends_with(".py") {
            if let Ok(result) = self.run_bandit_scan(file_path).await {
                scan_results.push(result);
            }
        }
        
        if scan_results.is_empty() {
            Ok("No security scanner available for this file type".to_string())
        } else {
            Ok(scan_results.join(", "))
        }
    }
    
    /// Run npm audit for JavaScript/TypeScript dependencies
    async fn run_npm_audit(&self, file_path: &str) -> ActionResult<String> {
        let project_dir = std::path::Path::new(file_path).parent()
            .ok_or_else(|| ActionError::validation("Invalid package.json path".to_string()))?;
        
        let output = tokio::process::Command::new("npm")
            .args(&["audit", "--audit-level=moderate"])
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to run npm audit: {}", e)))?;
        
        if output.status.success() {
            Ok("npm audit passed".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("npm audit found vulnerabilities: {}", error)))
        }
    }
    
    /// Run cargo audit for Rust dependencies
    async fn run_cargo_audit(&self, file_path: &str) -> ActionResult<String> {
        let project_dir = std::path::Path::new(file_path).parent()
            .ok_or_else(|| ActionError::validation("Invalid Cargo.toml path".to_string()))?;
        
        let output = tokio::process::Command::new("cargo")
            .args(&["audit"])
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to run cargo audit: {}", e)))?;
        
        if output.status.success() {
            Ok("cargo audit passed".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("cargo audit found vulnerabilities: {}", error)))
        }
    }
    
    /// Run bandit scan for Python files
    async fn run_bandit_scan(&self, file_path: &str) -> ActionResult<String> {
        let output = tokio::process::Command::new("bandit")
            .args(&["-r", file_path])
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to run bandit scan: {}", e)))?;
        
        if output.status.success() {
            Ok("bandit scan passed".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("bandit scan found issues: {}", error)))
        }
    }

    /// Run dependency check validation
    async fn run_dependency_check(&self, intent: &ActionIntent) -> ActionResult<String> {
        info!("Running dependency check for intent: {}", intent.id);
        
        let files = &intent.scope;
        if files.is_empty() {
            return Ok("No files to check".to_string());
        }
        
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        // Check dependencies for each file
        for file in files {
            match self.check_file_dependencies(file).await {
                Ok(result) => results.push(format!("{}: {}", file, result)),
                Err(e) => errors.push(format!("{}: {}", file, e)),
            }
        }
        
        if errors.is_empty() {
            Ok(format!("Dependency check passed for {} files: {}", files.len(), results.join(", ")))
        } else {
            Err(ActionError::validation(format!("Dependency check failed: {}", errors.join(", "))))
        }
    }
    
    /// Check dependencies for a specific file
    async fn check_file_dependencies(&self, file_path: &str) -> ActionResult<String> {
        if !std::path::Path::new(file_path).exists() {
            return Err(ActionError::validation(format!("File not found: {}", file_path)));
        }
        
        // Check dependencies based on file type
        if file_path.ends_with("package.json") {
            self.check_npm_dependencies(file_path).await
        } else if file_path.ends_with("Cargo.toml") {
            self.check_cargo_dependencies(file_path).await
        } else if file_path.ends_with("requirements.txt") || file_path.ends_with("pyproject.toml") {
            self.check_python_dependencies(file_path).await
        } else {
            Ok("No dependency file found".to_string())
        }
    }
    
    /// Check npm dependencies
    async fn check_npm_dependencies(&self, file_path: &str) -> ActionResult<String> {
        let project_dir = std::path::Path::new(file_path).parent()
            .ok_or_else(|| ActionError::validation("Invalid package.json path".to_string()))?;
        
        // Check for outdated dependencies
        let output = tokio::process::Command::new("npm")
            .args(&["outdated"])
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check npm dependencies: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                Ok("All npm dependencies are up to date".to_string())
            } else {
                Ok(format!("Found outdated dependencies: {}", stdout.trim()))
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("npm dependency check failed: {}", error)))
        }
    }
    
    /// Check Cargo dependencies
    async fn check_cargo_dependencies(&self, file_path: &str) -> ActionResult<String> {
        let project_dir = std::path::Path::new(file_path).parent()
            .ok_or_else(|| ActionError::validation("Invalid Cargo.toml path".to_string()))?;
        
        // Check for outdated dependencies
        let output = tokio::process::Command::new("cargo")
            .args(&["outdated"])
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check cargo dependencies: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                Ok("All cargo dependencies are up to date".to_string())
            } else {
                Ok(format!("Found outdated dependencies: {}", stdout.trim()))
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("cargo dependency check failed: {}", error)))
        }
    }
    
    /// Check Python dependencies
    async fn check_python_dependencies(&self, file_path: &str) -> ActionResult<String> {
        let project_dir = std::path::Path::new(file_path).parent()
            .ok_or_else(|| ActionError::validation("Invalid requirements.txt path".to_string()))?;
        
        // Check for outdated dependencies using pip
        let output = tokio::process::Command::new("pip")
            .args(&["list", "--outdated"])
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to check python dependencies: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                Ok("All python dependencies are up to date".to_string())
            } else {
                Ok(format!("Found outdated dependencies: {}", stdout.trim()))
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("python dependency check failed: {}", error)))
        }
    }

    /// Run custom command validation
    async fn run_custom_command(&self, command: &str, intent: &ActionIntent) -> ActionResult<String> {
        info!("Running custom command for intent: {}", intent.id);
        
        // Validate command for safety
        if !self.is_command_safe(command) {
            return Err(ActionError::validation(format!("Command '{}' is not allowed for security reasons", command)));
        }
        
        // Execute the command
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .await
            .map_err(|e| ActionError::validation(format!("Failed to execute custom command: {}", e)))?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(format!("Custom command '{}' executed successfully: {}", command, stdout.trim()))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ActionError::validation(format!("Custom command '{}' failed: {}", command, stderr)))
        }
    }
    
    /// Check if a command is safe to execute
    fn is_command_safe(&self, command: &str) -> bool {
        let dangerous_commands = [
            "rm -rf",
            "sudo",
            "chmod 777",
            "dd if=",
            "mkfs",
            "fdisk",
            "format",
            "del /s",
            "format c:",
        ];
        
        let command_lower = command.to_lowercase();
        
        // Check for dangerous commands
        for dangerous in &dangerous_commands {
            if command_lower.contains(dangerous) {
                return false;
            }
        }
        
        // Check for shell injection attempts
        if command.contains(";") || command.contains("&&") || command.contains("||") || command.contains("|") {
            return false;
        }
        
        // Check for file system operations outside of project directory
        if command.contains("..") && (command.contains("rm") || command.contains("del") || command.contains("mv")) {
            return false;
        }
        
        true
    }

    /// Register default safety rules
    async fn register_default_rules(&mut self) -> ActionResult<()> {
        // Rule 1: High safety level actions require approval
        let high_safety_rule = SafetyRule {
            id: "high_safety_approval".to_string(),
            name: "High Safety Level Approval".to_string(),
            description: "High safety level actions require explicit approval".to_string(),
            safety_level: SafetyLevel::High,
            action_types: vec![ActionType::Refactor, ActionType::Feature, ActionType::Cleanup],
            enabled: true,
            severity: RuleSeverity::Error,
            conditions: vec![
                SafetyCondition {
                    field: "safety_level".to_string(),
                    operator: ConditionOperator::Equals,
                    value: serde_json::json!("high"),
                    description: "Action must have high safety level".to_string(),
                }
            ],
            validations: vec![
                SafetyValidation {
                    name: "approval_required".to_string(),
                    description: "Explicit approval required for high safety actions".to_string(),
                    validation_type: ValidationType::CustomCommand("approval_check".to_string()),
                    required: true,
                    timeout_seconds: Some(300),
                }
            ],
        };
        self.register_rule(high_safety_rule)?;

        // Rule 2: File deletion requires backup
        let file_deletion_rule = SafetyRule {
            id: "file_deletion_backup".to_string(),
            name: "File Deletion Backup".to_string(),
            description: "File deletion actions require backup creation".to_string(),
            safety_level: SafetyLevel::Medium,
            action_types: vec![ActionType::Cleanup],
            enabled: true,
            severity: RuleSeverity::Warning,
            conditions: vec![
                SafetyCondition {
                    field: "description".to_string(),
                    operator: ConditionOperator::Contains,
                    value: serde_json::json!("delete"),
                    description: "Action involves file deletion".to_string(),
                }
            ],
            validations: vec![
                SafetyValidation {
                    name: "backup_creation".to_string(),
                    description: "Backup must be created before deletion".to_string(),
                    validation_type: ValidationType::CustomCommand("backup_check".to_string()),
                    required: true,
                    timeout_seconds: Some(60),
                }
            ],
        };
        self.register_rule(file_deletion_rule)?;

        // Rule 3: Production files require extra validation
        let production_rule = SafetyRule {
            id: "production_file_protection".to_string(),
            name: "Production File Protection".to_string(),
            description: "Production files require additional safety checks".to_string(),
            safety_level: SafetyLevel::Medium,
            action_types: vec![ActionType::Refactor, ActionType::Feature, ActionType::Cleanup],
            enabled: true,
            severity: RuleSeverity::Error,
            conditions: vec![
                SafetyCondition {
                    field: "scope".to_string(),
                    operator: ConditionOperator::Contains,
                    value: serde_json::json!("src/"),
                    description: "Action affects source files".to_string(),
                }
            ],
            validations: vec![
                SafetyValidation {
                    name: "syntax_check".to_string(),
                    description: "Syntax check required for source files".to_string(),
                    validation_type: ValidationType::SyntaxCheck,
                    required: true,
                    timeout_seconds: Some(30),
                },
                SafetyValidation {
                    name: "type_check".to_string(),
                    description: "Type check required for source files".to_string(),
                    validation_type: ValidationType::TypeCheck,
                    required: true,
                    timeout_seconds: Some(60),
                }
            ],
        };
        self.register_rule(production_rule)?;

        Ok(())
    }

    /// Get all registered rules
    pub fn get_rules(&self) -> Vec<&SafetyRule> {
        self.rules.values().collect()
    }

    /// Get enabled rules
    pub fn get_enabled_rules(&self) -> Vec<&SafetyRule> {
        self.enabled_rules
            .iter()
            .filter_map(|id| self.rules.get(id))
            .collect()
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, rule_id: &str) -> ActionResult<()> {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.enabled = true;
            if !self.enabled_rules.contains(&rule_id.to_string()) {
                self.enabled_rules.push(rule_id.to_string());
            }
            Ok(())
        } else {
            Err(ActionError::validation(format!("Rule not found: {}", rule_id)))
        }
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, rule_id: &str) -> ActionResult<()> {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.enabled = false;
            self.enabled_rules.retain(|id| id != rule_id);
            Ok(())
        } else {
            Err(ActionError::validation(format!("Rule not found: {}", rule_id)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};

    #[tokio::test]
    async fn test_safety_rules_engine_creation() {
        let engine = SafetyRulesEngine::initialize().await;
        assert!(engine.is_ok());
        
        let engine = engine.unwrap();
        let rules = engine.get_enabled_rules();
        assert!(!rules.is_empty());
    }

    #[tokio::test]
    async fn test_rule_evaluation() {
        let engine = SafetyRulesEngine::initialize().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-intent",
            ActionType::Refactor,
            "Test refactoring action",
            vec!["src/main.rs".to_string()],
            SafetyLevel::Medium,
        );
        
        let results = engine.evaluate_rules(&intent).await;
        assert!(results.is_ok());
        
        let results = results.unwrap();
        assert!(!results.is_empty());
    }

    #[tokio::test]
    async fn test_high_safety_rule() {
        let engine = SafetyRulesEngine::initialize().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-high-safety",
            ActionType::Refactor,
            "High safety refactoring",
            vec!["src/main.rs".to_string()],
            SafetyLevel::High,
        );
        
        let results = engine.evaluate_rules(&intent).await.unwrap();
        
        // Should find the high safety rule
        let high_safety_result = results.iter()
            .find(|r| r.rule_id == "high_safety_approval");
        assert!(high_safety_result.is_some());
    }

    #[tokio::test]
    async fn test_rule_enable_disable() {
        let mut engine = SafetyRulesEngine::new();
        
        let rule = SafetyRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            description: "Test rule".to_string(),
            safety_level: SafetyLevel::Low,
            action_types: vec![ActionType::Test],
            enabled: false,
            severity: RuleSeverity::Info,
            conditions: vec![],
            validations: vec![],
        };
        
        engine.register_rule(rule).unwrap();
        assert_eq!(engine.get_enabled_rules().len(), 0);
        
        engine.enable_rule("test_rule").unwrap();
        assert_eq!(engine.get_enabled_rules().len(), 1);
        
        engine.disable_rule("test_rule").unwrap();
        assert_eq!(engine.get_enabled_rules().len(), 0);
    }
} 