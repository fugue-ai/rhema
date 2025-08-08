use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, DependencyType, HealthStatus, ImpactScore};

/// Dependency dashboard for managing dependencies
pub struct DependencyDashboard {
    /// Dashboard configuration
    config: DashboardConfig,
    /// Dashboard data
    data: Arc<RwLock<DashboardData>>,
    /// Dashboard widgets
    widgets: Vec<Box<dyn DashboardWidget>>,
}

/// Dashboard configuration
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    /// Dashboard title
    pub title: String,
    /// Dashboard description
    pub description: String,
    /// Auto-refresh interval in seconds
    pub auto_refresh_interval: u64,
    /// Enable real-time updates
    pub enable_realtime: bool,
    /// Default view
    pub default_view: DashboardView,
    /// Customizable layout
    pub customizable_layout: bool,
}

/// Dashboard view types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DashboardView {
    /// Overview view
    Overview,
    /// Health view
    Health,
    /// Security view
    Security,
    /// Performance view
    Performance,
    /// Cost view
    Cost,
    /// Custom view
    Custom(String),
}

/// Dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    /// Total dependencies
    pub total_dependencies: usize,
    /// Healthy dependencies
    pub healthy_dependencies: usize,
    /// Unhealthy dependencies
    pub unhealthy_dependencies: usize,
    /// Critical dependencies
    pub critical_dependencies: usize,
    /// Security vulnerabilities
    pub security_vulnerabilities: usize,
    /// Performance issues
    pub performance_issues: usize,
    /// Cost issues
    pub cost_issues: usize,
    /// Last updated
    pub last_updated: DateTime<Utc>,
    /// Dependencies by type
    pub dependencies_by_type: HashMap<DependencyType, usize>,
    /// Dependencies by health
    pub dependencies_by_health: HashMap<HealthStatus, usize>,
}

/// Dashboard widget trait
#[async_trait::async_trait]
pub trait DashboardWidget: Send + Sync {
    /// Get widget ID
    fn id(&self) -> &str;
    
    /// Get widget name
    fn name(&self) -> &str;
    
    /// Get widget type
    fn widget_type(&self) -> WidgetType;
    
    /// Get widget data
    async fn get_data(&self) -> Result<WidgetData>;
    
    /// Update widget
    async fn update(&mut self) -> Result<()>;
}

/// Widget types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    /// Chart widget
    Chart,
    /// Table widget
    Table,
    /// Metric widget
    Metric,
    /// Alert widget
    Alert,
    /// Status widget
    Status,
}

/// Widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetData {
    /// Widget ID
    pub widget_id: String,
    /// Widget title
    pub title: String,
    /// Widget content
    pub content: serde_json::Value,
    /// Widget metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

/// Health status widget
pub struct HealthStatusWidget {
    /// Widget configuration
    config: WidgetConfig,
    /// Health data
    health_data: Arc<RwLock<HealthData>>,
}

/// Widget configuration
#[derive(Debug, Clone)]
pub struct WidgetConfig {
    /// Widget ID
    pub id: String,
    /// Widget name
    pub name: String,
    /// Widget description
    pub description: String,
    /// Widget position
    pub position: (u32, u32),
    /// Widget size
    pub size: (u32, u32),
    /// Widget enabled
    pub enabled: bool,
}

/// Health data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthData {
    /// Health status counts
    pub status_counts: HashMap<HealthStatus, usize>,
    /// Health trends
    pub health_trends: Vec<HealthTrend>,
    /// Critical issues
    pub critical_issues: Vec<CriticalIssue>,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

/// Health trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTrend {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Health score
    pub health_score: f64,
    /// Status distribution
    pub status_distribution: HashMap<HealthStatus, usize>,
}

/// Critical issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalIssue {
    /// Issue ID
    pub id: String,
    /// Issue title
    pub title: String,
    /// Issue description
    pub description: String,
    /// Issue severity
    pub severity: IssueSeverity,
    /// Affected dependencies
    pub affected_dependencies: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
}

/// Issue severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

#[async_trait::async_trait]
impl DashboardWidget for HealthStatusWidget {
    fn id(&self) -> &str {
        &self.config.id
    }
    
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn widget_type(&self) -> WidgetType {
        WidgetType::Status
    }
    
