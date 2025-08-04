//! Test fixtures and utilities for Rhema Coordination CLI testing

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tempfile::TempDir;
use rand::Rng;
use rand::thread_rng;

/// Sample agent data for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleAgent {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub scope: String,
    pub capabilities: Vec<String>,
    pub status: String,
}

/// Sample session data for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleSession {
    pub id: String,
    pub topic: String,
    pub participants: Vec<String>,
    pub status: String,
    pub created_at: String,
}

/// Sample message data for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleMessage {
    pub id: String,
    pub from: String,
    pub to: String,
    pub content: String,
    pub message_type: String,
    pub priority: String,
    pub payload: Option<serde_json::Value>,
    pub timestamp: String,
}

/// Coordination test fixtures
#[derive(Debug)]
pub struct CoordinationFixtures {
    pub agents: Vec<SampleAgent>,
    pub sessions: Vec<SampleSession>,
    pub messages: Vec<SampleMessage>,
    pub temp_dir: TempDir,
}

impl CoordinationFixtures {
    /// Create new coordination test fixtures
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        let agents = Self::create_sample_agents();
        let sessions = Self::create_sample_sessions();
        let messages = Self::create_sample_messages();
        
        Ok(Self {
            agents,
            sessions,
            messages,
            temp_dir,
        })
    }
    
    /// Create sample agents for testing
    fn create_sample_agents() -> Vec<SampleAgent> {
        vec![
            SampleAgent {
                id: "agent-001".to_string(),
                name: "CodeReviewAgent".to_string(),
                agent_type: "CodeReviewAgent".to_string(),
                scope: "backend".to_string(),
                capabilities: vec!["code-review".to_string(), "static-analysis".to_string()],
                status: "Available".to_string(),
            },
            SampleAgent {
                id: "agent-002".to_string(),
                name: "TestRunnerAgent".to_string(),
                agent_type: "TestAgent".to_string(),
                scope: "testing".to_string(),
                capabilities: vec!["unit-testing".to_string(), "integration-testing".to_string()],
                status: "Busy".to_string(),
            },
            SampleAgent {
                id: "agent-003".to_string(),
                name: "DocumentationAgent".to_string(),
                agent_type: "DocumentationAgent".to_string(),
                scope: "documentation".to_string(),
                capabilities: vec!["markdown".to_string(), "api-docs".to_string()],
                status: "Available".to_string(),
            },
            SampleAgent {
                id: "agent-004".to_string(),
                name: "SecurityScanAgent".to_string(),
                agent_type: "SecurityAgent".to_string(),
                scope: "security".to_string(),
                capabilities: vec!["vulnerability-scan".to_string(), "code-analysis".to_string()],
                status: "Available".to_string(),
            },
            SampleAgent {
                id: "agent-005".to_string(),
                name: "PerformanceAgent".to_string(),
                agent_type: "PerformanceAgent".to_string(),
                scope: "performance".to_string(),
                capabilities: vec!["benchmarking".to_string(), "profiling".to_string()],
                status: "Available".to_string(),
            },
        ]
    }
    
    /// Create sample sessions for testing
    fn create_sample_sessions() -> Vec<SampleSession> {
        vec![
            SampleSession {
                id: "session-001".to_string(),
                topic: "Code Review Session".to_string(),
                participants: vec!["agent-001".to_string(), "agent-002".to_string()],
                status: "Active".to_string(),
                created_at: "2024-01-15T10:00:00Z".to_string(),
            },
            SampleSession {
                id: "session-002".to_string(),
                topic: "Security Review Session".to_string(),
                participants: vec!["agent-001".to_string(), "agent-004".to_string()],
                status: "Active".to_string(),
                created_at: "2024-01-15T11:00:00Z".to_string(),
            },
            SampleSession {
                id: "session-003".to_string(),
                topic: "Documentation Session".to_string(),
                participants: vec!["agent-003".to_string()],
                status: "Completed".to_string(),
                created_at: "2024-01-15T09:00:00Z".to_string(),
            },
        ]
    }
    
    /// Create sample messages for testing
    fn create_sample_messages() -> Vec<SampleMessage> {
        vec![
            SampleMessage {
                id: "msg-001".to_string(),
                from: "agent-001".to_string(),
                to: "agent-002".to_string(),
                content: "Code review completed for feature branch".to_string(),
                message_type: "Status".to_string(),
                priority: "Normal".to_string(),
                payload: Some(serde_json::json!({
                    "branch": "feature/new-feature",
                    "status": "approved",
                    "comments": 3
                })),
                timestamp: "2024-01-15T10:30:00Z".to_string(),
            },
            SampleMessage {
                id: "msg-002".to_string(),
                from: "agent-002".to_string(),
                to: "agent-001".to_string(),
                content: "Tests passing for the reviewed code".to_string(),
                message_type: "Status".to_string(),
                priority: "Normal".to_string(),
                payload: Some(serde_json::json!({
                    "test_results": "passed",
                    "coverage": 85.5
                })),
                timestamp: "2024-01-15T10:35:00Z".to_string(),
            },
            SampleMessage {
                id: "msg-003".to_string(),
                from: "agent-004".to_string(),
                to: "agent-001".to_string(),
                content: "Security scan completed - no vulnerabilities found".to_string(),
                message_type: "Security".to_string(),
                priority: "High".to_string(),
                payload: Some(serde_json::json!({
                    "scan_type": "static_analysis",
                    "vulnerabilities": 0,
                    "severity": "low"
                })),
                timestamp: "2024-01-15T11:15:00Z".to_string(),
            },
        ]
    }
    
    /// Get agent by ID
    pub fn get_agent(&self, id: &str) -> Option<&SampleAgent> {
        self.agents.iter().find(|agent| agent.id == id)
    }
    
    /// Get session by ID
    pub fn get_session(&self, id: &str) -> Option<&SampleSession> {
        self.sessions.iter().find(|session| session.id == id)
    }
    
    /// Get message by ID
    pub fn get_message(&self, id: &str) -> Option<&SampleMessage> {
        self.messages.iter().find(|message| message.id == id)
    }
    
    /// Get agents by type
    pub fn get_agents_by_type(&self, agent_type: &str) -> Vec<&SampleAgent> {
        self.agents.iter()
            .filter(|agent| agent.agent_type == agent_type)
            .collect()
    }
    
    /// Get agents by scope
    pub fn get_agents_by_scope(&self, scope: &str) -> Vec<&SampleAgent> {
        self.agents.iter()
            .filter(|agent| agent.scope == scope)
            .collect()
    }
    
    /// Get active sessions
    pub fn get_active_sessions(&self) -> Vec<&SampleSession> {
        self.sessions.iter()
            .filter(|session| session.status == "Active")
            .collect()
    }
    
    /// Get messages by type
    pub fn get_messages_by_type(&self, message_type: &str) -> Vec<&SampleMessage> {
        self.messages.iter()
            .filter(|message| message.message_type == message_type)
            .collect()
    }
    
    /// Get messages between agents
    pub fn get_messages_between(&self, from: &str, to: &str) -> Vec<&SampleMessage> {
        self.messages.iter()
            .filter(|message| 
                (message.from == from && message.to == to) ||
                (message.from == to && message.to == from)
            )
            .collect()
    }
}

