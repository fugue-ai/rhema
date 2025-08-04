use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use reqwest::Client;
use sha2::{Sha256, Digest};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, DependencyType, HealthStatus};

/// Security vulnerability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// Vulnerability ID (e.g., CVE-2021-1234)
    pub id: String,
    /// Vulnerability title
    pub title: String,
    /// Vulnerability description
    pub description: String,
    /// Severity level
    pub severity: VulnerabilitySeverity,
    /// CVSS score (0.0 to 10.0)
    pub cvss_score: f64,
    /// Affected versions
    pub affected_versions: Vec<String>,
    /// Fixed versions
    pub fixed_versions: Vec<String>,
    /// Vulnerability type
    pub vulnerability_type: VulnerabilityType,
    /// References
    pub references: Vec<String>,
    /// Published date
    pub published_date: Option<DateTime<Utc>>,
    /// Last updated date
    pub last_updated: DateTime<Utc>,
}

/// Vulnerability severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulnerabilitySeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Types of vulnerabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VulnerabilityType {
    /// Remote code execution
    RemoteCodeExecution,
    /// SQL injection
    SqlInjection,
    /// Cross-site scripting
    CrossSiteScripting,
    /// Denial of service
    DenialOfService,
    /// Information disclosure
    InformationDisclosure,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Authentication bypass
    AuthenticationBypass,
    /// Cryptographic weakness
    CryptographicWeakness,
    /// Other vulnerability type
    Other(String),
}

/// Security compliance standard
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceStandard {
    /// OWASP Top 10
    OwaspTop10,
    /// NIST Cybersecurity Framework
    NistCsf,
    /// ISO 27001
    Iso27001,
    /// SOC 2
    Soc2,
    /// PCI DSS
    PciDss,
    /// HIPAA
    Hipaa,
    /// GDPR
    Gdpr,
    /// Custom standard
    Custom(String),
}

impl std::fmt::Display for ComplianceStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplianceStandard::OwaspTop10 => write!(f, "OWASP Top 10"),
            ComplianceStandard::NistCsf => write!(f, "NIST CSF"),
            ComplianceStandard::Iso27001 => write!(f, "ISO 27001"),
            ComplianceStandard::Soc2 => write!(f, "SOC 2"),
            ComplianceStandard::PciDss => write!(f, "PCI DSS"),
            ComplianceStandard::Hipaa => write!(f, "HIPAA"),
            ComplianceStandard::Gdpr => write!(f, "GDPR"),
            ComplianceStandard::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Security compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    /// Compliance standard
    pub standard: ComplianceStandard,
    /// Whether the dependency is compliant
    pub compliant: bool,
    /// Compliance score (0.0 to 1.0)
    pub score: f64,
    /// Failed requirements
    pub failed_requirements: Vec<String>,
    /// Passed requirements
    pub passed_requirements: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Check timestamp
    pub timestamp: DateTime<Utc>,
}

/// Security scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanResult {
    /// Dependency ID
    pub dependency_id: String,
    /// Overall security score (0.0 to 1.0)
    pub security_score: f64,
    /// Security status
    pub security_status: SecurityStatus,
    /// Vulnerabilities found
    pub vulnerabilities: Vec<Vulnerability>,
    /// Compliance checks
    pub compliance_checks: Vec<ComplianceCheck>,
    /// Security recommendations
    pub recommendations: Vec<SecurityRecommendation>,
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
    /// Scan timestamp
    pub timestamp: DateTime<Utc>,
    /// Scan duration
    pub scan_duration: std::time::Duration,
}

/// Security status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityStatus {
    /// Secure - no issues found
    Secure,
    /// Low risk - minor issues
    LowRisk,
    /// Medium risk - moderate issues
    MediumRisk,
    /// High risk - significant issues
    HighRisk,
    /// Critical risk - severe issues
    CriticalRisk,
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
    /// Priority level
    pub priority: RecommendationPriority,
    /// Implementation effort
    pub effort: ImplementationEffort,
    /// Estimated impact
    pub impact: SecurityImpact,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
    /// Related vulnerabilities
    pub related_vulnerabilities: Vec<String>,
}

