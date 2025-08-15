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

/// Deployment tool integration
pub struct DeploymentIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl DeploymentIntegration {
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

    /// Deploy to a target environment
    pub async fn deploy(
        &self,
        environment: &str,
        version: Option<&str>,
    ) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let deploy_command = config
            .custom_headers
            .get("deploy_command")
            .map(|s| s.as_str())
            .unwrap_or("echo 'Deploy command not configured'");

        let mut command = deploy_command.to_string();
        if let Some(version) = version {
            command = command.replace("{version}", version);
        }
        command = command.replace("{environment}", environment);

        let output = Command::new("sh").arg("-c").arg(&command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "environment": environment,
            "version": version,
            "command": command
        }))
    }

    /// Rollback to a previous version
    pub async fn rollback(
        &self,
        environment: &str,
        version: &str,
    ) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let rollback_command = config
            .custom_headers
            .get("rollback_command")
            .map(|s| s.as_str())
            .unwrap_or("echo 'Rollback command not configured'");

        let mut command = rollback_command.to_string();
        command = command.replace("{version}", version);
        command = command.replace("{environment}", environment);

        let output = Command::new("sh").arg("-c").arg(&command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "environment": environment,
            "version": version,
            "command": command
        }))
    }

    /// Get deployment status
    pub async fn get_deployment_status(&self, environment: &str) -> RhemaResult<serde_json::Value> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let status_command = config
            .custom_headers
            .get("status_command")
            .map(|s| s.as_str())
            .unwrap_or("echo 'Status command not configured'");

        let mut command = status_command.to_string();
        command = command.replace("{environment}", environment);

        let output = Command::new("sh").arg("-c").arg(&command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "environment": environment,
            "command": command
        }))
    }

    /// List available environments
    pub async fn list_environments(&self) -> RhemaResult<Vec<String>> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let list_command = config
            .custom_headers
            .get("list_command")
            .map(|s| s.as_str())
            .unwrap_or("echo 'dev,staging,prod'");

        let output = Command::new("sh").arg("-c").arg(list_command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let environments: Vec<String> = stdout
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(environments)
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for DeploymentIntegration {
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
            name: "deployment".to_string(),
            version: "1.0.0".to_string(),
            description: "Deployment integration for development".to_string(),
            integration_type: IntegrationType::Deployment,
            capabilities: vec![
                "deployment_management".to_string(),
                "environment_management".to_string(),
            ],
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
