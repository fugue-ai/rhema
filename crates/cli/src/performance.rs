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

use crate::{Rhema, RhemaResult};
use crate::{PerformanceMonitor, PerformanceConfig, UxData, UsageData, ReportPeriod, PerformanceReport};
use rhema_monitoring::{TrendDirection, PriorityLevel};
use colored::*;
use chrono::{Utc, Duration};
use std::sync::Arc;
// use tokio::sync::RwLock;

/// Performance monitoring commands
pub struct PerformanceCommands {
    monitor: Arc<PerformanceMonitor>,
}

impl PerformanceCommands {
    /// Create new performance commands
    pub fn new(config: Option<PerformanceConfig>) -> RhemaResult<Self> {
        let config = config.unwrap_or_else(PerformanceMonitor::default_config);
        let monitor = Arc::new(PerformanceMonitor::new(config)?);
        Ok(Self { monitor })
    }

    /// Start performance monitoring
    pub async fn start(&self) -> RhemaResult<()> {
        println!("🚀 Starting comprehensive performance monitoring...");
        
        self.monitor.start().await?;
        
        println!("✅ Performance monitoring started successfully");
        println!("📊 Monitoring components:");
        println!("   • System performance monitoring: {}", 
            if self.monitor.config.system_monitoring_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        println!("   • User experience monitoring: {}", 
            if self.monitor.config.ux_monitoring_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        println!("   • Usage analytics: {}", 
            if self.monitor.config.usage_analytics_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        println!("   • Performance reporting: {}", 
            if self.monitor.config.performance_reporting_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        
        Ok(())
    }

    /// Stop performance monitoring
    pub async fn stop(&self) -> RhemaResult<()> {
        println!("🛑 Stopping performance monitoring...");
        
        self.monitor.stop().await?;
        
        println!("✅ Performance monitoring stopped successfully");
        Ok(())
    }

    /// Show current system performance
    pub async fn system_status(&self) -> RhemaResult<()> {
        println!("💻 System Performance Status");
        println!("{}", "─".repeat(80));
        
        // Collect current system metrics
        let data = PerformanceMonitor::collect_system_metrics().await?;
        
        println!("📊 CPU Usage: {:.1}%", data.cpu_usage_percent);
        println!("🧠 Memory Usage: {:.1}% ({:.1} MB)", 
            data.memory_usage_percent, 
            data.memory_usage_bytes as f64 / 1024.0 / 1024.0);
        println!("💾 Disk I/O: {} ops/s, {:.1} MB/s", 
            data.disk_io_ops, 
            data.disk_io_bytes as f64 / 1024.0 / 1024.0);
        println!("🌐 Network I/O: {:.1} KB/s", 
            data.network_io_bytes as f64 / 1024.0);
        println!("⏱️  Network Latency: {:.1} ms", data.network_latency_ms);
        println!("📁 File System: {} ops/s, {:.1} ms avg", 
            data.fs_operations, data.fs_latency_ms);
        println!("🔄 Processes: {}, Threads: {}", data.process_count, data.thread_count);
        println!("📂 Open Files: {}", data.open_file_descriptors);
        
        // Check thresholds
        let thresholds = &self.monitor.config.thresholds;
        println!("\n⚠️  Threshold Alerts:");
        
        if data.cpu_usage_percent > thresholds.cpu_threshold {
            println!("   • CPU usage exceeds threshold: {:.1}% > {:.1}%", 
                data.cpu_usage_percent, thresholds.cpu_threshold);
        }
        
        if data.memory_usage_percent > thresholds.memory_threshold {
            println!("   • Memory usage exceeds threshold: {:.1}% > {:.1}%", 
                data.memory_usage_percent, thresholds.memory_threshold);
        }
        
        if data.network_latency_ms > thresholds.network_latency_threshold {
            println!("   • Network latency exceeds threshold: {:.1}ms > {:.1}ms", 
                data.network_latency_ms, thresholds.network_latency_threshold);
        }
        
        Ok(())
    }

    /// Record user experience metrics
    pub async fn record_ux(&self, command_name: &str, execution_time_ms: u64, success: bool, 
                          interaction_time_ms: Option<u64>, response_time_ms: Option<u64>, 
                          error_message: Option<&str>, satisfaction_score: Option<f64>) -> RhemaResult<()> {
        let data = UxData {
            timestamp: Utc::now(),
            command_name: command_name.to_string(),
            execution_time_ms,
            success,
            interaction_time_ms: interaction_time_ms.unwrap_or(0),
            response_time_ms: response_time_ms.unwrap_or(0),
            error_message: error_message.map(|s| s.to_string()),
            satisfaction_score,
        };
        
        self.monitor.record_ux_metrics(data).await?;
        
        if !success {
            println!("⚠️  UX metric recorded for failed command: {}", command_name);
        }
        
        Ok(())
    }

    /// Record usage analytics
    pub async fn record_usage(&self, user_id: &str, command_name: &str, feature_name: &str,
                             session_duration_seconds: u64, workflow_completed: bool,
                             usage_pattern: &str, user_behavior: &str) -> RhemaResult<()> {
        let data = UsageData {
            timestamp: Utc::now(),
            user_id: user_id.to_string(),
            command_name: command_name.to_string(),
            feature_name: feature_name.to_string(),
            session_duration_seconds,
            workflow_completed,
            usage_pattern: usage_pattern.to_string(),
            user_behavior: user_behavior.to_string(),
        };
        
        self.monitor.record_usage_analytics(data).await?;
        Ok(())
    }

    /// Generate performance report
    pub async fn generate_report(&self, hours: Option<u64>) -> RhemaResult<()> {
        let hours = hours.unwrap_or(24);
        println!("📊 Generating performance report for the last {} hours...", hours);
        
        let period = ReportPeriod {
            start: Utc::now() - Duration::hours(hours as i64),
            end: Utc::now(),
            duration_seconds: hours * 3600,
        };
        
        let report = self.monitor.generate_performance_report(period).await?;
        
        self.display_report(&report)?;
        
        Ok(())
    }

    /// Display performance report
    fn display_report(&self, report: &PerformanceReport) -> RhemaResult<()> {
        println!("\n📈 Performance Report");
        println!("{}", "═".repeat(80));
        println!("Report ID: {}", report.report_id);
        println!("Generated: {}", report.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Period: {} to {}", 
            report.period.start.format("%Y-%m-%d %H:%M:%S UTC"),
            report.period.end.format("%Y-%m-%d %H:%M:%S UTC"));
        
        // System Performance Summary
        println!("\n💻 System Performance Summary");
        println!("{}", "─".repeat(50));
        println!("CPU Usage: {:.1}% avg, {:.1}% peak", 
            report.system_performance.avg_cpu_usage,
            report.system_performance.peak_cpu_usage);
        println!("Memory Usage: {:.1}% avg, {:.1}% peak", 
            report.system_performance.avg_memory_usage,
            report.system_performance.peak_memory_usage);
        println!("Network Latency: {:.1} ms avg", report.system_performance.avg_network_latency);
        println!("Total Disk I/O: {:.1} MB", report.system_performance.total_disk_io as f64 / 1024.0 / 1024.0);
        println!("Total Network I/O: {:.1} MB", report.system_performance.total_network_io as f64 / 1024.0 / 1024.0);
        
        if !report.system_performance.bottlenecks.is_empty() {
            println!("🚨 Performance Bottlenecks:");
            for bottleneck in &report.system_performance.bottlenecks {
                println!("   • {}", bottleneck);
            }
        }
        
        // User Experience Summary
        println!("\n👤 User Experience Summary");
        println!("{}", "─".repeat(50));
        println!("Command Execution Time: {:.1} ms avg", report.ux_summary.avg_command_execution_time);
        println!("Command Success Rate: {:.1}%", report.ux_summary.command_success_rate);
        println!("Response Time: {:.1} ms avg", report.ux_summary.avg_response_time);
        println!("User Satisfaction: {:.1}/10 avg", report.ux_summary.avg_satisfaction_score);
        println!("Error Rate: {:.1}%", report.ux_summary.error_rate);
        
        if !report.ux_summary.common_errors.is_empty() {
            println!("🚨 Common Errors:");
            for error in &report.ux_summary.common_errors {
                println!("   • {}", error);
            }
        }
        
        if !report.ux_summary.improvements_needed.is_empty() {
            println!("🔧 UX Improvements Needed:");
            for improvement in &report.ux_summary.improvements_needed {
                println!("   • {}", improvement);
            }
        }
        
        // Usage Analytics Summary
        println!("\n📊 Usage Analytics Summary");
        println!("{}", "─".repeat(50));
        println!("Total Commands: {}", report.usage_summary.total_commands);
        println!("Feature Adoption Rate: {:.1}%", report.usage_summary.feature_adoption_rate);
        println!("Session Duration: {:.1} seconds avg", report.usage_summary.avg_session_duration);
        println!("Workflow Completion Rate: {:.1}%", report.usage_summary.workflow_completion_rate);
        
        if !report.usage_summary.most_used_commands.is_empty() {
            println!("🔥 Most Used Commands:");
            for (i, command) in report.usage_summary.most_used_commands.iter().enumerate().take(5) {
                println!("   {}. {}", i + 1, command);
            }
        }
        
        if !report.usage_summary.behavior_patterns.is_empty() {
            println!("🎯 User Behavior Patterns:");
            for pattern in &report.usage_summary.behavior_patterns {
                println!("   • {}", pattern);
            }
        }
        
        // Performance Trends
        println!("\n📈 Performance Trends");
        println!("{}", "─".repeat(50));
        for trend in &report.trends {
            let direction_emoji = match trend.direction {
                crate::performance::TrendDirection::Improving => "📈",
                crate::performance::TrendDirection::Declining => "📉",
                crate::performance::TrendDirection::Stable => "➡️",
                crate::performance::TrendDirection::Fluctuating => "📊",
            };
            
            println!("{} {}: {:.1}% change (confidence: {:.1}%)", 
                direction_emoji, trend.metric_name, trend.change_percentage, trend.confidence_level * 100.0);
            println!("   {}", trend.description);
        }
        
        // Optimization Recommendations
        println!("\n🔧 Optimization Recommendations");
        println!("{}", "─".repeat(50));
        for recommendation in &report.recommendations {
            let priority_emoji = match recommendation.priority {
                crate::performance::PriorityLevel::Critical => "🚨",
                crate::performance::PriorityLevel::High => "🔴",
                crate::performance::PriorityLevel::Medium => "🟡",
                crate::performance::PriorityLevel::Low => "🟢",
            };
            
            println!("{} {} (Priority: {:?})", priority_emoji, recommendation.title, recommendation.priority);
            println!("   {}", recommendation.description);
            println!("   Expected Impact: {}", recommendation.expected_impact);
            println!("   Implementation Effort: {}", recommendation.implementation_effort);
            println!();
        }
        
        // Impact Assessment
        println!("\n🎯 Performance Impact Assessment");
        println!("{}", "─".repeat(50));
        println!("Overall Performance Score: {:.1}/10", report.impact_assessment.overall_score);
        println!("Risk Assessment: {}", report.impact_assessment.risk_assessment);
        
        if !report.impact_assessment.improvements.is_empty() {
            println!("✅ Improvements:");
            for improvement in &report.impact_assessment.improvements {
                println!("   • {}", improvement);
            }
        }
        
        if !report.impact_assessment.degradations.is_empty() {
            println!("⚠️  Degradations:");
            for degradation in &report.impact_assessment.degradations {
                println!("   • {}", degradation);
            }
        }
        
        if !report.impact_assessment.action_items.is_empty() {
            println!("📋 Action Items:");
            for action in &report.impact_assessment.action_items {
                println!("   • {}", action);
            }
        }
        
        Ok(())
    }

    /// Show performance configuration
    pub fn show_config(&self) -> RhemaResult<()> {
        println!("⚙️  Performance Monitoring Configuration");
        println!("{}", "─".repeat(80));
        
        let config = &self.monitor.config;
        
        println!("📊 Monitoring Components:");
        println!("   • System monitoring: {}", 
            if config.system_monitoring_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        println!("   • UX monitoring: {}", 
            if config.ux_monitoring_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        println!("   • Usage analytics: {}", 
            if config.usage_analytics_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        println!("   • Performance reporting: {}", 
            if config.performance_reporting_enabled { "✅ Enabled".green() } else { "❌ Disabled".red() });
        
        println!("\n⏱️  Collection Intervals:");
        println!("   • Metrics collection: {} seconds", config.metrics_interval);
        println!("   • Report generation: {} hours", config.reporting.report_interval);
        println!("   • Dashboard refresh: {} seconds", config.reporting.dashboard.auto_refresh);
        
        println!("\n🚨 Performance Thresholds:");
        let thresholds = &config.thresholds;
        println!("   • CPU usage: {:.1}%", thresholds.cpu_threshold);
        println!("   • Memory usage: {:.1}%", thresholds.memory_threshold);
        println!("   • Disk I/O: {:.1} MB/s", thresholds.disk_io_threshold);
        println!("   • Network latency: {:.1} ms", thresholds.network_latency_threshold);
        println!("   • Command execution: {} ms", thresholds.command_execution_threshold);
        println!("   • Response time: {} ms", thresholds.response_time_threshold);
        println!("   • Error rate: {:.1}%", thresholds.error_rate_threshold);
        
        println!("\n📁 Storage Configuration:");
        println!("   • Storage type: {:?}", config.storage.storage_type);
        if let Some(path) = &config.storage.storage_path {
            println!("   • Storage path: {}", path.display());
        }
        println!("   • Retention: {} days", config.storage.retention.retention_days);
        println!("   • Archive old metrics: {}", 
            if config.storage.retention.archive_old_metrics { "Yes".green() } else { "No".red() });
        
        println!("\n📊 Reporting Configuration:");
        println!("   • Automated reports: {}", 
            if config.reporting.automated_reports { "Yes".green() } else { "No".red() });
        println!("   • Report formats: {:?}", config.reporting.formats);
        println!("   • Dashboard enabled: {}", 
            if config.reporting.dashboard.enabled { "Yes".green() } else { "No".red() });
        if config.reporting.dashboard.enabled {
            println!("   • Dashboard URL: http://{}:{}", 
                config.reporting.dashboard.host, config.reporting.dashboard.port);
        }
        
        Ok(())
    }

    /// Get monitor reference
    pub fn monitor(&self) -> Arc<PerformanceMonitor> {
        self.monitor.clone()
    }
}

/// Run performance monitoring command
pub async fn run_performance_command(_rhema: &Rhema, subcommand: &PerformanceSubcommands) -> RhemaResult<()> {
    let commands = PerformanceCommands::new(None)?;
    
    match subcommand {
        PerformanceSubcommands::Start => {
            commands.start().await?;
        }
        PerformanceSubcommands::Stop => {
            commands.stop().await?;
        }
        PerformanceSubcommands::Status => {
            commands.system_status().await?;
        }
        PerformanceSubcommands::Report { hours } => {
            commands.generate_report(*hours).await?;
        }
        PerformanceSubcommands::Config => {
            commands.show_config()?;
        }
    }
    
    Ok(())
}

/// Performance monitoring subcommands
#[derive(clap::Subcommand)]
pub enum PerformanceSubcommands {
    /// Start performance monitoring
    Start,
    
    /// Stop performance monitoring
    Stop,
    
    /// Show current system performance status
    Status,
    
    /// Generate performance report
    Report {
        /// Hours to include in report (default: 24)
        #[arg(long, value_name = "HOURS")]
        hours: Option<u64>,
    },
    
    /// Show performance monitoring configuration
    Config,
} 