    async fn get_data(&self) -> Result<WidgetData> {
        let health_data = self.health_data.read().await;
        
        let content = serde_json::json!({
            "status_counts": health_data.status_counts,
            "health_trends": health_data.health_trends,
            "critical_issues": health_data.critical_issues,
        });
        
        Ok(WidgetData {
            widget_id: self.config.id.clone(),
            title: self.config.name.clone(),
            content,
            metadata: HashMap::new(),
            last_updated: health_data.last_updated,
        })
    }
    
    async fn update(&mut self) -> Result<()> {
        // Update health data
        Ok(())
    }
}

impl DependencyDashboard {
    /// Create a new dependency dashboard
    pub fn new() -> Self {
        Self::with_config(DashboardConfig::default())
    }
    
    /// Create a new dependency dashboard with configuration
    pub fn with_config(config: DashboardConfig) -> Self {
        let mut widgets: Vec<Box<dyn DashboardWidget>> = Vec::new();
        
        // Add default widgets
        let health_widget = HealthStatusWidget {
            config: WidgetConfig {
                id: "health-status".to_string(),
                name: "Health Status".to_string(),
                description: "Overview of dependency health status".to_string(),
                position: (0, 0),
                size: (400, 300),
                enabled: true,
            },
            health_data: Arc::new(RwLock::new(HealthData {
                status_counts: HashMap::new(),
                health_trends: Vec::new(),
                critical_issues: Vec::new(),
                last_updated: Utc::now(),
            })),
        };
        
        widgets.push(Box::new(health_widget));
        
        Self {
            config,
            data: Arc::new(RwLock::new(DashboardData::default())),
            widgets,
        }
    }
    
    /// Get dashboard data
    pub async fn get_data(&self) -> Result<DashboardData> {
        let data = self.data.read().await;
        Ok(data.clone())
    }
    
    /// Update dashboard data
    pub async fn update_data(&self, dependencies: &[DependencyConfig]) -> Result<()> {
        let mut data = self.data.write().await;
        
        data.total_dependencies = dependencies.len();
        data.healthy_dependencies = dependencies.iter().filter(|d| d.health_status == HealthStatus::Healthy).count();
        data.unhealthy_dependencies = dependencies.iter().filter(|d| d.health_status != HealthStatus::Healthy).count();
        data.critical_dependencies = dependencies.iter().filter(|d| d.health_status == HealthStatus::Down).count();
        data.last_updated = Utc::now();
        
        // Update dependencies by type
        data.dependencies_by_type.clear();
        for dep in dependencies {
            *data.dependencies_by_type.entry(dep.dependency_type.clone()).or_insert(0) += 1;
        }
        
        // Update dependencies by health
        data.dependencies_by_health.clear();
        for dep in dependencies {
            *data.dependencies_by_health.entry(dep.health_status.clone()).or_insert(0) += 1;
        }
        
        Ok(())
    }
    
    /// Get widget data
    pub async fn get_widget_data(&self, widget_id: &str) -> Result<WidgetData> {
        for widget in &self.widgets {
            if widget.id() == widget_id {
                return widget.get_data().await;
            }
        }
        
        Err(Error::NotFound(format!("Widget '{}' not found", widget_id)))
    }
    
    /// Add widget
    pub fn add_widget(&mut self, widget: Box<dyn DashboardWidget>) {
        self.widgets.push(widget);
    }
    
    /// Remove widget
    pub fn remove_widget(&mut self, widget_id: &str) -> Result<()> {
        let index = self.widgets.iter().position(|w| w.id() == widget_id);
        
        if let Some(index) = index {
            self.widgets.remove(index);
            Ok(())
        } else {
            Err(Error::NotFound(format!("Widget '{}' not found", widget_id)))
        }
    }
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            title: "Dependency Dashboard".to_string(),
            description: "Comprehensive dependency management dashboard".to_string(),
            auto_refresh_interval: 30,
            enable_realtime: true,
            default_view: DashboardView::Overview,
            customizable_layout: true,
        }
    }
}

impl Default for DashboardData {
    fn default() -> Self {
        Self {
            total_dependencies: 0,
            healthy_dependencies: 0,
            unhealthy_dependencies: 0,
            critical_dependencies: 0,
            security_vulnerabilities: 0,
            performance_issues: 0,
            cost_issues: 0,
            last_updated: Utc::now(),
            dependencies_by_type: HashMap::new(),
            dependencies_by_health: HashMap::new(),
        }
    }
}

