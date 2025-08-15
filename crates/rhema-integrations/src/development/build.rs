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

use crate::{ExternalIntegration, IntegrationConfig, IntegrationStatus, IntegrationMetadata, IntegrationHttpClient, IntegrationType};
use rhema_core::{RhemaResult, RhemaError};

use std::collections::HashMap;
use std::process::Command;

/// Build tool integration
pub struct BuildIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl BuildIntegration {
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
    
    /// Build the project
    pub async fn build(&self, build_type: Option<&str>) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Build not configured".to_string()))?;
        let build_command = match build_type {
            Some("release") => config.custom_headers.get("release_command").map(|s| s.as_str()).unwrap_or("cargo build --release"),
            Some("debug") => config.custom_headers.get("debug_command").map(|s| s.as_str()).unwrap_or("cargo build"),
            _ => config.custom_headers.get("build_command").map(|s| s.as_str()).unwrap_or("cargo build"),
        };
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(build_command)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "build_type": build_type.unwrap_or("default"),
            "command": build_command
        }))
    }
    
    /// Clean build artifacts
    pub async fn clean(&self) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Build not configured".to_string()))?;
        let clean_command = config.custom_headers.get("clean_command").map(|s| s.as_str()).unwrap_or("cargo clean");
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(clean_command)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "command": clean_command
        }))
    }
    
    /// Check if project builds successfully
    pub async fn check(&self) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Build not configured".to_string()))?;
        let check_command = config.custom_headers.get("check_command").map(|s| s.as_str()).unwrap_or("cargo check");
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(check_command)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "command": check_command
        }))
    }
    
    /// Get build information
    pub async fn get_build_info(&self) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Build not configured".to_string()))?;
        let info_command = config.custom_headers.get("info_command").map(|s| s.as_str()).unwrap_or("cargo --version");
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(info_command)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        Ok(serde_json::json!({
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "command": info_command
        }))
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for BuildIntegration {
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
            name: "build".to_string(),
            version: "1.0.0".to_string(),
            description: "Build integration for development".to_string(),
            integration_type: IntegrationType::Build,
            capabilities: vec!["build_management".to_string(), "artifact_management".to_string()],
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
