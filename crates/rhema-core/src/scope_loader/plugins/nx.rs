use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::scope::Scope;
use super::super::plugin::ScopeLoaderPlugin;
use super::super::types::*;

/// Configuration for the Nx plugin
#[derive(Debug, Clone)]
pub struct NxPluginConfig {
    pub detect_workspaces: bool,
    pub include_dev_dependencies: bool,
    pub min_project_size: usize,
    pub max_depth: usize,
    pub detect_affected: bool,
}

impl Default for NxPluginConfig {
    fn default() -> Self {
        Self {
            detect_workspaces: true,
            include_dev_dependencies: true,
            min_project_size: 500,
            max_depth: 5,
            detect_affected: true,
        }
    }
}

/// Plugin for detecting Nx monorepo structures
pub struct NxPlugin {
    config: NxPluginConfig,
}

impl NxPlugin {
    /// Create a new Nx plugin with default configuration
    pub fn new() -> Self {
        Self {
            config: NxPluginConfig::default(),
        }
    }

    /// Create a new Nx plugin with custom configuration
    pub fn with_config(config: NxPluginConfig) -> Self {
        Self { config }
    }

    /// Parse nx.json file
    fn parse_nx_json(&self, path: &Path) -> Result<serde_json::Value, PluginError> {
        let nx_json_path = path.join("nx.json");
        if !nx_json_path.exists() {
            return Err(PluginError::PluginNotFound("nx.json not found".to_string()));
        }

        let content = std::fs::read_to_string(&nx_json_path)
            .map_err(|e| PluginError::IoError(e))?;

        let nx_json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| PluginError::JsonError(e))?;