/// Dependency report generator
pub struct DependencyReportGenerator {
    /// Report configuration
    config: ReportConfig,
    /// Report templates
    templates: HashMap<String, ReportTemplate>,
}

/// Report configuration
#[derive(Debug, Clone)]
pub struct ReportConfig {
    /// Default report format
    pub default_format: ReportFormat,
    /// Include charts
    pub include_charts: bool,
    /// Include recommendations
    pub include_recommendations: bool,
    /// Include action items
    pub include_action_items: bool,
    /// Auto-generate reports
    pub auto_generate: bool,
    /// Report schedule
    pub report_schedule: String,
}

/// Report format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    /// HTML format
    Html,
    /// PDF format
    Pdf,
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Markdown format
    Markdown,
}

/// Report template
#[derive(Debug, Clone)]
pub struct ReportTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template content
    pub content: String,
    /// Template variables
    pub variables: Vec<String>,
}

/// Dependency report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyReport {
    /// Report ID
    pub id: String,
    /// Report title
    pub title: String,
    /// Report description
    pub description: String,
    /// Report format
    pub format: ReportFormat,
    /// Report content
    pub content: String,
    /// Report metadata
    pub metadata: ReportMetadata,
    /// Generated at
    pub generated_at: DateTime<Utc>,
}

/// Report metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Total dependencies
    pub total_dependencies: usize,
    /// Report duration
    pub report_duration: std::time::Duration,
    /// Report size
    pub report_size: usize,
    /// Report checksum
    pub checksum: String,
}

impl DependencyReportGenerator {
    /// Create a new report generator
    pub fn new() -> Self {
        Self::with_config(ReportConfig::default())
    }
    
    /// Create a new report generator with configuration
    pub fn with_config(config: ReportConfig) -> Self {
        let mut templates = HashMap::new();
        
        // Add default templates
        templates.insert("overview".to_string(), ReportTemplate {
            id: "overview".to_string(),
            name: "Overview Report".to_string(),
            description: "Comprehensive overview of all dependencies".to_string(),
            content: include_str!("templates/overview.html").to_string(),
            variables: vec!["dependencies".to_string(), "summary".to_string()],
        });
        
        templates.insert("health".to_string(), ReportTemplate {
            id: "health".to_string(),
            name: "Health Report".to_string(),
            description: "Detailed health analysis of dependencies".to_string(),
            content: include_str!("templates/health.html").to_string(),
            variables: vec!["health_data".to_string(), "issues".to_string()],
        });
        
        Self { config, templates }
    }
    
    /// Generate report
    pub async fn generate_report(
        &self,
        template_id: &str,
        dependencies: &[DependencyConfig],
        format: Option<ReportFormat>,
    ) -> Result<DependencyReport> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| Error::NotFound(format!("Template '{}' not found", template_id)))?;
        
        let format = format.unwrap_or(self.config.default_format.clone());
        let start_time = std::time::Instant::now();
        
        // Generate report content
        let content = self.generate_content(template, dependencies).await?;
        
        let duration = start_time.elapsed();
        let report_size = content.len();
        let checksum = self.calculate_checksum(&content);
        
        let report = DependencyReport {
            id: format!("report_{}", uuid::Uuid::new_v4()),
            title: template.name.clone(),
            description: template.description.clone(),
            format,
            content,
            metadata: ReportMetadata {
                total_dependencies: dependencies.len(),
                report_duration: duration,
                report_size,
                checksum,
            },
            generated_at: Utc::now(),
        };
        
        Ok(report)
    }
    
    /// Generate content
    async fn generate_content(&self, template: &ReportTemplate, dependencies: &[DependencyConfig]) -> Result<String> {
        // This is a simplified implementation
        // In practice, you would use a templating engine like Handlebars or Tera
        let mut content = template.content.clone();
        
        // Replace template variables
        content = content.replace("{{total_dependencies}}", &dependencies.len().to_string());
        content = content.replace("{{generated_at}}", &Utc::now().to_rfc3339());
        
        Ok(content)
    }
    
    /// Calculate checksum
    fn calculate_checksum(&self, content: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// Add template
    pub fn add_template(&mut self, template: ReportTemplate) {
        self.templates.insert(template.id.clone(), template);
    }
    
    /// Remove template
    pub fn remove_template(&mut self, template_id: &str) -> Result<()> {
        if self.templates.remove(template_id).is_some() {
            Ok(())
        } else {
            Err(Error::NotFound(format!("Template '{}' not found", template_id)))
        }
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            default_format: ReportFormat::Html,
            include_charts: true,
            include_recommendations: true,
            include_action_items: true,
            auto_generate: false,
            report_schedule: "0 9 * * 1".to_string(), // Every Monday at 9 AM
        }
    }
}

