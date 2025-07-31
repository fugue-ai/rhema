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

use super::*;

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

#[async_trait]
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

/// Code Review integration
pub struct CodeReviewIntegration {
    pub config: Option<IntegrationConfig>,
    pub http_client: IntegrationHttpClient,
    pub status: IntegrationStatus,
}

impl CodeReviewIntegration {
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
    
    /// Create a pull request/merge request
    pub async fn create_pull_request(&self, title: &str, description: &str, source_branch: &str, target_branch: &str) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let pr_data = serde_json::json!({
            "title": title,
            "body": description,
            "head": source_branch,
            "base": target_branch
        });
        
        let url = format!("{}/repos/{}/pulls", base_url, config.custom_headers.get("repo").unwrap_or(&"".to_string()));
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        
        let response = self.http_client.post(&url, &pr_data.to_string(), Some(headers)).await?;
        let pr: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(pr["number"].as_u64().unwrap_or(0).to_string())
    }
    
    /// Get pull request details
    pub async fn get_pull_request(&self, pr_number: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/repos/{}/pulls/{}", base_url, config.custom_headers.get("repo").unwrap_or(&"".to_string()), pr_number);
        let mut headers = HashMap::new();
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let pr: serde_json::Value = serde_json::from_str(&response)?;
        Ok(pr)
    }
    
    /// Add a comment to a pull request
    pub async fn add_comment(&self, pr_number: &str, comment: &str, path: Option<&str>, line: Option<u32>) -> RhemaResult<String> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let mut comment_data = serde_json::json!({
            "body": comment
        });
        
        if let Some(path) = path {
            comment_data["path"] = serde_json::Value::String(path.to_string());
            if let Some(line) = line {
                comment_data["line"] = serde_json::Value::Number(serde_json::Number::from(line));
            }
        }
        
        let url = format!("{}/repos/{}/pulls/{}/comments", base_url, config.custom_headers.get("repo").unwrap_or(&"".to_string()), pr_number);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        
        let response = self.http_client.post(&url, &comment_data.to_string(), Some(headers)).await?;
        let comment_result: serde_json::Value = serde_json::from_str(&response)?;
        
        Ok(comment_result["id"].as_u64().unwrap_or(0).to_string())
    }
    
    /// Get pull request comments
    pub async fn get_comments(&self, pr_number: &str) -> RhemaResult<Vec<serde_json::Value>> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Code Review not configured".to_string()))?;
        let base_url = config.base_url.as_ref().ok_or_else(|| RhemaError::ConfigError("Base URL not configured".to_string()))?;
        
        let url = format!("{}/repos/{}/pulls/{}/comments", base_url, config.custom_headers.get("repo").unwrap_or(&"".to_string()), pr_number);
        let mut headers = HashMap::new();
        
        if let Some(token) = &config.token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        
        let response = self.http_client.get(&url, Some(headers)).await?;
        let comments: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(comments)
    }
}

#[async_trait]
impl ExternalIntegration for CodeReviewIntegration {
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
            name: "code_review".to_string(),
            version: "1.0.0".to_string(),
            description: "Code Review integration for development".to_string(),
            integration_type: IntegrationType::CodeReview,
            capabilities: vec!["review_management".to_string(), "comment_system".to_string()],
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let command = test_command.unwrap_or_else(|| config.custom_headers.get("test_command").map(|s| s.as_str()).unwrap_or("cargo test"));
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;
        
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let base_command = config.custom_headers.get("test_command").map(|s| s.as_str()).unwrap_or("cargo test");
        let command = format!("{} {}", base_command, test_name);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()?;
        
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let coverage_command = config.custom_headers.get("coverage_command").map(|s| s.as_str()).unwrap_or("cargo tarpaulin");
        
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Testing not configured".to_string()))?;
        let list_command = config.custom_headers.get("list_command").map(|s| s.as_str()).unwrap_or("cargo test --list");
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(list_command)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let tests: Vec<String> = stdout
            .lines()
            .filter(|line| line.contains("test"))
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .collect();
        
        Ok(tests)
    }
}

#[async_trait]
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

#[async_trait]
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
    pub async fn deploy(&self, environment: &str, version: Option<&str>) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let deploy_command = config.custom_headers.get("deploy_command").map(|s| s.as_str()).unwrap_or("echo 'Deploy command not configured'");
        
        let mut command = deploy_command.to_string();
        if let Some(version) = version {
            command = command.replace("{version}", version);
        }
        command = command.replace("{environment}", environment);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()?;
        
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
    pub async fn rollback(&self, environment: &str, version: &str) -> RhemaResult<serde_json::Value> {
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let rollback_command = config.custom_headers.get("rollback_command").map(|s| s.as_str()).unwrap_or("echo 'Rollback command not configured'");
        
        let mut command = rollback_command.to_string();
        command = command.replace("{version}", version);
        command = command.replace("{environment}", environment);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()?;
        
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let status_command = config.custom_headers.get("status_command").map(|s| s.as_str()).unwrap_or("echo 'Status command not configured'");
        
        let mut command = status_command.to_string();
        command = command.replace("{environment}", environment);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()?;
        
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
        let config = self.config.as_ref().ok_or_else(|| RhemaError::ConfigError("Deployment not configured".to_string()))?;
        let list_command = config.custom_headers.get("list_command").map(|s| s.as_str()).unwrap_or("echo 'dev,staging,prod'");
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(list_command)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let environments: Vec<String> = stdout
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        Ok(environments)
    }
}

#[async_trait]
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
            capabilities: vec!["deployment_management".to_string(), "environment_management".to_string()],
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