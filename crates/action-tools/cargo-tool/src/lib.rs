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

use async_trait::async_trait;
use serde_json::Value;
use tracing::{info, error};
use rhema_action_tool::{ActionIntent, ActionResult, ActionError, SafetyLevel};
use rhema_action_tool::{ValidationTool, TransformationTool, ToolResult};

/// Cargo validation and transformation tool
pub struct CargoTool;

/// Supported Cargo commands
#[derive(Debug, Clone, PartialEq)]
pub enum CargoCommand {
    Check,
    Build,
    Test,
    Clippy,
    Fmt,
    Audit,
    Outdated,
}

/// Cargo operation result
#[derive(Debug, Clone)]
pub struct CargoResult {
    pub command: CargoCommand,
    pub success: bool,
    pub output: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration: std::time::Duration,
}

/// Workspace member information
#[derive(Debug, Clone)]
pub struct WorkspaceMember {
    pub name: String,
    pub path: String,
    pub package_type: PackageType,
}

/// Package type classification
#[derive(Debug, Clone, PartialEq)]
pub enum PackageType {
    Library,
    Binary,
    Both,
    Unknown,
}

/// Workspace information
#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub root_path: String,
    pub members: Vec<WorkspaceMember>,
    pub workspace_config: Option<Value>,
}

/// Cargo tool configuration
#[derive(Debug, Clone)]
pub struct CargoConfig {
    pub commands: Vec<CargoCommand>,
    pub parallel: bool,
    pub json_output: bool,
    pub verbose: bool,
    pub workspace_mode: WorkspaceMode,
    pub member_filter: Option<Vec<String>>,
    pub exclude_members: Option<Vec<String>>,
}

/// Workspace execution mode
#[derive(Debug, Clone, PartialEq)]
pub enum WorkspaceMode {
    /// Execute on workspace root only
    RootOnly,
    /// Execute on all workspace members
    AllMembers,
    /// Execute on workspace root and all members
    RootAndMembers,
    /// Execute only on specified members
    SelectedMembers,
}

impl Default for CargoConfig {
    fn default() -> Self {
        Self {
            commands: vec![CargoCommand::Check],
            parallel: true,
            json_output: true,
            verbose: false,
            workspace_mode: WorkspaceMode::RootAndMembers,
            member_filter: None,
            exclude_members: None,
        }
    }
}

#[async_trait]
impl ValidationTool for CargoTool {
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Running Cargo validation for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        let config = self.parse_config(intent);
        
        // Find Cargo.toml files in the scope
        let cargo_files: Vec<&str> = intent.scope.iter()
            .filter(|file| file.ends_with("Cargo.toml"))
            .map(|s| s.as_str())
            .collect();
        
        if cargo_files.is_empty() {
            return Ok(ToolResult {
                success: true,
                changes: vec![],
                output: "No Cargo.toml files found to validate".to_string(),
                errors: vec![],
                warnings: vec![],
                duration: start.elapsed(),
            });
        }
        
        // Run cargo commands for each project
        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();
        let mut all_changes = Vec::new();
        
        for cargo_file in &cargo_files {
            match self.run_cargo_commands_with_workspace(cargo_file, &config).await {
                Ok(results) => {
                    for result in results {
                        all_errors.extend(result.errors);
                        all_warnings.extend(result.warnings);
                        if !result.output.is_empty() {
                            all_changes.push(result.output);
                        }
                    }
                },
                Err(e) => all_errors.push(format!("Cargo operations failed for {}: {}", cargo_file, e)),
            }
        }
        
