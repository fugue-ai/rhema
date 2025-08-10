use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::scope::Scope;
use super::super::plugin::ScopeLoaderPlugin;
use super::super::types::*;

/// Configuration for the Node.js plugin
#[derive(Debug, Clone)]
pub struct NodePluginConfig {
    pub detect_workspaces: bool,
    pub include_dev_dependencies: bool,
    pub min_package_size: usize,
    pub max_depth: usize,
    pub detect_pnpm: bool,
    pub detect_yarn: bool,
    pub detect_npm: bool,
}

impl Default for NodePluginConfig {
    fn default() -> Self {
        Self {
            detect_workspaces: true,
            include_dev_dependencies: true,
            min_package_size: 1000,
            max_depth: 5,
            detect_pnpm: true,
            detect_yarn: true,
            detect_npm: true,
        }
    }
}

/// Plugin for detecting Node.js packages (npm, yarn, pnpm)
pub struct NodePackagePlugin {
    config: NodePluginConfig,
}

impl NodePackagePlugin {
    /// Create a new Node.js plugin with default configuration
    pub fn new() -> Self {
        Self {
            config: NodePluginConfig::default(),
        }
    }

    /// Create a new Node.js plugin with custom configuration
    pub fn with_config(config: NodePluginConfig) -> Self {
        Self { config }
    }

    /// Detect the package manager type for a directory
    fn detect_package_manager(&self, path: &Path) -> Result<PackageManager, PluginError> {
        // Check for pnpm workspace
        if self.config.detect_pnpm && path.join("pnpm-workspace.yaml").exists() {
            return Ok(PackageManager::Pnpm);
        }

        // Check for yarn workspace
        if self.config.detect_yarn && path.join("yarn.lock").exists() {
            return Ok(PackageManager::Yarn);
        }

        // Check for pnpm lock
        if self.config.detect_pnpm && path.join("pnpm-lock.yaml").exists() {
            return Ok(PackageManager::Pnpm);
        }

        // Check for npm lock
        if self.config.detect_npm && path.join("package-lock.json").exists() {
            return Ok(PackageManager::Npm);
        }

        // Default to npm if package.json exists
        if path.join("package.json").exists() {
            return Ok(PackageManager::Npm);
        }

        Err(PluginError::UnsupportedPackageManager("No package manager detected".to_string()))
    }

    /// Parse a package.json file
    fn parse_package_json(&self, path: &Path) -> Result<PackageInfo, PluginError> {
        // Check if the path is actually a file
        if !path.is_file() {
            return Err(PluginError::InvalidPackageConfig(
                format!("Path is not a file: {}", path.display())
            ));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| PluginError::IoError(e))?;

        let package_json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| PluginError::JsonError(e))?;

        let name = package_json["name"]
            .as_str()
            .ok_or_else(|| PluginError::InvalidPackageConfig("Missing 'name' field".to_string()))?
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

