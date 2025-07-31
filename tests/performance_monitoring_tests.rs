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

use chrono::Utc;
use rhema::{
    PerformanceConfig, PerformanceMonitor, ReportPeriod, RhemaResult, SystemPerformanceData,
    UsageData, UxData,
};
use std::sync::Arc;

#[tokio::test]
async fn test_performance_monitor_creation() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    assert!(monitor.config.system_monitoring_enabled);
    assert!(monitor.config.ux_monitoring_enabled);
    assert!(monitor.config.usage_analytics_enabled);
    assert!(monitor.config.performance_reporting_enabled);

    Ok(())
}

#[tokio::test]
async fn test_performance_monitor_start_stop() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    // Start monitoring
    monitor.start().await?;

    // Check if running
    let running = monitor.running.read().await;
    assert!(*running);
    drop(running);

    // Stop monitoring
    monitor.stop().await?;

    // Check if stopped
    let running = monitor.running.read().await;
    assert!(!*running);

    Ok(())
}

#[tokio::test]
async fn test_system_metrics_recording() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    let data = SystemPerformanceData {
        timestamp: Utc::now(),
        cpu_usage_percent: 50.0,
        memory_usage_bytes: 1024 * 1024 * 256, // 256 MB
        memory_usage_percent: 25.0,
        disk_io_ops: 100,
        disk_io_bytes: 1024 * 1024,   // 1 MB
        network_io_bytes: 1024 * 512, // 512 KB
        network_latency_ms: 10.0,
        fs_operations: 50,
        fs_latency_ms: 5.0,
        process_count: 100,
        thread_count: 200,
        open_file_descriptors: 1000,
    };

    monitor.record_system_metrics(data).await?;

    Ok(())
}

#[tokio::test]
async fn test_ux_metrics_recording() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    let data = UxData {
        timestamp: Utc::now(),
        command_name: "query".to_string(),
        execution_time_ms: 150,
        success: true,
        interaction_time_ms: 50,
        response_time_ms: 25,
        error_message: None,
        satisfaction_score: Some(9.0),
    };

    monitor.record_ux_metrics(data).await?;

    Ok(())
}

#[tokio::test]
async fn test_usage_analytics_recording() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    let data = UsageData {
        timestamp: Utc::now(),
        user_id: "user123".to_string(),
        command_name: "query".to_string(),
        feature_name: "cql".to_string(),
        session_duration_seconds: 300,
        workflow_completed: true,
        usage_pattern: "interactive".to_string(),
        user_behavior: "exploratory".to_string(),
    };

    monitor.record_usage_analytics(data).await?;

    Ok(())
}

#[tokio::test]
async fn test_performance_report_generation() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    let period = ReportPeriod {
        start: Utc::now() - chrono::Duration::hours(24),
        end: Utc::now(),
        duration_seconds: 86400,
    };

    let report = monitor.generate_performance_report(period).await?;

    assert!(!report.report_id.is_empty());
    assert_eq!(report.period.duration_seconds, 86400);
    assert_eq!(report.trends.len(), 2); // Based on mock data
    assert_eq!(report.recommendations.len(), 2); // Based on mock data

    Ok(())
}

#[tokio::test]
async fn test_threshold_checking() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    // Test system metrics above thresholds
    let high_cpu_data = SystemPerformanceData {
        timestamp: Utc::now(),
        cpu_usage_percent: 90.0, // Above 80% threshold
        memory_usage_bytes: 1024 * 1024 * 256,
        memory_usage_percent: 25.0,
        disk_io_ops: 100,
        disk_io_bytes: 1024 * 1024,
        network_io_bytes: 1024 * 512,
        network_latency_ms: 10.0,
        fs_operations: 50,
        fs_latency_ms: 5.0,
        process_count: 100,
        thread_count: 200,
        open_file_descriptors: 1000,
    };

    monitor.record_system_metrics(high_cpu_data).await?;

    // Test UX metrics above thresholds
    let slow_command_data = UxData {
        timestamp: Utc::now(),
        command_name: "slow_query".to_string(),
        execution_time_ms: 6000, // Above 5000ms threshold
        success: true,
        interaction_time_ms: 50,
        response_time_ms: 25,
        error_message: None,
        satisfaction_score: Some(5.0),
    };

    monitor.record_ux_metrics(slow_command_data).await?;

    Ok(())
}

#[tokio::test]
async fn test_performance_config_validation() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();

    // Test default values
    assert_eq!(config.metrics_interval, 60);
    assert_eq!(config.thresholds.cpu_threshold, 80.0);
    assert_eq!(config.thresholds.memory_threshold, 85.0);
    assert_eq!(config.thresholds.command_execution_threshold, 5000);
    assert_eq!(config.thresholds.response_time_threshold, 1000);
    assert_eq!(config.thresholds.error_rate_threshold, 10.0);

    // Test reporting config
    assert!(config.reporting.automated_reports);
    assert_eq!(config.reporting.report_interval, 24);
    assert!(config.reporting.dashboard.enabled);
    assert_eq!(config.reporting.dashboard.port, 8080);

    // Test storage config
    assert_eq!(config.storage.retention.retention_days, 30);
    assert!(config.storage.retention.aggregate_old_metrics);
    assert!(config.storage.retention.archive_old_metrics);

    Ok(())
}