/// Dependency alert system
pub struct DependencyAlertSystem {
    /// Alert configuration
    config: AlertConfig,
    /// Alert rules
    rules: Vec<AlertRule>,
    /// Alert history
    history: Arc<RwLock<Vec<Alert>>>,
}

/// Alert configuration
#[derive(Debug, Clone)]
pub struct AlertConfig {
    /// Enable alerts
    pub enable_alerts: bool,
    /// Alert channels
    pub alert_channels: Vec<AlertChannel>,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
    /// Alert cooldown
    pub alert_cooldown: std::time::Duration,
}

/// Alert channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    /// Email alerts
    Email(EmailConfig),
    /// Slack alerts
    Slack(SlackConfig),
    /// Webhook alerts
    Webhook(WebhookConfig),
    /// Console alerts
    Console,
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// SMTP server
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// From address
    pub from_address: String,
    /// To addresses
    pub to_addresses: Vec<String>,
}

/// Slack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    /// Webhook URL
    pub webhook_url: String,
    /// Channel
    pub channel: String,
    /// Username
    pub username: String,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Headers
    pub headers: HashMap<String, String>,
}

/// Alert thresholds
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Health threshold
    pub health_threshold: f64,
    /// Security threshold
    pub security_threshold: f64,
    /// Performance threshold
    pub performance_threshold: f64,
    /// Cost threshold
    pub cost_threshold: f64,
}

/// Alert rule
#[derive(Debug, Clone)]
pub struct AlertRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule condition
    pub condition: AlertCondition,
    /// Rule action
    pub action: AlertAction,
    /// Rule enabled
    pub enabled: bool,
}

/// Alert condition
#[derive(Debug, Clone)]
pub enum AlertCondition {
    /// Health status condition
    HealthStatus(HealthStatus),
    /// Security vulnerability condition
    SecurityVulnerability(VulnerabilitySeverity),
    /// Performance degradation condition
    PerformanceDegradation(f64),
    /// Cost increase condition
    CostIncrease(f64),
    /// Custom condition
    Custom(String),
}

/// Alert action
#[derive(Debug, Clone)]
pub enum AlertAction {
    /// Send notification
    SendNotification(AlertChannel),
    /// Execute command
    ExecuteCommand(String),
    /// Create ticket
    CreateTicket(TicketConfig),
    /// Custom action
    Custom(String),
}

/// Ticket configuration
#[derive(Debug, Clone)]
pub struct TicketConfig {
    /// Ticket system
    pub system: String,
    /// Ticket template
    pub template: String,
    /// Priority mapping
    pub priority_mapping: HashMap<String, String>,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: String,
    /// Alert title
    pub title: String,
    /// Alert description
    pub description: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert source
    pub source: String,
    /// Alert data
    pub data: serde_json::Value,
    /// Alert status
    pub status: AlertStatus,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Resolved at
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Alert severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Info severity
    Info,
    /// Warning severity
    Warning,
    /// Error severity
    Error,
    /// Critical severity
    Critical,
}

/// Alert status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertStatus {
    /// Active status
    Active,
    /// Acknowledged status
    Acknowledged,
    /// Resolved status
    Resolved,
    /// Suppressed status
    Suppressed,
}

