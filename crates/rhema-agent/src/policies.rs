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

use crate::agent::{AgentCapability, AgentId, AgentType};
use crate::error::{AgentError, AgentResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Policy enforcement mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEnforcement {
    /// Policy is enforced strictly
    Strict,
    /// Policy is enforced with warnings
    Warning,
    /// Policy is not enforced
    Disabled,
}

impl std::fmt::Display for PolicyEnforcement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyEnforcement::Strict => write!(f, "Strict"),
            PolicyEnforcement::Warning => write!(f, "Warning"),
            PolicyEnforcement::Disabled => write!(f, "Disabled"),
        }
    }
}

/// Policy violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    /// Violation ID
    pub violation_id: String,
    /// Agent ID that violated the policy
    pub agent_id: AgentId,
    /// Policy that was violated
    pub policy_id: String,
    /// Violation description
    pub description: String,
    /// Violation severity
    pub severity: PolicyViolationSeverity,
    /// Violation timestamp
    pub timestamp: DateTime<Utc>,
    /// Violation context
    pub context: HashMap<String, serde_json::Value>,
}

/// Policy violation severity
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolicyViolationSeverity {
    /// Low severity violation
    Low = 1,
    /// Medium severity violation
    Medium = 2,
    /// High severity violation
    High = 3,
    /// Critical severity violation
    Critical = 4,
}

impl std::fmt::Display for PolicyViolationSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyViolationSeverity::Low => write!(f, "Low"),
            PolicyViolationSeverity::Medium => write!(f, "Medium"),
            PolicyViolationSeverity::High => write!(f, "High"),
            PolicyViolationSeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Policy ID
    pub policy_id: String,
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Policy enforcement mode
    pub enforcement: PolicyEnforcement,
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    /// Policy scope
    pub scope: PolicyScope,
    /// Policy priority
    pub priority: u8,
    /// Whether policy is active
    pub active: bool,
    /// Policy metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule ID
    pub rule_id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule condition
    pub condition: PolicyCondition,
    /// Rule action
    pub action: PolicyAction,
    /// Rule priority
    pub priority: u8,
}

/// Policy condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyCondition {
    /// Agent type condition
    AgentType(AgentType),
    /// Agent capability condition
    AgentCapability(AgentCapability),
    /// Resource usage condition
    ResourceUsage(ResourceUsageCondition),
    /// Time-based condition
    TimeBased(TimeBasedCondition),
    /// Custom condition
    Custom(String, HashMap<String, serde_json::Value>),
}

/// Resource usage condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageCondition {
    /// Maximum CPU usage percentage
    pub max_cpu_percent: Option<f64>,
    /// Maximum memory usage in MB
    pub max_memory_mb: Option<u64>,
    /// Maximum disk usage in MB
    pub max_disk_mb: Option<u64>,
    /// Maximum network usage in MB/s
    pub max_network_mbps: Option<f64>,
}

/// Time-based condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBasedCondition {
    /// Allowed time ranges
    pub allowed_ranges: Vec<TimeRange>,
    /// Allowed days of week
    pub allowed_days: Vec<u8>, // 0 = Sunday, 1 = Monday, etc.
    /// Allowed months
    pub allowed_months: Vec<u8>, // 1 = January, 2 = February, etc.
}

/// Time range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start time (HH:MM format)
    pub start_time: String,
    /// End time (HH:MM format)
    pub end_time: String,
}

/// Policy action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    /// Allow action
    Allow,
    /// Deny action
    Deny,
    /// Warn action
    Warn,
    /// Limit action
    Limit(PolicyLimit),
    /// Custom action
    Custom(String, HashMap<String, serde_json::Value>),
}

/// Policy limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyLimit {
    /// Limit type
    pub limit_type: String,
    /// Limit value
    pub limit_value: serde_json::Value,
    /// Limit duration in seconds
    pub duration: Option<u64>,
}

/// Policy scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyScope {
    /// Global scope
    Global,
    /// Agent-specific scope
    Agent(AgentId),
    /// Agent type scope
    AgentType(AgentType),
    /// Custom scope
    Custom(String),
}

