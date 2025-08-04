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

use rhema_git::{
    git::{
        monitoring::{
            GitMonitoringManager, MonitoringConfig, default_monitoring_config,
            AdvancedMonitoringConfig, PerformanceMonitoringConfig, MetricsConfig,
            RealtimeMonitoringConfig, MonitoringAlertingConfig, DashboardConfig,
        },
        hooks::{
            HookManager, HookConfig, default_hook_config, HookType,
            HookMonitoringConfig, HookAutomationConfig, HookMLConfig,
        },
    },
};
use git2::Repository;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::Utc;

/// Example demonstrating advanced Git Monitoring & Hooks integration
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Git Monitoring & Hooks Example");
    println!("========================================\n");

    // Initialize Git repository
    let repo = Repository::open(".")?;
    println!("✅ Git repository initialized");

    // Setup enhanced monitoring configuration
    let monitoring_config = setup_monitoring_config();
    println!("✅ Monitoring configuration created");

    // Setup enhanced hooks configuration
    let hooks_config = setup_hooks_config();
    println!("✅ Hooks configuration created");

    // Initialize monitoring manager
    let monitoring_manager = Arc::new(Mutex::new(
        GitMonitoringManager::new(repo.clone(), monitoring_config)?
    ));
    println!("✅ Monitoring manager initialized");

    // Initialize hooks manager with monitoring integration
    let hooks_manager = HookManager::new(
        repo,
        hooks_config,
        Some(monitoring_manager.clone()),
    );
    println!("✅ Hooks manager initialized with monitoring integration");

    // Start monitoring
    monitoring_manager.lock().unwrap().start_monitoring()?;
    println!("✅ Git monitoring started");

    // Install hooks
    hooks_manager.install_hooks()?;
    println!("✅ Git hooks installed");

    // Demonstrate monitoring features
    demonstrate_monitoring_features(&monitoring_manager).await?;

    // Demonstrate hooks features
    demonstrate_hooks_features(&hooks_manager).await?;

    // Demonstrate advanced features
    demonstrate_advanced_features(&monitoring_manager, &hooks_manager).await?;

    // Demonstrate real-world scenarios
    demonstrate_real_world_scenarios(&monitoring_manager, &hooks_manager).await?;

    // Stop monitoring
    monitoring_manager.lock().unwrap().stop_monitoring()?;
    println!("✅ Git monitoring stopped");

    println!("\n🎉 Example completed successfully!");
    Ok(())
}

/// Setup enhanced monitoring configuration with advanced features
fn setup_monitoring_config() -> MonitoringConfig {
    let mut config = default_monitoring_config();
    
    // Enable advanced monitoring features
    config.advanced = AdvancedMonitoringConfig {
        distributed_tracing: true,
        anomaly_detection: true,
        predictive_analytics: true,
        ml_insights: true,
        correlation_analysis: true,
        performance_profiling: true,
        resource_monitoring: true,
        security_monitoring: true,
    };

    // Configure performance monitoring
    config.performance = PerformanceMonitoringConfig {
        enabled: true,
        monitor_git_operations: true,
        monitor_context_operations: true,
        monitor_hook_execution: true,
        thresholds: config.performance.thresholds,
        sampling_rate: 0.1, // 10% sampling
    };

    // Configure metrics collection
    config.metrics = MetricsConfig {
        enabled: true,
        storage: config.metrics.storage,
        intervals: config.metrics.intervals,
        metrics: vec![
            config.metrics.metrics[0].clone(), // GitOperations
            config.metrics.metrics[1].clone(), // ContextOperations
            config.metrics.metrics[2].clone(), // PerformanceMetrics
            config.metrics.metrics[3].clone(), // SystemMetrics
        ],
        retention: config.metrics.retention,
    };

    // Configure real-time monitoring
    config.realtime = RealtimeMonitoringConfig {
        enabled: true,
        websocket: config.realtime.websocket,
        event_streaming: config.realtime.event_streaming,
        live_dashboards: true,
        realtime_alerts: true,
    };

    // Configure alerting
    config.alerting = MonitoringAlertingConfig {
        enabled: true,
        channels: config.alerting.channels,
        rules: config.alerting.rules,
        severity_levels: config.alerting.severity_levels,
    };

    // Configure dashboard
    config.dashboard = DashboardConfig {
        enabled: true,
        server: config.dashboard.server,
        widgets: config.dashboard.widgets,
        auto_refresh: 30,
    };

    config
}