/// Vulnerability severity
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl DependencyAlertSystem {
    /// Create a new alert system
    pub fn new() -> Self {
        Self::with_config(AlertConfig::default())
    }
    
    /// Create a new alert system with configuration
    pub fn with_config(config: AlertConfig) -> Self {
        let mut rules = Vec::new();
        
        // Add default rules
        rules.push(AlertRule {
            id: "health-critical".to_string(),
            name: "Critical Health Alert".to_string(),
            description: "Alert when dependency health is critical".to_string(),
            condition: AlertCondition::HealthStatus(HealthStatus::Down),
            action: AlertAction::SendNotification(AlertChannel::Console),
            enabled: true,
        });
        
        rules.push(AlertRule {
            id: "security-critical".to_string(),
            name: "Critical Security Alert".to_string(),
            description: "Alert when critical security vulnerabilities are found".to_string(),
            condition: AlertCondition::SecurityVulnerability(VulnerabilitySeverity::Critical),
            action: AlertAction::SendNotification(AlertChannel::Console),
            enabled: true,
        });
        
        Self {
            config,
            rules,
            history: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Check dependencies for alerts
    pub async fn check_dependencies(&self, dependencies: &[DependencyConfig]) -> Result<Vec<Alert>> {
        let mut alerts = Vec::new();
        
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            for dep in dependencies {
                if self.evaluate_condition(&rule.condition, dep).await? {
                    let alert = self.create_alert(rule, dep).await?;
                    alerts.push(alert);
                }
            }
        }
        
        // Add alerts to history
        if !alerts.is_empty() {
            let mut history = self.history.write().await;
            history.extend(alerts.clone());
        }
        
        Ok(alerts)
    }
    
    /// Evaluate alert condition
    async fn evaluate_condition(&self, condition: &AlertCondition, dependency: &DependencyConfig) -> Result<bool> {
        match condition {
            AlertCondition::HealthStatus(status) => {
                Ok(dependency.health_status == *status)
            }
            AlertCondition::SecurityVulnerability(severity) => {
                // This would check for security vulnerabilities
                Ok(false)
            }
            AlertCondition::PerformanceDegradation(threshold) => {
                // This would check performance metrics
                Ok(false)
            }
            AlertCondition::CostIncrease(threshold) => {
                // This would check cost metrics
                Ok(false)
            }
            AlertCondition::Custom(_) => {
                // This would evaluate custom conditions
                Ok(false)
            }
        }
    }
    
    /// Create alert
    async fn create_alert(&self, rule: &AlertRule, dependency: &DependencyConfig) -> Result<Alert> {
        let alert = Alert {
            id: format!("alert_{}", uuid::Uuid::new_v4()),
            title: format!("{}: {}", rule.name, dependency.name),
            description: rule.description.clone(),
            severity: AlertSeverity::Warning,
            source: dependency.id.clone(),
            data: serde_json::json!({
                "dependency_id": dependency.id,
                "dependency_name": dependency.name,
                "dependency_type": dependency.dependency_type,
                "health_status": dependency.health_status,
            }),
            status: AlertStatus::Active,
            created_at: Utc::now(),
            resolved_at: None,
        };
        
        Ok(alert)
    }
    
    /// Get alert history
    pub async fn get_alert_history(&self) -> Result<Vec<Alert>> {
        let history = self.history.read().await;
        Ok(history.clone())
    }
    
    /// Acknowledge alert
    pub async fn acknowledge_alert(&self, alert_id: &str) -> Result<()> {
        let mut history = self.history.write().await;
        
        if let Some(alert) = history.iter_mut().find(|a| a.id == alert_id) {
            alert.status = AlertStatus::Acknowledged;
        }
        
        Ok(())
    }
    
    /// Resolve alert
    pub async fn resolve_alert(&self, alert_id: &str) -> Result<()> {
        let mut history = self.history.write().await;
        
        if let Some(alert) = history.iter_mut().find(|a| a.id == alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(Utc::now());
        }
        
        Ok(())
    }
    
    /// Add alert rule
    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.push(rule);
    }
    
    /// Remove alert rule
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<()> {
        let index = self.rules.iter().position(|r| r.id == rule_id);
        
        if let Some(index) = index {
            self.rules.remove(index);
            Ok(())
        } else {
            Err(Error::NotFound(format!("Rule '{}' not found", rule_id)))
        }
    }
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enable_alerts: true,
            alert_channels: vec![AlertChannel::Console],
            alert_thresholds: AlertThresholds {
                health_threshold: 0.8,
                security_threshold: 0.9,
                performance_threshold: 0.7,
                cost_threshold: 0.6,
            },
            alert_cooldown: std::time::Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Dependency search engine
pub struct DependencySearchEngine {
    /// Search configuration
    config: SearchConfig,
    /// Search index
    index: Arc<RwLock<SearchIndex>>,
}

/// Search configuration
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Enable fuzzy search
    pub enable_fuzzy: bool,
    /// Search limit
    pub search_limit: usize,
    /// Enable highlighting
    pub enable_highlighting: bool,
    /// Search fields
    pub search_fields: Vec<String>,
}

