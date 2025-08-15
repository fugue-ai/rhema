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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Security testing system for production coordination
pub struct SecurityTestingSystem {
    /// Security testing configuration
    config: SecurityTestingConfig,
    /// Active security tests
    active_tests: Arc<RwLock<HashMap<String, SecurityTest>>>,
    /// Test history
    test_history: Arc<RwLock<Vec<SecurityTest>>>,
    /// Security scanners
    security_scanners: Vec<Box<dyn SecurityScanner + Send + Sync>>,
    /// Security statistics
    statistics: Arc<RwLock<SecurityStatistics>>,
}

/// Security testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTestingConfig {
    /// Enable security testing
    pub enabled: bool,
    /// Security testing mode
    pub mode: SecurityTestingMode,
    /// Test frequency (minutes)
    pub test_frequency_minutes: u64,
    /// Maximum concurrent tests
    pub max_concurrent_tests: usize,
    /// Enable automated remediation
    pub auto_remediation_enabled: bool,
    /// Enable compliance checking
    pub compliance_checking_enabled: bool,
    /// Security standards
    pub security_standards: Vec<SecurityStandard>,
    /// Vulnerability thresholds
    pub vulnerability_thresholds: VulnerabilityThresholds,
    /// Test scheduling
    pub scheduling: SecurityTestSchedule,
}

/// Security testing modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityTestingMode {
    /// Manual mode - tests must be triggered manually
    Manual,
    /// Scheduled mode - tests run on schedule
    Scheduled,
    /// Continuous mode - tests run continuously
    Continuous,
    /// Event-driven mode - tests run on security events
    EventDriven,
}

/// Security standards
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityStandard {
    OWASP,
    NIST,
    ISO27001,
    SOC2,
    PCI,
    HIPAA,
    GDPR,
    Custom(String),
}

/// Vulnerability thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityThresholds {
    /// Critical vulnerabilities allowed
    pub critical_allowed: u32,
    /// High vulnerabilities allowed
    pub high_allowed: u32,
    /// Medium vulnerabilities allowed
    pub medium_allowed: u32,
    /// Low vulnerabilities allowed
    pub low_allowed: u32,
    /// Info vulnerabilities allowed
    pub info_allowed: u32,
}

/// Security test schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTestSchedule {
    /// Schedule enabled
    pub enabled: bool,
    /// Schedule interval (minutes)
    pub interval_minutes: u64,
    /// Schedule start time
    pub start_time: Option<String>,
    /// Schedule end time
    pub end_time: Option<String>,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u8>,
    /// Test types to run
    pub test_types: Vec<SecurityTestType>,
}

/// Security test types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityTestType {
    /// Vulnerability scanning
    VulnerabilityScan { scan_depth: ScanDepth },
    /// Penetration testing
    PenetrationTest { scope: TestScope },
    /// Code security analysis
    CodeSecurityAnalysis { languages: Vec<String> },
    /// Dependency scanning
    DependencyScan { include_dev_deps: bool },
    /// Configuration audit
    ConfigurationAudit { config_files: Vec<String> },
    /// Access control testing
    AccessControlTest { test_users: Vec<String> },
    /// Encryption testing
    EncryptionTest { algorithms: Vec<String> },
    /// Network security testing
    NetworkSecurityTest { ports: Vec<u16> },
    /// API security testing
    ApiSecurityTest { endpoints: Vec<String> },
    /// Custom security test
    Custom {
        name: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Scan depth levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanDepth {
    Quick,
    Standard,
    Deep,
    Comprehensive,
}

/// Test scope
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestScope {
    External,
    Internal,
    WebApplication,
    MobileApplication,
    API,
    Infrastructure,
    Full,
}

/// Security test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTest {
    /// Unique test ID
    pub id: String,
    /// Test name
    pub name: String,
    /// Test description
    pub description: String,
    /// Test type
    pub test_type: SecurityTestType,
    /// Test status
    pub status: SecurityTestStatus,
    /// Test start time
    pub start_time: DateTime<Utc>,
    /// Test end time
    pub end_time: Option<DateTime<Utc>>,
    /// Test duration (seconds)
    pub duration_seconds: Option<u64>,
    /// Target components
    pub target_components: Vec<String>,
    /// Test parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Test results
    pub results: Option<SecurityTestResults>,
    /// Remediation status
    pub remediation_status: RemediationStatus,
    /// Test metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Security test status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityTestStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Remediated,
}

