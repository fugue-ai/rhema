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

use crate::{
    ExternalIntegration, IntegrationConfig, IntegrationHttpClient, IntegrationMetadata,
    IntegrationStatus, IntegrationType,
};
use rhema_core::{RhemaError, RhemaResult};

use std::collections::HashMap;
use std::process::Command;

/// Testing framework integration
pub struct TestingIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl TestingIntegration {
    pub fn new() -> Self {
        Self {
            config: None,
            http_client: IntegrationHttpClient::new(),
            status: IntegrationStatus {
                connected: false,
                last_check: chrono::Utc::now(),
                error_message: None,
                response_time_ms: None,
                rate_limit_remaining: None,
                rate_limit_reset: None,
            },
        }
    }

    /// Run tests
    pub async fn run_tests(&self, test_command: Option<&str>) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let command = test_command.unwrap_or_else(|| {
            config
                .custom_headers
                .get("test_command")
                .map(|s| s.as_str())
                .unwrap_or("cargo test")
        });

        let output = Command::new("sh").arg("-c").arg(command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "command": command
        }))
    }

    /// Run specific test
    pub async fn run_specific_test(&self, test_name: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let base_command = config
            .custom_headers
            .get("test_command")
            .map(|s| s.as_str())
            .unwrap_or("cargo test");
        let command = format!("{} {}", base_command, test_name);

        let output = Command::new("sh").arg("-c").arg(&command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "test_name": test_name,
            "command": command
        }))
    }

    /// Get test coverage
    pub async fn get_coverage(&self) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let coverage_command = config
            .custom_headers
            .get("coverage_command")
            .map(|s| s.as_str())
            .unwrap_or("cargo tarpaulin");

        let output = Command::new("sh")
            .arg("-c")
            .arg(coverage_command)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "command": coverage_command
        }))
    }

    /// List available tests
    pub async fn list_tests(&self) -> RhemaResult<Vec<String>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let list_command = config
            .custom_headers
            .get("list_command")
            .map(|s| s.as_str())
            .unwrap_or("cargo test --list");

        let output = Command::new("sh").arg("-c").arg(list_command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let tests: Vec<String> = stdout
            .lines()
            .filter(|line| line.contains("test"))
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .collect();

        Ok(tests)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for TestingIntegration {
    async fn initialize(&mut self, config: IntegrationConfig) -> RhemaResult<()> {
        self.config = Some(config);
        self.status.connected = true;
        self.status.last_check = chrono::Utc::now();
        Ok(())
    }

    async fn test_connection(&self) -> RhemaResult<bool> {
        Ok(self.config.is_some())
    }

    fn get_metadata(&self) -> IntegrationMetadata {
        IntegrationMetadata {
            name: "testing".to_string(),
            version: "1.0.0".to_string(),
            description: "Testing integration for development".to_string(),
            integration_type: IntegrationType::Testing,
            capabilities: vec!["test_execution".to_string(), "test_reporting".to_string()],
            required_config: vec!["api_key".to_string()],
            optional_config: vec!["base_url".to_string()],
        }
    }

    async fn get_status(&self) -> RhemaResult<IntegrationStatus> {
        Ok(self.status.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