/// Policy engine for managing and enforcing policies
pub struct PolicyEngine {
    /// Registered policies
    policies: Arc<RwLock<HashMap<String, Policy>>>,
    /// Policy violations
    violations: Arc<RwLock<Vec<PolicyViolation>>>,
    /// Policy statistics
    stats: Arc<RwLock<PolicyStats>>,
}

/// Policy statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStats {
    /// Total policies
    pub total_policies: usize,
    /// Active policies
    pub active_policies: usize,
    /// Total violations
    pub total_violations: usize,
    /// Violations by severity
    pub violations_by_severity: HashMap<String, usize>,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl Default for PolicyStats {
    fn default() -> Self {
        Self {
            total_policies: 0,
            active_policies: 0,
            total_violations: 0,
            violations_by_severity: HashMap::new(),
            last_update: Utc::now(),
        }
    }
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            policies: Arc::new(RwLock::new(HashMap::new())),
            violations: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(PolicyStats::default())),
        }
    }

    /// Initialize the policy engine
    pub async fn initialize(&self) -> AgentResult<()> {
        // Clear any existing data
        self.policies.write().await.clear();
        self.violations.write().await.clear();

        // Reset statistics
        let mut stats = self.stats.write().await;
        *stats = PolicyStats::default();

        Ok(())
    }

    /// Register a policy
    pub async fn register_policy(&self, policy: Policy) -> AgentResult<()> {
        let policy_id = policy.policy_id.clone();

        // Check if policy already exists
        {
            let policies = self.policies.read().await;
            if policies.contains_key(&policy_id) {
                return Err(AgentError::PolicyViolation {
                    violation: "Policy already exists".to_string(),
                });
            }
        }

        // Add policy
        {
            let mut policies = self.policies.write().await;
            policies.insert(policy_id, policy);
        }

        // Update statistics
        self.update_stats().await;

        Ok(())
    }

    /// Unregister a policy
    pub async fn unregister_policy(&self, policy_id: &str) -> AgentResult<()> {
        let mut policies = self.policies.write().await;

        if policies.remove(policy_id).is_some() {
            // Update statistics
            self.update_stats().await;
            Ok(())
        } else {
            Err(AgentError::PolicyViolation {
                violation: "Policy not found".to_string(),
            })
        }
    }

    /// Get a policy
    pub async fn get_policy(&self, policy_id: &str) -> AgentResult<Policy> {
        let policies = self.policies.read().await;

        policies
            .get(policy_id)
            .cloned()
            .ok_or_else(|| AgentError::PolicyViolation {
                violation: "Policy not found".to_string(),
            })
    }

    /// Get all policies
    pub async fn get_all_policies(&self) -> Vec<Policy> {
        let policies = self.policies.read().await;
        policies.values().cloned().collect()
    }

    /// Get active policies
    pub async fn get_active_policies(&self) -> Vec<Policy> {
        let policies = self.policies.read().await;
        policies
            .values()
            .filter(|policy| policy.active)
            .cloned()
            .collect()
    }

    /// Evaluate policies for an agent
    pub async fn evaluate_policies(
        &self,
        agent_id: &AgentId,
        agent_type: &AgentType,
        capabilities: &[AgentCapability],
        context: &HashMap<String, serde_json::Value>,
    ) -> AgentResult<PolicyEvaluationResult> {
        let policies = self.get_active_policies().await;
        let mut violations = Vec::new();
        let mut allowed_actions = Vec::new();
        let mut denied_actions = Vec::new();

        for policy in policies {
            // Check if policy applies to this agent
            if !self
                .policy_applies_to_agent(&policy, agent_id, agent_type)
                .await
            {
                continue;
            }

            // Evaluate policy rules
            for rule in &policy.rules {
                if self
                    .rule_condition_matches(&rule.condition, agent_type, capabilities, context)
                    .await
                {
                    match &rule.action {
                        PolicyAction::Allow => {
                            allowed_actions.push((policy.policy_id.clone(), rule.rule_id.clone()));
                        }
                        PolicyAction::Deny => {
                            denied_actions.push((policy.policy_id.clone(), rule.rule_id.clone()));

                            // Record violation
                            let violation = PolicyViolation {
                                violation_id: Uuid::new_v4().to_string(),
                                agent_id: agent_id.clone(),
                                policy_id: policy.policy_id.clone(),
                                description: format!(
                                    "Policy '{}' rule '{}' denied action",
                                    policy.name, rule.name
                                ),
                                severity: PolicyViolationSeverity::High,
                                timestamp: Utc::now(),
                                context: context.clone(),
                            };
                            violations.push(violation);
                        }
                        PolicyAction::Warn => {
                            // Record warning violation
                            let violation = PolicyViolation {
                                violation_id: Uuid::new_v4().to_string(),
                                agent_id: agent_id.clone(),
                                policy_id: policy.policy_id.clone(),
                                description: format!(
                                    "Policy '{}' rule '{}' warning",
                                    policy.name, rule.name
                                ),
                                severity: PolicyViolationSeverity::Medium,
                                timestamp: Utc::now(),
                                context: context.clone(),
                            };
                            violations.push(violation);
                        }
                        PolicyAction::Limit(limit) => {
                            // Handle limit action
                            allowed_actions.push((policy.policy_id.clone(), rule.rule_id.clone()));
                        }
                        PolicyAction::Custom(action_type, action_data) => {
                            // Handle custom action
                            allowed_actions.push((policy.policy_id.clone(), rule.rule_id.clone()));
                        }
                    }
                }
            }
        }

        // Store violations
        if !violations.is_empty() {
            let mut violations_store = self.violations.write().await;
            violations_store.extend(violations.clone());
        }

        // Update statistics
        self.update_stats().await;

        Ok(PolicyEvaluationResult {
            agent_id: agent_id.clone(),
            violations,
            allowed_actions,
            denied_actions,
            timestamp: Utc::now(),
        })
    }

    /// Check if policy applies to agent
    async fn policy_applies_to_agent(
        &self,
        policy: &Policy,
        agent_id: &AgentId,
        agent_type: &AgentType,
    ) -> bool {
        match &policy.scope {
            PolicyScope::Global => true,
            PolicyScope::Agent(policy_agent_id) => agent_id == policy_agent_id,
            PolicyScope::AgentType(policy_agent_type) => agent_type == policy_agent_type,
            PolicyScope::Custom(_) => true, // Custom scope logic would be implemented here
        }
    }

    /// Check if rule condition matches
    async fn rule_condition_matches(
        &self,
        condition: &PolicyCondition,
        agent_type: &AgentType,
        capabilities: &[AgentCapability],
        _context: &HashMap<String, serde_json::Value>,
    ) -> bool {
        match condition {
            PolicyCondition::AgentType(condition_type) => agent_type == condition_type,
            PolicyCondition::AgentCapability(condition_capability) => {
                capabilities.contains(condition_capability)
            }
            PolicyCondition::ResourceUsage(_) => {
                // Resource usage checking would be implemented here
                true
            }
            PolicyCondition::TimeBased(_) => {
                // Time-based checking would be implemented here
                true
            }
            PolicyCondition::Custom(_, _) => {
                // Custom condition logic would be implemented here
                true
            }
        }
    }

    /// Get policy violations
    pub async fn get_violations(&self, limit: Option<usize>) -> Vec<PolicyViolation> {
        let violations = self.violations.read().await;

        if let Some(limit) = limit {
            violations.iter().rev().take(limit).cloned().collect()
        } else {
            violations.iter().rev().cloned().collect()
        }
    }

    /// Get violations for an agent
    pub async fn get_agent_violations(
        &self,
        agent_id: &AgentId,
        limit: Option<usize>,
    ) -> Vec<PolicyViolation> {
        let violations = self.violations.read().await;

        let filtered: Vec<PolicyViolation> = violations
            .iter()
            .filter(|v| &v.agent_id == agent_id)
            .cloned()
            .collect();

        if let Some(limit) = limit {
            filtered.into_iter().rev().take(limit).collect()
        } else {
            filtered.into_iter().rev().collect()
        }
    }

    /// Get violation count
    pub async fn get_violation_count(&self) -> usize {
        self.violations.read().await.len()
    }

    /// Get policy statistics
    pub async fn get_stats(&self) -> PolicyStats {
        self.stats.read().await.clone()
    }

    /// Update policy statistics
    async fn update_stats(&self) {
        let policies = self.policies.read().await;
        let violations = self.violations.read().await;
        let mut stats = self.stats.write().await;

        stats.total_policies = policies.len();
        stats.active_policies = policies.values().filter(|p| p.active).count();
        stats.total_violations = violations.len();
        stats.last_update = Utc::now();

        // Count violations by severity
        stats.violations_by_severity.clear();
        for violation in violations.iter() {
            *stats
                .violations_by_severity
                .entry(violation.severity.to_string())
                .or_insert(0) += 1;
        }
    }

    /// Shutdown the policy engine
    pub async fn shutdown(&self) -> AgentResult<()> {
        // Clear all data
        self.policies.write().await.clear();
        self.violations.write().await.clear();

        // Reset statistics
        let mut stats = self.stats.write().await;
        *stats = PolicyStats::default();

        Ok(())
    }
}