/// Remediation status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemediationStatus {
    NotRequired,
    Pending,
    InProgress,
    Completed,
    Failed,
    ManualRequired,
}

/// Security test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTestResults {
    /// Vulnerabilities found
    pub vulnerabilities: Vec<Vulnerability>,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
    /// Recommendations
    pub recommendations: Vec<SecurityRecommendation>,
    /// Remediation actions
    pub remediation_actions: Vec<RemediationAction>,
    /// Test coverage
    pub test_coverage: TestCoverage,
}

/// Vulnerability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// Unique vulnerability ID
    pub id: String,
    /// Vulnerability name
    pub name: String,
    /// Vulnerability description
    pub description: String,
    /// Vulnerability severity
    pub severity: VulnerabilitySeverity,
    /// Vulnerability type
    pub vulnerability_type: VulnerabilityType,
    /// Affected component
    pub affected_component: String,
    /// CVSS score
    pub cvss_score: Option<f64>,
    /// CVE ID
    pub cve_id: Option<String>,
    /// Remediation steps
    pub remediation_steps: Vec<String>,
    /// Exploit available
    pub exploit_available: bool,
    /// False positive
    pub false_positive: bool,
    /// Verified
    pub verified: bool,
}

/// Vulnerability severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VulnerabilitySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Vulnerability types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilityType {
    SQLInjection,
    XSS,
    CSRF,
    Authentication,
    Authorization,
    Encryption,
    Configuration,
    Dependency,
    Network,
    API,
    Custom(String),
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// Overall compliance score
    pub compliance_score: f64,
    /// Standards compliance
    pub standards_compliance: HashMap<SecurityStandard, StandardCompliance>,
    /// Compliance violations
    pub violations: Vec<ComplianceViolation>,
    /// Compliance recommendations
    pub recommendations: Vec<String>,
}

/// Standard compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardCompliance {
    /// Compliance score
    pub score: f64,
    /// Compliant controls
    pub compliant_controls: u32,
    /// Non-compliant controls
    pub non_compliant_controls: u32,
    /// Total controls
    pub total_controls: u32,
    /// Compliance status
    pub status: ComplianceLevel,
}

/// Compliance levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceLevel {
    NonCompliant,
    PartiallyCompliant,
    Compliant,
    FullyCompliant,
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Violation ID
    pub id: String,
    /// Violation description
    pub description: String,
    /// Affected standard
    pub standard: SecurityStandard,
    /// Violation severity
    pub severity: VulnerabilitySeverity,
    /// Remediation required
    pub remediation_required: bool,
}

/// Risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk score
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Risk factors
    pub risk_factors: Vec<RiskFactor>,
    /// Risk mitigation strategies
    pub mitigation_strategies: Vec<String>,
}

/// Risk levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor name
    pub name: String,
    /// Factor description
    pub description: String,
    /// Factor impact
    pub impact: f64,
    /// Factor probability
    pub probability: f64,
    /// Factor risk score
    pub risk_score: f64,
}

/// Security recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    /// Recommendation ID
    pub id: String,
    /// Recommendation title
    pub title: String,
    /// Recommendation description
    pub description: String,
    /// Recommendation priority
    pub priority: RecommendationPriority,
    /// Implementation effort
    pub implementation_effort: ImplementationEffort,
    /// Business impact
    pub business_impact: BusinessImpact,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
}

/// Recommendation priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Implementation effort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Business impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessImpact {
    Low,
    Medium,
    High,
    Critical,
}