/// Setup enhanced hooks configuration with advanced features
fn setup_hooks_config() -> HookConfig {
    let mut config = default_hook_config();
    
    // Enable monitoring integration
    config.monitoring = HookMonitoringConfig {
        enabled: true,
        performance_tracking: true,
        execution_analytics: true,
        predictive_analysis: true,
        anomaly_detection: true,
        health_monitoring: true,
        alert_integration: true,
        dashboard_integration: true,
    };

    // Enable automation features
    config.automation = HookAutomationConfig {
        enabled: true,
        auto_fix: true,
        smart_validation: true,
        context_learning: true,
        pattern_recognition: true,
        adaptive_rules: true,
        self_optimization: true,
        predictive_actions: true,
    };

    // Enable ML features
    config.ml_features = HookMLConfig {
        enabled: true,
        code_quality_prediction: true,
        security_risk_assessment: true,
        performance_impact_prediction: true,
        conflict_prediction: true,
        optimization_suggestions: true,
        anomaly_detection: true,
        pattern_learning: true,
    };

    config
}

/// Demonstrate monitoring features
async fn demonstrate_monitoring_features(
    monitoring_manager: &Arc<Mutex<GitMonitoringManager>>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Demonstrating Monitoring Features");
    println!("=====================================");

    let manager = monitoring_manager.lock().unwrap();

    // Record sample Git operations
    println!("Recording sample Git operations...");
    manager.record_git_operation("commit", Duration::from_millis(150))?;
    manager.record_git_operation("push", Duration::from_millis(2500))?;
    manager.record_git_operation("pull", Duration::from_millis(800))?;
    manager.record_git_operation("merge", Duration::from_millis(1200))?;

    // Record sample context operations
    println!("Recording sample context operations...");
    manager.record_context_operation("context_validation", Duration::from_millis(50))?;
    manager.record_context_operation("context_update", Duration::from_millis(100))?;
    manager.record_context_operation("context_sync", Duration::from_millis(200))?;

    // Get monitoring status
    let status = manager.get_status()?;
    println!("📈 Monitoring Status:");
    println!("  - Active: {}", status.is_active);
    println!("  - Metrics Enabled: {}", status.metrics_enabled);
    println!("  - Performance Enabled: {}", status.performance_enabled);
    println!("  - Realtime Enabled: {}", status.realtime_enabled);
    println!("  - Metrics Count: {}", status.metrics_count);
    println!("  - Operations Count: {}", status.operations_count);
    println!("  - Events Count: {}", status.events_count);

    // Get performance metrics
    let performance_metrics = manager.get_performance_metrics()?;
    println!("⚡ Performance Metrics:");
    for metric in performance_metrics {
        println!("  - {}: {} operations, avg: {}ms", 
            metric.operation_name,
            metric.count,
            metric.average_duration.num_milliseconds()
        );
    }

    // Get recent events
    let recent_events = manager.get_recent_events(Some(5))?;
    println!("📋 Recent Events:");
    for event in recent_events {
        println!("  - {}: {:?} at {}", 
            event.event_type,
            event.severity,
            event.timestamp.format("%H:%M:%S")
        );
    }

    Ok(())
}