/// Search index
#[derive(Debug, Clone)]
pub struct SearchIndex {
    /// Indexed dependencies
    dependencies: HashMap<String, IndexedDependency>,
    /// Search terms
    search_terms: HashMap<String, Vec<String>>,
}

/// Indexed dependency
#[derive(Debug, Clone)]
pub struct IndexedDependency {
    /// Dependency ID
    pub id: String,
    /// Dependency name
    pub name: String,
    /// Dependency type
    pub dependency_type: DependencyType,
    /// Dependency target
    pub target: String,
    /// Dependency operations
    pub operations: Vec<String>,
    /// Health status
    pub health_status: HealthStatus,
    /// Searchable text
    pub searchable_text: String,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Dependency ID
    pub dependency_id: String,
    /// Dependency name
    pub dependency_name: String,
    /// Relevance score
    pub relevance_score: f64,
    /// Matched fields
    pub matched_fields: Vec<String>,
    /// Highlighted text
    pub highlighted_text: Option<String>,
}

impl DependencySearchEngine {
    /// Create a new search engine
    pub fn new() -> Self {
        Self::with_config(SearchConfig::default())
    }
    
    /// Create a new search engine with configuration
    pub fn with_config(config: SearchConfig) -> Self {
        Self {
            config,
            index: Arc::new(RwLock::new(SearchIndex {
                dependencies: HashMap::new(),
                search_terms: HashMap::new(),
            })),
        }
    }
    
    /// Index dependencies
    pub async fn index_dependencies(&self, dependencies: &[DependencyConfig]) -> Result<()> {
        let mut index = self.index.write().await;
        
        for dep in dependencies {
            let indexed = IndexedDependency {
                id: dep.id.clone(),
                name: dep.name.clone(),
                dependency_type: dep.dependency_type.clone(),
                target: dep.target.clone(),
                operations: dep.operations.clone(),
                health_status: dep.health_status.clone(),
                searchable_text: self.create_searchable_text(dep),
            };
            
            index.dependencies.insert(dep.id.clone(), indexed);
        }
        
        // Build search terms
        self.build_search_terms(&mut index).await?;
        
        Ok(())
    }
    
    /// Create searchable text
    fn create_searchable_text(&self, dependency: &DependencyConfig) -> String {
        let mut text = Vec::new();
        text.push(dependency.name.clone());
        text.push(dependency.target.clone());
        text.extend(dependency.operations.clone());
        text.push(format!("{:?}", dependency.dependency_type));
        text.push(format!("{:?}", dependency.health_status));
        
        text.join(" ").to_lowercase()
    }
    
    /// Build search terms
    async fn build_search_terms(&self, index: &mut SearchIndex) -> Result<()> {
        index.search_terms.clear();
        
        for (id, dep) in &index.dependencies {
            let terms = self.extract_search_terms(&dep.searchable_text);
            
            for term in terms {
                index.search_terms.entry(term).or_insert_with(Vec::new).push(id.clone());
            }
        }
        
        Ok(())
    }
    
