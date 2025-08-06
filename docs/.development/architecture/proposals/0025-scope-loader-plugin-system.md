# Scope Loader Plugin System

**Proposal**: Implement a comprehensive scope loader plugin system that can automatically detect boundaries of known package systems (npm, cargo, pnpm, yarn, etc.) and intelligently create Rhema scopes based on these boundaries.

## Problem Statement

The current Rhema scope discovery system has several limitations:

- **Manual Scope Creation**: Users must manually create `.rhema` directories and scope definitions
- **No Package System Awareness**: Rhema doesn't understand common package manager boundaries
- **Inconsistent Scope Boundaries**: Different users may create scopes with different boundaries for the same codebase
- **Missing Context**: No automatic detection of project structure, dependencies, and relationships
- **Limited Integration**: No integration with existing package manager workflows
- **Scalability Issues**: Manual scope management doesn't scale well for large monorepos

## Current Status

### ✅ **Implemented Components**

1. **Basic Scope Discovery**
   - Manual `.rhema` directory detection (`crates/rhema-core/src/scope.rs`)
   - Scope file parsing and validation
   - Dependency graph building
   - Scope hierarchy management

2. **Scope Management**
   - Scope definition schema (`schemas/scope.json`)
   - File discovery within scopes
   - Cross-scope dependency analysis
   - Scope validation and error handling

### ❌ **Current Limitations**

- Only discovers manually created `.rhema` directories
- No automatic scope boundary detection
- No package manager integration
- No intelligent scope suggestions
- Limited support for monorepo structures

## Proposed Solution

Implement a comprehensive scope loader plugin system that provides:

- **Automatic Package Boundary Detection**: Detect npm, cargo, pnpm, yarn, and other package manager boundaries
- **Plugin Architecture**: Extensible plugin system for different package managers and project types
- **Intelligent Scope Creation**: Automatically suggest and create scopes based on detected boundaries
- **Monorepo Support**: Handle complex monorepo structures with multiple package managers
- **Context-Aware Loading**: Load relevant context based on package structure and dependencies
- **Integration with Existing Workflows**: Seamlessly integrate with existing package manager workflows

## Core Components

### A. Plugin System Architecture

```rust
pub trait ScopeLoaderPlugin: Send + Sync {
    /// Plugin metadata
    fn metadata(&self) -> PluginMetadata;
    
    /// Check if this plugin can handle the given directory
    fn can_handle(&self, path: &Path) -> bool;
    
    /// Detect package boundaries in the given directory
    fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError>;
    
    /// Generate scope suggestions based on detected boundaries
    fn suggest_scopes(&self, boundaries: &[PackageBoundary]) -> Result<Vec<ScopeSuggestion>, PluginError>;
    
    /// Create scopes from suggestions
    fn create_scopes(&self, suggestions: &[ScopeSuggestion]) -> Result<Vec<Scope>, PluginError>;
    
    /// Load context for a specific scope
    fn load_context(&self, scope: &Scope) -> Result<ScopeContext, PluginError>;
}

pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub supported_package_managers: Vec<String>,
    pub priority: u32,
}
```

### B. Package Boundary Detection

```rust
pub struct PackageBoundary {
    pub path: PathBuf,
    pub package_manager: PackageManager,
    pub package_info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    pub scripts: HashMap<String, String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Cargo,
    Pip,
    Poetry,
    Go,
    Maven,
    Gradle,
    Custom(String),
}

pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
}
```

### C. Scope Suggestions

```rust
pub struct ScopeSuggestion {
    pub name: String,
    pub path: PathBuf,
    pub scope_type: ScopeType,
    pub confidence: f64,
    pub reasoning: String,
    pub files: Vec<PathBuf>,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

pub enum ScopeType {
    Package,
    Workspace,
    Monorepo,
    Service,
    Library,
    Application,
    Test,
    Documentation,
    Configuration,
    Custom(String),
}
```

### D. Plugin Registry

```rust
pub struct PluginRegistry {
    plugins: Vec<Box<dyn ScopeLoaderPlugin>>,
    plugin_configs: HashMap<String, PluginConfig>,
}

impl PluginRegistry {
    pub fn register_plugin(&mut self, plugin: Box<dyn ScopeLoaderPlugin>) -> Result<(), RegistryError>;
    
    pub fn get_plugins_for_path(&self, path: &Path) -> Vec<&dyn ScopeLoaderPlugin>;
    
    pub fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError>;
    
    pub fn suggest_scopes(&self, path: &Path) -> Result<Vec<ScopeSuggestion>, PluginError>;
    
    pub fn create_scopes(&self, path: &Path, suggestions: &[ScopeSuggestion]) -> Result<Vec<Scope>, PluginError>;
}
```

## Implementation Architecture

### A. Core Plugin System