    /// Parse dependencies from package.json
    fn parse_dependencies(&self, path: &Path) -> Result<Vec<Dependency>, PluginError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PluginError::IoError(e))?;

        let package_json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| PluginError::JsonError(e))?;

        let mut dependencies = Vec::new();

        // Parse runtime dependencies
        if let Some(deps) = package_json["dependencies"].as_object() {
            for (name, version) in deps {
                dependencies.push(Dependency {
                    name: name.clone(),
                    version: version.as_str().unwrap_or("*").to_string(),
                    dependency_type: DependencyType::Runtime,
                });
            }
        }

        // Parse dev dependencies if enabled
        if self.config.include_dev_dependencies {
            if let Some(dev_deps) = package_json["devDependencies"].as_object() {
                for (name, version) in dev_deps {
                    dependencies.push(Dependency {
                        name: name.clone(),
                        version: version.as_str().unwrap_or("*").to_string(),
                        dependency_type: DependencyType::Development,
                    });
                }
            }
        }

        // Parse peer dependencies
        if let Some(peer_deps) = package_json["peerDependencies"].as_object() {
            for (name, version) in peer_deps {
                dependencies.push(Dependency {
                    name: name.clone(),
                    version: version.as_str().unwrap_or("*").to_string(),
                    dependency_type: DependencyType::Peer,
                });
            }
        }

        Ok(dependencies)
    }

    /// Parse scripts from package.json
    fn parse_scripts(&self, path: &Path) -> Result<HashMap<String, String>, PluginError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PluginError::IoError(e))?;

        let package_json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| PluginError::JsonError(e))?;

        let mut scripts = HashMap::new();

        if let Some(scripts_obj) = package_json["scripts"].as_object() {
            for (name, script) in scripts_obj {
                if let Some(script_str) = script.as_str() {
                    scripts.insert(name.clone(), script_str.to_string());
                }
            }
        }

        Ok(scripts)
    }

    /// Parse metadata from package.json
    fn parse_metadata(&self, path: &Path) -> Result<HashMap<String, serde_json::Value>, PluginError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PluginError::IoError(e))?;

        let package_json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| PluginError::JsonError(e))?;

        let mut metadata = HashMap::new();

        // Add package manager info
        let package_manager = self.detect_package_manager(path)?;
        metadata.insert("package_manager".to_string(), serde_json::Value::String(package_manager.as_str().to_string()));

        // Add workspace info
        metadata.insert("is_workspace".to_string(), serde_json::Value::Bool(self.is_workspace(path)?));

        // Add private flag
        if let Some(private_flag) = package_json["private"].as_bool() {
            metadata.insert("private".to_string(), serde_json::Value::Bool(private_flag));
        }

        // Add keywords
        if let Some(keywords) = package_json["keywords"].as_array() {
            let keywords_vec: Vec<serde_json::Value> = keywords
                .iter()
                .filter_map(|k| k.as_str().map(|s| serde_json::Value::String(s.to_string())))
                .collect();
            metadata.insert("keywords".to_string(), serde_json::Value::Array(keywords_vec));
        }

        // Add engines
        if let Some(engines) = package_json["engines"].as_object() {
            metadata.insert("engines".to_string(), serde_json::Value::Object(engines.clone()));
        }

        Ok(metadata)
    }

    /// Check if a directory is a workspace
    fn is_workspace(&self, path: &Path) -> Result<bool, PluginError> {
        // Check for pnpm workspace
        if path.join("pnpm-workspace.yaml").exists() {
            return Ok(true);
        }

        // Check for yarn workspace
        if path.join("yarn.lock").exists() {
            let package_json = path.join("package.json");
            if package_json.exists() {
                let content = std::fs::read_to_string(&package_json)
                    .map_err(|e| PluginError::IoError(e))?;
                let package_json: serde_json::Value = serde_json::from_str(&content)
                    .map_err(|e| PluginError::JsonError(e))?;
                
                if let Some(workspaces) = package_json["workspaces"].as_array() {
                    return Ok(!workspaces.is_empty());
                }
            }
        }

        // Check for npm workspace
        if path.join("package-lock.json").exists() {
            let package_json = path.join("package.json");
            if package_json.exists() {
                let content = std::fs::read_to_string(&package_json)
                    .map_err(|e| PluginError::IoError(e))?;
                let package_json: serde_json::Value = serde_json::from_str(&content)
                    .map_err(|e| PluginError::JsonError(e))?;
                
                if let Some(workspaces) = package_json["workspaces"].as_array() {
                    return Ok(!workspaces.is_empty());
                }
            }
        }

        Ok(false)
    }

    /// Discover package files
    fn discover_package_files(&self, path: &Path) -> Result<Vec<PathBuf>, PluginError> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(path)
            .max_depth(self.config.max_depth)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();
            
            // Include JavaScript/TypeScript files
            if let Some(ext) = entry_path.extension() {
                match ext.to_str() {
                    Some("js") | Some("ts") | Some("jsx") | Some("tsx") | Some("mjs") | Some("cjs") => {
                        files.push(entry_path.to_path_buf());
                    }
                    _ => {}
                }
            }
            
            // Include package files
            if let Some(name) = entry_path.file_name() {
                match name.to_str() {
                    Some("package.json") | Some("package-lock.json") | Some("yarn.lock") | Some("pnpm-lock.yaml") | Some("pnpm-workspace.yaml") => {
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
        
        // Add workspace package.json
        let workspace_package_json = path.join("package.json");
        if workspace_package_json.exists() {
            files.push(workspace_package_json);
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

        // Look for member packages
        if self.is_workspace(path)? {
            for entry in WalkDir::new(path)
                .max_depth(2)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let entry_path = entry.path();
                if entry_path.is_dir() && entry_path.join("package.json").exists() {
                    files.push(entry_path.join("package.json"));
                }
            }
        }

        Ok(files)
    }
}

impl ScopeLoaderPlugin for NodePackagePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "node".to_string(),
            version: "1.0.0".to_string(),
            description: "Detects Node.js packages (npm, yarn, pnpm) and workspaces".to_string(),
            supported_package_managers: vec!["npm".to_string(), "yarn".to_string(), "pnpm".to_string()],
            priority: 100,
        }
    }

    fn can_handle(&self, path: &Path) -> bool {
        path.join("package.json").exists() || 
        path.join("yarn.lock").exists() || 
        path.join("pnpm-lock.yaml").exists() ||
        path.join("pnpm-workspace.yaml").exists()
    }

    fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError> {
        let mut boundaries = Vec::new();

        // Walk through the directory to find package.json files
        for entry in WalkDir::new(path)
            .max_depth(self.config.max_depth)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();
            
            if entry_path.file_name().and_then(|s| s.to_str()) == Some("package.json") {
                let package_info = self.parse_package_json(entry_path)?;
                let package_manager = self.detect_package_manager(entry_path.parent().unwrap())?;
                let dependencies = self.parse_dependencies(entry_path)?;
                let scripts = self.parse_scripts(entry_path)?;
                let metadata = self.parse_metadata(entry_path.parent().unwrap())?;

                boundaries.push(PackageBoundary {
                    path: entry_path.parent().unwrap().to_path_buf(),
                    package_manager,
                    package_info,
                    dependencies,
                    scripts,
                    metadata,
                });
            }
        }

        Ok(boundaries)
    }

    fn suggest_scopes(&self, boundaries: &[PackageBoundary]) -> Result<Vec<ScopeSuggestion>, PluginError> {
        let mut suggestions = Vec::new();

        for boundary in boundaries {
            // Create package scope
            suggestions.push(ScopeSuggestion {
                name: boundary.package_info.name.clone(),
                path: boundary.path.clone(),
                scope_type: ScopeType::Package,
                confidence: 0.95,
                reasoning: "Detected package.json with valid configuration".to_string(),
                files: self.discover_package_files(&boundary.path)?,
                dependencies: boundary.dependencies.iter().map(|d| d.name.clone()).collect(),
                metadata: boundary.metadata.clone(),
            });

            // Create workspace scope if applicable
            if self.is_workspace(&boundary.path)? {
                suggestions.push(ScopeSuggestion {
                    name: format!("{}_workspace", boundary.package_info.name),
                    path: boundary.path.clone(),
                    scope_type: ScopeType::Workspace,
                    confidence: 0.90,
                    reasoning: "Detected workspace configuration".to_string(),
                    files: self.discover_workspace_files(&boundary.path)?,
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
        let package_json = scope.path.join("package.json");
        if !package_json.exists() {
            return Err(PluginError::PluginNotFound("package.json not found".to_string()));
        }

        let _package_info = self.parse_package_json(&package_json)?;
        let dependencies = self.parse_dependencies(&package_json)?;
        let scripts = self.parse_scripts(&package_json)?;
        let metadata = self.parse_metadata(&scope.path)?;

        Ok(ScopeContext {
            scope_name: scope.definition.name.clone(),
            package_manager: self.detect_package_manager(&scope.path)?,
            dependencies,
            scripts,
            metadata,
        })
    }
}

impl NodePackagePlugin {
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
        if let Some(package_manager) = suggestion.metadata.get("package_manager") {
            mapping.insert(
                serde_yaml::Value::String("package_manager".to_string()),
                serde_yaml::Value::String(package_manager.as_str().unwrap_or("unknown").to_string()),
            );
        }

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

impl Default for NodePackagePlugin {
    fn default() -> Self {
        Self::new()
    }
}