/// Remediation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    /// Action ID
    pub id: String,
    /// Action title
    pub title: String,
    /// Action description
    pub description: String,
    /// Action type
    pub action_type: RemediationActionType,
    /// Action status
    pub status: RemediationActionStatus,
    /// Action steps
    pub steps: Vec<String>,
    /// Estimated time (minutes)
    pub estimated_time_minutes: u64,
    /// Automated
    pub automated: bool,
}

/// Remediation action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationActionType {
    Configuration,
    Code,
    Dependency,
    Infrastructure,
    Process,
    Training,
    Custom(String),
}

/// Remediation action status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationActionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Test coverage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCoverage {
    /// Overall coverage percentage
    pub coverage_percentage: f64,
    /// Covered components
    pub covered_components: Vec<String>,
    /// Uncovered components
    pub uncovered_components: Vec<String>,
    /// Coverage by component
    pub coverage_by_component: HashMap<String, f64>,
}

/// Security statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatistics {
    /// Total tests run
    pub total_tests: u64,
    /// Successful tests
    pub successful_tests: u64,
    /// Failed tests
    pub failed_tests: u64,
    /// Tests by type
    pub tests_by_type: HashMap<String, u64>,
    /// Total vulnerabilities found
    pub total_vulnerabilities: u64,
    /// Vulnerabilities by severity
    pub vulnerabilities_by_severity: HashMap<VulnerabilitySeverity, u64>,
    /// Average risk score
    pub avg_risk_score: f64,
    /// Last test timestamp
    pub last_test_timestamp: Option<DateTime<Utc>>,
}

/// Security scanner trait
pub trait SecurityScanner: Send + Sync {
    /// Run security scan
    fn run_scan(&self, test: &SecurityTest) -> Result<SecurityTestResults, String>;
    /// Get scanner name
    fn name(&self) -> &str;
    /// Check if scanner is enabled
    fn is_enabled(&self) -> bool;
    /// Get supported test types
    fn supported_test_types(&self) -> Vec<SecurityTestType>;
}

/// Vulnerability scanner
pub struct VulnerabilityScanner;

impl SecurityScanner for VulnerabilityScanner {
    fn run_scan(&self, test: &SecurityTest) -> Result<SecurityTestResults, String> {
        info!(
            "[VULNERABILITY_SCANNER] Running scan for test: {}",
            test.name
        );

        // Simulate vulnerability scan results
        let vulnerabilities = vec![Vulnerability {
            id: "vuln_001".to_string(),
            name: "SQL Injection Vulnerability".to_string(),
            description: "Potential SQL injection in user input validation".to_string(),
            severity: VulnerabilitySeverity::High,
            vulnerability_type: VulnerabilityType::SQLInjection,
            affected_component: "user_authentication".to_string(),
            cvss_score: Some(8.5),
            cve_id: Some("CVE-2024-0001".to_string()),
            remediation_steps: vec![
                "Use parameterized queries".to_string(),
                "Implement input validation".to_string(),
            ],
            exploit_available: true,
            false_positive: false,
            verified: true,
        }];

        let results = SecurityTestResults {
            vulnerabilities,
            compliance_status: ComplianceStatus {
                compliance_score: 85.0,
                standards_compliance: HashMap::new(),
                violations: Vec::new(),
                recommendations: vec!["Implement additional security controls".to_string()],
            },
            risk_assessment: RiskAssessment {
                risk_score: 7.5,
                risk_level: RiskLevel::Medium,
                risk_factors: Vec::new(),
                mitigation_strategies: vec!["Implement security controls".to_string()],
            },
            recommendations: Vec::new(),
            remediation_actions: Vec::new(),
            test_coverage: TestCoverage {
                coverage_percentage: 90.0,
                covered_components: vec!["api".to_string(), "database".to_string()],
                uncovered_components: vec!["legacy_system".to_string()],
                coverage_by_component: HashMap::new(),
            },
        };

        Ok(results)
    }

    fn name(&self) -> &str {
        "vulnerability_scanner"
    }