```rust
pub struct ScopeLoaderService {
    registry: PluginRegistry,
    cache: Arc<ScopeCache>,
    config: ScopeLoaderConfig,
}

impl ScopeLoaderService {
    pub async fn discover_scopes(&self, path: &Path) -> Result<Vec<Scope>, ScopeLoaderError> {
        // 1. Find applicable plugins
        let plugins = self.registry.get_plugins_for_path(path);
        
        // 2. Detect package boundaries
        let mut all_boundaries = Vec::new();
        for plugin in plugins {
            let boundaries = plugin.detect_boundaries(path)?;
            all_boundaries.extend(boundaries);
        }
        
        // 3. Generate scope suggestions
        let suggestions = self.generate_suggestions(&all_boundaries)?;
        
        // 4. Create scopes
        self.create_scopes_from_suggestions(&suggestions)
    }
    
    pub async fn auto_create_scopes(&self, path: &Path) -> Result<Vec<Scope>, ScopeLoaderError> {
        let suggestions = self.suggest_scopes(path)?;
        
        // Filter high-confidence suggestions
        let high_confidence: Vec<_> = suggestions
            .into_iter()
            .filter(|s| s.confidence >= self.config.min_confidence_threshold)
            .collect();
        
        self.create_scopes_from_suggestions(&high_confidence)
    }
}
```

### B. Built-in Plugins

#### 1. NPM/Yarn/Pnpm Plugin

```rust
pub struct NodePackagePlugin {
    config: NodePluginConfig,
}

impl ScopeLoaderPlugin for NodePackagePlugin {
    fn can_handle(&self, path: &Path) -> bool {
        path.join("package.json").exists() || 
        path.join("yarn.lock").exists() || 
        path.join("pnpm-lock.yaml").exists()
    }
    
    fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError> {
        let mut boundaries = Vec::new();
        
        // Detect package.json files
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == "package.json" {
                let package_info = self.parse_package_json(entry.path())?;
                let package_manager = self.detect_package_manager(entry.path().parent().unwrap())?;
                
                boundaries.push(PackageBoundary {
                    path: entry.path().parent().unwrap().to_path_buf(),
                    package_manager,
                    package_info,
                    dependencies: self.parse_dependencies(entry.path())?,
                    scripts: self.parse_scripts(entry.path())?,
                    metadata: self.parse_metadata(entry.path())?,
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
}
```

#### 2. Cargo Plugin

```rust
pub struct CargoPlugin {
    config: CargoPluginConfig,
}

impl ScopeLoaderPlugin for CargoPlugin {
    fn can_handle(&self, path: &Path) -> bool {
        path.join("Cargo.toml").exists() || path.join("Cargo.lock").exists()
    }
    
    fn detect_boundaries(&self, path: &Path) -> Result<Vec<PackageBoundary>, PluginError> {
        let mut boundaries = Vec::new();
        
        // Detect Cargo.toml files
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == "Cargo.toml" {
                let package_info = self.parse_cargo_toml(entry.path())?;
                
                boundaries.push(PackageBoundary {
                    path: entry.path().parent().unwrap().to_path_buf(),
                    package_manager: PackageManager::Cargo,
                    package_info,
                    dependencies: self.parse_cargo_dependencies(entry.path())?,
                    scripts: HashMap::new(), // Cargo doesn't have scripts like npm
                    metadata: self.parse_cargo_metadata(entry.path())?,
                });
            }
        }
        
        Ok(boundaries)
    }
    
    fn suggest_scopes(&self, boundaries: &[PackageBoundary]) -> Result<Vec<ScopeSuggestion>, PluginError> {
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
}
```

### C. CLI Integration

```rust
// New CLI commands
#[derive(Subcommand)]
pub enum ScopeCommands {
    /// Auto-discover and create scopes based on package boundaries
    AutoDiscover {
        /// Path to scan for package boundaries
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Minimum confidence threshold for scope creation
        #[arg(long, default_value = "0.8")]
        confidence: f64,
        
        /// Create scopes automatically without confirmation
        #[arg(long)]
        auto_create: bool,
        
        /// Show detailed reasoning for scope suggestions
        #[arg(long)]
        verbose: bool,
    },
    
    /// List available scope loader plugins
    ListPlugins,
    
    /// Test plugin detection on a specific path
    TestPlugin {
        /// Path to test
        path: PathBuf,
        
        /// Plugin name to test (optional)
        #[arg(long)]
        plugin: Option<String>,
    },
    
    /// Generate scope suggestions without creating them
    Suggest {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Output format
        #[arg(long, default_value = "table")]
        format: OutputFormat,
    },
}
```

## Implementation Roadmap

### Phase 1: Core Plugin System (4-6 weeks)