/// Demonstrate hooks features
async fn demonstrate_hooks_features(
    hooks_manager: &HookManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔗 Demonstrating Hooks Features");
    println!("===============================");

    // Check hook status
    let hook_status = hooks_manager.get_hook_status()?;
    println!("📋 Hook Status:");
    for (hook_type, installed) in hook_status {
        println!("  - {}: {}", hook_type.filename(), if installed { "✅" } else { "❌" });
    }

    // Execute sample hooks
    println!("\n🔄 Executing sample hooks...");
    
    // Pre-commit hook
    println!("Executing pre-commit hook...");
    let pre_commit_result = hooks_manager.execute_hook(HookType::PreCommit)?;
    print_hook_result("Pre-Commit", &pre_commit_result);

    // Post-commit hook
    println!("Executing post-commit hook...");
    let post_commit_result = hooks_manager.execute_hook(HookType::PostCommit)?;
    print_hook_result("Post-Commit", &post_commit_result);

    // Pre-push hook
    println!("Executing pre-push hook...");
    let pre_push_result = hooks_manager.execute_hook(HookType::PrePush)?;
    print_hook_result("Pre-Push", &pre_push_result);

    Ok(())
}

/// Demonstrate advanced features
async fn demonstrate_advanced_features(
    monitoring_manager: &Arc<Mutex<GitMonitoringManager>>,
    hooks_manager: &HookManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 Demonstrating Advanced Features");
    println!("===================================");

    let manager = monitoring_manager.lock().unwrap();

    // Demonstrate anomaly detection
    println!("🔍 Anomaly Detection:");
    let anomalies = manager.advanced_monitor.lock().unwrap()
        .detect_anomalies("git_operation_duration", 5000.0)?;
    
    for anomaly in anomalies {
        println!("  - Anomaly detected: {} (z-score: {:.2})", 
            anomaly.description, anomaly.z_score);
    }

    // Demonstrate predictive analytics
    println!("🔮 Predictive Analytics:");
    let predictions = manager.advanced_monitor.lock().unwrap()
        .predict_metrics("git_operation_duration", 5)?;
    
    for prediction in predictions {
        println!("  - Prediction: {:.2} (confidence: {:.2}) at {}", 
            prediction.value,
            prediction.confidence,
            prediction.timestamp.format("%H:%M:%S")
        );
    }

    // Demonstrate correlation analysis
    println!("📊 Correlation Analysis:");
    let correlations = manager.advanced_monitor.lock().unwrap()
        .analyze_correlations(&[
            ("git_operations", 100.0),
            ("context_operations", 50.0),
            ("performance_metrics", 75.0),
        ])?;
    
    for correlation in correlations {
        println!("  - {} ↔ {}: {:.2} ({:?})", 
            correlation.metric1,
            correlation.metric2,
            correlation.coefficient,
            correlation.strength
        );
    }

    // Demonstrate performance profiling
    println!("⚡ Performance Profiling:");
    let profile = manager.advanced_monitor.lock().unwrap()
        .profile_operation("complex_git_operation", Duration::from_millis(1500))?;
    
    println!("  - Operation: {}", profile.operation_name);
    println!("  - Hotspots: {}", profile.hotspots.len());
    println!("  - Recommendations: {}", profile.recommendations.len());

    // Demonstrate resource monitoring
    println!("💾 Resource Monitoring:");
    let resources = manager.advanced_monitor.lock().unwrap()
        .monitor_resources()?;
    
    println!("  - CPU Usage: {:.1}%", resources.cpu.usage_percentage);
    println!("  - Memory Usage: {:.1}%", 
        (resources.memory.used as f64 / resources.memory.total as f64) * 100.0);
    println!("  - Disk Usage: {:.1}%", 
        (resources.disk.used as f64 / resources.disk.total as f64) * 100.0);
    println!("  - Alerts: {}", resources.alerts.len());

    Ok(())
}