/// Policy evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationResult {
    /// Agent ID
    pub agent_id: AgentId,
    /// Policy violations
    pub violations: Vec<PolicyViolation>,
    /// Allowed actions
    pub allowed_actions: Vec<(String, String)>, // (policy_id, rule_id)
    /// Denied actions
    pub denied_actions: Vec<(String, String)>, // (policy_id, rule_id)
    /// Evaluation timestamp
    pub timestamp: DateTime<Utc>,
}

impl PolicyEvaluationResult {
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    pub fn has_critical_violations(&self) -> bool {
        self.violations
            .iter()
            .any(|v| v.severity == PolicyViolationSeverity::Critical)
    }

    pub fn is_allowed(&self) -> bool {
        self.denied_actions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentCapability, AgentType};

    #[tokio::test]
    async fn test_policy_engine_creation() {
        let engine = PolicyEngine::new();
        assert!(engine.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_policy_registration() {
        let engine = PolicyEngine::new();
        engine.initialize().await.unwrap();

        let policy = Policy {
            policy_id: "test-policy".to_string(),
            name: "Test Policy".to_string(),
            description: "A test policy".to_string(),
            enforcement: PolicyEnforcement::Strict,
            rules: vec![],
            scope: PolicyScope::Global,
            priority: 5,
            active: true,
            metadata: HashMap::new(),
        };

        assert!(engine.register_policy(policy).await.is_ok());

        let policies = engine.get_all_policies().await;
        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0].name, "Test Policy");
    }