/// Coordination test environment
#[derive(Debug)]
pub struct CoordinationTestEnv {
    pub fixtures: CoordinationFixtures,
    pub repo_path: PathBuf,
    pub coordination_config: CoordinationConfig,
}

/// Coordination configuration for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    pub max_agents: usize,
    pub max_sessions: usize,
    pub max_messages_per_session: usize,
    pub message_timeout: u64,
    pub session_timeout: u64,
    pub enable_monitoring: bool,
    pub enable_health_checks: bool,
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            max_agents: 100,
            max_sessions: 50,
            max_messages_per_session: 1000,
            message_timeout: 300, // 5 minutes
            session_timeout: 3600, // 1 hour
            enable_monitoring: true,
            enable_health_checks: true,
        }
    }
}

impl CoordinationTestEnv {
    /// Create new coordination test environment
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let fixtures = CoordinationFixtures::new()?;
        let repo_path = fixtures.temp_dir.path().to_path_buf();
        let coordination_config = CoordinationConfig::default();
        
        Ok(Self {
            fixtures,
            repo_path,
            coordination_config,
        })
    }
    
    /// Create test environment with custom configuration
    pub fn with_config(config: CoordinationConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let fixtures = CoordinationFixtures::new()?;
        let repo_path = fixtures.temp_dir.path().to_path_buf();
        
        Ok(Self {
            fixtures,
            repo_path,
            coordination_config: config,
        })
    }
    
    /// Setup coordination system for testing
    pub fn setup_coordination_system(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create .rhema directory
        let rhema_dir = self.repo_path.join(".rhema");
        std::fs::create_dir_all(&rhema_dir)?;
        
        // Create coordination configuration file
        let config_content = serde_yaml::to_string(&self.coordination_config)?;
        std::fs::write(rhema_dir.join("coordination.yaml"), config_content)?;
        
        // Create sample coordination data files
        self.create_sample_coordination_data()?;
        
        Ok(())
    }
    
    /// Create sample coordination data files
    fn create_sample_coordination_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rhema_dir = self.repo_path.join(".rhema");
        
        // Create agents file
        let agents_content = serde_yaml::to_string(&self.fixtures.agents)?;
        std::fs::write(rhema_dir.join("agents.yaml"), agents_content)?;
        
        // Create sessions file
        let sessions_content = serde_yaml::to_string(&self.fixtures.sessions)?;
        std::fs::write(rhema_dir.join("sessions.yaml"), sessions_content)?;
        
        // Create messages file
        let messages_content = serde_yaml::to_string(&self.fixtures.messages)?;
        std::fs::write(rhema_dir.join("messages.yaml"), messages_content)?;
        
        Ok(())
    }
    
    /// Clean up test environment
    pub fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Cleanup is handled automatically by TempDir
        Ok(())
    }
}