/// Demonstrate real-world scenarios
async fn demonstrate_real_world_scenarios(
    monitoring_manager: &Arc<Mutex<GitMonitoringManager>>,
    hooks_manager: &HookManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌍 Demonstrating Real-World Scenarios");
    println!("======================================");

    // Scenario 1: High-traffic development team
    println!("👥 Scenario 1: High-Traffic Development Team");
    simulate_high_traffic_scenario(monitoring_manager, hooks_manager).await?;

    // Scenario 2: Security-focused workflow
    println!("🔒 Scenario 2: Security-Focused Workflow");
    simulate_security_scenario(monitoring_manager, hooks_manager).await?;

    // Scenario 3: Performance optimization
    println!("⚡ Scenario 3: Performance Optimization");
    simulate_performance_scenario(monitoring_manager, hooks_manager).await?;

    // Scenario 4: Anomaly detection and response
    println!("🚨 Scenario 4: Anomaly Detection and Response");
    simulate_anomaly_scenario(monitoring_manager, hooks_manager).await?;

    Ok(())
}

/// Simulate high-traffic development team scenario
async fn simulate_high_traffic_scenario(
    monitoring_manager: &Arc<Mutex<GitMonitoringManager>>,
    hooks_manager: &HookManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Simulating high-traffic development team...");

    let manager = monitoring_manager.lock().unwrap();

    // Simulate multiple developers making commits
    for i in 1..=10 {
        let developer = format!("developer_{}", i);
        println!("    {} making commit...", developer);

        // Record Git operations
        manager.record_git_operation("commit", Duration::from_millis(100 + i * 10))?;
        manager.record_git_operation("push", Duration::from_millis(500 + i * 50))?;

        // Record context operations
        manager.record_context_operation("context_validation", Duration::from_millis(20 + i * 2))?;
        manager.record_context_operation("context_update", Duration::from_millis(30 + i * 3))?;

        // Execute hooks
        hooks_manager.execute_hook(HookType::PreCommit)?;
        hooks_manager.execute_hook(HookType::PostCommit)?;
    }

    // Check for performance bottlenecks
    let performance_metrics = manager.get_performance_metrics()?;
    let slow_operations: Vec<_> = performance_metrics
        .iter()
        .filter(|m| m.average_duration.num_milliseconds() > 500)
        .collect();

    if !slow_operations.is_empty() {
        println!("    ⚠️  Detected slow operations:");
        for op in slow_operations {
            println!("      - {}: {}ms average", 
                op.operation_name, 
                op.average_duration.num_milliseconds()
            );
        }
    }

    Ok(())
}

/// Simulate security-focused workflow scenario
async fn simulate_security_scenario(
    monitoring_manager: &Arc<Mutex<GitMonitoringManager>>,
    hooks_manager: &HookManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Simulating security-focused workflow...");

    let manager = monitoring_manager.lock().unwrap();

    // Simulate security-sensitive operations
    println!("    Running security checks...");
    
    // Record security-related operations
    manager.record_git_operation("security_scan", Duration::from_millis(2000))?;
    manager.record_git_operation("vulnerability_check", Duration::from_millis(1500))?;
    manager.record_git_operation("compliance_validation", Duration::from_millis(800))?;

    // Execute security-focused hooks
    let pre_push_result = hooks_manager.execute_hook(HookType::PrePush)?;
    if !pre_push_result.success {
        println!("    🚨 Security validation failed!");
        for error in &pre_push_result.errors {
            println!("      - Error: {}", error);
        }
    } else {
        println!("    ✅ Security validation passed");
    }

    // Check for security events
    let recent_events = manager.get_recent_events(Some(10))?;
    let security_events: Vec<_> = recent_events
        .iter()
        .filter(|e| matches!(e.event_type, rhema_git::git::monitoring::EventType::SecurityEvent))
        .collect();

    if !security_events.is_empty() {
        println!("    🔍 Security events detected:");
        for event in security_events {
            println!("      - {}: {:?}", 
                event.timestamp.format("%H:%M:%S"),
                event.severity
            );
        }
    }

    Ok(())
}