    #[tokio::test]
    async fn test_policy_evaluation() {
        let engine = PolicyEngine::new();
        engine.initialize().await.unwrap();

        let policy = Policy {
            policy_id: "test-policy".to_string(),
            name: "Test Policy".to_string(),
            description: "A test policy".to_string(),
            enforcement: PolicyEnforcement::Strict,
            rules: vec![PolicyRule {
                rule_id: "test-rule".to_string(),
                name: "Test Rule".to_string(),
                description: "A test rule".to_string(),
                condition: PolicyCondition::AgentType(AgentType::Development),
                action: PolicyAction::Allow,
                priority: 5,
            }],
            scope: PolicyScope::Global,
            priority: 5,
            active: true,
            metadata: HashMap::new(),
        };

        engine.register_policy(policy).await.unwrap();

        let agent_id = "test-agent".to_string();
        let agent_type = AgentType::Development;
        let capabilities = vec![AgentCapability::CodeExecution];
        let context = HashMap::new();

        let result = engine
            .evaluate_policies(&agent_id, &agent_type, &capabilities, &context)
            .await
            .unwrap();

        assert!(!result.has_violations());
        assert!(result.is_allowed());
    }

    #[test]
    fn test_policy_violation_severity() {
        assert!(PolicyViolationSeverity::Critical > PolicyViolationSeverity::High);
        assert!(PolicyViolationSeverity::High > PolicyViolationSeverity::Medium);
        assert!(PolicyViolationSeverity::Medium > PolicyViolationSeverity::Low);
    }

    #[test]
    fn test_policy_enforcement_display() {
        assert_eq!(PolicyEnforcement::Strict.to_string(), "Strict");
        assert_eq!(PolicyEnforcement::Warning.to_string(), "Warning");
        assert_eq!(PolicyEnforcement::Disabled.to_string(), "Disabled");
    }
}