        let success = all_errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes: all_changes,
            output: format!("Cargo validation completed for {} projects", cargo_files.len()),
            errors: all_errors,
            warnings: all_warnings,
            duration: start.elapsed(),
        })
    }
    
    fn name(&self) -> &str {
        "cargo"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if Cargo is installed
        tokio::process::Command::new("cargo")
            .args(&["--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[async_trait]
impl TransformationTool for CargoTool {
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        info!("Executing Cargo transformations for intent: {}", intent.id);
        
        let start = std::time::Instant::now();
        let config = self.parse_config(intent);
        
        // Find Cargo.toml files in the scope
        let cargo_files: Vec<&str> = intent.scope.iter()
            .filter(|file| file.ends_with("Cargo.toml"))
            .map(|s| s.as_str())
            .collect();
        
        if cargo_files.is_empty() {
            return Err(ActionError::Validation("No Cargo.toml files found for transformation".to_string()));
        }
        
        // Run transformation commands (fmt, clippy --fix)
        let mut all_errors = Vec::new();
        let mut all_warnings = Vec::new();
        let mut all_changes = Vec::new();
        
        for cargo_file in &cargo_files {
            match self.run_transformation_commands_with_workspace(cargo_file, &config).await {
                Ok(results) => {
                    for result in results {
                        all_errors.extend(result.errors);
                        all_warnings.extend(result.warnings);
                        if !result.output.is_empty() {
                            all_changes.push(result.output);
                        }
                    }
                },
                Err(e) => all_errors.push(format!("Cargo transformation failed for {}: {}", cargo_file, e)),
            }
        }
        
        let success = all_errors.is_empty();
        
        Ok(ToolResult {
            success,
            changes: all_changes,
            output: format!("Cargo transformations completed for {} projects", cargo_files.len()),
            errors: all_errors,
            warnings: all_warnings,
            duration: start.elapsed(),
        })
    }
    
    fn supports_language(&self, language: &str) -> bool {
        language == "rust"
    }
    
    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Medium
    }
    
    fn name(&self) -> &str {
        "cargo"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    async fn is_available(&self) -> bool {
        // Check if Cargo is installed
        tokio::process::Command::new("cargo")
            .args(&["--version"])
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl CargoTool {
    /// Parse configuration from intent metadata
    fn parse_config(&self, intent: &ActionIntent) -> CargoConfig {
        let mut config = CargoConfig::default();
        
        if !intent.metadata.is_null() {
            if let Some(commands) = intent.metadata.get("commands") {
                if let Some(cmd_array) = commands.as_array() {
                    config.commands = cmd_array.iter()
                        .filter_map(|cmd| {
                            cmd.as_str().and_then(|s| match s {
                                "check" => Some(CargoCommand::Check),
                                "build" => Some(CargoCommand::Build),
                                "test" => Some(CargoCommand::Test),
                                "clippy" => Some(CargoCommand::Clippy),
                                "fmt" => Some(CargoCommand::Fmt),
                                "audit" => Some(CargoCommand::Audit),
                                "outdated" => Some(CargoCommand::Outdated),
                                _ => None,
                            })
                        })
                        .collect();
                }
            }
            
            if let Some(parallel) = intent.metadata.get("parallel") {
                config.parallel = parallel.as_bool().unwrap_or(true);
            }
            
            if let Some(json_output) = intent.metadata.get("json_output") {
                config.json_output = json_output.as_bool().unwrap_or(true);
            }
            
            if let Some(verbose) = intent.metadata.get("verbose") {
                config.verbose = verbose.as_bool().unwrap_or(false);
            }
            
            // Parse workspace configuration
            if let Some(workspace_mode) = intent.metadata.get("workspace_mode") {
                config.workspace_mode = match workspace_mode.as_str() {
                    Some("root_only") => WorkspaceMode::RootOnly,
                    Some("all_members") => WorkspaceMode::AllMembers,
                    Some("root_and_members") => WorkspaceMode::RootAndMembers,
                    Some("selected_members") => WorkspaceMode::SelectedMembers,
                    _ => WorkspaceMode::RootAndMembers,
                };
            }
            
            if let Some(member_filter) = intent.metadata.get("member_filter") {
                if let Some(members) = member_filter.as_array() {
                    config.member_filter = members.iter()
                        .filter_map(|m| m.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                        .into();
                }
            }
            
            if let Some(exclude_members) = intent.metadata.get("exclude_members") {
                if let Some(members) = exclude_members.as_array() {
                    config.exclude_members = members.iter()
                        .filter_map(|m| m.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                        .into();
                }
            }
        }
        
        config
    }
    
    /// Detect workspace information from a Cargo.toml file
    async fn detect_workspace(&self, cargo_file: &str) -> Result<Option<WorkspaceInfo>, ActionError> {
        let project_dir = std::path::Path::new(cargo_file)
            .parent()
            .ok_or_else(|| ActionError::Validation("Invalid Cargo.toml path".to_string()))?;
        
        // Read and parse Cargo.toml
        let cargo_content = tokio::fs::read_to_string(cargo_file).await
            .map_err(|e| ActionError::Validation(format!("Failed to read Cargo.toml: {}", e)))?;
        
        // Simple TOML parsing for workspace detection
        if !cargo_content.contains("[workspace]") {
            return Ok(None); // Not a workspace
        }
        
        let mut workspace_info = WorkspaceInfo {
            root_path: project_dir.to_string_lossy().to_string(),
            members: Vec::new(),
            workspace_config: None,
        };
        
        // Extract workspace members
        if let Some(members_section) = self.extract_workspace_members(&cargo_content) {
            for member_path in members_section {
                let member_cargo_path = project_dir.join(&member_path).join("Cargo.toml");
                if member_cargo_path.exists() {
                    if let Ok(member_info) = self.get_package_info(&member_cargo_path).await {
                        workspace_info.members.push(WorkspaceMember {
                            name: member_info.name,
                            path: member_path,
                            package_type: member_info.package_type,
                        });
                    }
                }
            }
        }
        
        // Extract workspace configuration
        workspace_info.workspace_config = self.extract_workspace_config(&cargo_content);
        
        Ok(Some(workspace_info))
    }
    
    /// Extract workspace members from Cargo.toml content
    fn extract_workspace_members(&self, cargo_content: &str) -> Option<Vec<String>> {
        let mut members = Vec::new();
        let lines: Vec<&str> = cargo_content.lines().collect();
        
        let mut in_workspace_section = false;
        let mut in_members_section = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed == "[workspace]" {
                in_workspace_section = true;
                continue;
            }
            
            if in_workspace_section && trimmed == "members = [" {
                in_members_section = true;
                continue;
            }
            
            if in_members_section {
                if trimmed == "]" {
                    break;
                }
                
                // Skip empty lines and comments
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                
                // Extract member path from quoted string
                if trimmed.starts_with('"') {
                    let end_quote = trimmed[1..].find('"');
                    if let Some(end_pos) = end_quote {
                        let member = trimmed[1..end_pos+1].to_string();
                        members.push(member);
                    }
                }
            }
            
            // Exit workspace section if we encounter another section
            if in_workspace_section && trimmed.starts_with('[') && trimmed != "[workspace]" {
                in_workspace_section = false;
                in_members_section = false;
            }
        }
        
        if members.is_empty() {
            None
        } else {
            Some(members)
        }
    }
    
    /// Extract workspace configuration from Cargo.toml content
    fn extract_workspace_config(&self, cargo_content: &str) -> Option<Value> {
        // Simple extraction of workspace configuration
        // In a real implementation, you'd use a proper TOML parser
        let mut config = serde_json::Map::new();
        
        if cargo_content.contains("resolver = \"2\"") {
            config.insert("resolver".to_string(), Value::String("2".to_string()));
        }
        
        if !config.is_empty() {
            Some(Value::Object(config))
        } else {
            None
        }
    }
    
    /// Get package information from a Cargo.toml file
    async fn get_package_info(&self, cargo_path: &std::path::Path) -> Result<PackageInfo, ActionError> {
        let content = tokio::fs::read_to_string(cargo_path).await
            .map_err(|e| ActionError::Validation(format!("Failed to read Cargo.toml: {}", e)))?;
        
        let mut name = String::new();
        let mut has_lib = false;
        let mut has_bin = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("name = ") {
                if let Some(n) = trimmed.strip_prefix("name = ") {
                    name = n.trim_matches('"').to_string();
                }
            }
            
            if trimmed == "[lib]" {
                has_lib = true;
            }
            
            if trimmed == "[[bin]]" {
                has_bin = true;
            }
        }
        
        let package_type = match (has_lib, has_bin) {
            (true, true) => PackageType::Both,
            (true, false) => PackageType::Library,
            (false, true) => PackageType::Binary,
            (false, false) => PackageType::Unknown,
        };
        
        Ok(PackageInfo { name, package_type })
    }
    
    /// Run cargo commands with workspace support
    async fn run_cargo_commands_with_workspace(&self, cargo_file: &str, config: &CargoConfig) -> ActionResult<Vec<CargoResult>> {
        let project_dir = std::path::Path::new(cargo_file)
            .parent()
            .ok_or_else(|| ActionError::Validation("Invalid Cargo.toml path".to_string()))?;
        
        // Detect if this is a workspace
        let workspace_info = self.detect_workspace(cargo_file).await?;
        
        let mut results = Vec::new();
        
        match &workspace_info {
            Some(workspace) => {
                // Handle workspace execution
                match config.workspace_mode {
                    WorkspaceMode::RootOnly => {
                        // Execute only on workspace root
                        for command in &config.commands {
                            match self.execute_cargo_command(project_dir, command, config).await {
                                Ok(result) => results.push(result),
                                Err(e) => {
                                    error!("Failed to execute {:?} for workspace root: {}", command, e);
                                    results.push(CargoResult {
                                        command: command.clone(),
                                        success: false,
                                        output: String::new(),
                                        errors: vec![e.to_string()],
                                        warnings: vec![],
                                        duration: std::time::Duration::ZERO,
                                    });
                                }
                            }
                        }
                    },
                    WorkspaceMode::AllMembers => {
                        // Execute on all workspace members
                        for member in &workspace.members {
                            let member_path = project_dir.join(&member.path);
                            for command in &config.commands {
                                match self.execute_cargo_command(&member_path, command, config).await {
                                    Ok(mut result) => {
                                        result.output = format!("[{}] {}", member.name, result.output);
                                        results.push(result);
                                    },
                                    Err(e) => {
                                        error!("Failed to execute {:?} for member {}: {}", command, member.name, e);
                                        results.push(CargoResult {
                                            command: command.clone(),
                                            success: false,
                                            output: format!("[{}] Failed", member.name),
                                            errors: vec![format!("{}: {}", member.name, e)],
                                            warnings: vec![],
                                            duration: std::time::Duration::ZERO,
                                        });
                                    }
                                }
                            }
                        }
                    },
                    WorkspaceMode::RootAndMembers => {
                        // Execute on workspace root and all members
                        // First, execute on root
                        for command in &config.commands {
                            match self.execute_cargo_command(project_dir, command, config).await {
                                Ok(mut result) => {
                                    result.output = format!("[workspace] {}", result.output);
                                    results.push(result);
                                },
                                Err(e) => {
                                    error!("Failed to execute {:?} for workspace root: {}", command, e);
                                    results.push(CargoResult {
                                        command: command.clone(),
                                        success: false,
                                        output: "[workspace] Failed".to_string(),
                                        errors: vec![format!("workspace: {}", e)],
                                        warnings: vec![],
                                        duration: std::time::Duration::ZERO,
                                    });
                                }
                            }
                        }
                        
                        // Then execute on members
                        for member in &workspace.members {
                            let member_path = project_dir.join(&member.path);
                            for command in &config.commands {
                                match self.execute_cargo_command(&member_path, command, config).await {
                                    Ok(mut result) => {
                                        result.output = format!("[{}] {}", member.name, result.output);
                                        results.push(result);
                                    },
                                    Err(e) => {
                                        error!("Failed to execute {:?} for member {}: {}", command, member.name, e);
                                        results.push(CargoResult {
                                            command: command.clone(),
                                            success: false,
                                            output: format!("[{}] Failed", member.name),
                                            errors: vec![format!("{}: {}", member.name, e)],
                                            warnings: vec![],
                                            duration: std::time::Duration::ZERO,
                                        });
                                    }
                                }
                            }
                        }
                    },
                    WorkspaceMode::SelectedMembers => {
                        // Execute only on selected members
                        let selected_members = self.get_selected_members(&workspace.members, config);
                        for member in selected_members {
                            let member_path = project_dir.join(&member.path);
                            for command in &config.commands {
                                match self.execute_cargo_command(&member_path, command, config).await {
                                    Ok(mut result) => {
                                        result.output = format!("[{}] {}", member.name, result.output);
                                        results.push(result);
                                    },
                                    Err(e) => {
                                        error!("Failed to execute {:?} for member {}: {}", command, member.name, e);
                                        results.push(CargoResult {
                                            command: command.clone(),
                                            success: false,
                                            output: format!("[{}] Failed", member.name),
                                            errors: vec![format!("{}: {}", member.name, e)],
                                            warnings: vec![],
                                            duration: std::time::Duration::ZERO,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            },
            None => {
                // Not a workspace, execute normally
                for command in &config.commands {
                    match self.execute_cargo_command(project_dir, command, config).await {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            error!("Failed to execute {:?} for {}: {}", command, cargo_file, e);
                            results.push(CargoResult {
                                command: command.clone(),
                                success: false,
                                output: String::new(),
                                errors: vec![e.to_string()],
                                warnings: vec![],
                                duration: std::time::Duration::ZERO,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Run transformation commands with workspace support
    async fn run_transformation_commands_with_workspace(&self, cargo_file: &str, config: &CargoConfig) -> ActionResult<Vec<CargoResult>> {
        let project_dir = std::path::Path::new(cargo_file)
            .parent()
            .ok_or_else(|| ActionError::Validation("Invalid Cargo.toml path".to_string()))?;
        
        // Detect if this is a workspace
        let workspace_info = self.detect_workspace(cargo_file).await?;
        
        let mut results = Vec::new();
        
        match &workspace_info {
            Some(workspace) => {
                // Handle workspace transformation
                match config.workspace_mode {
                    WorkspaceMode::RootOnly => {
                        // Transform only workspace root
                        if config.commands.contains(&CargoCommand::Fmt) {
                            match self.execute_cargo_fmt(project_dir, config).await {
                                Ok(mut result) => {
                                    result.output = format!("[workspace] {}", result.output);
                                    results.push(result);
                                },
                                Err(e) => {
                                    error!("Failed to execute fmt for workspace root: {}", e);
                                    results.push(CargoResult {
                                        command: CargoCommand::Fmt,
                                        success: false,
                                        output: "[workspace] Failed".to_string(),
                                        errors: vec![format!("workspace: {}", e)],
                                        warnings: vec![],
                                        duration: std::time::Duration::ZERO,
                                    });
                                }
                            }
                        }
                        
                        if config.commands.contains(&CargoCommand::Clippy) {
                            match self.execute_cargo_clippy_fix(project_dir, config).await {
                                Ok(mut result) => {
                                    result.output = format!("[workspace] {}", result.output);
                                    results.push(result);
                                },
                                Err(e) => {
                                    error!("Failed to execute clippy fix for workspace root: {}", e);
                                    results.push(CargoResult {
                                        command: CargoCommand::Clippy,
                                        success: false,
                                        output: "[workspace] Failed".to_string(),
                                        errors: vec![format!("workspace: {}", e)],
                                        warnings: vec![],
                                        duration: std::time::Duration::ZERO,
                                    });
                                }
                            }
                        }
                    },
                    WorkspaceMode::AllMembers | WorkspaceMode::RootAndMembers => {
                        // Transform all workspace members
                        for member in &workspace.members {
                            let member_path = project_dir.join(&member.path);
                            
                            if config.commands.contains(&CargoCommand::Fmt) {
                                match self.execute_cargo_fmt(&member_path, config).await {
                                    Ok(mut result) => {
                                        result.output = format!("[{}] {}", member.name, result.output);
                                        results.push(result);
                                    },
                                    Err(e) => {
                                        error!("Failed to execute fmt for member {}: {}", member.name, e);
                                        results.push(CargoResult {
                                            command: CargoCommand::Fmt,
                                            success: false,
                                            output: format!("[{}] Failed", member.name),
                                            errors: vec![format!("{}: {}", member.name, e)],
                                            warnings: vec![],
                                            duration: std::time::Duration::ZERO,
                                        });
                                    }
                                }
                            }
                            
                            if config.commands.contains(&CargoCommand::Clippy) {
                                match self.execute_cargo_clippy_fix(&member_path, config).await {
                                    Ok(mut result) => {
                                        result.output = format!("[{}] {}", member.name, result.output);
                                        results.push(result);
                                    },
                                    Err(e) => {
                                        error!("Failed to execute clippy fix for member {}: {}", member.name, e);
                                        results.push(CargoResult {
                                            command: CargoCommand::Clippy,
                                            success: false,
                                            output: format!("[{}] Failed", member.name),
                                            errors: vec![format!("{}: {}", member.name, e)],
                                            warnings: vec![],
                                            duration: std::time::Duration::ZERO,
                                        });
                                    }
                                }
                            }
                        }
                    },
                    WorkspaceMode::SelectedMembers => {
                        // Transform only selected members
                        let selected_members = self.get_selected_members(&workspace.members, config);
                        for member in selected_members {
                            let member_path = project_dir.join(&member.path);
                            
                            if config.commands.contains(&CargoCommand::Fmt) {
                                match self.execute_cargo_fmt(&member_path, config).await {
                                    Ok(mut result) => {
                                        result.output = format!("[{}] {}", member.name, result.output);
                                        results.push(result);
                                    },
                                    Err(e) => {
                                        error!("Failed to execute fmt for member {}: {}", member.name, e);
                                        results.push(CargoResult {
                                            command: CargoCommand::Fmt,
                                            success: false,
                                            output: format!("[{}] Failed", member.name),
                                            errors: vec![format!("{}: {}", member.name, e)],
                                            warnings: vec![],
                                            duration: std::time::Duration::ZERO,
                                        });
                                    }
                                }
                            }
                            
                            if config.commands.contains(&CargoCommand::Clippy) {
                                match self.execute_cargo_clippy_fix(&member_path, config).await {
                                    Ok(mut result) => {
                                        result.output = format!("[{}] {}", member.name, result.output);
                                        results.push(result);
                                    },
                                    Err(e) => {
                                        error!("Failed to execute clippy fix for member {}: {}", member.name, e);
                                        results.push(CargoResult {
                                            command: CargoCommand::Clippy,
                                            success: false,
                                            output: format!("[{}] Failed", member.name),
                                            errors: vec![format!("{}: {}", member.name, e)],
                                            warnings: vec![],
                                            duration: std::time::Duration::ZERO,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            },
            None => {
                // Not a workspace, execute normally
                if config.commands.contains(&CargoCommand::Fmt) {
                    match self.execute_cargo_fmt(project_dir, config).await {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            error!("Failed to execute fmt for {}: {}", cargo_file, e);
                            results.push(CargoResult {
                                command: CargoCommand::Fmt,
                                success: false,
                                output: String::new(),
                                errors: vec![e.to_string()],
                                warnings: vec![],
                                duration: std::time::Duration::ZERO,
                            });
                        }
                    }
                }
                
                if config.commands.contains(&CargoCommand::Clippy) {
                    match self.execute_cargo_clippy_fix(project_dir, config).await {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            error!("Failed to execute clippy fix for {}: {}", cargo_file, e);
                            results.push(CargoResult {
                                command: CargoCommand::Clippy,
                                success: false,
                                output: String::new(),
                                errors: vec![e.to_string()],
                                warnings: vec![],
                                duration: std::time::Duration::ZERO,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Get selected members based on filter and exclude configuration
    fn get_selected_members<'a>(&self, members: &'a [WorkspaceMember], config: &CargoConfig) -> Vec<&'a WorkspaceMember> {
        members.iter()
            .filter(|member| {
                // Apply member filter
                if let Some(ref filter) = config.member_filter {
                    if !filter.contains(&member.name) {
                        return false;
                    }
                }
                
                // Apply exclude filter
                if let Some(ref exclude) = config.exclude_members {
                    if exclude.contains(&member.name) {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /// Run cargo commands for validation (legacy method)
    async fn run_cargo_commands(&self, cargo_file: &str, config: &CargoConfig) -> ActionResult<Vec<CargoResult>> {
        self.run_cargo_commands_with_workspace(cargo_file, config).await
    }
    
    /// Run transformation commands (legacy method)
    async fn run_transformation_commands(&self, cargo_file: &str, config: &CargoConfig) -> ActionResult<Vec<CargoResult>> {
        self.run_transformation_commands_with_workspace(cargo_file, config).await
    }
    
    /// Execute a specific cargo command
    async fn execute_cargo_command(&self, project_dir: &std::path::Path, command: &CargoCommand, config: &CargoConfig) -> ActionResult<CargoResult> {
        let start = std::time::Instant::now();
        
        let (_cmd, args) = self.build_command_args(command, config);
        
        let output = tokio::process::Command::new("cargo")
            .args(&args)
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution { 
                tool: "cargo".to_string(), 
                message: format!("Failed to run cargo {:?}: {}", command, e) 
            })?;
        
        let (errors, warnings) = self.parse_cargo_output(&output, command, config);
        let success = output.status.success() && errors.is_empty();
        
        Ok(CargoResult {
            command: command.clone(),
            success,
            output: String::from_utf8_lossy(&output.stdout).to_string(),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    /// Execute cargo fmt for formatting
    async fn execute_cargo_fmt(&self, project_dir: &std::path::Path, config: &CargoConfig) -> ActionResult<CargoResult> {
        let start = std::time::Instant::now();
        
        let mut args = vec!["fmt"];
        if config.json_output {
            args.push("--message-format=json");
        }
        if config.verbose {
            args.push("--verbose");
        }
        
        let output = tokio::process::Command::new("cargo")
            .args(&args)
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution { 
                tool: "cargo".to_string(), 
                message: format!("Failed to run cargo fmt: {}", e) 
            })?;
        
        let (errors, warnings) = self.parse_cargo_output(&output, &CargoCommand::Fmt, config);
        let success = output.status.success() && errors.is_empty();
        
        Ok(CargoResult {
            command: CargoCommand::Fmt,
            success,
            output: "Code formatting completed".to_string(),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    /// Execute cargo clippy with auto-fix
    async fn execute_cargo_clippy_fix(&self, project_dir: &std::path::Path, config: &CargoConfig) -> ActionResult<CargoResult> {
        let start = std::time::Instant::now();
        
        let mut args = vec!["clippy", "--fix"];
        if config.json_output {
            args.push("--message-format=json");
        }
        if config.verbose {
            args.push("--verbose");
        }
        
        let output = tokio::process::Command::new("cargo")
            .args(&args)
            .current_dir(project_dir)
            .output()
            .await
            .map_err(|e| ActionError::ToolExecution { 
                tool: "cargo".to_string(), 
                message: format!("Failed to run cargo clippy --fix: {}", e) 
            })?;
        
        let (errors, warnings) = self.parse_cargo_output(&output, &CargoCommand::Clippy, config);
        let success = output.status.success() && errors.is_empty();
        
        Ok(CargoResult {
            command: CargoCommand::Clippy,
            success,
            output: "Clippy auto-fix completed".to_string(),
            errors,
            warnings,
            duration: start.elapsed(),
        })
    }
    
    /// Build command arguments for a cargo command
    fn build_command_args(&self, command: &CargoCommand, config: &CargoConfig) -> (String, Vec<&'static str>) {
        let mut args = Vec::new();
        
        match command {
            CargoCommand::Check => {
                args.push("check");
                if config.json_output {
                    args.push("--message-format=json");
                }
            },
            CargoCommand::Build => {
                args.push("build");
                if config.json_output {
                    args.push("--message-format=json");
                }
            },
            CargoCommand::Test => {
                args.push("test");
                if config.json_output {
                    args.push("--message-format=json");
                }
            },
            CargoCommand::Clippy => {
                args.push("clippy");
                if config.json_output {
                    args.push("--message-format=json");
                }
            },
            CargoCommand::Audit => {
                args.push("audit");
                if config.json_output {
                    args.push("--output-format=json");
                }
            },
            CargoCommand::Outdated => {
                args.push("outdated");
                if config.json_output {
                    args.push("--format=json");
                }
            },
            CargoCommand::Fmt => {
                args.push("fmt");
                if config.json_output {
                    args.push("--message-format=json");
                }
            },
        }
        
        if config.verbose {
            args.push("--verbose");
        }
        
        ("cargo".to_string(), args)
    }
    
    /// Parse cargo output for errors and warnings
    fn parse_cargo_output(&self, output: &std::process::Output, _command: &CargoCommand, config: &CargoConfig) -> (Vec<String>, Vec<String>) {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        if config.json_output {
            // Try to parse JSON output
            if let Ok(json_str) = String::from_utf8(output.stdout.clone()) {
                for line in json_str.lines() {
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        if let Some(message) = json.get("message") {
                            if let Some(level) = message.get("level") {
                                let msg = message.get("message").and_then(|m| m.as_str()).unwrap_or("");
                                let span = message.get("spans").and_then(|s| s.get(0));
                                let file = span.and_then(|s| s.get("file_name")).and_then(|f| f.as_str()).unwrap_or("");
                                let line = span.and_then(|s| s.get("line_start")).and_then(|l| l.as_u64()).unwrap_or(0);
                                
                                let formatted_msg = format!("{}:{}: {}", file, line, msg);
                                
                                match level.as_str() {
                                    Some("error") => errors.push(formatted_msg),
                                    Some("warning") => warnings.push(formatted_msg),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Also parse stderr for non-JSON output
        let stderr = String::from_utf8_lossy(&output.stderr);
        for line in stderr.lines() {
            if line.contains("error:") {
                errors.push(line.to_string());
            } else if line.contains("warning:") {
                warnings.push(line.to_string());
            }
        }
        
        (errors, warnings)
    }
}

/// Package information structure
#[derive(Debug, Clone)]
struct PackageInfo {
    name: String,
    package_type: PackageType,
}

#[cfg(test)]
mod tests; 