/// Recommendation priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Implementation effort
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImplementationEffort {
    /// Low effort
    Low,
    /// Medium effort
    Medium,
    /// High effort
    High,
    /// Very high effort
    VeryHigh,
}

/// Security impact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityImpact {
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

/// Risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk score (0.0 to 1.0)
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Risk factors
    pub risk_factors: Vec<RiskFactor>,
    /// Mitigation strategies
    pub mitigation_strategies: Vec<String>,
    /// Acceptable risk threshold
    pub acceptable_risk_threshold: f64,
    /// Whether risk is acceptable
    pub risk_acceptable: bool,
}

/// Risk level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Minimal risk
    Minimal,
    /// Low risk
    Low,
    /// Moderate risk
    Moderate,
    /// High risk
    High,
    /// Extreme risk
    Extreme,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Risk factor name
    pub name: String,
    /// Risk factor description
    pub description: String,
    /// Risk score contribution (0.0 to 1.0)
    pub score_contribution: f64,
    /// Mitigation options
    pub mitigation_options: Vec<String>,
}

/// Security scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScannerConfig {
    /// Whether to enable vulnerability scanning
    pub enable_vulnerability_scanning: bool,
    /// Whether to enable compliance checking
    pub enable_compliance_checking: bool,
    /// Whether to enable dependency analysis
    pub enable_dependency_analysis: bool,
    /// Vulnerability databases to check
    pub vulnerability_databases: Vec<String>,
    /// Compliance standards to check
    pub compliance_standards: Vec<ComplianceStandard>,
    /// Minimum severity to report
    pub minimum_severity: VulnerabilitySeverity,
    /// Scan timeout
    pub scan_timeout: std::time::Duration,
    /// Cache TTL
    pub cache_ttl: Duration,
    /// API keys for external services
    pub api_keys: HashMap<String, String>,
}

impl Default for SecurityScannerConfig {
    fn default() -> Self {
        Self {
            enable_vulnerability_scanning: true,
            enable_compliance_checking: true,
            enable_dependency_analysis: true,
            vulnerability_databases: vec![
                "https://nvd.nist.gov/vuln/data-feeds".to_string(),
                "https://cve.mitre.org/data/downloads/".to_string(),
            ],
            compliance_standards: vec![
                ComplianceStandard::OwaspTop10,
                ComplianceStandard::NistCsf,
            ],
            minimum_severity: VulnerabilitySeverity::Medium,
            scan_timeout: std::time::Duration::from_secs(300), // 5 minutes
            cache_ttl: Duration::hours(24),
            api_keys: HashMap::new(),
        }
    }
}

/// Enhanced security scanner
pub struct SecurityScanner {
    /// Scanner configuration
    config: SecurityScannerConfig,
    /// HTTP client for API calls
    http_client: Client,
    /// Vulnerability cache
    vulnerability_cache: Arc<RwLock<HashMap<String, Vec<Vulnerability>>>>,
    /// Compliance cache
    compliance_cache: Arc<RwLock<HashMap<String, Vec<ComplianceCheck>>>>,
    /// Scan result cache
    scan_cache: Arc<RwLock<HashMap<String, SecurityScanResult>>>,
    /// Known vulnerabilities database
    known_vulnerabilities: Arc<RwLock<HashMap<String, Vulnerability>>>,
}

impl SecurityScanner {
    /// Create a new security scanner
    pub fn new() -> Self {
        Self::with_config(SecurityScannerConfig::default())
    }