/// Coordination test helpers
pub struct CoordinationTestHelpers;

impl CoordinationTestHelpers {
    /// Generate random agent ID
    pub fn random_agent_id() -> String {
        format!("agent-{:06}", rand::thread_rng().gen_range(0..1000000))
    }
    
    /// Generate random session ID
    pub fn random_session_id() -> String {
        format!("session-{:06}", rand::thread_rng().gen_range(0..1000000))
    }
    
    /// Generate random message ID
    pub fn random_message_id() -> String {
        format!("msg-{:06}", rand::thread_rng().gen_range(0..1000000))
    }
    
    /// Generate random agent name
    pub fn random_agent_name() -> String {
        let names = vec![
            "AlphaAgent", "BetaAgent", "GammaAgent", "DeltaAgent", "EpsilonAgent",
            "ZetaAgent", "EtaAgent", "ThetaAgent", "IotaAgent", "KappaAgent",
        ];
        let name = names[rand::thread_rng().gen_range(0..names.len())];
        format!("{}-{}", name, rand::thread_rng().gen_range(0..1000))
    }
    
    /// Generate random agent type
    pub fn random_agent_type() -> String {
        let types = vec![
            "CodeReviewAgent", "TestAgent", "DocumentationAgent", "SecurityAgent",
            "PerformanceAgent", "DeploymentAgent", "MonitoringAgent", "BackupAgent",
        ];
        types[rand::thread_rng().gen_range(0..types.len())].to_string()
    }
    
    /// Generate random scope
    pub fn random_scope() -> String {
        let scopes = vec![
            "backend", "frontend", "testing", "documentation", "security",
            "performance", "deployment", "monitoring", "infrastructure",
        ];
        scopes[rand::thread_rng().gen_range(0..scopes.len())].to_string()
    }
    
