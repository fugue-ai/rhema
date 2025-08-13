use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::super::plugin::ScopeLoaderPlugin;
use super::super::types::*;
use crate::scope::Scope;

/// Configuration for the Cargo plugin
#[derive(Debug, Clone)]
pub struct CargoPluginConfig {
    pub detect_workspaces: bool,
    pub include_dev_dependencies: bool,
    pub min_crate_size: usize,
    pub max_depth: usize,
}

impl Default for CargoPluginConfig {
    fn default() -> Self {
        Self {
            detect_workspaces: true,
            include_dev_dependencies: true,
            min_crate_size: 500,
            max_depth: 5,
        }
    }
}

/// Plugin for detecting Rust Cargo packages and workspaces
pub struct CargoPlugin {
    config: CargoPluginConfig,
}

impl CargoPlugin {
    /// Create a new Cargo plugin with default configuration
    pub fn new() -> Self {
        Self {
            config: CargoPluginConfig::default(),
        }
    }

    /// Create a new Cargo plugin with custom configuration
    pub fn with_config(config: CargoPluginConfig) -> Self {
        Self { config }
    }

    /// Parse a Cargo.toml file
    fn parse_cargo_toml(&self, path: &Path) -> Result<PackageInfo, PluginError> {
        let content = std::fs::read_to_string(path).map_err(|e| PluginError::IoError(e))?;

        // Simple TOML parsing for package info
        // In a real implementation, you'd use a proper TOML parser
        let package_info = self.extract_package_info(&content)?;
        Ok(package_info)
    }

    /// Extract package information from Cargo.toml content
    fn extract_package_info(&self, content: &str) -> Result<PackageInfo, PluginError> {
        let mut name = String::new();
        let mut version = String::new();
        let mut description = None;
        let mut author = None;
        let mut license = None;
        let mut repository = None;
        let mut is_workspace = false;

        let lines: Vec<&str> = content.lines().collect();
        let mut in_package_section = false;

        for line in lines {
            let line = line.trim();

            if line == "[package]" {
                in_package_section = true;
                continue;
            }

            if line == "[workspace]" {
                is_workspace = true;
                continue;
            }

            if in_package_section && line.starts_with('[') && line.ends_with(']') {
                break; // End of package section
            }

            if in_package_section {
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim().trim_matches('"');

                    match key {
                        "name" => name = value.to_string(),
                        "version" => version = value.to_string(),
                        "description" => description = Some(value.to_string()),
                        "authors" => author = Some(value.to_string()),
                        "license" => license = Some(value.to_string()),
                        "repository" => repository = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
        }

        // If this is a workspace and we don't have a package name, use a default
        if is_workspace && name.is_empty() {
            name = "workspace".to_string();
        }

        // If we still don't have a name, try to extract it from the path or use a default
        if name.is_empty() {
            name = "cargo-project".to_string();
        }

        // If no version, use a default
        if version.is_empty() {
            version = "0.0.0".to_string();
        }

        Ok(PackageInfo {
            name,
            version,
            description,
            author,
            license,
            repository,
        })
    }

    /// Parse dependencies from Cargo.toml
    fn parse_cargo_dependencies(&self, path: &Path) -> Result<Vec<Dependency>, PluginError> {
        let content = std::fs::read_to_string(path).map_err(|e| PluginError::IoError(e))?;

        let mut dependencies = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_dependencies_section = false;

        for line in lines {
            let line = line.trim();

            if line == "[dependencies]" {
                in_dependencies_section = true;
                continue;
            }

            if in_dependencies_section && line.starts_with('[') && line.ends_with(']') {
                break; // End of dependencies section
            }

            if in_dependencies_section && !line.is_empty() && !line.starts_with('#') {
                if let Some((dep_name, dep_version)) = line.split_once('=') {
                    let dep_name = dep_name.trim();
                    let dep_version = dep_version.trim().trim_matches('"');

                    dependencies.push(Dependency {
                        name: dep_name.to_string(),
                        version: dep_version.to_string(),
                        dependency_type: DependencyType::Runtime,
                    });
                }
            }
        }

        Ok(dependencies)
    }

    /// Check if a directory is a Cargo workspace
    fn is_workspace(&self, path: &Path) -> Result<bool, PluginError> {
        let cargo_toml = path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Ok(false);
        }

        let content = std::fs::read_to_string(&cargo_toml).map_err(|e| PluginError::IoError(e))?;

        // Check for workspace section
        Ok(content.contains("[workspace]"))
    }

    /// Discover crate files
    fn discover_crate_files(&self, path: &Path) -> Result<Vec<PathBuf>, PluginError> {
        let mut files = Vec::new();

        for entry in WalkDir::new(path)
            .max_depth(self.config.max_depth)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Include Rust source files
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    files.push(path.to_path_buf());
                }
            }