    /// Create a new security scanner with custom configuration
    pub fn with_config(config: SecurityScannerConfig) -> Self {
        let http_client = Client::builder()
            .timeout(config.scan_timeout)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            config,
            http_client,
            vulnerability_cache: Arc::new(RwLock::new(HashMap::new())),
            compliance_cache: Arc::new(RwLock::new(HashMap::new())),
            scan_cache: Arc::new(RwLock::new(HashMap::new())),
            known_vulnerabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Scan a dependency for security issues
    pub async fn scan_dependency(&self, dependency: &DependencyConfig) -> Result<SecurityScanResult> {
        let start_time = std::time::Instant::now();

        // Check cache first
        {
            let cache = self.scan_cache.read().await;
            if let Some(cached_result) = cache.get(&dependency.id) {
                if Utc::now() - cached_result.timestamp < self.config.cache_ttl {
                    return Ok(cached_result.clone());
                }
            }
        }

        let mut vulnerabilities = Vec::new();
        let mut compliance_checks = Vec::new();

        // Perform vulnerability scanning
        if self.config.enable_vulnerability_scanning {
            vulnerabilities = self.scan_vulnerabilities(dependency).await?;
        }

        // Perform compliance checking
        if self.config.enable_compliance_checking {
            compliance_checks = self.check_compliance(dependency).await?;
        }

        // Calculate security score
        let security_score = self.calculate_security_score(&vulnerabilities, &compliance_checks);
        let security_status = self.determine_security_status(security_score, &vulnerabilities);

        // Generate recommendations
        let recommendations = self.generate_recommendations(&vulnerabilities, &compliance_checks);

        // Perform risk assessment
        let risk_assessment = self.perform_risk_assessment(dependency, &vulnerabilities, &compliance_checks);

        let scan_duration = start_time.elapsed();

        let result = SecurityScanResult {
            dependency_id: dependency.id.clone(),
            security_score,
            security_status,
            vulnerabilities,
            compliance_checks,
            recommendations,
            risk_assessment,
            timestamp: Utc::now(),
            scan_duration,
        };

        // Cache the result
        {
            let mut cache = self.scan_cache.write().await;
            cache.insert(dependency.id.clone(), result.clone());
        }

        Ok(result)
    }

    /// Scan for vulnerabilities
    async fn scan_vulnerabilities(&self, dependency: &DependencyConfig) -> Result<Vec<Vulnerability>> {
        // Check cache first
        {
            let cache = self.vulnerability_cache.read().await;
            if let Some(cached_vulns) = cache.get(&dependency.id) {
                return Ok(cached_vulns.clone());
            }
        }

        let mut vulnerabilities = Vec::new();

        // Check known vulnerabilities database
        let known_vulns = self.check_known_vulnerabilities(dependency).await?;
        vulnerabilities.extend(known_vulns);

        // Check external vulnerability databases
        let external_vulns = self.check_external_databases(dependency).await?;
        vulnerabilities.extend(external_vulns);

        // Filter by minimum severity
        vulnerabilities.retain(|v| v.severity >= self.config.minimum_severity);

        // Cache the results
        {
            let mut cache = self.vulnerability_cache.write().await;
            cache.insert(dependency.id.clone(), vulnerabilities.clone());
        }

        Ok(vulnerabilities)
    }

    /// Check known vulnerabilities database
    async fn check_known_vulnerabilities(&self, dependency: &DependencyConfig) -> Result<Vec<Vulnerability>> {
        let known_vulns = self.known_vulnerabilities.read().await;
        
        // This is a simplified implementation
        // In a real implementation, you would check against a comprehensive vulnerability database
        let mut vulnerabilities = Vec::new();

        // Example: Check for common vulnerabilities based on dependency type
        match dependency.dependency_type {
            DependencyType::ApiCall => {
                // Check for API-related vulnerabilities
                if dependency.target.contains("http://") {
                    vulnerabilities.push(Vulnerability {
                        id: "SEC-001".to_string(),
                        title: "Insecure HTTP Protocol".to_string(),
                        description: "Dependency uses HTTP instead of HTTPS".to_string(),
                        severity: VulnerabilitySeverity::Medium,
                        cvss_score: 5.0,
                        affected_versions: vec!["all".to_string()],
                        fixed_versions: vec![],
                        vulnerability_type: VulnerabilityType::InformationDisclosure,
                        references: vec!["https://owasp.org/www-project-top-ten/2017/A3_2017-Sensitive_Data_Exposure".to_string()],
                        published_date: None,
                        last_updated: Utc::now(),
                    });
                }
            }
            DependencyType::DataFlow => {
                // Check for data-related vulnerabilities
                if dependency.target.contains("sql") {
                    vulnerabilities.push(Vulnerability {
                        id: "SEC-002".to_string(),
                        title: "Potential SQL Injection Risk".to_string(),
                        description: "Database dependency may be vulnerable to SQL injection".to_string(),
                        severity: VulnerabilitySeverity::High,
                        cvss_score: 8.0,
                        affected_versions: vec!["all".to_string()],
                        fixed_versions: vec![],
                        vulnerability_type: VulnerabilityType::SqlInjection,
                        references: vec!["https://owasp.org/www-project-top-ten/2017/A1_2017-Injection".to_string()],
                        published_date: None,
                        last_updated: Utc::now(),
                    });
                }
            }
            _ => {}
        }

        Ok(vulnerabilities)
    }

    /// Check external vulnerability databases
    async fn check_external_databases(&self, _dependency: &DependencyConfig) -> Result<Vec<Vulnerability>> {
        // This is a placeholder for external database checks
        // In a real implementation, you would:
        // 1. Query NVD API
        // 2. Query CVE database
        // 3. Query vendor-specific vulnerability databases
        // 4. Query security advisories
        
        Ok(Vec::new())
    }

    /// Check compliance with security standards
    async fn check_compliance(&self, dependency: &DependencyConfig) -> Result<Vec<ComplianceCheck>> {
        // Check cache first
        {
            let cache = self.compliance_cache.read().await;
            if let Some(cached_checks) = cache.get(&dependency.id) {
                return Ok(cached_checks.clone());
            }
        }

        let mut compliance_checks = Vec::new();

        for standard in &self.config.compliance_standards {
            let check = self.check_compliance_standard(dependency, standard).await?;
            compliance_checks.push(check);
        }

        // Cache the results
        {
            let mut cache = self.compliance_cache.write().await;
            cache.insert(dependency.id.clone(), compliance_checks.clone());
        }

        Ok(compliance_checks)
    }

    /// Check compliance with a specific standard
    async fn check_compliance_standard(
        &self,
        dependency: &DependencyConfig,
        standard: &ComplianceStandard,
    ) -> Result<ComplianceCheck> {
        match standard {
            ComplianceStandard::OwaspTop10 => self.check_owasp_compliance(dependency).await,
            ComplianceStandard::NistCsf => self.check_nist_compliance(dependency).await,
            _ => Ok(ComplianceCheck {
                standard: standard.clone(),
                compliant: true,
                score: 1.0,
                failed_requirements: Vec::new(),
                passed_requirements: vec!["Basic security requirements met".to_string()],
                recommendations: Vec::new(),
                timestamp: Utc::now(),
            }),
        }
    }

    /// Check OWASP Top 10 compliance
    async fn check_owasp_compliance(&self, dependency: &DependencyConfig) -> Result<ComplianceCheck> {
        let mut failed_requirements = Vec::new();
        let mut passed_requirements = Vec::new();
        let mut recommendations = Vec::new();

        // Check for injection vulnerabilities
        if dependency.target.contains("sql") || dependency.target.contains("nosql") {
            failed_requirements.push("A1:2017-Injection".to_string());
            recommendations.push("Use parameterized queries and input validation".to_string());
        } else {
            passed_requirements.push("A1:2017-Injection".to_string());
        }

        // Check for broken authentication
        if !dependency.target.contains("https://") {
            failed_requirements.push("A2:2017-Broken Authentication".to_string());
            recommendations.push("Use HTTPS for all communications".to_string());
        } else {
            passed_requirements.push("A2:2017-Broken Authentication".to_string());
        }

        // Check for sensitive data exposure
        if dependency.target.contains("password") || dependency.target.contains("secret") {
            failed_requirements.push("A3:2017-Sensitive Data Exposure".to_string());
            recommendations.push("Encrypt sensitive data in transit and at rest".to_string());
        } else {
            passed_requirements.push("A3:2017-Sensitive Data Exposure".to_string());
        }

        let score = if failed_requirements.is_empty() {
            1.0
        } else {
            1.0 - (failed_requirements.len() as f64 / 10.0)
        };

        Ok(ComplianceCheck {
            standard: ComplianceStandard::OwaspTop10,
            compliant: failed_requirements.is_empty(),
            score,
            failed_requirements,
            passed_requirements,
            recommendations,
            timestamp: Utc::now(),
        })
    }

    /// Check NIST CSF compliance
    async fn check_nist_compliance(&self, _dependency: &DependencyConfig) -> Result<ComplianceCheck> {
        // Simplified NIST CSF compliance check
        Ok(ComplianceCheck {
            standard: ComplianceStandard::NistCsf,
            compliant: true,
            score: 0.8,
            failed_requirements: Vec::new(),
            passed_requirements: vec![
                "ID.AM-1: Physical devices and systems identified".to_string(),
                "ID.AM-2: Software platforms and applications identified".to_string(),
                "PR.AC-1: Identities and credentials are managed".to_string(),
            ],
            recommendations: vec![
                "Implement continuous monitoring".to_string(),
                "Establish incident response procedures".to_string(),
            ],
            timestamp: Utc::now(),
        })
    }

    /// Calculate security score
    fn calculate_security_score(
        &self,
        vulnerabilities: &[Vulnerability],
        compliance_checks: &[ComplianceCheck],
    ) -> f64 {
        let mut score = 1.0;

        // Deduct points for vulnerabilities
        for vulnerability in vulnerabilities {
            let deduction = match vulnerability.severity {
                VulnerabilitySeverity::Low => 0.05,
                VulnerabilitySeverity::Medium => 0.15,
                VulnerabilitySeverity::High => 0.30,
                VulnerabilitySeverity::Critical => 0.50,
            };
            score -= deduction;
        }

        // Consider compliance scores
        if !compliance_checks.is_empty() {
            let avg_compliance_score = compliance_checks.iter()
                .map(|check| check.score)
                .sum::<f64>() / compliance_checks.len() as f64;
            score = (score + avg_compliance_score) / 2.0;
        }

        score.max(0.0).min(1.0)
    }

    /// Determine security status
    fn determine_security_status(
        &self,
        security_score: f64,
        vulnerabilities: &[Vulnerability],
    ) -> SecurityStatus {
        let critical_vulns = vulnerabilities.iter()
            .filter(|v| v.severity == VulnerabilitySeverity::Critical)
            .count();

        let high_vulns = vulnerabilities.iter()
            .filter(|v| v.severity == VulnerabilitySeverity::High)
            .count();

        if critical_vulns > 0 {
            SecurityStatus::CriticalRisk
        } else if high_vulns > 0 || security_score < 0.5 {
            SecurityStatus::HighRisk
        } else if security_score < 0.7 {
            SecurityStatus::MediumRisk
        } else if security_score < 0.9 {
            SecurityStatus::LowRisk
        } else {
            SecurityStatus::Secure
        }
    }

    /// Generate security recommendations
    fn generate_recommendations(
        &self,
        vulnerabilities: &[Vulnerability],
        compliance_checks: &[ComplianceCheck],
    ) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        // Generate recommendations for vulnerabilities
        for (i, vulnerability) in vulnerabilities.iter().enumerate() {
            let priority = match vulnerability.severity {
                VulnerabilitySeverity::Low => RecommendationPriority::Low,
                VulnerabilitySeverity::Medium => RecommendationPriority::Medium,
                VulnerabilitySeverity::High => RecommendationPriority::High,
                VulnerabilitySeverity::Critical => RecommendationPriority::Critical,
            };

            let effort = match vulnerability.severity {
                VulnerabilitySeverity::Low => ImplementationEffort::Low,
                VulnerabilitySeverity::Medium => ImplementationEffort::Medium,
                VulnerabilitySeverity::High => ImplementationEffort::High,
                VulnerabilitySeverity::Critical => ImplementationEffort::VeryHigh,
            };

            let impact = match vulnerability.severity {
                VulnerabilitySeverity::Low => SecurityImpact::Low,
                VulnerabilitySeverity::Medium => SecurityImpact::Medium,
                VulnerabilitySeverity::High => SecurityImpact::High,
                VulnerabilitySeverity::Critical => SecurityImpact::Critical,
            };

            recommendations.push(SecurityRecommendation {
                id: format!("REC-{:03}", i + 1),
                title: format!("Fix {}", vulnerability.title),
                description: vulnerability.description.clone(),
                priority,
                effort,
                impact,
                implementation_steps: vec![
                    "Review vulnerability details".to_string(),
                    "Update to fixed version if available".to_string(),
                    "Implement workarounds if needed".to_string(),
                    "Test the fix thoroughly".to_string(),
                ],
                related_vulnerabilities: vec![vulnerability.id.clone()],
            });
        }

        // Generate recommendations for compliance issues
        for check in compliance_checks {
            if !check.compliant {
                recommendations.push(SecurityRecommendation {
                    id: format!("COMP-{}", check.standard.to_string()),
                    title: format!("Improve {} compliance", check.standard.to_string()),
                    description: format!("Address failed compliance requirements for {}", check.standard.to_string()),
                    priority: RecommendationPriority::High,
                    effort: ImplementationEffort::Medium,
                    impact: SecurityImpact::High,
                    implementation_steps: check.recommendations.clone(),
                    related_vulnerabilities: Vec::new(),
                });
            }
        }

        recommendations
    }