        Ok(nx_json)
    }

    /// Parse package.json for Nx workspace
    fn parse_workspace_package_json(&self, path: &Path) -> Result<PackageInfo, PluginError> {
        let package_json_path = path.join("package.json");
        if !package_json_path.exists() {
            return Err(PluginError::PluginNotFound("package.json not found".to_string()));
        }

        let content = std::fs::read_to_string(&package_json_path)
            .map_err(|e| PluginError::IoError(e))?;

        let package_json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| PluginError::JsonError(e))?;

        let name = package_json["name"]
            .as_str()
            .unwrap_or("nx-workspace")
            .to_string();

        let version = package_json["version"]
            .as_str()
            .unwrap_or("0.0.0")
            .to_string();

        let description = package_json["description"].as_str().map(|s| s.to_string());
        let author = package_json["author"].as_str().map(|s| s.to_string());
        let license = package_json["license"].as_str().map(|s| s.to_string());
        let repository = package_json["repository"]["url"].as_str().map(|s| s.to_string());

        Ok(PackageInfo {
            name,
            version,
            description,
            author,
            license,
            repository,
        })
    }

    /// Get Nx projects from workspace
    fn get_nx_projects(&self, path: &Path) -> Result<Vec<String>, PluginError> {
        let nx_json = self.parse_nx_json(path)?;
        
        let mut projects = Vec::new();

        // Check for projects in nx.json
        if let Some(projects_obj) = nx_json["projects"].as_object() {
            for (project_name, _) in projects_obj {
                projects.push(project_name.clone());
            }
        }

        // Also check for projects in package.json workspaces
        let package_json_path = path.join("package.json");
        if package_json_path.exists() {
            let content = std::fs::read_to_string(&package_json_path)
                .map_err(|e| PluginError::IoError(e))?;
            let package_json: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| PluginError::JsonError(e))?;

            if let Some(workspaces) = package_json["workspaces"].as_array() {
                for workspace in workspaces {
                    if let Some(workspace_str) = workspace.as_str() {
                        // Extract project name from workspace path
                        if let Some(project_name) = workspace_str.split('/').last() {
                            if !projects.contains(&project_name.to_string()) {
                                projects.push(project_name.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(projects)
    }

    /// Parse project configuration
    fn parse_project_config(&self, workspace_path: &Path, project_name: &str) -> Result<HashMap<String, serde_json::Value>, PluginError> {
        let mut config = HashMap::new();

        // Try to find project.json
        let project_json_path = workspace_path.join(project_name).join("project.json");
        if project_json_path.exists() {
            let content = std::fs::read_to_string(&project_json_path)
                .map_err(|e| PluginError::IoError(e))?;
            let project_json: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| PluginError::JsonError(e))?;
            
            config.insert("project_type".to_string(), project_json["projectType"].clone());
            config.insert("source_root".to_string(), project_json["sourceRoot"].clone());
            config.insert("root".to_string(), project_json["root"].clone());
            
            if let Some(targets) = project_json["targets"].as_object() {
                let target_names: Vec<String> = targets.keys().cloned().collect();
                config.insert("targets".to_string(), serde_json::Value::Array(
                    target_names.into_iter().map(|s| serde_json::Value::String(s)).collect()
                ));
            }
        }

        // Try to find package.json for the project
        let project_package_json = workspace_path.join(project_name).join("package.json");
        if project_package_json.exists() {
            let content = std::fs::read_to_string(&project_package_json)
                .map_err(|e| PluginError::IoError(e))?;
            let package_json: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| PluginError::JsonError(e))?;
            
            config.insert("package_name".to_string(), package_json["name"].clone());
            config.insert("package_version".to_string(), package_json["version"].clone());
        }

        Ok(config)
    }

    /// Discover Nx project files
    fn discover_nx_project_files(&self, workspace_path: &Path, project_name: &str) -> Result<Vec<PathBuf>, PluginError> {
        let mut files = Vec::new();
        let project_path = workspace_path.join(project_name);
        
        if !project_path.exists() {
            return Ok(files);
        }

        for entry in WalkDir::new(&project_path)
            .max_depth(self.config.max_depth)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();
            
            // Include common source files
            if let Some(ext) = entry_path.extension() {
                match ext.to_str() {
                    Some("ts") | Some("js") | Some("tsx") | Some("jsx") | Some("json") | Some("yaml") | Some("yml") => {
                        files.push(entry_path.to_path_buf());
                    }
                    _ => {}
                }
            }
            
            // Include project configuration files
            if let Some(name) = entry_path.file_name() {
                match name.to_str() {
                    Some("project.json") | Some("package.json") | Some("tsconfig.json") | Some("jest.config.js") | Some("vite.config.ts") => {
                        files.push(entry_path.to_path_buf());
                    }
                    _ => {}
                }
            }
        }

        Ok(files)
    }

    /// Discover workspace files
    fn discover_workspace_files(&self, path: &Path) -> Result<Vec<PathBuf>, PluginError> {
        let mut files = Vec::new();
        
        // Add workspace configuration files
        let workspace_files = ["nx.json", "package.json", "tsconfig.base.json", "jest.preset.js"];
        for file_name in &workspace_files {
            let file_path = path.join(file_name);
            if file_path.exists() {
                files.push(file_path);
            }
        }

        // Add lock files
        let lock_files = ["package-lock.json", "yarn.lock", "pnpm-lock.yaml"];
        for lock_file in &lock_files {
            let lock_path = path.join(lock_file);
            if lock_path.exists() {
                files.push(lock_path);
            }
        }

        // Add workspace configuration
        let pnpm_workspace = path.join("pnpm-workspace.yaml");
        if pnpm_workspace.exists() {
            files.push(pnpm_workspace);
        }

        Ok(files)
    }

    /// Parse Nx metadata
    fn parse_nx_metadata(&self, path: &Path) -> Result<HashMap<String, serde_json::Value>, PluginError> {
        let mut metadata = HashMap::new();
        
        // Add basic metadata
        metadata.insert("package_manager".to_string(), serde_json::Value::String("nx".to_string()));
        metadata.insert("is_workspace".to_string(), serde_json::Value::Bool(true));
        
        // Add Nx-specific metadata
        if let Ok(nx_json) = self.parse_nx_json(path) {
            metadata.insert("nx_version".to_string(), nx_json["npmScope"].clone());
            
            if let Some(projects) = nx_json["projects"].as_object() {
                metadata.insert("project_count".to_string(), serde_json::Value::Number(serde_json::Number::from(projects.len())));
            }
        }
        
        // Add project list
        if let Ok(projects) = self.get_nx_projects(path) {
            let projects_array: Vec<serde_json::Value> = projects
                .iter()
                .map(|p| serde_json::Value::String(p.clone()))
                .collect();
            metadata.insert("projects".to_string(), serde_json::Value::Array(projects_array));
        }
        
        Ok(metadata)
    }
}

impl ScopeLoaderPlugin for NxPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "nx".to_string(),
            version: "1.0.0".to_string(),
            description: "Detects Nx monorepo structures and projects".to_string(),
            supported_package_managers: vec!["nx".to_string()],
            priority: 95,
        }
    }

    fn can_handle(&self, path: &Path) -> bool {
        path.join("nx.json").exists() || path.join(".nx").exists()
    }

    fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError> {
        let mut boundaries = Vec::new();

        // Create workspace boundary
        let workspace_info = self.parse_workspace_package_json(path)?;
        let workspace_metadata = self.parse_nx_metadata(path)?;
        let projects = self.get_nx_projects(path)?;

        boundaries.push(PackageBoundary {
            path: path.to_path_buf(),
            package_manager: PackageManager::Nx,
            package_info: workspace_info,
            dependencies: Vec::new(), // Will be populated from package.json
            scripts: HashMap::new(), // Will be populated from package.json
            metadata: workspace_metadata,
        });

        // Create project boundaries
        for project_name in projects {
            let project_path = path.join(&project_name);
            if project_path.exists() {
                let project_config = self.parse_project_config(path, &project_name)?;
                
                boundaries.push(PackageBoundary {
                    path: project_path,
                    package_manager: PackageManager::Nx,
                    package_info: PackageInfo {
                        name: project_name,
                        version: "0.0.0".to_string(),
                        description: None,
                        author: None,
                        license: None,
                        repository: None,
                    },
                    dependencies: Vec::new(),
                    scripts: HashMap::new(),
                    metadata: project_config,
                });
            }
        }

        Ok(boundaries)
    }

    fn suggest_scopes(&self, boundaries: &[PackageBoundary]) -> Result<Vec<ScopeSuggestion>, PluginError> {
        let mut suggestions = Vec::new();

        for boundary in boundaries {
            if boundary.path.file_name().and_then(|s| s.to_str()) == Some("nx.json") {
                // This is the workspace root
                suggestions.push(ScopeSuggestion {
                    name: format!("{}_workspace", boundary.package_info.name),
                    path: boundary.path.clone(),
                    scope_type: ScopeType::Monorepo,
                    confidence: 0.95,
                    reasoning: "Detected Nx workspace configuration".to_string(),
                    files: self.discover_workspace_files(&boundary.path)?,
                    dependencies: Vec::new(),
                    metadata: boundary.metadata.clone(),
                });
            } else {
                // This is a project
                let project_name = boundary.package_info.name.clone();
                suggestions.push(ScopeSuggestion {
                    name: project_name.clone(),
                    path: boundary.path.clone(),
                    scope_type: ScopeType::Application, // Default to application, could be refined based on project type
                    confidence: 0.90,
                    reasoning: format!("Detected Nx project: {}", project_name),
                    files: self.discover_nx_project_files(&boundary.path.parent().unwrap(), &project_name)?,
                    dependencies: Vec::new(),
                    metadata: boundary.metadata.clone(),
                });
            }
        }

        Ok(suggestions)
    }

    fn create_scopes(&self, suggestions: &[ScopeSuggestion]) -> Result<Vec<Scope>, PluginError> {
        let mut scopes = Vec::new();

        for suggestion in suggestions {
            // Create the scope directory structure
            let scope_path = &suggestion.path;
            
            // Create .rhema directory if it doesn't exist
            let rhema_dir = scope_path.join(".rhema");
            if !rhema_dir.exists() {
                std::fs::create_dir_all(&rhema_dir)
                    .map_err(|e| PluginError::PluginExecutionFailed(format!("Failed to create .rhema directory: {}", e)))?;
            }

            // Create rhema.yaml file
            let rhema_content = self.generate_rhema_yaml(suggestion)?;
            let rhema_file = rhema_dir.join("rhema.yaml");
            std::fs::write(&rhema_file, rhema_content)
                .map_err(|e| PluginError::PluginExecutionFailed(format!("Failed to write rhema.yaml: {}", e)))?;

            // Create the scope
            match Scope::new(scope_path.clone()) {
                Ok(scope) => scopes.push(scope),
                Err(e) => {
                    eprintln!("Failed to create scope for {}: {}", suggestion.name, e);
                }
            }
        }

        Ok(scopes)
    }

    fn load_context(&self, scope: &Scope) -> Result<ScopeContext, PluginError> {
        let nx_json = scope.path.join("nx.json");
        if !nx_json.exists() {
            return Err(PluginError::PluginNotFound("nx.json not found".to_string()));
        }

        let metadata = self.parse_nx_metadata(&scope.path)?;
        let _projects = self.get_nx_projects(&scope.path)?;

        Ok(ScopeContext {
            scope_name: scope.definition.name.clone(),
            package_manager: PackageManager::Nx,
            dependencies: Vec::new(), // Could be populated from workspace package.json
            scripts: HashMap::new(), // Could be populated from workspace package.json
            metadata,
        })
    }
}