1. **Plugin Architecture Foundation**
   - Implement `ScopeLoaderPlugin` trait
   - Create `PluginRegistry` with plugin management
   - Build `ScopeLoaderService` core functionality
   - Add plugin configuration system

2. **Package Boundary Detection**
   - Implement `PackageBoundary` and related structures
   - Create package manager detection logic
   - Build dependency parsing for common formats
   - Add metadata extraction capabilities

3. **Scope Suggestion Engine**
   - Implement `ScopeSuggestion` generation
   - Create confidence scoring algorithms
   - Build reasoning system for suggestions
   - Add scope type classification

### Phase 2: Built-in Plugins (3-4 weeks)

1. **Node.js Package Manager Plugin**
   - Support for npm, yarn, and pnpm
   - Package.json parsing and analysis
   - Workspace detection and handling
   - Script and dependency analysis

2. **Cargo Plugin**
   - Cargo.toml parsing and analysis
   - Workspace detection
   - Crate dependency analysis
   - Rust-specific metadata extraction

3. **Additional Package Manager Plugins**
   - Python (pip, poetry)
   - Go modules
   - Java (Maven, Gradle)
   - Custom plugin template

### Phase 3: CLI Integration (2-3 weeks)

1. **New CLI Commands**
   - `rhema scope auto-discover` command
   - `rhema scope suggest` command
   - `rhema scope list-plugins` command
   - `rhema scope test-plugin` command

2. **Interactive Scope Creation**
   - Confirmation prompts for scope creation
   - Scope preview and editing
   - Batch scope creation
   - Undo/rollback capabilities

3. **Configuration Management**
   - Plugin configuration files
   - Global and project-specific settings
   - Plugin enable/disable controls
   - Custom confidence thresholds

### Phase 4: Advanced Features (3-4 weeks)

1. **Monorepo Support**
   - Multi-package manager detection
   - Cross-package dependency analysis
   - Hierarchical scope organization
   - Monorepo-specific scope types

2. **Context Loading**
   - Automatic context file generation
   - Package-specific context templates
   - Dependency-aware context loading
   - Cross-scope context sharing

3. **Performance Optimization**
   - Plugin caching and memoization
   - Parallel boundary detection
   - Incremental scope updates
   - Background scope discovery

## Benefits

### Technical Benefits

- **Automated Scope Management**: Reduces manual scope creation overhead
- **Consistent Boundaries**: Ensures consistent scope boundaries across projects
- **Package Manager Integration**: Seamless integration with existing workflows
- **Extensible Architecture**: Plugin system allows for easy extension to new package managers
- **Performance**: Efficient boundary detection and scope creation
- **Scalability**: Handles large monorepos and complex project structures

### User Experience Improvements

- **Zero-Configuration Setup**: Automatic scope discovery for new projects
- **Intelligent Suggestions**: Context-aware scope recommendations
- **Interactive Workflow**: Preview and confirm scope creation
- **Batch Operations**: Handle multiple scopes efficiently
- **Undo Capabilities**: Rollback scope creation if needed
- **Visual Feedback**: Clear indication of detected boundaries and suggestions

### Business Impact

- **Reduced Onboarding Time**: New users can start using Rhema immediately
- **Improved Adoption**: Lower barrier to entry for scope-based development
- **Better Context Management**: More accurate and comprehensive context boundaries
- **Enhanced Collaboration**: Consistent scope boundaries across teams
- **Monorepo Support**: Better support for complex project structures

## Success Metrics

### Technical Metrics

- **Scope Detection Accuracy**: >95% accuracy in detecting package boundaries
- **Performance**: <2 seconds for boundary detection in typical projects
- **Plugin Coverage**: Support for top 10 package managers by usage
- **Memory Usage**: <50MB additional memory usage for plugin system
- **Error Rate**: <1% error rate in scope creation

### User Experience Metrics

- **Setup Time**: <5 minutes for initial scope setup
- **User Satisfaction**: >90% satisfaction with auto-discovered scopes
- **Adoption Rate**: >80% of users enable auto-discovery
- **Reduction in Manual Work**: >70% reduction in manual scope creation
- **Support Requests**: <10% increase in support requests related to scope management

### Business Metrics

- **User Onboarding**: 50% reduction in time to first successful scope creation
- **Feature Adoption**: 75% of users use auto-discovery within 30 days
- **Monorepo Support**: 90% of monorepo users successfully use the system
- **Plugin Ecosystem**: 5+ community-contributed plugins within 6 months

## Integration with Existing Features

### Enhanced Scope Discovery

The plugin system will enhance the existing `discover_scopes` function:

```rust
// Enhanced scope discovery with plugin support
pub fn discover_scopes(repo_root: &Path) -> Result<Vec<Scope>, RhemaError> {
    let mut scopes = Vec::new();
    
    // First, try plugin-based discovery
    if let Ok(plugin_scopes) = scope_loader_service.discover_scopes(repo_root).await {
        scopes.extend(plugin_scopes);
    }
    
    // Fall back to manual .rhema directory discovery
    for entry in WalkDir::new(repo_root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.is_dir() && path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
            if let Ok(scope) = Scope::new(path.to_path_buf()) {
                scopes.push(scope);
            }
        }
    }
    
    Ok(scopes)
}
```

### MCP Integration

The plugin system will integrate with the MCP system to provide real-time scope updates:

```rust
// MCP integration for real-time scope updates
impl RhemaMcpService {
    pub async fn handle_scope_discovery(&self, request: ScopeDiscoveryRequest) -> Result<ScopeDiscoveryResponse, McpError> {
        let scopes = self.scope_loader.discover_scopes(&request.path).await?;
        
        Ok(ScopeDiscoveryResponse {
            scopes: scopes.into_iter().map(|s| s.into()).collect(),
            boundaries: self.scope_loader.detect_boundaries(&request.path).await?,
            suggestions: self.scope_loader.suggest_scopes(&request.path).await?,
        })
    }
}
```

### Dependency Management Integration

The plugin system will enhance dependency analysis:

```rust
// Enhanced dependency analysis with package boundaries
impl DependencyManager {
    pub async fn analyze_dependencies_with_boundaries(&self, path: &Path) -> Result<DependencyAnalysis, DependencyError> {
        let boundaries = self.scope_loader.detect_boundaries(path).await?;
        let scopes = self.scope_loader.discover_scopes(path).await?;
        
        // Combine package manager dependencies with Rhema scope dependencies
        let mut analysis = DependencyAnalysis::new();
        
        for boundary in boundaries {
            analysis.add_package_dependencies(&boundary.dependencies);
        }
        
        for scope in scopes {
            analysis.add_scope_dependencies(&scope.get_dependency_paths());
        }
        
        Ok(analysis)
    }
}
```

## Configuration

### Plugin Configuration

```yaml
# .rhema/scope-loader.yaml
plugins:
  node:
    enabled: true
    priority: 100
    config:
      detect_workspaces: true
      include_dev_dependencies: false
      min_package_size: 1000
  
  cargo:
    enabled: true
    priority: 90
    config:
      detect_workspaces: true
      include_dev_dependencies: true
      min_crate_size: 500
  
  python:
    enabled: false
    priority: 80
    config:
      detect_virtual_envs: true
      include_test_dependencies: false

auto_discovery:
  enabled: true
  min_confidence: 0.8
  auto_create: false
  confirm_prompt: true
  
monorepo:
  enabled: true
  max_depth: 5
  cross_package_analysis: true
  hierarchical_scopes: true

caching:
  enabled: true
  cache_duration: 3600
  cache_path: ".rhema/cache"
```

### Global Configuration

```yaml
# ~/.rhema/config.yaml
scope_loader:
  plugins_path: "~/.rhema/plugins"
  auto_update_plugins: true
  plugin_timeout: 30
  
  default_plugins:
    - node
    - cargo
    - python
    - go
  
  plugin_configs:
    node:
      detect_workspaces: true
    cargo:
      detect_workspaces: true
```

## Testing Strategy

### Unit Tests

- Plugin trait implementation tests
- Package boundary detection tests
- Scope suggestion generation tests
- Plugin registry tests

### Integration Tests

- End-to-end scope discovery tests
- Multi-package manager scenarios
- Monorepo boundary detection
- Plugin configuration tests

### Performance Tests

- Large repository boundary detection
- Plugin loading performance
- Memory usage benchmarks
- Cache effectiveness tests

### User Acceptance Tests

- Real-world project scenarios
- Different package manager combinations
- Monorepo complexity levels
- User workflow validation

## Future Enhancements

### Advanced Plugin Features

- **Custom Plugin Development**: SDK for creating custom plugins
- **Plugin Marketplace**: Community plugin repository
- **Plugin Versioning**: Plugin dependency management
- **Plugin Sandboxing**: Security isolation for plugins

### Enhanced Detection

- **AI-Powered Boundary Detection**: Machine learning for boundary detection
- **Historical Analysis**: Learn from previous scope decisions
- **Team Collaboration**: Share scope patterns across teams
- **Context-Aware Suggestions**: Suggest scopes based on development patterns

### Integration Extensions

- **IDE Integration**: Real-time scope updates in IDEs
- **CI/CD Integration**: Automated scope validation in pipelines
- **Git Hooks**: Automatic scope updates on repository changes
- **Cloud Integration**: Remote scope discovery and sharing

---

*Proposal ID: 0025*  
*Status: Draft*  
*Priority: High*  
*Effort: 12-16 weeks*  
*Owner: Development Team*  
*Created: January 2025*  
*Next Review: February 2025* 