    fn is_enabled(&self) -> bool {
        true
    }

    fn supported_test_types(&self) -> Vec<SecurityTestType> {
        vec![
            SecurityTestType::VulnerabilityScan {
                scan_depth: ScanDepth::Standard,
            },
            SecurityTestType::PenetrationTest {
                scope: TestScope::WebApplication,
            },
        ]
    }
}

/// Code security analyzer
pub struct CodeSecurityAnalyzer;

impl SecurityScanner for CodeSecurityAnalyzer {
    fn run_scan(&self, test: &SecurityTest) -> Result<SecurityTestResults, String> {
        info!(
            "[CODE_SECURITY_ANALYZER] Running analysis for test: {}",
            test.name
        );

        // Simulate code security analysis results
        let vulnerabilities = vec![Vulnerability {
            id: "code_001".to_string(),
            name: "Hardcoded Password".to_string(),
            description: "Password found hardcoded in source code".to_string(),
            severity: VulnerabilitySeverity::Critical,
            vulnerability_type: VulnerabilityType::Configuration,
            affected_component: "config.rs".to_string(),
            cvss_score: Some(9.0),
            cve_id: None,
            remediation_steps: vec![
                "Move password to environment variables".to_string(),
                "Use secure configuration management".to_string(),
            ],
            exploit_available: false,
            false_positive: false,
            verified: true,
        }];

        let results = SecurityTestResults {
            vulnerabilities,
            compliance_status: ComplianceStatus {
                compliance_score: 95.0,
                standards_compliance: HashMap::new(),
                violations: Vec::new(),
                recommendations: vec!["Remove hardcoded credentials".to_string()],
            },
            risk_assessment: RiskAssessment {
                risk_score: 8.0,
                risk_level: RiskLevel::High,
                risk_factors: Vec::new(),
                mitigation_strategies: vec!["Implement secure coding practices".to_string()],
            },
            recommendations: Vec::new(),
            remediation_actions: Vec::new(),
            test_coverage: TestCoverage {
                coverage_percentage: 95.0,
                covered_components: vec!["src".to_string()],
                uncovered_components: Vec::new(),
                coverage_by_component: HashMap::new(),
            },
        };

        Ok(results)
    }

    fn name(&self) -> &str {
        "code_security_analyzer"
    }

    fn is_enabled(&self) -> bool {
        true
    }

    fn supported_test_types(&self) -> Vec<SecurityTestType> {
        vec![SecurityTestType::CodeSecurityAnalysis {
            languages: vec!["rust".to_string()],
        }]
    }
}

impl SecurityTestingSystem {
    /// Create new security testing system
    pub fn new(config: SecurityTestingConfig) -> Self {
        let mut security_scanners: Vec<Box<dyn SecurityScanner + Send + Sync>> = Vec::new();

        // Add default security scanners
        security_scanners.push(Box::new(VulnerabilityScanner));
        security_scanners.push(Box::new(CodeSecurityAnalyzer));

        Self {
            config,
            active_tests: Arc::new(RwLock::new(HashMap::new())),
            test_history: Arc::new(RwLock::new(Vec::new())),
            security_scanners,
            statistics: Arc::new(RwLock::new(SecurityStatistics::default())),
        }
    }

    /// Add security scanner
    pub fn add_security_scanner(&mut self, scanner: Box<dyn SecurityScanner + Send + Sync>) {
        self.security_scanners.push(scanner);
    }

