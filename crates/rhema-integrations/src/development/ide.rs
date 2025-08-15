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

/// IDE integration for development workflows
pub struct IDEIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl IDEIntegration {
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
    
    /// Open a file in the configured IDE
    pub async fn open_file(&self, file_path: &str, line: Option<u32>) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("IDE not configured".to_string()))?;
        let ide_command = config.custom_headers.get("ide_command").ok_or_else(|| RhemaError::ConfigError("IDE command not configured".to_string()))?;
        
        let mut command = Command::new(ide_command);
        
        if let Some(line) = line {
            if ide_command.contains("code") {
                // VS Code
                command.args(&["--goto", &format!("{}:{}", file_path, line)]);
            } else if ide_command.contains("vim") || ide_command.contains("nvim") {
                // Vim/Neovim
                command.args(&["+{}", file_path]);
            } else if ide_command.contains("subl") {
                // Sublime Text
                command.args(&[&format!("{}:{}", file_path, line)]);
            } else {
                // Generic
                command.arg(file_path);
            }
        } else {
            command.arg(file_path);
        }
        
        command.spawn()?;
        Ok(())
    }
    
    /// Open a project in the IDE
    pub async fn open_project(&self, project_path: &str) -> RhemaResult<()> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("IDE not configured".to_string()))?;
        let ide_command = config.custom_headers.get("ide_command").ok_or_else(|| RhemaError::ConfigError("IDE command not configured".to_string()))?;
        
        let mut command = Command::new(ide_command);
        command.arg(project_path);
        
        command.spawn()?;
        Ok(())
    }
    
    /// Get IDE status
    pub async fn get_ide_status(&self) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("IDE not configured".to_string()))?;
        let ide_command = config.custom_headers.get("ide_command").ok_or_else(|| RhemaError::ConfigError("IDE command not configured".to_string()))?;
        
        let output = Command::new(ide_command)
            .arg("--version")
            .output()?;
        
        let version = String::from_utf8_lossy(&output.stdout);
        
        Ok(serde_json::json!({
            "ide": ide_command,
            "version": version.trim(),
            "available": output.status.success()
        }))
    }
    
    /// Execute IDE command
    pub async fn execute_command(&self, command: &str, args: Vec<&str>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("IDE not configured".to_string()))?;
        let ide_command = config.custom_headers.get("ide_command").ok_or_else(|| RhemaError::ConfigError("IDE command not configured".to_string()))?;
        
        let mut cmd = Command::new(ide_command);
        cmd.arg(command);
        cmd.args(args);
        
        let output = cmd.output()?;
        let result = String::from_utf8_lossy(&output.stdout);
        
        Ok(result.to_string())
    }
}

#[async_trait::async_trait]
impl ExternalIntegration for IDEIntegration {
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
            name: "ide".to_string(),
            version: "1.0.0".to_string(),
            description: "IDE integration for development".to_string(),
            integration_type: IntegrationType::IDE,
            capabilities: vec!["code_analysis".to_string(), "refactoring".to_string()],
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