#[tokio::test]
async fn test_performance_monitor_integration() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    // Start monitoring
    monitor.start().await?;

    // Record various metrics
    let system_data = SystemPerformanceData {
        timestamp: Utc::now(),
        cpu_usage_percent: 30.0,
        memory_usage_bytes: 1024 * 1024 * 128,
        memory_usage_percent: 15.0,
        disk_io_ops: 50,
        disk_io_bytes: 1024 * 512,
        network_io_bytes: 1024 * 256,
        network_latency_ms: 5.0,
        fs_operations: 25,
        fs_latency_ms: 2.0,
        process_count: 50,
        thread_count: 100,
        open_file_descriptors: 500,
    };

    let ux_data = UxData {
        timestamp: Utc::now(),
        command_name: "test_command".to_string(),
        execution_time_ms: 100,
        success: true,
        interaction_time_ms: 25,
        response_time_ms: 10,
        error_message: None,
        satisfaction_score: Some(8.0),
    };

    let usage_data = UsageData {
        timestamp: Utc::now(),
        user_id: "test_user".to_string(),
        command_name: "test_command".to_string(),
        feature_name: "test_feature".to_string(),
        session_duration_seconds: 180,
        workflow_completed: true,
        usage_pattern: "test_pattern".to_string(),
        user_behavior: "test_behavior".to_string(),
    };

    // Record all metrics
    monitor.record_system_metrics(system_data).await?;
    monitor.record_ux_metrics(ux_data).await?;
    monitor.record_usage_analytics(usage_data).await?;

    // Generate report
    let period = ReportPeriod {
        start: Utc::now() - chrono::Duration::hours(1),
        end: Utc::now(),
        duration_seconds: 3600,
    };

    let report = monitor.generate_performance_report(period).await?;

    // Verify report contains expected data
    assert!(!report.report_id.is_empty());
    assert_eq!(report.period.duration_seconds, 3600);
    assert!(report.system_performance.avg_cpu_usage > 0.0);
    assert!(report.ux_summary.avg_command_execution_time > 0.0);
    assert!(report.usage_summary.total_commands > 0);

    // Stop monitoring
    monitor.stop().await?;

    Ok(())
}

#[tokio::test]
async fn test_performance_monitor_error_handling() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    // Test with invalid data (should not panic)
    let invalid_data = SystemPerformanceData {
        timestamp: Utc::now(),
        cpu_usage_percent: -10.0, // Invalid negative value
        memory_usage_bytes: 0,
        memory_usage_percent: 0.0,
        disk_io_ops: 0,
        disk_io_bytes: 0,
        network_io_bytes: 0,
        network_latency_ms: -5.0, // Invalid negative value
        fs_operations: 0,
        fs_latency_ms: 0.0,
        process_count: 0,
        thread_count: 0,
        open_file_descriptors: 0,
    };

    // Should not panic even with invalid data
    monitor.record_system_metrics(invalid_data).await?;

    Ok(())
}

#[tokio::test]
async fn test_performance_monitor_concurrent_access() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = Arc::new(PerformanceMonitor::new(config)?);

    // Start monitoring
    monitor.start().await?;

    // Spawn multiple tasks to record metrics concurrently
    let mut handles = vec![];

    for i in 0..10 {
        let monitor_clone = monitor.clone();
        let handle = tokio::spawn(async move {
            let system_data = SystemPerformanceData {
                timestamp: Utc::now(),
                cpu_usage_percent: 20.0 + (i as f64),
                memory_usage_bytes: 1024 * 1024 * (64 + i as u64),
                memory_usage_percent: 10.0 + (i as f64),
                disk_io_ops: 10 + i,
                disk_io_bytes: 1024 * (256 + i as u64),
                network_io_bytes: 1024 * (128 + i as u64),
                network_latency_ms: 2.0 + (i as f64),
                fs_operations: 5 + i,
                fs_latency_ms: 1.0 + (i as f64),
                process_count: 10 + i,
                thread_count: 20 + i,
                open_file_descriptors: 100 + i,
            };

            monitor_clone.record_system_metrics(system_data).await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await??;
    }

    // Stop monitoring
    monitor.stop().await?;

    Ok(())
}

#[tokio::test]
async fn test_performance_monitor_memory_usage() -> RhemaResult<()> {
    let config = PerformanceMonitor::default_config();
    let monitor = PerformanceMonitor::new(config)?;

    // Record many metrics to test memory usage
    for i in 0..1000 {
        let data = SystemPerformanceData {
            timestamp: Utc::now(),
            cpu_usage_percent: (i % 100) as f64,
            memory_usage_bytes: 1024 * 1024 * (i % 512) as u64,
            memory_usage_percent: (i % 100) as f64,
            disk_io_ops: (i % 1000) as u64,
            disk_io_bytes: 1024 * (i % 1024) as u64,
            network_io_bytes: 1024 * (i % 512) as u64,
            network_latency_ms: (i % 100) as f64,
            fs_operations: (i % 500) as u64,
            fs_latency_ms: (i % 50) as f64,
            process_count: (i % 200) as u64,
            thread_count: (i % 400) as u64,
            open_file_descriptors: (i % 1000) as u64,
        };

        monitor.record_system_metrics(data).await?;
    }

    // Generate report to test memory usage during analysis
    let period = ReportPeriod {
        start: Utc::now() - chrono::Duration::hours(1),
        end: Utc::now(),
        duration_seconds: 3600,
    };

    let report = monitor.generate_performance_report(period).await?;

    // Verify report was generated successfully
    assert!(!report.report_id.is_empty());

    Ok(())
}