    /// Start security test
    pub async fn start_test(
        &self,
        name: String,
        description: String,
        test_type: SecurityTestType,
        target_components: Vec<String>,
        parameters: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, String> {
        if !self.config.enabled {
            return Err("Security testing is disabled".to_string());
        }

        // Check concurrent test limit
        let active_tests = self.active_tests.read().await;
        if active_tests.len() >= self.config.max_concurrent_tests {
            return Err("Maximum concurrent tests reached".to_string());
        }

        let test_id = format!("security_{}", chrono::Utc::now().timestamp_millis());
        let test = SecurityTest {
            id: test_id.clone(),
            name,
            description,
            test_type,
            status: SecurityTestStatus::Pending,
            start_time: Utc::now(),
            end_time: None,
            duration_seconds: None,
            target_components,
            parameters: parameters.unwrap_or_default(),
            results: None,
            remediation_status: RemediationStatus::NotRequired,
            metadata: HashMap::new(),
        };

        // Store test
        {
            let mut active_tests = self.active_tests.write().await;
            active_tests.insert(test_id.clone(), test.clone());
        }

        // Execute test
        self.execute_test(&test).await?;

        info!("Security test started: {} ({})", test.name, test_id);
        Ok(test_id)
    }

    /// Stop security test
    pub async fn stop_test(&self, test_id: &str) -> Result<(), String> {
        let mut active_tests = self.active_tests.write().await;

        if let Some(test) = active_tests.get_mut(test_id) {
            test.status = SecurityTestStatus::Cancelled;
            test.end_time = Some(Utc::now());

            let start_time = test.start_time;
            test.duration_seconds =
                Some((test.end_time.unwrap() - start_time).num_seconds() as u64);

            // Move to history
            let test = active_tests.remove(test_id).unwrap();
            let mut test_history = self.test_history.write().await;
            test_history.push(test);

            info!("Security test stopped: {}", test_id);
            Ok(())
        } else {
            Err(format!("Test not found: {}", test_id))
        }
    }

    /// Get active tests
    pub async fn get_active_tests(&self) -> Vec<SecurityTest> {
        let active_tests = self.active_tests.read().await;
        active_tests.values().cloned().collect()
    }

    /// Get test by ID
    pub async fn get_test(&self, test_id: &str) -> Option<SecurityTest> {
        let active_tests = self.active_tests.read().await;
        if let Some(test) = active_tests.get(test_id) {
            return Some(test.clone());
        }

        let test_history = self.test_history.read().await;
        test_history.iter().find(|t| t.id == test_id).cloned()
    }

    /// Get security statistics
    pub async fn get_statistics(&self) -> SecurityStatistics {
        self.statistics.read().await.clone()
    }

    /// Run scheduled tests
    pub async fn run_scheduled_tests(&self) -> Result<Vec<String>, String> {
        if !self.config.scheduling.enabled {
            return Ok(Vec::new());
        }

        let mut test_ids = Vec::new();

        for test_type in &self.config.scheduling.test_types {
            let test_id = self
                .start_test(
                    format!("Scheduled {}", test_type.name()),
                    format!(
                        "Automatically scheduled security test: {}",
                        test_type.name()
                    ),
                    test_type.clone(),
                    vec!["system".to_string()],
                    None,
                )
                .await?;

            test_ids.push(test_id);
        }

        Ok(test_ids)
    }

    /// Execute test
    async fn execute_test(&self, test: &SecurityTest) -> Result<(), String> {
        let mut active_tests = self.active_tests.write().await;

        if let Some(test) = active_tests.get_mut(&test.id) {
            test.status = SecurityTestStatus::Running;

            // Find appropriate security scanner
            for scanner in &self.security_scanners {
                if scanner.is_enabled()
                    && scanner
                        .supported_test_types()
                        .iter()
                        .any(|t| t == &test.test_type)
                {
                    match scanner.run_scan(test) {
                        Ok(results) => {
                            test.results = Some(results);
                            test.status = SecurityTestStatus::Completed;

                            // Check for vulnerabilities that exceed thresholds
                            self.check_vulnerability_thresholds(test).await;

                            // Update statistics
                            self.update_statistics(test).await;

                            info!("Security test completed by {}: {}", scanner.name(), test.id);
                            return Ok(());
                        }
                        Err(e) => {
                            warn!("Security scanner {} failed: {}", scanner.name(), e);
                        }
                    }
                }
            }

            test.status = SecurityTestStatus::Failed;
            Err("No suitable security scanner found for test".to_string())
        } else {
            Err("Test not found".to_string())
        }
    }

    /// Check vulnerability thresholds
    async fn check_vulnerability_thresholds(&self, test: &SecurityTest) {
        if let Some(results) = &test.results {
            let mut severity_counts: HashMap<VulnerabilitySeverity, u32> = HashMap::new();

            for vulnerability in &results.vulnerabilities {
                *severity_counts
                    .entry(vulnerability.severity.clone())
                    .or_insert(0) += 1;
            }

            let thresholds = &self.config.vulnerability_thresholds;

            if let Some(&count) = severity_counts.get(&VulnerabilitySeverity::Critical) {
                if count > thresholds.critical_allowed {
                    error!(
                        "Critical vulnerability threshold exceeded: {} > {}",
                        count, thresholds.critical_allowed
                    );
                }
            }

            if let Some(&count) = severity_counts.get(&VulnerabilitySeverity::High) {
                if count > thresholds.high_allowed {
                    warn!(
                        "High vulnerability threshold exceeded: {} > {}",
                        count, thresholds.high_allowed
                    );
                }
            }
        }
    }

    /// Update security statistics
    async fn update_statistics(&self, test: &SecurityTest) {
        let mut stats = self.statistics.write().await;

        stats.total_tests += 1;
        stats.last_test_timestamp = Some(test.start_time);

        match test.status {
            SecurityTestStatus::Completed => stats.successful_tests += 1,
            SecurityTestStatus::Failed => stats.failed_tests += 1,
            _ => {}
        }

        // Update test type counts
        let test_type_name = test.test_type.name();
        *stats
            .tests_by_type
            .entry(test_type_name.to_string())
            .or_insert(0) += 1;

        // Update vulnerability counts
        if let Some(results) = &test.results {
            stats.total_vulnerabilities += results.vulnerabilities.len() as u64;

            for vulnerability in &results.vulnerabilities {
                *stats
                    .vulnerabilities_by_severity
                    .entry(vulnerability.severity.clone())
                    .or_insert(0) += 1;
            }

            // Update average risk score
            if let Some(risk_score) = Some(results.risk_assessment.risk_score) {
                let total_tests = stats.total_tests as f64;
                stats.avg_risk_score =
                    (stats.avg_risk_score * (total_tests - 1.0) + risk_score) / total_tests;
            }
        }
    }
}

impl SecurityTestType {
    /// Get test type name
    pub fn name(&self) -> &str {
        match self {
            SecurityTestType::VulnerabilityScan { .. } => "Vulnerability Scan",
            SecurityTestType::PenetrationTest { .. } => "Penetration Test",
            SecurityTestType::CodeSecurityAnalysis { .. } => "Code Security Analysis",
            SecurityTestType::DependencyScan { .. } => "Dependency Scan",
            SecurityTestType::ConfigurationAudit { .. } => "Configuration Audit",
            SecurityTestType::AccessControlTest { .. } => "Access Control Test",
            SecurityTestType::EncryptionTest { .. } => "Encryption Test",
            SecurityTestType::NetworkSecurityTest { .. } => "Network Security Test",
            SecurityTestType::ApiSecurityTest { .. } => "API Security Test",
            SecurityTestType::Custom { name, .. } => name,
        }
    }
}

impl Default for SecurityTestingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: SecurityTestingMode::Scheduled,
            test_frequency_minutes: 60,
            max_concurrent_tests: 5,
            auto_remediation_enabled: false,
            compliance_checking_enabled: true,
            security_standards: vec![SecurityStandard::OWASP, SecurityStandard::NIST],
            vulnerability_thresholds: VulnerabilityThresholds::default(),
            scheduling: SecurityTestSchedule::default(),
        }
    }
}

