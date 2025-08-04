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

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::schema::ActionIntent;
use crate::error::{ActionError, ActionResult};
use crate::tools::{ToolRegistry, ToolResult};

/// Result from validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub success: bool,
    pub message: String,
    pub details: HashMap<String, serde_json::Value>,
    pub duration: std::time::Duration,
}

/// Result from safety check
#[derive(Debug, Clone)]
pub struct SafetyCheckResult {
    pub success: bool,
    pub message: String,
    pub severity: SafetySeverity,
    pub details: HashMap<String, serde_json::Value>,
    pub duration: std::time::Duration,
}

/// Safety check severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Validation engine for running validation and safety checks
pub struct ValidationEngine {
    tool_registry: Arc<ToolRegistry>,
    validation_cache: Arc<RwLock<HashMap<String, ValidationResult>>>,
    safety_cache: Arc<RwLock<HashMap<String, SafetyCheckResult>>>,
}

impl ValidationEngine {
    /// Create a new validation engine
    pub async fn new() -> ActionResult<Self> {
        info!("Initializing Validation Engine");
        
        let tool_registry = Arc::new(ToolRegistry::new().await?);
        
        let engine = Self {
            tool_registry,
            validation_cache: Arc::new(RwLock::new(HashMap::new())),
            safety_cache: Arc::new(RwLock::new(HashMap::new())),
        };
        
        info!("Validation Engine initialized successfully");
        Ok(engine)
    }

    /// Initialize the validation engine (stub)
    pub async fn initialize() -> ActionResult<()> {
        info!("ValidationEngine initialized (stub)");
        Ok(())
    }

    /// Shutdown the validation engine (stub)
    pub async fn shutdown() -> ActionResult<()> {
        info!("ValidationEngine shutdown (stub)");
        Ok(())
    }
    
    /// Run a validation tool
    pub async fn run_validation(&self, tool_name: &str, intent: &ActionIntent) -> ActionResult<ValidationResult> {
        info!("Running validation tool: {} for intent: {}", tool_name, intent.id);
        
        let start_time = std::time::Instant::now();
        
        // Check cache first
        let cache_key = format!("{}:{}", tool_name, intent.id);
        {
            let cache = self.validation_cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                info!("Using cached validation result for tool: {}", tool_name);
                return Ok(cached_result.clone());
            }
        }
        
        // Execute validation tool
        let tool_result = self.tool_registry.execute_validation(tool_name, intent).await?;
        
        let duration = start_time.elapsed();
        
        let result = ValidationResult {
            success: tool_result.success,
            message: if tool_result.success {
                format!("Validation {} passed", tool_name)
            } else {
                format!("Validation {} failed: {}", tool_name, tool_result.errors.join("; "))
            },
            details: HashMap::new(), // TODO: Add detailed validation results
            duration,
        };
        
        // Cache the result
        {
            let mut cache = self.validation_cache.write().await;
            cache.insert(cache_key, result.clone());
        }
        
        if result.success {
            info!("Validation tool {} completed successfully", tool_name);
        } else {
            warn!("Validation tool {} failed", tool_name);
        }
        