            // Include Cargo.toml and Cargo.lock
            if let Some(name) = path.file_name() {
                if name == "Cargo.toml" || name == "Cargo.lock" {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }

    /// Discover workspace files
    fn discover_workspace_files(&self, path: &Path) -> Result<Vec<PathBuf>, PluginError> {
        let mut files = Vec::new();

        // Add workspace Cargo.toml
        let workspace_cargo_toml = path.join("Cargo.toml");
        if workspace_cargo_toml.exists() {
            files.push(workspace_cargo_toml);
        }

        // Add workspace Cargo.lock if it exists
        let workspace_cargo_lock = path.join("Cargo.lock");
        if workspace_cargo_lock.exists() {
            files.push(workspace_cargo_lock);
        }

        // Look for member crates
        if self.is_workspace(path)? {
            for entry in WalkDir::new(path)
                .max_depth(2)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let entry_path = entry.path();
                if entry_path.is_dir() && entry_path.join("Cargo.toml").exists() {
                    files.push(entry_path.join("Cargo.toml"));
                }
            }
        }

        Ok(files)
    }

    /// Parse Cargo metadata
    fn parse_cargo_metadata(
        &self,
        path: &Path,
    ) -> Result<HashMap<String, serde_json::Value>, PluginError> {
        let mut metadata = HashMap::new();

        // Add basic metadata
        metadata.insert(
            "package_manager".to_string(),
            serde_json::Value::String("cargo".to_string()),
        );
        metadata.insert(
            "is_workspace".to_string(),
            serde_json::Value::Bool(self.is_workspace(path)?),
        );

        // Add file count
        let files = self.discover_crate_files(path)?;
        metadata.insert(
            "file_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(files.len())),
        );

        Ok(metadata)
    }
}

impl ScopeLoaderPlugin for CargoPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "cargo".to_string(),
            version: "1.0.0".to_string(),
            description: "Detects Rust Cargo packages and workspaces".to_string(),
            supported_package_managers: vec!["cargo".to_string()],
            priority: 90,
        }
    }

    fn can_handle(&self, path: &Path) -> bool {
        path.join("Cargo.toml").exists() || path.join("Cargo.lock").exists()
    }

    fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError> {
        let mut boundaries = Vec::new();

        // Walk through the directory to find Cargo.toml files
        for entry in WalkDir::new(path)
            .max_depth(self.config.max_depth)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.file_name().and_then(|s| s.to_str()) == Some("Cargo.toml") {
                let package_info = self.parse_cargo_toml(entry_path)?;
                let dependencies = self.parse_cargo_dependencies(entry_path)?;
                let metadata = self.parse_cargo_metadata(entry_path.parent().unwrap())?;

                boundaries.push(PackageBoundary {
                    path: entry_path.parent().unwrap().to_path_buf(),
                    package_manager: PackageManager::Cargo,
                    package_info,
                    dependencies,
                    scripts: HashMap::new(), // Cargo doesn't have scripts like npm
                    metadata,
                });
            }
        }

        Ok(boundaries)
    }

    fn suggest_scopes(
        &self,
        boundaries: &[PackageBoundary],
    ) -> Result<Vec<ScopeSuggestion>, PluginError> {
        let mut suggestions = Vec::new();

        for boundary in boundaries {
            // Create crate scope
            suggestions.push(ScopeSuggestion {
                name: boundary.package_info.name.clone(),
                path: boundary.path.clone(),
                scope_type: ScopeType::Library,
                confidence: 0.95,
                reasoning: "Detected Cargo.toml with valid configuration".to_string(),
                files: self.discover_crate_files(&boundary.path)?,
                dependencies: boundary
                    .dependencies
                    .iter()
                    .map(|d| d.name.clone())
                    .collect(),
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
                std::fs::create_dir_all(&rhema_dir).map_err(|e| {
                    PluginError::PluginExecutionFailed(format!(
                        "Failed to create .rhema directory: {}",
                        e
                    ))
                })?;
            }

            // Create rhema.yaml file
            let rhema_content = self.generate_rhema_yaml(suggestion)?;
            let rhema_file = rhema_dir.join("rhema.yaml");
            std::fs::write(&rhema_file, rhema_content).map_err(|e| {
                PluginError::PluginExecutionFailed(format!("Failed to write rhema.yaml: {}", e))
            })?;

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
        let cargo_toml = scope.path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(PluginError::PluginNotFound(
                "Cargo.toml not found".to_string(),
            ));
        }

        let _package_info = self.parse_cargo_toml(&cargo_toml)?;
        let dependencies = self.parse_cargo_dependencies(&cargo_toml)?;
        let metadata = self.parse_cargo_metadata(&scope.path)?;

        Ok(ScopeContext {
            scope_name: scope.definition.name.clone(),
            package_manager: PackageManager::Cargo,
            dependencies,
            scripts: HashMap::new(),
            metadata,
        })
    }
}

impl CargoPlugin {
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
            serde_yaml::Value::String("cargo".to_string()),
        );

        // Add dependencies if any
        if !suggestion.dependencies.is_empty() {
            let deps: Vec<serde_yaml::Value> = suggestion
                .dependencies
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
            let metadata_value = serde_yaml::to_value(&suggestion.metadata).map_err(|e| {
                PluginError::PluginExecutionFailed(format!("Failed to serialize metadata: {}", e))
            })?;
            mapping.insert(
                serde_yaml::Value::String("metadata".to_string()),
                metadata_value,
            );
        }

        serde_yaml::to_string(&mapping).map_err(|e| {
            PluginError::PluginExecutionFailed(format!("Failed to serialize YAML: {}", e))
        })
    }
}

impl Default for CargoPlugin {
    fn default() -> Self {
        Self::new()
    }
}
