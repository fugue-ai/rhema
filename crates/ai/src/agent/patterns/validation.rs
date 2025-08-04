use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use super::{
    PatternContext, ValidationResult, PatternMetadata, PatternCategory,
    AgentInfo, AgentStatus, Constraint, ConstraintType, PatternState,
    PatternPhase, PatternStatus, PatternConfig, MemoryPool, CpuAllocator, NetworkResources,
    AgentPerformanceMetrics
};

/// Pattern validation rule
pub trait ValidationRule: Send + Sync {
    /// Validate the pattern context against this rule
    fn validate(&self, context: &PatternContext, metadata: &PatternMetadata) -> ValidationResult;
    
    /// Get rule name
    fn name(&self) -> &str;
    
    /// Get rule description
    fn description(&self) -> &str;
    
    /// Get rule priority (higher = more important)
    fn priority(&self) -> u32;
}

/// Agent capability validation rule
pub struct AgentCapabilityRule;

impl ValidationRule for AgentCapabilityRule {
    fn validate(&self, context: &PatternContext, metadata: &PatternMetadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        // Check if all required capabilities are available
        for required_capability in &metadata.required_capabilities {
            let agents_with_capability: Vec<&AgentInfo> = context.agents.iter()
                .filter(|agent| agent.capabilities.contains(required_capability))
                .collect();

            if agents_with_capability.is_empty() {
                errors.push(format!(
                    "No agent found with required capability: {}",
                    required_capability
                ));
            } else {
                details.insert(
                    format!("capability_{}", required_capability),
                    serde_json::json!({
                        "available_agents": agents_with_capability.len(),
                        "agent_ids": agents_with_capability.iter().map(|a| &a.id).collect::<Vec<_>>()
                    })
                );
            }
        }

        // Check agent availability
        let idle_agents = context.agents.iter()
            .filter(|agent| agent.status == AgentStatus::Idle)
            .count();
        
        let busy_agents = context.agents.iter()
            .filter(|agent| agent.status == AgentStatus::Busy || agent.status == AgentStatus::Working)
            .count();

        if idle_agents == 0 {
            errors.push("No idle agents available for pattern execution".to_string());
        } else if idle_agents < 2 {
            warnings.push("Limited number of idle agents available".to_string());
        }

        details.insert(
            "agent_availability".to_string(),
            serde_json::json!({
                "total_agents": context.agents.len(),
                "idle_agents": idle_agents,
                "busy_agents": busy_agents,
                "required_capabilities": metadata.required_capabilities.len()
            })
        );

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        }
    }

    fn name(&self) -> &str {
        "agent_capability_validation"
    }

    fn description(&self) -> &str {
        "Validates that required agent capabilities are available and agents are in appropriate states"
    }

    fn priority(&self) -> u32 {
        100
    }
}

/// Resource availability validation rule
pub struct ResourceAvailabilityRule;