impl Default for VulnerabilityThresholds {
    fn default() -> Self {
        Self {
            critical_allowed: 0,
            high_allowed: 2,
            medium_allowed: 10,
            low_allowed: 50,
            info_allowed: 100,
        }
    }
}

impl Default for SecurityTestSchedule {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_minutes: 60,
            start_time: None,
            end_time: None,
            days_of_week: vec![1, 2, 3, 4, 5], // Monday to Friday
            test_types: vec![
                SecurityTestType::VulnerabilityScan {
                    scan_depth: ScanDepth::Standard,
                },
                SecurityTestType::DependencyScan {
                    include_dev_deps: true,
                },
            ],
        }
    }
}

impl Default for SecurityStatistics {
    fn default() -> Self {
        Self {
            total_tests: 0,
            successful_tests: 0,
            failed_tests: 0,
            tests_by_type: HashMap::new(),
            total_vulnerabilities: 0,
            vulnerabilities_by_severity: HashMap::new(),
            avg_risk_score: 0.0,
            last_test_timestamp: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_testing_system_creation() {
        let config = SecurityTestingConfig::default();
        let security_system = SecurityTestingSystem::new(config);

        assert!(security_system.config.enabled);
        assert_eq!(security_system.config.mode, SecurityTestingMode::Scheduled);
    }

    #[tokio::test]
    async fn test_start_security_test() {
        let config = SecurityTestingConfig::default();
        let security_system = SecurityTestingSystem::new(config);

        let test_id = security_system
            .start_test(
                "Test Security Scan".to_string(),
                "This is a test security scan".to_string(),
                SecurityTestType::VulnerabilityScan {
                    scan_depth: ScanDepth::Standard,
                },
                vec!["api".to_string()],
                None,
            )
            .await
            .unwrap();

        assert!(!test_id.is_empty());

        let active_tests = security_system.get_active_tests().await;
        assert_eq!(active_tests.len(), 1);
        assert_eq!(active_tests[0].name, "Test Security Scan");
    }

    #[tokio::test]
    async fn test_stop_security_test() {
        let config = SecurityTestingConfig::default();
        let security_system = SecurityTestingSystem::new(config);

        let test_id = security_system
            .start_test(
                "Test Security Scan".to_string(),
                "This is a test security scan".to_string(),
                SecurityTestType::VulnerabilityScan {
                    scan_depth: ScanDepth::Standard,
                },
                vec!["api".to_string()],
                None,
            )
            .await
            .unwrap();

        // Wait a bit for the test to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        security_system.stop_test(&test_id).await.unwrap();

        let active_tests = security_system.get_active_tests().await;
        assert_eq!(active_tests.len(), 0);

        let test = security_system.get_test(&test_id).await.unwrap();
        assert_eq!(test.status, SecurityTestStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_security_test_type_name() {
        let vuln_scan = SecurityTestType::VulnerabilityScan {
            scan_depth: ScanDepth::Standard,
        };
        assert_eq!(vuln_scan.name(), "Vulnerability Scan");

        let pen_test = SecurityTestType::PenetrationTest {
            scope: TestScope::WebApplication,
        };
        assert_eq!(pen_test.name(), "Penetration Test");

        let custom_test = SecurityTestType::Custom {
            name: "Custom Security Test".to_string(),
            parameters: HashMap::new(),
        };
        assert_eq!(custom_test.name(), "Custom Security Test");
    }

    #[tokio::test]
    async fn test_vulnerability_scanner() {
        let scanner = VulnerabilityScanner;
        let test = SecurityTest {
            id: "test_001".to_string(),
            name: "Test Scan".to_string(),
            description: "Test vulnerability scan".to_string(),
            test_type: SecurityTestType::VulnerabilityScan {
                scan_depth: ScanDepth::Standard,
            },
            status: SecurityTestStatus::Pending,
            start_time: Utc::now(),
            end_time: None,
            duration_seconds: None,
            target_components: vec!["api".to_string()],
            parameters: HashMap::new(),
            results: None,
            remediation_status: RemediationStatus::NotRequired,
            metadata: HashMap::new(),
        };

        let results = scanner.run_scan(&test).unwrap();
        assert_eq!(results.vulnerabilities.len(), 1);
        assert_eq!(
            results.vulnerabilities[0].severity,
            VulnerabilitySeverity::High
        );
    }
}