/// Simulate performance optimization scenario
async fn simulate_performance_scenario(
    monitoring_manager: &Arc<Mutex<GitMonitoringManager>>,
    hooks_manager: &HookManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Simulating performance optimization...");

    let manager = monitoring_manager.lock().unwrap();

    // Simulate performance profiling
    println!("    Running performance profiling...");
    
    // Record operations with varying performance characteristics
    manager.record_git_operation("fast_operation", Duration::from_millis(50))?;
    manager.record_git_operation("medium_operation", Duration::from_millis(200))?;
    manager.record_git_operation("slow_operation", Duration::from_millis(1000))?;
    manager.record_git_operation("very_slow_operation", Duration::from_millis(3000))?;

    // Get performance insights
    let performance_metrics = manager.get_performance_metrics()?;
    let optimization_candidates: Vec<_> = performance_metrics
        .iter()
        .filter(|m| m.average_duration.num_milliseconds() > 500)
        .collect();

    println!("    🎯 Optimization candidates:");
    for candidate in optimization_candidates {
        println!("      - {}: {}ms average ({} slow operations)", 
            candidate.operation_name,
            candidate.average_duration.num_milliseconds(),
            candidate.slow_operations
        );
    }

    // Execute hooks with performance monitoring
    let pre_commit_result = hooks_manager.execute_hook(HookType::PreCommit)?;
    println!("    ⏱️  Pre-commit hook execution time: {}ms", 
        pre_commit_result.execution_time.as_millis());

    Ok(())
}

/// Simulate anomaly detection and response scenario
async fn simulate_anomaly_scenario(
    monitoring_manager: &Arc<Mutex<GitMonitoringManager>>,
    hooks_manager: &HookManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Simulating anomaly detection and response...");

    let manager = monitoring_manager.lock().unwrap();

    // Simulate normal operations first
    println!("    Recording normal operations...");
    for _ in 0..10 {
        manager.record_git_operation("normal_commit", Duration::from_millis(100))?;
    }

    // Simulate anomalous operation
    println!("    Recording anomalous operation...");
    manager.record_git_operation("anomalous_operation", Duration::from_millis(10000))?;

    // Check for anomalies
    let anomalies = manager.advanced_monitor.lock().unwrap()
        .detect_anomalies("git_operation_duration", 10000.0)?;

    if !anomalies.is_empty() {
        println!("    🚨 Anomalies detected:");
        for anomaly in anomalies {
            println!("      - {} (severity: {:?})", 
                anomaly.description, 
                anomaly.severity
            );
        }

        // Simulate automated response
        println!("    🤖 Triggering automated response...");
        let response_result = hooks_manager.execute_hook(HookType::PreCommit)?;
        if response_result.success {
            println!("      ✅ Automated response successful");
        } else {
            println!("      ❌ Automated response failed");
        }
    }

    Ok(())
}

/// Print hook execution result
fn print_hook_result(hook_name: &str, result: &rhema_git::git::hooks::HookResult) {
    println!("  📋 {} Hook Result:", hook_name);
    println!("    - Success: {}", if result.success { "✅" } else { "❌" });
    println!("    - Execution Time: {}ms", result.execution_time.as_millis());
    println!("    - Messages: {}", result.messages.len());
    println!("    - Errors: {}", result.errors.len());
    println!("    - Warnings: {}", result.warnings.len());
    
    if !result.messages.is_empty() {
        println!("    - Messages:");
        for msg in &result.messages {
            println!("      • {}", msg);
        }
    }
    
    if !result.errors.is_empty() {
        println!("    - Errors:");
        for error in &result.errors {
            println!("      • {}", error);
        }
    }
    
    if !result.warnings.is_empty() {
        println!("    - Warnings:");
        for warning in &result.warnings {
            println!("      • {}", warning);
        }
    }
} 