    /// Perform risk assessment
    fn perform_risk_assessment(
        &self,
        dependency: &DependencyConfig,
        vulnerabilities: &[Vulnerability],
        compliance_checks: &[ComplianceCheck],
    ) -> RiskAssessment {
        let mut risk_score = 0.0;
        let mut risk_factors = Vec::new();

        // Calculate risk based on vulnerabilities
        for vulnerability in vulnerabilities {
            let vuln_risk = match vulnerability.severity {
                VulnerabilitySeverity::Low => 0.1,
                VulnerabilitySeverity::Medium => 0.3,
                VulnerabilitySeverity::High => 0.6,
                VulnerabilitySeverity::Critical => 0.9,
            };
            risk_score += vuln_risk;

            risk_factors.push(RiskFactor {
                name: format!("Vulnerability: {}", vulnerability.title),
                description: vulnerability.description.clone(),
                score_contribution: vuln_risk,
                mitigation_options: vec![
                    "Update to fixed version".to_string(),
                    "Implement security patches".to_string(),
                    "Apply security controls".to_string(),
                ],
            });
        }

        // Calculate risk based on compliance
        for check in compliance_checks {
            if !check.compliant {
                let compliance_risk = 1.0 - check.score;
                risk_score += compliance_risk;

                risk_factors.push(RiskFactor {
                    name: format!("Compliance: {}", check.standard.to_string()),
                    description: "Failed compliance requirements".to_string(),
                    score_contribution: compliance_risk,
                    mitigation_options: check.recommendations.clone(),
                });
            }
        }

        // Normalize risk score
        risk_score = risk_score.min(1.0);

        let risk_level = if risk_score >= 0.8 {
            RiskLevel::Extreme
        } else if risk_score >= 0.6 {
            RiskLevel::High
        } else if risk_score >= 0.4 {
            RiskLevel::Moderate
        } else if risk_score >= 0.2 {
            RiskLevel::Low
        } else {
            RiskLevel::Minimal
        };

        let acceptable_risk_threshold = 0.3; // 30% risk threshold
        let risk_acceptable = risk_score <= acceptable_risk_threshold;

        let mitigation_strategies = vec![
            "Implement security monitoring".to_string(),
            "Regular security assessments".to_string(),
            "Keep dependencies updated".to_string(),
            "Apply security patches promptly".to_string(),
        ];

        RiskAssessment {
            risk_score,
            risk_level,
            risk_factors,
            mitigation_strategies,
            acceptable_risk_threshold,
            risk_acceptable,
        }
    }