impl ValidationRule for ResourceAvailabilityRule {
    fn validate(&self, context: &PatternContext, metadata: &PatternMetadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        // Check memory availability
        let available_memory_mb = context.resources.memory_pool.available_memory / (1024 * 1024);
        let total_memory_mb = context.resources.memory_pool.total_memory / (1024 * 1024);
        let memory_utilization = context.resources.memory_pool.allocated_memory as f64 / context.resources.memory_pool.total_memory as f64;

        if available_memory_mb < 100 {
            errors.push("Insufficient available memory (less than 100MB)".to_string());
        } else if available_memory_mb < 500 {
            warnings.push("Low available memory (less than 500MB)".to_string());
        }

        if memory_utilization > 0.9 {
            warnings.push("High memory utilization (over 90%)".to_string());
        }

        // Check CPU availability
        let available_cores = context.resources.cpu_allocator.available_cores;
        let total_cores = context.resources.cpu_allocator.total_cores;
        let cpu_utilization = context.resources.cpu_allocator.allocated_cores as f64 / context.resources.cpu_allocator.total_cores as f64;

        if available_cores == 0 {
            errors.push("No CPU cores available".to_string());
        } else if available_cores < 2 {
            warnings.push("Limited CPU cores available".to_string());
        }

        if cpu_utilization > 0.8 {
            warnings.push("High CPU utilization (over 80%)".to_string());
        }

        // Check network bandwidth
        let available_bandwidth = context.resources.network_resources.available_bandwidth;
        let total_bandwidth = context.resources.network_resources.available_bandwidth + context.resources.network_resources.allocated_bandwidth;
        let bandwidth_utilization = if total_bandwidth > 0 {
            context.resources.network_resources.allocated_bandwidth as f64 / total_bandwidth as f64
        } else {
            0.0
        };

        if available_bandwidth < 100 {
            warnings.push("Low network bandwidth available (less than 100Mbps)".to_string());
        }

        if bandwidth_utilization > 0.7 {
            warnings.push("High network bandwidth utilization (over 70%)".to_string());
        }

        // Check custom resources
        for required_resource in &metadata.required_resources {
            if !["memory", "cpu", "network"].contains(&required_resource.as_str()) {
                if !context.resources.custom_resources.contains_key(required_resource) {
                    warnings.push(format!(
                        "Custom resource not found: {}",
                        required_resource
                    ));
                }
            }
        }

        details.insert(
            "resource_availability".to_string(),
            serde_json::json!({
                "memory": {
                    "available_mb": available_memory_mb,
                    "total_mb": total_memory_mb,
                    "utilization": memory_utilization
                },
                "cpu": {
                    "available_cores": available_cores,
                    "total_cores": total_cores,
                    "utilization": cpu_utilization
                },
                "network": {
                    "available_bandwidth_mbps": available_bandwidth,
                    "total_bandwidth_mbps": total_bandwidth,
                    "utilization": bandwidth_utilization
                }
            })
        );

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        }
    }

    fn name(&self) -> &str {
        "resource_availability_validation"
    }

    fn description(&self) -> &str {
        "Validates resource availability including memory, CPU, and network bandwidth"
    }

    fn priority(&self) -> u32 {
        90
    }
}

/// Constraint validation rule
pub struct ConstraintValidationRule;

impl ValidationRule for ConstraintValidationRule {
    fn validate(&self, context: &PatternContext, _metadata: &PatternMetadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        let mut constraint_results = Vec::new();

        for constraint in &context.constraints {
            let mut constraint_valid = true;
            let mut constraint_errors = Vec::new();
            let mut constraint_warnings = Vec::new();

            match constraint.constraint_type {
                ConstraintType::ResourceAvailability => {
                    if let Some(min_memory) = constraint.parameters.get("min_memory_mb") {
                        if let Some(memory_mb) = min_memory.as_u64() {
                            let available_memory_mb = context.resources.memory_pool.available_memory / (1024 * 1024);
                            if available_memory_mb < memory_mb {
                                constraint_valid = false;
                                constraint_errors.push(format!(
                                    "Memory constraint violated: required {}MB, available {}MB",
                                    memory_mb, available_memory_mb
                                ));
                            }
                        }
                    }
                }
                ConstraintType::AgentCapability => {
                    if let Some(required_capability) = constraint.parameters.get("capability") {
                        if let Some(capability) = required_capability.as_str() {
                                                            let has_capability = context.agents.iter().any(|agent| {
                                    agent.capabilities.contains(&capability.to_string())
                                });
                            if !has_capability {
                                constraint_valid = false;
                                constraint_errors.push(format!(
                                    "Agent capability constraint violated: {} not available",
                                    capability
                                ));
                            }
                        }
                    }
                }
                ConstraintType::Temporal => {
                    if let Some(max_duration) = constraint.parameters.get("max_duration_seconds") {
                        if let Some(duration) = max_duration.as_u64() {
                            // This would be checked during execution
                            constraint_warnings.push("Temporal constraint will be validated during execution".to_string());
                        }
                    }
                }
                ConstraintType::Performance => {
                    if let Some(min_efficiency) = constraint.parameters.get("min_efficiency") {
                        if let Some(efficiency) = min_efficiency.as_f64() {
                            // This would be checked during execution
                            constraint_warnings.push("Performance constraint will be validated during execution".to_string());
                        }
                    }
                }
                _ => {
                    constraint_warnings.push(format!(
                        "Constraint type {:?} validation not implemented",
                        constraint.constraint_type
                    ));
                }
            }

            if constraint.is_hard && !constraint_valid {
                errors.extend(constraint_errors.clone());
            } else if !constraint_valid {
                warnings.extend(constraint_errors.clone());
            }

            warnings.extend(constraint_warnings.clone());

            constraint_results.push(serde_json::json!({
                "constraint_id": constraint.id,
                "constraint_type": format!("{:?}", constraint.constraint_type),
                "is_hard": constraint.is_hard,
                "priority": constraint.priority,
                "valid": constraint_valid,
                "errors": constraint_errors,
                "warnings": constraint_warnings
            }));
        }

        details.insert("constraint_results".to_string(), serde_json::Value::Array(constraint_results));

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        }
    }

    fn name(&self) -> &str {
        "constraint_validation"
    }

    fn description(&self) -> &str {
        "Validates pattern constraints including resource, capability, temporal, and performance constraints"
    }

    fn priority(&self) -> u32 {
        80
    }
}

