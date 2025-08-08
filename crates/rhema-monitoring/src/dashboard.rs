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

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
// use hyper_util::server::Server;
use tower_http::cors::CorsLayer;

use crate::locomo_integration::LocomoPerformanceIntegration;
use rhema_core::RhemaResult;

/// LOCOMO Dashboard Server
pub struct LocomoDashboardServer {
    integration: Arc<LocomoPerformanceIntegration>,
    server_config: DashboardServerConfig,
}

/// Dashboard Server Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
    pub cors_origins: Vec<String>,
    pub enable_websockets: bool,
    pub refresh_interval_ms: u64,
    pub max_connections: usize,
}

impl Default for DashboardServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            cors_origins: vec![
                "http://localhost:3000".to_string(),
                "http://127.0.0.1:3000".to_string(),
            ],
            enable_websockets: true,
            refresh_interval_ms: 5000,
            max_connections: 100,
        }
    }
}

/// Dashboard API Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Dashboard State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardState {
    pub current_metrics: serde_json::Value,
    pub recent_alerts: Vec<serde_json::Value>,
    pub performance_trends: serde_json::Value,
    pub quality_trends: serde_json::Value,
    pub optimization_trends: serde_json::Value,
    pub system_health: SystemHealthStatus,
    pub last_updated: DateTime<Utc>,
}

/// System Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthStatus {
    pub overall_status: HealthStatus,
    pub performance_status: HealthStatus,
    pub quality_status: HealthStatus,
    pub optimization_status: HealthStatus,
    pub alerts_count: usize,
    pub critical_alerts: usize,
}