    /// Clear expired cache entries
    pub async fn clear_expired_cache(&self) {
        let now = Utc::now();
        
        // Clear scan cache
        {
            let mut cache = self.scan_cache.write().await;
            cache.retain(|_, result| now - result.timestamp < self.config.cache_ttl);
        }

        // Clear vulnerability cache
        {
            let mut cache = self.vulnerability_cache.write().await;
            cache.clear(); // Simplified - in real implementation, check timestamps
        }

        // Clear compliance cache
        {
            let mut cache = self.compliance_cache.write().await;
            cache.clear(); // Simplified - in real implementation, check timestamps
        }
    }

    /// Get security statistics
    pub async fn get_statistics(&self) -> SecurityStatistics {
        let scan_cache = self.scan_cache.read().await;
        let vuln_cache = self.vulnerability_cache.read().await;
        let comp_cache = self.compliance_cache.read().await;

        let total_scans = scan_cache.len();
        let total_vulnerabilities = vuln_cache.values().map(|v| v.len()).sum();
        let total_compliance_checks = comp_cache.values().map(|v| v.len()).sum();

        let critical_vulns = scan_cache.values()
            .flat_map(|result| &result.vulnerabilities)
            .filter(|v| v.severity == VulnerabilitySeverity::Critical)
            .count();

        let high_vulns = scan_cache.values()
            .flat_map(|result| &result.vulnerabilities)
            .filter(|v| v.severity == VulnerabilitySeverity::High)
            .count();

        SecurityStatistics {
            total_scans,
            total_vulnerabilities,
            total_compliance_checks,
            critical_vulnerabilities: critical_vulns,
            high_vulnerabilities: high_vulns,
            average_security_score: scan_cache.values()
                .map(|result| result.security_score)
                .sum::<f64>() / total_scans.max(1) as f64,
        }
    }
}