/// Pattern complexity validation rule
pub struct ComplexityValidationRule;

impl ValidationRule for ComplexityValidationRule {
    fn validate(&self, context: &PatternContext, metadata: &PatternMetadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        // Check complexity vs available resources
        if metadata.complexity > 7 {
            if context.resources.cpu_allocator.available_cores < 2 {
                warnings.push("High complexity pattern with limited CPU resources".to_string());
            }
            if context.resources.memory_pool.available_memory < 500 * 1024 * 1024 {
                warnings.push("High complexity pattern with limited memory".to_string());
            }
        }

        // Check complexity vs agent count
        if metadata.complexity > 5 && context.agents.len() < 2 {
            warnings.push("High complexity pattern with limited agent availability".to_string());
        }

        // Check estimated execution time vs timeout
        if let Some(timeout_seconds) = context.config.timeout_seconds.checked_sub(metadata.estimated_execution_time_seconds) {
            if timeout_seconds < 10 {
                warnings.push("Estimated execution time close to timeout limit".to_string());
            }
        } else {
            errors.push("Estimated execution time exceeds timeout limit".to_string());
        }

        details.insert(
            "complexity_analysis".to_string(),
            serde_json::json!({
                "pattern_complexity": metadata.complexity,
                "estimated_execution_time": metadata.estimated_execution_time_seconds,
                "timeout_seconds": context.config.timeout_seconds,
                "available_cpu_cores": context.resources.cpu_allocator.available_cores,
                "available_memory_mb": context.resources.memory_pool.available_memory / (1024 * 1024),
                "agent_count": context.agents.len()
            })
        );

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        }
    }

    fn name(&self) -> &str {
        "complexity_validation"
    }

    fn description(&self) -> &str {
        "Validates pattern complexity against available resources and constraints"
    }

    fn priority(&self) -> u32 {
        70
    }
}

/// Pattern configuration validation rule
pub struct ConfigurationValidationRule;

impl ValidationRule for ConfigurationValidationRule {
    fn validate(&self, context: &PatternContext, metadata: &PatternMetadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        // Validate timeout configuration
        if context.config.timeout_seconds == 0 {
            errors.push("Pattern timeout must be greater than 0".to_string());
        } else if context.config.timeout_seconds < metadata.estimated_execution_time_seconds {
            warnings.push(format!(
                "Pattern timeout ({}) is less than estimated execution time ({})",
                context.config.timeout_seconds,
                metadata.estimated_execution_time_seconds
            ));
        }

        // Validate retry configuration
        if context.config.max_retries > 10 {
            warnings.push("Maximum retries is set very high, which may indicate configuration issues".to_string());
        }

        // Validate custom configuration
        if let Some(custom_config) = context.config.custom_config.get("required_dependencies") {
            if let Some(deps) = custom_config.as_array() {
                let mut missing_deps = Vec::new();
                for dep in deps {
                    if let Some(dep_name) = dep.as_str() {
                        // Check if dependency is available in context
                        if !context.state.data.contains_key(&format!("dependency_{}", dep_name)) {
                            missing_deps.push(dep_name.to_string());
                        }
                    }
                }
                if !missing_deps.is_empty() {
                    errors.push(format!("Missing required dependencies: {}", missing_deps.join(", ")));
                }
            }
        }

        // Validate monitoring configuration
        if context.config.enable_monitoring {
            if !context.config.custom_config.contains_key("monitoring_endpoint") {
                warnings.push("Monitoring enabled but no monitoring endpoint configured".to_string());
            }
        }

        // Validate rollback configuration
        if context.config.enable_rollback {
            if !context.config.custom_config.contains_key("checkpoint_interval") {
                warnings.push("Rollback enabled but no checkpoint interval configured".to_string());
            }
        }

        details.insert(
            "configuration_validation".to_string(),
            serde_json::json!({
                "timeout_seconds": context.config.timeout_seconds,
                "max_retries": context.config.max_retries,
                "enable_monitoring": context.config.enable_monitoring,
                "enable_rollback": context.config.enable_rollback,
                "custom_config_keys": context.config.custom_config.keys().collect::<Vec<_>>()
            })
        );

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        }
    }

    fn name(&self) -> &str {
        "configuration_validation"
    }

    fn description(&self) -> &str {
        "Validates pattern configuration parameters and custom settings"
    }

    fn priority(&self) -> u32 {
        8
    }
}