    /// Generate random capabilities
    pub fn random_capabilities() -> Vec<String> {
        let all_capabilities = vec![
            "code-review", "testing", "documentation", "security-scan",
            "performance-testing", "deployment", "monitoring", "backup",
            "static-analysis", "dynamic-analysis", "api-testing", "ui-testing",
        ];
        
        let num_capabilities = rand::thread_rng().gen_range(1..5); // 1-4 capabilities
        let mut capabilities = Vec::new();
        
        for _ in 0..num_capabilities {
            let capability = all_capabilities[rand::thread_rng().gen_range(0..all_capabilities.len())].to_string();
            if !capabilities.contains(&capability) {
                capabilities.push(capability);
            }
        }
        
        capabilities
    }
    
    /// Generate random message content
    pub fn random_message_content() -> String {
        let templates = vec![
            "Task completed successfully",
            "Review requested for branch {}",
            "Test results: {} passed, {} failed",
            "Security scan completed with {} findings",
            "Performance benchmark results: {} ms average",
            "Documentation updated for module {}",
            "Deployment status: {}",
            "Monitoring alert: {}",
        ];
        
        let template = templates[rand::thread_rng().gen_range(0..templates.len())];
        let random_value = rand::thread_rng().gen_range(0..100);
        
        template.replace("{}", &random_value.to_string())
    }
    
    /// Generate random message payload
    pub fn random_message_payload() -> serde_json::Value {
        let payload_types = vec![
            serde_json::json!({
                "status": "completed",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "duration": rand::thread_rng().gen_range(0..1000)
            }),
            serde_json::json!({
                "result": "success",
                "metrics": {
                    "accuracy": rand::thread_rng().gen_range(0.0..100.0),
                    "performance": rand::thread_rng().gen_range(0.0..100.0)
                }
            }),
            serde_json::json!({
                "summary": "processed",
                "stats": {
                    "processed": rand::thread_rng().gen_range(0..1000),
                    "errors": rand::thread_rng().gen_range(0..10)
                }
            }),
        ];
        
        payload_types[rand::thread_rng().gen_range(0..payload_types.len())].clone()
    }
    
    /// Create test agent data
    pub fn create_test_agent(name: Option<String>, agent_type: Option<String>, scope: Option<String>) -> SampleAgent {
        SampleAgent {
            id: Self::random_agent_id(),
            name: name.unwrap_or_else(Self::random_agent_name),
            agent_type: agent_type.unwrap_or_else(Self::random_agent_type),
            scope: scope.unwrap_or_else(Self::random_scope),
            capabilities: Self::random_capabilities(),
            status: "Available".to_string(),
        }
    }
    