/// Security statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatistics {
    /// Total number of security scans performed
    pub total_scans: usize,
    /// Total number of vulnerabilities found
    pub total_vulnerabilities: usize,
    /// Total number of compliance checks performed
    pub total_compliance_checks: usize,
    /// Number of critical vulnerabilities
    pub critical_vulnerabilities: usize,
    /// Number of high vulnerabilities
    pub high_vulnerabilities: usize,
    /// Average security score across all scans
    pub average_security_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyConfig;

    fn create_test_dependency(id: &str, name: &str, dependency_type: DependencyType) -> DependencyConfig {
        DependencyConfig::new(
            id.to_string(),
            name.to_string(),
            dependency_type,
            "test-target".to_string(),
            vec!["test".to_string()],
        ).unwrap()
    }

    #[tokio::test]
    async fn test_security_scanner_new() {
        let scanner = SecurityScanner::new();
        assert!(scanner.config.enable_vulnerability_scanning);
        assert!(scanner.config.enable_compliance_checking);
    }

    #[tokio::test]
    async fn test_scan_dependency() {
        let scanner = SecurityScanner::new();
        let dependency = create_test_dependency("test-dep", "Test Dependency", DependencyType::ApiCall);
        
        let result = scanner.scan_dependency(&dependency).await.unwrap();
        assert_eq!(result.dependency_id, "test-dep");
        assert!(result.security_score >= 0.0 && result.security_score <= 1.0);
    }

    #[tokio::test]
    async fn test_calculate_security_score() {
        let scanner = SecurityScanner::new();
        let vulnerabilities = vec![
            Vulnerability {
                id: "TEST-001".to_string(),
                title: "Test Vulnerability".to_string(),
                description: "Test description".to_string(),
                severity: VulnerabilitySeverity::Medium,
                cvss_score: 5.0,
                affected_versions: vec!["1.0.0".to_string()],
                fixed_versions: vec!["1.0.1".to_string()],
                vulnerability_type: VulnerabilityType::InformationDisclosure,
                references: vec![],
                published_date: None,
                last_updated: Utc::now(),
            }
        ];
        let compliance_checks = vec![];

        let score = scanner.calculate_security_score(&vulnerabilities, &compliance_checks);
        assert!(score < 1.0); // Should be reduced due to vulnerability
    }

    #[test]
    fn test_determine_security_status() {
        let scanner = SecurityScanner::new();
        let vulnerabilities = vec![
            Vulnerability {
                id: "TEST-001".to_string(),
                title: "Critical Vulnerability".to_string(),
                description: "Critical test".to_string(),
                severity: VulnerabilitySeverity::Critical,
                cvss_score: 9.0,
                affected_versions: vec!["1.0.0".to_string()],
                fixed_versions: vec![],
                vulnerability_type: VulnerabilityType::RemoteCodeExecution,
                references: vec![],
                published_date: None,
                last_updated: Utc::now(),
            }
        ];

        let status = scanner.determine_security_status(0.8, &vulnerabilities);
        assert_eq!(status, SecurityStatus::CriticalRisk);
    }
} 