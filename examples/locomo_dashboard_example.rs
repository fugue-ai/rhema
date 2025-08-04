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

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error};

use rhema_monitoring::{
    PerformanceMonitor, LocomoPerformanceIntegration, LocomoIntegrationConfig,
    LocomoDashboardServer, DashboardServerConfig
};
use rhema_locomo::{
    LocomoMetricsCollector, LocomoBenchmarkEngine, LocomoReportingSystem
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting LOCOMO Dashboard Example");

    // Create performance monitor
    let performance_monitor = Arc::new(PerformanceMonitor::new_default()?);
    info!("✅ Performance monitor initialized");

    // Create LOCOMO components
    let metrics_collector = Arc::new(LocomoMetricsCollector);
    let benchmark_engine = Arc::new(LocomoBenchmarkEngine);
    let reporting_system = Arc::new(LocomoReportingSystem);
    info!("✅ LOCOMO components initialized");

    // Configure LOCOMO integration
    let integration_config = LocomoIntegrationConfig {
        enable_locomo_monitoring: true,
        locomo_metrics_interval_seconds: 10,
        locomo_benchmark_interval_hours: 1,
        locomo_reporting_interval_hours: 1,
        performance_thresholds: Default::default(),
        alert_configuration: Default::default(),
        dashboard_config: Default::default(),
    };

    // Create LOCOMO performance integration
    let integration = Arc::new(LocomoPerformanceIntegration::new(
        performance_monitor,
        metrics_collector,
        benchmark_engine,
        reporting_system,
        integration_config,
    ));
    info!("✅ LOCOMO performance integration created");

    // Start the integration
    integration.start_integration().await?;
    info!("✅ LOCOMO integration started");

    // Configure dashboard server
    let dashboard_config = DashboardServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        enable_cors: true,
        cors_origins: vec!["http://localhost:3000".to_string()],
        enable_websockets: true,
        refresh_interval_ms: 5000,
        max_connections: 100,
    };

    // Create dashboard server
    let dashboard_server = LocomoDashboardServer::new(integration.clone(), dashboard_config);
    info!("✅ Dashboard server created");

    // Start dashboard server in background
    let dashboard_handle = tokio::spawn(async move {
        if let Err(e) = dashboard_server.start().await {
            error!("Dashboard server error: {}", e);
        }
    });

    info!("🌐 Dashboard server starting on http://127.0.0.1:8080");
    info!("📊 Open your browser to view the LOCOMO Performance Dashboard");

    // Simulate some activity
    simulate_activity(integration.clone()).await;

    // Keep the main thread alive
    loop {
        sleep(Duration::from_secs(30)).await;
        
        // Generate a report every 30 seconds
        match integration.generate_integrated_report(1).await {
            Ok(report) => {
                info!("📈 Generated integrated report: {:?}", report.report_type);
            }
            Err(e) => {
                warn!("Failed to generate report: {}", e);
            }
        }

        // Run benchmarks every 2 minutes
        static mut BENCHMARK_COUNTER: u32 = 0;
        unsafe {
            BENCHMARK_COUNTER += 1;
            if BENCHMARK_COUNTER % 4 == 0 {
                match integration.run_integrated_benchmarks().await {
                    Ok(benchmark_report) => {
                        info!("🏃 Ran integrated benchmarks: {:?}", benchmark_report.report_type);
                    }
                    Err(e) => {
                        warn!("Failed to run benchmarks: {}", e);
                    }
                }
            }
        }
    }

    // This will never be reached in this example, but in a real application
    // you would handle graceful shutdown
    #[allow(unreachable_code)]
    {
        dashboard_handle.abort();
        integration.stop_integration().await?;
        info!("🛑 LOCOMO integration stopped");
    }
}

/// Simulate some activity to generate metrics and alerts
async fn simulate_activity(integration: Arc<LocomoPerformanceIntegration>) {
    tokio::spawn(async move {
        let mut counter = 0;
        loop {
            sleep(Duration::from_secs(15)).await;
            counter += 1;

            // Collect integrated metrics
            match integration.collect_integrated_metrics().await {
                Ok(metrics) => {
                    info!("📊 Collected integrated metrics (iteration {})", counter);
                    info!("   Performance correlation: {:.2}", 
                          metrics.performance_correlation.overall_correlation_score);
                    info!("   Quality impact: {:.2}", 
                          metrics.quality_impact.overall_quality_impact);
                    info!("   Optimization effectiveness: {:.2}", 
                          metrics.optimization_effectiveness.overall_effectiveness_score);
                }
                Err(e) => {
                    warn!("Failed to collect metrics: {}", e);
                }
            }

            // Get dashboard data
            match integration.get_dashboard_data().await {
                Ok(dashboard_data) => {
                    info!("📈 Dashboard data updated with {} alerts", dashboard_data.alerts.len());
                }
                Err(e) => {
                    warn!("Failed to get dashboard data: {}", e);
                }
            }

            // Export data periodically
            if counter % 4 == 0 {
                for format in &["json", "csv", "html"] {
                    match integration.export_dashboard_data(format).await {
                        Ok(data) => {
                            info!("💾 Exported dashboard data as {} ({} bytes)", format, data.len());
                        }
                        Err(e) => {
                            warn!("Failed to export {} data: {}", format, e);
                        }
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_locomo_integration_creation() {
        let performance_monitor = Arc::new(PerformanceMonitor::new_default().unwrap());
        let metrics_collector = Arc::new(LocomoMetricsCollector);
        let benchmark_engine = Arc::new(LocomoBenchmarkEngine);
        let reporting_system = Arc::new(LocomoReportingSystem);
        let config = LocomoIntegrationConfig::default();

        let integration = LocomoPerformanceIntegration::new(
            performance_monitor,
            metrics_collector,
            benchmark_engine,
            reporting_system,
            config,
        );

        assert!(integration.integration_config.enable_locomo_monitoring);
    }

    #[tokio::test]
    async fn test_dashboard_server_creation() {
        let performance_monitor = Arc::new(PerformanceMonitor::new_default().unwrap());
        let metrics_collector = Arc::new(LocomoMetricsCollector);
        let benchmark_engine = Arc::new(LocomoBenchmarkEngine);
        let reporting_system = Arc::new(LocomoReportingSystem);
        let config = LocomoIntegrationConfig::default();

        let integration = Arc::new(LocomoPerformanceIntegration::new(
            performance_monitor,
            metrics_collector,
            benchmark_engine,
            reporting_system,
            config,
        ));

        let dashboard_config = DashboardServerConfig::default();
        let dashboard_server = LocomoDashboardServer::new(integration, dashboard_config);

        assert_eq!(dashboard_server.server_config.port, 8080);
        assert_eq!(dashboard_server.server_config.host, "127.0.0.1");
    }
} 