    /// Create test session data
    pub fn create_test_session(topic: Option<String>, participants: Option<Vec<String>>) -> SampleSession {
        SampleSession {
            id: Self::random_session_id(),
            topic: topic.unwrap_or_else(|| format!("Test Session {}", rand::thread_rng().gen_range(0..1000000))),
            participants: participants.unwrap_or_else(|| vec![Self::random_agent_id()]),
            status: "Active".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// Create test message data
    pub fn create_test_message(from: Option<String>, to: Option<String>, content: Option<String>) -> SampleMessage {
        SampleMessage {
            id: Self::random_message_id(),
            from: from.unwrap_or_else(Self::random_agent_id),
            to: to.unwrap_or_else(Self::random_agent_id),
            content: content.unwrap_or_else(Self::random_message_content),
            message_type: "Test".to_string(),
            priority: "Normal".to_string(),
            payload: Some(Self::random_message_payload()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// Validate agent data
    pub fn validate_agent(agent: &SampleAgent) -> Result<(), String> {
        if agent.id.is_empty() {
            return Err("Agent ID cannot be empty".to_string());
        }
        if agent.name.is_empty() {
            return Err("Agent name cannot be empty".to_string());
        }
        if agent.agent_type.is_empty() {
            return Err("Agent type cannot be empty".to_string());
        }
        if agent.scope.is_empty() {
            return Err("Agent scope cannot be empty".to_string());
        }
        Ok(())
    }
    
    /// Validate session data
    pub fn validate_session(session: &SampleSession) -> Result<(), String> {
        if session.id.is_empty() {
            return Err("Session ID cannot be empty".to_string());
        }
        if session.topic.is_empty() {
            return Err("Session topic cannot be empty".to_string());
        }
        if session.participants.is_empty() {
            return Err("Session must have at least one participant".to_string());
        }
        Ok(())
    }
    
    /// Validate message data
    pub fn validate_message(message: &SampleMessage) -> Result<(), String> {
        if message.id.is_empty() {
            return Err("Message ID cannot be empty".to_string());
        }
        if message.from.is_empty() {
            return Err("Message sender cannot be empty".to_string());
        }
        if message.to.is_empty() {
            return Err("Message recipient cannot be empty".to_string());
        }
        if message.content.is_empty() {
            return Err("Message content cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Coordination test assertions
pub struct CoordinationAssertions;

impl CoordinationAssertions {
    /// Assert that agent registration was successful
    pub fn assert_agent_registration_successful(output: &str, agent_name: &str) {
        assert!(output.contains(agent_name), "Output should contain agent name: {}", agent_name);
        assert!(
            output.contains("registered") || output.contains("success") || output.contains("created"),
            "Output should indicate successful registration"
        );
    }
    
    /// Assert that agent listing contains expected agents
    pub fn assert_agent_listing_contains(output: &str, expected_agents: &[&str]) {
        for agent in expected_agents {
            assert!(output.contains(agent), "Output should contain agent: {}", agent);
        }
    }
    
    /// Assert that session creation was successful
    pub fn assert_session_creation_successful(output: &str, session_topic: &str) {
        assert!(output.contains(session_topic), "Output should contain session topic: {}", session_topic);
        assert!(
            output.contains("session") || output.contains("created") || output.contains("success"),
            "Output should indicate successful session creation"
        );
    }
    
    /// Assert that message sending was successful
    pub fn assert_message_sending_successful(output: &str) {
        assert!(
            output.contains("sent") || output.contains("delivered") || output.contains("success"),
            "Output should indicate successful message sending"
        );
    }
    
    /// Assert that system stats are displayed
    pub fn assert_system_stats_displayed(output: &str) {
        assert!(
            output.contains("statistics") || output.contains("stats") || output.contains("agents") || output.contains("sessions"),
            "Output should contain system statistics"
        );
    }
    
    /// Assert that health check passed
    pub fn assert_health_check_passed(output: &str) {
        assert!(
            output.contains("health") || output.contains("status") || output.contains("ok") || output.contains("healthy"),
            "Output should indicate healthy system status"
        );
    }
    
    /// Assert that error handling is proper
    pub fn assert_error_handling_proper(error: &str) {
        assert!(!error.contains("panicked"), "Error should not be a panic");
        assert!(!error.contains("overflow"), "Error should not be an overflow");
        assert!(!error.contains("segmentation fault"), "Error should not be a segmentation fault");
    }
    
    /// Assert that security measures are in place
    pub fn assert_security_measures(output: &str) {
        // Should not expose sensitive information
        assert!(!output.contains("password"), "Output should not contain passwords");
        assert!(!output.contains("secret"), "Output should not contain secrets");
        assert!(!output.contains("token"), "Output should not contain tokens");
        assert!(!output.contains("key"), "Output should not contain keys");
        assert!(!output.contains("credential"), "Output should not contain credentials");
    }
    
    /// Assert performance requirements are met
    pub fn assert_performance_requirements(duration: std::time::Duration, max_duration_ms: u64) {
        assert!(
            duration.as_millis() < max_duration_ms as u128,
            "Operation took {}ms, should be under {}ms",
            duration.as_millis(),
            max_duration_ms
        );
    }
}

/// Coordination test utilities
pub struct CoordinationTestUtils;

impl CoordinationTestUtils {
    /// Parse agent ID from output
    pub fn parse_agent_id(output: &str) -> Option<String> {
        // Look for patterns like "agent-001" or "Agent ID: agent-001"
        let patterns = vec![
            r"agent-\d+",
            r"Agent ID: (agent-\d+)",
            r"Registered agent: (agent-\d+)",
        ];
        
        for pattern in patterns {
            if let Some(captures) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(output)) {
                if let Some(agent_id) = captures.get(1) {
                    return Some(agent_id.as_str().to_string());
                } else if let Some(agent_id) = captures.get(0) {
                    return Some(agent_id.as_str().to_string());
                }
            }
        }
        
        None
    }
    
    /// Parse session ID from output
    pub fn parse_session_id(output: &str) -> Option<String> {
        // Look for patterns like "session-001" or "Session ID: session-001"
        let patterns = vec![
            r"session-\d+",
            r"Session ID: (session-\d+)",
            r"Created session: (session-\d+)",
        ];
        
        for pattern in patterns {
            if let Some(captures) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(output)) {
                if let Some(session_id) = captures.get(1) {
                    return Some(session_id.as_str().to_string());
                } else if let Some(session_id) = captures.get(0) {
                    return Some(session_id.as_str().to_string());
                }
            }
        }
        
        None
    }
    
    /// Parse message count from output
    pub fn parse_message_count(output: &str) -> Option<usize> {
        // Look for patterns like "5 messages" or "Messages: 5"
        let patterns = vec![
            r"(\d+) messages?",
            r"Messages?: (\d+)",
            r"Total messages?: (\d+)",
        ];
        
        for pattern in patterns {
            if let Some(captures) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(output)) {
                if let Some(count) = captures.get(1) {
                    return count.as_str().parse::<usize>().ok();
                }
            }
        }
        
        None
    }
    
    /// Parse agent count from output
    pub fn parse_agent_count(output: &str) -> Option<usize> {
        // Look for patterns like "5 agents" or "Agents: 5"
        let patterns = vec![
            r"(\d+) agents?",
            r"Agents?: (\d+)",
            r"Total agents?: (\d+)",
        ];
        
        for pattern in patterns {
            if let Some(captures) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(output)) {
                if let Some(count) = captures.get(1) {
                    return count.as_str().parse::<usize>().ok();
                }
            }
        }
        
        None
    }
    
    /// Parse session count from output
    pub fn parse_session_count(output: &str) -> Option<usize> {
        // Look for patterns like "3 sessions" or "Sessions: 3"
        let patterns = vec![
            r"(\d+) sessions?",
            r"Sessions?: (\d+)",
            r"Total sessions?: (\d+)",
        ];
        
        for pattern in patterns {
            if let Some(captures) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(output)) {
                if let Some(count) = captures.get(1) {
                    return count.as_str().parse::<usize>().ok();
                }
            }
        }
        
        None
    }
    
    /// Extract JSON from output
    pub fn extract_json(output: &str) -> Option<serde_json::Value> {
        // Look for JSON content in the output
        if let Some(start) = output.find('{') {
            if let Some(end) = output.rfind('}') {
                if end > start {
                    let json_str = &output[start..=end];
                    return serde_json::from_str(json_str).ok();
                }
            }
        }
        
        None
    }
    
    /// Extract YAML from output
    pub fn extract_yaml(output: &str) -> Option<serde_yaml::Value> {
        // Look for YAML content in the output
        if let Some(start) = output.find("---") {
            if let Some(end) = output.rfind("---") {
                if end > start {
                    let yaml_str = &output[start..=end];
                    return serde_yaml::from_str(yaml_str).ok();
                }
            }
        }
        
        None
    }
    
    /// Wait for condition with timeout
    pub async fn wait_for_condition<F, Fut>(condition: F, timeout: std::time::Duration) -> Result<bool, tokio::time::error::Elapsed>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        tokio::time::timeout(timeout, async {
            loop {
                if condition().await {
                    return true;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }).await
    }
    
    /// Retry operation with exponential backoff
    pub async fn retry_with_backoff<F, Fut, T, E>(
        operation: F,
        max_retries: usize,
        initial_delay: std::time::Duration,
    ) -> Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        let mut delay = initial_delay;
        
        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_retries {
                        return Err(e);
                    }
                    eprintln!("Attempt {} failed: {:?}, retrying in {:?}", attempt + 1, e, delay);
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                }
            }
        }
        
        unreachable!()
    }
} 