/// Health Status Enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl LocomoDashboardServer {
    /// Create a new dashboard server
    pub fn new(
        integration: Arc<LocomoPerformanceIntegration>,
        config: DashboardServerConfig,
    ) -> Self {
        Self {
            integration,
            server_config: config,
        }
    }

    /// Start the dashboard server
    pub async fn start(&self) -> RhemaResult<()> {
        info!(
            "Starting LOCOMO Dashboard server on {}:{}",
            self.server_config.host, self.server_config.port
        );

        let app_state = Arc::new(DashboardAppState {
            integration: self.integration.clone(),
        });

        let mut app = Router::new()
            .route("/", get(Self::serve_dashboard))
            .route("/api/metrics", get(Self::get_metrics))
            .route("/api/alerts", get(Self::get_alerts))
            .route("/api/trends", get(Self::get_trends))
            .route("/api/health", get(Self::get_health))
            .route("/api/export/:format", get(Self::export_data))
            .route("/api/refresh", post(Self::refresh_data))
            .with_state(app_state);

        // Add CORS if enabled
        if self.server_config.enable_cors {
            let cors = CorsLayer::permissive();
            app = app.layer(cors);
        }

        let addr = format!("{}:{}", self.server_config.host, self.server_config.port)
            .parse::<std::net::SocketAddr>()
            .map_err(|e| rhema_core::RhemaError::SystemError(format!("Invalid address: {}", e)))?;

        info!("Dashboard server listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| rhema_core::RhemaError::SystemError(format!("Failed to bind: {}", e)))?;
        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| rhema_core::RhemaError::SystemError(format!("Server error: {}", e)))?;

        Ok(())
    }

    /// Serve the main dashboard HTML
    async fn serve_dashboard() -> Html<String> {
        Html(DASHBOARD_HTML.to_string())
    }

    /// Get current metrics
    async fn get_metrics(
        State(state): State<Arc<DashboardAppState>>,
    ) -> Result<Json<DashboardApiResponse<serde_json::Value>>, StatusCode> {
        match state.integration.get_dashboard_data().await {
            Ok(dashboard_data) => {
                let response = DashboardApiResponse {
                    success: true,
                    data: Some(serde_json::to_value(dashboard_data).unwrap_or_default()),
                    error: None,
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                let response = DashboardApiResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
        }
    }

    /// Get alerts
    async fn get_alerts(
        State(state): State<Arc<DashboardAppState>>,
    ) -> Result<Json<DashboardApiResponse<Vec<serde_json::Value>>>, StatusCode> {
        match state.integration.get_dashboard_data().await {
            Ok(dashboard_data) => {
                let alerts = dashboard_data
                    .alerts
                    .into_iter()
                    .map(|alert| serde_json::to_value(alert).unwrap_or_default())
                    .collect();

                let response = DashboardApiResponse {
                    success: true,
                    data: Some(alerts),
                    error: None,
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                let response = DashboardApiResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
        }
    }

    /// Get trends
    async fn get_trends(
        State(state): State<Arc<DashboardAppState>>,
    ) -> Result<Json<DashboardApiResponse<serde_json::Value>>, StatusCode> {
        match state.integration.get_dashboard_data().await {
            Ok(dashboard_data) => {
                let trends = serde_json::json!({
                    "performance": dashboard_data.performance_chart,
                    "quality": dashboard_data.quality_chart,
                    "optimization": dashboard_data.optimization_chart,
                });

                let response = DashboardApiResponse {
                    success: true,
                    data: Some(trends),
                    error: None,
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                let response = DashboardApiResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
        }
    }

    /// Get system health
    async fn get_health(
        State(state): State<Arc<DashboardAppState>>,
    ) -> Result<Json<DashboardApiResponse<SystemHealthStatus>>, StatusCode> {
        match state.integration.get_dashboard_data().await {
            Ok(dashboard_data) => {
                let alerts_count = dashboard_data.alerts.len();
                let critical_alerts = dashboard_data
                    .alerts
                    .iter()
                    .filter(|alert| alert.severity == "Critical")
                    .count();

                let health_status = SystemHealthStatus {
                    overall_status: if critical_alerts > 0 {
                        HealthStatus::Critical
                    } else if alerts_count > 5 {
                        HealthStatus::Warning
                    } else {
                        HealthStatus::Healthy
                    },
                    performance_status: HealthStatus::Healthy, // Would be calculated from metrics
                    quality_status: HealthStatus::Healthy,     // Would be calculated from metrics
                    optimization_status: HealthStatus::Healthy, // Would be calculated from metrics
                    alerts_count,
                    critical_alerts,
                };

                let response = DashboardApiResponse {
                    success: true,
                    data: Some(health_status),
                    error: None,
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                let response = DashboardApiResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
        }
    }

    /// Export data in specified format
    async fn export_data(
        State(state): State<Arc<DashboardAppState>>,
        axum::extract::Path(format): axum::extract::Path<String>,
    ) -> Result<Json<DashboardApiResponse<String>>, StatusCode> {
        match state.integration.export_dashboard_data(&format).await {
            Ok(data) => {
                let response = DashboardApiResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                let response = DashboardApiResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
        }
    }

    /// Refresh dashboard data
    async fn refresh_data(
        State(state): State<Arc<DashboardAppState>>,
    ) -> Result<Json<DashboardApiResponse<serde_json::Value>>, StatusCode> {
        // Force refresh by getting fresh data
        match state.integration.get_dashboard_data().await {
            Ok(dashboard_data) => {
                let response = DashboardApiResponse {
                    success: true,
                    data: Some(serde_json::to_value(dashboard_data).unwrap_or_default()),
                    error: None,
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                let response = DashboardApiResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now(),
                };
                Ok(Json(response))
            }
        }
    }
}

/// Dashboard Application State
struct DashboardAppState {
    integration: Arc<LocomoPerformanceIntegration>,
}

/// Dashboard HTML Template
const DASHBOARD_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>LOCOMO Performance Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }

        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: rgba(255, 255, 255, 0.95);
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }

        .header {
            background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }

        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
        }

        .header p {
            font-size: 1.1em;
            opacity: 0.9;
        }

        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            padding: 30px;
        }

        .metric-card {
            background: white;
            border-radius: 10px;
            padding: 25px;
            box-shadow: 0 5px 15px rgba(0, 0, 0, 0.08);
            border-left: 4px solid #3498db;
        }

        .metric-card h3 {
            color: #2c3e50;
            margin-bottom: 15px;
            font-size: 1.3em;
        }

        .metric-value {
            font-size: 2.5em;
            font-weight: bold;
            color: #3498db;
            margin-bottom: 10px;
        }

        .metric-label {
            color: #7f8c8d;
            font-size: 0.9em;
            text-transform: uppercase;
            letter-spacing: 1px;
        }

        .charts-section {
            padding: 30px;
            background: #f8f9fa;
        }

        .chart-container {
            background: white;
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 20px;
            box-shadow: 0 5px 15px rgba(0, 0, 0, 0.08);
        }

        .chart-container h3 {
            color: #2c3e50;
            margin-bottom: 15px;
        }

        .alerts-section {
            padding: 30px;
        }

        .alert {
            background: #fff3cd;
            border: 1px solid #ffeaa7;
            border-radius: 8px;
            padding: 15px;
            margin-bottom: 10px;
        }

        .alert.critical {
            background: #f8d7da;
            border-color: #f5c6cb;
        }

        .alert.warning {
            background: #fff3cd;
            border-color: #ffeaa7;
        }

        .controls {
            padding: 20px 30px;
            background: #ecf0f1;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .btn {
            background: #3498db;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 5px;
            cursor: pointer;
            font-size: 14px;
            transition: background 0.3s;
        }

        .btn:hover {
            background: #2980b9;
        }

        .status-indicator {
            display: inline-block;
            width: 12px;
            height: 12px;
            border-radius: 50%;
            margin-right: 8px;
        }

        .status-healthy { background: #27ae60; }
        .status-warning { background: #f39c12; }
        .status-critical { background: #e74c3c; }

        .loading {
            text-align: center;
            padding: 50px;
            color: #7f8c8d;
        }

        @media (max-width: 768px) {
            .metrics-grid {
                grid-template-columns: 1fr;
            }
            
            .controls {
                flex-direction: column;
                gap: 10px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 LOCOMO Performance Dashboard</h1>
            <p>Real-time monitoring and analytics for context optimization</p>
        </div>

        <div class="controls">
            <div>
                <span class="status-indicator status-healthy" id="connection-status"></span>
                <span id="last-updated">Last updated: Never</span>
            </div>
            <div>
                <button class="btn" onclick="refreshData()">Refresh</button>
                <button class="btn" onclick="exportData('json')">Export JSON</button>
                <button class="btn" onclick="exportData('csv')">Export CSV</button>
            </div>
        </div>

        <div class="metrics-grid">
            <div class="metric-card">
                <h3>Performance Score</h3>
                <div class="metric-value" id="performance-score">--</div>
                <div class="metric-label">Context Retrieval Latency</div>
            </div>
            <div class="metric-card">
                <h3>Quality Score</h3>
                <div class="metric-value" id="quality-score">--</div>
                <div class="metric-label">Context Relevance</div>
            </div>
            <div class="metric-card">
                <h3>Optimization Score</h3>
                <div class="metric-value" id="optimization-score">--</div>
                <div class="metric-label">AI Optimization</div>
            </div>
            <div class="metric-card">
                <h3>System Health</h3>
                <div class="metric-value" id="health-status">--</div>
                <div class="metric-label">Overall Status</div>
            </div>
        </div>

        <div class="charts-section">
            <div class="chart-container">
                <h3>Performance Trends</h3>
                <canvas id="performance-chart" width="400" height="200"></canvas>
            </div>
            <div class="chart-container">
                <h3>Quality Trends</h3>
                <canvas id="quality-chart" width="400" height="200"></canvas>
            </div>
            <div class="chart-container">
                <h3>Optimization Trends</h3>
                <canvas id="optimization-chart" width="400" height="200"></canvas>
            </div>
        </div>

        <div class="alerts-section">
            <h3>Recent Alerts</h3>
            <div id="alerts-container">
                <div class="loading">Loading alerts...</div>
            </div>
        </div>
    </div>

    <script>
        let performanceChart, qualityChart, optimizationChart;
        let refreshInterval;

        // Initialize dashboard
        document.addEventListener('DOMContentLoaded', function() {
            initializeCharts();
            loadData();
            startAutoRefresh();
        });

        function initializeCharts() {
            const ctx1 = document.getElementById('performance-chart').getContext('2d');
            const ctx2 = document.getElementById('quality-chart').getContext('2d');
            const ctx3 = document.getElementById('optimization-chart').getContext('2d');

            performanceChart = new Chart(ctx1, {
                type: 'line',
                data: {
                    labels: [],
                    datasets: [{
                        label: 'Performance Score',
                        data: [],
                        borderColor: '#3498db',
                        backgroundColor: 'rgba(52, 152, 219, 0.1)',
                        tension: 0.4
                    }]
                },
                options: {
                    responsive: true,
                    scales: {
                        y: {
                            beginAtZero: true,
                            max: 100
                        }
                    }
                }
            });

            qualityChart = new Chart(ctx2, {
                type: 'line',
                data: {
                    labels: [],
                    datasets: [{
                        label: 'Quality Score',
                        data: [],
                        borderColor: '#27ae60',
                        backgroundColor: 'rgba(39, 174, 96, 0.1)',
                        tension: 0.4
                    }]
                },
                options: {
                    responsive: true,
                    scales: {
                        y: {
                            beginAtZero: true,
                            max: 100
                        }
                    }
                }
            });

            optimizationChart = new Chart(ctx3, {
                type: 'line',
                data: {
                    labels: [],
                    datasets: [{
                        label: 'Optimization Score',
                        data: [],
                        borderColor: '#f39c12',
                        backgroundColor: 'rgba(243, 156, 18, 0.1)',
                        tension: 0.4
                    }]
                },
                options: {
                    responsive: true,
                    scales: {
                        y: {
                            beginAtZero: true,
                            max: 100
                        }
                    }
                }
            });
        }

        async function loadData() {
            try {
                updateConnectionStatus('connecting');
                
                const [metricsResponse, alertsResponse, trendsResponse, healthResponse] = await Promise.all([
                    fetch('/api/metrics'),
                    fetch('/api/alerts'),
                    fetch('/api/trends'),
                    fetch('/api/health')
                ]);

                if (metricsResponse.ok) {
                    const metricsData = await metricsResponse.json();
                    if (metricsData.success) {
                        updateMetrics(metricsData.data);
                    }
                }

                if (alertsResponse.ok) {
                    const alertsData = await alertsResponse.json();
                    if (alertsData.success) {
                        updateAlerts(alertsData.data);
                    }
                }

                if (trendsResponse.ok) {
                    const trendsData = await trendsResponse.json();
                    if (trendsData.success) {
                        updateCharts(trendsData.data);
                    }
                }

                if (healthResponse.ok) {
                    const healthData = await healthResponse.json();
                    if (healthData.success) {
                        updateHealth(healthData.data);
                    }
                }

                updateConnectionStatus('connected');
                updateLastUpdated();
            } catch (error) {
                console.error('Error loading data:', error);
                updateConnectionStatus('error');
            }
        }

        function updateMetrics(data) {
            if (data && data.current_metrics) {
                const metrics = data.current_metrics;
                document.getElementById('performance-score').textContent = 
                    metrics.context_retrieval_latency_ms ? metrics.context_retrieval_latency_ms.toFixed(2) + 'ms' : '--';
                document.getElementById('quality-score').textContent = 
                    metrics.quality_score ? (metrics.quality_score * 100).toFixed(1) + '%' : '--';
                document.getElementById('optimization-score').textContent = 
                    metrics.ai_optimization_score ? (metrics.ai_optimization_score * 100).toFixed(1) + '%' : '--';
            }
        }

        function updateAlerts(alerts) {
            const container = document.getElementById('alerts-container');
            if (!alerts || alerts.length === 0) {
                container.innerHTML = '<div class="alert">No active alerts</div>';
                return;
            }

            container.innerHTML = alerts.map(alert => `
                <div class="alert ${alert.severity?.toLowerCase() || 'info'}">
                    <strong>${alert.alert_type || 'Alert'}:</strong> ${alert.message || 'No message'}
                </div>
            `).join('');
        }

        function updateCharts(data) {
            if (data && data.performance) {
                updateChart(performanceChart, data.performance);
            }
            if (data && data.quality) {
                updateChart(qualityChart, data.quality);
            }
            if (data && data.optimization) {
                updateChart(optimizationChart, data.optimization);
            }
        }

        function updateChart(chart, chartData) {
            if (chartData.data && chartData.data.labels) {
                chart.data.labels = chartData.data.labels;
                chart.data.datasets[0].data = chartData.data.datasets[0].data;
                chart.update();
            }
        }

        function updateHealth(health) {
            if (health) {
                const status = health.overall_status || 'Unknown';
                document.getElementById('health-status').textContent = status;
                
                const statusIndicator = document.querySelector('.status-indicator');
                statusIndicator.className = 'status-indicator';
                statusIndicator.classList.add(`status-${status.toLowerCase()}`);
            }
        }

        function updateConnectionStatus(status) {
            const indicator = document.getElementById('connection-status');
            indicator.className = 'status-indicator';
            
            switch (status) {
                case 'connected':
                    indicator.classList.add('status-healthy');
                    break;
                case 'connecting':
                    indicator.classList.add('status-warning');
                    break;
                case 'error':
                    indicator.classList.add('status-critical');
                    break;
            }
        }

        function updateLastUpdated() {
            document.getElementById('last-updated').textContent = 
                'Last updated: ' + new Date().toLocaleTimeString();
        }

        function startAutoRefresh() {
            refreshInterval = setInterval(loadData, 5000); // Refresh every 5 seconds
        }

        function refreshData() {
            loadData();
        }

        async function exportData(format) {
            try {
                const response = await fetch(`/api/export/${format}`);
                if (response.ok) {
                    const data = await response.json();
                    if (data.success) {
                        const blob = new Blob([data.data], { type: 'text/plain' });
                        const url = window.URL.createObjectURL(blob);
                        const a = document.createElement('a');
                        a.href = url;
                        a.download = `locomo-dashboard.${format}`;
                        a.click();
                        window.URL.revokeObjectURL(url);
                    }
                }
            } catch (error) {
                console.error('Error exporting data:', error);
            }
        }

        // Cleanup on page unload
        window.addEventListener('beforeunload', function() {
            if (refreshInterval) {
                clearInterval(refreshInterval);
            }
        });
    </script>
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dashboard_server_creation() {
        let integration = Arc::new(LocomoPerformanceIntegration::new(
            Arc::new(crate::PerformanceMonitor::new_default().unwrap()),
            Arc::new(rhema_locomo::LocomoMetricsCollector),
            Arc::new(rhema_locomo::LocomoBenchmarkEngine),
            Arc::new(rhema_locomo::LocomoReportingSystem),
            crate::locomo_integration::LocomoIntegrationConfig::default(),
        ));

        let config = DashboardServerConfig::default();
        let server = LocomoDashboardServer::new(integration, config);

        assert_eq!(server.server_config.port, 8080);
        assert_eq!(server.server_config.host, "127.0.0.1");
    }

    #[test]
    fn test_dashboard_config_default() {
        let config = DashboardServerConfig::default();
        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "127.0.0.1");
        assert!(config.enable_cors);
        assert!(config.enable_websockets);
    }
}