        Ok(result)
    }
    
    /// Run a safety check
    pub async fn run_safety_check(&self, check_name: &str, intent: &ActionIntent) -> ActionResult<SafetyCheckResult> {
        info!("Running safety check: {} for intent: {}", check_name, intent.id);
        
        let start_time = std::time::Instant::now();
        
        // Check cache first
        let cache_key = format!("{}:{}", check_name, intent.id);
        {
            let cache = self.safety_cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                info!("Using cached safety check result for: {}", check_name);
                return Ok(cached_result.clone());
            }
        }
        
        // Execute safety check
        let tool_result = self.tool_registry.execute_safety_check(check_name, intent).await?;
        
        let duration = start_time.elapsed();
        
        let severity = self.determine_safety_severity(check_name, &tool_result);
        
        let result = SafetyCheckResult {
            success: tool_result.success,
            message: if tool_result.success {
                format!("Safety check {} passed", check_name)
            } else {
                format!("Safety check {} failed: {}", check_name, tool_result.errors.join("; "))
            },
            severity,
            details: HashMap::new(), // TODO: Add detailed safety check results
            duration,
        };
        
        // Cache the result
        {
            let mut cache = self.safety_cache.write().await;
            cache.insert(cache_key, result.clone());
        }
        
        if result.success {
            info!("Safety check {} completed successfully", check_name);
        } else {
            warn!("Safety check {} failed with severity: {:?}", check_name, result.severity);
        }
        
        Ok(result)
    }
    
    /// Determine safety severity based on check type and results
    fn determine_safety_severity(&self, check_name: &str, tool_result: &ToolResult) -> SafetySeverity {
        match check_name {
            "syntax_validation" => {
                if tool_result.success {
                    SafetySeverity::Low
                } else {
                    SafetySeverity::Critical
                }
            }
            "type_checking" => {
                if tool_result.success {
                    SafetySeverity::Low
                } else {
                    SafetySeverity::High
                }
            }
            "test_coverage" => {
                if tool_result.success {
                    SafetySeverity::Low
                } else {
                    SafetySeverity::Medium
                }
            }
            "security_scanning" => {
                if tool_result.success {
                    SafetySeverity::Low
                } else {
                    SafetySeverity::Critical
                }
            }
            "performance_check" => {
                if tool_result.success {
                    SafetySeverity::Low
                } else {
                    SafetySeverity::Medium
                }
            }
            "dependency_check" => {
                if tool_result.success {
                    SafetySeverity::Low
                } else {
                    SafetySeverity::High
                }
            }
            _ => {
                if tool_result.success {
                    SafetySeverity::Low
                } else {
                    SafetySeverity::Medium
                }
            }
        }
    }
    
    /// Run comprehensive validation for an intent
    pub async fn run_comprehensive_validation(&self, intent: &ActionIntent) -> ActionResult<ComprehensiveValidationResult> {
        info!("Running comprehensive validation for intent: {}", intent.id);
        
        let start_time = std::time::Instant::now();
        let mut validation_results = HashMap::new();
        let mut safety_results = HashMap::new();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Run all validation tools
        for validation_tool in &intent.transformation.validation {
            match self.run_validation(validation_tool, intent).await {
                Ok(result) => {
                    validation_results.insert(validation_tool.clone(), result.success);
                    if !result.success {
                        errors.push(format!("Validation {} failed: {}", validation_tool, result.message));
                    }
                }
                Err(e) => {
                    errors.push(format!("Validation {} error: {}", validation_tool, e));
                }
            }
        }
        
        // Run all safety checks
        for check in &intent.safety_checks.pre_execution {
            match self.run_safety_check(check, intent).await {
                Ok(result) => {
                    safety_results.insert(check.clone(), result.success);
                    if !result.success {
                        errors.push(format!("Safety check {} failed: {}", check, result.message));
                    }
                }
                Err(e) => {
                    errors.push(format!("Safety check {} error: {}", check, e));
                }
            }
        }
        
        for check in &intent.safety_checks.post_execution {
            match self.run_safety_check(check, intent).await {
                Ok(result) => {
                    safety_results.insert(check.clone(), result.success);
                    if !result.success {
                        errors.push(format!("Safety check {} failed: {}", check, result.message));
                    }
                }
                Err(e) => {
                    errors.push(format!("Safety check {} error: {}", check, e));
                }
            }
        }
        
        let duration = start_time.elapsed();
        let success = errors.is_empty();
        
        let result = ComprehensiveValidationResult {
            success,
            validation_results,
            safety_results,
            errors,
            warnings,
            duration,
        };
        
        if success {
            info!("Comprehensive validation completed successfully for intent: {}", intent.id);
        } else {
            warn!("Comprehensive validation failed for intent: {}", intent.id);
        }
        
        Ok(result)
    }
    
    /// Clear validation cache
    pub async fn clear_validation_cache(&self) {
        let mut cache = self.validation_cache.write().await;
        cache.clear();
        info!("Validation cache cleared");
    }
    
    /// Clear safety cache
    pub async fn clear_safety_cache(&self) {
        let mut cache = self.safety_cache.write().await;
        cache.clear();
        info!("Safety cache cleared");
    }
    
    /// Clear all caches
    pub async fn clear_all_caches(&self) {
        self.clear_validation_cache().await;
        self.clear_safety_cache().await;
        info!("All validation caches cleared");
    }
    
    /// Get validation cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        let validation_cache = self.validation_cache.read().await;
        let safety_cache = self.safety_cache.read().await;
        
        CacheStats {
            validation_cache_size: validation_cache.len(),
            safety_cache_size: safety_cache.len(),
            total_cache_size: validation_cache.len() + safety_cache.len(),
        }
    }
}

/// Comprehensive validation result
#[derive(Debug, Clone)]
pub struct ComprehensiveValidationResult {
    pub success: bool,
    pub validation_results: HashMap<String, bool>,
    pub safety_results: HashMap<String, bool>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration: std::time::Duration,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub validation_cache_size: usize,
    pub safety_cache_size: usize,
    pub total_cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};

    #[tokio::test]
    async fn test_validation_engine_creation() {
        let engine = ValidationEngine::new().await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_validation_engine_validation() {
        let engine = ValidationEngine::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-validation",
            ActionType::Test,
            "Test validation",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        let result = engine.run_validation("typescript", &intent).await;
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(validation_result.success);
    }

    #[tokio::test]
    async fn test_validation_engine_safety_check() {
        let engine = ValidationEngine::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-safety",
            ActionType::Test,
            "Test safety check",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        let result = engine.run_safety_check("syntax_validation", &intent).await;
        assert!(result.is_ok());
        
        let safety_result = result.unwrap();
        assert!(safety_result.success);
        assert_eq!(safety_result.severity, SafetySeverity::Low);
    }

    #[tokio::test]
    async fn test_comprehensive_validation() {
        let engine = ValidationEngine::new().await.unwrap();
        
        let mut intent = ActionIntent::new(
            "test-comprehensive",
            ActionType::Test,
            "Test comprehensive validation",
            vec!["src/".to_string()],
            SafetyLevel::Low,
        );
        
        intent.add_validation("typescript");
        intent.add_pre_execution_check("syntax_validation");
        
        let result = engine.run_comprehensive_validation(&intent).await;
        assert!(result.is_ok());
        
        let comprehensive_result = result.unwrap();
        assert!(comprehensive_result.success);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let engine = ValidationEngine::new().await.unwrap();
        
        // Test cache stats
        let stats = engine.get_cache_stats().await;
        assert_eq!(stats.total_cache_size, 0);
        
        // Test cache clearing
        engine.clear_all_caches().await;
        
        let stats_after_clear = engine.get_cache_stats().await;
        assert_eq!(stats_after_clear.total_cache_size, 0);
    }
} 