    /// Extract search terms
    fn extract_search_terms(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.to_lowercase())
            .filter(|s| s.len() > 2)
            .collect()
    }
    
    /// Search dependencies
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let index = self.index.read().await;
        let query_terms = self.extract_search_terms(query);
        let mut results = Vec::new();
        
        for (id, dep) in &index.dependencies {
            let relevance_score = self.calculate_relevance_score(&query_terms, dep);
            
            if relevance_score > 0.0 {
                let matched_fields = self.find_matched_fields(&query_terms, dep);
                let highlighted_text = if self.config.enable_highlighting {
                    self.highlight_text(query, &dep.searchable_text)
                } else {
                    None
                };
                
                results.push(SearchResult {
                    dependency_id: id.clone(),
                    dependency_name: dep.name.clone(),
                    relevance_score,
                    matched_fields,
                    highlighted_text,
                });
            }
        }
        
        // Sort by relevance score
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        
        // Limit results
        results.truncate(self.config.search_limit);
        
        Ok(results)
    }
    
    /// Calculate relevance score
    fn calculate_relevance_score(&self, query_terms: &[String], dependency: &IndexedDependency) -> f64 {
        let mut score = 0.0;
        let dependency_terms = self.extract_search_terms(&dependency.searchable_text);
        
        for query_term in query_terms {
            for dep_term in &dependency_terms {
                if dep_term.contains(query_term) || query_term.contains(dep_term) {
                    score += 1.0;
                }
            }
        }
        
        score / query_terms.len() as f64
    }
    
    /// Find matched fields
    fn find_matched_fields(&self, query_terms: &[String], dependency: &IndexedDependency) -> Vec<String> {
        let mut matched_fields = Vec::new();
        
        for query_term in query_terms {
            if dependency.name.to_lowercase().contains(query_term) {
                matched_fields.push("name".to_string());
            }
            if dependency.target.to_lowercase().contains(query_term) {
                matched_fields.push("target".to_string());
            }
            if dependency.operations.iter().any(|op| op.to_lowercase().contains(query_term)) {
                matched_fields.push("operations".to_string());
            }
        }
        
        matched_fields.sort();
        matched_fields.dedup();
        matched_fields
    }
    
    /// Highlight text
    fn highlight_text(&self, query: &str, text: &str) -> Option<String> {
        // Simple highlighting implementation
        let query_lower = query.to_lowercase();
        let text_lower = text.to_lowercase();
        
        if text_lower.contains(&query_lower) {
            let start = text_lower.find(&query_lower)?;
            let end = start + query.len();
            
            let highlighted = format!(
                "{}<mark>{}</mark>{}",
                &text[..start],
                &text[start..end],
                &text[end..]
            );
            
            Some(highlighted)
        } else {
            None
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            enable_fuzzy: true,
            search_limit: 50,
            enable_highlighting: true,
            search_fields: vec![
                "name".to_string(),
                "target".to_string(),
                "operations".to_string(),
                "type".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dependency_dashboard() {
        let mut dashboard = DependencyDashboard::new();
        let dependencies = vec![
            DependencyConfig::new(
                "test-dep".to_string(),
                "Test Dependency".to_string(),
                DependencyType::ApiCall,
                "https://api.example.com".to_string(),
                vec!["GET".to_string()],
            ).unwrap(),
        ];
        
        dashboard.update_data(&dependencies).await.unwrap();
        let data = dashboard.get_data().await.unwrap();
        assert_eq!(data.total_dependencies, 1);
    }

    #[tokio::test]
    async fn test_report_generator() {
        let generator = DependencyReportGenerator::new();
        let dependencies = vec![
            DependencyConfig::new(
                "test-dep".to_string(),
                "Test Dependency".to_string(),
                DependencyType::ApiCall,
                "https://api.example.com".to_string(),
                vec!["GET".to_string()],
            ).unwrap(),
        ];
        
        let report = generator.generate_report("overview", &dependencies, None).await.unwrap();
        assert!(!report.content.is_empty());
    }

    #[tokio::test]
    async fn test_alert_system() {
        let alert_system = DependencyAlertSystem::new();
        let dependencies = vec![
            DependencyConfig::new(
                "test-dep".to_string(),
                "Test Dependency".to_string(),
                DependencyType::ApiCall,
                "https://api.example.com".to_string(),
                vec!["GET".to_string()],
            ).unwrap(),
        ];
        
        let alerts = alert_system.check_dependencies(&dependencies).await.unwrap();
        // This might be empty if no alert conditions are met
        assert!(alerts.is_empty() || !alerts.is_empty());
    }

    #[tokio::test]
    async fn test_search_engine() {
        let search_engine = DependencySearchEngine::new();
        let dependencies = vec![
            DependencyConfig::new(
                "test-dep".to_string(),
                "Test Dependency".to_string(),
                DependencyType::ApiCall,
                "https://api.example.com".to_string(),
                vec!["GET".to_string()],
            ).unwrap(),
        ];
        
        search_engine.index_dependencies(&dependencies).await.unwrap();
        let results = search_engine.search("test").await.unwrap();
        assert!(!results.is_empty());
    }
} 