/// Pattern dependency validation rule
pub struct DependencyValidationRule;

impl ValidationRule for DependencyValidationRule {
    fn validate(&self, context: &PatternContext, metadata: &PatternMetadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        // Check pattern dependencies
        if let Some(dependencies) = context.state.data.get("pattern_dependencies") {
            if let Some(dep_array) = dependencies.as_array() {
                let mut missing_deps = Vec::new();
                let mut circular_deps = Vec::new();
                
                for dep in dep_array {
                    if let Some(dep_obj) = dep.as_object() {
                        if let Some(dep_id) = dep_obj.get("pattern_id").and_then(|v| v.as_str()) {
                            // Check if dependent pattern exists in registry
                            if !context.state.data.contains_key(&format!("pattern_{}", dep_id)) {
                                missing_deps.push(dep_id.to_string());
                            }
                            
                            // Check for circular dependencies
                            if dep_id == metadata.name {
                                circular_deps.push(dep_id.to_string());
                            }
                        }
                    }
                }
                
                if !missing_deps.is_empty() {
                    errors.push(format!("Missing pattern dependencies: {}", missing_deps.join(", ")));
                }
                
                if !circular_deps.is_empty() {
                    errors.push(format!("Circular pattern dependencies detected: {}", circular_deps.join(", ")));
                }
            }
        }

        // Check resource dependencies
        for required_resource in &metadata.required_resources {
            let resource_available = match required_resource.as_str() {
                "memory" => context.resources.memory_pool.available_memory > 0,
                "cpu" => context.resources.cpu_allocator.available_cores > 0,
                "network" => context.resources.network_resources.available_bandwidth > 0,
                _ => context.resources.custom_resources.contains_key(required_resource),
            };
            
            if !resource_available {
                errors.push(format!("Required resource not available: {}", required_resource));
            }
        }

        // Check agent dependencies
        if let Some(agent_deps) = context.state.data.get("agent_dependencies") {
            if let Some(agent_array) = agent_deps.as_array() {
                for agent_dep in agent_array {
                    if let Some(agent_id) = agent_dep.as_str() {
                        let agent_exists = context.agents.iter().any(|agent| agent.id == agent_id);
                        if !agent_exists {
                            errors.push(format!("Required agent not available: {}", agent_id));
                        }
                    }
                }
            }
        }

        // Check external service dependencies
        if let Some(service_deps) = context.state.data.get("service_dependencies") {
            if let Some(service_array) = service_deps.as_array() {
                for service_dep in service_array {
                    if let Some(service_obj) = service_dep.as_object() {
                        if let Some(service_name) = service_obj.get("name").and_then(|v| v.as_str()) {
                            if let Some(required) = service_obj.get("required").and_then(|v| v.as_bool()) {
                                if required {
                                    // Check if service is available (simplified check)
                                    let service_available = context.state.data.contains_key(&format!("service_{}", service_name));
                                    if !service_available {
                                        errors.push(format!("Required external service not available: {}", service_name));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        details.insert(
            "dependency_validation".to_string(),
            serde_json::json!({
                "required_resources": metadata.required_resources,
                "required_capabilities": metadata.required_capabilities,
                "pattern_dependencies": context.state.data.get("pattern_dependencies"),
                "agent_dependencies": context.state.data.get("agent_dependencies"),
                "service_dependencies": context.state.data.get("service_dependencies")
            })
        );

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        }
    }

    fn name(&self) -> &str {
        "dependency_validation"
    }

    fn description(&self) -> &str {
        "Validates pattern dependencies including other patterns, resources, agents, and external services"
    }

    fn priority(&self) -> u32 {
        9
    }
}

/// Pattern state validation rule
pub struct StateValidationRule;

impl ValidationRule for StateValidationRule {
    fn validate(&self, context: &PatternContext, _metadata: &PatternMetadata) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = HashMap::new();

        // Validate pattern state
        match context.state.status {
            PatternStatus::Running => {
                if context.state.phase == PatternPhase::Completed {
                    errors.push("Pattern state inconsistency: status is Running but phase is Completed".to_string());
                }
            }
            PatternStatus::Completed => {
                if context.state.phase != PatternPhase::Completed {
                    errors.push("Pattern state inconsistency: status is Completed but phase is not Completed".to_string());
                }
            }
            PatternStatus::Failed => {
                if context.state.phase != PatternPhase::Failed {
                    errors.push("Pattern state inconsistency: status is Failed but phase is not Failed".to_string());
                }
            }
            PatternStatus::Cancelled => {
                if context.state.phase != PatternPhase::Failed {
                    warnings.push("Pattern is cancelled but phase is not Failed".to_string());
                }
            }
            _ => {}
        }

        // Validate progress consistency
        if context.state.progress < 0.0 || context.state.progress > 1.0 {
            errors.push("Pattern progress must be between 0.0 and 1.0".to_string());
        }

        // Validate timestamps
        if let Some(ended_at) = context.state.ended_at {
            if ended_at < context.state.started_at {
                errors.push("Pattern end time cannot be before start time".to_string());
            }
        }

        // Validate agent states
        for agent in &context.agents {
            if agent.current_workload < 0.0 || agent.current_workload > 1.0 {
                warnings.push(format!("Agent {} workload is outside normal range: {}", agent.id, agent.current_workload));
            }
            
            if agent.performance_metrics.success_rate < 0.0 || agent.performance_metrics.success_rate > 1.0 {
                warnings.push(format!("Agent {} success rate is outside normal range: {}", agent.id, agent.performance_metrics.success_rate));
            }
        }

        details.insert(
            "state_validation".to_string(),
            serde_json::json!({
                "pattern_status": format!("{:?}", context.state.status),
                "pattern_phase": format!("{:?}", context.state.phase),
                "progress": context.state.progress,
                "started_at": context.state.started_at,
                "ended_at": context.state.ended_at,
                "agent_count": context.agents.len(),
                "resource_utilization": {
                    "memory": context.resources.memory_pool.allocated_memory as f64 / context.resources.memory_pool.total_memory as f64,
                    "cpu": context.resources.cpu_allocator.allocated_cores as f64 / context.resources.cpu_allocator.total_cores as f64,
                    "network": context.resources.network_resources.allocated_bandwidth as f64 / context.resources.network_resources.available_bandwidth as f64
                }
            })
        );

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            details,
        }
    }

    fn name(&self) -> &str {
        "state_validation"
    }

    fn description(&self) -> &str {
        "Validates pattern and agent state consistency and data integrity"
    }

    fn priority(&self) -> u32 {
        7
    }
}

/// Pattern validation engine
pub struct PatternValidationEngine {
    rules: Vec<Box<dyn ValidationRule>>,
    validation_history: Arc<RwLock<Vec<ValidationRecord>>>,
}

impl PatternValidationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            validation_history: Arc::new(RwLock::new(Vec::new())),
        };

        // Register default validation rules
        engine.register_rule(Box::new(AgentCapabilityRule));
        engine.register_rule(Box::new(ResourceAvailabilityRule));
        engine.register_rule(Box::new(ConstraintValidationRule));
        engine.register_rule(Box::new(ComplexityValidationRule));
        engine.register_rule(Box::new(ConfigurationValidationRule));
        engine.register_rule(Box::new(DependencyValidationRule));
        engine.register_rule(Box::new(StateValidationRule));

        engine
    }

    /// Register a validation rule
    pub fn register_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
        // Sort rules by priority (highest first)
        self.rules.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    /// Validate pattern context
    pub async fn validate_pattern(
        &self,
        pattern_id: &str,
        context: &PatternContext,
        metadata: &PatternMetadata,
    ) -> ValidationResult {
        let start_time = Utc::now();
        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();
        let mut all_details = HashMap::new();
        let mut rule_results = Vec::new();

        // Run all validation rules
        for rule in &self.rules {
            let rule_result = rule.validate(context, metadata);
            
            rule_results.push(serde_json::json!({
                "rule_name": rule.name(),
                "rule_description": rule.description(),
                "rule_priority": rule.priority(),
                "is_valid": rule_result.is_valid,
                "error_count": rule_result.errors.len(),
                "warning_count": rule_result.warnings.len()
            }));

            all_errors.extend(rule_result.errors);
            all_warnings.extend(rule_result.warnings);
            all_details.extend(rule_result.details);
        }

        // Add rule summary to details
        all_details.insert("rule_results".to_string(), serde_json::Value::Array(rule_results));

        let end_time = Utc::now();
        let validation_duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0;

        let result = ValidationResult {
            is_valid: all_errors.is_empty(),
            errors: all_errors,
            warnings: all_warnings,
            details: all_details,
        };

        // Record validation
        let record = ValidationRecord {
            pattern_id: pattern_id.to_string(),
            timestamp: start_time,
            duration_seconds: validation_duration,
            is_valid: result.is_valid,
            error_count: result.errors.len(),
            warning_count: result.warnings.len(),
            rule_count: self.rules.len(),
        };

        {
            let mut history = self.validation_history.write().await;
            history.push(record);
            
            // Keep only last 1000 records
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        result
    }

    /// Get validation statistics
    pub async fn get_validation_statistics(&self) -> ValidationStatistics {
        let history = self.validation_history.read().await;
        
        let mut stats = ValidationStatistics {
            total_validations: history.len(),
            successful_validations: 0,
            failed_validations: 0,
            average_validation_time: 0.0,
            total_errors: 0,
            total_warnings: 0,
            most_common_errors: HashMap::new(),
        };

        let mut total_duration = 0.0;

        for record in history.iter() {
            total_duration += record.duration_seconds;
            
            if record.is_valid {
                stats.successful_validations += 1;
            } else {
                stats.failed_validations += 1;
            }

            stats.total_errors += record.error_count;
            stats.total_warnings += record.warning_count;
        }

        if stats.total_validations > 0 {
            stats.average_validation_time = total_duration / stats.total_validations as f64;
        }

        stats
    }

    /// Get validation history
    pub async fn get_validation_history(&self, limit: Option<usize>) -> Vec<ValidationRecord> {
        let history = self.validation_history.read().await;
        let limit = limit.unwrap_or(100);
        history.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for PatternValidationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation record for tracking validation history
#[derive(Debug, Clone)]
pub struct ValidationRecord {
    pub pattern_id: String,
    pub timestamp: DateTime<Utc>,
    pub duration_seconds: f64,
    pub is_valid: bool,
    pub error_count: usize,
    pub warning_count: usize,
    pub rule_count: usize,
}

/// Validation statistics
#[derive(Debug, Clone)]
pub struct ValidationStatistics {
    pub total_validations: usize,
    pub successful_validations: usize,
    pub failed_validations: usize,
    pub average_validation_time: f64,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub most_common_errors: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validation_engine_creation() {
        let engine = PatternValidationEngine::new();
        assert!(!engine.rules.is_empty());
        assert_eq!(engine.rules.len(), 7); // Default validation rules
    }

    #[tokio::test]
    async fn test_agent_capability_validation() {
        let rule = AgentCapabilityRule;
        let metadata = PatternMetadata {
            name: "Test Pattern".to_string(),
            description: "Test pattern".to_string(),
            version: "1.0.0".to_string(),
            category: PatternCategory::TaskDistribution,
            required_capabilities: vec!["test_capability".to_string()],
            required_resources: vec![],
            complexity: 5,
            estimated_execution_time_seconds: 10,
        };

        let context = PatternContext {
            agents: vec![
                AgentInfo {
                    id: "agent1".to_string(),
                    name: "Test Agent".to_string(),
                    capabilities: vec!["test_capability".to_string()],
                    status: AgentStatus::Idle,
                    performance_metrics: AgentPerformanceMetrics::default(),
                    current_workload: 0.0,
                    assigned_tasks: vec![],
                }
            ],
            resources: ResourcePool {
                file_locks: HashMap::new(),
                memory_pool: MemoryPool {
                    total_memory: 1024 * 1024 * 1024,
                    available_memory: 512 * 1024 * 1024,
                    allocated_memory: 512 * 1024 * 1024,
                    reservations: HashMap::new(),
                },
                cpu_allocator: CpuAllocator {
                    total_cores: 8,
                    available_cores: 4,
                    allocated_cores: 4,
                    reservations: HashMap::new(),
                },
                network_resources: NetworkResources {
                    available_bandwidth: 1000,
                    allocated_bandwidth: 500,
                    connections: HashMap::new(),
                },
                custom_resources: HashMap::new(),
            },
            constraints: vec![],
            state: PatternState {
                pattern_id: "test".to_string(),
                phase: PatternPhase::Initializing,
                started_at: Utc::now(),
                ended_at: None,
                progress: 0.0,
                status: PatternStatus::Pending,
                data: HashMap::new(),
            },
            config: PatternConfig {
                timeout_seconds: 30,
                max_retries: 3,
                enable_rollback: true,
                enable_monitoring: true,
                custom_config: HashMap::new(),
            },
            session_id: None,
            parent_pattern_id: None,
        };

        let result = rule.validate(&context, &metadata);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validation_engine_integration() {
        let engine = PatternValidationEngine::new();
        
        let metadata = PatternMetadata {
            name: "Test Pattern".to_string(),
            description: "Test pattern".to_string(),
            version: "1.0.0".to_string(),
            category: PatternCategory::TaskDistribution,
            required_capabilities: vec!["test_capability".to_string()],
            required_resources: vec!["memory".to_string(), "cpu".to_string()],
            complexity: 5,
            estimated_execution_time_seconds: 10,
        };

        let context = PatternContext {
            agents: vec![
                AgentInfo {
                    id: "agent1".to_string(),
                    name: "Test Agent".to_string(),
                    capabilities: vec!["test_capability".to_string()],
                    status: AgentStatus::Idle,
                    performance_metrics: AgentPerformanceMetrics::default(),
                    current_workload: 0.0,
                    assigned_tasks: vec![],
                }
            ],
            resources: ResourcePool {
                file_locks: HashMap::new(),
                memory_pool: MemoryPool {
                    total_memory: 1024 * 1024 * 1024,
                    available_memory: 512 * 1024 * 1024,
                    allocated_memory: 512 * 1024 * 1024,
                    reservations: HashMap::new(),
                },
                cpu_allocator: CpuAllocator {
                    total_cores: 8,
                    available_cores: 4,
                    allocated_cores: 4,
                    reservations: HashMap::new(),
                },
                network_resources: NetworkResources {
                    available_bandwidth: 1000,
                    allocated_bandwidth: 500,
                    connections: HashMap::new(),
                },
                custom_resources: HashMap::new(),
            },
            constraints: vec![],
            state: PatternState {
                pattern_id: "test".to_string(),
                phase: PatternPhase::Initializing,
                started_at: Utc::now(),
                ended_at: None,
                progress: 0.0,
                status: PatternStatus::Pending,
                data: HashMap::new(),
            },
            config: PatternConfig {
                timeout_seconds: 30,
                max_retries: 3,
                enable_rollback: true,
                enable_monitoring: true,
                custom_config: HashMap::new(),
            },
            session_id: None,
            parent_pattern_id: None,
        };

        let result = engine.validate_pattern("test_pattern", &context, &metadata).await;
        assert!(result.is_valid);
        
        let stats = engine.get_validation_statistics().await;
        assert_eq!(stats.total_validations, 1);
        assert_eq!(stats.successful_validations, 1);
    }
} 