impl NxPlugin {
    /// Generate rhema.yaml content for a scope suggestion
    fn generate_rhema_yaml(&self, suggestion: &ScopeSuggestion) -> Result<String, PluginError> {
        let mut mapping = serde_yaml::Mapping::new();
        
        // Add scope name
        mapping.insert(
            serde_yaml::Value::String("name".to_string()),
            serde_yaml::Value::String(suggestion.name.clone()),
        );

        // Add scope type
        mapping.insert(
            serde_yaml::Value::String("type".to_string()),
            serde_yaml::Value::String(suggestion.scope_type.as_str().to_string()),
        );

        // Add description
        mapping.insert(
            serde_yaml::Value::String("description".to_string()),
            serde_yaml::Value::String(suggestion.reasoning.clone()),
        );

        // Add confidence
        mapping.insert(
            serde_yaml::Value::String("confidence".to_string()),
            serde_yaml::Value::Number(serde_yaml::Number::from(suggestion.confidence as i64)),
        );

        // Add package manager
        mapping.insert(
            serde_yaml::Value::String("package_manager".to_string()),
            serde_yaml::Value::String("nx".to_string()),
        );

        // Add dependencies if any
        if !suggestion.dependencies.is_empty() {
            let deps: Vec<serde_yaml::Value> = suggestion.dependencies
                .iter()
                .map(|d| serde_yaml::Value::String(d.clone()))
                .collect();
            mapping.insert(
                serde_yaml::Value::String("dependencies".to_string()),
                serde_yaml::Value::Sequence(deps),
            );
        }

        // Add metadata if any
        if !suggestion.metadata.is_empty() {
            let metadata_value = serde_yaml::to_value(&suggestion.metadata)
                .map_err(|e| PluginError::PluginExecutionFailed(format!("Failed to serialize metadata: {}", e)))?;
            mapping.insert(
                serde_yaml::Value::String("metadata".to_string()),
                metadata_value,
            );
        }

        serde_yaml::to_string(&mapping)
            .map_err(|e| PluginError::PluginExecutionFailed(format!("Failed to serialize YAML: {}", e)))
    }
}

impl Default for NxPlugin {
    fn default() -> Self {
        Self::new()
    }
}
